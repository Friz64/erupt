#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_RAY_TRACING_MOTION_BLUR_SPEC_VERSION")]
pub const NV_RAY_TRACING_MOTION_BLUR_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_RAY_TRACING_MOTION_BLUR_EXTENSION_NAME")]
pub const NV_RAY_TRACING_MOTION_BLUR_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_NV_ray_tracing_motion_blur");
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureMotionInfoFlagsNV.html) · Bitmask of [`AccelerationStructureMotionInfoFlagBitsNV`]"] # [doc (alias = "VkAccelerationStructureMotionInfoFlagsNV")] # [derive (Default)] # [repr (transparent)] pub struct AccelerationStructureMotionInfoFlagsNV : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`AccelerationStructureMotionInfoFlagsNV`]"]
#[doc(alias = "VkAccelerationStructureMotionInfoFlagBitsNV")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct AccelerationStructureMotionInfoFlagBitsNV(pub u32);
impl AccelerationStructureMotionInfoFlagBitsNV {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> AccelerationStructureMotionInfoFlagsNV {
        AccelerationStructureMotionInfoFlagsNV::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for AccelerationStructureMotionInfoFlagBitsNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureMotionInstanceFlagsNV.html) · Bitmask of [`AccelerationStructureMotionInstanceFlagBitsNV`]"] # [doc (alias = "VkAccelerationStructureMotionInstanceFlagsNV")] # [derive (Default)] # [repr (transparent)] pub struct AccelerationStructureMotionInstanceFlagsNV : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`AccelerationStructureMotionInstanceFlagsNV`]"]
