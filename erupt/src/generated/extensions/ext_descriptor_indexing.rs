# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_EXT_descriptor_indexing.html)\n\n## Extends\n- [`DescriptorBindingFlagBits`](../../vk1_2/struct.DescriptorBindingFlagBits.html)\n- [`DescriptorPoolCreateFlagBits`](../../vk1_0/struct.DescriptorPoolCreateFlagBits.html)\n- [`DescriptorSetLayoutCreateFlagBits`](../../vk1_0/struct.DescriptorSetLayoutCreateFlagBits.html)\n- [`Result`](../../vk1_0/struct.Result.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_DESCRIPTOR_INDEXING_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_DESCRIPTOR_INDEXING_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_descriptor_indexing");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorSetLayoutBindingFlagsCreateInfoEXT.html) · Alias"]
pub type DescriptorSetLayoutBindingFlagsCreateInfoEXT =
    crate::vk1_2::DescriptorSetLayoutBindingFlagsCreateInfo;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDescriptorIndexingFeaturesEXT.html) · Alias"]
pub type PhysicalDeviceDescriptorIndexingFeaturesEXT =
    crate::vk1_2::PhysicalDeviceDescriptorIndexingFeatures;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDescriptorIndexingPropertiesEXT.html) · Alias"]
pub type PhysicalDeviceDescriptorIndexingPropertiesEXT =
    crate::vk1_2::PhysicalDeviceDescriptorIndexingProperties;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorSetVariableDescriptorCountAllocateInfoEXT.html) · Alias"]
pub type DescriptorSetVariableDescriptorCountAllocateInfoEXT =
    crate::vk1_2::DescriptorSetVariableDescriptorCountAllocateInfo;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorSetVariableDescriptorCountLayoutSupportEXT.html) · Alias"]
pub type DescriptorSetVariableDescriptorCountLayoutSupportEXT =
    crate::vk1_2::DescriptorSetVariableDescriptorCountLayoutSupport;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorBindingFlagsEXT.html) · Alias"]
pub type DescriptorBindingFlagsEXT = crate::vk1_2::DescriptorBindingFlags;
