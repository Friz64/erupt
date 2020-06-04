# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_KHR_external_memory_capabilities.html)\n\n## Extends\n- [`ExternalMemoryFeatureFlagBits`](../../vk1_1/struct.ExternalMemoryFeatureFlagBits.html)\n- [`ExternalMemoryHandleTypeFlagBits`](../../vk1_1/struct.ExternalMemoryHandleTypeFlagBits.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_external_memory_capabilities");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const LUID_SIZE_KHR: u32 = 8;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceExternalBufferPropertiesKHR.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceExternalBufferPropertiesKHR =
    unsafe extern "system" fn(
        physical_device: crate::vk1_0::PhysicalDevice,
        p_external_buffer_info: *const crate::vk1_1::PhysicalDeviceExternalBufferInfo,
        p_external_buffer_properties: *mut crate::vk1_1::ExternalBufferProperties,
    ) -> std::ffi::c_void;
#[doc = "Provides Instance Commands for [`KhrExternalMemoryCapabilitiesInstanceLoaderExt`](trait.KhrExternalMemoryCapabilitiesInstanceLoaderExt.html)"]
pub struct KhrExternalMemoryCapabilitiesInstanceCommands {
    pub get_physical_device_external_buffer_properties_khr:
        Option<PFN_vkGetPhysicalDeviceExternalBufferPropertiesKHR>,
}
impl KhrExternalMemoryCapabilitiesInstanceCommands {
    #[inline]
    pub fn load(
        loader: &crate::InstanceLoader,
    ) -> Option<KhrExternalMemoryCapabilitiesInstanceCommands> {
        unsafe {
            let mut success = false;
            let table = KhrExternalMemoryCapabilitiesInstanceCommands {
                get_physical_device_external_buffer_properties_khr: std::mem::transmute({
                    let symbol = loader.symbol("vkGetPhysicalDeviceExternalBufferPropertiesKHR");
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
) -> &KhrExternalMemoryCapabilitiesInstanceCommands {
    loader
        .khr_external_memory_capabilities
        .as_ref()
        .expect("`khr_external_memory_capabilities` not loaded")
}
#[doc = "Provides high level command wrappers for [`KhrExternalMemoryCapabilitiesInstanceCommands`](struct.KhrExternalMemoryCapabilitiesInstanceCommands.html)"]
pub trait KhrExternalMemoryCapabilitiesInstanceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceExternalBufferPropertiesKHR.html) · Instance Command"]
    unsafe fn get_physical_device_external_buffer_properties_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        external_buffer_info: &crate::vk1_1::PhysicalDeviceExternalBufferInfo,
        external_buffer_properties: Option<crate::vk1_1::ExternalBufferProperties>,
    ) -> crate::vk1_1::ExternalBufferProperties;
}
impl KhrExternalMemoryCapabilitiesInstanceLoaderExt for crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceExternalBufferPropertiesKHR.html) · Instance Command"]
    unsafe fn get_physical_device_external_buffer_properties_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        external_buffer_info: &crate::vk1_1::PhysicalDeviceExternalBufferInfo,
        external_buffer_properties: Option<crate::vk1_1::ExternalBufferProperties>,
    ) -> crate::vk1_1::ExternalBufferProperties {
        let function = instance_commands(self)
            .get_physical_device_external_buffer_properties_khr
            .as_ref()
            .expect("`get_physical_device_external_buffer_properties_khr` not available");
        let mut external_buffer_properties =
            external_buffer_properties.unwrap_or_else(|| Default::default());
        let _val = function(
            physical_device,
            external_buffer_info,
            &mut external_buffer_properties,
        );
        external_buffer_properties
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalMemoryHandleTypeFlagsKHR.html) · Alias"]
pub type ExternalMemoryHandleTypeFlagsKHR = crate::vk1_1::ExternalMemoryHandleTypeFlags;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalMemoryFeatureFlagsKHR.html) · Alias"]
pub type ExternalMemoryFeatureFlagsKHR = crate::vk1_1::ExternalMemoryFeatureFlags;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalMemoryPropertiesKHR.html) · Alias"]
pub type ExternalMemoryPropertiesKHR = crate::vk1_1::ExternalMemoryProperties;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceExternalImageFormatInfoKHR.html) · Alias"]
pub type PhysicalDeviceExternalImageFormatInfoKHR =
    crate::vk1_1::PhysicalDeviceExternalImageFormatInfo;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalImageFormatPropertiesKHR.html) · Alias"]
pub type ExternalImageFormatPropertiesKHR = crate::vk1_1::ExternalImageFormatProperties;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceExternalBufferInfoKHR.html) · Alias"]
pub type PhysicalDeviceExternalBufferInfoKHR = crate::vk1_1::PhysicalDeviceExternalBufferInfo;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalBufferPropertiesKHR.html) · Alias"]
pub type ExternalBufferPropertiesKHR = crate::vk1_1::ExternalBufferProperties;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceIDPropertiesKHR.html) · Alias"]
pub type PhysicalDeviceIDPropertiesKHR = crate::vk1_1::PhysicalDeviceIDProperties;
