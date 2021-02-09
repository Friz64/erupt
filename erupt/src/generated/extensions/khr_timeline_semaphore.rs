#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_TIMELINE_SEMAPHORE_SPEC_VERSION")]
pub const KHR_TIMELINE_SEMAPHORE_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_TIMELINE_SEMAPHORE_EXTENSION_NAME")]
pub const KHR_TIMELINE_SEMAPHORE_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_timeline_semaphore");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_SEMAPHORE_COUNTER_VALUE_KHR: *const std::os::raw::c_char =
    crate::cstr!("vkGetSemaphoreCounterValueKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_WAIT_SEMAPHORES_KHR: *const std::os::raw::c_char = crate::cstr!("vkWaitSemaphoresKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_SIGNAL_SEMAPHORE_KHR: *const std::os::raw::c_char =
    crate::cstr!("vkSignalSemaphoreKHR");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreWaitFlagsKHR.html) · Alias"]
#[doc(alias = "VkSemaphoreWaitFlagsKHR")]
#[allow(non_camel_case_types)]
pub type SemaphoreWaitFlagsKHR = crate::vk1_2::SemaphoreWaitFlags;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreTypeKHR.html) · Alias"]
#[doc(alias = "VkSemaphoreTypeKHR")]
#[allow(non_camel_case_types)]
pub type SemaphoreTypeKHR = crate::vk1_2::SemaphoreType;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreWaitFlagBitsKHR.html) · Alias"]
#[doc(alias = "VkSemaphoreWaitFlagBitsKHR")]
#[allow(non_camel_case_types)]
pub type SemaphoreWaitFlagBitsKHR = crate::vk1_2::SemaphoreWaitFlagBits;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceTimelineSemaphoreFeaturesKHR.html) · Alias"]
#[doc(alias = "VkPhysicalDeviceTimelineSemaphoreFeaturesKHR")]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceTimelineSemaphoreFeaturesKHR =
    crate::vk1_2::PhysicalDeviceTimelineSemaphoreFeatures;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceTimelineSemaphoreFeaturesKHR.html) · Alias"]
#[doc(alias = "VkPhysicalDeviceTimelineSemaphoreFeaturesKHR")]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceTimelineSemaphoreFeaturesKHRBuilder<'a> =
    crate::vk1_2::PhysicalDeviceTimelineSemaphoreFeaturesBuilder<'a>;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceTimelineSemaphorePropertiesKHR.html) · Alias"]
#[doc(alias = "VkPhysicalDeviceTimelineSemaphorePropertiesKHR")]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceTimelineSemaphorePropertiesKHR =
    crate::vk1_2::PhysicalDeviceTimelineSemaphoreProperties;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceTimelineSemaphorePropertiesKHR.html) · Alias"]
#[doc(alias = "VkPhysicalDeviceTimelineSemaphorePropertiesKHR")]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceTimelineSemaphorePropertiesKHRBuilder<'a> =
    crate::vk1_2::PhysicalDeviceTimelineSemaphorePropertiesBuilder<'a>;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreTypeCreateInfoKHR.html) · Alias"]
#[doc(alias = "VkSemaphoreTypeCreateInfoKHR")]
#[allow(non_camel_case_types)]
pub type SemaphoreTypeCreateInfoKHR = crate::vk1_2::SemaphoreTypeCreateInfo;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreTypeCreateInfoKHR.html) · Alias"]
#[doc(alias = "VkSemaphoreTypeCreateInfoKHR")]
#[allow(non_camel_case_types)]
pub type SemaphoreTypeCreateInfoKHRBuilder<'a> = crate::vk1_2::SemaphoreTypeCreateInfoBuilder<'a>;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkTimelineSemaphoreSubmitInfoKHR.html) · Alias"]
#[doc(alias = "VkTimelineSemaphoreSubmitInfoKHR")]
#[allow(non_camel_case_types)]
pub type TimelineSemaphoreSubmitInfoKHR = crate::vk1_2::TimelineSemaphoreSubmitInfo;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkTimelineSemaphoreSubmitInfoKHR.html) · Alias"]
#[doc(alias = "VkTimelineSemaphoreSubmitInfoKHR")]
#[allow(non_camel_case_types)]
pub type TimelineSemaphoreSubmitInfoKHRBuilder<'a> =
    crate::vk1_2::TimelineSemaphoreSubmitInfoBuilder<'a>;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreWaitInfoKHR.html) · Alias"]
