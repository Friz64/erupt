# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_EXT_extended_dynamic_state.html)\n\n## Extends\n- [`DynamicState`](../../vk1_0/struct.DynamicState.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_EXTENDED_DYNAMIC_STATE_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_EXTENDED_DYNAMIC_STATE_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_extended_dynamic_state");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetCullModeEXT.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetCullModeEXT = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    cull_mode: crate::vk1_0::CullModeFlags,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetFrontFaceEXT.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetFrontFaceEXT = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    front_face: crate::vk1_0::FrontFace,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetPrimitiveTopologyEXT.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetPrimitiveTopologyEXT = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    primitive_topology: crate::vk1_0::PrimitiveTopology,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetViewportWithCountEXT.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetViewportWithCountEXT = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    viewport_count: u32,
    p_viewports: *const crate::vk1_0::Viewport,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetScissorWithCountEXT.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetScissorWithCountEXT = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    scissor_count: u32,
    p_scissors: *const crate::vk1_0::Rect2D,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBindVertexBuffers2EXT.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindVertexBuffers2EXT = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    first_binding: u32,
    binding_count: u32,
    p_buffers: *const crate::vk1_0::Buffer,
    p_offsets: *const crate::vk1_0::DeviceSize,
    p_sizes: *const crate::vk1_0::DeviceSize,
    p_strides: *const crate::vk1_0::DeviceSize,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDepthTestEnableEXT.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDepthTestEnableEXT = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    depth_test_enable: crate::vk1_0::Bool32,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDepthWriteEnableEXT.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDepthWriteEnableEXT = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    depth_write_enable: crate::vk1_0::Bool32,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDepthCompareOpEXT.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDepthCompareOpEXT = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    depth_compare_op: crate::vk1_0::CompareOp,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDepthBoundsTestEnableEXT.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDepthBoundsTestEnableEXT = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    depth_bounds_test_enable: crate::vk1_0::Bool32,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetStencilTestEnableEXT.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetStencilTestEnableEXT = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    stencil_test_enable: crate::vk1_0::Bool32,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetStencilOpEXT.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetStencilOpEXT = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    face_mask: crate::vk1_0::StencilFaceFlags,
    fail_op: crate::vk1_0::StencilOp,
    pass_op: crate::vk1_0::StencilOp,
    depth_fail_op: crate::vk1_0::StencilOp,
    compare_op: crate::vk1_0::CompareOp,
) -> std::ffi::c_void;
#[doc = "Provides Device Commands for [`ExtExtendedDynamicStateDeviceLoaderExt`](trait.ExtExtendedDynamicStateDeviceLoaderExt.html)"]
pub struct ExtExtendedDynamicStateDeviceCommands {
    pub cmd_set_cull_mode_ext: Option<PFN_vkCmdSetCullModeEXT>,
    pub cmd_set_front_face_ext: Option<PFN_vkCmdSetFrontFaceEXT>,
    pub cmd_set_primitive_topology_ext: Option<PFN_vkCmdSetPrimitiveTopologyEXT>,
    pub cmd_set_viewport_with_count_ext: Option<PFN_vkCmdSetViewportWithCountEXT>,
    pub cmd_set_scissor_with_count_ext: Option<PFN_vkCmdSetScissorWithCountEXT>,
    pub cmd_bind_vertex_buffers2_ext: Option<PFN_vkCmdBindVertexBuffers2EXT>,
    pub cmd_set_depth_test_enable_ext: Option<PFN_vkCmdSetDepthTestEnableEXT>,
    pub cmd_set_depth_write_enable_ext: Option<PFN_vkCmdSetDepthWriteEnableEXT>,
    pub cmd_set_depth_compare_op_ext: Option<PFN_vkCmdSetDepthCompareOpEXT>,
    pub cmd_set_depth_bounds_test_enable_ext: Option<PFN_vkCmdSetDepthBoundsTestEnableEXT>,
    pub cmd_set_stencil_test_enable_ext: Option<PFN_vkCmdSetStencilTestEnableEXT>,
    pub cmd_set_stencil_op_ext: Option<PFN_vkCmdSetStencilOpEXT>,
}
impl ExtExtendedDynamicStateDeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<ExtExtendedDynamicStateDeviceCommands> {
        unsafe {
            let mut success = false;
            let table = ExtExtendedDynamicStateDeviceCommands {
                cmd_set_cull_mode_ext: std::mem::transmute({
                    let symbol = loader.symbol("vkCmdSetCullModeEXT");
                    success |= symbol.is_some();
                    symbol
                }),
                cmd_set_front_face_ext: std::mem::transmute({
                    let symbol = loader.symbol("vkCmdSetFrontFaceEXT");
                    success |= symbol.is_some();
                    symbol
                }),
                cmd_set_primitive_topology_ext: std::mem::transmute({
                    let symbol = loader.symbol("vkCmdSetPrimitiveTopologyEXT");
                    success |= symbol.is_some();
                    symbol
                }),
                cmd_set_viewport_with_count_ext: std::mem::transmute({
                    let symbol = loader.symbol("vkCmdSetViewportWithCountEXT");
                    success |= symbol.is_some();
                    symbol
                }),
                cmd_set_scissor_with_count_ext: std::mem::transmute({
                    let symbol = loader.symbol("vkCmdSetScissorWithCountEXT");
                    success |= symbol.is_some();
                    symbol
                }),
                cmd_bind_vertex_buffers2_ext: std::mem::transmute({
                    let symbol = loader.symbol("vkCmdBindVertexBuffers2EXT");
                    success |= symbol.is_some();
                    symbol
                }),
                cmd_set_depth_test_enable_ext: std::mem::transmute({
                    let symbol = loader.symbol("vkCmdSetDepthTestEnableEXT");
                    success |= symbol.is_some();
                    symbol
                }),
                cmd_set_depth_write_enable_ext: std::mem::transmute({
                    let symbol = loader.symbol("vkCmdSetDepthWriteEnableEXT");
                    success |= symbol.is_some();
                    symbol
                }),
                cmd_set_depth_compare_op_ext: std::mem::transmute({
                    let symbol = loader.symbol("vkCmdSetDepthCompareOpEXT");
                    success |= symbol.is_some();
                    symbol
                }),
                cmd_set_depth_bounds_test_enable_ext: std::mem::transmute({
                    let symbol = loader.symbol("vkCmdSetDepthBoundsTestEnableEXT");
                    success |= symbol.is_some();
                    symbol
                }),
                cmd_set_stencil_test_enable_ext: std::mem::transmute({
                    let symbol = loader.symbol("vkCmdSetStencilTestEnableEXT");
                    success |= symbol.is_some();
                    symbol
                }),
                cmd_set_stencil_op_ext: std::mem::transmute({
                    let symbol = loader.symbol("vkCmdSetStencilOpEXT");
                    success |= symbol.is_some();
                    symbol
                }),
            };
            if success {
                Some(table)
            } else {
                None
            }
        }
    }
}
#[inline]
fn device_commands(loader: &crate::DeviceLoader) -> &ExtExtendedDynamicStateDeviceCommands {
    loader
        .ext_extended_dynamic_state
        .as_ref()
        .expect("`ext_extended_dynamic_state` not loaded")
}
#[doc = "Provides high level command wrappers for [`ExtExtendedDynamicStateDeviceCommands`](struct.ExtExtendedDynamicStateDeviceCommands.html)"]
pub trait ExtExtendedDynamicStateDeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetCullModeEXT.html) · Device Command"]
    unsafe fn cmd_set_cull_mode_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        cull_mode: crate::vk1_0::CullModeFlags,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetFrontFaceEXT.html) · Device Command"]
    unsafe fn cmd_set_front_face_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        front_face: crate::vk1_0::FrontFace,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetPrimitiveTopologyEXT.html) · Device Command"]
    unsafe fn cmd_set_primitive_topology_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        primitive_topology: crate::vk1_0::PrimitiveTopology,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetViewportWithCountEXT.html) · Device Command"]
    unsafe fn cmd_set_viewport_with_count_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        viewports: &[crate::vk1_0::ViewportBuilder],
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetScissorWithCountEXT.html) · Device Command"]
    unsafe fn cmd_set_scissor_with_count_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        scissors: &[crate::vk1_0::Rect2DBuilder],
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBindVertexBuffers2EXT.html) · Device Command"]
    unsafe fn cmd_bind_vertex_buffers2_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        first_binding: u32,
        buffers: &[crate::vk1_0::Buffer],
        offsets: &[crate::vk1_0::DeviceSize],
        sizes: &[crate::vk1_0::DeviceSize],
        strides: &[crate::vk1_0::DeviceSize],
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDepthTestEnableEXT.html) · Device Command"]
    unsafe fn cmd_set_depth_test_enable_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        depth_test_enable: bool,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDepthWriteEnableEXT.html) · Device Command"]
    unsafe fn cmd_set_depth_write_enable_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        depth_write_enable: bool,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDepthCompareOpEXT.html) · Device Command"]
    unsafe fn cmd_set_depth_compare_op_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        depth_compare_op: crate::vk1_0::CompareOp,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDepthBoundsTestEnableEXT.html) · Device Command"]
    unsafe fn cmd_set_depth_bounds_test_enable_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        depth_bounds_test_enable: bool,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetStencilTestEnableEXT.html) · Device Command"]
    unsafe fn cmd_set_stencil_test_enable_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        stencil_test_enable: bool,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetStencilOpEXT.html) · Device Command"]
    unsafe fn cmd_set_stencil_op_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        face_mask: crate::vk1_0::StencilFaceFlags,
        fail_op: crate::vk1_0::StencilOp,
        pass_op: crate::vk1_0::StencilOp,
        depth_fail_op: crate::vk1_0::StencilOp,
        compare_op: crate::vk1_0::CompareOp,
    ) -> ();
}
impl ExtExtendedDynamicStateDeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetCullModeEXT.html) · Device Command"]
    unsafe fn cmd_set_cull_mode_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        cull_mode: crate::vk1_0::CullModeFlags,
    ) -> () {
        let function = device_commands(self)
            .cmd_set_cull_mode_ext
            .as_ref()
            .expect("`cmd_set_cull_mode_ext` not available");
        let _val = function(command_buffer, cull_mode);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetFrontFaceEXT.html) · Device Command"]
    unsafe fn cmd_set_front_face_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        front_face: crate::vk1_0::FrontFace,
    ) -> () {
        let function = device_commands(self)
            .cmd_set_front_face_ext
            .as_ref()
            .expect("`cmd_set_front_face_ext` not available");
        let _val = function(command_buffer, front_face);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetPrimitiveTopologyEXT.html) · Device Command"]
    unsafe fn cmd_set_primitive_topology_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        primitive_topology: crate::vk1_0::PrimitiveTopology,
    ) -> () {
        let function = device_commands(self)
            .cmd_set_primitive_topology_ext
            .as_ref()
            .expect("`cmd_set_primitive_topology_ext` not available");
        let _val = function(command_buffer, primitive_topology);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetViewportWithCountEXT.html) · Device Command"]
    unsafe fn cmd_set_viewport_with_count_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        viewports: &[crate::vk1_0::ViewportBuilder],
    ) -> () {
        let function = device_commands(self)
            .cmd_set_viewport_with_count_ext
            .as_ref()
            .expect("`cmd_set_viewport_with_count_ext` not available");
        let viewport_count = viewports.len() as _;
        let _val = function(command_buffer, viewport_count, viewports.as_ptr() as _);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetScissorWithCountEXT.html) · Device Command"]
    unsafe fn cmd_set_scissor_with_count_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        scissors: &[crate::vk1_0::Rect2DBuilder],
    ) -> () {
        let function = device_commands(self)
            .cmd_set_scissor_with_count_ext
            .as_ref()
            .expect("`cmd_set_scissor_with_count_ext` not available");
        let scissor_count = scissors.len() as _;
        let _val = function(command_buffer, scissor_count, scissors.as_ptr() as _);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBindVertexBuffers2EXT.html) · Device Command"]
    unsafe fn cmd_bind_vertex_buffers2_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        first_binding: u32,
        buffers: &[crate::vk1_0::Buffer],
        offsets: &[crate::vk1_0::DeviceSize],
        sizes: &[crate::vk1_0::DeviceSize],
        strides: &[crate::vk1_0::DeviceSize],
    ) -> () {
        let function = device_commands(self)
            .cmd_bind_vertex_buffers2_ext
            .as_ref()
            .expect("`cmd_bind_vertex_buffers2_ext` not available");
        let binding_count = buffers
            .len()
            .min(offsets.len())
            .min(sizes.len())
            .min(strides.len()) as _;
        let _val = function(
            command_buffer,
            first_binding,
            binding_count,
            buffers.as_ptr() as _,
            offsets.as_ptr() as _,
            sizes.as_ptr() as _,
            strides.as_ptr() as _,
        );
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDepthTestEnableEXT.html) · Device Command"]
    unsafe fn cmd_set_depth_test_enable_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        depth_test_enable: bool,
    ) -> () {
        let function = device_commands(self)
            .cmd_set_depth_test_enable_ext
            .as_ref()
            .expect("`cmd_set_depth_test_enable_ext` not available");
        let _val = function(command_buffer, depth_test_enable as _);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDepthWriteEnableEXT.html) · Device Command"]
    unsafe fn cmd_set_depth_write_enable_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        depth_write_enable: bool,
    ) -> () {
        let function = device_commands(self)
            .cmd_set_depth_write_enable_ext
            .as_ref()
            .expect("`cmd_set_depth_write_enable_ext` not available");
        let _val = function(command_buffer, depth_write_enable as _);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDepthCompareOpEXT.html) · Device Command"]
    unsafe fn cmd_set_depth_compare_op_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        depth_compare_op: crate::vk1_0::CompareOp,
    ) -> () {
        let function = device_commands(self)
            .cmd_set_depth_compare_op_ext
            .as_ref()
            .expect("`cmd_set_depth_compare_op_ext` not available");
        let _val = function(command_buffer, depth_compare_op);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDepthBoundsTestEnableEXT.html) · Device Command"]
    unsafe fn cmd_set_depth_bounds_test_enable_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        depth_bounds_test_enable: bool,
    ) -> () {
        let function = device_commands(self)
            .cmd_set_depth_bounds_test_enable_ext
            .as_ref()
            .expect("`cmd_set_depth_bounds_test_enable_ext` not available");
        let _val = function(command_buffer, depth_bounds_test_enable as _);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetStencilTestEnableEXT.html) · Device Command"]
    unsafe fn cmd_set_stencil_test_enable_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        stencil_test_enable: bool,
    ) -> () {
        let function = device_commands(self)
            .cmd_set_stencil_test_enable_ext
            .as_ref()
            .expect("`cmd_set_stencil_test_enable_ext` not available");
        let _val = function(command_buffer, stencil_test_enable as _);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetStencilOpEXT.html) · Device Command"]
    unsafe fn cmd_set_stencil_op_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        face_mask: crate::vk1_0::StencilFaceFlags,
        fail_op: crate::vk1_0::StencilOp,
        pass_op: crate::vk1_0::StencilOp,
        depth_fail_op: crate::vk1_0::StencilOp,
        compare_op: crate::vk1_0::CompareOp,
    ) -> () {
        let function = device_commands(self)
            .cmd_set_stencil_op_ext
            .as_ref()
            .expect("`cmd_set_stencil_op_ext` not available");
        let _val = function(
            command_buffer,
            face_mask,
            fail_op,
            pass_op,
            depth_fail_op,
            compare_op,
        );
        ()
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceExtendedDynamicStateFeaturesEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceExtendedDynamicStateFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub extended_dynamic_state: crate::vk1_0::Bool32,
}
impl PhysicalDeviceExtendedDynamicStateFeaturesEXT {
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
    pub fn builder<'a>(self) -> PhysicalDeviceExtendedDynamicStateFeaturesEXTBuilder<'a> {
        PhysicalDeviceExtendedDynamicStateFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceExtendedDynamicStateFeaturesEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceExtendedDynamicStateFeaturesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "extended_dynamic_state",
                &(self.extended_dynamic_state != 0),
            )
            .finish()
    }
}
impl Default for PhysicalDeviceExtendedDynamicStateFeaturesEXT {
    fn default() -> PhysicalDeviceExtendedDynamicStateFeaturesEXT {
        PhysicalDeviceExtendedDynamicStateFeaturesEXT {
            s_type:
                crate::vk1_0::StructureType::PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            extended_dynamic_state: Default::default(),
        }
    }
}
impl crate::ExtendableBy<PhysicalDeviceExtendedDynamicStateFeaturesEXT>
    for crate::vk1_1::PhysicalDeviceFeatures2
{
}
impl crate::ExtendableBy<PhysicalDeviceExtendedDynamicStateFeaturesEXT>
    for crate::vk1_0::DeviceCreateInfo
{
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceExtendedDynamicStateFeaturesEXT.html) · Builder of [`PhysicalDeviceExtendedDynamicStateFeaturesEXT`](struct.PhysicalDeviceExtendedDynamicStateFeaturesEXT.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceExtendedDynamicStateFeaturesEXTBuilder<'a>(
    PhysicalDeviceExtendedDynamicStateFeaturesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceExtendedDynamicStateFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceExtendedDynamicStateFeaturesEXTBuilder<'a> {
        PhysicalDeviceExtendedDynamicStateFeaturesEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn extended_dynamic_state(mut self, extended_dynamic_state: bool) -> Self {
        self.0.extended_dynamic_state = extended_dynamic_state as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceExtendedDynamicStateFeaturesEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceExtendedDynamicStateFeaturesEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceExtendedDynamicStateFeaturesEXTBuilder<'a> {
    type Target = PhysicalDeviceExtendedDynamicStateFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceExtendedDynamicStateFeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
