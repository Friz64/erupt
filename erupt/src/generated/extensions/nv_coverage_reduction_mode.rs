#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_COVERAGE_REDUCTION_MODE_SPEC_VERSION")]
pub const NV_COVERAGE_REDUCTION_MODE_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_COVERAGE_REDUCTION_MODE_EXTENSION_NAME")]
pub const NV_COVERAGE_REDUCTION_MODE_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_NV_coverage_reduction_mode");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_PHYSICAL_DEVICE_SUPPORTED_FRAMEBUFFER_MIXED_SAMPLES_COMBINATIONS_NV: *const std::os::raw::c_char = crate::cstr!("vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV");
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCoverageReductionStateCreateFlagsNV.html) · Bitmask of [`PipelineCoverageReductionStateCreateFlagBitsNV`]"] # [doc (alias = "VkPipelineCoverageReductionStateCreateFlagsNV")] # [derive (Default)] # [repr (transparent)] pub struct PipelineCoverageReductionStateCreateFlagsNV : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`PipelineCoverageReductionStateCreateFlagsNV`]"]
#[doc(alias = "VkPipelineCoverageReductionStateCreateFlagBitsNV")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
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
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::nv_coverage_reduction_mode`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_COVERAGE_REDUCTION_MODE_FEATURES_NV: Self = Self(1000250000);
    pub const PIPELINE_COVERAGE_REDUCTION_STATE_CREATE_INFO_NV: Self = Self(1000250001);
    pub const FRAMEBUFFER_MIXED_SAMPLES_COMBINATION_NV: Self = Self(1000250002);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCoverageReductionModeNV.html) · Enum"]
#[doc(alias = "VkCoverageReductionModeNV")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct CoverageReductionModeNV(pub i32);
impl std::fmt::Debug for CoverageReductionModeNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::MERGE_NV => "MERGE_NV",
            &Self::TRUNCATE_NV => "TRUNCATE_NV",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::nv_coverage_reduction_mode`]"]
impl crate::extensions::nv_coverage_reduction_mode::CoverageReductionModeNV {
    pub const MERGE_NV: Self = Self(0);
    pub const TRUNCATE_NV: Self = Self(1);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV = unsafe extern "system" fn(physical_device: crate::vk1_0::PhysicalDevice, p_combination_count: *mut u32, p_combinations: *mut crate::extensions::nv_coverage_reduction_mode::FramebufferMixedSamplesCombinationNV) -> crate::vk1_0::Result;
impl<'a> crate::ExtendableFromConst<'a, PhysicalDeviceCoverageReductionModeFeaturesNV> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, PhysicalDeviceCoverageReductionModeFeaturesNVBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, PipelineCoverageReductionStateCreateInfoNV> for crate::vk1_0::PipelineMultisampleStateCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, PipelineCoverageReductionStateCreateInfoNVBuilder<'_>> for crate::vk1_0::PipelineMultisampleStateCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceCoverageReductionModeFeaturesNV> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceCoverageReductionModeFeaturesNVBuilder<'_>> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceCoverageReductionModeFeaturesNV.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceCoverageReductionModeFeaturesNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceCoverageReductionModeFeaturesNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub coverage_reduction_mode: crate::vk1_0::Bool32,
}
impl Default for PhysicalDeviceCoverageReductionModeFeaturesNV {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_COVERAGE_REDUCTION_MODE_FEATURES_NV, p_next: std::ptr::null_mut(), coverage_reduction_mode: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceCoverageReductionModeFeaturesNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceCoverageReductionModeFeaturesNV").field("s_type", &self.s_type).field("p_next", &self.p_next).field("coverage_reduction_mode", &(self.coverage_reduction_mode != 0)).finish()
    }
}
impl PhysicalDeviceCoverageReductionModeFeaturesNV {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceCoverageReductionModeFeaturesNVBuilder<'a> {
        PhysicalDeviceCoverageReductionModeFeaturesNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceCoverageReductionModeFeaturesNV.html) · Builder of [`PhysicalDeviceCoverageReductionModeFeaturesNV`]"]
#[repr(transparent)]
pub struct PhysicalDeviceCoverageReductionModeFeaturesNVBuilder<'a>(PhysicalDeviceCoverageReductionModeFeaturesNV, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceCoverageReductionModeFeaturesNVBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceCoverageReductionModeFeaturesNVBuilder<'a> {
        PhysicalDeviceCoverageReductionModeFeaturesNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn coverage_reduction_mode(mut self, coverage_reduction_mode: bool) -> Self {
        self.0.coverage_reduction_mode = coverage_reduction_mode as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceCoverageReductionModeFeaturesNV {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceCoverageReductionModeFeaturesNVBuilder<'a> {
    fn default() -> PhysicalDeviceCoverageReductionModeFeaturesNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceCoverageReductionModeFeaturesNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
#[doc(alias = "VkPipelineCoverageReductionStateCreateInfoNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineCoverageReductionStateCreateInfoNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::nv_coverage_reduction_mode::PipelineCoverageReductionStateCreateFlagsNV,
    pub coverage_reduction_mode: crate::extensions::nv_coverage_reduction_mode::CoverageReductionModeNV,
}
impl Default for PipelineCoverageReductionStateCreateInfoNV {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PIPELINE_COVERAGE_REDUCTION_STATE_CREATE_INFO_NV, p_next: std::ptr::null(), flags: Default::default(), coverage_reduction_mode: Default::default() }
    }
}
impl std::fmt::Debug for PipelineCoverageReductionStateCreateInfoNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PipelineCoverageReductionStateCreateInfoNV").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("coverage_reduction_mode", &self.coverage_reduction_mode).finish()
    }
}
impl PipelineCoverageReductionStateCreateInfoNV {
    #[inline]
    pub fn into_builder<'a>(self) -> PipelineCoverageReductionStateCreateInfoNVBuilder<'a> {
        PipelineCoverageReductionStateCreateInfoNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCoverageReductionStateCreateInfoNV.html) · Builder of [`PipelineCoverageReductionStateCreateInfoNV`]"]
#[repr(transparent)]
pub struct PipelineCoverageReductionStateCreateInfoNVBuilder<'a>(PipelineCoverageReductionStateCreateInfoNV, std::marker::PhantomData<&'a ()>);
impl<'a> PipelineCoverageReductionStateCreateInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineCoverageReductionStateCreateInfoNVBuilder<'a> {
        PipelineCoverageReductionStateCreateInfoNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::nv_coverage_reduction_mode::PipelineCoverageReductionStateCreateFlagsNV) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn coverage_reduction_mode(mut self, coverage_reduction_mode: crate::extensions::nv_coverage_reduction_mode::CoverageReductionModeNV) -> Self {
        self.0.coverage_reduction_mode = coverage_reduction_mode as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PipelineCoverageReductionStateCreateInfoNV {
        self.0
    }
}
impl<'a> std::default::Default for PipelineCoverageReductionStateCreateInfoNVBuilder<'a> {
    fn default() -> PipelineCoverageReductionStateCreateInfoNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PipelineCoverageReductionStateCreateInfoNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFramebufferMixedSamplesCombinationNV.html) · Structure"]
