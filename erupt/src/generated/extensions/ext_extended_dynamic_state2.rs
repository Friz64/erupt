#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_EXTENDED_DYNAMIC_STATE_2_SPEC_VERSION")]
pub const EXT_EXTENDED_DYNAMIC_STATE_2_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_EXTENDED_DYNAMIC_STATE_2_EXTENSION_NAME")]
pub const EXT_EXTENDED_DYNAMIC_STATE_2_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_extended_dynamic_state2");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_SET_PATCH_CONTROL_POINTS_EXT: *const std::os::raw::c_char = crate::cstr!("vkCmdSetPatchControlPointsEXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_SET_RASTERIZER_DISCARD_ENABLE_EXT: *const std::os::raw::c_char = crate::cstr!("vkCmdSetRasterizerDiscardEnableEXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_SET_DEPTH_BIAS_ENABLE_EXT: *const std::os::raw::c_char = crate::cstr!("vkCmdSetDepthBiasEnableEXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_SET_LOGIC_OP_EXT: *const std::os::raw::c_char = crate::cstr!("vkCmdSetLogicOpEXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_SET_PRIMITIVE_RESTART_ENABLE_EXT: *const std::os::raw::c_char = crate::cstr!("vkCmdSetPrimitiveRestartEnableEXT");
#[doc = "Provided by [`crate::extensions::ext_extended_dynamic_state2`]"]
impl crate::vk1_0::DynamicState {
    pub const PATCH_CONTROL_POINTS_EXT: Self = Self(1000377000);
    pub const RASTERIZER_DISCARD_ENABLE_EXT: Self = Self(1000377001);
    pub const DEPTH_BIAS_ENABLE_EXT: Self = Self(1000377002);
    pub const LOGIC_OP_EXT: Self = Self(1000377003);
    pub const PRIMITIVE_RESTART_ENABLE_EXT: Self = Self(1000377004);
}
#[doc = "Provided by [`crate::extensions::ext_extended_dynamic_state2`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_2_FEATURES_EXT: Self = Self(1000377000);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetPatchControlPointsEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetPatchControlPointsEXT = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, patch_control_points: u32) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetRasterizerDiscardEnableEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetRasterizerDiscardEnableEXT = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, rasterizer_discard_enable: crate::vk1_0::Bool32) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDepthBiasEnableEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDepthBiasEnableEXT = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, depth_bias_enable: crate::vk1_0::Bool32) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetLogicOpEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetLogicOpEXT = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, logic_op: crate::vk1_0::LogicOp) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetPrimitiveRestartEnableEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetPrimitiveRestartEnableEXT = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, primitive_restart_enable: crate::vk1_0::Bool32) -> ();
impl<'a> crate::ExtendableFromConst<'a, PhysicalDeviceExtendedDynamicState2FeaturesEXT> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, PhysicalDeviceExtendedDynamicState2FeaturesEXTBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceExtendedDynamicState2FeaturesEXT> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceExtendedDynamicState2FeaturesEXTBuilder<'_>> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceExtendedDynamicState2FeaturesEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceExtendedDynamicState2FeaturesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceExtendedDynamicState2FeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub extended_dynamic_state2: crate::vk1_0::Bool32,
    pub extended_dynamic_state2_logic_op: crate::vk1_0::Bool32,
    pub extended_dynamic_state2_patch_control_points: crate::vk1_0::Bool32,
}
impl Default for PhysicalDeviceExtendedDynamicState2FeaturesEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_2_FEATURES_EXT, p_next: std::ptr::null_mut(), extended_dynamic_state2: Default::default(), extended_dynamic_state2_logic_op: Default::default(), extended_dynamic_state2_patch_control_points: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceExtendedDynamicState2FeaturesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceExtendedDynamicState2FeaturesEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("extended_dynamic_state2", &(self.extended_dynamic_state2 != 0)).field("extended_dynamic_state2_logic_op", &(self.extended_dynamic_state2_logic_op != 0)).field("extended_dynamic_state2_patch_control_points", &(self.extended_dynamic_state2_patch_control_points != 0)).finish()
    }
}
impl PhysicalDeviceExtendedDynamicState2FeaturesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceExtendedDynamicState2FeaturesEXTBuilder<'a> {
        PhysicalDeviceExtendedDynamicState2FeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceExtendedDynamicState2FeaturesEXT.html) · Builder of [`PhysicalDeviceExtendedDynamicState2FeaturesEXT`]"]