#[doc(alias = "VkAccelerationStructureMotionInstanceFlagBitsNV")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct AccelerationStructureMotionInstanceFlagBitsNV(pub u32);
impl AccelerationStructureMotionInstanceFlagBitsNV {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> AccelerationStructureMotionInstanceFlagsNV {
        AccelerationStructureMotionInstanceFlagsNV::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for AccelerationStructureMotionInstanceFlagBitsNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::nv_ray_tracing_motion_blur`]"]
impl crate::vk1_0::PipelineCreateFlagBits {
    pub const RAY_TRACING_ALLOW_MOTION_NV: Self = Self(1048576);
}
#[doc = "Provided by [`crate::extensions::nv_ray_tracing_motion_blur`]"]
impl crate::vk1_0::StructureType {
    pub const ACCELERATION_STRUCTURE_GEOMETRY_MOTION_TRIANGLES_DATA_NV: Self = Self(1000327000);
    pub const PHYSICAL_DEVICE_RAY_TRACING_MOTION_BLUR_FEATURES_NV: Self = Self(1000327001);
    pub const ACCELERATION_STRUCTURE_MOTION_INFO_NV: Self = Self(1000327002);
}
#[doc = "Provided by [`crate::extensions::nv_ray_tracing_motion_blur`]"]
impl crate::extensions::khr_acceleration_structure::BuildAccelerationStructureFlagBitsKHR {
    pub const MOTION_NV: Self = Self(32);
}
#[doc = "Provided by [`crate::extensions::nv_ray_tracing_motion_blur`]"]
impl crate::extensions::khr_acceleration_structure::AccelerationStructureCreateFlagBitsKHR {
    pub const MOTION_NV: Self = Self(4);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureMotionInstanceTypeNV.html) · Enum"]
#[doc(alias = "VkAccelerationStructureMotionInstanceTypeNV")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct AccelerationStructureMotionInstanceTypeNV(pub i32);
impl std::fmt::Debug for AccelerationStructureMotionInstanceTypeNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::STATIC_NV => "STATIC_NV",
            &Self::MATRIX_MOTION_NV => "MATRIX_MOTION_NV",
            &Self::SRT_MOTION_NV => "SRT_MOTION_NV",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::nv_ray_tracing_motion_blur`]"]
impl crate::extensions::nv_ray_tracing_motion_blur::AccelerationStructureMotionInstanceTypeNV {
    pub const STATIC_NV: Self = Self(0);
    pub const MATRIX_MOTION_NV: Self = Self(1);
    pub const SRT_MOTION_NV: Self = Self(2);
}
impl<'a> crate::ExtendableFromConst<'a, PhysicalDeviceRayTracingMotionBlurFeaturesNV> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, PhysicalDeviceRayTracingMotionBlurFeaturesNVBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceRayTracingMotionBlurFeaturesNV> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceRayTracingMotionBlurFeaturesNVBuilder<'_>> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, AccelerationStructureGeometryMotionTrianglesDataNV> for crate::extensions::khr_acceleration_structure::AccelerationStructureGeometryTrianglesDataKHRBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, AccelerationStructureGeometryMotionTrianglesDataNVBuilder<'_>> for crate::extensions::khr_acceleration_structure::AccelerationStructureGeometryTrianglesDataKHRBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, AccelerationStructureMotionInfoNV> for crate::extensions::khr_acceleration_structure::AccelerationStructureCreateInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, AccelerationStructureMotionInfoNVBuilder<'_>> for crate::extensions::khr_acceleration_structure::AccelerationStructureCreateInfoKHRBuilder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceRayTracingMotionBlurFeaturesNV.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceRayTracingMotionBlurFeaturesNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceRayTracingMotionBlurFeaturesNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub ray_tracing_motion_blur: crate::vk1_0::Bool32,
    pub ray_tracing_motion_blur_pipeline_trace_rays_indirect: crate::vk1_0::Bool32,
}
impl PhysicalDeviceRayTracingMotionBlurFeaturesNV {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_RAY_TRACING_MOTION_BLUR_FEATURES_NV;
}
impl Default for PhysicalDeviceRayTracingMotionBlurFeaturesNV {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_RAY_TRACING_MOTION_BLUR_FEATURES_NV, p_next: std::ptr::null_mut(), ray_tracing_motion_blur: Default::default(), ray_tracing_motion_blur_pipeline_trace_rays_indirect: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceRayTracingMotionBlurFeaturesNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceRayTracingMotionBlurFeaturesNV").field("s_type", &self.s_type).field("p_next", &self.p_next).field("ray_tracing_motion_blur", &(self.ray_tracing_motion_blur != 0)).field("ray_tracing_motion_blur_pipeline_trace_rays_indirect", &(self.ray_tracing_motion_blur_pipeline_trace_rays_indirect != 0)).finish()
    }
}
impl PhysicalDeviceRayTracingMotionBlurFeaturesNV {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceRayTracingMotionBlurFeaturesNVBuilder<'a> {
        PhysicalDeviceRayTracingMotionBlurFeaturesNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceRayTracingMotionBlurFeaturesNV.html) · Builder of [`PhysicalDeviceRayTracingMotionBlurFeaturesNV`]"]
