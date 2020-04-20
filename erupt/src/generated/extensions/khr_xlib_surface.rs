# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_KHR_xlib_surface.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_XLIB_SURFACE_SPEC_VERSION: u32 = 6;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_XLIB_SURFACE_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_xlib_surface");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateXlibSurfaceKHR.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateXlibSurfaceKHR = unsafe extern "system" fn(
    instance: crate::vk1_0::Instance,
    p_create_info: *const crate::extensions::khr_xlib_surface::XlibSurfaceCreateInfoKHR,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
    p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceXlibPresentationSupportKHR.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR =
    unsafe extern "system" fn(
        physical_device: crate::vk1_0::PhysicalDevice,
        queue_family_index: u32,
        dpy: *mut *const std::ffi::c_void,
        visual_id: std::os::raw::c_uint,
    ) -> crate::vk1_0::Bool32;
#[doc = "Provides Instance Commands for [`KhrXlibSurfaceInstanceLoaderExt`](trait.KhrXlibSurfaceInstanceLoaderExt.html)"]
pub struct KhrXlibSurfaceInstanceCommands {
    pub create_xlib_surface_khr: PFN_vkCreateXlibSurfaceKHR,
    pub get_physical_device_xlib_presentation_support_khr:
        PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR,
}
impl KhrXlibSurfaceInstanceCommands {
    #[inline]
    pub fn load(loader: &crate::InstanceLoader) -> Option<KhrXlibSurfaceInstanceCommands> {
        unsafe {
            Some(KhrXlibSurfaceInstanceCommands {
                create_xlib_surface_khr: std::mem::transmute(
                    loader.symbol("vkCreateXlibSurfaceKHR")?,
                ),
                get_physical_device_xlib_presentation_support_khr: std::mem::transmute(
                    loader.symbol("vkGetPhysicalDeviceXlibPresentationSupportKHR")?,
                ),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`KhrXlibSurfaceInstanceCommands`](struct.KhrXlibSurfaceInstanceCommands.html)"]
pub trait KhrXlibSurfaceInstanceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateXlibSurfaceKHR.html) · Instance Command"]
    unsafe fn create_xlib_surface_khr(
        &self,
        create_info: &crate::extensions::khr_xlib_surface::XlibSurfaceCreateInfoKHR,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        surface: Option<crate::extensions::khr_surface::SurfaceKHR>,
    ) -> crate::utils::VulkanResult<crate::extensions::khr_surface::SurfaceKHR>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceXlibPresentationSupportKHR.html) · Instance Command"]
    unsafe fn get_physical_device_xlib_presentation_support_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        queue_family_index: u32,
        dpy: *mut *const std::ffi::c_void,
        visual_id: std::os::raw::c_uint,
    ) -> crate::vk1_0::Bool32;
}
impl KhrXlibSurfaceInstanceLoaderExt for crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateXlibSurfaceKHR.html) · Instance Command"]
    unsafe fn create_xlib_surface_khr(
        &self,
        create_info: &crate::extensions::khr_xlib_surface::XlibSurfaceCreateInfoKHR,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        surface: Option<crate::extensions::khr_surface::SurfaceKHR>,
    ) -> crate::utils::VulkanResult<crate::extensions::khr_surface::SurfaceKHR> {
        let function = self
            .khr_xlib_surface
            .as_ref()
            .expect("`khr_xlib_surface` not loaded")
            .create_xlib_surface_khr;
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
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceXlibPresentationSupportKHR.html) · Instance Command"]
    unsafe fn get_physical_device_xlib_presentation_support_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        queue_family_index: u32,
        dpy: *mut *const std::ffi::c_void,
        visual_id: std::os::raw::c_uint,
    ) -> crate::vk1_0::Bool32 {
        let function = self
            .khr_xlib_surface
            .as_ref()
            .expect("`khr_xlib_surface` not loaded")
            .get_physical_device_xlib_presentation_support_khr;
        let _val = function(physical_device, queue_family_index, dpy, visual_id);
        _val
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkXlibSurfaceCreateInfoKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XlibSurfaceCreateInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::khr_xlib_surface::XlibSurfaceCreateFlagsKHR,
    pub dpy: *mut *const std::ffi::c_void,
    pub window: std::os::raw::c_ulong,
}
impl XlibSurfaceCreateInfoKHR {
    #[inline]
    pub fn builder<'a>(self) -> XlibSurfaceCreateInfoKHRBuilder<'a> {
        XlibSurfaceCreateInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for XlibSurfaceCreateInfoKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("XlibSurfaceCreateInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("dpy", &self.dpy)
            .field("window", &self.window)
            .finish()
    }
}
impl Default for XlibSurfaceCreateInfoKHR {
    fn default() -> XlibSurfaceCreateInfoKHR {
        XlibSurfaceCreateInfoKHR {
            s_type: crate::vk1_0::StructureType::XLIB_SURFACE_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            flags: Default::default(),
            dpy: std::ptr::null_mut(),
            window: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkXlibSurfaceCreateInfoKHR.html) · Builder of [`XlibSurfaceCreateInfoKHR`](struct.XlibSurfaceCreateInfoKHR.html)"]
#[repr(transparent)]
pub struct XlibSurfaceCreateInfoKHRBuilder<'a>(
    XlibSurfaceCreateInfoKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> XlibSurfaceCreateInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> XlibSurfaceCreateInfoKHRBuilder<'a> {
        XlibSurfaceCreateInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn flags(
        mut self,
        flags: crate::extensions::khr_xlib_surface::XlibSurfaceCreateFlagsKHR,
    ) -> Self {
        self.0.flags = flags;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn dpy(mut self, dpy: &'a mut *const std::ffi::c_void) -> Self {
        self.0.dpy = dpy;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn window(mut self, window: std::os::raw::c_ulong) -> Self {
        self.0.window = window;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> XlibSurfaceCreateInfoKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for XlibSurfaceCreateInfoKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "<s>Vulkan Manual Page</s> · Flag Bits of [`XlibSurfaceCreateFlagsKHR`](struct.XlibSurfaceCreateFlagsKHR.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
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
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            _ => "(unknown)",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkXlibSurfaceCreateFlagsKHR.html) · Flags of [`XlibSurfaceCreateFlagBitsKHR`](struct.XlibSurfaceCreateFlagBitsKHR.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct XlibSurfaceCreateFlagsKHR : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
