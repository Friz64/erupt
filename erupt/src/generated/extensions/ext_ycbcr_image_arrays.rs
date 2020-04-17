# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_EXT_ycbcr_image_arrays.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_YCBCR_IMAGE_ARRAYS_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_YCBCR_IMAGE_ARRAYS_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_ycbcr_image_arrays");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceYcbcrImageArraysFeaturesEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceYcbcrImageArraysFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub ycbcr_image_arrays: crate::vk1_0::Bool32,
}
impl PhysicalDeviceYcbcrImageArraysFeaturesEXT {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceYcbcrImageArraysFeaturesEXT,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceYcbcrImageArraysFeaturesEXTBuilder<'a> {
        PhysicalDeviceYcbcrImageArraysFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceYcbcrImageArraysFeaturesEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceYcbcrImageArraysFeaturesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("ycbcr_image_arrays", &(self.ycbcr_image_arrays != 0))
            .finish()
    }
}
impl Default for PhysicalDeviceYcbcrImageArraysFeaturesEXT {
    fn default() -> PhysicalDeviceYcbcrImageArraysFeaturesEXT {
        PhysicalDeviceYcbcrImageArraysFeaturesEXT {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_YCBCR_IMAGE_ARRAYS_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            ycbcr_image_arrays: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceYcbcrImageArraysFeaturesEXT::extend`](struct.PhysicalDeviceYcbcrImageArraysFeaturesEXT.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceYcbcrImageArraysFeaturesEXT {}
impl ExtendableByPhysicalDeviceYcbcrImageArraysFeaturesEXT
    for crate::vk1_1::PhysicalDeviceFeatures2
{
}
impl ExtendableByPhysicalDeviceYcbcrImageArraysFeaturesEXT for crate::vk1_0::DeviceCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceYcbcrImageArraysFeaturesEXT`](struct.PhysicalDeviceYcbcrImageArraysFeaturesEXT.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceYcbcrImageArraysFeaturesEXTBuilder<'a>(
    PhysicalDeviceYcbcrImageArraysFeaturesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceYcbcrImageArraysFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceYcbcrImageArraysFeaturesEXTBuilder<'a> {
        PhysicalDeviceYcbcrImageArraysFeaturesEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn ycbcr_image_arrays(mut self, ycbcr_image_arrays: bool) -> Self {
        self.0.ycbcr_image_arrays = ycbcr_image_arrays as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceYcbcrImageArraysFeaturesEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceYcbcrImageArraysFeaturesEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceYcbcrImageArraysFeaturesEXTBuilder<'a> {
    type Target = PhysicalDeviceYcbcrImageArraysFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceYcbcrImageArraysFeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
