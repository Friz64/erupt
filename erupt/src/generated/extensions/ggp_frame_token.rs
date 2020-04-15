# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_GGP_frame_token.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
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
    pub frame_token: u32,
}
impl PresentFrameTokenGGP {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPresentFrameTokenGGP,
    {
        unsafe {
            crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
        }
    }
    #[inline]
    pub fn builder<'a>(self) -> PresentFrameTokenGGPBuilder<'a> {
        PresentFrameTokenGGPBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PresentFrameTokenGGP {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PresentFrameTokenGGP")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("frame_token", &self.frame_token)
            .finish()
    }
}
impl Default for PresentFrameTokenGGP {
    fn default() -> PresentFrameTokenGGP {
        PresentFrameTokenGGP {
            s_type: crate::vk1_0::StructureType::PRESENT_FRAME_TOKEN_GGP,
            p_next: std::ptr::null(),
            frame_token: Default::default(),
        }
    }
}
#[doc = "Used by [`PresentFrameTokenGGP::extend`](struct.PresentFrameTokenGGP.html#method.extend)"]
pub trait ExtendableByPresentFrameTokenGGP {}
impl ExtendableByPresentFrameTokenGGP for crate::extensions::khr_swapchain::PresentInfoKHR {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PresentFrameTokenGGP`](struct.PresentFrameTokenGGP.html)"]
#[repr(transparent)]
pub struct PresentFrameTokenGGPBuilder<'a>(PresentFrameTokenGGP, std::marker::PhantomData<&'a ()>);
impl<'a> PresentFrameTokenGGPBuilder<'a> {
    #[inline]
    pub fn new() -> PresentFrameTokenGGPBuilder<'a> {
        PresentFrameTokenGGPBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn frame_token(mut self, frame_token: u32) -> Self {
        self.0.frame_token = frame_token;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PresentFrameTokenGGP {
        self.0
    }
}
impl<'a> std::fmt::Debug for PresentFrameTokenGGPBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
