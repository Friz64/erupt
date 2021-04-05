#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_SHADER_UNUSED_KHR")]
pub const SHADER_UNUSED_KHR: u32 = 4294967295;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_RAY_TRACING_PIPELINE_SPEC_VERSION")]
pub const KHR_RAY_TRACING_PIPELINE_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_RAY_TRACING_PIPELINE_EXTENSION_NAME")]
pub const KHR_RAY_TRACING_PIPELINE_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_ray_tracing_pipeline");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_TRACE_RAYS_KHR: *const std::os::raw::c_char = crate::cstr!("vkCmdTraceRaysKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_RAY_TRACING_SHADER_GROUP_HANDLES_KHR: *const std::os::raw::c_char = crate::cstr!("vkGetRayTracingShaderGroupHandlesKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_RAY_TRACING_CAPTURE_REPLAY_SHADER_GROUP_HANDLES_KHR: *const std::os::raw::c_char = crate::cstr!("vkGetRayTracingCaptureReplayShaderGroupHandlesKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_RAY_TRACING_PIPELINES_KHR: *const std::os::raw::c_char = crate::cstr!("vkCreateRayTracingPipelinesKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_TRACE_RAYS_INDIRECT_KHR: *const std::os::raw::c_char = crate::cstr!("vkCmdTraceRaysIndirectKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_RAY_TRACING_SHADER_GROUP_STACK_SIZE_KHR: *const std::os::raw::c_char = crate::cstr!("vkGetRayTracingShaderGroupStackSizeKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_SET_RAY_TRACING_PIPELINE_STACK_SIZE_KHR: *const std::os::raw::c_char = crate::cstr!("vkCmdSetRayTracingPipelineStackSizeKHR");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRayTracingShaderGroupTypeKHR.html) · Enum"]
#[doc(alias = "VkRayTracingShaderGroupTypeKHR")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct RayTracingShaderGroupTypeKHR(pub i32);
impl std::fmt::Debug for RayTracingShaderGroupTypeKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::GENERAL_KHR => "GENERAL_KHR",
            &Self::TRIANGLES_HIT_GROUP_KHR => "TRIANGLES_HIT_GROUP_KHR",
            &Self::PROCEDURAL_HIT_GROUP_KHR => "PROCEDURAL_HIT_GROUP_KHR",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::khr_ray_tracing_pipeline`]"]
impl RayTracingShaderGroupTypeKHR {
    pub const GENERAL_KHR: Self = Self(0);
    pub const TRIANGLES_HIT_GROUP_KHR: Self = Self(1);
    pub const PROCEDURAL_HIT_GROUP_KHR: Self = Self(2);
}
#[doc = "Provided by [`crate::extensions::nv_ray_tracing`]"]
impl RayTracingShaderGroupTypeKHR {
    pub const GENERAL_NV: Self = Self::GENERAL_KHR;
    pub const TRIANGLES_HIT_GROUP_NV: Self = Self::TRIANGLES_HIT_GROUP_KHR;
    pub const PROCEDURAL_HIT_GROUP_NV: Self = Self::PROCEDURAL_HIT_GROUP_KHR;
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkShaderGroupShaderKHR.html) · Enum"]
#[doc(alias = "VkShaderGroupShaderKHR")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct ShaderGroupShaderKHR(pub i32);
impl std::fmt::Debug for ShaderGroupShaderKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::GENERAL_KHR => "GENERAL_KHR",
            &Self::CLOSEST_HIT_KHR => "CLOSEST_HIT_KHR",
            &Self::ANY_HIT_KHR => "ANY_HIT_KHR",
            &Self::INTERSECTION_KHR => "INTERSECTION_KHR",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::khr_ray_tracing_pipeline`]"]
impl ShaderGroupShaderKHR {
    pub const GENERAL_KHR: Self = Self(0);
    pub const CLOSEST_HIT_KHR: Self = Self(1);
    pub const ANY_HIT_KHR: Self = Self(2);
    pub const INTERSECTION_KHR: Self = Self(3);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdTraceRaysKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdTraceRaysKHR = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    p_raygen_shader_binding_table: *const crate::extensions::khr_ray_tracing_pipeline::StridedDeviceAddressRegionKHR,
    p_miss_shader_binding_table: *const crate::extensions::khr_ray_tracing_pipeline::StridedDeviceAddressRegionKHR,
    p_hit_shader_binding_table: *const crate::extensions::khr_ray_tracing_pipeline::StridedDeviceAddressRegionKHR,
    p_callable_shader_binding_table: *const crate::extensions::khr_ray_tracing_pipeline::StridedDeviceAddressRegionKHR,
    width: u32,
    height: u32,
    depth: u32,
) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetRayTracingShaderGroupHandlesKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetRayTracingShaderGroupHandlesKHR =
    unsafe extern "system" fn(device: crate::vk1_0::Device, pipeline: crate::vk1_0::Pipeline, first_group: u32, group_count: u32, data_size: usize, p_data: *mut std::ffi::c_void) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetRayTracingCaptureReplayShaderGroupHandlesKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetRayTracingCaptureReplayShaderGroupHandlesKHR =
    unsafe extern "system" fn(device: crate::vk1_0::Device, pipeline: crate::vk1_0::Pipeline, first_group: u32, group_count: u32, data_size: usize, p_data: *mut std::ffi::c_void) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateRayTracingPipelinesKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateRayTracingPipelinesKHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    deferred_operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
    pipeline_cache: crate::vk1_0::PipelineCache,
    create_info_count: u32,
    p_create_infos: *const crate::extensions::khr_ray_tracing_pipeline::RayTracingPipelineCreateInfoKHR,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
    p_pipelines: *mut crate::vk1_0::Pipeline,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdTraceRaysIndirectKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdTraceRaysIndirectKHR = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    p_raygen_shader_binding_table: *const crate::extensions::khr_ray_tracing_pipeline::StridedDeviceAddressRegionKHR,
    p_miss_shader_binding_table: *const crate::extensions::khr_ray_tracing_pipeline::StridedDeviceAddressRegionKHR,
    p_hit_shader_binding_table: *const crate::extensions::khr_ray_tracing_pipeline::StridedDeviceAddressRegionKHR,
    p_callable_shader_binding_table: *const crate::extensions::khr_ray_tracing_pipeline::StridedDeviceAddressRegionKHR,
    indirect_device_address: crate::vk1_0::DeviceAddress,
) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetRayTracingShaderGroupStackSizeKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetRayTracingShaderGroupStackSizeKHR =
    unsafe extern "system" fn(device: crate::vk1_0::Device, pipeline: crate::vk1_0::Pipeline, group: u32, group_shader: crate::extensions::khr_ray_tracing_pipeline::ShaderGroupShaderKHR) -> crate::vk1_0::DeviceSize;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetRayTracingPipelineStackSizeKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetRayTracingPipelineStackSizeKHR = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, pipeline_stack_size: u32) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRayTracingShaderGroupCreateInfoKHR.html) · Structure"]
