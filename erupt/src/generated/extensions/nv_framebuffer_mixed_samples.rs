#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_FRAMEBUFFER_MIXED_SAMPLES_SPEC_VERSION")]
pub const NV_FRAMEBUFFER_MIXED_SAMPLES_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_FRAMEBUFFER_MIXED_SAMPLES_EXTENSION_NAME")]
pub const NV_FRAMEBUFFER_MIXED_SAMPLES_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_NV_framebuffer_mixed_samples");
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCoverageModulationStateCreateFlagsNV.html) · Bitmask of [`PipelineCoverageModulationStateCreateFlagBitsNV`]"] # [doc (alias = "VkPipelineCoverageModulationStateCreateFlagsNV")] # [derive (Default)] # [repr (transparent)] pub struct PipelineCoverageModulationStateCreateFlagsNV : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`PipelineCoverageModulationStateCreateFlagsNV`]"]
#[doc(alias = "VkPipelineCoverageModulationStateCreateFlagBitsNV")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct PipelineCoverageModulationStateCreateFlagBitsNV(pub u32);
impl PipelineCoverageModulationStateCreateFlagBitsNV {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> PipelineCoverageModulationStateCreateFlagsNV {
        PipelineCoverageModulationStateCreateFlagsNV::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for PipelineCoverageModulationStateCreateFlagBitsNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::nv_framebuffer_mixed_samples`]"]
impl crate::vk1_0::StructureType {
    pub const PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV: Self = Self(1000152000);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCoverageModulationModeNV.html) · Enum"]
#[doc(alias = "VkCoverageModulationModeNV")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct CoverageModulationModeNV(pub i32);
impl std::fmt::Debug for CoverageModulationModeNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::NONE_NV => "NONE_NV",
            &Self::RGB_NV => "RGB_NV",
            &Self::ALPHA_NV => "ALPHA_NV",
            &Self::RGBA_NV => "RGBA_NV",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::nv_framebuffer_mixed_samples`]"]
impl crate::extensions::nv_framebuffer_mixed_samples::CoverageModulationModeNV {
    pub const NONE_NV: Self = Self(0);
    pub const RGB_NV: Self = Self(1);
    pub const ALPHA_NV: Self = Self(2);
    pub const RGBA_NV: Self = Self(3);
}
impl<'a> crate::ExtendableFromConst<'a, PipelineCoverageModulationStateCreateInfoNV> for crate::vk1_0::PipelineMultisampleStateCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, PipelineCoverageModulationStateCreateInfoNVBuilder<'_>> for crate::vk1_0::PipelineMultisampleStateCreateInfoBuilder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCoverageModulationStateCreateInfoNV.html) · Structure"]
#[doc(alias = "VkPipelineCoverageModulationStateCreateInfoNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineCoverageModulationStateCreateInfoNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::nv_framebuffer_mixed_samples::PipelineCoverageModulationStateCreateFlagsNV,
    pub coverage_modulation_mode: crate::extensions::nv_framebuffer_mixed_samples::CoverageModulationModeNV,
    pub coverage_modulation_table_enable: crate::vk1_0::Bool32,
    pub coverage_modulation_table_count: u32,
    pub p_coverage_modulation_table: *const std::os::raw::c_float,
}
impl PipelineCoverageModulationStateCreateInfoNV {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV;
}
impl Default for PipelineCoverageModulationStateCreateInfoNV {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), flags: Default::default(), coverage_modulation_mode: Default::default(), coverage_modulation_table_enable: Default::default(), coverage_modulation_table_count: Default::default(), p_coverage_modulation_table: std::ptr::null() }
    }
}
impl std::fmt::Debug for PipelineCoverageModulationStateCreateInfoNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PipelineCoverageModulationStateCreateInfoNV").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("coverage_modulation_mode", &self.coverage_modulation_mode).field("coverage_modulation_table_enable", &(self.coverage_modulation_table_enable != 0)).field("coverage_modulation_table_count", &self.coverage_modulation_table_count).field("p_coverage_modulation_table", &self.p_coverage_modulation_table).finish()
    }
}
impl PipelineCoverageModulationStateCreateInfoNV {
    #[inline]
    pub fn into_builder<'a>(self) -> PipelineCoverageModulationStateCreateInfoNVBuilder<'a> {
        PipelineCoverageModulationStateCreateInfoNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCoverageModulationStateCreateInfoNV.html) · Builder of [`PipelineCoverageModulationStateCreateInfoNV`]"]
#[repr(transparent)]
pub struct PipelineCoverageModulationStateCreateInfoNVBuilder<'a>(PipelineCoverageModulationStateCreateInfoNV, std::marker::PhantomData<&'a ()>);
impl<'a> PipelineCoverageModulationStateCreateInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineCoverageModulationStateCreateInfoNVBuilder<'a> {
        PipelineCoverageModulationStateCreateInfoNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::nv_framebuffer_mixed_samples::PipelineCoverageModulationStateCreateFlagsNV) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn coverage_modulation_mode(mut self, coverage_modulation_mode: crate::extensions::nv_framebuffer_mixed_samples::CoverageModulationModeNV) -> Self {
        self.0.coverage_modulation_mode = coverage_modulation_mode as _;
        self
    }
    #[inline]
    pub fn coverage_modulation_table_enable(mut self, coverage_modulation_table_enable: bool) -> Self {
        self.0.coverage_modulation_table_enable = coverage_modulation_table_enable as _;
        self
    }
    #[inline]
    pub fn coverage_modulation_table(mut self, coverage_modulation_table: &'a [std::os::raw::c_float]) -> Self {
        self.0.p_coverage_modulation_table = coverage_modulation_table.as_ptr() as _;
        self.0.coverage_modulation_table_count = coverage_modulation_table.len() as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PipelineCoverageModulationStateCreateInfoNV {
        self.0
    }
}
impl<'a> std::default::Default for PipelineCoverageModulationStateCreateInfoNVBuilder<'a> {
    fn default() -> PipelineCoverageModulationStateCreateInfoNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PipelineCoverageModulationStateCreateInfoNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PipelineCoverageModulationStateCreateInfoNVBuilder<'a> {
    type Target = PipelineCoverageModulationStateCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineCoverageModulationStateCreateInfoNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
