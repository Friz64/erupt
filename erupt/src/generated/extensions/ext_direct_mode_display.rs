#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_DIRECT_MODE_DISPLAY_SPEC_VERSION")]
pub const EXT_DIRECT_MODE_DISPLAY_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_DIRECT_MODE_DISPLAY_EXTENSION_NAME")]
pub const EXT_DIRECT_MODE_DISPLAY_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_direct_mode_display");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_RELEASE_DISPLAY_EXT: *const std::os::raw::c_char = crate::cstr!("vkReleaseDisplayEXT");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkReleaseDisplayEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkReleaseDisplayEXT = unsafe extern "system" fn(physical_device: crate::vk1_0::PhysicalDevice, display: crate::extensions::khr_display::DisplayKHR) -> crate::vk1_0::Result;
#[doc = "Provided by [`crate::extensions::ext_direct_mode_display`]"]
impl crate::InstanceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkReleaseDisplayEXT.html) · Function"]
    #[doc(alias = "vkReleaseDisplayEXT")]
    pub unsafe fn release_display_ext(&self, physical_device: crate::vk1_0::PhysicalDevice, display: crate::extensions::khr_display::DisplayKHR) -> crate::utils::VulkanResult<()> {
        let _function = self.release_display_ext.expect("tried to call a function that isn't loaded");
        let _return = _function(physical_device as _, display as _);
        crate::utils::VulkanResult::new(_return, ())
    }
}