#[repr(transparent)]
pub struct PhysicalDeviceRayTracingMotionBlurFeaturesNVBuilder<'a>(PhysicalDeviceRayTracingMotionBlurFeaturesNV, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceRayTracingMotionBlurFeaturesNVBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceRayTracingMotionBlurFeaturesNVBuilder<'a> {
        PhysicalDeviceRayTracingMotionBlurFeaturesNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn ray_tracing_motion_blur(mut self, ray_tracing_motion_blur: bool) -> Self {
        self.0.ray_tracing_motion_blur = ray_tracing_motion_blur as _;
        self
    }
    #[inline]
    pub fn ray_tracing_motion_blur_pipeline_trace_rays_indirect(mut self, ray_tracing_motion_blur_pipeline_trace_rays_indirect: bool) -> Self {
        self.0.ray_tracing_motion_blur_pipeline_trace_rays_indirect = ray_tracing_motion_blur_pipeline_trace_rays_indirect as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceRayTracingMotionBlurFeaturesNV {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceRayTracingMotionBlurFeaturesNVBuilder<'a> {
    fn default() -> PhysicalDeviceRayTracingMotionBlurFeaturesNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceRayTracingMotionBlurFeaturesNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceRayTracingMotionBlurFeaturesNVBuilder<'a> {
    type Target = PhysicalDeviceRayTracingMotionBlurFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceRayTracingMotionBlurFeaturesNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureGeometryMotionTrianglesDataNV.html) · Structure"]
#[doc(alias = "VkAccelerationStructureGeometryMotionTrianglesDataNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AccelerationStructureGeometryMotionTrianglesDataNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub vertex_data: crate::extensions::khr_acceleration_structure::DeviceOrHostAddressConstKHR,
}
impl AccelerationStructureGeometryMotionTrianglesDataNV {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::ACCELERATION_STRUCTURE_GEOMETRY_MOTION_TRIANGLES_DATA_NV;
}
impl Default for AccelerationStructureGeometryMotionTrianglesDataNV {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::ACCELERATION_STRUCTURE_GEOMETRY_MOTION_TRIANGLES_DATA_NV, p_next: std::ptr::null(), vertex_data: Default::default() }
    }
}
impl std::fmt::Debug for AccelerationStructureGeometryMotionTrianglesDataNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AccelerationStructureGeometryMotionTrianglesDataNV").field("s_type", &self.s_type).field("p_next", &self.p_next).field("vertex_data", &self.vertex_data).finish()
    }
}
impl AccelerationStructureGeometryMotionTrianglesDataNV {
    #[inline]
    pub fn into_builder<'a>(self) -> AccelerationStructureGeometryMotionTrianglesDataNVBuilder<'a> {
        AccelerationStructureGeometryMotionTrianglesDataNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureGeometryMotionTrianglesDataNV.html) · Builder of [`AccelerationStructureGeometryMotionTrianglesDataNV`]"]
#[repr(transparent)]
pub struct AccelerationStructureGeometryMotionTrianglesDataNVBuilder<'a>(AccelerationStructureGeometryMotionTrianglesDataNV, std::marker::PhantomData<&'a ()>);
impl<'a> AccelerationStructureGeometryMotionTrianglesDataNVBuilder<'a> {
    #[inline]
    pub fn new() -> AccelerationStructureGeometryMotionTrianglesDataNVBuilder<'a> {
        AccelerationStructureGeometryMotionTrianglesDataNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn vertex_data(mut self, vertex_data: crate::extensions::khr_acceleration_structure::DeviceOrHostAddressConstKHR) -> Self {
        self.0.vertex_data = vertex_data as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> AccelerationStructureGeometryMotionTrianglesDataNV {
        self.0
    }
}
impl<'a> std::default::Default for AccelerationStructureGeometryMotionTrianglesDataNVBuilder<'a> {
    fn default() -> AccelerationStructureGeometryMotionTrianglesDataNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for AccelerationStructureGeometryMotionTrianglesDataNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for AccelerationStructureGeometryMotionTrianglesDataNVBuilder<'a> {
    type Target = AccelerationStructureGeometryMotionTrianglesDataNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for AccelerationStructureGeometryMotionTrianglesDataNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureMotionInfoNV.html) · Structure"]
#[doc(alias = "VkAccelerationStructureMotionInfoNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AccelerationStructureMotionInfoNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub max_instances: u32,
    pub flags: crate::extensions::nv_ray_tracing_motion_blur::AccelerationStructureMotionInfoFlagsNV,
}
impl AccelerationStructureMotionInfoNV {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::ACCELERATION_STRUCTURE_MOTION_INFO_NV;
}
impl Default for AccelerationStructureMotionInfoNV {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::ACCELERATION_STRUCTURE_MOTION_INFO_NV, p_next: std::ptr::null(), max_instances: Default::default(), flags: Default::default() }
    }
}
impl std::fmt::Debug for AccelerationStructureMotionInfoNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AccelerationStructureMotionInfoNV").field("s_type", &self.s_type).field("p_next", &self.p_next).field("max_instances", &self.max_instances).field("flags", &self.flags).finish()
    }
}
impl AccelerationStructureMotionInfoNV {
    #[inline]
    pub fn into_builder<'a>(self) -> AccelerationStructureMotionInfoNVBuilder<'a> {
        AccelerationStructureMotionInfoNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureMotionInfoNV.html) · Builder of [`AccelerationStructureMotionInfoNV`]"]
#[repr(transparent)]
pub struct AccelerationStructureMotionInfoNVBuilder<'a>(AccelerationStructureMotionInfoNV, std::marker::PhantomData<&'a ()>);
impl<'a> AccelerationStructureMotionInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> AccelerationStructureMotionInfoNVBuilder<'a> {
        AccelerationStructureMotionInfoNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn max_instances(mut self, max_instances: u32) -> Self {
        self.0.max_instances = max_instances as _;
        self
    }
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::nv_ray_tracing_motion_blur::AccelerationStructureMotionInfoFlagsNV) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> AccelerationStructureMotionInfoNV {
        self.0
    }
}
impl<'a> std::default::Default for AccelerationStructureMotionInfoNVBuilder<'a> {
    fn default() -> AccelerationStructureMotionInfoNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for AccelerationStructureMotionInfoNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for AccelerationStructureMotionInfoNVBuilder<'a> {
    type Target = AccelerationStructureMotionInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for AccelerationStructureMotionInfoNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSRTDataNV.html) · Structure"]
