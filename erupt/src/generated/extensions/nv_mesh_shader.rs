#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_MESH_SHADER_SPEC_VERSION")]
pub const NV_MESH_SHADER_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_MESH_SHADER_EXTENSION_NAME")]
pub const NV_MESH_SHADER_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_NV_mesh_shader");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_DRAW_MESH_TASKS_NV: *const std::os::raw::c_char = crate::cstr!("vkCmdDrawMeshTasksNV");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_DRAW_MESH_TASKS_INDIRECT_NV: *const std::os::raw::c_char = crate::cstr!("vkCmdDrawMeshTasksIndirectNV");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_DRAW_MESH_TASKS_INDIRECT_COUNT_NV: *const std::os::raw::c_char = crate::cstr!("vkCmdDrawMeshTasksIndirectCountNV");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawMeshTasksNV.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawMeshTasksNV = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, task_count: u32, first_task: u32) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawMeshTasksIndirectNV.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawMeshTasksIndirectNV = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, buffer: crate::vk1_0::Buffer, offset: crate::vk1_0::DeviceSize, draw_count: u32, stride: u32) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawMeshTasksIndirectCountNV.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawMeshTasksIndirectCountNV = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, buffer: crate::vk1_0::Buffer, offset: crate::vk1_0::DeviceSize, count_buffer: crate::vk1_0::Buffer, count_buffer_offset: crate::vk1_0::DeviceSize, max_draw_count: u32, stride: u32) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMeshShaderFeaturesNV.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceMeshShaderFeaturesNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceMeshShaderFeaturesNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub task_shader: crate::vk1_0::Bool32,
    pub mesh_shader: crate::vk1_0::Bool32,
}
impl Default for PhysicalDeviceMeshShaderFeaturesNV {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_MESH_SHADER_FEATURES_NV, p_next: std::ptr::null_mut(), task_shader: Default::default(), mesh_shader: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceMeshShaderFeaturesNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceMeshShaderFeaturesNV").field("s_type", &self.s_type).field("p_next", &self.p_next).field("task_shader", &(self.task_shader != 0)).field("mesh_shader", &(self.mesh_shader != 0)).finish()
    }
}
impl PhysicalDeviceMeshShaderFeaturesNV {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceMeshShaderFeaturesNVBuilder<'a> {
        PhysicalDeviceMeshShaderFeaturesNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMeshShaderFeaturesNV.html) · Builder of [`PhysicalDeviceMeshShaderFeaturesNV`]"]
