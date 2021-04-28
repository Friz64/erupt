#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_IMAGE_ROBUSTNESS_SPEC_VERSION")]
pub const EXT_IMAGE_ROBUSTNESS_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_IMAGE_ROBUSTNESS_EXTENSION_NAME")]
pub const EXT_IMAGE_ROBUSTNESS_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_image_robustness");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceImageRobustnessFeaturesEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceImageRobustnessFeaturesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceImageRobustnessFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub robust_image_access: crate::vk1_0::Bool32,
}
impl Default for PhysicalDeviceImageRobustnessFeaturesEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES_EXT, p_next: std::ptr::null_mut(), robust_image_access: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceImageRobustnessFeaturesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceImageRobustnessFeaturesEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("robust_image_access", &(self.robust_image_access != 0)).finish()
    }
}
impl PhysicalDeviceImageRobustnessFeaturesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceImageRobustnessFeaturesEXTBuilder<'a> {
        PhysicalDeviceImageRobustnessFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceImageRobustnessFeaturesEXT.html) · Builder of [`PhysicalDeviceImageRobustnessFeaturesEXT`]"]
#[repr(transparent)]
pub struct PhysicalDeviceImageRobustnessFeaturesEXTBuilder<'a>(PhysicalDeviceImageRobustnessFeaturesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceImageRobustnessFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceImageRobustnessFeaturesEXTBuilder<'a> {
        PhysicalDeviceImageRobustnessFeaturesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn robust_image_access(mut self, robust_image_access: bool) -> Self {
        self.0.robust_image_access = robust_image_access as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceImageRobustnessFeaturesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceImageRobustnessFeaturesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceImageRobustnessFeaturesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceImageRobustnessFeaturesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceImageRobustnessFeaturesEXTBuilder<'a> {
    type Target = PhysicalDeviceImageRobustnessFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceImageRobustnessFeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
