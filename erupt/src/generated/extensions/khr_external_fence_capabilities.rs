#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_EXTERNAL_FENCE_CAPABILITIES_SPEC_VERSION")]
pub const KHR_EXTERNAL_FENCE_CAPABILITIES_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_EXTERNAL_FENCE_CAPABILITIES_EXTENSION_NAME")]
pub const KHR_EXTERNAL_FENCE_CAPABILITIES_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_external_fence_capabilities");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_PHYSICAL_DEVICE_EXTERNAL_FENCE_PROPERTIES_KHR: *const std::os::raw::c_char = crate::cstr!("vkGetPhysicalDeviceExternalFencePropertiesKHR");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalFenceHandleTypeFlagsKHR.html) · Alias"]
#[doc(alias = "VkExternalFenceHandleTypeFlagsKHR")]
#[allow(non_camel_case_types)]
pub type ExternalFenceHandleTypeFlagsKHR = crate::vk1_1::ExternalFenceHandleTypeFlags;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalFenceFeatureFlagsKHR.html) · Alias"]
#[doc(alias = "VkExternalFenceFeatureFlagsKHR")]
#[allow(non_camel_case_types)]
pub type ExternalFenceFeatureFlagsKHR = crate::vk1_1::ExternalFenceFeatureFlags;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalFenceHandleTypeFlagBitsKHR.html) · Alias"]
#[doc(alias = "VkExternalFenceHandleTypeFlagBitsKHR")]
#[allow(non_camel_case_types)]
pub type ExternalFenceHandleTypeFlagBitsKHR = crate::vk1_1::ExternalFenceHandleTypeFlagBits;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalFenceFeatureFlagBitsKHR.html) · Alias"]
#[doc(alias = "VkExternalFenceFeatureFlagBitsKHR")]
#[allow(non_camel_case_types)]
pub type ExternalFenceFeatureFlagBitsKHR = crate::vk1_1::ExternalFenceFeatureFlagBits;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceExternalFenceInfoKHR.html) · Alias"]
#[doc(alias = "VkPhysicalDeviceExternalFenceInfoKHR")]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceExternalFenceInfoKHR = crate::vk1_1::PhysicalDeviceExternalFenceInfo;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceExternalFenceInfoKHR.html) · Alias"]
#[doc(alias = "VkPhysicalDeviceExternalFenceInfoKHR")]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceExternalFenceInfoKHRBuilder<'a> = crate::vk1_1::PhysicalDeviceExternalFenceInfoBuilder<'a>;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalFencePropertiesKHR.html) · Alias"]
#[doc(alias = "VkExternalFencePropertiesKHR")]
#[allow(non_camel_case_types)]
pub type ExternalFencePropertiesKHR = crate::vk1_1::ExternalFenceProperties;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalFencePropertiesKHR.html) · Alias"]
#[doc(alias = "VkExternalFencePropertiesKHR")]
#[allow(non_camel_case_types)]
pub type ExternalFencePropertiesKHRBuilder<'a> = crate::vk1_1::ExternalFencePropertiesBuilder<'a>;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceExternalFencePropertiesKHR.html) · Alias"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceExternalFencePropertiesKHR = crate::vk1_1::PFN_vkGetPhysicalDeviceExternalFenceProperties;
#[doc = "Provided by [`crate::extensions::khr_external_fence_capabilities`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO_KHR: Self = Self::PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO;
    pub const EXTERNAL_FENCE_PROPERTIES_KHR: Self = Self::EXTERNAL_FENCE_PROPERTIES;
}
#[doc = "Provided by [`crate::extensions::khr_external_fence_capabilities`]"]
impl crate::vk1_1::ExternalFenceHandleTypeFlagBits {
    pub const OPAQUE_FD_KHR: Self = Self::OPAQUE_FD;
    pub const OPAQUE_WIN32_KHR: Self = Self::OPAQUE_WIN32;
    pub const OPAQUE_WIN32_KMT_KHR: Self = Self::OPAQUE_WIN32_KMT;
    pub const SYNC_FD_KHR: Self = Self::SYNC_FD;
}
#[doc = "Provided by [`crate::extensions::khr_external_fence_capabilities`]"]
impl crate::vk1_1::ExternalFenceFeatureFlagBits {
    pub const EXPORTABLE_KHR: Self = Self::EXPORTABLE;
    pub const IMPORTABLE_KHR: Self = Self::IMPORTABLE;
}
#[doc = "Provided by [`crate::extensions::khr_external_fence_capabilities`]"]
impl crate::InstanceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceExternalFencePropertiesKHR.html) · Function"]
    #[doc(alias = "vkGetPhysicalDeviceExternalFencePropertiesKHR")]
    pub unsafe fn get_physical_device_external_fence_properties_khr(&self, physical_device: crate::vk1_0::PhysicalDevice, external_fence_info: &crate::vk1_1::PhysicalDeviceExternalFenceInfo, external_fence_properties: Option<crate::vk1_1::ExternalFenceProperties>) -> crate::vk1_1::ExternalFenceProperties {
        let _function = self.get_physical_device_external_fence_properties_khr.expect("tried to call a function that isn't loaded");
        let mut external_fence_properties = match external_fence_properties {
            Some(v) => v,
            None => Default::default(),
        };
        let _return = _function(physical_device as _, external_fence_info as _, &mut external_fence_properties);
        external_fence_properties
    }
}
