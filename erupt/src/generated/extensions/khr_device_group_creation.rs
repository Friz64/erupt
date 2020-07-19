#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const MAX_DEVICE_GROUP_SIZE_KHR: u32 = 32;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_DEVICE_GROUP_CREATION_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_DEVICE_GROUP_CREATION_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_device_group_creation");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_ENUMERATE_PHYSICAL_DEVICE_GROUPS_KHR: *const std::os::raw::c_char =
    crate::cstr!("vkEnumeratePhysicalDeviceGroupsKHR");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceGroupPropertiesKHR.html) · Alias"]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceGroupPropertiesKHR = crate::vk1_1::PhysicalDeviceGroupProperties;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceGroupPropertiesKHR.html) · Alias"]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceGroupPropertiesKHRBuilder<'a> =
    crate::vk1_1::PhysicalDeviceGroupPropertiesBuilder<'a>;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceGroupDeviceCreateInfoKHR.html) · Alias"]
#[allow(non_camel_case_types)]
pub type DeviceGroupDeviceCreateInfoKHR = crate::vk1_1::DeviceGroupDeviceCreateInfo;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceGroupDeviceCreateInfoKHR.html) · Alias"]
#[allow(non_camel_case_types)]
pub type DeviceGroupDeviceCreateInfoKHRBuilder<'a> =
    crate::vk1_1::DeviceGroupDeviceCreateInfoBuilder<'a>;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumeratePhysicalDeviceGroupsKHR.html) · Alias"]
#[allow(non_camel_case_types)]
pub type PFN_vkEnumeratePhysicalDeviceGroupsKHR = crate::vk1_1::PFN_vkEnumeratePhysicalDeviceGroups;
#[doc = "Provided by [`extensions::khr_device_group_creation`](extensions/khr_device_group_creation/index.html)"]
impl crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumeratePhysicalDeviceGroupsKHR.html) · Function"]
    pub unsafe fn enumerate_physical_device_groups_khr(
        &self,
        physical_device_group_count: Option<u32>,
    ) -> crate::utils::VulkanResult<Vec<crate::vk1_1::PhysicalDeviceGroupProperties>> {
        let _function = self
            .enumerate_physical_device_groups_khr
            .expect("`enumerate_physical_device_groups_khr` is not loaded");
        let mut physical_device_group_count = match physical_device_group_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                _function(self.handle, &mut v, std::ptr::null_mut());
                v
            }
        };
        let mut physical_device_group_properties =
            vec![Default::default(); physical_device_group_count as _];
        let _return = _function(
            self.handle,
            &mut physical_device_group_count,
            physical_device_group_properties.as_mut_ptr(),
        );
        crate::utils::VulkanResult::new(_return, physical_device_group_properties)
    }
}
