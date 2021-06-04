#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_SCALAR_BLOCK_LAYOUT_SPEC_VERSION")]
pub const EXT_SCALAR_BLOCK_LAYOUT_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_SCALAR_BLOCK_LAYOUT_EXTENSION_NAME")]
pub const EXT_SCALAR_BLOCK_LAYOUT_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_scalar_block_layout");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceScalarBlockLayoutFeaturesEXT.html) · Alias"]
#[doc(alias = "VkPhysicalDeviceScalarBlockLayoutFeaturesEXT")]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceScalarBlockLayoutFeaturesEXT = crate::vk1_2::PhysicalDeviceScalarBlockLayoutFeatures;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceScalarBlockLayoutFeaturesEXT.html) · Alias"]
#[doc(alias = "VkPhysicalDeviceScalarBlockLayoutFeaturesEXT")]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceScalarBlockLayoutFeaturesEXTBuilder<'a> = crate::vk1_2::PhysicalDeviceScalarBlockLayoutFeaturesBuilder<'a>;
#[doc = "Provided by [`crate::extensions::ext_scalar_block_layout`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES_EXT: Self = Self::PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES;
}
