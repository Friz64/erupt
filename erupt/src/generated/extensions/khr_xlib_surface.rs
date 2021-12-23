#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_XLIB_SURFACE_SPEC_VERSION")]
pub const KHR_XLIB_SURFACE_SPEC_VERSION: u32 = 6;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_XLIB_SURFACE_EXTENSION_NAME")]
pub const KHR_XLIB_SURFACE_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_xlib_surface");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_XLIB_SURFACE_KHR: *const std::os::raw::c_char = crate::cstr!("vkCreateXlibSurfaceKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_PHYSICAL_DEVICE_XLIB_PRESENTATION_SUPPORT_KHR: *const std::os::raw::c_char = crate::cstr!("vkGetPhysicalDeviceXlibPresentationSupportKHR");
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkXlibSurfaceCreateFlagsKHR.html) · Bitmask of [`XlibSurfaceCreateFlagBitsKHR`]"] # [doc (alias = "VkXlibSurfaceCreateFlagsKHR")] # [derive (Default)] # [repr (transparent)] pub struct XlibSurfaceCreateFlagsKHR : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`XlibSurfaceCreateFlagsKHR`]"]
#[doc(alias = "VkXlibSurfaceCreateFlagBitsKHR")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct XlibSurfaceCreateFlagBitsKHR(pub u32);
impl XlibSurfaceCreateFlagBitsKHR {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> XlibSurfaceCreateFlagsKHR {
        XlibSurfaceCreateFlagsKHR::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for XlibSurfaceCreateFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::khr_xlib_surface`]"]
impl crate::vk1_0::StructureType {
    pub const XLIB_SURFACE_CREATE_INFO_KHR: Self = Self(1000004000);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateXlibSurfaceKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateXlibSurfaceKHR = unsafe extern "system" fn(instance: crate::vk1_0::Instance, p_create_info: *const crate::extensions::khr_xlib_surface::XlibSurfaceCreateInfoKHR, p_allocator: *const crate::vk1_0::AllocationCallbacks, p_surface: *mut crate::extensions::khr_surface::SurfaceKHR) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceXlibPresentationSupportKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR = unsafe extern "system" fn(physical_device: crate::vk1_0::PhysicalDevice, queue_family_index: u32, dpy: *mut std::ffi::c_void, visual_id: u64) -> crate::vk1_0::Bool32;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkXlibSurfaceCreateInfoKHR.html) · Structure"]
#[doc(alias = "VkXlibSurfaceCreateInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XlibSurfaceCreateInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::khr_xlib_surface::XlibSurfaceCreateFlagsKHR,
    pub dpy: *mut std::ffi::c_void,
    pub window: u64,
}
impl XlibSurfaceCreateInfoKHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::XLIB_SURFACE_CREATE_INFO_KHR;
}
impl Default for XlibSurfaceCreateInfoKHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), flags: Default::default(), dpy: std::ptr::null_mut(), window: Default::default() }
    }
}
impl std::fmt::Debug for XlibSurfaceCreateInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("XlibSurfaceCreateInfoKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("dpy", &self.dpy).field("window", &self.window).finish()
    }
}
impl XlibSurfaceCreateInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> XlibSurfaceCreateInfoKHRBuilder<'a> {
        XlibSurfaceCreateInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkXlibSurfaceCreateInfoKHR.html) · Builder of [`XlibSurfaceCreateInfoKHR`]"]
#[repr(transparent)]
pub struct XlibSurfaceCreateInfoKHRBuilder<'a>(XlibSurfaceCreateInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> XlibSurfaceCreateInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> XlibSurfaceCreateInfoKHRBuilder<'a> {
        XlibSurfaceCreateInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn flags(mut self, flags: crate::extensions::khr_xlib_surface::XlibSurfaceCreateFlagsKHR) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn dpy(mut self, dpy: *mut std::ffi::c_void) -> Self {
        self.0.dpy = dpy;
        self
    }
    #[inline]
    #[must_use]
    pub fn window(mut self, window: u64) -> Self {
        self.0.window = window as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> XlibSurfaceCreateInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for XlibSurfaceCreateInfoKHRBuilder<'a> {
    fn default() -> XlibSurfaceCreateInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for XlibSurfaceCreateInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for XlibSurfaceCreateInfoKHRBuilder<'a> {
    type Target = XlibSurfaceCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for XlibSurfaceCreateInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::khr_xlib_surface`]"]
impl crate::InstanceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateXlibSurfaceKHR.html) · Function"]
    #[doc(alias = "vkCreateXlibSurfaceKHR")]
    pub unsafe fn create_xlib_surface_khr(&self, create_info: &crate::extensions::khr_xlib_surface::XlibSurfaceCreateInfoKHR, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> crate::utils::VulkanResult<crate::extensions::khr_surface::SurfaceKHR> {
        let _function = self.create_xlib_surface_khr.expect(crate::NOT_LOADED_MESSAGE);
        let mut surface = Default::default();
        let _return = _function(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut surface,
        );
        crate::utils::VulkanResult::new(_return, surface)
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceXlibPresentationSupportKHR.html) · Function"]
    #[doc(alias = "vkGetPhysicalDeviceXlibPresentationSupportKHR")]
    pub unsafe fn get_physical_device_xlib_presentation_support_khr(&self, physical_device: crate::vk1_0::PhysicalDevice, queue_family_index: u32, dpy: *mut std::ffi::c_void, visual_id: u64) -> bool {
        let _function = self.get_physical_device_xlib_presentation_support_khr.expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(physical_device as _, queue_family_index as _, dpy, visual_id as _);
        _return != 0
    }
}
