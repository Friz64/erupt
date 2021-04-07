use erupt::{vk, EntryLoader, ExtendableFromMut, InstanceLoader};
use std::ffi::CStr;

const TITLE: &str = "erupt_examples: pointer_chain";

fn main() {
    let entry = EntryLoader::new().unwrap();
    println!(
        "{} - Vulkan Instance {}.{}.{}",
        TITLE,
        vk::version_major(entry.instance_version()),
        vk::version_minor(entry.instance_version()),
        vk::version_patch(entry.instance_version())
    );

    let info = vk::ApplicationInfoBuilder::new().api_version(vk::make_version(1, 1, 0));
    let instance_info = vk::InstanceCreateInfoBuilder::new().application_info(&info);
    let instance = unsafe { InstanceLoader::new(&entry, &instance_info, None) }.unwrap();

    let physical_devices = unsafe { instance.enumerate_physical_devices(None) }.unwrap();
    for physical_device in physical_devices {
        let device_properties = unsafe { instance.get_physical_device_properties(physical_device) };
        println!("Physical device: {:?}", unsafe {
            CStr::from_ptr(device_properties.device_name.as_ptr())
        });

        let vk1_1 = device_properties.api_version >= vk::make_version(1, 1, 0);
        let vk1_2 = device_properties.api_version >= vk::make_version(1, 2, 0);

        if vk1_1 {
            let mut vk1_1features = vk::PhysicalDeviceVulkan11FeaturesBuilder::new();
            let mut vk1_2features = vk::PhysicalDeviceVulkan12FeaturesBuilder::new();

            let mut device_features2_builder =
                vk::PhysicalDeviceFeatures2Builder::new().extend_from(&mut vk1_1features);

            if vk1_2 {
                device_features2_builder = device_features2_builder.extend_from(&mut vk1_2features);
            }

            let device_features2 = unsafe {
                instance.get_physical_device_features2(
                    physical_device,
                    Some(device_features2_builder.build()),
                )
            };

            println!("Vulkan 1.0 Features: {:?}", device_features2.features);
            println!("Vulkan 1.1 Features: {:?}", vk1_1features);
            if vk1_2 {
                println!("Vulkan 1.2 Features: {:?}", vk1_2features);
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
