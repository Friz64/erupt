// This code roughly follows https://vulkan-tutorial.com/ - Drawing a triangle

use erupt::{
    cstr,
    extensions::{ext_debug_utils::*, khr_surface::*, khr_swapchain::*},
    utils::{self, surface},
    vk1_0::*,
    CoreLoader, DeviceLoader, InstanceLoader,
};
use std::{
    ffi::{c_void, CStr, CString},
    os::raw::c_char,
};
use structopt::StructOpt;
use winit::{
    event::{
        DeviceEvent, ElementState, Event, KeyboardInput, StartCause, VirtualKeyCode, WindowEvent,
    },
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

const TITLE: &str = "erupt_examples: triangle";
const FRAMES_IN_FLIGHT: usize = 2;
const LAYER_KHRONOS_VALIDATION: *const c_char = cstr!("VK_LAYER_KHRONOS_validation");
const SHADER_VERT: &[u8] = include_bytes!("triangle.vert.spv");
const SHADER_FRAG: &[u8] = include_bytes!("triangle.frag.spv");

#[derive(Debug, StructOpt)]
struct Opt {
    /// Use validation layers
    #[structopt(short, long)]
    validation_layers: bool,
}

unsafe extern "system" fn debug_callback(
    _message_severity: DebugUtilsMessageSeverityFlagBitsEXT,
    _message_types: DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const DebugUtilsMessengerCallbackDataEXT,
    _p_user_data: *mut c_void,
) -> Bool32 {
    eprintln!(
        "{}",
        CStr::from_ptr((*p_callback_data).p_message).to_string_lossy()
    );

    FALSE
}

fn main() {
    let opt = Opt::from_args();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(TITLE)
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();

    let mut core = CoreLoader::new().unwrap();
    core.load_vk1_0().unwrap();

    let api_version = core.instance_version();
    println!(
        "{} - Vulkan {}.{}.{}",
        TITLE,
        erupt::version_major(api_version),
        erupt::version_minor(api_version),
        erupt::version_patch(api_version)
    );

    // https://vulkan-tutorial.com/Drawing_a_triangle/Setup/Instance
    let application_name = CString::new("Hello Triangle").unwrap();
    let engine_name = CString::new("No Engine").unwrap();
    let app_info = ApplicationInfoBuilder::new()
        .application_name(&application_name)
        .application_version(erupt::make_version(1, 0, 0))
        .engine_name(&engine_name)
        .engine_version(erupt::make_version(1, 0, 0))
        .api_version(erupt::make_version(1, 0, 0));

    let mut instance_extensions = surface::enumerate_required_extensions(&window).unwrap();
    if opt.validation_layers {
        instance_extensions.push(EXT_DEBUG_UTILS_EXTENSION_NAME);
    }

    let mut instance_layers = Vec::new();
    if opt.validation_layers {
        instance_layers.push(LAYER_KHRONOS_VALIDATION);
    }

    let device_extensions = vec![KHR_SWAPCHAIN_EXTENSION_NAME];

    let mut device_layers = Vec::new();
    if opt.validation_layers {
        device_layers.push(LAYER_KHRONOS_VALIDATION);
    }

    let create_info = InstanceCreateInfoBuilder::new()
        .application_info(&app_info)
        .enabled_extension_names(&instance_extensions)
        .enabled_layer_names(&instance_layers);

    let mut instance = InstanceLoader::new(
        &core,
        unsafe { core.create_instance(&create_info, None, None) }.unwrap(),
    )
    .unwrap();
    instance.load_vk1_0().unwrap();

    // https://vulkan-tutorial.com/Drawing_a_triangle/Setup/Validation_layers
    let messenger = if opt.validation_layers {
        instance.load_ext_debug_utils().unwrap();

        let create_info = DebugUtilsMessengerCreateInfoEXTBuilder::new()
            .message_severity(
                DebugUtilsMessageSeverityFlagsEXT::VERBOSE_EXT
                    | DebugUtilsMessageSeverityFlagsEXT::WARNING_EXT
                    | DebugUtilsMessageSeverityFlagsEXT::ERROR_EXT,
            )
            .message_type(
                DebugUtilsMessageTypeFlagsEXT::GENERAL_EXT
                    | DebugUtilsMessageTypeFlagsEXT::VALIDATION_EXT
                    | DebugUtilsMessageTypeFlagsEXT::PERFORMANCE_EXT,
            )
            .pfn_user_callback(Some(debug_callback));

        unsafe { instance.create_debug_utils_messenger_ext(&create_info, None, None) }.unwrap()
    } else {
        Default::default()
    };

    // https://vulkan-tutorial.com/Drawing_a_triangle/Presentation/Window_surface
    let surface = unsafe { surface::create_surface(&mut instance, &window, None) }.unwrap();

    // https://vulkan-tutorial.com/Drawing_a_triangle/Setup/Physical_devices_and_queue_families
    let (physical_device, queue_family, format, present_mode, properties) =
        unsafe { instance.enumerate_physical_devices(None) }
            .unwrap()
            .into_iter()
            .filter_map(|physical_device| unsafe {
                let queue_family = match instance
                    .get_physical_device_queue_family_properties(physical_device, None)
                    .into_iter()
                    .enumerate()
                    .position(|(i, properties)| {
                        properties.queue_flags.contains(QueueFlags::GRAPHICS)
                            && instance
                                .get_physical_device_surface_support_khr(
                                    physical_device,
                                    i as u32,
                                    surface,
                                    None,
                                )
                                .unwrap()
                                == true
                    }) {
                    Some(queue_family) => queue_family as u32,
                    None => return None,
                };

                let formats = instance
                    .get_physical_device_surface_formats_khr(physical_device, surface, None)
                    .unwrap();
                let format = match formats
                    .iter()
                    .find(|surface_format| {
                        surface_format.format == Format::B8G8R8A8_SRGB
                            && surface_format.color_space == ColorSpaceKHR::SRGB_NONLINEAR_KHR
                    })
                    .or_else(|| formats.get(0))
                {
                    Some(surface_format) => surface_format.clone(),
                    None => return None,
                };

                let present_mode = instance
                    .get_physical_device_surface_present_modes_khr(physical_device, surface, None)
                    .unwrap()
                    .into_iter()
                    .find(|present_mode| present_mode == &PresentModeKHR::MAILBOX_KHR)
                    .unwrap_or(PresentModeKHR::FIFO_KHR);

                let supported_extensions = instance
                    .enumerate_device_extension_properties(physical_device, None, None)
                    .unwrap();
                if !device_extensions.iter().all(|device_extension| {
                    let device_extension = CStr::from_ptr(*device_extension);

                    supported_extensions.iter().any(|properties| {
                        CStr::from_ptr(properties.extension_name.as_ptr()) == device_extension
                    })
                }) {
                    return None;
                }

                let properties = instance.get_physical_device_properties(physical_device, None);
                Some((
                    physical_device,
                    queue_family,
                    format,
                    present_mode,
                    properties,
                ))
            })
            .max_by_key(|(_, _, _, _, properties)| match properties.device_type {
                PhysicalDeviceType::DISCRETE_GPU => 2,
                PhysicalDeviceType::INTEGRATED_GPU => 1,
                _ => 0,
            })
            .expect("No suitable physical device found");

    println!("Using physical device: {:?}", unsafe {
        CStr::from_ptr(properties.device_name.as_ptr())
    });

    // https://vulkan-tutorial.com/Drawing_a_triangle/Setup/Logical_device_and_queues
    let queue_create_info = vec![DeviceQueueCreateInfoBuilder::new()
        .queue_family_index(queue_family)
        .queue_priorities(&[1.0])];
    let features = PhysicalDeviceFeaturesBuilder::new();

    let create_info = DeviceCreateInfoBuilder::new()
        .queue_create_infos(&queue_create_info)
        .enabled_features(&features)
        .enabled_extension_names(&device_extensions)
        .enabled_layer_names(&device_layers);

    let mut device = DeviceLoader::new(
        &instance,
        unsafe { instance.create_device(physical_device, &create_info, None, None) }.unwrap(),
    )
    .unwrap();
    device.load_vk1_0().unwrap();
    device.load_khr_swapchain().unwrap();

    let queue = unsafe { device.get_device_queue(queue_family, 0, None) };

    // https://vulkan-tutorial.com/Drawing_a_triangle/Presentation/Swap_chain
    let surface_caps = unsafe {
        instance.get_physical_device_surface_capabilities_khr(physical_device, surface, None)
    }
    .unwrap();
    let mut image_count = surface_caps.min_image_count + 1;
    if surface_caps.max_image_count > 0 && image_count > surface_caps.max_image_count {
        image_count = surface_caps.max_image_count;
    }

    let create_info = SwapchainCreateInfoKHRBuilder::new()
        .surface(surface)
        .min_image_count(image_count)
        .image_format(format.format)
        .image_color_space(format.color_space)
        .image_extent(surface_caps.current_extent)
        .image_array_layers(1)
        .image_usage(ImageUsageFlags::COLOR_ATTACHMENT)
        .image_sharing_mode(SharingMode::EXCLUSIVE)
        .pre_transform(surface_caps.current_transform)
        .composite_alpha(CompositeAlphaFlagBitsKHR::OPAQUE_KHR)
        .present_mode(present_mode)
        .clipped(true)
        .old_swapchain(SwapchainKHR::null());

    let swapchain = unsafe { device.create_swapchain_khr(&create_info, None, None) }.unwrap();
    let swapchain_images = unsafe { device.get_swapchain_images_khr(swapchain, None) }.unwrap();

    // https://vulkan-tutorial.com/Drawing_a_triangle/Presentation/Image_views
    let swapchain_image_views: Vec<_> = swapchain_images
        .iter()
        .map(|swapchain_image| {
            let create_info = ImageViewCreateInfoBuilder::new()
                .image(*swapchain_image)
                .view_type(ImageViewType::_2D)
                .format(format.format)
                .components(ComponentMapping {
                    r: ComponentSwizzle::IDENTITY,
                    g: ComponentSwizzle::IDENTITY,
                    b: ComponentSwizzle::IDENTITY,
                    a: ComponentSwizzle::IDENTITY,
                })
                .subresource_range(unsafe {
                    ImageSubresourceRangeBuilder::new()
                        .aspect_mask(ImageAspectFlags::COLOR)
                        .base_mip_level(0)
                        .level_count(1)
                        .base_array_layer(0)
                        .layer_count(1)
                        .discard()
                });
            unsafe { device.create_image_view(&create_info, None, None) }.unwrap()
        })
        .collect();

    // https://vulkan-tutorial.com/Drawing_a_triangle/Graphics_pipeline_basics/Shader_modules
    let entry_point = CString::new("main").unwrap();

    let vert_decoded = utils::decode_spv(SHADER_VERT).unwrap();
    let create_info = ShaderModuleCreateInfoBuilder::new().code(&vert_decoded);
    let shader_vert = unsafe { device.create_shader_module(&create_info, None, None) }.unwrap();

    let frag_decoded = utils::decode_spv(SHADER_FRAG).unwrap();
    let create_info = ShaderModuleCreateInfoBuilder::new().code(&frag_decoded);
    let shader_frag = unsafe { device.create_shader_module(&create_info, None, None) }.unwrap();

    let shader_stages = vec![
        PipelineShaderStageCreateInfoBuilder::new()
            .stage(ShaderStageFlagBits::VERTEX)
            .module(shader_vert)
            .name(&entry_point),
        PipelineShaderStageCreateInfoBuilder::new()
            .stage(ShaderStageFlagBits::FRAGMENT)
            .module(shader_frag)
            .name(&entry_point),
    ];

    // https://vulkan-tutorial.com/Drawing_a_triangle/Graphics_pipeline_basics/Fixed_functions
    let vertex_input = PipelineVertexInputStateCreateInfoBuilder::new();

    let input_assembly = PipelineInputAssemblyStateCreateInfoBuilder::new()
        .topology(PrimitiveTopology::TRIANGLE_LIST)
        .primitive_restart_enable(false);

    let viewports = vec![ViewportBuilder::new()
        .x(0.0)
        .y(0.0)
        .width(surface_caps.current_extent.width as f32)
        .height(surface_caps.current_extent.height as f32)
        .min_depth(0.0)
        .max_depth(1.0)];
    let scissors = vec![Rect2DBuilder::new()
        .offset(Offset2D { x: 0, y: 0 })
        .extent(surface_caps.current_extent)];
    let viewport_state = PipelineViewportStateCreateInfoBuilder::new()
        .viewports(&viewports)
        .scissors(&scissors);

    let rasterizer = PipelineRasterizationStateCreateInfoBuilder::new()
        .depth_clamp_enable(false)
        .rasterizer_discard_enable(false)
        .polygon_mode(PolygonMode::FILL)
        .line_width(1.0)
        .cull_mode(CullModeFlags::BACK)
        .front_face(FrontFace::CLOCKWISE)
        .depth_clamp_enable(false);

    let multisampling = PipelineMultisampleStateCreateInfoBuilder::new()
        .sample_shading_enable(false)
        .rasterization_samples(SampleCountFlagBits::_1);

    let color_blend_attachments = vec![PipelineColorBlendAttachmentStateBuilder::new()
        .color_write_mask(
            ColorComponentFlags::R
                | ColorComponentFlags::G
                | ColorComponentFlags::B
                | ColorComponentFlags::A,
        )
        .blend_enable(false)];
    let color_blending = PipelineColorBlendStateCreateInfoBuilder::new()
        .logic_op_enable(false)
        .attachments(&color_blend_attachments);

    let create_info = PipelineLayoutCreateInfoBuilder::new();
    let pipeline_layout =
        unsafe { device.create_pipeline_layout(&create_info, None, None) }.unwrap();

    // https://vulkan-tutorial.com/Drawing_a_triangle/Graphics_pipeline_basics/Render_passes
    let attachments = vec![AttachmentDescriptionBuilder::new()
        .format(format.format)
        .samples(SampleCountFlagBits::_1)
        .load_op(AttachmentLoadOp::CLEAR)
        .store_op(AttachmentStoreOp::STORE)
        .stencil_load_op(AttachmentLoadOp::DONT_CARE)
        .stencil_store_op(AttachmentStoreOp::DONT_CARE)
        .initial_layout(ImageLayout::UNDEFINED)
        .final_layout(ImageLayout::PRESENT_SRC_KHR)];

    let color_attachment_refs = vec![AttachmentReferenceBuilder::new()
        .attachment(0)
        .layout(ImageLayout::COLOR_ATTACHMENT_OPTIMAL)];
    let subpasses = vec![SubpassDescriptionBuilder::new()
        .pipeline_bind_point(PipelineBindPoint::GRAPHICS)
        .color_attachments(&color_attachment_refs)];
    let dependencies = vec![SubpassDependencyBuilder::new()
        .src_subpass(SUBPASS_EXTERNAL)
        .dst_subpass(0)
        .src_stage_mask(PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
        .src_access_mask(AccessFlags::empty())
        .dst_stage_mask(PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
        .dst_access_mask(AccessFlags::COLOR_ATTACHMENT_WRITE)];

    let create_info = RenderPassCreateInfoBuilder::new()
        .attachments(&attachments)
        .subpasses(&subpasses)
        .dependencies(&dependencies);

    let render_pass = unsafe { device.create_render_pass(&create_info, None, None) }.unwrap();

    // https://vulkan-tutorial.com/Drawing_a_triangle/Graphics_pipeline_basics/Conclusion
    let create_info = GraphicsPipelineCreateInfoBuilder::new()
        .stages(&shader_stages)
        .vertex_input_state(&vertex_input)
        .input_assembly_state(&input_assembly)
        .viewport_state(&viewport_state)
        .rasterization_state(&rasterizer)
        .multisample_state(&multisampling)
        .color_blend_state(&color_blending)
        .layout(pipeline_layout)
        .render_pass(render_pass)
        .subpass(0);

    let pipeline =
        unsafe { device.create_graphics_pipelines(PipelineCache::null(), &[create_info], None) }
            .unwrap()[0];

    // https://vulkan-tutorial.com/Drawing_a_triangle/Drawing/Framebuffers
    let swapchain_framebuffers: Vec<_> = swapchain_image_views
        .iter()
        .map(|image_view| {
            let attachments = vec![*image_view];
            let create_info = FramebufferCreateInfoBuilder::new()
                .render_pass(render_pass)
                .attachments(&attachments)
                .width(surface_caps.current_extent.width)
                .height(surface_caps.current_extent.height)
                .layers(1);

            unsafe { device.create_framebuffer(&create_info, None, None) }.unwrap()
        })
        .collect();

    // https://vulkan-tutorial.com/Drawing_a_triangle/Drawing/Command_buffers
    let create_info = CommandPoolCreateInfoBuilder::new().queue_family_index(queue_family);
    let command_pool = unsafe { device.create_command_pool(&create_info, None, None) }.unwrap();

    let allocate_info = CommandBufferAllocateInfoBuilder::new()
        .command_pool(command_pool)
        .level(CommandBufferLevel::PRIMARY)
        .command_buffer_count(swapchain_framebuffers.len() as _);
    let command_buffers = unsafe { device.allocate_command_buffers(&allocate_info) }.unwrap();

    for (&command_buffer, &framebuffer) in command_buffers.iter().zip(swapchain_framebuffers.iter())
    {
        let begin_info = CommandBufferBeginInfoBuilder::new();
        unsafe { device.begin_command_buffer(command_buffer, &begin_info) }.unwrap();

        let clear_values = vec![ClearValue {
            color: ClearColorValue {
                float32: [0.0, 0.0, 0.0, 1.0],
            },
        }];
        let begin_info = RenderPassBeginInfoBuilder::new()
            .render_pass(render_pass)
            .framebuffer(framebuffer)
            .render_area(Rect2D {
                offset: Offset2D { x: 0, y: 0 },
                extent: surface_caps.current_extent,
            })
            .clear_values(&clear_values);

        unsafe {
            device.cmd_begin_render_pass(command_buffer, &begin_info, SubpassContents::INLINE);
            device.cmd_bind_pipeline(command_buffer, PipelineBindPoint::GRAPHICS, pipeline);
            device.cmd_draw(command_buffer, 3, 1, 0, 0);
            device.cmd_end_render_pass(command_buffer);

            device.end_command_buffer(command_buffer).unwrap();
        }
    }

    // https://vulkan-tutorial.com/en/Drawing_a_triangle/Drawing/Rendering_and_presentation
    let create_info = SemaphoreCreateInfoBuilder::new();
    let image_available_semaphores: Vec<_> = (0..FRAMES_IN_FLIGHT)
        .map(|_| unsafe { device.create_semaphore(&create_info, None, None) }.unwrap())
        .collect();
    let render_finished_semaphores: Vec<_> = (0..FRAMES_IN_FLIGHT)
        .map(|_| unsafe { device.create_semaphore(&create_info, None, None) }.unwrap())
        .collect();

    let create_info = FenceCreateInfoBuilder::new().flags(FenceCreateFlags::SIGNALED);
    let in_flight_fences: Vec<_> = (0..FRAMES_IN_FLIGHT)
        .map(|_| unsafe { device.create_fence(&create_info, None, None) }.unwrap())
        .collect();
    let mut images_in_flight: Vec<_> = swapchain_images.iter().map(|_| Fence::null()).collect();

    let mut frame = 0;
    event_loop.run(move |event, _, control_flow| match event {
        Event::NewEvents(StartCause::Init) => {
            *control_flow = ControlFlow::Poll;
        }
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            _ => (),
        },
        Event::DeviceEvent { event, .. } => match event {
            DeviceEvent::Key(KeyboardInput {
                virtual_keycode: Some(keycode),
                state,
                ..
            }) => match (keycode, state) {
                (VirtualKeyCode::Escape, ElementState::Released) => {
                    *control_flow = ControlFlow::Exit
                }
                _ => (),
            },
            _ => (),
        },
        Event::MainEventsCleared => {
            unsafe {
                device
                    .wait_for_fences(&[in_flight_fences[frame]], true, u64::MAX)
                    .unwrap();
            }

            let image_index = unsafe {
                device.acquire_next_image_khr(
                    swapchain,
                    u64::MAX,
                    image_available_semaphores[frame],
                    Fence::null(),
                    None,
                )
            }
            .unwrap();

            let image_in_flight = images_in_flight[image_index as usize];
            if !image_in_flight.is_null() {
                unsafe { device.wait_for_fences(&[image_in_flight], true, u64::MAX) }.unwrap();
            }
            images_in_flight[image_index as usize] = in_flight_fences[frame];

            let wait_semaphores = vec![image_available_semaphores[frame]];
            let command_buffers = vec![command_buffers[image_index as usize]];
            let signal_semaphores = vec![render_finished_semaphores[frame]];
            let submit_info = SubmitInfoBuilder::new()
                .wait_semaphores(&wait_semaphores)
                .wait_dst_stage_mask(&[PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT])
                .command_buffers(&command_buffers)
                .signal_semaphores(&signal_semaphores);
            unsafe {
                let in_flight_fence = in_flight_fences[frame];
                device.reset_fences(&[in_flight_fence]).unwrap();
                device
                    .queue_submit(queue, &[submit_info], in_flight_fence)
                    .unwrap()
            }

            let swapchains = vec![swapchain];
            let image_indices = vec![image_index];
            let present_info = PresentInfoKHRBuilder::new()
                .wait_semaphores(&signal_semaphores)
                .swapchains(&swapchains)
                .image_indices(&image_indices);

            unsafe { device.queue_present_khr(queue, &present_info) }.unwrap();

            frame = (frame + 1) % FRAMES_IN_FLIGHT;
        }
        Event::LoopDestroyed => unsafe {
            device.device_wait_idle().unwrap();

            for &semaphore in image_available_semaphores
                .iter()
                .chain(render_finished_semaphores.iter())
            {
                device.destroy_semaphore(semaphore, None);
            }

            for &fence in &in_flight_fences {
                device.destroy_fence(fence, None);
            }

            device.destroy_command_pool(command_pool, None);

            for &framebuffer in &swapchain_framebuffers {
                device.destroy_framebuffer(framebuffer, None);
            }

            device.destroy_pipeline(pipeline, None);

            device.destroy_render_pass(render_pass, None);

            device.destroy_pipeline_layout(pipeline_layout, None);

            device.destroy_shader_module(shader_vert, None);
            device.destroy_shader_module(shader_frag, None);

            for &image_view in &swapchain_image_views {
                device.destroy_image_view(image_view, None);
            }

            device.destroy_swapchain_khr(swapchain, None);

            device.destroy_device(None);

            instance.destroy_surface_khr(surface, None);

            if !messenger.is_null() {
                instance.destroy_debug_utils_messenger_ext(messenger, None);
            }

            instance.destroy_instance(None);
            println!("Exited cleanly");
        },
        _ => (),
    })
}
