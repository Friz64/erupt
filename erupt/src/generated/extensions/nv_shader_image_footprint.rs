# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_NV_shader_image_footprint.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NV_SHADER_IMAGE_FOOTPRINT_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NV_SHADER_IMAGE_FOOTPRINT_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_NV_shader_image_footprint");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderImageFootprintFeaturesNV.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceShaderImageFootprintFeaturesNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub image_footprint: crate::vk1_0::Bool32,
}
impl PhysicalDeviceShaderImageFootprintFeaturesNV {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceShaderImageFootprintFeaturesNV,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceShaderImageFootprintFeaturesNVBuilder<'a> {
        PhysicalDeviceShaderImageFootprintFeaturesNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceShaderImageFootprintFeaturesNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceShaderImageFootprintFeaturesNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("image_footprint", &(self.image_footprint != 0))
            .finish()
    }
}
impl Default for PhysicalDeviceShaderImageFootprintFeaturesNV {
    fn default() -> PhysicalDeviceShaderImageFootprintFeaturesNV {
        PhysicalDeviceShaderImageFootprintFeaturesNV {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_SHADER_IMAGE_FOOTPRINT_FEATURES_NV,
            p_next: std::ptr::null_mut(),
            image_footprint: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceShaderImageFootprintFeaturesNV::extend`](struct.PhysicalDeviceShaderImageFootprintFeaturesNV.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceShaderImageFootprintFeaturesNV {}
impl ExtendableByPhysicalDeviceShaderImageFootprintFeaturesNV
    for crate::vk1_1::PhysicalDeviceFeatures2
{
}
impl ExtendableByPhysicalDeviceShaderImageFootprintFeaturesNV for crate::vk1_0::DeviceCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceShaderImageFootprintFeaturesNV`](struct.PhysicalDeviceShaderImageFootprintFeaturesNV.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceShaderImageFootprintFeaturesNVBuilder<'a>(
    PhysicalDeviceShaderImageFootprintFeaturesNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceShaderImageFootprintFeaturesNVBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceShaderImageFootprintFeaturesNVBuilder<'a> {
        PhysicalDeviceShaderImageFootprintFeaturesNVBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn image_footprint(mut self, image_footprint: bool) -> Self {
        self.0.image_footprint = image_footprint as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceShaderImageFootprintFeaturesNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceShaderImageFootprintFeaturesNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
