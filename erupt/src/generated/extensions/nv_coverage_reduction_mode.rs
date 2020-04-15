# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_NV_coverage_reduction_mode.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NV_COVERAGE_REDUCTION_MODE_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NV_COVERAGE_REDUCTION_MODE_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_NV_coverage_reduction_mode");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV = unsafe extern "system" fn ( physical_device : crate :: vk1_0 :: PhysicalDevice , p_combination_count : * mut u32 , p_combinations : * mut crate :: extensions :: nv_coverage_reduction_mode :: FramebufferMixedSamplesCombinationNV , ) -> crate :: vk1_0 :: Result ;
#[doc = "Provides Instance Commands for [`NvCoverageReductionModeInstanceLoaderExt`](trait.NvCoverageReductionModeInstanceLoaderExt.html)"]
pub struct NvCoverageReductionModeInstanceCommands {
    pub get_physical_device_supported_framebuffer_mixed_samples_combinations_nv:
        PFN_vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV,
}
impl NvCoverageReductionModeInstanceCommands {
    #[inline]
    pub fn load(loader: &crate::InstanceLoader) -> Option<NvCoverageReductionModeInstanceCommands> {
        unsafe {
            Some(NvCoverageReductionModeInstanceCommands {
                get_physical_device_supported_framebuffer_mixed_samples_combinations_nv:
                    std::mem::transmute(loader.symbol(
                        "vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV",
                    )?),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`NvCoverageReductionModeInstanceCommands`](struct.NvCoverageReductionModeInstanceCommands.html)"]
pub trait NvCoverageReductionModeInstanceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV.html) · Instance Command"]
    unsafe fn get_physical_device_supported_framebuffer_mixed_samples_combinations_nv(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        combination_count: Option<u32>,
    ) -> crate::utils::VulkanResult<
        Vec<crate::extensions::nv_coverage_reduction_mode::FramebufferMixedSamplesCombinationNV>,
    >;
}
impl NvCoverageReductionModeInstanceLoaderExt for crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV.html) · Instance Command"]
    unsafe fn get_physical_device_supported_framebuffer_mixed_samples_combinations_nv(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        combination_count: Option<u32>,
    ) -> crate::utils::VulkanResult<
        Vec<crate::extensions::nv_coverage_reduction_mode::FramebufferMixedSamplesCombinationNV>,
    > {
        let function = self
            .nv_coverage_reduction_mode
            .as_ref()
            .expect("`nv_coverage_reduction_mode` not loaded")
            .get_physical_device_supported_framebuffer_mixed_samples_combinations_nv;
        let mut combination_count = combination_count.unwrap_or_else(|| {
            let mut val = Default::default();
            function(physical_device, &mut val, std::ptr::null_mut());
            val
        });
        let mut combinations = vec![Default::default(); combination_count as _];
        let _val = function(
            physical_device,
            &mut combination_count,
            combinations.as_mut_ptr(),
        );
        crate::utils::VulkanResult::new(_val, combinations)
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFramebufferMixedSamplesCombinationNV.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FramebufferMixedSamplesCombinationNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub coverage_reduction_mode:
        crate::extensions::nv_coverage_reduction_mode::CoverageReductionModeNV,
    pub rasterization_samples: crate::vk1_0::SampleCountFlagBits,
    pub depth_stencil_samples: crate::vk1_0::SampleCountFlags,
    pub color_samples: crate::vk1_0::SampleCountFlags,
}
impl FramebufferMixedSamplesCombinationNV {
    #[inline]
    pub fn builder<'a>(self) -> FramebufferMixedSamplesCombinationNVBuilder<'a> {
        FramebufferMixedSamplesCombinationNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for FramebufferMixedSamplesCombinationNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("FramebufferMixedSamplesCombinationNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("coverage_reduction_mode", &self.coverage_reduction_mode)
            .field("rasterization_samples", &self.rasterization_samples)
            .field("depth_stencil_samples", &self.depth_stencil_samples)
            .field("color_samples", &self.color_samples)
            .finish()
    }
}
impl Default for FramebufferMixedSamplesCombinationNV {
    fn default() -> FramebufferMixedSamplesCombinationNV {
        FramebufferMixedSamplesCombinationNV {
            s_type: crate::vk1_0::StructureType::FRAMEBUFFER_MIXED_SAMPLES_COMBINATION_NV,
            p_next: std::ptr::null_mut(),
            coverage_reduction_mode: Default::default(),
            rasterization_samples: Default::default(),
            depth_stencil_samples: Default::default(),
            color_samples: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`FramebufferMixedSamplesCombinationNV`](struct.FramebufferMixedSamplesCombinationNV.html)"]
#[repr(transparent)]
pub struct FramebufferMixedSamplesCombinationNVBuilder<'a>(
    FramebufferMixedSamplesCombinationNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> FramebufferMixedSamplesCombinationNVBuilder<'a> {
    #[inline]
    pub fn new() -> FramebufferMixedSamplesCombinationNVBuilder<'a> {
        FramebufferMixedSamplesCombinationNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn coverage_reduction_mode(
        mut self,
        coverage_reduction_mode : crate :: extensions :: nv_coverage_reduction_mode :: CoverageReductionModeNV,
    ) -> Self {
        self.0.coverage_reduction_mode = coverage_reduction_mode;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn rasterization_samples(
        mut self,
        rasterization_samples: crate::vk1_0::SampleCountFlagBits,
    ) -> Self {
        self.0.rasterization_samples = rasterization_samples;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn depth_stencil_samples(
        mut self,
        depth_stencil_samples: crate::vk1_0::SampleCountFlags,
    ) -> Self {
        self.0.depth_stencil_samples = depth_stencil_samples;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn color_samples(mut self, color_samples: crate::vk1_0::SampleCountFlags) -> Self {
        self.0.color_samples = color_samples;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> FramebufferMixedSamplesCombinationNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for FramebufferMixedSamplesCombinationNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for FramebufferMixedSamplesCombinationNVBuilder<'a> {
    type Target = FramebufferMixedSamplesCombinationNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for FramebufferMixedSamplesCombinationNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCoverageReductionModeNV.html) · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct CoverageReductionModeNV(pub i32);
#[doc = "[Part of `extensions::nv_coverage_reduction_mode`](../../extensions/nv_coverage_reduction_mode/index.html)"]
impl CoverageReductionModeNV {
    pub const MERGE_NV: Self = Self(0);
    pub const TRUNCATE_NV: Self = Self(1);
}
impl std::fmt::Debug for CoverageReductionModeNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::MERGE_NV => "MERGE_NV",
            &Self::TRUNCATE_NV => "TRUNCATE_NV",
            _ => "Unknown enum variant",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceCoverageReductionModeFeaturesNV.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceCoverageReductionModeFeaturesNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub coverage_reduction_mode: crate::vk1_0::Bool32,
}
impl PhysicalDeviceCoverageReductionModeFeaturesNV {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceCoverageReductionModeFeaturesNV,
    {
        unsafe {
            crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
        }
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceCoverageReductionModeFeaturesNVBuilder<'a> {
        PhysicalDeviceCoverageReductionModeFeaturesNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceCoverageReductionModeFeaturesNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceCoverageReductionModeFeaturesNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "coverage_reduction_mode",
                &(self.coverage_reduction_mode != 0),
            )
            .finish()
    }
}
impl Default for PhysicalDeviceCoverageReductionModeFeaturesNV {
    fn default() -> PhysicalDeviceCoverageReductionModeFeaturesNV {
        PhysicalDeviceCoverageReductionModeFeaturesNV {
            s_type:
                crate::vk1_0::StructureType::PHYSICAL_DEVICE_COVERAGE_REDUCTION_MODE_FEATURES_NV,
            p_next: std::ptr::null_mut(),
            coverage_reduction_mode: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceCoverageReductionModeFeaturesNV::extend`](struct.PhysicalDeviceCoverageReductionModeFeaturesNV.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceCoverageReductionModeFeaturesNV {}
impl ExtendableByPhysicalDeviceCoverageReductionModeFeaturesNV
    for crate::vk1_1::PhysicalDeviceFeatures2
{
}
impl ExtendableByPhysicalDeviceCoverageReductionModeFeaturesNV for crate::vk1_0::DeviceCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceCoverageReductionModeFeaturesNV`](struct.PhysicalDeviceCoverageReductionModeFeaturesNV.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceCoverageReductionModeFeaturesNVBuilder<'a>(
    PhysicalDeviceCoverageReductionModeFeaturesNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceCoverageReductionModeFeaturesNVBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceCoverageReductionModeFeaturesNVBuilder<'a> {
        PhysicalDeviceCoverageReductionModeFeaturesNVBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn coverage_reduction_mode(mut self, coverage_reduction_mode: bool) -> Self {
        self.0.coverage_reduction_mode = coverage_reduction_mode as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceCoverageReductionModeFeaturesNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceCoverageReductionModeFeaturesNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceCoverageReductionModeFeaturesNVBuilder<'a> {
    type Target = PhysicalDeviceCoverageReductionModeFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceCoverageReductionModeFeaturesNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCoverageReductionStateCreateInfoNV.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineCoverageReductionStateCreateInfoNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags:
        crate::extensions::nv_coverage_reduction_mode::PipelineCoverageReductionStateCreateFlagsNV,
    pub coverage_reduction_mode:
        crate::extensions::nv_coverage_reduction_mode::CoverageReductionModeNV,
}
impl PipelineCoverageReductionStateCreateInfoNV {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPipelineCoverageReductionStateCreateInfoNV,
    {
        unsafe {
            crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
        }
    }
    #[inline]
    pub fn builder<'a>(self) -> PipelineCoverageReductionStateCreateInfoNVBuilder<'a> {
        PipelineCoverageReductionStateCreateInfoNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PipelineCoverageReductionStateCreateInfoNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PipelineCoverageReductionStateCreateInfoNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("coverage_reduction_mode", &self.coverage_reduction_mode)
            .finish()
    }
}
impl Default for PipelineCoverageReductionStateCreateInfoNV {
    fn default() -> PipelineCoverageReductionStateCreateInfoNV {
        PipelineCoverageReductionStateCreateInfoNV {
            s_type: crate::vk1_0::StructureType::PIPELINE_COVERAGE_REDUCTION_STATE_CREATE_INFO_NV,
            p_next: std::ptr::null(),
            flags: Default::default(),
            coverage_reduction_mode: Default::default(),
        }
    }
}
#[doc = "Used by [`PipelineCoverageReductionStateCreateInfoNV::extend`](struct.PipelineCoverageReductionStateCreateInfoNV.html#method.extend)"]
pub trait ExtendableByPipelineCoverageReductionStateCreateInfoNV {}
impl ExtendableByPipelineCoverageReductionStateCreateInfoNV
    for crate::vk1_0::PipelineMultisampleStateCreateInfo
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PipelineCoverageReductionStateCreateInfoNV`](struct.PipelineCoverageReductionStateCreateInfoNV.html)"]
#[repr(transparent)]
pub struct PipelineCoverageReductionStateCreateInfoNVBuilder<'a>(
    PipelineCoverageReductionStateCreateInfoNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PipelineCoverageReductionStateCreateInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineCoverageReductionStateCreateInfoNVBuilder<'a> {
        PipelineCoverageReductionStateCreateInfoNVBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn flags(
        mut self,
        flags : crate :: extensions :: nv_coverage_reduction_mode :: PipelineCoverageReductionStateCreateFlagsNV,
    ) -> Self {
        self.0.flags = flags;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn coverage_reduction_mode(
        mut self,
        coverage_reduction_mode : crate :: extensions :: nv_coverage_reduction_mode :: CoverageReductionModeNV,
    ) -> Self {
        self.0.coverage_reduction_mode = coverage_reduction_mode;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PipelineCoverageReductionStateCreateInfoNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for PipelineCoverageReductionStateCreateInfoNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PipelineCoverageReductionStateCreateInfoNVBuilder<'a> {
    type Target = PipelineCoverageReductionStateCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineCoverageReductionStateCreateInfoNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Flag Bits of [`PipelineCoverageReductionStateCreateFlagsNV`](struct.PipelineCoverageReductionStateCreateFlagsNV.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PipelineCoverageReductionStateCreateFlagBitsNV(pub u32);
impl PipelineCoverageReductionStateCreateFlagBitsNV {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> PipelineCoverageReductionStateCreateFlagsNV {
        PipelineCoverageReductionStateCreateFlagsNV::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for PipelineCoverageReductionStateCreateFlagBitsNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            _ => "Unknown enum variant",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCoverageReductionStateCreateFlagsNV.html) · Flags of [`PipelineCoverageReductionStateCreateFlagBitsNV`](struct.PipelineCoverageReductionStateCreateFlagBitsNV.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct PipelineCoverageReductionStateCreateFlagsNV : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
