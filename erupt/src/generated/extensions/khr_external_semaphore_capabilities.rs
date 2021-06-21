#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_SPEC_VERSION")]
pub const KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_EXTENSION_NAME")]
pub const KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_external_semaphore_capabilities");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_PROPERTIES_KHR: *const std::os::raw::c_char = crate::cstr!("vkGetPhysicalDeviceExternalSemaphorePropertiesKHR");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalSemaphoreHandleTypeFlagsKHR.html) · Alias"]
#[doc(alias = "VkExternalSemaphoreHandleTypeFlagsKHR")]
#[allow(non_camel_case_types)]
pub type ExternalSemaphoreHandleTypeFlagsKHR = crate::vk1_1::ExternalSemaphoreHandleTypeFlags;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalSemaphoreFeatureFlagsKHR.html) · Alias"]
#[doc(alias = "VkExternalSemaphoreFeatureFlagsKHR")]
#[allow(non_camel_case_types)]
pub type ExternalSemaphoreFeatureFlagsKHR = crate::vk1_1::ExternalSemaphoreFeatureFlags;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalSemaphoreHandleTypeFlagBitsKHR.html) · Alias"]
#[doc(alias = "VkExternalSemaphoreHandleTypeFlagBitsKHR")]
#[allow(non_camel_case_types)]
pub type ExternalSemaphoreHandleTypeFlagBitsKHR = crate::vk1_1::ExternalSemaphoreHandleTypeFlagBits;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalSemaphoreFeatureFlagBitsKHR.html) · Alias"]
#[doc(alias = "VkExternalSemaphoreFeatureFlagBitsKHR")]
#[allow(non_camel_case_types)]
pub type ExternalSemaphoreFeatureFlagBitsKHR = crate::vk1_1::ExternalSemaphoreFeatureFlagBits;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceExternalSemaphoreInfoKHR.html) · Alias"]
#[doc(alias = "VkPhysicalDeviceExternalSemaphoreInfoKHR")]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceExternalSemaphoreInfoKHR = crate::vk1_1::PhysicalDeviceExternalSemaphoreInfo;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceExternalSemaphoreInfoKHR.html) · Alias"]
#[doc(alias = "VkPhysicalDeviceExternalSemaphoreInfoKHR")]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceExternalSemaphoreInfoKHRBuilder<'a> = crate::vk1_1::PhysicalDeviceExternalSemaphoreInfoBuilder<'a>;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalSemaphorePropertiesKHR.html) · Alias"]
#[doc(alias = "VkExternalSemaphorePropertiesKHR")]
#[allow(non_camel_case_types)]
pub type ExternalSemaphorePropertiesKHR = crate::vk1_1::ExternalSemaphoreProperties;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalSemaphorePropertiesKHR.html) · Alias"]
#[doc(alias = "VkExternalSemaphorePropertiesKHR")]
#[allow(non_camel_case_types)]
pub type ExternalSemaphorePropertiesKHRBuilder<'a> = crate::vk1_1::ExternalSemaphorePropertiesBuilder<'a>;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceExternalSemaphorePropertiesKHR.html) · Alias"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceExternalSemaphorePropertiesKHR = crate::vk1_1::PFN_vkGetPhysicalDeviceExternalSemaphoreProperties;
#[doc = "Provided by [`crate::extensions::khr_external_semaphore_capabilities`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO_KHR: Self = Self::PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO;
    pub const EXTERNAL_SEMAPHORE_PROPERTIES_KHR: Self = Self::EXTERNAL_SEMAPHORE_PROPERTIES;
}
#[doc = "Provided by [`crate::extensions::khr_external_semaphore_capabilities`]"]
impl crate::vk1_1::ExternalSemaphoreHandleTypeFlagBits {
    pub const OPAQUE_FD_KHR: Self = Self::OPAQUE_FD;
    pub const OPAQUE_WIN32_KHR: Self = Self::OPAQUE_WIN32;
    pub const OPAQUE_WIN32_KMT_KHR: Self = Self::OPAQUE_WIN32_KMT;
    pub const D3D12_FENCE_KHR: Self = Self::D3D12_FENCE;
    pub const SYNC_FD_KHR: Self = Self::SYNC_FD;
}
#[doc = "Provided by [`crate::extensions::khr_external_semaphore_capabilities`]"]
impl crate::vk1_1::ExternalSemaphoreFeatureFlagBits {
    pub const EXPORTABLE_KHR: Self = Self::EXPORTABLE;
    pub const IMPORTABLE_KHR: Self = Self::IMPORTABLE;
}
#[doc = "Provided by [`crate::extensions::khr_external_semaphore_capabilities`]"]
impl crate::InstanceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceExternalSemaphorePropertiesKHR.html) · Function"]
    #[doc(alias = "vkGetPhysicalDeviceExternalSemaphorePropertiesKHR")]
    pub unsafe fn get_physical_device_external_semaphore_properties_khr(&self, physical_device: crate::vk1_0::PhysicalDevice, external_semaphore_info: &crate::vk1_1::PhysicalDeviceExternalSemaphoreInfo, external_semaphore_properties: Option<crate::vk1_1::ExternalSemaphoreProperties>) -> crate::vk1_1::ExternalSemaphoreProperties {
        let _function = self.get_physical_device_external_semaphore_properties_khr.expect(crate::NOT_LOADED_MESSAGE);
        let mut external_semaphore_properties = match external_semaphore_properties {
            Some(v) => v,
            None => Default::default(),
        };
        let _return = _function(physical_device as _, external_semaphore_info as _, &mut external_semaphore_properties);
        external_semaphore_properties
    }
}
