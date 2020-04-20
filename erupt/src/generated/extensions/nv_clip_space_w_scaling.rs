# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_NV_clip_space_w_scaling.html)\n\n## Extends\n- [`DynamicState`](../../vk1_0/struct.DynamicState.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NV_CLIP_SPACE_W_SCALING_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NV_CLIP_SPACE_W_SCALING_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_NV_clip_space_w_scaling");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetViewportWScalingNV.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetViewportWScalingNV = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    first_viewport: u32,
    viewport_count: u32,
    p_viewport_w_scalings: *const crate::extensions::nv_clip_space_w_scaling::ViewportWScalingNV,
) -> std::ffi::c_void;
#[doc = "Provides Device Commands for [`NvClipSpaceWScalingDeviceLoaderExt`](trait.NvClipSpaceWScalingDeviceLoaderExt.html)"]
pub struct NvClipSpaceWScalingDeviceCommands {
    pub cmd_set_viewport_w_scaling_nv: PFN_vkCmdSetViewportWScalingNV,
}
impl NvClipSpaceWScalingDeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<NvClipSpaceWScalingDeviceCommands> {
        unsafe {
            Some(NvClipSpaceWScalingDeviceCommands {
                cmd_set_viewport_w_scaling_nv: std::mem::transmute(
                    loader.symbol("vkCmdSetViewportWScalingNV")?,
                ),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`NvClipSpaceWScalingDeviceCommands`](struct.NvClipSpaceWScalingDeviceCommands.html)"]
pub trait NvClipSpaceWScalingDeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetViewportWScalingNV.html) · Device Command"]
    unsafe fn cmd_set_viewport_w_scaling_nv(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        first_viewport: u32,
        viewport_w_scalings : & [ crate :: extensions :: nv_clip_space_w_scaling :: ViewportWScalingNVBuilder ],
    ) -> ();
}
impl NvClipSpaceWScalingDeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetViewportWScalingNV.html) · Device Command"]
    unsafe fn cmd_set_viewport_w_scaling_nv(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        first_viewport: u32,
        viewport_w_scalings : & [ crate :: extensions :: nv_clip_space_w_scaling :: ViewportWScalingNVBuilder ],
    ) -> () {
        let function = self
            .nv_clip_space_w_scaling
            .as_ref()
            .expect("`nv_clip_space_w_scaling` not loaded")
            .cmd_set_viewport_w_scaling_nv;
        let viewport_count = viewport_w_scalings.len() as _;
        let _val = function(
            command_buffer,
            first_viewport,
            viewport_count,
            viewport_w_scalings.as_ptr() as _,
        );
        ()
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkViewportWScalingNV.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ViewportWScalingNV {
    pub xcoeff: f32,
    pub ycoeff: f32,
}
impl ViewportWScalingNV {
    #[inline]
    pub fn builder<'a>(self) -> ViewportWScalingNVBuilder<'a> {
        ViewportWScalingNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ViewportWScalingNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ViewportWScalingNV")
            .field("xcoeff", &self.xcoeff)
            .field("ycoeff", &self.ycoeff)
            .finish()
    }
}
impl Default for ViewportWScalingNV {
    fn default() -> ViewportWScalingNV {
        ViewportWScalingNV {
            xcoeff: Default::default(),
            ycoeff: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkViewportWScalingNV.html) · Builder of [`ViewportWScalingNV`](struct.ViewportWScalingNV.html)"]
#[repr(transparent)]
pub struct ViewportWScalingNVBuilder<'a>(ViewportWScalingNV, std::marker::PhantomData<&'a ()>);
impl<'a> ViewportWScalingNVBuilder<'a> {
    #[inline]
    pub fn new() -> ViewportWScalingNVBuilder<'a> {
        ViewportWScalingNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn xcoeff(mut self, xcoeff: f32) -> Self {
        self.0.xcoeff = xcoeff;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn ycoeff(mut self, ycoeff: f32) -> Self {
        self.0.ycoeff = ycoeff;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> ViewportWScalingNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for ViewportWScalingNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineViewportWScalingStateCreateInfoNV.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineViewportWScalingStateCreateInfoNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub viewport_w_scaling_enable: crate::vk1_0::Bool32,
    pub viewport_count: u32,
    pub p_viewport_w_scalings:
        *const crate::extensions::nv_clip_space_w_scaling::ViewportWScalingNV,
}
impl PipelineViewportWScalingStateCreateInfoNV {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: crate::ExtendableBy<Self>,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PipelineViewportWScalingStateCreateInfoNVBuilder<'a> {
        PipelineViewportWScalingStateCreateInfoNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PipelineViewportWScalingStateCreateInfoNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PipelineViewportWScalingStateCreateInfoNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "viewport_w_scaling_enable",
                &(self.viewport_w_scaling_enable != 0),
            )
            .field("viewport_count", &self.viewport_count)
            .field("p_viewport_w_scalings", &self.p_viewport_w_scalings)
            .finish()
    }
}
impl Default for PipelineViewportWScalingStateCreateInfoNV {
    fn default() -> PipelineViewportWScalingStateCreateInfoNV {
        PipelineViewportWScalingStateCreateInfoNV {
            s_type: crate::vk1_0::StructureType::PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV,
            p_next: std::ptr::null(),
            viewport_w_scaling_enable: Default::default(),
            viewport_count: Default::default(),
            p_viewport_w_scalings: std::ptr::null(),
        }
    }
}
impl crate::ExtendableBy<PipelineViewportWScalingStateCreateInfoNV>
    for crate::vk1_0::PipelineViewportStateCreateInfo
{
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineViewportWScalingStateCreateInfoNV.html) · Builder of [`PipelineViewportWScalingStateCreateInfoNV`](struct.PipelineViewportWScalingStateCreateInfoNV.html)"]
#[repr(transparent)]
pub struct PipelineViewportWScalingStateCreateInfoNVBuilder<'a>(
    PipelineViewportWScalingStateCreateInfoNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PipelineViewportWScalingStateCreateInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineViewportWScalingStateCreateInfoNVBuilder<'a> {
        PipelineViewportWScalingStateCreateInfoNVBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn viewport_w_scaling_enable(mut self, viewport_w_scaling_enable: bool) -> Self {
        self.0.viewport_w_scaling_enable = viewport_w_scaling_enable as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn viewport_w_scalings(
        mut self,
        viewport_w_scalings : &'a [ crate :: extensions :: nv_clip_space_w_scaling :: ViewportWScalingNVBuilder ],
    ) -> Self {
        self.0.viewport_count = viewport_w_scalings.len() as _;
        self.0.p_viewport_w_scalings = viewport_w_scalings.as_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PipelineViewportWScalingStateCreateInfoNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for PipelineViewportWScalingStateCreateInfoNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