#[doc(alias = "VkSRTDataNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SRTDataNV {
    pub sx: std::os::raw::c_float,
    pub a: std::os::raw::c_float,
    pub b: std::os::raw::c_float,
    pub pvx: std::os::raw::c_float,
    pub sy: std::os::raw::c_float,
    pub c: std::os::raw::c_float,
    pub pvy: std::os::raw::c_float,
    pub sz: std::os::raw::c_float,
    pub pvz: std::os::raw::c_float,
    pub qx: std::os::raw::c_float,
    pub qy: std::os::raw::c_float,
    pub qz: std::os::raw::c_float,
    pub qw: std::os::raw::c_float,
    pub tx: std::os::raw::c_float,
    pub ty: std::os::raw::c_float,
    pub tz: std::os::raw::c_float,
}
impl Default for SRTDataNV {
    fn default() -> Self {
        Self { sx: Default::default(), a: Default::default(), b: Default::default(), pvx: Default::default(), sy: Default::default(), c: Default::default(), pvy: Default::default(), sz: Default::default(), pvz: Default::default(), qx: Default::default(), qy: Default::default(), qz: Default::default(), qw: Default::default(), tx: Default::default(), ty: Default::default(), tz: Default::default() }
    }
}
impl std::fmt::Debug for SRTDataNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SRTDataNV").field("sx", &self.sx).field("a", &self.a).field("b", &self.b).field("pvx", &self.pvx).field("sy", &self.sy).field("c", &self.c).field("pvy", &self.pvy).field("sz", &self.sz).field("pvz", &self.pvz).field("qx", &self.qx).field("qy", &self.qy).field("qz", &self.qz).field("qw", &self.qw).field("tx", &self.tx).field("ty", &self.ty).field("tz", &self.tz).finish()
    }
}
impl SRTDataNV {
    #[inline]
    pub fn into_builder<'a>(self) -> SRTDataNVBuilder<'a> {
        SRTDataNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSRTDataNV.html) · Builder of [`SRTDataNV`]"]
#[repr(transparent)]
pub struct SRTDataNVBuilder<'a>(SRTDataNV, std::marker::PhantomData<&'a ()>);
impl<'a> SRTDataNVBuilder<'a> {
    #[inline]
    pub fn new() -> SRTDataNVBuilder<'a> {
        SRTDataNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn sx(mut self, sx: std::os::raw::c_float) -> Self {
        self.0.sx = sx as _;
        self
    }
    #[inline]
    pub fn a(mut self, a: std::os::raw::c_float) -> Self {
        self.0.a = a as _;
        self
    }
    #[inline]
    pub fn b(mut self, b: std::os::raw::c_float) -> Self {
        self.0.b = b as _;
        self
    }
    #[inline]
    pub fn pvx(mut self, pvx: std::os::raw::c_float) -> Self {
        self.0.pvx = pvx as _;
        self
    }
    #[inline]
    pub fn sy(mut self, sy: std::os::raw::c_float) -> Self {
        self.0.sy = sy as _;
        self
    }
    #[inline]
    pub fn c(mut self, c: std::os::raw::c_float) -> Self {
        self.0.c = c as _;
        self
    }
    #[inline]
    pub fn pvy(mut self, pvy: std::os::raw::c_float) -> Self {
        self.0.pvy = pvy as _;
        self
    }
    #[inline]
    pub fn sz(mut self, sz: std::os::raw::c_float) -> Self {
        self.0.sz = sz as _;
        self
    }
    #[inline]
    pub fn pvz(mut self, pvz: std::os::raw::c_float) -> Self {
        self.0.pvz = pvz as _;
        self
    }
    #[inline]
    pub fn qx(mut self, qx: std::os::raw::c_float) -> Self {
        self.0.qx = qx as _;
        self
    }
    #[inline]
    pub fn qy(mut self, qy: std::os::raw::c_float) -> Self {
        self.0.qy = qy as _;
        self
    }
    #[inline]
    pub fn qz(mut self, qz: std::os::raw::c_float) -> Self {
        self.0.qz = qz as _;
        self
    }
    #[inline]
    pub fn qw(mut self, qw: std::os::raw::c_float) -> Self {
        self.0.qw = qw as _;
        self
    }
    #[inline]
    pub fn tx(mut self, tx: std::os::raw::c_float) -> Self {
        self.0.tx = tx as _;
        self
    }
    #[inline]
    pub fn ty(mut self, ty: std::os::raw::c_float) -> Self {
        self.0.ty = ty as _;
        self
    }
    #[inline]
    pub fn tz(mut self, tz: std::os::raw::c_float) -> Self {
        self.0.tz = tz as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> SRTDataNV {
        self.0
    }
}
impl<'a> std::default::Default for SRTDataNVBuilder<'a> {
    fn default() -> SRTDataNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for SRTDataNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for SRTDataNVBuilder<'a> {
    type Target = SRTDataNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SRTDataNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureSRTMotionInstanceNV.html) · Structure"]
#[doc(alias = "VkAccelerationStructureSRTMotionInstanceNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AccelerationStructureSRTMotionInstanceNV {
    pub transform_t0: crate::extensions::nv_ray_tracing_motion_blur::SRTDataNV,
    pub transform_t1: crate::extensions::nv_ray_tracing_motion_blur::SRTDataNV,
    pub instance_custom_index_and_mask: u32,
    pub instance_shader_binding_table_record_offset_and_flags: u32,
    pub acceleration_structure_reference: u64,
}
impl Default for AccelerationStructureSRTMotionInstanceNV {
    fn default() -> Self {
        Self { transform_t0: Default::default(), transform_t1: Default::default(), instance_custom_index_and_mask: Default::default(), instance_shader_binding_table_record_offset_and_flags: Default::default(), acceleration_structure_reference: Default::default() }
    }
}
impl std::fmt::Debug for AccelerationStructureSRTMotionInstanceNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AccelerationStructureSRTMotionInstanceNV").field("transform_t0", &self.transform_t0).field("transform_t1", &self.transform_t1).field("instance_custom_index_and_mask", &format!("{:#b}", &self.instance_custom_index_and_mask)).field("instance_shader_binding_table_record_offset_and_flags", &format!("{:#b}", &self.instance_shader_binding_table_record_offset_and_flags)).field("acceleration_structure_reference", &self.acceleration_structure_reference).finish()
    }
}
impl AccelerationStructureSRTMotionInstanceNV {
    #[inline]
    pub fn into_builder<'a>(self) -> AccelerationStructureSRTMotionInstanceNVBuilder<'a> {
        AccelerationStructureSRTMotionInstanceNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureSRTMotionInstanceNV.html) · Builder of [`AccelerationStructureSRTMotionInstanceNV`]"]
#[repr(transparent)]
pub struct AccelerationStructureSRTMotionInstanceNVBuilder<'a>(AccelerationStructureSRTMotionInstanceNV, std::marker::PhantomData<&'a ()>);
impl<'a> AccelerationStructureSRTMotionInstanceNVBuilder<'a> {
    #[inline]
    pub fn new() -> AccelerationStructureSRTMotionInstanceNVBuilder<'a> {
        AccelerationStructureSRTMotionInstanceNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn transform_t0(mut self, transform_t0: crate::extensions::nv_ray_tracing_motion_blur::SRTDataNV) -> Self {
        self.0.transform_t0 = transform_t0 as _;
        self
    }
    #[inline]
    pub fn transform_t1(mut self, transform_t1: crate::extensions::nv_ray_tracing_motion_blur::SRTDataNV) -> Self {
        self.0.transform_t1 = transform_t1 as _;
        self
    }
    #[inline]
    pub fn instance_custom_index(mut self, instance_custom_index: u32) -> Self {
        self.0.instance_custom_index_and_mask = crate::bits_copy!(self.0.instance_custom_index_and_mask, instance_custom_index, 0usize, 23usize);
        self
    }
    #[inline]
    pub fn mask(mut self, mask: u32) -> Self {
        self.0.instance_custom_index_and_mask = crate::bits_copy!(self.0.instance_custom_index_and_mask, mask, 24usize, 31usize);
        self
    }
    #[inline]
    pub fn instance_shader_binding_table_record_offset(mut self, instance_shader_binding_table_record_offset: u32) -> Self {
        self.0.instance_shader_binding_table_record_offset_and_flags = crate::bits_copy!(self.0.instance_shader_binding_table_record_offset_and_flags, instance_shader_binding_table_record_offset, 0usize, 23usize);
        self
    }
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::khr_acceleration_structure::GeometryInstanceFlagsKHR) -> Self {
        self.0.instance_shader_binding_table_record_offset_and_flags = crate::bits_copy!(self.0.instance_shader_binding_table_record_offset_and_flags, flags.bits(), 24usize, 31usize);
        self
    }
    #[inline]
    pub fn acceleration_structure_reference(mut self, acceleration_structure_reference: u64) -> Self {
        self.0.acceleration_structure_reference = acceleration_structure_reference as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> AccelerationStructureSRTMotionInstanceNV {
        self.0
    }
}
impl<'a> std::default::Default for AccelerationStructureSRTMotionInstanceNVBuilder<'a> {
    fn default() -> AccelerationStructureSRTMotionInstanceNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for AccelerationStructureSRTMotionInstanceNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for AccelerationStructureSRTMotionInstanceNVBuilder<'a> {
    type Target = AccelerationStructureSRTMotionInstanceNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for AccelerationStructureSRTMotionInstanceNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureMatrixMotionInstanceNV.html) · Structure"]
