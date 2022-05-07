use erupt::{vk, EntryLoader, ExtendableFrom, InstanceLoader};
use std::{ffi::CStr, sync::Arc};

const TITLE: &str = "erupt_examples: pointer_chain";

fn main() {
    let entry = Arc::new(EntryLoader::new().unwrap());
    println!(
        "{} - Vulkan Instance {}.{}.{}",
        TITLE,
        vk::api_version_major(entry.instance_version()),
        vk::api_version_minor(entry.instance_version()),
        vk::api_version_patch(entry.instance_version())
    );

    let info = vk::ApplicationInfoBuilder::new().api_version(vk::make_api_version(0, 1, 1, 0));
    let instance_info = vk::InstanceCreateInfoBuilder::new().application_info(&info);
    let instance = Arc::new(unsafe { InstanceLoader::new(&entry, &instance_info) }.unwrap());

    let physical_devices = unsafe { instance.enumerate_physical_devices(None) }.unwrap();
    for physical_device in physical_devices {
        let device_properties = unsafe { instance.get_physical_device_properties(physical_device) };
        println!("Physical device: {:?}", unsafe {
            CStr::from_ptr(device_properties.device_name.as_ptr())
        });

        let vk1_1 = device_properties.api_version >= vk::make_api_version(0, 1, 1, 0);
        let vk1_2 = device_properties.api_version >= vk::make_api_version(0, 1, 2, 0);
        let vk1_3 = device_properties.api_version >= vk::make_api_version(0, 1, 3, 0);

        if vk1_1 {
            let mut vk1_1features = vk::PhysicalDeviceVulkan11FeaturesBuilder::new();
            let mut vk1_2features = vk::PhysicalDeviceVulkan12FeaturesBuilder::new();
            let mut vk1_3features = vk::PhysicalDeviceVulkan13FeaturesBuilder::new();

            let mut device_features2 =
                vk::PhysicalDeviceFeatures2Builder::new().extend_from(&mut vk1_1features);

            if vk1_2 {
                device_features2 = device_features2.extend_from(&mut vk1_2features);
            }

            if vk1_3 {
                device_features2 = device_features2.extend_from(&mut vk1_3features);
            }

            unsafe {
                instance.get_physical_device_features2(physical_device, &mut device_features2)
            };

            println!("Vulkan 1.0 Features: {:?}", device_features2.features);
            println!("Vulkan 1.1 Features: {:?}", vk1_1features);
            if vk1_2 {
                println!("Vulkan 1.2 Features: {:?}", vk1_2features);
            } else {
                println!("Vulkan 1.2 is not supported");
            }

            if vk1_3 {
                println!("Vulkan 1.3 Features: {:?}", vk1_3features);
            } else {
                println!("Vulkan 1.3 is not supported");
            }
        } else {
            println!("Vulkan 1.1 required");
        }

        println!();
    }

    unsafe {
        instance.destroy_instance(None);
    }
}
