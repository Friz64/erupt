# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_KHR_external_semaphore.html)\n\n## Extends\n- [`SemaphoreImportFlagBits`](../../vk1_1/struct.SemaphoreImportFlagBits.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_EXTERNAL_SEMAPHORE_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_EXTERNAL_SEMAPHORE_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_external_semaphore");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreImportFlagsKHR.html) · Alias"]
pub type SemaphoreImportFlagsKHR = crate::vk1_1::SemaphoreImportFlags;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExportSemaphoreCreateInfoKHR.html) · Alias"]
pub type ExportSemaphoreCreateInfoKHR = crate::vk1_1::ExportSemaphoreCreateInfo;