#[doc(alias = "VkFramebufferMixedSamplesCombinationNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FramebufferMixedSamplesCombinationNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub coverage_reduction_mode: crate::extensions::nv_coverage_reduction_mode::CoverageReductionModeNV,
    pub rasterization_samples: crate::vk1_0::SampleCountFlagBits,
    pub depth_stencil_samples: crate::vk1_0::SampleCountFlags,
    pub color_samples: crate::vk1_0::SampleCountFlags,
}
impl Default for FramebufferMixedSamplesCombinationNV {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::FRAMEBUFFER_MIXED_SAMPLES_COMBINATION_NV, p_next: std::ptr::null_mut(), coverage_reduction_mode: Default::default(), rasterization_samples: Default::default(), depth_stencil_samples: Default::default(), color_samples: Default::default() }
    }
}
impl std::fmt::Debug for FramebufferMixedSamplesCombinationNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FramebufferMixedSamplesCombinationNV").field("s_type", &self.s_type).field("p_next", &self.p_next).field("coverage_reduction_mode", &self.coverage_reduction_mode).field("rasterization_samples", &self.rasterization_samples).field("depth_stencil_samples", &self.depth_stencil_samples).field("color_samples", &self.color_samples).finish()
    }
}
impl FramebufferMixedSamplesCombinationNV {
    #[inline]
    pub fn into_builder<'a>(self) -> FramebufferMixedSamplesCombinationNVBuilder<'a> {
        FramebufferMixedSamplesCombinationNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFramebufferMixedSamplesCombinationNV.html) · Builder of [`FramebufferMixedSamplesCombinationNV`]"]
#[repr(transparent)]
pub struct FramebufferMixedSamplesCombinationNVBuilder<'a>(FramebufferMixedSamplesCombinationNV, std::marker::PhantomData<&'a ()>);
impl<'a> FramebufferMixedSamplesCombinationNVBuilder<'a> {
    #[inline]
    pub fn new() -> FramebufferMixedSamplesCombinationNVBuilder<'a> {
        FramebufferMixedSamplesCombinationNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn coverage_reduction_mode(mut self, coverage_reduction_mode: crate::extensions::nv_coverage_reduction_mode::CoverageReductionModeNV) -> Self {
        self.0.coverage_reduction_mode = coverage_reduction_mode as _;
        self
    }
    #[inline]
    pub fn rasterization_samples(mut self, rasterization_samples: crate::vk1_0::SampleCountFlagBits) -> Self {
        self.0.rasterization_samples = rasterization_samples as _;
        self
    }
    #[inline]
    pub fn depth_stencil_samples(mut self, depth_stencil_samples: crate::vk1_0::SampleCountFlags) -> Self {
        self.0.depth_stencil_samples = depth_stencil_samples as _;
        self
    }
    #[inline]
    pub fn color_samples(mut self, color_samples: crate::vk1_0::SampleCountFlags) -> Self {
        self.0.color_samples = color_samples as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> FramebufferMixedSamplesCombinationNV {
        self.0
    }
}
impl<'a> std::default::Default for FramebufferMixedSamplesCombinationNVBuilder<'a> {
    fn default() -> FramebufferMixedSamplesCombinationNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for FramebufferMixedSamplesCombinationNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
#[doc = "Provided by [`crate::extensions::nv_coverage_reduction_mode`]"]
impl crate::InstanceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV.html) · Function"]
    #[doc(alias = "vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV")]
    pub unsafe fn get_physical_device_supported_framebuffer_mixed_samples_combinations_nv(&self, physical_device: crate::vk1_0::PhysicalDevice, combination_count: Option<u32>) -> crate::utils::VulkanResult<Vec<crate::extensions::nv_coverage_reduction_mode::FramebufferMixedSamplesCombinationNV>> {
        let _function = self.get_physical_device_supported_framebuffer_mixed_samples_combinations_nv.expect(crate::NOT_LOADED_MESSAGE);
        let mut combination_count = match combination_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                _function(physical_device as _, &mut v, std::ptr::null_mut());
                v
            }
        };
        let mut combinations = vec![Default::default(); combination_count as _];
        let _return = _function(physical_device as _, &mut combination_count, combinations.as_mut_ptr());
        crate::utils::VulkanResult::new(_return, combinations)
    }
}
