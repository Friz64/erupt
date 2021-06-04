#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_BLEND_OPERATION_ADVANCED_SPEC_VERSION")]
pub const EXT_BLEND_OPERATION_ADVANCED_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_BLEND_OPERATION_ADVANCED_EXTENSION_NAME")]
pub const EXT_BLEND_OPERATION_ADVANCED_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_blend_operation_advanced");
#[doc = "Provided by [`crate::extensions::ext_blend_operation_advanced`]"]
impl crate::vk1_0::BlendOp {
    pub const ZERO_EXT: Self = Self(1000148000);
    pub const SRC_EXT: Self = Self(1000148001);
    pub const DST_EXT: Self = Self(1000148002);
    pub const SRC_OVER_EXT: Self = Self(1000148003);
    pub const DST_OVER_EXT: Self = Self(1000148004);
    pub const SRC_IN_EXT: Self = Self(1000148005);
    pub const DST_IN_EXT: Self = Self(1000148006);
    pub const SRC_OUT_EXT: Self = Self(1000148007);
    pub const DST_OUT_EXT: Self = Self(1000148008);
    pub const SRC_ATOP_EXT: Self = Self(1000148009);
    pub const DST_ATOP_EXT: Self = Self(1000148010);
    pub const XOR_EXT: Self = Self(1000148011);
    pub const MULTIPLY_EXT: Self = Self(1000148012);
    pub const SCREEN_EXT: Self = Self(1000148013);
    pub const OVERLAY_EXT: Self = Self(1000148014);
    pub const DARKEN_EXT: Self = Self(1000148015);
    pub const LIGHTEN_EXT: Self = Self(1000148016);
    pub const COLORDODGE_EXT: Self = Self(1000148017);
    pub const COLORBURN_EXT: Self = Self(1000148018);
    pub const HARDLIGHT_EXT: Self = Self(1000148019);
    pub const SOFTLIGHT_EXT: Self = Self(1000148020);
    pub const DIFFERENCE_EXT: Self = Self(1000148021);
    pub const EXCLUSION_EXT: Self = Self(1000148022);
    pub const INVERT_EXT: Self = Self(1000148023);
    pub const INVERT_RGB_EXT: Self = Self(1000148024);
    pub const LINEARDODGE_EXT: Self = Self(1000148025);
    pub const LINEARBURN_EXT: Self = Self(1000148026);
    pub const VIVIDLIGHT_EXT: Self = Self(1000148027);
    pub const LINEARLIGHT_EXT: Self = Self(1000148028);
    pub const PINLIGHT_EXT: Self = Self(1000148029);
    pub const HARDMIX_EXT: Self = Self(1000148030);
    pub const HSL_HUE_EXT: Self = Self(1000148031);
    pub const HSL_SATURATION_EXT: Self = Self(1000148032);
    pub const HSL_COLOR_EXT: Self = Self(1000148033);
    pub const HSL_LUMINOSITY_EXT: Self = Self(1000148034);
    pub const PLUS_EXT: Self = Self(1000148035);
    pub const PLUS_CLAMPED_EXT: Self = Self(1000148036);
    pub const PLUS_CLAMPED_ALPHA_EXT: Self = Self(1000148037);
    pub const PLUS_DARKER_EXT: Self = Self(1000148038);
    pub const MINUS_EXT: Self = Self(1000148039);
    pub const MINUS_CLAMPED_EXT: Self = Self(1000148040);
    pub const CONTRAST_EXT: Self = Self(1000148041);
    pub const INVERT_OVG_EXT: Self = Self(1000148042);
    pub const RED_EXT: Self = Self(1000148043);
    pub const GREEN_EXT: Self = Self(1000148044);
    pub const BLUE_EXT: Self = Self(1000148045);
}
#[doc = "Provided by [`crate::extensions::ext_blend_operation_advanced`]"]
impl crate::vk1_0::AccessFlagBits {
    pub const COLOR_ATTACHMENT_READ_NONCOHERENT_EXT: Self = Self(524288);
}
#[doc = "Provided by [`crate::extensions::ext_blend_operation_advanced`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT: Self = Self(1000148000);
    pub const PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT: Self = Self(1000148001);
    pub const PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT: Self = Self(1000148002);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBlendOverlapEXT.html) · Enum"]