#[doc(alias = "VkRayTracingShaderGroupCreateInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RayTracingShaderGroupCreateInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub _type: crate::extensions::khr_ray_tracing_pipeline::RayTracingShaderGroupTypeKHR,
    pub general_shader: u32,
    pub closest_hit_shader: u32,
    pub any_hit_shader: u32,
    pub intersection_shader: u32,
    pub p_shader_group_capture_replay_handle: *const std::ffi::c_void,
}
impl Default for RayTracingShaderGroupCreateInfoKHR {
    fn default() -> Self {
        Self {
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
impl std::fmt::Debug for RayTracingShaderGroupCreateInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RayTracingShaderGroupCreateInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("_type", &self._type)
            .field("general_shader", &self.general_shader)
            .field("closest_hit_shader", &self.closest_hit_shader)
            .field("any_hit_shader", &self.any_hit_shader)
            .field("intersection_shader", &self.intersection_shader)
            .field("p_shader_group_capture_replay_handle", &self.p_shader_group_capture_replay_handle)
            .finish()
    }
}
impl RayTracingShaderGroupCreateInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> RayTracingShaderGroupCreateInfoKHRBuilder<'a> {
        RayTracingShaderGroupCreateInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRayTracingShaderGroupCreateInfoKHR.html) · Builder of [`RayTracingShaderGroupCreateInfoKHR`]"]
