#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_SURFACE_PROTECTED_CAPABILITIES_SPEC_VERSION")]
pub const KHR_SURFACE_PROTECTED_CAPABILITIES_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_SURFACE_PROTECTED_CAPABILITIES_EXTENSION_NAME")]
pub const KHR_SURFACE_PROTECTED_CAPABILITIES_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_surface_protected_capabilities");
#[doc = "Provided by [`crate::extensions::khr_surface_protected_capabilities`]"]
impl crate::vk1_0::StructureType {
    pub const SURFACE_PROTECTED_CAPABILITIES_KHR: Self = Self(1000239000);
}
impl<'a> crate::ExtendableFrom<'a, SurfaceProtectedCapabilitiesKHR> for crate::extensions::khr_get_surface_capabilities2::SurfaceCapabilities2KHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, SurfaceProtectedCapabilitiesKHRBuilder<'_>> for crate::extensions::khr_get_surface_capabilities2::SurfaceCapabilities2KHRBuilder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceProtectedCapabilitiesKHR.html) · Structure"]
#[doc(alias = "VkSurfaceProtectedCapabilitiesKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SurfaceProtectedCapabilitiesKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub supports_protected: crate::vk1_0::Bool32,
}
impl SurfaceProtectedCapabilitiesKHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::SURFACE_PROTECTED_CAPABILITIES_KHR;
}
impl Default for SurfaceProtectedCapabilitiesKHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), supports_protected: Default::default() }
    }
}
impl std::fmt::Debug for SurfaceProtectedCapabilitiesKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SurfaceProtectedCapabilitiesKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("supports_protected", &(self.supports_protected != 0)).finish()
    }
}
impl SurfaceProtectedCapabilitiesKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> SurfaceProtectedCapabilitiesKHRBuilder<'a> {
        SurfaceProtectedCapabilitiesKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceProtectedCapabilitiesKHR.html) · Builder of [`SurfaceProtectedCapabilitiesKHR`]"]
#[repr(transparent)]
pub struct SurfaceProtectedCapabilitiesKHRBuilder<'a>(SurfaceProtectedCapabilitiesKHR, std::marker::PhantomData<&'a ()>);
impl<'a> SurfaceProtectedCapabilitiesKHRBuilder<'a> {
    #[inline]
    pub fn new() -> SurfaceProtectedCapabilitiesKHRBuilder<'a> {
        SurfaceProtectedCapabilitiesKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn supports_protected(mut self, supports_protected: bool) -> Self {
        self.0.supports_protected = supports_protected as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> SurfaceProtectedCapabilitiesKHR {
        self.0
    }
}
impl<'a> std::default::Default for SurfaceProtectedCapabilitiesKHRBuilder<'a> {
    fn default() -> SurfaceProtectedCapabilitiesKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for SurfaceProtectedCapabilitiesKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
