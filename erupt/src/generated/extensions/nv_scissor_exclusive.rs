// DO NOT EDIT: @generated by erupt's generator
///<s>Vulkan Manual Page</s> · Constant
#[doc(alias = "VK_NV_SCISSOR_EXCLUSIVE_SPEC_VERSION")]
pub const NV_SCISSOR_EXCLUSIVE_SPEC_VERSION: u32 = 1;
///<s>Vulkan Manual Page</s> · Constant
#[doc(alias = "VK_NV_SCISSOR_EXCLUSIVE_EXTENSION_NAME")]
pub const NV_SCISSOR_EXCLUSIVE_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!(
    "VK_NV_scissor_exclusive"
);
///<s>Vulkan Manual Page</s> · Constant
pub const FN_CMD_SET_EXCLUSIVE_SCISSOR_NV: *const std::os::raw::c_char = crate::cstr!(
    "vkCmdSetExclusiveScissorNV"
);
///Provided by [`crate::extensions::nv_scissor_exclusive`]
impl crate::vk1_0::DynamicState {
    pub const EXCLUSIVE_SCISSOR_NV: Self = Self(1000205001);
}
///Provided by [`crate::extensions::nv_scissor_exclusive`]
impl crate::vk1_0::StructureType {
    pub const PIPELINE_VIEWPORT_EXCLUSIVE_SCISSOR_STATE_CREATE_INFO_NV: Self = Self(
        1000205000,
    );
    pub const PHYSICAL_DEVICE_EXCLUSIVE_SCISSOR_FEATURES_NV: Self = Self(1000205002);
}
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetExclusiveScissorNV.html) · Function
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetExclusiveScissorNV = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    first_exclusive_scissor: u32,
    exclusive_scissor_count: u32,
    p_exclusive_scissors: *const crate::vk1_0::Rect2D,
) -> ();
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceExclusiveScissorFeaturesNV>
for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceExclusiveScissorFeaturesNVBuilder<'_>>
for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PipelineViewportExclusiveScissorStateCreateInfoNV>
for crate::vk1_0::PipelineViewportStateCreateInfoBuilder<'a> {}
impl<
    'a,
> crate::ExtendableFrom<'a, PipelineViewportExclusiveScissorStateCreateInfoNVBuilder<'_>>
for crate::vk1_0::PipelineViewportStateCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceExclusiveScissorFeaturesNV>
for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceExclusiveScissorFeaturesNVBuilder<'_>>
for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExclusiveScissorFeaturesNV.html) · Structure
#[doc(alias = "VkPhysicalDeviceExclusiveScissorFeaturesNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceExclusiveScissorFeaturesNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub exclusive_scissor: crate::vk1_0::Bool32,
}
impl PhysicalDeviceExclusiveScissorFeaturesNV {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_EXCLUSIVE_SCISSOR_FEATURES_NV;
}
impl Default for PhysicalDeviceExclusiveScissorFeaturesNV {
    fn default() -> Self {
        Self {
            s_type: Self::STRUCTURE_TYPE,
            p_next: std::ptr::null_mut(),
            exclusive_scissor: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceExclusiveScissorFeaturesNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceExclusiveScissorFeaturesNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("exclusive_scissor", &(self.exclusive_scissor != 0))
            .finish()
    }
}
impl PhysicalDeviceExclusiveScissorFeaturesNV {
    #[inline]
    pub fn into_builder<'a>(
        self,
    ) -> PhysicalDeviceExclusiveScissorFeaturesNVBuilder<'a> {
        PhysicalDeviceExclusiveScissorFeaturesNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExclusiveScissorFeaturesNV.html) · Builder of [`PhysicalDeviceExclusiveScissorFeaturesNV`]
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
    #[inline]
    #[must_use]
    pub fn exclusive_scissor(mut self, exclusive_scissor: bool) -> Self {
        self.0.exclusive_scissor = exclusive_scissor as _;
        self
    }
    #[inline]
    /// Discards all lifetime information.
    /// Use the `Deref` and `DerefMut` implementations if possible.
    pub fn build_dangling(self) -> PhysicalDeviceExclusiveScissorFeaturesNV {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceExclusiveScissorFeaturesNVBuilder<'a> {
    fn default() -> PhysicalDeviceExclusiveScissorFeaturesNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceExclusiveScissorFeaturesNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportExclusiveScissorStateCreateInfoNV.html) · Structure