#[repr(transparent)]
pub struct RayTracingShaderGroupCreateInfoKHRBuilder<'a>(RayTracingShaderGroupCreateInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> RayTracingShaderGroupCreateInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> RayTracingShaderGroupCreateInfoKHRBuilder<'a> {
        RayTracingShaderGroupCreateInfoKHRBuilder(Default::default(), std::marker::PhantomData)
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
    pub fn shader_group_capture_replay_handle(mut self, shader_group_capture_replay_handle: *const std::ffi::c_void) -> Self {
        self.0.p_shader_group_capture_replay_handle = shader_group_capture_replay_handle;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> RayTracingShaderGroupCreateInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for RayTracingShaderGroupCreateInfoKHRBuilder<'a> {
    fn default() -> RayTracingShaderGroupCreateInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for RayTracingShaderGroupCreateInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
unsafe impl crate::Repr<RayTracingShaderGroupCreateInfoKHR> for RayTracingShaderGroupCreateInfoKHRBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRayTracingPipelineCreateInfoKHR.html) · Structure"]
#[doc(alias = "VkRayTracingPipelineCreateInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RayTracingPipelineCreateInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::vk1_0::PipelineCreateFlags,
    pub stage_count: u32,
    pub p_stages: *const crate::vk1_0::PipelineShaderStageCreateInfo,
    pub group_count: u32,
    pub p_groups: *const crate::extensions::khr_ray_tracing_pipeline::RayTracingShaderGroupCreateInfoKHR,
    pub max_pipeline_ray_recursion_depth: u32,
    pub p_library_info: *const crate::extensions::khr_pipeline_library::PipelineLibraryCreateInfoKHR,
    pub p_library_interface: *const crate::extensions::khr_ray_tracing_pipeline::RayTracingPipelineInterfaceCreateInfoKHR,
    pub p_dynamic_state: *const crate::vk1_0::PipelineDynamicStateCreateInfo,
    pub layout: crate::vk1_0::PipelineLayout,
    pub base_pipeline_handle: crate::vk1_0::Pipeline,
    pub base_pipeline_index: i32,
}
impl Default for RayTracingPipelineCreateInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::RAY_TRACING_PIPELINE_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            flags: Default::default(),
            stage_count: Default::default(),
            p_stages: std::ptr::null(),
            group_count: Default::default(),
            p_groups: std::ptr::null(),
            max_pipeline_ray_recursion_depth: Default::default(),
            p_library_info: std::ptr::null(),
            p_library_interface: std::ptr::null(),
            p_dynamic_state: std::ptr::null(),
            layout: Default::default(),
            base_pipeline_handle: Default::default(),
            base_pipeline_index: Default::default(),
        }
    }
}
impl std::fmt::Debug for RayTracingPipelineCreateInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RayTracingPipelineCreateInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("stage_count", &self.stage_count)
            .field("p_stages", &self.p_stages)
            .field("group_count", &self.group_count)
            .field("p_groups", &self.p_groups)
            .field("max_pipeline_ray_recursion_depth", &self.max_pipeline_ray_recursion_depth)
            .field("p_library_info", &self.p_library_info)
            .field("p_library_interface", &self.p_library_interface)
            .field("p_dynamic_state", &self.p_dynamic_state)
            .field("layout", &self.layout)
            .field("base_pipeline_handle", &self.base_pipeline_handle)
            .field("base_pipeline_index", &self.base_pipeline_index)
            .finish()
    }
}
impl RayTracingPipelineCreateInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> RayTracingPipelineCreateInfoKHRBuilder<'a> {
        RayTracingPipelineCreateInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl<'a> crate::ExtendableFromConst<'a, crate::extensions::ext_pipeline_creation_feedback::PipelineCreationFeedbackCreateInfoEXT> for RayTracingPipelineCreateInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, crate::extensions::ext_pipeline_creation_feedback::PipelineCreationFeedbackCreateInfoEXTBuilder<'_>> for RayTracingPipelineCreateInfoKHRBuilder<'a> {}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRayTracingPipelineCreateInfoKHR.html) · Builder of [`RayTracingPipelineCreateInfoKHR`]"]
