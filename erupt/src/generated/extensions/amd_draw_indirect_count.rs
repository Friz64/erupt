# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_AMD_draw_indirect_count.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const AMD_DRAW_INDIRECT_COUNT_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const AMD_DRAW_INDIRECT_COUNT_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_AMD_draw_indirect_count");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawIndirectCountAMD.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawIndirectCountAMD = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    buffer: crate::vk1_0::Buffer,
    offset: crate::vk1_0::DeviceSize,
    count_buffer: crate::vk1_0::Buffer,
    count_buffer_offset: crate::vk1_0::DeviceSize,
    max_draw_count: u32,
    stride: u32,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawIndexedIndirectCountAMD.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawIndexedIndirectCountAMD = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    buffer: crate::vk1_0::Buffer,
    offset: crate::vk1_0::DeviceSize,
    count_buffer: crate::vk1_0::Buffer,
    count_buffer_offset: crate::vk1_0::DeviceSize,
    max_draw_count: u32,
    stride: u32,
) -> std::ffi::c_void;
#[doc = "Provides Device Commands for [`AmdDrawIndirectCountDeviceLoaderExt`](trait.AmdDrawIndirectCountDeviceLoaderExt.html)"]
pub struct AmdDrawIndirectCountDeviceCommands {
    pub cmd_draw_indirect_count_amd: Option<PFN_vkCmdDrawIndirectCountAMD>,
    pub cmd_draw_indexed_indirect_count_amd: Option<PFN_vkCmdDrawIndexedIndirectCountAMD>,
}
impl AmdDrawIndirectCountDeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<AmdDrawIndirectCountDeviceCommands> {
        unsafe {
            let mut success = false;
            let table = AmdDrawIndirectCountDeviceCommands {
                cmd_draw_indirect_count_amd: std::mem::transmute({
                    let symbol = loader.symbol("vkCmdDrawIndirectCountAMD");
                    success |= symbol.is_some();
                    symbol
                }),
                cmd_draw_indexed_indirect_count_amd: std::mem::transmute({
                    let symbol = loader.symbol("vkCmdDrawIndexedIndirectCountAMD");
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
fn device_commands(loader: &crate::DeviceLoader) -> &AmdDrawIndirectCountDeviceCommands {
    loader
        .amd_draw_indirect_count
        .as_ref()
        .expect("`amd_draw_indirect_count` not loaded")
}
#[doc = "Provides high level command wrappers for [`AmdDrawIndirectCountDeviceCommands`](struct.AmdDrawIndirectCountDeviceCommands.html)"]
pub trait AmdDrawIndirectCountDeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawIndirectCountAMD.html) · Device Command"]
    unsafe fn cmd_draw_indirect_count_amd(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        buffer: crate::vk1_0::Buffer,
        offset: crate::vk1_0::DeviceSize,
        count_buffer: crate::vk1_0::Buffer,
        count_buffer_offset: crate::vk1_0::DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawIndexedIndirectCountAMD.html) · Device Command"]
    unsafe fn cmd_draw_indexed_indirect_count_amd(
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
impl AmdDrawIndirectCountDeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawIndirectCountAMD.html) · Device Command"]
    unsafe fn cmd_draw_indirect_count_amd(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        buffer: crate::vk1_0::Buffer,
        offset: crate::vk1_0::DeviceSize,
        count_buffer: crate::vk1_0::Buffer,
        count_buffer_offset: crate::vk1_0::DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) -> () {
        let function = device_commands(self)
            .cmd_draw_indirect_count_amd
            .as_ref()
            .expect("`cmd_draw_indirect_count_amd` not available");
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
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawIndexedIndirectCountAMD.html) · Device Command"]
    unsafe fn cmd_draw_indexed_indirect_count_amd(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        buffer: crate::vk1_0::Buffer,
        offset: crate::vk1_0::DeviceSize,
        count_buffer: crate::vk1_0::Buffer,
        count_buffer_offset: crate::vk1_0::DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) -> () {
        let function = device_commands(self)
            .cmd_draw_indexed_indirect_count_amd
            .as_ref()
            .expect("`cmd_draw_indexed_indirect_count_amd` not available");
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
