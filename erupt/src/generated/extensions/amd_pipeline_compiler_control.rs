#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const AMD_PIPELINE_COMPILER_CONTROL_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const AMD_PIPELINE_COMPILER_CONTROL_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_AMD_pipeline_compiler_control");
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCompilerControlFlagsAMD.html) · Bitmask of [`PipelineCompilerControlFlagBitsAMD`](./struct.PipelineCompilerControlFlagBitsAMD.html)"] # [derive (Default)] # [repr (transparent)] pub struct PipelineCompilerControlFlagsAMD : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`PipelineCompilerControlFlagsAMD`](./struct.PipelineCompilerControlFlagsAMD.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct PipelineCompilerControlFlagBitsAMD(pub u32);
impl PipelineCompilerControlFlagBitsAMD {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> PipelineCompilerControlFlagsAMD {
        PipelineCompilerControlFlagsAMD::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for PipelineCompilerControlFlagBitsAMD {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCompilerControlCreateInfoAMD.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineCompilerControlCreateInfoAMD {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub compiler_control_flags:
        crate::extensions::amd_pipeline_compiler_control::PipelineCompilerControlFlagsAMD,
}
impl Default for PipelineCompilerControlCreateInfoAMD {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PIPELINE_COMPILER_CONTROL_CREATE_INFO_AMD,
            p_next: std::ptr::null(),
            compiler_control_flags: Default::default(),
        }
    }
}
impl std::fmt::Debug for PipelineCompilerControlCreateInfoAMD {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PipelineCompilerControlCreateInfoAMD")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("compiler_control_flags", &self.compiler_control_flags)
            .finish()
    }
}
impl PipelineCompilerControlCreateInfoAMD {
    #[inline]
    pub fn into_builder<'a>(self) -> PipelineCompilerControlCreateInfoAMDBuilder<'a> {
        PipelineCompilerControlCreateInfoAMDBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCompilerControlCreateInfoAMD.html) · Builder of [`PipelineCompilerControlCreateInfoAMD`](struct.PipelineCompilerControlCreateInfoAMD.html)"]
#[repr(transparent)]
pub struct PipelineCompilerControlCreateInfoAMDBuilder<'a>(
    PipelineCompilerControlCreateInfoAMD,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PipelineCompilerControlCreateInfoAMDBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineCompilerControlCreateInfoAMDBuilder<'a> {
        PipelineCompilerControlCreateInfoAMDBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn compiler_control_flags(
        mut self,
        compiler_control_flags : crate :: extensions :: amd_pipeline_compiler_control :: PipelineCompilerControlFlagsAMD,
    ) -> Self {
        self.0.compiler_control_flags = compiler_control_flags as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PipelineCompilerControlCreateInfoAMD {
        self.0
    }
}
impl<'a> std::default::Default for PipelineCompilerControlCreateInfoAMDBuilder<'a> {
    fn default() -> PipelineCompilerControlCreateInfoAMDBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PipelineCompilerControlCreateInfoAMDBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PipelineCompilerControlCreateInfoAMDBuilder<'a> {
    type Target = PipelineCompilerControlCreateInfoAMD;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineCompilerControlCreateInfoAMDBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
