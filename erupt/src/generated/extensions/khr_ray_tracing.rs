# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_KHR_ray_tracing.html)\n\n## Extends\n- [`AccessFlagBits`](../../vk1_0/struct.AccessFlagBits.html)\n- [`BufferUsageFlagBits`](../../vk1_0/struct.BufferUsageFlagBits.html)\n- [`DebugReportObjectTypeEXT`](../../extensions/ext_debug_report/struct.DebugReportObjectTypeEXT.html)\n- [`DescriptorType`](../../vk1_0/struct.DescriptorType.html)\n- [`FormatFeatureFlagBits`](../../vk1_0/struct.FormatFeatureFlagBits.html)\n- [`IndexType`](../../vk1_0/struct.IndexType.html)\n- [`ObjectType`](../../vk1_0/struct.ObjectType.html)\n- [`PipelineBindPoint`](../../vk1_0/struct.PipelineBindPoint.html)\n- [`PipelineCreateFlagBits`](../../vk1_0/struct.PipelineCreateFlagBits.html)\n- [`PipelineStageFlagBits`](../../vk1_0/struct.PipelineStageFlagBits.html)\n- [`QueryType`](../../vk1_0/struct.QueryType.html)\n- [`Result`](../../vk1_0/struct.Result.html)\n- [`ShaderStageFlagBits`](../../vk1_0/struct.ShaderStageFlagBits.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_RAY_TRACING_SPEC_VERSION: u32 = 8;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_RAY_TRACING_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_ray_tracing");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const SHADER_UNUSED_KHR: u32 = !0u32;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateAccelerationStructureKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateAccelerationStructureKHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_create_info: *const crate::extensions::khr_ray_tracing::AccelerationStructureCreateInfoKHR,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
    p_acceleration_structure: *mut crate::extensions::khr_ray_tracing::AccelerationStructureKHR,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyAccelerationStructureKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyAccelerationStructureKHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    acceleration_structure: crate::extensions::khr_ray_tracing::AccelerationStructureKHR,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetAccelerationStructureMemoryRequirementsKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetAccelerationStructureMemoryRequirementsKHR = unsafe extern "system" fn ( device : crate :: vk1_0 :: Device , p_info : * const crate :: extensions :: khr_ray_tracing :: AccelerationStructureMemoryRequirementsInfoKHR , p_memory_requirements : * mut crate :: vk1_1 :: MemoryRequirements2 , ) -> std :: ffi :: c_void ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBindAccelerationStructureMemoryKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkBindAccelerationStructureMemoryKHR = unsafe extern "system" fn ( device : crate :: vk1_0 :: Device , bind_info_count : u32 , p_bind_infos : * const crate :: extensions :: khr_ray_tracing :: BindAccelerationStructureMemoryInfoKHR , ) -> crate :: vk1_0 :: Result ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBuildAccelerationStructureKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBuildAccelerationStructureKHR = unsafe extern "system" fn ( command_buffer : crate :: vk1_0 :: CommandBuffer , info_count : u32 , p_infos : * const crate :: extensions :: khr_ray_tracing :: AccelerationStructureBuildGeometryInfoKHR , pp_offset_infos : * const * const crate :: extensions :: khr_ray_tracing :: AccelerationStructureBuildOffsetInfoKHR , ) -> std :: ffi :: c_void ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBuildAccelerationStructureIndirectKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBuildAccelerationStructureIndirectKHR =
    unsafe extern "system" fn(
        command_buffer: crate::vk1_0::CommandBuffer,
        p_info: *const crate::extensions::khr_ray_tracing::AccelerationStructureBuildGeometryInfoKHR,
        indirect_buffer: crate::vk1_0::Buffer,
        indirect_offset: crate::vk1_0::DeviceSize,
        indirect_stride: u32,
    ) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBuildAccelerationStructureKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkBuildAccelerationStructureKHR = unsafe extern "system" fn ( device : crate :: vk1_0 :: Device , info_count : u32 , p_infos : * const crate :: extensions :: khr_ray_tracing :: AccelerationStructureBuildGeometryInfoKHR , pp_offset_infos : * const * const crate :: extensions :: khr_ray_tracing :: AccelerationStructureBuildOffsetInfoKHR , ) -> crate :: vk1_0 :: Result ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCopyAccelerationStructureKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCopyAccelerationStructureKHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_info: *const crate::extensions::khr_ray_tracing::CopyAccelerationStructureInfoKHR,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCopyAccelerationStructureToMemoryKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCopyAccelerationStructureToMemoryKHR =
    unsafe extern "system" fn(
        device: crate::vk1_0::Device,
        p_info: *const crate::extensions::khr_ray_tracing::CopyAccelerationStructureToMemoryInfoKHR,
    ) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCopyMemoryToAccelerationStructureKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCopyMemoryToAccelerationStructureKHR =
    unsafe extern "system" fn(
        device: crate::vk1_0::Device,
        p_info: *const crate::extensions::khr_ray_tracing::CopyMemoryToAccelerationStructureInfoKHR,
    ) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkWriteAccelerationStructuresPropertiesKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkWriteAccelerationStructuresPropertiesKHR = unsafe extern "system" fn ( device : crate :: vk1_0 :: Device , acceleration_structure_count : u32 , p_acceleration_structures : * const crate :: extensions :: khr_ray_tracing :: AccelerationStructureKHR , query_type : crate :: vk1_0 :: QueryType , data_size : usize , p_data : * mut std :: ffi :: c_void , stride : usize , ) -> crate :: vk1_0 :: Result ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyAccelerationStructureKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyAccelerationStructureKHR = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    p_info: *const crate::extensions::khr_ray_tracing::CopyAccelerationStructureInfoKHR,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyAccelerationStructureToMemoryKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyAccelerationStructureToMemoryKHR =
    unsafe extern "system" fn(
        command_buffer: crate::vk1_0::CommandBuffer,
        p_info: *const crate::extensions::khr_ray_tracing::CopyAccelerationStructureToMemoryInfoKHR,
    ) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyMemoryToAccelerationStructureKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyMemoryToAccelerationStructureKHR =
    unsafe extern "system" fn(
        command_buffer: crate::vk1_0::CommandBuffer,
        p_info: *const crate::extensions::khr_ray_tracing::CopyMemoryToAccelerationStructureInfoKHR,
    ) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdTraceRaysKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdTraceRaysKHR = unsafe extern "system" fn ( command_buffer : crate :: vk1_0 :: CommandBuffer , p_raygen_shader_binding_table : * const crate :: extensions :: khr_ray_tracing :: StridedBufferRegionKHR , p_miss_shader_binding_table : * const crate :: extensions :: khr_ray_tracing :: StridedBufferRegionKHR , p_hit_shader_binding_table : * const crate :: extensions :: khr_ray_tracing :: StridedBufferRegionKHR , p_callable_shader_binding_table : * const crate :: extensions :: khr_ray_tracing :: StridedBufferRegionKHR , width : u32 , height : u32 , depth : u32 , ) -> std :: ffi :: c_void ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateRayTracingPipelinesKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateRayTracingPipelinesKHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    pipeline_cache: crate::vk1_0::PipelineCache,
    create_info_count: u32,
    p_create_infos: *const crate::extensions::khr_ray_tracing::RayTracingPipelineCreateInfoKHR,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
    p_pipelines: *mut crate::vk1_0::Pipeline,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetRayTracingShaderGroupHandlesKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetRayTracingShaderGroupHandlesKHR =
    unsafe extern "system" fn(
        device: crate::vk1_0::Device,
        pipeline: crate::vk1_0::Pipeline,
        first_group: u32,
        group_count: u32,
        data_size: usize,
        p_data: *mut std::ffi::c_void,
    ) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetAccelerationStructureDeviceAddressKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetAccelerationStructureDeviceAddressKHR =
    unsafe extern "system" fn(
        device: crate::vk1_0::Device,
        p_info: *const crate::extensions::khr_ray_tracing::AccelerationStructureDeviceAddressInfoKHR,
    ) -> crate::vk1_2::DeviceAddress;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetRayTracingCaptureReplayShaderGroupHandlesKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetRayTracingCaptureReplayShaderGroupHandlesKHR =
    unsafe extern "system" fn(
        device: crate::vk1_0::Device,
        pipeline: crate::vk1_0::Pipeline,
        first_group: u32,
        group_count: u32,
        data_size: usize,
        p_data: *mut std::ffi::c_void,
    ) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdWriteAccelerationStructuresPropertiesKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdWriteAccelerationStructuresPropertiesKHR = unsafe extern "system" fn ( command_buffer : crate :: vk1_0 :: CommandBuffer , acceleration_structure_count : u32 , p_acceleration_structures : * const crate :: extensions :: khr_ray_tracing :: AccelerationStructureKHR , query_type : crate :: vk1_0 :: QueryType , query_pool : crate :: vk1_0 :: QueryPool , first_query : u32 , ) -> std :: ffi :: c_void ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdTraceRaysIndirectKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdTraceRaysIndirectKHR = unsafe extern "system" fn ( command_buffer : crate :: vk1_0 :: CommandBuffer , p_raygen_shader_binding_table : * const crate :: extensions :: khr_ray_tracing :: StridedBufferRegionKHR , p_miss_shader_binding_table : * const crate :: extensions :: khr_ray_tracing :: StridedBufferRegionKHR , p_hit_shader_binding_table : * const crate :: extensions :: khr_ray_tracing :: StridedBufferRegionKHR , p_callable_shader_binding_table : * const crate :: extensions :: khr_ray_tracing :: StridedBufferRegionKHR , buffer : crate :: vk1_0 :: Buffer , offset : crate :: vk1_0 :: DeviceSize , ) -> std :: ffi :: c_void ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceAccelerationStructureCompatibilityKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceAccelerationStructureCompatibilityKHR =
    unsafe extern "system" fn(
        device: crate::vk1_0::Device,
        version: *const crate::extensions::khr_ray_tracing::AccelerationStructureVersionKHR,
    ) -> crate::vk1_0::Result;
