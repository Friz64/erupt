use erupt::{
    cstr,
    extensions::ext_debug_utils::*,
    utils::allocator::{Allocator, AllocatorCreateInfo, MemoryTypeFinder},
    vk1_0::*,
    CoreLoader, DeviceLoader, InstanceLoader,
};
use std::{
    ffi::{c_void, CStr, CString},
    os::raw::c_char,
};
use structopt::StructOpt;

const LAYER_KHRONOS_VALIDATION: *const c_char = cstr!("VK_LAYER_KHRONOS_validation");

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

fn main() {
    let opt = Opt::from_args();

    let mut core = CoreLoader::new().unwrap();
    core.load_vk1_0().unwrap();

    let api_version = core.instance_version();
    println!(
        "erupt-examples: allocator - Vulkan {}.{}.{}",
        erupt::version_major(api_version),
        erupt::version_minor(api_version),
        erupt::version_patch(api_version)
    );

    let application_name = CString::new("allocator").unwrap();
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

    let create_info = BufferCreateInfoBuilder::new()
        .sharing_mode(SharingMode::EXCLUSIVE)
        .usage(BufferUsageFlags::UNIFORM_BUFFER)
        .size(17);

    let buffer = allocator
        .allocate(
            &device,
            unsafe { device.create_buffer(&create_info, None, None) }.unwrap(),
            MemoryTypeFinder::dynamic(),
        )
        .unwrap();
    let map = buffer.map(&device, ..);
    println!("{:?}", map);

    // Destruction
    unsafe {
        allocator.free(&device, buffer);

        device.device_wait_idle().unwrap();

        device.destroy_device(None);

        if !messenger.is_null() {
            instance.destroy_debug_utils_messenger_ext(messenger, None);
        }

        instance.destroy_instance(None);
    }

    println!("Exited cleanly");
}
