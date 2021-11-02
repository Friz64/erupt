#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_MVK_IOS_SURFACE_SPEC_VERSION")]
pub const MVK_IOS_SURFACE_SPEC_VERSION: u32 = 3;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_MVK_IOS_SURFACE_EXTENSION_NAME")]
pub const MVK_IOS_SURFACE_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_MVK_ios_surface");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_IOS_SURFACE_MVK: *const std::os::raw::c_char = crate::cstr!("vkCreateIOSSurfaceMVK");
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkIOSSurfaceCreateFlagsMVK.html) · Bitmask of [`IOSSurfaceCreateFlagBitsMVK`]"] # [doc (alias = "VkIOSSurfaceCreateFlagsMVK")] # [derive (Default)] # [repr (transparent)] pub struct IOSSurfaceCreateFlagsMVK : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`IOSSurfaceCreateFlagsMVK`]"]
#[doc(alias = "VkIOSSurfaceCreateFlagBitsMVK")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct IOSSurfaceCreateFlagBitsMVK(pub u32);
impl IOSSurfaceCreateFlagBitsMVK {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> IOSSurfaceCreateFlagsMVK {
        IOSSurfaceCreateFlagsMVK::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for IOSSurfaceCreateFlagBitsMVK {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::mvk_ios_surface`]"]
impl crate::vk1_0::StructureType {
    pub const IOS_SURFACE_CREATE_INFO_MVK: Self = Self(1000122000);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateIOSSurfaceMVK.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateIOSSurfaceMVK = unsafe extern "system" fn(instance: crate::vk1_0::Instance, p_create_info: *const crate::extensions::mvk_ios_surface::IOSSurfaceCreateInfoMVK, p_allocator: *const crate::vk1_0::AllocationCallbacks, p_surface: *mut crate::extensions::khr_surface::SurfaceKHR) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkIOSSurfaceCreateInfoMVK.html) · Structure"]
#[doc(alias = "VkIOSSurfaceCreateInfoMVK")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IOSSurfaceCreateInfoMVK {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::mvk_ios_surface::IOSSurfaceCreateFlagsMVK,
    pub p_view: *const std::ffi::c_void,
}
impl IOSSurfaceCreateInfoMVK {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::IOS_SURFACE_CREATE_INFO_MVK;
}
impl Default for IOSSurfaceCreateInfoMVK {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), flags: Default::default(), p_view: std::ptr::null() }
    }
}
impl std::fmt::Debug for IOSSurfaceCreateInfoMVK {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IOSSurfaceCreateInfoMVK").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("p_view", &self.p_view).finish()
    }
}
impl IOSSurfaceCreateInfoMVK {
    #[inline]
    pub fn into_builder<'a>(self) -> IOSSurfaceCreateInfoMVKBuilder<'a> {
        IOSSurfaceCreateInfoMVKBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkIOSSurfaceCreateInfoMVK.html) · Builder of [`IOSSurfaceCreateInfoMVK`]"]
#[repr(transparent)]
pub struct IOSSurfaceCreateInfoMVKBuilder<'a>(IOSSurfaceCreateInfoMVK, std::marker::PhantomData<&'a ()>);
impl<'a> IOSSurfaceCreateInfoMVKBuilder<'a> {
    #[inline]
    pub fn new() -> IOSSurfaceCreateInfoMVKBuilder<'a> {
        IOSSurfaceCreateInfoMVKBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::mvk_ios_surface::IOSSurfaceCreateFlagsMVK) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn view(mut self, view: *const std::ffi::c_void) -> Self {
        self.0.p_view = view;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> IOSSurfaceCreateInfoMVK {
        self.0
    }
}
impl<'a> std::default::Default for IOSSurfaceCreateInfoMVKBuilder<'a> {
    fn default() -> IOSSurfaceCreateInfoMVKBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for IOSSurfaceCreateInfoMVKBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for IOSSurfaceCreateInfoMVKBuilder<'a> {
    type Target = IOSSurfaceCreateInfoMVK;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for IOSSurfaceCreateInfoMVKBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::mvk_ios_surface`]"]
impl crate::InstanceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateIOSSurfaceMVK.html) · Function"]
    #[doc(alias = "vkCreateIOSSurfaceMVK")]
    pub unsafe fn create_ios_surface_mvk(&self, create_info: &crate::extensions::mvk_ios_surface::IOSSurfaceCreateInfoMVK, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> crate::utils::VulkanResult<crate::extensions::khr_surface::SurfaceKHR> {
        let _function = self.create_ios_surface_mvk.expect(crate::NOT_LOADED_MESSAGE);
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
