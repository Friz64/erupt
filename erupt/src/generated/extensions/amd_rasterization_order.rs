#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_AMD_RASTERIZATION_ORDER_SPEC_VERSION")]
pub const AMD_RASTERIZATION_ORDER_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_AMD_RASTERIZATION_ORDER_EXTENSION_NAME")]
pub const AMD_RASTERIZATION_ORDER_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_AMD_rasterization_order");
#[doc = "Provided by [`crate::extensions::amd_rasterization_order`]"]
impl crate::vk1_0::StructureType {
    pub const PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD: Self = Self(1000018000);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRasterizationOrderAMD.html) · Enum"]
#[doc(alias = "VkRasterizationOrderAMD")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct RasterizationOrderAMD(pub i32);
impl std::fmt::Debug for RasterizationOrderAMD {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::STRICT_AMD => "STRICT_AMD",
            &Self::RELAXED_AMD => "RELAXED_AMD",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::amd_rasterization_order`]"]
impl crate::extensions::amd_rasterization_order::RasterizationOrderAMD {
    pub const STRICT_AMD: Self = Self(0);
    pub const RELAXED_AMD: Self = Self(1);
}
impl<'a> crate::ExtendableFromConst<'a, PipelineRasterizationStateRasterizationOrderAMD> for crate::vk1_0::PipelineRasterizationStateCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, PipelineRasterizationStateRasterizationOrderAMDBuilder<'_>> for crate::vk1_0::PipelineRasterizationStateCreateInfoBuilder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineRasterizationStateRasterizationOrderAMD.html) · Structure"]
#[doc(alias = "VkPipelineRasterizationStateRasterizationOrderAMD")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineRasterizationStateRasterizationOrderAMD {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub rasterization_order: crate::extensions::amd_rasterization_order::RasterizationOrderAMD,
}
impl PipelineRasterizationStateRasterizationOrderAMD {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD;
}
impl Default for PipelineRasterizationStateRasterizationOrderAMD {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD, p_next: std::ptr::null(), rasterization_order: Default::default() }
    }
}
impl std::fmt::Debug for PipelineRasterizationStateRasterizationOrderAMD {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PipelineRasterizationStateRasterizationOrderAMD").field("s_type", &self.s_type).field("p_next", &self.p_next).field("rasterization_order", &self.rasterization_order).finish()
    }
}
impl PipelineRasterizationStateRasterizationOrderAMD {
    #[inline]
    pub fn into_builder<'a>(self) -> PipelineRasterizationStateRasterizationOrderAMDBuilder<'a> {
        PipelineRasterizationStateRasterizationOrderAMDBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineRasterizationStateRasterizationOrderAMD.html) · Builder of [`PipelineRasterizationStateRasterizationOrderAMD`]"]
#[repr(transparent)]
pub struct PipelineRasterizationStateRasterizationOrderAMDBuilder<'a>(PipelineRasterizationStateRasterizationOrderAMD, std::marker::PhantomData<&'a ()>);
impl<'a> PipelineRasterizationStateRasterizationOrderAMDBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineRasterizationStateRasterizationOrderAMDBuilder<'a> {
        PipelineRasterizationStateRasterizationOrderAMDBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn rasterization_order(mut self, rasterization_order: crate::extensions::amd_rasterization_order::RasterizationOrderAMD) -> Self {
        self.0.rasterization_order = rasterization_order as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PipelineRasterizationStateRasterizationOrderAMD {
        self.0
    }
}
impl<'a> std::default::Default for PipelineRasterizationStateRasterizationOrderAMDBuilder<'a> {
    fn default() -> PipelineRasterizationStateRasterizationOrderAMDBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PipelineRasterizationStateRasterizationOrderAMDBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PipelineRasterizationStateRasterizationOrderAMDBuilder<'a> {
    type Target = PipelineRasterizationStateRasterizationOrderAMD;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineRasterizationStateRasterizationOrderAMDBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
