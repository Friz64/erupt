#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_DIRECTFB_SURFACE_SPEC_VERSION")]
pub const EXT_DIRECTFB_SURFACE_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_DIRECTFB_SURFACE_EXTENSION_NAME")]
pub const EXT_DIRECTFB_SURFACE_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_directfb_surface");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_DIRECT_FB_SURFACE_EXT: *const std::os::raw::c_char = crate::cstr!("vkCreateDirectFBSurfaceEXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_PHYSICAL_DEVICE_DIRECT_FB_PRESENTATION_SUPPORT_EXT: *const std::os::raw::c_char = crate::cstr!("vkGetPhysicalDeviceDirectFBPresentationSupportEXT");
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDirectFBSurfaceCreateFlagsEXT.html) · Bitmask of [`DirectFBSurfaceCreateFlagBitsEXT`]"] # [doc (alias = "VkDirectFBSurfaceCreateFlagsEXT")] # [derive (Default)] # [repr (transparent)] pub struct DirectFBSurfaceCreateFlagsEXT : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`DirectFBSurfaceCreateFlagsEXT`]"]
#[doc(alias = "VkDirectFBSurfaceCreateFlagBitsEXT")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct DirectFBSurfaceCreateFlagBitsEXT(pub u32);
impl DirectFBSurfaceCreateFlagBitsEXT {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> DirectFBSurfaceCreateFlagsEXT {
        DirectFBSurfaceCreateFlagsEXT::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for DirectFBSurfaceCreateFlagBitsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDirectFBSurfaceEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateDirectFBSurfaceEXT = unsafe extern "system" fn(
    instance: crate::vk1_0::Instance,
    p_create_info: *const crate::extensions::ext_directfb_surface::DirectFBSurfaceCreateInfoEXT,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
    p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceDirectFBPresentationSupportEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceDirectFBPresentationSupportEXT = unsafe extern "system" fn(physical_device: crate::vk1_0::PhysicalDevice, queue_family_index: u32, dfb: *mut std::ffi::c_void) -> crate::vk1_0::Bool32;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDirectFBSurfaceCreateInfoEXT.html) · Structure"]
#[doc(alias = "VkDirectFBSurfaceCreateInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DirectFBSurfaceCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::ext_directfb_surface::DirectFBSurfaceCreateFlagsEXT,
    pub dfb: *mut std::ffi::c_void,
    pub surface: *mut std::ffi::c_void,
}
impl Default for DirectFBSurfaceCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::DIRECTFB_SURFACE_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            flags: Default::default(),
            dfb: std::ptr::null_mut(),
            surface: std::ptr::null_mut(),
        }
    }
}
impl std::fmt::Debug for DirectFBSurfaceCreateInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DirectFBSurfaceCreateInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("dfb", &self.dfb)
            .field("surface", &self.surface)
            .finish()
    }
}
impl DirectFBSurfaceCreateInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> DirectFBSurfaceCreateInfoEXTBuilder<'a> {
        DirectFBSurfaceCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDirectFBSurfaceCreateInfoEXT.html) · Builder of [`DirectFBSurfaceCreateInfoEXT`]"]
#[repr(transparent)]
pub struct DirectFBSurfaceCreateInfoEXTBuilder<'a>(DirectFBSurfaceCreateInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> DirectFBSurfaceCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> DirectFBSurfaceCreateInfoEXTBuilder<'a> {
        DirectFBSurfaceCreateInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::ext_directfb_surface::DirectFBSurfaceCreateFlagsEXT) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn dfb(mut self, dfb: *mut std::ffi::c_void) -> Self {
        self.0.dfb = dfb;
        self
    }
    #[inline]
    pub fn surface(mut self, surface: *mut std::ffi::c_void) -> Self {
        self.0.surface = surface;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DirectFBSurfaceCreateInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for DirectFBSurfaceCreateInfoEXTBuilder<'a> {
    fn default() -> DirectFBSurfaceCreateInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DirectFBSurfaceCreateInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DirectFBSurfaceCreateInfoEXTBuilder<'a> {
    type Target = DirectFBSurfaceCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DirectFBSurfaceCreateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<DirectFBSurfaceCreateInfoEXT> for DirectFBSurfaceCreateInfoEXTBuilder<'_> {}
#[doc = "Provided by [`crate::extensions::ext_directfb_surface`]"]
impl crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDirectFBSurfaceEXT.html) · Function"]
    #[doc(alias = "vkCreateDirectFBSurfaceEXT")]
    pub unsafe fn create_direct_fb_surface_ext(
        &self,
        create_info: &crate::extensions::ext_directfb_surface::DirectFBSurfaceCreateInfoEXT,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
    ) -> crate::utils::VulkanResult<crate::extensions::khr_surface::SurfaceKHR> {
        let _function = self.create_direct_fb_surface_ext.expect("`create_direct_fb_surface_ext` is not loaded");
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
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceDirectFBPresentationSupportEXT.html) · Function"]
    #[doc(alias = "vkGetPhysicalDeviceDirectFBPresentationSupportEXT")]
    pub unsafe fn get_physical_device_direct_fb_presentation_support_ext(&self, physical_device: crate::vk1_0::PhysicalDevice, queue_family_index: u32, dfb: *mut std::ffi::c_void) -> bool {
        let _function = self.get_physical_device_direct_fb_presentation_support_ext.expect("`get_physical_device_direct_fb_presentation_support_ext` is not loaded");
        let _return = _function(physical_device as _, queue_family_index as _, dfb);
        _return != 0
    }
}
