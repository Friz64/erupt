#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_FRAGMENT_SHADING_RATE_SPEC_VERSION")]
pub const KHR_FRAGMENT_SHADING_RATE_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_FRAGMENT_SHADING_RATE_EXTENSION_NAME")]
pub const KHR_FRAGMENT_SHADING_RATE_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_fragment_shading_rate");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_SET_FRAGMENT_SHADING_RATE_KHR: *const std::os::raw::c_char = crate::cstr!("vkCmdSetFragmentShadingRateKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATES_KHR: *const std::os::raw::c_char = crate::cstr!("vkGetPhysicalDeviceFragmentShadingRatesKHR");
#[doc = "Provided by [`crate::extensions::khr_fragment_shading_rate`]"]
impl crate::vk1_0::DynamicState {
    pub const FRAGMENT_SHADING_RATE_KHR: Self = Self(1000226000);
}
#[doc = "Provided by [`crate::extensions::khr_fragment_shading_rate`]"]
impl crate::vk1_0::FormatFeatureFlagBits {
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self = Self(1073741824);
}
#[doc = "Provided by [`crate::extensions::khr_fragment_shading_rate`]"]
impl crate::vk1_0::ImageLayout {
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR: Self = Self(1000164003);
}
#[doc = "Provided by [`crate::extensions::khr_fragment_shading_rate`]"]
impl crate::vk1_0::ImageUsageFlagBits {
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self = Self(256);
}
#[doc = "Provided by [`crate::extensions::khr_fragment_shading_rate`]"]
impl crate::vk1_0::AccessFlagBits {
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR: Self = Self(8388608);
}
#[doc = "Provided by [`crate::extensions::khr_fragment_shading_rate`]"]
impl crate::vk1_0::StructureType {
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR: Self = Self(1000226000);
    pub const PIPELINE_FRAGMENT_SHADING_RATE_STATE_CREATE_INFO_KHR: Self = Self(1000226001);
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_PROPERTIES_KHR: Self = Self(1000226002);
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_FEATURES_KHR: Self = Self(1000226003);
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_KHR: Self = Self(1000226004);
}
#[doc = "Provided by [`crate::extensions::khr_fragment_shading_rate`]"]
impl crate::vk1_0::PipelineStageFlagBits {
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self = Self(4194304);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFragmentShadingRateCombinerOpKHR.html) · Enum"]
#[doc(alias = "VkFragmentShadingRateCombinerOpKHR")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct FragmentShadingRateCombinerOpKHR(pub i32);
impl std::fmt::Debug for FragmentShadingRateCombinerOpKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::KEEP_KHR => "KEEP_KHR",
            &Self::REPLACE_KHR => "REPLACE_KHR",
            &Self::MIN_KHR => "MIN_KHR",
            &Self::MAX_KHR => "MAX_KHR",
            &Self::MUL_KHR => "MUL_KHR",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::khr_fragment_shading_rate`]"]
impl crate::extensions::khr_fragment_shading_rate::FragmentShadingRateCombinerOpKHR {
    pub const KEEP_KHR: Self = Self(0);
    pub const REPLACE_KHR: Self = Self(1);
    pub const MIN_KHR: Self = Self(2);
    pub const MAX_KHR: Self = Self(3);
    pub const MUL_KHR: Self = Self(4);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetFragmentShadingRateKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetFragmentShadingRateKHR = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, p_fragment_size: *const crate::vk1_0::Extent2D, combiner_ops: [crate::extensions::khr_fragment_shading_rate::FragmentShadingRateCombinerOpKHR; 2]) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceFragmentShadingRatesKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceFragmentShadingRatesKHR = unsafe extern "system" fn(physical_device: crate::vk1_0::PhysicalDevice, p_fragment_shading_rate_count: *mut u32, p_fragment_shading_rates: *mut crate::extensions::khr_fragment_shading_rate::PhysicalDeviceFragmentShadingRateKHR) -> crate::vk1_0::Result;
impl<'a> crate::ExtendableFromConst<'a, PhysicalDeviceFragmentShadingRateFeaturesKHR> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, PhysicalDeviceFragmentShadingRateFeaturesKHRBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, PipelineFragmentShadingRateStateCreateInfoKHR> for crate::vk1_0::GraphicsPipelineCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, PipelineFragmentShadingRateStateCreateInfoKHRBuilder<'_>> for crate::vk1_0::GraphicsPipelineCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceFragmentShadingRateFeaturesKHR> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceFragmentShadingRateFeaturesKHRBuilder<'_>> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceFragmentShadingRatePropertiesKHR> for crate::vk1_1::PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceFragmentShadingRatePropertiesKHRBuilder<'_>> for crate::vk1_1::PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, FragmentShadingRateAttachmentInfoKHR> for crate::vk1_2::SubpassDescription2Builder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, FragmentShadingRateAttachmentInfoKHRBuilder<'_>> for crate::vk1_2::SubpassDescription2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFragmentShadingRateAttachmentInfoKHR.html) · Structure"]
#[doc(alias = "VkFragmentShadingRateAttachmentInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FragmentShadingRateAttachmentInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub p_fragment_shading_rate_attachment: *const crate::vk1_2::AttachmentReference2,
    pub shading_rate_attachment_texel_size: crate::vk1_0::Extent2D,
}
impl FragmentShadingRateAttachmentInfoKHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR;
}
impl Default for FragmentShadingRateAttachmentInfoKHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), p_fragment_shading_rate_attachment: std::ptr::null(), shading_rate_attachment_texel_size: Default::default() }
    }
}
impl std::fmt::Debug for FragmentShadingRateAttachmentInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FragmentShadingRateAttachmentInfoKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("p_fragment_shading_rate_attachment", &self.p_fragment_shading_rate_attachment).field("shading_rate_attachment_texel_size", &self.shading_rate_attachment_texel_size).finish()
    }
}
impl FragmentShadingRateAttachmentInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> FragmentShadingRateAttachmentInfoKHRBuilder<'a> {
        FragmentShadingRateAttachmentInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFragmentShadingRateAttachmentInfoKHR.html) · Builder of [`FragmentShadingRateAttachmentInfoKHR`]"]
#[repr(transparent)]
pub struct FragmentShadingRateAttachmentInfoKHRBuilder<'a>(FragmentShadingRateAttachmentInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> FragmentShadingRateAttachmentInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> FragmentShadingRateAttachmentInfoKHRBuilder<'a> {
        FragmentShadingRateAttachmentInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn fragment_shading_rate_attachment(mut self, fragment_shading_rate_attachment: &'a crate::vk1_2::AttachmentReference2) -> Self {
        self.0.p_fragment_shading_rate_attachment = fragment_shading_rate_attachment as _;
        self
    }
    #[inline]
    pub fn shading_rate_attachment_texel_size(mut self, shading_rate_attachment_texel_size: crate::vk1_0::Extent2D) -> Self {
        self.0.shading_rate_attachment_texel_size = shading_rate_attachment_texel_size as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> FragmentShadingRateAttachmentInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for FragmentShadingRateAttachmentInfoKHRBuilder<'a> {
    fn default() -> FragmentShadingRateAttachmentInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for FragmentShadingRateAttachmentInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for FragmentShadingRateAttachmentInfoKHRBuilder<'a> {
    type Target = FragmentShadingRateAttachmentInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for FragmentShadingRateAttachmentInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineFragmentShadingRateStateCreateInfoKHR.html) · Structure"]
#[doc(alias = "VkPipelineFragmentShadingRateStateCreateInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineFragmentShadingRateStateCreateInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub fragment_size: crate::vk1_0::Extent2D,
    pub combiner_ops: [crate::extensions::khr_fragment_shading_rate::FragmentShadingRateCombinerOpKHR; 2],
}
impl PipelineFragmentShadingRateStateCreateInfoKHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PIPELINE_FRAGMENT_SHADING_RATE_STATE_CREATE_INFO_KHR;
}
impl Default for PipelineFragmentShadingRateStateCreateInfoKHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), fragment_size: Default::default(), combiner_ops: unsafe { std::mem::zeroed() } }
    }
}
impl std::fmt::Debug for PipelineFragmentShadingRateStateCreateInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PipelineFragmentShadingRateStateCreateInfoKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("fragment_size", &self.fragment_size).field("combiner_ops", &self.combiner_ops).finish()
    }
}
impl PipelineFragmentShadingRateStateCreateInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> PipelineFragmentShadingRateStateCreateInfoKHRBuilder<'a> {
        PipelineFragmentShadingRateStateCreateInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineFragmentShadingRateStateCreateInfoKHR.html) · Builder of [`PipelineFragmentShadingRateStateCreateInfoKHR`]"]
#[repr(transparent)]
pub struct PipelineFragmentShadingRateStateCreateInfoKHRBuilder<'a>(PipelineFragmentShadingRateStateCreateInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> PipelineFragmentShadingRateStateCreateInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineFragmentShadingRateStateCreateInfoKHRBuilder<'a> {
        PipelineFragmentShadingRateStateCreateInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn fragment_size(mut self, fragment_size: crate::vk1_0::Extent2D) -> Self {
        self.0.fragment_size = fragment_size as _;
        self
    }
    #[inline]
    pub fn combiner_ops(mut self, combiner_ops: [crate::extensions::khr_fragment_shading_rate::FragmentShadingRateCombinerOpKHR; 2]) -> Self {
        self.0.combiner_ops = combiner_ops as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PipelineFragmentShadingRateStateCreateInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for PipelineFragmentShadingRateStateCreateInfoKHRBuilder<'a> {
    fn default() -> PipelineFragmentShadingRateStateCreateInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PipelineFragmentShadingRateStateCreateInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PipelineFragmentShadingRateStateCreateInfoKHRBuilder<'a> {
    type Target = PipelineFragmentShadingRateStateCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineFragmentShadingRateStateCreateInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceFragmentShadingRateFeaturesKHR.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceFragmentShadingRateFeaturesKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceFragmentShadingRateFeaturesKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub pipeline_fragment_shading_rate: crate::vk1_0::Bool32,
    pub primitive_fragment_shading_rate: crate::vk1_0::Bool32,
    pub attachment_fragment_shading_rate: crate::vk1_0::Bool32,
}
impl PhysicalDeviceFragmentShadingRateFeaturesKHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_FEATURES_KHR;
}
impl Default for PhysicalDeviceFragmentShadingRateFeaturesKHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), pipeline_fragment_shading_rate: Default::default(), primitive_fragment_shading_rate: Default::default(), attachment_fragment_shading_rate: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceFragmentShadingRateFeaturesKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceFragmentShadingRateFeaturesKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("pipeline_fragment_shading_rate", &(self.pipeline_fragment_shading_rate != 0)).field("primitive_fragment_shading_rate", &(self.primitive_fragment_shading_rate != 0)).field("attachment_fragment_shading_rate", &(self.attachment_fragment_shading_rate != 0)).finish()
    }
}
impl PhysicalDeviceFragmentShadingRateFeaturesKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceFragmentShadingRateFeaturesKHRBuilder<'a> {
        PhysicalDeviceFragmentShadingRateFeaturesKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceFragmentShadingRateFeaturesKHR.html) · Builder of [`PhysicalDeviceFragmentShadingRateFeaturesKHR`]"]
#[repr(transparent)]
pub struct PhysicalDeviceFragmentShadingRateFeaturesKHRBuilder<'a>(PhysicalDeviceFragmentShadingRateFeaturesKHR, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceFragmentShadingRateFeaturesKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceFragmentShadingRateFeaturesKHRBuilder<'a> {
        PhysicalDeviceFragmentShadingRateFeaturesKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn pipeline_fragment_shading_rate(mut self, pipeline_fragment_shading_rate: bool) -> Self {
        self.0.pipeline_fragment_shading_rate = pipeline_fragment_shading_rate as _;
        self
    }
    #[inline]
    pub fn primitive_fragment_shading_rate(mut self, primitive_fragment_shading_rate: bool) -> Self {
        self.0.primitive_fragment_shading_rate = primitive_fragment_shading_rate as _;
        self
    }
    #[inline]
    pub fn attachment_fragment_shading_rate(mut self, attachment_fragment_shading_rate: bool) -> Self {
        self.0.attachment_fragment_shading_rate = attachment_fragment_shading_rate as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceFragmentShadingRateFeaturesKHR {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceFragmentShadingRateFeaturesKHRBuilder<'a> {
    fn default() -> PhysicalDeviceFragmentShadingRateFeaturesKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceFragmentShadingRateFeaturesKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceFragmentShadingRateFeaturesKHRBuilder<'a> {
    type Target = PhysicalDeviceFragmentShadingRateFeaturesKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceFragmentShadingRateFeaturesKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceFragmentShadingRatePropertiesKHR.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceFragmentShadingRatePropertiesKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceFragmentShadingRatePropertiesKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub min_fragment_shading_rate_attachment_texel_size: crate::vk1_0::Extent2D,
    pub max_fragment_shading_rate_attachment_texel_size: crate::vk1_0::Extent2D,
    pub max_fragment_shading_rate_attachment_texel_size_aspect_ratio: u32,
    pub primitive_fragment_shading_rate_with_multiple_viewports: crate::vk1_0::Bool32,
    pub layered_shading_rate_attachments: crate::vk1_0::Bool32,
    pub fragment_shading_rate_non_trivial_combiner_ops: crate::vk1_0::Bool32,
    pub max_fragment_size: crate::vk1_0::Extent2D,
    pub max_fragment_size_aspect_ratio: u32,
    pub max_fragment_shading_rate_coverage_samples: u32,
    pub max_fragment_shading_rate_rasterization_samples: crate::vk1_0::SampleCountFlagBits,
    pub fragment_shading_rate_with_shader_depth_stencil_writes: crate::vk1_0::Bool32,
    pub fragment_shading_rate_with_sample_mask: crate::vk1_0::Bool32,
    pub fragment_shading_rate_with_shader_sample_mask: crate::vk1_0::Bool32,
    pub fragment_shading_rate_with_conservative_rasterization: crate::vk1_0::Bool32,
    pub fragment_shading_rate_with_fragment_shader_interlock: crate::vk1_0::Bool32,
    pub fragment_shading_rate_with_custom_sample_locations: crate::vk1_0::Bool32,
    pub fragment_shading_rate_strict_multiply_combiner: crate::vk1_0::Bool32,
}
impl PhysicalDeviceFragmentShadingRatePropertiesKHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_PROPERTIES_KHR;
}
impl Default for PhysicalDeviceFragmentShadingRatePropertiesKHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), min_fragment_shading_rate_attachment_texel_size: Default::default(), max_fragment_shading_rate_attachment_texel_size: Default::default(), max_fragment_shading_rate_attachment_texel_size_aspect_ratio: Default::default(), primitive_fragment_shading_rate_with_multiple_viewports: Default::default(), layered_shading_rate_attachments: Default::default(), fragment_shading_rate_non_trivial_combiner_ops: Default::default(), max_fragment_size: Default::default(), max_fragment_size_aspect_ratio: Default::default(), max_fragment_shading_rate_coverage_samples: Default::default(), max_fragment_shading_rate_rasterization_samples: Default::default(), fragment_shading_rate_with_shader_depth_stencil_writes: Default::default(), fragment_shading_rate_with_sample_mask: Default::default(), fragment_shading_rate_with_shader_sample_mask: Default::default(), fragment_shading_rate_with_conservative_rasterization: Default::default(), fragment_shading_rate_with_fragment_shader_interlock: Default::default(), fragment_shading_rate_with_custom_sample_locations: Default::default(), fragment_shading_rate_strict_multiply_combiner: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceFragmentShadingRatePropertiesKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceFragmentShadingRatePropertiesKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("min_fragment_shading_rate_attachment_texel_size", &self.min_fragment_shading_rate_attachment_texel_size).field("max_fragment_shading_rate_attachment_texel_size", &self.max_fragment_shading_rate_attachment_texel_size).field("max_fragment_shading_rate_attachment_texel_size_aspect_ratio", &self.max_fragment_shading_rate_attachment_texel_size_aspect_ratio).field("primitive_fragment_shading_rate_with_multiple_viewports", &(self.primitive_fragment_shading_rate_with_multiple_viewports != 0)).field("layered_shading_rate_attachments", &(self.layered_shading_rate_attachments != 0)).field("fragment_shading_rate_non_trivial_combiner_ops", &(self.fragment_shading_rate_non_trivial_combiner_ops != 0)).field("max_fragment_size", &self.max_fragment_size).field("max_fragment_size_aspect_ratio", &self.max_fragment_size_aspect_ratio).field("max_fragment_shading_rate_coverage_samples", &self.max_fragment_shading_rate_coverage_samples).field("max_fragment_shading_rate_rasterization_samples", &self.max_fragment_shading_rate_rasterization_samples).field("fragment_shading_rate_with_shader_depth_stencil_writes", &(self.fragment_shading_rate_with_shader_depth_stencil_writes != 0)).field("fragment_shading_rate_with_sample_mask", &(self.fragment_shading_rate_with_sample_mask != 0)).field("fragment_shading_rate_with_shader_sample_mask", &(self.fragment_shading_rate_with_shader_sample_mask != 0)).field("fragment_shading_rate_with_conservative_rasterization", &(self.fragment_shading_rate_with_conservative_rasterization != 0)).field("fragment_shading_rate_with_fragment_shader_interlock", &(self.fragment_shading_rate_with_fragment_shader_interlock != 0)).field("fragment_shading_rate_with_custom_sample_locations", &(self.fragment_shading_rate_with_custom_sample_locations != 0)).field("fragment_shading_rate_strict_multiply_combiner", &(self.fragment_shading_rate_strict_multiply_combiner != 0)).finish()
    }
}
impl PhysicalDeviceFragmentShadingRatePropertiesKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceFragmentShadingRatePropertiesKHRBuilder<'a> {
        PhysicalDeviceFragmentShadingRatePropertiesKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceFragmentShadingRatePropertiesKHR.html) · Builder of [`PhysicalDeviceFragmentShadingRatePropertiesKHR`]"]
#[repr(transparent)]
pub struct PhysicalDeviceFragmentShadingRatePropertiesKHRBuilder<'a>(PhysicalDeviceFragmentShadingRatePropertiesKHR, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceFragmentShadingRatePropertiesKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceFragmentShadingRatePropertiesKHRBuilder<'a> {
        PhysicalDeviceFragmentShadingRatePropertiesKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn min_fragment_shading_rate_attachment_texel_size(mut self, min_fragment_shading_rate_attachment_texel_size: crate::vk1_0::Extent2D) -> Self {
        self.0.min_fragment_shading_rate_attachment_texel_size = min_fragment_shading_rate_attachment_texel_size as _;
        self
    }
    #[inline]
    pub fn max_fragment_shading_rate_attachment_texel_size(mut self, max_fragment_shading_rate_attachment_texel_size: crate::vk1_0::Extent2D) -> Self {
        self.0.max_fragment_shading_rate_attachment_texel_size = max_fragment_shading_rate_attachment_texel_size as _;
        self
    }
    #[inline]
    pub fn max_fragment_shading_rate_attachment_texel_size_aspect_ratio(mut self, max_fragment_shading_rate_attachment_texel_size_aspect_ratio: u32) -> Self {
        self.0.max_fragment_shading_rate_attachment_texel_size_aspect_ratio = max_fragment_shading_rate_attachment_texel_size_aspect_ratio as _;
        self
    }
    #[inline]
    pub fn primitive_fragment_shading_rate_with_multiple_viewports(mut self, primitive_fragment_shading_rate_with_multiple_viewports: bool) -> Self {
        self.0.primitive_fragment_shading_rate_with_multiple_viewports = primitive_fragment_shading_rate_with_multiple_viewports as _;
        self
    }
    #[inline]
    pub fn layered_shading_rate_attachments(mut self, layered_shading_rate_attachments: bool) -> Self {
        self.0.layered_shading_rate_attachments = layered_shading_rate_attachments as _;
        self
    }
    #[inline]
    pub fn fragment_shading_rate_non_trivial_combiner_ops(mut self, fragment_shading_rate_non_trivial_combiner_ops: bool) -> Self {
        self.0.fragment_shading_rate_non_trivial_combiner_ops = fragment_shading_rate_non_trivial_combiner_ops as _;
        self
    }
    #[inline]
    pub fn max_fragment_size(mut self, max_fragment_size: crate::vk1_0::Extent2D) -> Self {
        self.0.max_fragment_size = max_fragment_size as _;
        self
    }
    #[inline]
    pub fn max_fragment_size_aspect_ratio(mut self, max_fragment_size_aspect_ratio: u32) -> Self {
        self.0.max_fragment_size_aspect_ratio = max_fragment_size_aspect_ratio as _;
        self
    }
    #[inline]
    pub fn max_fragment_shading_rate_coverage_samples(mut self, max_fragment_shading_rate_coverage_samples: u32) -> Self {
        self.0.max_fragment_shading_rate_coverage_samples = max_fragment_shading_rate_coverage_samples as _;
        self
    }
    #[inline]
    pub fn max_fragment_shading_rate_rasterization_samples(mut self, max_fragment_shading_rate_rasterization_samples: crate::vk1_0::SampleCountFlagBits) -> Self {
        self.0.max_fragment_shading_rate_rasterization_samples = max_fragment_shading_rate_rasterization_samples as _;
        self
    }
    #[inline]
    pub fn fragment_shading_rate_with_shader_depth_stencil_writes(mut self, fragment_shading_rate_with_shader_depth_stencil_writes: bool) -> Self {
        self.0.fragment_shading_rate_with_shader_depth_stencil_writes = fragment_shading_rate_with_shader_depth_stencil_writes as _;
        self
    }
    #[inline]
    pub fn fragment_shading_rate_with_sample_mask(mut self, fragment_shading_rate_with_sample_mask: bool) -> Self {
        self.0.fragment_shading_rate_with_sample_mask = fragment_shading_rate_with_sample_mask as _;
        self
    }
    #[inline]
    pub fn fragment_shading_rate_with_shader_sample_mask(mut self, fragment_shading_rate_with_shader_sample_mask: bool) -> Self {
        self.0.fragment_shading_rate_with_shader_sample_mask = fragment_shading_rate_with_shader_sample_mask as _;
        self
    }
    #[inline]
    pub fn fragment_shading_rate_with_conservative_rasterization(mut self, fragment_shading_rate_with_conservative_rasterization: bool) -> Self {
        self.0.fragment_shading_rate_with_conservative_rasterization = fragment_shading_rate_with_conservative_rasterization as _;
        self
    }
    #[inline]
    pub fn fragment_shading_rate_with_fragment_shader_interlock(mut self, fragment_shading_rate_with_fragment_shader_interlock: bool) -> Self {
        self.0.fragment_shading_rate_with_fragment_shader_interlock = fragment_shading_rate_with_fragment_shader_interlock as _;
        self
    }
    #[inline]
    pub fn fragment_shading_rate_with_custom_sample_locations(mut self, fragment_shading_rate_with_custom_sample_locations: bool) -> Self {
        self.0.fragment_shading_rate_with_custom_sample_locations = fragment_shading_rate_with_custom_sample_locations as _;
        self
    }
    #[inline]
    pub fn fragment_shading_rate_strict_multiply_combiner(mut self, fragment_shading_rate_strict_multiply_combiner: bool) -> Self {
        self.0.fragment_shading_rate_strict_multiply_combiner = fragment_shading_rate_strict_multiply_combiner as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceFragmentShadingRatePropertiesKHR {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceFragmentShadingRatePropertiesKHRBuilder<'a> {
    fn default() -> PhysicalDeviceFragmentShadingRatePropertiesKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceFragmentShadingRatePropertiesKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceFragmentShadingRatePropertiesKHRBuilder<'a> {
    type Target = PhysicalDeviceFragmentShadingRatePropertiesKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceFragmentShadingRatePropertiesKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceFragmentShadingRateKHR.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceFragmentShadingRateKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceFragmentShadingRateKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub sample_counts: crate::vk1_0::SampleCountFlags,
    pub fragment_size: crate::vk1_0::Extent2D,
}
impl PhysicalDeviceFragmentShadingRateKHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_KHR;
}
impl Default for PhysicalDeviceFragmentShadingRateKHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), sample_counts: Default::default(), fragment_size: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceFragmentShadingRateKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceFragmentShadingRateKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("sample_counts", &self.sample_counts).field("fragment_size", &self.fragment_size).finish()
    }
}
impl PhysicalDeviceFragmentShadingRateKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceFragmentShadingRateKHRBuilder<'a> {
        PhysicalDeviceFragmentShadingRateKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceFragmentShadingRateKHR.html) · Builder of [`PhysicalDeviceFragmentShadingRateKHR`]"]
#[repr(transparent)]
pub struct PhysicalDeviceFragmentShadingRateKHRBuilder<'a>(PhysicalDeviceFragmentShadingRateKHR, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceFragmentShadingRateKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceFragmentShadingRateKHRBuilder<'a> {
        PhysicalDeviceFragmentShadingRateKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn sample_counts(mut self, sample_counts: crate::vk1_0::SampleCountFlags) -> Self {
        self.0.sample_counts = sample_counts as _;
        self
    }
    #[inline]
    pub fn fragment_size(mut self, fragment_size: crate::vk1_0::Extent2D) -> Self {
        self.0.fragment_size = fragment_size as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceFragmentShadingRateKHR {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceFragmentShadingRateKHRBuilder<'a> {
    fn default() -> PhysicalDeviceFragmentShadingRateKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceFragmentShadingRateKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceFragmentShadingRateKHRBuilder<'a> {
    type Target = PhysicalDeviceFragmentShadingRateKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceFragmentShadingRateKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::khr_fragment_shading_rate`]"]
