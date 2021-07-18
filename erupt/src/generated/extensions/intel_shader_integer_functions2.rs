#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_INTEL_SHADER_INTEGER_FUNCTIONS_2_SPEC_VERSION")]
pub const INTEL_SHADER_INTEGER_FUNCTIONS_2_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_INTEL_SHADER_INTEGER_FUNCTIONS_2_EXTENSION_NAME")]
pub const INTEL_SHADER_INTEGER_FUNCTIONS_2_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_INTEL_shader_integer_functions2");
#[doc = "Provided by [`crate::extensions::intel_shader_integer_functions2`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_SHADER_INTEGER_FUNCTIONS_2_FEATURES_INTEL: Self = Self(1000209000);
}
impl<'a> crate::ExtendableFromConst<'a, PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, PhysicalDeviceShaderIntegerFunctions2FeaturesINTELBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceShaderIntegerFunctions2FeaturesINTELBuilder<'_>> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub shader_integer_functions2: crate::vk1_0::Bool32,
}
impl PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_SHADER_INTEGER_FUNCTIONS_2_FEATURES_INTEL;
}
impl Default for PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), shader_integer_functions2: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL").field("s_type", &self.s_type).field("p_next", &self.p_next).field("shader_integer_functions2", &(self.shader_integer_functions2 != 0)).finish()
    }
}
impl PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceShaderIntegerFunctions2FeaturesINTELBuilder<'a> {
        PhysicalDeviceShaderIntegerFunctions2FeaturesINTELBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL.html) · Builder of [`PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL`]"]
#[repr(transparent)]
pub struct PhysicalDeviceShaderIntegerFunctions2FeaturesINTELBuilder<'a>(PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceShaderIntegerFunctions2FeaturesINTELBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceShaderIntegerFunctions2FeaturesINTELBuilder<'a> {
        PhysicalDeviceShaderIntegerFunctions2FeaturesINTELBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn shader_integer_functions2(mut self, shader_integer_functions2: bool) -> Self {
        self.0.shader_integer_functions2 = shader_integer_functions2 as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceShaderIntegerFunctions2FeaturesINTELBuilder<'a> {
    fn default() -> PhysicalDeviceShaderIntegerFunctions2FeaturesINTELBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceShaderIntegerFunctions2FeaturesINTELBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceShaderIntegerFunctions2FeaturesINTELBuilder<'a> {
    type Target = PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceShaderIntegerFunctions2FeaturesINTELBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
