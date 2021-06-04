#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_VARIABLE_POINTERS_SPEC_VERSION")]
pub const KHR_VARIABLE_POINTERS_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_VARIABLE_POINTERS_EXTENSION_NAME")]
pub const KHR_VARIABLE_POINTERS_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_variable_pointers");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceVariablePointersFeaturesKHR.html) · Alias"]
#[doc(alias = "VkPhysicalDeviceVariablePointersFeaturesKHR")]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceVariablePointersFeaturesKHR = crate::vk1_1::PhysicalDeviceVariablePointersFeatures;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceVariablePointersFeaturesKHR.html) · Alias"]
#[doc(alias = "VkPhysicalDeviceVariablePointersFeaturesKHR")]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceVariablePointersFeaturesKHRBuilder<'a> = crate::vk1_1::PhysicalDeviceVariablePointersFeaturesBuilder<'a>;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceVariablePointerFeaturesKHR.html) · Alias"]
#[doc(alias = "VkPhysicalDeviceVariablePointerFeaturesKHR")]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceVariablePointerFeaturesKHR = crate::vk1_1::PhysicalDeviceVariablePointersFeatures;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceVariablePointerFeaturesKHR.html) · Alias"]
#[doc(alias = "VkPhysicalDeviceVariablePointerFeaturesKHR")]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceVariablePointerFeaturesKHRBuilder<'a> = crate::vk1_1::PhysicalDeviceVariablePointersFeaturesBuilder<'a>;
#[doc = "Provided by [`crate::extensions::khr_variable_pointers`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES_KHR: Self = Self::PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES;
    pub const PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES_KHR: Self = Self::PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES_KHR;
}
