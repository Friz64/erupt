# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_KHR_surface_protected_capabilities.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_SURFACE_PROTECTED_CAPABILITIES_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_SURFACE_PROTECTED_CAPABILITIES_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_surface_protected_capabilities");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceProtectedCapabilitiesKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SurfaceProtectedCapabilitiesKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub supports_protected: crate::vk1_0::Bool32,
}
impl SurfaceProtectedCapabilitiesKHR {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableBySurfaceProtectedCapabilitiesKHR,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> SurfaceProtectedCapabilitiesKHRBuilder<'a> {
        SurfaceProtectedCapabilitiesKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for SurfaceProtectedCapabilitiesKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("SurfaceProtectedCapabilitiesKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("supports_protected", &(self.supports_protected != 0))
            .finish()
    }
}
impl Default for SurfaceProtectedCapabilitiesKHR {
    fn default() -> SurfaceProtectedCapabilitiesKHR {
        SurfaceProtectedCapabilitiesKHR {
            s_type: crate::vk1_0::StructureType::SURFACE_PROTECTED_CAPABILITIES_KHR,
            p_next: std::ptr::null(),
            supports_protected: Default::default(),
        }
    }
}
#[doc = "Used by [`SurfaceProtectedCapabilitiesKHR::extend`](struct.SurfaceProtectedCapabilitiesKHR.html#method.extend)"]
pub trait ExtendableBySurfaceProtectedCapabilitiesKHR {}
impl ExtendableBySurfaceProtectedCapabilitiesKHR
    for crate::extensions::khr_get_surface_capabilities2::SurfaceCapabilities2KHR
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`SurfaceProtectedCapabilitiesKHR`](struct.SurfaceProtectedCapabilitiesKHR.html)"]
#[repr(transparent)]
pub struct SurfaceProtectedCapabilitiesKHRBuilder<'a>(
    SurfaceProtectedCapabilitiesKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> SurfaceProtectedCapabilitiesKHRBuilder<'a> {
    #[inline]
    pub fn new() -> SurfaceProtectedCapabilitiesKHRBuilder<'a> {
        SurfaceProtectedCapabilitiesKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn supports_protected(mut self, supports_protected: bool) -> Self {
        self.0.supports_protected = supports_protected as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> SurfaceProtectedCapabilitiesKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for SurfaceProtectedCapabilitiesKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for SurfaceProtectedCapabilitiesKHRBuilder<'a> {
    type Target = SurfaceProtectedCapabilitiesKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SurfaceProtectedCapabilitiesKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
