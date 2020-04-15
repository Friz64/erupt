# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_EXT_sampler_filter_minmax.html)\n\n## Extends\n- [`FormatFeatureFlagBits`](../../vk1_0/struct.FormatFeatureFlagBits.html)\n- [`SamplerReductionMode`](../../vk1_2/struct.SamplerReductionMode.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_SAMPLER_FILTER_MINMAX_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_SAMPLER_FILTER_MINMAX_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_sampler_filter_minmax");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerReductionModeEXT.html) · Alias"]
pub type SamplerReductionModeEXT = crate::vk1_2::SamplerReductionMode;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerReductionModeCreateInfoEXT.html) · Alias"]
pub type SamplerReductionModeCreateInfoEXT = crate::vk1_2::SamplerReductionModeCreateInfo;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT.html) · Alias"]
pub type PhysicalDeviceSamplerFilterMinmaxPropertiesEXT =
    crate::vk1_2::PhysicalDeviceSamplerFilterMinmaxProperties;
