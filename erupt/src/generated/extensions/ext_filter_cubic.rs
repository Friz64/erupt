#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_FILTER_CUBIC_SPEC_VERSION")]
pub const EXT_FILTER_CUBIC_SPEC_VERSION: u32 = 3;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_FILTER_CUBIC_EXTENSION_NAME")]
pub const EXT_FILTER_CUBIC_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_filter_cubic");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceImageViewImageFormatInfoEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceImageViewImageFormatInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceImageViewImageFormatInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub image_view_type: crate::vk1_0::ImageViewType,
}
impl Default for PhysicalDeviceImageViewImageFormatInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_IMAGE_VIEW_IMAGE_FORMAT_INFO_EXT,
            p_next: std::ptr::null_mut(),
            image_view_type: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceImageViewImageFormatInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceImageViewImageFormatInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("image_view_type", &self.image_view_type)
            .finish()
    }
}
impl PhysicalDeviceImageViewImageFormatInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceImageViewImageFormatInfoEXTBuilder<'a> {
        PhysicalDeviceImageViewImageFormatInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceImageViewImageFormatInfoEXT.html) · Builder of [`PhysicalDeviceImageViewImageFormatInfoEXT`]"]
#[repr(transparent)]
pub struct PhysicalDeviceImageViewImageFormatInfoEXTBuilder<'a>(PhysicalDeviceImageViewImageFormatInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceImageViewImageFormatInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceImageViewImageFormatInfoEXTBuilder<'a> {
        PhysicalDeviceImageViewImageFormatInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn image_view_type(mut self, image_view_type: crate::vk1_0::ImageViewType) -> Self {
        self.0.image_view_type = image_view_type as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceImageViewImageFormatInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceImageViewImageFormatInfoEXTBuilder<'a> {
    fn default() -> PhysicalDeviceImageViewImageFormatInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceImageViewImageFormatInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
#[doc(alias = "VkFilterCubicImageViewImageFormatPropertiesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FilterCubicImageViewImageFormatPropertiesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub filter_cubic: crate::vk1_0::Bool32,
    pub filter_cubic_minmax: crate::vk1_0::Bool32,
}
impl Default for FilterCubicImageViewImageFormatPropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::FILTER_CUBIC_IMAGE_VIEW_IMAGE_FORMAT_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            filter_cubic: Default::default(),
            filter_cubic_minmax: Default::default(),
        }
    }
}
impl std::fmt::Debug for FilterCubicImageViewImageFormatPropertiesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FilterCubicImageViewImageFormatPropertiesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("filter_cubic", &(self.filter_cubic != 0))
            .field("filter_cubic_minmax", &(self.filter_cubic_minmax != 0))
            .finish()
    }
}
impl FilterCubicImageViewImageFormatPropertiesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> FilterCubicImageViewImageFormatPropertiesEXTBuilder<'a> {
        FilterCubicImageViewImageFormatPropertiesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFilterCubicImageViewImageFormatPropertiesEXT.html) · Builder of [`FilterCubicImageViewImageFormatPropertiesEXT`]"]
#[repr(transparent)]
pub struct FilterCubicImageViewImageFormatPropertiesEXTBuilder<'a>(FilterCubicImageViewImageFormatPropertiesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> FilterCubicImageViewImageFormatPropertiesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> FilterCubicImageViewImageFormatPropertiesEXTBuilder<'a> {
        FilterCubicImageViewImageFormatPropertiesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn filter_cubic(mut self, filter_cubic: bool) -> Self {
        self.0.filter_cubic = filter_cubic as _;
        self
    }
    #[inline]
    pub fn filter_cubic_minmax(mut self, filter_cubic_minmax: bool) -> Self {
        self.0.filter_cubic_minmax = filter_cubic_minmax as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> FilterCubicImageViewImageFormatPropertiesEXT {
        self.0
    }
}
impl<'a> std::default::Default for FilterCubicImageViewImageFormatPropertiesEXTBuilder<'a> {
    fn default() -> FilterCubicImageViewImageFormatPropertiesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for FilterCubicImageViewImageFormatPropertiesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
