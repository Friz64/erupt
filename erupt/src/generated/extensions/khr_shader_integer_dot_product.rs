#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_SHADER_INTEGER_DOT_PRODUCT_SPEC_VERSION")]
pub const KHR_SHADER_INTEGER_DOT_PRODUCT_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_SHADER_INTEGER_DOT_PRODUCT_EXTENSION_NAME")]
pub const KHR_SHADER_INTEGER_DOT_PRODUCT_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_shader_integer_dot_product");
#[doc = "Provided by [`crate::extensions::khr_shader_integer_dot_product`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES_KHR: Self = Self(1000280000);
    pub const PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES_KHR: Self = Self(1000280001);
}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceShaderIntegerDotProductFeaturesKHR> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceShaderIntegerDotProductFeaturesKHRBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceShaderIntegerDotProductFeaturesKHR> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceShaderIntegerDotProductFeaturesKHRBuilder<'_>> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceShaderIntegerDotProductPropertiesKHR> for crate::vk1_1::PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceShaderIntegerDotProductPropertiesKHRBuilder<'_>> for crate::vk1_1::PhysicalDeviceProperties2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderIntegerDotProductFeaturesKHR.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceShaderIntegerDotProductFeaturesKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceShaderIntegerDotProductFeaturesKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub shader_integer_dot_product: crate::vk1_0::Bool32,
}
impl PhysicalDeviceShaderIntegerDotProductFeaturesKHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES_KHR;
}
impl Default for PhysicalDeviceShaderIntegerDotProductFeaturesKHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), shader_integer_dot_product: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceShaderIntegerDotProductFeaturesKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceShaderIntegerDotProductFeaturesKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("shader_integer_dot_product", &(self.shader_integer_dot_product != 0)).finish()
    }
}
impl PhysicalDeviceShaderIntegerDotProductFeaturesKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceShaderIntegerDotProductFeaturesKHRBuilder<'a> {
        PhysicalDeviceShaderIntegerDotProductFeaturesKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderIntegerDotProductFeaturesKHR.html) · Builder of [`PhysicalDeviceShaderIntegerDotProductFeaturesKHR`]"]