#[repr(transparent)]
pub struct RayTracingPipelineCreateInfoKHRBuilder<'a>(RayTracingPipelineCreateInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> RayTracingPipelineCreateInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> RayTracingPipelineCreateInfoKHRBuilder<'a> {
        RayTracingPipelineCreateInfoKHRBuilder(Default::default(), std::marker::PhantomData)
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
    pub fn groups(mut self, groups: &'a [impl crate::Repr<crate::extensions::khr_ray_tracing_pipeline::RayTracingShaderGroupCreateInfoKHR>]) -> Self {
        self.0.p_groups = groups.as_ptr() as _;
        self.0.group_count = groups.len() as _;
        self
    }
    #[inline]
    pub fn max_pipeline_ray_recursion_depth(mut self, max_pipeline_ray_recursion_depth: u32) -> Self {
        self.0.max_pipeline_ray_recursion_depth = max_pipeline_ray_recursion_depth as _;
        self
    }
    #[inline]
    pub fn library_info(mut self, library_info: &'a crate::extensions::khr_pipeline_library::PipelineLibraryCreateInfoKHR) -> Self {
        self.0.p_library_info = library_info as _;
        self
    }
    #[inline]
    pub fn library_interface(mut self, library_interface: &'a crate::extensions::khr_ray_tracing_pipeline::RayTracingPipelineInterfaceCreateInfoKHR) -> Self {
        self.0.p_library_interface = library_interface as _;
        self
    }
    #[inline]
    pub fn dynamic_state(mut self, dynamic_state: &'a crate::vk1_0::PipelineDynamicStateCreateInfo) -> Self {
        self.0.p_dynamic_state = dynamic_state as _;
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
    pub fn build(self) -> RayTracingPipelineCreateInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for RayTracingPipelineCreateInfoKHRBuilder<'a> {
    fn default() -> RayTracingPipelineCreateInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for RayTracingPipelineCreateInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
unsafe impl crate::Repr<RayTracingPipelineCreateInfoKHR> for RayTracingPipelineCreateInfoKHRBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceRayTracingPipelineFeaturesKHR.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceRayTracingPipelineFeaturesKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceRayTracingPipelineFeaturesKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub ray_tracing_pipeline: crate::vk1_0::Bool32,
    pub ray_tracing_pipeline_shader_group_handle_capture_replay: crate::vk1_0::Bool32,
    pub ray_tracing_pipeline_shader_group_handle_capture_replay_mixed: crate::vk1_0::Bool32,
    pub ray_tracing_pipeline_trace_rays_indirect: crate::vk1_0::Bool32,
    pub ray_traversal_primitive_culling: crate::vk1_0::Bool32,
}
impl Default for PhysicalDeviceRayTracingPipelineFeaturesKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_FEATURES_KHR,
            p_next: std::ptr::null_mut(),
            ray_tracing_pipeline: Default::default(),
            ray_tracing_pipeline_shader_group_handle_capture_replay: Default::default(),
            ray_tracing_pipeline_shader_group_handle_capture_replay_mixed: Default::default(),
            ray_tracing_pipeline_trace_rays_indirect: Default::default(),
            ray_traversal_primitive_culling: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceRayTracingPipelineFeaturesKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceRayTracingPipelineFeaturesKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("ray_tracing_pipeline", &(self.ray_tracing_pipeline != 0))
            .field("ray_tracing_pipeline_shader_group_handle_capture_replay", &(self.ray_tracing_pipeline_shader_group_handle_capture_replay != 0))
            .field("ray_tracing_pipeline_shader_group_handle_capture_replay_mixed", &(self.ray_tracing_pipeline_shader_group_handle_capture_replay_mixed != 0))
            .field("ray_tracing_pipeline_trace_rays_indirect", &(self.ray_tracing_pipeline_trace_rays_indirect != 0))
            .field("ray_traversal_primitive_culling", &(self.ray_traversal_primitive_culling != 0))
            .finish()
    }
}
impl PhysicalDeviceRayTracingPipelineFeaturesKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceRayTracingPipelineFeaturesKHRBuilder<'a> {
        PhysicalDeviceRayTracingPipelineFeaturesKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceRayTracingPipelineFeaturesKHR.html) · Builder of [`PhysicalDeviceRayTracingPipelineFeaturesKHR`]"]
