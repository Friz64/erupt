# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_KHR_driver_properties.html)\n\n## Extends\n- [`DriverId`](../../vk1_2/struct.DriverId.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_DRIVER_PROPERTIES_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_DRIVER_PROPERTIES_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_driver_properties");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const MAX_DRIVER_NAME_SIZE_KHR: u32 = 256;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const MAX_DRIVER_INFO_SIZE_KHR: u32 = 256;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDriverIdKHR.html) · Alias"]
pub type DriverIdKHR = crate::vk1_2::DriverId;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkConformanceVersionKHR.html) · Alias"]
pub type ConformanceVersionKHR = crate::vk1_2::ConformanceVersion;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDriverPropertiesKHR.html) · Alias"]
pub type PhysicalDeviceDriverPropertiesKHR = crate::vk1_2::PhysicalDeviceDriverProperties;