#[doc(alias = "VkBlendOverlapEXT")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct BlendOverlapEXT(pub i32);
impl std::fmt::Debug for BlendOverlapEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::UNCORRELATED_EXT => "UNCORRELATED_EXT",
            &Self::DISJOINT_EXT => "DISJOINT_EXT",
            &Self::CONJOINT_EXT => "CONJOINT_EXT",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::ext_blend_operation_advanced`]"]
impl crate::extensions::ext_blend_operation_advanced::BlendOverlapEXT {
    pub const UNCORRELATED_EXT: Self = Self(0);
    pub const DISJOINT_EXT: Self = Self(1);
    pub const CONJOINT_EXT: Self = Self(2);
}
impl<'a> crate::ExtendableFromConst<'a, PhysicalDeviceBlendOperationAdvancedFeaturesEXT> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, PhysicalDeviceBlendOperationAdvancedFeaturesEXTBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, PipelineColorBlendAdvancedStateCreateInfoEXT> for crate::vk1_0::PipelineColorBlendStateCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, PipelineColorBlendAdvancedStateCreateInfoEXTBuilder<'_>> for crate::vk1_0::PipelineColorBlendStateCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceBlendOperationAdvancedFeaturesEXT> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceBlendOperationAdvancedFeaturesEXTBuilder<'_>> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceBlendOperationAdvancedPropertiesEXT> for crate::vk1_1::PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceBlendOperationAdvancedPropertiesEXTBuilder<'_>> for crate::vk1_1::PhysicalDeviceProperties2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceBlendOperationAdvancedFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub advanced_blend_coherent_operations: crate::vk1_0::Bool32,
}
impl Default for PhysicalDeviceBlendOperationAdvancedFeaturesEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT, p_next: std::ptr::null_mut(), advanced_blend_coherent_operations: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceBlendOperationAdvancedFeaturesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceBlendOperationAdvancedFeaturesEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("advanced_blend_coherent_operations", &(self.advanced_blend_coherent_operations != 0)).finish()
    }
}
impl PhysicalDeviceBlendOperationAdvancedFeaturesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceBlendOperationAdvancedFeaturesEXTBuilder<'a> {
        PhysicalDeviceBlendOperationAdvancedFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT.html) · Builder of [`PhysicalDeviceBlendOperationAdvancedFeaturesEXT`]"]
#[repr(transparent)]
pub struct PhysicalDeviceBlendOperationAdvancedFeaturesEXTBuilder<'a>(PhysicalDeviceBlendOperationAdvancedFeaturesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceBlendOperationAdvancedFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceBlendOperationAdvancedFeaturesEXTBuilder<'a> {
        PhysicalDeviceBlendOperationAdvancedFeaturesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn advanced_blend_coherent_operations(mut self, advanced_blend_coherent_operations: bool) -> Self {
        self.0.advanced_blend_coherent_operations = advanced_blend_coherent_operations as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceBlendOperationAdvancedFeaturesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceBlendOperationAdvancedFeaturesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceBlendOperationAdvancedFeaturesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceBlendOperationAdvancedFeaturesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceBlendOperationAdvancedFeaturesEXTBuilder<'a> {
    type Target = PhysicalDeviceBlendOperationAdvancedFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceBlendOperationAdvancedFeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceBlendOperationAdvancedPropertiesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub advanced_blend_max_color_attachments: u32,
    pub advanced_blend_independent_blend: crate::vk1_0::Bool32,
    pub advanced_blend_non_premultiplied_src_color: crate::vk1_0::Bool32,
    pub advanced_blend_non_premultiplied_dst_color: crate::vk1_0::Bool32,
    pub advanced_blend_correlated_overlap: crate::vk1_0::Bool32,
    pub advanced_blend_all_operations: crate::vk1_0::Bool32,
}
impl Default for PhysicalDeviceBlendOperationAdvancedPropertiesEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT, p_next: std::ptr::null_mut(), advanced_blend_max_color_attachments: Default::default(), advanced_blend_independent_blend: Default::default(), advanced_blend_non_premultiplied_src_color: Default::default(), advanced_blend_non_premultiplied_dst_color: Default::default(), advanced_blend_correlated_overlap: Default::default(), advanced_blend_all_operations: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceBlendOperationAdvancedPropertiesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceBlendOperationAdvancedPropertiesEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("advanced_blend_max_color_attachments", &self.advanced_blend_max_color_attachments).field("advanced_blend_independent_blend", &(self.advanced_blend_independent_blend != 0)).field("advanced_blend_non_premultiplied_src_color", &(self.advanced_blend_non_premultiplied_src_color != 0)).field("advanced_blend_non_premultiplied_dst_color", &(self.advanced_blend_non_premultiplied_dst_color != 0)).field("advanced_blend_correlated_overlap", &(self.advanced_blend_correlated_overlap != 0)).field("advanced_blend_all_operations", &(self.advanced_blend_all_operations != 0)).finish()
    }
}
impl PhysicalDeviceBlendOperationAdvancedPropertiesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceBlendOperationAdvancedPropertiesEXTBuilder<'a> {
        PhysicalDeviceBlendOperationAdvancedPropertiesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT.html) · Builder of [`PhysicalDeviceBlendOperationAdvancedPropertiesEXT`]"]
#[repr(transparent)]
pub struct PhysicalDeviceBlendOperationAdvancedPropertiesEXTBuilder<'a>(PhysicalDeviceBlendOperationAdvancedPropertiesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceBlendOperationAdvancedPropertiesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceBlendOperationAdvancedPropertiesEXTBuilder<'a> {
        PhysicalDeviceBlendOperationAdvancedPropertiesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn advanced_blend_max_color_attachments(mut self, advanced_blend_max_color_attachments: u32) -> Self {
        self.0.advanced_blend_max_color_attachments = advanced_blend_max_color_attachments as _;
        self
    }
    #[inline]
    pub fn advanced_blend_independent_blend(mut self, advanced_blend_independent_blend: bool) -> Self {
        self.0.advanced_blend_independent_blend = advanced_blend_independent_blend as _;
        self
    }
    #[inline]
    pub fn advanced_blend_non_premultiplied_src_color(mut self, advanced_blend_non_premultiplied_src_color: bool) -> Self {
        self.0.advanced_blend_non_premultiplied_src_color = advanced_blend_non_premultiplied_src_color as _;
        self
    }
    #[inline]
    pub fn advanced_blend_non_premultiplied_dst_color(mut self, advanced_blend_non_premultiplied_dst_color: bool) -> Self {
        self.0.advanced_blend_non_premultiplied_dst_color = advanced_blend_non_premultiplied_dst_color as _;
        self
    }
    #[inline]
    pub fn advanced_blend_correlated_overlap(mut self, advanced_blend_correlated_overlap: bool) -> Self {
        self.0.advanced_blend_correlated_overlap = advanced_blend_correlated_overlap as _;
        self
    }
    #[inline]
    pub fn advanced_blend_all_operations(mut self, advanced_blend_all_operations: bool) -> Self {
        self.0.advanced_blend_all_operations = advanced_blend_all_operations as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceBlendOperationAdvancedPropertiesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceBlendOperationAdvancedPropertiesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceBlendOperationAdvancedPropertiesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceBlendOperationAdvancedPropertiesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceBlendOperationAdvancedPropertiesEXTBuilder<'a> {
    type Target = PhysicalDeviceBlendOperationAdvancedPropertiesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceBlendOperationAdvancedPropertiesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineColorBlendAdvancedStateCreateInfoEXT.html) · Structure"]
#[doc(alias = "VkPipelineColorBlendAdvancedStateCreateInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineColorBlendAdvancedStateCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub src_premultiplied: crate::vk1_0::Bool32,
    pub dst_premultiplied: crate::vk1_0::Bool32,
    pub blend_overlap: crate::extensions::ext_blend_operation_advanced::BlendOverlapEXT,
}
impl Default for PipelineColorBlendAdvancedStateCreateInfoEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT, p_next: std::ptr::null(), src_premultiplied: Default::default(), dst_premultiplied: Default::default(), blend_overlap: Default::default() }
    }
}
impl std::fmt::Debug for PipelineColorBlendAdvancedStateCreateInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PipelineColorBlendAdvancedStateCreateInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("src_premultiplied", &(self.src_premultiplied != 0)).field("dst_premultiplied", &(self.dst_premultiplied != 0)).field("blend_overlap", &self.blend_overlap).finish()
    }
}
impl PipelineColorBlendAdvancedStateCreateInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PipelineColorBlendAdvancedStateCreateInfoEXTBuilder<'a> {
        PipelineColorBlendAdvancedStateCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineColorBlendAdvancedStateCreateInfoEXT.html) · Builder of [`PipelineColorBlendAdvancedStateCreateInfoEXT`]"]
#[repr(transparent)]
pub struct PipelineColorBlendAdvancedStateCreateInfoEXTBuilder<'a>(PipelineColorBlendAdvancedStateCreateInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PipelineColorBlendAdvancedStateCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineColorBlendAdvancedStateCreateInfoEXTBuilder<'a> {
        PipelineColorBlendAdvancedStateCreateInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn src_premultiplied(mut self, src_premultiplied: bool) -> Self {
        self.0.src_premultiplied = src_premultiplied as _;
        self
    }
    #[inline]
    pub fn dst_premultiplied(mut self, dst_premultiplied: bool) -> Self {
        self.0.dst_premultiplied = dst_premultiplied as _;
        self
    }
    #[inline]
    pub fn blend_overlap(mut self, blend_overlap: crate::extensions::ext_blend_operation_advanced::BlendOverlapEXT) -> Self {
        self.0.blend_overlap = blend_overlap as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PipelineColorBlendAdvancedStateCreateInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for PipelineColorBlendAdvancedStateCreateInfoEXTBuilder<'a> {
    fn default() -> PipelineColorBlendAdvancedStateCreateInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PipelineColorBlendAdvancedStateCreateInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PipelineColorBlendAdvancedStateCreateInfoEXTBuilder<'a> {
    type Target = PipelineColorBlendAdvancedStateCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineColorBlendAdvancedStateCreateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