#[doc(alias = "VkPipelineViewportExclusiveScissorStateCreateInfoNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineViewportExclusiveScissorStateCreateInfoNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub exclusive_scissor_count: u32,
    pub p_exclusive_scissors: *const crate::vk1_0::Rect2D,
}
impl PipelineViewportExclusiveScissorStateCreateInfoNV {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PIPELINE_VIEWPORT_EXCLUSIVE_SCISSOR_STATE_CREATE_INFO_NV;
}
impl Default for PipelineViewportExclusiveScissorStateCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: Self::STRUCTURE_TYPE,
            p_next: std::ptr::null(),
            exclusive_scissor_count: Default::default(),
            p_exclusive_scissors: std::ptr::null(),
        }
    }
}
impl std::fmt::Debug for PipelineViewportExclusiveScissorStateCreateInfoNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PipelineViewportExclusiveScissorStateCreateInfoNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("exclusive_scissor_count", &self.exclusive_scissor_count)
            .field("p_exclusive_scissors", &self.p_exclusive_scissors)
            .finish()
    }
}
impl PipelineViewportExclusiveScissorStateCreateInfoNV {
    #[inline]
    pub fn into_builder<'a>(
        self,
    ) -> PipelineViewportExclusiveScissorStateCreateInfoNVBuilder<'a> {
        PipelineViewportExclusiveScissorStateCreateInfoNVBuilder(
            self,
            std::marker::PhantomData,
        )
    }
}
#[derive(Copy, Clone)]
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportExclusiveScissorStateCreateInfoNV.html) · Builder of [`PipelineViewportExclusiveScissorStateCreateInfoNV`]
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
    #[inline]
    #[must_use]
    pub fn exclusive_scissors(
        mut self,
        exclusive_scissors: &'a [crate::vk1_0::Rect2DBuilder],
    ) -> Self {
        self.0.p_exclusive_scissors = exclusive_scissors.as_ptr() as _;
        self.0.exclusive_scissor_count = exclusive_scissors.len() as _;
        self
    }
    #[inline]
    /// Discards all lifetime information.
    /// Use the `Deref` and `DerefMut` implementations if possible.
    pub fn build_dangling(self) -> PipelineViewportExclusiveScissorStateCreateInfoNV {
        self.0
    }
}
impl<'a> std::default::Default
for PipelineViewportExclusiveScissorStateCreateInfoNVBuilder<'a> {
    fn default() -> PipelineViewportExclusiveScissorStateCreateInfoNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug
for PipelineViewportExclusiveScissorStateCreateInfoNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref
for PipelineViewportExclusiveScissorStateCreateInfoNVBuilder<'a> {
    type Target = PipelineViewportExclusiveScissorStateCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut
for PipelineViewportExclusiveScissorStateCreateInfoNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
///Provided by [`crate::extensions::nv_scissor_exclusive`]
impl crate::DeviceLoader {
    #[inline]
    #[track_caller]
    ///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetExclusiveScissorNV.html) · Function
    #[doc(alias = "vkCmdSetExclusiveScissorNV")]
    pub unsafe fn cmd_set_exclusive_scissor_nv(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        first_exclusive_scissor: u32,
        exclusive_scissors: &[crate::vk1_0::Rect2DBuilder],
    ) -> () {
        let _function = self
            .cmd_set_exclusive_scissor_nv
            .expect(crate::NOT_LOADED_MESSAGE);
        let exclusive_scissor_count = exclusive_scissors.len();
        let _return = _function(
            command_buffer as _,
            first_exclusive_scissor as _,
            exclusive_scissor_count as _,
            exclusive_scissors.as_ptr() as _,
        );
        ()
    }
}
