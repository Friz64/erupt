# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_KHR_win32_surface.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_WIN32_SURFACE_SPEC_VERSION: u32 = 6;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_WIN32_SURFACE_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_win32_surface");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateWin32SurfaceKHR.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateWin32SurfaceKHR = unsafe extern "system" fn(
    instance: crate::vk1_0::Instance,
    p_create_info: *const crate::extensions::khr_win32_surface::Win32SurfaceCreateInfoKHR,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
    p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceWin32PresentationSupportKHR.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR =
    unsafe extern "system" fn(
        physical_device: crate::vk1_0::PhysicalDevice,
        queue_family_index: u32,
    ) -> crate::vk1_0::Bool32;
#[doc = "Provides Instance Commands for [`KhrWin32SurfaceInstanceLoaderExt`](trait.KhrWin32SurfaceInstanceLoaderExt.html)"]
pub struct KhrWin32SurfaceInstanceCommands {
    pub create_win32_surface_khr: PFN_vkCreateWin32SurfaceKHR,
    pub get_physical_device_win32_presentation_support_khr:
        PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR,
}
impl KhrWin32SurfaceInstanceCommands {
    #[inline]
    pub fn load(loader: &crate::InstanceLoader) -> Option<KhrWin32SurfaceInstanceCommands> {
        unsafe {
            Some(KhrWin32SurfaceInstanceCommands {
                create_win32_surface_khr: std::mem::transmute(
                    loader.symbol("vkCreateWin32SurfaceKHR")?,
                ),
                get_physical_device_win32_presentation_support_khr: std::mem::transmute(
                    loader.symbol("vkGetPhysicalDeviceWin32PresentationSupportKHR")?,
                ),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`KhrWin32SurfaceInstanceCommands`](struct.KhrWin32SurfaceInstanceCommands.html)"]
pub trait KhrWin32SurfaceInstanceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateWin32SurfaceKHR.html) · Instance Command"]
    unsafe fn create_win32_surface_khr(
        &self,
        create_info: &crate::extensions::khr_win32_surface::Win32SurfaceCreateInfoKHR,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        surface: Option<crate::extensions::khr_surface::SurfaceKHR>,
    ) -> crate::utils::VulkanResult<crate::extensions::khr_surface::SurfaceKHR>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceWin32PresentationSupportKHR.html) · Instance Command"]
    unsafe fn get_physical_device_win32_presentation_support_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        queue_family_index: u32,
    ) -> crate::vk1_0::Bool32;
}
impl KhrWin32SurfaceInstanceLoaderExt for crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateWin32SurfaceKHR.html) · Instance Command"]
    unsafe fn create_win32_surface_khr(
        &self,
        create_info: &crate::extensions::khr_win32_surface::Win32SurfaceCreateInfoKHR,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        surface: Option<crate::extensions::khr_surface::SurfaceKHR>,
    ) -> crate::utils::VulkanResult<crate::extensions::khr_surface::SurfaceKHR> {
        let function = self
            .khr_win32_surface
            .as_ref()
            .expect("`khr_win32_surface` not loaded")
            .create_win32_surface_khr;
        let mut surface = surface.unwrap_or_else(|| Default::default());
        let _val = function(
            self.handle,
            create_info,
            if let Some(allocator) = allocator {
                allocator
            } else {
                std::ptr::null()
            },
            &mut surface,
        );
        crate::utils::VulkanResult::new(_val, surface)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceWin32PresentationSupportKHR.html) · Instance Command"]
    unsafe fn get_physical_device_win32_presentation_support_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        queue_family_index: u32,
    ) -> crate::vk1_0::Bool32 {
        let function = self
            .khr_win32_surface
            .as_ref()
            .expect("`khr_win32_surface` not loaded")
            .get_physical_device_win32_presentation_support_khr;
        let _val = function(physical_device, queue_family_index);
        _val
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkWin32SurfaceCreateInfoKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Win32SurfaceCreateInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::khr_win32_surface::Win32SurfaceCreateFlagsKHR,
    pub hinstance: *mut std::ffi::c_void,
    pub hwnd: *mut std::ffi::c_void,
}
impl Win32SurfaceCreateInfoKHR {
    #[inline]
    pub fn builder<'a>(self) -> Win32SurfaceCreateInfoKHRBuilder<'a> {
        Win32SurfaceCreateInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for Win32SurfaceCreateInfoKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("Win32SurfaceCreateInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("hinstance", &self.hinstance)
            .field("hwnd", &self.hwnd)
            .finish()
    }
}
impl Default for Win32SurfaceCreateInfoKHR {
    fn default() -> Win32SurfaceCreateInfoKHR {
        Win32SurfaceCreateInfoKHR {
            s_type: crate::vk1_0::StructureType::WIN32_SURFACE_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            flags: Default::default(),
            hinstance: std::ptr::null_mut(),
            hwnd: std::ptr::null_mut(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`Win32SurfaceCreateInfoKHR`](struct.Win32SurfaceCreateInfoKHR.html)"]
#[repr(transparent)]
pub struct Win32SurfaceCreateInfoKHRBuilder<'a>(
    Win32SurfaceCreateInfoKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> Win32SurfaceCreateInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> Win32SurfaceCreateInfoKHRBuilder<'a> {
        Win32SurfaceCreateInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn flags(
        mut self,
        flags: crate::extensions::khr_win32_surface::Win32SurfaceCreateFlagsKHR,
    ) -> Self {
        self.0.flags = flags;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn hinstance(mut self, hinstance: &'a mut std::ffi::c_void) -> Self {
        self.0.hinstance = hinstance;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn hwnd(mut self, hwnd: &'a mut std::ffi::c_void) -> Self {
        self.0.hwnd = hwnd;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> Win32SurfaceCreateInfoKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for Win32SurfaceCreateInfoKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "<s>Vulkan Manual Page</s> · Flag Bits of [`Win32SurfaceCreateFlagsKHR`](struct.Win32SurfaceCreateFlagsKHR.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
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
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            _ => "(unknown)",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkWin32SurfaceCreateFlagsKHR.html) · Flags of [`Win32SurfaceCreateFlagBitsKHR`](struct.Win32SurfaceCreateFlagBitsKHR.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct Win32SurfaceCreateFlagsKHR : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
