# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_EXT_pipeline_creation_cache_control.html)\n\n## Extends\n- [`PipelineCacheCreateFlagBits`](../../vk1_0/struct.PipelineCacheCreateFlagBits.html)\n- [`PipelineCreateFlagBits`](../../vk1_0/struct.PipelineCreateFlagBits.html)\n- [`Result`](../../vk1_0/struct.Result.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_PIPELINE_CREATION_CACHE_CONTROL_SPEC_VERSION: u32 = 3;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_PIPELINE_CREATION_CACHE_CONTROL_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_pipeline_creation_cache_control");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevicePipelineCreationCacheControlFeaturesEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDevicePipelineCreationCacheControlFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub pipeline_creation_cache_control: crate::vk1_0::Bool32,
}
impl PhysicalDevicePipelineCreationCacheControlFeaturesEXT {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDevicePipelineCreationCacheControlFeaturesEXT,
    {
        unsafe {
            crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
        }
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDevicePipelineCreationCacheControlFeaturesEXTBuilder<'a> {
        PhysicalDevicePipelineCreationCacheControlFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDevicePipelineCreationCacheControlFeaturesEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDevicePipelineCreationCacheControlFeaturesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "pipeline_creation_cache_control",
                &(self.pipeline_creation_cache_control != 0),
            )
            .finish()
    }
}
impl Default for PhysicalDevicePipelineCreationCacheControlFeaturesEXT {
    fn default() -> PhysicalDevicePipelineCreationCacheControlFeaturesEXT {
        PhysicalDevicePipelineCreationCacheControlFeaturesEXT { s_type : crate :: vk1_0 :: StructureType :: PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES_EXT , p_next : std :: ptr :: null_mut ( ) , pipeline_creation_cache_control : Default :: default ( ) , }
    }
}
#[doc = "Used by [`PhysicalDevicePipelineCreationCacheControlFeaturesEXT::extend`](struct.PhysicalDevicePipelineCreationCacheControlFeaturesEXT.html#method.extend)"]
pub trait ExtendableByPhysicalDevicePipelineCreationCacheControlFeaturesEXT {}
impl ExtendableByPhysicalDevicePipelineCreationCacheControlFeaturesEXT
    for crate::vk1_1::PhysicalDeviceFeatures2
{
}
impl ExtendableByPhysicalDevicePipelineCreationCacheControlFeaturesEXT
    for crate::vk1_0::DeviceCreateInfo
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDevicePipelineCreationCacheControlFeaturesEXT`](struct.PhysicalDevicePipelineCreationCacheControlFeaturesEXT.html)"]
#[repr(transparent)]
pub struct PhysicalDevicePipelineCreationCacheControlFeaturesEXTBuilder<'a>(
    PhysicalDevicePipelineCreationCacheControlFeaturesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDevicePipelineCreationCacheControlFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDevicePipelineCreationCacheControlFeaturesEXTBuilder<'a> {
        PhysicalDevicePipelineCreationCacheControlFeaturesEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn pipeline_creation_cache_control(
        mut self,
        pipeline_creation_cache_control: bool,
    ) -> Self {
        self.0.pipeline_creation_cache_control = pipeline_creation_cache_control as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDevicePipelineCreationCacheControlFeaturesEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDevicePipelineCreationCacheControlFeaturesEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
