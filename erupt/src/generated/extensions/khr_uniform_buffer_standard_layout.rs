#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_UNIFORM_BUFFER_STANDARD_LAYOUT_SPEC_VERSION")]
pub const KHR_UNIFORM_BUFFER_STANDARD_LAYOUT_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_UNIFORM_BUFFER_STANDARD_LAYOUT_EXTENSION_NAME")]
pub const KHR_UNIFORM_BUFFER_STANDARD_LAYOUT_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_uniform_buffer_standard_layout");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceUniformBufferStandardLayoutFeaturesKHR.html) · Alias"]
#[doc(alias = "VkPhysicalDeviceUniformBufferStandardLayoutFeaturesKHR")]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceUniformBufferStandardLayoutFeaturesKHR = crate::vk1_2::PhysicalDeviceUniformBufferStandardLayoutFeatures;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceUniformBufferStandardLayoutFeaturesKHR.html) · Alias"]
#[doc(alias = "VkPhysicalDeviceUniformBufferStandardLayoutFeaturesKHR")]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceUniformBufferStandardLayoutFeaturesKHRBuilder<'a> = crate::vk1_2::PhysicalDeviceUniformBufferStandardLayoutFeaturesBuilder<'a>;
#[doc = "Provided by [`crate::extensions::khr_uniform_buffer_standard_layout`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES_KHR: Self = Self::PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES;
}
