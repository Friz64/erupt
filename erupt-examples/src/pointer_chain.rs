use erupt::{vk1_0::*, vk1_1::*, vk1_2::*, CoreLoader, InstanceLoader};

fn main() {
    unsafe {
        let mut core = CoreLoader::new().unwrap();
        core.load_vk1_0().unwrap();

        let api_version = core.instance_version();
        println!(
            "erupt-examples: pointer-chain - Vulkan {}.{}.{}",
            erupt::version_major(api_version),
            erupt::version_minor(api_version),
            erupt::version_patch(api_version)
        );

        let info = ApplicationInfo::default()
            .builder()
            .api_version(erupt::make_version(1, 2, 0));
        let instance_info = InstanceCreateInfo::default()
            .builder()
            .application_info(&info);

        let mut instance = InstanceLoader::new(
            &core,
            core.create_instance(&instance_info, None, None).unwrap(),
        )
        .unwrap();
        instance.load_vk1_0().unwrap();
        instance.load_vk1_1().unwrap();
        println!("Instance: {:p}", instance.handle);

        let physical_devices = instance.enumerate_physical_devices(None).unwrap();
        let physical_device = physical_devices[0];
        println!("Physical Device: {:p}", physical_device);

        let mut features2 = PhysicalDeviceFeatures2::default();
        let mut vk11features = PhysicalDeviceVulkan11Features::default();
        vk11features.extend(&mut features2);
        let mut vk12features = PhysicalDeviceVulkan12Features::default();
        vk12features.extend(&mut features2);

        let features2 = instance.get_physical_device_features2(physical_device, Some(features2));

        println!("Vulkan 1.0 Features: {:?}", features2.features);
        println!("Vulkan 1.1 Features: {:?}", vk11features);
        println!("Vulkan 1.2 Features: {:?}", vk12features);

        instance.destroy_instance(None);
    }
}
