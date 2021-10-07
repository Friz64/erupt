#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_FRAGMENT_COVERAGE_TO_COLOR_SPEC_VERSION")]
pub const NV_FRAGMENT_COVERAGE_TO_COLOR_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_FRAGMENT_COVERAGE_TO_COLOR_EXTENSION_NAME")]
pub const NV_FRAGMENT_COVERAGE_TO_COLOR_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_NV_fragment_coverage_to_color");
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCoverageToColorStateCreateFlagsNV.html) · Bitmask of [`PipelineCoverageToColorStateCreateFlagBitsNV`]"] # [doc (alias = "VkPipelineCoverageToColorStateCreateFlagsNV")] # [derive (Default)] # [repr (transparent)] pub struct PipelineCoverageToColorStateCreateFlagsNV : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`PipelineCoverageToColorStateCreateFlagsNV`]"]
#[doc(alias = "VkPipelineCoverageToColorStateCreateFlagBitsNV")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct PipelineCoverageToColorStateCreateFlagBitsNV(pub u32);
impl PipelineCoverageToColorStateCreateFlagBitsNV {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> PipelineCoverageToColorStateCreateFlagsNV {
        PipelineCoverageToColorStateCreateFlagsNV::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for PipelineCoverageToColorStateCreateFlagBitsNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::nv_fragment_coverage_to_color`]"]
impl crate::vk1_0::StructureType {
    pub const PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV: Self = Self(1000149000);
}
impl<'a> crate::ExtendableFrom<'a, PipelineCoverageToColorStateCreateInfoNV> for crate::vk1_0::PipelineMultisampleStateCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PipelineCoverageToColorStateCreateInfoNVBuilder<'_>> for crate::vk1_0::PipelineMultisampleStateCreateInfoBuilder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCoverageToColorStateCreateInfoNV.html) · Structure"]
#[doc(alias = "VkPipelineCoverageToColorStateCreateInfoNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineCoverageToColorStateCreateInfoNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::nv_fragment_coverage_to_color::PipelineCoverageToColorStateCreateFlagsNV,
    pub coverage_to_color_enable: crate::vk1_0::Bool32,
    pub coverage_to_color_location: u32,
}
impl PipelineCoverageToColorStateCreateInfoNV {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV;
}
impl Default for PipelineCoverageToColorStateCreateInfoNV {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), flags: Default::default(), coverage_to_color_enable: Default::default(), coverage_to_color_location: Default::default() }
    }
}
impl std::fmt::Debug for PipelineCoverageToColorStateCreateInfoNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PipelineCoverageToColorStateCreateInfoNV").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("coverage_to_color_enable", &(self.coverage_to_color_enable != 0)).field("coverage_to_color_location", &self.coverage_to_color_location).finish()
    }
}
impl PipelineCoverageToColorStateCreateInfoNV {
    #[inline]
    pub fn into_builder<'a>(self) -> PipelineCoverageToColorStateCreateInfoNVBuilder<'a> {
        PipelineCoverageToColorStateCreateInfoNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCoverageToColorStateCreateInfoNV.html) · Builder of [`PipelineCoverageToColorStateCreateInfoNV`]"]
#[repr(transparent)]
pub struct PipelineCoverageToColorStateCreateInfoNVBuilder<'a>(PipelineCoverageToColorStateCreateInfoNV, std::marker::PhantomData<&'a ()>);
impl<'a> PipelineCoverageToColorStateCreateInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineCoverageToColorStateCreateInfoNVBuilder<'a> {
        PipelineCoverageToColorStateCreateInfoNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::nv_fragment_coverage_to_color::PipelineCoverageToColorStateCreateFlagsNV) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn coverage_to_color_enable(mut self, coverage_to_color_enable: bool) -> Self {
        self.0.coverage_to_color_enable = coverage_to_color_enable as _;
        self
    }
    #[inline]
    pub fn coverage_to_color_location(mut self, coverage_to_color_location: u32) -> Self {
        self.0.coverage_to_color_location = coverage_to_color_location as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PipelineCoverageToColorStateCreateInfoNV {
        self.0
    }
}
impl<'a> std::default::Default for PipelineCoverageToColorStateCreateInfoNVBuilder<'a> {
    fn default() -> PipelineCoverageToColorStateCreateInfoNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PipelineCoverageToColorStateCreateInfoNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PipelineCoverageToColorStateCreateInfoNVBuilder<'a> {
    type Target = PipelineCoverageToColorStateCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineCoverageToColorStateCreateInfoNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
