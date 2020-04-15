# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_KHR_draw_indirect_count.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_DRAW_INDIRECT_COUNT_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_DRAW_INDIRECT_COUNT_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_draw_indirect_count");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawIndirectCountKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawIndirectCountKHR = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    buffer: crate::vk1_0::Buffer,
    offset: crate::vk1_0::DeviceSize,
    count_buffer: crate::vk1_0::Buffer,
    count_buffer_offset: crate::vk1_0::DeviceSize,
    max_draw_count: u32,
    stride: u32,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawIndexedIndirectCountKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawIndexedIndirectCountKHR = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    buffer: crate::vk1_0::Buffer,
    offset: crate::vk1_0::DeviceSize,
    count_buffer: crate::vk1_0::Buffer,
    count_buffer_offset: crate::vk1_0::DeviceSize,
    max_draw_count: u32,
    stride: u32,
) -> std::ffi::c_void;
#[doc = "Provides Device Commands for [`KhrDrawIndirectCountDeviceLoaderExt`](trait.KhrDrawIndirectCountDeviceLoaderExt.html)"]
pub struct KhrDrawIndirectCountDeviceCommands {
    pub cmd_draw_indirect_count_khr: PFN_vkCmdDrawIndirectCountKHR,
    pub cmd_draw_indexed_indirect_count_khr: PFN_vkCmdDrawIndexedIndirectCountKHR,
}
impl KhrDrawIndirectCountDeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<KhrDrawIndirectCountDeviceCommands> {
        unsafe {
            Some(KhrDrawIndirectCountDeviceCommands {
                cmd_draw_indirect_count_khr: std::mem::transmute(
                    loader.symbol("vkCmdDrawIndirectCountKHR")?,
                ),
                cmd_draw_indexed_indirect_count_khr: std::mem::transmute(
                    loader.symbol("vkCmdDrawIndexedIndirectCountKHR")?,
                ),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`KhrDrawIndirectCountDeviceCommands`](struct.KhrDrawIndirectCountDeviceCommands.html)"]
pub trait KhrDrawIndirectCountDeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawIndirectCountKHR.html) · Device Command"]
    unsafe fn cmd_draw_indirect_count_khr(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        buffer: crate::vk1_0::Buffer,
        offset: crate::vk1_0::DeviceSize,
        count_buffer: crate::vk1_0::Buffer,
        count_buffer_offset: crate::vk1_0::DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawIndexedIndirectCountKHR.html) · Device Command"]
    unsafe fn cmd_draw_indexed_indirect_count_khr(
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
impl KhrDrawIndirectCountDeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawIndirectCountKHR.html) · Device Command"]
    unsafe fn cmd_draw_indirect_count_khr(
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
            .khr_draw_indirect_count
            .as_ref()
            .expect("`khr_draw_indirect_count` not loaded")
            .cmd_draw_indirect_count_khr;
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
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawIndexedIndirectCountKHR.html) · Device Command"]
    unsafe fn cmd_draw_indexed_indirect_count_khr(
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
            .khr_draw_indirect_count
            .as_ref()
            .expect("`khr_draw_indirect_count` not loaded")
            .cmd_draw_indexed_indirect_count_khr;
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
