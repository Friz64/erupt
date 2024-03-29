// DO NOT EDIT: @generated by erupt's generator
///<s>Vulkan Manual Page</s> · Constant
#[doc(alias = "VK_GGP_FRAME_TOKEN_SPEC_VERSION")]
pub const GGP_FRAME_TOKEN_SPEC_VERSION: u32 = 1;
///<s>Vulkan Manual Page</s> · Constant
#[doc(alias = "VK_GGP_FRAME_TOKEN_EXTENSION_NAME")]
pub const GGP_FRAME_TOKEN_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!(
    "VK_GGP_frame_token"
);
///Provided by [`crate::extensions::ggp_frame_token`]
impl crate::vk1_0::StructureType {
    pub const PRESENT_FRAME_TOKEN_GGP: Self = Self(1000191000);
}
impl<'a> crate::ExtendableFrom<'a, PresentFrameTokenGGP>
for crate::extensions::khr_swapchain::PresentInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PresentFrameTokenGGPBuilder<'_>>
for crate::extensions::khr_swapchain::PresentInfoKHRBuilder<'a> {}
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPresentFrameTokenGGP.html) · Structure
#[doc(alias = "VkPresentFrameTokenGGP")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PresentFrameTokenGGP {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub frame_token: u64,
}
impl PresentFrameTokenGGP {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PRESENT_FRAME_TOKEN_GGP;
}
impl Default for PresentFrameTokenGGP {
    fn default() -> Self {
        Self {
            s_type: Self::STRUCTURE_TYPE,
            p_next: std::ptr::null(),
            frame_token: Default::default(),
        }
    }
}
impl std::fmt::Debug for PresentFrameTokenGGP {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PresentFrameTokenGGP")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("frame_token", &self.frame_token)
            .finish()
    }
}
impl PresentFrameTokenGGP {
    #[inline]
    pub fn into_builder<'a>(self) -> PresentFrameTokenGGPBuilder<'a> {
        PresentFrameTokenGGPBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPresentFrameTokenGGP.html) · Builder of [`PresentFrameTokenGGP`]
#[repr(transparent)]
pub struct PresentFrameTokenGGPBuilder<'a>(
    PresentFrameTokenGGP,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PresentFrameTokenGGPBuilder<'a> {
    #[inline]
    pub fn new() -> PresentFrameTokenGGPBuilder<'a> {
        PresentFrameTokenGGPBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn frame_token(mut self, frame_token: u64) -> Self {
        self.0.frame_token = frame_token as _;
        self
    }
    #[inline]
    /// Discards all lifetime information.
    /// Use the `Deref` and `DerefMut` implementations if possible.
    pub fn build_dangling(self) -> PresentFrameTokenGGP {
        self.0
    }
}
impl<'a> std::default::Default for PresentFrameTokenGGPBuilder<'a> {
    fn default() -> PresentFrameTokenGGPBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PresentFrameTokenGGPBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PresentFrameTokenGGPBuilder<'a> {
    type Target = PresentFrameTokenGGP;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PresentFrameTokenGGPBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
