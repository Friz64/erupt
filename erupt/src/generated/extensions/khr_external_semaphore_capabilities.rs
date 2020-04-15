# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_KHR_external_semaphore_capabilities.html)\n\n## Extends\n- [`ExternalSemaphoreFeatureFlagBits`](../../vk1_1/struct.ExternalSemaphoreFeatureFlagBits.html)\n- [`ExternalSemaphoreHandleTypeFlagBits`](../../vk1_1/struct.ExternalSemaphoreHandleTypeFlagBits.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_external_semaphore_capabilities");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceExternalSemaphorePropertiesKHR.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceExternalSemaphorePropertiesKHR =
    unsafe extern "system" fn(
        physical_device: crate::vk1_0::PhysicalDevice,
        p_external_semaphore_info: *const crate::vk1_1::PhysicalDeviceExternalSemaphoreInfo,
        p_external_semaphore_properties: *mut crate::vk1_1::ExternalSemaphoreProperties,
    ) -> std::ffi::c_void;
#[doc = "Provides Instance Commands for [`KhrExternalSemaphoreCapabilitiesInstanceLoaderExt`](trait.KhrExternalSemaphoreCapabilitiesInstanceLoaderExt.html)"]
pub struct KhrExternalSemaphoreCapabilitiesInstanceCommands {
    pub get_physical_device_external_semaphore_properties_khr:
        PFN_vkGetPhysicalDeviceExternalSemaphorePropertiesKHR,
}
impl KhrExternalSemaphoreCapabilitiesInstanceCommands {
    #[inline]
    pub fn load(
        loader: &crate::InstanceLoader,
    ) -> Option<KhrExternalSemaphoreCapabilitiesInstanceCommands> {
        unsafe {
            Some(KhrExternalSemaphoreCapabilitiesInstanceCommands {
                get_physical_device_external_semaphore_properties_khr: std::mem::transmute(
                    loader.symbol("vkGetPhysicalDeviceExternalSemaphorePropertiesKHR")?,
                ),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`KhrExternalSemaphoreCapabilitiesInstanceCommands`](struct.KhrExternalSemaphoreCapabilitiesInstanceCommands.html)"]
pub trait KhrExternalSemaphoreCapabilitiesInstanceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceExternalSemaphorePropertiesKHR.html) · Instance Command"]
    unsafe fn get_physical_device_external_semaphore_properties_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        external_semaphore_info: &crate::vk1_1::PhysicalDeviceExternalSemaphoreInfo,
        external_semaphore_properties: Option<crate::vk1_1::ExternalSemaphoreProperties>,
    ) -> crate::vk1_1::ExternalSemaphoreProperties;
}
impl KhrExternalSemaphoreCapabilitiesInstanceLoaderExt for crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceExternalSemaphorePropertiesKHR.html) · Instance Command"]
    unsafe fn get_physical_device_external_semaphore_properties_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        external_semaphore_info: &crate::vk1_1::PhysicalDeviceExternalSemaphoreInfo,
        external_semaphore_properties: Option<crate::vk1_1::ExternalSemaphoreProperties>,
    ) -> crate::vk1_1::ExternalSemaphoreProperties {
        let function = self
            .khr_external_semaphore_capabilities
            .as_ref()
            .expect("`khr_external_semaphore_capabilities` not loaded")
            .get_physical_device_external_semaphore_properties_khr;
        let mut external_semaphore_properties =
            external_semaphore_properties.unwrap_or_else(|| Default::default());
        let _val = function(
            physical_device,
            external_semaphore_info,
            &mut external_semaphore_properties,
        );
        external_semaphore_properties
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalSemaphoreHandleTypeFlagsKHR.html) · Alias"]
pub type ExternalSemaphoreHandleTypeFlagsKHR = crate::vk1_1::ExternalSemaphoreHandleTypeFlags;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalSemaphoreFeatureFlagsKHR.html) · Alias"]
pub type ExternalSemaphoreFeatureFlagsKHR = crate::vk1_1::ExternalSemaphoreFeatureFlags;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceExternalSemaphoreInfoKHR.html) · Alias"]
pub type PhysicalDeviceExternalSemaphoreInfoKHR = crate::vk1_1::PhysicalDeviceExternalSemaphoreInfo;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalSemaphorePropertiesKHR.html) · Alias"]
pub type ExternalSemaphorePropertiesKHR = crate::vk1_1::ExternalSemaphoreProperties;
