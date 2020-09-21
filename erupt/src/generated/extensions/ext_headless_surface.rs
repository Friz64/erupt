#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_HEADLESS_SURFACE_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_HEADLESS_SURFACE_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_headless_surface");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_HEADLESS_SURFACE_EXT: *const std::os::raw::c_char =
    crate::cstr!("vkCreateHeadlessSurfaceEXT");
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkHeadlessSurfaceCreateFlagsEXT.html) · Bitmask of [`HeadlessSurfaceCreateFlagBitsEXT`](./struct.HeadlessSurfaceCreateFlagBitsEXT.html)"] # [derive (Default)] # [repr (transparent)] pub struct HeadlessSurfaceCreateFlagsEXT : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`HeadlessSurfaceCreateFlagsEXT`](./struct.HeadlessSurfaceCreateFlagsEXT.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct HeadlessSurfaceCreateFlagBitsEXT(pub u32);
impl HeadlessSurfaceCreateFlagBitsEXT {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> HeadlessSurfaceCreateFlagsEXT {
        HeadlessSurfaceCreateFlagsEXT::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for HeadlessSurfaceCreateFlagBitsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateHeadlessSurfaceEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateHeadlessSurfaceEXT = unsafe extern "system" fn(
    instance: crate::vk1_0::Instance,
    p_create_info: *const crate::extensions::ext_headless_surface::HeadlessSurfaceCreateInfoEXT,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
    p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkHeadlessSurfaceCreateInfoEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HeadlessSurfaceCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::ext_headless_surface::HeadlessSurfaceCreateFlagsEXT,
}
impl Default for HeadlessSurfaceCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::HEADLESS_SURFACE_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            flags: Default::default(),
        }
    }
}
impl std::fmt::Debug for HeadlessSurfaceCreateInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("HeadlessSurfaceCreateInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .finish()
    }
}
impl HeadlessSurfaceCreateInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> HeadlessSurfaceCreateInfoEXTBuilder<'a> {
        HeadlessSurfaceCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkHeadlessSurfaceCreateInfoEXT.html) · Builder of [`HeadlessSurfaceCreateInfoEXT`](struct.HeadlessSurfaceCreateInfoEXT.html)"]
#[repr(transparent)]
pub struct HeadlessSurfaceCreateInfoEXTBuilder<'a>(
    HeadlessSurfaceCreateInfoEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> HeadlessSurfaceCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> HeadlessSurfaceCreateInfoEXTBuilder<'a> {
        HeadlessSurfaceCreateInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(
        mut self,
        flags: crate::extensions::ext_headless_surface::HeadlessSurfaceCreateFlagsEXT,
    ) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> HeadlessSurfaceCreateInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for HeadlessSurfaceCreateInfoEXTBuilder<'a> {
    fn default() -> HeadlessSurfaceCreateInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for HeadlessSurfaceCreateInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for HeadlessSurfaceCreateInfoEXTBuilder<'a> {
    type Target = HeadlessSurfaceCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for HeadlessSurfaceCreateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`extensions::ext_headless_surface`](extensions/ext_headless_surface/index.html)"]
impl crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateHeadlessSurfaceEXT.html) · Function"]
    pub unsafe fn create_headless_surface_ext(
        &self,
        create_info: &crate::extensions::ext_headless_surface::HeadlessSurfaceCreateInfoEXT,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        surface: Option<crate::extensions::khr_surface::SurfaceKHR>,
    ) -> crate::utils::VulkanResult<crate::extensions::khr_surface::SurfaceKHR> {
        let _function = self
            .create_headless_surface_ext
            .expect("`create_headless_surface_ext` is not loaded");
        let mut surface = match surface {
            Some(v) => v,
            None => Default::default(),
        };
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
}
