# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_KHR_timeline_semaphore.html)\n\n## Extends\n- [`SemaphoreType`](../../vk1_2/struct.SemaphoreType.html)\n- [`SemaphoreWaitFlagBits`](../../vk1_2/struct.SemaphoreWaitFlagBits.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_TIMELINE_SEMAPHORE_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_TIMELINE_SEMAPHORE_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_timeline_semaphore");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSemaphoreCounterValueKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetSemaphoreCounterValueKHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    semaphore: crate::vk1_0::Semaphore,
    p_value: *mut u64,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkWaitSemaphoresKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkWaitSemaphoresKHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_wait_info: *const crate::vk1_2::SemaphoreWaitInfo,
    timeout: u64,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSignalSemaphoreKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkSignalSemaphoreKHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_signal_info: *const crate::vk1_2::SemaphoreSignalInfo,
) -> crate::vk1_0::Result;
#[doc = "Provides Device Commands for [`KhrTimelineSemaphoreDeviceLoaderExt`](trait.KhrTimelineSemaphoreDeviceLoaderExt.html)"]
pub struct KhrTimelineSemaphoreDeviceCommands {
    pub get_semaphore_counter_value_khr: PFN_vkGetSemaphoreCounterValueKHR,
    pub wait_semaphores_khr: PFN_vkWaitSemaphoresKHR,
    pub signal_semaphore_khr: PFN_vkSignalSemaphoreKHR,
}
impl KhrTimelineSemaphoreDeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<KhrTimelineSemaphoreDeviceCommands> {
        unsafe {
            Some(KhrTimelineSemaphoreDeviceCommands {
                get_semaphore_counter_value_khr: std::mem::transmute(
                    loader.symbol("vkGetSemaphoreCounterValueKHR")?,
                ),
                wait_semaphores_khr: std::mem::transmute(loader.symbol("vkWaitSemaphoresKHR")?),
                signal_semaphore_khr: std::mem::transmute(loader.symbol("vkSignalSemaphoreKHR")?),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`KhrTimelineSemaphoreDeviceCommands`](struct.KhrTimelineSemaphoreDeviceCommands.html)"]
pub trait KhrTimelineSemaphoreDeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSemaphoreCounterValueKHR.html) · Device Command"]
    unsafe fn get_semaphore_counter_value_khr(
        &self,
        semaphore: crate::vk1_0::Semaphore,
        value: Option<u64>,
    ) -> crate::utils::VulkanResult<u64>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkWaitSemaphoresKHR.html) · Device Command"]
    unsafe fn wait_semaphores_khr(
        &self,
        wait_info: &crate::vk1_2::SemaphoreWaitInfo,
        timeout: u64,
    ) -> crate::utils::VulkanResult<()>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSignalSemaphoreKHR.html) · Device Command"]
    unsafe fn signal_semaphore_khr(
        &self,
        signal_info: &crate::vk1_2::SemaphoreSignalInfo,
    ) -> crate::utils::VulkanResult<()>;
}
impl KhrTimelineSemaphoreDeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSemaphoreCounterValueKHR.html) · Device Command"]
    unsafe fn get_semaphore_counter_value_khr(
        &self,
        semaphore: crate::vk1_0::Semaphore,
        value: Option<u64>,
    ) -> crate::utils::VulkanResult<u64> {
        let function = self
            .khr_timeline_semaphore
            .as_ref()
            .expect("`khr_timeline_semaphore` not loaded")
            .get_semaphore_counter_value_khr;
        let mut value = value.unwrap_or_else(|| Default::default());
        let _val = function(self.handle, semaphore, &mut value);
        crate::utils::VulkanResult::new(_val, value)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkWaitSemaphoresKHR.html) · Device Command"]
    unsafe fn wait_semaphores_khr(
        &self,
        wait_info: &crate::vk1_2::SemaphoreWaitInfo,
        timeout: u64,
    ) -> crate::utils::VulkanResult<()> {
        let function = self
            .khr_timeline_semaphore
            .as_ref()
            .expect("`khr_timeline_semaphore` not loaded")
            .wait_semaphores_khr;
        let _val = function(self.handle, wait_info, timeout);
        crate::utils::VulkanResult::new(_val, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSignalSemaphoreKHR.html) · Device Command"]
    unsafe fn signal_semaphore_khr(
        &self,
        signal_info: &crate::vk1_2::SemaphoreSignalInfo,
    ) -> crate::utils::VulkanResult<()> {
        let function = self
            .khr_timeline_semaphore
            .as_ref()
            .expect("`khr_timeline_semaphore` not loaded")
            .signal_semaphore_khr;
        let _val = function(self.handle, signal_info);
        crate::utils::VulkanResult::new(_val, ())
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreTypeKHR.html) · Alias"]
pub type SemaphoreTypeKHR = crate::vk1_2::SemaphoreType;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceTimelineSemaphoreFeaturesKHR.html) · Alias"]
pub type PhysicalDeviceTimelineSemaphoreFeaturesKHR =
    crate::vk1_2::PhysicalDeviceTimelineSemaphoreFeatures;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceTimelineSemaphorePropertiesKHR.html) · Alias"]
pub type PhysicalDeviceTimelineSemaphorePropertiesKHR =
    crate::vk1_2::PhysicalDeviceTimelineSemaphoreProperties;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreTypeCreateInfoKHR.html) · Alias"]
pub type SemaphoreTypeCreateInfoKHR = crate::vk1_2::SemaphoreTypeCreateInfo;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkTimelineSemaphoreSubmitInfoKHR.html) · Alias"]
pub type TimelineSemaphoreSubmitInfoKHR = crate::vk1_2::TimelineSemaphoreSubmitInfo;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreWaitFlagsKHR.html) · Alias"]
pub type SemaphoreWaitFlagsKHR = crate::vk1_2::SemaphoreWaitFlags;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreWaitInfoKHR.html) · Alias"]
pub type SemaphoreWaitInfoKHR = crate::vk1_2::SemaphoreWaitInfo;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreSignalInfoKHR.html) · Alias"]
pub type SemaphoreSignalInfoKHR = crate::vk1_2::SemaphoreSignalInfo;
