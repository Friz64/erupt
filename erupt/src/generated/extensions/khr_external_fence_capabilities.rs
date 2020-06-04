# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_KHR_external_fence_capabilities.html)\n\n## Extends\n- [`ExternalFenceFeatureFlagBits`](../../vk1_1/struct.ExternalFenceFeatureFlagBits.html)\n- [`ExternalFenceHandleTypeFlagBits`](../../vk1_1/struct.ExternalFenceHandleTypeFlagBits.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_EXTERNAL_FENCE_CAPABILITIES_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_EXTERNAL_FENCE_CAPABILITIES_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_external_fence_capabilities");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceExternalFencePropertiesKHR.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceExternalFencePropertiesKHR =
    unsafe extern "system" fn(
        physical_device: crate::vk1_0::PhysicalDevice,
        p_external_fence_info: *const crate::vk1_1::PhysicalDeviceExternalFenceInfo,
        p_external_fence_properties: *mut crate::vk1_1::ExternalFenceProperties,
    ) -> std::ffi::c_void;
#[doc = "Provides Instance Commands for [`KhrExternalFenceCapabilitiesInstanceLoaderExt`](trait.KhrExternalFenceCapabilitiesInstanceLoaderExt.html)"]
pub struct KhrExternalFenceCapabilitiesInstanceCommands {
    pub get_physical_device_external_fence_properties_khr:
        Option<PFN_vkGetPhysicalDeviceExternalFencePropertiesKHR>,
}
impl KhrExternalFenceCapabilitiesInstanceCommands {
    #[inline]
    pub fn load(
        loader: &crate::InstanceLoader,
    ) -> Option<KhrExternalFenceCapabilitiesInstanceCommands> {
        unsafe {
            let mut success = false;
            let table = KhrExternalFenceCapabilitiesInstanceCommands {
                get_physical_device_external_fence_properties_khr: std::mem::transmute({
                    let symbol = loader.symbol("vkGetPhysicalDeviceExternalFencePropertiesKHR");
                    success |= symbol.is_some();
                    symbol
                }),
            };
            if success {
                Some(table)
            } else {
                None
            }
        }
    }
}
#[inline]
fn instance_commands(
    loader: &crate::InstanceLoader,
) -> &KhrExternalFenceCapabilitiesInstanceCommands {
    loader
        .khr_external_fence_capabilities
        .as_ref()
        .expect("`khr_external_fence_capabilities` not loaded")
}
#[doc = "Provides high level command wrappers for [`KhrExternalFenceCapabilitiesInstanceCommands`](struct.KhrExternalFenceCapabilitiesInstanceCommands.html)"]
pub trait KhrExternalFenceCapabilitiesInstanceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceExternalFencePropertiesKHR.html) · Instance Command"]
    unsafe fn get_physical_device_external_fence_properties_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        external_fence_info: &crate::vk1_1::PhysicalDeviceExternalFenceInfo,
        external_fence_properties: Option<crate::vk1_1::ExternalFenceProperties>,
    ) -> crate::vk1_1::ExternalFenceProperties;
}
impl KhrExternalFenceCapabilitiesInstanceLoaderExt for crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceExternalFencePropertiesKHR.html) · Instance Command"]
    unsafe fn get_physical_device_external_fence_properties_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        external_fence_info: &crate::vk1_1::PhysicalDeviceExternalFenceInfo,
        external_fence_properties: Option<crate::vk1_1::ExternalFenceProperties>,
    ) -> crate::vk1_1::ExternalFenceProperties {
        let function = instance_commands(self)
            .get_physical_device_external_fence_properties_khr
            .as_ref()
            .expect("`get_physical_device_external_fence_properties_khr` not available");
        let mut external_fence_properties =
            external_fence_properties.unwrap_or_else(|| Default::default());
        let _val = function(
            physical_device,
            external_fence_info,
            &mut external_fence_properties,
        );
        external_fence_properties
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalFenceHandleTypeFlagsKHR.html) · Alias"]
pub type ExternalFenceHandleTypeFlagsKHR = crate::vk1_1::ExternalFenceHandleTypeFlags;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalFenceFeatureFlagsKHR.html) · Alias"]
pub type ExternalFenceFeatureFlagsKHR = crate::vk1_1::ExternalFenceFeatureFlags;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceExternalFenceInfoKHR.html) · Alias"]
pub type PhysicalDeviceExternalFenceInfoKHR = crate::vk1_1::PhysicalDeviceExternalFenceInfo;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalFencePropertiesKHR.html) · Alias"]
pub type ExternalFencePropertiesKHR = crate::vk1_1::ExternalFenceProperties;
