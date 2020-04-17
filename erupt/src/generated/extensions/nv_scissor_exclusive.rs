# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_NV_scissor_exclusive.html)\n\n## Extends\n- [`DynamicState`](../../vk1_0/struct.DynamicState.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NV_SCISSOR_EXCLUSIVE_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NV_SCISSOR_EXCLUSIVE_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_NV_scissor_exclusive");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetExclusiveScissorNV.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetExclusiveScissorNV = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    first_exclusive_scissor: u32,
    exclusive_scissor_count: u32,
    p_exclusive_scissors: *const crate::vk1_0::Rect2D,
) -> std::ffi::c_void;
#[doc = "Provides Device Commands for [`NvScissorExclusiveDeviceLoaderExt`](trait.NvScissorExclusiveDeviceLoaderExt.html)"]
pub struct NvScissorExclusiveDeviceCommands {
    pub cmd_set_exclusive_scissor_nv: PFN_vkCmdSetExclusiveScissorNV,
}
impl NvScissorExclusiveDeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<NvScissorExclusiveDeviceCommands> {
        unsafe {
            Some(NvScissorExclusiveDeviceCommands {
                cmd_set_exclusive_scissor_nv: std::mem::transmute(
                    loader.symbol("vkCmdSetExclusiveScissorNV")?,
                ),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`NvScissorExclusiveDeviceCommands`](struct.NvScissorExclusiveDeviceCommands.html)"]
pub trait NvScissorExclusiveDeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetExclusiveScissorNV.html) · Device Command"]
    unsafe fn cmd_set_exclusive_scissor_nv(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        first_exclusive_scissor: u32,
        exclusive_scissors: &[crate::vk1_0::Rect2DBuilder],
    ) -> ();
}
impl NvScissorExclusiveDeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetExclusiveScissorNV.html) · Device Command"]
    unsafe fn cmd_set_exclusive_scissor_nv(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        first_exclusive_scissor: u32,
        exclusive_scissors: &[crate::vk1_0::Rect2DBuilder],
    ) -> () {
        let function = self
            .nv_scissor_exclusive
            .as_ref()
            .expect("`nv_scissor_exclusive` not loaded")
            .cmd_set_exclusive_scissor_nv;
        let exclusive_scissor_count = exclusive_scissors.len() as _;
        let _val = function(
            command_buffer,
            first_exclusive_scissor,
            exclusive_scissor_count,
            exclusive_scissors.as_ptr() as _,
        );
        ()
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineViewportExclusiveScissorStateCreateInfoNV.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineViewportExclusiveScissorStateCreateInfoNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub exclusive_scissor_count: u32,
    pub p_exclusive_scissors: *const crate::vk1_0::Rect2D,
}
impl PipelineViewportExclusiveScissorStateCreateInfoNV {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPipelineViewportExclusiveScissorStateCreateInfoNV,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PipelineViewportExclusiveScissorStateCreateInfoNVBuilder<'a> {
        PipelineViewportExclusiveScissorStateCreateInfoNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PipelineViewportExclusiveScissorStateCreateInfoNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PipelineViewportExclusiveScissorStateCreateInfoNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("exclusive_scissor_count", &self.exclusive_scissor_count)
            .field("p_exclusive_scissors", &self.p_exclusive_scissors)
            .finish()
    }
}
impl Default for PipelineViewportExclusiveScissorStateCreateInfoNV {
    fn default() -> PipelineViewportExclusiveScissorStateCreateInfoNV {
        PipelineViewportExclusiveScissorStateCreateInfoNV { s_type : crate :: vk1_0 :: StructureType :: PIPELINE_VIEWPORT_EXCLUSIVE_SCISSOR_STATE_CREATE_INFO_NV , p_next : std :: ptr :: null ( ) , exclusive_scissor_count : Default :: default ( ) , p_exclusive_scissors : std :: ptr :: null ( ) , }
    }
}
#[doc = "Used by [`PipelineViewportExclusiveScissorStateCreateInfoNV::extend`](struct.PipelineViewportExclusiveScissorStateCreateInfoNV.html#method.extend)"]
pub trait ExtendableByPipelineViewportExclusiveScissorStateCreateInfoNV {}
impl ExtendableByPipelineViewportExclusiveScissorStateCreateInfoNV
    for crate::vk1_0::PipelineViewportStateCreateInfo
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PipelineViewportExclusiveScissorStateCreateInfoNV`](struct.PipelineViewportExclusiveScissorStateCreateInfoNV.html)"]
#[repr(transparent)]
pub struct PipelineViewportExclusiveScissorStateCreateInfoNVBuilder<'a>(
    PipelineViewportExclusiveScissorStateCreateInfoNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PipelineViewportExclusiveScissorStateCreateInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineViewportExclusiveScissorStateCreateInfoNVBuilder<'a> {
        PipelineViewportExclusiveScissorStateCreateInfoNVBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn exclusive_scissors(
        mut self,
        exclusive_scissors: &'a [crate::vk1_0::Rect2DBuilder],
    ) -> Self {
        self.0.exclusive_scissor_count = exclusive_scissors.len() as _;
        self.0.p_exclusive_scissors = exclusive_scissors.as_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PipelineViewportExclusiveScissorStateCreateInfoNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for PipelineViewportExclusiveScissorStateCreateInfoNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PipelineViewportExclusiveScissorStateCreateInfoNVBuilder<'a> {
    type Target = PipelineViewportExclusiveScissorStateCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineViewportExclusiveScissorStateCreateInfoNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceExclusiveScissorFeaturesNV.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceExclusiveScissorFeaturesNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub exclusive_scissor: crate::vk1_0::Bool32,
}
impl PhysicalDeviceExclusiveScissorFeaturesNV {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceExclusiveScissorFeaturesNV,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceExclusiveScissorFeaturesNVBuilder<'a> {
        PhysicalDeviceExclusiveScissorFeaturesNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceExclusiveScissorFeaturesNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceExclusiveScissorFeaturesNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("exclusive_scissor", &(self.exclusive_scissor != 0))
            .finish()
    }
}
impl Default for PhysicalDeviceExclusiveScissorFeaturesNV {
    fn default() -> PhysicalDeviceExclusiveScissorFeaturesNV {
        PhysicalDeviceExclusiveScissorFeaturesNV {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_EXCLUSIVE_SCISSOR_FEATURES_NV,
            p_next: std::ptr::null_mut(),
            exclusive_scissor: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceExclusiveScissorFeaturesNV::extend`](struct.PhysicalDeviceExclusiveScissorFeaturesNV.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceExclusiveScissorFeaturesNV {}
impl ExtendableByPhysicalDeviceExclusiveScissorFeaturesNV
    for crate::vk1_1::PhysicalDeviceFeatures2
{
}
impl ExtendableByPhysicalDeviceExclusiveScissorFeaturesNV for crate::vk1_0::DeviceCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceExclusiveScissorFeaturesNV`](struct.PhysicalDeviceExclusiveScissorFeaturesNV.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceExclusiveScissorFeaturesNVBuilder<'a>(
    PhysicalDeviceExclusiveScissorFeaturesNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceExclusiveScissorFeaturesNVBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceExclusiveScissorFeaturesNVBuilder<'a> {
        PhysicalDeviceExclusiveScissorFeaturesNVBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn exclusive_scissor(mut self, exclusive_scissor: bool) -> Self {
        self.0.exclusive_scissor = exclusive_scissor as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceExclusiveScissorFeaturesNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceExclusiveScissorFeaturesNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceExclusiveScissorFeaturesNVBuilder<'a> {
    type Target = PhysicalDeviceExclusiveScissorFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceExclusiveScissorFeaturesNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
