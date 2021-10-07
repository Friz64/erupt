#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_IMAGE_ROBUSTNESS_SPEC_VERSION")]
pub const EXT_IMAGE_ROBUSTNESS_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_IMAGE_ROBUSTNESS_EXTENSION_NAME")]
pub const EXT_IMAGE_ROBUSTNESS_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_image_robustness");
#[doc = "Provided by [`crate::extensions::ext_image_robustness`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES_EXT: Self = Self(1000335000);
}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceImageRobustnessFeaturesEXT> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceImageRobustnessFeaturesEXTBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceImageRobustnessFeaturesEXT> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceImageRobustnessFeaturesEXTBuilder<'_>> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceImageRobustnessFeaturesEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceImageRobustnessFeaturesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceImageRobustnessFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub robust_image_access: crate::vk1_0::Bool32,
}
impl PhysicalDeviceImageRobustnessFeaturesEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES_EXT;
}
impl Default for PhysicalDeviceImageRobustnessFeaturesEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), robust_image_access: Default::default() }
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
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
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
