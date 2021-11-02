#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_INCREMENTAL_PRESENT_SPEC_VERSION")]
pub const KHR_INCREMENTAL_PRESENT_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_INCREMENTAL_PRESENT_EXTENSION_NAME")]
pub const KHR_INCREMENTAL_PRESENT_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_incremental_present");
#[doc = "Provided by [`crate::extensions::khr_incremental_present`]"]
impl crate::vk1_0::StructureType {
    pub const PRESENT_REGIONS_KHR: Self = Self(1000084000);
}
impl<'a> crate::ExtendableFrom<'a, PresentRegionsKHR> for crate::extensions::khr_swapchain::PresentInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PresentRegionsKHRBuilder<'_>> for crate::extensions::khr_swapchain::PresentInfoKHRBuilder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPresentRegionsKHR.html) · Structure"]
#[doc(alias = "VkPresentRegionsKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PresentRegionsKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub swapchain_count: u32,
    pub p_regions: *const crate::extensions::khr_incremental_present::PresentRegionKHR,
}
impl PresentRegionsKHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PRESENT_REGIONS_KHR;
}
impl Default for PresentRegionsKHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), swapchain_count: Default::default(), p_regions: std::ptr::null() }
    }
}
impl std::fmt::Debug for PresentRegionsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PresentRegionsKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("swapchain_count", &self.swapchain_count).field("p_regions", &self.p_regions).finish()
    }
}
impl PresentRegionsKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> PresentRegionsKHRBuilder<'a> {
        PresentRegionsKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPresentRegionsKHR.html) · Builder of [`PresentRegionsKHR`]"]
#[repr(transparent)]
pub struct PresentRegionsKHRBuilder<'a>(PresentRegionsKHR, std::marker::PhantomData<&'a ()>);
impl<'a> PresentRegionsKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PresentRegionsKHRBuilder<'a> {
        PresentRegionsKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn regions(mut self, regions: &'a [crate::extensions::khr_incremental_present::PresentRegionKHRBuilder]) -> Self {
        self.0.p_regions = regions.as_ptr() as _;
        self.0.swapchain_count = regions.len() as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> PresentRegionsKHR {
        self.0
    }
}
impl<'a> std::default::Default for PresentRegionsKHRBuilder<'a> {
    fn default() -> PresentRegionsKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PresentRegionsKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PresentRegionsKHRBuilder<'a> {
    type Target = PresentRegionsKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PresentRegionsKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPresentRegionKHR.html) · Structure"]
#[doc(alias = "VkPresentRegionKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PresentRegionKHR {
    pub rectangle_count: u32,
    pub p_rectangles: *const crate::extensions::khr_incremental_present::RectLayerKHR,
}
impl Default for PresentRegionKHR {
    fn default() -> Self {
        Self { rectangle_count: Default::default(), p_rectangles: std::ptr::null() }
    }
}
impl std::fmt::Debug for PresentRegionKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PresentRegionKHR").field("rectangle_count", &self.rectangle_count).field("p_rectangles", &self.p_rectangles).finish()
    }
}
impl PresentRegionKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> PresentRegionKHRBuilder<'a> {
        PresentRegionKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPresentRegionKHR.html) · Builder of [`PresentRegionKHR`]"]
#[repr(transparent)]
pub struct PresentRegionKHRBuilder<'a>(PresentRegionKHR, std::marker::PhantomData<&'a ()>);
impl<'a> PresentRegionKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PresentRegionKHRBuilder<'a> {
        PresentRegionKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn rectangles(mut self, rectangles: &'a [crate::extensions::khr_incremental_present::RectLayerKHRBuilder]) -> Self {
        self.0.p_rectangles = rectangles.as_ptr() as _;
        self.0.rectangle_count = rectangles.len() as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> PresentRegionKHR {
        self.0
    }
}
impl<'a> std::default::Default for PresentRegionKHRBuilder<'a> {
    fn default() -> PresentRegionKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PresentRegionKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PresentRegionKHRBuilder<'a> {
    type Target = PresentRegionKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PresentRegionKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRectLayerKHR.html) · Structure"]
#[doc(alias = "VkRectLayerKHR")]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct RectLayerKHR {
    pub offset: crate::vk1_0::Offset2D,
    pub extent: crate::vk1_0::Extent2D,
    pub layer: u32,
}
impl Default for RectLayerKHR {
    fn default() -> Self {
        Self { offset: Default::default(), extent: Default::default(), layer: Default::default() }
    }
}
impl std::fmt::Debug for RectLayerKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RectLayerKHR").field("offset", &self.offset).field("extent", &self.extent).field("layer", &self.layer).finish()
    }
}
impl RectLayerKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> RectLayerKHRBuilder<'a> {
        RectLayerKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRectLayerKHR.html) · Builder of [`RectLayerKHR`]"]
#[repr(transparent)]
pub struct RectLayerKHRBuilder<'a>(RectLayerKHR, std::marker::PhantomData<&'a ()>);
impl<'a> RectLayerKHRBuilder<'a> {
    #[inline]
    pub fn new() -> RectLayerKHRBuilder<'a> {
        RectLayerKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn offset(mut self, offset: crate::vk1_0::Offset2D) -> Self {
        self.0.offset = offset as _;
        self
    }
    #[inline]
    pub fn extent(mut self, extent: crate::vk1_0::Extent2D) -> Self {
        self.0.extent = extent as _;
        self
    }
    #[inline]
    pub fn layer(mut self, layer: u32) -> Self {
        self.0.layer = layer as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> RectLayerKHR {
        self.0
    }
}
impl<'a> std::default::Default for RectLayerKHRBuilder<'a> {
    fn default() -> RectLayerKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for RectLayerKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for RectLayerKHRBuilder<'a> {
    type Target = RectLayerKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for RectLayerKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
