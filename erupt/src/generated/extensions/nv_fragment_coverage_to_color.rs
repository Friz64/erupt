# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_NV_fragment_coverage_to_color.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NV_FRAGMENT_COVERAGE_TO_COLOR_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NV_FRAGMENT_COVERAGE_TO_COLOR_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_NV_fragment_coverage_to_color");
#[doc = "<s>Vulkan Manual Page</s> · Flag Bits of [`PipelineCoverageToColorStateCreateFlagsNV`](struct.PipelineCoverageToColorStateCreateFlagsNV.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
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
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            _ => "Unknown enum variant",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCoverageToColorStateCreateFlagsNV.html) · Flags of [`PipelineCoverageToColorStateCreateFlagBitsNV`](struct.PipelineCoverageToColorStateCreateFlagBitsNV.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct PipelineCoverageToColorStateCreateFlagsNV : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCoverageToColorStateCreateInfoNV.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineCoverageToColorStateCreateInfoNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags:
        crate::extensions::nv_fragment_coverage_to_color::PipelineCoverageToColorStateCreateFlagsNV,
    pub coverage_to_color_enable: crate::vk1_0::Bool32,
    pub coverage_to_color_location: u32,
}
impl PipelineCoverageToColorStateCreateInfoNV {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPipelineCoverageToColorStateCreateInfoNV,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PipelineCoverageToColorStateCreateInfoNVBuilder<'a> {
        PipelineCoverageToColorStateCreateInfoNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PipelineCoverageToColorStateCreateInfoNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PipelineCoverageToColorStateCreateInfoNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field(
                "coverage_to_color_enable",
                &(self.coverage_to_color_enable != 0),
            )
            .field(
                "coverage_to_color_location",
                &self.coverage_to_color_location,
            )
            .finish()
    }
}
impl Default for PipelineCoverageToColorStateCreateInfoNV {
    fn default() -> PipelineCoverageToColorStateCreateInfoNV {
        PipelineCoverageToColorStateCreateInfoNV {
            s_type: crate::vk1_0::StructureType::PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV,
            p_next: std::ptr::null(),
            flags: Default::default(),
            coverage_to_color_enable: Default::default(),
            coverage_to_color_location: Default::default(),
        }
    }
}
#[doc = "Used by [`PipelineCoverageToColorStateCreateInfoNV::extend`](struct.PipelineCoverageToColorStateCreateInfoNV.html#method.extend)"]
pub trait ExtendableByPipelineCoverageToColorStateCreateInfoNV {}
impl ExtendableByPipelineCoverageToColorStateCreateInfoNV
    for crate::vk1_0::PipelineMultisampleStateCreateInfo
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PipelineCoverageToColorStateCreateInfoNV`](struct.PipelineCoverageToColorStateCreateInfoNV.html)"]
#[repr(transparent)]
pub struct PipelineCoverageToColorStateCreateInfoNVBuilder<'a>(
    PipelineCoverageToColorStateCreateInfoNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PipelineCoverageToColorStateCreateInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineCoverageToColorStateCreateInfoNVBuilder<'a> {
        PipelineCoverageToColorStateCreateInfoNVBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn flags(
        mut self,
        flags : crate :: extensions :: nv_fragment_coverage_to_color :: PipelineCoverageToColorStateCreateFlagsNV,
    ) -> Self {
        self.0.flags = flags;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn coverage_to_color_enable(mut self, coverage_to_color_enable: bool) -> Self {
        self.0.coverage_to_color_enable = coverage_to_color_enable as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn coverage_to_color_location(mut self, coverage_to_color_location: u32) -> Self {
        self.0.coverage_to_color_location = coverage_to_color_location;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PipelineCoverageToColorStateCreateInfoNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for PipelineCoverageToColorStateCreateInfoNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
