use erupt::{
    utils::{
        self,
        features::{self, AbstractFeaturesNode},
    },
    vk, EntryLoader, ExtendableFrom, InstanceLoader,
};
use std::{ptr, sync::Arc};

const TITLE: &str = "erupt_examples: features";

fn main() {
    let mut buffer_device_address_features =
        vk::PhysicalDeviceBufferDeviceAddressFeaturesBuilder::new().buffer_device_address(true);

    unsafe {
        let node = ptr::addr_of_mut!(buffer_device_address_features) as _;
        dbg!(&mut *AbstractFeaturesNode::from_raw(node).unwrap());
        dbg!(AbstractFeaturesNode::copy_single(node).unwrap());
    }

    let mut mesh_shader_features_nv = vk::PhysicalDeviceMeshShaderFeaturesNVBuilder::new(); //.mesh_shader(true);
    let mut features = vk::PhysicalDeviceFeatures2Builder::new()
        .features(
            vk::PhysicalDeviceFeaturesBuilder::new()
                .multi_viewport(true)
                .inherited_queries(true)
                .build(),
        )
        .extend_from(&mut buffer_device_address_features)
        .extend_from(&mut mesh_shader_features_nv);

    unsafe {
        let node = ptr::addr_of_mut!(features) as _;
        let copied = AbstractFeaturesNode::copy_chain(node).unwrap();
        for node in utils::iterate_ptr_chain(copied as _) {
            dbg!(node);
        }

        AbstractFeaturesNode::free_chain(copied).unwrap();
    }

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
        unsafe {
            dbg!(features::supported(&instance, physical_device, &features)).unwrap();
        }
    }

    unsafe {
        instance.destroy_instance(None);
    }
}
