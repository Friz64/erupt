#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_CLIP_SPACE_W_SCALING_SPEC_VERSION")]
pub const NV_CLIP_SPACE_W_SCALING_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_CLIP_SPACE_W_SCALING_EXTENSION_NAME")]
pub const NV_CLIP_SPACE_W_SCALING_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_NV_clip_space_w_scaling");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_SET_VIEWPORT_W_SCALING_NV: *const std::os::raw::c_char = crate::cstr!("vkCmdSetViewportWScalingNV");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetViewportWScalingNV.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetViewportWScalingNV =
    unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, first_viewport: u32, viewport_count: u32, p_viewport_w_scalings: *const crate::extensions::nv_clip_space_w_scaling::ViewportWScalingNV) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkViewportWScalingNV.html) · Structure"]
#[doc(alias = "VkViewportWScalingNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ViewportWScalingNV {
    pub xcoeff: std::os::raw::c_float,
    pub ycoeff: std::os::raw::c_float,
}
impl Default for ViewportWScalingNV {
    fn default() -> Self {
        Self {
            xcoeff: Default::default(),
            ycoeff: Default::default(),
        }
    }
}
impl std::fmt::Debug for ViewportWScalingNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ViewportWScalingNV").field("xcoeff", &self.xcoeff).field("ycoeff", &self.ycoeff).finish()
    }
}
impl ViewportWScalingNV {
    #[inline]
    pub fn into_builder<'a>(self) -> ViewportWScalingNVBuilder<'a> {
        ViewportWScalingNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkViewportWScalingNV.html) · Builder of [`ViewportWScalingNV`]"]
#[repr(transparent)]
pub struct ViewportWScalingNVBuilder<'a>(ViewportWScalingNV, std::marker::PhantomData<&'a ()>);
impl<'a> ViewportWScalingNVBuilder<'a> {
    #[inline]
    pub fn new() -> ViewportWScalingNVBuilder<'a> {
        ViewportWScalingNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn xcoeff(mut self, xcoeff: std::os::raw::c_float) -> Self {
        self.0.xcoeff = xcoeff as _;
        self
    }
    #[inline]
    pub fn ycoeff(mut self, ycoeff: std::os::raw::c_float) -> Self {
        self.0.ycoeff = ycoeff as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ViewportWScalingNV {
        self.0
    }
}
impl<'a> std::default::Default for ViewportWScalingNVBuilder<'a> {
    fn default() -> ViewportWScalingNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ViewportWScalingNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ViewportWScalingNVBuilder<'a> {
    type Target = ViewportWScalingNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ViewportWScalingNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<ViewportWScalingNV> for ViewportWScalingNVBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineViewportWScalingStateCreateInfoNV.html) · Structure"]
#[doc(alias = "VkPipelineViewportWScalingStateCreateInfoNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineViewportWScalingStateCreateInfoNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub viewport_w_scaling_enable: crate::vk1_0::Bool32,
    pub viewport_count: u32,
    pub p_viewport_w_scalings: *const crate::extensions::nv_clip_space_w_scaling::ViewportWScalingNV,
}
impl Default for PipelineViewportWScalingStateCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV,
            p_next: std::ptr::null(),
            viewport_w_scaling_enable: Default::default(),
            viewport_count: Default::default(),
            p_viewport_w_scalings: std::ptr::null(),
        }
    }
}
impl std::fmt::Debug for PipelineViewportWScalingStateCreateInfoNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PipelineViewportWScalingStateCreateInfoNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("viewport_w_scaling_enable", &(self.viewport_w_scaling_enable != 0))
            .field("viewport_count", &self.viewport_count)
            .field("p_viewport_w_scalings", &self.p_viewport_w_scalings)
            .finish()
    }
}
impl PipelineViewportWScalingStateCreateInfoNV {
    #[inline]
    pub fn into_builder<'a>(self) -> PipelineViewportWScalingStateCreateInfoNVBuilder<'a> {
        PipelineViewportWScalingStateCreateInfoNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineViewportWScalingStateCreateInfoNV.html) · Builder of [`PipelineViewportWScalingStateCreateInfoNV`]"]
#[repr(transparent)]
pub struct PipelineViewportWScalingStateCreateInfoNVBuilder<'a>(PipelineViewportWScalingStateCreateInfoNV, std::marker::PhantomData<&'a ()>);
impl<'a> PipelineViewportWScalingStateCreateInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineViewportWScalingStateCreateInfoNVBuilder<'a> {
        PipelineViewportWScalingStateCreateInfoNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn viewport_w_scaling_enable(mut self, viewport_w_scaling_enable: bool) -> Self {
        self.0.viewport_w_scaling_enable = viewport_w_scaling_enable as _;
        self
    }
    #[inline]
    pub fn viewport_w_scalings(mut self, viewport_w_scalings: &'a [impl crate::Repr<crate::extensions::nv_clip_space_w_scaling::ViewportWScalingNV>]) -> Self {
        self.0.p_viewport_w_scalings = viewport_w_scalings.as_ptr() as _;
        self.0.viewport_count = viewport_w_scalings.len() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PipelineViewportWScalingStateCreateInfoNV {
        self.0
    }
}
impl<'a> std::default::Default for PipelineViewportWScalingStateCreateInfoNVBuilder<'a> {
    fn default() -> PipelineViewportWScalingStateCreateInfoNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PipelineViewportWScalingStateCreateInfoNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PipelineViewportWScalingStateCreateInfoNVBuilder<'a> {
    type Target = PipelineViewportWScalingStateCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineViewportWScalingStateCreateInfoNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<PipelineViewportWScalingStateCreateInfoNV> for PipelineViewportWScalingStateCreateInfoNVBuilder<'_> {}
#[doc = "Provided by [`crate::extensions::nv_clip_space_w_scaling`]"]
impl crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetViewportWScalingNV.html) · Function"]
    #[doc(alias = "vkCmdSetViewportWScalingNV")]
    pub unsafe fn cmd_set_viewport_w_scaling_nv(&self, command_buffer: crate::vk1_0::CommandBuffer, first_viewport: u32, viewport_w_scalings: &[impl crate::Repr<crate::extensions::nv_clip_space_w_scaling::ViewportWScalingNV>]) -> () {
        let _function = self.cmd_set_viewport_w_scaling_nv.expect("`cmd_set_viewport_w_scaling_nv` is not loaded");
        let viewport_count = viewport_w_scalings.len();
        let _return = _function(command_buffer as _, first_viewport as _, viewport_count as _, viewport_w_scalings.as_ptr() as _);
        ()
    }
}
