// DO NOT EDIT: @generated by erupt's generator
///<s>Vulkan Manual Page</s> · Constant
#[doc(alias = "VK_QUEUE_FAMILY_EXTERNAL_KHR")]
pub const QUEUE_FAMILY_EXTERNAL_KHR: u32 = 4294967294;
///<s>Vulkan Manual Page</s> · Constant
#[doc(alias = "VK_KHR_EXTERNAL_MEMORY_SPEC_VERSION")]
pub const KHR_EXTERNAL_MEMORY_SPEC_VERSION: u32 = 1;
///<s>Vulkan Manual Page</s> · Constant
#[doc(alias = "VK_KHR_EXTERNAL_MEMORY_EXTENSION_NAME")]
pub const KHR_EXTERNAL_MEMORY_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!(
    "VK_KHR_external_memory"
);
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryImageCreateInfoKHR.html) · Alias
#[doc(alias = "VkExternalMemoryImageCreateInfoKHR")]
#[allow(non_camel_case_types)]
pub type ExternalMemoryImageCreateInfoKHR = crate::vk1_1::ExternalMemoryImageCreateInfo;
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryImageCreateInfoKHR.html) · Alias
#[doc(alias = "VkExternalMemoryImageCreateInfoKHR")]
#[allow(non_camel_case_types)]
pub type ExternalMemoryImageCreateInfoKHRBuilder<'a> = crate::vk1_1::ExternalMemoryImageCreateInfoBuilder<
    'a,
>;
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryBufferCreateInfoKHR.html) · Alias
#[doc(alias = "VkExternalMemoryBufferCreateInfoKHR")]
#[allow(non_camel_case_types)]
pub type ExternalMemoryBufferCreateInfoKHR = crate::vk1_1::ExternalMemoryBufferCreateInfo;
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryBufferCreateInfoKHR.html) · Alias
#[doc(alias = "VkExternalMemoryBufferCreateInfoKHR")]
#[allow(non_camel_case_types)]
pub type ExternalMemoryBufferCreateInfoKHRBuilder<'a> = crate::vk1_1::ExternalMemoryBufferCreateInfoBuilder<
    'a,
>;
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExportMemoryAllocateInfoKHR.html) · Alias
#[doc(alias = "VkExportMemoryAllocateInfoKHR")]
#[allow(non_camel_case_types)]
pub type ExportMemoryAllocateInfoKHR = crate::vk1_1::ExportMemoryAllocateInfo;
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExportMemoryAllocateInfoKHR.html) · Alias
#[doc(alias = "VkExportMemoryAllocateInfoKHR")]
#[allow(non_camel_case_types)]
pub type ExportMemoryAllocateInfoKHRBuilder<'a> = crate::vk1_1::ExportMemoryAllocateInfoBuilder<
    'a,
>;
///Provided by [`crate::extensions::khr_external_memory`]
impl crate::vk1_0::Result {
    pub const ERROR_INVALID_EXTERNAL_HANDLE_KHR: Self = Self::ERROR_INVALID_EXTERNAL_HANDLE;
}
///Provided by [`crate::extensions::khr_external_memory`]
impl crate::vk1_0::StructureType {
    pub const EXTERNAL_MEMORY_BUFFER_CREATE_INFO_KHR: Self = Self::EXTERNAL_MEMORY_BUFFER_CREATE_INFO;
    pub const EXTERNAL_MEMORY_IMAGE_CREATE_INFO_KHR: Self = Self::EXTERNAL_MEMORY_IMAGE_CREATE_INFO;
    pub const EXPORT_MEMORY_ALLOCATE_INFO_KHR: Self = Self::EXPORT_MEMORY_ALLOCATE_INFO;
}
