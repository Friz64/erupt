use erupt::{vk, CustomEntryLoader, DeviceLoaderBuilder, InstanceLoaderBuilder};
use libloading::Library;
use std::{ffi::CStr, sync::Arc};

const TITLE: &str = "erupt_examples: custom_loaders";

#[cfg(all(
    unix,
    not(any(target_os = "macos", target_os = "ios", target_os = "android"))
))]
const LIB_PATH: &str = "libvulkan.so.1";

#[cfg(target_os = "android")]
const LIB_PATH: &str = "libvulkan.so";

#[cfg(any(target_os = "macos", target_os = "ios"))]
const LIB_PATH: &str = "libvulkan.dylib";

#[cfg(windows)]
const LIB_PATH: &str = "vulkan-1.dll";

fn main() {
    let library = unsafe { Library::new(LIB_PATH).expect("failed to load vulkan library") };
    let entry = Arc::new(unsafe {
        CustomEntryLoader::with_library(library, |library, name| {
            let cstr = CStr::from_ptr(name);
            let bytes = cstr.to_bytes_with_nul();
            library.get(bytes).ok().map(|symbol| *symbol)
        })
        .expect("failed to create entry loader")
    });

    println!(
        "{} - Vulkan Instance {}.{}.{}",
        TITLE,
        vk::api_version_major(entry.instance_version()),
        vk::api_version_minor(entry.instance_version()),
        vk::api_version_patch(entry.instance_version())
    );

    let app_info = vk::ApplicationInfoBuilder::new().api_version(vk::make_api_version(0, 1, 0, 0));
    let instance_info = vk::InstanceCreateInfoBuilder::new().application_info(&app_info);

    let instance = Arc::new(unsafe {
        InstanceLoaderBuilder::new()
            .create_instance_fn(Box::new({
                let entry = entry.clone(); // make use of the Arc
                move |create_info, allocator| entry.create_instance(create_info, allocator)
            }))
            .symbol_fn(&mut |instance, name| (entry.get_instance_proc_addr)(instance, name))
            .build(&entry, &instance_info)
            .expect("failed to create instance")
    });

    let physical_device = unsafe { instance.enumerate_physical_devices(None) }
        .expect("failed to enumerate physical devices")[0];

    let device_info = vk::DeviceCreateInfoBuilder::new();
    let device = Arc::new(unsafe {
        DeviceLoaderBuilder::new()
            .create_device_fn(Box::new({
                let instance = instance.clone(); // make use of the Arc
                move |physical_device, create_info, allocator| {
                    instance.create_device(physical_device, create_info, allocator)
                }
            }))
            .symbol_fn(&mut |device, name| (instance.get_device_proc_addr)(device, name))
            .build(&instance, physical_device, &device_info)
            .expect("failed to create device")
    });

    unsafe {
        device.destroy_device(None);
        instance.destroy_instance(None);
    }
}
