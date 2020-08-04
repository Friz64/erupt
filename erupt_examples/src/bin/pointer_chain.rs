use erupt::{vk1_0, vk1_1, vk1_2, EntryLoader, ExtendableFrom, InstanceLoader};
use std::ffi::CStr;

const TITLE: &str = "erupt_examples: pointer_chain";

fn main() {
    let entry = EntryLoader::new().unwrap();
    println!(
        "{} - Vulkan Instance {}.{}.{}",
        TITLE,
        vk1_0::version_major(entry.instance_version()),
        vk1_0::version_minor(entry.instance_version()),
        vk1_0::version_patch(entry.instance_version())
    );

    let info = vk1_0::ApplicationInfoBuilder::new().api_version(vk1_0::make_version(1, 1, 0));
    let instance_info = vk1_0::InstanceCreateInfoBuilder::new().application_info(&info);
    let instance = InstanceLoader::new(&entry, &instance_info, None).unwrap();

    let physical_devices = unsafe { instance.enumerate_physical_devices(None) }.unwrap();
    for physical_device in physical_devices {
        let properties = unsafe { instance.get_physical_device_properties(physical_device, None) };
        println!("Physical device: {:?}", unsafe {
            CStr::from_ptr(properties.device_name.as_ptr())
        });

        let vk1_1 = properties.api_version >= vk1_0::make_version(1, 1, 0);
        let vk1_2 = properties.api_version >= vk1_0::make_version(1, 2, 0);

        if vk1_1 {
            let mut vk1_1features = vk1_2::PhysicalDeviceVulkan11FeaturesBuilder::new();
            let mut vk1_2features = vk1_2::PhysicalDeviceVulkan12FeaturesBuilder::new();

            let mut features2 =
                vk1_1::PhysicalDeviceFeatures2Builder::new().extend_from(&mut vk1_1features);

            if vk1_2 {
                features2 = features2.extend_from(&mut vk1_2features);
            }

            let features2 = unsafe {
                instance.get_physical_device_features2(physical_device, Some(features2.build()))
            };

            println!("Vulkan 1.0 Features: {:?}", features2.features);
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
