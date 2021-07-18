#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_LINE_RASTERIZATION_SPEC_VERSION")]
pub const EXT_LINE_RASTERIZATION_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_LINE_RASTERIZATION_EXTENSION_NAME")]
pub const EXT_LINE_RASTERIZATION_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_line_rasterization");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_SET_LINE_STIPPLE_EXT: *const std::os::raw::c_char = crate::cstr!("vkCmdSetLineStippleEXT");
#[doc = "Provided by [`crate::extensions::ext_line_rasterization`]"]
impl crate::vk1_0::DynamicState {
    pub const LINE_STIPPLE_EXT: Self = Self(1000259000);
}
#[doc = "Provided by [`crate::extensions::ext_line_rasterization`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES_EXT: Self = Self(1000259000);
    pub const PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO_EXT: Self = Self(1000259001);
    pub const PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES_EXT: Self = Self(1000259002);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkLineRasterizationModeEXT.html) · Enum"]
#[doc(alias = "VkLineRasterizationModeEXT")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct LineRasterizationModeEXT(pub i32);
impl std::fmt::Debug for LineRasterizationModeEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::DEFAULT_EXT => "DEFAULT_EXT",
            &Self::RECTANGULAR_EXT => "RECTANGULAR_EXT",
            &Self::BRESENHAM_EXT => "BRESENHAM_EXT",
            &Self::RECTANGULAR_SMOOTH_EXT => "RECTANGULAR_SMOOTH_EXT",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::ext_line_rasterization`]"]
impl crate::extensions::ext_line_rasterization::LineRasterizationModeEXT {
    pub const DEFAULT_EXT: Self = Self(0);
    pub const RECTANGULAR_EXT: Self = Self(1);
    pub const BRESENHAM_EXT: Self = Self(2);
    pub const RECTANGULAR_SMOOTH_EXT: Self = Self(3);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetLineStippleEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetLineStippleEXT = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, line_stipple_factor: u32, line_stipple_pattern: u16) -> ();
impl<'a> crate::ExtendableFromConst<'a, PhysicalDeviceLineRasterizationFeaturesEXT> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, PhysicalDeviceLineRasterizationFeaturesEXTBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, PipelineRasterizationLineStateCreateInfoEXT> for crate::vk1_0::PipelineRasterizationStateCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, PipelineRasterizationLineStateCreateInfoEXTBuilder<'_>> for crate::vk1_0::PipelineRasterizationStateCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceLineRasterizationFeaturesEXT> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceLineRasterizationFeaturesEXTBuilder<'_>> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceLineRasterizationPropertiesEXT> for crate::vk1_1::PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceLineRasterizationPropertiesEXTBuilder<'_>> for crate::vk1_1::PhysicalDeviceProperties2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceLineRasterizationFeaturesEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceLineRasterizationFeaturesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceLineRasterizationFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub rectangular_lines: crate::vk1_0::Bool32,
    pub bresenham_lines: crate::vk1_0::Bool32,
    pub smooth_lines: crate::vk1_0::Bool32,
    pub stippled_rectangular_lines: crate::vk1_0::Bool32,
    pub stippled_bresenham_lines: crate::vk1_0::Bool32,
    pub stippled_smooth_lines: crate::vk1_0::Bool32,
}
impl PhysicalDeviceLineRasterizationFeaturesEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES_EXT;
}
impl Default for PhysicalDeviceLineRasterizationFeaturesEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES_EXT, p_next: std::ptr::null_mut(), rectangular_lines: Default::default(), bresenham_lines: Default::default(), smooth_lines: Default::default(), stippled_rectangular_lines: Default::default(), stippled_bresenham_lines: Default::default(), stippled_smooth_lines: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceLineRasterizationFeaturesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceLineRasterizationFeaturesEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("rectangular_lines", &(self.rectangular_lines != 0)).field("bresenham_lines", &(self.bresenham_lines != 0)).field("smooth_lines", &(self.smooth_lines != 0)).field("stippled_rectangular_lines", &(self.stippled_rectangular_lines != 0)).field("stippled_bresenham_lines", &(self.stippled_bresenham_lines != 0)).field("stippled_smooth_lines", &(self.stippled_smooth_lines != 0)).finish()
    }
}
impl PhysicalDeviceLineRasterizationFeaturesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceLineRasterizationFeaturesEXTBuilder<'a> {
        PhysicalDeviceLineRasterizationFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceLineRasterizationFeaturesEXT.html) · Builder of [`PhysicalDeviceLineRasterizationFeaturesEXT`]"]
#[repr(transparent)]
pub struct PhysicalDeviceLineRasterizationFeaturesEXTBuilder<'a>(PhysicalDeviceLineRasterizationFeaturesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceLineRasterizationFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceLineRasterizationFeaturesEXTBuilder<'a> {
        PhysicalDeviceLineRasterizationFeaturesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn rectangular_lines(mut self, rectangular_lines: bool) -> Self {
        self.0.rectangular_lines = rectangular_lines as _;
        self
    }
    #[inline]
    pub fn bresenham_lines(mut self, bresenham_lines: bool) -> Self {
        self.0.bresenham_lines = bresenham_lines as _;
        self
    }
    #[inline]
    pub fn smooth_lines(mut self, smooth_lines: bool) -> Self {
        self.0.smooth_lines = smooth_lines as _;
        self
    }
    #[inline]
    pub fn stippled_rectangular_lines(mut self, stippled_rectangular_lines: bool) -> Self {
        self.0.stippled_rectangular_lines = stippled_rectangular_lines as _;
        self
    }
    #[inline]
    pub fn stippled_bresenham_lines(mut self, stippled_bresenham_lines: bool) -> Self {
        self.0.stippled_bresenham_lines = stippled_bresenham_lines as _;
        self
    }
    #[inline]
    pub fn stippled_smooth_lines(mut self, stippled_smooth_lines: bool) -> Self {
        self.0.stippled_smooth_lines = stippled_smooth_lines as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceLineRasterizationFeaturesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceLineRasterizationFeaturesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceLineRasterizationFeaturesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceLineRasterizationFeaturesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceLineRasterizationFeaturesEXTBuilder<'a> {
    type Target = PhysicalDeviceLineRasterizationFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceLineRasterizationFeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceLineRasterizationPropertiesEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceLineRasterizationPropertiesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceLineRasterizationPropertiesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub line_sub_pixel_precision_bits: u32,
}
impl PhysicalDeviceLineRasterizationPropertiesEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES_EXT;
}
impl Default for PhysicalDeviceLineRasterizationPropertiesEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES_EXT, p_next: std::ptr::null_mut(), line_sub_pixel_precision_bits: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceLineRasterizationPropertiesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceLineRasterizationPropertiesEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("line_sub_pixel_precision_bits", &self.line_sub_pixel_precision_bits).finish()
    }
}
impl PhysicalDeviceLineRasterizationPropertiesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceLineRasterizationPropertiesEXTBuilder<'a> {
        PhysicalDeviceLineRasterizationPropertiesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceLineRasterizationPropertiesEXT.html) · Builder of [`PhysicalDeviceLineRasterizationPropertiesEXT`]"]
#[repr(transparent)]
pub struct PhysicalDeviceLineRasterizationPropertiesEXTBuilder<'a>(PhysicalDeviceLineRasterizationPropertiesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceLineRasterizationPropertiesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceLineRasterizationPropertiesEXTBuilder<'a> {
        PhysicalDeviceLineRasterizationPropertiesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn line_sub_pixel_precision_bits(mut self, line_sub_pixel_precision_bits: u32) -> Self {
        self.0.line_sub_pixel_precision_bits = line_sub_pixel_precision_bits as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceLineRasterizationPropertiesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceLineRasterizationPropertiesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceLineRasterizationPropertiesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceLineRasterizationPropertiesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceLineRasterizationPropertiesEXTBuilder<'a> {
    type Target = PhysicalDeviceLineRasterizationPropertiesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceLineRasterizationPropertiesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineRasterizationLineStateCreateInfoEXT.html) · Structure"]
#[doc(alias = "VkPipelineRasterizationLineStateCreateInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineRasterizationLineStateCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub line_rasterization_mode: crate::extensions::ext_line_rasterization::LineRasterizationModeEXT,
    pub stippled_line_enable: crate::vk1_0::Bool32,
    pub line_stipple_factor: u32,
    pub line_stipple_pattern: u16,
}
impl PipelineRasterizationLineStateCreateInfoEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO_EXT;
}
impl Default for PipelineRasterizationLineStateCreateInfoEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO_EXT, p_next: std::ptr::null(), line_rasterization_mode: Default::default(), stippled_line_enable: Default::default(), line_stipple_factor: Default::default(), line_stipple_pattern: Default::default() }
    }
}
impl std::fmt::Debug for PipelineRasterizationLineStateCreateInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PipelineRasterizationLineStateCreateInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("line_rasterization_mode", &self.line_rasterization_mode).field("stippled_line_enable", &(self.stippled_line_enable != 0)).field("line_stipple_factor", &self.line_stipple_factor).field("line_stipple_pattern", &self.line_stipple_pattern).finish()
    }
}
impl PipelineRasterizationLineStateCreateInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PipelineRasterizationLineStateCreateInfoEXTBuilder<'a> {
        PipelineRasterizationLineStateCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineRasterizationLineStateCreateInfoEXT.html) · Builder of [`PipelineRasterizationLineStateCreateInfoEXT`]"]
#[repr(transparent)]
pub struct PipelineRasterizationLineStateCreateInfoEXTBuilder<'a>(PipelineRasterizationLineStateCreateInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PipelineRasterizationLineStateCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineRasterizationLineStateCreateInfoEXTBuilder<'a> {
        PipelineRasterizationLineStateCreateInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn line_rasterization_mode(mut self, line_rasterization_mode: crate::extensions::ext_line_rasterization::LineRasterizationModeEXT) -> Self {
        self.0.line_rasterization_mode = line_rasterization_mode as _;
        self
    }
    #[inline]
    pub fn stippled_line_enable(mut self, stippled_line_enable: bool) -> Self {
        self.0.stippled_line_enable = stippled_line_enable as _;
        self
    }
    #[inline]
    pub fn line_stipple_factor(mut self, line_stipple_factor: u32) -> Self {
        self.0.line_stipple_factor = line_stipple_factor as _;
        self
    }
    #[inline]
    pub fn line_stipple_pattern(mut self, line_stipple_pattern: u16) -> Self {
        self.0.line_stipple_pattern = line_stipple_pattern as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PipelineRasterizationLineStateCreateInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for PipelineRasterizationLineStateCreateInfoEXTBuilder<'a> {
    fn default() -> PipelineRasterizationLineStateCreateInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PipelineRasterizationLineStateCreateInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PipelineRasterizationLineStateCreateInfoEXTBuilder<'a> {
    type Target = PipelineRasterizationLineStateCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineRasterizationLineStateCreateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::ext_line_rasterization`]"]
impl crate::DeviceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetLineStippleEXT.html) · Function"]
    #[doc(alias = "vkCmdSetLineStippleEXT")]
    pub unsafe fn cmd_set_line_stipple_ext(&self, command_buffer: crate::vk1_0::CommandBuffer, line_stipple_factor: u32, line_stipple_pattern: u16) -> () {
        let _function = self.cmd_set_line_stipple_ext.expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(command_buffer as _, line_stipple_factor as _, line_stipple_pattern as _);
        ()
    }
}