#[repr(transparent)]
pub struct PhysicalDeviceExtendedDynamicState2FeaturesEXTBuilder<'a>(PhysicalDeviceExtendedDynamicState2FeaturesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceExtendedDynamicState2FeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceExtendedDynamicState2FeaturesEXTBuilder<'a> {
        PhysicalDeviceExtendedDynamicState2FeaturesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn extended_dynamic_state2(mut self, extended_dynamic_state2: bool) -> Self {
        self.0.extended_dynamic_state2 = extended_dynamic_state2 as _;
        self
    }
    #[inline]
    pub fn extended_dynamic_state2_logic_op(mut self, extended_dynamic_state2_logic_op: bool) -> Self {
        self.0.extended_dynamic_state2_logic_op = extended_dynamic_state2_logic_op as _;
        self
    }
    #[inline]
    pub fn extended_dynamic_state2_patch_control_points(mut self, extended_dynamic_state2_patch_control_points: bool) -> Self {
        self.0.extended_dynamic_state2_patch_control_points = extended_dynamic_state2_patch_control_points as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceExtendedDynamicState2FeaturesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceExtendedDynamicState2FeaturesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceExtendedDynamicState2FeaturesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceExtendedDynamicState2FeaturesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceExtendedDynamicState2FeaturesEXTBuilder<'a> {
    type Target = PhysicalDeviceExtendedDynamicState2FeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceExtendedDynamicState2FeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::ext_extended_dynamic_state2`]"]
impl crate::DeviceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetPatchControlPointsEXT.html) · Function"]
    #[doc(alias = "vkCmdSetPatchControlPointsEXT")]
    pub unsafe fn cmd_set_patch_control_points_ext(&self, command_buffer: crate::vk1_0::CommandBuffer, patch_control_points: u32) -> () {
        let _function = self.cmd_set_patch_control_points_ext.expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(command_buffer as _, patch_control_points as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetRasterizerDiscardEnableEXT.html) · Function"]
    #[doc(alias = "vkCmdSetRasterizerDiscardEnableEXT")]
    pub unsafe fn cmd_set_rasterizer_discard_enable_ext(&self, command_buffer: crate::vk1_0::CommandBuffer, rasterizer_discard_enable: bool) -> () {
        let _function = self.cmd_set_rasterizer_discard_enable_ext.expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(command_buffer as _, rasterizer_discard_enable as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDepthBiasEnableEXT.html) · Function"]
    #[doc(alias = "vkCmdSetDepthBiasEnableEXT")]
    pub unsafe fn cmd_set_depth_bias_enable_ext(&self, command_buffer: crate::vk1_0::CommandBuffer, depth_bias_enable: bool) -> () {
        let _function = self.cmd_set_depth_bias_enable_ext.expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(command_buffer as _, depth_bias_enable as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetLogicOpEXT.html) · Function"]
    #[doc(alias = "vkCmdSetLogicOpEXT")]
    pub unsafe fn cmd_set_logic_op_ext(&self, command_buffer: crate::vk1_0::CommandBuffer, logic_op: crate::vk1_0::LogicOp) -> () {
        let _function = self.cmd_set_logic_op_ext.expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(command_buffer as _, logic_op as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetPrimitiveRestartEnableEXT.html) · Function"]
    #[doc(alias = "vkCmdSetPrimitiveRestartEnableEXT")]
    pub unsafe fn cmd_set_primitive_restart_enable_ext(&self, command_buffer: crate::vk1_0::CommandBuffer, primitive_restart_enable: bool) -> () {
        let _function = self.cmd_set_primitive_restart_enable_ext.expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(command_buffer as _, primitive_restart_enable as _);
        ()
    }
}
