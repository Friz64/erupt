#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_SWAPCHAIN_MUTABLE_FORMAT_SPEC_VERSION")]
pub const KHR_SWAPCHAIN_MUTABLE_FORMAT_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_SWAPCHAIN_MUTABLE_FORMAT_EXTENSION_NAME")]
pub const KHR_SWAPCHAIN_MUTABLE_FORMAT_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_swapchain_mutable_format");
#[doc = "Provided by [`crate::extensions::khr_swapchain_mutable_format`]"]
impl crate::extensions::khr_swapchain::SwapchainCreateFlagBitsKHR {
    pub const MUTABLE_FORMAT_KHR: Self = Self(4);
}
