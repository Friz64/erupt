# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_AMD_rasterization_order.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const AMD_RASTERIZATION_ORDER_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const AMD_RASTERIZATION_ORDER_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_AMD_rasterization_order");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRasterizationOrderAMD.html) · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct RasterizationOrderAMD(pub i32);
#[doc = "[Part of `extensions::amd_rasterization_order`](../../extensions/amd_rasterization_order/index.html)"]
impl RasterizationOrderAMD {
    pub const STRICT_AMD: Self = Self(0);
    pub const RELAXED_AMD: Self = Self(1);
}
impl std::fmt::Debug for RasterizationOrderAMD {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::STRICT_AMD => "STRICT_AMD",
            &Self::RELAXED_AMD => "RELAXED_AMD",
            _ => "Unknown enum variant",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineRasterizationStateRasterizationOrderAMD.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineRasterizationStateRasterizationOrderAMD {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub rasterization_order: crate::extensions::amd_rasterization_order::RasterizationOrderAMD,
}
impl PipelineRasterizationStateRasterizationOrderAMD {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPipelineRasterizationStateRasterizationOrderAMD,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PipelineRasterizationStateRasterizationOrderAMDBuilder<'a> {
        PipelineRasterizationStateRasterizationOrderAMDBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PipelineRasterizationStateRasterizationOrderAMD {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PipelineRasterizationStateRasterizationOrderAMD")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("rasterization_order", &self.rasterization_order)
            .finish()
    }
}
impl Default for PipelineRasterizationStateRasterizationOrderAMD {
    fn default() -> PipelineRasterizationStateRasterizationOrderAMD {
        PipelineRasterizationStateRasterizationOrderAMD {
            s_type:
                crate::vk1_0::StructureType::PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD,
            p_next: std::ptr::null(),
            rasterization_order: Default::default(),
        }
    }
}
#[doc = "Used by [`PipelineRasterizationStateRasterizationOrderAMD::extend`](struct.PipelineRasterizationStateRasterizationOrderAMD.html#method.extend)"]
pub trait ExtendableByPipelineRasterizationStateRasterizationOrderAMD {}
impl ExtendableByPipelineRasterizationStateRasterizationOrderAMD
    for crate::vk1_0::PipelineRasterizationStateCreateInfo
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PipelineRasterizationStateRasterizationOrderAMD`](struct.PipelineRasterizationStateRasterizationOrderAMD.html)"]
#[repr(transparent)]
pub struct PipelineRasterizationStateRasterizationOrderAMDBuilder<'a>(
    PipelineRasterizationStateRasterizationOrderAMD,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PipelineRasterizationStateRasterizationOrderAMDBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineRasterizationStateRasterizationOrderAMDBuilder<'a> {
        PipelineRasterizationStateRasterizationOrderAMDBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn rasterization_order(
        mut self,
        rasterization_order: crate::extensions::amd_rasterization_order::RasterizationOrderAMD,
    ) -> Self {
        self.0.rasterization_order = rasterization_order;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PipelineRasterizationStateRasterizationOrderAMD {
        self.0
    }
}
impl<'a> std::fmt::Debug for PipelineRasterizationStateRasterizationOrderAMDBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
