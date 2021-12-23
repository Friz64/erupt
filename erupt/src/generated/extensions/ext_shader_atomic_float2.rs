#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_SHADER_ATOMIC_FLOAT_2_SPEC_VERSION")]
pub const EXT_SHADER_ATOMIC_FLOAT_2_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_SHADER_ATOMIC_FLOAT_2_EXTENSION_NAME")]
pub const EXT_SHADER_ATOMIC_FLOAT_2_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_shader_atomic_float2");
#[doc = "Provided by [`crate::extensions::ext_shader_atomic_float2`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_2_FEATURES_EXT: Self = Self(1000273000);
}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceShaderAtomicFloat2FeaturesEXT> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceShaderAtomicFloat2FeaturesEXTBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceShaderAtomicFloat2FeaturesEXT> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceShaderAtomicFloat2FeaturesEXTBuilder<'_>> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderAtomicFloat2FeaturesEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceShaderAtomicFloat2FeaturesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceShaderAtomicFloat2FeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub shader_buffer_float16_atomics: crate::vk1_0::Bool32,
    pub shader_buffer_float16_atomic_add: crate::vk1_0::Bool32,
    pub shader_buffer_float16_atomic_min_max: crate::vk1_0::Bool32,
    pub shader_buffer_float32_atomic_min_max: crate::vk1_0::Bool32,
    pub shader_buffer_float64_atomic_min_max: crate::vk1_0::Bool32,
    pub shader_shared_float16_atomics: crate::vk1_0::Bool32,
    pub shader_shared_float16_atomic_add: crate::vk1_0::Bool32,
    pub shader_shared_float16_atomic_min_max: crate::vk1_0::Bool32,
    pub shader_shared_float32_atomic_min_max: crate::vk1_0::Bool32,
    pub shader_shared_float64_atomic_min_max: crate::vk1_0::Bool32,
    pub shader_image_float32_atomic_min_max: crate::vk1_0::Bool32,
    pub sparse_image_float32_atomic_min_max: crate::vk1_0::Bool32,
}
impl PhysicalDeviceShaderAtomicFloat2FeaturesEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_2_FEATURES_EXT;
}
impl Default for PhysicalDeviceShaderAtomicFloat2FeaturesEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), shader_buffer_float16_atomics: Default::default(), shader_buffer_float16_atomic_add: Default::default(), shader_buffer_float16_atomic_min_max: Default::default(), shader_buffer_float32_atomic_min_max: Default::default(), shader_buffer_float64_atomic_min_max: Default::default(), shader_shared_float16_atomics: Default::default(), shader_shared_float16_atomic_add: Default::default(), shader_shared_float16_atomic_min_max: Default::default(), shader_shared_float32_atomic_min_max: Default::default(), shader_shared_float64_atomic_min_max: Default::default(), shader_image_float32_atomic_min_max: Default::default(), sparse_image_float32_atomic_min_max: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceShaderAtomicFloat2FeaturesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceShaderAtomicFloat2FeaturesEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("shader_buffer_float16_atomics", &(self.shader_buffer_float16_atomics != 0)).field("shader_buffer_float16_atomic_add", &(self.shader_buffer_float16_atomic_add != 0)).field("shader_buffer_float16_atomic_min_max", &(self.shader_buffer_float16_atomic_min_max != 0)).field("shader_buffer_float32_atomic_min_max", &(self.shader_buffer_float32_atomic_min_max != 0)).field("shader_buffer_float64_atomic_min_max", &(self.shader_buffer_float64_atomic_min_max != 0)).field("shader_shared_float16_atomics", &(self.shader_shared_float16_atomics != 0)).field("shader_shared_float16_atomic_add", &(self.shader_shared_float16_atomic_add != 0)).field("shader_shared_float16_atomic_min_max", &(self.shader_shared_float16_atomic_min_max != 0)).field("shader_shared_float32_atomic_min_max", &(self.shader_shared_float32_atomic_min_max != 0)).field("shader_shared_float64_atomic_min_max", &(self.shader_shared_float64_atomic_min_max != 0)).field("shader_image_float32_atomic_min_max", &(self.shader_image_float32_atomic_min_max != 0)).field("sparse_image_float32_atomic_min_max", &(self.sparse_image_float32_atomic_min_max != 0)).finish()
    }
}
impl PhysicalDeviceShaderAtomicFloat2FeaturesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceShaderAtomicFloat2FeaturesEXTBuilder<'a> {
        PhysicalDeviceShaderAtomicFloat2FeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderAtomicFloat2FeaturesEXT.html) · Builder of [`PhysicalDeviceShaderAtomicFloat2FeaturesEXT`]"]
