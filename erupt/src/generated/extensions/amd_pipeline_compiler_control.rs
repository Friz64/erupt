# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_AMD_pipeline_compiler_control.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const AMD_PIPELINE_COMPILER_CONTROL_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const AMD_PIPELINE_COMPILER_CONTROL_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_AMD_pipeline_compiler_control");
#[doc = "<s>Vulkan Manual Page</s> · Flag Bits of [`PipelineCompilerControlFlagsAMD`](struct.PipelineCompilerControlFlagsAMD.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PipelineCompilerControlFlagBitsAMD(pub u32);
impl PipelineCompilerControlFlagBitsAMD {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> PipelineCompilerControlFlagsAMD {
        PipelineCompilerControlFlagsAMD::from_bits_truncate(self.0)
    }
}
#[doc = "[Part of `extensions::amd_pipeline_compiler_control`](../../extensions/amd_pipeline_compiler_control/index.html)"]
impl PipelineCompilerControlFlagBitsAMD {}
impl std::fmt::Debug for PipelineCompilerControlFlagBitsAMD {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            _ => "Unknown enum variant",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCompilerControlFlagsAMD.html) · Flags of [`PipelineCompilerControlFlagBitsAMD`](struct.PipelineCompilerControlFlagBitsAMD.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct PipelineCompilerControlFlagsAMD : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCompilerControlCreateInfoAMD.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineCompilerControlCreateInfoAMD {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub compiler_control_flags:
        crate::extensions::amd_pipeline_compiler_control::PipelineCompilerControlFlagsAMD,
}
impl PipelineCompilerControlCreateInfoAMD {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPipelineCompilerControlCreateInfoAMD,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PipelineCompilerControlCreateInfoAMDBuilder<'a> {
        PipelineCompilerControlCreateInfoAMDBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PipelineCompilerControlCreateInfoAMD {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PipelineCompilerControlCreateInfoAMD")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("compiler_control_flags", &self.compiler_control_flags)
            .finish()
    }
}
impl Default for PipelineCompilerControlCreateInfoAMD {
    fn default() -> PipelineCompilerControlCreateInfoAMD {
        PipelineCompilerControlCreateInfoAMD {
            s_type: crate::vk1_0::StructureType::PIPELINE_COMPILER_CONTROL_CREATE_INFO_AMD,
            p_next: std::ptr::null(),
            compiler_control_flags: Default::default(),
        }
    }
}
#[doc = "Used by [`PipelineCompilerControlCreateInfoAMD::extend`](struct.PipelineCompilerControlCreateInfoAMD.html#method.extend)"]
pub trait ExtendableByPipelineCompilerControlCreateInfoAMD {}
impl ExtendableByPipelineCompilerControlCreateInfoAMD for crate::vk1_0::GraphicsPipelineCreateInfo {}
impl ExtendableByPipelineCompilerControlCreateInfoAMD for crate::vk1_0::ComputePipelineCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PipelineCompilerControlCreateInfoAMD`](struct.PipelineCompilerControlCreateInfoAMD.html)"]
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
    #[allow(unused_mut)]
    #[inline]
    pub fn compiler_control_flags(
        mut self,
        compiler_control_flags : crate :: extensions :: amd_pipeline_compiler_control :: PipelineCompilerControlFlagsAMD,
    ) -> Self {
        self.0.compiler_control_flags = compiler_control_flags;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PipelineCompilerControlCreateInfoAMD {
        self.0
    }
}
impl<'a> std::fmt::Debug for PipelineCompilerControlCreateInfoAMDBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
