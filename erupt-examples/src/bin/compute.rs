use erupt::{
    cstr,
    extensions::ext_debug_utils::*,
    utils::{
        self,
        allocator::{Allocator, AllocatorCreateInfo, MemoryTypeFinder},
    },
    vk1_0::*,
    CoreLoader, DeviceLoader, InstanceLoader,
};
use std::{
    convert::TryInto,
    ffi::{c_void, CStr, CString},
    mem::{self, MaybeUninit},
    os::raw::c_char,
    ptr, slice,
};
use structopt::StructOpt;

const LAYER_KHRONOS_VALIDATION: *const c_char = cstr!("VK_LAYER_KHRONOS_validation");
const SHADER: &[u8] = include_bytes!("compute.comp.spv");

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

#[derive(Debug, StructOpt)]
struct Opt {
    /// Use validation layers
    #[structopt(short, long)]
    validation_layers: bool,
}

#[derive(Debug)]
#[repr(C)]
struct Buffer {
    data: [f32; 32],
}

impl Buffer {
    const SIZE: usize = mem::size_of::<Self>();

    fn from_bytes(bytes: &[u8]) -> Buffer {
        assert_eq!(bytes.len(), Self::SIZE);
        let mut buf = MaybeUninit::<Buffer>::uninit();
        unsafe {
            ptr::copy_nonoverlapping(bytes.as_ptr(), buf.as_mut_ptr() as *mut u8, Self::SIZE);
            buf.assume_init()
        }
    }

    fn as_bytes(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self as *const Self as *const u8, Self::SIZE) }
    }
}

