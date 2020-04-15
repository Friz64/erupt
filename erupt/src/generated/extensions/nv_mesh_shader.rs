# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_NV_mesh_shader.html)\n\n## Extends\n- [`PipelineStageFlagBits`](../../vk1_0/struct.PipelineStageFlagBits.html)\n- [`ShaderStageFlagBits`](../../vk1_0/struct.ShaderStageFlagBits.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NV_MESH_SHADER_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NV_MESH_SHADER_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_NV_mesh_shader");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawMeshTasksNV.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawMeshTasksNV = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    task_count: u32,
    first_task: u32,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawMeshTasksIndirectNV.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawMeshTasksIndirectNV = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    buffer: crate::vk1_0::Buffer,
    offset: crate::vk1_0::DeviceSize,
    draw_count: u32,
    stride: u32,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawMeshTasksIndirectCountNV.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawMeshTasksIndirectCountNV = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    buffer: crate::vk1_0::Buffer,
    offset: crate::vk1_0::DeviceSize,
    count_buffer: crate::vk1_0::Buffer,
    count_buffer_offset: crate::vk1_0::DeviceSize,
    max_draw_count: u32,
    stride: u32,
) -> std::ffi::c_void;
#[doc = "Provides Device Commands for [`NvMeshShaderDeviceLoaderExt`](trait.NvMeshShaderDeviceLoaderExt.html)"]
pub struct NvMeshShaderDeviceCommands {
    pub cmd_draw_mesh_tasks_nv: PFN_vkCmdDrawMeshTasksNV,
    pub cmd_draw_mesh_tasks_indirect_nv: PFN_vkCmdDrawMeshTasksIndirectNV,
    pub cmd_draw_mesh_tasks_indirect_count_nv: PFN_vkCmdDrawMeshTasksIndirectCountNV,
}
impl NvMeshShaderDeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<NvMeshShaderDeviceCommands> {
        unsafe {
            Some(NvMeshShaderDeviceCommands {
                cmd_draw_mesh_tasks_nv: std::mem::transmute(loader.symbol("vkCmdDrawMeshTasksNV")?),
                cmd_draw_mesh_tasks_indirect_nv: std::mem::transmute(
                    loader.symbol("vkCmdDrawMeshTasksIndirectNV")?,
                ),
                cmd_draw_mesh_tasks_indirect_count_nv: std::mem::transmute(
                    loader.symbol("vkCmdDrawMeshTasksIndirectCountNV")?,
                ),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`NvMeshShaderDeviceCommands`](struct.NvMeshShaderDeviceCommands.html)"]
pub trait NvMeshShaderDeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawMeshTasksNV.html) · Device Command"]
    unsafe fn cmd_draw_mesh_tasks_nv(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        task_count: u32,
        first_task: u32,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawMeshTasksIndirectNV.html) · Device Command"]
    unsafe fn cmd_draw_mesh_tasks_indirect_nv(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        buffer: crate::vk1_0::Buffer,
        offset: crate::vk1_0::DeviceSize,
        draw_count: u32,
        stride: u32,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawMeshTasksIndirectCountNV.html) · Device Command"]
    unsafe fn cmd_draw_mesh_tasks_indirect_count_nv(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        buffer: crate::vk1_0::Buffer,
        offset: crate::vk1_0::DeviceSize,
        count_buffer: crate::vk1_0::Buffer,
        count_buffer_offset: crate::vk1_0::DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) -> ();
}
impl NvMeshShaderDeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawMeshTasksNV.html) · Device Command"]
    unsafe fn cmd_draw_mesh_tasks_nv(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        task_count: u32,
        first_task: u32,
    ) -> () {
        let function = self
            .nv_mesh_shader
            .as_ref()
            .expect("`nv_mesh_shader` not loaded")
            .cmd_draw_mesh_tasks_nv;
        let _val = function(command_buffer, task_count, first_task);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawMeshTasksIndirectNV.html) · Device Command"]
    unsafe fn cmd_draw_mesh_tasks_indirect_nv(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        buffer: crate::vk1_0::Buffer,
        offset: crate::vk1_0::DeviceSize,
        draw_count: u32,
        stride: u32,
    ) -> () {
        let function = self
            .nv_mesh_shader
            .as_ref()
            .expect("`nv_mesh_shader` not loaded")
            .cmd_draw_mesh_tasks_indirect_nv;
        let _val = function(command_buffer, buffer, offset, draw_count, stride);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawMeshTasksIndirectCountNV.html) · Device Command"]
    unsafe fn cmd_draw_mesh_tasks_indirect_count_nv(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        buffer: crate::vk1_0::Buffer,
        offset: crate::vk1_0::DeviceSize,
        count_buffer: crate::vk1_0::Buffer,
        count_buffer_offset: crate::vk1_0::DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) -> () {
        let function = self
            .nv_mesh_shader
            .as_ref()
            .expect("`nv_mesh_shader` not loaded")
            .cmd_draw_mesh_tasks_indirect_count_nv;
        let _val = function(
            command_buffer,
            buffer,
            offset,
            count_buffer,
            count_buffer_offset,
            max_draw_count,
            stride,
        );
        ()
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMeshShaderFeaturesNV.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceMeshShaderFeaturesNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub task_shader: crate::vk1_0::Bool32,
    pub mesh_shader: crate::vk1_0::Bool32,
}
impl PhysicalDeviceMeshShaderFeaturesNV {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceMeshShaderFeaturesNV,
    {
        unsafe {
            crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
        }
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceMeshShaderFeaturesNVBuilder<'a> {
        PhysicalDeviceMeshShaderFeaturesNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceMeshShaderFeaturesNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceMeshShaderFeaturesNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("task_shader", &(self.task_shader != 0))
            .field("mesh_shader", &(self.mesh_shader != 0))
            .finish()
    }
}
impl Default for PhysicalDeviceMeshShaderFeaturesNV {
    fn default() -> PhysicalDeviceMeshShaderFeaturesNV {
        PhysicalDeviceMeshShaderFeaturesNV {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_MESH_SHADER_FEATURES_NV,
            p_next: std::ptr::null_mut(),
            task_shader: Default::default(),
            mesh_shader: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceMeshShaderFeaturesNV::extend`](struct.PhysicalDeviceMeshShaderFeaturesNV.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceMeshShaderFeaturesNV {}
impl ExtendableByPhysicalDeviceMeshShaderFeaturesNV for crate::vk1_1::PhysicalDeviceFeatures2 {}
impl ExtendableByPhysicalDeviceMeshShaderFeaturesNV for crate::vk1_0::DeviceCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceMeshShaderFeaturesNV`](struct.PhysicalDeviceMeshShaderFeaturesNV.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceMeshShaderFeaturesNVBuilder<'a>(
    PhysicalDeviceMeshShaderFeaturesNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceMeshShaderFeaturesNVBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceMeshShaderFeaturesNVBuilder<'a> {
        PhysicalDeviceMeshShaderFeaturesNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn task_shader(mut self, task_shader: bool) -> Self {
        self.0.task_shader = task_shader as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn mesh_shader(mut self, mesh_shader: bool) -> Self {
        self.0.mesh_shader = mesh_shader as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceMeshShaderFeaturesNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceMeshShaderFeaturesNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
impl PhysicalDeviceMeshShaderPropertiesNV {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceMeshShaderPropertiesNV,
    {
        unsafe {
            crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
        }
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceMeshShaderPropertiesNVBuilder<'a> {
        PhysicalDeviceMeshShaderPropertiesNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceMeshShaderPropertiesNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceMeshShaderPropertiesNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("max_draw_mesh_tasks_count", &self.max_draw_mesh_tasks_count)
            .field(
                "max_task_work_group_invocations",
                &self.max_task_work_group_invocations,
            )
            .field("max_task_work_group_size", &self.max_task_work_group_size)
            .field(
                "max_task_total_memory_size",
                &self.max_task_total_memory_size,
            )
            .field("max_task_output_count", &self.max_task_output_count)
            .field(
                "max_mesh_work_group_invocations",
                &self.max_mesh_work_group_invocations,
            )
            .field("max_mesh_work_group_size", &self.max_mesh_work_group_size)
            .field(
                "max_mesh_total_memory_size",
                &self.max_mesh_total_memory_size,
            )
            .field("max_mesh_output_vertices", &self.max_mesh_output_vertices)
            .field(
                "max_mesh_output_primitives",
                &self.max_mesh_output_primitives,
            )
            .field(
                "max_mesh_multiview_view_count",
                &self.max_mesh_multiview_view_count,
            )
            .field(
                "mesh_output_per_vertex_granularity",
                &self.mesh_output_per_vertex_granularity,
            )
            .field(
                "mesh_output_per_primitive_granularity",
                &self.mesh_output_per_primitive_granularity,
            )
            .finish()
    }
}
impl Default for PhysicalDeviceMeshShaderPropertiesNV {
    fn default() -> PhysicalDeviceMeshShaderPropertiesNV {
        PhysicalDeviceMeshShaderPropertiesNV {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_NV,
            p_next: std::ptr::null_mut(),
            max_draw_mesh_tasks_count: Default::default(),
            max_task_work_group_invocations: Default::default(),
            max_task_work_group_size: Default::default(),
            max_task_total_memory_size: Default::default(),
            max_task_output_count: Default::default(),
            max_mesh_work_group_invocations: Default::default(),
            max_mesh_work_group_size: Default::default(),
            max_mesh_total_memory_size: Default::default(),
            max_mesh_output_vertices: Default::default(),
            max_mesh_output_primitives: Default::default(),
            max_mesh_multiview_view_count: Default::default(),
            mesh_output_per_vertex_granularity: Default::default(),
            mesh_output_per_primitive_granularity: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceMeshShaderPropertiesNV::extend`](struct.PhysicalDeviceMeshShaderPropertiesNV.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceMeshShaderPropertiesNV {}
impl ExtendableByPhysicalDeviceMeshShaderPropertiesNV for crate::vk1_1::PhysicalDeviceProperties2 {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceMeshShaderPropertiesNV`](struct.PhysicalDeviceMeshShaderPropertiesNV.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceMeshShaderPropertiesNVBuilder<'a>(
    PhysicalDeviceMeshShaderPropertiesNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceMeshShaderPropertiesNVBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceMeshShaderPropertiesNVBuilder<'a> {
        PhysicalDeviceMeshShaderPropertiesNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_draw_mesh_tasks_count(mut self, max_draw_mesh_tasks_count: u32) -> Self {
        self.0.max_draw_mesh_tasks_count = max_draw_mesh_tasks_count;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_task_work_group_invocations(mut self, max_task_work_group_invocations: u32) -> Self {
        self.0.max_task_work_group_invocations = max_task_work_group_invocations;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_task_work_group_size(mut self, max_task_work_group_size: [u32; 3]) -> Self {
        self.0.max_task_work_group_size = max_task_work_group_size;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_task_total_memory_size(mut self, max_task_total_memory_size: u32) -> Self {
        self.0.max_task_total_memory_size = max_task_total_memory_size;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_task_output_count(mut self, max_task_output_count: u32) -> Self {
        self.0.max_task_output_count = max_task_output_count;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_mesh_work_group_invocations(mut self, max_mesh_work_group_invocations: u32) -> Self {
        self.0.max_mesh_work_group_invocations = max_mesh_work_group_invocations;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_mesh_work_group_size(mut self, max_mesh_work_group_size: [u32; 3]) -> Self {
        self.0.max_mesh_work_group_size = max_mesh_work_group_size;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_mesh_total_memory_size(mut self, max_mesh_total_memory_size: u32) -> Self {
        self.0.max_mesh_total_memory_size = max_mesh_total_memory_size;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_mesh_output_vertices(mut self, max_mesh_output_vertices: u32) -> Self {
        self.0.max_mesh_output_vertices = max_mesh_output_vertices;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_mesh_output_primitives(mut self, max_mesh_output_primitives: u32) -> Self {
        self.0.max_mesh_output_primitives = max_mesh_output_primitives;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_mesh_multiview_view_count(mut self, max_mesh_multiview_view_count: u32) -> Self {
        self.0.max_mesh_multiview_view_count = max_mesh_multiview_view_count;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn mesh_output_per_vertex_granularity(
        mut self,
        mesh_output_per_vertex_granularity: u32,
    ) -> Self {
        self.0.mesh_output_per_vertex_granularity = mesh_output_per_vertex_granularity;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn mesh_output_per_primitive_granularity(
        mut self,
        mesh_output_per_primitive_granularity: u32,
    ) -> Self {
        self.0.mesh_output_per_primitive_granularity = mesh_output_per_primitive_granularity;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceMeshShaderPropertiesNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceMeshShaderPropertiesNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DrawMeshTasksIndirectCommandNV {
    pub task_count: u32,
    pub first_task: u32,
}
impl DrawMeshTasksIndirectCommandNV {
    #[inline]
    pub fn builder<'a>(self) -> DrawMeshTasksIndirectCommandNVBuilder<'a> {
        DrawMeshTasksIndirectCommandNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for DrawMeshTasksIndirectCommandNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("DrawMeshTasksIndirectCommandNV")
            .field("task_count", &self.task_count)
            .field("first_task", &self.first_task)
            .finish()
    }
}
impl Default for DrawMeshTasksIndirectCommandNV {
    fn default() -> DrawMeshTasksIndirectCommandNV {
        DrawMeshTasksIndirectCommandNV {
            task_count: Default::default(),
            first_task: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`DrawMeshTasksIndirectCommandNV`](struct.DrawMeshTasksIndirectCommandNV.html)"]
#[repr(transparent)]
pub struct DrawMeshTasksIndirectCommandNVBuilder<'a>(
    DrawMeshTasksIndirectCommandNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> DrawMeshTasksIndirectCommandNVBuilder<'a> {
    #[inline]
    pub fn new() -> DrawMeshTasksIndirectCommandNVBuilder<'a> {
        DrawMeshTasksIndirectCommandNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn task_count(mut self, task_count: u32) -> Self {
        self.0.task_count = task_count;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn first_task(mut self, first_task: u32) -> Self {
        self.0.first_task = first_task;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> DrawMeshTasksIndirectCommandNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for DrawMeshTasksIndirectCommandNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
