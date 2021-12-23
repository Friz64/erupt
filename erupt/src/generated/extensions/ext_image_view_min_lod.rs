#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_IMAGE_VIEW_MIN_LOD_SPEC_VERSION")]
pub const EXT_IMAGE_VIEW_MIN_LOD_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_IMAGE_VIEW_MIN_LOD_EXTENSION_NAME")]
pub const EXT_IMAGE_VIEW_MIN_LOD_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_image_view_min_lod");
#[doc = "Provided by [`crate::extensions::ext_image_view_min_lod`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_IMAGE_VIEW_MIN_LOD_FEATURES_EXT: Self = Self(1000391000);
    pub const IMAGE_VIEW_MIN_LOD_CREATE_INFO_EXT: Self = Self(1000391001);
}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceImageViewMinLodFeaturesEXT> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceImageViewMinLodFeaturesEXTBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, ImageViewMinLodCreateInfoEXT> for crate::vk1_0::ImageViewCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, ImageViewMinLodCreateInfoEXTBuilder<'_>> for crate::vk1_0::ImageViewCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceImageViewMinLodFeaturesEXT> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceImageViewMinLodFeaturesEXTBuilder<'_>> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceImageViewMinLodFeaturesEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceImageViewMinLodFeaturesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceImageViewMinLodFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub min_lod: crate::vk1_0::Bool32,
}
impl PhysicalDeviceImageViewMinLodFeaturesEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_IMAGE_VIEW_MIN_LOD_FEATURES_EXT;
}
impl Default for PhysicalDeviceImageViewMinLodFeaturesEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), min_lod: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceImageViewMinLodFeaturesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceImageViewMinLodFeaturesEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("min_lod", &(self.min_lod != 0)).finish()
    }
}
impl PhysicalDeviceImageViewMinLodFeaturesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceImageViewMinLodFeaturesEXTBuilder<'a> {
        PhysicalDeviceImageViewMinLodFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceImageViewMinLodFeaturesEXT.html) · Builder of [`PhysicalDeviceImageViewMinLodFeaturesEXT`]"]
#[repr(transparent)]
pub struct PhysicalDeviceImageViewMinLodFeaturesEXTBuilder<'a>(PhysicalDeviceImageViewMinLodFeaturesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceImageViewMinLodFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceImageViewMinLodFeaturesEXTBuilder<'a> {
        PhysicalDeviceImageViewMinLodFeaturesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn min_lod(mut self, min_lod: bool) -> Self {
        self.0.min_lod = min_lod as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> PhysicalDeviceImageViewMinLodFeaturesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceImageViewMinLodFeaturesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceImageViewMinLodFeaturesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceImageViewMinLodFeaturesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceImageViewMinLodFeaturesEXTBuilder<'a> {
    type Target = PhysicalDeviceImageViewMinLodFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceImageViewMinLodFeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageViewMinLodCreateInfoEXT.html) · Structure"]
#[doc(alias = "VkImageViewMinLodCreateInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImageViewMinLodCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub min_lod: std::os::raw::c_float,
}
impl ImageViewMinLodCreateInfoEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::IMAGE_VIEW_MIN_LOD_CREATE_INFO_EXT;
}
impl Default for ImageViewMinLodCreateInfoEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), min_lod: Default::default() }
    }
}
impl std::fmt::Debug for ImageViewMinLodCreateInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ImageViewMinLodCreateInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("min_lod", &self.min_lod).finish()
    }
}
impl ImageViewMinLodCreateInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> ImageViewMinLodCreateInfoEXTBuilder<'a> {
        ImageViewMinLodCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageViewMinLodCreateInfoEXT.html) · Builder of [`ImageViewMinLodCreateInfoEXT`]"]
#[repr(transparent)]
pub struct ImageViewMinLodCreateInfoEXTBuilder<'a>(ImageViewMinLodCreateInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> ImageViewMinLodCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> ImageViewMinLodCreateInfoEXTBuilder<'a> {
        ImageViewMinLodCreateInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn min_lod(mut self, min_lod: std::os::raw::c_float) -> Self {
        self.0.min_lod = min_lod as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> ImageViewMinLodCreateInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for ImageViewMinLodCreateInfoEXTBuilder<'a> {
    fn default() -> ImageViewMinLodCreateInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ImageViewMinLodCreateInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ImageViewMinLodCreateInfoEXTBuilder<'a> {
    type Target = ImageViewMinLodCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ImageViewMinLodCreateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
