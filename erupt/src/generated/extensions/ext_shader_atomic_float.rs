#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_SHADER_ATOMIC_FLOAT_SPEC_VERSION")]
pub const EXT_SHADER_ATOMIC_FLOAT_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_SHADER_ATOMIC_FLOAT_EXTENSION_NAME")]
pub const EXT_SHADER_ATOMIC_FLOAT_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_shader_atomic_float");
#[doc = "Provided by [`crate::extensions::ext_shader_atomic_float`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_FEATURES_EXT: Self = Self(1000260000);
}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceShaderAtomicFloatFeaturesEXT> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceShaderAtomicFloatFeaturesEXTBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceShaderAtomicFloatFeaturesEXT> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceShaderAtomicFloatFeaturesEXTBuilder<'_>> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderAtomicFloatFeaturesEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceShaderAtomicFloatFeaturesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceShaderAtomicFloatFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub shader_buffer_float32_atomics: crate::vk1_0::Bool32,
    pub shader_buffer_float32_atomic_add: crate::vk1_0::Bool32,
    pub shader_buffer_float64_atomics: crate::vk1_0::Bool32,
    pub shader_buffer_float64_atomic_add: crate::vk1_0::Bool32,
    pub shader_shared_float32_atomics: crate::vk1_0::Bool32,
    pub shader_shared_float32_atomic_add: crate::vk1_0::Bool32,
    pub shader_shared_float64_atomics: crate::vk1_0::Bool32,
    pub shader_shared_float64_atomic_add: crate::vk1_0::Bool32,
    pub shader_image_float32_atomics: crate::vk1_0::Bool32,
    pub shader_image_float32_atomic_add: crate::vk1_0::Bool32,
    pub sparse_image_float32_atomics: crate::vk1_0::Bool32,
    pub sparse_image_float32_atomic_add: crate::vk1_0::Bool32,
}
impl PhysicalDeviceShaderAtomicFloatFeaturesEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_FEATURES_EXT;
}
impl Default for PhysicalDeviceShaderAtomicFloatFeaturesEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), shader_buffer_float32_atomics: Default::default(), shader_buffer_float32_atomic_add: Default::default(), shader_buffer_float64_atomics: Default::default(), shader_buffer_float64_atomic_add: Default::default(), shader_shared_float32_atomics: Default::default(), shader_shared_float32_atomic_add: Default::default(), shader_shared_float64_atomics: Default::default(), shader_shared_float64_atomic_add: Default::default(), shader_image_float32_atomics: Default::default(), shader_image_float32_atomic_add: Default::default(), sparse_image_float32_atomics: Default::default(), sparse_image_float32_atomic_add: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceShaderAtomicFloatFeaturesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceShaderAtomicFloatFeaturesEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("shader_buffer_float32_atomics", &(self.shader_buffer_float32_atomics != 0)).field("shader_buffer_float32_atomic_add", &(self.shader_buffer_float32_atomic_add != 0)).field("shader_buffer_float64_atomics", &(self.shader_buffer_float64_atomics != 0)).field("shader_buffer_float64_atomic_add", &(self.shader_buffer_float64_atomic_add != 0)).field("shader_shared_float32_atomics", &(self.shader_shared_float32_atomics != 0)).field("shader_shared_float32_atomic_add", &(self.shader_shared_float32_atomic_add != 0)).field("shader_shared_float64_atomics", &(self.shader_shared_float64_atomics != 0)).field("shader_shared_float64_atomic_add", &(self.shader_shared_float64_atomic_add != 0)).field("shader_image_float32_atomics", &(self.shader_image_float32_atomics != 0)).field("shader_image_float32_atomic_add", &(self.shader_image_float32_atomic_add != 0)).field("sparse_image_float32_atomics", &(self.sparse_image_float32_atomics != 0)).field("sparse_image_float32_atomic_add", &(self.sparse_image_float32_atomic_add != 0)).finish()
    }
}
impl PhysicalDeviceShaderAtomicFloatFeaturesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceShaderAtomicFloatFeaturesEXTBuilder<'a> {
        PhysicalDeviceShaderAtomicFloatFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderAtomicFloatFeaturesEXT.html) · Builder of [`PhysicalDeviceShaderAtomicFloatFeaturesEXT`]"]
