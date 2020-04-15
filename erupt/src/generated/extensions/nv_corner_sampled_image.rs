# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_NV_corner_sampled_image.html)\n\n## Extends\n- [`ImageCreateFlagBits`](../../vk1_0/struct.ImageCreateFlagBits.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NV_CORNER_SAMPLED_IMAGE_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NV_CORNER_SAMPLED_IMAGE_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_NV_corner_sampled_image");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceCornerSampledImageFeaturesNV.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceCornerSampledImageFeaturesNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub corner_sampled_image: crate::vk1_0::Bool32,
}
impl PhysicalDeviceCornerSampledImageFeaturesNV {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceCornerSampledImageFeaturesNV,
    {
        unsafe {
            crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
        }
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceCornerSampledImageFeaturesNVBuilder<'a> {
        PhysicalDeviceCornerSampledImageFeaturesNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceCornerSampledImageFeaturesNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceCornerSampledImageFeaturesNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("corner_sampled_image", &(self.corner_sampled_image != 0))
            .finish()
    }
}
impl Default for PhysicalDeviceCornerSampledImageFeaturesNV {
    fn default() -> PhysicalDeviceCornerSampledImageFeaturesNV {
        PhysicalDeviceCornerSampledImageFeaturesNV {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_CORNER_SAMPLED_IMAGE_FEATURES_NV,
            p_next: std::ptr::null_mut(),
            corner_sampled_image: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceCornerSampledImageFeaturesNV::extend`](struct.PhysicalDeviceCornerSampledImageFeaturesNV.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceCornerSampledImageFeaturesNV {}
impl ExtendableByPhysicalDeviceCornerSampledImageFeaturesNV
    for crate::vk1_1::PhysicalDeviceFeatures2
{
}
impl ExtendableByPhysicalDeviceCornerSampledImageFeaturesNV for crate::vk1_0::DeviceCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceCornerSampledImageFeaturesNV`](struct.PhysicalDeviceCornerSampledImageFeaturesNV.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceCornerSampledImageFeaturesNVBuilder<'a>(
    PhysicalDeviceCornerSampledImageFeaturesNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceCornerSampledImageFeaturesNVBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceCornerSampledImageFeaturesNVBuilder<'a> {
        PhysicalDeviceCornerSampledImageFeaturesNVBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn corner_sampled_image(mut self, corner_sampled_image: bool) -> Self {
        self.0.corner_sampled_image = corner_sampled_image as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceCornerSampledImageFeaturesNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceCornerSampledImageFeaturesNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
