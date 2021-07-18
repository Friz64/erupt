#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_INHERITED_VIEWPORT_SCISSOR_SPEC_VERSION")]
pub const NV_INHERITED_VIEWPORT_SCISSOR_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_INHERITED_VIEWPORT_SCISSOR_EXTENSION_NAME")]
pub const NV_INHERITED_VIEWPORT_SCISSOR_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_NV_inherited_viewport_scissor");
#[doc = "Provided by [`crate::extensions::nv_inherited_viewport_scissor`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_INHERITED_VIEWPORT_SCISSOR_FEATURES_NV: Self = Self(1000278000);
    pub const COMMAND_BUFFER_INHERITANCE_VIEWPORT_SCISSOR_INFO_NV: Self = Self(1000278001);
}
impl<'a> crate::ExtendableFromConst<'a, PhysicalDeviceInheritedViewportScissorFeaturesNV> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, PhysicalDeviceInheritedViewportScissorFeaturesNVBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, CommandBufferInheritanceViewportScissorInfoNV> for crate::vk1_0::CommandBufferInheritanceInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, CommandBufferInheritanceViewportScissorInfoNVBuilder<'_>> for crate::vk1_0::CommandBufferInheritanceInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceInheritedViewportScissorFeaturesNV> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceInheritedViewportScissorFeaturesNVBuilder<'_>> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceInheritedViewportScissorFeaturesNV.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceInheritedViewportScissorFeaturesNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceInheritedViewportScissorFeaturesNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub inherited_viewport_scissor2_d: crate::vk1_0::Bool32,
}
impl PhysicalDeviceInheritedViewportScissorFeaturesNV {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_INHERITED_VIEWPORT_SCISSOR_FEATURES_NV;
}
impl Default for PhysicalDeviceInheritedViewportScissorFeaturesNV {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_INHERITED_VIEWPORT_SCISSOR_FEATURES_NV, p_next: std::ptr::null_mut(), inherited_viewport_scissor2_d: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceInheritedViewportScissorFeaturesNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceInheritedViewportScissorFeaturesNV").field("s_type", &self.s_type).field("p_next", &self.p_next).field("inherited_viewport_scissor2_d", &(self.inherited_viewport_scissor2_d != 0)).finish()
    }
}
impl PhysicalDeviceInheritedViewportScissorFeaturesNV {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceInheritedViewportScissorFeaturesNVBuilder<'a> {
        PhysicalDeviceInheritedViewportScissorFeaturesNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceInheritedViewportScissorFeaturesNV.html) · Builder of [`PhysicalDeviceInheritedViewportScissorFeaturesNV`]"]
#[repr(transparent)]
pub struct PhysicalDeviceInheritedViewportScissorFeaturesNVBuilder<'a>(PhysicalDeviceInheritedViewportScissorFeaturesNV, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceInheritedViewportScissorFeaturesNVBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceInheritedViewportScissorFeaturesNVBuilder<'a> {
        PhysicalDeviceInheritedViewportScissorFeaturesNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn inherited_viewport_scissor2_d(mut self, inherited_viewport_scissor2_d: bool) -> Self {
        self.0.inherited_viewport_scissor2_d = inherited_viewport_scissor2_d as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceInheritedViewportScissorFeaturesNV {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceInheritedViewportScissorFeaturesNVBuilder<'a> {
    fn default() -> PhysicalDeviceInheritedViewportScissorFeaturesNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceInheritedViewportScissorFeaturesNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceInheritedViewportScissorFeaturesNVBuilder<'a> {
    type Target = PhysicalDeviceInheritedViewportScissorFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceInheritedViewportScissorFeaturesNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandBufferInheritanceViewportScissorInfoNV.html) · Structure"]
#[doc(alias = "VkCommandBufferInheritanceViewportScissorInfoNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CommandBufferInheritanceViewportScissorInfoNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub viewport_scissor2_d: crate::vk1_0::Bool32,
    pub viewport_depth_count: u32,
    pub p_viewport_depths: *const crate::vk1_0::Viewport,
}
impl CommandBufferInheritanceViewportScissorInfoNV {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::COMMAND_BUFFER_INHERITANCE_VIEWPORT_SCISSOR_INFO_NV;
}
impl Default for CommandBufferInheritanceViewportScissorInfoNV {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::COMMAND_BUFFER_INHERITANCE_VIEWPORT_SCISSOR_INFO_NV, p_next: std::ptr::null(), viewport_scissor2_d: Default::default(), viewport_depth_count: Default::default(), p_viewport_depths: std::ptr::null() }
    }
}
impl std::fmt::Debug for CommandBufferInheritanceViewportScissorInfoNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CommandBufferInheritanceViewportScissorInfoNV").field("s_type", &self.s_type).field("p_next", &self.p_next).field("viewport_scissor2_d", &(self.viewport_scissor2_d != 0)).field("viewport_depth_count", &self.viewport_depth_count).field("p_viewport_depths", &self.p_viewport_depths).finish()
    }
}
impl CommandBufferInheritanceViewportScissorInfoNV {
    #[inline]
    pub fn into_builder<'a>(self) -> CommandBufferInheritanceViewportScissorInfoNVBuilder<'a> {
        CommandBufferInheritanceViewportScissorInfoNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandBufferInheritanceViewportScissorInfoNV.html) · Builder of [`CommandBufferInheritanceViewportScissorInfoNV`]"]
#[repr(transparent)]
pub struct CommandBufferInheritanceViewportScissorInfoNVBuilder<'a>(CommandBufferInheritanceViewportScissorInfoNV, std::marker::PhantomData<&'a ()>);
impl<'a> CommandBufferInheritanceViewportScissorInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> CommandBufferInheritanceViewportScissorInfoNVBuilder<'a> {
        CommandBufferInheritanceViewportScissorInfoNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn viewport_scissor2_d(mut self, viewport_scissor2_d: bool) -> Self {
        self.0.viewport_scissor2_d = viewport_scissor2_d as _;
        self
    }
    #[inline]
    pub fn viewport_depth_count(mut self, viewport_depth_count: u32) -> Self {
        self.0.viewport_depth_count = viewport_depth_count as _;
        self
    }
    #[inline]
    pub fn viewport_depths(mut self, viewport_depths: &'a crate::vk1_0::Viewport) -> Self {
        self.0.p_viewport_depths = viewport_depths as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> CommandBufferInheritanceViewportScissorInfoNV {
        self.0
    }
}
impl<'a> std::default::Default for CommandBufferInheritanceViewportScissorInfoNVBuilder<'a> {
    fn default() -> CommandBufferInheritanceViewportScissorInfoNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for CommandBufferInheritanceViewportScissorInfoNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for CommandBufferInheritanceViewportScissorInfoNVBuilder<'a> {
    type Target = CommandBufferInheritanceViewportScissorInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for CommandBufferInheritanceViewportScissorInfoNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
