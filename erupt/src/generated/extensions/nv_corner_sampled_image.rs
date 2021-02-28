#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_CORNER_SAMPLED_IMAGE_SPEC_VERSION")]
pub const NV_CORNER_SAMPLED_IMAGE_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_CORNER_SAMPLED_IMAGE_EXTENSION_NAME")]
pub const NV_CORNER_SAMPLED_IMAGE_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_NV_corner_sampled_image");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceCornerSampledImageFeaturesNV.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceCornerSampledImageFeaturesNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceCornerSampledImageFeaturesNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub corner_sampled_image: crate::vk1_0::Bool32,
}
impl Default for PhysicalDeviceCornerSampledImageFeaturesNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_CORNER_SAMPLED_IMAGE_FEATURES_NV,
            p_next: std::ptr::null_mut(),
            corner_sampled_image: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceCornerSampledImageFeaturesNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceCornerSampledImageFeaturesNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("corner_sampled_image", &(self.corner_sampled_image != 0))
            .finish()
    }
}
impl PhysicalDeviceCornerSampledImageFeaturesNV {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceCornerSampledImageFeaturesNVBuilder<'a> {
        PhysicalDeviceCornerSampledImageFeaturesNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceCornerSampledImageFeaturesNV.html) · Builder of [`PhysicalDeviceCornerSampledImageFeaturesNV`]"]
#[repr(transparent)]
pub struct PhysicalDeviceCornerSampledImageFeaturesNVBuilder<'a>(PhysicalDeviceCornerSampledImageFeaturesNV, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceCornerSampledImageFeaturesNVBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceCornerSampledImageFeaturesNVBuilder<'a> {
        PhysicalDeviceCornerSampledImageFeaturesNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn corner_sampled_image(mut self, corner_sampled_image: bool) -> Self {
        self.0.corner_sampled_image = corner_sampled_image as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceCornerSampledImageFeaturesNV {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceCornerSampledImageFeaturesNVBuilder<'a> {
    fn default() -> PhysicalDeviceCornerSampledImageFeaturesNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceCornerSampledImageFeaturesNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceCornerSampledImageFeaturesNVBuilder<'a> {
    type Target = PhysicalDeviceCornerSampledImageFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceCornerSampledImageFeaturesNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
