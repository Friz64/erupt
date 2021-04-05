#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_SHADER_UNUSED_NV")]
pub const SHADER_UNUSED_NV: u32 = 4294967295;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_RAY_TRACING_SPEC_VERSION")]
pub const NV_RAY_TRACING_SPEC_VERSION: u32 = 3;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_RAY_TRACING_EXTENSION_NAME")]
pub const NV_RAY_TRACING_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_NV_ray_tracing");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_COMPILE_DEFERRED_NV: *const std::os::raw::c_char = crate::cstr!("vkCompileDeferredNV");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_ACCELERATION_STRUCTURE_NV: *const std::os::raw::c_char = crate::cstr!("vkCreateAccelerationStructureNV");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_DESTROY_ACCELERATION_STRUCTURE_NV: *const std::os::raw::c_char = crate::cstr!("vkDestroyAccelerationStructureNV");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_NV: *const std::os::raw::c_char = crate::cstr!("vkGetAccelerationStructureMemoryRequirementsNV");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_BIND_ACCELERATION_STRUCTURE_MEMORY_NV: *const std::os::raw::c_char = crate::cstr!("vkBindAccelerationStructureMemoryNV");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_COPY_ACCELERATION_STRUCTURE_NV: *const std::os::raw::c_char = crate::cstr!("vkCmdCopyAccelerationStructureNV");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_WRITE_ACCELERATION_STRUCTURES_PROPERTIES_NV: *const std::os::raw::c_char = crate::cstr!("vkCmdWriteAccelerationStructuresPropertiesNV");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_BUILD_ACCELERATION_STRUCTURE_NV: *const std::os::raw::c_char = crate::cstr!("vkCmdBuildAccelerationStructureNV");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_TRACE_RAYS_NV: *const std::os::raw::c_char = crate::cstr!("vkCmdTraceRaysNV");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_ACCELERATION_STRUCTURE_HANDLE_NV: *const std::os::raw::c_char = crate::cstr!("vkGetAccelerationStructureHandleNV");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_RAY_TRACING_PIPELINES_NV: *const std::os::raw::c_char = crate::cstr!("vkCreateRayTracingPipelinesNV");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_RAY_TRACING_SHADER_GROUP_HANDLES_NV: *const std::os::raw::c_char = crate::cstr!("vkGetRayTracingShaderGroupHandlesNV");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeometryFlagsNV.html) · Alias"]
#[doc(alias = "VkGeometryFlagsNV")]
#[allow(non_camel_case_types)]
pub type GeometryFlagsNV = crate::extensions::khr_acceleration_structure::GeometryFlagsKHR;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeometryInstanceFlagsNV.html) · Alias"]
#[doc(alias = "VkGeometryInstanceFlagsNV")]
#[allow(non_camel_case_types)]
pub type GeometryInstanceFlagsNV = crate::extensions::khr_acceleration_structure::GeometryInstanceFlagsKHR;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBuildAccelerationStructureFlagsNV.html) · Alias"]
#[doc(alias = "VkBuildAccelerationStructureFlagsNV")]
#[allow(non_camel_case_types)]
pub type BuildAccelerationStructureFlagsNV = crate::extensions::khr_acceleration_structure::BuildAccelerationStructureFlagsKHR;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeometryFlagBitsNV.html) · Alias"]
#[doc(alias = "VkGeometryFlagBitsNV")]
#[allow(non_camel_case_types)]
pub type GeometryFlagBitsNV = crate::extensions::khr_acceleration_structure::GeometryFlagBitsKHR;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeometryInstanceFlagBitsNV.html) · Alias"]
#[doc(alias = "VkGeometryInstanceFlagBitsNV")]
#[allow(non_camel_case_types)]
pub type GeometryInstanceFlagBitsNV = crate::extensions::khr_acceleration_structure::GeometryInstanceFlagBitsKHR;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBuildAccelerationStructureFlagBitsNV.html) · Alias"]
#[doc(alias = "VkBuildAccelerationStructureFlagBitsNV")]
#[allow(non_camel_case_types)]
pub type BuildAccelerationStructureFlagBitsNV = crate::extensions::khr_acceleration_structure::BuildAccelerationStructureFlagBitsKHR;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCopyAccelerationStructureModeNV.html) · Alias"]
#[doc(alias = "VkCopyAccelerationStructureModeNV")]
#[allow(non_camel_case_types)]
pub type CopyAccelerationStructureModeNV = crate::extensions::khr_acceleration_structure::CopyAccelerationStructureModeKHR;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureTypeNV.html) · Alias"]
#[doc(alias = "VkAccelerationStructureTypeNV")]
#[allow(non_camel_case_types)]
pub type AccelerationStructureTypeNV = crate::extensions::khr_acceleration_structure::AccelerationStructureTypeKHR;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeometryTypeNV.html) · Alias"]
#[doc(alias = "VkGeometryTypeNV")]
#[allow(non_camel_case_types)]
pub type GeometryTypeNV = crate::extensions::khr_acceleration_structure::GeometryTypeKHR;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRayTracingShaderGroupTypeNV.html) · Alias"]
#[doc(alias = "VkRayTracingShaderGroupTypeNV")]
#[allow(non_camel_case_types)]
pub type RayTracingShaderGroupTypeNV = crate::extensions::khr_ray_tracing_pipeline::RayTracingShaderGroupTypeKHR;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAabbPositionsNV.html) · Alias"]
#[doc(alias = "VkAabbPositionsNV")]
#[allow(non_camel_case_types)]
pub type AabbPositionsNV = crate::extensions::khr_acceleration_structure::AabbPositionsKHR;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAabbPositionsNV.html) · Alias"]
#[doc(alias = "VkAabbPositionsNV")]
#[allow(non_camel_case_types)]
pub type AabbPositionsNVBuilder<'a> = crate::extensions::khr_acceleration_structure::AabbPositionsKHRBuilder<'a>;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkTransformMatrixNV.html) · Alias"]
#[doc(alias = "VkTransformMatrixNV")]
#[allow(non_camel_case_types)]
pub type TransformMatrixNV = crate::extensions::khr_acceleration_structure::TransformMatrixKHR;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkTransformMatrixNV.html) · Alias"]
#[doc(alias = "VkTransformMatrixNV")]
#[allow(non_camel_case_types)]
pub type TransformMatrixNVBuilder<'a> = crate::extensions::khr_acceleration_structure::TransformMatrixKHRBuilder<'a>;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureInstanceNV.html) · Alias"]
#[doc(alias = "VkAccelerationStructureInstanceNV")]
#[allow(non_camel_case_types)]
pub type AccelerationStructureInstanceNV = crate::extensions::khr_acceleration_structure::AccelerationStructureInstanceKHR;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureInstanceNV.html) · Alias"]
#[doc(alias = "VkAccelerationStructureInstanceNV")]
#[allow(non_camel_case_types)]
pub type AccelerationStructureInstanceNVBuilder<'a> = crate::extensions::khr_acceleration_structure::AccelerationStructureInstanceKHRBuilder<'a>;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetRayTracingShaderGroupHandlesNV.html) · Alias"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetRayTracingShaderGroupHandlesNV = crate::extensions::khr_ray_tracing_pipeline::PFN_vkGetRayTracingShaderGroupHandlesKHR;
crate::non_dispatchable_handle!(
    AccelerationStructureNV,
    ACCELERATION_STRUCTURE_NV,
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureNV.html) · Non-dispatchable Handle",
    "VkAccelerationStructureNV"
);
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureMemoryRequirementsTypeNV.html) · Enum"]
#[doc(alias = "VkAccelerationStructureMemoryRequirementsTypeNV")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct AccelerationStructureMemoryRequirementsTypeNV(pub i32);
impl std::fmt::Debug for AccelerationStructureMemoryRequirementsTypeNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::OBJECT_NV => "OBJECT_NV",
            &Self::BUILD_SCRATCH_NV => "BUILD_SCRATCH_NV",
            &Self::UPDATE_SCRATCH_NV => "UPDATE_SCRATCH_NV",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::nv_ray_tracing`]"]
impl AccelerationStructureMemoryRequirementsTypeNV {
    pub const OBJECT_NV: Self = Self(0);
    pub const BUILD_SCRATCH_NV: Self = Self(1);
    pub const UPDATE_SCRATCH_NV: Self = Self(2);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCompileDeferredNV.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCompileDeferredNV = unsafe extern "system" fn(device: crate::vk1_0::Device, pipeline: crate::vk1_0::Pipeline, shader: u32) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateAccelerationStructureNV.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateAccelerationStructureNV = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_create_info: *const crate::extensions::nv_ray_tracing::AccelerationStructureCreateInfoNV,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
    p_acceleration_structure: *mut crate::extensions::nv_ray_tracing::AccelerationStructureNV,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyAccelerationStructureNV.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyAccelerationStructureNV = unsafe extern "system" fn(device: crate::vk1_0::Device, acceleration_structure: crate::extensions::nv_ray_tracing::AccelerationStructureNV, p_allocator: *const crate::vk1_0::AllocationCallbacks) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetAccelerationStructureMemoryRequirementsNV.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetAccelerationStructureMemoryRequirementsNV = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_info: *const crate::extensions::nv_ray_tracing::AccelerationStructureMemoryRequirementsInfoNV,
    p_memory_requirements: *mut crate::extensions::khr_get_memory_requirements2::MemoryRequirements2KHR,
) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBindAccelerationStructureMemoryNV.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkBindAccelerationStructureMemoryNV = unsafe extern "system" fn(device: crate::vk1_0::Device, bind_info_count: u32, p_bind_infos: *const crate::extensions::nv_ray_tracing::BindAccelerationStructureMemoryInfoNV) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyAccelerationStructureNV.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyAccelerationStructureNV = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    dst: crate::extensions::nv_ray_tracing::AccelerationStructureNV,
    src: crate::extensions::nv_ray_tracing::AccelerationStructureNV,
    mode: crate::extensions::khr_acceleration_structure::CopyAccelerationStructureModeKHR,
) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdWriteAccelerationStructuresPropertiesNV.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdWriteAccelerationStructuresPropertiesNV = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    acceleration_structure_count: u32,
    p_acceleration_structures: *const crate::extensions::nv_ray_tracing::AccelerationStructureNV,
    query_type: crate::vk1_0::QueryType,
    query_pool: crate::vk1_0::QueryPool,
    first_query: u32,
) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBuildAccelerationStructureNV.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBuildAccelerationStructureNV = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    p_info: *const crate::extensions::nv_ray_tracing::AccelerationStructureInfoNV,
    instance_data: crate::vk1_0::Buffer,
    instance_offset: crate::vk1_0::DeviceSize,
    update: crate::vk1_0::Bool32,
    dst: crate::extensions::nv_ray_tracing::AccelerationStructureNV,
    src: crate::extensions::nv_ray_tracing::AccelerationStructureNV,
    scratch: crate::vk1_0::Buffer,
    scratch_offset: crate::vk1_0::DeviceSize,
) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdTraceRaysNV.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdTraceRaysNV = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    raygen_shader_binding_table_buffer: crate::vk1_0::Buffer,
    raygen_shader_binding_offset: crate::vk1_0::DeviceSize,
    miss_shader_binding_table_buffer: crate::vk1_0::Buffer,
    miss_shader_binding_offset: crate::vk1_0::DeviceSize,
    miss_shader_binding_stride: crate::vk1_0::DeviceSize,
    hit_shader_binding_table_buffer: crate::vk1_0::Buffer,
    hit_shader_binding_offset: crate::vk1_0::DeviceSize,
    hit_shader_binding_stride: crate::vk1_0::DeviceSize,
    callable_shader_binding_table_buffer: crate::vk1_0::Buffer,
    callable_shader_binding_offset: crate::vk1_0::DeviceSize,
    callable_shader_binding_stride: crate::vk1_0::DeviceSize,
    width: u32,
    height: u32,
    depth: u32,
) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetAccelerationStructureHandleNV.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetAccelerationStructureHandleNV =
    unsafe extern "system" fn(device: crate::vk1_0::Device, acceleration_structure: crate::extensions::nv_ray_tracing::AccelerationStructureNV, data_size: usize, p_data: *mut std::ffi::c_void) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateRayTracingPipelinesNV.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateRayTracingPipelinesNV = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    pipeline_cache: crate::vk1_0::PipelineCache,
    create_info_count: u32,
    p_create_infos: *const crate::extensions::nv_ray_tracing::RayTracingPipelineCreateInfoNV,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
    p_pipelines: *mut crate::vk1_0::Pipeline,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRayTracingShaderGroupCreateInfoNV.html) · Structure"]
#[doc(alias = "VkRayTracingShaderGroupCreateInfoNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RayTracingShaderGroupCreateInfoNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub _type: crate::extensions::khr_ray_tracing_pipeline::RayTracingShaderGroupTypeKHR,
    pub general_shader: u32,
    pub closest_hit_shader: u32,
    pub any_hit_shader: u32,
    pub intersection_shader: u32,
}
impl Default for RayTracingShaderGroupCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::RAY_TRACING_SHADER_GROUP_CREATE_INFO_NV,
            p_next: std::ptr::null(),
            _type: Default::default(),
            general_shader: Default::default(),
            closest_hit_shader: Default::default(),
            any_hit_shader: Default::default(),
            intersection_shader: Default::default(),
        }
    }
}
impl std::fmt::Debug for RayTracingShaderGroupCreateInfoNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RayTracingShaderGroupCreateInfoNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("_type", &self._type)
            .field("general_shader", &self.general_shader)
            .field("closest_hit_shader", &self.closest_hit_shader)
            .field("any_hit_shader", &self.any_hit_shader)
            .field("intersection_shader", &self.intersection_shader)
            .finish()
    }
}
impl RayTracingShaderGroupCreateInfoNV {
    #[inline]
    pub fn into_builder<'a>(self) -> RayTracingShaderGroupCreateInfoNVBuilder<'a> {
        RayTracingShaderGroupCreateInfoNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRayTracingShaderGroupCreateInfoNV.html) · Builder of [`RayTracingShaderGroupCreateInfoNV`]"]
#[repr(transparent)]
pub struct RayTracingShaderGroupCreateInfoNVBuilder<'a>(RayTracingShaderGroupCreateInfoNV, std::marker::PhantomData<&'a ()>);
impl<'a> RayTracingShaderGroupCreateInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> RayTracingShaderGroupCreateInfoNVBuilder<'a> {
        RayTracingShaderGroupCreateInfoNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn _type(mut self, _type: crate::extensions::khr_ray_tracing_pipeline::RayTracingShaderGroupTypeKHR) -> Self {
        self.0._type = _type as _;
        self
    }
    #[inline]
    pub fn general_shader(mut self, general_shader: u32) -> Self {
        self.0.general_shader = general_shader as _;
        self
    }
    #[inline]
    pub fn closest_hit_shader(mut self, closest_hit_shader: u32) -> Self {
        self.0.closest_hit_shader = closest_hit_shader as _;
        self
    }
    #[inline]
    pub fn any_hit_shader(mut self, any_hit_shader: u32) -> Self {
        self.0.any_hit_shader = any_hit_shader as _;
        self
    }
    #[inline]
    pub fn intersection_shader(mut self, intersection_shader: u32) -> Self {
        self.0.intersection_shader = intersection_shader as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> RayTracingShaderGroupCreateInfoNV {
        self.0
    }
}
impl<'a> std::default::Default for RayTracingShaderGroupCreateInfoNVBuilder<'a> {
    fn default() -> RayTracingShaderGroupCreateInfoNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for RayTracingShaderGroupCreateInfoNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for RayTracingShaderGroupCreateInfoNVBuilder<'a> {
    type Target = RayTracingShaderGroupCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for RayTracingShaderGroupCreateInfoNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<RayTracingShaderGroupCreateInfoNV> for RayTracingShaderGroupCreateInfoNVBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRayTracingPipelineCreateInfoNV.html) · Structure"]
#[doc(alias = "VkRayTracingPipelineCreateInfoNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RayTracingPipelineCreateInfoNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::vk1_0::PipelineCreateFlags,
    pub stage_count: u32,
    pub p_stages: *const crate::vk1_0::PipelineShaderStageCreateInfo,
    pub group_count: u32,
    pub p_groups: *const crate::extensions::nv_ray_tracing::RayTracingShaderGroupCreateInfoNV,
    pub max_recursion_depth: u32,
    pub layout: crate::vk1_0::PipelineLayout,
    pub base_pipeline_handle: crate::vk1_0::Pipeline,
    pub base_pipeline_index: i32,
}
impl Default for RayTracingPipelineCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::RAY_TRACING_PIPELINE_CREATE_INFO_NV,
            p_next: std::ptr::null(),
            flags: Default::default(),
            stage_count: Default::default(),
            p_stages: std::ptr::null(),
            group_count: Default::default(),
            p_groups: std::ptr::null(),
            max_recursion_depth: Default::default(),
            layout: Default::default(),
            base_pipeline_handle: Default::default(),
            base_pipeline_index: Default::default(),
        }
    }
}
impl std::fmt::Debug for RayTracingPipelineCreateInfoNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RayTracingPipelineCreateInfoNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("stage_count", &self.stage_count)
            .field("p_stages", &self.p_stages)
            .field("group_count", &self.group_count)
            .field("p_groups", &self.p_groups)
            .field("max_recursion_depth", &self.max_recursion_depth)
            .field("layout", &self.layout)
            .field("base_pipeline_handle", &self.base_pipeline_handle)
            .field("base_pipeline_index", &self.base_pipeline_index)
            .finish()
    }
}
impl RayTracingPipelineCreateInfoNV {
    #[inline]
    pub fn into_builder<'a>(self) -> RayTracingPipelineCreateInfoNVBuilder<'a> {
        RayTracingPipelineCreateInfoNVBuilder(self, std::marker::PhantomData)
    }
}
impl<'a> crate::ExtendableFromConst<'a, crate::extensions::ext_pipeline_creation_feedback::PipelineCreationFeedbackCreateInfoEXT> for RayTracingPipelineCreateInfoNVBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, crate::extensions::ext_pipeline_creation_feedback::PipelineCreationFeedbackCreateInfoEXTBuilder<'_>> for RayTracingPipelineCreateInfoNVBuilder<'a> {}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRayTracingPipelineCreateInfoNV.html) · Builder of [`RayTracingPipelineCreateInfoNV`]"]
#[repr(transparent)]
pub struct RayTracingPipelineCreateInfoNVBuilder<'a>(RayTracingPipelineCreateInfoNV, std::marker::PhantomData<&'a ()>);
impl<'a> RayTracingPipelineCreateInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> RayTracingPipelineCreateInfoNVBuilder<'a> {
        RayTracingPipelineCreateInfoNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_0::PipelineCreateFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn stages(mut self, stages: &'a [impl crate::Repr<crate::vk1_0::PipelineShaderStageCreateInfo>]) -> Self {
        self.0.p_stages = stages.as_ptr() as _;
        self.0.stage_count = stages.len() as _;
        self
    }
    #[inline]
    pub fn groups(mut self, groups: &'a [impl crate::Repr<crate::extensions::nv_ray_tracing::RayTracingShaderGroupCreateInfoNV>]) -> Self {
        self.0.p_groups = groups.as_ptr() as _;
        self.0.group_count = groups.len() as _;
        self
    }
    #[inline]
    pub fn max_recursion_depth(mut self, max_recursion_depth: u32) -> Self {
        self.0.max_recursion_depth = max_recursion_depth as _;
        self
    }
    #[inline]
    pub fn layout(mut self, layout: crate::vk1_0::PipelineLayout) -> Self {
        self.0.layout = layout as _;
        self
    }
    #[inline]
    pub fn base_pipeline_handle(mut self, base_pipeline_handle: crate::vk1_0::Pipeline) -> Self {
        self.0.base_pipeline_handle = base_pipeline_handle as _;
        self
    }
    #[inline]
    pub fn base_pipeline_index(mut self, base_pipeline_index: i32) -> Self {
        self.0.base_pipeline_index = base_pipeline_index as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> RayTracingPipelineCreateInfoNV {
        self.0
    }
}
impl<'a> std::default::Default for RayTracingPipelineCreateInfoNVBuilder<'a> {
    fn default() -> RayTracingPipelineCreateInfoNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for RayTracingPipelineCreateInfoNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for RayTracingPipelineCreateInfoNVBuilder<'a> {
    type Target = RayTracingPipelineCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for RayTracingPipelineCreateInfoNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<RayTracingPipelineCreateInfoNV> for RayTracingPipelineCreateInfoNVBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeometryTrianglesNV.html) · Structure"]
#[doc(alias = "VkGeometryTrianglesNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GeometryTrianglesNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub vertex_data: crate::vk1_0::Buffer,
    pub vertex_offset: crate::vk1_0::DeviceSize,
    pub vertex_count: u32,
    pub vertex_stride: crate::vk1_0::DeviceSize,
    pub vertex_format: crate::vk1_0::Format,
    pub index_data: crate::vk1_0::Buffer,
    pub index_offset: crate::vk1_0::DeviceSize,
    pub index_count: u32,
    pub index_type: crate::vk1_0::IndexType,
    pub transform_data: crate::vk1_0::Buffer,
    pub transform_offset: crate::vk1_0::DeviceSize,
}
impl Default for GeometryTrianglesNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::GEOMETRY_TRIANGLES_NV,
            p_next: std::ptr::null(),
            vertex_data: Default::default(),
            vertex_offset: Default::default(),
            vertex_count: Default::default(),
            vertex_stride: Default::default(),
            vertex_format: Default::default(),
            index_data: Default::default(),
            index_offset: Default::default(),
            index_count: Default::default(),
            index_type: Default::default(),
            transform_data: Default::default(),
            transform_offset: Default::default(),
        }
    }
}
impl std::fmt::Debug for GeometryTrianglesNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("GeometryTrianglesNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("vertex_data", &self.vertex_data)
            .field("vertex_offset", &self.vertex_offset)
            .field("vertex_count", &self.vertex_count)
            .field("vertex_stride", &self.vertex_stride)
            .field("vertex_format", &self.vertex_format)
            .field("index_data", &self.index_data)
            .field("index_offset", &self.index_offset)
            .field("index_count", &self.index_count)
            .field("index_type", &self.index_type)
            .field("transform_data", &self.transform_data)
            .field("transform_offset", &self.transform_offset)
            .finish()
    }
}
impl GeometryTrianglesNV {
    #[inline]
    pub fn into_builder<'a>(self) -> GeometryTrianglesNVBuilder<'a> {
        GeometryTrianglesNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeometryTrianglesNV.html) · Builder of [`GeometryTrianglesNV`]"]
#[repr(transparent)]
pub struct GeometryTrianglesNVBuilder<'a>(GeometryTrianglesNV, std::marker::PhantomData<&'a ()>);
impl<'a> GeometryTrianglesNVBuilder<'a> {
    #[inline]
    pub fn new() -> GeometryTrianglesNVBuilder<'a> {
        GeometryTrianglesNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn vertex_data(mut self, vertex_data: crate::vk1_0::Buffer) -> Self {
        self.0.vertex_data = vertex_data as _;
        self
    }
    #[inline]
    pub fn vertex_offset(mut self, vertex_offset: crate::vk1_0::DeviceSize) -> Self {
        self.0.vertex_offset = vertex_offset as _;
        self
    }
    #[inline]
    pub fn vertex_count(mut self, vertex_count: u32) -> Self {
        self.0.vertex_count = vertex_count as _;
        self
    }
    #[inline]
    pub fn vertex_stride(mut self, vertex_stride: crate::vk1_0::DeviceSize) -> Self {
        self.0.vertex_stride = vertex_stride as _;
        self
    }
    #[inline]
    pub fn vertex_format(mut self, vertex_format: crate::vk1_0::Format) -> Self {
        self.0.vertex_format = vertex_format as _;
        self
    }
    #[inline]
    pub fn index_data(mut self, index_data: crate::vk1_0::Buffer) -> Self {
        self.0.index_data = index_data as _;
        self
    }
    #[inline]
    pub fn index_offset(mut self, index_offset: crate::vk1_0::DeviceSize) -> Self {
        self.0.index_offset = index_offset as _;
        self
    }
    #[inline]
    pub fn index_count(mut self, index_count: u32) -> Self {
        self.0.index_count = index_count as _;
        self
    }
    #[inline]
    pub fn index_type(mut self, index_type: crate::vk1_0::IndexType) -> Self {
        self.0.index_type = index_type as _;
        self
    }
    #[inline]
    pub fn transform_data(mut self, transform_data: crate::vk1_0::Buffer) -> Self {
        self.0.transform_data = transform_data as _;
        self
    }
    #[inline]
    pub fn transform_offset(mut self, transform_offset: crate::vk1_0::DeviceSize) -> Self {
        self.0.transform_offset = transform_offset as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> GeometryTrianglesNV {
        self.0
    }
}
impl<'a> std::default::Default for GeometryTrianglesNVBuilder<'a> {
    fn default() -> GeometryTrianglesNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for GeometryTrianglesNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for GeometryTrianglesNVBuilder<'a> {
    type Target = GeometryTrianglesNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for GeometryTrianglesNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<GeometryTrianglesNV> for GeometryTrianglesNVBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeometryAABBNV.html) · Structure"]
#[doc(alias = "VkGeometryAABBNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GeometryAABBNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub aabb_data: crate::vk1_0::Buffer,
    pub num_aab_bs: u32,
    pub stride: u32,
    pub offset: crate::vk1_0::DeviceSize,
}
impl Default for GeometryAABBNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::GEOMETRY_AABB_NV,
            p_next: std::ptr::null(),
            aabb_data: Default::default(),
            num_aab_bs: Default::default(),
            stride: Default::default(),
            offset: Default::default(),
        }
    }
}
impl std::fmt::Debug for GeometryAABBNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("GeometryAABBNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("aabb_data", &self.aabb_data)
            .field("num_aab_bs", &self.num_aab_bs)
            .field("stride", &self.stride)
            .field("offset", &self.offset)
            .finish()
    }
}
impl GeometryAABBNV {
    #[inline]
    pub fn into_builder<'a>(self) -> GeometryAABBNVBuilder<'a> {
        GeometryAABBNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeometryAABBNV.html) · Builder of [`GeometryAABBNV`]"]
#[repr(transparent)]
pub struct GeometryAABBNVBuilder<'a>(GeometryAABBNV, std::marker::PhantomData<&'a ()>);
impl<'a> GeometryAABBNVBuilder<'a> {
    #[inline]
    pub fn new() -> GeometryAABBNVBuilder<'a> {
        GeometryAABBNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn aabb_data(mut self, aabb_data: crate::vk1_0::Buffer) -> Self {
        self.0.aabb_data = aabb_data as _;
        self
    }
    #[inline]
    pub fn num_aab_bs(mut self, num_aab_bs: u32) -> Self {
        self.0.num_aab_bs = num_aab_bs as _;
        self
    }
    #[inline]
    pub fn stride(mut self, stride: u32) -> Self {
        self.0.stride = stride as _;
        self
    }
    #[inline]
    pub fn offset(mut self, offset: crate::vk1_0::DeviceSize) -> Self {
        self.0.offset = offset as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> GeometryAABBNV {
        self.0
    }
}
impl<'a> std::default::Default for GeometryAABBNVBuilder<'a> {
    fn default() -> GeometryAABBNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for GeometryAABBNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for GeometryAABBNVBuilder<'a> {
    type Target = GeometryAABBNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for GeometryAABBNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<GeometryAABBNV> for GeometryAABBNVBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeometryDataNV.html) · Structure"]
#[doc(alias = "VkGeometryDataNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GeometryDataNV {
    pub triangles: crate::extensions::nv_ray_tracing::GeometryTrianglesNV,
    pub aabbs: crate::extensions::nv_ray_tracing::GeometryAABBNV,
}
impl Default for GeometryDataNV {
    fn default() -> Self {
        Self {
            triangles: Default::default(),
            aabbs: Default::default(),
        }
    }
}
impl std::fmt::Debug for GeometryDataNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("GeometryDataNV").field("triangles", &self.triangles).field("aabbs", &self.aabbs).finish()
    }
}
impl GeometryDataNV {
    #[inline]
    pub fn into_builder<'a>(self) -> GeometryDataNVBuilder<'a> {
        GeometryDataNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeometryDataNV.html) · Builder of [`GeometryDataNV`]"]
#[repr(transparent)]
pub struct GeometryDataNVBuilder<'a>(GeometryDataNV, std::marker::PhantomData<&'a ()>);
impl<'a> GeometryDataNVBuilder<'a> {
    #[inline]
    pub fn new() -> GeometryDataNVBuilder<'a> {
        GeometryDataNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn triangles(mut self, triangles: crate::extensions::nv_ray_tracing::GeometryTrianglesNV) -> Self {
        self.0.triangles = triangles as _;
        self
    }
    #[inline]
    pub fn aabbs(mut self, aabbs: crate::extensions::nv_ray_tracing::GeometryAABBNV) -> Self {
        self.0.aabbs = aabbs as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> GeometryDataNV {
        self.0
    }
}
impl<'a> std::default::Default for GeometryDataNVBuilder<'a> {
    fn default() -> GeometryDataNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for GeometryDataNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for GeometryDataNVBuilder<'a> {
    type Target = GeometryDataNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for GeometryDataNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<GeometryDataNV> for GeometryDataNVBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeometryNV.html) · Structure"]
#[doc(alias = "VkGeometryNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GeometryNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub geometry_type: crate::extensions::khr_acceleration_structure::GeometryTypeKHR,
    pub geometry: crate::extensions::nv_ray_tracing::GeometryDataNV,
    pub flags: crate::extensions::khr_acceleration_structure::GeometryFlagsKHR,
}
impl Default for GeometryNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::GEOMETRY_NV,
            p_next: std::ptr::null(),
            geometry_type: Default::default(),
            geometry: Default::default(),
            flags: Default::default(),
        }
    }
}
impl std::fmt::Debug for GeometryNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("GeometryNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("geometry_type", &self.geometry_type)
            .field("geometry", &self.geometry)
            .field("flags", &self.flags)
            .finish()
    }
}
impl GeometryNV {
    #[inline]
    pub fn into_builder<'a>(self) -> GeometryNVBuilder<'a> {
        GeometryNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeometryNV.html) · Builder of [`GeometryNV`]"]
#[repr(transparent)]
pub struct GeometryNVBuilder<'a>(GeometryNV, std::marker::PhantomData<&'a ()>);
impl<'a> GeometryNVBuilder<'a> {
    #[inline]
    pub fn new() -> GeometryNVBuilder<'a> {
        GeometryNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn geometry_type(mut self, geometry_type: crate::extensions::khr_acceleration_structure::GeometryTypeKHR) -> Self {
        self.0.geometry_type = geometry_type as _;
        self
    }
    #[inline]
    pub fn geometry(mut self, geometry: crate::extensions::nv_ray_tracing::GeometryDataNV) -> Self {
        self.0.geometry = geometry as _;
        self
    }
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::khr_acceleration_structure::GeometryFlagsKHR) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> GeometryNV {
        self.0
    }
}
impl<'a> std::default::Default for GeometryNVBuilder<'a> {
    fn default() -> GeometryNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for GeometryNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for GeometryNVBuilder<'a> {
    type Target = GeometryNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for GeometryNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<GeometryNV> for GeometryNVBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureInfoNV.html) · Structure"]
#[doc(alias = "VkAccelerationStructureInfoNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AccelerationStructureInfoNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub _type: crate::extensions::nv_ray_tracing::AccelerationStructureTypeNV,
    pub flags: crate::extensions::nv_ray_tracing::BuildAccelerationStructureFlagsNV,
    pub instance_count: u32,
    pub geometry_count: u32,
    pub p_geometries: *const crate::extensions::nv_ray_tracing::GeometryNV,
}
impl Default for AccelerationStructureInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::ACCELERATION_STRUCTURE_INFO_NV,
            p_next: std::ptr::null(),
            _type: Default::default(),
            flags: Default::default(),
            instance_count: Default::default(),
            geometry_count: Default::default(),
            p_geometries: std::ptr::null(),
        }
    }
}
impl std::fmt::Debug for AccelerationStructureInfoNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AccelerationStructureInfoNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("_type", &self._type)
            .field("flags", &self.flags)
            .field("instance_count", &self.instance_count)
            .field("geometry_count", &self.geometry_count)
            .field("p_geometries", &self.p_geometries)
            .finish()
    }
}
impl AccelerationStructureInfoNV {
    #[inline]
    pub fn into_builder<'a>(self) -> AccelerationStructureInfoNVBuilder<'a> {
        AccelerationStructureInfoNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureInfoNV.html) · Builder of [`AccelerationStructureInfoNV`]"]
#[repr(transparent)]
pub struct AccelerationStructureInfoNVBuilder<'a>(AccelerationStructureInfoNV, std::marker::PhantomData<&'a ()>);
impl<'a> AccelerationStructureInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> AccelerationStructureInfoNVBuilder<'a> {
        AccelerationStructureInfoNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn _type(mut self, _type: crate::extensions::nv_ray_tracing::AccelerationStructureTypeNV) -> Self {
        self.0._type = _type as _;
        self
    }
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::nv_ray_tracing::BuildAccelerationStructureFlagsNV) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn instance_count(mut self, instance_count: u32) -> Self {
        self.0.instance_count = instance_count as _;
        self
    }
    #[inline]
    pub fn geometries(mut self, geometries: &'a [impl crate::Repr<crate::extensions::nv_ray_tracing::GeometryNV>]) -> Self {
        self.0.p_geometries = geometries.as_ptr() as _;
        self.0.geometry_count = geometries.len() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> AccelerationStructureInfoNV {
        self.0
    }
}
impl<'a> std::default::Default for AccelerationStructureInfoNVBuilder<'a> {
    fn default() -> AccelerationStructureInfoNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for AccelerationStructureInfoNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for AccelerationStructureInfoNVBuilder<'a> {
    type Target = AccelerationStructureInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for AccelerationStructureInfoNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<AccelerationStructureInfoNV> for AccelerationStructureInfoNVBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureCreateInfoNV.html) · Structure"]
#[doc(alias = "VkAccelerationStructureCreateInfoNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AccelerationStructureCreateInfoNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub compacted_size: crate::vk1_0::DeviceSize,
    pub info: crate::extensions::nv_ray_tracing::AccelerationStructureInfoNV,
}
impl Default for AccelerationStructureCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::ACCELERATION_STRUCTURE_CREATE_INFO_NV,
            p_next: std::ptr::null(),
            compacted_size: Default::default(),
            info: Default::default(),
        }
    }
}
impl std::fmt::Debug for AccelerationStructureCreateInfoNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AccelerationStructureCreateInfoNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("compacted_size", &self.compacted_size)
            .field("info", &self.info)
            .finish()
    }
}
impl AccelerationStructureCreateInfoNV {
    #[inline]
    pub fn into_builder<'a>(self) -> AccelerationStructureCreateInfoNVBuilder<'a> {
        AccelerationStructureCreateInfoNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureCreateInfoNV.html) · Builder of [`AccelerationStructureCreateInfoNV`]"]
#[repr(transparent)]
pub struct AccelerationStructureCreateInfoNVBuilder<'a>(AccelerationStructureCreateInfoNV, std::marker::PhantomData<&'a ()>);
impl<'a> AccelerationStructureCreateInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> AccelerationStructureCreateInfoNVBuilder<'a> {
        AccelerationStructureCreateInfoNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn compacted_size(mut self, compacted_size: crate::vk1_0::DeviceSize) -> Self {
        self.0.compacted_size = compacted_size as _;
        self
    }
    #[inline]
    pub fn info(mut self, info: crate::extensions::nv_ray_tracing::AccelerationStructureInfoNV) -> Self {
        self.0.info = info as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> AccelerationStructureCreateInfoNV {
        self.0
    }
}
impl<'a> std::default::Default for AccelerationStructureCreateInfoNVBuilder<'a> {
    fn default() -> AccelerationStructureCreateInfoNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for AccelerationStructureCreateInfoNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for AccelerationStructureCreateInfoNVBuilder<'a> {
    type Target = AccelerationStructureCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for AccelerationStructureCreateInfoNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<AccelerationStructureCreateInfoNV> for AccelerationStructureCreateInfoNVBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindAccelerationStructureMemoryInfoNV.html) · Structure"]
#[doc(alias = "VkBindAccelerationStructureMemoryInfoNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BindAccelerationStructureMemoryInfoNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub acceleration_structure: crate::extensions::nv_ray_tracing::AccelerationStructureNV,
    pub memory: crate::vk1_0::DeviceMemory,
    pub memory_offset: crate::vk1_0::DeviceSize,
    pub device_index_count: u32,
    pub p_device_indices: *const u32,
}
impl Default for BindAccelerationStructureMemoryInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::BIND_ACCELERATION_STRUCTURE_MEMORY_INFO_NV,
            p_next: std::ptr::null(),
            acceleration_structure: Default::default(),
            memory: Default::default(),
            memory_offset: Default::default(),
            device_index_count: Default::default(),
            p_device_indices: std::ptr::null(),
        }
    }
}
impl std::fmt::Debug for BindAccelerationStructureMemoryInfoNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BindAccelerationStructureMemoryInfoNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("acceleration_structure", &self.acceleration_structure)
            .field("memory", &self.memory)
            .field("memory_offset", &self.memory_offset)
            .field("device_index_count", &self.device_index_count)
            .field("p_device_indices", &self.p_device_indices)
            .finish()
    }
}
impl BindAccelerationStructureMemoryInfoNV {
    #[inline]
    pub fn into_builder<'a>(self) -> BindAccelerationStructureMemoryInfoNVBuilder<'a> {
        BindAccelerationStructureMemoryInfoNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindAccelerationStructureMemoryInfoNV.html) · Builder of [`BindAccelerationStructureMemoryInfoNV`]"]
#[repr(transparent)]
pub struct BindAccelerationStructureMemoryInfoNVBuilder<'a>(BindAccelerationStructureMemoryInfoNV, std::marker::PhantomData<&'a ()>);
impl<'a> BindAccelerationStructureMemoryInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> BindAccelerationStructureMemoryInfoNVBuilder<'a> {
        BindAccelerationStructureMemoryInfoNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn acceleration_structure(mut self, acceleration_structure: crate::extensions::nv_ray_tracing::AccelerationStructureNV) -> Self {
        self.0.acceleration_structure = acceleration_structure as _;
        self
    }
    #[inline]
    pub fn memory(mut self, memory: crate::vk1_0::DeviceMemory) -> Self {
        self.0.memory = memory as _;
        self
    }
    #[inline]
    pub fn memory_offset(mut self, memory_offset: crate::vk1_0::DeviceSize) -> Self {
        self.0.memory_offset = memory_offset as _;
        self
    }
    #[inline]
    pub fn device_indices(mut self, device_indices: &'a [u32]) -> Self {
        self.0.p_device_indices = device_indices.as_ptr() as _;
        self.0.device_index_count = device_indices.len() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> BindAccelerationStructureMemoryInfoNV {
        self.0
    }
}
impl<'a> std::default::Default for BindAccelerationStructureMemoryInfoNVBuilder<'a> {
    fn default() -> BindAccelerationStructureMemoryInfoNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for BindAccelerationStructureMemoryInfoNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for BindAccelerationStructureMemoryInfoNVBuilder<'a> {
    type Target = BindAccelerationStructureMemoryInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for BindAccelerationStructureMemoryInfoNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<BindAccelerationStructureMemoryInfoNV> for BindAccelerationStructureMemoryInfoNVBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkWriteDescriptorSetAccelerationStructureNV.html) · Structure"]
#[doc(alias = "VkWriteDescriptorSetAccelerationStructureNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WriteDescriptorSetAccelerationStructureNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub acceleration_structure_count: u32,
    pub p_acceleration_structures: *const crate::extensions::nv_ray_tracing::AccelerationStructureNV,
}
impl Default for WriteDescriptorSetAccelerationStructureNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_NV,
            p_next: std::ptr::null(),
            acceleration_structure_count: Default::default(),
            p_acceleration_structures: std::ptr::null(),
        }
    }
}
impl std::fmt::Debug for WriteDescriptorSetAccelerationStructureNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("WriteDescriptorSetAccelerationStructureNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("acceleration_structure_count", &self.acceleration_structure_count)
            .field("p_acceleration_structures", &self.p_acceleration_structures)
            .finish()
    }
}
impl WriteDescriptorSetAccelerationStructureNV {
    #[inline]
    pub fn into_builder<'a>(self) -> WriteDescriptorSetAccelerationStructureNVBuilder<'a> {
        WriteDescriptorSetAccelerationStructureNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkWriteDescriptorSetAccelerationStructureNV.html) · Builder of [`WriteDescriptorSetAccelerationStructureNV`]"]
#[repr(transparent)]
pub struct WriteDescriptorSetAccelerationStructureNVBuilder<'a>(WriteDescriptorSetAccelerationStructureNV, std::marker::PhantomData<&'a ()>);
impl<'a> WriteDescriptorSetAccelerationStructureNVBuilder<'a> {
    #[inline]
    pub fn new() -> WriteDescriptorSetAccelerationStructureNVBuilder<'a> {
        WriteDescriptorSetAccelerationStructureNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn acceleration_structures(mut self, acceleration_structures: &'a [crate::extensions::nv_ray_tracing::AccelerationStructureNV]) -> Self {
        self.0.p_acceleration_structures = acceleration_structures.as_ptr() as _;
        self.0.acceleration_structure_count = acceleration_structures.len() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> WriteDescriptorSetAccelerationStructureNV {
        self.0
    }
}
impl<'a> std::default::Default for WriteDescriptorSetAccelerationStructureNVBuilder<'a> {
    fn default() -> WriteDescriptorSetAccelerationStructureNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for WriteDescriptorSetAccelerationStructureNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for WriteDescriptorSetAccelerationStructureNVBuilder<'a> {
    type Target = WriteDescriptorSetAccelerationStructureNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for WriteDescriptorSetAccelerationStructureNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<WriteDescriptorSetAccelerationStructureNV> for WriteDescriptorSetAccelerationStructureNVBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureMemoryRequirementsInfoNV.html) · Structure"]
#[doc(alias = "VkAccelerationStructureMemoryRequirementsInfoNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AccelerationStructureMemoryRequirementsInfoNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub _type: crate::extensions::nv_ray_tracing::AccelerationStructureMemoryRequirementsTypeNV,
    pub acceleration_structure: crate::extensions::nv_ray_tracing::AccelerationStructureNV,
}
impl Default for AccelerationStructureMemoryRequirementsInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO_NV,
            p_next: std::ptr::null(),
            _type: Default::default(),
            acceleration_structure: Default::default(),
        }
    }
}
impl std::fmt::Debug for AccelerationStructureMemoryRequirementsInfoNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AccelerationStructureMemoryRequirementsInfoNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("_type", &self._type)
            .field("acceleration_structure", &self.acceleration_structure)
            .finish()
    }
}
impl AccelerationStructureMemoryRequirementsInfoNV {
    #[inline]
    pub fn into_builder<'a>(self) -> AccelerationStructureMemoryRequirementsInfoNVBuilder<'a> {
        AccelerationStructureMemoryRequirementsInfoNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureMemoryRequirementsInfoNV.html) · Builder of [`AccelerationStructureMemoryRequirementsInfoNV`]"]
#[repr(transparent)]
pub struct AccelerationStructureMemoryRequirementsInfoNVBuilder<'a>(AccelerationStructureMemoryRequirementsInfoNV, std::marker::PhantomData<&'a ()>);
impl<'a> AccelerationStructureMemoryRequirementsInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> AccelerationStructureMemoryRequirementsInfoNVBuilder<'a> {
        AccelerationStructureMemoryRequirementsInfoNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn _type(mut self, _type: crate::extensions::nv_ray_tracing::AccelerationStructureMemoryRequirementsTypeNV) -> Self {
        self.0._type = _type as _;
        self
    }
    #[inline]
    pub fn acceleration_structure(mut self, acceleration_structure: crate::extensions::nv_ray_tracing::AccelerationStructureNV) -> Self {
        self.0.acceleration_structure = acceleration_structure as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> AccelerationStructureMemoryRequirementsInfoNV {
        self.0
    }
}
impl<'a> std::default::Default for AccelerationStructureMemoryRequirementsInfoNVBuilder<'a> {
    fn default() -> AccelerationStructureMemoryRequirementsInfoNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for AccelerationStructureMemoryRequirementsInfoNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for AccelerationStructureMemoryRequirementsInfoNVBuilder<'a> {
    type Target = AccelerationStructureMemoryRequirementsInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for AccelerationStructureMemoryRequirementsInfoNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<AccelerationStructureMemoryRequirementsInfoNV> for AccelerationStructureMemoryRequirementsInfoNVBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceRayTracingPropertiesNV.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceRayTracingPropertiesNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceRayTracingPropertiesNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub shader_group_handle_size: u32,
    pub max_recursion_depth: u32,
    pub max_shader_group_stride: u32,
    pub shader_group_base_alignment: u32,
    pub max_geometry_count: u64,
    pub max_instance_count: u64,
    pub max_triangle_count: u64,
    pub max_descriptor_set_acceleration_structures: u32,
}
impl Default for PhysicalDeviceRayTracingPropertiesNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_RAY_TRACING_PROPERTIES_NV,
            p_next: std::ptr::null_mut(),
            shader_group_handle_size: Default::default(),
            max_recursion_depth: Default::default(),
            max_shader_group_stride: Default::default(),
            shader_group_base_alignment: Default::default(),
            max_geometry_count: Default::default(),
            max_instance_count: Default::default(),
            max_triangle_count: Default::default(),
            max_descriptor_set_acceleration_structures: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceRayTracingPropertiesNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceRayTracingPropertiesNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("shader_group_handle_size", &self.shader_group_handle_size)
            .field("max_recursion_depth", &self.max_recursion_depth)
            .field("max_shader_group_stride", &self.max_shader_group_stride)
            .field("shader_group_base_alignment", &self.shader_group_base_alignment)
            .field("max_geometry_count", &self.max_geometry_count)
            .field("max_instance_count", &self.max_instance_count)
            .field("max_triangle_count", &self.max_triangle_count)
            .field("max_descriptor_set_acceleration_structures", &self.max_descriptor_set_acceleration_structures)
            .finish()
    }
}
impl PhysicalDeviceRayTracingPropertiesNV {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceRayTracingPropertiesNVBuilder<'a> {
        PhysicalDeviceRayTracingPropertiesNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceRayTracingPropertiesNV.html) · Builder of [`PhysicalDeviceRayTracingPropertiesNV`]"]
#[repr(transparent)]
pub struct PhysicalDeviceRayTracingPropertiesNVBuilder<'a>(PhysicalDeviceRayTracingPropertiesNV, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceRayTracingPropertiesNVBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceRayTracingPropertiesNVBuilder<'a> {
        PhysicalDeviceRayTracingPropertiesNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn shader_group_handle_size(mut self, shader_group_handle_size: u32) -> Self {
        self.0.shader_group_handle_size = shader_group_handle_size as _;
        self
    }
    #[inline]
    pub fn max_recursion_depth(mut self, max_recursion_depth: u32) -> Self {
        self.0.max_recursion_depth = max_recursion_depth as _;
        self
    }
    #[inline]
    pub fn max_shader_group_stride(mut self, max_shader_group_stride: u32) -> Self {
        self.0.max_shader_group_stride = max_shader_group_stride as _;
        self
    }
    #[inline]
    pub fn shader_group_base_alignment(mut self, shader_group_base_alignment: u32) -> Self {
        self.0.shader_group_base_alignment = shader_group_base_alignment as _;
        self
    }
    #[inline]
    pub fn max_geometry_count(mut self, max_geometry_count: u64) -> Self {
        self.0.max_geometry_count = max_geometry_count as _;
        self
    }
    #[inline]
    pub fn max_instance_count(mut self, max_instance_count: u64) -> Self {
        self.0.max_instance_count = max_instance_count as _;
        self
    }
    #[inline]
    pub fn max_triangle_count(mut self, max_triangle_count: u64) -> Self {
        self.0.max_triangle_count = max_triangle_count as _;
        self
    }
    #[inline]
    pub fn max_descriptor_set_acceleration_structures(mut self, max_descriptor_set_acceleration_structures: u32) -> Self {
        self.0.max_descriptor_set_acceleration_structures = max_descriptor_set_acceleration_structures as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceRayTracingPropertiesNV {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceRayTracingPropertiesNVBuilder<'a> {
    fn default() -> PhysicalDeviceRayTracingPropertiesNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceRayTracingPropertiesNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceRayTracingPropertiesNVBuilder<'a> {
    type Target = PhysicalDeviceRayTracingPropertiesNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceRayTracingPropertiesNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<PhysicalDeviceRayTracingPropertiesNV> for PhysicalDeviceRayTracingPropertiesNVBuilder<'_> {}
#[doc = "Provided by [`crate::extensions::nv_ray_tracing`]"]
impl crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCompileDeferredNV.html) · Function"]
    #[doc(alias = "vkCompileDeferredNV")]
    pub unsafe fn compile_deferred_nv(&self, pipeline: crate::vk1_0::Pipeline, shader: u32) -> crate::utils::VulkanResult<()> {
        let _function = self.compile_deferred_nv.expect("`compile_deferred_nv` is not loaded");
        let _return = _function(self.handle, pipeline as _, shader as _);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateAccelerationStructureNV.html) · Function"]
    #[doc(alias = "vkCreateAccelerationStructureNV")]
    pub unsafe fn create_acceleration_structure_nv(
        &self,
        create_info: &crate::extensions::nv_ray_tracing::AccelerationStructureCreateInfoNV,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
    ) -> crate::utils::VulkanResult<crate::extensions::nv_ray_tracing::AccelerationStructureNV> {
        let _function = self.create_acceleration_structure_nv.expect("`create_acceleration_structure_nv` is not loaded");
        let mut acceleration_structure = Default::default();
        let _return = _function(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut acceleration_structure,
        );
        crate::utils::VulkanResult::new(_return, acceleration_structure)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyAccelerationStructureNV.html) · Function"]
    #[doc(alias = "vkDestroyAccelerationStructureNV")]
    pub unsafe fn destroy_acceleration_structure_nv(&self, acceleration_structure: Option<crate::extensions::nv_ray_tracing::AccelerationStructureNV>, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> () {
        let _function = self.destroy_acceleration_structure_nv.expect("`destroy_acceleration_structure_nv` is not loaded");
        let _return = _function(
            self.handle,
            match acceleration_structure {
                Some(v) => v,
                None => Default::default(),
            },
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetAccelerationStructureMemoryRequirementsNV.html) · Function"]
    #[doc(alias = "vkGetAccelerationStructureMemoryRequirementsNV")]
    pub unsafe fn get_acceleration_structure_memory_requirements_nv(&self, info: &crate::extensions::nv_ray_tracing::AccelerationStructureMemoryRequirementsInfoNV) -> crate::extensions::khr_get_memory_requirements2::MemoryRequirements2KHR {
        let _function = self.get_acceleration_structure_memory_requirements_nv.expect("`get_acceleration_structure_memory_requirements_nv` is not loaded");
        let mut memory_requirements = Default::default();
        let _return = _function(self.handle, info as _, &mut memory_requirements);
        memory_requirements
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBindAccelerationStructureMemoryNV.html) · Function"]
    #[doc(alias = "vkBindAccelerationStructureMemoryNV")]
    pub unsafe fn bind_acceleration_structure_memory_nv(&self, bind_infos: &[impl crate::Repr<crate::extensions::nv_ray_tracing::BindAccelerationStructureMemoryInfoNV>]) -> crate::utils::VulkanResult<()> {
        let _function = self.bind_acceleration_structure_memory_nv.expect("`bind_acceleration_structure_memory_nv` is not loaded");
        let bind_info_count = bind_infos.len();
        let _return = _function(self.handle, bind_info_count as _, bind_infos.as_ptr() as _);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyAccelerationStructureNV.html) · Function"]
    #[doc(alias = "vkCmdCopyAccelerationStructureNV")]
    pub unsafe fn cmd_copy_acceleration_structure_nv(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        dst: crate::extensions::nv_ray_tracing::AccelerationStructureNV,
        src: crate::extensions::nv_ray_tracing::AccelerationStructureNV,
        mode: crate::extensions::khr_acceleration_structure::CopyAccelerationStructureModeKHR,
    ) -> () {
        let _function = self.cmd_copy_acceleration_structure_nv.expect("`cmd_copy_acceleration_structure_nv` is not loaded");
        let _return = _function(command_buffer as _, dst as _, src as _, mode as _);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdWriteAccelerationStructuresPropertiesNV.html) · Function"]
    #[doc(alias = "vkCmdWriteAccelerationStructuresPropertiesNV")]
    pub unsafe fn cmd_write_acceleration_structures_properties_nv(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        acceleration_structures: &[crate::extensions::nv_ray_tracing::AccelerationStructureNV],
        query_type: crate::vk1_0::QueryType,
        query_pool: crate::vk1_0::QueryPool,
        first_query: u32,
    ) -> () {
        let _function = self.cmd_write_acceleration_structures_properties_nv.expect("`cmd_write_acceleration_structures_properties_nv` is not loaded");
        let acceleration_structure_count = acceleration_structures.len();
        let _return = _function(command_buffer as _, acceleration_structure_count as _, acceleration_structures.as_ptr() as _, query_type as _, query_pool as _, first_query as _);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBuildAccelerationStructureNV.html) · Function"]
    #[doc(alias = "vkCmdBuildAccelerationStructureNV")]
    pub unsafe fn cmd_build_acceleration_structure_nv(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        info: &crate::extensions::nv_ray_tracing::AccelerationStructureInfoNV,
        instance_data: Option<crate::vk1_0::Buffer>,
        instance_offset: crate::vk1_0::DeviceSize,
        update: bool,
        dst: crate::extensions::nv_ray_tracing::AccelerationStructureNV,
        src: Option<crate::extensions::nv_ray_tracing::AccelerationStructureNV>,
        scratch: crate::vk1_0::Buffer,
        scratch_offset: crate::vk1_0::DeviceSize,
    ) -> () {
        let _function = self.cmd_build_acceleration_structure_nv.expect("`cmd_build_acceleration_structure_nv` is not loaded");
        let _return = _function(
            command_buffer as _,
            info as _,
            match instance_data {
                Some(v) => v,
                None => Default::default(),
            },
            instance_offset as _,
            update as _,
            dst as _,
            match src {
                Some(v) => v,
                None => Default::default(),
            },
            scratch as _,
            scratch_offset as _,
        );
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdTraceRaysNV.html) · Function"]
    #[doc(alias = "vkCmdTraceRaysNV")]
    pub unsafe fn cmd_trace_rays_nv(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        raygen_shader_binding_table_buffer: crate::vk1_0::Buffer,
        raygen_shader_binding_offset: crate::vk1_0::DeviceSize,
        miss_shader_binding_table_buffer: Option<crate::vk1_0::Buffer>,
        miss_shader_binding_offset: crate::vk1_0::DeviceSize,
        miss_shader_binding_stride: crate::vk1_0::DeviceSize,
        hit_shader_binding_table_buffer: Option<crate::vk1_0::Buffer>,
        hit_shader_binding_offset: crate::vk1_0::DeviceSize,
        hit_shader_binding_stride: crate::vk1_0::DeviceSize,
        callable_shader_binding_table_buffer: Option<crate::vk1_0::Buffer>,
        callable_shader_binding_offset: crate::vk1_0::DeviceSize,
        callable_shader_binding_stride: crate::vk1_0::DeviceSize,
        width: u32,
        height: u32,
        depth: u32,
    ) -> () {
        let _function = self.cmd_trace_rays_nv.expect("`cmd_trace_rays_nv` is not loaded");
        let _return = _function(
            command_buffer as _,
            raygen_shader_binding_table_buffer as _,
            raygen_shader_binding_offset as _,
            match miss_shader_binding_table_buffer {
                Some(v) => v,
                None => Default::default(),
            },
            miss_shader_binding_offset as _,
            miss_shader_binding_stride as _,
            match hit_shader_binding_table_buffer {
                Some(v) => v,
                None => Default::default(),
            },
            hit_shader_binding_offset as _,
            hit_shader_binding_stride as _,
            match callable_shader_binding_table_buffer {
                Some(v) => v,
                None => Default::default(),
            },
            callable_shader_binding_offset as _,
            callable_shader_binding_stride as _,
            width as _,
            height as _,
            depth as _,
        );
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetAccelerationStructureHandleNV.html) · Function"]
    #[doc(alias = "vkGetAccelerationStructureHandleNV")]
    pub unsafe fn get_acceleration_structure_handle_nv(&self, acceleration_structure: crate::extensions::nv_ray_tracing::AccelerationStructureNV, data_size: usize, data: *mut std::ffi::c_void) -> crate::utils::VulkanResult<()> {
        let _function = self.get_acceleration_structure_handle_nv.expect("`get_acceleration_structure_handle_nv` is not loaded");
        let _return = _function(self.handle, acceleration_structure as _, data_size, data);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateRayTracingPipelinesNV.html) · Function"]
    #[doc(alias = "vkCreateRayTracingPipelinesNV")]
    pub unsafe fn create_ray_tracing_pipelines_nv(
        &self,
        pipeline_cache: Option<crate::vk1_0::PipelineCache>,
        create_infos: &[impl crate::Repr<crate::extensions::nv_ray_tracing::RayTracingPipelineCreateInfoNV>],
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
    ) -> crate::utils::VulkanResult<Vec<crate::vk1_0::Pipeline>> {
        let _function = self.create_ray_tracing_pipelines_nv.expect("`create_ray_tracing_pipelines_nv` is not loaded");
        let create_info_count = create_infos.len();
        let mut pipelines = vec![Default::default(); create_info_count as _];
        let _return = _function(
            self.handle,
            match pipeline_cache {
                Some(v) => v,
                None => Default::default(),
            },
            create_info_count as _,
            create_infos.as_ptr() as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            pipelines.as_mut_ptr(),
        );
        crate::utils::VulkanResult::new(_return, pipelines)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetRayTracingShaderGroupHandlesNV.html) · Function"]
    #[doc(alias = "vkGetRayTracingShaderGroupHandlesNV")]
    pub unsafe fn get_ray_tracing_shader_group_handles_nv(&self, pipeline: crate::vk1_0::Pipeline, first_group: u32, group_count: u32, data_size: usize, data: *mut std::ffi::c_void) -> crate::utils::VulkanResult<()> {
        let _function = self.get_ray_tracing_shader_group_handles_nv.expect("`get_ray_tracing_shader_group_handles_nv` is not loaded");
        let _return = _function(self.handle, pipeline as _, first_group as _, group_count as _, data_size, data);
        crate::utils::VulkanResult::new(_return, ())
    }
}
