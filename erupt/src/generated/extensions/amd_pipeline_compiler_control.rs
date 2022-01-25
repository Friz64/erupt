// DO NOT EDIT: @generated by erupt's generator
///<s>Vulkan Manual Page</s> · Constant
#[doc(alias = "VK_AMD_PIPELINE_COMPILER_CONTROL_SPEC_VERSION")]
pub const AMD_PIPELINE_COMPILER_CONTROL_SPEC_VERSION: u32 = 1;
///<s>Vulkan Manual Page</s> · Constant
#[doc(alias = "VK_AMD_PIPELINE_COMPILER_CONTROL_EXTENSION_NAME")]
pub const AMD_PIPELINE_COMPILER_CONTROL_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!(
    "VK_AMD_pipeline_compiler_control"
);
///Provided by [`crate::extensions::amd_pipeline_compiler_control`]
impl crate::vk1_0::StructureType {
    pub const PIPELINE_COMPILER_CONTROL_CREATE_INFO_AMD: Self = Self(1000183000);
}
bitflags::bitflags! {
    #[doc =
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCompilerControlFlagsAMD.html) · Bitmask of [`PipelineCompilerControlFlagBitsAMD`]"]
    #[doc(alias = "VkPipelineCompilerControlFlagsAMD")] #[derive(Default)]
    #[repr(transparent)] pub struct PipelineCompilerControlFlagsAMD : u32 {
    #[cfg(empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0; }
}
///<s>Vulkan Manual Page</s> · Bits enum of [`PipelineCompilerControlFlagsAMD`]
#[doc(alias = "VkPipelineCompilerControlFlagBitsAMD")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct PipelineCompilerControlFlagBitsAMD(pub u32);
impl PipelineCompilerControlFlagBitsAMD {
    #[inline]
    ///Converts this enum variant to the corresponding bitmask
    pub const fn bitmask(&self) -> PipelineCompilerControlFlagsAMD {
        PipelineCompilerControlFlagsAMD::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for PipelineCompilerControlFlagBitsAMD {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f
            .write_str(
                match self {
                    _ => "(unknown variant)",
                },
            )
    }
}
impl<'a> crate::ExtendableFrom<'a, PipelineCompilerControlCreateInfoAMD>
for crate::vk1_0::ComputePipelineCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PipelineCompilerControlCreateInfoAMDBuilder<'_>>
for crate::vk1_0::ComputePipelineCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PipelineCompilerControlCreateInfoAMD>
for crate::vk1_0::GraphicsPipelineCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PipelineCompilerControlCreateInfoAMDBuilder<'_>>
for crate::vk1_0::GraphicsPipelineCreateInfoBuilder<'a> {}
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCompilerControlCreateInfoAMD.html) · Structure
#[doc(alias = "VkPipelineCompilerControlCreateInfoAMD")]
#[derive(Copy, Clone, )]
#[repr(C)]
pub struct PipelineCompilerControlCreateInfoAMD {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub compiler_control_flags: crate::extensions::amd_pipeline_compiler_control::PipelineCompilerControlFlagsAMD,
}
impl PipelineCompilerControlCreateInfoAMD {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PIPELINE_COMPILER_CONTROL_CREATE_INFO_AMD;
}
impl Default for PipelineCompilerControlCreateInfoAMD {
    fn default() -> Self {
        Self {
            s_type: Self::STRUCTURE_TYPE,
            p_next: std::ptr::null(),
            compiler_control_flags: Default::default(),
        }
    }
}
impl std::fmt::Debug for PipelineCompilerControlCreateInfoAMD {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f
            .debug_struct("PipelineCompilerControlCreateInfoAMD")
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
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCompilerControlCreateInfoAMD.html) · Builder of [`PipelineCompilerControlCreateInfoAMD`]
#[repr(transparent)]
pub struct PipelineCompilerControlCreateInfoAMDBuilder<'a>(
    PipelineCompilerControlCreateInfoAMD,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PipelineCompilerControlCreateInfoAMDBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineCompilerControlCreateInfoAMDBuilder<'a> {
        PipelineCompilerControlCreateInfoAMDBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[inline]
    #[must_use]
    pub fn compiler_control_flags(
        mut self,
        compiler_control_flags: crate::extensions::amd_pipeline_compiler_control::PipelineCompilerControlFlagsAMD,
    ) -> Self {
        self.0.compiler_control_flags = compiler_control_flags as _;
        self
    }
    #[inline]
    /// Discards all lifetime information.
    /// Use the `Deref` and `DerefMut` implementations if possible.
    pub fn build_dangling(self) -> PipelineCompilerControlCreateInfoAMD {
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
