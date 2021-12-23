#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_PROVOKING_VERTEX_SPEC_VERSION")]
pub const EXT_PROVOKING_VERTEX_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_PROVOKING_VERTEX_EXTENSION_NAME")]
pub const EXT_PROVOKING_VERTEX_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_provoking_vertex");
#[doc = "Provided by [`crate::extensions::ext_provoking_vertex`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_PROVOKING_VERTEX_FEATURES_EXT: Self = Self(1000254000);
    pub const PIPELINE_RASTERIZATION_PROVOKING_VERTEX_STATE_CREATE_INFO_EXT: Self = Self(1000254001);
    pub const PHYSICAL_DEVICE_PROVOKING_VERTEX_PROPERTIES_EXT: Self = Self(1000254002);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkProvokingVertexModeEXT.html) · Enum"]
#[doc(alias = "VkProvokingVertexModeEXT")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct ProvokingVertexModeEXT(pub i32);
impl std::fmt::Debug for ProvokingVertexModeEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::FIRST_VERTEX_EXT => "FIRST_VERTEX_EXT",
            &Self::LAST_VERTEX_EXT => "LAST_VERTEX_EXT",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::ext_provoking_vertex`]"]
impl crate::extensions::ext_provoking_vertex::ProvokingVertexModeEXT {
    pub const FIRST_VERTEX_EXT: Self = Self(0);
    pub const LAST_VERTEX_EXT: Self = Self(1);
}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceProvokingVertexFeaturesEXT> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceProvokingVertexFeaturesEXTBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PipelineRasterizationProvokingVertexStateCreateInfoEXT> for crate::vk1_0::PipelineRasterizationStateCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PipelineRasterizationProvokingVertexStateCreateInfoEXTBuilder<'_>> for crate::vk1_0::PipelineRasterizationStateCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceProvokingVertexFeaturesEXT> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceProvokingVertexFeaturesEXTBuilder<'_>> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceProvokingVertexPropertiesEXT> for crate::vk1_1::PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceProvokingVertexPropertiesEXTBuilder<'_>> for crate::vk1_1::PhysicalDeviceProperties2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceProvokingVertexFeaturesEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceProvokingVertexFeaturesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceProvokingVertexFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub provoking_vertex_last: crate::vk1_0::Bool32,
    pub transform_feedback_preserves_provoking_vertex: crate::vk1_0::Bool32,
}
impl PhysicalDeviceProvokingVertexFeaturesEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_PROVOKING_VERTEX_FEATURES_EXT;
}
impl Default for PhysicalDeviceProvokingVertexFeaturesEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), provoking_vertex_last: Default::default(), transform_feedback_preserves_provoking_vertex: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceProvokingVertexFeaturesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceProvokingVertexFeaturesEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("provoking_vertex_last", &(self.provoking_vertex_last != 0)).field("transform_feedback_preserves_provoking_vertex", &(self.transform_feedback_preserves_provoking_vertex != 0)).finish()
    }
}
impl PhysicalDeviceProvokingVertexFeaturesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceProvokingVertexFeaturesEXTBuilder<'a> {
        PhysicalDeviceProvokingVertexFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceProvokingVertexFeaturesEXT.html) · Builder of [`PhysicalDeviceProvokingVertexFeaturesEXT`]"]
#[repr(transparent)]
pub struct PhysicalDeviceProvokingVertexFeaturesEXTBuilder<'a>(PhysicalDeviceProvokingVertexFeaturesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceProvokingVertexFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceProvokingVertexFeaturesEXTBuilder<'a> {
        PhysicalDeviceProvokingVertexFeaturesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn provoking_vertex_last(mut self, provoking_vertex_last: bool) -> Self {
        self.0.provoking_vertex_last = provoking_vertex_last as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn transform_feedback_preserves_provoking_vertex(mut self, transform_feedback_preserves_provoking_vertex: bool) -> Self {
        self.0.transform_feedback_preserves_provoking_vertex = transform_feedback_preserves_provoking_vertex as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> PhysicalDeviceProvokingVertexFeaturesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceProvokingVertexFeaturesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceProvokingVertexFeaturesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceProvokingVertexFeaturesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceProvokingVertexFeaturesEXTBuilder<'a> {
    type Target = PhysicalDeviceProvokingVertexFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceProvokingVertexFeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceProvokingVertexPropertiesEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceProvokingVertexPropertiesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceProvokingVertexPropertiesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub provoking_vertex_mode_per_pipeline: crate::vk1_0::Bool32,
    pub transform_feedback_preserves_triangle_fan_provoking_vertex: crate::vk1_0::Bool32,
}
impl PhysicalDeviceProvokingVertexPropertiesEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_PROVOKING_VERTEX_PROPERTIES_EXT;
}
impl Default for PhysicalDeviceProvokingVertexPropertiesEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), provoking_vertex_mode_per_pipeline: Default::default(), transform_feedback_preserves_triangle_fan_provoking_vertex: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceProvokingVertexPropertiesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceProvokingVertexPropertiesEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("provoking_vertex_mode_per_pipeline", &(self.provoking_vertex_mode_per_pipeline != 0)).field("transform_feedback_preserves_triangle_fan_provoking_vertex", &(self.transform_feedback_preserves_triangle_fan_provoking_vertex != 0)).finish()
    }
}
impl PhysicalDeviceProvokingVertexPropertiesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceProvokingVertexPropertiesEXTBuilder<'a> {
        PhysicalDeviceProvokingVertexPropertiesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceProvokingVertexPropertiesEXT.html) · Builder of [`PhysicalDeviceProvokingVertexPropertiesEXT`]"]
#[repr(transparent)]
pub struct PhysicalDeviceProvokingVertexPropertiesEXTBuilder<'a>(PhysicalDeviceProvokingVertexPropertiesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceProvokingVertexPropertiesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceProvokingVertexPropertiesEXTBuilder<'a> {
        PhysicalDeviceProvokingVertexPropertiesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn provoking_vertex_mode_per_pipeline(mut self, provoking_vertex_mode_per_pipeline: bool) -> Self {
        self.0.provoking_vertex_mode_per_pipeline = provoking_vertex_mode_per_pipeline as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn transform_feedback_preserves_triangle_fan_provoking_vertex(mut self, transform_feedback_preserves_triangle_fan_provoking_vertex: bool) -> Self {
        self.0.transform_feedback_preserves_triangle_fan_provoking_vertex = transform_feedback_preserves_triangle_fan_provoking_vertex as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> PhysicalDeviceProvokingVertexPropertiesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceProvokingVertexPropertiesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceProvokingVertexPropertiesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceProvokingVertexPropertiesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceProvokingVertexPropertiesEXTBuilder<'a> {
    type Target = PhysicalDeviceProvokingVertexPropertiesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceProvokingVertexPropertiesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineRasterizationProvokingVertexStateCreateInfoEXT.html) · Structure"]
#[doc(alias = "VkPipelineRasterizationProvokingVertexStateCreateInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineRasterizationProvokingVertexStateCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub provoking_vertex_mode: crate::extensions::ext_provoking_vertex::ProvokingVertexModeEXT,
}
impl PipelineRasterizationProvokingVertexStateCreateInfoEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PIPELINE_RASTERIZATION_PROVOKING_VERTEX_STATE_CREATE_INFO_EXT;
}
impl Default for PipelineRasterizationProvokingVertexStateCreateInfoEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), provoking_vertex_mode: Default::default() }
    }
}
impl std::fmt::Debug for PipelineRasterizationProvokingVertexStateCreateInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PipelineRasterizationProvokingVertexStateCreateInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("provoking_vertex_mode", &self.provoking_vertex_mode).finish()
    }
}
impl PipelineRasterizationProvokingVertexStateCreateInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PipelineRasterizationProvokingVertexStateCreateInfoEXTBuilder<'a> {
        PipelineRasterizationProvokingVertexStateCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineRasterizationProvokingVertexStateCreateInfoEXT.html) · Builder of [`PipelineRasterizationProvokingVertexStateCreateInfoEXT`]"]
#[repr(transparent)]
pub struct PipelineRasterizationProvokingVertexStateCreateInfoEXTBuilder<'a>(PipelineRasterizationProvokingVertexStateCreateInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PipelineRasterizationProvokingVertexStateCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineRasterizationProvokingVertexStateCreateInfoEXTBuilder<'a> {
        PipelineRasterizationProvokingVertexStateCreateInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn provoking_vertex_mode(mut self, provoking_vertex_mode: crate::extensions::ext_provoking_vertex::ProvokingVertexModeEXT) -> Self {
        self.0.provoking_vertex_mode = provoking_vertex_mode as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> PipelineRasterizationProvokingVertexStateCreateInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for PipelineRasterizationProvokingVertexStateCreateInfoEXTBuilder<'a> {
    fn default() -> PipelineRasterizationProvokingVertexStateCreateInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PipelineRasterizationProvokingVertexStateCreateInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PipelineRasterizationProvokingVertexStateCreateInfoEXTBuilder<'a> {
    type Target = PipelineRasterizationProvokingVertexStateCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineRasterizationProvokingVertexStateCreateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