impl crate::DeviceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetFragmentShadingRateKHR.html) · Function"]
    #[doc(alias = "vkCmdSetFragmentShadingRateKHR")]
    pub unsafe fn cmd_set_fragment_shading_rate_khr(&self, command_buffer: crate::vk1_0::CommandBuffer, fragment_size: &crate::vk1_0::Extent2D, combiner_ops: [crate::extensions::khr_fragment_shading_rate::FragmentShadingRateCombinerOpKHR; 2]) -> () {
        let _function = self.cmd_set_fragment_shading_rate_khr.expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(command_buffer as _, fragment_size as _, combiner_ops as _);
        ()
    }
}
#[doc = "Provided by [`crate::extensions::khr_fragment_shading_rate`]"]
impl crate::InstanceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceFragmentShadingRatesKHR.html) · Function"]
    #[doc(alias = "vkGetPhysicalDeviceFragmentShadingRatesKHR")]
    pub unsafe fn get_physical_device_fragment_shading_rates_khr(&self, physical_device: crate::vk1_0::PhysicalDevice, fragment_shading_rate_count: Option<u32>) -> crate::utils::VulkanResult<crate::SmallVec<crate::extensions::khr_fragment_shading_rate::PhysicalDeviceFragmentShadingRateKHR>> {
        let _function = self.get_physical_device_fragment_shading_rates_khr.expect(crate::NOT_LOADED_MESSAGE);
        let mut fragment_shading_rate_count = match fragment_shading_rate_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                _function(physical_device as _, &mut v, std::ptr::null_mut());
                v
            }
        };
        let mut fragment_shading_rates = crate::SmallVec::from_elem(Default::default(), fragment_shading_rate_count as _);
        let _return = _function(physical_device as _, &mut fragment_shading_rate_count, fragment_shading_rates.as_mut_ptr());
        crate::utils::VulkanResult::new(_return, fragment_shading_rates)
    }
}
