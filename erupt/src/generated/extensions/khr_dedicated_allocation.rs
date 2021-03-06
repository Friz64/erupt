#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_DEDICATED_ALLOCATION_SPEC_VERSION")]
pub const KHR_DEDICATED_ALLOCATION_SPEC_VERSION: u32 = 3;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_DEDICATED_ALLOCATION_EXTENSION_NAME")]
pub const KHR_DEDICATED_ALLOCATION_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_dedicated_allocation");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryDedicatedRequirementsKHR.html) · Alias"]
#[doc(alias = "VkMemoryDedicatedRequirementsKHR")]
#[allow(non_camel_case_types)]
pub type MemoryDedicatedRequirementsKHR = crate::vk1_1::MemoryDedicatedRequirements;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryDedicatedRequirementsKHR.html) · Alias"]
#[doc(alias = "VkMemoryDedicatedRequirementsKHR")]
#[allow(non_camel_case_types)]
pub type MemoryDedicatedRequirementsKHRBuilder<'a> = crate::vk1_1::MemoryDedicatedRequirementsBuilder<'a>;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryDedicatedAllocateInfoKHR.html) · Alias"]
#[doc(alias = "VkMemoryDedicatedAllocateInfoKHR")]
#[allow(non_camel_case_types)]
pub type MemoryDedicatedAllocateInfoKHR = crate::vk1_1::MemoryDedicatedAllocateInfo;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryDedicatedAllocateInfoKHR.html) · Alias"]
#[doc(alias = "VkMemoryDedicatedAllocateInfoKHR")]
#[allow(non_camel_case_types)]
pub type MemoryDedicatedAllocateInfoKHRBuilder<'a> = crate::vk1_1::MemoryDedicatedAllocateInfoBuilder<'a>;
#[doc = "Provided by [`crate::extensions::khr_dedicated_allocation`]"]
impl crate::vk1_0::StructureType {
    pub const MEMORY_DEDICATED_REQUIREMENTS_KHR: Self = Self::MEMORY_DEDICATED_REQUIREMENTS;
    pub const MEMORY_DEDICATED_ALLOCATE_INFO_KHR: Self = Self::MEMORY_DEDICATED_ALLOCATE_INFO;
}