#[doc(alias = "VkAccelerationStructureMatrixMotionInstanceNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AccelerationStructureMatrixMotionInstanceNV {
    pub transform_t0: crate::extensions::khr_acceleration_structure::TransformMatrixKHR,
    pub transform_t1: crate::extensions::khr_acceleration_structure::TransformMatrixKHR,
    pub instance_custom_index_and_mask: u32,
    pub instance_shader_binding_table_record_offset_and_flags: u32,
    pub acceleration_structure_reference: u64,
}
impl Default for AccelerationStructureMatrixMotionInstanceNV {
    fn default() -> Self {
        Self { transform_t0: Default::default(), transform_t1: Default::default(), instance_custom_index_and_mask: Default::default(), instance_shader_binding_table_record_offset_and_flags: Default::default(), acceleration_structure_reference: Default::default() }
    }
}
impl std::fmt::Debug for AccelerationStructureMatrixMotionInstanceNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AccelerationStructureMatrixMotionInstanceNV").field("transform_t0", &self.transform_t0).field("transform_t1", &self.transform_t1).field("instance_custom_index_and_mask", &format!("{:#b}", &self.instance_custom_index_and_mask)).field("instance_shader_binding_table_record_offset_and_flags", &format!("{:#b}", &self.instance_shader_binding_table_record_offset_and_flags)).field("acceleration_structure_reference", &self.acceleration_structure_reference).finish()
    }
}
impl AccelerationStructureMatrixMotionInstanceNV {
    #[inline]
    pub fn into_builder<'a>(self) -> AccelerationStructureMatrixMotionInstanceNVBuilder<'a> {
        AccelerationStructureMatrixMotionInstanceNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureMatrixMotionInstanceNV.html) · Builder of [`AccelerationStructureMatrixMotionInstanceNV`]"]
#[repr(transparent)]
pub struct AccelerationStructureMatrixMotionInstanceNVBuilder<'a>(AccelerationStructureMatrixMotionInstanceNV, std::marker::PhantomData<&'a ()>);
impl<'a> AccelerationStructureMatrixMotionInstanceNVBuilder<'a> {
    #[inline]
    pub fn new() -> AccelerationStructureMatrixMotionInstanceNVBuilder<'a> {
        AccelerationStructureMatrixMotionInstanceNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn transform_t0(mut self, transform_t0: crate::extensions::khr_acceleration_structure::TransformMatrixKHR) -> Self {
        self.0.transform_t0 = transform_t0 as _;
        self
    }
    #[inline]
    pub fn transform_t1(mut self, transform_t1: crate::extensions::khr_acceleration_structure::TransformMatrixKHR) -> Self {
        self.0.transform_t1 = transform_t1 as _;
        self
    }
    #[inline]
    pub fn instance_custom_index(mut self, instance_custom_index: u32) -> Self {
        self.0.instance_custom_index_and_mask = crate::bits_copy!(self.0.instance_custom_index_and_mask, instance_custom_index, 0usize, 23usize);
        self
    }
    #[inline]
    pub fn mask(mut self, mask: u32) -> Self {
        self.0.instance_custom_index_and_mask = crate::bits_copy!(self.0.instance_custom_index_and_mask, mask, 24usize, 31usize);
        self
    }
    #[inline]
    pub fn instance_shader_binding_table_record_offset(mut self, instance_shader_binding_table_record_offset: u32) -> Self {
        self.0.instance_shader_binding_table_record_offset_and_flags = crate::bits_copy!(self.0.instance_shader_binding_table_record_offset_and_flags, instance_shader_binding_table_record_offset, 0usize, 23usize);
        self
    }
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::khr_acceleration_structure::GeometryInstanceFlagsKHR) -> Self {
        self.0.instance_shader_binding_table_record_offset_and_flags = crate::bits_copy!(self.0.instance_shader_binding_table_record_offset_and_flags, flags.bits(), 24usize, 31usize);
        self
    }
    #[inline]
    pub fn acceleration_structure_reference(mut self, acceleration_structure_reference: u64) -> Self {
        self.0.acceleration_structure_reference = acceleration_structure_reference as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> AccelerationStructureMatrixMotionInstanceNV {
        self.0
    }
}
impl<'a> std::default::Default for AccelerationStructureMatrixMotionInstanceNVBuilder<'a> {
    fn default() -> AccelerationStructureMatrixMotionInstanceNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for AccelerationStructureMatrixMotionInstanceNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for AccelerationStructureMatrixMotionInstanceNVBuilder<'a> {
    type Target = AccelerationStructureMatrixMotionInstanceNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for AccelerationStructureMatrixMotionInstanceNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureMotionInstanceDataNV.html) · Structure"]