#[repr(transparent)]
pub struct PhysicalDeviceMeshShaderFeaturesNVBuilder<'a>(PhysicalDeviceMeshShaderFeaturesNV, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceMeshShaderFeaturesNVBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceMeshShaderFeaturesNVBuilder<'a> {
        PhysicalDeviceMeshShaderFeaturesNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn task_shader(mut self, task_shader: bool) -> Self {
        self.0.task_shader = task_shader as _;
        self
    }
    #[inline]
    pub fn mesh_shader(mut self, mesh_shader: bool) -> Self {
        self.0.mesh_shader = mesh_shader as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceMeshShaderFeaturesNV {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceMeshShaderFeaturesNVBuilder<'a> {
    fn default() -> PhysicalDeviceMeshShaderFeaturesNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceMeshShaderFeaturesNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceMeshShaderFeaturesNVBuilder<'a> {
    type Target = PhysicalDeviceMeshShaderFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceMeshShaderFeaturesNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMeshShaderPropertiesNV.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceMeshShaderPropertiesNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceMeshShaderPropertiesNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub max_draw_mesh_tasks_count: u32,
    pub max_task_work_group_invocations: u32,
    pub max_task_work_group_size: [u32; 3],
    pub max_task_total_memory_size: u32,
    pub max_task_output_count: u32,
    pub max_mesh_work_group_invocations: u32,
    pub max_mesh_work_group_size: [u32; 3],
    pub max_mesh_total_memory_size: u32,
    pub max_mesh_output_vertices: u32,
    pub max_mesh_output_primitives: u32,
    pub max_mesh_multiview_view_count: u32,
    pub mesh_output_per_vertex_granularity: u32,
    pub mesh_output_per_primitive_granularity: u32,
}
impl Default for PhysicalDeviceMeshShaderPropertiesNV {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_NV, p_next: std::ptr::null_mut(), max_draw_mesh_tasks_count: Default::default(), max_task_work_group_invocations: Default::default(), max_task_work_group_size: unsafe { std::mem::zeroed() }, max_task_total_memory_size: Default::default(), max_task_output_count: Default::default(), max_mesh_work_group_invocations: Default::default(), max_mesh_work_group_size: unsafe { std::mem::zeroed() }, max_mesh_total_memory_size: Default::default(), max_mesh_output_vertices: Default::default(), max_mesh_output_primitives: Default::default(), max_mesh_multiview_view_count: Default::default(), mesh_output_per_vertex_granularity: Default::default(), mesh_output_per_primitive_granularity: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceMeshShaderPropertiesNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceMeshShaderPropertiesNV").field("s_type", &self.s_type).field("p_next", &self.p_next).field("max_draw_mesh_tasks_count", &self.max_draw_mesh_tasks_count).field("max_task_work_group_invocations", &self.max_task_work_group_invocations).field("max_task_work_group_size", &self.max_task_work_group_size).field("max_task_total_memory_size", &self.max_task_total_memory_size).field("max_task_output_count", &self.max_task_output_count).field("max_mesh_work_group_invocations", &self.max_mesh_work_group_invocations).field("max_mesh_work_group_size", &self.max_mesh_work_group_size).field("max_mesh_total_memory_size", &self.max_mesh_total_memory_size).field("max_mesh_output_vertices", &self.max_mesh_output_vertices).field("max_mesh_output_primitives", &self.max_mesh_output_primitives).field("max_mesh_multiview_view_count", &self.max_mesh_multiview_view_count).field("mesh_output_per_vertex_granularity", &self.mesh_output_per_vertex_granularity).field("mesh_output_per_primitive_granularity", &self.mesh_output_per_primitive_granularity).finish()
    }
}
impl PhysicalDeviceMeshShaderPropertiesNV {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceMeshShaderPropertiesNVBuilder<'a> {
        PhysicalDeviceMeshShaderPropertiesNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMeshShaderPropertiesNV.html) · Builder of [`PhysicalDeviceMeshShaderPropertiesNV`]"]
#[repr(transparent)]
pub struct PhysicalDeviceMeshShaderPropertiesNVBuilder<'a>(PhysicalDeviceMeshShaderPropertiesNV, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceMeshShaderPropertiesNVBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceMeshShaderPropertiesNVBuilder<'a> {
        PhysicalDeviceMeshShaderPropertiesNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn max_draw_mesh_tasks_count(mut self, max_draw_mesh_tasks_count: u32) -> Self {
        self.0.max_draw_mesh_tasks_count = max_draw_mesh_tasks_count as _;
        self
    }
    #[inline]
    pub fn max_task_work_group_invocations(mut self, max_task_work_group_invocations: u32) -> Self {
        self.0.max_task_work_group_invocations = max_task_work_group_invocations as _;
        self
    }
    #[inline]
    pub fn max_task_work_group_size(mut self, max_task_work_group_size: [u32; 3]) -> Self {
        self.0.max_task_work_group_size = max_task_work_group_size as _;
        self
    }
    #[inline]
    pub fn max_task_total_memory_size(mut self, max_task_total_memory_size: u32) -> Self {
        self.0.max_task_total_memory_size = max_task_total_memory_size as _;
        self
    }
    #[inline]
    pub fn max_task_output_count(mut self, max_task_output_count: u32) -> Self {
        self.0.max_task_output_count = max_task_output_count as _;
        self
    }
    #[inline]
    pub fn max_mesh_work_group_invocations(mut self, max_mesh_work_group_invocations: u32) -> Self {
        self.0.max_mesh_work_group_invocations = max_mesh_work_group_invocations as _;
        self
    }
    #[inline]
    pub fn max_mesh_work_group_size(mut self, max_mesh_work_group_size: [u32; 3]) -> Self {
        self.0.max_mesh_work_group_size = max_mesh_work_group_size as _;
        self
    }
    #[inline]
    pub fn max_mesh_total_memory_size(mut self, max_mesh_total_memory_size: u32) -> Self {
        self.0.max_mesh_total_memory_size = max_mesh_total_memory_size as _;
        self
    }
    #[inline]
    pub fn max_mesh_output_vertices(mut self, max_mesh_output_vertices: u32) -> Self {
        self.0.max_mesh_output_vertices = max_mesh_output_vertices as _;
        self
    }
    #[inline]
    pub fn max_mesh_output_primitives(mut self, max_mesh_output_primitives: u32) -> Self {
        self.0.max_mesh_output_primitives = max_mesh_output_primitives as _;
        self
    }
    #[inline]
    pub fn max_mesh_multiview_view_count(mut self, max_mesh_multiview_view_count: u32) -> Self {
        self.0.max_mesh_multiview_view_count = max_mesh_multiview_view_count as _;
        self
    }
    #[inline]
    pub fn mesh_output_per_vertex_granularity(mut self, mesh_output_per_vertex_granularity: u32) -> Self {
        self.0.mesh_output_per_vertex_granularity = mesh_output_per_vertex_granularity as _;
        self
    }
    #[inline]
    pub fn mesh_output_per_primitive_granularity(mut self, mesh_output_per_primitive_granularity: u32) -> Self {
        self.0.mesh_output_per_primitive_granularity = mesh_output_per_primitive_granularity as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceMeshShaderPropertiesNV {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceMeshShaderPropertiesNVBuilder<'a> {
    fn default() -> PhysicalDeviceMeshShaderPropertiesNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceMeshShaderPropertiesNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceMeshShaderPropertiesNVBuilder<'a> {
    type Target = PhysicalDeviceMeshShaderPropertiesNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceMeshShaderPropertiesNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDrawMeshTasksIndirectCommandNV.html) · Structure"]
#[doc(alias = "VkDrawMeshTasksIndirectCommandNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DrawMeshTasksIndirectCommandNV {
    pub task_count: u32,
    pub first_task: u32,
}
impl Default for DrawMeshTasksIndirectCommandNV {
    fn default() -> Self {
        Self { task_count: Default::default(), first_task: Default::default() }
    }
}
impl std::fmt::Debug for DrawMeshTasksIndirectCommandNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DrawMeshTasksIndirectCommandNV").field("task_count", &self.task_count).field("first_task", &self.first_task).finish()
    }
}
impl DrawMeshTasksIndirectCommandNV {
    #[inline]
    pub fn into_builder<'a>(self) -> DrawMeshTasksIndirectCommandNVBuilder<'a> {
        DrawMeshTasksIndirectCommandNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDrawMeshTasksIndirectCommandNV.html) · Builder of [`DrawMeshTasksIndirectCommandNV`]"]
#[repr(transparent)]
pub struct DrawMeshTasksIndirectCommandNVBuilder<'a>(DrawMeshTasksIndirectCommandNV, std::marker::PhantomData<&'a ()>);
impl<'a> DrawMeshTasksIndirectCommandNVBuilder<'a> {
    #[inline]
    pub fn new() -> DrawMeshTasksIndirectCommandNVBuilder<'a> {
        DrawMeshTasksIndirectCommandNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn task_count(mut self, task_count: u32) -> Self {
        self.0.task_count = task_count as _;
        self
    }
    #[inline]
    pub fn first_task(mut self, first_task: u32) -> Self {
        self.0.first_task = first_task as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DrawMeshTasksIndirectCommandNV {
        self.0
    }
}
impl<'a> std::default::Default for DrawMeshTasksIndirectCommandNVBuilder<'a> {
    fn default() -> DrawMeshTasksIndirectCommandNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DrawMeshTasksIndirectCommandNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DrawMeshTasksIndirectCommandNVBuilder<'a> {
    type Target = DrawMeshTasksIndirectCommandNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DrawMeshTasksIndirectCommandNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::nv_mesh_shader`]"]
impl crate::DeviceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawMeshTasksNV.html) · Function"]
    #[doc(alias = "vkCmdDrawMeshTasksNV")]
    pub unsafe fn cmd_draw_mesh_tasks_nv(&self, command_buffer: crate::vk1_0::CommandBuffer, task_count: u32, first_task: u32) -> () {
        let _function = self.cmd_draw_mesh_tasks_nv.expect("tried to call a function that isn't loaded");
        let _return = _function(command_buffer as _, task_count as _, first_task as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawMeshTasksIndirectNV.html) · Function"]
    #[doc(alias = "vkCmdDrawMeshTasksIndirectNV")]
    pub unsafe fn cmd_draw_mesh_tasks_indirect_nv(&self, command_buffer: crate::vk1_0::CommandBuffer, buffer: crate::vk1_0::Buffer, offset: crate::vk1_0::DeviceSize, draw_count: u32, stride: u32) -> () {
        let _function = self.cmd_draw_mesh_tasks_indirect_nv.expect("tried to call a function that isn't loaded");
        let _return = _function(command_buffer as _, buffer as _, offset as _, draw_count as _, stride as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawMeshTasksIndirectCountNV.html) · Function"]
    #[doc(alias = "vkCmdDrawMeshTasksIndirectCountNV")]
    pub unsafe fn cmd_draw_mesh_tasks_indirect_count_nv(&self, command_buffer: crate::vk1_0::CommandBuffer, buffer: crate::vk1_0::Buffer, offset: crate::vk1_0::DeviceSize, count_buffer: crate::vk1_0::Buffer, count_buffer_offset: crate::vk1_0::DeviceSize, max_draw_count: u32, stride: u32) -> () {
        let _function = self.cmd_draw_mesh_tasks_indirect_count_nv.expect("tried to call a function that isn't loaded");
        let _return = _function(command_buffer as _, buffer as _, offset as _, count_buffer as _, count_buffer_offset as _, max_draw_count as _, stride as _);
        ()
    }
}
