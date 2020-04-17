# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_EXT_fragment_shader_interlock.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
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
impl PhysicalDeviceFragmentShaderInterlockFeaturesEXT {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceFragmentShaderInterlockFeaturesEXT,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceFragmentShaderInterlockFeaturesEXTBuilder<'a> {
        PhysicalDeviceFragmentShaderInterlockFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceFragmentShaderInterlockFeaturesEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceFragmentShaderInterlockFeaturesEXT")
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
impl Default for PhysicalDeviceFragmentShaderInterlockFeaturesEXT {
    fn default() -> PhysicalDeviceFragmentShaderInterlockFeaturesEXT {
        PhysicalDeviceFragmentShaderInterlockFeaturesEXT {
            s_type:
                crate::vk1_0::StructureType::PHYSICAL_DEVICE_FRAGMENT_SHADER_INTERLOCK_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            fragment_shader_sample_interlock: Default::default(),
            fragment_shader_pixel_interlock: Default::default(),
            fragment_shader_shading_rate_interlock: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceFragmentShaderInterlockFeaturesEXT::extend`](struct.PhysicalDeviceFragmentShaderInterlockFeaturesEXT.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceFragmentShaderInterlockFeaturesEXT {}
impl ExtendableByPhysicalDeviceFragmentShaderInterlockFeaturesEXT
    for crate::vk1_1::PhysicalDeviceFeatures2
{
}
impl ExtendableByPhysicalDeviceFragmentShaderInterlockFeaturesEXT
    for crate::vk1_0::DeviceCreateInfo
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceFragmentShaderInterlockFeaturesEXT`](struct.PhysicalDeviceFragmentShaderInterlockFeaturesEXT.html)"]
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
    #[allow(unused_mut)]
    #[inline]
    pub fn fragment_shader_sample_interlock(
        mut self,
        fragment_shader_sample_interlock: bool,
    ) -> Self {
        self.0.fragment_shader_sample_interlock = fragment_shader_sample_interlock as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn fragment_shader_pixel_interlock(
        mut self,
        fragment_shader_pixel_interlock: bool,
    ) -> Self {
        self.0.fragment_shader_pixel_interlock = fragment_shader_pixel_interlock as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn fragment_shader_shading_rate_interlock(
        mut self,
        fragment_shader_shading_rate_interlock: bool,
    ) -> Self {
        self.0.fragment_shader_shading_rate_interlock =
            fragment_shader_shading_rate_interlock as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceFragmentShaderInterlockFeaturesEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceFragmentShaderInterlockFeaturesEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