#[doc(alias = "VkSemaphoreWaitInfoKHR")]
#[allow(non_camel_case_types)]
pub type SemaphoreWaitInfoKHR = crate::vk1_2::SemaphoreWaitInfo;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreWaitInfoKHR.html) · Alias"]
#[doc(alias = "VkSemaphoreWaitInfoKHR")]
#[allow(non_camel_case_types)]
pub type SemaphoreWaitInfoKHRBuilder<'a> = crate::vk1_2::SemaphoreWaitInfoBuilder<'a>;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreSignalInfoKHR.html) · Alias"]
#[doc(alias = "VkSemaphoreSignalInfoKHR")]
#[allow(non_camel_case_types)]
pub type SemaphoreSignalInfoKHR = crate::vk1_2::SemaphoreSignalInfo;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreSignalInfoKHR.html) · Alias"]
#[doc(alias = "VkSemaphoreSignalInfoKHR")]
#[allow(non_camel_case_types)]
pub type SemaphoreSignalInfoKHRBuilder<'a> = crate::vk1_2::SemaphoreSignalInfoBuilder<'a>;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSemaphoreCounterValueKHR.html) · Alias"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetSemaphoreCounterValueKHR = crate::vk1_2::PFN_vkGetSemaphoreCounterValue;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkWaitSemaphoresKHR.html) · Alias"]
#[allow(non_camel_case_types)]
pub type PFN_vkWaitSemaphoresKHR = crate::vk1_2::PFN_vkWaitSemaphores;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSignalSemaphoreKHR.html) · Alias"]
#[allow(non_camel_case_types)]
pub type PFN_vkSignalSemaphoreKHR = crate::vk1_2::PFN_vkSignalSemaphore;
#[doc = "Provided by [`crate::extensions::khr_timeline_semaphore`]"]
impl crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSemaphoreCounterValueKHR.html) · Function"]
    #[doc(alias = "vkGetSemaphoreCounterValueKHR")]
    pub unsafe fn get_semaphore_counter_value_khr(
        &self,
        semaphore: crate::vk1_0::Semaphore,
        value: Option<u64>,
    ) -> crate::utils::VulkanResult<u64> {
        let _function = self
            .get_semaphore_counter_value_khr
            .expect("`get_semaphore_counter_value_khr` is not loaded");
        let mut value = match value {
            Some(v) => v,
            None => Default::default(),
        };
        let _return = _function(self.handle, semaphore as _, &mut value);
        crate::utils::VulkanResult::new(_return, value)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkWaitSemaphoresKHR.html) · Function"]
    #[doc(alias = "vkWaitSemaphoresKHR")]
    pub unsafe fn wait_semaphores_khr(
        &self,
        wait_info: &crate::vk1_2::SemaphoreWaitInfo,
        timeout: u64,
    ) -> crate::utils::VulkanResult<()> {
        let _function = self
            .wait_semaphores_khr
            .expect("`wait_semaphores_khr` is not loaded");
        let _return = _function(self.handle, wait_info as _, timeout as _);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSignalSemaphoreKHR.html) · Function"]
    #[doc(alias = "vkSignalSemaphoreKHR")]
    pub unsafe fn signal_semaphore_khr(
        &self,
        signal_info: &crate::vk1_2::SemaphoreSignalInfo,
    ) -> crate::utils::VulkanResult<()> {
        let _function = self
            .signal_semaphore_khr
            .expect("`signal_semaphore_khr` is not loaded");
        let _return = _function(self.handle, signal_info as _);
        crate::utils::VulkanResult::new(_return, ())
    }
}