#[repr(transparent)]
pub struct PhysicalDeviceShaderIntegerDotProductFeaturesKHRBuilder<'a>(PhysicalDeviceShaderIntegerDotProductFeaturesKHR, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceShaderIntegerDotProductFeaturesKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceShaderIntegerDotProductFeaturesKHRBuilder<'a> {
        PhysicalDeviceShaderIntegerDotProductFeaturesKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn shader_integer_dot_product(mut self, shader_integer_dot_product: bool) -> Self {
        self.0.shader_integer_dot_product = shader_integer_dot_product as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceShaderIntegerDotProductFeaturesKHR {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceShaderIntegerDotProductFeaturesKHRBuilder<'a> {
    fn default() -> PhysicalDeviceShaderIntegerDotProductFeaturesKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceShaderIntegerDotProductFeaturesKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceShaderIntegerDotProductFeaturesKHRBuilder<'a> {
    type Target = PhysicalDeviceShaderIntegerDotProductFeaturesKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceShaderIntegerDotProductFeaturesKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderIntegerDotProductPropertiesKHR.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceShaderIntegerDotProductPropertiesKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceShaderIntegerDotProductPropertiesKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub integer_dot_product8_bit_unsigned_accelerated: crate::vk1_0::Bool32,
    pub integer_dot_product8_bit_signed_accelerated: crate::vk1_0::Bool32,
    pub integer_dot_product8_bit_mixed_signedness_accelerated: crate::vk1_0::Bool32,
    pub integer_dot_product4x8_bit_packed_unsigned_accelerated: crate::vk1_0::Bool32,
    pub integer_dot_product4x8_bit_packed_signed_accelerated: crate::vk1_0::Bool32,
    pub integer_dot_product4x8_bit_packed_mixed_signedness_accelerated: crate::vk1_0::Bool32,
    pub integer_dot_product16_bit_unsigned_accelerated: crate::vk1_0::Bool32,
    pub integer_dot_product16_bit_signed_accelerated: crate::vk1_0::Bool32,
    pub integer_dot_product16_bit_mixed_signedness_accelerated: crate::vk1_0::Bool32,
    pub integer_dot_product32_bit_unsigned_accelerated: crate::vk1_0::Bool32,
    pub integer_dot_product32_bit_signed_accelerated: crate::vk1_0::Bool32,
    pub integer_dot_product32_bit_mixed_signedness_accelerated: crate::vk1_0::Bool32,
    pub integer_dot_product64_bit_unsigned_accelerated: crate::vk1_0::Bool32,
    pub integer_dot_product64_bit_signed_accelerated: crate::vk1_0::Bool32,
    pub integer_dot_product64_bit_mixed_signedness_accelerated: crate::vk1_0::Bool32,
    pub integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated: crate::vk1_0::Bool32,
    pub integer_dot_product_accumulating_saturating8_bit_signed_accelerated: crate::vk1_0::Bool32,
    pub integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated: crate::vk1_0::Bool32,
    pub integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated: crate::vk1_0::Bool32,
    pub integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated: crate::vk1_0::Bool32,
    pub integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated: crate::vk1_0::Bool32,
    pub integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated: crate::vk1_0::Bool32,
    pub integer_dot_product_accumulating_saturating16_bit_signed_accelerated: crate::vk1_0::Bool32,
    pub integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated: crate::vk1_0::Bool32,
    pub integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated: crate::vk1_0::Bool32,
    pub integer_dot_product_accumulating_saturating32_bit_signed_accelerated: crate::vk1_0::Bool32,
    pub integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated: crate::vk1_0::Bool32,
    pub integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated: crate::vk1_0::Bool32,
    pub integer_dot_product_accumulating_saturating64_bit_signed_accelerated: crate::vk1_0::Bool32,
    pub integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated: crate::vk1_0::Bool32,
}
impl PhysicalDeviceShaderIntegerDotProductPropertiesKHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES_KHR;
}
impl Default for PhysicalDeviceShaderIntegerDotProductPropertiesKHR {
    fn default() -> Self {
        Self {
            s_type: Self::STRUCTURE_TYPE,
            p_next: std::ptr::null_mut(),
            integer_dot_product8_bit_unsigned_accelerated: Default::default(),
            integer_dot_product8_bit_signed_accelerated: Default::default(),
            integer_dot_product8_bit_mixed_signedness_accelerated: Default::default(),
            integer_dot_product4x8_bit_packed_unsigned_accelerated: Default::default(),
            integer_dot_product4x8_bit_packed_signed_accelerated: Default::default(),
            integer_dot_product4x8_bit_packed_mixed_signedness_accelerated: Default::default(),
            integer_dot_product16_bit_unsigned_accelerated: Default::default(),
            integer_dot_product16_bit_signed_accelerated: Default::default(),
            integer_dot_product16_bit_mixed_signedness_accelerated: Default::default(),
            integer_dot_product32_bit_unsigned_accelerated: Default::default(),
            integer_dot_product32_bit_signed_accelerated: Default::default(),
            integer_dot_product32_bit_mixed_signedness_accelerated: Default::default(),
            integer_dot_product64_bit_unsigned_accelerated: Default::default(),
            integer_dot_product64_bit_signed_accelerated: Default::default(),
            integer_dot_product64_bit_mixed_signedness_accelerated: Default::default(),
            integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated: Default::default(),
            integer_dot_product_accumulating_saturating8_bit_signed_accelerated: Default::default(),
            integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated: Default::default(),
            integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated: Default::default(),
            integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated: Default::default(),
            integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated: Default::default(),
            integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated: Default::default(),
            integer_dot_product_accumulating_saturating16_bit_signed_accelerated: Default::default(),
            integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated: Default::default(),
            integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated: Default::default(),
            integer_dot_product_accumulating_saturating32_bit_signed_accelerated: Default::default(),
            integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated: Default::default(),
            integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated: Default::default(),
            integer_dot_product_accumulating_saturating64_bit_signed_accelerated: Default::default(),
            integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceShaderIntegerDotProductPropertiesKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceShaderIntegerDotProductPropertiesKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("integer_dot_product8_bit_unsigned_accelerated", &(self.integer_dot_product8_bit_unsigned_accelerated != 0)).field("integer_dot_product8_bit_signed_accelerated", &(self.integer_dot_product8_bit_signed_accelerated != 0)).field("integer_dot_product8_bit_mixed_signedness_accelerated", &(self.integer_dot_product8_bit_mixed_signedness_accelerated != 0)).field("integer_dot_product4x8_bit_packed_unsigned_accelerated", &(self.integer_dot_product4x8_bit_packed_unsigned_accelerated != 0)).field("integer_dot_product4x8_bit_packed_signed_accelerated", &(self.integer_dot_product4x8_bit_packed_signed_accelerated != 0)).field("integer_dot_product4x8_bit_packed_mixed_signedness_accelerated", &(self.integer_dot_product4x8_bit_packed_mixed_signedness_accelerated != 0)).field("integer_dot_product16_bit_unsigned_accelerated", &(self.integer_dot_product16_bit_unsigned_accelerated != 0)).field("integer_dot_product16_bit_signed_accelerated", &(self.integer_dot_product16_bit_signed_accelerated != 0)).field("integer_dot_product16_bit_mixed_signedness_accelerated", &(self.integer_dot_product16_bit_mixed_signedness_accelerated != 0)).field("integer_dot_product32_bit_unsigned_accelerated", &(self.integer_dot_product32_bit_unsigned_accelerated != 0)).field("integer_dot_product32_bit_signed_accelerated", &(self.integer_dot_product32_bit_signed_accelerated != 0)).field("integer_dot_product32_bit_mixed_signedness_accelerated", &(self.integer_dot_product32_bit_mixed_signedness_accelerated != 0)).field("integer_dot_product64_bit_unsigned_accelerated", &(self.integer_dot_product64_bit_unsigned_accelerated != 0)).field("integer_dot_product64_bit_signed_accelerated", &(self.integer_dot_product64_bit_signed_accelerated != 0)).field("integer_dot_product64_bit_mixed_signedness_accelerated", &(self.integer_dot_product64_bit_mixed_signedness_accelerated != 0)).field("integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated", &(self.integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated != 0)).field("integer_dot_product_accumulating_saturating8_bit_signed_accelerated", &(self.integer_dot_product_accumulating_saturating8_bit_signed_accelerated != 0)).field("integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated", &(self.integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated != 0)).field("integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated", &(self.integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated != 0)).field("integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated", &(self.integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated != 0)).field("integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated", &(self.integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated != 0)).field("integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated", &(self.integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated != 0)).field("integer_dot_product_accumulating_saturating16_bit_signed_accelerated", &(self.integer_dot_product_accumulating_saturating16_bit_signed_accelerated != 0)).field("integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated", &(self.integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated != 0)).field("integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated", &(self.integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated != 0)).field("integer_dot_product_accumulating_saturating32_bit_signed_accelerated", &(self.integer_dot_product_accumulating_saturating32_bit_signed_accelerated != 0)).field("integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated", &(self.integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated != 0)).field("integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated", &(self.integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated != 0)).field("integer_dot_product_accumulating_saturating64_bit_signed_accelerated", &(self.integer_dot_product_accumulating_saturating64_bit_signed_accelerated != 0)).field("integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated", &(self.integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated != 0)).finish()
    }
}
impl PhysicalDeviceShaderIntegerDotProductPropertiesKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceShaderIntegerDotProductPropertiesKHRBuilder<'a> {
        PhysicalDeviceShaderIntegerDotProductPropertiesKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderIntegerDotProductPropertiesKHR.html) · Builder of [`PhysicalDeviceShaderIntegerDotProductPropertiesKHR`]"]
#[repr(transparent)]
pub struct PhysicalDeviceShaderIntegerDotProductPropertiesKHRBuilder<'a>(PhysicalDeviceShaderIntegerDotProductPropertiesKHR, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceShaderIntegerDotProductPropertiesKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceShaderIntegerDotProductPropertiesKHRBuilder<'a> {
        PhysicalDeviceShaderIntegerDotProductPropertiesKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn integer_dot_product8_bit_unsigned_accelerated(mut self, integer_dot_product8_bit_unsigned_accelerated: bool) -> Self {
        self.0.integer_dot_product8_bit_unsigned_accelerated = integer_dot_product8_bit_unsigned_accelerated as _;
        self
    }
    #[inline]
    pub fn integer_dot_product8_bit_signed_accelerated(mut self, integer_dot_product8_bit_signed_accelerated: bool) -> Self {
        self.0.integer_dot_product8_bit_signed_accelerated = integer_dot_product8_bit_signed_accelerated as _;
        self
    }
    #[inline]
    pub fn integer_dot_product8_bit_mixed_signedness_accelerated(mut self, integer_dot_product8_bit_mixed_signedness_accelerated: bool) -> Self {
        self.0.integer_dot_product8_bit_mixed_signedness_accelerated = integer_dot_product8_bit_mixed_signedness_accelerated as _;
        self
    }
    #[inline]
    pub fn integer_dot_product4x8_bit_packed_unsigned_accelerated(mut self, integer_dot_product4x8_bit_packed_unsigned_accelerated: bool) -> Self {
        self.0.integer_dot_product4x8_bit_packed_unsigned_accelerated = integer_dot_product4x8_bit_packed_unsigned_accelerated as _;
        self
    }
    #[inline]
    pub fn integer_dot_product4x8_bit_packed_signed_accelerated(mut self, integer_dot_product4x8_bit_packed_signed_accelerated: bool) -> Self {
        self.0.integer_dot_product4x8_bit_packed_signed_accelerated = integer_dot_product4x8_bit_packed_signed_accelerated as _;
        self
    }
    #[inline]
    pub fn integer_dot_product4x8_bit_packed_mixed_signedness_accelerated(mut self, integer_dot_product4x8_bit_packed_mixed_signedness_accelerated: bool) -> Self {
        self.0.integer_dot_product4x8_bit_packed_mixed_signedness_accelerated = integer_dot_product4x8_bit_packed_mixed_signedness_accelerated as _;
        self
    }
    #[inline]
    pub fn integer_dot_product16_bit_unsigned_accelerated(mut self, integer_dot_product16_bit_unsigned_accelerated: bool) -> Self {
        self.0.integer_dot_product16_bit_unsigned_accelerated = integer_dot_product16_bit_unsigned_accelerated as _;
        self
    }
    #[inline]
    pub fn integer_dot_product16_bit_signed_accelerated(mut self, integer_dot_product16_bit_signed_accelerated: bool) -> Self {
        self.0.integer_dot_product16_bit_signed_accelerated = integer_dot_product16_bit_signed_accelerated as _;
        self
    }
    #[inline]
    pub fn integer_dot_product16_bit_mixed_signedness_accelerated(mut self, integer_dot_product16_bit_mixed_signedness_accelerated: bool) -> Self {
        self.0.integer_dot_product16_bit_mixed_signedness_accelerated = integer_dot_product16_bit_mixed_signedness_accelerated as _;
        self
    }
    #[inline]
    pub fn integer_dot_product32_bit_unsigned_accelerated(mut self, integer_dot_product32_bit_unsigned_accelerated: bool) -> Self {
        self.0.integer_dot_product32_bit_unsigned_accelerated = integer_dot_product32_bit_unsigned_accelerated as _;
        self
    }
    #[inline]
    pub fn integer_dot_product32_bit_signed_accelerated(mut self, integer_dot_product32_bit_signed_accelerated: bool) -> Self {
        self.0.integer_dot_product32_bit_signed_accelerated = integer_dot_product32_bit_signed_accelerated as _;
        self
    }
    #[inline]
    pub fn integer_dot_product32_bit_mixed_signedness_accelerated(mut self, integer_dot_product32_bit_mixed_signedness_accelerated: bool) -> Self {
        self.0.integer_dot_product32_bit_mixed_signedness_accelerated = integer_dot_product32_bit_mixed_signedness_accelerated as _;
        self
    }
    #[inline]
    pub fn integer_dot_product64_bit_unsigned_accelerated(mut self, integer_dot_product64_bit_unsigned_accelerated: bool) -> Self {
        self.0.integer_dot_product64_bit_unsigned_accelerated = integer_dot_product64_bit_unsigned_accelerated as _;
        self
    }
    #[inline]
    pub fn integer_dot_product64_bit_signed_accelerated(mut self, integer_dot_product64_bit_signed_accelerated: bool) -> Self {
        self.0.integer_dot_product64_bit_signed_accelerated = integer_dot_product64_bit_signed_accelerated as _;
        self
    }
    #[inline]
    pub fn integer_dot_product64_bit_mixed_signedness_accelerated(mut self, integer_dot_product64_bit_mixed_signedness_accelerated: bool) -> Self {
        self.0.integer_dot_product64_bit_mixed_signedness_accelerated = integer_dot_product64_bit_mixed_signedness_accelerated as _;
        self
    }
    #[inline]
    pub fn integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated(mut self, integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated: bool) -> Self {
        self.0.integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated = integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated as _;
        self
    }
    #[inline]
    pub fn integer_dot_product_accumulating_saturating8_bit_signed_accelerated(mut self, integer_dot_product_accumulating_saturating8_bit_signed_accelerated: bool) -> Self {
        self.0.integer_dot_product_accumulating_saturating8_bit_signed_accelerated = integer_dot_product_accumulating_saturating8_bit_signed_accelerated as _;
        self
    }
    #[inline]
    pub fn integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated(mut self, integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated: bool) -> Self {
        self.0.integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated = integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated as _;
        self
    }
    #[inline]
    pub fn integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated(mut self, integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated: bool) -> Self {
        self.0.integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated = integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated as _;
        self
    }
    #[inline]
    pub fn integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated(mut self, integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated: bool) -> Self {
        self.0.integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated = integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated as _;
        self
    }
    #[inline]
    pub fn integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated(mut self, integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated: bool) -> Self {
        self.0.integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated = integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated as _;
        self
    }
    #[inline]
    pub fn integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated(mut self, integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated: bool) -> Self {
        self.0.integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated = integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated as _;
        self
    }
    #[inline]
    pub fn integer_dot_product_accumulating_saturating16_bit_signed_accelerated(mut self, integer_dot_product_accumulating_saturating16_bit_signed_accelerated: bool) -> Self {
        self.0.integer_dot_product_accumulating_saturating16_bit_signed_accelerated = integer_dot_product_accumulating_saturating16_bit_signed_accelerated as _;
        self
    }
    #[inline]
    pub fn integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated(mut self, integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated: bool) -> Self {
        self.0.integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated = integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated as _;
        self
    }
    #[inline]
    pub fn integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated(mut self, integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated: bool) -> Self {
        self.0.integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated = integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated as _;
        self
    }
    #[inline]
    pub fn integer_dot_product_accumulating_saturating32_bit_signed_accelerated(mut self, integer_dot_product_accumulating_saturating32_bit_signed_accelerated: bool) -> Self {
        self.0.integer_dot_product_accumulating_saturating32_bit_signed_accelerated = integer_dot_product_accumulating_saturating32_bit_signed_accelerated as _;
        self
    }
    #[inline]
    pub fn integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated(mut self, integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated: bool) -> Self {
        self.0.integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated = integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated as _;
        self
    }
    #[inline]
    pub fn integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated(mut self, integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated: bool) -> Self {
        self.0.integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated = integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated as _;
        self
    }
    #[inline]
    pub fn integer_dot_product_accumulating_saturating64_bit_signed_accelerated(mut self, integer_dot_product_accumulating_saturating64_bit_signed_accelerated: bool) -> Self {
        self.0.integer_dot_product_accumulating_saturating64_bit_signed_accelerated = integer_dot_product_accumulating_saturating64_bit_signed_accelerated as _;
        self
    }
    #[inline]
    pub fn integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated(mut self, integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated: bool) -> Self {
        self.0.integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated = integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceShaderIntegerDotProductPropertiesKHR {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceShaderIntegerDotProductPropertiesKHRBuilder<'a> {
    fn default() -> PhysicalDeviceShaderIntegerDotProductPropertiesKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceShaderIntegerDotProductPropertiesKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceShaderIntegerDotProductPropertiesKHRBuilder<'a> {
    type Target = PhysicalDeviceShaderIntegerDotProductPropertiesKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceShaderIntegerDotProductPropertiesKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
