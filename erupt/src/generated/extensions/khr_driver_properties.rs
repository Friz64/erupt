#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_MAX_DRIVER_NAME_SIZE_KHR")]
pub const MAX_DRIVER_NAME_SIZE_KHR: u32 = 256;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_MAX_DRIVER_INFO_SIZE_KHR")]
pub const MAX_DRIVER_INFO_SIZE_KHR: u32 = 256;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_DRIVER_PROPERTIES_SPEC_VERSION")]
pub const KHR_DRIVER_PROPERTIES_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_DRIVER_PROPERTIES_EXTENSION_NAME")]
pub const KHR_DRIVER_PROPERTIES_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_driver_properties");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDriverIdKHR.html) · Alias"]
#[doc(alias = "VkDriverIdKHR")]
#[allow(non_camel_case_types)]
pub type DriverIdKHR = crate::vk1_2::DriverId;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkConformanceVersionKHR.html) · Alias"]
#[doc(alias = "VkConformanceVersionKHR")]
#[allow(non_camel_case_types)]
pub type ConformanceVersionKHR = crate::vk1_2::ConformanceVersion;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkConformanceVersionKHR.html) · Alias"]
#[doc(alias = "VkConformanceVersionKHR")]
#[allow(non_camel_case_types)]
pub type ConformanceVersionKHRBuilder<'a> = crate::vk1_2::ConformanceVersionBuilder<'a>;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDriverPropertiesKHR.html) · Alias"]
#[doc(alias = "VkPhysicalDeviceDriverPropertiesKHR")]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceDriverPropertiesKHR = crate::vk1_2::PhysicalDeviceDriverProperties;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDriverPropertiesKHR.html) · Alias"]
#[doc(alias = "VkPhysicalDeviceDriverPropertiesKHR")]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceDriverPropertiesKHRBuilder<'a> =
    crate::vk1_2::PhysicalDeviceDriverPropertiesBuilder<'a>;