fn main() {
    let opt = Opt::from_args();

    let mut core = CoreLoader::new().unwrap();
    core.load_vk1_0().unwrap();

    let api_version = core.instance_version();
    println!(
        "erupt-examples: compute - Vulkan {}.{}.{}",
        erupt::version_major(api_version),
        erupt::version_minor(api_version),
        erupt::version_patch(api_version)
    );

    let application_name = CString::new("compute").unwrap();
    let engine_name = CString::new("erupt").unwrap();
    let app_info = ApplicationInfoBuilder::new()
        .application_name(&application_name)
        .application_version(erupt::make_version(1, 0, 0))
        .engine_name(&engine_name)
        .engine_version(erupt::make_version(1, 0, 0))
        .api_version(erupt::make_version(1, 1, 0));

    let mut instance_extensions = Vec::new();
    let mut instance_layers = Vec::new();
    let mut device_layers = Vec::new();
    if opt.validation_layers {
        instance_extensions.push(EXT_DEBUG_UTILS_EXTENSION_NAME);
        instance_layers.push(LAYER_KHRONOS_VALIDATION);
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

    let (physical_device, queue_family, properties) =
        unsafe { instance.enumerate_physical_devices(None) }
            .unwrap()
            .into_iter()
            .filter_map(|physical_device| unsafe {
                let queue_family = match instance
                    .get_physical_device_queue_family_properties(physical_device, None)
                    .into_iter()
                    .position(|properties| properties.queue_flags.contains(QueueFlags::COMPUTE))
                {
                    Some(queue_family) => queue_family as u32,
                    None => return None,
                };

                let properties = instance.get_physical_device_properties(physical_device, None);
                Some((physical_device, queue_family, properties))
            })
            .max_by_key(|(_, _, properties)| match properties.device_type {
                PhysicalDeviceType::DISCRETE_GPU => 2,
                PhysicalDeviceType::INTEGRATED_GPU => 1,
                _ => 0,
            })
            .expect("No suitable physical device found");

    println!("Using physical device: {:?}", unsafe {
        CStr::from_ptr(properties.device_name.as_ptr())
    });

    let queue_create_info = vec![DeviceQueueCreateInfoBuilder::new()
        .queue_family_index(queue_family)
        .queue_priorities(&[1.0])];
    let features = PhysicalDeviceFeaturesBuilder::new();

    let create_info = DeviceCreateInfoBuilder::new()
        .queue_create_infos(&queue_create_info)
        .enabled_features(&features)
        .enabled_layer_names(&device_layers);

    let mut device = DeviceLoader::new(
        &instance,
        unsafe { instance.create_device(physical_device, &create_info, None, None) }.unwrap(),
    )
    .unwrap();
    device.load_vk1_0().unwrap();

    let queue = unsafe { device.get_device_queue(queue_family, 0, None) };

    let mut allocator =
        Allocator::new(&instance, physical_device, AllocatorCreateInfo::default()).unwrap();

    let data = Buffer {
        data: (0..32)
            .map(|i| i as f32)
            .collect::<Vec<_>>()
            .as_slice()
            .try_into()
            .unwrap(),
    };
    let data_size = mem::size_of_val(&data) as DeviceSize;

    let create_info = BufferCreateInfoBuilder::new()
        .sharing_mode(SharingMode::EXCLUSIVE)
        .usage(BufferUsageFlags::STORAGE_BUFFER)
        .size(data_size);

    let buffer = allocator
        .allocate(
            &device,
            unsafe { device.create_buffer(&create_info, None, None) }.unwrap(),
            MemoryTypeFinder::dynamic(),
        )
        .unwrap();
    let mut map = buffer.map(&device, ..data_size).unwrap();
    map.import(data.as_bytes());
    map.unmap(&device).unwrap();

    let desc_pool_sizes = &[DescriptorPoolSizeBuilder::new()
        .descriptor_count(1)
        ._type(DescriptorType::STORAGE_BUFFER)];
    let desc_pool_info = DescriptorPoolCreateInfoBuilder::new()
        .pool_sizes(desc_pool_sizes)
        .max_sets(1);
    let desc_pool = unsafe { device.create_descriptor_pool(&desc_pool_info, None, None) }.unwrap();

    let desc_layout_bindings = &[DescriptorSetLayoutBindingBuilder::new()
        .binding(0)
        .descriptor_count(1)
        .descriptor_type(DescriptorType::STORAGE_BUFFER)
        .stage_flags(ShaderStageFlags::COMPUTE)];
    let desc_layout_info =
        DescriptorSetLayoutCreateInfoBuilder::new().bindings(desc_layout_bindings);
    let desc_layout =
        unsafe { device.create_descriptor_set_layout(&desc_layout_info, None, None) }.unwrap();

    let desc_layouts = &[desc_layout];
    let desc_info = DescriptorSetAllocateInfoBuilder::new()
        .descriptor_pool(desc_pool)
        .set_layouts(desc_layouts);
    let desc_set = unsafe { device.allocate_descriptor_sets(&desc_info) }.unwrap()[0];

    unsafe {
        device.update_descriptor_sets(
            &[WriteDescriptorSetBuilder::new()
                .dst_set(desc_set)
                .dst_binding(0)
                .descriptor_type(DescriptorType::STORAGE_BUFFER)
                .buffer_info(&[DescriptorBufferInfoBuilder::new()
                    .buffer(*buffer.object())
                    .offset(buffer.region().start)
                    .range(data_size)])],
            &[],
        )
    };

    let pipeline_layout_desc_layouts = &[desc_layout];
    let pipeline_layout_info =
        PipelineLayoutCreateInfoBuilder::new().set_layouts(pipeline_layout_desc_layouts);
    let pipeline_layout =
        unsafe { device.create_pipeline_layout(&pipeline_layout_info, None, None) }.unwrap();

    let spv_code = utils::decode_spv(SHADER).unwrap();
    let create_info = ShaderModuleCreateInfoBuilder::new().code(&spv_code);
    let shader_mod = unsafe { device.create_shader_module(&create_info, None, None) }.unwrap();

    let entry_point = CString::new("main").unwrap();
    let shader_stage = PipelineShaderStageCreateInfoBuilder::new()
        .stage(ShaderStageFlagBits::COMPUTE)
        .module(shader_mod)
        .name(&entry_point);

    let pipeline_info = &[ComputePipelineCreateInfoBuilder::new()
        .layout(pipeline_layout)
        .stage(unsafe { shader_stage.discard() })];
    let pipeline =
        unsafe { device.create_compute_pipelines(PipelineCache::null(), pipeline_info, None) }
            .unwrap()[0];

    let cmd_pool_info = CommandPoolCreateInfoBuilder::new().queue_family_index(queue_family);
    let cmd_pool = unsafe { device.create_command_pool(&cmd_pool_info, None, None) }.unwrap();

    let cmd_buf_info = CommandBufferAllocateInfoBuilder::new()
        .command_pool(cmd_pool)
        .command_buffer_count(1)
        .level(CommandBufferLevel::PRIMARY);
    let cmd_buf = unsafe { device.allocate_command_buffers(&cmd_buf_info) }.unwrap()[0];

    let begin_info =
        CommandBufferBeginInfoBuilder::new().flags(CommandBufferUsageFlags::ONE_TIME_SUBMIT);
    unsafe {
        device.begin_command_buffer(cmd_buf, &begin_info).unwrap();
        device.cmd_bind_pipeline(cmd_buf, PipelineBindPoint::COMPUTE, pipeline);
        device.cmd_bind_descriptor_sets(
            cmd_buf,
            PipelineBindPoint::COMPUTE,
            pipeline_layout,
            0,
            &[desc_set],
            &[],
        );
        device.cmd_dispatch(cmd_buf, 1, 1, 1);
        device.end_command_buffer(cmd_buf).unwrap();
    }

    let fence_info = FenceCreateInfoBuilder::new();
    let fence = unsafe { device.create_fence(&fence_info, None, None) }.unwrap();

    let cmd_bufs = &[cmd_buf];
    let submit_info = &[SubmitInfoBuilder::new().command_buffers(cmd_bufs)];
    unsafe {
        device.queue_submit(queue, submit_info, fence).unwrap();
        device.wait_for_fences(&[fence], true, u64::MAX).unwrap();
    }

    let map = buffer.map(&device, ..data_size).unwrap();
    println!("input: {:?}", data);
    println!("output: {:?}", Buffer::from_bytes(map.read()));
    map.unmap(&device).unwrap();

    // Destruction
    unsafe {
        device.device_wait_idle().unwrap();

        allocator.free(&device, buffer);
        device.destroy_pipeline(pipeline, None);
        device.destroy_pipeline_layout(pipeline_layout, None);
        device.destroy_command_pool(cmd_pool, None);
        device.destroy_fence(fence, None);
        device.destroy_descriptor_set_layout(desc_layout, None);
        device.destroy_descriptor_pool(desc_pool, None);
        device.destroy_shader_module(shader_mod, None);
        device.destroy_device(None);

        if !messenger.is_null() {
            instance.destroy_debug_utils_messenger_ext(messenger, None);
        }

        instance.destroy_instance(None);
    }

    println!("Exited cleanly");
}
