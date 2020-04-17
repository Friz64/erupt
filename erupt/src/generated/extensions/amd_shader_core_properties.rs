# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_AMD_shader_core_properties.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const AMD_SHADER_CORE_PROPERTIES_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const AMD_SHADER_CORE_PROPERTIES_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_AMD_shader_core_properties");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderCorePropertiesAMD.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceShaderCorePropertiesAMD {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub shader_engine_count: u32,
    pub shader_arrays_per_engine_count: u32,
    pub compute_units_per_shader_array: u32,
    pub simd_per_compute_unit: u32,
    pub wavefronts_per_simd: u32,
    pub wavefront_size: u32,
    pub sgprs_per_simd: u32,
    pub min_sgpr_allocation: u32,
    pub max_sgpr_allocation: u32,
    pub sgpr_allocation_granularity: u32,
    pub vgprs_per_simd: u32,
    pub min_vgpr_allocation: u32,
    pub max_vgpr_allocation: u32,
    pub vgpr_allocation_granularity: u32,
}
impl PhysicalDeviceShaderCorePropertiesAMD {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceShaderCorePropertiesAMD,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceShaderCorePropertiesAMDBuilder<'a> {
        PhysicalDeviceShaderCorePropertiesAMDBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceShaderCorePropertiesAMD {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceShaderCorePropertiesAMD")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("shader_engine_count", &self.shader_engine_count)
            .field(
                "shader_arrays_per_engine_count",
                &self.shader_arrays_per_engine_count,
            )
            .field(
                "compute_units_per_shader_array",
                &self.compute_units_per_shader_array,
            )
            .field("simd_per_compute_unit", &self.simd_per_compute_unit)
            .field("wavefronts_per_simd", &self.wavefronts_per_simd)
            .field("wavefront_size", &self.wavefront_size)
            .field("sgprs_per_simd", &self.sgprs_per_simd)
            .field("min_sgpr_allocation", &self.min_sgpr_allocation)
            .field("max_sgpr_allocation", &self.max_sgpr_allocation)
            .field(
                "sgpr_allocation_granularity",
                &self.sgpr_allocation_granularity,
            )
            .field("vgprs_per_simd", &self.vgprs_per_simd)
            .field("min_vgpr_allocation", &self.min_vgpr_allocation)
            .field("max_vgpr_allocation", &self.max_vgpr_allocation)
            .field(
                "vgpr_allocation_granularity",
                &self.vgpr_allocation_granularity,
            )
            .finish()
    }
}
impl Default for PhysicalDeviceShaderCorePropertiesAMD {
    fn default() -> PhysicalDeviceShaderCorePropertiesAMD {
        PhysicalDeviceShaderCorePropertiesAMD {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_AMD,
            p_next: std::ptr::null_mut(),
            shader_engine_count: Default::default(),
            shader_arrays_per_engine_count: Default::default(),
            compute_units_per_shader_array: Default::default(),
            simd_per_compute_unit: Default::default(),
            wavefronts_per_simd: Default::default(),
            wavefront_size: Default::default(),
            sgprs_per_simd: Default::default(),
            min_sgpr_allocation: Default::default(),
            max_sgpr_allocation: Default::default(),
            sgpr_allocation_granularity: Default::default(),
            vgprs_per_simd: Default::default(),
            min_vgpr_allocation: Default::default(),
            max_vgpr_allocation: Default::default(),
            vgpr_allocation_granularity: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceShaderCorePropertiesAMD::extend`](struct.PhysicalDeviceShaderCorePropertiesAMD.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceShaderCorePropertiesAMD {}
impl ExtendableByPhysicalDeviceShaderCorePropertiesAMD for crate::vk1_1::PhysicalDeviceProperties2 {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceShaderCorePropertiesAMD`](struct.PhysicalDeviceShaderCorePropertiesAMD.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceShaderCorePropertiesAMDBuilder<'a>(
    PhysicalDeviceShaderCorePropertiesAMD,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceShaderCorePropertiesAMDBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceShaderCorePropertiesAMDBuilder<'a> {
        PhysicalDeviceShaderCorePropertiesAMDBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_engine_count(mut self, shader_engine_count: u32) -> Self {
        self.0.shader_engine_count = shader_engine_count;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_arrays_per_engine_count(mut self, shader_arrays_per_engine_count: u32) -> Self {
        self.0.shader_arrays_per_engine_count = shader_arrays_per_engine_count;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn compute_units_per_shader_array(mut self, compute_units_per_shader_array: u32) -> Self {
        self.0.compute_units_per_shader_array = compute_units_per_shader_array;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn simd_per_compute_unit(mut self, simd_per_compute_unit: u32) -> Self {
        self.0.simd_per_compute_unit = simd_per_compute_unit;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn wavefronts_per_simd(mut self, wavefronts_per_simd: u32) -> Self {
        self.0.wavefronts_per_simd = wavefronts_per_simd;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn wavefront_size(mut self, wavefront_size: u32) -> Self {
        self.0.wavefront_size = wavefront_size;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn sgprs_per_simd(mut self, sgprs_per_simd: u32) -> Self {
        self.0.sgprs_per_simd = sgprs_per_simd;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn min_sgpr_allocation(mut self, min_sgpr_allocation: u32) -> Self {
        self.0.min_sgpr_allocation = min_sgpr_allocation;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_sgpr_allocation(mut self, max_sgpr_allocation: u32) -> Self {
        self.0.max_sgpr_allocation = max_sgpr_allocation;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn sgpr_allocation_granularity(mut self, sgpr_allocation_granularity: u32) -> Self {
        self.0.sgpr_allocation_granularity = sgpr_allocation_granularity;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn vgprs_per_simd(mut self, vgprs_per_simd: u32) -> Self {
        self.0.vgprs_per_simd = vgprs_per_simd;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn min_vgpr_allocation(mut self, min_vgpr_allocation: u32) -> Self {
        self.0.min_vgpr_allocation = min_vgpr_allocation;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_vgpr_allocation(mut self, max_vgpr_allocation: u32) -> Self {
        self.0.max_vgpr_allocation = max_vgpr_allocation;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn vgpr_allocation_granularity(mut self, vgpr_allocation_granularity: u32) -> Self {
        self.0.vgpr_allocation_granularity = vgpr_allocation_granularity;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceShaderCorePropertiesAMD {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceShaderCorePropertiesAMDBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceShaderCorePropertiesAMDBuilder<'a> {
    type Target = PhysicalDeviceShaderCorePropertiesAMD;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceShaderCorePropertiesAMDBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
