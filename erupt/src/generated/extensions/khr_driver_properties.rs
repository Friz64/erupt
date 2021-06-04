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
pub const KHR_DRIVER_PROPERTIES_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_driver_properties");
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
pub type PhysicalDeviceDriverPropertiesKHRBuilder<'a> = crate::vk1_2::PhysicalDeviceDriverPropertiesBuilder<'a>;
#[doc = "Provided by [`crate::extensions::khr_driver_properties`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_DRIVER_PROPERTIES_KHR: Self = Self::PHYSICAL_DEVICE_DRIVER_PROPERTIES;
}
#[doc = "Provided by [`crate::extensions::khr_driver_properties`]"]
impl crate::vk1_2::DriverId {
    pub const AMD_PROPRIETARY_KHR: Self = Self::AMD_PROPRIETARY;
    pub const AMD_OPEN_SOURCE_KHR: Self = Self::AMD_OPEN_SOURCE;
    pub const MESA_RADV_KHR: Self = Self::MESA_RADV;
    pub const NVIDIA_PROPRIETARY_KHR: Self = Self::NVIDIA_PROPRIETARY;
    pub const INTEL_PROPRIETARY_WINDOWS_KHR: Self = Self::INTEL_PROPRIETARY_WINDOWS;
    pub const INTEL_OPEN_SOURCE_MESA_KHR: Self = Self::INTEL_OPEN_SOURCE_MESA;
    pub const IMAGINATION_PROPRIETARY_KHR: Self = Self::IMAGINATION_PROPRIETARY;
    pub const QUALCOMM_PROPRIETARY_KHR: Self = Self::QUALCOMM_PROPRIETARY;
    pub const ARM_PROPRIETARY_KHR: Self = Self::ARM_PROPRIETARY;
    pub const GOOGLE_SWIFTSHADER_KHR: Self = Self::GOOGLE_SWIFTSHADER;
    pub const GGP_PROPRIETARY_KHR: Self = Self::GGP_PROPRIETARY;
    pub const BROADCOM_PROPRIETARY_KHR: Self = Self::BROADCOM_PROPRIETARY;
}
