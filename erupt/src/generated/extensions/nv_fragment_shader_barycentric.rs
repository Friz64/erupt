#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_FRAGMENT_SHADER_BARYCENTRIC_SPEC_VERSION")]
pub const NV_FRAGMENT_SHADER_BARYCENTRIC_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_FRAGMENT_SHADER_BARYCENTRIC_EXTENSION_NAME")]
pub const NV_FRAGMENT_SHADER_BARYCENTRIC_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_NV_fragment_shader_barycentric");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceFragmentShaderBarycentricFeaturesNV.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceFragmentShaderBarycentricFeaturesNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceFragmentShaderBarycentricFeaturesNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub fragment_shader_barycentric: crate::vk1_0::Bool32,
}
impl Default for PhysicalDeviceFragmentShaderBarycentricFeaturesNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_NV,
            p_next: std::ptr::null_mut(),
            fragment_shader_barycentric: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceFragmentShaderBarycentricFeaturesNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceFragmentShaderBarycentricFeaturesNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("fragment_shader_barycentric", &(self.fragment_shader_barycentric != 0))
            .finish()
    }
}
impl PhysicalDeviceFragmentShaderBarycentricFeaturesNV {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceFragmentShaderBarycentricFeaturesNVBuilder<'a> {
        PhysicalDeviceFragmentShaderBarycentricFeaturesNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceFragmentShaderBarycentricFeaturesNV.html) · Builder of [`PhysicalDeviceFragmentShaderBarycentricFeaturesNV`]"]
#[repr(transparent)]
pub struct PhysicalDeviceFragmentShaderBarycentricFeaturesNVBuilder<'a>(PhysicalDeviceFragmentShaderBarycentricFeaturesNV, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceFragmentShaderBarycentricFeaturesNVBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceFragmentShaderBarycentricFeaturesNVBuilder<'a> {
        PhysicalDeviceFragmentShaderBarycentricFeaturesNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn fragment_shader_barycentric(mut self, fragment_shader_barycentric: bool) -> Self {
        self.0.fragment_shader_barycentric = fragment_shader_barycentric as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceFragmentShaderBarycentricFeaturesNV {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceFragmentShaderBarycentricFeaturesNVBuilder<'a> {
    fn default() -> PhysicalDeviceFragmentShaderBarycentricFeaturesNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceFragmentShaderBarycentricFeaturesNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceFragmentShaderBarycentricFeaturesNVBuilder<'a> {
    type Target = PhysicalDeviceFragmentShaderBarycentricFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceFragmentShaderBarycentricFeaturesNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
