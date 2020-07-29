// This code roughly follows https://vulkan-tutorial.com/ - Drawing a triangle

use erupt::{
    cstr,
    extensions::{ext_debug_utils, khr_surface, khr_swapchain},
    utils::{self, surface},
    vk1_0, DeviceLoader, EntryLoader, InstanceLoader,
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
    _message_severity: ext_debug_utils::DebugUtilsMessageSeverityFlagBitsEXT,
    _message_types: ext_debug_utils::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const ext_debug_utils::DebugUtilsMessengerCallbackDataEXT,
    _p_user_data: *mut c_void,
) -> vk1_0::Bool32 {
    eprintln!(
        "{}",
        CStr::from_ptr((*p_callback_data).p_message).to_string_lossy()
    );

    vk1_0::FALSE
}

fn main() {
    let opt = Opt::from_args();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(TITLE)
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();

    let entry = EntryLoader::new().unwrap();
    println!(
        "{} - Vulkan Instance {}.{}.{}",
        TITLE,
        vk1_0::version_major(entry.instance_version()),
        vk1_0::version_minor(entry.instance_version()),
        vk1_0::version_patch(entry.instance_version())
    );

    // https://vulkan-tutorial.com/Drawing_a_triangle/Setup/Instance
    let application_name = CString::new("Hello Triangle").unwrap();
    let engine_name = CString::new("No Engine").unwrap();
    let app_info = vk1_0::ApplicationInfoBuilder::new()
        .application_name(&application_name)
        .application_version(vk1_0::make_version(1, 0, 0))
        .engine_name(&engine_name)
        .engine_version(vk1_0::make_version(1, 0, 0))
        .api_version(vk1_0::make_version(1, 0, 0));

    let mut instance_extensions = surface::enumerate_required_extensions(&window).unwrap();
    if opt.validation_layers {
        instance_extensions.push(ext_debug_utils::EXT_DEBUG_UTILS_EXTENSION_NAME);
    }

    let mut instance_layers = Vec::new();
    if opt.validation_layers {
        instance_layers.push(LAYER_KHRONOS_VALIDATION);
    }

    let device_extensions = vec![khr_swapchain::KHR_SWAPCHAIN_EXTENSION_NAME];

    let mut device_layers = Vec::new();
    if opt.validation_layers {
        device_layers.push(LAYER_KHRONOS_VALIDATION);
    }

    let create_info = vk1_0::InstanceCreateInfoBuilder::new()
        .application_info(&app_info)
        .enabled_extension_names(&instance_extensions)
        .enabled_layer_names(&instance_layers);

    let mut instance = InstanceLoader::new(&entry, &create_info, None).unwrap();

    // https://vulkan-tutorial.com/Drawing_a_triangle/Setup/Validation_layers
    let messenger = if opt.validation_layers {
        let create_info = ext_debug_utils::DebugUtilsMessengerCreateInfoEXTBuilder::new()
            .message_severity(
                ext_debug_utils::DebugUtilsMessageSeverityFlagsEXT::VERBOSE_EXT
                    | ext_debug_utils::DebugUtilsMessageSeverityFlagsEXT::WARNING_EXT
                    | ext_debug_utils::DebugUtilsMessageSeverityFlagsEXT::ERROR_EXT,
            )
            .message_type(
                ext_debug_utils::DebugUtilsMessageTypeFlagsEXT::GENERAL_EXT
                    | ext_debug_utils::DebugUtilsMessageTypeFlagsEXT::VALIDATION_EXT
                    | ext_debug_utils::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE_EXT,
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
                        properties.queue_flags.contains(vk1_0::QueueFlags::GRAPHICS)
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
                        surface_format.format == vk1_0::Format::B8G8R8A8_SRGB
                            && surface_format.color_space
                                == khr_surface::ColorSpaceKHR::SRGB_NONLINEAR_KHR
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
                    .find(|present_mode| present_mode == &khr_surface::PresentModeKHR::MAILBOX_KHR)
                    .unwrap_or(khr_surface::PresentModeKHR::FIFO_KHR);

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
                vk1_0::PhysicalDeviceType::DISCRETE_GPU => 2,
                vk1_0::PhysicalDeviceType::INTEGRATED_GPU => 1,
                _ => 0,
            })
            .expect("No suitable physical device found");

    println!("Using physical device: {:?}", unsafe {
        CStr::from_ptr(properties.device_name.as_ptr())
    });

    // https://vulkan-tutorial.com/Drawing_a_triangle/Setup/Logical_device_and_queues
    let queue_create_info = vec![vk1_0::DeviceQueueCreateInfoBuilder::new()
        .queue_family_index(queue_family)
        .queue_priorities(&[1.0])];
    let features = vk1_0::PhysicalDeviceFeaturesBuilder::new();

    let create_info = vk1_0::DeviceCreateInfoBuilder::new()
        .queue_create_infos(&queue_create_info)
        .enabled_features(&features)
        .enabled_extension_names(&device_extensions)
        .enabled_layer_names(&device_layers);

    let device = DeviceLoader::new(&instance, physical_device, &create_info, None).unwrap();
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

    let create_info = khr_swapchain::SwapchainCreateInfoKHRBuilder::new()
        .surface(surface)
        .min_image_count(image_count)
        .image_format(format.format)
        .image_color_space(format.color_space)
        .image_extent(surface_caps.current_extent)
        .image_array_layers(1)
        .image_usage(vk1_0::ImageUsageFlags::COLOR_ATTACHMENT)
        .image_sharing_mode(vk1_0::SharingMode::EXCLUSIVE)
        .pre_transform(surface_caps.current_transform)
        .composite_alpha(khr_surface::CompositeAlphaFlagBitsKHR::OPAQUE_KHR)
        .present_mode(present_mode)
        .clipped(true)
        .old_swapchain(khr_swapchain::SwapchainKHR::null());

    let swapchain = unsafe { device.create_swapchain_khr(&create_info, None, None) }.unwrap();
    let swapchain_images = unsafe { device.get_swapchain_images_khr(swapchain, None) }.unwrap();

    // https://vulkan-tutorial.com/Drawing_a_triangle/Presentation/Image_views
    let swapchain_image_views: Vec<_> = swapchain_images
        .iter()
        .map(|swapchain_image| {
            let create_info = vk1_0::ImageViewCreateInfoBuilder::new()
                .image(*swapchain_image)
                .view_type(vk1_0::ImageViewType::_2D)
                .format(format.format)
                .components(vk1_0::ComponentMapping {
                    r: vk1_0::ComponentSwizzle::IDENTITY,
                    g: vk1_0::ComponentSwizzle::IDENTITY,
                    b: vk1_0::ComponentSwizzle::IDENTITY,
                    a: vk1_0::ComponentSwizzle::IDENTITY,
                })
                .subresource_range(
                    vk1_0::ImageSubresourceRangeBuilder::new()
                        .aspect_mask(vk1_0::ImageAspectFlags::COLOR)
                        .base_mip_level(0)
                        .level_count(1)
                        .base_array_layer(0)
                        .layer_count(1)
                        .build(),
                );
            unsafe { device.create_image_view(&create_info, None, None) }.unwrap()
        })
        .collect();

    // https://vulkan-tutorial.com/Drawing_a_triangle/Graphics_pipeline_basics/Shader_modules
    let entry_point = CString::new("main").unwrap();

    let vert_decoded = utils::decode_spv(SHADER_VERT).unwrap();
    let create_info = vk1_0::ShaderModuleCreateInfoBuilder::new().code(&vert_decoded);
    let shader_vert = unsafe { device.create_shader_module(&create_info, None, None) }.unwrap();

    let frag_decoded = utils::decode_spv(SHADER_FRAG).unwrap();
    let create_info = vk1_0::ShaderModuleCreateInfoBuilder::new().code(&frag_decoded);
    let shader_frag = unsafe { device.create_shader_module(&create_info, None, None) }.unwrap();

    let shader_stages = vec![
        vk1_0::PipelineShaderStageCreateInfoBuilder::new()
            .stage(vk1_0::ShaderStageFlagBits::VERTEX)
            .module(shader_vert)
            .name(&entry_point),
        vk1_0::PipelineShaderStageCreateInfoBuilder::new()
            .stage(vk1_0::ShaderStageFlagBits::FRAGMENT)
            .module(shader_frag)
            .name(&entry_point),
    ];

    // https://vulkan-tutorial.com/Drawing_a_triangle/Graphics_pipeline_basics/Fixed_functions
    let vertex_input = vk1_0::PipelineVertexInputStateCreateInfoBuilder::new();

    let input_assembly = vk1_0::PipelineInputAssemblyStateCreateInfoBuilder::new()
        .topology(vk1_0::PrimitiveTopology::TRIANGLE_LIST)
        .primitive_restart_enable(false);

    let viewports = vec![vk1_0::ViewportBuilder::new()
        .x(0.0)
        .y(0.0)
        .width(surface_caps.current_extent.width as f32)
        .height(surface_caps.current_extent.height as f32)
        .min_depth(0.0)
        .max_depth(1.0)];
    let scissors = vec![vk1_0::Rect2DBuilder::new()
        .offset(vk1_0::Offset2D { x: 0, y: 0 })
        .extent(surface_caps.current_extent)];
    let viewport_state = vk1_0::PipelineViewportStateCreateInfoBuilder::new()
        .viewports(&viewports)
        .scissors(&scissors);

    let rasterizer = vk1_0::PipelineRasterizationStateCreateInfoBuilder::new()
        .depth_clamp_enable(false)
        .rasterizer_discard_enable(false)
        .polygon_mode(vk1_0::PolygonMode::FILL)
        .line_width(1.0)
        .cull_mode(vk1_0::CullModeFlags::BACK)
        .front_face(vk1_0::FrontFace::CLOCKWISE)
        .depth_clamp_enable(false);

    let multisampling = vk1_0::PipelineMultisampleStateCreateInfoBuilder::new()
        .sample_shading_enable(false)
        .rasterization_samples(vk1_0::SampleCountFlagBits::_1);

    let color_blend_attachments = vec![vk1_0::PipelineColorBlendAttachmentStateBuilder::new()
        .color_write_mask(
            vk1_0::ColorComponentFlags::R
                | vk1_0::ColorComponentFlags::G
                | vk1_0::ColorComponentFlags::B
                | vk1_0::ColorComponentFlags::A,
        )
        .blend_enable(false)];
    let color_blending = vk1_0::PipelineColorBlendStateCreateInfoBuilder::new()
        .logic_op_enable(false)
        .attachments(&color_blend_attachments);

    let create_info = vk1_0::PipelineLayoutCreateInfoBuilder::new();
    let pipeline_layout =
        unsafe { device.create_pipeline_layout(&create_info, None, None) }.unwrap();

    // https://vulkan-tutorial.com/Drawing_a_triangle/Graphics_pipeline_basics/Render_passes
    let attachments = vec![vk1_0::AttachmentDescriptionBuilder::new()
        .format(format.format)
        .samples(vk1_0::SampleCountFlagBits::_1)
        .load_op(vk1_0::AttachmentLoadOp::CLEAR)
        .store_op(vk1_0::AttachmentStoreOp::STORE)
        .stencil_load_op(vk1_0::AttachmentLoadOp::DONT_CARE)
        .stencil_store_op(vk1_0::AttachmentStoreOp::DONT_CARE)
        .initial_layout(vk1_0::ImageLayout::UNDEFINED)
        .final_layout(vk1_0::ImageLayout::PRESENT_SRC_KHR)];

    let color_attachment_refs = vec![vk1_0::AttachmentReferenceBuilder::new()
        .attachment(0)
        .layout(vk1_0::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)];
    let subpasses = vec![vk1_0::SubpassDescriptionBuilder::new()
        .pipeline_bind_point(vk1_0::PipelineBindPoint::GRAPHICS)
        .color_attachments(&color_attachment_refs)];
    let dependencies = vec![vk1_0::SubpassDependencyBuilder::new()
        .src_subpass(vk1_0::SUBPASS_EXTERNAL)
        .dst_subpass(0)
        .src_stage_mask(vk1_0::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
        .src_access_mask(vk1_0::AccessFlags::empty())
        .dst_stage_mask(vk1_0::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
        .dst_access_mask(vk1_0::AccessFlags::COLOR_ATTACHMENT_WRITE)];

    let create_info = vk1_0::RenderPassCreateInfoBuilder::new()
        .attachments(&attachments)
        .subpasses(&subpasses)
        .dependencies(&dependencies);

    let render_pass = unsafe { device.create_render_pass(&create_info, None, None) }.unwrap();

    // https://vulkan-tutorial.com/Drawing_a_triangle/Graphics_pipeline_basics/Conclusion
    let create_info = vk1_0::GraphicsPipelineCreateInfoBuilder::new()
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
        unsafe { device.create_graphics_pipelines(None, &[create_info], None) }.unwrap()[0];

    // https://vulkan-tutorial.com/Drawing_a_triangle/Drawing/Framebuffers
    let swapchain_framebuffers: Vec<_> = swapchain_image_views
        .iter()
        .map(|image_view| {
            let attachments = vec![*image_view];
            let create_info = vk1_0::FramebufferCreateInfoBuilder::new()
                .render_pass(render_pass)
                .attachments(&attachments)
                .width(surface_caps.current_extent.width)
                .height(surface_caps.current_extent.height)
                .layers(1);

            unsafe { device.create_framebuffer(&create_info, None, None) }.unwrap()
        })
        .collect();

    // https://vulkan-tutorial.com/Drawing_a_triangle/Drawing/Command_buffers
    let create_info = vk1_0::CommandPoolCreateInfoBuilder::new().queue_family_index(queue_family);
    let command_pool = unsafe { device.create_command_pool(&create_info, None, None) }.unwrap();

    let allocate_info = vk1_0::CommandBufferAllocateInfoBuilder::new()
        .command_pool(command_pool)
        .level(vk1_0::CommandBufferLevel::PRIMARY)
        .command_buffer_count(swapchain_framebuffers.len() as _);
    let command_buffers = unsafe { device.allocate_command_buffers(&allocate_info) }.unwrap();

    for (&command_buffer, &framebuffer) in command_buffers.iter().zip(swapchain_framebuffers.iter())
    {
        let begin_info = vk1_0::CommandBufferBeginInfoBuilder::new();
        unsafe { device.begin_command_buffer(command_buffer, &begin_info) }.unwrap();

        let clear_values = vec![vk1_0::ClearValue {
            color: vk1_0::ClearColorValue {
                float32: [0.0, 0.0, 0.0, 1.0],
            },
        }];
        let begin_info = vk1_0::RenderPassBeginInfoBuilder::new()
            .render_pass(render_pass)
            .framebuffer(framebuffer)
            .render_area(vk1_0::Rect2D {
                offset: vk1_0::Offset2D { x: 0, y: 0 },
                extent: surface_caps.current_extent,
            })
            .clear_values(&clear_values);

        unsafe {
            device.cmd_begin_render_pass(
                command_buffer,
                &begin_info,
                vk1_0::SubpassContents::INLINE,
            );

            device.cmd_bind_pipeline(command_buffer, vk1_0::PipelineBindPoint::GRAPHICS, pipeline);
            device.cmd_draw(command_buffer, 3, 1, 0, 0);
            device.cmd_end_render_pass(command_buffer);

            device.end_command_buffer(command_buffer).unwrap();
        }
    }

    // https://vulkan-tutorial.com/en/Drawing_a_triangle/Drawing/Rendering_and_presentation
    let create_info = vk1_0::SemaphoreCreateInfoBuilder::new();
    let image_available_semaphores: Vec<_> = (0..FRAMES_IN_FLIGHT)
        .map(|_| unsafe { device.create_semaphore(&create_info, None, None) }.unwrap())
        .collect();
    let render_finished_semaphores: Vec<_> = (0..FRAMES_IN_FLIGHT)
        .map(|_| unsafe { device.create_semaphore(&create_info, None, None) }.unwrap())
        .collect();

    let create_info = vk1_0::FenceCreateInfoBuilder::new().flags(vk1_0::FenceCreateFlags::SIGNALED);
    let in_flight_fences: Vec<_> = (0..FRAMES_IN_FLIGHT)
        .map(|_| unsafe { device.create_fence(&create_info, None, None) }.unwrap())
        .collect();
    let mut images_in_flight: Vec<_> = swapchain_images
        .iter()
        .map(|_| vk1_0::Fence::null())
        .collect();

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
                    Some(image_available_semaphores[frame]),
                    None,
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
            let submit_info = vk1_0::SubmitInfoBuilder::new()
                .wait_semaphores(&wait_semaphores)
                .wait_dst_stage_mask(&[vk1_0::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT])
                .command_buffers(&command_buffers)
                .signal_semaphores(&signal_semaphores);
            unsafe {
                let in_flight_fence = in_flight_fences[frame];
                device.reset_fences(&[in_flight_fence]).unwrap();
                device
                    .queue_submit(queue, &[submit_info], Some(in_flight_fence))
                    .unwrap()
            }

            let swapchains = vec![swapchain];
            let image_indices = vec![image_index];
            let present_info = khr_swapchain::PresentInfoKHRBuilder::new()
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
                device.destroy_semaphore(Some(semaphore), None);
            }

            for &fence in &in_flight_fences {
                device.destroy_fence(Some(fence), None);
            }

            device.destroy_command_pool(Some(command_pool), None);

            for &framebuffer in &swapchain_framebuffers {
                device.destroy_framebuffer(Some(framebuffer), None);
            }

            device.destroy_pipeline(Some(pipeline), None);

            device.destroy_render_pass(Some(render_pass), None);

            device.destroy_pipeline_layout(Some(pipeline_layout), None);

            device.destroy_shader_module(Some(shader_vert), None);
            device.destroy_shader_module(Some(shader_frag), None);

            for &image_view in &swapchain_image_views {
                device.destroy_image_view(Some(image_view), None);
            }

            device.destroy_swapchain_khr(Some(swapchain), None);

            device.destroy_device(None);

            instance.destroy_surface_khr(Some(surface), None);

            if !messenger.is_null() {
                instance.destroy_debug_utils_messenger_ext(Some(messenger), None);
            }

            instance.destroy_instance(None);
            println!("Exited cleanly");
        },
        _ => (),
    })
}
