# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_EXT_filter_cubic.html)\n\n## Extends\n- [`Filter`](../../vk1_0/struct.Filter.html)\n- [`FormatFeatureFlagBits`](../../vk1_0/struct.FormatFeatureFlagBits.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_FILTER_CUBIC_SPEC_VERSION: u32 = 3;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_FILTER_CUBIC_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_filter_cubic");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceImageViewImageFormatInfoEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceImageViewImageFormatInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub image_view_type: crate::vk1_0::ImageViewType,
}
impl PhysicalDeviceImageViewImageFormatInfoEXT {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceImageViewImageFormatInfoEXT,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceImageViewImageFormatInfoEXTBuilder<'a> {
        PhysicalDeviceImageViewImageFormatInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceImageViewImageFormatInfoEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceImageViewImageFormatInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("image_view_type", &self.image_view_type)
            .finish()
    }
}
impl Default for PhysicalDeviceImageViewImageFormatInfoEXT {
    fn default() -> PhysicalDeviceImageViewImageFormatInfoEXT {
        PhysicalDeviceImageViewImageFormatInfoEXT {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_IMAGE_VIEW_IMAGE_FORMAT_INFO_EXT,
            p_next: std::ptr::null_mut(),
            image_view_type: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceImageViewImageFormatInfoEXT::extend`](struct.PhysicalDeviceImageViewImageFormatInfoEXT.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceImageViewImageFormatInfoEXT {}
impl ExtendableByPhysicalDeviceImageViewImageFormatInfoEXT
    for crate::vk1_1::PhysicalDeviceImageFormatInfo2
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceImageViewImageFormatInfoEXT`](struct.PhysicalDeviceImageViewImageFormatInfoEXT.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceImageViewImageFormatInfoEXTBuilder<'a>(
    PhysicalDeviceImageViewImageFormatInfoEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceImageViewImageFormatInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceImageViewImageFormatInfoEXTBuilder<'a> {
        PhysicalDeviceImageViewImageFormatInfoEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn image_view_type(mut self, image_view_type: crate::vk1_0::ImageViewType) -> Self {
        self.0.image_view_type = image_view_type;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceImageViewImageFormatInfoEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceImageViewImageFormatInfoEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceImageViewImageFormatInfoEXTBuilder<'a> {
    type Target = PhysicalDeviceImageViewImageFormatInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceImageViewImageFormatInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFilterCubicImageViewImageFormatPropertiesEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FilterCubicImageViewImageFormatPropertiesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub filter_cubic: crate::vk1_0::Bool32,
    pub filter_cubic_minmax: crate::vk1_0::Bool32,
}
impl FilterCubicImageViewImageFormatPropertiesEXT {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByFilterCubicImageViewImageFormatPropertiesEXT,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> FilterCubicImageViewImageFormatPropertiesEXTBuilder<'a> {
        FilterCubicImageViewImageFormatPropertiesEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for FilterCubicImageViewImageFormatPropertiesEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("FilterCubicImageViewImageFormatPropertiesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("filter_cubic", &(self.filter_cubic != 0))
            .field("filter_cubic_minmax", &(self.filter_cubic_minmax != 0))
            .finish()
    }
}
impl Default for FilterCubicImageViewImageFormatPropertiesEXT {
    fn default() -> FilterCubicImageViewImageFormatPropertiesEXT {
        FilterCubicImageViewImageFormatPropertiesEXT {
            s_type:
                crate::vk1_0::StructureType::FILTER_CUBIC_IMAGE_VIEW_IMAGE_FORMAT_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            filter_cubic: Default::default(),
            filter_cubic_minmax: Default::default(),
        }
    }
}
#[doc = "Used by [`FilterCubicImageViewImageFormatPropertiesEXT::extend`](struct.FilterCubicImageViewImageFormatPropertiesEXT.html#method.extend)"]
pub trait ExtendableByFilterCubicImageViewImageFormatPropertiesEXT {}
impl ExtendableByFilterCubicImageViewImageFormatPropertiesEXT
    for crate::vk1_1::ImageFormatProperties2
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`FilterCubicImageViewImageFormatPropertiesEXT`](struct.FilterCubicImageViewImageFormatPropertiesEXT.html)"]
#[repr(transparent)]
pub struct FilterCubicImageViewImageFormatPropertiesEXTBuilder<'a>(
    FilterCubicImageViewImageFormatPropertiesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> FilterCubicImageViewImageFormatPropertiesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> FilterCubicImageViewImageFormatPropertiesEXTBuilder<'a> {
        FilterCubicImageViewImageFormatPropertiesEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn filter_cubic(mut self, filter_cubic: bool) -> Self {
        self.0.filter_cubic = filter_cubic as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn filter_cubic_minmax(mut self, filter_cubic_minmax: bool) -> Self {
        self.0.filter_cubic_minmax = filter_cubic_minmax as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> FilterCubicImageViewImageFormatPropertiesEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for FilterCubicImageViewImageFormatPropertiesEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for FilterCubicImageViewImageFormatPropertiesEXTBuilder<'a> {
    type Target = FilterCubicImageViewImageFormatPropertiesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for FilterCubicImageViewImageFormatPropertiesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
