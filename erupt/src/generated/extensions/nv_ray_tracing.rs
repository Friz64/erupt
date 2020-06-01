# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_NV_ray_tracing.html)\n\n## Extends\n- [`AccelerationStructureMemoryRequirementsTypeKHR`](../../extensions/khr_ray_tracing/struct.AccelerationStructureMemoryRequirementsTypeKHR.html)\n- [`AccelerationStructureTypeKHR`](../../extensions/khr_ray_tracing/struct.AccelerationStructureTypeKHR.html)\n- [`AccessFlagBits`](../../vk1_0/struct.AccessFlagBits.html)\n- [`BufferUsageFlagBits`](../../vk1_0/struct.BufferUsageFlagBits.html)\n- [`BuildAccelerationStructureFlagBitsKHR`](../../extensions/khr_ray_tracing/struct.BuildAccelerationStructureFlagBitsKHR.html)\n- [`CopyAccelerationStructureModeKHR`](../../extensions/khr_ray_tracing/struct.CopyAccelerationStructureModeKHR.html)\n- [`DebugReportObjectTypeEXT`](../../extensions/ext_debug_report/struct.DebugReportObjectTypeEXT.html)\n- [`DescriptorType`](../../vk1_0/struct.DescriptorType.html)\n- [`GeometryFlagBitsKHR`](../../extensions/khr_ray_tracing/struct.GeometryFlagBitsKHR.html)\n- [`GeometryInstanceFlagBitsKHR`](../../extensions/khr_ray_tracing/struct.GeometryInstanceFlagBitsKHR.html)\n- [`GeometryTypeKHR`](../../extensions/khr_ray_tracing/struct.GeometryTypeKHR.html)\n- [`IndexType`](../../vk1_0/struct.IndexType.html)\n- [`ObjectType`](../../vk1_0/struct.ObjectType.html)\n- [`PipelineBindPoint`](../../vk1_0/struct.PipelineBindPoint.html)\n- [`PipelineCreateFlagBits`](../../vk1_0/struct.PipelineCreateFlagBits.html)\n- [`PipelineStageFlagBits`](../../vk1_0/struct.PipelineStageFlagBits.html)\n- [`QueryType`](../../vk1_0/struct.QueryType.html)\n- [`RayTracingShaderGroupTypeKHR`](../../extensions/khr_ray_tracing/struct.RayTracingShaderGroupTypeKHR.html)\n- [`ShaderStageFlagBits`](../../vk1_0/struct.ShaderStageFlagBits.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NV_RAY_TRACING_SPEC_VERSION: u32 = 3;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NV_RAY_TRACING_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_NV_ray_tracing");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const SHADER_UNUSED_NV: u32 = !0u32;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateAccelerationStructureNV.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateAccelerationStructureNV = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_create_info: *const crate::extensions::nv_ray_tracing::AccelerationStructureCreateInfoNV,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
    p_acceleration_structure: *mut crate::extensions::nv_ray_tracing::AccelerationStructureNV,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyAccelerationStructureNV.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyAccelerationStructureNV = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    acceleration_structure: crate::extensions::khr_ray_tracing::AccelerationStructureKHR,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetAccelerationStructureMemoryRequirementsNV.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetAccelerationStructureMemoryRequirementsNV = unsafe extern "system" fn ( device : crate :: vk1_0 :: Device , p_info : * const crate :: extensions :: nv_ray_tracing :: AccelerationStructureMemoryRequirementsInfoNV , p_memory_requirements : * mut crate :: extensions :: khr_get_memory_requirements2 :: MemoryRequirements2KHR , ) -> std :: ffi :: c_void ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBindAccelerationStructureMemoryNV.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkBindAccelerationStructureMemoryNV = unsafe extern "system" fn ( device : crate :: vk1_0 :: Device , bind_info_count : u32 , p_bind_infos : * const crate :: extensions :: khr_ray_tracing :: BindAccelerationStructureMemoryInfoKHR , ) -> crate :: vk1_0 :: Result ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBuildAccelerationStructureNV.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBuildAccelerationStructureNV = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    p_info: *const crate::extensions::nv_ray_tracing::AccelerationStructureInfoNV,
    instance_data: crate::vk1_0::Buffer,
    instance_offset: crate::vk1_0::DeviceSize,
    update: crate::vk1_0::Bool32,
    dst: crate::extensions::khr_ray_tracing::AccelerationStructureKHR,
    src: crate::extensions::khr_ray_tracing::AccelerationStructureKHR,
    scratch: crate::vk1_0::Buffer,
    scratch_offset: crate::vk1_0::DeviceSize,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyAccelerationStructureNV.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyAccelerationStructureNV = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    dst: crate::extensions::khr_ray_tracing::AccelerationStructureKHR,
    src: crate::extensions::khr_ray_tracing::AccelerationStructureKHR,
    mode: crate::extensions::khr_ray_tracing::CopyAccelerationStructureModeKHR,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdTraceRaysNV.html) · Device Command"]
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
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateRayTracingPipelinesNV.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateRayTracingPipelinesNV = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    pipeline_cache: crate::vk1_0::PipelineCache,
    create_info_count: u32,
    p_create_infos: *const crate::extensions::nv_ray_tracing::RayTracingPipelineCreateInfoNV,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
    p_pipelines: *mut crate::vk1_0::Pipeline,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetRayTracingShaderGroupHandlesNV.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetRayTracingShaderGroupHandlesNV =
    unsafe extern "system" fn(
        device: crate::vk1_0::Device,
        pipeline: crate::vk1_0::Pipeline,
        first_group: u32,
        group_count: u32,
        data_size: usize,
        p_data: *mut std::ffi::c_void,
    ) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetAccelerationStructureHandleNV.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetAccelerationStructureHandleNV = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    acceleration_structure: crate::extensions::khr_ray_tracing::AccelerationStructureKHR,
    data_size: usize,
    p_data: *mut std::ffi::c_void,
)
    -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdWriteAccelerationStructuresPropertiesNV.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdWriteAccelerationStructuresPropertiesNV = unsafe extern "system" fn ( command_buffer : crate :: vk1_0 :: CommandBuffer , acceleration_structure_count : u32 , p_acceleration_structures : * const crate :: extensions :: khr_ray_tracing :: AccelerationStructureKHR , query_type : crate :: vk1_0 :: QueryType , query_pool : crate :: vk1_0 :: QueryPool , first_query : u32 , ) -> std :: ffi :: c_void ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCompileDeferredNV.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCompileDeferredNV = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    pipeline: crate::vk1_0::Pipeline,
    shader: u32,
) -> crate::vk1_0::Result;
#[doc = "Provides Device Commands for [`NvRayTracingDeviceLoaderExt`](trait.NvRayTracingDeviceLoaderExt.html)"]
pub struct NvRayTracingDeviceCommands {
    pub create_acceleration_structure_nv: PFN_vkCreateAccelerationStructureNV,
    pub destroy_acceleration_structure_nv: PFN_vkDestroyAccelerationStructureNV,
    pub get_acceleration_structure_memory_requirements_nv:
        PFN_vkGetAccelerationStructureMemoryRequirementsNV,
    pub bind_acceleration_structure_memory_nv: PFN_vkBindAccelerationStructureMemoryNV,
    pub cmd_build_acceleration_structure_nv: PFN_vkCmdBuildAccelerationStructureNV,
    pub cmd_copy_acceleration_structure_nv: PFN_vkCmdCopyAccelerationStructureNV,
    pub cmd_trace_rays_nv: PFN_vkCmdTraceRaysNV,
    pub create_ray_tracing_pipelines_nv: PFN_vkCreateRayTracingPipelinesNV,
    pub get_ray_tracing_shader_group_handles_nv: PFN_vkGetRayTracingShaderGroupHandlesNV,
    pub get_acceleration_structure_handle_nv: PFN_vkGetAccelerationStructureHandleNV,
    pub cmd_write_acceleration_structures_properties_nv:
        PFN_vkCmdWriteAccelerationStructuresPropertiesNV,
    pub compile_deferred_nv: PFN_vkCompileDeferredNV,
}
impl NvRayTracingDeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<NvRayTracingDeviceCommands> {
        unsafe {
            Some(NvRayTracingDeviceCommands {
                create_acceleration_structure_nv: std::mem::transmute(
                    loader.symbol("vkCreateAccelerationStructureNV")?,
                ),
                destroy_acceleration_structure_nv: std::mem::transmute(
                    loader.symbol("vkDestroyAccelerationStructureNV")?,
                ),
                get_acceleration_structure_memory_requirements_nv: std::mem::transmute(
                    loader.symbol("vkGetAccelerationStructureMemoryRequirementsNV")?,
                ),
                bind_acceleration_structure_memory_nv: std::mem::transmute(
                    loader.symbol("vkBindAccelerationStructureMemoryNV")?,
                ),
                cmd_build_acceleration_structure_nv: std::mem::transmute(
                    loader.symbol("vkCmdBuildAccelerationStructureNV")?,
                ),
                cmd_copy_acceleration_structure_nv: std::mem::transmute(
                    loader.symbol("vkCmdCopyAccelerationStructureNV")?,
                ),
                cmd_trace_rays_nv: std::mem::transmute(loader.symbol("vkCmdTraceRaysNV")?),
                create_ray_tracing_pipelines_nv: std::mem::transmute(
                    loader.symbol("vkCreateRayTracingPipelinesNV")?,
                ),
                get_ray_tracing_shader_group_handles_nv: std::mem::transmute(
                    loader.symbol("vkGetRayTracingShaderGroupHandlesNV")?,
                ),
                get_acceleration_structure_handle_nv: std::mem::transmute(
                    loader.symbol("vkGetAccelerationStructureHandleNV")?,
                ),
                cmd_write_acceleration_structures_properties_nv: std::mem::transmute(
                    loader.symbol("vkCmdWriteAccelerationStructuresPropertiesNV")?,
                ),
                compile_deferred_nv: std::mem::transmute(loader.symbol("vkCompileDeferredNV")?),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`NvRayTracingDeviceCommands`](struct.NvRayTracingDeviceCommands.html)"]
pub trait NvRayTracingDeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateAccelerationStructureNV.html) · Device Command"]
    unsafe fn create_acceleration_structure_nv(
        &self,
        create_info: &crate::extensions::nv_ray_tracing::AccelerationStructureCreateInfoNV,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        acceleration_structure: Option<crate::extensions::nv_ray_tracing::AccelerationStructureNV>,
    ) -> crate::utils::VulkanResult<crate::extensions::nv_ray_tracing::AccelerationStructureNV>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyAccelerationStructureNV.html) · Device Command"]
    unsafe fn destroy_acceleration_structure_nv(
        &self,
        acceleration_structure: crate::extensions::khr_ray_tracing::AccelerationStructureKHR,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetAccelerationStructureMemoryRequirementsNV.html) · Device Command"]
    unsafe fn get_acceleration_structure_memory_requirements_nv(
        &self,
        info: &crate::extensions::nv_ray_tracing::AccelerationStructureMemoryRequirementsInfoNV,
        memory_requirements: Option<
            crate::extensions::khr_get_memory_requirements2::MemoryRequirements2KHR,
        >,
    ) -> crate::extensions::khr_get_memory_requirements2::MemoryRequirements2KHR;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBindAccelerationStructureMemoryNV.html) · Device Command"]
    unsafe fn bind_acceleration_structure_memory_nv(
        &self,
        bind_infos : & [ crate :: extensions :: khr_ray_tracing :: BindAccelerationStructureMemoryInfoKHRBuilder ],
    ) -> crate::utils::VulkanResult<()>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBuildAccelerationStructureNV.html) · Device Command"]
    unsafe fn cmd_build_acceleration_structure_nv(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        info: &crate::extensions::nv_ray_tracing::AccelerationStructureInfoNV,
        instance_data: crate::vk1_0::Buffer,
        instance_offset: crate::vk1_0::DeviceSize,
        update: bool,
        dst: crate::extensions::khr_ray_tracing::AccelerationStructureKHR,
        src: crate::extensions::khr_ray_tracing::AccelerationStructureKHR,
        scratch: crate::vk1_0::Buffer,
        scratch_offset: crate::vk1_0::DeviceSize,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyAccelerationStructureNV.html) · Device Command"]
    unsafe fn cmd_copy_acceleration_structure_nv(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        dst: crate::extensions::khr_ray_tracing::AccelerationStructureKHR,
        src: crate::extensions::khr_ray_tracing::AccelerationStructureKHR,
        mode: crate::extensions::khr_ray_tracing::CopyAccelerationStructureModeKHR,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdTraceRaysNV.html) · Device Command"]
    unsafe fn cmd_trace_rays_nv(
        &self,
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
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateRayTracingPipelinesNV.html) · Device Command"]
    unsafe fn create_ray_tracing_pipelines_nv(
        &self,
        pipeline_cache: crate::vk1_0::PipelineCache,
        create_infos: &[crate::extensions::nv_ray_tracing::RayTracingPipelineCreateInfoNVBuilder],
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
    ) -> crate::utils::VulkanResult<Vec<crate::vk1_0::Pipeline>>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetRayTracingShaderGroupHandlesNV.html) · Device Command"]
    unsafe fn get_ray_tracing_shader_group_handles_nv(
        &self,
        pipeline: crate::vk1_0::Pipeline,
        first_group: u32,
        group_count: u32,
        data_size: usize,
        data: *mut std::ffi::c_void,
    ) -> crate::utils::VulkanResult<()>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetAccelerationStructureHandleNV.html) · Device Command"]
    unsafe fn get_acceleration_structure_handle_nv(
        &self,
        acceleration_structure: crate::extensions::khr_ray_tracing::AccelerationStructureKHR,
        data_size: usize,
        data: *mut std::ffi::c_void,
    ) -> crate::utils::VulkanResult<()>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdWriteAccelerationStructuresPropertiesNV.html) · Device Command"]
    unsafe fn cmd_write_acceleration_structures_properties_nv(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        acceleration_structures: &[crate::extensions::khr_ray_tracing::AccelerationStructureKHR],
        query_type: crate::vk1_0::QueryType,
        query_pool: crate::vk1_0::QueryPool,
        first_query: u32,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCompileDeferredNV.html) · Device Command"]
    unsafe fn compile_deferred_nv(
        &self,
        pipeline: crate::vk1_0::Pipeline,
        shader: u32,
    ) -> crate::utils::VulkanResult<()>;
}
impl NvRayTracingDeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateAccelerationStructureNV.html) · Device Command"]
    unsafe fn create_acceleration_structure_nv(
        &self,
        create_info: &crate::extensions::nv_ray_tracing::AccelerationStructureCreateInfoNV,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        acceleration_structure: Option<crate::extensions::nv_ray_tracing::AccelerationStructureNV>,
    ) -> crate::utils::VulkanResult<crate::extensions::nv_ray_tracing::AccelerationStructureNV>
    {
        let function = self
            .nv_ray_tracing
            .as_ref()
            .expect("`nv_ray_tracing` not loaded")
            .create_acceleration_structure_nv;
        let mut acceleration_structure =
            acceleration_structure.unwrap_or_else(|| Default::default());
        let _val = function(
            self.handle,
            create_info,
            if let Some(allocator) = allocator {
                allocator
            } else {
                std::ptr::null()
            },
            &mut acceleration_structure,
        );
        crate::utils::VulkanResult::new(_val, acceleration_structure)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyAccelerationStructureNV.html) · Device Command"]
    unsafe fn destroy_acceleration_structure_nv(
        &self,
        acceleration_structure: crate::extensions::khr_ray_tracing::AccelerationStructureKHR,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
    ) -> () {
        let function = self
            .nv_ray_tracing
            .as_ref()
            .expect("`nv_ray_tracing` not loaded")
            .destroy_acceleration_structure_nv;
        let _val = function(
            self.handle,
            acceleration_structure,
            if let Some(allocator) = allocator {
                allocator
            } else {
                std::ptr::null()
            },
        );
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetAccelerationStructureMemoryRequirementsNV.html) · Device Command"]
    unsafe fn get_acceleration_structure_memory_requirements_nv(
        &self,
        info: &crate::extensions::nv_ray_tracing::AccelerationStructureMemoryRequirementsInfoNV,
        memory_requirements: Option<
            crate::extensions::khr_get_memory_requirements2::MemoryRequirements2KHR,
        >,
    ) -> crate::extensions::khr_get_memory_requirements2::MemoryRequirements2KHR {
        let function = self
            .nv_ray_tracing
            .as_ref()
            .expect("`nv_ray_tracing` not loaded")
            .get_acceleration_structure_memory_requirements_nv;
        let mut memory_requirements = memory_requirements.unwrap_or_else(|| Default::default());
        let _val = function(self.handle, info, &mut memory_requirements);
        memory_requirements
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBindAccelerationStructureMemoryNV.html) · Device Command"]
    unsafe fn bind_acceleration_structure_memory_nv(
        &self,
        bind_infos : & [ crate :: extensions :: khr_ray_tracing :: BindAccelerationStructureMemoryInfoKHRBuilder ],
    ) -> crate::utils::VulkanResult<()> {
        let function = self
            .nv_ray_tracing
            .as_ref()
            .expect("`nv_ray_tracing` not loaded")
            .bind_acceleration_structure_memory_nv;
        let bind_info_count = bind_infos.len() as _;
        let _val = function(self.handle, bind_info_count, bind_infos.as_ptr() as _);
        crate::utils::VulkanResult::new(_val, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBuildAccelerationStructureNV.html) · Device Command"]
    unsafe fn cmd_build_acceleration_structure_nv(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        info: &crate::extensions::nv_ray_tracing::AccelerationStructureInfoNV,
        instance_data: crate::vk1_0::Buffer,
        instance_offset: crate::vk1_0::DeviceSize,
        update: bool,
        dst: crate::extensions::khr_ray_tracing::AccelerationStructureKHR,
        src: crate::extensions::khr_ray_tracing::AccelerationStructureKHR,
        scratch: crate::vk1_0::Buffer,
        scratch_offset: crate::vk1_0::DeviceSize,
    ) -> () {
        let function = self
            .nv_ray_tracing
            .as_ref()
            .expect("`nv_ray_tracing` not loaded")
            .cmd_build_acceleration_structure_nv;
        let _val = function(
            command_buffer,
            info,
            instance_data,
            instance_offset,
            update as _,
            dst,
            src,
            scratch,
            scratch_offset,
        );
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyAccelerationStructureNV.html) · Device Command"]
    unsafe fn cmd_copy_acceleration_structure_nv(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        dst: crate::extensions::khr_ray_tracing::AccelerationStructureKHR,
        src: crate::extensions::khr_ray_tracing::AccelerationStructureKHR,
        mode: crate::extensions::khr_ray_tracing::CopyAccelerationStructureModeKHR,
    ) -> () {
        let function = self
            .nv_ray_tracing
            .as_ref()
            .expect("`nv_ray_tracing` not loaded")
            .cmd_copy_acceleration_structure_nv;
        let _val = function(command_buffer, dst, src, mode);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdTraceRaysNV.html) · Device Command"]
    unsafe fn cmd_trace_rays_nv(
        &self,
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
    ) -> () {
        let function = self
            .nv_ray_tracing
            .as_ref()
            .expect("`nv_ray_tracing` not loaded")
            .cmd_trace_rays_nv;
        let _val = function(
            command_buffer,
            raygen_shader_binding_table_buffer,
            raygen_shader_binding_offset,
            miss_shader_binding_table_buffer,
            miss_shader_binding_offset,
            miss_shader_binding_stride,
            hit_shader_binding_table_buffer,
            hit_shader_binding_offset,
            hit_shader_binding_stride,
            callable_shader_binding_table_buffer,
            callable_shader_binding_offset,
            callable_shader_binding_stride,
            width,
            height,
            depth,
        );
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateRayTracingPipelinesNV.html) · Device Command"]
    unsafe fn create_ray_tracing_pipelines_nv(
        &self,
        pipeline_cache: crate::vk1_0::PipelineCache,
        create_infos: &[crate::extensions::nv_ray_tracing::RayTracingPipelineCreateInfoNVBuilder],
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
    ) -> crate::utils::VulkanResult<Vec<crate::vk1_0::Pipeline>> {
        let function = self
            .nv_ray_tracing
            .as_ref()
            .expect("`nv_ray_tracing` not loaded")
            .create_ray_tracing_pipelines_nv;
        let create_info_count = create_infos.len() as _;
        let mut pipelines = vec![Default::default(); create_info_count as _];
        let _val = function(
            self.handle,
            pipeline_cache,
            create_info_count,
            create_infos.as_ptr() as _,
            if let Some(allocator) = allocator {
                allocator
            } else {
                std::ptr::null()
            },
            pipelines.as_mut_ptr(),
        );
        crate::utils::VulkanResult::new(_val, pipelines)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetRayTracingShaderGroupHandlesNV.html) · Device Command"]
    unsafe fn get_ray_tracing_shader_group_handles_nv(
        &self,
        pipeline: crate::vk1_0::Pipeline,
        first_group: u32,
        group_count: u32,
        data_size: usize,
        data: *mut std::ffi::c_void,
    ) -> crate::utils::VulkanResult<()> {
        let function = self
            .nv_ray_tracing
            .as_ref()
            .expect("`nv_ray_tracing` not loaded")
            .get_ray_tracing_shader_group_handles_nv;
        let _val = function(
            self.handle,
            pipeline,
            first_group,
            group_count,
            data_size,
            data,
        );
        crate::utils::VulkanResult::new(_val, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetAccelerationStructureHandleNV.html) · Device Command"]
    unsafe fn get_acceleration_structure_handle_nv(
        &self,
        acceleration_structure: crate::extensions::khr_ray_tracing::AccelerationStructureKHR,
        data_size: usize,
        data: *mut std::ffi::c_void,
    ) -> crate::utils::VulkanResult<()> {
        let function = self
            .nv_ray_tracing
            .as_ref()
            .expect("`nv_ray_tracing` not loaded")
            .get_acceleration_structure_handle_nv;
        let _val = function(self.handle, acceleration_structure, data_size, data);
        crate::utils::VulkanResult::new(_val, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdWriteAccelerationStructuresPropertiesNV.html) · Device Command"]
    unsafe fn cmd_write_acceleration_structures_properties_nv(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        acceleration_structures: &[crate::extensions::khr_ray_tracing::AccelerationStructureKHR],
        query_type: crate::vk1_0::QueryType,
        query_pool: crate::vk1_0::QueryPool,
        first_query: u32,
    ) -> () {
        let function = self
            .nv_ray_tracing
            .as_ref()
            .expect("`nv_ray_tracing` not loaded")
            .cmd_write_acceleration_structures_properties_nv;
        let acceleration_structure_count = acceleration_structures.len() as _;
        let _val = function(
            command_buffer,
            acceleration_structure_count,
            acceleration_structures.as_ptr() as _,
            query_type,
            query_pool,
            first_query,
        );
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCompileDeferredNV.html) · Device Command"]
    unsafe fn compile_deferred_nv(
        &self,
        pipeline: crate::vk1_0::Pipeline,
        shader: u32,
    ) -> crate::utils::VulkanResult<()> {
        let function = self
            .nv_ray_tracing
            .as_ref()
            .expect("`nv_ray_tracing` not loaded")
            .compile_deferred_nv;
        let _val = function(self.handle, pipeline, shader);
        crate::utils::VulkanResult::new(_val, ())
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureCreateInfoNV.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AccelerationStructureCreateInfoNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub compacted_size: crate::vk1_0::DeviceSize,
    pub info: crate::extensions::nv_ray_tracing::AccelerationStructureInfoNV,
}
impl AccelerationStructureCreateInfoNV {
    #[inline]
    pub fn builder<'a>(self) -> AccelerationStructureCreateInfoNVBuilder<'a> {
        AccelerationStructureCreateInfoNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for AccelerationStructureCreateInfoNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("AccelerationStructureCreateInfoNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("compacted_size", &self.compacted_size)
            .field("info", &self.info)
            .finish()
    }
}
impl Default for AccelerationStructureCreateInfoNV {
    fn default() -> AccelerationStructureCreateInfoNV {
        AccelerationStructureCreateInfoNV {
            s_type: crate::vk1_0::StructureType::ACCELERATION_STRUCTURE_CREATE_INFO_NV,
            p_next: std::ptr::null(),
            compacted_size: Default::default(),
            info: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureCreateInfoNV.html) · Builder of [`AccelerationStructureCreateInfoNV`](struct.AccelerationStructureCreateInfoNV.html)"]
#[repr(transparent)]
pub struct AccelerationStructureCreateInfoNVBuilder<'a>(
    AccelerationStructureCreateInfoNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> AccelerationStructureCreateInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> AccelerationStructureCreateInfoNVBuilder<'a> {
        AccelerationStructureCreateInfoNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn compacted_size(mut self, compacted_size: crate::vk1_0::DeviceSize) -> Self {
        self.0.compacted_size = compacted_size;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn info(
        mut self,
        info: crate::extensions::nv_ray_tracing::AccelerationStructureInfoNV,
    ) -> Self {
        self.0.info = info;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> AccelerationStructureCreateInfoNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for AccelerationStructureCreateInfoNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureInfoNV.html) · Structure"]
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
impl AccelerationStructureInfoNV {
    #[inline]
    pub fn builder<'a>(self) -> AccelerationStructureInfoNVBuilder<'a> {
        AccelerationStructureInfoNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for AccelerationStructureInfoNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("AccelerationStructureInfoNV")
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
impl Default for AccelerationStructureInfoNV {
    fn default() -> AccelerationStructureInfoNV {
        AccelerationStructureInfoNV {
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
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureInfoNV.html) · Builder of [`AccelerationStructureInfoNV`](struct.AccelerationStructureInfoNV.html)"]
#[repr(transparent)]
pub struct AccelerationStructureInfoNVBuilder<'a>(
    AccelerationStructureInfoNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> AccelerationStructureInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> AccelerationStructureInfoNVBuilder<'a> {
        AccelerationStructureInfoNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn _type(
        mut self,
        _type: crate::extensions::nv_ray_tracing::AccelerationStructureTypeNV,
    ) -> Self {
        self.0._type = _type;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn flags(
        mut self,
        flags: crate::extensions::nv_ray_tracing::BuildAccelerationStructureFlagsNV,
    ) -> Self {
        self.0.flags = flags;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn instance_count(mut self, instance_count: u32) -> Self {
        self.0.instance_count = instance_count;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn geometries(
        mut self,
        geometries: &'a [crate::extensions::nv_ray_tracing::GeometryNVBuilder],
    ) -> Self {
        self.0.geometry_count = geometries.len() as _;
        self.0.p_geometries = geometries.as_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> AccelerationStructureInfoNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for AccelerationStructureInfoNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureTypeNV.html) · Alias"]
pub type AccelerationStructureTypeNV =
    crate::extensions::khr_ray_tracing::AccelerationStructureTypeKHR;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBuildAccelerationStructureFlagsNV.html) · Alias"]
pub type BuildAccelerationStructureFlagsNV =
    crate::extensions::khr_ray_tracing::BuildAccelerationStructureFlagsKHR;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeometryNV.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GeometryNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub geometry_type: crate::extensions::khr_ray_tracing::GeometryTypeKHR,
    pub geometry: crate::extensions::nv_ray_tracing::GeometryDataNV,
    pub flags: crate::extensions::khr_ray_tracing::GeometryFlagsKHR,
}
impl GeometryNV {
    #[inline]
    pub fn builder<'a>(self) -> GeometryNVBuilder<'a> {
        GeometryNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for GeometryNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("GeometryNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("geometry_type", &self.geometry_type)
            .field("geometry", &self.geometry)
            .field("flags", &self.flags)
            .finish()
    }
}
impl Default for GeometryNV {
    fn default() -> GeometryNV {
        GeometryNV {
            s_type: crate::vk1_0::StructureType::GEOMETRY_NV,
            p_next: std::ptr::null(),
            geometry_type: Default::default(),
            geometry: Default::default(),
            flags: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeometryNV.html) · Builder of [`GeometryNV`](struct.GeometryNV.html)"]
#[repr(transparent)]
pub struct GeometryNVBuilder<'a>(GeometryNV, std::marker::PhantomData<&'a ()>);
impl<'a> GeometryNVBuilder<'a> {
    #[inline]
    pub fn new() -> GeometryNVBuilder<'a> {
        GeometryNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn geometry_type(
        mut self,
        geometry_type: crate::extensions::khr_ray_tracing::GeometryTypeKHR,
    ) -> Self {
        self.0.geometry_type = geometry_type;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn geometry(mut self, geometry: crate::extensions::nv_ray_tracing::GeometryDataNV) -> Self {
        self.0.geometry = geometry;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::khr_ray_tracing::GeometryFlagsKHR) -> Self {
        self.0.flags = flags;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> GeometryNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for GeometryNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeometryDataNV.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GeometryDataNV {
    pub triangles: crate::extensions::nv_ray_tracing::GeometryTrianglesNV,
    pub aabbs: crate::extensions::nv_ray_tracing::GeometryAABBNV,
}
impl GeometryDataNV {
    #[inline]
    pub fn builder<'a>(self) -> GeometryDataNVBuilder<'a> {
        GeometryDataNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for GeometryDataNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("GeometryDataNV")
            .field("triangles", &self.triangles)
            .field("aabbs", &self.aabbs)
            .finish()
    }
}
impl Default for GeometryDataNV {
    fn default() -> GeometryDataNV {
        GeometryDataNV {
            triangles: Default::default(),
            aabbs: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeometryDataNV.html) · Builder of [`GeometryDataNV`](struct.GeometryDataNV.html)"]
#[repr(transparent)]
pub struct GeometryDataNVBuilder<'a>(GeometryDataNV, std::marker::PhantomData<&'a ()>);
impl<'a> GeometryDataNVBuilder<'a> {
    #[inline]
    pub fn new() -> GeometryDataNVBuilder<'a> {
        GeometryDataNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn triangles(
        mut self,
        triangles: crate::extensions::nv_ray_tracing::GeometryTrianglesNV,
    ) -> Self {
        self.0.triangles = triangles;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn aabbs(mut self, aabbs: crate::extensions::nv_ray_tracing::GeometryAABBNV) -> Self {
        self.0.aabbs = aabbs;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> GeometryDataNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for GeometryDataNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeometryTrianglesNV.html) · Structure"]
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
impl GeometryTrianglesNV {
    #[inline]
    pub fn builder<'a>(self) -> GeometryTrianglesNVBuilder<'a> {
        GeometryTrianglesNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for GeometryTrianglesNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("GeometryTrianglesNV")
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
impl Default for GeometryTrianglesNV {
    fn default() -> GeometryTrianglesNV {
        GeometryTrianglesNV {
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
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeometryTrianglesNV.html) · Builder of [`GeometryTrianglesNV`](struct.GeometryTrianglesNV.html)"]
#[repr(transparent)]
pub struct GeometryTrianglesNVBuilder<'a>(GeometryTrianglesNV, std::marker::PhantomData<&'a ()>);
impl<'a> GeometryTrianglesNVBuilder<'a> {
    #[inline]
    pub fn new() -> GeometryTrianglesNVBuilder<'a> {
        GeometryTrianglesNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn vertex_data(mut self, vertex_data: crate::vk1_0::Buffer) -> Self {
        self.0.vertex_data = vertex_data;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn vertex_offset(mut self, vertex_offset: crate::vk1_0::DeviceSize) -> Self {
        self.0.vertex_offset = vertex_offset;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn vertex_count(mut self, vertex_count: u32) -> Self {
        self.0.vertex_count = vertex_count;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn vertex_stride(mut self, vertex_stride: crate::vk1_0::DeviceSize) -> Self {
        self.0.vertex_stride = vertex_stride;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn vertex_format(mut self, vertex_format: crate::vk1_0::Format) -> Self {
        self.0.vertex_format = vertex_format;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn index_data(mut self, index_data: crate::vk1_0::Buffer) -> Self {
        self.0.index_data = index_data;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn index_offset(mut self, index_offset: crate::vk1_0::DeviceSize) -> Self {
        self.0.index_offset = index_offset;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn index_count(mut self, index_count: u32) -> Self {
        self.0.index_count = index_count;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn index_type(mut self, index_type: crate::vk1_0::IndexType) -> Self {
        self.0.index_type = index_type;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn transform_data(mut self, transform_data: crate::vk1_0::Buffer) -> Self {
        self.0.transform_data = transform_data;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn transform_offset(mut self, transform_offset: crate::vk1_0::DeviceSize) -> Self {
        self.0.transform_offset = transform_offset;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> GeometryTrianglesNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for GeometryTrianglesNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeometryAABBNV.html) · Structure"]
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
impl GeometryAABBNV {
    #[inline]
    pub fn builder<'a>(self) -> GeometryAABBNVBuilder<'a> {
        GeometryAABBNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for GeometryAABBNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("GeometryAABBNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("aabb_data", &self.aabb_data)
            .field("num_aab_bs", &self.num_aab_bs)
            .field("stride", &self.stride)
            .field("offset", &self.offset)
            .finish()
    }
}
impl Default for GeometryAABBNV {
    fn default() -> GeometryAABBNV {
        GeometryAABBNV {
            s_type: crate::vk1_0::StructureType::GEOMETRY_AABB_NV,
            p_next: std::ptr::null(),
            aabb_data: Default::default(),
            num_aab_bs: Default::default(),
            stride: Default::default(),
            offset: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeometryAABBNV.html) · Builder of [`GeometryAABBNV`](struct.GeometryAABBNV.html)"]
#[repr(transparent)]
pub struct GeometryAABBNVBuilder<'a>(GeometryAABBNV, std::marker::PhantomData<&'a ()>);
impl<'a> GeometryAABBNVBuilder<'a> {
    #[inline]
    pub fn new() -> GeometryAABBNVBuilder<'a> {
        GeometryAABBNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn aabb_data(mut self, aabb_data: crate::vk1_0::Buffer) -> Self {
        self.0.aabb_data = aabb_data;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn num_aab_bs(mut self, num_aab_bs: u32) -> Self {
        self.0.num_aab_bs = num_aab_bs;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn stride(mut self, stride: u32) -> Self {
        self.0.stride = stride;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn offset(mut self, offset: crate::vk1_0::DeviceSize) -> Self {
        self.0.offset = offset;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> GeometryAABBNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for GeometryAABBNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureNV.html) · Alias"]
pub type AccelerationStructureNV = crate::extensions::khr_ray_tracing::AccelerationStructureKHR;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureMemoryRequirementsInfoNV.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AccelerationStructureMemoryRequirementsInfoNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub _type: crate::extensions::nv_ray_tracing::AccelerationStructureMemoryRequirementsTypeNV,
    pub acceleration_structure: crate::extensions::nv_ray_tracing::AccelerationStructureNV,
}
impl AccelerationStructureMemoryRequirementsInfoNV {
    #[inline]
    pub fn builder<'a>(self) -> AccelerationStructureMemoryRequirementsInfoNVBuilder<'a> {
        AccelerationStructureMemoryRequirementsInfoNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for AccelerationStructureMemoryRequirementsInfoNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("AccelerationStructureMemoryRequirementsInfoNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("_type", &self._type)
            .field("acceleration_structure", &self.acceleration_structure)
            .finish()
    }
}
impl Default for AccelerationStructureMemoryRequirementsInfoNV {
    fn default() -> AccelerationStructureMemoryRequirementsInfoNV {
        AccelerationStructureMemoryRequirementsInfoNV {
            s_type: crate::vk1_0::StructureType::ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO_NV,
            p_next: std::ptr::null(),
            _type: Default::default(),
            acceleration_structure: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureMemoryRequirementsInfoNV.html) · Builder of [`AccelerationStructureMemoryRequirementsInfoNV`](struct.AccelerationStructureMemoryRequirementsInfoNV.html)"]
#[repr(transparent)]
pub struct AccelerationStructureMemoryRequirementsInfoNVBuilder<'a>(
    AccelerationStructureMemoryRequirementsInfoNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> AccelerationStructureMemoryRequirementsInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> AccelerationStructureMemoryRequirementsInfoNVBuilder<'a> {
        AccelerationStructureMemoryRequirementsInfoNVBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn _type(
        mut self,
        _type: crate::extensions::nv_ray_tracing::AccelerationStructureMemoryRequirementsTypeNV,
    ) -> Self {
        self.0._type = _type;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn acceleration_structure(
        mut self,
        acceleration_structure: crate::extensions::nv_ray_tracing::AccelerationStructureNV,
    ) -> Self {
        self.0.acceleration_structure = acceleration_structure;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> AccelerationStructureMemoryRequirementsInfoNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for AccelerationStructureMemoryRequirementsInfoNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureMemoryRequirementsTypeNV.html) · Alias"]
pub type AccelerationStructureMemoryRequirementsTypeNV =
    crate::extensions::khr_ray_tracing::AccelerationStructureMemoryRequirementsTypeKHR;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRayTracingPipelineCreateInfoNV.html) · Structure"]
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
impl RayTracingPipelineCreateInfoNV {
    #[inline]
    pub fn builder<'a>(self) -> RayTracingPipelineCreateInfoNVBuilder<'a> {
        RayTracingPipelineCreateInfoNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for RayTracingPipelineCreateInfoNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("RayTracingPipelineCreateInfoNV")
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
impl Default for RayTracingPipelineCreateInfoNV {
    fn default() -> RayTracingPipelineCreateInfoNV {
        RayTracingPipelineCreateInfoNV {
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
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRayTracingPipelineCreateInfoNV.html) · Builder of [`RayTracingPipelineCreateInfoNV`](struct.RayTracingPipelineCreateInfoNV.html)"]
#[repr(transparent)]
pub struct RayTracingPipelineCreateInfoNVBuilder<'a>(
    RayTracingPipelineCreateInfoNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> RayTracingPipelineCreateInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> RayTracingPipelineCreateInfoNVBuilder<'a> {
        RayTracingPipelineCreateInfoNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_0::PipelineCreateFlags) -> Self {
        self.0.flags = flags;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn stages(
        mut self,
        stages: &'a [crate::vk1_0::PipelineShaderStageCreateInfoBuilder],
    ) -> Self {
        self.0.stage_count = stages.len() as _;
        self.0.p_stages = stages.as_ptr() as _;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn groups(
        mut self,
        groups: &'a [crate::extensions::nv_ray_tracing::RayTracingShaderGroupCreateInfoNVBuilder],
    ) -> Self {
        self.0.group_count = groups.len() as _;
        self.0.p_groups = groups.as_ptr() as _;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_recursion_depth(mut self, max_recursion_depth: u32) -> Self {
        self.0.max_recursion_depth = max_recursion_depth;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn layout(mut self, layout: crate::vk1_0::PipelineLayout) -> Self {
        self.0.layout = layout;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn base_pipeline_handle(mut self, base_pipeline_handle: crate::vk1_0::Pipeline) -> Self {
        self.0.base_pipeline_handle = base_pipeline_handle;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn base_pipeline_index(mut self, base_pipeline_index: i32) -> Self {
        self.0.base_pipeline_index = base_pipeline_index;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> RayTracingPipelineCreateInfoNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for RayTracingPipelineCreateInfoNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRayTracingShaderGroupCreateInfoNV.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RayTracingShaderGroupCreateInfoNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub _type: crate::extensions::khr_ray_tracing::RayTracingShaderGroupTypeKHR,
    pub general_shader: u32,
    pub closest_hit_shader: u32,
    pub any_hit_shader: u32,
    pub intersection_shader: u32,
}
impl RayTracingShaderGroupCreateInfoNV {
    #[inline]
    pub fn builder<'a>(self) -> RayTracingShaderGroupCreateInfoNVBuilder<'a> {
        RayTracingShaderGroupCreateInfoNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for RayTracingShaderGroupCreateInfoNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("RayTracingShaderGroupCreateInfoNV")
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
impl Default for RayTracingShaderGroupCreateInfoNV {
    fn default() -> RayTracingShaderGroupCreateInfoNV {
        RayTracingShaderGroupCreateInfoNV {
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
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRayTracingShaderGroupCreateInfoNV.html) · Builder of [`RayTracingShaderGroupCreateInfoNV`](struct.RayTracingShaderGroupCreateInfoNV.html)"]
#[repr(transparent)]
pub struct RayTracingShaderGroupCreateInfoNVBuilder<'a>(
    RayTracingShaderGroupCreateInfoNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> RayTracingShaderGroupCreateInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> RayTracingShaderGroupCreateInfoNVBuilder<'a> {
        RayTracingShaderGroupCreateInfoNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn _type(
        mut self,
        _type: crate::extensions::khr_ray_tracing::RayTracingShaderGroupTypeKHR,
    ) -> Self {
        self.0._type = _type;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn general_shader(mut self, general_shader: u32) -> Self {
        self.0.general_shader = general_shader;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn closest_hit_shader(mut self, closest_hit_shader: u32) -> Self {
        self.0.closest_hit_shader = closest_hit_shader;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn any_hit_shader(mut self, any_hit_shader: u32) -> Self {
        self.0.any_hit_shader = any_hit_shader;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn intersection_shader(mut self, intersection_shader: u32) -> Self {
        self.0.intersection_shader = intersection_shader;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> RayTracingShaderGroupCreateInfoNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for RayTracingShaderGroupCreateInfoNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRayTracingShaderGroupTypeNV.html) · Alias"]
pub type RayTracingShaderGroupTypeNV =
    crate::extensions::khr_ray_tracing::RayTracingShaderGroupTypeKHR;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeometryTypeNV.html) · Alias"]
pub type GeometryTypeNV = crate::extensions::khr_ray_tracing::GeometryTypeKHR;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeometryFlagsNV.html) · Alias"]
pub type GeometryFlagsNV = crate::extensions::khr_ray_tracing::GeometryFlagsKHR;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeometryInstanceFlagsNV.html) · Alias"]
pub type GeometryInstanceFlagsNV = crate::extensions::khr_ray_tracing::GeometryInstanceFlagsKHR;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCopyAccelerationStructureModeNV.html) · Alias"]
pub type CopyAccelerationStructureModeNV =
    crate::extensions::khr_ray_tracing::CopyAccelerationStructureModeKHR;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindAccelerationStructureMemoryInfoNV.html) · Alias"]
pub type BindAccelerationStructureMemoryInfoNV =
    crate::extensions::khr_ray_tracing::BindAccelerationStructureMemoryInfoKHR;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkWriteDescriptorSetAccelerationStructureNV.html) · Alias"]
pub type WriteDescriptorSetAccelerationStructureNV =
    crate::extensions::khr_ray_tracing::WriteDescriptorSetAccelerationStructureKHR;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceRayTracingPropertiesNV.html) · Structure"]
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
impl PhysicalDeviceRayTracingPropertiesNV {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: crate::ExtendableBy<Self>,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceRayTracingPropertiesNVBuilder<'a> {
        PhysicalDeviceRayTracingPropertiesNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceRayTracingPropertiesNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceRayTracingPropertiesNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("shader_group_handle_size", &self.shader_group_handle_size)
            .field("max_recursion_depth", &self.max_recursion_depth)
            .field("max_shader_group_stride", &self.max_shader_group_stride)
            .field(
                "shader_group_base_alignment",
                &self.shader_group_base_alignment,
            )
            .field("max_geometry_count", &self.max_geometry_count)
            .field("max_instance_count", &self.max_instance_count)
            .field("max_triangle_count", &self.max_triangle_count)
            .field(
                "max_descriptor_set_acceleration_structures",
                &self.max_descriptor_set_acceleration_structures,
            )
            .finish()
    }
}
impl Default for PhysicalDeviceRayTracingPropertiesNV {
    fn default() -> PhysicalDeviceRayTracingPropertiesNV {
        PhysicalDeviceRayTracingPropertiesNV {
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
impl crate::ExtendableBy<PhysicalDeviceRayTracingPropertiesNV>
    for crate::vk1_1::PhysicalDeviceProperties2
{
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceRayTracingPropertiesNV.html) · Builder of [`PhysicalDeviceRayTracingPropertiesNV`](struct.PhysicalDeviceRayTracingPropertiesNV.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceRayTracingPropertiesNVBuilder<'a>(
    PhysicalDeviceRayTracingPropertiesNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceRayTracingPropertiesNVBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceRayTracingPropertiesNVBuilder<'a> {
        PhysicalDeviceRayTracingPropertiesNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_group_handle_size(mut self, shader_group_handle_size: u32) -> Self {
        self.0.shader_group_handle_size = shader_group_handle_size;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_recursion_depth(mut self, max_recursion_depth: u32) -> Self {
        self.0.max_recursion_depth = max_recursion_depth;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_shader_group_stride(mut self, max_shader_group_stride: u32) -> Self {
        self.0.max_shader_group_stride = max_shader_group_stride;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_group_base_alignment(mut self, shader_group_base_alignment: u32) -> Self {
        self.0.shader_group_base_alignment = shader_group_base_alignment;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_geometry_count(mut self, max_geometry_count: u64) -> Self {
        self.0.max_geometry_count = max_geometry_count;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_instance_count(mut self, max_instance_count: u64) -> Self {
        self.0.max_instance_count = max_instance_count;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_triangle_count(mut self, max_triangle_count: u64) -> Self {
        self.0.max_triangle_count = max_triangle_count;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_descriptor_set_acceleration_structures(
        mut self,
        max_descriptor_set_acceleration_structures: u32,
    ) -> Self {
        self.0.max_descriptor_set_acceleration_structures =
            max_descriptor_set_acceleration_structures;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceRayTracingPropertiesNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceRayTracingPropertiesNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkTransformMatrixNV.html) · Alias"]
pub type TransformMatrixNV = crate::extensions::khr_ray_tracing::TransformMatrixKHR;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAabbPositionsNV.html) · Alias"]
pub type AabbPositionsNV = crate::extensions::khr_ray_tracing::AabbPositionsKHR;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureInstanceNV.html) · Alias"]
pub type AccelerationStructureInstanceNV =
    crate::extensions::khr_ray_tracing::AccelerationStructureInstanceKHR;
