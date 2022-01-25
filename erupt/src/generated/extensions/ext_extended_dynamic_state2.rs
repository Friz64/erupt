// DO NOT EDIT: @generated by erupt's generator
///<s>Vulkan Manual Page</s> · Constant
#[doc(alias = "VK_EXT_EXTENDED_DYNAMIC_STATE_2_SPEC_VERSION")]
pub const EXT_EXTENDED_DYNAMIC_STATE_2_SPEC_VERSION: u32 = 1;
///<s>Vulkan Manual Page</s> · Constant
#[doc(alias = "VK_EXT_EXTENDED_DYNAMIC_STATE_2_EXTENSION_NAME")]
pub const EXT_EXTENDED_DYNAMIC_STATE_2_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!(
    "VK_EXT_extended_dynamic_state2"
);
///<s>Vulkan Manual Page</s> · Constant
pub const FN_CMD_SET_PATCH_CONTROL_POINTS_EXT: *const std::os::raw::c_char = crate::cstr!(
    "vkCmdSetPatchControlPointsEXT"
);
///<s>Vulkan Manual Page</s> · Constant
pub const FN_CMD_SET_LOGIC_OP_EXT: *const std::os::raw::c_char = crate::cstr!(
    "vkCmdSetLogicOpEXT"
);
///<s>Vulkan Manual Page</s> · Constant
pub const FN_CMD_SET_RASTERIZER_DISCARD_ENABLE_EXT: *const std::os::raw::c_char = crate::cstr!(
    "vkCmdSetRasterizerDiscardEnableEXT"
);
///<s>Vulkan Manual Page</s> · Constant
pub const FN_CMD_SET_DEPTH_BIAS_ENABLE_EXT: *const std::os::raw::c_char = crate::cstr!(
    "vkCmdSetDepthBiasEnableEXT"
);
///<s>Vulkan Manual Page</s> · Constant
pub const FN_CMD_SET_PRIMITIVE_RESTART_ENABLE_EXT: *const std::os::raw::c_char = crate::cstr!(
    "vkCmdSetPrimitiveRestartEnableEXT"
);
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetRasterizerDiscardEnableEXT.html) · Alias
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetRasterizerDiscardEnableEXT = crate::vk1_3::PFN_vkCmdSetRasterizerDiscardEnable;
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBiasEnableEXT.html) · Alias
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDepthBiasEnableEXT = crate::vk1_3::PFN_vkCmdSetDepthBiasEnable;
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPrimitiveRestartEnableEXT.html) · Alias
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetPrimitiveRestartEnableEXT = crate::vk1_3::PFN_vkCmdSetPrimitiveRestartEnable;
///Provided by [`crate::extensions::ext_extended_dynamic_state2`]
impl crate::vk1_0::DynamicState {
    pub const PATCH_CONTROL_POINTS_EXT: Self = Self(1000377000);
    pub const LOGIC_OP_EXT: Self = Self(1000377003);
    pub const RASTERIZER_DISCARD_ENABLE_EXT: Self = Self::RASTERIZER_DISCARD_ENABLE;
    pub const DEPTH_BIAS_ENABLE_EXT: Self = Self::DEPTH_BIAS_ENABLE;
    pub const PRIMITIVE_RESTART_ENABLE_EXT: Self = Self::PRIMITIVE_RESTART_ENABLE;
}
///Provided by [`crate::extensions::ext_extended_dynamic_state2`]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_2_FEATURES_EXT: Self = Self(
        1000377000,
    );
}
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPatchControlPointsEXT.html) · Function
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetPatchControlPointsEXT = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    patch_control_points: u32,
) -> ();
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetLogicOpEXT.html) · Function
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetLogicOpEXT = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    logic_op: crate::vk1_0::LogicOp,
) -> ();
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceExtendedDynamicState2FeaturesEXT>
for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<
    'a,
> crate::ExtendableFrom<'a, PhysicalDeviceExtendedDynamicState2FeaturesEXTBuilder<'_>>
for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceExtendedDynamicState2FeaturesEXT>
for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<
    'a,
