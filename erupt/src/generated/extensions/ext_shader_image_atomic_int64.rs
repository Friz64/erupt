#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_SHADER_IMAGE_ATOMIC_INT64_SPEC_VERSION")]
pub const EXT_SHADER_IMAGE_ATOMIC_INT64_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_SHADER_IMAGE_ATOMIC_INT64_EXTENSION_NAME")]
pub const EXT_SHADER_IMAGE_ATOMIC_INT64_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_shader_image_atomic_int64");
#[doc = "Provided by [`crate::extensions::ext_shader_image_atomic_int64`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_SHADER_IMAGE_ATOMIC_INT64_FEATURES_EXT: Self = Self(1000234000);
}
impl<'a> crate::ExtendableFromConst<'a, PhysicalDeviceShaderImageAtomicInt64FeaturesEXT> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, PhysicalDeviceShaderImageAtomicInt64FeaturesEXTBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceShaderImageAtomicInt64FeaturesEXT> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceShaderImageAtomicInt64FeaturesEXTBuilder<'_>> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderImageAtomicInt64FeaturesEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceShaderImageAtomicInt64FeaturesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceShaderImageAtomicInt64FeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub shader_image_int64_atomics: crate::vk1_0::Bool32,
    pub sparse_image_int64_atomics: crate::vk1_0::Bool32,
}
impl Default for PhysicalDeviceShaderImageAtomicInt64FeaturesEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_SHADER_IMAGE_ATOMIC_INT64_FEATURES_EXT, p_next: std::ptr::null_mut(), shader_image_int64_atomics: Default::default(), sparse_image_int64_atomics: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceShaderImageAtomicInt64FeaturesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceShaderImageAtomicInt64FeaturesEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("shader_image_int64_atomics", &(self.shader_image_int64_atomics != 0)).field("sparse_image_int64_atomics", &(self.sparse_image_int64_atomics != 0)).finish()
    }
}
impl PhysicalDeviceShaderImageAtomicInt64FeaturesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceShaderImageAtomicInt64FeaturesEXTBuilder<'a> {
        PhysicalDeviceShaderImageAtomicInt64FeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderImageAtomicInt64FeaturesEXT.html) · Builder of [`PhysicalDeviceShaderImageAtomicInt64FeaturesEXT`]"]
#[repr(transparent)]
pub struct PhysicalDeviceShaderImageAtomicInt64FeaturesEXTBuilder<'a>(PhysicalDeviceShaderImageAtomicInt64FeaturesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceShaderImageAtomicInt64FeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceShaderImageAtomicInt64FeaturesEXTBuilder<'a> {
        PhysicalDeviceShaderImageAtomicInt64FeaturesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn shader_image_int64_atomics(mut self, shader_image_int64_atomics: bool) -> Self {
        self.0.shader_image_int64_atomics = shader_image_int64_atomics as _;
        self
    }
    #[inline]
    pub fn sparse_image_int64_atomics(mut self, sparse_image_int64_atomics: bool) -> Self {
        self.0.sparse_image_int64_atomics = sparse_image_int64_atomics as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceShaderImageAtomicInt64FeaturesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceShaderImageAtomicInt64FeaturesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceShaderImageAtomicInt64FeaturesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceShaderImageAtomicInt64FeaturesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceShaderImageAtomicInt64FeaturesEXTBuilder<'a> {
    type Target = PhysicalDeviceShaderImageAtomicInt64FeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceShaderImageAtomicInt64FeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
