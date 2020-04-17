# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_NV_compute_shader_derivatives.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NV_COMPUTE_SHADER_DERIVATIVES_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NV_COMPUTE_SHADER_DERIVATIVES_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_NV_compute_shader_derivatives");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceComputeShaderDerivativesFeaturesNV.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceComputeShaderDerivativesFeaturesNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub compute_derivative_group_quads: crate::vk1_0::Bool32,
    pub compute_derivative_group_linear: crate::vk1_0::Bool32,
}
impl PhysicalDeviceComputeShaderDerivativesFeaturesNV {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceComputeShaderDerivativesFeaturesNV,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceComputeShaderDerivativesFeaturesNVBuilder<'a> {
        PhysicalDeviceComputeShaderDerivativesFeaturesNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceComputeShaderDerivativesFeaturesNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceComputeShaderDerivativesFeaturesNV")
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
impl Default for PhysicalDeviceComputeShaderDerivativesFeaturesNV {
    fn default() -> PhysicalDeviceComputeShaderDerivativesFeaturesNV {
        PhysicalDeviceComputeShaderDerivativesFeaturesNV {
            s_type:
                crate::vk1_0::StructureType::PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_NV,
            p_next: std::ptr::null_mut(),
            compute_derivative_group_quads: Default::default(),
            compute_derivative_group_linear: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceComputeShaderDerivativesFeaturesNV::extend`](struct.PhysicalDeviceComputeShaderDerivativesFeaturesNV.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceComputeShaderDerivativesFeaturesNV {}
impl ExtendableByPhysicalDeviceComputeShaderDerivativesFeaturesNV
    for crate::vk1_1::PhysicalDeviceFeatures2
{
}
impl ExtendableByPhysicalDeviceComputeShaderDerivativesFeaturesNV
    for crate::vk1_0::DeviceCreateInfo
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceComputeShaderDerivativesFeaturesNV`](struct.PhysicalDeviceComputeShaderDerivativesFeaturesNV.html)"]
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
    #[allow(unused_mut)]
    #[inline]
    pub fn compute_derivative_group_quads(mut self, compute_derivative_group_quads: bool) -> Self {
        self.0.compute_derivative_group_quads = compute_derivative_group_quads as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn compute_derivative_group_linear(
        mut self,
        compute_derivative_group_linear: bool,
    ) -> Self {
        self.0.compute_derivative_group_linear = compute_derivative_group_linear as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceComputeShaderDerivativesFeaturesNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceComputeShaderDerivativesFeaturesNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