> crate::ExtendableFrom<'a, PhysicalDeviceExtendedDynamicState2FeaturesEXTBuilder<'_>>
for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExtendedDynamicState2FeaturesEXT.html) · Structure
#[doc(alias = "VkPhysicalDeviceExtendedDynamicState2FeaturesEXT")]
#[derive(Copy, Clone, )]
#[repr(C)]
pub struct PhysicalDeviceExtendedDynamicState2FeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub extended_dynamic_state2: crate::vk1_0::Bool32,
    pub extended_dynamic_state2_logic_op: crate::vk1_0::Bool32,
    pub extended_dynamic_state2_patch_control_points: crate::vk1_0::Bool32,
}
impl PhysicalDeviceExtendedDynamicState2FeaturesEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_2_FEATURES_EXT;
}
impl Default for PhysicalDeviceExtendedDynamicState2FeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: Self::STRUCTURE_TYPE,
            p_next: std::ptr::null_mut(),
            extended_dynamic_state2: Default::default(),
            extended_dynamic_state2_logic_op: Default::default(),
            extended_dynamic_state2_patch_control_points: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceExtendedDynamicState2FeaturesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f
            .debug_struct("PhysicalDeviceExtendedDynamicState2FeaturesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("extended_dynamic_state2", &(self.extended_dynamic_state2 != 0))
            .field(
                "extended_dynamic_state2_logic_op",
                &(self.extended_dynamic_state2_logic_op != 0),
            )
            .field(
                "extended_dynamic_state2_patch_control_points",
                &(self.extended_dynamic_state2_patch_control_points != 0),
            )
            .finish()
    }
}
impl PhysicalDeviceExtendedDynamicState2FeaturesEXT {
    #[inline]
    pub fn into_builder<'a>(
        self,
    ) -> PhysicalDeviceExtendedDynamicState2FeaturesEXTBuilder<'a> {
        PhysicalDeviceExtendedDynamicState2FeaturesEXTBuilder(
            self,
            std::marker::PhantomData,
        )
    }
}
#[derive(Copy, Clone)]
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExtendedDynamicState2FeaturesEXT.html) · Builder of [`PhysicalDeviceExtendedDynamicState2FeaturesEXT`]
#[repr(transparent)]
pub struct PhysicalDeviceExtendedDynamicState2FeaturesEXTBuilder<'a>(
    PhysicalDeviceExtendedDynamicState2FeaturesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceExtendedDynamicState2FeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceExtendedDynamicState2FeaturesEXTBuilder<'a> {
        PhysicalDeviceExtendedDynamicState2FeaturesEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[inline]
    #[must_use]
    pub fn extended_dynamic_state2(mut self, extended_dynamic_state2: bool) -> Self {
        self.0.extended_dynamic_state2 = extended_dynamic_state2 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn extended_dynamic_state2_logic_op(
        mut self,
        extended_dynamic_state2_logic_op: bool,
    ) -> Self {
        self.0.extended_dynamic_state2_logic_op = extended_dynamic_state2_logic_op as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn extended_dynamic_state2_patch_control_points(
        mut self,
        extended_dynamic_state2_patch_control_points: bool,
    ) -> Self {
        self
            .0
            .extended_dynamic_state2_patch_control_points = extended_dynamic_state2_patch_control_points
            as _;
        self
    }
    #[inline]
    /// Discards all lifetime information.
    /// Use the `Deref` and `DerefMut` implementations if possible.
    pub fn build_dangling(self) -> PhysicalDeviceExtendedDynamicState2FeaturesEXT {
        self.0
    }
}
impl<'a> std::default::Default
for PhysicalDeviceExtendedDynamicState2FeaturesEXTBuilder<'a> {
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
impl<'a> std::ops::DerefMut
for PhysicalDeviceExtendedDynamicState2FeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
///Provided by [`crate::extensions::ext_extended_dynamic_state2`]
impl crate::DeviceLoader {
    #[inline]
    #[track_caller]
    ///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPatchControlPointsEXT.html) · Function
    #[doc(alias = "vkCmdSetPatchControlPointsEXT")]
    pub unsafe fn cmd_set_patch_control_points_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        patch_control_points: u32,
    ) -> () {
        let _function = self
            .cmd_set_patch_control_points_ext
            .expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(command_buffer as _, patch_control_points as _);
        ()
    }
    #[inline]
    #[track_caller]
    ///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetLogicOpEXT.html) · Function
    #[doc(alias = "vkCmdSetLogicOpEXT")]
    pub unsafe fn cmd_set_logic_op_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        logic_op: crate::vk1_0::LogicOp,
    ) -> () {
        let _function = self.cmd_set_logic_op_ext.expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(command_buffer as _, logic_op as _);
        ()
    }
    #[inline]
    #[track_caller]
    ///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetRasterizerDiscardEnableEXT.html) · Function
    #[doc(alias = "vkCmdSetRasterizerDiscardEnableEXT")]
    pub unsafe fn cmd_set_rasterizer_discard_enable_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        rasterizer_discard_enable: bool,
    ) -> () {
        let _function = self
            .cmd_set_rasterizer_discard_enable_ext
            .expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(command_buffer as _, rasterizer_discard_enable as _);
        ()
    }
    #[inline]
    #[track_caller]
    ///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBiasEnableEXT.html) · Function
    #[doc(alias = "vkCmdSetDepthBiasEnableEXT")]
    pub unsafe fn cmd_set_depth_bias_enable_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        depth_bias_enable: bool,
    ) -> () {
        let _function = self
            .cmd_set_depth_bias_enable_ext
            .expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(command_buffer as _, depth_bias_enable as _);
        ()
    }
    #[inline]
    #[track_caller]
    ///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPrimitiveRestartEnableEXT.html) · Function
    #[doc(alias = "vkCmdSetPrimitiveRestartEnableEXT")]
    pub unsafe fn cmd_set_primitive_restart_enable_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        primitive_restart_enable: bool,
    ) -> () {
        let _function = self
            .cmd_set_primitive_restart_enable_ext
            .expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(command_buffer as _, primitive_restart_enable as _);
        ()
    }
}
