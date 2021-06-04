#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_SAMPLER_FILTER_MINMAX_SPEC_VERSION")]
pub const EXT_SAMPLER_FILTER_MINMAX_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_SAMPLER_FILTER_MINMAX_EXTENSION_NAME")]
pub const EXT_SAMPLER_FILTER_MINMAX_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_sampler_filter_minmax");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerReductionModeEXT.html) · Alias"]
#[doc(alias = "VkSamplerReductionModeEXT")]
#[allow(non_camel_case_types)]
pub type SamplerReductionModeEXT = crate::vk1_2::SamplerReductionMode;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT.html) · Alias"]
#[doc(alias = "VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT")]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceSamplerFilterMinmaxPropertiesEXT = crate::vk1_2::PhysicalDeviceSamplerFilterMinmaxProperties;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT.html) · Alias"]
#[doc(alias = "VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT")]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceSamplerFilterMinmaxPropertiesEXTBuilder<'a> = crate::vk1_2::PhysicalDeviceSamplerFilterMinmaxPropertiesBuilder<'a>;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerReductionModeCreateInfoEXT.html) · Alias"]
#[doc(alias = "VkSamplerReductionModeCreateInfoEXT")]
#[allow(non_camel_case_types)]
pub type SamplerReductionModeCreateInfoEXT = crate::vk1_2::SamplerReductionModeCreateInfo;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerReductionModeCreateInfoEXT.html) · Alias"]
#[doc(alias = "VkSamplerReductionModeCreateInfoEXT")]
#[allow(non_camel_case_types)]
pub type SamplerReductionModeCreateInfoEXTBuilder<'a> = crate::vk1_2::SamplerReductionModeCreateInfoBuilder<'a>;
#[doc = "Provided by [`crate::extensions::ext_sampler_filter_minmax`]"]
impl crate::vk1_0::FormatFeatureFlagBits {
    pub const SAMPLED_IMAGE_FILTER_MINMAX_EXT: Self = Self::SAMPLED_IMAGE_FILTER_MINMAX;
}
#[doc = "Provided by [`crate::extensions::ext_sampler_filter_minmax`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES_EXT: Self = Self::PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES;
    pub const SAMPLER_REDUCTION_MODE_CREATE_INFO_EXT: Self = Self::SAMPLER_REDUCTION_MODE_CREATE_INFO;
}
#[doc = "Provided by [`crate::extensions::ext_sampler_filter_minmax`]"]
impl crate::vk1_2::SamplerReductionMode {
    pub const WEIGHTED_AVERAGE_EXT: Self = Self::WEIGHTED_AVERAGE;
    pub const MIN_EXT: Self = Self::MIN;
    pub const MAX_EXT: Self = Self::MAX;
}
