# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_KHR_external_fence.html)\n\n## Extends\n- [`FenceImportFlagBits`](../../vk1_1/struct.FenceImportFlagBits.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_EXTERNAL_FENCE_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_EXTERNAL_FENCE_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_external_fence");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFenceImportFlagsKHR.html) · Alias"]
pub type FenceImportFlagsKHR = crate::vk1_1::FenceImportFlags;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExportFenceCreateInfoKHR.html) · Alias"]
pub type ExportFenceCreateInfoKHR = crate::vk1_1::ExportFenceCreateInfo;
