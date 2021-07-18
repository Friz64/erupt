//! ## Versioning Warning ⚠️
//!
//! This is a Vulkan **provisional/beta** extension and **must** be used with
//! caution. Its API/behaviour has not been finalized yet and _may_ therefore
//! change in ways that break backwards compatibility between revisions, and
//! before final release of a non-provisional version of this extension.
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_PORTABILITY_SUBSET_SPEC_VERSION")]
pub const KHR_PORTABILITY_SUBSET_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_PORTABILITY_SUBSET_EXTENSION_NAME")]
pub const KHR_PORTABILITY_SUBSET_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_portability_subset");
#[doc = "Provided by [`crate::extensions::khr_portability_subset`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_PORTABILITY_SUBSET_FEATURES_KHR: Self = Self(1000163000);
    pub const PHYSICAL_DEVICE_PORTABILITY_SUBSET_PROPERTIES_KHR: Self = Self(1000163001);
}
impl<'a> crate::ExtendableFromConst<'a, PhysicalDevicePortabilitySubsetFeaturesKHR> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, PhysicalDevicePortabilitySubsetFeaturesKHRBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDevicePortabilitySubsetFeaturesKHR> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDevicePortabilitySubsetFeaturesKHRBuilder<'_>> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDevicePortabilitySubsetPropertiesKHR> for crate::vk1_1::PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDevicePortabilitySubsetPropertiesKHRBuilder<'_>> for crate::vk1_1::PhysicalDeviceProperties2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevicePortabilitySubsetFeaturesKHR.html) · Structure"]
#[doc(alias = "VkPhysicalDevicePortabilitySubsetFeaturesKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDevicePortabilitySubsetFeaturesKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub constant_alpha_color_blend_factors: crate::vk1_0::Bool32,
    pub events: crate::vk1_0::Bool32,
    pub image_view_format_reinterpretation: crate::vk1_0::Bool32,
    pub image_view_format_swizzle: crate::vk1_0::Bool32,
    pub image_view2_d_on3_d_image: crate::vk1_0::Bool32,
    pub multisample_array_image: crate::vk1_0::Bool32,
    pub mutable_comparison_samplers: crate::vk1_0::Bool32,
    pub point_polygons: crate::vk1_0::Bool32,
    pub sampler_mip_lod_bias: crate::vk1_0::Bool32,
    pub separate_stencil_mask_ref: crate::vk1_0::Bool32,
    pub shader_sample_rate_interpolation_functions: crate::vk1_0::Bool32,
    pub tessellation_isolines: crate::vk1_0::Bool32,
    pub tessellation_point_mode: crate::vk1_0::Bool32,
    pub triangle_fans: crate::vk1_0::Bool32,
    pub vertex_attribute_access_beyond_stride: crate::vk1_0::Bool32,
}
impl PhysicalDevicePortabilitySubsetFeaturesKHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_PORTABILITY_SUBSET_FEATURES_KHR;
}
impl Default for PhysicalDevicePortabilitySubsetFeaturesKHR {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_PORTABILITY_SUBSET_FEATURES_KHR, p_next: std::ptr::null_mut(), constant_alpha_color_blend_factors: Default::default(), events: Default::default(), image_view_format_reinterpretation: Default::default(), image_view_format_swizzle: Default::default(), image_view2_d_on3_d_image: Default::default(), multisample_array_image: Default::default(), mutable_comparison_samplers: Default::default(), point_polygons: Default::default(), sampler_mip_lod_bias: Default::default(), separate_stencil_mask_ref: Default::default(), shader_sample_rate_interpolation_functions: Default::default(), tessellation_isolines: Default::default(), tessellation_point_mode: Default::default(), triangle_fans: Default::default(), vertex_attribute_access_beyond_stride: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDevicePortabilitySubsetFeaturesKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDevicePortabilitySubsetFeaturesKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("constant_alpha_color_blend_factors", &(self.constant_alpha_color_blend_factors != 0)).field("events", &(self.events != 0)).field("image_view_format_reinterpretation", &(self.image_view_format_reinterpretation != 0)).field("image_view_format_swizzle", &(self.image_view_format_swizzle != 0)).field("image_view2_d_on3_d_image", &(self.image_view2_d_on3_d_image != 0)).field("multisample_array_image", &(self.multisample_array_image != 0)).field("mutable_comparison_samplers", &(self.mutable_comparison_samplers != 0)).field("point_polygons", &(self.point_polygons != 0)).field("sampler_mip_lod_bias", &(self.sampler_mip_lod_bias != 0)).field("separate_stencil_mask_ref", &(self.separate_stencil_mask_ref != 0)).field("shader_sample_rate_interpolation_functions", &(self.shader_sample_rate_interpolation_functions != 0)).field("tessellation_isolines", &(self.tessellation_isolines != 0)).field("tessellation_point_mode", &(self.tessellation_point_mode != 0)).field("triangle_fans", &(self.triangle_fans != 0)).field("vertex_attribute_access_beyond_stride", &(self.vertex_attribute_access_beyond_stride != 0)).finish()
    }
}
impl PhysicalDevicePortabilitySubsetFeaturesKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDevicePortabilitySubsetFeaturesKHRBuilder<'a> {
        PhysicalDevicePortabilitySubsetFeaturesKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevicePortabilitySubsetFeaturesKHR.html) · Builder of [`PhysicalDevicePortabilitySubsetFeaturesKHR`]"]