#[repr(transparent)]
pub struct PhysicalDeviceRayTracingPipelineFeaturesKHRBuilder<'a>(PhysicalDeviceRayTracingPipelineFeaturesKHR, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceRayTracingPipelineFeaturesKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceRayTracingPipelineFeaturesKHRBuilder<'a> {
        PhysicalDeviceRayTracingPipelineFeaturesKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn ray_tracing_pipeline(mut self, ray_tracing_pipeline: bool) -> Self {
        self.0.ray_tracing_pipeline = ray_tracing_pipeline as _;
        self
    }
    #[inline]
    pub fn ray_tracing_pipeline_shader_group_handle_capture_replay(mut self, ray_tracing_pipeline_shader_group_handle_capture_replay: bool) -> Self {
        self.0.ray_tracing_pipeline_shader_group_handle_capture_replay = ray_tracing_pipeline_shader_group_handle_capture_replay as _;
        self
    }
    #[inline]
    pub fn ray_tracing_pipeline_shader_group_handle_capture_replay_mixed(mut self, ray_tracing_pipeline_shader_group_handle_capture_replay_mixed: bool) -> Self {
        self.0.ray_tracing_pipeline_shader_group_handle_capture_replay_mixed = ray_tracing_pipeline_shader_group_handle_capture_replay_mixed as _;
        self
    }
    #[inline]
    pub fn ray_tracing_pipeline_trace_rays_indirect(mut self, ray_tracing_pipeline_trace_rays_indirect: bool) -> Self {
        self.0.ray_tracing_pipeline_trace_rays_indirect = ray_tracing_pipeline_trace_rays_indirect as _;
        self
    }
    #[inline]
    pub fn ray_traversal_primitive_culling(mut self, ray_traversal_primitive_culling: bool) -> Self {
        self.0.ray_traversal_primitive_culling = ray_traversal_primitive_culling as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceRayTracingPipelineFeaturesKHR {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceRayTracingPipelineFeaturesKHRBuilder<'a> {
    fn default() -> PhysicalDeviceRayTracingPipelineFeaturesKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceRayTracingPipelineFeaturesKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceRayTracingPipelineFeaturesKHRBuilder<'a> {
    type Target = PhysicalDeviceRayTracingPipelineFeaturesKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceRayTracingPipelineFeaturesKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<PhysicalDeviceRayTracingPipelineFeaturesKHR> for PhysicalDeviceRayTracingPipelineFeaturesKHRBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceRayTracingPipelinePropertiesKHR.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceRayTracingPipelinePropertiesKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceRayTracingPipelinePropertiesKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub shader_group_handle_size: u32,
    pub max_ray_recursion_depth: u32,
    pub max_shader_group_stride: u32,
    pub shader_group_base_alignment: u32,
    pub shader_group_handle_capture_replay_size: u32,
    pub max_ray_dispatch_invocation_count: u32,
    pub shader_group_handle_alignment: u32,
    pub max_ray_hit_attribute_size: u32,
}
impl Default for PhysicalDeviceRayTracingPipelinePropertiesKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_PROPERTIES_KHR,
            p_next: std::ptr::null_mut(),
            shader_group_handle_size: Default::default(),
            max_ray_recursion_depth: Default::default(),
            max_shader_group_stride: Default::default(),
            shader_group_base_alignment: Default::default(),
            shader_group_handle_capture_replay_size: Default::default(),
            max_ray_dispatch_invocation_count: Default::default(),
            shader_group_handle_alignment: Default::default(),
            max_ray_hit_attribute_size: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceRayTracingPipelinePropertiesKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceRayTracingPipelinePropertiesKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("shader_group_handle_size", &self.shader_group_handle_size)
            .field("max_ray_recursion_depth", &self.max_ray_recursion_depth)
            .field("max_shader_group_stride", &self.max_shader_group_stride)
            .field("shader_group_base_alignment", &self.shader_group_base_alignment)
            .field("shader_group_handle_capture_replay_size", &self.shader_group_handle_capture_replay_size)
            .field("max_ray_dispatch_invocation_count", &self.max_ray_dispatch_invocation_count)
            .field("shader_group_handle_alignment", &self.shader_group_handle_alignment)
            .field("max_ray_hit_attribute_size", &self.max_ray_hit_attribute_size)
            .finish()
    }
}
impl PhysicalDeviceRayTracingPipelinePropertiesKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceRayTracingPipelinePropertiesKHRBuilder<'a> {
        PhysicalDeviceRayTracingPipelinePropertiesKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceRayTracingPipelinePropertiesKHR.html) · Builder of [`PhysicalDeviceRayTracingPipelinePropertiesKHR`]"]
