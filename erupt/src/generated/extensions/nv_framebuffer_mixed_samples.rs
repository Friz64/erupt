# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_NV_framebuffer_mixed_samples.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NV_FRAMEBUFFER_MIXED_SAMPLES_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NV_FRAMEBUFFER_MIXED_SAMPLES_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_NV_framebuffer_mixed_samples");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCoverageModulationStateCreateInfoNV.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineCoverageModulationStateCreateInfoNV { pub s_type : crate :: vk1_0 :: StructureType , pub p_next : * const std :: ffi :: c_void , pub flags : crate :: extensions :: nv_framebuffer_mixed_samples :: PipelineCoverageModulationStateCreateFlagsNV , pub coverage_modulation_mode : crate :: extensions :: nv_framebuffer_mixed_samples :: CoverageModulationModeNV , pub coverage_modulation_table_enable : crate :: vk1_0 :: Bool32 , pub coverage_modulation_table_count : u32 , pub p_coverage_modulation_table : * const f32 , }
impl PipelineCoverageModulationStateCreateInfoNV {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPipelineCoverageModulationStateCreateInfoNV,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PipelineCoverageModulationStateCreateInfoNVBuilder<'a> {
        PipelineCoverageModulationStateCreateInfoNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PipelineCoverageModulationStateCreateInfoNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PipelineCoverageModulationStateCreateInfoNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("coverage_modulation_mode", &self.coverage_modulation_mode)
            .field(
                "coverage_modulation_table_enable",
                &(self.coverage_modulation_table_enable != 0),
            )
            .field(
                "coverage_modulation_table_count",
                &self.coverage_modulation_table_count,
            )
            .field(
                "p_coverage_modulation_table",
                &self.p_coverage_modulation_table,
            )
            .finish()
    }
}
impl Default for PipelineCoverageModulationStateCreateInfoNV {
    fn default() -> PipelineCoverageModulationStateCreateInfoNV {
        PipelineCoverageModulationStateCreateInfoNV {
            s_type: crate::vk1_0::StructureType::PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV,
            p_next: std::ptr::null(),
            flags: Default::default(),
            coverage_modulation_mode: Default::default(),
            coverage_modulation_table_enable: Default::default(),
            coverage_modulation_table_count: Default::default(),
            p_coverage_modulation_table: std::ptr::null(),
        }
    }
}
#[doc = "Used by [`PipelineCoverageModulationStateCreateInfoNV::extend`](struct.PipelineCoverageModulationStateCreateInfoNV.html#method.extend)"]
pub trait ExtendableByPipelineCoverageModulationStateCreateInfoNV {}
impl ExtendableByPipelineCoverageModulationStateCreateInfoNV
    for crate::vk1_0::PipelineMultisampleStateCreateInfo
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PipelineCoverageModulationStateCreateInfoNV`](struct.PipelineCoverageModulationStateCreateInfoNV.html)"]
#[repr(transparent)]
pub struct PipelineCoverageModulationStateCreateInfoNVBuilder<'a>(
    PipelineCoverageModulationStateCreateInfoNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PipelineCoverageModulationStateCreateInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineCoverageModulationStateCreateInfoNVBuilder<'a> {
        PipelineCoverageModulationStateCreateInfoNVBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn flags(
        mut self,
        flags : crate :: extensions :: nv_framebuffer_mixed_samples :: PipelineCoverageModulationStateCreateFlagsNV,
    ) -> Self {
        self.0.flags = flags;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn coverage_modulation_mode(
        mut self,
        coverage_modulation_mode : crate :: extensions :: nv_framebuffer_mixed_samples :: CoverageModulationModeNV,
    ) -> Self {
        self.0.coverage_modulation_mode = coverage_modulation_mode;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn coverage_modulation_table_enable(
        mut self,
        coverage_modulation_table_enable: bool,
    ) -> Self {
        self.0.coverage_modulation_table_enable = coverage_modulation_table_enable as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn coverage_modulation_table(mut self, coverage_modulation_table: &'a [f32]) -> Self {
        self.0.coverage_modulation_table_count = coverage_modulation_table.len() as _;
        self.0.p_coverage_modulation_table = coverage_modulation_table.as_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PipelineCoverageModulationStateCreateInfoNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for PipelineCoverageModulationStateCreateInfoNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "<s>Vulkan Manual Page</s> · Flag Bits of [`PipelineCoverageModulationStateCreateFlagsNV`](struct.PipelineCoverageModulationStateCreateFlagsNV.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
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
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            _ => "Unknown enum variant",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCoverageModulationStateCreateFlagsNV.html) · Flags of [`PipelineCoverageModulationStateCreateFlagBitsNV`](struct.PipelineCoverageModulationStateCreateFlagBitsNV.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct PipelineCoverageModulationStateCreateFlagsNV : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCoverageModulationModeNV.html) · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct CoverageModulationModeNV(pub i32);
#[doc = "[Part of `extensions::nv_framebuffer_mixed_samples`](../../extensions/nv_framebuffer_mixed_samples/index.html)"]
impl CoverageModulationModeNV {
    pub const NONE_NV: Self = Self(0);
    pub const RGB_NV: Self = Self(1);
    pub const ALPHA_NV: Self = Self(2);
    pub const RGBA_NV: Self = Self(3);
}
impl std::fmt::Debug for CoverageModulationModeNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::NONE_NV => "NONE_NV",
            &Self::RGB_NV => "RGB_NV",
            &Self::ALPHA_NV => "ALPHA_NV",
            &Self::RGBA_NV => "RGBA_NV",
            _ => "Unknown enum variant",
        })
    }
}
