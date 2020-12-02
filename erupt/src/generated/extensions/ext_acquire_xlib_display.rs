#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_ACQUIRE_XLIB_DISPLAY_SPEC_VERSION")]
pub const EXT_ACQUIRE_XLIB_DISPLAY_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_ACQUIRE_XLIB_DISPLAY_EXTENSION_NAME")]
pub const EXT_ACQUIRE_XLIB_DISPLAY_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_acquire_xlib_display");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "FN_ACQUIRE_XLIB_DISPLAY_EXT")]
pub const FN_ACQUIRE_XLIB_DISPLAY_EXT: *const std::os::raw::c_char =
    crate::cstr!("vkAcquireXlibDisplayEXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "FN_GET_RAND_R_OUTPUT_DISPLAY_EXT")]
pub const FN_GET_RAND_R_OUTPUT_DISPLAY_EXT: *const std::os::raw::c_char =
    crate::cstr!("vkGetRandROutputDisplayEXT");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAcquireXlibDisplayEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkAcquireXlibDisplayEXT = unsafe extern "system" fn(
    physical_device: crate::vk1_0::PhysicalDevice,
    dpy: *mut std::ffi::c_void,
    display: crate::extensions::khr_display::DisplayKHR,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetRandROutputDisplayEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetRandROutputDisplayEXT = unsafe extern "system" fn(
    physical_device: crate::vk1_0::PhysicalDevice,
    dpy: *mut std::ffi::c_void,
    rr_output: u64,
    p_display: *mut crate::extensions::khr_display::DisplayKHR,
) -> crate::vk1_0::Result;
#[doc = "Provided by [`extensions::ext_acquire_xlib_display`](extensions/ext_acquire_xlib_display/index.html)"]
impl crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAcquireXlibDisplayEXT.html) · Function"]
    #[doc(alias = "vkAcquireXlibDisplayEXT")]
    pub unsafe fn acquire_xlib_display_ext(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        dpy: *mut std::ffi::c_void,
        display: crate::extensions::khr_display::DisplayKHR,
    ) -> crate::utils::VulkanResult<()> {
        let _function = self
            .acquire_xlib_display_ext
            .expect("`acquire_xlib_display_ext` is not loaded");
        let _return = _function(physical_device as _, dpy, display as _);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetRandROutputDisplayEXT.html) · Function"]
    #[doc(alias = "vkGetRandROutputDisplayEXT")]
    pub unsafe fn get_rand_r_output_display_ext(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        dpy: *mut std::ffi::c_void,
        rr_output: u64,
        display: Option<crate::extensions::khr_display::DisplayKHR>,
    ) -> crate::utils::VulkanResult<crate::extensions::khr_display::DisplayKHR> {
        let _function = self
            .get_rand_r_output_display_ext
            .expect("`get_rand_r_output_display_ext` is not loaded");
        let mut display = match display {
            Some(v) => v,
            None => Default::default(),
        };
        let _return = _function(physical_device as _, dpy, rr_output as _, &mut display);
        crate::utils::VulkanResult::new(_return, display)
    }
}
