# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_EXT_acquire_xlib_display.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_ACQUIRE_XLIB_DISPLAY_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_ACQUIRE_XLIB_DISPLAY_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_acquire_xlib_display");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAcquireXlibDisplayEXT.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkAcquireXlibDisplayEXT = unsafe extern "system" fn(
    physical_device: crate::vk1_0::PhysicalDevice,
    dpy: *mut *const std::ffi::c_void,
    display: crate::extensions::khr_display::DisplayKHR,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetRandROutputDisplayEXT.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetRandROutputDisplayEXT = unsafe extern "system" fn(
    physical_device: crate::vk1_0::PhysicalDevice,
    dpy: *mut *const std::ffi::c_void,
    rr_output: std::os::raw::c_ulong,
    p_display: *mut crate::extensions::khr_display::DisplayKHR,
) -> crate::vk1_0::Result;
#[doc = "Provides Instance Commands for [`ExtAcquireXlibDisplayInstanceLoaderExt`](trait.ExtAcquireXlibDisplayInstanceLoaderExt.html)"]
pub struct ExtAcquireXlibDisplayInstanceCommands {
    pub acquire_xlib_display_ext: PFN_vkAcquireXlibDisplayEXT,
    pub get_rand_r_output_display_ext: PFN_vkGetRandROutputDisplayEXT,
}
impl ExtAcquireXlibDisplayInstanceCommands {
    #[inline]
    pub fn load(loader: &crate::InstanceLoader) -> Option<ExtAcquireXlibDisplayInstanceCommands> {
        unsafe {
            Some(ExtAcquireXlibDisplayInstanceCommands {
                acquire_xlib_display_ext: std::mem::transmute(
                    loader.symbol("vkAcquireXlibDisplayEXT")?,
                ),
                get_rand_r_output_display_ext: std::mem::transmute(
                    loader.symbol("vkGetRandROutputDisplayEXT")?,
                ),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`ExtAcquireXlibDisplayInstanceCommands`](struct.ExtAcquireXlibDisplayInstanceCommands.html)"]
pub trait ExtAcquireXlibDisplayInstanceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAcquireXlibDisplayEXT.html) · Instance Command"]
    unsafe fn acquire_xlib_display_ext(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        dpy: *mut *const std::ffi::c_void,
        display: crate::extensions::khr_display::DisplayKHR,
    ) -> crate::utils::VulkanResult<()>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetRandROutputDisplayEXT.html) · Instance Command"]
    unsafe fn get_rand_r_output_display_ext(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        dpy: *mut *const std::ffi::c_void,
        rr_output: std::os::raw::c_ulong,
        display: Option<crate::extensions::khr_display::DisplayKHR>,
    ) -> crate::utils::VulkanResult<crate::extensions::khr_display::DisplayKHR>;
}
impl ExtAcquireXlibDisplayInstanceLoaderExt for crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAcquireXlibDisplayEXT.html) · Instance Command"]
    unsafe fn acquire_xlib_display_ext(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        dpy: *mut *const std::ffi::c_void,
        display: crate::extensions::khr_display::DisplayKHR,
    ) -> crate::utils::VulkanResult<()> {
        let function = self
            .ext_acquire_xlib_display
            .as_ref()
            .expect("`ext_acquire_xlib_display` not loaded")
            .acquire_xlib_display_ext;
        let _val = function(physical_device, dpy, display);
        crate::utils::VulkanResult::new(_val, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetRandROutputDisplayEXT.html) · Instance Command"]
    unsafe fn get_rand_r_output_display_ext(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        dpy: *mut *const std::ffi::c_void,
        rr_output: std::os::raw::c_ulong,
        display: Option<crate::extensions::khr_display::DisplayKHR>,
    ) -> crate::utils::VulkanResult<crate::extensions::khr_display::DisplayKHR> {
        let function = self
            .ext_acquire_xlib_display
            .as_ref()
            .expect("`ext_acquire_xlib_display` not loaded")
            .get_rand_r_output_display_ext;
        let mut display = display.unwrap_or_else(|| Default::default());
        let _val = function(physical_device, dpy, rr_output, &mut display);
        crate::utils::VulkanResult::new(_val, display)
    }
}
