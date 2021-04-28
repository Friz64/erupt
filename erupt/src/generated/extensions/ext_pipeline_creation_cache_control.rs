#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_PIPELINE_CREATION_CACHE_CONTROL_SPEC_VERSION")]
pub const EXT_PIPELINE_CREATION_CACHE_CONTROL_SPEC_VERSION: u32 = 3;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_PIPELINE_CREATION_CACHE_CONTROL_EXTENSION_NAME")]
pub const EXT_PIPELINE_CREATION_CACHE_CONTROL_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_pipeline_creation_cache_control");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevicePipelineCreationCacheControlFeaturesEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDevicePipelineCreationCacheControlFeaturesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDevicePipelineCreationCacheControlFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub pipeline_creation_cache_control: crate::vk1_0::Bool32,
}
impl Default for PhysicalDevicePipelineCreationCacheControlFeaturesEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES_EXT, p_next: std::ptr::null_mut(), pipeline_creation_cache_control: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDevicePipelineCreationCacheControlFeaturesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDevicePipelineCreationCacheControlFeaturesEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("pipeline_creation_cache_control", &(self.pipeline_creation_cache_control != 0)).finish()
    }
}
impl PhysicalDevicePipelineCreationCacheControlFeaturesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDevicePipelineCreationCacheControlFeaturesEXTBuilder<'a> {
        PhysicalDevicePipelineCreationCacheControlFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevicePipelineCreationCacheControlFeaturesEXT.html) · Builder of [`PhysicalDevicePipelineCreationCacheControlFeaturesEXT`]"]
#[repr(transparent)]
pub struct PhysicalDevicePipelineCreationCacheControlFeaturesEXTBuilder<'a>(PhysicalDevicePipelineCreationCacheControlFeaturesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDevicePipelineCreationCacheControlFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDevicePipelineCreationCacheControlFeaturesEXTBuilder<'a> {
        PhysicalDevicePipelineCreationCacheControlFeaturesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn pipeline_creation_cache_control(mut self, pipeline_creation_cache_control: bool) -> Self {
        self.0.pipeline_creation_cache_control = pipeline_creation_cache_control as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDevicePipelineCreationCacheControlFeaturesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDevicePipelineCreationCacheControlFeaturesEXTBuilder<'a> {
    fn default() -> PhysicalDevicePipelineCreationCacheControlFeaturesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDevicePipelineCreationCacheControlFeaturesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDevicePipelineCreationCacheControlFeaturesEXTBuilder<'a> {
    type Target = PhysicalDevicePipelineCreationCacheControlFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDevicePipelineCreationCacheControlFeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
