# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_INTEL_shader_integer_functions2.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const INTEL_SHADER_INTEGER_FUNCTIONS_2_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const INTEL_SHADER_INTEGER_FUNCTIONS_2_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_INTEL_shader_integer_functions2");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub shader_integer_functions2: crate::vk1_0::Bool32,
}
impl PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL,
    {
        unsafe {
            crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
        }
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceShaderIntegerFunctions2FeaturesINTELBuilder<'a> {
        PhysicalDeviceShaderIntegerFunctions2FeaturesINTELBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "shader_integer_functions2",
                &(self.shader_integer_functions2 != 0),
            )
            .finish()
    }
}
impl Default for PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {
    fn default() -> PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {
        PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL { s_type : crate :: vk1_0 :: StructureType :: PHYSICAL_DEVICE_SHADER_INTEGER_FUNCTIONS_2_FEATURES_INTEL , p_next : std :: ptr :: null_mut ( ) , shader_integer_functions2 : Default :: default ( ) , }
    }
}
#[doc = "Used by [`PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL::extend`](struct.PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {}
impl ExtendableByPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL
    for crate::vk1_1::PhysicalDeviceFeatures2
{
}
impl ExtendableByPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL
    for crate::vk1_0::DeviceCreateInfo
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL`](struct.PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceShaderIntegerFunctions2FeaturesINTELBuilder<'a>(
    PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceShaderIntegerFunctions2FeaturesINTELBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceShaderIntegerFunctions2FeaturesINTELBuilder<'a> {
        PhysicalDeviceShaderIntegerFunctions2FeaturesINTELBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_integer_functions2(mut self, shader_integer_functions2: bool) -> Self {
        self.0.shader_integer_functions2 = shader_integer_functions2 as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceShaderIntegerFunctions2FeaturesINTELBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
