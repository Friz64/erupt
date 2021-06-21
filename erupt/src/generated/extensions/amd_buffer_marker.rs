#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_AMD_BUFFER_MARKER_SPEC_VERSION")]
pub const AMD_BUFFER_MARKER_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_AMD_BUFFER_MARKER_EXTENSION_NAME")]
pub const AMD_BUFFER_MARKER_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_AMD_buffer_marker");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_WRITE_BUFFER_MARKER_AMD: *const std::os::raw::c_char = crate::cstr!("vkCmdWriteBufferMarkerAMD");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdWriteBufferMarkerAMD.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdWriteBufferMarkerAMD = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, pipeline_stage: crate::vk1_0::PipelineStageFlagBits, dst_buffer: crate::vk1_0::Buffer, dst_offset: crate::vk1_0::DeviceSize, marker: u32) -> ();
#[doc = "Provided by [`crate::extensions::amd_buffer_marker`]"]
impl crate::DeviceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdWriteBufferMarkerAMD.html) · Function"]
    #[doc(alias = "vkCmdWriteBufferMarkerAMD")]
    pub unsafe fn cmd_write_buffer_marker_amd(&self, command_buffer: crate::vk1_0::CommandBuffer, pipeline_stage: crate::vk1_0::PipelineStageFlagBits, dst_buffer: crate::vk1_0::Buffer, dst_offset: crate::vk1_0::DeviceSize, marker: u32) -> () {
        let _function = self.cmd_write_buffer_marker_amd.expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(command_buffer as _, pipeline_stage as _, dst_buffer as _, dst_offset as _, marker as _);
        ()
    }
}
