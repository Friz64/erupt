#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_EXTERNAL_MEMORY_DMA_BUF_SPEC_VERSION")]
pub const EXT_EXTERNAL_MEMORY_DMA_BUF_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_EXTERNAL_MEMORY_DMA_BUF_EXTENSION_NAME")]
pub const EXT_EXTERNAL_MEMORY_DMA_BUF_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_external_memory_dma_buf");
#[doc = "Provided by [`crate::extensions::ext_external_memory_dma_buf`]"]
impl crate::vk1_1::ExternalMemoryHandleTypeFlagBits {
    pub const DMA_BUF_EXT: Self = Self(512);
}