#[repr(transparent)]
pub struct PhysicalDeviceRayTracingPipelinePropertiesKHRBuilder<'a>(PhysicalDeviceRayTracingPipelinePropertiesKHR, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceRayTracingPipelinePropertiesKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceRayTracingPipelinePropertiesKHRBuilder<'a> {
        PhysicalDeviceRayTracingPipelinePropertiesKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn shader_group_handle_size(mut self, shader_group_handle_size: u32) -> Self {
        self.0.shader_group_handle_size = shader_group_handle_size as _;
        self
    }
    #[inline]
    pub fn max_ray_recursion_depth(mut self, max_ray_recursion_depth: u32) -> Self {
        self.0.max_ray_recursion_depth = max_ray_recursion_depth as _;
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
    pub fn shader_group_handle_capture_replay_size(mut self, shader_group_handle_capture_replay_size: u32) -> Self {
        self.0.shader_group_handle_capture_replay_size = shader_group_handle_capture_replay_size as _;
        self
    }
    #[inline]
    pub fn max_ray_dispatch_invocation_count(mut self, max_ray_dispatch_invocation_count: u32) -> Self {
        self.0.max_ray_dispatch_invocation_count = max_ray_dispatch_invocation_count as _;
        self
    }
    #[inline]
    pub fn shader_group_handle_alignment(mut self, shader_group_handle_alignment: u32) -> Self {
        self.0.shader_group_handle_alignment = shader_group_handle_alignment as _;
        self
    }
    #[inline]
    pub fn max_ray_hit_attribute_size(mut self, max_ray_hit_attribute_size: u32) -> Self {
        self.0.max_ray_hit_attribute_size = max_ray_hit_attribute_size as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceRayTracingPipelinePropertiesKHR {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceRayTracingPipelinePropertiesKHRBuilder<'a> {
    fn default() -> PhysicalDeviceRayTracingPipelinePropertiesKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceRayTracingPipelinePropertiesKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceRayTracingPipelinePropertiesKHRBuilder<'a> {
    type Target = PhysicalDeviceRayTracingPipelinePropertiesKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceRayTracingPipelinePropertiesKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<PhysicalDeviceRayTracingPipelinePropertiesKHR> for PhysicalDeviceRayTracingPipelinePropertiesKHRBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkStridedDeviceAddressRegionKHR.html) · Structure"]
#[doc(alias = "VkStridedDeviceAddressRegionKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StridedDeviceAddressRegionKHR {
    pub device_address: crate::vk1_0::DeviceAddress,
    pub stride: crate::vk1_0::DeviceSize,
    pub size: crate::vk1_0::DeviceSize,
}
impl Default for StridedDeviceAddressRegionKHR {
    fn default() -> Self {
        Self {
            device_address: Default::default(),
            stride: Default::default(),
            size: Default::default(),
        }
    }
}
impl std::fmt::Debug for StridedDeviceAddressRegionKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StridedDeviceAddressRegionKHR")
            .field("device_address", &self.device_address)
            .field("stride", &self.stride)
            .field("size", &self.size)
            .finish()
    }
}
impl StridedDeviceAddressRegionKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> StridedDeviceAddressRegionKHRBuilder<'a> {
        StridedDeviceAddressRegionKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkStridedDeviceAddressRegionKHR.html) · Builder of [`StridedDeviceAddressRegionKHR`]"]
#[repr(transparent)]
pub struct StridedDeviceAddressRegionKHRBuilder<'a>(StridedDeviceAddressRegionKHR, std::marker::PhantomData<&'a ()>);
impl<'a> StridedDeviceAddressRegionKHRBuilder<'a> {
    #[inline]
    pub fn new() -> StridedDeviceAddressRegionKHRBuilder<'a> {
        StridedDeviceAddressRegionKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn device_address(mut self, device_address: crate::vk1_0::DeviceAddress) -> Self {
        self.0.device_address = device_address as _;
        self
    }
    #[inline]
    pub fn stride(mut self, stride: crate::vk1_0::DeviceSize) -> Self {
        self.0.stride = stride as _;
        self
    }
    #[inline]
    pub fn size(mut self, size: crate::vk1_0::DeviceSize) -> Self {
        self.0.size = size as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> StridedDeviceAddressRegionKHR {
        self.0
    }
}
impl<'a> std::default::Default for StridedDeviceAddressRegionKHRBuilder<'a> {
    fn default() -> StridedDeviceAddressRegionKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for StridedDeviceAddressRegionKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for StridedDeviceAddressRegionKHRBuilder<'a> {
    type Target = StridedDeviceAddressRegionKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StridedDeviceAddressRegionKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<StridedDeviceAddressRegionKHR> for StridedDeviceAddressRegionKHRBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkTraceRaysIndirectCommandKHR.html) · Structure"]
#[doc(alias = "VkTraceRaysIndirectCommandKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TraceRaysIndirectCommandKHR {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}
impl Default for TraceRaysIndirectCommandKHR {
    fn default() -> Self {
        Self {
            width: Default::default(),
            height: Default::default(),
            depth: Default::default(),
        }
    }
}
impl std::fmt::Debug for TraceRaysIndirectCommandKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TraceRaysIndirectCommandKHR").field("width", &self.width).field("height", &self.height).field("depth", &self.depth).finish()
    }
}
impl TraceRaysIndirectCommandKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> TraceRaysIndirectCommandKHRBuilder<'a> {
        TraceRaysIndirectCommandKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkTraceRaysIndirectCommandKHR.html) · Builder of [`TraceRaysIndirectCommandKHR`]"]
