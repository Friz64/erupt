#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_FRAGMENT_SHADER_INTERLOCK_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_FRAGMENT_SHADER_INTERLOCK_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_fragment_shader_interlock");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceFragmentShaderInterlockFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub fragment_shader_sample_interlock: crate::vk1_0::Bool32,
    pub fragment_shader_pixel_interlock: crate::vk1_0::Bool32,
    pub fragment_shader_shading_rate_interlock: crate::vk1_0::Bool32,
}
impl Default for PhysicalDeviceFragmentShaderInterlockFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type:
                crate::vk1_0::StructureType::PHYSICAL_DEVICE_FRAGMENT_SHADER_INTERLOCK_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            fragment_shader_sample_interlock: Default::default(),
            fragment_shader_pixel_interlock: Default::default(),
            fragment_shader_shading_rate_interlock: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceFragmentShaderInterlockFeaturesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceFragmentShaderInterlockFeaturesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "fragment_shader_sample_interlock",
                &(self.fragment_shader_sample_interlock != 0),
            )
            .field(
                "fragment_shader_pixel_interlock",
                &(self.fragment_shader_pixel_interlock != 0),
            )
            .field(
                "fragment_shader_shading_rate_interlock",
                &(self.fragment_shader_shading_rate_interlock != 0),
            )
            .finish()
    }
}
impl PhysicalDeviceFragmentShaderInterlockFeaturesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceFragmentShaderInterlockFeaturesEXTBuilder<'a> {
        PhysicalDeviceFragmentShaderInterlockFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT.html) · Builder of [`PhysicalDeviceFragmentShaderInterlockFeaturesEXT`](struct.PhysicalDeviceFragmentShaderInterlockFeaturesEXT.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceFragmentShaderInterlockFeaturesEXTBuilder<'a>(
    PhysicalDeviceFragmentShaderInterlockFeaturesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceFragmentShaderInterlockFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceFragmentShaderInterlockFeaturesEXTBuilder<'a> {
        PhysicalDeviceFragmentShaderInterlockFeaturesEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[inline]
    pub fn fragment_shader_sample_interlock(
        mut self,
        fragment_shader_sample_interlock: bool,
    ) -> Self {
        self.0.fragment_shader_sample_interlock = fragment_shader_sample_interlock as _;
        self
    }
    #[inline]
    pub fn fragment_shader_pixel_interlock(
        mut self,
        fragment_shader_pixel_interlock: bool,
    ) -> Self {
        self.0.fragment_shader_pixel_interlock = fragment_shader_pixel_interlock as _;
        self
    }
    #[inline]
    pub fn fragment_shader_shading_rate_interlock(
        mut self,
        fragment_shader_shading_rate_interlock: bool,
    ) -> Self {
        self.0.fragment_shader_shading_rate_interlock = fragment_shader_shading_rate_interlock as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceFragmentShaderInterlockFeaturesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceFragmentShaderInterlockFeaturesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceFragmentShaderInterlockFeaturesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceFragmentShaderInterlockFeaturesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceFragmentShaderInterlockFeaturesEXTBuilder<'a> {
    type Target = PhysicalDeviceFragmentShaderInterlockFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceFragmentShaderInterlockFeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
