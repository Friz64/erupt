// DO NOT EDIT: @generated by erupt's generator
///<s>Vulkan Manual Page</s> · Constant
#[doc(alias = "VK_MAX_GLOBAL_PRIORITY_SIZE_EXT")]
pub const MAX_GLOBAL_PRIORITY_SIZE_EXT: u32 = 16;
///<s>Vulkan Manual Page</s> · Constant
#[doc(alias = "VK_EXT_GLOBAL_PRIORITY_QUERY_SPEC_VERSION")]
pub const EXT_GLOBAL_PRIORITY_QUERY_SPEC_VERSION: u32 = 1;
///<s>Vulkan Manual Page</s> · Constant
#[doc(alias = "VK_EXT_GLOBAL_PRIORITY_QUERY_EXTENSION_NAME")]
pub const EXT_GLOBAL_PRIORITY_QUERY_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!(
    "VK_EXT_global_priority_query"
);
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceGlobalPriorityQueryFeaturesEXT.html) · Alias
#[doc(alias = "VkPhysicalDeviceGlobalPriorityQueryFeaturesEXT")]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceGlobalPriorityQueryFeaturesEXT = crate::extensions::khr_global_priority::PhysicalDeviceGlobalPriorityQueryFeaturesKHR;
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceGlobalPriorityQueryFeaturesEXT.html) · Alias
#[doc(alias = "VkPhysicalDeviceGlobalPriorityQueryFeaturesEXT")]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceGlobalPriorityQueryFeaturesEXTBuilder<'a> = crate::extensions::khr_global_priority::PhysicalDeviceGlobalPriorityQueryFeaturesKHRBuilder<
    'a,
>;
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueueFamilyGlobalPriorityPropertiesEXT.html) · Alias
#[doc(alias = "VkQueueFamilyGlobalPriorityPropertiesEXT")]
#[allow(non_camel_case_types)]
pub type QueueFamilyGlobalPriorityPropertiesEXT = crate::extensions::khr_global_priority::QueueFamilyGlobalPriorityPropertiesKHR;
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueueFamilyGlobalPriorityPropertiesEXT.html) · Alias
#[doc(alias = "VkQueueFamilyGlobalPriorityPropertiesEXT")]
#[allow(non_camel_case_types)]
pub type QueueFamilyGlobalPriorityPropertiesEXTBuilder<'a> = crate::extensions::khr_global_priority::QueueFamilyGlobalPriorityPropertiesKHRBuilder<
    'a,
>;
///Provided by [`crate::extensions::ext_global_priority_query`]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_EXT: Self = Self::PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_KHR;
    pub const QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_EXT: Self = Self::QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_KHR;
}