#[repr(transparent)]
pub struct PhysicalDevicePortabilitySubsetFeaturesKHRBuilder<'a>(PhysicalDevicePortabilitySubsetFeaturesKHR, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDevicePortabilitySubsetFeaturesKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDevicePortabilitySubsetFeaturesKHRBuilder<'a> {
        PhysicalDevicePortabilitySubsetFeaturesKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn constant_alpha_color_blend_factors(mut self, constant_alpha_color_blend_factors: bool) -> Self {
        self.0.constant_alpha_color_blend_factors = constant_alpha_color_blend_factors as _;
        self
    }
    #[inline]
    pub fn events(mut self, events: bool) -> Self {
        self.0.events = events as _;
        self
    }
    #[inline]
    pub fn image_view_format_reinterpretation(mut self, image_view_format_reinterpretation: bool) -> Self {
        self.0.image_view_format_reinterpretation = image_view_format_reinterpretation as _;
        self
    }
    #[inline]
    pub fn image_view_format_swizzle(mut self, image_view_format_swizzle: bool) -> Self {
        self.0.image_view_format_swizzle = image_view_format_swizzle as _;
        self
    }
    #[inline]
    pub fn image_view2_d_on3_d_image(mut self, image_view2_d_on3_d_image: bool) -> Self {
        self.0.image_view2_d_on3_d_image = image_view2_d_on3_d_image as _;
        self
    }
    #[inline]
    pub fn multisample_array_image(mut self, multisample_array_image: bool) -> Self {
        self.0.multisample_array_image = multisample_array_image as _;
        self
    }
    #[inline]
    pub fn mutable_comparison_samplers(mut self, mutable_comparison_samplers: bool) -> Self {
        self.0.mutable_comparison_samplers = mutable_comparison_samplers as _;
        self
    }
    #[inline]
    pub fn point_polygons(mut self, point_polygons: bool) -> Self {
        self.0.point_polygons = point_polygons as _;
        self
    }
    #[inline]
    pub fn sampler_mip_lod_bias(mut self, sampler_mip_lod_bias: bool) -> Self {
        self.0.sampler_mip_lod_bias = sampler_mip_lod_bias as _;
        self
    }
    #[inline]
    pub fn separate_stencil_mask_ref(mut self, separate_stencil_mask_ref: bool) -> Self {
        self.0.separate_stencil_mask_ref = separate_stencil_mask_ref as _;
        self
    }
    #[inline]
    pub fn shader_sample_rate_interpolation_functions(mut self, shader_sample_rate_interpolation_functions: bool) -> Self {
        self.0.shader_sample_rate_interpolation_functions = shader_sample_rate_interpolation_functions as _;
        self
    }
    #[inline]
    pub fn tessellation_isolines(mut self, tessellation_isolines: bool) -> Self {
        self.0.tessellation_isolines = tessellation_isolines as _;
        self
    }
    #[inline]
    pub fn tessellation_point_mode(mut self, tessellation_point_mode: bool) -> Self {
        self.0.tessellation_point_mode = tessellation_point_mode as _;
        self
    }
    #[inline]
    pub fn triangle_fans(mut self, triangle_fans: bool) -> Self {
        self.0.triangle_fans = triangle_fans as _;
        self
    }
    #[inline]
    pub fn vertex_attribute_access_beyond_stride(mut self, vertex_attribute_access_beyond_stride: bool) -> Self {
        self.0.vertex_attribute_access_beyond_stride = vertex_attribute_access_beyond_stride as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDevicePortabilitySubsetFeaturesKHR {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDevicePortabilitySubsetFeaturesKHRBuilder<'a> {
    fn default() -> PhysicalDevicePortabilitySubsetFeaturesKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDevicePortabilitySubsetFeaturesKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDevicePortabilitySubsetFeaturesKHRBuilder<'a> {
    type Target = PhysicalDevicePortabilitySubsetFeaturesKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDevicePortabilitySubsetFeaturesKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevicePortabilitySubsetPropertiesKHR.html) · Structure"]
#[doc(alias = "VkPhysicalDevicePortabilitySubsetPropertiesKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDevicePortabilitySubsetPropertiesKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub min_vertex_input_binding_stride_alignment: u32,
}
impl PhysicalDevicePortabilitySubsetPropertiesKHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_PORTABILITY_SUBSET_PROPERTIES_KHR;
}
impl Default for PhysicalDevicePortabilitySubsetPropertiesKHR {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_PORTABILITY_SUBSET_PROPERTIES_KHR, p_next: std::ptr::null_mut(), min_vertex_input_binding_stride_alignment: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDevicePortabilitySubsetPropertiesKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDevicePortabilitySubsetPropertiesKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("min_vertex_input_binding_stride_alignment", &self.min_vertex_input_binding_stride_alignment).finish()
    }
}
impl PhysicalDevicePortabilitySubsetPropertiesKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDevicePortabilitySubsetPropertiesKHRBuilder<'a> {
        PhysicalDevicePortabilitySubsetPropertiesKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevicePortabilitySubsetPropertiesKHR.html) · Builder of [`PhysicalDevicePortabilitySubsetPropertiesKHR`]"]
#[repr(transparent)]
pub struct PhysicalDevicePortabilitySubsetPropertiesKHRBuilder<'a>(PhysicalDevicePortabilitySubsetPropertiesKHR, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDevicePortabilitySubsetPropertiesKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDevicePortabilitySubsetPropertiesKHRBuilder<'a> {
        PhysicalDevicePortabilitySubsetPropertiesKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn min_vertex_input_binding_stride_alignment(mut self, min_vertex_input_binding_stride_alignment: u32) -> Self {
        self.0.min_vertex_input_binding_stride_alignment = min_vertex_input_binding_stride_alignment as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDevicePortabilitySubsetPropertiesKHR {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDevicePortabilitySubsetPropertiesKHRBuilder<'a> {
    fn default() -> PhysicalDevicePortabilitySubsetPropertiesKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDevicePortabilitySubsetPropertiesKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDevicePortabilitySubsetPropertiesKHRBuilder<'a> {
    type Target = PhysicalDevicePortabilitySubsetPropertiesKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDevicePortabilitySubsetPropertiesKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
