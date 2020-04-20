# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_NV_fragment_shader_barycentric.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NV_FRAGMENT_SHADER_BARYCENTRIC_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NV_FRAGMENT_SHADER_BARYCENTRIC_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_NV_fragment_shader_barycentric");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceFragmentShaderBarycentricFeaturesNV.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceFragmentShaderBarycentricFeaturesNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub fragment_shader_barycentric: crate::vk1_0::Bool32,
}
impl PhysicalDeviceFragmentShaderBarycentricFeaturesNV {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: crate::ExtendableBy<Self>,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceFragmentShaderBarycentricFeaturesNVBuilder<'a> {
        PhysicalDeviceFragmentShaderBarycentricFeaturesNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceFragmentShaderBarycentricFeaturesNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceFragmentShaderBarycentricFeaturesNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "fragment_shader_barycentric",
                &(self.fragment_shader_barycentric != 0),
            )
            .finish()
    }
}
impl Default for PhysicalDeviceFragmentShaderBarycentricFeaturesNV {
    fn default() -> PhysicalDeviceFragmentShaderBarycentricFeaturesNV {
        PhysicalDeviceFragmentShaderBarycentricFeaturesNV {
            s_type:
                crate::vk1_0::StructureType::PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_NV,
            p_next: std::ptr::null_mut(),
            fragment_shader_barycentric: Default::default(),
        }
    }
}
impl crate::ExtendableBy<PhysicalDeviceFragmentShaderBarycentricFeaturesNV>
    for crate::vk1_1::PhysicalDeviceFeatures2
{
}
impl crate::ExtendableBy<PhysicalDeviceFragmentShaderBarycentricFeaturesNV>
    for crate::vk1_0::DeviceCreateInfo
{
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceFragmentShaderBarycentricFeaturesNV.html) · Builder of [`PhysicalDeviceFragmentShaderBarycentricFeaturesNV`](struct.PhysicalDeviceFragmentShaderBarycentricFeaturesNV.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceFragmentShaderBarycentricFeaturesNVBuilder<'a>(
    PhysicalDeviceFragmentShaderBarycentricFeaturesNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceFragmentShaderBarycentricFeaturesNVBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceFragmentShaderBarycentricFeaturesNVBuilder<'a> {
        PhysicalDeviceFragmentShaderBarycentricFeaturesNVBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn fragment_shader_barycentric(mut self, fragment_shader_barycentric: bool) -> Self {
        self.0.fragment_shader_barycentric = fragment_shader_barycentric as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceFragmentShaderBarycentricFeaturesNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceFragmentShaderBarycentricFeaturesNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
