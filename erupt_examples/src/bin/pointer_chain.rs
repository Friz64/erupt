use erupt::{vk1_0::*, vk1_1::*, vk1_2::*, CoreLoader, InstanceLoader};
use std::ffi::CStr;

const TITLE: &str = "erupt_examples: pointer_chain";

fn main() {
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

    let info = ApplicationInfo::default()
        .builder()
        .api_version(erupt::make_version(1, 1, 0));
    let instance_info = InstanceCreateInfo::default()
        .builder()
        .application_info(&info);

    let mut instance = InstanceLoader::new(
        &core,
        unsafe { core.create_instance(&instance_info, None, None) }.unwrap(),
    )
    .unwrap();
    instance.load_vk1_0().unwrap();
    instance.load_vk1_1().unwrap();

    let physical_devices = unsafe { instance.enumerate_physical_devices(None) }.unwrap();

    for physical_device in physical_devices {
        let properties = unsafe { instance.get_physical_device_properties(physical_device, None) };
        println!("Physical device: {:?}", unsafe {
            CStr::from_ptr(properties.device_name.as_ptr())
        });

        let vk11 = properties.api_version >= erupt::make_version(1, 1, 0);
        let vk12 = properties.api_version >= erupt::make_version(1, 2, 0);

        if vk11 {
            let mut features2 = PhysicalDeviceFeatures2::default();
            let mut vk11features = PhysicalDeviceVulkan11Features::default();
            let mut vk12features = PhysicalDeviceVulkan12Features::default();
            unsafe {
                vk11features.extend(&mut features2);

                if vk12 {
                    vk12features.extend(&mut features2);
                }
            }

            let features2 =
                unsafe { instance.get_physical_device_features2(physical_device, Some(features2)) };

            println!("Vulkan 1.0 Features: {:?}", features2.features);
            println!("Vulkan 1.1 Features: {:?}", vk11features);

            if vk12 {
                println!("Vulkan 1.2 Features: {:?}", vk12features);
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
