#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const GGP_FRAME_TOKEN_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const GGP_FRAME_TOKEN_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_GGP_frame_token");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPresentFrameTokenGGP.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PresentFrameTokenGGP {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub frame_token: u64,
}
impl Default for PresentFrameTokenGGP {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PRESENT_FRAME_TOKEN_GGP,
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPresentFrameTokenGGP.html) · Builder of [`PresentFrameTokenGGP`](struct.PresentFrameTokenGGP.html)"]
#[repr(transparent)]
pub struct PresentFrameTokenGGPBuilder<'a>(PresentFrameTokenGGP, std::marker::PhantomData<&'a ()>);
impl<'a> PresentFrameTokenGGPBuilder<'a> {
    #[inline]
    pub fn new() -> PresentFrameTokenGGPBuilder<'a> {
        PresentFrameTokenGGPBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn frame_token(mut self, frame_token: u64) -> Self {
        self.0.frame_token = frame_token as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PresentFrameTokenGGP {
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
