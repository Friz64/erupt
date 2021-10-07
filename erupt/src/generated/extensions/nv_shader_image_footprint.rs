#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_SHADER_IMAGE_FOOTPRINT_SPEC_VERSION")]
pub const NV_SHADER_IMAGE_FOOTPRINT_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_SHADER_IMAGE_FOOTPRINT_EXTENSION_NAME")]
pub const NV_SHADER_IMAGE_FOOTPRINT_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_NV_shader_image_footprint");
#[doc = "Provided by [`crate::extensions::nv_shader_image_footprint`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_SHADER_IMAGE_FOOTPRINT_FEATURES_NV: Self = Self(1000204000);
}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceShaderImageFootprintFeaturesNV> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceShaderImageFootprintFeaturesNVBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceShaderImageFootprintFeaturesNV> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceShaderImageFootprintFeaturesNVBuilder<'_>> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderImageFootprintFeaturesNV.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceShaderImageFootprintFeaturesNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceShaderImageFootprintFeaturesNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub image_footprint: crate::vk1_0::Bool32,
}
impl PhysicalDeviceShaderImageFootprintFeaturesNV {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_SHADER_IMAGE_FOOTPRINT_FEATURES_NV;
}
impl Default for PhysicalDeviceShaderImageFootprintFeaturesNV {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), image_footprint: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceShaderImageFootprintFeaturesNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceShaderImageFootprintFeaturesNV").field("s_type", &self.s_type).field("p_next", &self.p_next).field("image_footprint", &(self.image_footprint != 0)).finish()
    }
}
impl PhysicalDeviceShaderImageFootprintFeaturesNV {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceShaderImageFootprintFeaturesNVBuilder<'a> {
        PhysicalDeviceShaderImageFootprintFeaturesNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderImageFootprintFeaturesNV.html) · Builder of [`PhysicalDeviceShaderImageFootprintFeaturesNV`]"]
#[repr(transparent)]
pub struct PhysicalDeviceShaderImageFootprintFeaturesNVBuilder<'a>(PhysicalDeviceShaderImageFootprintFeaturesNV, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceShaderImageFootprintFeaturesNVBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceShaderImageFootprintFeaturesNVBuilder<'a> {
        PhysicalDeviceShaderImageFootprintFeaturesNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn image_footprint(mut self, image_footprint: bool) -> Self {
        self.0.image_footprint = image_footprint as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceShaderImageFootprintFeaturesNV {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceShaderImageFootprintFeaturesNVBuilder<'a> {
    fn default() -> PhysicalDeviceShaderImageFootprintFeaturesNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceShaderImageFootprintFeaturesNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceShaderImageFootprintFeaturesNVBuilder<'a> {
    type Target = PhysicalDeviceShaderImageFootprintFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceShaderImageFootprintFeaturesNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