#[repr(transparent)]
pub struct TraceRaysIndirectCommandKHRBuilder<'a>(TraceRaysIndirectCommandKHR, std::marker::PhantomData<&'a ()>);
impl<'a> TraceRaysIndirectCommandKHRBuilder<'a> {
    #[inline]
    pub fn new() -> TraceRaysIndirectCommandKHRBuilder<'a> {
        TraceRaysIndirectCommandKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn width(mut self, width: u32) -> Self {
        self.0.width = width as _;
        self
    }
    #[inline]
    pub fn height(mut self, height: u32) -> Self {
        self.0.height = height as _;
        self
    }
    #[inline]
    pub fn depth(mut self, depth: u32) -> Self {
        self.0.depth = depth as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> TraceRaysIndirectCommandKHR {
        self.0
    }
}
impl<'a> std::default::Default for TraceRaysIndirectCommandKHRBuilder<'a> {
    fn default() -> TraceRaysIndirectCommandKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for TraceRaysIndirectCommandKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
unsafe impl crate::Repr<TraceRaysIndirectCommandKHR> for TraceRaysIndirectCommandKHRBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRayTracingPipelineInterfaceCreateInfoKHR.html) · Structure"]
#[doc(alias = "VkRayTracingPipelineInterfaceCreateInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RayTracingPipelineInterfaceCreateInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub max_pipeline_ray_payload_size: u32,
    pub max_pipeline_ray_hit_attribute_size: u32,
}
impl Default for RayTracingPipelineInterfaceCreateInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::RAY_TRACING_PIPELINE_INTERFACE_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            max_pipeline_ray_payload_size: Default::default(),
            max_pipeline_ray_hit_attribute_size: Default::default(),
        }
    }
}
impl std::fmt::Debug for RayTracingPipelineInterfaceCreateInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RayTracingPipelineInterfaceCreateInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("max_pipeline_ray_payload_size", &self.max_pipeline_ray_payload_size)
            .field("max_pipeline_ray_hit_attribute_size", &self.max_pipeline_ray_hit_attribute_size)
            .finish()
    }
}
impl RayTracingPipelineInterfaceCreateInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> RayTracingPipelineInterfaceCreateInfoKHRBuilder<'a> {
        RayTracingPipelineInterfaceCreateInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRayTracingPipelineInterfaceCreateInfoKHR.html) · Builder of [`RayTracingPipelineInterfaceCreateInfoKHR`]"]
