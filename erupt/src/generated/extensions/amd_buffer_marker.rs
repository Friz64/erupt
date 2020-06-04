# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_AMD_buffer_marker.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const AMD_BUFFER_MARKER_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const AMD_BUFFER_MARKER_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_AMD_buffer_marker");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdWriteBufferMarkerAMD.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdWriteBufferMarkerAMD = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    pipeline_stage: crate::vk1_0::PipelineStageFlagBits,
    dst_buffer: crate::vk1_0::Buffer,
    dst_offset: crate::vk1_0::DeviceSize,
    marker: u32,
) -> std::ffi::c_void;
#[doc = "Provides Device Commands for [`AmdBufferMarkerDeviceLoaderExt`](trait.AmdBufferMarkerDeviceLoaderExt.html)"]
pub struct AmdBufferMarkerDeviceCommands {
    pub cmd_write_buffer_marker_amd: Option<PFN_vkCmdWriteBufferMarkerAMD>,
}
impl AmdBufferMarkerDeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<AmdBufferMarkerDeviceCommands> {
        unsafe {
            let mut success = false;
            let table = AmdBufferMarkerDeviceCommands {
                cmd_write_buffer_marker_amd: std::mem::transmute({
                    let symbol = loader.symbol("vkCmdWriteBufferMarkerAMD");
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
fn device_commands(loader: &crate::DeviceLoader) -> &AmdBufferMarkerDeviceCommands {
    loader
        .amd_buffer_marker
        .as_ref()
        .expect("`amd_buffer_marker` not loaded")
}
#[doc = "Provides high level command wrappers for [`AmdBufferMarkerDeviceCommands`](struct.AmdBufferMarkerDeviceCommands.html)"]
pub trait AmdBufferMarkerDeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdWriteBufferMarkerAMD.html) · Device Command"]
    unsafe fn cmd_write_buffer_marker_amd(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        pipeline_stage: crate::vk1_0::PipelineStageFlagBits,
        dst_buffer: crate::vk1_0::Buffer,
        dst_offset: crate::vk1_0::DeviceSize,
        marker: u32,
    ) -> ();
}
impl AmdBufferMarkerDeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdWriteBufferMarkerAMD.html) · Device Command"]
    unsafe fn cmd_write_buffer_marker_amd(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        pipeline_stage: crate::vk1_0::PipelineStageFlagBits,
        dst_buffer: crate::vk1_0::Buffer,
        dst_offset: crate::vk1_0::DeviceSize,
        marker: u32,
    ) -> () {
        let function = device_commands(self)
            .cmd_write_buffer_marker_amd
            .as_ref()
            .expect("`cmd_write_buffer_marker_amd` not available");
        let _val = function(
            command_buffer,
            pipeline_stage,
            dst_buffer,
            dst_offset,
            marker,
        );
        ()
    }
}
