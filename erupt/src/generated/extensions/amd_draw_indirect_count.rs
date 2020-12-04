#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_AMD_DRAW_INDIRECT_COUNT_SPEC_VERSION")]
pub const AMD_DRAW_INDIRECT_COUNT_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_AMD_DRAW_INDIRECT_COUNT_EXTENSION_NAME")]
pub const AMD_DRAW_INDIRECT_COUNT_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_AMD_draw_indirect_count");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "FN_CMD_DRAW_INDIRECT_COUNT_AMD")]
pub const FN_CMD_DRAW_INDIRECT_COUNT_AMD: *const std::os::raw::c_char =
    crate::cstr!("vkCmdDrawIndirectCountAMD");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "FN_CMD_DRAW_INDEXED_INDIRECT_COUNT_AMD")]
pub const FN_CMD_DRAW_INDEXED_INDIRECT_COUNT_AMD: *const std::os::raw::c_char =
    crate::cstr!("vkCmdDrawIndexedIndirectCountAMD");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawIndirectCountAMD.html) · Alias"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawIndirectCountAMD = crate::vk1_2::PFN_vkCmdDrawIndirectCount;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawIndexedIndirectCountAMD.html) · Alias"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawIndexedIndirectCountAMD = crate::vk1_2::PFN_vkCmdDrawIndexedIndirectCount;
#[doc = "Provided by [`crate::extensions::amd_draw_indirect_count`]"]
impl crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawIndirectCountAMD.html) · Function"]
    #[doc(alias = "vkCmdDrawIndirectCountAMD")]
    pub unsafe fn cmd_draw_indirect_count_amd(
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
            .cmd_draw_indirect_count_amd
            .expect("`cmd_draw_indirect_count_amd` is not loaded");
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
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawIndexedIndirectCountAMD.html) · Function"]
    #[doc(alias = "vkCmdDrawIndexedIndirectCountAMD")]
    pub unsafe fn cmd_draw_indexed_indirect_count_amd(
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
            .cmd_draw_indexed_indirect_count_amd
            .expect("`cmd_draw_indexed_indirect_count_amd` is not loaded");
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
