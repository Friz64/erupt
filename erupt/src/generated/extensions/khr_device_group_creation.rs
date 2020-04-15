# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_KHR_device_group_creation.html)\n\n## Extends\n- [`MemoryHeapFlagBits`](../../vk1_0/struct.MemoryHeapFlagBits.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_DEVICE_GROUP_CREATION_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_DEVICE_GROUP_CREATION_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_device_group_creation");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const MAX_DEVICE_GROUP_SIZE_KHR: u32 = 32;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumeratePhysicalDeviceGroupsKHR.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkEnumeratePhysicalDeviceGroupsKHR = unsafe extern "system" fn(
    instance: crate::vk1_0::Instance,
    p_physical_device_group_count: *mut u32,
    p_physical_device_group_properties: *mut crate::vk1_1::PhysicalDeviceGroupProperties,
)
    -> crate::vk1_0::Result;
#[doc = "Provides Instance Commands for [`KhrDeviceGroupCreationInstanceLoaderExt`](trait.KhrDeviceGroupCreationInstanceLoaderExt.html)"]
pub struct KhrDeviceGroupCreationInstanceCommands {
    pub enumerate_physical_device_groups_khr: PFN_vkEnumeratePhysicalDeviceGroupsKHR,
}
impl KhrDeviceGroupCreationInstanceCommands {
    #[inline]
    pub fn load(loader: &crate::InstanceLoader) -> Option<KhrDeviceGroupCreationInstanceCommands> {
        unsafe {
            Some(KhrDeviceGroupCreationInstanceCommands {
                enumerate_physical_device_groups_khr: std::mem::transmute(
                    loader.symbol("vkEnumeratePhysicalDeviceGroupsKHR")?,
                ),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`KhrDeviceGroupCreationInstanceCommands`](struct.KhrDeviceGroupCreationInstanceCommands.html)"]
pub trait KhrDeviceGroupCreationInstanceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumeratePhysicalDeviceGroupsKHR.html) · Instance Command"]
    unsafe fn enumerate_physical_device_groups_khr(
        &self,
        physical_device_group_count: Option<u32>,
    ) -> crate::utils::VulkanResult<Vec<crate::vk1_1::PhysicalDeviceGroupProperties>>;
}
impl KhrDeviceGroupCreationInstanceLoaderExt for crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumeratePhysicalDeviceGroupsKHR.html) · Instance Command"]
    unsafe fn enumerate_physical_device_groups_khr(
        &self,
        physical_device_group_count: Option<u32>,
    ) -> crate::utils::VulkanResult<Vec<crate::vk1_1::PhysicalDeviceGroupProperties>> {
        let function = self
            .khr_device_group_creation
            .as_ref()
            .expect("`khr_device_group_creation` not loaded")
            .enumerate_physical_device_groups_khr;
        let mut physical_device_group_count = physical_device_group_count.unwrap_or_else(|| {
            let mut val = Default::default();
            function(self.handle, &mut val, std::ptr::null_mut());
            val
        });
        let mut physical_device_group_properties =
            vec![Default::default(); physical_device_group_count as _];
        let _val = function(
            self.handle,
            &mut physical_device_group_count,
            physical_device_group_properties.as_mut_ptr(),
        );
        crate::utils::VulkanResult::new(_val, physical_device_group_properties)
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceGroupPropertiesKHR.html) · Alias"]
pub type PhysicalDeviceGroupPropertiesKHR = crate::vk1_1::PhysicalDeviceGroupProperties;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceGroupDeviceCreateInfoKHR.html) · Alias"]
pub type DeviceGroupDeviceCreateInfoKHR = crate::vk1_1::DeviceGroupDeviceCreateInfo;
