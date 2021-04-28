#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_YCBCR_IMAGE_ARRAYS_SPEC_VERSION")]
pub const EXT_YCBCR_IMAGE_ARRAYS_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_YCBCR_IMAGE_ARRAYS_EXTENSION_NAME")]
pub const EXT_YCBCR_IMAGE_ARRAYS_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_ycbcr_image_arrays");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceYcbcrImageArraysFeaturesEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceYcbcrImageArraysFeaturesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceYcbcrImageArraysFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub ycbcr_image_arrays: crate::vk1_0::Bool32,
}
impl Default for PhysicalDeviceYcbcrImageArraysFeaturesEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_YCBCR_IMAGE_ARRAYS_FEATURES_EXT, p_next: std::ptr::null_mut(), ycbcr_image_arrays: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceYcbcrImageArraysFeaturesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceYcbcrImageArraysFeaturesEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("ycbcr_image_arrays", &(self.ycbcr_image_arrays != 0)).finish()
    }
}
impl PhysicalDeviceYcbcrImageArraysFeaturesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceYcbcrImageArraysFeaturesEXTBuilder<'a> {
        PhysicalDeviceYcbcrImageArraysFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceYcbcrImageArraysFeaturesEXT.html) · Builder of [`PhysicalDeviceYcbcrImageArraysFeaturesEXT`]"]
#[repr(transparent)]
pub struct PhysicalDeviceYcbcrImageArraysFeaturesEXTBuilder<'a>(PhysicalDeviceYcbcrImageArraysFeaturesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceYcbcrImageArraysFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceYcbcrImageArraysFeaturesEXTBuilder<'a> {
        PhysicalDeviceYcbcrImageArraysFeaturesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn ycbcr_image_arrays(mut self, ycbcr_image_arrays: bool) -> Self {
        self.0.ycbcr_image_arrays = ycbcr_image_arrays as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceYcbcrImageArraysFeaturesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceYcbcrImageArraysFeaturesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceYcbcrImageArraysFeaturesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceYcbcrImageArraysFeaturesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
