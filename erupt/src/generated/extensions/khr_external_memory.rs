# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_KHR_external_memory.html)\n\n## Extends\n- [`Result`](../../vk1_0/struct.Result.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_EXTERNAL_MEMORY_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_EXTERNAL_MEMORY_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_external_memory");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const QUEUE_FAMILY_EXTERNAL_KHR: u32 = !0u32 - 1;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalMemoryImageCreateInfoKHR.html) · Alias"]
pub type ExternalMemoryImageCreateInfoKHR = crate::vk1_1::ExternalMemoryImageCreateInfo;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalMemoryBufferCreateInfoKHR.html) · Alias"]
pub type ExternalMemoryBufferCreateInfoKHR = crate::vk1_1::ExternalMemoryBufferCreateInfo;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExportMemoryAllocateInfoKHR.html) · Alias"]
pub type ExportMemoryAllocateInfoKHR = crate::vk1_1::ExportMemoryAllocateInfo;
