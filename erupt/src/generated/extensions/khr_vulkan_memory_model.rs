// DO NOT EDIT: @generated by erupt's generator
///<s>Vulkan Manual Page</s> · Constant
#[doc(alias = "VK_KHR_VULKAN_MEMORY_MODEL_SPEC_VERSION")]
pub const KHR_VULKAN_MEMORY_MODEL_SPEC_VERSION: u32 = 3;
///<s>Vulkan Manual Page</s> · Constant
#[doc(alias = "VK_KHR_VULKAN_MEMORY_MODEL_EXTENSION_NAME")]
pub const KHR_VULKAN_MEMORY_MODEL_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!(
    "VK_KHR_vulkan_memory_model"
);
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVulkanMemoryModelFeaturesKHR.html) · Alias
#[doc(alias = "VkPhysicalDeviceVulkanMemoryModelFeaturesKHR")]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceVulkanMemoryModelFeaturesKHR = crate::vk1_2::PhysicalDeviceVulkanMemoryModelFeatures;
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVulkanMemoryModelFeaturesKHR.html) · Alias
#[doc(alias = "VkPhysicalDeviceVulkanMemoryModelFeaturesKHR")]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceVulkanMemoryModelFeaturesKHRBuilder<'a> = crate::vk1_2::PhysicalDeviceVulkanMemoryModelFeaturesBuilder<
    'a,
>;
///Provided by [`crate::extensions::khr_vulkan_memory_model`]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES_KHR: Self = Self::PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES;
}