#[doc = "Provides Device Commands for [`KhrRayTracingDeviceLoaderExt`](trait.KhrRayTracingDeviceLoaderExt.html)"]
pub struct KhrRayTracingDeviceCommands {
    pub create_acceleration_structure_khr: PFN_vkCreateAccelerationStructureKHR,
    pub destroy_acceleration_structure_khr: PFN_vkDestroyAccelerationStructureKHR,
    pub get_acceleration_structure_memory_requirements_khr:
        PFN_vkGetAccelerationStructureMemoryRequirementsKHR,
    pub bind_acceleration_structure_memory_khr: PFN_vkBindAccelerationStructureMemoryKHR,
    pub cmd_build_acceleration_structure_khr: PFN_vkCmdBuildAccelerationStructureKHR,
    pub cmd_build_acceleration_structure_indirect_khr:
        PFN_vkCmdBuildAccelerationStructureIndirectKHR,
    pub build_acceleration_structure_khr: PFN_vkBuildAccelerationStructureKHR,
    pub copy_acceleration_structure_khr: PFN_vkCopyAccelerationStructureKHR,
    pub copy_acceleration_structure_to_memory_khr: PFN_vkCopyAccelerationStructureToMemoryKHR,
    pub copy_memory_to_acceleration_structure_khr: PFN_vkCopyMemoryToAccelerationStructureKHR,
    pub write_acceleration_structures_properties_khr:
        PFN_vkWriteAccelerationStructuresPropertiesKHR,
    pub cmd_copy_acceleration_structure_khr: PFN_vkCmdCopyAccelerationStructureKHR,
    pub cmd_copy_acceleration_structure_to_memory_khr:
        PFN_vkCmdCopyAccelerationStructureToMemoryKHR,
    pub cmd_copy_memory_to_acceleration_structure_khr:
        PFN_vkCmdCopyMemoryToAccelerationStructureKHR,
    pub cmd_trace_rays_khr: PFN_vkCmdTraceRaysKHR,
    pub create_ray_tracing_pipelines_khr: PFN_vkCreateRayTracingPipelinesKHR,
    pub get_ray_tracing_shader_group_handles_khr: PFN_vkGetRayTracingShaderGroupHandlesKHR,
    pub get_acceleration_structure_device_address_khr:
        PFN_vkGetAccelerationStructureDeviceAddressKHR,
    pub get_ray_tracing_capture_replay_shader_group_handles_khr:
        PFN_vkGetRayTracingCaptureReplayShaderGroupHandlesKHR,
    pub cmd_write_acceleration_structures_properties_khr:
        PFN_vkCmdWriteAccelerationStructuresPropertiesKHR,
    pub cmd_trace_rays_indirect_khr: PFN_vkCmdTraceRaysIndirectKHR,
    pub get_device_acceleration_structure_compatibility_khr:
        PFN_vkGetDeviceAccelerationStructureCompatibilityKHR,
}
impl KhrRayTracingDeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<KhrRayTracingDeviceCommands> {
        unsafe {
            Some(KhrRayTracingDeviceCommands {
                create_acceleration_structure_khr: std::mem::transmute(
                    loader.symbol("vkCreateAccelerationStructureKHR")?,
                ),
                destroy_acceleration_structure_khr: std::mem::transmute(
                    loader.symbol("vkDestroyAccelerationStructureKHR")?,
                ),
                get_acceleration_structure_memory_requirements_khr: std::mem::transmute(
                    loader.symbol("vkGetAccelerationStructureMemoryRequirementsKHR")?,
                ),
                bind_acceleration_structure_memory_khr: std::mem::transmute(
                    loader.symbol("vkBindAccelerationStructureMemoryKHR")?,
                ),
                cmd_build_acceleration_structure_khr: std::mem::transmute(
                    loader.symbol("vkCmdBuildAccelerationStructureKHR")?,
                ),
                cmd_build_acceleration_structure_indirect_khr: std::mem::transmute(
                    loader.symbol("vkCmdBuildAccelerationStructureIndirectKHR")?,
                ),
                build_acceleration_structure_khr: std::mem::transmute(
                    loader.symbol("vkBuildAccelerationStructureKHR")?,
                ),
                copy_acceleration_structure_khr: std::mem::transmute(
                    loader.symbol("vkCopyAccelerationStructureKHR")?,
                ),
                copy_acceleration_structure_to_memory_khr: std::mem::transmute(
                    loader.symbol("vkCopyAccelerationStructureToMemoryKHR")?,
                ),
                copy_memory_to_acceleration_structure_khr: std::mem::transmute(
                    loader.symbol("vkCopyMemoryToAccelerationStructureKHR")?,
                ),
                write_acceleration_structures_properties_khr: std::mem::transmute(
                    loader.symbol("vkWriteAccelerationStructuresPropertiesKHR")?,
                ),
                cmd_copy_acceleration_structure_khr: std::mem::transmute(
                    loader.symbol("vkCmdCopyAccelerationStructureKHR")?,
                ),
                cmd_copy_acceleration_structure_to_memory_khr: std::mem::transmute(
                    loader.symbol("vkCmdCopyAccelerationStructureToMemoryKHR")?,
                ),
                cmd_copy_memory_to_acceleration_structure_khr: std::mem::transmute(
                    loader.symbol("vkCmdCopyMemoryToAccelerationStructureKHR")?,
                ),
                cmd_trace_rays_khr: std::mem::transmute(loader.symbol("vkCmdTraceRaysKHR")?),
                create_ray_tracing_pipelines_khr: std::mem::transmute(
                    loader.symbol("vkCreateRayTracingPipelinesKHR")?,
                ),
                get_ray_tracing_shader_group_handles_khr: std::mem::transmute(
                    loader.symbol("vkGetRayTracingShaderGroupHandlesKHR")?,
                ),
                get_acceleration_structure_device_address_khr: std::mem::transmute(
                    loader.symbol("vkGetAccelerationStructureDeviceAddressKHR")?,
                ),
                get_ray_tracing_capture_replay_shader_group_handles_khr: std::mem::transmute(
                    loader.symbol("vkGetRayTracingCaptureReplayShaderGroupHandlesKHR")?,
                ),
                cmd_write_acceleration_structures_properties_khr: std::mem::transmute(
                    loader.symbol("vkCmdWriteAccelerationStructuresPropertiesKHR")?,
                ),
                cmd_trace_rays_indirect_khr: std::mem::transmute(
                    loader.symbol("vkCmdTraceRaysIndirectKHR")?,
                ),
                get_device_acceleration_structure_compatibility_khr: std::mem::transmute(
                    loader.symbol("vkGetDeviceAccelerationStructureCompatibilityKHR")?,
                ),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`KhrRayTracingDeviceCommands`](struct.KhrRayTracingDeviceCommands.html)"]
pub trait KhrRayTracingDeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateAccelerationStructureKHR.html) · Device Command"]
    unsafe fn create_acceleration_structure_khr(
        &self,
        create_info: &crate::extensions::khr_ray_tracing::AccelerationStructureCreateInfoKHR,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        acceleration_structure: Option<
            crate::extensions::khr_ray_tracing::AccelerationStructureKHR,
        >,
    ) -> crate::utils::VulkanResult<crate::extensions::khr_ray_tracing::AccelerationStructureKHR>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyAccelerationStructureKHR.html) · Device Command"]
    unsafe fn destroy_acceleration_structure_khr(
        &self,
        acceleration_structure: crate::extensions::khr_ray_tracing::AccelerationStructureKHR,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetAccelerationStructureMemoryRequirementsKHR.html) · Device Command"]
    unsafe fn get_acceleration_structure_memory_requirements_khr(
        &self,
        info: &crate::extensions::khr_ray_tracing::AccelerationStructureMemoryRequirementsInfoKHR,
        memory_requirements: Option<crate::vk1_1::MemoryRequirements2>,
    ) -> crate::vk1_1::MemoryRequirements2;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBindAccelerationStructureMemoryKHR.html) · Device Command"]
    unsafe fn bind_acceleration_structure_memory_khr(
        &self,
        bind_infos : & [ crate :: extensions :: khr_ray_tracing :: BindAccelerationStructureMemoryInfoKHRBuilder ],
    ) -> crate::utils::VulkanResult<()>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBuildAccelerationStructureKHR.html) · Device Command"]
    unsafe fn cmd_build_acceleration_structure_khr(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        infos : & [ crate :: extensions :: khr_ray_tracing :: AccelerationStructureBuildGeometryInfoKHRBuilder ],
        offset_infos : & [ * const crate :: extensions :: khr_ray_tracing :: AccelerationStructureBuildOffsetInfoKHRBuilder ],
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBuildAccelerationStructureIndirectKHR.html) · Device Command"]
    unsafe fn cmd_build_acceleration_structure_indirect_khr(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        info: &crate::extensions::khr_ray_tracing::AccelerationStructureBuildGeometryInfoKHR,
        indirect_buffer: crate::vk1_0::Buffer,
        indirect_offset: crate::vk1_0::DeviceSize,
        indirect_stride: u32,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBuildAccelerationStructureKHR.html) · Device Command"]
    unsafe fn build_acceleration_structure_khr(
        &self,
        infos : & [ crate :: extensions :: khr_ray_tracing :: AccelerationStructureBuildGeometryInfoKHRBuilder ],
        offset_infos : & [ * const crate :: extensions :: khr_ray_tracing :: AccelerationStructureBuildOffsetInfoKHRBuilder ],
    ) -> crate::utils::VulkanResult<()>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCopyAccelerationStructureKHR.html) · Device Command"]
    unsafe fn copy_acceleration_structure_khr(
        &self,
        info: &crate::extensions::khr_ray_tracing::CopyAccelerationStructureInfoKHR,
    ) -> crate::utils::VulkanResult<()>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCopyAccelerationStructureToMemoryKHR.html) · Device Command"]
    unsafe fn copy_acceleration_structure_to_memory_khr(
        &self,
        info: &crate::extensions::khr_ray_tracing::CopyAccelerationStructureToMemoryInfoKHR,
    ) -> crate::utils::VulkanResult<()>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCopyMemoryToAccelerationStructureKHR.html) · Device Command"]
    unsafe fn copy_memory_to_acceleration_structure_khr(
        &self,
        info: &crate::extensions::khr_ray_tracing::CopyMemoryToAccelerationStructureInfoKHR,
    ) -> crate::utils::VulkanResult<()>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkWriteAccelerationStructuresPropertiesKHR.html) · Device Command"]
    unsafe fn write_acceleration_structures_properties_khr(
        &self,
        acceleration_structures: &[crate::extensions::khr_ray_tracing::AccelerationStructureKHR],
        query_type: crate::vk1_0::QueryType,
        data_size: usize,
        data: *mut std::ffi::c_void,
        stride: usize,
    ) -> crate::utils::VulkanResult<()>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyAccelerationStructureKHR.html) · Device Command"]
    unsafe fn cmd_copy_acceleration_structure_khr(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        info: &crate::extensions::khr_ray_tracing::CopyAccelerationStructureInfoKHR,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyAccelerationStructureToMemoryKHR.html) · Device Command"]
    unsafe fn cmd_copy_acceleration_structure_to_memory_khr(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        info: &crate::extensions::khr_ray_tracing::CopyAccelerationStructureToMemoryInfoKHR,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyMemoryToAccelerationStructureKHR.html) · Device Command"]
    unsafe fn cmd_copy_memory_to_acceleration_structure_khr(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        info: &crate::extensions::khr_ray_tracing::CopyMemoryToAccelerationStructureInfoKHR,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdTraceRaysKHR.html) · Device Command"]
    unsafe fn cmd_trace_rays_khr(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        raygen_shader_binding_table: &crate::extensions::khr_ray_tracing::StridedBufferRegionKHR,
        miss_shader_binding_table: &crate::extensions::khr_ray_tracing::StridedBufferRegionKHR,
        hit_shader_binding_table: &crate::extensions::khr_ray_tracing::StridedBufferRegionKHR,
        callable_shader_binding_table: &crate::extensions::khr_ray_tracing::StridedBufferRegionKHR,
        width: u32,
        height: u32,
        depth: u32,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateRayTracingPipelinesKHR.html) · Device Command"]
    unsafe fn create_ray_tracing_pipelines_khr(
        &self,
        pipeline_cache: crate::vk1_0::PipelineCache,
        create_infos : & [ crate :: extensions :: khr_ray_tracing :: RayTracingPipelineCreateInfoKHRBuilder ],
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
    ) -> crate::utils::VulkanResult<Vec<crate::vk1_0::Pipeline>>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetRayTracingShaderGroupHandlesKHR.html) · Device Command"]
    unsafe fn get_ray_tracing_shader_group_handles_khr(
        &self,
        pipeline: crate::vk1_0::Pipeline,
        first_group: u32,
        group_count: u32,
        data_size: usize,
        data: *mut std::ffi::c_void,
    ) -> crate::utils::VulkanResult<()>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetAccelerationStructureDeviceAddressKHR.html) · Device Command"]
    unsafe fn get_acceleration_structure_device_address_khr(
        &self,
        info: &crate::extensions::khr_ray_tracing::AccelerationStructureDeviceAddressInfoKHR,
    ) -> crate::vk1_2::DeviceAddress;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetRayTracingCaptureReplayShaderGroupHandlesKHR.html) · Device Command"]
    unsafe fn get_ray_tracing_capture_replay_shader_group_handles_khr(
        &self,
        pipeline: crate::vk1_0::Pipeline,
        first_group: u32,
        group_count: u32,
        data_size: usize,
        data: *mut std::ffi::c_void,
    ) -> crate::utils::VulkanResult<()>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdWriteAccelerationStructuresPropertiesKHR.html) · Device Command"]
    unsafe fn cmd_write_acceleration_structures_properties_khr(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        acceleration_structures: &[crate::extensions::khr_ray_tracing::AccelerationStructureKHR],
        query_type: crate::vk1_0::QueryType,
        query_pool: crate::vk1_0::QueryPool,
        first_query: u32,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdTraceRaysIndirectKHR.html) · Device Command"]
    unsafe fn cmd_trace_rays_indirect_khr(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        raygen_shader_binding_table: &crate::extensions::khr_ray_tracing::StridedBufferRegionKHR,
        miss_shader_binding_table: &crate::extensions::khr_ray_tracing::StridedBufferRegionKHR,
        hit_shader_binding_table: &crate::extensions::khr_ray_tracing::StridedBufferRegionKHR,
        callable_shader_binding_table: &crate::extensions::khr_ray_tracing::StridedBufferRegionKHR,
        buffer: crate::vk1_0::Buffer,
        offset: crate::vk1_0::DeviceSize,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceAccelerationStructureCompatibilityKHR.html) · Device Command"]
    unsafe fn get_device_acceleration_structure_compatibility_khr(
        &self,
        version: &crate::extensions::khr_ray_tracing::AccelerationStructureVersionKHR,
    ) -> crate::utils::VulkanResult<()>;
}
impl KhrRayTracingDeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateAccelerationStructureKHR.html) · Device Command"]
    unsafe fn create_acceleration_structure_khr(
        &self,
        create_info: &crate::extensions::khr_ray_tracing::AccelerationStructureCreateInfoKHR,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        acceleration_structure: Option<
            crate::extensions::khr_ray_tracing::AccelerationStructureKHR,
        >,
    ) -> crate::utils::VulkanResult<crate::extensions::khr_ray_tracing::AccelerationStructureKHR>
    {
        let function = self
            .khr_ray_tracing
            .as_ref()
            .expect("`khr_ray_tracing` not loaded")
            .create_acceleration_structure_khr;
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
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyAccelerationStructureKHR.html) · Device Command"]
    unsafe fn destroy_acceleration_structure_khr(
        &self,
        acceleration_structure: crate::extensions::khr_ray_tracing::AccelerationStructureKHR,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
    ) -> () {
        let function = self
            .khr_ray_tracing
            .as_ref()
            .expect("`khr_ray_tracing` not loaded")
            .destroy_acceleration_structure_khr;
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
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetAccelerationStructureMemoryRequirementsKHR.html) · Device Command"]
    unsafe fn get_acceleration_structure_memory_requirements_khr(
        &self,
        info: &crate::extensions::khr_ray_tracing::AccelerationStructureMemoryRequirementsInfoKHR,
        memory_requirements: Option<crate::vk1_1::MemoryRequirements2>,
    ) -> crate::vk1_1::MemoryRequirements2 {
        let function = self
            .khr_ray_tracing
            .as_ref()
            .expect("`khr_ray_tracing` not loaded")
            .get_acceleration_structure_memory_requirements_khr;
        let mut memory_requirements = memory_requirements.unwrap_or_else(|| Default::default());
        let _val = function(self.handle, info, &mut memory_requirements);
        memory_requirements
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBindAccelerationStructureMemoryKHR.html) · Device Command"]
    unsafe fn bind_acceleration_structure_memory_khr(
        &self,
        bind_infos : & [ crate :: extensions :: khr_ray_tracing :: BindAccelerationStructureMemoryInfoKHRBuilder ],
    ) -> crate::utils::VulkanResult<()> {
        let function = self
            .khr_ray_tracing
            .as_ref()
            .expect("`khr_ray_tracing` not loaded")
            .bind_acceleration_structure_memory_khr;
        let bind_info_count = bind_infos.len() as _;
        let _val = function(self.handle, bind_info_count, bind_infos.as_ptr() as _);
        crate::utils::VulkanResult::new(_val, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBuildAccelerationStructureKHR.html) · Device Command"]
    unsafe fn cmd_build_acceleration_structure_khr(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        infos : & [ crate :: extensions :: khr_ray_tracing :: AccelerationStructureBuildGeometryInfoKHRBuilder ],
        offset_infos : & [ * const crate :: extensions :: khr_ray_tracing :: AccelerationStructureBuildOffsetInfoKHRBuilder ],
    ) -> () {
        let function = self
            .khr_ray_tracing
            .as_ref()
            .expect("`khr_ray_tracing` not loaded")
            .cmd_build_acceleration_structure_khr;
        let info_count = infos.len().min(offset_infos.len()) as _;
        let _val = function(
            command_buffer,
            info_count,
            infos.as_ptr() as _,
            offset_infos.as_ptr() as _,
        );
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBuildAccelerationStructureIndirectKHR.html) · Device Command"]
    unsafe fn cmd_build_acceleration_structure_indirect_khr(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        info: &crate::extensions::khr_ray_tracing::AccelerationStructureBuildGeometryInfoKHR,
        indirect_buffer: crate::vk1_0::Buffer,
        indirect_offset: crate::vk1_0::DeviceSize,
        indirect_stride: u32,
    ) -> () {
        let function = self
            .khr_ray_tracing
            .as_ref()
            .expect("`khr_ray_tracing` not loaded")
            .cmd_build_acceleration_structure_indirect_khr;
        let _val = function(
            command_buffer,
            info,
            indirect_buffer,
            indirect_offset,
            indirect_stride,
        );
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBuildAccelerationStructureKHR.html) · Device Command"]
    unsafe fn build_acceleration_structure_khr(
        &self,
        infos : & [ crate :: extensions :: khr_ray_tracing :: AccelerationStructureBuildGeometryInfoKHRBuilder ],
        offset_infos : & [ * const crate :: extensions :: khr_ray_tracing :: AccelerationStructureBuildOffsetInfoKHRBuilder ],
    ) -> crate::utils::VulkanResult<()> {
        let function = self
            .khr_ray_tracing
            .as_ref()
            .expect("`khr_ray_tracing` not loaded")
            .build_acceleration_structure_khr;
        let info_count = infos.len().min(offset_infos.len()) as _;
        let _val = function(
            self.handle,
            info_count,
            infos.as_ptr() as _,
            offset_infos.as_ptr() as _,
        );
        crate::utils::VulkanResult::new(_val, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCopyAccelerationStructureKHR.html) · Device Command"]
    unsafe fn copy_acceleration_structure_khr(
        &self,
        info: &crate::extensions::khr_ray_tracing::CopyAccelerationStructureInfoKHR,
    ) -> crate::utils::VulkanResult<()> {
        let function = self
            .khr_ray_tracing
            .as_ref()
            .expect("`khr_ray_tracing` not loaded")
            .copy_acceleration_structure_khr;
        let _val = function(self.handle, info);
        crate::utils::VulkanResult::new(_val, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCopyAccelerationStructureToMemoryKHR.html) · Device Command"]
    unsafe fn copy_acceleration_structure_to_memory_khr(
        &self,
        info: &crate::extensions::khr_ray_tracing::CopyAccelerationStructureToMemoryInfoKHR,
    ) -> crate::utils::VulkanResult<()> {
        let function = self
            .khr_ray_tracing
            .as_ref()
            .expect("`khr_ray_tracing` not loaded")
            .copy_acceleration_structure_to_memory_khr;
        let _val = function(self.handle, info);
        crate::utils::VulkanResult::new(_val, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCopyMemoryToAccelerationStructureKHR.html) · Device Command"]
    unsafe fn copy_memory_to_acceleration_structure_khr(
        &self,
        info: &crate::extensions::khr_ray_tracing::CopyMemoryToAccelerationStructureInfoKHR,
    ) -> crate::utils::VulkanResult<()> {
        let function = self
            .khr_ray_tracing
            .as_ref()
            .expect("`khr_ray_tracing` not loaded")
            .copy_memory_to_acceleration_structure_khr;
        let _val = function(self.handle, info);
        crate::utils::VulkanResult::new(_val, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkWriteAccelerationStructuresPropertiesKHR.html) · Device Command"]
    unsafe fn write_acceleration_structures_properties_khr(
        &self,
        acceleration_structures: &[crate::extensions::khr_ray_tracing::AccelerationStructureKHR],
        query_type: crate::vk1_0::QueryType,
        data_size: usize,
        data: *mut std::ffi::c_void,
        stride: usize,
    ) -> crate::utils::VulkanResult<()> {
        let function = self
            .khr_ray_tracing
            .as_ref()
            .expect("`khr_ray_tracing` not loaded")
            .write_acceleration_structures_properties_khr;
        let acceleration_structure_count = acceleration_structures.len() as _;
        let _val = function(
            self.handle,
            acceleration_structure_count,
            acceleration_structures.as_ptr() as _,
            query_type,
            data_size,
            data,
            stride,
        );
        crate::utils::VulkanResult::new(_val, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyAccelerationStructureKHR.html) · Device Command"]
    unsafe fn cmd_copy_acceleration_structure_khr(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        info: &crate::extensions::khr_ray_tracing::CopyAccelerationStructureInfoKHR,
    ) -> () {
        let function = self
            .khr_ray_tracing
            .as_ref()
            .expect("`khr_ray_tracing` not loaded")
            .cmd_copy_acceleration_structure_khr;
        let _val = function(command_buffer, info);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyAccelerationStructureToMemoryKHR.html) · Device Command"]
    unsafe fn cmd_copy_acceleration_structure_to_memory_khr(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        info: &crate::extensions::khr_ray_tracing::CopyAccelerationStructureToMemoryInfoKHR,
    ) -> () {
        let function = self
            .khr_ray_tracing
            .as_ref()
            .expect("`khr_ray_tracing` not loaded")
            .cmd_copy_acceleration_structure_to_memory_khr;
        let _val = function(command_buffer, info);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyMemoryToAccelerationStructureKHR.html) · Device Command"]
    unsafe fn cmd_copy_memory_to_acceleration_structure_khr(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        info: &crate::extensions::khr_ray_tracing::CopyMemoryToAccelerationStructureInfoKHR,
    ) -> () {
        let function = self
            .khr_ray_tracing
            .as_ref()
            .expect("`khr_ray_tracing` not loaded")
            .cmd_copy_memory_to_acceleration_structure_khr;
        let _val = function(command_buffer, info);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdTraceRaysKHR.html) · Device Command"]
    unsafe fn cmd_trace_rays_khr(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        raygen_shader_binding_table: &crate::extensions::khr_ray_tracing::StridedBufferRegionKHR,
        miss_shader_binding_table: &crate::extensions::khr_ray_tracing::StridedBufferRegionKHR,
        hit_shader_binding_table: &crate::extensions::khr_ray_tracing::StridedBufferRegionKHR,
        callable_shader_binding_table: &crate::extensions::khr_ray_tracing::StridedBufferRegionKHR,
        width: u32,
        height: u32,
        depth: u32,
    ) -> () {
        let function = self
            .khr_ray_tracing
            .as_ref()
            .expect("`khr_ray_tracing` not loaded")
            .cmd_trace_rays_khr;
        let _val = function(
            command_buffer,
            raygen_shader_binding_table,
            miss_shader_binding_table,
            hit_shader_binding_table,
            callable_shader_binding_table,
            width,
            height,
            depth,
        );
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateRayTracingPipelinesKHR.html) · Device Command"]
    unsafe fn create_ray_tracing_pipelines_khr(
        &self,
        pipeline_cache: crate::vk1_0::PipelineCache,
        create_infos : & [ crate :: extensions :: khr_ray_tracing :: RayTracingPipelineCreateInfoKHRBuilder ],
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
    ) -> crate::utils::VulkanResult<Vec<crate::vk1_0::Pipeline>> {
        let function = self
            .khr_ray_tracing
            .as_ref()
            .expect("`khr_ray_tracing` not loaded")
            .create_ray_tracing_pipelines_khr;
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
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetRayTracingShaderGroupHandlesKHR.html) · Device Command"]
    unsafe fn get_ray_tracing_shader_group_handles_khr(
        &self,
        pipeline: crate::vk1_0::Pipeline,
        first_group: u32,
        group_count: u32,
        data_size: usize,
        data: *mut std::ffi::c_void,
    ) -> crate::utils::VulkanResult<()> {
        let function = self
            .khr_ray_tracing
            .as_ref()
            .expect("`khr_ray_tracing` not loaded")
            .get_ray_tracing_shader_group_handles_khr;
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
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetAccelerationStructureDeviceAddressKHR.html) · Device Command"]
    unsafe fn get_acceleration_structure_device_address_khr(
        &self,
        info: &crate::extensions::khr_ray_tracing::AccelerationStructureDeviceAddressInfoKHR,
    ) -> crate::vk1_2::DeviceAddress {
        let function = self
            .khr_ray_tracing
            .as_ref()
            .expect("`khr_ray_tracing` not loaded")
            .get_acceleration_structure_device_address_khr;
        let _val = function(self.handle, info);
        _val
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetRayTracingCaptureReplayShaderGroupHandlesKHR.html) · Device Command"]
    unsafe fn get_ray_tracing_capture_replay_shader_group_handles_khr(
        &self,
        pipeline: crate::vk1_0::Pipeline,
        first_group: u32,
        group_count: u32,
        data_size: usize,
        data: *mut std::ffi::c_void,
    ) -> crate::utils::VulkanResult<()> {
        let function = self
            .khr_ray_tracing
            .as_ref()
            .expect("`khr_ray_tracing` not loaded")
            .get_ray_tracing_capture_replay_shader_group_handles_khr;
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
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdWriteAccelerationStructuresPropertiesKHR.html) · Device Command"]
    unsafe fn cmd_write_acceleration_structures_properties_khr(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        acceleration_structures: &[crate::extensions::khr_ray_tracing::AccelerationStructureKHR],
        query_type: crate::vk1_0::QueryType,
        query_pool: crate::vk1_0::QueryPool,
        first_query: u32,
    ) -> () {
        let function = self
            .khr_ray_tracing
            .as_ref()
            .expect("`khr_ray_tracing` not loaded")
            .cmd_write_acceleration_structures_properties_khr;
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
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdTraceRaysIndirectKHR.html) · Device Command"]
    unsafe fn cmd_trace_rays_indirect_khr(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        raygen_shader_binding_table: &crate::extensions::khr_ray_tracing::StridedBufferRegionKHR,
        miss_shader_binding_table: &crate::extensions::khr_ray_tracing::StridedBufferRegionKHR,
        hit_shader_binding_table: &crate::extensions::khr_ray_tracing::StridedBufferRegionKHR,
        callable_shader_binding_table: &crate::extensions::khr_ray_tracing::StridedBufferRegionKHR,
        buffer: crate::vk1_0::Buffer,
        offset: crate::vk1_0::DeviceSize,
    ) -> () {
        let function = self
            .khr_ray_tracing
            .as_ref()
            .expect("`khr_ray_tracing` not loaded")
            .cmd_trace_rays_indirect_khr;
        let _val = function(
            command_buffer,
            raygen_shader_binding_table,
            miss_shader_binding_table,
            hit_shader_binding_table,
            callable_shader_binding_table,
            buffer,
            offset,
        );
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceAccelerationStructureCompatibilityKHR.html) · Device Command"]
    unsafe fn get_device_acceleration_structure_compatibility_khr(
        &self,
        version: &crate::extensions::khr_ray_tracing::AccelerationStructureVersionKHR,
    ) -> crate::utils::VulkanResult<()> {
        let function = self
            .khr_ray_tracing
            .as_ref()
            .expect("`khr_ray_tracing` not loaded")
            .get_device_acceleration_structure_compatibility_khr;
        let _val = function(self.handle, version);
        crate::utils::VulkanResult::new(_val, ())
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureCreateInfoKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AccelerationStructureCreateInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub compacted_size: crate::vk1_0::DeviceSize,
    pub _type: crate::extensions::khr_ray_tracing::AccelerationStructureTypeKHR,
    pub flags: crate::extensions::khr_ray_tracing::BuildAccelerationStructureFlagsKHR,
    pub max_geometry_count: u32,
    pub p_geometry_infos:
        *const crate::extensions::khr_ray_tracing::AccelerationStructureCreateGeometryTypeInfoKHR,
    pub device_address: crate::vk1_2::DeviceAddress,
}
impl AccelerationStructureCreateInfoKHR {
    #[inline]
    pub fn builder<'a>(self) -> AccelerationStructureCreateInfoKHRBuilder<'a> {
        AccelerationStructureCreateInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for AccelerationStructureCreateInfoKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("AccelerationStructureCreateInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("compacted_size", &self.compacted_size)
            .field("_type", &self._type)
            .field("flags", &self.flags)
            .field("max_geometry_count", &self.max_geometry_count)
            .field("p_geometry_infos", &self.p_geometry_infos)
            .field("device_address", &self.device_address)
            .finish()
    }
}
impl Default for AccelerationStructureCreateInfoKHR {
    fn default() -> AccelerationStructureCreateInfoKHR {
        AccelerationStructureCreateInfoKHR {
            s_type: crate::vk1_0::StructureType::ACCELERATION_STRUCTURE_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            compacted_size: Default::default(),
            _type: Default::default(),
            flags: Default::default(),
            max_geometry_count: Default::default(),
            p_geometry_infos: std::ptr::null(),
            device_address: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`AccelerationStructureCreateInfoKHR`](struct.AccelerationStructureCreateInfoKHR.html)"]
#[repr(transparent)]
pub struct AccelerationStructureCreateInfoKHRBuilder<'a>(
    AccelerationStructureCreateInfoKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> AccelerationStructureCreateInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> AccelerationStructureCreateInfoKHRBuilder<'a> {
        AccelerationStructureCreateInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn compacted_size(mut self, compacted_size: crate::vk1_0::DeviceSize) -> Self {
        self.0.compacted_size = compacted_size;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn _type(
        mut self,
        _type: crate::extensions::khr_ray_tracing::AccelerationStructureTypeKHR,
    ) -> Self {
        self.0._type = _type;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn flags(
        mut self,
        flags: crate::extensions::khr_ray_tracing::BuildAccelerationStructureFlagsKHR,
    ) -> Self {
        self.0.flags = flags;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn geometry_infos(
        mut self,
        geometry_infos : &'a [ crate :: extensions :: khr_ray_tracing :: AccelerationStructureCreateGeometryTypeInfoKHRBuilder ],
    ) -> Self {
        self.0.max_geometry_count = geometry_infos.len() as _;
        self.0.p_geometry_infos = geometry_infos.as_ptr() as _;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn device_address(mut self, device_address: crate::vk1_2::DeviceAddress) -> Self {
        self.0.device_address = device_address;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> AccelerationStructureCreateInfoKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for AccelerationStructureCreateInfoKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for AccelerationStructureCreateInfoKHRBuilder<'a> {
    type Target = AccelerationStructureCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for AccelerationStructureCreateInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureTypeKHR.html) · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct AccelerationStructureTypeKHR(pub i32);
#[doc = "[Part of `extensions::khr_ray_tracing`](../../extensions/khr_ray_tracing/index.html)"]
impl AccelerationStructureTypeKHR {
    pub const TOP_LEVEL_KHR: Self = Self(0);
    pub const BOTTOM_LEVEL_KHR: Self = Self(1);
}
#[doc = "[Part of `extensions::nv_ray_tracing`](../../extensions/nv_ray_tracing/index.html)"]
impl AccelerationStructureTypeKHR {
    pub const TOP_LEVEL_NV: Self = Self::TOP_LEVEL_KHR;
    pub const BOTTOM_LEVEL_NV: Self = Self::BOTTOM_LEVEL_KHR;
}
impl std::fmt::Debug for AccelerationStructureTypeKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::TOP_LEVEL_KHR => "TOP_LEVEL_KHR",
            &Self::BOTTOM_LEVEL_KHR => "BOTTOM_LEVEL_KHR",
            _ => "(unknown)",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBuildAccelerationStructureFlagBitsKHR.html) · Flag Bits of [`BuildAccelerationStructureFlagsKHR`](struct.BuildAccelerationStructureFlagsKHR.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct BuildAccelerationStructureFlagBitsKHR(pub u32);
impl BuildAccelerationStructureFlagBitsKHR {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> BuildAccelerationStructureFlagsKHR {
        BuildAccelerationStructureFlagsKHR::from_bits_truncate(self.0)
    }
}
#[doc = "[Part of `extensions::khr_ray_tracing`](../../extensions/khr_ray_tracing/index.html)"]
impl BuildAccelerationStructureFlagBitsKHR {
    pub const ALLOW_UPDATE_KHR: Self = Self(0x00000001);
    pub const ALLOW_COMPACTION_KHR: Self = Self(0x00000002);
    pub const PREFER_FAST_TRACE_KHR: Self = Self(0x00000004);
    pub const PREFER_FAST_BUILD_KHR: Self = Self(0x00000008);
    pub const LOW_MEMORY_KHR: Self = Self(0x00000010);
}
#[doc = "[Part of `extensions::nv_ray_tracing`](../../extensions/nv_ray_tracing/index.html)"]
impl BuildAccelerationStructureFlagBitsKHR {
    pub const ALLOW_UPDATE_NV: Self = Self::ALLOW_UPDATE_KHR;
    pub const ALLOW_COMPACTION_NV: Self = Self::ALLOW_COMPACTION_KHR;
    pub const PREFER_FAST_TRACE_NV: Self = Self::PREFER_FAST_TRACE_KHR;
    pub const PREFER_FAST_BUILD_NV: Self = Self::PREFER_FAST_BUILD_KHR;
    pub const LOW_MEMORY_NV: Self = Self::LOW_MEMORY_KHR;
}
impl std::fmt::Debug for BuildAccelerationStructureFlagBitsKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::ALLOW_UPDATE_KHR => "ALLOW_UPDATE_KHR",
            &Self::ALLOW_COMPACTION_KHR => "ALLOW_COMPACTION_KHR",
            &Self::PREFER_FAST_TRACE_KHR => "PREFER_FAST_TRACE_KHR",
            &Self::PREFER_FAST_BUILD_KHR => "PREFER_FAST_BUILD_KHR",
            &Self::LOW_MEMORY_KHR => "LOW_MEMORY_KHR",
            _ => "(unknown)",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBuildAccelerationStructureFlagsKHR.html) · Flags of [`BuildAccelerationStructureFlagBitsKHR`](struct.BuildAccelerationStructureFlagBitsKHR.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct BuildAccelerationStructureFlagsKHR : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; const ALLOW_UPDATE_KHR = BuildAccelerationStructureFlagBitsKHR :: ALLOW_UPDATE_KHR . 0 ; const ALLOW_COMPACTION_KHR = BuildAccelerationStructureFlagBitsKHR :: ALLOW_COMPACTION_KHR . 0 ; const PREFER_FAST_TRACE_KHR = BuildAccelerationStructureFlagBitsKHR :: PREFER_FAST_TRACE_KHR . 0 ; const PREFER_FAST_BUILD_KHR = BuildAccelerationStructureFlagBitsKHR :: PREFER_FAST_BUILD_KHR . 0 ; const LOW_MEMORY_KHR = BuildAccelerationStructureFlagBitsKHR :: LOW_MEMORY_KHR . 0 ; const ALLOW_UPDATE_NV = BuildAccelerationStructureFlagBitsKHR :: ALLOW_UPDATE_NV . 0 ; const ALLOW_COMPACTION_NV = BuildAccelerationStructureFlagBitsKHR :: ALLOW_COMPACTION_NV . 0 ; const PREFER_FAST_TRACE_NV = BuildAccelerationStructureFlagBitsKHR :: PREFER_FAST_TRACE_NV . 0 ; const PREFER_FAST_BUILD_NV = BuildAccelerationStructureFlagBitsKHR :: PREFER_FAST_BUILD_NV . 0 ; const LOW_MEMORY_NV = BuildAccelerationStructureFlagBitsKHR :: LOW_MEMORY_NV . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureCreateGeometryTypeInfoKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AccelerationStructureCreateGeometryTypeInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub geometry_type: crate::extensions::khr_ray_tracing::GeometryTypeKHR,
    pub max_primitive_count: u32,
    pub index_type: crate::vk1_0::IndexType,
    pub max_vertex_count: u32,
    pub vertex_format: crate::vk1_0::Format,
    pub allows_transforms: crate::vk1_0::Bool32,
}
impl AccelerationStructureCreateGeometryTypeInfoKHR {
    #[inline]
    pub fn builder<'a>(self) -> AccelerationStructureCreateGeometryTypeInfoKHRBuilder<'a> {
        AccelerationStructureCreateGeometryTypeInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for AccelerationStructureCreateGeometryTypeInfoKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("AccelerationStructureCreateGeometryTypeInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("geometry_type", &self.geometry_type)
            .field("max_primitive_count", &self.max_primitive_count)
            .field("index_type", &self.index_type)
            .field("max_vertex_count", &self.max_vertex_count)
            .field("vertex_format", &self.vertex_format)
            .field("allows_transforms", &(self.allows_transforms != 0))
            .finish()
    }
}
impl Default for AccelerationStructureCreateGeometryTypeInfoKHR {
    fn default() -> AccelerationStructureCreateGeometryTypeInfoKHR {
        AccelerationStructureCreateGeometryTypeInfoKHR {
            s_type:
                crate::vk1_0::StructureType::ACCELERATION_STRUCTURE_CREATE_GEOMETRY_TYPE_INFO_KHR,
            p_next: std::ptr::null(),
            geometry_type: Default::default(),
            max_primitive_count: Default::default(),
            index_type: Default::default(),
            max_vertex_count: Default::default(),
            vertex_format: Default::default(),
            allows_transforms: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`AccelerationStructureCreateGeometryTypeInfoKHR`](struct.AccelerationStructureCreateGeometryTypeInfoKHR.html)"]
#[repr(transparent)]
pub struct AccelerationStructureCreateGeometryTypeInfoKHRBuilder<'a>(
    AccelerationStructureCreateGeometryTypeInfoKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> AccelerationStructureCreateGeometryTypeInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> AccelerationStructureCreateGeometryTypeInfoKHRBuilder<'a> {
        AccelerationStructureCreateGeometryTypeInfoKHRBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
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
    pub fn max_primitive_count(mut self, max_primitive_count: u32) -> Self {
        self.0.max_primitive_count = max_primitive_count;
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
    pub fn max_vertex_count(mut self, max_vertex_count: u32) -> Self {
        self.0.max_vertex_count = max_vertex_count;
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
    pub fn allows_transforms(mut self, allows_transforms: bool) -> Self {
        self.0.allows_transforms = allows_transforms as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> AccelerationStructureCreateGeometryTypeInfoKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for AccelerationStructureCreateGeometryTypeInfoKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for AccelerationStructureCreateGeometryTypeInfoKHRBuilder<'a> {
    type Target = AccelerationStructureCreateGeometryTypeInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for AccelerationStructureCreateGeometryTypeInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeometryTypeKHR.html) · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct GeometryTypeKHR(pub i32);
#[doc = "[Part of `extensions::khr_ray_tracing`](../../extensions/khr_ray_tracing/index.html)"]
impl GeometryTypeKHR {
    pub const TRIANGLES_KHR: Self = Self(0);
    pub const AABBS_KHR: Self = Self(1);
}
#[doc = "[Part of `extensions::khr_ray_tracing`](../../extensions/khr_ray_tracing/index.html)"]
impl GeometryTypeKHR {
    pub const INSTANCES_KHR: Self = Self(1000150000);
}
#[doc = "[Part of `extensions::nv_ray_tracing`](../../extensions/nv_ray_tracing/index.html)"]
impl GeometryTypeKHR {
    pub const TRIANGLES_NV: Self = Self::TRIANGLES_KHR;
    pub const AABBS_NV: Self = Self::AABBS_KHR;
}
impl std::fmt::Debug for GeometryTypeKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::TRIANGLES_KHR => "TRIANGLES_KHR",
            &Self::AABBS_KHR => "AABBS_KHR",
            &Self::INSTANCES_KHR => "INSTANCES_KHR",
            _ => "(unknown)",
        })
    }
}
crate :: non_dispatchable_handle ! ( AccelerationStructureKHR , ACCELERATION_STRUCTURE_KHR , doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureKHR.html) · Non-dispatchable Handle" ) ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureMemoryRequirementsInfoKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AccelerationStructureMemoryRequirementsInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub _type: crate::extensions::khr_ray_tracing::AccelerationStructureMemoryRequirementsTypeKHR,
    pub build_type: crate::extensions::khr_ray_tracing::AccelerationStructureBuildTypeKHR,
    pub acceleration_structure: crate::extensions::khr_ray_tracing::AccelerationStructureKHR,
}
impl AccelerationStructureMemoryRequirementsInfoKHR {
    #[inline]
    pub fn builder<'a>(self) -> AccelerationStructureMemoryRequirementsInfoKHRBuilder<'a> {
        AccelerationStructureMemoryRequirementsInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for AccelerationStructureMemoryRequirementsInfoKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("AccelerationStructureMemoryRequirementsInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("_type", &self._type)
            .field("build_type", &self.build_type)
            .field("acceleration_structure", &self.acceleration_structure)
            .finish()
    }
}
impl Default for AccelerationStructureMemoryRequirementsInfoKHR {
    fn default() -> AccelerationStructureMemoryRequirementsInfoKHR {
        AccelerationStructureMemoryRequirementsInfoKHR {
            s_type:
                crate::vk1_0::StructureType::ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO_KHR,
            p_next: std::ptr::null(),
            _type: Default::default(),
            build_type: Default::default(),
            acceleration_structure: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`AccelerationStructureMemoryRequirementsInfoKHR`](struct.AccelerationStructureMemoryRequirementsInfoKHR.html)"]
#[repr(transparent)]
pub struct AccelerationStructureMemoryRequirementsInfoKHRBuilder<'a>(
    AccelerationStructureMemoryRequirementsInfoKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> AccelerationStructureMemoryRequirementsInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> AccelerationStructureMemoryRequirementsInfoKHRBuilder<'a> {
        AccelerationStructureMemoryRequirementsInfoKHRBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn _type(
        mut self,
        _type: crate::extensions::khr_ray_tracing::AccelerationStructureMemoryRequirementsTypeKHR,
    ) -> Self {
        self.0._type = _type;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn build_type(
        mut self,
        build_type: crate::extensions::khr_ray_tracing::AccelerationStructureBuildTypeKHR,
    ) -> Self {
        self.0.build_type = build_type;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn acceleration_structure(
        mut self,
        acceleration_structure: crate::extensions::khr_ray_tracing::AccelerationStructureKHR,
    ) -> Self {
        self.0.acceleration_structure = acceleration_structure;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> AccelerationStructureMemoryRequirementsInfoKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for AccelerationStructureMemoryRequirementsInfoKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for AccelerationStructureMemoryRequirementsInfoKHRBuilder<'a> {
    type Target = AccelerationStructureMemoryRequirementsInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for AccelerationStructureMemoryRequirementsInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureMemoryRequirementsTypeKHR.html) · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct AccelerationStructureMemoryRequirementsTypeKHR(pub i32);
#[doc = "[Part of `extensions::khr_ray_tracing`](../../extensions/khr_ray_tracing/index.html)"]
impl AccelerationStructureMemoryRequirementsTypeKHR {
    pub const OBJECT_KHR: Self = Self(0);
    pub const BUILD_SCRATCH_KHR: Self = Self(1);
    pub const UPDATE_SCRATCH_KHR: Self = Self(2);
}
#[doc = "[Part of `extensions::nv_ray_tracing`](../../extensions/nv_ray_tracing/index.html)"]
impl AccelerationStructureMemoryRequirementsTypeKHR {
    pub const OBJECT_NV: Self = Self::OBJECT_KHR;
    pub const BUILD_SCRATCH_NV: Self = Self::BUILD_SCRATCH_KHR;
    pub const UPDATE_SCRATCH_NV: Self = Self::UPDATE_SCRATCH_KHR;
}
impl std::fmt::Debug for AccelerationStructureMemoryRequirementsTypeKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::OBJECT_KHR => "OBJECT_KHR",
            &Self::BUILD_SCRATCH_KHR => "BUILD_SCRATCH_KHR",
            &Self::UPDATE_SCRATCH_KHR => "UPDATE_SCRATCH_KHR",
            _ => "(unknown)",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureBuildTypeKHR.html) · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct AccelerationStructureBuildTypeKHR(pub i32);
#[doc = "[Part of `extensions::khr_ray_tracing`](../../extensions/khr_ray_tracing/index.html)"]
impl AccelerationStructureBuildTypeKHR {}
impl std::fmt::Debug for AccelerationStructureBuildTypeKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            _ => "(unknown)",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindAccelerationStructureMemoryInfoKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BindAccelerationStructureMemoryInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub acceleration_structure: crate::extensions::khr_ray_tracing::AccelerationStructureKHR,
    pub memory: crate::vk1_0::DeviceMemory,
    pub memory_offset: crate::vk1_0::DeviceSize,
    pub device_index_count: u32,
    pub p_device_indices: *const u32,
}
impl BindAccelerationStructureMemoryInfoKHR {
    #[inline]
    pub fn builder<'a>(self) -> BindAccelerationStructureMemoryInfoKHRBuilder<'a> {
        BindAccelerationStructureMemoryInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for BindAccelerationStructureMemoryInfoKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("BindAccelerationStructureMemoryInfoKHR")
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
impl Default for BindAccelerationStructureMemoryInfoKHR {
    fn default() -> BindAccelerationStructureMemoryInfoKHR {
        BindAccelerationStructureMemoryInfoKHR {
            s_type: crate::vk1_0::StructureType::BIND_ACCELERATION_STRUCTURE_MEMORY_INFO_KHR,
            p_next: std::ptr::null(),
            acceleration_structure: Default::default(),
            memory: Default::default(),
            memory_offset: Default::default(),
            device_index_count: Default::default(),
            p_device_indices: std::ptr::null(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`BindAccelerationStructureMemoryInfoKHR`](struct.BindAccelerationStructureMemoryInfoKHR.html)"]
#[repr(transparent)]
pub struct BindAccelerationStructureMemoryInfoKHRBuilder<'a>(
    BindAccelerationStructureMemoryInfoKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> BindAccelerationStructureMemoryInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> BindAccelerationStructureMemoryInfoKHRBuilder<'a> {
        BindAccelerationStructureMemoryInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn acceleration_structure(
        mut self,
        acceleration_structure: crate::extensions::khr_ray_tracing::AccelerationStructureKHR,
    ) -> Self {
        self.0.acceleration_structure = acceleration_structure;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn memory(mut self, memory: crate::vk1_0::DeviceMemory) -> Self {
        self.0.memory = memory;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn memory_offset(mut self, memory_offset: crate::vk1_0::DeviceSize) -> Self {
        self.0.memory_offset = memory_offset;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn device_indices(mut self, device_indices: &'a [u32]) -> Self {
        self.0.device_index_count = device_indices.len() as _;
        self.0.p_device_indices = device_indices.as_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> BindAccelerationStructureMemoryInfoKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for BindAccelerationStructureMemoryInfoKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for BindAccelerationStructureMemoryInfoKHRBuilder<'a> {
    type Target = BindAccelerationStructureMemoryInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for BindAccelerationStructureMemoryInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureBuildGeometryInfoKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AccelerationStructureBuildGeometryInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub _type: crate::extensions::khr_ray_tracing::AccelerationStructureTypeKHR,
    pub flags: crate::extensions::khr_ray_tracing::BuildAccelerationStructureFlagsKHR,
    pub update: crate::vk1_0::Bool32,
    pub src_acceleration_structure: crate::extensions::khr_ray_tracing::AccelerationStructureKHR,
    pub dst_acceleration_structure: crate::extensions::khr_ray_tracing::AccelerationStructureKHR,
    pub geometry_array_of_pointers: crate::vk1_0::Bool32,
    pub geometry_count: u32,
    pub pp_geometries:
        *const *const crate::extensions::khr_ray_tracing::AccelerationStructureGeometryKHR,
    pub scratch_data: crate::extensions::khr_ray_tracing::DeviceOrHostAddressKHR,
}
impl AccelerationStructureBuildGeometryInfoKHR {
    #[inline]
    pub fn builder<'a>(self) -> AccelerationStructureBuildGeometryInfoKHRBuilder<'a> {
        AccelerationStructureBuildGeometryInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for AccelerationStructureBuildGeometryInfoKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("AccelerationStructureBuildGeometryInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("_type", &self._type)
            .field("flags", &self.flags)
            .field("update", &(self.update != 0))
            .field(
                "src_acceleration_structure",
                &self.src_acceleration_structure,
            )
            .field(
                "dst_acceleration_structure",
                &self.dst_acceleration_structure,
            )
            .field(
                "geometry_array_of_pointers",
                &(self.geometry_array_of_pointers != 0),
            )
            .field("geometry_count", &self.geometry_count)
            .field("pp_geometries", &self.pp_geometries)
            .field("scratch_data", &self.scratch_data)
            .finish()
    }
}
impl Default for AccelerationStructureBuildGeometryInfoKHR {
    fn default() -> AccelerationStructureBuildGeometryInfoKHR {
        AccelerationStructureBuildGeometryInfoKHR {
            s_type: crate::vk1_0::StructureType::ACCELERATION_STRUCTURE_BUILD_GEOMETRY_INFO_KHR,
            p_next: std::ptr::null(),
            _type: Default::default(),
            flags: Default::default(),
            update: Default::default(),
            src_acceleration_structure: Default::default(),
            dst_acceleration_structure: Default::default(),
            geometry_array_of_pointers: Default::default(),
            geometry_count: Default::default(),
            pp_geometries: std::ptr::null(),
            scratch_data: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`AccelerationStructureBuildGeometryInfoKHR`](struct.AccelerationStructureBuildGeometryInfoKHR.html)"]
#[repr(transparent)]
pub struct AccelerationStructureBuildGeometryInfoKHRBuilder<'a>(
    AccelerationStructureBuildGeometryInfoKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> AccelerationStructureBuildGeometryInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> AccelerationStructureBuildGeometryInfoKHRBuilder<'a> {
        AccelerationStructureBuildGeometryInfoKHRBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn _type(
        mut self,
        _type: crate::extensions::khr_ray_tracing::AccelerationStructureTypeKHR,
    ) -> Self {
        self.0._type = _type;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn flags(
        mut self,
        flags: crate::extensions::khr_ray_tracing::BuildAccelerationStructureFlagsKHR,
    ) -> Self {
        self.0.flags = flags;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn update(mut self, update: bool) -> Self {
        self.0.update = update as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn src_acceleration_structure(
        mut self,
        src_acceleration_structure: crate::extensions::khr_ray_tracing::AccelerationStructureKHR,
    ) -> Self {
        self.0.src_acceleration_structure = src_acceleration_structure;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn dst_acceleration_structure(
        mut self,
        dst_acceleration_structure: crate::extensions::khr_ray_tracing::AccelerationStructureKHR,
    ) -> Self {
        self.0.dst_acceleration_structure = dst_acceleration_structure;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn geometry_array_of_pointers(mut self, geometry_array_of_pointers: bool) -> Self {
        self.0.geometry_array_of_pointers = geometry_array_of_pointers as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn geometry_count(mut self, geometry_count: u32) -> Self {
        self.0.geometry_count = geometry_count;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn geometries(
        mut self,
        geometries: &'a *const crate::extensions::khr_ray_tracing::AccelerationStructureGeometryKHR,
    ) -> Self {
        self.0.pp_geometries = geometries;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn scratch_data(
        mut self,
        scratch_data: crate::extensions::khr_ray_tracing::DeviceOrHostAddressKHR,
    ) -> Self {
        self.0.scratch_data = scratch_data;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> AccelerationStructureBuildGeometryInfoKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for AccelerationStructureBuildGeometryInfoKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for AccelerationStructureBuildGeometryInfoKHRBuilder<'a> {
    type Target = AccelerationStructureBuildGeometryInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for AccelerationStructureBuildGeometryInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureGeometryKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AccelerationStructureGeometryKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub geometry_type: crate::extensions::khr_ray_tracing::GeometryTypeKHR,
    pub geometry: crate::extensions::khr_ray_tracing::AccelerationStructureGeometryDataKHR,
    pub flags: crate::extensions::khr_ray_tracing::GeometryFlagsKHR,
}
impl AccelerationStructureGeometryKHR {
    #[inline]
    pub fn builder<'a>(self) -> AccelerationStructureGeometryKHRBuilder<'a> {
        AccelerationStructureGeometryKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for AccelerationStructureGeometryKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("AccelerationStructureGeometryKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("geometry_type", &self.geometry_type)
            .field("geometry", &self.geometry)
            .field("flags", &self.flags)
            .finish()
    }
}
impl Default for AccelerationStructureGeometryKHR {
    fn default() -> AccelerationStructureGeometryKHR {
        AccelerationStructureGeometryKHR {
            s_type: crate::vk1_0::StructureType::ACCELERATION_STRUCTURE_GEOMETRY_KHR,
            p_next: std::ptr::null(),
            geometry_type: Default::default(),
            geometry: Default::default(),
            flags: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`AccelerationStructureGeometryKHR`](struct.AccelerationStructureGeometryKHR.html)"]
#[repr(transparent)]
pub struct AccelerationStructureGeometryKHRBuilder<'a>(
    AccelerationStructureGeometryKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> AccelerationStructureGeometryKHRBuilder<'a> {
    #[inline]
    pub fn new() -> AccelerationStructureGeometryKHRBuilder<'a> {
        AccelerationStructureGeometryKHRBuilder(Default::default(), std::marker::PhantomData)
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
    pub fn geometry(
        mut self,
        geometry: crate::extensions::khr_ray_tracing::AccelerationStructureGeometryDataKHR,
    ) -> Self {
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
    pub unsafe fn discard(self) -> AccelerationStructureGeometryKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for AccelerationStructureGeometryKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for AccelerationStructureGeometryKHRBuilder<'a> {
    type Target = AccelerationStructureGeometryKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for AccelerationStructureGeometryKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureGeometryDataKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub union AccelerationStructureGeometryDataKHR {
    pub triangles:
        crate::extensions::khr_ray_tracing::AccelerationStructureGeometryTrianglesDataKHR,
    pub aabbs: crate::extensions::khr_ray_tracing::AccelerationStructureGeometryAabbsDataKHR,
    pub instances:
        crate::extensions::khr_ray_tracing::AccelerationStructureGeometryInstancesDataKHR,
}
impl std::fmt::Debug for AccelerationStructureGeometryDataKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("AccelerationStructureGeometryDataKHR")
            .finish()
    }
}
impl Default for AccelerationStructureGeometryDataKHR {
    fn default() -> AccelerationStructureGeometryDataKHR {
        unsafe { std::mem::zeroed() }
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureGeometryTrianglesDataKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AccelerationStructureGeometryTrianglesDataKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub vertex_format: crate::vk1_0::Format,
    pub vertex_data: crate::extensions::khr_ray_tracing::DeviceOrHostAddressConstKHR,
    pub vertex_stride: crate::vk1_0::DeviceSize,
    pub index_type: crate::vk1_0::IndexType,
    pub index_data: crate::extensions::khr_ray_tracing::DeviceOrHostAddressConstKHR,
    pub transform_data: crate::extensions::khr_ray_tracing::DeviceOrHostAddressConstKHR,
}
impl AccelerationStructureGeometryTrianglesDataKHR {
    #[inline]
    pub fn builder<'a>(self) -> AccelerationStructureGeometryTrianglesDataKHRBuilder<'a> {
        AccelerationStructureGeometryTrianglesDataKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for AccelerationStructureGeometryTrianglesDataKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("AccelerationStructureGeometryTrianglesDataKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("vertex_format", &self.vertex_format)
            .field("vertex_data", &self.vertex_data)
            .field("vertex_stride", &self.vertex_stride)
            .field("index_type", &self.index_type)
            .field("index_data", &self.index_data)
            .field("transform_data", &self.transform_data)
            .finish()
    }
}
impl Default for AccelerationStructureGeometryTrianglesDataKHR {
    fn default() -> AccelerationStructureGeometryTrianglesDataKHR {
        AccelerationStructureGeometryTrianglesDataKHR {
            s_type: crate::vk1_0::StructureType::ACCELERATION_STRUCTURE_GEOMETRY_TRIANGLES_DATA_KHR,
            p_next: std::ptr::null(),
            vertex_format: Default::default(),
            vertex_data: Default::default(),
            vertex_stride: Default::default(),
            index_type: Default::default(),
            index_data: Default::default(),
            transform_data: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`AccelerationStructureGeometryTrianglesDataKHR`](struct.AccelerationStructureGeometryTrianglesDataKHR.html)"]
#[repr(transparent)]
pub struct AccelerationStructureGeometryTrianglesDataKHRBuilder<'a>(
    AccelerationStructureGeometryTrianglesDataKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> AccelerationStructureGeometryTrianglesDataKHRBuilder<'a> {
    #[inline]
    pub fn new() -> AccelerationStructureGeometryTrianglesDataKHRBuilder<'a> {
        AccelerationStructureGeometryTrianglesDataKHRBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn vertex_format(mut self, vertex_format: crate::vk1_0::Format) -> Self {
        self.0.vertex_format = vertex_format;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn vertex_data(
        mut self,
        vertex_data: crate::extensions::khr_ray_tracing::DeviceOrHostAddressConstKHR,
    ) -> Self {
        self.0.vertex_data = vertex_data;
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
    pub fn index_type(mut self, index_type: crate::vk1_0::IndexType) -> Self {
        self.0.index_type = index_type;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn index_data(
        mut self,
        index_data: crate::extensions::khr_ray_tracing::DeviceOrHostAddressConstKHR,
    ) -> Self {
        self.0.index_data = index_data;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn transform_data(
        mut self,
        transform_data: crate::extensions::khr_ray_tracing::DeviceOrHostAddressConstKHR,
    ) -> Self {
        self.0.transform_data = transform_data;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> AccelerationStructureGeometryTrianglesDataKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for AccelerationStructureGeometryTrianglesDataKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for AccelerationStructureGeometryTrianglesDataKHRBuilder<'a> {
    type Target = AccelerationStructureGeometryTrianglesDataKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for AccelerationStructureGeometryTrianglesDataKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceOrHostAddressConstKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub union DeviceOrHostAddressConstKHR {
    pub device_address: crate::vk1_2::DeviceAddress,
    pub host_address: *const std::ffi::c_void,
}
impl std::fmt::Debug for DeviceOrHostAddressConstKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("DeviceOrHostAddressConstKHR").finish()
    }
}
impl Default for DeviceOrHostAddressConstKHR {
    fn default() -> DeviceOrHostAddressConstKHR {
        unsafe { std::mem::zeroed() }
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureGeometryAabbsDataKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AccelerationStructureGeometryAabbsDataKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub data: crate::extensions::khr_ray_tracing::DeviceOrHostAddressConstKHR,
    pub stride: crate::vk1_0::DeviceSize,
}
impl AccelerationStructureGeometryAabbsDataKHR {
    #[inline]
    pub fn builder<'a>(self) -> AccelerationStructureGeometryAabbsDataKHRBuilder<'a> {
        AccelerationStructureGeometryAabbsDataKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for AccelerationStructureGeometryAabbsDataKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("AccelerationStructureGeometryAabbsDataKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("data", &self.data)
            .field("stride", &self.stride)
            .finish()
    }
}
impl Default for AccelerationStructureGeometryAabbsDataKHR {
    fn default() -> AccelerationStructureGeometryAabbsDataKHR {
        AccelerationStructureGeometryAabbsDataKHR {
            s_type: crate::vk1_0::StructureType::ACCELERATION_STRUCTURE_GEOMETRY_AABBS_DATA_KHR,
            p_next: std::ptr::null(),
            data: Default::default(),
            stride: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`AccelerationStructureGeometryAabbsDataKHR`](struct.AccelerationStructureGeometryAabbsDataKHR.html)"]
#[repr(transparent)]
pub struct AccelerationStructureGeometryAabbsDataKHRBuilder<'a>(
    AccelerationStructureGeometryAabbsDataKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> AccelerationStructureGeometryAabbsDataKHRBuilder<'a> {
    #[inline]
    pub fn new() -> AccelerationStructureGeometryAabbsDataKHRBuilder<'a> {
        AccelerationStructureGeometryAabbsDataKHRBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn data(
        mut self,
        data: crate::extensions::khr_ray_tracing::DeviceOrHostAddressConstKHR,
    ) -> Self {
        self.0.data = data;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn stride(mut self, stride: crate::vk1_0::DeviceSize) -> Self {
        self.0.stride = stride;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> AccelerationStructureGeometryAabbsDataKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for AccelerationStructureGeometryAabbsDataKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for AccelerationStructureGeometryAabbsDataKHRBuilder<'a> {
    type Target = AccelerationStructureGeometryAabbsDataKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for AccelerationStructureGeometryAabbsDataKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureGeometryInstancesDataKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AccelerationStructureGeometryInstancesDataKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub array_of_pointers: crate::vk1_0::Bool32,
    pub data: crate::extensions::khr_ray_tracing::DeviceOrHostAddressConstKHR,
}
impl AccelerationStructureGeometryInstancesDataKHR {
    #[inline]
    pub fn builder<'a>(self) -> AccelerationStructureGeometryInstancesDataKHRBuilder<'a> {
        AccelerationStructureGeometryInstancesDataKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for AccelerationStructureGeometryInstancesDataKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("AccelerationStructureGeometryInstancesDataKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("array_of_pointers", &(self.array_of_pointers != 0))
            .field("data", &self.data)
            .finish()
    }
}
impl Default for AccelerationStructureGeometryInstancesDataKHR {
    fn default() -> AccelerationStructureGeometryInstancesDataKHR {
        AccelerationStructureGeometryInstancesDataKHR {
            s_type: crate::vk1_0::StructureType::ACCELERATION_STRUCTURE_GEOMETRY_INSTANCES_DATA_KHR,
            p_next: std::ptr::null(),
            array_of_pointers: Default::default(),
            data: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`AccelerationStructureGeometryInstancesDataKHR`](struct.AccelerationStructureGeometryInstancesDataKHR.html)"]
#[repr(transparent)]
pub struct AccelerationStructureGeometryInstancesDataKHRBuilder<'a>(
    AccelerationStructureGeometryInstancesDataKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> AccelerationStructureGeometryInstancesDataKHRBuilder<'a> {
    #[inline]
    pub fn new() -> AccelerationStructureGeometryInstancesDataKHRBuilder<'a> {
        AccelerationStructureGeometryInstancesDataKHRBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn array_of_pointers(mut self, array_of_pointers: bool) -> Self {
        self.0.array_of_pointers = array_of_pointers as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn data(
        mut self,
        data: crate::extensions::khr_ray_tracing::DeviceOrHostAddressConstKHR,
    ) -> Self {
        self.0.data = data;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> AccelerationStructureGeometryInstancesDataKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for AccelerationStructureGeometryInstancesDataKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for AccelerationStructureGeometryInstancesDataKHRBuilder<'a> {
    type Target = AccelerationStructureGeometryInstancesDataKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for AccelerationStructureGeometryInstancesDataKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeometryFlagBitsKHR.html) · Flag Bits of [`GeometryFlagsKHR`](struct.GeometryFlagsKHR.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct GeometryFlagBitsKHR(pub u32);
impl GeometryFlagBitsKHR {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> GeometryFlagsKHR {
        GeometryFlagsKHR::from_bits_truncate(self.0)
    }
}
#[doc = "[Part of `extensions::khr_ray_tracing`](../../extensions/khr_ray_tracing/index.html)"]
impl GeometryFlagBitsKHR {
    pub const OPAQUE_KHR: Self = Self(0x00000001);
    pub const NO_DUPLICATE_ANY_HIT_INVOCATION_KHR: Self = Self(0x00000002);
}
#[doc = "[Part of `extensions::nv_ray_tracing`](../../extensions/nv_ray_tracing/index.html)"]
impl GeometryFlagBitsKHR {
    pub const OPAQUE_NV: Self = Self::OPAQUE_KHR;
    pub const NO_DUPLICATE_ANY_HIT_INVOCATION_NV: Self = Self::NO_DUPLICATE_ANY_HIT_INVOCATION_KHR;
}
impl std::fmt::Debug for GeometryFlagBitsKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::OPAQUE_KHR => "OPAQUE_KHR",
            &Self::NO_DUPLICATE_ANY_HIT_INVOCATION_KHR => "NO_DUPLICATE_ANY_HIT_INVOCATION_KHR",
            _ => "(unknown)",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeometryFlagsKHR.html) · Flags of [`GeometryFlagBitsKHR`](struct.GeometryFlagBitsKHR.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct GeometryFlagsKHR : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; const OPAQUE_KHR = GeometryFlagBitsKHR :: OPAQUE_KHR . 0 ; const NO_DUPLICATE_ANY_HIT_INVOCATION_KHR = GeometryFlagBitsKHR :: NO_DUPLICATE_ANY_HIT_INVOCATION_KHR . 0 ; const OPAQUE_NV = GeometryFlagBitsKHR :: OPAQUE_NV . 0 ; const NO_DUPLICATE_ANY_HIT_INVOCATION_NV = GeometryFlagBitsKHR :: NO_DUPLICATE_ANY_HIT_INVOCATION_NV . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceOrHostAddressKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub union DeviceOrHostAddressKHR {
    pub device_address: crate::vk1_2::DeviceAddress,
    pub host_address: *mut std::ffi::c_void,
}
impl std::fmt::Debug for DeviceOrHostAddressKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("DeviceOrHostAddressKHR").finish()
    }
}
impl Default for DeviceOrHostAddressKHR {
    fn default() -> DeviceOrHostAddressKHR {
        unsafe { std::mem::zeroed() }
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureBuildOffsetInfoKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AccelerationStructureBuildOffsetInfoKHR {
    pub primitive_count: u32,
    pub primitive_offset: u32,
    pub first_vertex: u32,
    pub transform_offset: u32,
}
impl AccelerationStructureBuildOffsetInfoKHR {
    #[inline]
    pub fn builder<'a>(self) -> AccelerationStructureBuildOffsetInfoKHRBuilder<'a> {
        AccelerationStructureBuildOffsetInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for AccelerationStructureBuildOffsetInfoKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("AccelerationStructureBuildOffsetInfoKHR")
            .field("primitive_count", &self.primitive_count)
            .field("primitive_offset", &self.primitive_offset)
            .field("first_vertex", &self.first_vertex)
            .field("transform_offset", &self.transform_offset)
            .finish()
    }
}
impl Default for AccelerationStructureBuildOffsetInfoKHR {
    fn default() -> AccelerationStructureBuildOffsetInfoKHR {
        AccelerationStructureBuildOffsetInfoKHR {
            primitive_count: Default::default(),
            primitive_offset: Default::default(),
            first_vertex: Default::default(),
            transform_offset: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`AccelerationStructureBuildOffsetInfoKHR`](struct.AccelerationStructureBuildOffsetInfoKHR.html)"]
#[repr(transparent)]
pub struct AccelerationStructureBuildOffsetInfoKHRBuilder<'a>(
    AccelerationStructureBuildOffsetInfoKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> AccelerationStructureBuildOffsetInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> AccelerationStructureBuildOffsetInfoKHRBuilder<'a> {
        AccelerationStructureBuildOffsetInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn primitive_count(mut self, primitive_count: u32) -> Self {
        self.0.primitive_count = primitive_count;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn primitive_offset(mut self, primitive_offset: u32) -> Self {
        self.0.primitive_offset = primitive_offset;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn first_vertex(mut self, first_vertex: u32) -> Self {
        self.0.first_vertex = first_vertex;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn transform_offset(mut self, transform_offset: u32) -> Self {
        self.0.transform_offset = transform_offset;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> AccelerationStructureBuildOffsetInfoKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for AccelerationStructureBuildOffsetInfoKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for AccelerationStructureBuildOffsetInfoKHRBuilder<'a> {
    type Target = AccelerationStructureBuildOffsetInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for AccelerationStructureBuildOffsetInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCopyAccelerationStructureInfoKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CopyAccelerationStructureInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub src: crate::extensions::khr_ray_tracing::AccelerationStructureKHR,
    pub dst: crate::extensions::khr_ray_tracing::AccelerationStructureKHR,
    pub mode: crate::extensions::khr_ray_tracing::CopyAccelerationStructureModeKHR,
}
impl CopyAccelerationStructureInfoKHR {
    #[inline]
    pub fn builder<'a>(self) -> CopyAccelerationStructureInfoKHRBuilder<'a> {
        CopyAccelerationStructureInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for CopyAccelerationStructureInfoKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("CopyAccelerationStructureInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("src", &self.src)
            .field("dst", &self.dst)
            .field("mode", &self.mode)
            .finish()
    }
}
impl Default for CopyAccelerationStructureInfoKHR {
    fn default() -> CopyAccelerationStructureInfoKHR {
        CopyAccelerationStructureInfoKHR {
            s_type: crate::vk1_0::StructureType::COPY_ACCELERATION_STRUCTURE_INFO_KHR,
            p_next: std::ptr::null(),
            src: Default::default(),
            dst: Default::default(),
            mode: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`CopyAccelerationStructureInfoKHR`](struct.CopyAccelerationStructureInfoKHR.html)"]
#[repr(transparent)]
pub struct CopyAccelerationStructureInfoKHRBuilder<'a>(
    CopyAccelerationStructureInfoKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> CopyAccelerationStructureInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> CopyAccelerationStructureInfoKHRBuilder<'a> {
        CopyAccelerationStructureInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn src(
        mut self,
        src: crate::extensions::khr_ray_tracing::AccelerationStructureKHR,
    ) -> Self {
        self.0.src = src;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn dst(
        mut self,
        dst: crate::extensions::khr_ray_tracing::AccelerationStructureKHR,
    ) -> Self {
        self.0.dst = dst;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn mode(
        mut self,
        mode: crate::extensions::khr_ray_tracing::CopyAccelerationStructureModeKHR,
    ) -> Self {
        self.0.mode = mode;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> CopyAccelerationStructureInfoKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for CopyAccelerationStructureInfoKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for CopyAccelerationStructureInfoKHRBuilder<'a> {
    type Target = CopyAccelerationStructureInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for CopyAccelerationStructureInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCopyAccelerationStructureModeKHR.html) · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct CopyAccelerationStructureModeKHR(pub i32);
#[doc = "[Part of `extensions::khr_ray_tracing`](../../extensions/khr_ray_tracing/index.html)"]
impl CopyAccelerationStructureModeKHR {
    pub const CLONE_KHR: Self = Self(0);
    pub const COMPACT_KHR: Self = Self(1);
    pub const SERIALIZE_KHR: Self = Self(2);
    pub const DESERIALIZE_KHR: Self = Self(3);
}
#[doc = "[Part of `extensions::nv_ray_tracing`](../../extensions/nv_ray_tracing/index.html)"]
impl CopyAccelerationStructureModeKHR {
    pub const CLONE_NV: Self = Self::CLONE_KHR;
    pub const COMPACT_NV: Self = Self::COMPACT_KHR;
}
impl std::fmt::Debug for CopyAccelerationStructureModeKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::CLONE_KHR => "CLONE_KHR",
            &Self::COMPACT_KHR => "COMPACT_KHR",
            &Self::SERIALIZE_KHR => "SERIALIZE_KHR",
            &Self::DESERIALIZE_KHR => "DESERIALIZE_KHR",
            _ => "(unknown)",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCopyAccelerationStructureToMemoryInfoKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CopyAccelerationStructureToMemoryInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub src: crate::extensions::khr_ray_tracing::AccelerationStructureKHR,
    pub dst: crate::extensions::khr_ray_tracing::DeviceOrHostAddressKHR,
    pub mode: crate::extensions::khr_ray_tracing::CopyAccelerationStructureModeKHR,
}
impl CopyAccelerationStructureToMemoryInfoKHR {
    #[inline]
    pub fn builder<'a>(self) -> CopyAccelerationStructureToMemoryInfoKHRBuilder<'a> {
        CopyAccelerationStructureToMemoryInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for CopyAccelerationStructureToMemoryInfoKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("CopyAccelerationStructureToMemoryInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("src", &self.src)
            .field("dst", &self.dst)
            .field("mode", &self.mode)
            .finish()
    }
}
impl Default for CopyAccelerationStructureToMemoryInfoKHR {
    fn default() -> CopyAccelerationStructureToMemoryInfoKHR {
        CopyAccelerationStructureToMemoryInfoKHR {
            s_type: crate::vk1_0::StructureType::COPY_ACCELERATION_STRUCTURE_TO_MEMORY_INFO_KHR,
            p_next: std::ptr::null(),
            src: Default::default(),
            dst: Default::default(),
            mode: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`CopyAccelerationStructureToMemoryInfoKHR`](struct.CopyAccelerationStructureToMemoryInfoKHR.html)"]
#[repr(transparent)]
pub struct CopyAccelerationStructureToMemoryInfoKHRBuilder<'a>(
    CopyAccelerationStructureToMemoryInfoKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> CopyAccelerationStructureToMemoryInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> CopyAccelerationStructureToMemoryInfoKHRBuilder<'a> {
        CopyAccelerationStructureToMemoryInfoKHRBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn src(
        mut self,
        src: crate::extensions::khr_ray_tracing::AccelerationStructureKHR,
    ) -> Self {
        self.0.src = src;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn dst(mut self, dst: crate::extensions::khr_ray_tracing::DeviceOrHostAddressKHR) -> Self {
        self.0.dst = dst;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn mode(
        mut self,
        mode: crate::extensions::khr_ray_tracing::CopyAccelerationStructureModeKHR,
    ) -> Self {
        self.0.mode = mode;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> CopyAccelerationStructureToMemoryInfoKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for CopyAccelerationStructureToMemoryInfoKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for CopyAccelerationStructureToMemoryInfoKHRBuilder<'a> {
    type Target = CopyAccelerationStructureToMemoryInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for CopyAccelerationStructureToMemoryInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCopyMemoryToAccelerationStructureInfoKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CopyMemoryToAccelerationStructureInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub src: crate::extensions::khr_ray_tracing::DeviceOrHostAddressConstKHR,
    pub dst: crate::extensions::khr_ray_tracing::AccelerationStructureKHR,
    pub mode: crate::extensions::khr_ray_tracing::CopyAccelerationStructureModeKHR,
}
impl CopyMemoryToAccelerationStructureInfoKHR {
    #[inline]
    pub fn builder<'a>(self) -> CopyMemoryToAccelerationStructureInfoKHRBuilder<'a> {
        CopyMemoryToAccelerationStructureInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for CopyMemoryToAccelerationStructureInfoKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("CopyMemoryToAccelerationStructureInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("src", &self.src)
            .field("dst", &self.dst)
            .field("mode", &self.mode)
            .finish()
    }
}
impl Default for CopyMemoryToAccelerationStructureInfoKHR {
    fn default() -> CopyMemoryToAccelerationStructureInfoKHR {
        CopyMemoryToAccelerationStructureInfoKHR {
            s_type: crate::vk1_0::StructureType::COPY_MEMORY_TO_ACCELERATION_STRUCTURE_INFO_KHR,
            p_next: std::ptr::null(),
            src: Default::default(),
            dst: Default::default(),
            mode: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`CopyMemoryToAccelerationStructureInfoKHR`](struct.CopyMemoryToAccelerationStructureInfoKHR.html)"]
#[repr(transparent)]
pub struct CopyMemoryToAccelerationStructureInfoKHRBuilder<'a>(
    CopyMemoryToAccelerationStructureInfoKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> CopyMemoryToAccelerationStructureInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> CopyMemoryToAccelerationStructureInfoKHRBuilder<'a> {
        CopyMemoryToAccelerationStructureInfoKHRBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn src(
        mut self,
        src: crate::extensions::khr_ray_tracing::DeviceOrHostAddressConstKHR,
    ) -> Self {
        self.0.src = src;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn dst(
        mut self,
        dst: crate::extensions::khr_ray_tracing::AccelerationStructureKHR,
    ) -> Self {
        self.0.dst = dst;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn mode(
        mut self,
        mode: crate::extensions::khr_ray_tracing::CopyAccelerationStructureModeKHR,
    ) -> Self {
        self.0.mode = mode;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> CopyMemoryToAccelerationStructureInfoKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for CopyMemoryToAccelerationStructureInfoKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for CopyMemoryToAccelerationStructureInfoKHRBuilder<'a> {
    type Target = CopyMemoryToAccelerationStructureInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for CopyMemoryToAccelerationStructureInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkStridedBufferRegionKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StridedBufferRegionKHR {
    pub buffer: crate::vk1_0::Buffer,
    pub offset: crate::vk1_0::DeviceSize,
    pub stride: crate::vk1_0::DeviceSize,
    pub size: crate::vk1_0::DeviceSize,
}
impl StridedBufferRegionKHR {
    #[inline]
    pub fn builder<'a>(self) -> StridedBufferRegionKHRBuilder<'a> {
        StridedBufferRegionKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for StridedBufferRegionKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("StridedBufferRegionKHR")
            .field("buffer", &self.buffer)
            .field("offset", &self.offset)
            .field("stride", &self.stride)
            .field("size", &self.size)
            .finish()
    }
}
impl Default for StridedBufferRegionKHR {
    fn default() -> StridedBufferRegionKHR {
        StridedBufferRegionKHR {
            buffer: Default::default(),
            offset: Default::default(),
            stride: Default::default(),
            size: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`StridedBufferRegionKHR`](struct.StridedBufferRegionKHR.html)"]
#[repr(transparent)]
pub struct StridedBufferRegionKHRBuilder<'a>(
    StridedBufferRegionKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> StridedBufferRegionKHRBuilder<'a> {
    #[inline]
    pub fn new() -> StridedBufferRegionKHRBuilder<'a> {
        StridedBufferRegionKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn buffer(mut self, buffer: crate::vk1_0::Buffer) -> Self {
        self.0.buffer = buffer;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn offset(mut self, offset: crate::vk1_0::DeviceSize) -> Self {
        self.0.offset = offset;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn stride(mut self, stride: crate::vk1_0::DeviceSize) -> Self {
        self.0.stride = stride;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn size(mut self, size: crate::vk1_0::DeviceSize) -> Self {
        self.0.size = size;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> StridedBufferRegionKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for StridedBufferRegionKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for StridedBufferRegionKHRBuilder<'a> {
    type Target = StridedBufferRegionKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StridedBufferRegionKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRayTracingPipelineCreateInfoKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RayTracingPipelineCreateInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::vk1_0::PipelineCreateFlags,
    pub stage_count: u32,
    pub p_stages: *const crate::vk1_0::PipelineShaderStageCreateInfo,
    pub group_count: u32,
    pub p_groups: *const crate::extensions::khr_ray_tracing::RayTracingShaderGroupCreateInfoKHR,
    pub max_recursion_depth: u32,
    pub libraries: crate::extensions::khr_pipeline_library::PipelineLibraryCreateInfoKHR,
    pub p_library_interface:
        *const crate::extensions::khr_ray_tracing::RayTracingPipelineInterfaceCreateInfoKHR,
    pub layout: crate::vk1_0::PipelineLayout,
    pub base_pipeline_handle: crate::vk1_0::Pipeline,
    pub base_pipeline_index: i32,
}
impl RayTracingPipelineCreateInfoKHR {
    #[inline]
    pub fn builder<'a>(self) -> RayTracingPipelineCreateInfoKHRBuilder<'a> {
        RayTracingPipelineCreateInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for RayTracingPipelineCreateInfoKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("RayTracingPipelineCreateInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("stage_count", &self.stage_count)
            .field("p_stages", &self.p_stages)
            .field("group_count", &self.group_count)
            .field("p_groups", &self.p_groups)
            .field("max_recursion_depth", &self.max_recursion_depth)
            .field("libraries", &self.libraries)
            .field("p_library_interface", &self.p_library_interface)
            .field("layout", &self.layout)
            .field("base_pipeline_handle", &self.base_pipeline_handle)
            .field("base_pipeline_index", &self.base_pipeline_index)
            .finish()
    }
}
impl Default for RayTracingPipelineCreateInfoKHR {
    fn default() -> RayTracingPipelineCreateInfoKHR {
        RayTracingPipelineCreateInfoKHR {
            s_type: crate::vk1_0::StructureType::RAY_TRACING_PIPELINE_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            flags: Default::default(),
            stage_count: Default::default(),
            p_stages: std::ptr::null(),
            group_count: Default::default(),
            p_groups: std::ptr::null(),
            max_recursion_depth: Default::default(),
            libraries: Default::default(),
            p_library_interface: std::ptr::null(),
            layout: Default::default(),
            base_pipeline_handle: Default::default(),
            base_pipeline_index: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`RayTracingPipelineCreateInfoKHR`](struct.RayTracingPipelineCreateInfoKHR.html)"]
#[repr(transparent)]
pub struct RayTracingPipelineCreateInfoKHRBuilder<'a>(
    RayTracingPipelineCreateInfoKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> RayTracingPipelineCreateInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> RayTracingPipelineCreateInfoKHRBuilder<'a> {
        RayTracingPipelineCreateInfoKHRBuilder(Default::default(), std::marker::PhantomData)
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
        groups : &'a [ crate :: extensions :: khr_ray_tracing :: RayTracingShaderGroupCreateInfoKHRBuilder ],
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
    pub fn libraries(
        mut self,
        libraries: crate::extensions::khr_pipeline_library::PipelineLibraryCreateInfoKHR,
    ) -> Self {
        self.0.libraries = libraries;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn library_interface(
        mut self,
        library_interface : &'a crate :: extensions :: khr_ray_tracing :: RayTracingPipelineInterfaceCreateInfoKHR,
    ) -> Self {
        self.0.p_library_interface = library_interface;
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
    pub unsafe fn discard(self) -> RayTracingPipelineCreateInfoKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for RayTracingPipelineCreateInfoKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for RayTracingPipelineCreateInfoKHRBuilder<'a> {
    type Target = RayTracingPipelineCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for RayTracingPipelineCreateInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRayTracingShaderGroupCreateInfoKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RayTracingShaderGroupCreateInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub _type: crate::extensions::khr_ray_tracing::RayTracingShaderGroupTypeKHR,
    pub general_shader: u32,
    pub closest_hit_shader: u32,
    pub any_hit_shader: u32,
    pub intersection_shader: u32,
    pub p_shader_group_capture_replay_handle: *const std::ffi::c_void,
}
impl RayTracingShaderGroupCreateInfoKHR {
    #[inline]
    pub fn builder<'a>(self) -> RayTracingShaderGroupCreateInfoKHRBuilder<'a> {
        RayTracingShaderGroupCreateInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for RayTracingShaderGroupCreateInfoKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("RayTracingShaderGroupCreateInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("_type", &self._type)
            .field("general_shader", &self.general_shader)
            .field("closest_hit_shader", &self.closest_hit_shader)
            .field("any_hit_shader", &self.any_hit_shader)
            .field("intersection_shader", &self.intersection_shader)
            .field(
                "p_shader_group_capture_replay_handle",
                &self.p_shader_group_capture_replay_handle,
            )
            .finish()
    }
}
impl Default for RayTracingShaderGroupCreateInfoKHR {
    fn default() -> RayTracingShaderGroupCreateInfoKHR {
        RayTracingShaderGroupCreateInfoKHR {
            s_type: crate::vk1_0::StructureType::RAY_TRACING_SHADER_GROUP_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            _type: Default::default(),
            general_shader: Default::default(),
            closest_hit_shader: Default::default(),
            any_hit_shader: Default::default(),
            intersection_shader: Default::default(),
            p_shader_group_capture_replay_handle: std::ptr::null(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`RayTracingShaderGroupCreateInfoKHR`](struct.RayTracingShaderGroupCreateInfoKHR.html)"]
#[repr(transparent)]
pub struct RayTracingShaderGroupCreateInfoKHRBuilder<'a>(
    RayTracingShaderGroupCreateInfoKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> RayTracingShaderGroupCreateInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> RayTracingShaderGroupCreateInfoKHRBuilder<'a> {
        RayTracingShaderGroupCreateInfoKHRBuilder(Default::default(), std::marker::PhantomData)
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
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_group_capture_replay_handle(
        mut self,
        shader_group_capture_replay_handle: &'a std::ffi::c_void,
    ) -> Self {
        self.0.p_shader_group_capture_replay_handle = shader_group_capture_replay_handle;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> RayTracingShaderGroupCreateInfoKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for RayTracingShaderGroupCreateInfoKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for RayTracingShaderGroupCreateInfoKHRBuilder<'a> {
    type Target = RayTracingShaderGroupCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for RayTracingShaderGroupCreateInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRayTracingShaderGroupTypeKHR.html) · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct RayTracingShaderGroupTypeKHR(pub i32);
#[doc = "[Part of `extensions::khr_ray_tracing`](../../extensions/khr_ray_tracing/index.html)"]
impl RayTracingShaderGroupTypeKHR {
    pub const GENERAL_KHR: Self = Self(0);
    pub const TRIANGLES_HIT_GROUP_KHR: Self = Self(1);
    pub const PROCEDURAL_HIT_GROUP_KHR: Self = Self(2);
}
#[doc = "[Part of `extensions::nv_ray_tracing`](../../extensions/nv_ray_tracing/index.html)"]
impl RayTracingShaderGroupTypeKHR {
    pub const GENERAL_NV: Self = Self::GENERAL_KHR;
    pub const TRIANGLES_HIT_GROUP_NV: Self = Self::TRIANGLES_HIT_GROUP_KHR;
    pub const PROCEDURAL_HIT_GROUP_NV: Self = Self::PROCEDURAL_HIT_GROUP_KHR;
}
impl std::fmt::Debug for RayTracingShaderGroupTypeKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::GENERAL_KHR => "GENERAL_KHR",
            &Self::TRIANGLES_HIT_GROUP_KHR => "TRIANGLES_HIT_GROUP_KHR",
            &Self::PROCEDURAL_HIT_GROUP_KHR => "PROCEDURAL_HIT_GROUP_KHR",
            _ => "(unknown)",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRayTracingPipelineInterfaceCreateInfoKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RayTracingPipelineInterfaceCreateInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub max_payload_size: u32,
    pub max_attribute_size: u32,
    pub max_callable_size: u32,
}
impl RayTracingPipelineInterfaceCreateInfoKHR {
    #[inline]
    pub fn builder<'a>(self) -> RayTracingPipelineInterfaceCreateInfoKHRBuilder<'a> {
        RayTracingPipelineInterfaceCreateInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for RayTracingPipelineInterfaceCreateInfoKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("RayTracingPipelineInterfaceCreateInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("max_payload_size", &self.max_payload_size)
            .field("max_attribute_size", &self.max_attribute_size)
            .field("max_callable_size", &self.max_callable_size)
            .finish()
    }
}
impl Default for RayTracingPipelineInterfaceCreateInfoKHR {
    fn default() -> RayTracingPipelineInterfaceCreateInfoKHR {
        RayTracingPipelineInterfaceCreateInfoKHR {
            s_type: crate::vk1_0::StructureType::RAY_TRACING_PIPELINE_INTERFACE_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            max_payload_size: Default::default(),
            max_attribute_size: Default::default(),
            max_callable_size: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`RayTracingPipelineInterfaceCreateInfoKHR`](struct.RayTracingPipelineInterfaceCreateInfoKHR.html)"]
#[repr(transparent)]
pub struct RayTracingPipelineInterfaceCreateInfoKHRBuilder<'a>(
    RayTracingPipelineInterfaceCreateInfoKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> RayTracingPipelineInterfaceCreateInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> RayTracingPipelineInterfaceCreateInfoKHRBuilder<'a> {
        RayTracingPipelineInterfaceCreateInfoKHRBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_payload_size(mut self, max_payload_size: u32) -> Self {
        self.0.max_payload_size = max_payload_size;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_attribute_size(mut self, max_attribute_size: u32) -> Self {
        self.0.max_attribute_size = max_attribute_size;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_callable_size(mut self, max_callable_size: u32) -> Self {
        self.0.max_callable_size = max_callable_size;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> RayTracingPipelineInterfaceCreateInfoKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for RayTracingPipelineInterfaceCreateInfoKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for RayTracingPipelineInterfaceCreateInfoKHRBuilder<'a> {
    type Target = RayTracingPipelineInterfaceCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for RayTracingPipelineInterfaceCreateInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureDeviceAddressInfoKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AccelerationStructureDeviceAddressInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub acceleration_structure: crate::extensions::khr_ray_tracing::AccelerationStructureKHR,
}
impl AccelerationStructureDeviceAddressInfoKHR {
    #[inline]
    pub fn builder<'a>(self) -> AccelerationStructureDeviceAddressInfoKHRBuilder<'a> {
        AccelerationStructureDeviceAddressInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for AccelerationStructureDeviceAddressInfoKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("AccelerationStructureDeviceAddressInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("acceleration_structure", &self.acceleration_structure)
            .finish()
    }
}
impl Default for AccelerationStructureDeviceAddressInfoKHR {
    fn default() -> AccelerationStructureDeviceAddressInfoKHR {
        AccelerationStructureDeviceAddressInfoKHR {
            s_type: crate::vk1_0::StructureType::ACCELERATION_STRUCTURE_DEVICE_ADDRESS_INFO_KHR,
            p_next: std::ptr::null(),
            acceleration_structure: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`AccelerationStructureDeviceAddressInfoKHR`](struct.AccelerationStructureDeviceAddressInfoKHR.html)"]
#[repr(transparent)]
pub struct AccelerationStructureDeviceAddressInfoKHRBuilder<'a>(
    AccelerationStructureDeviceAddressInfoKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> AccelerationStructureDeviceAddressInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> AccelerationStructureDeviceAddressInfoKHRBuilder<'a> {
        AccelerationStructureDeviceAddressInfoKHRBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn acceleration_structure(
        mut self,
        acceleration_structure: crate::extensions::khr_ray_tracing::AccelerationStructureKHR,
    ) -> Self {
        self.0.acceleration_structure = acceleration_structure;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> AccelerationStructureDeviceAddressInfoKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for AccelerationStructureDeviceAddressInfoKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for AccelerationStructureDeviceAddressInfoKHRBuilder<'a> {
    type Target = AccelerationStructureDeviceAddressInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for AccelerationStructureDeviceAddressInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureVersionKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AccelerationStructureVersionKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub version_data: *const u8,
}
impl AccelerationStructureVersionKHR {
    #[inline]
    pub fn builder<'a>(self) -> AccelerationStructureVersionKHRBuilder<'a> {
        AccelerationStructureVersionKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for AccelerationStructureVersionKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("AccelerationStructureVersionKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("version_data", &self.version_data)
            .finish()
    }
}
impl Default for AccelerationStructureVersionKHR {
    fn default() -> AccelerationStructureVersionKHR {
        AccelerationStructureVersionKHR {
            s_type: crate::vk1_0::StructureType::ACCELERATION_STRUCTURE_VERSION_KHR,
            p_next: std::ptr::null(),
            version_data: std::ptr::null(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`AccelerationStructureVersionKHR`](struct.AccelerationStructureVersionKHR.html)"]
#[repr(transparent)]
pub struct AccelerationStructureVersionKHRBuilder<'a>(
    AccelerationStructureVersionKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> AccelerationStructureVersionKHRBuilder<'a> {
    #[inline]
    pub fn new() -> AccelerationStructureVersionKHRBuilder<'a> {
        AccelerationStructureVersionKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn version_data(mut self, version_data: &'a u8) -> Self {
        self.0.version_data = version_data;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> AccelerationStructureVersionKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for AccelerationStructureVersionKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for AccelerationStructureVersionKHRBuilder<'a> {
    type Target = AccelerationStructureVersionKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for AccelerationStructureVersionKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAabbPositionsKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AabbPositionsKHR {
    pub min_x: f32,
    pub min_y: f32,
    pub min_z: f32,
    pub max_x: f32,
    pub max_y: f32,
    pub max_z: f32,
}
impl AabbPositionsKHR {
    #[inline]
    pub fn builder<'a>(self) -> AabbPositionsKHRBuilder<'a> {
        AabbPositionsKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for AabbPositionsKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("AabbPositionsKHR")
            .field("min_x", &self.min_x)
            .field("min_y", &self.min_y)
            .field("min_z", &self.min_z)
            .field("max_x", &self.max_x)
            .field("max_y", &self.max_y)
            .field("max_z", &self.max_z)
            .finish()
    }
}
impl Default for AabbPositionsKHR {
    fn default() -> AabbPositionsKHR {
        AabbPositionsKHR {
            min_x: Default::default(),
            min_y: Default::default(),
            min_z: Default::default(),
            max_x: Default::default(),
            max_y: Default::default(),
            max_z: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`AabbPositionsKHR`](struct.AabbPositionsKHR.html)"]
#[repr(transparent)]
pub struct AabbPositionsKHRBuilder<'a>(AabbPositionsKHR, std::marker::PhantomData<&'a ()>);
impl<'a> AabbPositionsKHRBuilder<'a> {
    #[inline]
    pub fn new() -> AabbPositionsKHRBuilder<'a> {
        AabbPositionsKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn min_x(mut self, min_x: f32) -> Self {
        self.0.min_x = min_x;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn min_y(mut self, min_y: f32) -> Self {
        self.0.min_y = min_y;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn min_z(mut self, min_z: f32) -> Self {
        self.0.min_z = min_z;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_x(mut self, max_x: f32) -> Self {
        self.0.max_x = max_x;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_y(mut self, max_y: f32) -> Self {
        self.0.max_y = max_y;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_z(mut self, max_z: f32) -> Self {
        self.0.max_z = max_z;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> AabbPositionsKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for AabbPositionsKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for AabbPositionsKHRBuilder<'a> {
    type Target = AabbPositionsKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for AabbPositionsKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkTransformMatrixKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TransformMatrixKHR {
    pub matrix: [[f32; 3]; 4],
}
impl TransformMatrixKHR {
    #[inline]
    pub fn builder<'a>(self) -> TransformMatrixKHRBuilder<'a> {
        TransformMatrixKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for TransformMatrixKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("TransformMatrixKHR")
            .field("matrix", &self.matrix)
            .finish()
    }
}
impl Default for TransformMatrixKHR {
    fn default() -> TransformMatrixKHR {
        TransformMatrixKHR {
            matrix: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`TransformMatrixKHR`](struct.TransformMatrixKHR.html)"]
#[repr(transparent)]
pub struct TransformMatrixKHRBuilder<'a>(TransformMatrixKHR, std::marker::PhantomData<&'a ()>);
impl<'a> TransformMatrixKHRBuilder<'a> {
    #[inline]
    pub fn new() -> TransformMatrixKHRBuilder<'a> {
        TransformMatrixKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn matrix(mut self, matrix: [[f32; 3]; 4]) -> Self {
        self.0.matrix = matrix;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> TransformMatrixKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for TransformMatrixKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for TransformMatrixKHRBuilder<'a> {
    type Target = TransformMatrixKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for TransformMatrixKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureInstanceKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AccelerationStructureInstanceKHR {
    pub transform: crate::extensions::khr_ray_tracing::TransformMatrixKHR,
    pub instance_custom_index: u32,
    pub mask: u32,
    pub instance_shader_binding_table_record_offset: u32,
    pub flags: crate::extensions::khr_ray_tracing::GeometryInstanceFlagsKHR,
    pub acceleration_structure_reference: u64,
}
impl AccelerationStructureInstanceKHR {
    #[inline]
    pub fn builder<'a>(self) -> AccelerationStructureInstanceKHRBuilder<'a> {
        AccelerationStructureInstanceKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for AccelerationStructureInstanceKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("AccelerationStructureInstanceKHR")
            .field("transform", &self.transform)
            .field("instance_custom_index", &self.instance_custom_index)
            .field("mask", &self.mask)
            .field(
                "instance_shader_binding_table_record_offset",
                &self.instance_shader_binding_table_record_offset,
            )
            .field("flags", &self.flags)
            .field(
                "acceleration_structure_reference",
                &self.acceleration_structure_reference,
            )
            .finish()
    }
}
impl Default for AccelerationStructureInstanceKHR {
    fn default() -> AccelerationStructureInstanceKHR {
        AccelerationStructureInstanceKHR {
            transform: Default::default(),
            instance_custom_index: Default::default(),
            mask: Default::default(),
            instance_shader_binding_table_record_offset: Default::default(),
            flags: Default::default(),
            acceleration_structure_reference: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`AccelerationStructureInstanceKHR`](struct.AccelerationStructureInstanceKHR.html)"]
#[repr(transparent)]
pub struct AccelerationStructureInstanceKHRBuilder<'a>(
    AccelerationStructureInstanceKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> AccelerationStructureInstanceKHRBuilder<'a> {
    #[inline]
    pub fn new() -> AccelerationStructureInstanceKHRBuilder<'a> {
        AccelerationStructureInstanceKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn transform(
        mut self,
        transform: crate::extensions::khr_ray_tracing::TransformMatrixKHR,
    ) -> Self {
        self.0.transform = transform;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn instance_custom_index(mut self, instance_custom_index: u32) -> Self {
        self.0.instance_custom_index = instance_custom_index;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn mask(mut self, mask: u32) -> Self {
        self.0.mask = mask;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn instance_shader_binding_table_record_offset(
        mut self,
        instance_shader_binding_table_record_offset: u32,
    ) -> Self {
        self.0.instance_shader_binding_table_record_offset =
            instance_shader_binding_table_record_offset;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn flags(
        mut self,
        flags: crate::extensions::khr_ray_tracing::GeometryInstanceFlagsKHR,
    ) -> Self {
        self.0.flags = flags;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn acceleration_structure_reference(
        mut self,
        acceleration_structure_reference: u64,
    ) -> Self {
        self.0.acceleration_structure_reference = acceleration_structure_reference;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> AccelerationStructureInstanceKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for AccelerationStructureInstanceKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for AccelerationStructureInstanceKHRBuilder<'a> {
    type Target = AccelerationStructureInstanceKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for AccelerationStructureInstanceKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeometryInstanceFlagBitsKHR.html) · Flag Bits of [`GeometryInstanceFlagsKHR`](struct.GeometryInstanceFlagsKHR.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct GeometryInstanceFlagBitsKHR(pub u32);
impl GeometryInstanceFlagBitsKHR {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> GeometryInstanceFlagsKHR {
        GeometryInstanceFlagsKHR::from_bits_truncate(self.0)
    }
}
#[doc = "[Part of `extensions::khr_ray_tracing`](../../extensions/khr_ray_tracing/index.html)"]
impl GeometryInstanceFlagBitsKHR {
    pub const TRIANGLE_FACING_CULL_DISABLE_KHR: Self = Self(0x00000001);
    pub const TRIANGLE_FRONT_COUNTERCLOCKWISE_KHR: Self = Self(0x00000002);
    pub const FORCE_OPAQUE_KHR: Self = Self(0x00000004);
    pub const FORCE_NO_OPAQUE_KHR: Self = Self(0x00000008);
}
#[doc = "[Part of `extensions::nv_ray_tracing`](../../extensions/nv_ray_tracing/index.html)"]
impl GeometryInstanceFlagBitsKHR {
    pub const TRIANGLE_CULL_DISABLE_NV: Self = Self::TRIANGLE_FACING_CULL_DISABLE_KHR;
    pub const TRIANGLE_FRONT_COUNTERCLOCKWISE_NV: Self = Self::TRIANGLE_FRONT_COUNTERCLOCKWISE_KHR;
    pub const FORCE_OPAQUE_NV: Self = Self::FORCE_OPAQUE_KHR;
    pub const FORCE_NO_OPAQUE_NV: Self = Self::FORCE_NO_OPAQUE_KHR;
}
impl std::fmt::Debug for GeometryInstanceFlagBitsKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::TRIANGLE_FACING_CULL_DISABLE_KHR => "TRIANGLE_FACING_CULL_DISABLE_KHR",
            &Self::TRIANGLE_FRONT_COUNTERCLOCKWISE_KHR => "TRIANGLE_FRONT_COUNTERCLOCKWISE_KHR",
            &Self::FORCE_OPAQUE_KHR => "FORCE_OPAQUE_KHR",
            &Self::FORCE_NO_OPAQUE_KHR => "FORCE_NO_OPAQUE_KHR",
            _ => "(unknown)",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeometryInstanceFlagsKHR.html) · Flags of [`GeometryInstanceFlagBitsKHR`](struct.GeometryInstanceFlagBitsKHR.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct GeometryInstanceFlagsKHR : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; const TRIANGLE_FACING_CULL_DISABLE_KHR = GeometryInstanceFlagBitsKHR :: TRIANGLE_FACING_CULL_DISABLE_KHR . 0 ; const TRIANGLE_FRONT_COUNTERCLOCKWISE_KHR = GeometryInstanceFlagBitsKHR :: TRIANGLE_FRONT_COUNTERCLOCKWISE_KHR . 0 ; const FORCE_OPAQUE_KHR = GeometryInstanceFlagBitsKHR :: FORCE_OPAQUE_KHR . 0 ; const FORCE_NO_OPAQUE_KHR = GeometryInstanceFlagBitsKHR :: FORCE_NO_OPAQUE_KHR . 0 ; const TRIANGLE_CULL_DISABLE_NV = GeometryInstanceFlagBitsKHR :: TRIANGLE_CULL_DISABLE_NV . 0 ; const TRIANGLE_FRONT_COUNTERCLOCKWISE_NV = GeometryInstanceFlagBitsKHR :: TRIANGLE_FRONT_COUNTERCLOCKWISE_NV . 0 ; const FORCE_OPAQUE_NV = GeometryInstanceFlagBitsKHR :: FORCE_OPAQUE_NV . 0 ; const FORCE_NO_OPAQUE_NV = GeometryInstanceFlagBitsKHR :: FORCE_NO_OPAQUE_NV . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkWriteDescriptorSetAccelerationStructureKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WriteDescriptorSetAccelerationStructureKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub acceleration_structure_count: u32,
    pub p_acceleration_structures:
        *const crate::extensions::khr_ray_tracing::AccelerationStructureKHR,
}
impl WriteDescriptorSetAccelerationStructureKHR {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByWriteDescriptorSetAccelerationStructureKHR,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> WriteDescriptorSetAccelerationStructureKHRBuilder<'a> {
        WriteDescriptorSetAccelerationStructureKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for WriteDescriptorSetAccelerationStructureKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("WriteDescriptorSetAccelerationStructureKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "acceleration_structure_count",
                &self.acceleration_structure_count,
            )
            .field("p_acceleration_structures", &self.p_acceleration_structures)
            .finish()
    }
}
impl Default for WriteDescriptorSetAccelerationStructureKHR {
    fn default() -> WriteDescriptorSetAccelerationStructureKHR {
        WriteDescriptorSetAccelerationStructureKHR {
            s_type: crate::vk1_0::StructureType::WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_KHR,
            p_next: std::ptr::null(),
            acceleration_structure_count: Default::default(),
            p_acceleration_structures: std::ptr::null(),
        }
    }
}
#[doc = "Used by [`WriteDescriptorSetAccelerationStructureKHR::extend`](struct.WriteDescriptorSetAccelerationStructureKHR.html#method.extend)"]
pub trait ExtendableByWriteDescriptorSetAccelerationStructureKHR {}
impl ExtendableByWriteDescriptorSetAccelerationStructureKHR for crate::vk1_0::WriteDescriptorSet {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`WriteDescriptorSetAccelerationStructureKHR`](struct.WriteDescriptorSetAccelerationStructureKHR.html)"]
#[repr(transparent)]
pub struct WriteDescriptorSetAccelerationStructureKHRBuilder<'a>(
    WriteDescriptorSetAccelerationStructureKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> WriteDescriptorSetAccelerationStructureKHRBuilder<'a> {
    #[inline]
    pub fn new() -> WriteDescriptorSetAccelerationStructureKHRBuilder<'a> {
        WriteDescriptorSetAccelerationStructureKHRBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn acceleration_structures(
        mut self,
        acceleration_structures : &'a [ crate :: extensions :: khr_ray_tracing :: AccelerationStructureKHR ],
    ) -> Self {
        self.0.acceleration_structure_count = acceleration_structures.len() as _;
        self.0.p_acceleration_structures = acceleration_structures.as_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> WriteDescriptorSetAccelerationStructureKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for WriteDescriptorSetAccelerationStructureKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for WriteDescriptorSetAccelerationStructureKHRBuilder<'a> {
    type Target = WriteDescriptorSetAccelerationStructureKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for WriteDescriptorSetAccelerationStructureKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceRayTracingFeaturesKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceRayTracingFeaturesKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub ray_tracing: crate::vk1_0::Bool32,
    pub ray_tracing_shader_group_handle_capture_replay: crate::vk1_0::Bool32,
    pub ray_tracing_shader_group_handle_capture_replay_mixed: crate::vk1_0::Bool32,
    pub ray_tracing_acceleration_structure_capture_replay: crate::vk1_0::Bool32,
    pub ray_tracing_indirect_trace_rays: crate::vk1_0::Bool32,
    pub ray_tracing_indirect_acceleration_structure_build: crate::vk1_0::Bool32,
    pub ray_tracing_host_acceleration_structure_commands: crate::vk1_0::Bool32,
    pub ray_query: crate::vk1_0::Bool32,
    pub ray_tracing_primitive_culling: crate::vk1_0::Bool32,
}
impl PhysicalDeviceRayTracingFeaturesKHR {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceRayTracingFeaturesKHR,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceRayTracingFeaturesKHRBuilder<'a> {
        PhysicalDeviceRayTracingFeaturesKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceRayTracingFeaturesKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceRayTracingFeaturesKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("ray_tracing", &(self.ray_tracing != 0))
            .field(
                "ray_tracing_shader_group_handle_capture_replay",
                &(self.ray_tracing_shader_group_handle_capture_replay != 0),
            )
            .field(
                "ray_tracing_shader_group_handle_capture_replay_mixed",
                &(self.ray_tracing_shader_group_handle_capture_replay_mixed != 0),
            )
            .field(
                "ray_tracing_acceleration_structure_capture_replay",
                &(self.ray_tracing_acceleration_structure_capture_replay != 0),
            )
            .field(
                "ray_tracing_indirect_trace_rays",
                &(self.ray_tracing_indirect_trace_rays != 0),
            )
            .field(
                "ray_tracing_indirect_acceleration_structure_build",
                &(self.ray_tracing_indirect_acceleration_structure_build != 0),
            )
            .field(
                "ray_tracing_host_acceleration_structure_commands",
                &(self.ray_tracing_host_acceleration_structure_commands != 0),
            )
            .field("ray_query", &(self.ray_query != 0))
            .field(
                "ray_tracing_primitive_culling",
                &(self.ray_tracing_primitive_culling != 0),
            )
            .finish()
    }
}
impl Default for PhysicalDeviceRayTracingFeaturesKHR {
    fn default() -> PhysicalDeviceRayTracingFeaturesKHR {
        PhysicalDeviceRayTracingFeaturesKHR {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_RAY_TRACING_FEATURES_KHR,
            p_next: std::ptr::null_mut(),
            ray_tracing: Default::default(),
            ray_tracing_shader_group_handle_capture_replay: Default::default(),
            ray_tracing_shader_group_handle_capture_replay_mixed: Default::default(),
            ray_tracing_acceleration_structure_capture_replay: Default::default(),
            ray_tracing_indirect_trace_rays: Default::default(),
            ray_tracing_indirect_acceleration_structure_build: Default::default(),
            ray_tracing_host_acceleration_structure_commands: Default::default(),
            ray_query: Default::default(),
            ray_tracing_primitive_culling: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceRayTracingFeaturesKHR::extend`](struct.PhysicalDeviceRayTracingFeaturesKHR.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceRayTracingFeaturesKHR {}
impl ExtendableByPhysicalDeviceRayTracingFeaturesKHR for crate::vk1_1::PhysicalDeviceFeatures2 {}
impl ExtendableByPhysicalDeviceRayTracingFeaturesKHR for crate::vk1_0::DeviceCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceRayTracingFeaturesKHR`](struct.PhysicalDeviceRayTracingFeaturesKHR.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceRayTracingFeaturesKHRBuilder<'a>(
    PhysicalDeviceRayTracingFeaturesKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceRayTracingFeaturesKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceRayTracingFeaturesKHRBuilder<'a> {
        PhysicalDeviceRayTracingFeaturesKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn ray_tracing(mut self, ray_tracing: bool) -> Self {
        self.0.ray_tracing = ray_tracing as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn ray_tracing_shader_group_handle_capture_replay(
        mut self,
        ray_tracing_shader_group_handle_capture_replay: bool,
    ) -> Self {
        self.0.ray_tracing_shader_group_handle_capture_replay =
            ray_tracing_shader_group_handle_capture_replay as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn ray_tracing_shader_group_handle_capture_replay_mixed(
        mut self,
        ray_tracing_shader_group_handle_capture_replay_mixed: bool,
    ) -> Self {
        self.0.ray_tracing_shader_group_handle_capture_replay_mixed =
            ray_tracing_shader_group_handle_capture_replay_mixed as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn ray_tracing_acceleration_structure_capture_replay(
        mut self,
        ray_tracing_acceleration_structure_capture_replay: bool,
    ) -> Self {
        self.0.ray_tracing_acceleration_structure_capture_replay =
            ray_tracing_acceleration_structure_capture_replay as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn ray_tracing_indirect_trace_rays(
        mut self,
        ray_tracing_indirect_trace_rays: bool,
    ) -> Self {
        self.0.ray_tracing_indirect_trace_rays = ray_tracing_indirect_trace_rays as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn ray_tracing_indirect_acceleration_structure_build(
        mut self,
        ray_tracing_indirect_acceleration_structure_build: bool,
    ) -> Self {
        self.0.ray_tracing_indirect_acceleration_structure_build =
            ray_tracing_indirect_acceleration_structure_build as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn ray_tracing_host_acceleration_structure_commands(
        mut self,
        ray_tracing_host_acceleration_structure_commands: bool,
    ) -> Self {
        self.0.ray_tracing_host_acceleration_structure_commands =
            ray_tracing_host_acceleration_structure_commands as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn ray_query(mut self, ray_query: bool) -> Self {
        self.0.ray_query = ray_query as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn ray_tracing_primitive_culling(mut self, ray_tracing_primitive_culling: bool) -> Self {
        self.0.ray_tracing_primitive_culling = ray_tracing_primitive_culling as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceRayTracingFeaturesKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceRayTracingFeaturesKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceRayTracingFeaturesKHRBuilder<'a> {
    type Target = PhysicalDeviceRayTracingFeaturesKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceRayTracingFeaturesKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceRayTracingPropertiesKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceRayTracingPropertiesKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub shader_group_handle_size: u32,
    pub max_recursion_depth: u32,
    pub max_shader_group_stride: u32,
    pub shader_group_base_alignment: u32,
    pub max_geometry_count: u64,
    pub max_instance_count: u64,
    pub max_primitive_count: u64,
    pub max_descriptor_set_acceleration_structures: u32,
    pub shader_group_handle_capture_replay_size: u32,
}
impl PhysicalDeviceRayTracingPropertiesKHR {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceRayTracingPropertiesKHR,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceRayTracingPropertiesKHRBuilder<'a> {
        PhysicalDeviceRayTracingPropertiesKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceRayTracingPropertiesKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceRayTracingPropertiesKHR")
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
            .field("max_primitive_count", &self.max_primitive_count)
            .field(
                "max_descriptor_set_acceleration_structures",
                &self.max_descriptor_set_acceleration_structures,
            )
            .field(
                "shader_group_handle_capture_replay_size",
                &self.shader_group_handle_capture_replay_size,
            )
            .finish()
    }
}
impl Default for PhysicalDeviceRayTracingPropertiesKHR {
    fn default() -> PhysicalDeviceRayTracingPropertiesKHR {
        PhysicalDeviceRayTracingPropertiesKHR {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_RAY_TRACING_PROPERTIES_KHR,
            p_next: std::ptr::null_mut(),
            shader_group_handle_size: Default::default(),
            max_recursion_depth: Default::default(),
            max_shader_group_stride: Default::default(),
            shader_group_base_alignment: Default::default(),
            max_geometry_count: Default::default(),
            max_instance_count: Default::default(),
            max_primitive_count: Default::default(),
            max_descriptor_set_acceleration_structures: Default::default(),
            shader_group_handle_capture_replay_size: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceRayTracingPropertiesKHR::extend`](struct.PhysicalDeviceRayTracingPropertiesKHR.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceRayTracingPropertiesKHR {}
impl ExtendableByPhysicalDeviceRayTracingPropertiesKHR for crate::vk1_1::PhysicalDeviceProperties2 {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceRayTracingPropertiesKHR`](struct.PhysicalDeviceRayTracingPropertiesKHR.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceRayTracingPropertiesKHRBuilder<'a>(
    PhysicalDeviceRayTracingPropertiesKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceRayTracingPropertiesKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceRayTracingPropertiesKHRBuilder<'a> {
        PhysicalDeviceRayTracingPropertiesKHRBuilder(Default::default(), std::marker::PhantomData)
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
    pub fn max_primitive_count(mut self, max_primitive_count: u64) -> Self {
        self.0.max_primitive_count = max_primitive_count;
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
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_group_handle_capture_replay_size(
        mut self,
        shader_group_handle_capture_replay_size: u32,
    ) -> Self {
        self.0.shader_group_handle_capture_replay_size = shader_group_handle_capture_replay_size;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceRayTracingPropertiesKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceRayTracingPropertiesKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceRayTracingPropertiesKHRBuilder<'a> {
    type Target = PhysicalDeviceRayTracingPropertiesKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceRayTracingPropertiesKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkTraceRaysIndirectCommandKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TraceRaysIndirectCommandKHR {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}
impl TraceRaysIndirectCommandKHR {
    #[inline]
    pub fn builder<'a>(self) -> TraceRaysIndirectCommandKHRBuilder<'a> {
        TraceRaysIndirectCommandKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for TraceRaysIndirectCommandKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("TraceRaysIndirectCommandKHR")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("depth", &self.depth)
            .finish()
    }
}
impl Default for TraceRaysIndirectCommandKHR {
    fn default() -> TraceRaysIndirectCommandKHR {
        TraceRaysIndirectCommandKHR {
            width: Default::default(),
            height: Default::default(),
            depth: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`TraceRaysIndirectCommandKHR`](struct.TraceRaysIndirectCommandKHR.html)"]
#[repr(transparent)]
pub struct TraceRaysIndirectCommandKHRBuilder<'a>(
    TraceRaysIndirectCommandKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> TraceRaysIndirectCommandKHRBuilder<'a> {
    #[inline]
    pub fn new() -> TraceRaysIndirectCommandKHRBuilder<'a> {
        TraceRaysIndirectCommandKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn width(mut self, width: u32) -> Self {
        self.0.width = width;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn height(mut self, height: u32) -> Self {
        self.0.height = height;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn depth(mut self, depth: u32) -> Self {
        self.0.depth = depth;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> TraceRaysIndirectCommandKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for TraceRaysIndirectCommandKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for TraceRaysIndirectCommandKHRBuilder<'a> {
    type Target = TraceRaysIndirectCommandKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for TraceRaysIndirectCommandKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
