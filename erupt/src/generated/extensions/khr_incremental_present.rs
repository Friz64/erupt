# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_KHR_incremental_present.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_INCREMENTAL_PRESENT_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_INCREMENTAL_PRESENT_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_incremental_present");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPresentRegionsKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PresentRegionsKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub swapchain_count: u32,
    pub p_regions: *const crate::extensions::khr_incremental_present::PresentRegionKHR,
}
impl PresentRegionsKHR {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: crate::ExtendableBy<Self>,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PresentRegionsKHRBuilder<'a> {
        PresentRegionsKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PresentRegionsKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PresentRegionsKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("swapchain_count", &self.swapchain_count)
            .field("p_regions", &self.p_regions)
            .finish()
    }
}
impl Default for PresentRegionsKHR {
    fn default() -> PresentRegionsKHR {
        PresentRegionsKHR {
            s_type: crate::vk1_0::StructureType::PRESENT_REGIONS_KHR,
            p_next: std::ptr::null(),
            swapchain_count: Default::default(),
            p_regions: std::ptr::null(),
        }
    }
}
impl crate::ExtendableBy<PresentRegionsKHR> for crate::extensions::khr_swapchain::PresentInfoKHR {}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPresentRegionsKHR.html) · Builder of [`PresentRegionsKHR`](struct.PresentRegionsKHR.html)"]
#[repr(transparent)]
pub struct PresentRegionsKHRBuilder<'a>(PresentRegionsKHR, std::marker::PhantomData<&'a ()>);
impl<'a> PresentRegionsKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PresentRegionsKHRBuilder<'a> {
        PresentRegionsKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn regions(
        mut self,
        regions: &'a [crate::extensions::khr_incremental_present::PresentRegionKHRBuilder],
    ) -> Self {
        self.0.swapchain_count = regions.len() as _;
        self.0.p_regions = regions.as_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PresentRegionsKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for PresentRegionsKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PresentRegionKHR {
    pub rectangle_count: u32,
    pub p_rectangles: *const crate::extensions::khr_incremental_present::RectLayerKHR,
}
impl PresentRegionKHR {
    #[inline]
    pub fn builder<'a>(self) -> PresentRegionKHRBuilder<'a> {
        PresentRegionKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PresentRegionKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PresentRegionKHR")
            .field("rectangle_count", &self.rectangle_count)
            .field("p_rectangles", &self.p_rectangles)
            .finish()
    }
}
impl Default for PresentRegionKHR {
    fn default() -> PresentRegionKHR {
        PresentRegionKHR {
            rectangle_count: Default::default(),
            p_rectangles: std::ptr::null(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPresentRegionKHR.html) · Builder of [`PresentRegionKHR`](struct.PresentRegionKHR.html)"]
#[repr(transparent)]
pub struct PresentRegionKHRBuilder<'a>(PresentRegionKHR, std::marker::PhantomData<&'a ()>);
impl<'a> PresentRegionKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PresentRegionKHRBuilder<'a> {
        PresentRegionKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn rectangles(
        mut self,
        rectangles: &'a [crate::extensions::khr_incremental_present::RectLayerKHRBuilder],
    ) -> Self {
        self.0.rectangle_count = rectangles.len() as _;
        self.0.p_rectangles = rectangles.as_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PresentRegionKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for PresentRegionKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RectLayerKHR {
    pub offset: crate::vk1_0::Offset2D,
    pub extent: crate::vk1_0::Extent2D,
    pub layer: u32,
}
impl RectLayerKHR {
    #[inline]
    pub fn builder<'a>(self) -> RectLayerKHRBuilder<'a> {
        RectLayerKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for RectLayerKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("RectLayerKHR")
            .field("offset", &self.offset)
            .field("extent", &self.extent)
            .field("layer", &self.layer)
            .finish()
    }
}
impl Default for RectLayerKHR {
    fn default() -> RectLayerKHR {
        RectLayerKHR {
            offset: Default::default(),
            extent: Default::default(),
            layer: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRectLayerKHR.html) · Builder of [`RectLayerKHR`](struct.RectLayerKHR.html)"]
#[repr(transparent)]
pub struct RectLayerKHRBuilder<'a>(RectLayerKHR, std::marker::PhantomData<&'a ()>);
impl<'a> RectLayerKHRBuilder<'a> {
    #[inline]
    pub fn new() -> RectLayerKHRBuilder<'a> {
        RectLayerKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn offset(mut self, offset: crate::vk1_0::Offset2D) -> Self {
        self.0.offset = offset;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn extent(mut self, extent: crate::vk1_0::Extent2D) -> Self {
        self.0.extent = extent;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn layer(mut self, layer: u32) -> Self {
        self.0.layer = layer;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> RectLayerKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for RectLayerKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
