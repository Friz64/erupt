// DO NOT EDIT: @generated by erupt's generator
///<s>Vulkan Manual Page</s> · Constant
#[doc(alias = "VK_KHR_16BIT_STORAGE_SPEC_VERSION")]
pub const KHR_16BIT_STORAGE_SPEC_VERSION: u32 = 1;
///<s>Vulkan Manual Page</s> · Constant
#[doc(alias = "VK_KHR_16BIT_STORAGE_EXTENSION_NAME")]
pub const KHR_16BIT_STORAGE_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!(
    "VK_KHR_16bit_storage"
);
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevice16BitStorageFeaturesKHR.html) · Alias
#[doc(alias = "VkPhysicalDevice16BitStorageFeaturesKHR")]
#[allow(non_camel_case_types)]
pub type PhysicalDevice16BitStorageFeaturesKHR = crate::vk1_1::PhysicalDevice16BitStorageFeatures;
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevice16BitStorageFeaturesKHR.html) · Alias
#[doc(alias = "VkPhysicalDevice16BitStorageFeaturesKHR")]
#[allow(non_camel_case_types)]
pub type PhysicalDevice16BitStorageFeaturesKHRBuilder<'a> = crate::vk1_1::PhysicalDevice16BitStorageFeaturesBuilder<
    'a,
>;
///Provided by [`crate::extensions::khr_16bit_storage`]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES_KHR: Self = Self::PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES;
}
