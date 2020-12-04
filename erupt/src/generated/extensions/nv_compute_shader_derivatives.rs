#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_COMPUTE_SHADER_DERIVATIVES_SPEC_VERSION")]
pub const NV_COMPUTE_SHADER_DERIVATIVES_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_COMPUTE_SHADER_DERIVATIVES_EXTENSION_NAME")]
pub const NV_COMPUTE_SHADER_DERIVATIVES_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_NV_compute_shader_derivatives");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceComputeShaderDerivativesFeaturesNV.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceComputeShaderDerivativesFeaturesNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceComputeShaderDerivativesFeaturesNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub compute_derivative_group_quads: crate::vk1_0::Bool32,
    pub compute_derivative_group_linear: crate::vk1_0::Bool32,
}
impl Default for PhysicalDeviceComputeShaderDerivativesFeaturesNV {
    fn default() -> Self {
        Self {
            s_type:
                crate::vk1_0::StructureType::PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_NV,
            p_next: std::ptr::null_mut(),
            compute_derivative_group_quads: Default::default(),
            compute_derivative_group_linear: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceComputeShaderDerivativesFeaturesNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceComputeShaderDerivativesFeaturesNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "compute_derivative_group_quads",
                &(self.compute_derivative_group_quads != 0),
            )
            .field(
                "compute_derivative_group_linear",
                &(self.compute_derivative_group_linear != 0),
            )
            .finish()
    }
}
impl PhysicalDeviceComputeShaderDerivativesFeaturesNV {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceComputeShaderDerivativesFeaturesNVBuilder<'a> {
        PhysicalDeviceComputeShaderDerivativesFeaturesNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceComputeShaderDerivativesFeaturesNV.html) · Builder of [`PhysicalDeviceComputeShaderDerivativesFeaturesNV`]"]
#[repr(transparent)]
pub struct PhysicalDeviceComputeShaderDerivativesFeaturesNVBuilder<'a>(
    PhysicalDeviceComputeShaderDerivativesFeaturesNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceComputeShaderDerivativesFeaturesNVBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceComputeShaderDerivativesFeaturesNVBuilder<'a> {
        PhysicalDeviceComputeShaderDerivativesFeaturesNVBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[inline]
    pub fn compute_derivative_group_quads(mut self, compute_derivative_group_quads: bool) -> Self {
        self.0.compute_derivative_group_quads = compute_derivative_group_quads as _;
        self
    }
    #[inline]
    pub fn compute_derivative_group_linear(
        mut self,
        compute_derivative_group_linear: bool,
    ) -> Self {
        self.0.compute_derivative_group_linear = compute_derivative_group_linear as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceComputeShaderDerivativesFeaturesNV {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceComputeShaderDerivativesFeaturesNVBuilder<'a> {
    fn default() -> PhysicalDeviceComputeShaderDerivativesFeaturesNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceComputeShaderDerivativesFeaturesNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceComputeShaderDerivativesFeaturesNVBuilder<'a> {
    type Target = PhysicalDeviceComputeShaderDerivativesFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceComputeShaderDerivativesFeaturesNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
