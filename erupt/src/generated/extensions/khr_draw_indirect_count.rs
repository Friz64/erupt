#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_DRAW_INDIRECT_COUNT_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_DRAW_INDIRECT_COUNT_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_draw_indirect_count");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_DRAW_INDIRECT_COUNT_KHR: *const std::os::raw::c_char =
    crate::cstr!("vkCmdDrawIndirectCountKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_DRAW_INDEXED_INDIRECT_COUNT_KHR: *const std::os::raw::c_char =
    crate::cstr!("vkCmdDrawIndexedIndirectCountKHR");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawIndirectCountKHR.html) · Alias"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawIndirectCountKHR = crate::vk1_2::PFN_vkCmdDrawIndirectCount;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawIndexedIndirectCountKHR.html) · Alias"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawIndexedIndirectCountKHR = crate::vk1_2::PFN_vkCmdDrawIndexedIndirectCount;
#[doc = "Provided by [`extensions::khr_draw_indirect_count`](extensions/khr_draw_indirect_count/index.html)"]
impl crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawIndirectCountKHR.html) · Function"]
    pub unsafe fn cmd_draw_indirect_count_khr(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        buffer: crate::vk1_0::Buffer,
        offset: crate::vk1_0::DeviceSize,
        count_buffer: crate::vk1_0::Buffer,
        count_buffer_offset: crate::vk1_0::DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) -> () {
        let _function = self
            .cmd_draw_indirect_count_khr
            .expect("`cmd_draw_indirect_count_khr` is not loaded");
        let _return = _function(
            command_buffer as _,
            buffer as _,
            offset as _,
            count_buffer as _,
            count_buffer_offset as _,
            max_draw_count as _,
            stride as _,
        );
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawIndexedIndirectCountKHR.html) · Function"]
    pub unsafe fn cmd_draw_indexed_indirect_count_khr(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        buffer: crate::vk1_0::Buffer,
        offset: crate::vk1_0::DeviceSize,
        count_buffer: crate::vk1_0::Buffer,
        count_buffer_offset: crate::vk1_0::DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) -> () {
        let _function = self
            .cmd_draw_indexed_indirect_count_khr
            .expect("`cmd_draw_indexed_indirect_count_khr` is not loaded");
        let _return = _function(
            command_buffer as _,
            buffer as _,
            offset as _,
            count_buffer as _,
            count_buffer_offset as _,
            max_draw_count as _,
            stride as _,
        );
        ()
    }
}