#[doc(alias = "VkAccelerationStructureMotionInstanceDataNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub union AccelerationStructureMotionInstanceDataNV {
    pub static_instance: crate::extensions::khr_acceleration_structure::AccelerationStructureInstanceKHR,
    pub matrix_motion_instance: crate::extensions::nv_ray_tracing_motion_blur::AccelerationStructureMatrixMotionInstanceNV,
    pub srt_motion_instance: crate::extensions::nv_ray_tracing_motion_blur::AccelerationStructureSRTMotionInstanceNV,
}
impl Default for AccelerationStructureMotionInstanceDataNV {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for AccelerationStructureMotionInstanceDataNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AccelerationStructureMotionInstanceDataNV").finish()
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureMotionInstanceNV.html) · Structure"]
#[doc(alias = "VkAccelerationStructureMotionInstanceNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AccelerationStructureMotionInstanceNV {
    pub _type: crate::extensions::nv_ray_tracing_motion_blur::AccelerationStructureMotionInstanceTypeNV,
    pub flags: crate::extensions::nv_ray_tracing_motion_blur::AccelerationStructureMotionInstanceFlagsNV,
    pub data: crate::extensions::nv_ray_tracing_motion_blur::AccelerationStructureMotionInstanceDataNV,
}
impl Default for AccelerationStructureMotionInstanceNV {
    fn default() -> Self {
        Self { _type: Default::default(), flags: Default::default(), data: Default::default() }
    }
}
impl std::fmt::Debug for AccelerationStructureMotionInstanceNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AccelerationStructureMotionInstanceNV").field("_type", &self._type).field("flags", &self.flags).field("data", &self.data).finish()
    }
}
impl AccelerationStructureMotionInstanceNV {
    #[inline]
    pub fn into_builder<'a>(self) -> AccelerationStructureMotionInstanceNVBuilder<'a> {
        AccelerationStructureMotionInstanceNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureMotionInstanceNV.html) · Builder of [`AccelerationStructureMotionInstanceNV`]"]
#[repr(transparent)]
pub struct AccelerationStructureMotionInstanceNVBuilder<'a>(AccelerationStructureMotionInstanceNV, std::marker::PhantomData<&'a ()>);
impl<'a> AccelerationStructureMotionInstanceNVBuilder<'a> {
    #[inline]
    pub fn new() -> AccelerationStructureMotionInstanceNVBuilder<'a> {
        AccelerationStructureMotionInstanceNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn _type(mut self, _type: crate::extensions::nv_ray_tracing_motion_blur::AccelerationStructureMotionInstanceTypeNV) -> Self {
        self.0._type = _type as _;
        self
    }
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::nv_ray_tracing_motion_blur::AccelerationStructureMotionInstanceFlagsNV) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn data(mut self, data: crate::extensions::nv_ray_tracing_motion_blur::AccelerationStructureMotionInstanceDataNV) -> Self {
        self.0.data = data as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> AccelerationStructureMotionInstanceNV {
        self.0
    }
}
impl<'a> std::default::Default for AccelerationStructureMotionInstanceNVBuilder<'a> {
    fn default() -> AccelerationStructureMotionInstanceNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for AccelerationStructureMotionInstanceNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for AccelerationStructureMotionInstanceNVBuilder<'a> {
    type Target = AccelerationStructureMotionInstanceNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for AccelerationStructureMotionInstanceNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
