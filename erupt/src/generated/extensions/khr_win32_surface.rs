#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_WIN32_SURFACE_SPEC_VERSION")]
pub const KHR_WIN32_SURFACE_SPEC_VERSION: u32 = 6;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_WIN32_SURFACE_EXTENSION_NAME")]
pub const KHR_WIN32_SURFACE_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_win32_surface");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_WIN32_SURFACE_KHR: *const std::os::raw::c_char = crate::cstr!("vkCreateWin32SurfaceKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_PHYSICAL_DEVICE_WIN32_PRESENTATION_SUPPORT_KHR: *const std::os::raw::c_char = crate::cstr!("vkGetPhysicalDeviceWin32PresentationSupportKHR");
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkWin32SurfaceCreateFlagsKHR.html) · Bitmask of [`Win32SurfaceCreateFlagBitsKHR`]"] # [doc (alias = "VkWin32SurfaceCreateFlagsKHR")] # [derive (Default)] # [repr (transparent)] pub struct Win32SurfaceCreateFlagsKHR : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`Win32SurfaceCreateFlagsKHR`]"]
#[doc(alias = "VkWin32SurfaceCreateFlagBitsKHR")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct Win32SurfaceCreateFlagBitsKHR(pub u32);
impl Win32SurfaceCreateFlagBitsKHR {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> Win32SurfaceCreateFlagsKHR {
        Win32SurfaceCreateFlagsKHR::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for Win32SurfaceCreateFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateWin32SurfaceKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateWin32SurfaceKHR = unsafe extern "system" fn(instance: crate::vk1_0::Instance, p_create_info: *const crate::extensions::khr_win32_surface::Win32SurfaceCreateInfoKHR, p_allocator: *const crate::vk1_0::AllocationCallbacks, p_surface: *mut crate::extensions::khr_surface::SurfaceKHR) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceWin32PresentationSupportKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR = unsafe extern "system" fn(physical_device: crate::vk1_0::PhysicalDevice, queue_family_index: u32) -> crate::vk1_0::Bool32;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkWin32SurfaceCreateInfoKHR.html) · Structure"]
#[doc(alias = "VkWin32SurfaceCreateInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Win32SurfaceCreateInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::khr_win32_surface::Win32SurfaceCreateFlagsKHR,
    pub hinstance: *mut std::ffi::c_void,
    pub hwnd: *mut std::ffi::c_void,
}
impl Default for Win32SurfaceCreateInfoKHR {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::WIN32_SURFACE_CREATE_INFO_KHR, p_next: std::ptr::null(), flags: Default::default(), hinstance: std::ptr::null_mut(), hwnd: std::ptr::null_mut() }
    }
}
impl std::fmt::Debug for Win32SurfaceCreateInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Win32SurfaceCreateInfoKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("hinstance", &self.hinstance).field("hwnd", &self.hwnd).finish()
    }
}
impl Win32SurfaceCreateInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> Win32SurfaceCreateInfoKHRBuilder<'a> {
        Win32SurfaceCreateInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkWin32SurfaceCreateInfoKHR.html) · Builder of [`Win32SurfaceCreateInfoKHR`]"]
#[repr(transparent)]
pub struct Win32SurfaceCreateInfoKHRBuilder<'a>(Win32SurfaceCreateInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> Win32SurfaceCreateInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> Win32SurfaceCreateInfoKHRBuilder<'a> {
        Win32SurfaceCreateInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::khr_win32_surface::Win32SurfaceCreateFlagsKHR) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn hinstance(mut self, hinstance: *mut std::ffi::c_void) -> Self {
        self.0.hinstance = hinstance;
        self
    }
    #[inline]
    pub fn hwnd(mut self, hwnd: *mut std::ffi::c_void) -> Self {
        self.0.hwnd = hwnd;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> Win32SurfaceCreateInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for Win32SurfaceCreateInfoKHRBuilder<'a> {
    fn default() -> Win32SurfaceCreateInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for Win32SurfaceCreateInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for Win32SurfaceCreateInfoKHRBuilder<'a> {
    type Target = Win32SurfaceCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for Win32SurfaceCreateInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::khr_win32_surface`]"]
impl crate::InstanceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateWin32SurfaceKHR.html) · Function"]
    #[doc(alias = "vkCreateWin32SurfaceKHR")]
    pub unsafe fn create_win32_surface_khr(&self, create_info: &crate::extensions::khr_win32_surface::Win32SurfaceCreateInfoKHR, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> crate::utils::VulkanResult<crate::extensions::khr_surface::SurfaceKHR> {
        let _function = self.create_win32_surface_khr.expect("tried to call a function that isn't loaded");
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
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceWin32PresentationSupportKHR.html) · Function"]
    #[doc(alias = "vkGetPhysicalDeviceWin32PresentationSupportKHR")]
    pub unsafe fn get_physical_device_win32_presentation_support_khr(&self, physical_device: crate::vk1_0::PhysicalDevice, queue_family_index: u32) -> bool {
        let _function = self.get_physical_device_win32_presentation_support_khr.expect("tried to call a function that isn't loaded");
        let _return = _function(physical_device as _, queue_family_index as _);
        _return != 0
    }
}
