#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_MVK_MACOS_SURFACE_SPEC_VERSION")]
pub const MVK_MACOS_SURFACE_SPEC_VERSION: u32 = 3;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_MVK_MACOS_SURFACE_EXTENSION_NAME")]
pub const MVK_MACOS_SURFACE_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_MVK_macos_surface");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_MAC_OS_SURFACE_MVK: *const std::os::raw::c_char = crate::cstr!("vkCreateMacOSSurfaceMVK");
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMacOSSurfaceCreateFlagsMVK.html) · Bitmask of [`MacOSSurfaceCreateFlagBitsMVK`]"] # [doc (alias = "VkMacOSSurfaceCreateFlagsMVK")] # [derive (Default)] # [repr (transparent)] pub struct MacOSSurfaceCreateFlagsMVK : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`MacOSSurfaceCreateFlagsMVK`]"]
#[doc(alias = "VkMacOSSurfaceCreateFlagBitsMVK")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct MacOSSurfaceCreateFlagBitsMVK(pub u32);
impl MacOSSurfaceCreateFlagBitsMVK {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> MacOSSurfaceCreateFlagsMVK {
        MacOSSurfaceCreateFlagsMVK::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for MacOSSurfaceCreateFlagBitsMVK {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::mvk_macos_surface`]"]
impl crate::vk1_0::StructureType {
    pub const MACOS_SURFACE_CREATE_INFO_MVK: Self = Self(1000123000);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateMacOSSurfaceMVK.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateMacOSSurfaceMVK = unsafe extern "system" fn(instance: crate::vk1_0::Instance, p_create_info: *const crate::extensions::mvk_macos_surface::MacOSSurfaceCreateInfoMVK, p_allocator: *const crate::vk1_0::AllocationCallbacks, p_surface: *mut crate::extensions::khr_surface::SurfaceKHR) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMacOSSurfaceCreateInfoMVK.html) · Structure"]
#[doc(alias = "VkMacOSSurfaceCreateInfoMVK")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MacOSSurfaceCreateInfoMVK {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::mvk_macos_surface::MacOSSurfaceCreateFlagsMVK,
    pub p_view: *const std::ffi::c_void,
}
impl MacOSSurfaceCreateInfoMVK {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::MACOS_SURFACE_CREATE_INFO_MVK;
}
impl Default for MacOSSurfaceCreateInfoMVK {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), flags: Default::default(), p_view: std::ptr::null() }
    }
}
impl std::fmt::Debug for MacOSSurfaceCreateInfoMVK {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MacOSSurfaceCreateInfoMVK").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("p_view", &self.p_view).finish()
    }
}
impl MacOSSurfaceCreateInfoMVK {
    #[inline]
    pub fn into_builder<'a>(self) -> MacOSSurfaceCreateInfoMVKBuilder<'a> {
        MacOSSurfaceCreateInfoMVKBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMacOSSurfaceCreateInfoMVK.html) · Builder of [`MacOSSurfaceCreateInfoMVK`]"]
#[repr(transparent)]
pub struct MacOSSurfaceCreateInfoMVKBuilder<'a>(MacOSSurfaceCreateInfoMVK, std::marker::PhantomData<&'a ()>);
impl<'a> MacOSSurfaceCreateInfoMVKBuilder<'a> {
    #[inline]
    pub fn new() -> MacOSSurfaceCreateInfoMVKBuilder<'a> {
        MacOSSurfaceCreateInfoMVKBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn flags(mut self, flags: crate::extensions::mvk_macos_surface::MacOSSurfaceCreateFlagsMVK) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn view(mut self, view: *const std::ffi::c_void) -> Self {
        self.0.p_view = view;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> MacOSSurfaceCreateInfoMVK {
        self.0
    }
}
impl<'a> std::default::Default for MacOSSurfaceCreateInfoMVKBuilder<'a> {
    fn default() -> MacOSSurfaceCreateInfoMVKBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for MacOSSurfaceCreateInfoMVKBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for MacOSSurfaceCreateInfoMVKBuilder<'a> {
    type Target = MacOSSurfaceCreateInfoMVK;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for MacOSSurfaceCreateInfoMVKBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::mvk_macos_surface`]"]
impl crate::InstanceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateMacOSSurfaceMVK.html) · Function"]
    #[doc(alias = "vkCreateMacOSSurfaceMVK")]
    pub unsafe fn create_mac_os_surface_mvk(&self, create_info: &crate::extensions::mvk_macos_surface::MacOSSurfaceCreateInfoMVK, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> crate::utils::VulkanResult<crate::extensions::khr_surface::SurfaceKHR> {
        let _function = self.create_mac_os_surface_mvk.expect(crate::NOT_LOADED_MESSAGE);
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
}