#[repr(transparent)]
pub struct RayTracingPipelineInterfaceCreateInfoKHRBuilder<'a>(RayTracingPipelineInterfaceCreateInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> RayTracingPipelineInterfaceCreateInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> RayTracingPipelineInterfaceCreateInfoKHRBuilder<'a> {
        RayTracingPipelineInterfaceCreateInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn max_pipeline_ray_payload_size(mut self, max_pipeline_ray_payload_size: u32) -> Self {
        self.0.max_pipeline_ray_payload_size = max_pipeline_ray_payload_size as _;
        self
    }
    #[inline]
    pub fn max_pipeline_ray_hit_attribute_size(mut self, max_pipeline_ray_hit_attribute_size: u32) -> Self {
        self.0.max_pipeline_ray_hit_attribute_size = max_pipeline_ray_hit_attribute_size as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> RayTracingPipelineInterfaceCreateInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for RayTracingPipelineInterfaceCreateInfoKHRBuilder<'a> {
    fn default() -> RayTracingPipelineInterfaceCreateInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for RayTracingPipelineInterfaceCreateInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
unsafe impl crate::Repr<RayTracingPipelineInterfaceCreateInfoKHR> for RayTracingPipelineInterfaceCreateInfoKHRBuilder<'_> {}
#[doc = "Provided by [`crate::extensions::khr_ray_tracing_pipeline`]"]
impl crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdTraceRaysKHR.html) · Function"]
    #[doc(alias = "vkCmdTraceRaysKHR")]
    pub unsafe fn cmd_trace_rays_khr(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        raygen_shader_binding_table: &crate::extensions::khr_ray_tracing_pipeline::StridedDeviceAddressRegionKHR,
        miss_shader_binding_table: &crate::extensions::khr_ray_tracing_pipeline::StridedDeviceAddressRegionKHR,
        hit_shader_binding_table: &crate::extensions::khr_ray_tracing_pipeline::StridedDeviceAddressRegionKHR,
        callable_shader_binding_table: &crate::extensions::khr_ray_tracing_pipeline::StridedDeviceAddressRegionKHR,
        width: u32,
        height: u32,
        depth: u32,
    ) -> () {
        let _function = self.cmd_trace_rays_khr.expect("`cmd_trace_rays_khr` is not loaded");
        let _return = _function(
            command_buffer as _,
            raygen_shader_binding_table as _,
            miss_shader_binding_table as _,
            hit_shader_binding_table as _,
            callable_shader_binding_table as _,
            width as _,
            height as _,
            depth as _,
        );
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetRayTracingShaderGroupHandlesKHR.html) · Function"]
    #[doc(alias = "vkGetRayTracingShaderGroupHandlesKHR")]
    pub unsafe fn get_ray_tracing_shader_group_handles_khr(&self, pipeline: crate::vk1_0::Pipeline, first_group: u32, group_count: u32, data_size: usize, data: *mut std::ffi::c_void) -> crate::utils::VulkanResult<()> {
        let _function = self.get_ray_tracing_shader_group_handles_khr.expect("`get_ray_tracing_shader_group_handles_khr` is not loaded");
        let _return = _function(self.handle, pipeline as _, first_group as _, group_count as _, data_size, data);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetRayTracingCaptureReplayShaderGroupHandlesKHR.html) · Function"]
    #[doc(alias = "vkGetRayTracingCaptureReplayShaderGroupHandlesKHR")]
    pub unsafe fn get_ray_tracing_capture_replay_shader_group_handles_khr(&self, pipeline: crate::vk1_0::Pipeline, first_group: u32, group_count: u32, data_size: usize, data: *mut std::ffi::c_void) -> crate::utils::VulkanResult<()> {
        let _function = self.get_ray_tracing_capture_replay_shader_group_handles_khr.expect("`get_ray_tracing_capture_replay_shader_group_handles_khr` is not loaded");
        let _return = _function(self.handle, pipeline as _, first_group as _, group_count as _, data_size, data);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateRayTracingPipelinesKHR.html) · Function"]
    #[doc(alias = "vkCreateRayTracingPipelinesKHR")]
    pub unsafe fn create_ray_tracing_pipelines_khr(
        &self,
        deferred_operation: Option<crate::extensions::khr_deferred_host_operations::DeferredOperationKHR>,
        pipeline_cache: Option<crate::vk1_0::PipelineCache>,
        create_infos: &[impl crate::Repr<crate::extensions::khr_ray_tracing_pipeline::RayTracingPipelineCreateInfoKHR>],
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
    ) -> crate::utils::VulkanResult<Vec<crate::vk1_0::Pipeline>> {
        let _function = self.create_ray_tracing_pipelines_khr.expect("`create_ray_tracing_pipelines_khr` is not loaded");
        let create_info_count = create_infos.len();
        let mut pipelines = vec![Default::default(); create_info_count as _];
        let _return = _function(
            self.handle,
            match deferred_operation {
                Some(v) => v,
                None => Default::default(),
            },
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
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdTraceRaysIndirectKHR.html) · Function"]
    #[doc(alias = "vkCmdTraceRaysIndirectKHR")]
    pub unsafe fn cmd_trace_rays_indirect_khr(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        raygen_shader_binding_table: &crate::extensions::khr_ray_tracing_pipeline::StridedDeviceAddressRegionKHR,
        miss_shader_binding_table: &crate::extensions::khr_ray_tracing_pipeline::StridedDeviceAddressRegionKHR,
        hit_shader_binding_table: &crate::extensions::khr_ray_tracing_pipeline::StridedDeviceAddressRegionKHR,
        callable_shader_binding_table: &crate::extensions::khr_ray_tracing_pipeline::StridedDeviceAddressRegionKHR,
        indirect_device_address: crate::vk1_0::DeviceAddress,
    ) -> () {
        let _function = self.cmd_trace_rays_indirect_khr.expect("`cmd_trace_rays_indirect_khr` is not loaded");
        let _return = _function(
            command_buffer as _,
            raygen_shader_binding_table as _,
            miss_shader_binding_table as _,
            hit_shader_binding_table as _,
            callable_shader_binding_table as _,
            indirect_device_address as _,
        );
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetRayTracingShaderGroupStackSizeKHR.html) · Function"]
    #[doc(alias = "vkGetRayTracingShaderGroupStackSizeKHR")]
    pub unsafe fn get_ray_tracing_shader_group_stack_size_khr(&self, pipeline: crate::vk1_0::Pipeline, group: u32, group_shader: crate::extensions::khr_ray_tracing_pipeline::ShaderGroupShaderKHR) -> crate::vk1_0::DeviceSize {
        let _function = self.get_ray_tracing_shader_group_stack_size_khr.expect("`get_ray_tracing_shader_group_stack_size_khr` is not loaded");
        let _return = _function(self.handle, pipeline as _, group as _, group_shader as _);
        _return
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetRayTracingPipelineStackSizeKHR.html) · Function"]
    #[doc(alias = "vkCmdSetRayTracingPipelineStackSizeKHR")]
    pub unsafe fn cmd_set_ray_tracing_pipeline_stack_size_khr(&self, command_buffer: crate::vk1_0::CommandBuffer, pipeline_stack_size: u32) -> () {
        let _function = self.cmd_set_ray_tracing_pipeline_stack_size_khr.expect("`cmd_set_ray_tracing_pipeline_stack_size_khr` is not loaded");
        let _return = _function(command_buffer as _, pipeline_stack_size as _);
        ()
    }
}