#[repr(transparent)]
pub struct PhysicalDeviceShaderAtomicFloat2FeaturesEXTBuilder<'a>(PhysicalDeviceShaderAtomicFloat2FeaturesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceShaderAtomicFloat2FeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceShaderAtomicFloat2FeaturesEXTBuilder<'a> {
        PhysicalDeviceShaderAtomicFloat2FeaturesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn shader_buffer_float16_atomics(mut self, shader_buffer_float16_atomics: bool) -> Self {
        self.0.shader_buffer_float16_atomics = shader_buffer_float16_atomics as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn shader_buffer_float16_atomic_add(mut self, shader_buffer_float16_atomic_add: bool) -> Self {
        self.0.shader_buffer_float16_atomic_add = shader_buffer_float16_atomic_add as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn shader_buffer_float16_atomic_min_max(mut self, shader_buffer_float16_atomic_min_max: bool) -> Self {
        self.0.shader_buffer_float16_atomic_min_max = shader_buffer_float16_atomic_min_max as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn shader_buffer_float32_atomic_min_max(mut self, shader_buffer_float32_atomic_min_max: bool) -> Self {
        self.0.shader_buffer_float32_atomic_min_max = shader_buffer_float32_atomic_min_max as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn shader_buffer_float64_atomic_min_max(mut self, shader_buffer_float64_atomic_min_max: bool) -> Self {
        self.0.shader_buffer_float64_atomic_min_max = shader_buffer_float64_atomic_min_max as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn shader_shared_float16_atomics(mut self, shader_shared_float16_atomics: bool) -> Self {
        self.0.shader_shared_float16_atomics = shader_shared_float16_atomics as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn shader_shared_float16_atomic_add(mut self, shader_shared_float16_atomic_add: bool) -> Self {
        self.0.shader_shared_float16_atomic_add = shader_shared_float16_atomic_add as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn shader_shared_float16_atomic_min_max(mut self, shader_shared_float16_atomic_min_max: bool) -> Self {
        self.0.shader_shared_float16_atomic_min_max = shader_shared_float16_atomic_min_max as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn shader_shared_float32_atomic_min_max(mut self, shader_shared_float32_atomic_min_max: bool) -> Self {
        self.0.shader_shared_float32_atomic_min_max = shader_shared_float32_atomic_min_max as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn shader_shared_float64_atomic_min_max(mut self, shader_shared_float64_atomic_min_max: bool) -> Self {
        self.0.shader_shared_float64_atomic_min_max = shader_shared_float64_atomic_min_max as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn shader_image_float32_atomic_min_max(mut self, shader_image_float32_atomic_min_max: bool) -> Self {
        self.0.shader_image_float32_atomic_min_max = shader_image_float32_atomic_min_max as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn sparse_image_float32_atomic_min_max(mut self, sparse_image_float32_atomic_min_max: bool) -> Self {
        self.0.sparse_image_float32_atomic_min_max = sparse_image_float32_atomic_min_max as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> PhysicalDeviceShaderAtomicFloat2FeaturesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceShaderAtomicFloat2FeaturesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceShaderAtomicFloat2FeaturesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceShaderAtomicFloat2FeaturesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceShaderAtomicFloat2FeaturesEXTBuilder<'a> {
    type Target = PhysicalDeviceShaderAtomicFloat2FeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceShaderAtomicFloat2FeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