#[repr(transparent)]
pub struct PhysicalDeviceShaderAtomicFloatFeaturesEXTBuilder<'a>(PhysicalDeviceShaderAtomicFloatFeaturesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceShaderAtomicFloatFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceShaderAtomicFloatFeaturesEXTBuilder<'a> {
        PhysicalDeviceShaderAtomicFloatFeaturesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn shader_buffer_float32_atomics(mut self, shader_buffer_float32_atomics: bool) -> Self {
        self.0.shader_buffer_float32_atomics = shader_buffer_float32_atomics as _;
        self
    }
    #[inline]
    pub fn shader_buffer_float32_atomic_add(mut self, shader_buffer_float32_atomic_add: bool) -> Self {
        self.0.shader_buffer_float32_atomic_add = shader_buffer_float32_atomic_add as _;
        self
    }
    #[inline]
    pub fn shader_buffer_float64_atomics(mut self, shader_buffer_float64_atomics: bool) -> Self {
        self.0.shader_buffer_float64_atomics = shader_buffer_float64_atomics as _;
        self
    }
    #[inline]
    pub fn shader_buffer_float64_atomic_add(mut self, shader_buffer_float64_atomic_add: bool) -> Self {
        self.0.shader_buffer_float64_atomic_add = shader_buffer_float64_atomic_add as _;
        self
    }
    #[inline]
    pub fn shader_shared_float32_atomics(mut self, shader_shared_float32_atomics: bool) -> Self {
        self.0.shader_shared_float32_atomics = shader_shared_float32_atomics as _;
        self
    }
    #[inline]
    pub fn shader_shared_float32_atomic_add(mut self, shader_shared_float32_atomic_add: bool) -> Self {
        self.0.shader_shared_float32_atomic_add = shader_shared_float32_atomic_add as _;
        self
    }
    #[inline]
    pub fn shader_shared_float64_atomics(mut self, shader_shared_float64_atomics: bool) -> Self {
        self.0.shader_shared_float64_atomics = shader_shared_float64_atomics as _;
        self
    }
    #[inline]
    pub fn shader_shared_float64_atomic_add(mut self, shader_shared_float64_atomic_add: bool) -> Self {
        self.0.shader_shared_float64_atomic_add = shader_shared_float64_atomic_add as _;
        self
    }
    #[inline]
    pub fn shader_image_float32_atomics(mut self, shader_image_float32_atomics: bool) -> Self {
        self.0.shader_image_float32_atomics = shader_image_float32_atomics as _;
        self
    }
    #[inline]
    pub fn shader_image_float32_atomic_add(mut self, shader_image_float32_atomic_add: bool) -> Self {
        self.0.shader_image_float32_atomic_add = shader_image_float32_atomic_add as _;
        self
    }
    #[inline]
    pub fn sparse_image_float32_atomics(mut self, sparse_image_float32_atomics: bool) -> Self {
        self.0.sparse_image_float32_atomics = sparse_image_float32_atomics as _;
        self
    }
    #[inline]
    pub fn sparse_image_float32_atomic_add(mut self, sparse_image_float32_atomic_add: bool) -> Self {
        self.0.sparse_image_float32_atomic_add = sparse_image_float32_atomic_add as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceShaderAtomicFloatFeaturesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceShaderAtomicFloatFeaturesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceShaderAtomicFloatFeaturesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceShaderAtomicFloatFeaturesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceShaderAtomicFloatFeaturesEXTBuilder<'a> {
    type Target = PhysicalDeviceShaderAtomicFloatFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceShaderAtomicFloatFeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
