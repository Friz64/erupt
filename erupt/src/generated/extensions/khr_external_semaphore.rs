#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_EXTERNAL_SEMAPHORE_SPEC_VERSION")]
pub const KHR_EXTERNAL_SEMAPHORE_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_EXTERNAL_SEMAPHORE_EXTENSION_NAME")]
pub const KHR_EXTERNAL_SEMAPHORE_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_external_semaphore");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreImportFlagsKHR.html) · Alias"]
#[doc(alias = "VkSemaphoreImportFlagsKHR")]
#[allow(non_camel_case_types)]
pub type SemaphoreImportFlagsKHR = crate::vk1_1::SemaphoreImportFlags;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreImportFlagBitsKHR.html) · Alias"]
#[doc(alias = "VkSemaphoreImportFlagBitsKHR")]
#[allow(non_camel_case_types)]
pub type SemaphoreImportFlagBitsKHR = crate::vk1_1::SemaphoreImportFlagBits;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExportSemaphoreCreateInfoKHR.html) · Alias"]
#[doc(alias = "VkExportSemaphoreCreateInfoKHR")]
#[allow(non_camel_case_types)]
pub type ExportSemaphoreCreateInfoKHR = crate::vk1_1::ExportSemaphoreCreateInfo;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExportSemaphoreCreateInfoKHR.html) · Alias"]
#[doc(alias = "VkExportSemaphoreCreateInfoKHR")]
#[allow(non_camel_case_types)]
pub type ExportSemaphoreCreateInfoKHRBuilder<'a> = crate::vk1_1::ExportSemaphoreCreateInfoBuilder<'a>;
#[doc = "Provided by [`crate::extensions::khr_external_semaphore`]"]
impl crate::vk1_0::StructureType {
    pub const EXPORT_SEMAPHORE_CREATE_INFO_KHR: Self = Self::EXPORT_SEMAPHORE_CREATE_INFO;
}
#[doc = "Provided by [`crate::extensions::khr_external_semaphore`]"]
impl crate::vk1_1::SemaphoreImportFlagBits {
    pub const TEMPORARY_KHR: Self = Self::TEMPORARY;
}
