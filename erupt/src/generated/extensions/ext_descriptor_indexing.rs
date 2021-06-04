#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_DESCRIPTOR_INDEXING_SPEC_VERSION")]
pub const EXT_DESCRIPTOR_INDEXING_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_DESCRIPTOR_INDEXING_EXTENSION_NAME")]
pub const EXT_DESCRIPTOR_INDEXING_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_descriptor_indexing");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorBindingFlagsEXT.html) · Alias"]
#[doc(alias = "VkDescriptorBindingFlagsEXT")]
#[allow(non_camel_case_types)]
pub type DescriptorBindingFlagsEXT = crate::vk1_2::DescriptorBindingFlags;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorBindingFlagBitsEXT.html) · Alias"]
#[doc(alias = "VkDescriptorBindingFlagBitsEXT")]
#[allow(non_camel_case_types)]
pub type DescriptorBindingFlagBitsEXT = crate::vk1_2::DescriptorBindingFlagBits;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDescriptorIndexingFeaturesEXT.html) · Alias"]
#[doc(alias = "VkPhysicalDeviceDescriptorIndexingFeaturesEXT")]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceDescriptorIndexingFeaturesEXT = crate::vk1_2::PhysicalDeviceDescriptorIndexingFeatures;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDescriptorIndexingFeaturesEXT.html) · Alias"]
#[doc(alias = "VkPhysicalDeviceDescriptorIndexingFeaturesEXT")]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceDescriptorIndexingFeaturesEXTBuilder<'a> = crate::vk1_2::PhysicalDeviceDescriptorIndexingFeaturesBuilder<'a>;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDescriptorIndexingPropertiesEXT.html) · Alias"]
#[doc(alias = "VkPhysicalDeviceDescriptorIndexingPropertiesEXT")]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceDescriptorIndexingPropertiesEXT = crate::vk1_2::PhysicalDeviceDescriptorIndexingProperties;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDescriptorIndexingPropertiesEXT.html) · Alias"]
#[doc(alias = "VkPhysicalDeviceDescriptorIndexingPropertiesEXT")]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceDescriptorIndexingPropertiesEXTBuilder<'a> = crate::vk1_2::PhysicalDeviceDescriptorIndexingPropertiesBuilder<'a>;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorSetLayoutBindingFlagsCreateInfoEXT.html) · Alias"]
#[doc(alias = "VkDescriptorSetLayoutBindingFlagsCreateInfoEXT")]
#[allow(non_camel_case_types)]
pub type DescriptorSetLayoutBindingFlagsCreateInfoEXT = crate::vk1_2::DescriptorSetLayoutBindingFlagsCreateInfo;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorSetLayoutBindingFlagsCreateInfoEXT.html) · Alias"]
#[doc(alias = "VkDescriptorSetLayoutBindingFlagsCreateInfoEXT")]
#[allow(non_camel_case_types)]
pub type DescriptorSetLayoutBindingFlagsCreateInfoEXTBuilder<'a> = crate::vk1_2::DescriptorSetLayoutBindingFlagsCreateInfoBuilder<'a>;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorSetVariableDescriptorCountAllocateInfoEXT.html) · Alias"]
#[doc(alias = "VkDescriptorSetVariableDescriptorCountAllocateInfoEXT")]
#[allow(non_camel_case_types)]
pub type DescriptorSetVariableDescriptorCountAllocateInfoEXT = crate::vk1_2::DescriptorSetVariableDescriptorCountAllocateInfo;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorSetVariableDescriptorCountAllocateInfoEXT.html) · Alias"]
#[doc(alias = "VkDescriptorSetVariableDescriptorCountAllocateInfoEXT")]
#[allow(non_camel_case_types)]
pub type DescriptorSetVariableDescriptorCountAllocateInfoEXTBuilder<'a> = crate::vk1_2::DescriptorSetVariableDescriptorCountAllocateInfoBuilder<'a>;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorSetVariableDescriptorCountLayoutSupportEXT.html) · Alias"]
#[doc(alias = "VkDescriptorSetVariableDescriptorCountLayoutSupportEXT")]
#[allow(non_camel_case_types)]
pub type DescriptorSetVariableDescriptorCountLayoutSupportEXT = crate::vk1_2::DescriptorSetVariableDescriptorCountLayoutSupport;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorSetVariableDescriptorCountLayoutSupportEXT.html) · Alias"]
#[doc(alias = "VkDescriptorSetVariableDescriptorCountLayoutSupportEXT")]
#[allow(non_camel_case_types)]
pub type DescriptorSetVariableDescriptorCountLayoutSupportEXTBuilder<'a> = crate::vk1_2::DescriptorSetVariableDescriptorCountLayoutSupportBuilder<'a>;
#[doc = "Provided by [`crate::extensions::ext_descriptor_indexing`]"]
impl crate::vk1_0::DescriptorSetLayoutCreateFlagBits {
    pub const UPDATE_AFTER_BIND_POOL_EXT: Self = Self::UPDATE_AFTER_BIND_POOL;
}
#[doc = "Provided by [`crate::extensions::ext_descriptor_indexing`]"]
impl crate::vk1_0::Result {
    pub const ERROR_FRAGMENTATION_EXT: Self = Self::ERROR_FRAGMENTATION;
}
#[doc = "Provided by [`crate::extensions::ext_descriptor_indexing`]"]
impl crate::vk1_0::StructureType {
    pub const DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO_EXT: Self = Self::DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO;
    pub const PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES_EXT: Self = Self::PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES;
    pub const PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES_EXT: Self = Self::PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES;
    pub const DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO_EXT: Self = Self::DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO;
    pub const DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT_EXT: Self = Self::DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT;
}
#[doc = "Provided by [`crate::extensions::ext_descriptor_indexing`]"]
impl crate::vk1_0::DescriptorPoolCreateFlagBits {
    pub const UPDATE_AFTER_BIND_EXT: Self = Self::UPDATE_AFTER_BIND;
}
#[doc = "Provided by [`crate::extensions::ext_descriptor_indexing`]"]
impl crate::vk1_2::DescriptorBindingFlagBits {
    pub const UPDATE_AFTER_BIND_EXT: Self = Self::UPDATE_AFTER_BIND;
    pub const UPDATE_UNUSED_WHILE_PENDING_EXT: Self = Self::UPDATE_UNUSED_WHILE_PENDING;
    pub const PARTIALLY_BOUND_EXT: Self = Self::PARTIALLY_BOUND;
    pub const VARIABLE_DESCRIPTOR_COUNT_EXT: Self = Self::VARIABLE_DESCRIPTOR_COUNT;
}
