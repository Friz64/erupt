// DO NOT EDIT: @generated by erupt's generator
///<s>Vulkan Manual Page</s> · Constant
#[doc(alias = "VK_NV_COMPUTE_SHADER_DERIVATIVES_SPEC_VERSION")]
pub const NV_COMPUTE_SHADER_DERIVATIVES_SPEC_VERSION: u32 = 1;
///<s>Vulkan Manual Page</s> · Constant
#[doc(alias = "VK_NV_COMPUTE_SHADER_DERIVATIVES_EXTENSION_NAME")]
pub const NV_COMPUTE_SHADER_DERIVATIVES_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!(
    "VK_NV_compute_shader_derivatives"
);
///Provided by [`crate::extensions::nv_compute_shader_derivatives`]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_NV: Self = Self(
        1000201000,
    );
}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceComputeShaderDerivativesFeaturesNV>
for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<
    'a,
> crate::ExtendableFrom<'a, PhysicalDeviceComputeShaderDerivativesFeaturesNVBuilder<'_>>
for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceComputeShaderDerivativesFeaturesNV>
for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<
    'a,
> crate::ExtendableFrom<'a, PhysicalDeviceComputeShaderDerivativesFeaturesNVBuilder<'_>>
for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceComputeShaderDerivativesFeaturesNV.html) · Structure
#[doc(alias = "VkPhysicalDeviceComputeShaderDerivativesFeaturesNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceComputeShaderDerivativesFeaturesNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub compute_derivative_group_quads: crate::vk1_0::Bool32,
    pub compute_derivative_group_linear: crate::vk1_0::Bool32,
}
impl PhysicalDeviceComputeShaderDerivativesFeaturesNV {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_NV;
}
impl Default for PhysicalDeviceComputeShaderDerivativesFeaturesNV {
    fn default() -> Self {
        Self {
            s_type: Self::STRUCTURE_TYPE,
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
    pub fn into_builder<'a>(
        self,
    ) -> PhysicalDeviceComputeShaderDerivativesFeaturesNVBuilder<'a> {
        PhysicalDeviceComputeShaderDerivativesFeaturesNVBuilder(
            self,
            std::marker::PhantomData,
        )
    }
}
#[derive(Copy, Clone)]
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceComputeShaderDerivativesFeaturesNV.html) · Builder of [`PhysicalDeviceComputeShaderDerivativesFeaturesNV`]
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
    #[must_use]
    pub fn compute_derivative_group_quads(
        mut self,
        compute_derivative_group_quads: bool,
    ) -> Self {
        self.0.compute_derivative_group_quads = compute_derivative_group_quads as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn compute_derivative_group_linear(
        mut self,
        compute_derivative_group_linear: bool,
    ) -> Self {
        self.0.compute_derivative_group_linear = compute_derivative_group_linear as _;
        self
    }
    #[inline]
    /// Discards all lifetime information.
    /// Use the `Deref` and `DerefMut` implementations if possible.
    pub fn build_dangling(self) -> PhysicalDeviceComputeShaderDerivativesFeaturesNV {
        self.0
    }
}
impl<'a> std::default::Default
for PhysicalDeviceComputeShaderDerivativesFeaturesNVBuilder<'a> {
    fn default() -> PhysicalDeviceComputeShaderDerivativesFeaturesNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug
for PhysicalDeviceComputeShaderDerivativesFeaturesNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref
for PhysicalDeviceComputeShaderDerivativesFeaturesNVBuilder<'a> {
    type Target = PhysicalDeviceComputeShaderDerivativesFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut
for PhysicalDeviceComputeShaderDerivativesFeaturesNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
