#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const MAX_DRIVER_NAME_SIZE: u32 = 256;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const MAX_DRIVER_INFO_SIZE: u32 = 256;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawIndirectCount.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawIndirectCount = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    buffer: crate::vk1_0::Buffer,
    offset: crate::vk1_0::DeviceSize,
    count_buffer: crate::vk1_0::Buffer,
    count_buffer_offset: crate::vk1_0::DeviceSize,
    max_draw_count: u32,
    stride: u32,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawIndexedIndirectCount.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawIndexedIndirectCount = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    buffer: crate::vk1_0::Buffer,
    offset: crate::vk1_0::DeviceSize,
    count_buffer: crate::vk1_0::Buffer,
    count_buffer_offset: crate::vk1_0::DeviceSize,
    max_draw_count: u32,
    stride: u32,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateRenderPass2.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateRenderPass2 = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_create_info: *const crate::vk1_2::RenderPassCreateInfo2,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
    p_render_pass: *mut crate::vk1_0::RenderPass,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBeginRenderPass2.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBeginRenderPass2 = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    p_render_pass_begin: *const crate::vk1_0::RenderPassBeginInfo,
    p_subpass_begin_info: *const crate::vk1_2::SubpassBeginInfo,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdNextSubpass2.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdNextSubpass2 = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    p_subpass_begin_info: *const crate::vk1_2::SubpassBeginInfo,
    p_subpass_end_info: *const crate::vk1_2::SubpassEndInfo,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdEndRenderPass2.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdEndRenderPass2 = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    p_subpass_end_info: *const crate::vk1_2::SubpassEndInfo,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkResetQueryPool.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkResetQueryPool = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    query_pool: crate::vk1_0::QueryPool,
    first_query: u32,
    query_count: u32,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSemaphoreCounterValue.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetSemaphoreCounterValue = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    semaphore: crate::vk1_0::Semaphore,
    p_value: *mut u64,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkWaitSemaphores.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkWaitSemaphores = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_wait_info: *const crate::vk1_2::SemaphoreWaitInfo,
    timeout: u64,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSignalSemaphore.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkSignalSemaphore = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_signal_info: *const crate::vk1_2::SemaphoreSignalInfo,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetBufferDeviceAddress.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetBufferDeviceAddress = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_info: *const crate::vk1_2::BufferDeviceAddressInfo,
) -> crate::vk1_2::DeviceAddress;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetBufferOpaqueCaptureAddress.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetBufferOpaqueCaptureAddress = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_info: *const crate::vk1_2::BufferDeviceAddressInfo,
) -> u64;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceMemoryOpaqueCaptureAddress.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceMemoryOpaqueCaptureAddress = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_info: *const crate::vk1_2::DeviceMemoryOpaqueCaptureAddressInfo,
) -> u64;
#[doc = "Provides Device Commands for [`Vk12DeviceLoaderExt`](trait.Vk12DeviceLoaderExt.html)"]
pub struct Vk12DeviceCommands {
    pub cmd_draw_indirect_count: PFN_vkCmdDrawIndirectCount,
    pub cmd_draw_indexed_indirect_count: PFN_vkCmdDrawIndexedIndirectCount,
    pub create_render_pass2: PFN_vkCreateRenderPass2,
    pub cmd_begin_render_pass2: PFN_vkCmdBeginRenderPass2,
    pub cmd_next_subpass2: PFN_vkCmdNextSubpass2,
    pub cmd_end_render_pass2: PFN_vkCmdEndRenderPass2,
    pub reset_query_pool: PFN_vkResetQueryPool,
    pub get_semaphore_counter_value: PFN_vkGetSemaphoreCounterValue,
    pub wait_semaphores: PFN_vkWaitSemaphores,
    pub signal_semaphore: PFN_vkSignalSemaphore,
    pub get_buffer_device_address: PFN_vkGetBufferDeviceAddress,
    pub get_buffer_opaque_capture_address: PFN_vkGetBufferOpaqueCaptureAddress,
    pub get_device_memory_opaque_capture_address: PFN_vkGetDeviceMemoryOpaqueCaptureAddress,
}
impl Vk12DeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<Vk12DeviceCommands> {
        unsafe {
            Some(Vk12DeviceCommands {
                cmd_draw_indirect_count: std::mem::transmute(
                    loader.symbol("vkCmdDrawIndirectCount")?,
                ),
                cmd_draw_indexed_indirect_count: std::mem::transmute(
                    loader.symbol("vkCmdDrawIndexedIndirectCount")?,
                ),
                create_render_pass2: std::mem::transmute(loader.symbol("vkCreateRenderPass2")?),
                cmd_begin_render_pass2: std::mem::transmute(
                    loader.symbol("vkCmdBeginRenderPass2")?,
                ),
                cmd_next_subpass2: std::mem::transmute(loader.symbol("vkCmdNextSubpass2")?),
                cmd_end_render_pass2: std::mem::transmute(loader.symbol("vkCmdEndRenderPass2")?),
                reset_query_pool: std::mem::transmute(loader.symbol("vkResetQueryPool")?),
                get_semaphore_counter_value: std::mem::transmute(
                    loader.symbol("vkGetSemaphoreCounterValue")?,
                ),
                wait_semaphores: std::mem::transmute(loader.symbol("vkWaitSemaphores")?),
                signal_semaphore: std::mem::transmute(loader.symbol("vkSignalSemaphore")?),
                get_buffer_device_address: std::mem::transmute(
                    loader.symbol("vkGetBufferDeviceAddress")?,
                ),
                get_buffer_opaque_capture_address: std::mem::transmute(
                    loader.symbol("vkGetBufferOpaqueCaptureAddress")?,
                ),
                get_device_memory_opaque_capture_address: std::mem::transmute(
                    loader.symbol("vkGetDeviceMemoryOpaqueCaptureAddress")?,
                ),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`Vk12DeviceCommands`](struct.Vk12DeviceCommands.html)"]
pub trait Vk12DeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawIndirectCount.html) · Device Command"]
    unsafe fn cmd_draw_indirect_count(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        buffer: crate::vk1_0::Buffer,
        offset: crate::vk1_0::DeviceSize,
        count_buffer: crate::vk1_0::Buffer,
        count_buffer_offset: crate::vk1_0::DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawIndexedIndirectCount.html) · Device Command"]
    unsafe fn cmd_draw_indexed_indirect_count(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        buffer: crate::vk1_0::Buffer,
        offset: crate::vk1_0::DeviceSize,
        count_buffer: crate::vk1_0::Buffer,
        count_buffer_offset: crate::vk1_0::DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateRenderPass2.html) · Device Command"]
    unsafe fn create_render_pass2(
        &self,
        create_info: &crate::vk1_2::RenderPassCreateInfo2,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        render_pass: Option<crate::vk1_0::RenderPass>,
    ) -> crate::utils::VulkanResult<crate::vk1_0::RenderPass>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBeginRenderPass2.html) · Device Command"]
    unsafe fn cmd_begin_render_pass2(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        render_pass_begin: &crate::vk1_0::RenderPassBeginInfo,
        subpass_begin_info: &crate::vk1_2::SubpassBeginInfo,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdNextSubpass2.html) · Device Command"]
    unsafe fn cmd_next_subpass2(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        subpass_begin_info: &crate::vk1_2::SubpassBeginInfo,
        subpass_end_info: &crate::vk1_2::SubpassEndInfo,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdEndRenderPass2.html) · Device Command"]
    unsafe fn cmd_end_render_pass2(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        subpass_end_info: &crate::vk1_2::SubpassEndInfo,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkResetQueryPool.html) · Device Command"]
    unsafe fn reset_query_pool(
        &self,
        query_pool: crate::vk1_0::QueryPool,
        first_query: u32,
        query_count: u32,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSemaphoreCounterValue.html) · Device Command"]
    unsafe fn get_semaphore_counter_value(
        &self,
        semaphore: crate::vk1_0::Semaphore,
        value: Option<u64>,
    ) -> crate::utils::VulkanResult<u64>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkWaitSemaphores.html) · Device Command"]
    unsafe fn wait_semaphores(
        &self,
        wait_info: &crate::vk1_2::SemaphoreWaitInfo,
        timeout: u64,
    ) -> crate::utils::VulkanResult<()>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSignalSemaphore.html) · Device Command"]
    unsafe fn signal_semaphore(
        &self,
        signal_info: &crate::vk1_2::SemaphoreSignalInfo,
    ) -> crate::utils::VulkanResult<()>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetBufferDeviceAddress.html) · Device Command"]
    unsafe fn get_buffer_device_address(
        &self,
        info: &crate::vk1_2::BufferDeviceAddressInfo,
    ) -> crate::vk1_2::DeviceAddress;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetBufferOpaqueCaptureAddress.html) · Device Command"]
    unsafe fn get_buffer_opaque_capture_address(
        &self,
        info: &crate::vk1_2::BufferDeviceAddressInfo,
    ) -> u64;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceMemoryOpaqueCaptureAddress.html) · Device Command"]
    unsafe fn get_device_memory_opaque_capture_address(
        &self,
        info: &crate::vk1_2::DeviceMemoryOpaqueCaptureAddressInfo,
    ) -> u64;
}
impl Vk12DeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawIndirectCount.html) · Device Command"]
    unsafe fn cmd_draw_indirect_count(
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
            .vk1_2
            .as_ref()
            .expect("`vk1_2` not loaded")
            .cmd_draw_indirect_count;
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
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawIndexedIndirectCount.html) · Device Command"]
    unsafe fn cmd_draw_indexed_indirect_count(
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
            .vk1_2
            .as_ref()
            .expect("`vk1_2` not loaded")
            .cmd_draw_indexed_indirect_count;
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
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateRenderPass2.html) · Device Command"]
    unsafe fn create_render_pass2(
        &self,
        create_info: &crate::vk1_2::RenderPassCreateInfo2,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        render_pass: Option<crate::vk1_0::RenderPass>,
    ) -> crate::utils::VulkanResult<crate::vk1_0::RenderPass> {
        let function = self
            .vk1_2
            .as_ref()
            .expect("`vk1_2` not loaded")
            .create_render_pass2;
        let mut render_pass = render_pass.unwrap_or_else(|| Default::default());
        let _val = function(
            self.handle,
            create_info,
            if let Some(allocator) = allocator {
                allocator
            } else {
                std::ptr::null()
            },
            &mut render_pass,
        );
        crate::utils::VulkanResult::new(_val, render_pass)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBeginRenderPass2.html) · Device Command"]
    unsafe fn cmd_begin_render_pass2(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        render_pass_begin: &crate::vk1_0::RenderPassBeginInfo,
        subpass_begin_info: &crate::vk1_2::SubpassBeginInfo,
    ) -> () {
        let function = self
            .vk1_2
            .as_ref()
            .expect("`vk1_2` not loaded")
            .cmd_begin_render_pass2;
        let _val = function(command_buffer, render_pass_begin, subpass_begin_info);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdNextSubpass2.html) · Device Command"]
    unsafe fn cmd_next_subpass2(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        subpass_begin_info: &crate::vk1_2::SubpassBeginInfo,
        subpass_end_info: &crate::vk1_2::SubpassEndInfo,
    ) -> () {
        let function = self
            .vk1_2
            .as_ref()
            .expect("`vk1_2` not loaded")
            .cmd_next_subpass2;
        let _val = function(command_buffer, subpass_begin_info, subpass_end_info);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdEndRenderPass2.html) · Device Command"]
    unsafe fn cmd_end_render_pass2(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        subpass_end_info: &crate::vk1_2::SubpassEndInfo,
    ) -> () {
        let function = self
            .vk1_2
            .as_ref()
            .expect("`vk1_2` not loaded")
            .cmd_end_render_pass2;
        let _val = function(command_buffer, subpass_end_info);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkResetQueryPool.html) · Device Command"]
    unsafe fn reset_query_pool(
        &self,
        query_pool: crate::vk1_0::QueryPool,
        first_query: u32,
        query_count: u32,
    ) -> () {
        let function = self
            .vk1_2
            .as_ref()
            .expect("`vk1_2` not loaded")
            .reset_query_pool;
        let _val = function(self.handle, query_pool, first_query, query_count);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSemaphoreCounterValue.html) · Device Command"]
    unsafe fn get_semaphore_counter_value(
        &self,
        semaphore: crate::vk1_0::Semaphore,
        value: Option<u64>,
    ) -> crate::utils::VulkanResult<u64> {
        let function = self
            .vk1_2
            .as_ref()
            .expect("`vk1_2` not loaded")
            .get_semaphore_counter_value;
        let mut value = value.unwrap_or_else(|| Default::default());
        let _val = function(self.handle, semaphore, &mut value);
        crate::utils::VulkanResult::new(_val, value)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkWaitSemaphores.html) · Device Command"]
    unsafe fn wait_semaphores(
        &self,
        wait_info: &crate::vk1_2::SemaphoreWaitInfo,
        timeout: u64,
    ) -> crate::utils::VulkanResult<()> {
        let function = self
            .vk1_2
            .as_ref()
            .expect("`vk1_2` not loaded")
            .wait_semaphores;
        let _val = function(self.handle, wait_info, timeout);
        crate::utils::VulkanResult::new(_val, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSignalSemaphore.html) · Device Command"]
    unsafe fn signal_semaphore(
        &self,
        signal_info: &crate::vk1_2::SemaphoreSignalInfo,
    ) -> crate::utils::VulkanResult<()> {
        let function = self
            .vk1_2
            .as_ref()
            .expect("`vk1_2` not loaded")
            .signal_semaphore;
        let _val = function(self.handle, signal_info);
        crate::utils::VulkanResult::new(_val, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetBufferDeviceAddress.html) · Device Command"]
    unsafe fn get_buffer_device_address(
        &self,
        info: &crate::vk1_2::BufferDeviceAddressInfo,
    ) -> crate::vk1_2::DeviceAddress {
        let function = self
            .vk1_2
            .as_ref()
            .expect("`vk1_2` not loaded")
            .get_buffer_device_address;
        let _val = function(self.handle, info);
        _val
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetBufferOpaqueCaptureAddress.html) · Device Command"]
    unsafe fn get_buffer_opaque_capture_address(
        &self,
        info: &crate::vk1_2::BufferDeviceAddressInfo,
    ) -> u64 {
        let function = self
            .vk1_2
            .as_ref()
            .expect("`vk1_2` not loaded")
            .get_buffer_opaque_capture_address;
        let _val = function(self.handle, info);
        _val
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceMemoryOpaqueCaptureAddress.html) · Device Command"]
    unsafe fn get_device_memory_opaque_capture_address(
        &self,
        info: &crate::vk1_2::DeviceMemoryOpaqueCaptureAddressInfo,
    ) -> u64 {
        let function = self
            .vk1_2
            .as_ref()
            .expect("`vk1_2` not loaded")
            .get_device_memory_opaque_capture_address;
        let _val = function(self.handle, info);
        _val
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderPassCreateInfo2.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RenderPassCreateInfo2 {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::vk1_0::RenderPassCreateFlags,
    pub attachment_count: u32,
    pub p_attachments: *const crate::vk1_2::AttachmentDescription2,
    pub subpass_count: u32,
    pub p_subpasses: *const crate::vk1_2::SubpassDescription2,
    pub dependency_count: u32,
    pub p_dependencies: *const crate::vk1_2::SubpassDependency2,
    pub correlated_view_mask_count: u32,
    pub p_correlated_view_masks: *const u32,
}
impl RenderPassCreateInfo2 {
    #[inline]
    pub fn builder<'a>(self) -> RenderPassCreateInfo2Builder<'a> {
        RenderPassCreateInfo2Builder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for RenderPassCreateInfo2 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("RenderPassCreateInfo2")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("attachment_count", &self.attachment_count)
            .field("p_attachments", &self.p_attachments)
            .field("subpass_count", &self.subpass_count)
            .field("p_subpasses", &self.p_subpasses)
            .field("dependency_count", &self.dependency_count)
            .field("p_dependencies", &self.p_dependencies)
            .field(
                "correlated_view_mask_count",
                &self.correlated_view_mask_count,
            )
            .field("p_correlated_view_masks", &self.p_correlated_view_masks)
            .finish()
    }
}
impl Default for RenderPassCreateInfo2 {
    fn default() -> RenderPassCreateInfo2 {
        RenderPassCreateInfo2 {
            s_type: crate::vk1_0::StructureType::RENDER_PASS_CREATE_INFO_2,
            p_next: std::ptr::null(),
            flags: Default::default(),
            attachment_count: Default::default(),
            p_attachments: std::ptr::null(),
            subpass_count: Default::default(),
            p_subpasses: std::ptr::null(),
            dependency_count: Default::default(),
            p_dependencies: std::ptr::null(),
            correlated_view_mask_count: Default::default(),
            p_correlated_view_masks: std::ptr::null(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`RenderPassCreateInfo2`](struct.RenderPassCreateInfo2.html)"]
#[repr(transparent)]
pub struct RenderPassCreateInfo2Builder<'a>(
    RenderPassCreateInfo2,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> RenderPassCreateInfo2Builder<'a> {
    #[inline]
    pub fn new() -> RenderPassCreateInfo2Builder<'a> {
        RenderPassCreateInfo2Builder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_0::RenderPassCreateFlags) -> Self {
        self.0.flags = flags;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn attachments(
        mut self,
        attachments: &'a [crate::vk1_2::AttachmentDescription2Builder],
    ) -> Self {
        self.0.attachment_count = attachments.len() as _;
        self.0.p_attachments = attachments.as_ptr() as _;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn subpasses(mut self, subpasses: &'a [crate::vk1_2::SubpassDescription2Builder]) -> Self {
        self.0.subpass_count = subpasses.len() as _;
        self.0.p_subpasses = subpasses.as_ptr() as _;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn dependencies(
        mut self,
        dependencies: &'a [crate::vk1_2::SubpassDependency2Builder],
    ) -> Self {
        self.0.dependency_count = dependencies.len() as _;
        self.0.p_dependencies = dependencies.as_ptr() as _;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn correlated_view_masks(mut self, correlated_view_masks: &'a [u32]) -> Self {
        self.0.correlated_view_mask_count = correlated_view_masks.len() as _;
        self.0.p_correlated_view_masks = correlated_view_masks.as_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> RenderPassCreateInfo2 {
        self.0
    }
}
impl<'a> std::fmt::Debug for RenderPassCreateInfo2Builder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for RenderPassCreateInfo2Builder<'a> {
    type Target = RenderPassCreateInfo2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for RenderPassCreateInfo2Builder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAttachmentDescription2.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AttachmentDescription2 {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::vk1_0::AttachmentDescriptionFlags,
    pub format: crate::vk1_0::Format,
    pub samples: crate::vk1_0::SampleCountFlagBits,
    pub load_op: crate::vk1_0::AttachmentLoadOp,
    pub store_op: crate::vk1_0::AttachmentStoreOp,
    pub stencil_load_op: crate::vk1_0::AttachmentLoadOp,
    pub stencil_store_op: crate::vk1_0::AttachmentStoreOp,
    pub initial_layout: crate::vk1_0::ImageLayout,
    pub final_layout: crate::vk1_0::ImageLayout,
}
impl AttachmentDescription2 {
    #[inline]
    pub fn builder<'a>(self) -> AttachmentDescription2Builder<'a> {
        AttachmentDescription2Builder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for AttachmentDescription2 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("AttachmentDescription2")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("format", &self.format)
            .field("samples", &self.samples)
            .field("load_op", &self.load_op)
            .field("store_op", &self.store_op)
            .field("stencil_load_op", &self.stencil_load_op)
            .field("stencil_store_op", &self.stencil_store_op)
            .field("initial_layout", &self.initial_layout)
            .field("final_layout", &self.final_layout)
            .finish()
    }
}
impl Default for AttachmentDescription2 {
    fn default() -> AttachmentDescription2 {
        AttachmentDescription2 {
            s_type: crate::vk1_0::StructureType::ATTACHMENT_DESCRIPTION_2,
            p_next: std::ptr::null(),
            flags: Default::default(),
            format: Default::default(),
            samples: Default::default(),
            load_op: Default::default(),
            store_op: Default::default(),
            stencil_load_op: Default::default(),
            stencil_store_op: Default::default(),
            initial_layout: Default::default(),
            final_layout: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`AttachmentDescription2`](struct.AttachmentDescription2.html)"]
#[repr(transparent)]
pub struct AttachmentDescription2Builder<'a>(
    AttachmentDescription2,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> AttachmentDescription2Builder<'a> {
    #[inline]
    pub fn new() -> AttachmentDescription2Builder<'a> {
        AttachmentDescription2Builder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_0::AttachmentDescriptionFlags) -> Self {
        self.0.flags = flags;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn format(mut self, format: crate::vk1_0::Format) -> Self {
        self.0.format = format;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn samples(mut self, samples: crate::vk1_0::SampleCountFlagBits) -> Self {
        self.0.samples = samples;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn load_op(mut self, load_op: crate::vk1_0::AttachmentLoadOp) -> Self {
        self.0.load_op = load_op;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn store_op(mut self, store_op: crate::vk1_0::AttachmentStoreOp) -> Self {
        self.0.store_op = store_op;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn stencil_load_op(mut self, stencil_load_op: crate::vk1_0::AttachmentLoadOp) -> Self {
        self.0.stencil_load_op = stencil_load_op;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn stencil_store_op(mut self, stencil_store_op: crate::vk1_0::AttachmentStoreOp) -> Self {
        self.0.stencil_store_op = stencil_store_op;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn initial_layout(mut self, initial_layout: crate::vk1_0::ImageLayout) -> Self {
        self.0.initial_layout = initial_layout;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn final_layout(mut self, final_layout: crate::vk1_0::ImageLayout) -> Self {
        self.0.final_layout = final_layout;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> AttachmentDescription2 {
        self.0
    }
}
impl<'a> std::fmt::Debug for AttachmentDescription2Builder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for AttachmentDescription2Builder<'a> {
    type Target = AttachmentDescription2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for AttachmentDescription2Builder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubpassDescription2.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SubpassDescription2 {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::vk1_0::SubpassDescriptionFlags,
    pub pipeline_bind_point: crate::vk1_0::PipelineBindPoint,
    pub view_mask: u32,
    pub input_attachment_count: u32,
    pub p_input_attachments: *const crate::vk1_2::AttachmentReference2,
    pub color_attachment_count: u32,
    pub p_color_attachments: *const crate::vk1_2::AttachmentReference2,
    pub p_resolve_attachments: *const crate::vk1_2::AttachmentReference2,
    pub p_depth_stencil_attachment: *const crate::vk1_2::AttachmentReference2,
    pub preserve_attachment_count: u32,
    pub p_preserve_attachments: *const u32,
}
impl SubpassDescription2 {
    #[inline]
    pub fn builder<'a>(self) -> SubpassDescription2Builder<'a> {
        SubpassDescription2Builder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for SubpassDescription2 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("SubpassDescription2")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("pipeline_bind_point", &self.pipeline_bind_point)
            .field("view_mask", &self.view_mask)
            .field("input_attachment_count", &self.input_attachment_count)
            .field("p_input_attachments", &self.p_input_attachments)
            .field("color_attachment_count", &self.color_attachment_count)
            .field("p_color_attachments", &self.p_color_attachments)
            .field("p_resolve_attachments", &self.p_resolve_attachments)
            .field(
                "p_depth_stencil_attachment",
                &self.p_depth_stencil_attachment,
            )
            .field("preserve_attachment_count", &self.preserve_attachment_count)
            .field("p_preserve_attachments", &self.p_preserve_attachments)
            .finish()
    }
}
impl Default for SubpassDescription2 {
    fn default() -> SubpassDescription2 {
        SubpassDescription2 {
            s_type: crate::vk1_0::StructureType::SUBPASS_DESCRIPTION_2,
            p_next: std::ptr::null(),
            flags: Default::default(),
            pipeline_bind_point: Default::default(),
            view_mask: Default::default(),
            input_attachment_count: Default::default(),
            p_input_attachments: std::ptr::null(),
            color_attachment_count: Default::default(),
            p_color_attachments: std::ptr::null(),
            p_resolve_attachments: std::ptr::null(),
            p_depth_stencil_attachment: std::ptr::null(),
            preserve_attachment_count: Default::default(),
            p_preserve_attachments: std::ptr::null(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`SubpassDescription2`](struct.SubpassDescription2.html)"]
#[repr(transparent)]
pub struct SubpassDescription2Builder<'a>(SubpassDescription2, std::marker::PhantomData<&'a ()>);
impl<'a> SubpassDescription2Builder<'a> {
    #[inline]
    pub fn new() -> SubpassDescription2Builder<'a> {
        SubpassDescription2Builder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_0::SubpassDescriptionFlags) -> Self {
        self.0.flags = flags;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn pipeline_bind_point(
        mut self,
        pipeline_bind_point: crate::vk1_0::PipelineBindPoint,
    ) -> Self {
        self.0.pipeline_bind_point = pipeline_bind_point;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn view_mask(mut self, view_mask: u32) -> Self {
        self.0.view_mask = view_mask;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn input_attachments(
        mut self,
        input_attachments: &'a [crate::vk1_2::AttachmentReference2Builder],
    ) -> Self {
        self.0.input_attachment_count = input_attachments.len() as _;
        self.0.p_input_attachments = input_attachments.as_ptr() as _;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn color_attachments(
        mut self,
        color_attachments: &'a [crate::vk1_2::AttachmentReference2Builder],
    ) -> Self {
        self.0.color_attachment_count = color_attachments.len() as _;
        self.0.p_color_attachments = color_attachments.as_ptr() as _;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn resolve_attachments(
        mut self,
        resolve_attachments: &'a [crate::vk1_2::AttachmentReference2Builder],
    ) -> Self {
        self.0.color_attachment_count = resolve_attachments.len() as _;
        self.0.p_resolve_attachments = resolve_attachments.as_ptr() as _;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn depth_stencil_attachment(
        mut self,
        depth_stencil_attachment: &'a crate::vk1_2::AttachmentReference2,
    ) -> Self {
        self.0.p_depth_stencil_attachment = depth_stencil_attachment;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn preserve_attachments(mut self, preserve_attachments: &'a [u32]) -> Self {
        self.0.preserve_attachment_count = preserve_attachments.len() as _;
        self.0.p_preserve_attachments = preserve_attachments.as_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> SubpassDescription2 {
        self.0
    }
}
impl<'a> std::fmt::Debug for SubpassDescription2Builder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for SubpassDescription2Builder<'a> {
    type Target = SubpassDescription2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SubpassDescription2Builder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAttachmentReference2.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AttachmentReference2 {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub attachment: u32,
    pub layout: crate::vk1_0::ImageLayout,
    pub aspect_mask: crate::vk1_0::ImageAspectFlags,
}
impl AttachmentReference2 {
    #[inline]
    pub fn builder<'a>(self) -> AttachmentReference2Builder<'a> {
        AttachmentReference2Builder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for AttachmentReference2 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("AttachmentReference2")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("attachment", &self.attachment)
            .field("layout", &self.layout)
            .field("aspect_mask", &self.aspect_mask)
            .finish()
    }
}
impl Default for AttachmentReference2 {
    fn default() -> AttachmentReference2 {
        AttachmentReference2 {
            s_type: crate::vk1_0::StructureType::ATTACHMENT_REFERENCE_2,
            p_next: std::ptr::null(),
            attachment: Default::default(),
            layout: Default::default(),
            aspect_mask: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`AttachmentReference2`](struct.AttachmentReference2.html)"]
#[repr(transparent)]
pub struct AttachmentReference2Builder<'a>(AttachmentReference2, std::marker::PhantomData<&'a ()>);
impl<'a> AttachmentReference2Builder<'a> {
    #[inline]
    pub fn new() -> AttachmentReference2Builder<'a> {
        AttachmentReference2Builder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn attachment(mut self, attachment: u32) -> Self {
        self.0.attachment = attachment;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn layout(mut self, layout: crate::vk1_0::ImageLayout) -> Self {
        self.0.layout = layout;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn aspect_mask(mut self, aspect_mask: crate::vk1_0::ImageAspectFlags) -> Self {
        self.0.aspect_mask = aspect_mask;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> AttachmentReference2 {
        self.0
    }
}
impl<'a> std::fmt::Debug for AttachmentReference2Builder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for AttachmentReference2Builder<'a> {
    type Target = AttachmentReference2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for AttachmentReference2Builder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubpassDependency2.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SubpassDependency2 {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub src_subpass: u32,
    pub dst_subpass: u32,
    pub src_stage_mask: crate::vk1_0::PipelineStageFlags,
    pub dst_stage_mask: crate::vk1_0::PipelineStageFlags,
    pub src_access_mask: crate::vk1_0::AccessFlags,
    pub dst_access_mask: crate::vk1_0::AccessFlags,
    pub dependency_flags: crate::vk1_0::DependencyFlags,
    pub view_offset: i32,
}
impl SubpassDependency2 {
    #[inline]
    pub fn builder<'a>(self) -> SubpassDependency2Builder<'a> {
        SubpassDependency2Builder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for SubpassDependency2 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("SubpassDependency2")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("src_subpass", &self.src_subpass)
            .field("dst_subpass", &self.dst_subpass)
            .field("src_stage_mask", &self.src_stage_mask)
            .field("dst_stage_mask", &self.dst_stage_mask)
            .field("src_access_mask", &self.src_access_mask)
            .field("dst_access_mask", &self.dst_access_mask)
            .field("dependency_flags", &self.dependency_flags)
            .field("view_offset", &self.view_offset)
            .finish()
    }
}
impl Default for SubpassDependency2 {
    fn default() -> SubpassDependency2 {
        SubpassDependency2 {
            s_type: crate::vk1_0::StructureType::SUBPASS_DEPENDENCY_2,
            p_next: std::ptr::null(),
            src_subpass: Default::default(),
            dst_subpass: Default::default(),
            src_stage_mask: Default::default(),
            dst_stage_mask: Default::default(),
            src_access_mask: Default::default(),
            dst_access_mask: Default::default(),
            dependency_flags: Default::default(),
            view_offset: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`SubpassDependency2`](struct.SubpassDependency2.html)"]
#[repr(transparent)]
pub struct SubpassDependency2Builder<'a>(SubpassDependency2, std::marker::PhantomData<&'a ()>);
impl<'a> SubpassDependency2Builder<'a> {
    #[inline]
    pub fn new() -> SubpassDependency2Builder<'a> {
        SubpassDependency2Builder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn src_subpass(mut self, src_subpass: u32) -> Self {
        self.0.src_subpass = src_subpass;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn dst_subpass(mut self, dst_subpass: u32) -> Self {
        self.0.dst_subpass = dst_subpass;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn src_stage_mask(mut self, src_stage_mask: crate::vk1_0::PipelineStageFlags) -> Self {
        self.0.src_stage_mask = src_stage_mask;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn dst_stage_mask(mut self, dst_stage_mask: crate::vk1_0::PipelineStageFlags) -> Self {
        self.0.dst_stage_mask = dst_stage_mask;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn src_access_mask(mut self, src_access_mask: crate::vk1_0::AccessFlags) -> Self {
        self.0.src_access_mask = src_access_mask;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn dst_access_mask(mut self, dst_access_mask: crate::vk1_0::AccessFlags) -> Self {
        self.0.dst_access_mask = dst_access_mask;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn dependency_flags(mut self, dependency_flags: crate::vk1_0::DependencyFlags) -> Self {
        self.0.dependency_flags = dependency_flags;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn view_offset(mut self, view_offset: i32) -> Self {
        self.0.view_offset = view_offset;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> SubpassDependency2 {
        self.0
    }
}
impl<'a> std::fmt::Debug for SubpassDependency2Builder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for SubpassDependency2Builder<'a> {
    type Target = SubpassDependency2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SubpassDependency2Builder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubpassBeginInfo.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SubpassBeginInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub contents: crate::vk1_0::SubpassContents,
}
impl SubpassBeginInfo {
    #[inline]
    pub fn builder<'a>(self) -> SubpassBeginInfoBuilder<'a> {
        SubpassBeginInfoBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for SubpassBeginInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("SubpassBeginInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("contents", &self.contents)
            .finish()
    }
}
impl Default for SubpassBeginInfo {
    fn default() -> SubpassBeginInfo {
        SubpassBeginInfo {
            s_type: crate::vk1_0::StructureType::SUBPASS_BEGIN_INFO,
            p_next: std::ptr::null(),
            contents: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`SubpassBeginInfo`](struct.SubpassBeginInfo.html)"]
#[repr(transparent)]
pub struct SubpassBeginInfoBuilder<'a>(SubpassBeginInfo, std::marker::PhantomData<&'a ()>);
impl<'a> SubpassBeginInfoBuilder<'a> {
    #[inline]
    pub fn new() -> SubpassBeginInfoBuilder<'a> {
        SubpassBeginInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn contents(mut self, contents: crate::vk1_0::SubpassContents) -> Self {
        self.0.contents = contents;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> SubpassBeginInfo {
        self.0
    }
}
impl<'a> std::fmt::Debug for SubpassBeginInfoBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for SubpassBeginInfoBuilder<'a> {
    type Target = SubpassBeginInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SubpassBeginInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubpassEndInfo.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SubpassEndInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
}
impl SubpassEndInfo {
    #[inline]
    pub fn builder<'a>(self) -> SubpassEndInfoBuilder<'a> {
        SubpassEndInfoBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for SubpassEndInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("SubpassEndInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .finish()
    }
}
impl Default for SubpassEndInfo {
    fn default() -> SubpassEndInfo {
        SubpassEndInfo {
            s_type: crate::vk1_0::StructureType::SUBPASS_END_INFO,
            p_next: std::ptr::null(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`SubpassEndInfo`](struct.SubpassEndInfo.html)"]
#[repr(transparent)]
pub struct SubpassEndInfoBuilder<'a>(SubpassEndInfo, std::marker::PhantomData<&'a ()>);
impl<'a> SubpassEndInfoBuilder<'a> {
    #[inline]
    pub fn new() -> SubpassEndInfoBuilder<'a> {
        SubpassEndInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> SubpassEndInfo {
        self.0
    }
}
impl<'a> std::fmt::Debug for SubpassEndInfoBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for SubpassEndInfoBuilder<'a> {
    type Target = SubpassEndInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SubpassEndInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreWaitInfo.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SemaphoreWaitInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::vk1_2::SemaphoreWaitFlags,
    pub semaphore_count: u32,
    pub p_semaphores: *const crate::vk1_0::Semaphore,
    pub p_values: *const u64,
}
impl SemaphoreWaitInfo {
    #[inline]
    pub fn builder<'a>(self) -> SemaphoreWaitInfoBuilder<'a> {
        SemaphoreWaitInfoBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for SemaphoreWaitInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("SemaphoreWaitInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("semaphore_count", &self.semaphore_count)
            .field("p_semaphores", &self.p_semaphores)
            .field("p_values", &self.p_values)
            .finish()
    }
}
impl Default for SemaphoreWaitInfo {
    fn default() -> SemaphoreWaitInfo {
        SemaphoreWaitInfo {
            s_type: crate::vk1_0::StructureType::SEMAPHORE_WAIT_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            semaphore_count: Default::default(),
            p_semaphores: std::ptr::null(),
            p_values: std::ptr::null(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`SemaphoreWaitInfo`](struct.SemaphoreWaitInfo.html)"]
#[repr(transparent)]
pub struct SemaphoreWaitInfoBuilder<'a>(SemaphoreWaitInfo, std::marker::PhantomData<&'a ()>);
impl<'a> SemaphoreWaitInfoBuilder<'a> {
    #[inline]
    pub fn new() -> SemaphoreWaitInfoBuilder<'a> {
        SemaphoreWaitInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_2::SemaphoreWaitFlags) -> Self {
        self.0.flags = flags;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn semaphores(mut self, semaphores: &'a [crate::vk1_0::Semaphore]) -> Self {
        self.0.semaphore_count = semaphores.len() as _;
        self.0.p_semaphores = semaphores.as_ptr() as _;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn values(mut self, values: &'a [u64]) -> Self {
        self.0.semaphore_count = values.len() as _;
        self.0.p_values = values.as_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> SemaphoreWaitInfo {
        self.0
    }
}
impl<'a> std::fmt::Debug for SemaphoreWaitInfoBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for SemaphoreWaitInfoBuilder<'a> {
    type Target = SemaphoreWaitInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SemaphoreWaitInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreWaitFlagBits.html) · Flag Bits of [`SemaphoreWaitFlags`](struct.SemaphoreWaitFlags.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct SemaphoreWaitFlagBits(pub u32);
impl SemaphoreWaitFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> SemaphoreWaitFlags {
        SemaphoreWaitFlags::from_bits_truncate(self.0)
    }
}
#[doc = "[Part of `vk1_2`](../vk1_2/index.html)"]
impl SemaphoreWaitFlagBits {
    pub const ANY: Self = Self(0x00000001);
}
#[doc = "[Part of `extensions::khr_timeline_semaphore`](../extensions/khr_timeline_semaphore/index.html)"]
impl SemaphoreWaitFlagBits {
    pub const ANY_KHR: Self = Self::ANY;
}
impl std::fmt::Debug for SemaphoreWaitFlagBits {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::ANY => "ANY",
            _ => "Unknown enum variant",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreWaitFlags.html) · Flags of [`SemaphoreWaitFlagBits`](struct.SemaphoreWaitFlagBits.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct SemaphoreWaitFlags : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; const ANY = SemaphoreWaitFlagBits :: ANY . 0 ; const ANY_KHR = SemaphoreWaitFlagBits :: ANY_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreSignalInfo.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SemaphoreSignalInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub semaphore: crate::vk1_0::Semaphore,
    pub value: u64,
}
impl SemaphoreSignalInfo {
    #[inline]
    pub fn builder<'a>(self) -> SemaphoreSignalInfoBuilder<'a> {
        SemaphoreSignalInfoBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for SemaphoreSignalInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("SemaphoreSignalInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("semaphore", &self.semaphore)
            .field("value", &self.value)
            .finish()
    }
}
impl Default for SemaphoreSignalInfo {
    fn default() -> SemaphoreSignalInfo {
        SemaphoreSignalInfo {
            s_type: crate::vk1_0::StructureType::SEMAPHORE_SIGNAL_INFO,
            p_next: std::ptr::null(),
            semaphore: Default::default(),
            value: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`SemaphoreSignalInfo`](struct.SemaphoreSignalInfo.html)"]
#[repr(transparent)]
pub struct SemaphoreSignalInfoBuilder<'a>(SemaphoreSignalInfo, std::marker::PhantomData<&'a ()>);
impl<'a> SemaphoreSignalInfoBuilder<'a> {
    #[inline]
    pub fn new() -> SemaphoreSignalInfoBuilder<'a> {
        SemaphoreSignalInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn semaphore(mut self, semaphore: crate::vk1_0::Semaphore) -> Self {
        self.0.semaphore = semaphore;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn value(mut self, value: u64) -> Self {
        self.0.value = value;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> SemaphoreSignalInfo {
        self.0
    }
}
impl<'a> std::fmt::Debug for SemaphoreSignalInfoBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for SemaphoreSignalInfoBuilder<'a> {
    type Target = SemaphoreSignalInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SemaphoreSignalInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferDeviceAddressInfo.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BufferDeviceAddressInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub buffer: crate::vk1_0::Buffer,
}
impl BufferDeviceAddressInfo {
    #[inline]
    pub fn builder<'a>(self) -> BufferDeviceAddressInfoBuilder<'a> {
        BufferDeviceAddressInfoBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for BufferDeviceAddressInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("BufferDeviceAddressInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("buffer", &self.buffer)
            .finish()
    }
}
impl Default for BufferDeviceAddressInfo {
    fn default() -> BufferDeviceAddressInfo {
        BufferDeviceAddressInfo {
            s_type: crate::vk1_0::StructureType::BUFFER_DEVICE_ADDRESS_INFO,
            p_next: std::ptr::null(),
            buffer: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`BufferDeviceAddressInfo`](struct.BufferDeviceAddressInfo.html)"]
#[repr(transparent)]
pub struct BufferDeviceAddressInfoBuilder<'a>(
    BufferDeviceAddressInfo,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> BufferDeviceAddressInfoBuilder<'a> {
    #[inline]
    pub fn new() -> BufferDeviceAddressInfoBuilder<'a> {
        BufferDeviceAddressInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn buffer(mut self, buffer: crate::vk1_0::Buffer) -> Self {
        self.0.buffer = buffer;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> BufferDeviceAddressInfo {
        self.0
    }
}
impl<'a> std::fmt::Debug for BufferDeviceAddressInfoBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for BufferDeviceAddressInfoBuilder<'a> {
    type Target = BufferDeviceAddressInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for BufferDeviceAddressInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceAddress.html) · Base Type"]
pub type DeviceAddress = u64;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceMemoryOpaqueCaptureAddressInfo.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DeviceMemoryOpaqueCaptureAddressInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub memory: crate::vk1_0::DeviceMemory,
}
impl DeviceMemoryOpaqueCaptureAddressInfo {
    #[inline]
    pub fn builder<'a>(self) -> DeviceMemoryOpaqueCaptureAddressInfoBuilder<'a> {
        DeviceMemoryOpaqueCaptureAddressInfoBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for DeviceMemoryOpaqueCaptureAddressInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("DeviceMemoryOpaqueCaptureAddressInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("memory", &self.memory)
            .finish()
    }
}
impl Default for DeviceMemoryOpaqueCaptureAddressInfo {
    fn default() -> DeviceMemoryOpaqueCaptureAddressInfo {
        DeviceMemoryOpaqueCaptureAddressInfo {
            s_type: crate::vk1_0::StructureType::DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO,
            p_next: std::ptr::null(),
            memory: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`DeviceMemoryOpaqueCaptureAddressInfo`](struct.DeviceMemoryOpaqueCaptureAddressInfo.html)"]
#[repr(transparent)]
pub struct DeviceMemoryOpaqueCaptureAddressInfoBuilder<'a>(
    DeviceMemoryOpaqueCaptureAddressInfo,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> DeviceMemoryOpaqueCaptureAddressInfoBuilder<'a> {
    #[inline]
    pub fn new() -> DeviceMemoryOpaqueCaptureAddressInfoBuilder<'a> {
        DeviceMemoryOpaqueCaptureAddressInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn memory(mut self, memory: crate::vk1_0::DeviceMemory) -> Self {
        self.0.memory = memory;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> DeviceMemoryOpaqueCaptureAddressInfo {
        self.0
    }
}
impl<'a> std::fmt::Debug for DeviceMemoryOpaqueCaptureAddressInfoBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for DeviceMemoryOpaqueCaptureAddressInfoBuilder<'a> {
    type Target = DeviceMemoryOpaqueCaptureAddressInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DeviceMemoryOpaqueCaptureAddressInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceVulkan11Features.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceVulkan11Features {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub storage_buffer16_bit_access: crate::vk1_0::Bool32,
    pub uniform_and_storage_buffer16_bit_access: crate::vk1_0::Bool32,
    pub storage_push_constant16: crate::vk1_0::Bool32,
    pub storage_input_output16: crate::vk1_0::Bool32,
    pub multiview: crate::vk1_0::Bool32,
    pub multiview_geometry_shader: crate::vk1_0::Bool32,
    pub multiview_tessellation_shader: crate::vk1_0::Bool32,
    pub variable_pointers_storage_buffer: crate::vk1_0::Bool32,
    pub variable_pointers: crate::vk1_0::Bool32,
    pub protected_memory: crate::vk1_0::Bool32,
    pub sampler_ycbcr_conversion: crate::vk1_0::Bool32,
    pub shader_draw_parameters: crate::vk1_0::Bool32,
}
impl PhysicalDeviceVulkan11Features {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceVulkan11Features,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceVulkan11FeaturesBuilder<'a> {
        PhysicalDeviceVulkan11FeaturesBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceVulkan11Features {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceVulkan11Features")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "storage_buffer16_bit_access",
                &(self.storage_buffer16_bit_access != 0),
            )
            .field(
                "uniform_and_storage_buffer16_bit_access",
                &(self.uniform_and_storage_buffer16_bit_access != 0),
            )
            .field(
                "storage_push_constant16",
                &(self.storage_push_constant16 != 0),
            )
            .field(
                "storage_input_output16",
                &(self.storage_input_output16 != 0),
            )
            .field("multiview", &(self.multiview != 0))
            .field(
                "multiview_geometry_shader",
                &(self.multiview_geometry_shader != 0),
            )
            .field(
                "multiview_tessellation_shader",
                &(self.multiview_tessellation_shader != 0),
            )
            .field(
                "variable_pointers_storage_buffer",
                &(self.variable_pointers_storage_buffer != 0),
            )
            .field("variable_pointers", &(self.variable_pointers != 0))
            .field("protected_memory", &(self.protected_memory != 0))
            .field(
                "sampler_ycbcr_conversion",
                &(self.sampler_ycbcr_conversion != 0),
            )
            .field(
                "shader_draw_parameters",
                &(self.shader_draw_parameters != 0),
            )
            .finish()
    }
}
impl Default for PhysicalDeviceVulkan11Features {
    fn default() -> PhysicalDeviceVulkan11Features {
        PhysicalDeviceVulkan11Features {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_VULKAN_1_1_FEATURES,
            p_next: std::ptr::null_mut(),
            storage_buffer16_bit_access: Default::default(),
            uniform_and_storage_buffer16_bit_access: Default::default(),
            storage_push_constant16: Default::default(),
            storage_input_output16: Default::default(),
            multiview: Default::default(),
            multiview_geometry_shader: Default::default(),
            multiview_tessellation_shader: Default::default(),
            variable_pointers_storage_buffer: Default::default(),
            variable_pointers: Default::default(),
            protected_memory: Default::default(),
            sampler_ycbcr_conversion: Default::default(),
            shader_draw_parameters: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceVulkan11Features::extend`](struct.PhysicalDeviceVulkan11Features.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceVulkan11Features {}
impl ExtendableByPhysicalDeviceVulkan11Features for crate::vk1_1::PhysicalDeviceFeatures2 {}
impl ExtendableByPhysicalDeviceVulkan11Features for crate::vk1_0::DeviceCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceVulkan11Features`](struct.PhysicalDeviceVulkan11Features.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceVulkan11FeaturesBuilder<'a>(
    PhysicalDeviceVulkan11Features,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceVulkan11FeaturesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceVulkan11FeaturesBuilder<'a> {
        PhysicalDeviceVulkan11FeaturesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn storage_buffer16_bit_access(mut self, storage_buffer16_bit_access: bool) -> Self {
        self.0.storage_buffer16_bit_access = storage_buffer16_bit_access as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn uniform_and_storage_buffer16_bit_access(
        mut self,
        uniform_and_storage_buffer16_bit_access: bool,
    ) -> Self {
        self.0.uniform_and_storage_buffer16_bit_access =
            uniform_and_storage_buffer16_bit_access as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn storage_push_constant16(mut self, storage_push_constant16: bool) -> Self {
        self.0.storage_push_constant16 = storage_push_constant16 as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn storage_input_output16(mut self, storage_input_output16: bool) -> Self {
        self.0.storage_input_output16 = storage_input_output16 as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn multiview(mut self, multiview: bool) -> Self {
        self.0.multiview = multiview as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn multiview_geometry_shader(mut self, multiview_geometry_shader: bool) -> Self {
        self.0.multiview_geometry_shader = multiview_geometry_shader as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn multiview_tessellation_shader(mut self, multiview_tessellation_shader: bool) -> Self {
        self.0.multiview_tessellation_shader = multiview_tessellation_shader as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn variable_pointers_storage_buffer(
        mut self,
        variable_pointers_storage_buffer: bool,
    ) -> Self {
        self.0.variable_pointers_storage_buffer = variable_pointers_storage_buffer as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn variable_pointers(mut self, variable_pointers: bool) -> Self {
        self.0.variable_pointers = variable_pointers as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn protected_memory(mut self, protected_memory: bool) -> Self {
        self.0.protected_memory = protected_memory as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn sampler_ycbcr_conversion(mut self, sampler_ycbcr_conversion: bool) -> Self {
        self.0.sampler_ycbcr_conversion = sampler_ycbcr_conversion as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_draw_parameters(mut self, shader_draw_parameters: bool) -> Self {
        self.0.shader_draw_parameters = shader_draw_parameters as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceVulkan11Features {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceVulkan11FeaturesBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceVulkan11FeaturesBuilder<'a> {
    type Target = PhysicalDeviceVulkan11Features;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceVulkan11FeaturesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceVulkan11Properties.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceVulkan11Properties {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub device_uuid: [u8; crate::vk1_0::UUID_SIZE as usize],
    pub driver_uuid: [u8; crate::vk1_0::UUID_SIZE as usize],
    pub device_luid: [u8; crate::vk1_1::LUID_SIZE as usize],
    pub device_node_mask: u32,
    pub device_luid_valid: crate::vk1_0::Bool32,
    pub subgroup_size: u32,
    pub subgroup_supported_stages: crate::vk1_0::ShaderStageFlags,
    pub subgroup_supported_operations: crate::vk1_1::SubgroupFeatureFlags,
    pub subgroup_quad_operations_in_all_stages: crate::vk1_0::Bool32,
    pub point_clipping_behavior: crate::vk1_1::PointClippingBehavior,
    pub max_multiview_view_count: u32,
    pub max_multiview_instance_index: u32,
    pub protected_no_fault: crate::vk1_0::Bool32,
    pub max_per_set_descriptors: u32,
    pub max_memory_allocation_size: crate::vk1_0::DeviceSize,
}
impl PhysicalDeviceVulkan11Properties {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceVulkan11Properties,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceVulkan11PropertiesBuilder<'a> {
        PhysicalDeviceVulkan11PropertiesBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceVulkan11Properties {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceVulkan11Properties")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("device_uuid", &self.device_uuid)
            .field("driver_uuid", &self.driver_uuid)
            .field("device_luid", &self.device_luid)
            .field("device_node_mask", &self.device_node_mask)
            .field("device_luid_valid", &(self.device_luid_valid != 0))
            .field("subgroup_size", &self.subgroup_size)
            .field("subgroup_supported_stages", &self.subgroup_supported_stages)
            .field(
                "subgroup_supported_operations",
                &self.subgroup_supported_operations,
            )
            .field(
                "subgroup_quad_operations_in_all_stages",
                &(self.subgroup_quad_operations_in_all_stages != 0),
            )
            .field("point_clipping_behavior", &self.point_clipping_behavior)
            .field("max_multiview_view_count", &self.max_multiview_view_count)
            .field(
                "max_multiview_instance_index",
                &self.max_multiview_instance_index,
            )
            .field("protected_no_fault", &(self.protected_no_fault != 0))
            .field("max_per_set_descriptors", &self.max_per_set_descriptors)
            .field(
                "max_memory_allocation_size",
                &self.max_memory_allocation_size,
            )
            .finish()
    }
}
impl Default for PhysicalDeviceVulkan11Properties {
    fn default() -> PhysicalDeviceVulkan11Properties {
        PhysicalDeviceVulkan11Properties {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES,
            p_next: std::ptr::null_mut(),
            device_uuid: Default::default(),
            driver_uuid: Default::default(),
            device_luid: Default::default(),
            device_node_mask: Default::default(),
            device_luid_valid: Default::default(),
            subgroup_size: Default::default(),
            subgroup_supported_stages: Default::default(),
            subgroup_supported_operations: Default::default(),
            subgroup_quad_operations_in_all_stages: Default::default(),
            point_clipping_behavior: Default::default(),
            max_multiview_view_count: Default::default(),
            max_multiview_instance_index: Default::default(),
            protected_no_fault: Default::default(),
            max_per_set_descriptors: Default::default(),
            max_memory_allocation_size: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceVulkan11Properties::extend`](struct.PhysicalDeviceVulkan11Properties.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceVulkan11Properties {}
impl ExtendableByPhysicalDeviceVulkan11Properties for crate::vk1_1::PhysicalDeviceProperties2 {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceVulkan11Properties`](struct.PhysicalDeviceVulkan11Properties.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceVulkan11PropertiesBuilder<'a>(
    PhysicalDeviceVulkan11Properties,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceVulkan11PropertiesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceVulkan11PropertiesBuilder<'a> {
        PhysicalDeviceVulkan11PropertiesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn device_uuid(mut self, device_uuid: [u8; crate::vk1_0::UUID_SIZE as usize]) -> Self {
        self.0.device_uuid = device_uuid;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn driver_uuid(mut self, driver_uuid: [u8; crate::vk1_0::UUID_SIZE as usize]) -> Self {
        self.0.driver_uuid = driver_uuid;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn device_luid(mut self, device_luid: [u8; crate::vk1_1::LUID_SIZE as usize]) -> Self {
        self.0.device_luid = device_luid;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn device_node_mask(mut self, device_node_mask: u32) -> Self {
        self.0.device_node_mask = device_node_mask;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn device_luid_valid(mut self, device_luid_valid: bool) -> Self {
        self.0.device_luid_valid = device_luid_valid as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn subgroup_size(mut self, subgroup_size: u32) -> Self {
        self.0.subgroup_size = subgroup_size;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn subgroup_supported_stages(
        mut self,
        subgroup_supported_stages: crate::vk1_0::ShaderStageFlags,
    ) -> Self {
        self.0.subgroup_supported_stages = subgroup_supported_stages;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn subgroup_supported_operations(
        mut self,
        subgroup_supported_operations: crate::vk1_1::SubgroupFeatureFlags,
    ) -> Self {
        self.0.subgroup_supported_operations = subgroup_supported_operations;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn subgroup_quad_operations_in_all_stages(
        mut self,
        subgroup_quad_operations_in_all_stages: bool,
    ) -> Self {
        self.0.subgroup_quad_operations_in_all_stages =
            subgroup_quad_operations_in_all_stages as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn point_clipping_behavior(
        mut self,
        point_clipping_behavior: crate::vk1_1::PointClippingBehavior,
    ) -> Self {
        self.0.point_clipping_behavior = point_clipping_behavior;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_multiview_view_count(mut self, max_multiview_view_count: u32) -> Self {
        self.0.max_multiview_view_count = max_multiview_view_count;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_multiview_instance_index(mut self, max_multiview_instance_index: u32) -> Self {
        self.0.max_multiview_instance_index = max_multiview_instance_index;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn protected_no_fault(mut self, protected_no_fault: bool) -> Self {
        self.0.protected_no_fault = protected_no_fault as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_per_set_descriptors(mut self, max_per_set_descriptors: u32) -> Self {
        self.0.max_per_set_descriptors = max_per_set_descriptors;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_memory_allocation_size(
        mut self,
        max_memory_allocation_size: crate::vk1_0::DeviceSize,
    ) -> Self {
        self.0.max_memory_allocation_size = max_memory_allocation_size;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceVulkan11Properties {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceVulkan11PropertiesBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceVulkan11PropertiesBuilder<'a> {
    type Target = PhysicalDeviceVulkan11Properties;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceVulkan11PropertiesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceVulkan12Features.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceVulkan12Features {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub sampler_mirror_clamp_to_edge: crate::vk1_0::Bool32,
    pub draw_indirect_count: crate::vk1_0::Bool32,
    pub storage_buffer8_bit_access: crate::vk1_0::Bool32,
    pub uniform_and_storage_buffer8_bit_access: crate::vk1_0::Bool32,
    pub storage_push_constant8: crate::vk1_0::Bool32,
    pub shader_buffer_int64_atomics: crate::vk1_0::Bool32,
    pub shader_shared_int64_atomics: crate::vk1_0::Bool32,
    pub shader_float16: crate::vk1_0::Bool32,
    pub shader_int8: crate::vk1_0::Bool32,
    pub descriptor_indexing: crate::vk1_0::Bool32,
    pub shader_input_attachment_array_dynamic_indexing: crate::vk1_0::Bool32,
    pub shader_uniform_texel_buffer_array_dynamic_indexing: crate::vk1_0::Bool32,
    pub shader_storage_texel_buffer_array_dynamic_indexing: crate::vk1_0::Bool32,
    pub shader_uniform_buffer_array_non_uniform_indexing: crate::vk1_0::Bool32,
    pub shader_sampled_image_array_non_uniform_indexing: crate::vk1_0::Bool32,
    pub shader_storage_buffer_array_non_uniform_indexing: crate::vk1_0::Bool32,
    pub shader_storage_image_array_non_uniform_indexing: crate::vk1_0::Bool32,
    pub shader_input_attachment_array_non_uniform_indexing: crate::vk1_0::Bool32,
    pub shader_uniform_texel_buffer_array_non_uniform_indexing: crate::vk1_0::Bool32,
    pub shader_storage_texel_buffer_array_non_uniform_indexing: crate::vk1_0::Bool32,
    pub descriptor_binding_uniform_buffer_update_after_bind: crate::vk1_0::Bool32,
    pub descriptor_binding_sampled_image_update_after_bind: crate::vk1_0::Bool32,
    pub descriptor_binding_storage_image_update_after_bind: crate::vk1_0::Bool32,
    pub descriptor_binding_storage_buffer_update_after_bind: crate::vk1_0::Bool32,
    pub descriptor_binding_uniform_texel_buffer_update_after_bind: crate::vk1_0::Bool32,
    pub descriptor_binding_storage_texel_buffer_update_after_bind: crate::vk1_0::Bool32,
    pub descriptor_binding_update_unused_while_pending: crate::vk1_0::Bool32,
    pub descriptor_binding_partially_bound: crate::vk1_0::Bool32,
    pub descriptor_binding_variable_descriptor_count: crate::vk1_0::Bool32,
    pub runtime_descriptor_array: crate::vk1_0::Bool32,
    pub sampler_filter_minmax: crate::vk1_0::Bool32,
    pub scalar_block_layout: crate::vk1_0::Bool32,
    pub imageless_framebuffer: crate::vk1_0::Bool32,
    pub uniform_buffer_standard_layout: crate::vk1_0::Bool32,
    pub shader_subgroup_extended_types: crate::vk1_0::Bool32,
    pub separate_depth_stencil_layouts: crate::vk1_0::Bool32,
    pub host_query_reset: crate::vk1_0::Bool32,
    pub timeline_semaphore: crate::vk1_0::Bool32,
    pub buffer_device_address: crate::vk1_0::Bool32,
    pub buffer_device_address_capture_replay: crate::vk1_0::Bool32,
    pub buffer_device_address_multi_device: crate::vk1_0::Bool32,
    pub vulkan_memory_model: crate::vk1_0::Bool32,
    pub vulkan_memory_model_device_scope: crate::vk1_0::Bool32,
    pub vulkan_memory_model_availability_visibility_chains: crate::vk1_0::Bool32,
    pub shader_output_viewport_index: crate::vk1_0::Bool32,
    pub shader_output_layer: crate::vk1_0::Bool32,
    pub subgroup_broadcast_dynamic_id: crate::vk1_0::Bool32,
}
impl PhysicalDeviceVulkan12Features {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceVulkan12Features,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceVulkan12FeaturesBuilder<'a> {
        PhysicalDeviceVulkan12FeaturesBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceVulkan12Features {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceVulkan12Features")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "sampler_mirror_clamp_to_edge",
                &(self.sampler_mirror_clamp_to_edge != 0),
            )
            .field("draw_indirect_count", &(self.draw_indirect_count != 0))
            .field(
                "storage_buffer8_bit_access",
                &(self.storage_buffer8_bit_access != 0),
            )
            .field(
                "uniform_and_storage_buffer8_bit_access",
                &(self.uniform_and_storage_buffer8_bit_access != 0),
            )
            .field(
                "storage_push_constant8",
                &(self.storage_push_constant8 != 0),
            )
            .field(
                "shader_buffer_int64_atomics",
                &(self.shader_buffer_int64_atomics != 0),
            )
            .field(
                "shader_shared_int64_atomics",
                &(self.shader_shared_int64_atomics != 0),
            )
            .field("shader_float16", &(self.shader_float16 != 0))
            .field("shader_int8", &(self.shader_int8 != 0))
            .field("descriptor_indexing", &(self.descriptor_indexing != 0))
            .field(
                "shader_input_attachment_array_dynamic_indexing",
                &(self.shader_input_attachment_array_dynamic_indexing != 0),
            )
            .field(
                "shader_uniform_texel_buffer_array_dynamic_indexing",
                &(self.shader_uniform_texel_buffer_array_dynamic_indexing != 0),
            )
            .field(
                "shader_storage_texel_buffer_array_dynamic_indexing",
                &(self.shader_storage_texel_buffer_array_dynamic_indexing != 0),
            )
            .field(
                "shader_uniform_buffer_array_non_uniform_indexing",
                &(self.shader_uniform_buffer_array_non_uniform_indexing != 0),
            )
            .field(
                "shader_sampled_image_array_non_uniform_indexing",
                &(self.shader_sampled_image_array_non_uniform_indexing != 0),
            )
            .field(
                "shader_storage_buffer_array_non_uniform_indexing",
                &(self.shader_storage_buffer_array_non_uniform_indexing != 0),
            )
            .field(
                "shader_storage_image_array_non_uniform_indexing",
                &(self.shader_storage_image_array_non_uniform_indexing != 0),
            )
            .field(
                "shader_input_attachment_array_non_uniform_indexing",
                &(self.shader_input_attachment_array_non_uniform_indexing != 0),
            )
            .field(
                "shader_uniform_texel_buffer_array_non_uniform_indexing",
                &(self.shader_uniform_texel_buffer_array_non_uniform_indexing != 0),
            )
            .field(
                "shader_storage_texel_buffer_array_non_uniform_indexing",
                &(self.shader_storage_texel_buffer_array_non_uniform_indexing != 0),
            )
            .field(
                "descriptor_binding_uniform_buffer_update_after_bind",
                &(self.descriptor_binding_uniform_buffer_update_after_bind != 0),
            )
            .field(
                "descriptor_binding_sampled_image_update_after_bind",
                &(self.descriptor_binding_sampled_image_update_after_bind != 0),
            )
            .field(
                "descriptor_binding_storage_image_update_after_bind",
                &(self.descriptor_binding_storage_image_update_after_bind != 0),
            )
            .field(
                "descriptor_binding_storage_buffer_update_after_bind",
                &(self.descriptor_binding_storage_buffer_update_after_bind != 0),
            )
            .field(
                "descriptor_binding_uniform_texel_buffer_update_after_bind",
                &(self.descriptor_binding_uniform_texel_buffer_update_after_bind != 0),
            )
            .field(
                "descriptor_binding_storage_texel_buffer_update_after_bind",
                &(self.descriptor_binding_storage_texel_buffer_update_after_bind != 0),
            )
            .field(
                "descriptor_binding_update_unused_while_pending",
                &(self.descriptor_binding_update_unused_while_pending != 0),
            )
            .field(
                "descriptor_binding_partially_bound",
                &(self.descriptor_binding_partially_bound != 0),
            )
            .field(
                "descriptor_binding_variable_descriptor_count",
                &(self.descriptor_binding_variable_descriptor_count != 0),
            )
            .field(
                "runtime_descriptor_array",
                &(self.runtime_descriptor_array != 0),
            )
            .field("sampler_filter_minmax", &(self.sampler_filter_minmax != 0))
            .field("scalar_block_layout", &(self.scalar_block_layout != 0))
            .field("imageless_framebuffer", &(self.imageless_framebuffer != 0))
            .field(
                "uniform_buffer_standard_layout",
                &(self.uniform_buffer_standard_layout != 0),
            )
            .field(
                "shader_subgroup_extended_types",
                &(self.shader_subgroup_extended_types != 0),
            )
            .field(
                "separate_depth_stencil_layouts",
                &(self.separate_depth_stencil_layouts != 0),
            )
            .field("host_query_reset", &(self.host_query_reset != 0))
            .field("timeline_semaphore", &(self.timeline_semaphore != 0))
            .field("buffer_device_address", &(self.buffer_device_address != 0))
            .field(
                "buffer_device_address_capture_replay",
                &(self.buffer_device_address_capture_replay != 0),
            )
            .field(
                "buffer_device_address_multi_device",
                &(self.buffer_device_address_multi_device != 0),
            )
            .field("vulkan_memory_model", &(self.vulkan_memory_model != 0))
            .field(
                "vulkan_memory_model_device_scope",
                &(self.vulkan_memory_model_device_scope != 0),
            )
            .field(
                "vulkan_memory_model_availability_visibility_chains",
                &(self.vulkan_memory_model_availability_visibility_chains != 0),
            )
            .field(
                "shader_output_viewport_index",
                &(self.shader_output_viewport_index != 0),
            )
            .field("shader_output_layer", &(self.shader_output_layer != 0))
            .field(
                "subgroup_broadcast_dynamic_id",
                &(self.subgroup_broadcast_dynamic_id != 0),
            )
            .finish()
    }
}
impl Default for PhysicalDeviceVulkan12Features {
    fn default() -> PhysicalDeviceVulkan12Features {
        PhysicalDeviceVulkan12Features {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_VULKAN_1_2_FEATURES,
            p_next: std::ptr::null_mut(),
            sampler_mirror_clamp_to_edge: Default::default(),
            draw_indirect_count: Default::default(),
            storage_buffer8_bit_access: Default::default(),
            uniform_and_storage_buffer8_bit_access: Default::default(),
            storage_push_constant8: Default::default(),
            shader_buffer_int64_atomics: Default::default(),
            shader_shared_int64_atomics: Default::default(),
            shader_float16: Default::default(),
            shader_int8: Default::default(),
            descriptor_indexing: Default::default(),
            shader_input_attachment_array_dynamic_indexing: Default::default(),
            shader_uniform_texel_buffer_array_dynamic_indexing: Default::default(),
            shader_storage_texel_buffer_array_dynamic_indexing: Default::default(),
            shader_uniform_buffer_array_non_uniform_indexing: Default::default(),
            shader_sampled_image_array_non_uniform_indexing: Default::default(),
            shader_storage_buffer_array_non_uniform_indexing: Default::default(),
            shader_storage_image_array_non_uniform_indexing: Default::default(),
            shader_input_attachment_array_non_uniform_indexing: Default::default(),
            shader_uniform_texel_buffer_array_non_uniform_indexing: Default::default(),
            shader_storage_texel_buffer_array_non_uniform_indexing: Default::default(),
            descriptor_binding_uniform_buffer_update_after_bind: Default::default(),
            descriptor_binding_sampled_image_update_after_bind: Default::default(),
            descriptor_binding_storage_image_update_after_bind: Default::default(),
            descriptor_binding_storage_buffer_update_after_bind: Default::default(),
            descriptor_binding_uniform_texel_buffer_update_after_bind: Default::default(),
            descriptor_binding_storage_texel_buffer_update_after_bind: Default::default(),
            descriptor_binding_update_unused_while_pending: Default::default(),
            descriptor_binding_partially_bound: Default::default(),
            descriptor_binding_variable_descriptor_count: Default::default(),
            runtime_descriptor_array: Default::default(),
            sampler_filter_minmax: Default::default(),
            scalar_block_layout: Default::default(),
            imageless_framebuffer: Default::default(),
            uniform_buffer_standard_layout: Default::default(),
            shader_subgroup_extended_types: Default::default(),
            separate_depth_stencil_layouts: Default::default(),
            host_query_reset: Default::default(),
            timeline_semaphore: Default::default(),
            buffer_device_address: Default::default(),
            buffer_device_address_capture_replay: Default::default(),
            buffer_device_address_multi_device: Default::default(),
            vulkan_memory_model: Default::default(),
            vulkan_memory_model_device_scope: Default::default(),
            vulkan_memory_model_availability_visibility_chains: Default::default(),
            shader_output_viewport_index: Default::default(),
            shader_output_layer: Default::default(),
            subgroup_broadcast_dynamic_id: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceVulkan12Features::extend`](struct.PhysicalDeviceVulkan12Features.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceVulkan12Features {}
impl ExtendableByPhysicalDeviceVulkan12Features for crate::vk1_1::PhysicalDeviceFeatures2 {}
impl ExtendableByPhysicalDeviceVulkan12Features for crate::vk1_0::DeviceCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceVulkan12Features`](struct.PhysicalDeviceVulkan12Features.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceVulkan12FeaturesBuilder<'a>(
    PhysicalDeviceVulkan12Features,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceVulkan12FeaturesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceVulkan12FeaturesBuilder<'a> {
        PhysicalDeviceVulkan12FeaturesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn sampler_mirror_clamp_to_edge(mut self, sampler_mirror_clamp_to_edge: bool) -> Self {
        self.0.sampler_mirror_clamp_to_edge = sampler_mirror_clamp_to_edge as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn draw_indirect_count(mut self, draw_indirect_count: bool) -> Self {
        self.0.draw_indirect_count = draw_indirect_count as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn storage_buffer8_bit_access(mut self, storage_buffer8_bit_access: bool) -> Self {
        self.0.storage_buffer8_bit_access = storage_buffer8_bit_access as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn uniform_and_storage_buffer8_bit_access(
        mut self,
        uniform_and_storage_buffer8_bit_access: bool,
    ) -> Self {
        self.0.uniform_and_storage_buffer8_bit_access =
            uniform_and_storage_buffer8_bit_access as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn storage_push_constant8(mut self, storage_push_constant8: bool) -> Self {
        self.0.storage_push_constant8 = storage_push_constant8 as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_buffer_int64_atomics(mut self, shader_buffer_int64_atomics: bool) -> Self {
        self.0.shader_buffer_int64_atomics = shader_buffer_int64_atomics as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_shared_int64_atomics(mut self, shader_shared_int64_atomics: bool) -> Self {
        self.0.shader_shared_int64_atomics = shader_shared_int64_atomics as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_float16(mut self, shader_float16: bool) -> Self {
        self.0.shader_float16 = shader_float16 as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_int8(mut self, shader_int8: bool) -> Self {
        self.0.shader_int8 = shader_int8 as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn descriptor_indexing(mut self, descriptor_indexing: bool) -> Self {
        self.0.descriptor_indexing = descriptor_indexing as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_input_attachment_array_dynamic_indexing(
        mut self,
        shader_input_attachment_array_dynamic_indexing: bool,
    ) -> Self {
        self.0.shader_input_attachment_array_dynamic_indexing =
            shader_input_attachment_array_dynamic_indexing as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_uniform_texel_buffer_array_dynamic_indexing(
        mut self,
        shader_uniform_texel_buffer_array_dynamic_indexing: bool,
    ) -> Self {
        self.0.shader_uniform_texel_buffer_array_dynamic_indexing =
            shader_uniform_texel_buffer_array_dynamic_indexing as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_storage_texel_buffer_array_dynamic_indexing(
        mut self,
        shader_storage_texel_buffer_array_dynamic_indexing: bool,
    ) -> Self {
        self.0.shader_storage_texel_buffer_array_dynamic_indexing =
            shader_storage_texel_buffer_array_dynamic_indexing as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_uniform_buffer_array_non_uniform_indexing(
        mut self,
        shader_uniform_buffer_array_non_uniform_indexing: bool,
    ) -> Self {
        self.0.shader_uniform_buffer_array_non_uniform_indexing =
            shader_uniform_buffer_array_non_uniform_indexing as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_sampled_image_array_non_uniform_indexing(
        mut self,
        shader_sampled_image_array_non_uniform_indexing: bool,
    ) -> Self {
        self.0.shader_sampled_image_array_non_uniform_indexing =
            shader_sampled_image_array_non_uniform_indexing as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_storage_buffer_array_non_uniform_indexing(
        mut self,
        shader_storage_buffer_array_non_uniform_indexing: bool,
    ) -> Self {
        self.0.shader_storage_buffer_array_non_uniform_indexing =
            shader_storage_buffer_array_non_uniform_indexing as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_storage_image_array_non_uniform_indexing(
        mut self,
        shader_storage_image_array_non_uniform_indexing: bool,
    ) -> Self {
        self.0.shader_storage_image_array_non_uniform_indexing =
            shader_storage_image_array_non_uniform_indexing as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_input_attachment_array_non_uniform_indexing(
        mut self,
        shader_input_attachment_array_non_uniform_indexing: bool,
    ) -> Self {
        self.0.shader_input_attachment_array_non_uniform_indexing =
            shader_input_attachment_array_non_uniform_indexing as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_uniform_texel_buffer_array_non_uniform_indexing(
        mut self,
        shader_uniform_texel_buffer_array_non_uniform_indexing: bool,
    ) -> Self {
        self.0
            .shader_uniform_texel_buffer_array_non_uniform_indexing =
            shader_uniform_texel_buffer_array_non_uniform_indexing as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_storage_texel_buffer_array_non_uniform_indexing(
        mut self,
        shader_storage_texel_buffer_array_non_uniform_indexing: bool,
    ) -> Self {
        self.0
            .shader_storage_texel_buffer_array_non_uniform_indexing =
            shader_storage_texel_buffer_array_non_uniform_indexing as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn descriptor_binding_uniform_buffer_update_after_bind(
        mut self,
        descriptor_binding_uniform_buffer_update_after_bind: bool,
    ) -> Self {
        self.0.descriptor_binding_uniform_buffer_update_after_bind =
            descriptor_binding_uniform_buffer_update_after_bind as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn descriptor_binding_sampled_image_update_after_bind(
        mut self,
        descriptor_binding_sampled_image_update_after_bind: bool,
    ) -> Self {
        self.0.descriptor_binding_sampled_image_update_after_bind =
            descriptor_binding_sampled_image_update_after_bind as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn descriptor_binding_storage_image_update_after_bind(
        mut self,
        descriptor_binding_storage_image_update_after_bind: bool,
    ) -> Self {
        self.0.descriptor_binding_storage_image_update_after_bind =
            descriptor_binding_storage_image_update_after_bind as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn descriptor_binding_storage_buffer_update_after_bind(
        mut self,
        descriptor_binding_storage_buffer_update_after_bind: bool,
    ) -> Self {
        self.0.descriptor_binding_storage_buffer_update_after_bind =
            descriptor_binding_storage_buffer_update_after_bind as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn descriptor_binding_uniform_texel_buffer_update_after_bind(
        mut self,
        descriptor_binding_uniform_texel_buffer_update_after_bind: bool,
    ) -> Self {
        self.0
            .descriptor_binding_uniform_texel_buffer_update_after_bind =
            descriptor_binding_uniform_texel_buffer_update_after_bind as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn descriptor_binding_storage_texel_buffer_update_after_bind(
        mut self,
        descriptor_binding_storage_texel_buffer_update_after_bind: bool,
    ) -> Self {
        self.0
            .descriptor_binding_storage_texel_buffer_update_after_bind =
            descriptor_binding_storage_texel_buffer_update_after_bind as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn descriptor_binding_update_unused_while_pending(
        mut self,
        descriptor_binding_update_unused_while_pending: bool,
    ) -> Self {
        self.0.descriptor_binding_update_unused_while_pending =
            descriptor_binding_update_unused_while_pending as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn descriptor_binding_partially_bound(
        mut self,
        descriptor_binding_partially_bound: bool,
    ) -> Self {
        self.0.descriptor_binding_partially_bound = descriptor_binding_partially_bound as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn descriptor_binding_variable_descriptor_count(
        mut self,
        descriptor_binding_variable_descriptor_count: bool,
    ) -> Self {
        self.0.descriptor_binding_variable_descriptor_count =
            descriptor_binding_variable_descriptor_count as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn runtime_descriptor_array(mut self, runtime_descriptor_array: bool) -> Self {
        self.0.runtime_descriptor_array = runtime_descriptor_array as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn sampler_filter_minmax(mut self, sampler_filter_minmax: bool) -> Self {
        self.0.sampler_filter_minmax = sampler_filter_minmax as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn scalar_block_layout(mut self, scalar_block_layout: bool) -> Self {
        self.0.scalar_block_layout = scalar_block_layout as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn imageless_framebuffer(mut self, imageless_framebuffer: bool) -> Self {
        self.0.imageless_framebuffer = imageless_framebuffer as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn uniform_buffer_standard_layout(mut self, uniform_buffer_standard_layout: bool) -> Self {
        self.0.uniform_buffer_standard_layout = uniform_buffer_standard_layout as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_subgroup_extended_types(mut self, shader_subgroup_extended_types: bool) -> Self {
        self.0.shader_subgroup_extended_types = shader_subgroup_extended_types as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn separate_depth_stencil_layouts(mut self, separate_depth_stencil_layouts: bool) -> Self {
        self.0.separate_depth_stencil_layouts = separate_depth_stencil_layouts as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn host_query_reset(mut self, host_query_reset: bool) -> Self {
        self.0.host_query_reset = host_query_reset as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn timeline_semaphore(mut self, timeline_semaphore: bool) -> Self {
        self.0.timeline_semaphore = timeline_semaphore as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn buffer_device_address(mut self, buffer_device_address: bool) -> Self {
        self.0.buffer_device_address = buffer_device_address as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn buffer_device_address_capture_replay(
        mut self,
        buffer_device_address_capture_replay: bool,
    ) -> Self {
        self.0.buffer_device_address_capture_replay = buffer_device_address_capture_replay as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn buffer_device_address_multi_device(
        mut self,
        buffer_device_address_multi_device: bool,
    ) -> Self {
        self.0.buffer_device_address_multi_device = buffer_device_address_multi_device as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn vulkan_memory_model(mut self, vulkan_memory_model: bool) -> Self {
        self.0.vulkan_memory_model = vulkan_memory_model as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn vulkan_memory_model_device_scope(
        mut self,
        vulkan_memory_model_device_scope: bool,
    ) -> Self {
        self.0.vulkan_memory_model_device_scope = vulkan_memory_model_device_scope as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn vulkan_memory_model_availability_visibility_chains(
        mut self,
        vulkan_memory_model_availability_visibility_chains: bool,
    ) -> Self {
        self.0.vulkan_memory_model_availability_visibility_chains =
            vulkan_memory_model_availability_visibility_chains as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_output_viewport_index(mut self, shader_output_viewport_index: bool) -> Self {
        self.0.shader_output_viewport_index = shader_output_viewport_index as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_output_layer(mut self, shader_output_layer: bool) -> Self {
        self.0.shader_output_layer = shader_output_layer as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn subgroup_broadcast_dynamic_id(mut self, subgroup_broadcast_dynamic_id: bool) -> Self {
        self.0.subgroup_broadcast_dynamic_id = subgroup_broadcast_dynamic_id as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceVulkan12Features {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceVulkan12FeaturesBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceVulkan12FeaturesBuilder<'a> {
    type Target = PhysicalDeviceVulkan12Features;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceVulkan12FeaturesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceVulkan12Properties.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceVulkan12Properties {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub driver_id: crate::vk1_2::DriverId,
    pub driver_name: [std::os::raw::c_char; crate::vk1_2::MAX_DRIVER_NAME_SIZE as usize],
    pub driver_info: [std::os::raw::c_char; crate::vk1_2::MAX_DRIVER_INFO_SIZE as usize],
    pub conformance_version: crate::vk1_2::ConformanceVersion,
    pub denorm_behavior_independence: crate::vk1_2::ShaderFloatControlsIndependence,
    pub rounding_mode_independence: crate::vk1_2::ShaderFloatControlsIndependence,
    pub shader_signed_zero_inf_nan_preserve_float16: crate::vk1_0::Bool32,
    pub shader_signed_zero_inf_nan_preserve_float32: crate::vk1_0::Bool32,
    pub shader_signed_zero_inf_nan_preserve_float64: crate::vk1_0::Bool32,
    pub shader_denorm_preserve_float16: crate::vk1_0::Bool32,
    pub shader_denorm_preserve_float32: crate::vk1_0::Bool32,
    pub shader_denorm_preserve_float64: crate::vk1_0::Bool32,
    pub shader_denorm_flush_to_zero_float16: crate::vk1_0::Bool32,
    pub shader_denorm_flush_to_zero_float32: crate::vk1_0::Bool32,
    pub shader_denorm_flush_to_zero_float64: crate::vk1_0::Bool32,
    pub shader_rounding_mode_rte_float16: crate::vk1_0::Bool32,
    pub shader_rounding_mode_rte_float32: crate::vk1_0::Bool32,
    pub shader_rounding_mode_rte_float64: crate::vk1_0::Bool32,
    pub shader_rounding_mode_rtz_float16: crate::vk1_0::Bool32,
    pub shader_rounding_mode_rtz_float32: crate::vk1_0::Bool32,
    pub shader_rounding_mode_rtz_float64: crate::vk1_0::Bool32,
    pub max_update_after_bind_descriptors_in_all_pools: u32,
    pub shader_uniform_buffer_array_non_uniform_indexing_native: crate::vk1_0::Bool32,
    pub shader_sampled_image_array_non_uniform_indexing_native: crate::vk1_0::Bool32,
    pub shader_storage_buffer_array_non_uniform_indexing_native: crate::vk1_0::Bool32,
    pub shader_storage_image_array_non_uniform_indexing_native: crate::vk1_0::Bool32,
    pub shader_input_attachment_array_non_uniform_indexing_native: crate::vk1_0::Bool32,
    pub robust_buffer_access_update_after_bind: crate::vk1_0::Bool32,
    pub quad_divergent_implicit_lod: crate::vk1_0::Bool32,
    pub max_per_stage_descriptor_update_after_bind_samplers: u32,
    pub max_per_stage_descriptor_update_after_bind_uniform_buffers: u32,
    pub max_per_stage_descriptor_update_after_bind_storage_buffers: u32,
    pub max_per_stage_descriptor_update_after_bind_sampled_images: u32,
    pub max_per_stage_descriptor_update_after_bind_storage_images: u32,
    pub max_per_stage_descriptor_update_after_bind_input_attachments: u32,
    pub max_per_stage_update_after_bind_resources: u32,
    pub max_descriptor_set_update_after_bind_samplers: u32,
    pub max_descriptor_set_update_after_bind_uniform_buffers: u32,
    pub max_descriptor_set_update_after_bind_uniform_buffers_dynamic: u32,
    pub max_descriptor_set_update_after_bind_storage_buffers: u32,
    pub max_descriptor_set_update_after_bind_storage_buffers_dynamic: u32,
    pub max_descriptor_set_update_after_bind_sampled_images: u32,
    pub max_descriptor_set_update_after_bind_storage_images: u32,
    pub max_descriptor_set_update_after_bind_input_attachments: u32,
    pub supported_depth_resolve_modes: crate::vk1_2::ResolveModeFlags,
    pub supported_stencil_resolve_modes: crate::vk1_2::ResolveModeFlags,
    pub independent_resolve_none: crate::vk1_0::Bool32,
    pub independent_resolve: crate::vk1_0::Bool32,
    pub filter_minmax_single_component_formats: crate::vk1_0::Bool32,
    pub filter_minmax_image_component_mapping: crate::vk1_0::Bool32,
    pub max_timeline_semaphore_value_difference: u64,
    pub framebuffer_integer_color_sample_counts: crate::vk1_0::SampleCountFlags,
}
impl PhysicalDeviceVulkan12Properties {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceVulkan12Properties,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceVulkan12PropertiesBuilder<'a> {
        PhysicalDeviceVulkan12PropertiesBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceVulkan12Properties {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceVulkan12Properties")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("driver_id", &self.driver_id)
            .field("driver_name", &unsafe {
                std::ffi::CStr::from_ptr(self.driver_name.as_ptr() as _)
            })
            .field("driver_info", &unsafe {
                std::ffi::CStr::from_ptr(self.driver_info.as_ptr() as _)
            })
            .field("conformance_version", &self.conformance_version)
            .field(
                "denorm_behavior_independence",
                &self.denorm_behavior_independence,
            )
            .field(
                "rounding_mode_independence",
                &self.rounding_mode_independence,
            )
            .field(
                "shader_signed_zero_inf_nan_preserve_float16",
                &(self.shader_signed_zero_inf_nan_preserve_float16 != 0),
            )
            .field(
                "shader_signed_zero_inf_nan_preserve_float32",
                &(self.shader_signed_zero_inf_nan_preserve_float32 != 0),
            )
            .field(
                "shader_signed_zero_inf_nan_preserve_float64",
                &(self.shader_signed_zero_inf_nan_preserve_float64 != 0),
            )
            .field(
                "shader_denorm_preserve_float16",
                &(self.shader_denorm_preserve_float16 != 0),
            )
            .field(
                "shader_denorm_preserve_float32",
                &(self.shader_denorm_preserve_float32 != 0),
            )
            .field(
                "shader_denorm_preserve_float64",
                &(self.shader_denorm_preserve_float64 != 0),
            )
            .field(
                "shader_denorm_flush_to_zero_float16",
                &(self.shader_denorm_flush_to_zero_float16 != 0),
            )
            .field(
                "shader_denorm_flush_to_zero_float32",
                &(self.shader_denorm_flush_to_zero_float32 != 0),
            )
            .field(
                "shader_denorm_flush_to_zero_float64",
                &(self.shader_denorm_flush_to_zero_float64 != 0),
            )
            .field(
                "shader_rounding_mode_rte_float16",
                &(self.shader_rounding_mode_rte_float16 != 0),
            )
            .field(
                "shader_rounding_mode_rte_float32",
                &(self.shader_rounding_mode_rte_float32 != 0),
            )
            .field(
                "shader_rounding_mode_rte_float64",
                &(self.shader_rounding_mode_rte_float64 != 0),
            )
            .field(
                "shader_rounding_mode_rtz_float16",
                &(self.shader_rounding_mode_rtz_float16 != 0),
            )
            .field(
                "shader_rounding_mode_rtz_float32",
                &(self.shader_rounding_mode_rtz_float32 != 0),
            )
            .field(
                "shader_rounding_mode_rtz_float64",
                &(self.shader_rounding_mode_rtz_float64 != 0),
            )
            .field(
                "max_update_after_bind_descriptors_in_all_pools",
                &self.max_update_after_bind_descriptors_in_all_pools,
            )
            .field(
                "shader_uniform_buffer_array_non_uniform_indexing_native",
                &(self.shader_uniform_buffer_array_non_uniform_indexing_native != 0),
            )
            .field(
                "shader_sampled_image_array_non_uniform_indexing_native",
                &(self.shader_sampled_image_array_non_uniform_indexing_native != 0),
            )
            .field(
                "shader_storage_buffer_array_non_uniform_indexing_native",
                &(self.shader_storage_buffer_array_non_uniform_indexing_native != 0),
            )
            .field(
                "shader_storage_image_array_non_uniform_indexing_native",
                &(self.shader_storage_image_array_non_uniform_indexing_native != 0),
            )
            .field(
                "shader_input_attachment_array_non_uniform_indexing_native",
                &(self.shader_input_attachment_array_non_uniform_indexing_native != 0),
            )
            .field(
                "robust_buffer_access_update_after_bind",
                &(self.robust_buffer_access_update_after_bind != 0),
            )
            .field(
                "quad_divergent_implicit_lod",
                &(self.quad_divergent_implicit_lod != 0),
            )
            .field(
                "max_per_stage_descriptor_update_after_bind_samplers",
                &self.max_per_stage_descriptor_update_after_bind_samplers,
            )
            .field(
                "max_per_stage_descriptor_update_after_bind_uniform_buffers",
                &self.max_per_stage_descriptor_update_after_bind_uniform_buffers,
            )
            .field(
                "max_per_stage_descriptor_update_after_bind_storage_buffers",
                &self.max_per_stage_descriptor_update_after_bind_storage_buffers,
            )
            .field(
                "max_per_stage_descriptor_update_after_bind_sampled_images",
                &self.max_per_stage_descriptor_update_after_bind_sampled_images,
            )
            .field(
                "max_per_stage_descriptor_update_after_bind_storage_images",
                &self.max_per_stage_descriptor_update_after_bind_storage_images,
            )
            .field(
                "max_per_stage_descriptor_update_after_bind_input_attachments",
                &self.max_per_stage_descriptor_update_after_bind_input_attachments,
            )
            .field(
                "max_per_stage_update_after_bind_resources",
                &self.max_per_stage_update_after_bind_resources,
            )
            .field(
                "max_descriptor_set_update_after_bind_samplers",
                &self.max_descriptor_set_update_after_bind_samplers,
            )
            .field(
                "max_descriptor_set_update_after_bind_uniform_buffers",
                &self.max_descriptor_set_update_after_bind_uniform_buffers,
            )
            .field(
                "max_descriptor_set_update_after_bind_uniform_buffers_dynamic",
                &self.max_descriptor_set_update_after_bind_uniform_buffers_dynamic,
            )
            .field(
                "max_descriptor_set_update_after_bind_storage_buffers",
                &self.max_descriptor_set_update_after_bind_storage_buffers,
            )
            .field(
                "max_descriptor_set_update_after_bind_storage_buffers_dynamic",
                &self.max_descriptor_set_update_after_bind_storage_buffers_dynamic,
            )
            .field(
                "max_descriptor_set_update_after_bind_sampled_images",
                &self.max_descriptor_set_update_after_bind_sampled_images,
            )
            .field(
                "max_descriptor_set_update_after_bind_storage_images",
                &self.max_descriptor_set_update_after_bind_storage_images,
            )
            .field(
                "max_descriptor_set_update_after_bind_input_attachments",
                &self.max_descriptor_set_update_after_bind_input_attachments,
            )
            .field(
                "supported_depth_resolve_modes",
                &self.supported_depth_resolve_modes,
            )
            .field(
                "supported_stencil_resolve_modes",
                &self.supported_stencil_resolve_modes,
            )
            .field(
                "independent_resolve_none",
                &(self.independent_resolve_none != 0),
            )
            .field("independent_resolve", &(self.independent_resolve != 0))
            .field(
                "filter_minmax_single_component_formats",
                &(self.filter_minmax_single_component_formats != 0),
            )
            .field(
                "filter_minmax_image_component_mapping",
                &(self.filter_minmax_image_component_mapping != 0),
            )
            .field(
                "max_timeline_semaphore_value_difference",
                &self.max_timeline_semaphore_value_difference,
            )
            .field(
                "framebuffer_integer_color_sample_counts",
                &self.framebuffer_integer_color_sample_counts,
            )
            .finish()
    }
}
impl Default for PhysicalDeviceVulkan12Properties {
    fn default() -> PhysicalDeviceVulkan12Properties {
        PhysicalDeviceVulkan12Properties {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES,
            p_next: std::ptr::null_mut(),
            driver_id: Default::default(),
            driver_name: unsafe { std::mem::zeroed() },
            driver_info: unsafe { std::mem::zeroed() },
            conformance_version: Default::default(),
            denorm_behavior_independence: Default::default(),
            rounding_mode_independence: Default::default(),
            shader_signed_zero_inf_nan_preserve_float16: Default::default(),
            shader_signed_zero_inf_nan_preserve_float32: Default::default(),
            shader_signed_zero_inf_nan_preserve_float64: Default::default(),
            shader_denorm_preserve_float16: Default::default(),
            shader_denorm_preserve_float32: Default::default(),
            shader_denorm_preserve_float64: Default::default(),
            shader_denorm_flush_to_zero_float16: Default::default(),
            shader_denorm_flush_to_zero_float32: Default::default(),
            shader_denorm_flush_to_zero_float64: Default::default(),
            shader_rounding_mode_rte_float16: Default::default(),
            shader_rounding_mode_rte_float32: Default::default(),
            shader_rounding_mode_rte_float64: Default::default(),
            shader_rounding_mode_rtz_float16: Default::default(),
            shader_rounding_mode_rtz_float32: Default::default(),
            shader_rounding_mode_rtz_float64: Default::default(),
            max_update_after_bind_descriptors_in_all_pools: Default::default(),
            shader_uniform_buffer_array_non_uniform_indexing_native: Default::default(),
            shader_sampled_image_array_non_uniform_indexing_native: Default::default(),
            shader_storage_buffer_array_non_uniform_indexing_native: Default::default(),
            shader_storage_image_array_non_uniform_indexing_native: Default::default(),
            shader_input_attachment_array_non_uniform_indexing_native: Default::default(),
            robust_buffer_access_update_after_bind: Default::default(),
            quad_divergent_implicit_lod: Default::default(),
            max_per_stage_descriptor_update_after_bind_samplers: Default::default(),
            max_per_stage_descriptor_update_after_bind_uniform_buffers: Default::default(),
            max_per_stage_descriptor_update_after_bind_storage_buffers: Default::default(),
            max_per_stage_descriptor_update_after_bind_sampled_images: Default::default(),
            max_per_stage_descriptor_update_after_bind_storage_images: Default::default(),
            max_per_stage_descriptor_update_after_bind_input_attachments: Default::default(),
            max_per_stage_update_after_bind_resources: Default::default(),
            max_descriptor_set_update_after_bind_samplers: Default::default(),
            max_descriptor_set_update_after_bind_uniform_buffers: Default::default(),
            max_descriptor_set_update_after_bind_uniform_buffers_dynamic: Default::default(),
            max_descriptor_set_update_after_bind_storage_buffers: Default::default(),
            max_descriptor_set_update_after_bind_storage_buffers_dynamic: Default::default(),
            max_descriptor_set_update_after_bind_sampled_images: Default::default(),
            max_descriptor_set_update_after_bind_storage_images: Default::default(),
            max_descriptor_set_update_after_bind_input_attachments: Default::default(),
            supported_depth_resolve_modes: Default::default(),
            supported_stencil_resolve_modes: Default::default(),
            independent_resolve_none: Default::default(),
            independent_resolve: Default::default(),
            filter_minmax_single_component_formats: Default::default(),
            filter_minmax_image_component_mapping: Default::default(),
            max_timeline_semaphore_value_difference: Default::default(),
            framebuffer_integer_color_sample_counts: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceVulkan12Properties::extend`](struct.PhysicalDeviceVulkan12Properties.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceVulkan12Properties {}
impl ExtendableByPhysicalDeviceVulkan12Properties for crate::vk1_1::PhysicalDeviceProperties2 {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceVulkan12Properties`](struct.PhysicalDeviceVulkan12Properties.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceVulkan12PropertiesBuilder<'a>(
    PhysicalDeviceVulkan12Properties,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceVulkan12PropertiesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceVulkan12PropertiesBuilder<'a> {
        PhysicalDeviceVulkan12PropertiesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn driver_id(mut self, driver_id: crate::vk1_2::DriverId) -> Self {
        self.0.driver_id = driver_id;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn driver_name(
        mut self,
        driver_name: [std::os::raw::c_char; crate::vk1_2::MAX_DRIVER_NAME_SIZE as usize],
    ) -> Self {
        self.0.driver_name = driver_name;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn driver_info(
        mut self,
        driver_info: [std::os::raw::c_char; crate::vk1_2::MAX_DRIVER_INFO_SIZE as usize],
    ) -> Self {
        self.0.driver_info = driver_info;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn conformance_version(
        mut self,
        conformance_version: crate::vk1_2::ConformanceVersion,
    ) -> Self {
        self.0.conformance_version = conformance_version;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn denorm_behavior_independence(
        mut self,
        denorm_behavior_independence: crate::vk1_2::ShaderFloatControlsIndependence,
    ) -> Self {
        self.0.denorm_behavior_independence = denorm_behavior_independence;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn rounding_mode_independence(
        mut self,
        rounding_mode_independence: crate::vk1_2::ShaderFloatControlsIndependence,
    ) -> Self {
        self.0.rounding_mode_independence = rounding_mode_independence;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_signed_zero_inf_nan_preserve_float16(
        mut self,
        shader_signed_zero_inf_nan_preserve_float16: bool,
    ) -> Self {
        self.0.shader_signed_zero_inf_nan_preserve_float16 =
            shader_signed_zero_inf_nan_preserve_float16 as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_signed_zero_inf_nan_preserve_float32(
        mut self,
        shader_signed_zero_inf_nan_preserve_float32: bool,
    ) -> Self {
        self.0.shader_signed_zero_inf_nan_preserve_float32 =
            shader_signed_zero_inf_nan_preserve_float32 as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_signed_zero_inf_nan_preserve_float64(
        mut self,
        shader_signed_zero_inf_nan_preserve_float64: bool,
    ) -> Self {
        self.0.shader_signed_zero_inf_nan_preserve_float64 =
            shader_signed_zero_inf_nan_preserve_float64 as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_denorm_preserve_float16(mut self, shader_denorm_preserve_float16: bool) -> Self {
        self.0.shader_denorm_preserve_float16 = shader_denorm_preserve_float16 as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_denorm_preserve_float32(mut self, shader_denorm_preserve_float32: bool) -> Self {
        self.0.shader_denorm_preserve_float32 = shader_denorm_preserve_float32 as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_denorm_preserve_float64(mut self, shader_denorm_preserve_float64: bool) -> Self {
        self.0.shader_denorm_preserve_float64 = shader_denorm_preserve_float64 as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_denorm_flush_to_zero_float16(
        mut self,
        shader_denorm_flush_to_zero_float16: bool,
    ) -> Self {
        self.0.shader_denorm_flush_to_zero_float16 = shader_denorm_flush_to_zero_float16 as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_denorm_flush_to_zero_float32(
        mut self,
        shader_denorm_flush_to_zero_float32: bool,
    ) -> Self {
        self.0.shader_denorm_flush_to_zero_float32 = shader_denorm_flush_to_zero_float32 as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_denorm_flush_to_zero_float64(
        mut self,
        shader_denorm_flush_to_zero_float64: bool,
    ) -> Self {
        self.0.shader_denorm_flush_to_zero_float64 = shader_denorm_flush_to_zero_float64 as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_rounding_mode_rte_float16(
        mut self,
        shader_rounding_mode_rte_float16: bool,
    ) -> Self {
        self.0.shader_rounding_mode_rte_float16 = shader_rounding_mode_rte_float16 as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_rounding_mode_rte_float32(
        mut self,
        shader_rounding_mode_rte_float32: bool,
    ) -> Self {
        self.0.shader_rounding_mode_rte_float32 = shader_rounding_mode_rte_float32 as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_rounding_mode_rte_float64(
        mut self,
        shader_rounding_mode_rte_float64: bool,
    ) -> Self {
        self.0.shader_rounding_mode_rte_float64 = shader_rounding_mode_rte_float64 as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_rounding_mode_rtz_float16(
        mut self,
        shader_rounding_mode_rtz_float16: bool,
    ) -> Self {
        self.0.shader_rounding_mode_rtz_float16 = shader_rounding_mode_rtz_float16 as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_rounding_mode_rtz_float32(
        mut self,
        shader_rounding_mode_rtz_float32: bool,
    ) -> Self {
        self.0.shader_rounding_mode_rtz_float32 = shader_rounding_mode_rtz_float32 as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_rounding_mode_rtz_float64(
        mut self,
        shader_rounding_mode_rtz_float64: bool,
    ) -> Self {
        self.0.shader_rounding_mode_rtz_float64 = shader_rounding_mode_rtz_float64 as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_update_after_bind_descriptors_in_all_pools(
        mut self,
        max_update_after_bind_descriptors_in_all_pools: u32,
    ) -> Self {
        self.0.max_update_after_bind_descriptors_in_all_pools =
            max_update_after_bind_descriptors_in_all_pools;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_uniform_buffer_array_non_uniform_indexing_native(
        mut self,
        shader_uniform_buffer_array_non_uniform_indexing_native: bool,
    ) -> Self {
        self.0
            .shader_uniform_buffer_array_non_uniform_indexing_native =
            shader_uniform_buffer_array_non_uniform_indexing_native as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_sampled_image_array_non_uniform_indexing_native(
        mut self,
        shader_sampled_image_array_non_uniform_indexing_native: bool,
    ) -> Self {
        self.0
            .shader_sampled_image_array_non_uniform_indexing_native =
            shader_sampled_image_array_non_uniform_indexing_native as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_storage_buffer_array_non_uniform_indexing_native(
        mut self,
        shader_storage_buffer_array_non_uniform_indexing_native: bool,
    ) -> Self {
        self.0
            .shader_storage_buffer_array_non_uniform_indexing_native =
            shader_storage_buffer_array_non_uniform_indexing_native as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_storage_image_array_non_uniform_indexing_native(
        mut self,
        shader_storage_image_array_non_uniform_indexing_native: bool,
    ) -> Self {
        self.0
            .shader_storage_image_array_non_uniform_indexing_native =
            shader_storage_image_array_non_uniform_indexing_native as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_input_attachment_array_non_uniform_indexing_native(
        mut self,
        shader_input_attachment_array_non_uniform_indexing_native: bool,
    ) -> Self {
        self.0
            .shader_input_attachment_array_non_uniform_indexing_native =
            shader_input_attachment_array_non_uniform_indexing_native as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn robust_buffer_access_update_after_bind(
        mut self,
        robust_buffer_access_update_after_bind: bool,
    ) -> Self {
        self.0.robust_buffer_access_update_after_bind =
            robust_buffer_access_update_after_bind as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn quad_divergent_implicit_lod(mut self, quad_divergent_implicit_lod: bool) -> Self {
        self.0.quad_divergent_implicit_lod = quad_divergent_implicit_lod as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_per_stage_descriptor_update_after_bind_samplers(
        mut self,
        max_per_stage_descriptor_update_after_bind_samplers: u32,
    ) -> Self {
        self.0.max_per_stage_descriptor_update_after_bind_samplers =
            max_per_stage_descriptor_update_after_bind_samplers;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_per_stage_descriptor_update_after_bind_uniform_buffers(
        mut self,
        max_per_stage_descriptor_update_after_bind_uniform_buffers: u32,
    ) -> Self {
        self.0
            .max_per_stage_descriptor_update_after_bind_uniform_buffers =
            max_per_stage_descriptor_update_after_bind_uniform_buffers;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_per_stage_descriptor_update_after_bind_storage_buffers(
        mut self,
        max_per_stage_descriptor_update_after_bind_storage_buffers: u32,
    ) -> Self {
        self.0
            .max_per_stage_descriptor_update_after_bind_storage_buffers =
            max_per_stage_descriptor_update_after_bind_storage_buffers;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_per_stage_descriptor_update_after_bind_sampled_images(
        mut self,
        max_per_stage_descriptor_update_after_bind_sampled_images: u32,
    ) -> Self {
        self.0
            .max_per_stage_descriptor_update_after_bind_sampled_images =
            max_per_stage_descriptor_update_after_bind_sampled_images;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_per_stage_descriptor_update_after_bind_storage_images(
        mut self,
        max_per_stage_descriptor_update_after_bind_storage_images: u32,
    ) -> Self {
        self.0
            .max_per_stage_descriptor_update_after_bind_storage_images =
            max_per_stage_descriptor_update_after_bind_storage_images;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_per_stage_descriptor_update_after_bind_input_attachments(
        mut self,
        max_per_stage_descriptor_update_after_bind_input_attachments: u32,
    ) -> Self {
        self.0
            .max_per_stage_descriptor_update_after_bind_input_attachments =
            max_per_stage_descriptor_update_after_bind_input_attachments;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_per_stage_update_after_bind_resources(
        mut self,
        max_per_stage_update_after_bind_resources: u32,
    ) -> Self {
        self.0.max_per_stage_update_after_bind_resources =
            max_per_stage_update_after_bind_resources;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_descriptor_set_update_after_bind_samplers(
        mut self,
        max_descriptor_set_update_after_bind_samplers: u32,
    ) -> Self {
        self.0.max_descriptor_set_update_after_bind_samplers =
            max_descriptor_set_update_after_bind_samplers;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_descriptor_set_update_after_bind_uniform_buffers(
        mut self,
        max_descriptor_set_update_after_bind_uniform_buffers: u32,
    ) -> Self {
        self.0.max_descriptor_set_update_after_bind_uniform_buffers =
            max_descriptor_set_update_after_bind_uniform_buffers;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_descriptor_set_update_after_bind_uniform_buffers_dynamic(
        mut self,
        max_descriptor_set_update_after_bind_uniform_buffers_dynamic: u32,
    ) -> Self {
        self.0
            .max_descriptor_set_update_after_bind_uniform_buffers_dynamic =
            max_descriptor_set_update_after_bind_uniform_buffers_dynamic;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_descriptor_set_update_after_bind_storage_buffers(
        mut self,
        max_descriptor_set_update_after_bind_storage_buffers: u32,
    ) -> Self {
        self.0.max_descriptor_set_update_after_bind_storage_buffers =
            max_descriptor_set_update_after_bind_storage_buffers;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_descriptor_set_update_after_bind_storage_buffers_dynamic(
        mut self,
        max_descriptor_set_update_after_bind_storage_buffers_dynamic: u32,
    ) -> Self {
        self.0
            .max_descriptor_set_update_after_bind_storage_buffers_dynamic =
            max_descriptor_set_update_after_bind_storage_buffers_dynamic;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_descriptor_set_update_after_bind_sampled_images(
        mut self,
        max_descriptor_set_update_after_bind_sampled_images: u32,
    ) -> Self {
        self.0.max_descriptor_set_update_after_bind_sampled_images =
            max_descriptor_set_update_after_bind_sampled_images;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_descriptor_set_update_after_bind_storage_images(
        mut self,
        max_descriptor_set_update_after_bind_storage_images: u32,
    ) -> Self {
        self.0.max_descriptor_set_update_after_bind_storage_images =
            max_descriptor_set_update_after_bind_storage_images;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_descriptor_set_update_after_bind_input_attachments(
        mut self,
        max_descriptor_set_update_after_bind_input_attachments: u32,
    ) -> Self {
        self.0
            .max_descriptor_set_update_after_bind_input_attachments =
            max_descriptor_set_update_after_bind_input_attachments;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn supported_depth_resolve_modes(
        mut self,
        supported_depth_resolve_modes: crate::vk1_2::ResolveModeFlags,
    ) -> Self {
        self.0.supported_depth_resolve_modes = supported_depth_resolve_modes;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn supported_stencil_resolve_modes(
        mut self,
        supported_stencil_resolve_modes: crate::vk1_2::ResolveModeFlags,
    ) -> Self {
        self.0.supported_stencil_resolve_modes = supported_stencil_resolve_modes;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn independent_resolve_none(mut self, independent_resolve_none: bool) -> Self {
        self.0.independent_resolve_none = independent_resolve_none as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn independent_resolve(mut self, independent_resolve: bool) -> Self {
        self.0.independent_resolve = independent_resolve as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn filter_minmax_single_component_formats(
        mut self,
        filter_minmax_single_component_formats: bool,
    ) -> Self {
        self.0.filter_minmax_single_component_formats =
            filter_minmax_single_component_formats as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn filter_minmax_image_component_mapping(
        mut self,
        filter_minmax_image_component_mapping: bool,
    ) -> Self {
        self.0.filter_minmax_image_component_mapping = filter_minmax_image_component_mapping as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_timeline_semaphore_value_difference(
        mut self,
        max_timeline_semaphore_value_difference: u64,
    ) -> Self {
        self.0.max_timeline_semaphore_value_difference = max_timeline_semaphore_value_difference;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn framebuffer_integer_color_sample_counts(
        mut self,
        framebuffer_integer_color_sample_counts: crate::vk1_0::SampleCountFlags,
    ) -> Self {
        self.0.framebuffer_integer_color_sample_counts = framebuffer_integer_color_sample_counts;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceVulkan12Properties {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceVulkan12PropertiesBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceVulkan12PropertiesBuilder<'a> {
    type Target = PhysicalDeviceVulkan12Properties;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceVulkan12PropertiesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDriverId.html) · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct DriverId(pub i32);
#[doc = "[Part of `vk1_2`](../vk1_2/index.html)"]
impl DriverId {
    pub const AMD_PROPRIETARY: Self = Self(1);
    pub const AMD_OPEN_SOURCE: Self = Self(2);
    pub const MESA_RADV: Self = Self(3);
    pub const NVIDIA_PROPRIETARY: Self = Self(4);
    pub const INTEL_PROPRIETARY_WINDOWS: Self = Self(5);
    pub const INTEL_OPEN_SOURCE_MESA: Self = Self(6);
    pub const IMAGINATION_PROPRIETARY: Self = Self(7);
    pub const QUALCOMM_PROPRIETARY: Self = Self(8);
    pub const ARM_PROPRIETARY: Self = Self(9);
    pub const GOOGLE_SWIFTSHADER: Self = Self(10);
    pub const GGP_PROPRIETARY: Self = Self(11);
    pub const BROADCOM_PROPRIETARY: Self = Self(12);
}
#[doc = "[Part of `extensions::khr_driver_properties`](../extensions/khr_driver_properties/index.html)"]
impl DriverId {
    pub const AMD_PROPRIETARY_KHR: Self = Self::AMD_PROPRIETARY;
    pub const AMD_OPEN_SOURCE_KHR: Self = Self::AMD_OPEN_SOURCE;
    pub const MESA_RADV_KHR: Self = Self::MESA_RADV;
    pub const NVIDIA_PROPRIETARY_KHR: Self = Self::NVIDIA_PROPRIETARY;
    pub const INTEL_PROPRIETARY_WINDOWS_KHR: Self = Self::INTEL_PROPRIETARY_WINDOWS;
    pub const INTEL_OPEN_SOURCE_MESA_KHR: Self = Self::INTEL_OPEN_SOURCE_MESA;
    pub const IMAGINATION_PROPRIETARY_KHR: Self = Self::IMAGINATION_PROPRIETARY;
    pub const QUALCOMM_PROPRIETARY_KHR: Self = Self::QUALCOMM_PROPRIETARY;
    pub const ARM_PROPRIETARY_KHR: Self = Self::ARM_PROPRIETARY;
    pub const GOOGLE_SWIFTSHADER_KHR: Self = Self::GOOGLE_SWIFTSHADER;
    pub const GGP_PROPRIETARY_KHR: Self = Self::GGP_PROPRIETARY;
    pub const BROADCOM_PROPRIETARY_KHR: Self = Self::BROADCOM_PROPRIETARY;
}
impl std::fmt::Debug for DriverId {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::AMD_PROPRIETARY => "AMD_PROPRIETARY",
            &Self::AMD_OPEN_SOURCE => "AMD_OPEN_SOURCE",
            &Self::MESA_RADV => "MESA_RADV",
            &Self::NVIDIA_PROPRIETARY => "NVIDIA_PROPRIETARY",
            &Self::INTEL_PROPRIETARY_WINDOWS => "INTEL_PROPRIETARY_WINDOWS",
            &Self::INTEL_OPEN_SOURCE_MESA => "INTEL_OPEN_SOURCE_MESA",
            &Self::IMAGINATION_PROPRIETARY => "IMAGINATION_PROPRIETARY",
            &Self::QUALCOMM_PROPRIETARY => "QUALCOMM_PROPRIETARY",
            &Self::ARM_PROPRIETARY => "ARM_PROPRIETARY",
            &Self::GOOGLE_SWIFTSHADER => "GOOGLE_SWIFTSHADER",
            &Self::GGP_PROPRIETARY => "GGP_PROPRIETARY",
            &Self::BROADCOM_PROPRIETARY => "BROADCOM_PROPRIETARY",
            _ => "Unknown enum variant",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkConformanceVersion.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ConformanceVersion {
    pub major: u8,
    pub minor: u8,
    pub subminor: u8,
    pub patch: u8,
}
impl ConformanceVersion {
    #[inline]
    pub fn builder<'a>(self) -> ConformanceVersionBuilder<'a> {
        ConformanceVersionBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ConformanceVersion {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ConformanceVersion")
            .field("major", &self.major)
            .field("minor", &self.minor)
            .field("subminor", &self.subminor)
            .field("patch", &self.patch)
            .finish()
    }
}
impl Default for ConformanceVersion {
    fn default() -> ConformanceVersion {
        ConformanceVersion {
            major: Default::default(),
            minor: Default::default(),
            subminor: Default::default(),
            patch: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`ConformanceVersion`](struct.ConformanceVersion.html)"]
#[repr(transparent)]
pub struct ConformanceVersionBuilder<'a>(ConformanceVersion, std::marker::PhantomData<&'a ()>);
impl<'a> ConformanceVersionBuilder<'a> {
    #[inline]
    pub fn new() -> ConformanceVersionBuilder<'a> {
        ConformanceVersionBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn major(mut self, major: u8) -> Self {
        self.0.major = major;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn minor(mut self, minor: u8) -> Self {
        self.0.minor = minor;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn subminor(mut self, subminor: u8) -> Self {
        self.0.subminor = subminor;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn patch(mut self, patch: u8) -> Self {
        self.0.patch = patch;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> ConformanceVersion {
        self.0
    }
}
impl<'a> std::fmt::Debug for ConformanceVersionBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for ConformanceVersionBuilder<'a> {
    type Target = ConformanceVersion;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ConformanceVersionBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkShaderFloatControlsIndependence.html) · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ShaderFloatControlsIndependence(pub i32);
#[doc = "[Part of `vk1_2`](../vk1_2/index.html)"]
impl ShaderFloatControlsIndependence {
    pub const _32_ONLY: Self = Self(0);
    pub const ALL: Self = Self(1);
    pub const NONE: Self = Self(2);
}
#[doc = "[Part of `extensions::khr_shader_float_controls`](../extensions/khr_shader_float_controls/index.html)"]
impl ShaderFloatControlsIndependence {
    pub const _32_ONLY_KHR: Self = Self::_32_ONLY;
    pub const ALL_KHR: Self = Self::ALL;
    pub const NONE_KHR: Self = Self::NONE;
}
impl std::fmt::Debug for ShaderFloatControlsIndependence {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::_32_ONLY => "32_ONLY",
            &Self::ALL => "ALL",
            &Self::NONE => "NONE",
            _ => "Unknown enum variant",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkResolveModeFlagBits.html) · Flag Bits of [`ResolveModeFlags`](struct.ResolveModeFlags.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ResolveModeFlagBits(pub u32);
impl ResolveModeFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> ResolveModeFlags {
        ResolveModeFlags::from_bits_truncate(self.0)
    }
}
#[doc = "[Part of `vk1_2`](../vk1_2/index.html)"]
impl ResolveModeFlagBits {
    pub const NONE: Self = Self(0);
    pub const SAMPLE_ZERO: Self = Self(0x00000001);
    pub const AVERAGE: Self = Self(0x00000002);
    pub const MIN: Self = Self(0x00000004);
    pub const MAX: Self = Self(0x00000008);
}
#[doc = "[Part of `extensions::khr_depth_stencil_resolve`](../extensions/khr_depth_stencil_resolve/index.html)"]
impl ResolveModeFlagBits {
    pub const NONE_KHR: Self = Self::NONE;
    pub const SAMPLE_ZERO_KHR: Self = Self::SAMPLE_ZERO;
    pub const AVERAGE_KHR: Self = Self::AVERAGE;
    pub const MIN_KHR: Self = Self::MIN;
    pub const MAX_KHR: Self = Self::MAX;
}
impl std::fmt::Debug for ResolveModeFlagBits {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::NONE => "NONE",
            &Self::SAMPLE_ZERO => "SAMPLE_ZERO",
            &Self::AVERAGE => "AVERAGE",
            &Self::MIN => "MIN",
            &Self::MAX => "MAX",
            _ => "Unknown enum variant",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkResolveModeFlags.html) · Flags of [`ResolveModeFlagBits`](struct.ResolveModeFlagBits.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct ResolveModeFlags : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; const NONE = ResolveModeFlagBits :: NONE . 0 ; const SAMPLE_ZERO = ResolveModeFlagBits :: SAMPLE_ZERO . 0 ; const AVERAGE = ResolveModeFlagBits :: AVERAGE . 0 ; const MIN = ResolveModeFlagBits :: MIN . 0 ; const MAX = ResolveModeFlagBits :: MAX . 0 ; const NONE_KHR = ResolveModeFlagBits :: NONE_KHR . 0 ; const SAMPLE_ZERO_KHR = ResolveModeFlagBits :: SAMPLE_ZERO_KHR . 0 ; const AVERAGE_KHR = ResolveModeFlagBits :: AVERAGE_KHR . 0 ; const MIN_KHR = ResolveModeFlagBits :: MIN_KHR . 0 ; const MAX_KHR = ResolveModeFlagBits :: MAX_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageFormatListCreateInfo.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImageFormatListCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub view_format_count: u32,
    pub p_view_formats: *const crate::vk1_0::Format,
}
impl ImageFormatListCreateInfo {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByImageFormatListCreateInfo,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> ImageFormatListCreateInfoBuilder<'a> {
        ImageFormatListCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ImageFormatListCreateInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ImageFormatListCreateInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("view_format_count", &self.view_format_count)
            .field("p_view_formats", &self.p_view_formats)
            .finish()
    }
}
impl Default for ImageFormatListCreateInfo {
    fn default() -> ImageFormatListCreateInfo {
        ImageFormatListCreateInfo {
            s_type: crate::vk1_0::StructureType::IMAGE_FORMAT_LIST_CREATE_INFO,
            p_next: std::ptr::null(),
            view_format_count: Default::default(),
            p_view_formats: std::ptr::null(),
        }
    }
}
#[doc = "Used by [`ImageFormatListCreateInfo::extend`](struct.ImageFormatListCreateInfo.html#method.extend)"]
pub trait ExtendableByImageFormatListCreateInfo {}
impl ExtendableByImageFormatListCreateInfo for crate::vk1_0::ImageCreateInfo {}
impl ExtendableByImageFormatListCreateInfo
    for crate::extensions::khr_swapchain::SwapchainCreateInfoKHR
{
}
impl ExtendableByImageFormatListCreateInfo for crate::vk1_1::PhysicalDeviceImageFormatInfo2 {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`ImageFormatListCreateInfo`](struct.ImageFormatListCreateInfo.html)"]
#[repr(transparent)]
pub struct ImageFormatListCreateInfoBuilder<'a>(
    ImageFormatListCreateInfo,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> ImageFormatListCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> ImageFormatListCreateInfoBuilder<'a> {
        ImageFormatListCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn view_formats(mut self, view_formats: &'a [crate::vk1_0::Format]) -> Self {
        self.0.view_format_count = view_formats.len() as _;
        self.0.p_view_formats = view_formats.as_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> ImageFormatListCreateInfo {
        self.0
    }
}
impl<'a> std::fmt::Debug for ImageFormatListCreateInfoBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for ImageFormatListCreateInfoBuilder<'a> {
    type Target = ImageFormatListCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ImageFormatListCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevice8BitStorageFeatures.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDevice8BitStorageFeatures {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub storage_buffer8_bit_access: crate::vk1_0::Bool32,
    pub uniform_and_storage_buffer8_bit_access: crate::vk1_0::Bool32,
    pub storage_push_constant8: crate::vk1_0::Bool32,
}
impl PhysicalDevice8BitStorageFeatures {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDevice8BitStorageFeatures,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDevice8BitStorageFeaturesBuilder<'a> {
        PhysicalDevice8BitStorageFeaturesBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDevice8BitStorageFeatures {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDevice8BitStorageFeatures")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "storage_buffer8_bit_access",
                &(self.storage_buffer8_bit_access != 0),
            )
            .field(
                "uniform_and_storage_buffer8_bit_access",
                &(self.uniform_and_storage_buffer8_bit_access != 0),
            )
            .field(
                "storage_push_constant8",
                &(self.storage_push_constant8 != 0),
            )
            .finish()
    }
}
impl Default for PhysicalDevice8BitStorageFeatures {
    fn default() -> PhysicalDevice8BitStorageFeatures {
        PhysicalDevice8BitStorageFeatures {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES,
            p_next: std::ptr::null_mut(),
            storage_buffer8_bit_access: Default::default(),
            uniform_and_storage_buffer8_bit_access: Default::default(),
            storage_push_constant8: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDevice8BitStorageFeatures::extend`](struct.PhysicalDevice8BitStorageFeatures.html#method.extend)"]
pub trait ExtendableByPhysicalDevice8BitStorageFeatures {}
impl ExtendableByPhysicalDevice8BitStorageFeatures for crate::vk1_1::PhysicalDeviceFeatures2 {}
impl ExtendableByPhysicalDevice8BitStorageFeatures for crate::vk1_0::DeviceCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDevice8BitStorageFeatures`](struct.PhysicalDevice8BitStorageFeatures.html)"]
#[repr(transparent)]
pub struct PhysicalDevice8BitStorageFeaturesBuilder<'a>(
    PhysicalDevice8BitStorageFeatures,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDevice8BitStorageFeaturesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDevice8BitStorageFeaturesBuilder<'a> {
        PhysicalDevice8BitStorageFeaturesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn storage_buffer8_bit_access(mut self, storage_buffer8_bit_access: bool) -> Self {
        self.0.storage_buffer8_bit_access = storage_buffer8_bit_access as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn uniform_and_storage_buffer8_bit_access(
        mut self,
        uniform_and_storage_buffer8_bit_access: bool,
    ) -> Self {
        self.0.uniform_and_storage_buffer8_bit_access =
            uniform_and_storage_buffer8_bit_access as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn storage_push_constant8(mut self, storage_push_constant8: bool) -> Self {
        self.0.storage_push_constant8 = storage_push_constant8 as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDevice8BitStorageFeatures {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDevice8BitStorageFeaturesBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDevice8BitStorageFeaturesBuilder<'a> {
    type Target = PhysicalDevice8BitStorageFeatures;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDevice8BitStorageFeaturesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDriverProperties.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceDriverProperties {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub driver_id: crate::vk1_2::DriverId,
    pub driver_name: [std::os::raw::c_char; crate::vk1_2::MAX_DRIVER_NAME_SIZE as usize],
    pub driver_info: [std::os::raw::c_char; crate::vk1_2::MAX_DRIVER_INFO_SIZE as usize],
    pub conformance_version: crate::vk1_2::ConformanceVersion,
}
impl PhysicalDeviceDriverProperties {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceDriverProperties,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceDriverPropertiesBuilder<'a> {
        PhysicalDeviceDriverPropertiesBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceDriverProperties {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceDriverProperties")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("driver_id", &self.driver_id)
            .field("driver_name", &unsafe {
                std::ffi::CStr::from_ptr(self.driver_name.as_ptr() as _)
            })
            .field("driver_info", &unsafe {
                std::ffi::CStr::from_ptr(self.driver_info.as_ptr() as _)
            })
            .field("conformance_version", &self.conformance_version)
            .finish()
    }
}
impl Default for PhysicalDeviceDriverProperties {
    fn default() -> PhysicalDeviceDriverProperties {
        PhysicalDeviceDriverProperties {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_DRIVER_PROPERTIES,
            p_next: std::ptr::null_mut(),
            driver_id: Default::default(),
            driver_name: unsafe { std::mem::zeroed() },
            driver_info: unsafe { std::mem::zeroed() },
            conformance_version: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceDriverProperties::extend`](struct.PhysicalDeviceDriverProperties.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceDriverProperties {}
impl ExtendableByPhysicalDeviceDriverProperties for crate::vk1_1::PhysicalDeviceProperties2 {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceDriverProperties`](struct.PhysicalDeviceDriverProperties.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceDriverPropertiesBuilder<'a>(
    PhysicalDeviceDriverProperties,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceDriverPropertiesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceDriverPropertiesBuilder<'a> {
        PhysicalDeviceDriverPropertiesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn driver_id(mut self, driver_id: crate::vk1_2::DriverId) -> Self {
        self.0.driver_id = driver_id;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn driver_name(
        mut self,
        driver_name: [std::os::raw::c_char; crate::vk1_2::MAX_DRIVER_NAME_SIZE as usize],
    ) -> Self {
        self.0.driver_name = driver_name;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn driver_info(
        mut self,
        driver_info: [std::os::raw::c_char; crate::vk1_2::MAX_DRIVER_INFO_SIZE as usize],
    ) -> Self {
        self.0.driver_info = driver_info;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn conformance_version(
        mut self,
        conformance_version: crate::vk1_2::ConformanceVersion,
    ) -> Self {
        self.0.conformance_version = conformance_version;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceDriverProperties {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceDriverPropertiesBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceDriverPropertiesBuilder<'a> {
    type Target = PhysicalDeviceDriverProperties;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceDriverPropertiesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderAtomicInt64Features.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceShaderAtomicInt64Features {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub shader_buffer_int64_atomics: crate::vk1_0::Bool32,
    pub shader_shared_int64_atomics: crate::vk1_0::Bool32,
}
impl PhysicalDeviceShaderAtomicInt64Features {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceShaderAtomicInt64Features,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceShaderAtomicInt64FeaturesBuilder<'a> {
        PhysicalDeviceShaderAtomicInt64FeaturesBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceShaderAtomicInt64Features {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceShaderAtomicInt64Features")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "shader_buffer_int64_atomics",
                &(self.shader_buffer_int64_atomics != 0),
            )
            .field(
                "shader_shared_int64_atomics",
                &(self.shader_shared_int64_atomics != 0),
            )
            .finish()
    }
}
impl Default for PhysicalDeviceShaderAtomicInt64Features {
    fn default() -> PhysicalDeviceShaderAtomicInt64Features {
        PhysicalDeviceShaderAtomicInt64Features {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES,
            p_next: std::ptr::null_mut(),
            shader_buffer_int64_atomics: Default::default(),
            shader_shared_int64_atomics: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceShaderAtomicInt64Features::extend`](struct.PhysicalDeviceShaderAtomicInt64Features.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceShaderAtomicInt64Features {}
impl ExtendableByPhysicalDeviceShaderAtomicInt64Features for crate::vk1_1::PhysicalDeviceFeatures2 {}
impl ExtendableByPhysicalDeviceShaderAtomicInt64Features for crate::vk1_0::DeviceCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceShaderAtomicInt64Features`](struct.PhysicalDeviceShaderAtomicInt64Features.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceShaderAtomicInt64FeaturesBuilder<'a>(
    PhysicalDeviceShaderAtomicInt64Features,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceShaderAtomicInt64FeaturesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceShaderAtomicInt64FeaturesBuilder<'a> {
        PhysicalDeviceShaderAtomicInt64FeaturesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_buffer_int64_atomics(mut self, shader_buffer_int64_atomics: bool) -> Self {
        self.0.shader_buffer_int64_atomics = shader_buffer_int64_atomics as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_shared_int64_atomics(mut self, shader_shared_int64_atomics: bool) -> Self {
        self.0.shader_shared_int64_atomics = shader_shared_int64_atomics as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceShaderAtomicInt64Features {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceShaderAtomicInt64FeaturesBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceShaderAtomicInt64FeaturesBuilder<'a> {
    type Target = PhysicalDeviceShaderAtomicInt64Features;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceShaderAtomicInt64FeaturesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderFloat16Int8Features.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceShaderFloat16Int8Features {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub shader_float16: crate::vk1_0::Bool32,
    pub shader_int8: crate::vk1_0::Bool32,
}
impl PhysicalDeviceShaderFloat16Int8Features {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceShaderFloat16Int8Features,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceShaderFloat16Int8FeaturesBuilder<'a> {
        PhysicalDeviceShaderFloat16Int8FeaturesBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceShaderFloat16Int8Features {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceShaderFloat16Int8Features")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("shader_float16", &(self.shader_float16 != 0))
            .field("shader_int8", &(self.shader_int8 != 0))
            .finish()
    }
}
impl Default for PhysicalDeviceShaderFloat16Int8Features {
    fn default() -> PhysicalDeviceShaderFloat16Int8Features {
        PhysicalDeviceShaderFloat16Int8Features {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES,
            p_next: std::ptr::null_mut(),
            shader_float16: Default::default(),
            shader_int8: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceShaderFloat16Int8Features::extend`](struct.PhysicalDeviceShaderFloat16Int8Features.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceShaderFloat16Int8Features {}
impl ExtendableByPhysicalDeviceShaderFloat16Int8Features for crate::vk1_1::PhysicalDeviceFeatures2 {}
impl ExtendableByPhysicalDeviceShaderFloat16Int8Features for crate::vk1_0::DeviceCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceShaderFloat16Int8Features`](struct.PhysicalDeviceShaderFloat16Int8Features.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceShaderFloat16Int8FeaturesBuilder<'a>(
    PhysicalDeviceShaderFloat16Int8Features,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceShaderFloat16Int8FeaturesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceShaderFloat16Int8FeaturesBuilder<'a> {
        PhysicalDeviceShaderFloat16Int8FeaturesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_float16(mut self, shader_float16: bool) -> Self {
        self.0.shader_float16 = shader_float16 as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_int8(mut self, shader_int8: bool) -> Self {
        self.0.shader_int8 = shader_int8 as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceShaderFloat16Int8Features {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceShaderFloat16Int8FeaturesBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceShaderFloat16Int8FeaturesBuilder<'a> {
    type Target = PhysicalDeviceShaderFloat16Int8Features;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceShaderFloat16Int8FeaturesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceFloatControlsProperties.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceFloatControlsProperties {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub denorm_behavior_independence: crate::vk1_2::ShaderFloatControlsIndependence,
    pub rounding_mode_independence: crate::vk1_2::ShaderFloatControlsIndependence,
    pub shader_signed_zero_inf_nan_preserve_float16: crate::vk1_0::Bool32,
    pub shader_signed_zero_inf_nan_preserve_float32: crate::vk1_0::Bool32,
    pub shader_signed_zero_inf_nan_preserve_float64: crate::vk1_0::Bool32,
    pub shader_denorm_preserve_float16: crate::vk1_0::Bool32,
    pub shader_denorm_preserve_float32: crate::vk1_0::Bool32,
    pub shader_denorm_preserve_float64: crate::vk1_0::Bool32,
    pub shader_denorm_flush_to_zero_float16: crate::vk1_0::Bool32,
    pub shader_denorm_flush_to_zero_float32: crate::vk1_0::Bool32,
    pub shader_denorm_flush_to_zero_float64: crate::vk1_0::Bool32,
    pub shader_rounding_mode_rte_float16: crate::vk1_0::Bool32,
    pub shader_rounding_mode_rte_float32: crate::vk1_0::Bool32,
    pub shader_rounding_mode_rte_float64: crate::vk1_0::Bool32,
    pub shader_rounding_mode_rtz_float16: crate::vk1_0::Bool32,
    pub shader_rounding_mode_rtz_float32: crate::vk1_0::Bool32,
    pub shader_rounding_mode_rtz_float64: crate::vk1_0::Bool32,
}
impl PhysicalDeviceFloatControlsProperties {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceFloatControlsProperties,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceFloatControlsPropertiesBuilder<'a> {
        PhysicalDeviceFloatControlsPropertiesBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceFloatControlsProperties {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceFloatControlsProperties")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "denorm_behavior_independence",
                &self.denorm_behavior_independence,
            )
            .field(
                "rounding_mode_independence",
                &self.rounding_mode_independence,
            )
            .field(
                "shader_signed_zero_inf_nan_preserve_float16",
                &(self.shader_signed_zero_inf_nan_preserve_float16 != 0),
            )
            .field(
                "shader_signed_zero_inf_nan_preserve_float32",
                &(self.shader_signed_zero_inf_nan_preserve_float32 != 0),
            )
            .field(
                "shader_signed_zero_inf_nan_preserve_float64",
                &(self.shader_signed_zero_inf_nan_preserve_float64 != 0),
            )
            .field(
                "shader_denorm_preserve_float16",
                &(self.shader_denorm_preserve_float16 != 0),
            )
            .field(
                "shader_denorm_preserve_float32",
                &(self.shader_denorm_preserve_float32 != 0),
            )
            .field(
                "shader_denorm_preserve_float64",
                &(self.shader_denorm_preserve_float64 != 0),
            )
            .field(
                "shader_denorm_flush_to_zero_float16",
                &(self.shader_denorm_flush_to_zero_float16 != 0),
            )
            .field(
                "shader_denorm_flush_to_zero_float32",
                &(self.shader_denorm_flush_to_zero_float32 != 0),
            )
            .field(
                "shader_denorm_flush_to_zero_float64",
                &(self.shader_denorm_flush_to_zero_float64 != 0),
            )
            .field(
                "shader_rounding_mode_rte_float16",
                &(self.shader_rounding_mode_rte_float16 != 0),
            )
            .field(
                "shader_rounding_mode_rte_float32",
                &(self.shader_rounding_mode_rte_float32 != 0),
            )
            .field(
                "shader_rounding_mode_rte_float64",
                &(self.shader_rounding_mode_rte_float64 != 0),
            )
            .field(
                "shader_rounding_mode_rtz_float16",
                &(self.shader_rounding_mode_rtz_float16 != 0),
            )
            .field(
                "shader_rounding_mode_rtz_float32",
                &(self.shader_rounding_mode_rtz_float32 != 0),
            )
            .field(
                "shader_rounding_mode_rtz_float64",
                &(self.shader_rounding_mode_rtz_float64 != 0),
            )
            .finish()
    }
}
impl Default for PhysicalDeviceFloatControlsProperties {
    fn default() -> PhysicalDeviceFloatControlsProperties {
        PhysicalDeviceFloatControlsProperties {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES,
            p_next: std::ptr::null_mut(),
            denorm_behavior_independence: Default::default(),
            rounding_mode_independence: Default::default(),
            shader_signed_zero_inf_nan_preserve_float16: Default::default(),
            shader_signed_zero_inf_nan_preserve_float32: Default::default(),
            shader_signed_zero_inf_nan_preserve_float64: Default::default(),
            shader_denorm_preserve_float16: Default::default(),
            shader_denorm_preserve_float32: Default::default(),
            shader_denorm_preserve_float64: Default::default(),
            shader_denorm_flush_to_zero_float16: Default::default(),
            shader_denorm_flush_to_zero_float32: Default::default(),
            shader_denorm_flush_to_zero_float64: Default::default(),
            shader_rounding_mode_rte_float16: Default::default(),
            shader_rounding_mode_rte_float32: Default::default(),
            shader_rounding_mode_rte_float64: Default::default(),
            shader_rounding_mode_rtz_float16: Default::default(),
            shader_rounding_mode_rtz_float32: Default::default(),
            shader_rounding_mode_rtz_float64: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceFloatControlsProperties::extend`](struct.PhysicalDeviceFloatControlsProperties.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceFloatControlsProperties {}
impl ExtendableByPhysicalDeviceFloatControlsProperties for crate::vk1_1::PhysicalDeviceProperties2 {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceFloatControlsProperties`](struct.PhysicalDeviceFloatControlsProperties.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceFloatControlsPropertiesBuilder<'a>(
    PhysicalDeviceFloatControlsProperties,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceFloatControlsPropertiesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceFloatControlsPropertiesBuilder<'a> {
        PhysicalDeviceFloatControlsPropertiesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn denorm_behavior_independence(
        mut self,
        denorm_behavior_independence: crate::vk1_2::ShaderFloatControlsIndependence,
    ) -> Self {
        self.0.denorm_behavior_independence = denorm_behavior_independence;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn rounding_mode_independence(
        mut self,
        rounding_mode_independence: crate::vk1_2::ShaderFloatControlsIndependence,
    ) -> Self {
        self.0.rounding_mode_independence = rounding_mode_independence;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_signed_zero_inf_nan_preserve_float16(
        mut self,
        shader_signed_zero_inf_nan_preserve_float16: bool,
    ) -> Self {
        self.0.shader_signed_zero_inf_nan_preserve_float16 =
            shader_signed_zero_inf_nan_preserve_float16 as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_signed_zero_inf_nan_preserve_float32(
        mut self,
        shader_signed_zero_inf_nan_preserve_float32: bool,
    ) -> Self {
        self.0.shader_signed_zero_inf_nan_preserve_float32 =
            shader_signed_zero_inf_nan_preserve_float32 as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_signed_zero_inf_nan_preserve_float64(
        mut self,
        shader_signed_zero_inf_nan_preserve_float64: bool,
    ) -> Self {
        self.0.shader_signed_zero_inf_nan_preserve_float64 =
            shader_signed_zero_inf_nan_preserve_float64 as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_denorm_preserve_float16(mut self, shader_denorm_preserve_float16: bool) -> Self {
        self.0.shader_denorm_preserve_float16 = shader_denorm_preserve_float16 as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_denorm_preserve_float32(mut self, shader_denorm_preserve_float32: bool) -> Self {
        self.0.shader_denorm_preserve_float32 = shader_denorm_preserve_float32 as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_denorm_preserve_float64(mut self, shader_denorm_preserve_float64: bool) -> Self {
        self.0.shader_denorm_preserve_float64 = shader_denorm_preserve_float64 as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_denorm_flush_to_zero_float16(
        mut self,
        shader_denorm_flush_to_zero_float16: bool,
    ) -> Self {
        self.0.shader_denorm_flush_to_zero_float16 = shader_denorm_flush_to_zero_float16 as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_denorm_flush_to_zero_float32(
        mut self,
        shader_denorm_flush_to_zero_float32: bool,
    ) -> Self {
        self.0.shader_denorm_flush_to_zero_float32 = shader_denorm_flush_to_zero_float32 as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_denorm_flush_to_zero_float64(
        mut self,
        shader_denorm_flush_to_zero_float64: bool,
    ) -> Self {
        self.0.shader_denorm_flush_to_zero_float64 = shader_denorm_flush_to_zero_float64 as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_rounding_mode_rte_float16(
        mut self,
        shader_rounding_mode_rte_float16: bool,
    ) -> Self {
        self.0.shader_rounding_mode_rte_float16 = shader_rounding_mode_rte_float16 as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_rounding_mode_rte_float32(
        mut self,
        shader_rounding_mode_rte_float32: bool,
    ) -> Self {
        self.0.shader_rounding_mode_rte_float32 = shader_rounding_mode_rte_float32 as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_rounding_mode_rte_float64(
        mut self,
        shader_rounding_mode_rte_float64: bool,
    ) -> Self {
        self.0.shader_rounding_mode_rte_float64 = shader_rounding_mode_rte_float64 as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_rounding_mode_rtz_float16(
        mut self,
        shader_rounding_mode_rtz_float16: bool,
    ) -> Self {
        self.0.shader_rounding_mode_rtz_float16 = shader_rounding_mode_rtz_float16 as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_rounding_mode_rtz_float32(
        mut self,
        shader_rounding_mode_rtz_float32: bool,
    ) -> Self {
        self.0.shader_rounding_mode_rtz_float32 = shader_rounding_mode_rtz_float32 as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_rounding_mode_rtz_float64(
        mut self,
        shader_rounding_mode_rtz_float64: bool,
    ) -> Self {
        self.0.shader_rounding_mode_rtz_float64 = shader_rounding_mode_rtz_float64 as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceFloatControlsProperties {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceFloatControlsPropertiesBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceFloatControlsPropertiesBuilder<'a> {
    type Target = PhysicalDeviceFloatControlsProperties;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceFloatControlsPropertiesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorSetLayoutBindingFlagsCreateInfo.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DescriptorSetLayoutBindingFlagsCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub binding_count: u32,
    pub p_binding_flags: *const crate::vk1_2::DescriptorBindingFlags,
}
impl DescriptorSetLayoutBindingFlagsCreateInfo {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByDescriptorSetLayoutBindingFlagsCreateInfo,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> DescriptorSetLayoutBindingFlagsCreateInfoBuilder<'a> {
        DescriptorSetLayoutBindingFlagsCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for DescriptorSetLayoutBindingFlagsCreateInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("DescriptorSetLayoutBindingFlagsCreateInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("binding_count", &self.binding_count)
            .field("p_binding_flags", &self.p_binding_flags)
            .finish()
    }
}
impl Default for DescriptorSetLayoutBindingFlagsCreateInfo {
    fn default() -> DescriptorSetLayoutBindingFlagsCreateInfo {
        DescriptorSetLayoutBindingFlagsCreateInfo {
            s_type: crate::vk1_0::StructureType::DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO,
            p_next: std::ptr::null(),
            binding_count: Default::default(),
            p_binding_flags: std::ptr::null(),
        }
    }
}
#[doc = "Used by [`DescriptorSetLayoutBindingFlagsCreateInfo::extend`](struct.DescriptorSetLayoutBindingFlagsCreateInfo.html#method.extend)"]
pub trait ExtendableByDescriptorSetLayoutBindingFlagsCreateInfo {}
impl ExtendableByDescriptorSetLayoutBindingFlagsCreateInfo
    for crate::vk1_0::DescriptorSetLayoutCreateInfo
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`DescriptorSetLayoutBindingFlagsCreateInfo`](struct.DescriptorSetLayoutBindingFlagsCreateInfo.html)"]
#[repr(transparent)]
pub struct DescriptorSetLayoutBindingFlagsCreateInfoBuilder<'a>(
    DescriptorSetLayoutBindingFlagsCreateInfo,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> DescriptorSetLayoutBindingFlagsCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> DescriptorSetLayoutBindingFlagsCreateInfoBuilder<'a> {
        DescriptorSetLayoutBindingFlagsCreateInfoBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn binding_flags(
        mut self,
        binding_flags: &'a [crate::vk1_2::DescriptorBindingFlags],
    ) -> Self {
        self.0.binding_count = binding_flags.len() as _;
        self.0.p_binding_flags = binding_flags.as_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> DescriptorSetLayoutBindingFlagsCreateInfo {
        self.0
    }
}
impl<'a> std::fmt::Debug for DescriptorSetLayoutBindingFlagsCreateInfoBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for DescriptorSetLayoutBindingFlagsCreateInfoBuilder<'a> {
    type Target = DescriptorSetLayoutBindingFlagsCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DescriptorSetLayoutBindingFlagsCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorBindingFlagBits.html) · Flag Bits of [`DescriptorBindingFlags`](struct.DescriptorBindingFlags.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct DescriptorBindingFlagBits(pub u32);
impl DescriptorBindingFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> DescriptorBindingFlags {
        DescriptorBindingFlags::from_bits_truncate(self.0)
    }
}
#[doc = "[Part of `vk1_2`](../vk1_2/index.html)"]
impl DescriptorBindingFlagBits {
    pub const UPDATE_AFTER_BIND: Self = Self(0x00000001);
    pub const UPDATE_UNUSED_WHILE_PENDING: Self = Self(0x00000002);
    pub const PARTIALLY_BOUND: Self = Self(0x00000004);
    pub const VARIABLE_DESCRIPTOR_COUNT: Self = Self(0x00000008);
}
#[doc = "[Part of `extensions::ext_descriptor_indexing`](../extensions/ext_descriptor_indexing/index.html)"]
impl DescriptorBindingFlagBits {
    pub const UPDATE_AFTER_BIND_EXT: Self = Self::UPDATE_AFTER_BIND;
    pub const UPDATE_UNUSED_WHILE_PENDING_EXT: Self = Self::UPDATE_UNUSED_WHILE_PENDING;
    pub const PARTIALLY_BOUND_EXT: Self = Self::PARTIALLY_BOUND;
    pub const VARIABLE_DESCRIPTOR_COUNT_EXT: Self = Self::VARIABLE_DESCRIPTOR_COUNT;
}
impl std::fmt::Debug for DescriptorBindingFlagBits {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::UPDATE_AFTER_BIND => "UPDATE_AFTER_BIND",
            &Self::UPDATE_UNUSED_WHILE_PENDING => "UPDATE_UNUSED_WHILE_PENDING",
            &Self::PARTIALLY_BOUND => "PARTIALLY_BOUND",
            &Self::VARIABLE_DESCRIPTOR_COUNT => "VARIABLE_DESCRIPTOR_COUNT",
            _ => "Unknown enum variant",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorBindingFlags.html) · Flags of [`DescriptorBindingFlagBits`](struct.DescriptorBindingFlagBits.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct DescriptorBindingFlags : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; const UPDATE_AFTER_BIND = DescriptorBindingFlagBits :: UPDATE_AFTER_BIND . 0 ; const UPDATE_UNUSED_WHILE_PENDING = DescriptorBindingFlagBits :: UPDATE_UNUSED_WHILE_PENDING . 0 ; const PARTIALLY_BOUND = DescriptorBindingFlagBits :: PARTIALLY_BOUND . 0 ; const VARIABLE_DESCRIPTOR_COUNT = DescriptorBindingFlagBits :: VARIABLE_DESCRIPTOR_COUNT . 0 ; const UPDATE_AFTER_BIND_EXT = DescriptorBindingFlagBits :: UPDATE_AFTER_BIND_EXT . 0 ; const UPDATE_UNUSED_WHILE_PENDING_EXT = DescriptorBindingFlagBits :: UPDATE_UNUSED_WHILE_PENDING_EXT . 0 ; const PARTIALLY_BOUND_EXT = DescriptorBindingFlagBits :: PARTIALLY_BOUND_EXT . 0 ; const VARIABLE_DESCRIPTOR_COUNT_EXT = DescriptorBindingFlagBits :: VARIABLE_DESCRIPTOR_COUNT_EXT . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDescriptorIndexingFeatures.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceDescriptorIndexingFeatures {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub shader_input_attachment_array_dynamic_indexing: crate::vk1_0::Bool32,
    pub shader_uniform_texel_buffer_array_dynamic_indexing: crate::vk1_0::Bool32,
    pub shader_storage_texel_buffer_array_dynamic_indexing: crate::vk1_0::Bool32,
    pub shader_uniform_buffer_array_non_uniform_indexing: crate::vk1_0::Bool32,
    pub shader_sampled_image_array_non_uniform_indexing: crate::vk1_0::Bool32,
    pub shader_storage_buffer_array_non_uniform_indexing: crate::vk1_0::Bool32,
    pub shader_storage_image_array_non_uniform_indexing: crate::vk1_0::Bool32,
    pub shader_input_attachment_array_non_uniform_indexing: crate::vk1_0::Bool32,
    pub shader_uniform_texel_buffer_array_non_uniform_indexing: crate::vk1_0::Bool32,
    pub shader_storage_texel_buffer_array_non_uniform_indexing: crate::vk1_0::Bool32,
    pub descriptor_binding_uniform_buffer_update_after_bind: crate::vk1_0::Bool32,
    pub descriptor_binding_sampled_image_update_after_bind: crate::vk1_0::Bool32,
    pub descriptor_binding_storage_image_update_after_bind: crate::vk1_0::Bool32,
    pub descriptor_binding_storage_buffer_update_after_bind: crate::vk1_0::Bool32,
    pub descriptor_binding_uniform_texel_buffer_update_after_bind: crate::vk1_0::Bool32,
    pub descriptor_binding_storage_texel_buffer_update_after_bind: crate::vk1_0::Bool32,
    pub descriptor_binding_update_unused_while_pending: crate::vk1_0::Bool32,
    pub descriptor_binding_partially_bound: crate::vk1_0::Bool32,
    pub descriptor_binding_variable_descriptor_count: crate::vk1_0::Bool32,
    pub runtime_descriptor_array: crate::vk1_0::Bool32,
}
impl PhysicalDeviceDescriptorIndexingFeatures {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceDescriptorIndexingFeatures,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceDescriptorIndexingFeaturesBuilder<'a> {
        PhysicalDeviceDescriptorIndexingFeaturesBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceDescriptorIndexingFeatures {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceDescriptorIndexingFeatures")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "shader_input_attachment_array_dynamic_indexing",
                &(self.shader_input_attachment_array_dynamic_indexing != 0),
            )
            .field(
                "shader_uniform_texel_buffer_array_dynamic_indexing",
                &(self.shader_uniform_texel_buffer_array_dynamic_indexing != 0),
            )
            .field(
                "shader_storage_texel_buffer_array_dynamic_indexing",
                &(self.shader_storage_texel_buffer_array_dynamic_indexing != 0),
            )
            .field(
                "shader_uniform_buffer_array_non_uniform_indexing",
                &(self.shader_uniform_buffer_array_non_uniform_indexing != 0),
            )
            .field(
                "shader_sampled_image_array_non_uniform_indexing",
                &(self.shader_sampled_image_array_non_uniform_indexing != 0),
            )
            .field(
                "shader_storage_buffer_array_non_uniform_indexing",
                &(self.shader_storage_buffer_array_non_uniform_indexing != 0),
            )
            .field(
                "shader_storage_image_array_non_uniform_indexing",
                &(self.shader_storage_image_array_non_uniform_indexing != 0),
            )
            .field(
                "shader_input_attachment_array_non_uniform_indexing",
                &(self.shader_input_attachment_array_non_uniform_indexing != 0),
            )
            .field(
                "shader_uniform_texel_buffer_array_non_uniform_indexing",
                &(self.shader_uniform_texel_buffer_array_non_uniform_indexing != 0),
            )
            .field(
                "shader_storage_texel_buffer_array_non_uniform_indexing",
                &(self.shader_storage_texel_buffer_array_non_uniform_indexing != 0),
            )
            .field(
                "descriptor_binding_uniform_buffer_update_after_bind",
                &(self.descriptor_binding_uniform_buffer_update_after_bind != 0),
            )
            .field(
                "descriptor_binding_sampled_image_update_after_bind",
                &(self.descriptor_binding_sampled_image_update_after_bind != 0),
            )
            .field(
                "descriptor_binding_storage_image_update_after_bind",
                &(self.descriptor_binding_storage_image_update_after_bind != 0),
            )
            .field(
                "descriptor_binding_storage_buffer_update_after_bind",
                &(self.descriptor_binding_storage_buffer_update_after_bind != 0),
            )
            .field(
                "descriptor_binding_uniform_texel_buffer_update_after_bind",
                &(self.descriptor_binding_uniform_texel_buffer_update_after_bind != 0),
            )
            .field(
                "descriptor_binding_storage_texel_buffer_update_after_bind",
                &(self.descriptor_binding_storage_texel_buffer_update_after_bind != 0),
            )
            .field(
                "descriptor_binding_update_unused_while_pending",
                &(self.descriptor_binding_update_unused_while_pending != 0),
            )
            .field(
                "descriptor_binding_partially_bound",
                &(self.descriptor_binding_partially_bound != 0),
            )
            .field(
                "descriptor_binding_variable_descriptor_count",
                &(self.descriptor_binding_variable_descriptor_count != 0),
            )
            .field(
                "runtime_descriptor_array",
                &(self.runtime_descriptor_array != 0),
            )
            .finish()
    }
}
impl Default for PhysicalDeviceDescriptorIndexingFeatures {
    fn default() -> PhysicalDeviceDescriptorIndexingFeatures {
        PhysicalDeviceDescriptorIndexingFeatures {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES,
            p_next: std::ptr::null_mut(),
            shader_input_attachment_array_dynamic_indexing: Default::default(),
            shader_uniform_texel_buffer_array_dynamic_indexing: Default::default(),
            shader_storage_texel_buffer_array_dynamic_indexing: Default::default(),
            shader_uniform_buffer_array_non_uniform_indexing: Default::default(),
            shader_sampled_image_array_non_uniform_indexing: Default::default(),
            shader_storage_buffer_array_non_uniform_indexing: Default::default(),
            shader_storage_image_array_non_uniform_indexing: Default::default(),
            shader_input_attachment_array_non_uniform_indexing: Default::default(),
            shader_uniform_texel_buffer_array_non_uniform_indexing: Default::default(),
            shader_storage_texel_buffer_array_non_uniform_indexing: Default::default(),
            descriptor_binding_uniform_buffer_update_after_bind: Default::default(),
            descriptor_binding_sampled_image_update_after_bind: Default::default(),
            descriptor_binding_storage_image_update_after_bind: Default::default(),
            descriptor_binding_storage_buffer_update_after_bind: Default::default(),
            descriptor_binding_uniform_texel_buffer_update_after_bind: Default::default(),
            descriptor_binding_storage_texel_buffer_update_after_bind: Default::default(),
            descriptor_binding_update_unused_while_pending: Default::default(),
            descriptor_binding_partially_bound: Default::default(),
            descriptor_binding_variable_descriptor_count: Default::default(),
            runtime_descriptor_array: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceDescriptorIndexingFeatures::extend`](struct.PhysicalDeviceDescriptorIndexingFeatures.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceDescriptorIndexingFeatures {}
impl ExtendableByPhysicalDeviceDescriptorIndexingFeatures
    for crate::vk1_1::PhysicalDeviceFeatures2
{
}
impl ExtendableByPhysicalDeviceDescriptorIndexingFeatures for crate::vk1_0::DeviceCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceDescriptorIndexingFeatures`](struct.PhysicalDeviceDescriptorIndexingFeatures.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceDescriptorIndexingFeaturesBuilder<'a>(
    PhysicalDeviceDescriptorIndexingFeatures,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceDescriptorIndexingFeaturesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceDescriptorIndexingFeaturesBuilder<'a> {
        PhysicalDeviceDescriptorIndexingFeaturesBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_input_attachment_array_dynamic_indexing(
        mut self,
        shader_input_attachment_array_dynamic_indexing: bool,
    ) -> Self {
        self.0.shader_input_attachment_array_dynamic_indexing =
            shader_input_attachment_array_dynamic_indexing as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_uniform_texel_buffer_array_dynamic_indexing(
        mut self,
        shader_uniform_texel_buffer_array_dynamic_indexing: bool,
    ) -> Self {
        self.0.shader_uniform_texel_buffer_array_dynamic_indexing =
            shader_uniform_texel_buffer_array_dynamic_indexing as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_storage_texel_buffer_array_dynamic_indexing(
        mut self,
        shader_storage_texel_buffer_array_dynamic_indexing: bool,
    ) -> Self {
        self.0.shader_storage_texel_buffer_array_dynamic_indexing =
            shader_storage_texel_buffer_array_dynamic_indexing as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_uniform_buffer_array_non_uniform_indexing(
        mut self,
        shader_uniform_buffer_array_non_uniform_indexing: bool,
    ) -> Self {
        self.0.shader_uniform_buffer_array_non_uniform_indexing =
            shader_uniform_buffer_array_non_uniform_indexing as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_sampled_image_array_non_uniform_indexing(
        mut self,
        shader_sampled_image_array_non_uniform_indexing: bool,
    ) -> Self {
        self.0.shader_sampled_image_array_non_uniform_indexing =
            shader_sampled_image_array_non_uniform_indexing as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_storage_buffer_array_non_uniform_indexing(
        mut self,
        shader_storage_buffer_array_non_uniform_indexing: bool,
    ) -> Self {
        self.0.shader_storage_buffer_array_non_uniform_indexing =
            shader_storage_buffer_array_non_uniform_indexing as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_storage_image_array_non_uniform_indexing(
        mut self,
        shader_storage_image_array_non_uniform_indexing: bool,
    ) -> Self {
        self.0.shader_storage_image_array_non_uniform_indexing =
            shader_storage_image_array_non_uniform_indexing as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_input_attachment_array_non_uniform_indexing(
        mut self,
        shader_input_attachment_array_non_uniform_indexing: bool,
    ) -> Self {
        self.0.shader_input_attachment_array_non_uniform_indexing =
            shader_input_attachment_array_non_uniform_indexing as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_uniform_texel_buffer_array_non_uniform_indexing(
        mut self,
        shader_uniform_texel_buffer_array_non_uniform_indexing: bool,
    ) -> Self {
        self.0
            .shader_uniform_texel_buffer_array_non_uniform_indexing =
            shader_uniform_texel_buffer_array_non_uniform_indexing as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_storage_texel_buffer_array_non_uniform_indexing(
        mut self,
        shader_storage_texel_buffer_array_non_uniform_indexing: bool,
    ) -> Self {
        self.0
            .shader_storage_texel_buffer_array_non_uniform_indexing =
            shader_storage_texel_buffer_array_non_uniform_indexing as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn descriptor_binding_uniform_buffer_update_after_bind(
        mut self,
        descriptor_binding_uniform_buffer_update_after_bind: bool,
    ) -> Self {
        self.0.descriptor_binding_uniform_buffer_update_after_bind =
            descriptor_binding_uniform_buffer_update_after_bind as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn descriptor_binding_sampled_image_update_after_bind(
        mut self,
        descriptor_binding_sampled_image_update_after_bind: bool,
    ) -> Self {
        self.0.descriptor_binding_sampled_image_update_after_bind =
            descriptor_binding_sampled_image_update_after_bind as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn descriptor_binding_storage_image_update_after_bind(
        mut self,
        descriptor_binding_storage_image_update_after_bind: bool,
    ) -> Self {
        self.0.descriptor_binding_storage_image_update_after_bind =
            descriptor_binding_storage_image_update_after_bind as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn descriptor_binding_storage_buffer_update_after_bind(
        mut self,
        descriptor_binding_storage_buffer_update_after_bind: bool,
    ) -> Self {
        self.0.descriptor_binding_storage_buffer_update_after_bind =
            descriptor_binding_storage_buffer_update_after_bind as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn descriptor_binding_uniform_texel_buffer_update_after_bind(
        mut self,
        descriptor_binding_uniform_texel_buffer_update_after_bind: bool,
    ) -> Self {
        self.0
            .descriptor_binding_uniform_texel_buffer_update_after_bind =
            descriptor_binding_uniform_texel_buffer_update_after_bind as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn descriptor_binding_storage_texel_buffer_update_after_bind(
        mut self,
        descriptor_binding_storage_texel_buffer_update_after_bind: bool,
    ) -> Self {
        self.0
            .descriptor_binding_storage_texel_buffer_update_after_bind =
            descriptor_binding_storage_texel_buffer_update_after_bind as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn descriptor_binding_update_unused_while_pending(
        mut self,
        descriptor_binding_update_unused_while_pending: bool,
    ) -> Self {
        self.0.descriptor_binding_update_unused_while_pending =
            descriptor_binding_update_unused_while_pending as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn descriptor_binding_partially_bound(
        mut self,
        descriptor_binding_partially_bound: bool,
    ) -> Self {
        self.0.descriptor_binding_partially_bound = descriptor_binding_partially_bound as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn descriptor_binding_variable_descriptor_count(
        mut self,
        descriptor_binding_variable_descriptor_count: bool,
    ) -> Self {
        self.0.descriptor_binding_variable_descriptor_count =
            descriptor_binding_variable_descriptor_count as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn runtime_descriptor_array(mut self, runtime_descriptor_array: bool) -> Self {
        self.0.runtime_descriptor_array = runtime_descriptor_array as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceDescriptorIndexingFeatures {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceDescriptorIndexingFeaturesBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceDescriptorIndexingFeaturesBuilder<'a> {
    type Target = PhysicalDeviceDescriptorIndexingFeatures;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceDescriptorIndexingFeaturesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDescriptorIndexingProperties.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceDescriptorIndexingProperties {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub max_update_after_bind_descriptors_in_all_pools: u32,
    pub shader_uniform_buffer_array_non_uniform_indexing_native: crate::vk1_0::Bool32,
    pub shader_sampled_image_array_non_uniform_indexing_native: crate::vk1_0::Bool32,
    pub shader_storage_buffer_array_non_uniform_indexing_native: crate::vk1_0::Bool32,
    pub shader_storage_image_array_non_uniform_indexing_native: crate::vk1_0::Bool32,
    pub shader_input_attachment_array_non_uniform_indexing_native: crate::vk1_0::Bool32,
    pub robust_buffer_access_update_after_bind: crate::vk1_0::Bool32,
    pub quad_divergent_implicit_lod: crate::vk1_0::Bool32,
    pub max_per_stage_descriptor_update_after_bind_samplers: u32,
    pub max_per_stage_descriptor_update_after_bind_uniform_buffers: u32,
    pub max_per_stage_descriptor_update_after_bind_storage_buffers: u32,
    pub max_per_stage_descriptor_update_after_bind_sampled_images: u32,
    pub max_per_stage_descriptor_update_after_bind_storage_images: u32,
    pub max_per_stage_descriptor_update_after_bind_input_attachments: u32,
    pub max_per_stage_update_after_bind_resources: u32,
    pub max_descriptor_set_update_after_bind_samplers: u32,
    pub max_descriptor_set_update_after_bind_uniform_buffers: u32,
    pub max_descriptor_set_update_after_bind_uniform_buffers_dynamic: u32,
    pub max_descriptor_set_update_after_bind_storage_buffers: u32,
    pub max_descriptor_set_update_after_bind_storage_buffers_dynamic: u32,
    pub max_descriptor_set_update_after_bind_sampled_images: u32,
    pub max_descriptor_set_update_after_bind_storage_images: u32,
    pub max_descriptor_set_update_after_bind_input_attachments: u32,
}
impl PhysicalDeviceDescriptorIndexingProperties {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceDescriptorIndexingProperties,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceDescriptorIndexingPropertiesBuilder<'a> {
        PhysicalDeviceDescriptorIndexingPropertiesBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceDescriptorIndexingProperties {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceDescriptorIndexingProperties")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "max_update_after_bind_descriptors_in_all_pools",
                &self.max_update_after_bind_descriptors_in_all_pools,
            )
            .field(
                "shader_uniform_buffer_array_non_uniform_indexing_native",
                &(self.shader_uniform_buffer_array_non_uniform_indexing_native != 0),
            )
            .field(
                "shader_sampled_image_array_non_uniform_indexing_native",
                &(self.shader_sampled_image_array_non_uniform_indexing_native != 0),
            )
            .field(
                "shader_storage_buffer_array_non_uniform_indexing_native",
                &(self.shader_storage_buffer_array_non_uniform_indexing_native != 0),
            )
            .field(
                "shader_storage_image_array_non_uniform_indexing_native",
                &(self.shader_storage_image_array_non_uniform_indexing_native != 0),
            )
            .field(
                "shader_input_attachment_array_non_uniform_indexing_native",
                &(self.shader_input_attachment_array_non_uniform_indexing_native != 0),
            )
            .field(
                "robust_buffer_access_update_after_bind",
                &(self.robust_buffer_access_update_after_bind != 0),
            )
            .field(
                "quad_divergent_implicit_lod",
                &(self.quad_divergent_implicit_lod != 0),
            )
            .field(
                "max_per_stage_descriptor_update_after_bind_samplers",
                &self.max_per_stage_descriptor_update_after_bind_samplers,
            )
            .field(
                "max_per_stage_descriptor_update_after_bind_uniform_buffers",
                &self.max_per_stage_descriptor_update_after_bind_uniform_buffers,
            )
            .field(
                "max_per_stage_descriptor_update_after_bind_storage_buffers",
                &self.max_per_stage_descriptor_update_after_bind_storage_buffers,
            )
            .field(
                "max_per_stage_descriptor_update_after_bind_sampled_images",
                &self.max_per_stage_descriptor_update_after_bind_sampled_images,
            )
            .field(
                "max_per_stage_descriptor_update_after_bind_storage_images",
                &self.max_per_stage_descriptor_update_after_bind_storage_images,
            )
            .field(
                "max_per_stage_descriptor_update_after_bind_input_attachments",
                &self.max_per_stage_descriptor_update_after_bind_input_attachments,
            )
            .field(
                "max_per_stage_update_after_bind_resources",
                &self.max_per_stage_update_after_bind_resources,
            )
            .field(
                "max_descriptor_set_update_after_bind_samplers",
                &self.max_descriptor_set_update_after_bind_samplers,
            )
            .field(
                "max_descriptor_set_update_after_bind_uniform_buffers",
                &self.max_descriptor_set_update_after_bind_uniform_buffers,
            )
            .field(
                "max_descriptor_set_update_after_bind_uniform_buffers_dynamic",
                &self.max_descriptor_set_update_after_bind_uniform_buffers_dynamic,
            )
            .field(
                "max_descriptor_set_update_after_bind_storage_buffers",
                &self.max_descriptor_set_update_after_bind_storage_buffers,
            )
            .field(
                "max_descriptor_set_update_after_bind_storage_buffers_dynamic",
                &self.max_descriptor_set_update_after_bind_storage_buffers_dynamic,
            )
            .field(
                "max_descriptor_set_update_after_bind_sampled_images",
                &self.max_descriptor_set_update_after_bind_sampled_images,
            )
            .field(
                "max_descriptor_set_update_after_bind_storage_images",
                &self.max_descriptor_set_update_after_bind_storage_images,
            )
            .field(
                "max_descriptor_set_update_after_bind_input_attachments",
                &self.max_descriptor_set_update_after_bind_input_attachments,
            )
            .finish()
    }
}
impl Default for PhysicalDeviceDescriptorIndexingProperties {
    fn default() -> PhysicalDeviceDescriptorIndexingProperties {
        PhysicalDeviceDescriptorIndexingProperties {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES,
            p_next: std::ptr::null_mut(),
            max_update_after_bind_descriptors_in_all_pools: Default::default(),
            shader_uniform_buffer_array_non_uniform_indexing_native: Default::default(),
            shader_sampled_image_array_non_uniform_indexing_native: Default::default(),
            shader_storage_buffer_array_non_uniform_indexing_native: Default::default(),
            shader_storage_image_array_non_uniform_indexing_native: Default::default(),
            shader_input_attachment_array_non_uniform_indexing_native: Default::default(),
            robust_buffer_access_update_after_bind: Default::default(),
            quad_divergent_implicit_lod: Default::default(),
            max_per_stage_descriptor_update_after_bind_samplers: Default::default(),
            max_per_stage_descriptor_update_after_bind_uniform_buffers: Default::default(),
            max_per_stage_descriptor_update_after_bind_storage_buffers: Default::default(),
            max_per_stage_descriptor_update_after_bind_sampled_images: Default::default(),
            max_per_stage_descriptor_update_after_bind_storage_images: Default::default(),
            max_per_stage_descriptor_update_after_bind_input_attachments: Default::default(),
            max_per_stage_update_after_bind_resources: Default::default(),
            max_descriptor_set_update_after_bind_samplers: Default::default(),
            max_descriptor_set_update_after_bind_uniform_buffers: Default::default(),
            max_descriptor_set_update_after_bind_uniform_buffers_dynamic: Default::default(),
            max_descriptor_set_update_after_bind_storage_buffers: Default::default(),
            max_descriptor_set_update_after_bind_storage_buffers_dynamic: Default::default(),
            max_descriptor_set_update_after_bind_sampled_images: Default::default(),
            max_descriptor_set_update_after_bind_storage_images: Default::default(),
            max_descriptor_set_update_after_bind_input_attachments: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceDescriptorIndexingProperties::extend`](struct.PhysicalDeviceDescriptorIndexingProperties.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceDescriptorIndexingProperties {}
impl ExtendableByPhysicalDeviceDescriptorIndexingProperties
    for crate::vk1_1::PhysicalDeviceProperties2
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceDescriptorIndexingProperties`](struct.PhysicalDeviceDescriptorIndexingProperties.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceDescriptorIndexingPropertiesBuilder<'a>(
    PhysicalDeviceDescriptorIndexingProperties,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceDescriptorIndexingPropertiesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceDescriptorIndexingPropertiesBuilder<'a> {
        PhysicalDeviceDescriptorIndexingPropertiesBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_update_after_bind_descriptors_in_all_pools(
        mut self,
        max_update_after_bind_descriptors_in_all_pools: u32,
    ) -> Self {
        self.0.max_update_after_bind_descriptors_in_all_pools =
            max_update_after_bind_descriptors_in_all_pools;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_uniform_buffer_array_non_uniform_indexing_native(
        mut self,
        shader_uniform_buffer_array_non_uniform_indexing_native: bool,
    ) -> Self {
        self.0
            .shader_uniform_buffer_array_non_uniform_indexing_native =
            shader_uniform_buffer_array_non_uniform_indexing_native as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_sampled_image_array_non_uniform_indexing_native(
        mut self,
        shader_sampled_image_array_non_uniform_indexing_native: bool,
    ) -> Self {
        self.0
            .shader_sampled_image_array_non_uniform_indexing_native =
            shader_sampled_image_array_non_uniform_indexing_native as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_storage_buffer_array_non_uniform_indexing_native(
        mut self,
        shader_storage_buffer_array_non_uniform_indexing_native: bool,
    ) -> Self {
        self.0
            .shader_storage_buffer_array_non_uniform_indexing_native =
            shader_storage_buffer_array_non_uniform_indexing_native as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_storage_image_array_non_uniform_indexing_native(
        mut self,
        shader_storage_image_array_non_uniform_indexing_native: bool,
    ) -> Self {
        self.0
            .shader_storage_image_array_non_uniform_indexing_native =
            shader_storage_image_array_non_uniform_indexing_native as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_input_attachment_array_non_uniform_indexing_native(
        mut self,
        shader_input_attachment_array_non_uniform_indexing_native: bool,
    ) -> Self {
        self.0
            .shader_input_attachment_array_non_uniform_indexing_native =
            shader_input_attachment_array_non_uniform_indexing_native as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn robust_buffer_access_update_after_bind(
        mut self,
        robust_buffer_access_update_after_bind: bool,
    ) -> Self {
        self.0.robust_buffer_access_update_after_bind =
            robust_buffer_access_update_after_bind as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn quad_divergent_implicit_lod(mut self, quad_divergent_implicit_lod: bool) -> Self {
        self.0.quad_divergent_implicit_lod = quad_divergent_implicit_lod as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_per_stage_descriptor_update_after_bind_samplers(
        mut self,
        max_per_stage_descriptor_update_after_bind_samplers: u32,
    ) -> Self {
        self.0.max_per_stage_descriptor_update_after_bind_samplers =
            max_per_stage_descriptor_update_after_bind_samplers;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_per_stage_descriptor_update_after_bind_uniform_buffers(
        mut self,
        max_per_stage_descriptor_update_after_bind_uniform_buffers: u32,
    ) -> Self {
        self.0
            .max_per_stage_descriptor_update_after_bind_uniform_buffers =
            max_per_stage_descriptor_update_after_bind_uniform_buffers;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_per_stage_descriptor_update_after_bind_storage_buffers(
        mut self,
        max_per_stage_descriptor_update_after_bind_storage_buffers: u32,
    ) -> Self {
        self.0
            .max_per_stage_descriptor_update_after_bind_storage_buffers =
            max_per_stage_descriptor_update_after_bind_storage_buffers;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_per_stage_descriptor_update_after_bind_sampled_images(
        mut self,
        max_per_stage_descriptor_update_after_bind_sampled_images: u32,
    ) -> Self {
        self.0
            .max_per_stage_descriptor_update_after_bind_sampled_images =
            max_per_stage_descriptor_update_after_bind_sampled_images;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_per_stage_descriptor_update_after_bind_storage_images(
        mut self,
        max_per_stage_descriptor_update_after_bind_storage_images: u32,
    ) -> Self {
        self.0
            .max_per_stage_descriptor_update_after_bind_storage_images =
            max_per_stage_descriptor_update_after_bind_storage_images;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_per_stage_descriptor_update_after_bind_input_attachments(
        mut self,
        max_per_stage_descriptor_update_after_bind_input_attachments: u32,
    ) -> Self {
        self.0
            .max_per_stage_descriptor_update_after_bind_input_attachments =
            max_per_stage_descriptor_update_after_bind_input_attachments;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_per_stage_update_after_bind_resources(
        mut self,
        max_per_stage_update_after_bind_resources: u32,
    ) -> Self {
        self.0.max_per_stage_update_after_bind_resources =
            max_per_stage_update_after_bind_resources;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_descriptor_set_update_after_bind_samplers(
        mut self,
        max_descriptor_set_update_after_bind_samplers: u32,
    ) -> Self {
        self.0.max_descriptor_set_update_after_bind_samplers =
            max_descriptor_set_update_after_bind_samplers;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_descriptor_set_update_after_bind_uniform_buffers(
        mut self,
        max_descriptor_set_update_after_bind_uniform_buffers: u32,
    ) -> Self {
        self.0.max_descriptor_set_update_after_bind_uniform_buffers =
            max_descriptor_set_update_after_bind_uniform_buffers;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_descriptor_set_update_after_bind_uniform_buffers_dynamic(
        mut self,
        max_descriptor_set_update_after_bind_uniform_buffers_dynamic: u32,
    ) -> Self {
        self.0
            .max_descriptor_set_update_after_bind_uniform_buffers_dynamic =
            max_descriptor_set_update_after_bind_uniform_buffers_dynamic;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_descriptor_set_update_after_bind_storage_buffers(
        mut self,
        max_descriptor_set_update_after_bind_storage_buffers: u32,
    ) -> Self {
        self.0.max_descriptor_set_update_after_bind_storage_buffers =
            max_descriptor_set_update_after_bind_storage_buffers;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_descriptor_set_update_after_bind_storage_buffers_dynamic(
        mut self,
        max_descriptor_set_update_after_bind_storage_buffers_dynamic: u32,
    ) -> Self {
        self.0
            .max_descriptor_set_update_after_bind_storage_buffers_dynamic =
            max_descriptor_set_update_after_bind_storage_buffers_dynamic;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_descriptor_set_update_after_bind_sampled_images(
        mut self,
        max_descriptor_set_update_after_bind_sampled_images: u32,
    ) -> Self {
        self.0.max_descriptor_set_update_after_bind_sampled_images =
            max_descriptor_set_update_after_bind_sampled_images;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_descriptor_set_update_after_bind_storage_images(
        mut self,
        max_descriptor_set_update_after_bind_storage_images: u32,
    ) -> Self {
        self.0.max_descriptor_set_update_after_bind_storage_images =
            max_descriptor_set_update_after_bind_storage_images;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_descriptor_set_update_after_bind_input_attachments(
        mut self,
        max_descriptor_set_update_after_bind_input_attachments: u32,
    ) -> Self {
        self.0
            .max_descriptor_set_update_after_bind_input_attachments =
            max_descriptor_set_update_after_bind_input_attachments;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceDescriptorIndexingProperties {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceDescriptorIndexingPropertiesBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceDescriptorIndexingPropertiesBuilder<'a> {
    type Target = PhysicalDeviceDescriptorIndexingProperties;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceDescriptorIndexingPropertiesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorSetVariableDescriptorCountAllocateInfo.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DescriptorSetVariableDescriptorCountAllocateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub descriptor_set_count: u32,
    pub p_descriptor_counts: *const u32,
}
impl DescriptorSetVariableDescriptorCountAllocateInfo {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByDescriptorSetVariableDescriptorCountAllocateInfo,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> DescriptorSetVariableDescriptorCountAllocateInfoBuilder<'a> {
        DescriptorSetVariableDescriptorCountAllocateInfoBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for DescriptorSetVariableDescriptorCountAllocateInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("DescriptorSetVariableDescriptorCountAllocateInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("descriptor_set_count", &self.descriptor_set_count)
            .field("p_descriptor_counts", &self.p_descriptor_counts)
            .finish()
    }
}
impl Default for DescriptorSetVariableDescriptorCountAllocateInfo {
    fn default() -> DescriptorSetVariableDescriptorCountAllocateInfo {
        DescriptorSetVariableDescriptorCountAllocateInfo {
            s_type:
                crate::vk1_0::StructureType::DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO,
            p_next: std::ptr::null(),
            descriptor_set_count: Default::default(),
            p_descriptor_counts: std::ptr::null(),
        }
    }
}
#[doc = "Used by [`DescriptorSetVariableDescriptorCountAllocateInfo::extend`](struct.DescriptorSetVariableDescriptorCountAllocateInfo.html#method.extend)"]
pub trait ExtendableByDescriptorSetVariableDescriptorCountAllocateInfo {}
impl ExtendableByDescriptorSetVariableDescriptorCountAllocateInfo
    for crate::vk1_0::DescriptorSetAllocateInfo
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`DescriptorSetVariableDescriptorCountAllocateInfo`](struct.DescriptorSetVariableDescriptorCountAllocateInfo.html)"]
#[repr(transparent)]
pub struct DescriptorSetVariableDescriptorCountAllocateInfoBuilder<'a>(
    DescriptorSetVariableDescriptorCountAllocateInfo,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> DescriptorSetVariableDescriptorCountAllocateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> DescriptorSetVariableDescriptorCountAllocateInfoBuilder<'a> {
        DescriptorSetVariableDescriptorCountAllocateInfoBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn descriptor_counts(mut self, descriptor_counts: &'a [u32]) -> Self {
        self.0.descriptor_set_count = descriptor_counts.len() as _;
        self.0.p_descriptor_counts = descriptor_counts.as_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> DescriptorSetVariableDescriptorCountAllocateInfo {
        self.0
    }
}
impl<'a> std::fmt::Debug for DescriptorSetVariableDescriptorCountAllocateInfoBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for DescriptorSetVariableDescriptorCountAllocateInfoBuilder<'a> {
    type Target = DescriptorSetVariableDescriptorCountAllocateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DescriptorSetVariableDescriptorCountAllocateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorSetVariableDescriptorCountLayoutSupport.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DescriptorSetVariableDescriptorCountLayoutSupport {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub max_variable_descriptor_count: u32,
}
impl DescriptorSetVariableDescriptorCountLayoutSupport {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByDescriptorSetVariableDescriptorCountLayoutSupport,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> DescriptorSetVariableDescriptorCountLayoutSupportBuilder<'a> {
        DescriptorSetVariableDescriptorCountLayoutSupportBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for DescriptorSetVariableDescriptorCountLayoutSupport {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("DescriptorSetVariableDescriptorCountLayoutSupport")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "max_variable_descriptor_count",
                &self.max_variable_descriptor_count,
            )
            .finish()
    }
}
impl Default for DescriptorSetVariableDescriptorCountLayoutSupport {
    fn default() -> DescriptorSetVariableDescriptorCountLayoutSupport {
        DescriptorSetVariableDescriptorCountLayoutSupport {
            s_type:
                crate::vk1_0::StructureType::DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT,
            p_next: std::ptr::null_mut(),
            max_variable_descriptor_count: Default::default(),
        }
    }
}
#[doc = "Used by [`DescriptorSetVariableDescriptorCountLayoutSupport::extend`](struct.DescriptorSetVariableDescriptorCountLayoutSupport.html#method.extend)"]
pub trait ExtendableByDescriptorSetVariableDescriptorCountLayoutSupport {}
impl ExtendableByDescriptorSetVariableDescriptorCountLayoutSupport
    for crate::vk1_1::DescriptorSetLayoutSupport
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`DescriptorSetVariableDescriptorCountLayoutSupport`](struct.DescriptorSetVariableDescriptorCountLayoutSupport.html)"]
#[repr(transparent)]
pub struct DescriptorSetVariableDescriptorCountLayoutSupportBuilder<'a>(
    DescriptorSetVariableDescriptorCountLayoutSupport,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> DescriptorSetVariableDescriptorCountLayoutSupportBuilder<'a> {
    #[inline]
    pub fn new() -> DescriptorSetVariableDescriptorCountLayoutSupportBuilder<'a> {
        DescriptorSetVariableDescriptorCountLayoutSupportBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_variable_descriptor_count(mut self, max_variable_descriptor_count: u32) -> Self {
        self.0.max_variable_descriptor_count = max_variable_descriptor_count;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> DescriptorSetVariableDescriptorCountLayoutSupport {
        self.0
    }
}
impl<'a> std::fmt::Debug for DescriptorSetVariableDescriptorCountLayoutSupportBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for DescriptorSetVariableDescriptorCountLayoutSupportBuilder<'a> {
    type Target = DescriptorSetVariableDescriptorCountLayoutSupport;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DescriptorSetVariableDescriptorCountLayoutSupportBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubpassDescriptionDepthStencilResolve.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SubpassDescriptionDepthStencilResolve {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub depth_resolve_mode: crate::vk1_2::ResolveModeFlagBits,
    pub stencil_resolve_mode: crate::vk1_2::ResolveModeFlagBits,
    pub p_depth_stencil_resolve_attachment: *const crate::vk1_2::AttachmentReference2,
}
impl SubpassDescriptionDepthStencilResolve {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableBySubpassDescriptionDepthStencilResolve,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> SubpassDescriptionDepthStencilResolveBuilder<'a> {
        SubpassDescriptionDepthStencilResolveBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for SubpassDescriptionDepthStencilResolve {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("SubpassDescriptionDepthStencilResolve")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("depth_resolve_mode", &self.depth_resolve_mode)
            .field("stencil_resolve_mode", &self.stencil_resolve_mode)
            .field(
                "p_depth_stencil_resolve_attachment",
                &self.p_depth_stencil_resolve_attachment,
            )
            .finish()
    }
}
impl Default for SubpassDescriptionDepthStencilResolve {
    fn default() -> SubpassDescriptionDepthStencilResolve {
        SubpassDescriptionDepthStencilResolve {
            s_type: crate::vk1_0::StructureType::SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE,
            p_next: std::ptr::null(),
            depth_resolve_mode: Default::default(),
            stencil_resolve_mode: Default::default(),
            p_depth_stencil_resolve_attachment: std::ptr::null(),
        }
    }
}
#[doc = "Used by [`SubpassDescriptionDepthStencilResolve::extend`](struct.SubpassDescriptionDepthStencilResolve.html#method.extend)"]
pub trait ExtendableBySubpassDescriptionDepthStencilResolve {}
impl ExtendableBySubpassDescriptionDepthStencilResolve for crate::vk1_2::SubpassDescription2 {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`SubpassDescriptionDepthStencilResolve`](struct.SubpassDescriptionDepthStencilResolve.html)"]
#[repr(transparent)]
pub struct SubpassDescriptionDepthStencilResolveBuilder<'a>(
    SubpassDescriptionDepthStencilResolve,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> SubpassDescriptionDepthStencilResolveBuilder<'a> {
    #[inline]
    pub fn new() -> SubpassDescriptionDepthStencilResolveBuilder<'a> {
        SubpassDescriptionDepthStencilResolveBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn depth_resolve_mode(
        mut self,
        depth_resolve_mode: crate::vk1_2::ResolveModeFlagBits,
    ) -> Self {
        self.0.depth_resolve_mode = depth_resolve_mode;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn stencil_resolve_mode(
        mut self,
        stencil_resolve_mode: crate::vk1_2::ResolveModeFlagBits,
    ) -> Self {
        self.0.stencil_resolve_mode = stencil_resolve_mode;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn depth_stencil_resolve_attachment(
        mut self,
        depth_stencil_resolve_attachment: &'a crate::vk1_2::AttachmentReference2,
    ) -> Self {
        self.0.p_depth_stencil_resolve_attachment = depth_stencil_resolve_attachment;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> SubpassDescriptionDepthStencilResolve {
        self.0
    }
}
impl<'a> std::fmt::Debug for SubpassDescriptionDepthStencilResolveBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for SubpassDescriptionDepthStencilResolveBuilder<'a> {
    type Target = SubpassDescriptionDepthStencilResolve;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SubpassDescriptionDepthStencilResolveBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDepthStencilResolveProperties.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceDepthStencilResolveProperties {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub supported_depth_resolve_modes: crate::vk1_2::ResolveModeFlags,
    pub supported_stencil_resolve_modes: crate::vk1_2::ResolveModeFlags,
    pub independent_resolve_none: crate::vk1_0::Bool32,
    pub independent_resolve: crate::vk1_0::Bool32,
}
impl PhysicalDeviceDepthStencilResolveProperties {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceDepthStencilResolveProperties,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceDepthStencilResolvePropertiesBuilder<'a> {
        PhysicalDeviceDepthStencilResolvePropertiesBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceDepthStencilResolveProperties {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceDepthStencilResolveProperties")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "supported_depth_resolve_modes",
                &self.supported_depth_resolve_modes,
            )
            .field(
                "supported_stencil_resolve_modes",
                &self.supported_stencil_resolve_modes,
            )
            .field(
                "independent_resolve_none",
                &(self.independent_resolve_none != 0),
            )
            .field("independent_resolve", &(self.independent_resolve != 0))
            .finish()
    }
}
impl Default for PhysicalDeviceDepthStencilResolveProperties {
    fn default() -> PhysicalDeviceDepthStencilResolveProperties {
        PhysicalDeviceDepthStencilResolveProperties {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES,
            p_next: std::ptr::null_mut(),
            supported_depth_resolve_modes: Default::default(),
            supported_stencil_resolve_modes: Default::default(),
            independent_resolve_none: Default::default(),
            independent_resolve: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceDepthStencilResolveProperties::extend`](struct.PhysicalDeviceDepthStencilResolveProperties.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceDepthStencilResolveProperties {}
impl ExtendableByPhysicalDeviceDepthStencilResolveProperties
    for crate::vk1_1::PhysicalDeviceProperties2
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceDepthStencilResolveProperties`](struct.PhysicalDeviceDepthStencilResolveProperties.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceDepthStencilResolvePropertiesBuilder<'a>(
    PhysicalDeviceDepthStencilResolveProperties,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceDepthStencilResolvePropertiesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceDepthStencilResolvePropertiesBuilder<'a> {
        PhysicalDeviceDepthStencilResolvePropertiesBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn supported_depth_resolve_modes(
        mut self,
        supported_depth_resolve_modes: crate::vk1_2::ResolveModeFlags,
    ) -> Self {
        self.0.supported_depth_resolve_modes = supported_depth_resolve_modes;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn supported_stencil_resolve_modes(
        mut self,
        supported_stencil_resolve_modes: crate::vk1_2::ResolveModeFlags,
    ) -> Self {
        self.0.supported_stencil_resolve_modes = supported_stencil_resolve_modes;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn independent_resolve_none(mut self, independent_resolve_none: bool) -> Self {
        self.0.independent_resolve_none = independent_resolve_none as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn independent_resolve(mut self, independent_resolve: bool) -> Self {
        self.0.independent_resolve = independent_resolve as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceDepthStencilResolveProperties {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceDepthStencilResolvePropertiesBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceDepthStencilResolvePropertiesBuilder<'a> {
    type Target = PhysicalDeviceDepthStencilResolveProperties;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceDepthStencilResolvePropertiesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceScalarBlockLayoutFeatures.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceScalarBlockLayoutFeatures {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub scalar_block_layout: crate::vk1_0::Bool32,
}
impl PhysicalDeviceScalarBlockLayoutFeatures {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceScalarBlockLayoutFeatures,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceScalarBlockLayoutFeaturesBuilder<'a> {
        PhysicalDeviceScalarBlockLayoutFeaturesBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceScalarBlockLayoutFeatures {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceScalarBlockLayoutFeatures")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("scalar_block_layout", &(self.scalar_block_layout != 0))
            .finish()
    }
}
impl Default for PhysicalDeviceScalarBlockLayoutFeatures {
    fn default() -> PhysicalDeviceScalarBlockLayoutFeatures {
        PhysicalDeviceScalarBlockLayoutFeatures {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES,
            p_next: std::ptr::null_mut(),
            scalar_block_layout: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceScalarBlockLayoutFeatures::extend`](struct.PhysicalDeviceScalarBlockLayoutFeatures.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceScalarBlockLayoutFeatures {}
impl ExtendableByPhysicalDeviceScalarBlockLayoutFeatures for crate::vk1_1::PhysicalDeviceFeatures2 {}
impl ExtendableByPhysicalDeviceScalarBlockLayoutFeatures for crate::vk1_0::DeviceCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceScalarBlockLayoutFeatures`](struct.PhysicalDeviceScalarBlockLayoutFeatures.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceScalarBlockLayoutFeaturesBuilder<'a>(
    PhysicalDeviceScalarBlockLayoutFeatures,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceScalarBlockLayoutFeaturesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceScalarBlockLayoutFeaturesBuilder<'a> {
        PhysicalDeviceScalarBlockLayoutFeaturesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn scalar_block_layout(mut self, scalar_block_layout: bool) -> Self {
        self.0.scalar_block_layout = scalar_block_layout as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceScalarBlockLayoutFeatures {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceScalarBlockLayoutFeaturesBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceScalarBlockLayoutFeaturesBuilder<'a> {
    type Target = PhysicalDeviceScalarBlockLayoutFeatures;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceScalarBlockLayoutFeaturesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageStencilUsageCreateInfo.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImageStencilUsageCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub stencil_usage: crate::vk1_0::ImageUsageFlags,
}
impl ImageStencilUsageCreateInfo {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByImageStencilUsageCreateInfo,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> ImageStencilUsageCreateInfoBuilder<'a> {
        ImageStencilUsageCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ImageStencilUsageCreateInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ImageStencilUsageCreateInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("stencil_usage", &self.stencil_usage)
            .finish()
    }
}
impl Default for ImageStencilUsageCreateInfo {
    fn default() -> ImageStencilUsageCreateInfo {
        ImageStencilUsageCreateInfo {
            s_type: crate::vk1_0::StructureType::IMAGE_STENCIL_USAGE_CREATE_INFO,
            p_next: std::ptr::null(),
            stencil_usage: Default::default(),
        }
    }
}
#[doc = "Used by [`ImageStencilUsageCreateInfo::extend`](struct.ImageStencilUsageCreateInfo.html#method.extend)"]
pub trait ExtendableByImageStencilUsageCreateInfo {}
impl ExtendableByImageStencilUsageCreateInfo for crate::vk1_0::ImageCreateInfo {}
impl ExtendableByImageStencilUsageCreateInfo for crate::vk1_1::PhysicalDeviceImageFormatInfo2 {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`ImageStencilUsageCreateInfo`](struct.ImageStencilUsageCreateInfo.html)"]
#[repr(transparent)]
pub struct ImageStencilUsageCreateInfoBuilder<'a>(
    ImageStencilUsageCreateInfo,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> ImageStencilUsageCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> ImageStencilUsageCreateInfoBuilder<'a> {
        ImageStencilUsageCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn stencil_usage(mut self, stencil_usage: crate::vk1_0::ImageUsageFlags) -> Self {
        self.0.stencil_usage = stencil_usage;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> ImageStencilUsageCreateInfo {
        self.0
    }
}
impl<'a> std::fmt::Debug for ImageStencilUsageCreateInfoBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for ImageStencilUsageCreateInfoBuilder<'a> {
    type Target = ImageStencilUsageCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ImageStencilUsageCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerReductionMode.html) · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct SamplerReductionMode(pub i32);
#[doc = "[Part of `vk1_2`](../vk1_2/index.html)"]
impl SamplerReductionMode {
    pub const WEIGHTED_AVERAGE: Self = Self(0);
    pub const MIN: Self = Self(1);
    pub const MAX: Self = Self(2);
}
#[doc = "[Part of `extensions::ext_sampler_filter_minmax`](../extensions/ext_sampler_filter_minmax/index.html)"]
impl SamplerReductionMode {
    pub const WEIGHTED_AVERAGE_EXT: Self = Self::WEIGHTED_AVERAGE;
    pub const MIN_EXT: Self = Self::MIN;
    pub const MAX_EXT: Self = Self::MAX;
}
impl std::fmt::Debug for SamplerReductionMode {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::WEIGHTED_AVERAGE => "WEIGHTED_AVERAGE",
            &Self::MIN => "MIN",
            &Self::MAX => "MAX",
            _ => "Unknown enum variant",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerReductionModeCreateInfo.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SamplerReductionModeCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub reduction_mode: crate::vk1_2::SamplerReductionMode,
}
impl SamplerReductionModeCreateInfo {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableBySamplerReductionModeCreateInfo,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> SamplerReductionModeCreateInfoBuilder<'a> {
        SamplerReductionModeCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for SamplerReductionModeCreateInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("SamplerReductionModeCreateInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("reduction_mode", &self.reduction_mode)
            .finish()
    }
}
impl Default for SamplerReductionModeCreateInfo {
    fn default() -> SamplerReductionModeCreateInfo {
        SamplerReductionModeCreateInfo {
            s_type: crate::vk1_0::StructureType::SAMPLER_REDUCTION_MODE_CREATE_INFO,
            p_next: std::ptr::null(),
            reduction_mode: Default::default(),
        }
    }
}
#[doc = "Used by [`SamplerReductionModeCreateInfo::extend`](struct.SamplerReductionModeCreateInfo.html#method.extend)"]
pub trait ExtendableBySamplerReductionModeCreateInfo {}
impl ExtendableBySamplerReductionModeCreateInfo for crate::vk1_0::SamplerCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`SamplerReductionModeCreateInfo`](struct.SamplerReductionModeCreateInfo.html)"]
#[repr(transparent)]
pub struct SamplerReductionModeCreateInfoBuilder<'a>(
    SamplerReductionModeCreateInfo,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> SamplerReductionModeCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> SamplerReductionModeCreateInfoBuilder<'a> {
        SamplerReductionModeCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn reduction_mode(mut self, reduction_mode: crate::vk1_2::SamplerReductionMode) -> Self {
        self.0.reduction_mode = reduction_mode;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> SamplerReductionModeCreateInfo {
        self.0
    }
}
impl<'a> std::fmt::Debug for SamplerReductionModeCreateInfoBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for SamplerReductionModeCreateInfoBuilder<'a> {
    type Target = SamplerReductionModeCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SamplerReductionModeCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSamplerFilterMinmaxProperties.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceSamplerFilterMinmaxProperties {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub filter_minmax_single_component_formats: crate::vk1_0::Bool32,
    pub filter_minmax_image_component_mapping: crate::vk1_0::Bool32,
}
impl PhysicalDeviceSamplerFilterMinmaxProperties {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceSamplerFilterMinmaxProperties,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceSamplerFilterMinmaxPropertiesBuilder<'a> {
        PhysicalDeviceSamplerFilterMinmaxPropertiesBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceSamplerFilterMinmaxProperties {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceSamplerFilterMinmaxProperties")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "filter_minmax_single_component_formats",
                &(self.filter_minmax_single_component_formats != 0),
            )
            .field(
                "filter_minmax_image_component_mapping",
                &(self.filter_minmax_image_component_mapping != 0),
            )
            .finish()
    }
}
impl Default for PhysicalDeviceSamplerFilterMinmaxProperties {
    fn default() -> PhysicalDeviceSamplerFilterMinmaxProperties {
        PhysicalDeviceSamplerFilterMinmaxProperties {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES,
            p_next: std::ptr::null_mut(),
            filter_minmax_single_component_formats: Default::default(),
            filter_minmax_image_component_mapping: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceSamplerFilterMinmaxProperties::extend`](struct.PhysicalDeviceSamplerFilterMinmaxProperties.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceSamplerFilterMinmaxProperties {}
impl ExtendableByPhysicalDeviceSamplerFilterMinmaxProperties
    for crate::vk1_1::PhysicalDeviceProperties2
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceSamplerFilterMinmaxProperties`](struct.PhysicalDeviceSamplerFilterMinmaxProperties.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceSamplerFilterMinmaxPropertiesBuilder<'a>(
    PhysicalDeviceSamplerFilterMinmaxProperties,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceSamplerFilterMinmaxPropertiesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceSamplerFilterMinmaxPropertiesBuilder<'a> {
        PhysicalDeviceSamplerFilterMinmaxPropertiesBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn filter_minmax_single_component_formats(
        mut self,
        filter_minmax_single_component_formats: bool,
    ) -> Self {
        self.0.filter_minmax_single_component_formats =
            filter_minmax_single_component_formats as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn filter_minmax_image_component_mapping(
        mut self,
        filter_minmax_image_component_mapping: bool,
    ) -> Self {
        self.0.filter_minmax_image_component_mapping = filter_minmax_image_component_mapping as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceSamplerFilterMinmaxProperties {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceSamplerFilterMinmaxPropertiesBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceSamplerFilterMinmaxPropertiesBuilder<'a> {
    type Target = PhysicalDeviceSamplerFilterMinmaxProperties;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceSamplerFilterMinmaxPropertiesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceVulkanMemoryModelFeatures.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceVulkanMemoryModelFeatures {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub vulkan_memory_model: crate::vk1_0::Bool32,
    pub vulkan_memory_model_device_scope: crate::vk1_0::Bool32,
    pub vulkan_memory_model_availability_visibility_chains: crate::vk1_0::Bool32,
}
impl PhysicalDeviceVulkanMemoryModelFeatures {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceVulkanMemoryModelFeatures,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceVulkanMemoryModelFeaturesBuilder<'a> {
        PhysicalDeviceVulkanMemoryModelFeaturesBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceVulkanMemoryModelFeatures {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceVulkanMemoryModelFeatures")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("vulkan_memory_model", &(self.vulkan_memory_model != 0))
            .field(
                "vulkan_memory_model_device_scope",
                &(self.vulkan_memory_model_device_scope != 0),
            )
            .field(
                "vulkan_memory_model_availability_visibility_chains",
                &(self.vulkan_memory_model_availability_visibility_chains != 0),
            )
            .finish()
    }
}
impl Default for PhysicalDeviceVulkanMemoryModelFeatures {
    fn default() -> PhysicalDeviceVulkanMemoryModelFeatures {
        PhysicalDeviceVulkanMemoryModelFeatures {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES,
            p_next: std::ptr::null_mut(),
            vulkan_memory_model: Default::default(),
            vulkan_memory_model_device_scope: Default::default(),
            vulkan_memory_model_availability_visibility_chains: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceVulkanMemoryModelFeatures::extend`](struct.PhysicalDeviceVulkanMemoryModelFeatures.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceVulkanMemoryModelFeatures {}
impl ExtendableByPhysicalDeviceVulkanMemoryModelFeatures for crate::vk1_1::PhysicalDeviceFeatures2 {}
impl ExtendableByPhysicalDeviceVulkanMemoryModelFeatures for crate::vk1_0::DeviceCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceVulkanMemoryModelFeatures`](struct.PhysicalDeviceVulkanMemoryModelFeatures.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceVulkanMemoryModelFeaturesBuilder<'a>(
    PhysicalDeviceVulkanMemoryModelFeatures,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceVulkanMemoryModelFeaturesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceVulkanMemoryModelFeaturesBuilder<'a> {
        PhysicalDeviceVulkanMemoryModelFeaturesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn vulkan_memory_model(mut self, vulkan_memory_model: bool) -> Self {
        self.0.vulkan_memory_model = vulkan_memory_model as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn vulkan_memory_model_device_scope(
        mut self,
        vulkan_memory_model_device_scope: bool,
    ) -> Self {
        self.0.vulkan_memory_model_device_scope = vulkan_memory_model_device_scope as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn vulkan_memory_model_availability_visibility_chains(
        mut self,
        vulkan_memory_model_availability_visibility_chains: bool,
    ) -> Self {
        self.0.vulkan_memory_model_availability_visibility_chains =
            vulkan_memory_model_availability_visibility_chains as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceVulkanMemoryModelFeatures {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceVulkanMemoryModelFeaturesBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceVulkanMemoryModelFeaturesBuilder<'a> {
    type Target = PhysicalDeviceVulkanMemoryModelFeatures;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceVulkanMemoryModelFeaturesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceImagelessFramebufferFeatures.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceImagelessFramebufferFeatures {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub imageless_framebuffer: crate::vk1_0::Bool32,
}
impl PhysicalDeviceImagelessFramebufferFeatures {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceImagelessFramebufferFeatures,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceImagelessFramebufferFeaturesBuilder<'a> {
        PhysicalDeviceImagelessFramebufferFeaturesBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceImagelessFramebufferFeatures {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceImagelessFramebufferFeatures")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("imageless_framebuffer", &(self.imageless_framebuffer != 0))
            .finish()
    }
}
impl Default for PhysicalDeviceImagelessFramebufferFeatures {
    fn default() -> PhysicalDeviceImagelessFramebufferFeatures {
        PhysicalDeviceImagelessFramebufferFeatures {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES,
            p_next: std::ptr::null_mut(),
            imageless_framebuffer: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceImagelessFramebufferFeatures::extend`](struct.PhysicalDeviceImagelessFramebufferFeatures.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceImagelessFramebufferFeatures {}
impl ExtendableByPhysicalDeviceImagelessFramebufferFeatures
    for crate::vk1_1::PhysicalDeviceFeatures2
{
}
impl ExtendableByPhysicalDeviceImagelessFramebufferFeatures for crate::vk1_0::DeviceCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceImagelessFramebufferFeatures`](struct.PhysicalDeviceImagelessFramebufferFeatures.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceImagelessFramebufferFeaturesBuilder<'a>(
    PhysicalDeviceImagelessFramebufferFeatures,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceImagelessFramebufferFeaturesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceImagelessFramebufferFeaturesBuilder<'a> {
        PhysicalDeviceImagelessFramebufferFeaturesBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn imageless_framebuffer(mut self, imageless_framebuffer: bool) -> Self {
        self.0.imageless_framebuffer = imageless_framebuffer as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceImagelessFramebufferFeatures {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceImagelessFramebufferFeaturesBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceImagelessFramebufferFeaturesBuilder<'a> {
    type Target = PhysicalDeviceImagelessFramebufferFeatures;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceImagelessFramebufferFeaturesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFramebufferAttachmentsCreateInfo.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FramebufferAttachmentsCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub attachment_image_info_count: u32,
    pub p_attachment_image_infos: *const crate::vk1_2::FramebufferAttachmentImageInfo,
}
impl FramebufferAttachmentsCreateInfo {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByFramebufferAttachmentsCreateInfo,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> FramebufferAttachmentsCreateInfoBuilder<'a> {
        FramebufferAttachmentsCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for FramebufferAttachmentsCreateInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("FramebufferAttachmentsCreateInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "attachment_image_info_count",
                &self.attachment_image_info_count,
            )
            .field("p_attachment_image_infos", &self.p_attachment_image_infos)
            .finish()
    }
}
impl Default for FramebufferAttachmentsCreateInfo {
    fn default() -> FramebufferAttachmentsCreateInfo {
        FramebufferAttachmentsCreateInfo {
            s_type: crate::vk1_0::StructureType::FRAMEBUFFER_ATTACHMENTS_CREATE_INFO,
            p_next: std::ptr::null(),
            attachment_image_info_count: Default::default(),
            p_attachment_image_infos: std::ptr::null(),
        }
    }
}
#[doc = "Used by [`FramebufferAttachmentsCreateInfo::extend`](struct.FramebufferAttachmentsCreateInfo.html#method.extend)"]
pub trait ExtendableByFramebufferAttachmentsCreateInfo {}
impl ExtendableByFramebufferAttachmentsCreateInfo for crate::vk1_0::FramebufferCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`FramebufferAttachmentsCreateInfo`](struct.FramebufferAttachmentsCreateInfo.html)"]
#[repr(transparent)]
pub struct FramebufferAttachmentsCreateInfoBuilder<'a>(
    FramebufferAttachmentsCreateInfo,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> FramebufferAttachmentsCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> FramebufferAttachmentsCreateInfoBuilder<'a> {
        FramebufferAttachmentsCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn attachment_image_infos(
        mut self,
        attachment_image_infos: &'a [crate::vk1_2::FramebufferAttachmentImageInfoBuilder],
    ) -> Self {
        self.0.attachment_image_info_count = attachment_image_infos.len() as _;
        self.0.p_attachment_image_infos = attachment_image_infos.as_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> FramebufferAttachmentsCreateInfo {
        self.0
    }
}
impl<'a> std::fmt::Debug for FramebufferAttachmentsCreateInfoBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for FramebufferAttachmentsCreateInfoBuilder<'a> {
    type Target = FramebufferAttachmentsCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for FramebufferAttachmentsCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFramebufferAttachmentImageInfo.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FramebufferAttachmentImageInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::vk1_0::ImageCreateFlags,
    pub usage: crate::vk1_0::ImageUsageFlags,
    pub width: u32,
    pub height: u32,
    pub layer_count: u32,
    pub view_format_count: u32,
    pub p_view_formats: *const crate::vk1_0::Format,
}
impl FramebufferAttachmentImageInfo {
    #[inline]
    pub fn builder<'a>(self) -> FramebufferAttachmentImageInfoBuilder<'a> {
        FramebufferAttachmentImageInfoBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for FramebufferAttachmentImageInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("FramebufferAttachmentImageInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("usage", &self.usage)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("layer_count", &self.layer_count)
            .field("view_format_count", &self.view_format_count)
            .field("p_view_formats", &self.p_view_formats)
            .finish()
    }
}
impl Default for FramebufferAttachmentImageInfo {
    fn default() -> FramebufferAttachmentImageInfo {
        FramebufferAttachmentImageInfo {
            s_type: crate::vk1_0::StructureType::FRAMEBUFFER_ATTACHMENT_IMAGE_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            usage: Default::default(),
            width: Default::default(),
            height: Default::default(),
            layer_count: Default::default(),
            view_format_count: Default::default(),
            p_view_formats: std::ptr::null(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`FramebufferAttachmentImageInfo`](struct.FramebufferAttachmentImageInfo.html)"]
#[repr(transparent)]
pub struct FramebufferAttachmentImageInfoBuilder<'a>(
    FramebufferAttachmentImageInfo,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> FramebufferAttachmentImageInfoBuilder<'a> {
    #[inline]
    pub fn new() -> FramebufferAttachmentImageInfoBuilder<'a> {
        FramebufferAttachmentImageInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_0::ImageCreateFlags) -> Self {
        self.0.flags = flags;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn usage(mut self, usage: crate::vk1_0::ImageUsageFlags) -> Self {
        self.0.usage = usage;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn width(mut self, width: u32) -> Self {
        self.0.width = width;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn height(mut self, height: u32) -> Self {
        self.0.height = height;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn layer_count(mut self, layer_count: u32) -> Self {
        self.0.layer_count = layer_count;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn view_formats(mut self, view_formats: &'a [crate::vk1_0::Format]) -> Self {
        self.0.view_format_count = view_formats.len() as _;
        self.0.p_view_formats = view_formats.as_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> FramebufferAttachmentImageInfo {
        self.0
    }
}
impl<'a> std::fmt::Debug for FramebufferAttachmentImageInfoBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for FramebufferAttachmentImageInfoBuilder<'a> {
    type Target = FramebufferAttachmentImageInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for FramebufferAttachmentImageInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderPassAttachmentBeginInfo.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RenderPassAttachmentBeginInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub attachment_count: u32,
    pub p_attachments: *const crate::vk1_0::ImageView,
}
impl RenderPassAttachmentBeginInfo {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByRenderPassAttachmentBeginInfo,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> RenderPassAttachmentBeginInfoBuilder<'a> {
        RenderPassAttachmentBeginInfoBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for RenderPassAttachmentBeginInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("RenderPassAttachmentBeginInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("attachment_count", &self.attachment_count)
            .field("p_attachments", &self.p_attachments)
            .finish()
    }
}
impl Default for RenderPassAttachmentBeginInfo {
    fn default() -> RenderPassAttachmentBeginInfo {
        RenderPassAttachmentBeginInfo {
            s_type: crate::vk1_0::StructureType::RENDER_PASS_ATTACHMENT_BEGIN_INFO,
            p_next: std::ptr::null(),
            attachment_count: Default::default(),
            p_attachments: std::ptr::null(),
        }
    }
}
#[doc = "Used by [`RenderPassAttachmentBeginInfo::extend`](struct.RenderPassAttachmentBeginInfo.html#method.extend)"]
pub trait ExtendableByRenderPassAttachmentBeginInfo {}
impl ExtendableByRenderPassAttachmentBeginInfo for crate::vk1_0::RenderPassBeginInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`RenderPassAttachmentBeginInfo`](struct.RenderPassAttachmentBeginInfo.html)"]
#[repr(transparent)]
pub struct RenderPassAttachmentBeginInfoBuilder<'a>(
    RenderPassAttachmentBeginInfo,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> RenderPassAttachmentBeginInfoBuilder<'a> {
    #[inline]
    pub fn new() -> RenderPassAttachmentBeginInfoBuilder<'a> {
        RenderPassAttachmentBeginInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn attachments(mut self, attachments: &'a [crate::vk1_0::ImageView]) -> Self {
        self.0.attachment_count = attachments.len() as _;
        self.0.p_attachments = attachments.as_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> RenderPassAttachmentBeginInfo {
        self.0
    }
}
impl<'a> std::fmt::Debug for RenderPassAttachmentBeginInfoBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for RenderPassAttachmentBeginInfoBuilder<'a> {
    type Target = RenderPassAttachmentBeginInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for RenderPassAttachmentBeginInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceUniformBufferStandardLayoutFeatures.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceUniformBufferStandardLayoutFeatures {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub uniform_buffer_standard_layout: crate::vk1_0::Bool32,
}
impl PhysicalDeviceUniformBufferStandardLayoutFeatures {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceUniformBufferStandardLayoutFeatures,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceUniformBufferStandardLayoutFeaturesBuilder<'a> {
        PhysicalDeviceUniformBufferStandardLayoutFeaturesBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceUniformBufferStandardLayoutFeatures {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceUniformBufferStandardLayoutFeatures")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "uniform_buffer_standard_layout",
                &(self.uniform_buffer_standard_layout != 0),
            )
            .finish()
    }
}
impl Default for PhysicalDeviceUniformBufferStandardLayoutFeatures {
    fn default() -> PhysicalDeviceUniformBufferStandardLayoutFeatures {
        PhysicalDeviceUniformBufferStandardLayoutFeatures {
            s_type:
                crate::vk1_0::StructureType::PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES,
            p_next: std::ptr::null_mut(),
            uniform_buffer_standard_layout: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceUniformBufferStandardLayoutFeatures::extend`](struct.PhysicalDeviceUniformBufferStandardLayoutFeatures.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceUniformBufferStandardLayoutFeatures {}
impl ExtendableByPhysicalDeviceUniformBufferStandardLayoutFeatures
    for crate::vk1_1::PhysicalDeviceFeatures2
{
}
impl ExtendableByPhysicalDeviceUniformBufferStandardLayoutFeatures
    for crate::vk1_0::DeviceCreateInfo
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceUniformBufferStandardLayoutFeatures`](struct.PhysicalDeviceUniformBufferStandardLayoutFeatures.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceUniformBufferStandardLayoutFeaturesBuilder<'a>(
    PhysicalDeviceUniformBufferStandardLayoutFeatures,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceUniformBufferStandardLayoutFeaturesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceUniformBufferStandardLayoutFeaturesBuilder<'a> {
        PhysicalDeviceUniformBufferStandardLayoutFeaturesBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn uniform_buffer_standard_layout(mut self, uniform_buffer_standard_layout: bool) -> Self {
        self.0.uniform_buffer_standard_layout = uniform_buffer_standard_layout as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceUniformBufferStandardLayoutFeatures {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceUniformBufferStandardLayoutFeaturesBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceUniformBufferStandardLayoutFeaturesBuilder<'a> {
    type Target = PhysicalDeviceUniformBufferStandardLayoutFeatures;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceUniformBufferStandardLayoutFeaturesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceShaderSubgroupExtendedTypesFeatures {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub shader_subgroup_extended_types: crate::vk1_0::Bool32,
}
impl PhysicalDeviceShaderSubgroupExtendedTypesFeatures {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceShaderSubgroupExtendedTypesFeatures,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceShaderSubgroupExtendedTypesFeaturesBuilder<'a> {
        PhysicalDeviceShaderSubgroupExtendedTypesFeaturesBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceShaderSubgroupExtendedTypesFeatures {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceShaderSubgroupExtendedTypesFeatures")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "shader_subgroup_extended_types",
                &(self.shader_subgroup_extended_types != 0),
            )
            .finish()
    }
}
impl Default for PhysicalDeviceShaderSubgroupExtendedTypesFeatures {
    fn default() -> PhysicalDeviceShaderSubgroupExtendedTypesFeatures {
        PhysicalDeviceShaderSubgroupExtendedTypesFeatures {
            s_type:
                crate::vk1_0::StructureType::PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES,
            p_next: std::ptr::null_mut(),
            shader_subgroup_extended_types: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceShaderSubgroupExtendedTypesFeatures::extend`](struct.PhysicalDeviceShaderSubgroupExtendedTypesFeatures.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceShaderSubgroupExtendedTypesFeatures {}
impl ExtendableByPhysicalDeviceShaderSubgroupExtendedTypesFeatures
    for crate::vk1_1::PhysicalDeviceFeatures2
{
}
impl ExtendableByPhysicalDeviceShaderSubgroupExtendedTypesFeatures
    for crate::vk1_0::DeviceCreateInfo
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceShaderSubgroupExtendedTypesFeatures`](struct.PhysicalDeviceShaderSubgroupExtendedTypesFeatures.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceShaderSubgroupExtendedTypesFeaturesBuilder<'a>(
    PhysicalDeviceShaderSubgroupExtendedTypesFeatures,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceShaderSubgroupExtendedTypesFeaturesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceShaderSubgroupExtendedTypesFeaturesBuilder<'a> {
        PhysicalDeviceShaderSubgroupExtendedTypesFeaturesBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_subgroup_extended_types(mut self, shader_subgroup_extended_types: bool) -> Self {
        self.0.shader_subgroup_extended_types = shader_subgroup_extended_types as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceShaderSubgroupExtendedTypesFeatures {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceShaderSubgroupExtendedTypesFeaturesBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceShaderSubgroupExtendedTypesFeaturesBuilder<'a> {
    type Target = PhysicalDeviceShaderSubgroupExtendedTypesFeatures;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceShaderSubgroupExtendedTypesFeaturesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceSeparateDepthStencilLayoutsFeatures {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub separate_depth_stencil_layouts: crate::vk1_0::Bool32,
}
impl PhysicalDeviceSeparateDepthStencilLayoutsFeatures {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceSeparateDepthStencilLayoutsFeatures,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceSeparateDepthStencilLayoutsFeaturesBuilder<'a> {
        PhysicalDeviceSeparateDepthStencilLayoutsFeaturesBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceSeparateDepthStencilLayoutsFeatures {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceSeparateDepthStencilLayoutsFeatures")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "separate_depth_stencil_layouts",
                &(self.separate_depth_stencil_layouts != 0),
            )
            .finish()
    }
}
impl Default for PhysicalDeviceSeparateDepthStencilLayoutsFeatures {
    fn default() -> PhysicalDeviceSeparateDepthStencilLayoutsFeatures {
        PhysicalDeviceSeparateDepthStencilLayoutsFeatures {
            s_type:
                crate::vk1_0::StructureType::PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES,
            p_next: std::ptr::null_mut(),
            separate_depth_stencil_layouts: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceSeparateDepthStencilLayoutsFeatures::extend`](struct.PhysicalDeviceSeparateDepthStencilLayoutsFeatures.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceSeparateDepthStencilLayoutsFeatures {}
impl ExtendableByPhysicalDeviceSeparateDepthStencilLayoutsFeatures
    for crate::vk1_1::PhysicalDeviceFeatures2
{
}
impl ExtendableByPhysicalDeviceSeparateDepthStencilLayoutsFeatures
    for crate::vk1_0::DeviceCreateInfo
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceSeparateDepthStencilLayoutsFeatures`](struct.PhysicalDeviceSeparateDepthStencilLayoutsFeatures.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceSeparateDepthStencilLayoutsFeaturesBuilder<'a>(
    PhysicalDeviceSeparateDepthStencilLayoutsFeatures,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceSeparateDepthStencilLayoutsFeaturesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceSeparateDepthStencilLayoutsFeaturesBuilder<'a> {
        PhysicalDeviceSeparateDepthStencilLayoutsFeaturesBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn separate_depth_stencil_layouts(mut self, separate_depth_stencil_layouts: bool) -> Self {
        self.0.separate_depth_stencil_layouts = separate_depth_stencil_layouts as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceSeparateDepthStencilLayoutsFeatures {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceSeparateDepthStencilLayoutsFeaturesBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceSeparateDepthStencilLayoutsFeaturesBuilder<'a> {
    type Target = PhysicalDeviceSeparateDepthStencilLayoutsFeatures;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceSeparateDepthStencilLayoutsFeaturesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAttachmentReferenceStencilLayout.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AttachmentReferenceStencilLayout {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub stencil_layout: crate::vk1_0::ImageLayout,
}
impl AttachmentReferenceStencilLayout {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByAttachmentReferenceStencilLayout,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> AttachmentReferenceStencilLayoutBuilder<'a> {
        AttachmentReferenceStencilLayoutBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for AttachmentReferenceStencilLayout {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("AttachmentReferenceStencilLayout")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("stencil_layout", &self.stencil_layout)
            .finish()
    }
}
impl Default for AttachmentReferenceStencilLayout {
    fn default() -> AttachmentReferenceStencilLayout {
        AttachmentReferenceStencilLayout {
            s_type: crate::vk1_0::StructureType::ATTACHMENT_REFERENCE_STENCIL_LAYOUT,
            p_next: std::ptr::null_mut(),
            stencil_layout: Default::default(),
        }
    }
}
#[doc = "Used by [`AttachmentReferenceStencilLayout::extend`](struct.AttachmentReferenceStencilLayout.html#method.extend)"]
pub trait ExtendableByAttachmentReferenceStencilLayout {}
impl ExtendableByAttachmentReferenceStencilLayout for crate::vk1_2::AttachmentReference2 {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`AttachmentReferenceStencilLayout`](struct.AttachmentReferenceStencilLayout.html)"]
#[repr(transparent)]
pub struct AttachmentReferenceStencilLayoutBuilder<'a>(
    AttachmentReferenceStencilLayout,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> AttachmentReferenceStencilLayoutBuilder<'a> {
    #[inline]
    pub fn new() -> AttachmentReferenceStencilLayoutBuilder<'a> {
        AttachmentReferenceStencilLayoutBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn stencil_layout(mut self, stencil_layout: crate::vk1_0::ImageLayout) -> Self {
        self.0.stencil_layout = stencil_layout;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> AttachmentReferenceStencilLayout {
        self.0
    }
}
impl<'a> std::fmt::Debug for AttachmentReferenceStencilLayoutBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for AttachmentReferenceStencilLayoutBuilder<'a> {
    type Target = AttachmentReferenceStencilLayout;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for AttachmentReferenceStencilLayoutBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAttachmentDescriptionStencilLayout.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AttachmentDescriptionStencilLayout {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub stencil_initial_layout: crate::vk1_0::ImageLayout,
    pub stencil_final_layout: crate::vk1_0::ImageLayout,
}
impl AttachmentDescriptionStencilLayout {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByAttachmentDescriptionStencilLayout,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> AttachmentDescriptionStencilLayoutBuilder<'a> {
        AttachmentDescriptionStencilLayoutBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for AttachmentDescriptionStencilLayout {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("AttachmentDescriptionStencilLayout")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("stencil_initial_layout", &self.stencil_initial_layout)
            .field("stencil_final_layout", &self.stencil_final_layout)
            .finish()
    }
}
impl Default for AttachmentDescriptionStencilLayout {
    fn default() -> AttachmentDescriptionStencilLayout {
        AttachmentDescriptionStencilLayout {
            s_type: crate::vk1_0::StructureType::ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT,
            p_next: std::ptr::null_mut(),
            stencil_initial_layout: Default::default(),
            stencil_final_layout: Default::default(),
        }
    }
}
#[doc = "Used by [`AttachmentDescriptionStencilLayout::extend`](struct.AttachmentDescriptionStencilLayout.html#method.extend)"]
pub trait ExtendableByAttachmentDescriptionStencilLayout {}
impl ExtendableByAttachmentDescriptionStencilLayout for crate::vk1_2::AttachmentDescription2 {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`AttachmentDescriptionStencilLayout`](struct.AttachmentDescriptionStencilLayout.html)"]
#[repr(transparent)]
pub struct AttachmentDescriptionStencilLayoutBuilder<'a>(
    AttachmentDescriptionStencilLayout,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> AttachmentDescriptionStencilLayoutBuilder<'a> {
    #[inline]
    pub fn new() -> AttachmentDescriptionStencilLayoutBuilder<'a> {
        AttachmentDescriptionStencilLayoutBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn stencil_initial_layout(
        mut self,
        stencil_initial_layout: crate::vk1_0::ImageLayout,
    ) -> Self {
        self.0.stencil_initial_layout = stencil_initial_layout;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn stencil_final_layout(mut self, stencil_final_layout: crate::vk1_0::ImageLayout) -> Self {
        self.0.stencil_final_layout = stencil_final_layout;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> AttachmentDescriptionStencilLayout {
        self.0
    }
}
impl<'a> std::fmt::Debug for AttachmentDescriptionStencilLayoutBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for AttachmentDescriptionStencilLayoutBuilder<'a> {
    type Target = AttachmentDescriptionStencilLayout;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for AttachmentDescriptionStencilLayoutBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceHostQueryResetFeatures.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceHostQueryResetFeatures {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub host_query_reset: crate::vk1_0::Bool32,
}
impl PhysicalDeviceHostQueryResetFeatures {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceHostQueryResetFeatures,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceHostQueryResetFeaturesBuilder<'a> {
        PhysicalDeviceHostQueryResetFeaturesBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceHostQueryResetFeatures {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceHostQueryResetFeatures")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("host_query_reset", &(self.host_query_reset != 0))
            .finish()
    }
}
impl Default for PhysicalDeviceHostQueryResetFeatures {
    fn default() -> PhysicalDeviceHostQueryResetFeatures {
        PhysicalDeviceHostQueryResetFeatures {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES,
            p_next: std::ptr::null_mut(),
            host_query_reset: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceHostQueryResetFeatures::extend`](struct.PhysicalDeviceHostQueryResetFeatures.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceHostQueryResetFeatures {}
impl ExtendableByPhysicalDeviceHostQueryResetFeatures for crate::vk1_1::PhysicalDeviceFeatures2 {}
impl ExtendableByPhysicalDeviceHostQueryResetFeatures for crate::vk1_0::DeviceCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceHostQueryResetFeatures`](struct.PhysicalDeviceHostQueryResetFeatures.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceHostQueryResetFeaturesBuilder<'a>(
    PhysicalDeviceHostQueryResetFeatures,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceHostQueryResetFeaturesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceHostQueryResetFeaturesBuilder<'a> {
        PhysicalDeviceHostQueryResetFeaturesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn host_query_reset(mut self, host_query_reset: bool) -> Self {
        self.0.host_query_reset = host_query_reset as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceHostQueryResetFeatures {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceHostQueryResetFeaturesBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceHostQueryResetFeaturesBuilder<'a> {
    type Target = PhysicalDeviceHostQueryResetFeatures;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceHostQueryResetFeaturesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreType.html) · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct SemaphoreType(pub i32);
#[doc = "[Part of `vk1_2`](../vk1_2/index.html)"]
impl SemaphoreType {
    pub const BINARY: Self = Self(0);
    pub const TIMELINE: Self = Self(1);
}
#[doc = "[Part of `extensions::khr_timeline_semaphore`](../extensions/khr_timeline_semaphore/index.html)"]
impl SemaphoreType {
    pub const BINARY_KHR: Self = Self::BINARY;
    pub const TIMELINE_KHR: Self = Self::TIMELINE;
}
impl std::fmt::Debug for SemaphoreType {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::BINARY => "BINARY",
            &Self::TIMELINE => "TIMELINE",
            _ => "Unknown enum variant",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceTimelineSemaphoreFeatures.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceTimelineSemaphoreFeatures {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub timeline_semaphore: crate::vk1_0::Bool32,
}
impl PhysicalDeviceTimelineSemaphoreFeatures {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceTimelineSemaphoreFeatures,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceTimelineSemaphoreFeaturesBuilder<'a> {
        PhysicalDeviceTimelineSemaphoreFeaturesBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceTimelineSemaphoreFeatures {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceTimelineSemaphoreFeatures")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("timeline_semaphore", &(self.timeline_semaphore != 0))
            .finish()
    }
}
impl Default for PhysicalDeviceTimelineSemaphoreFeatures {
    fn default() -> PhysicalDeviceTimelineSemaphoreFeatures {
        PhysicalDeviceTimelineSemaphoreFeatures {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES,
            p_next: std::ptr::null_mut(),
            timeline_semaphore: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceTimelineSemaphoreFeatures::extend`](struct.PhysicalDeviceTimelineSemaphoreFeatures.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceTimelineSemaphoreFeatures {}
impl ExtendableByPhysicalDeviceTimelineSemaphoreFeatures for crate::vk1_1::PhysicalDeviceFeatures2 {}
impl ExtendableByPhysicalDeviceTimelineSemaphoreFeatures for crate::vk1_0::DeviceCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceTimelineSemaphoreFeatures`](struct.PhysicalDeviceTimelineSemaphoreFeatures.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceTimelineSemaphoreFeaturesBuilder<'a>(
    PhysicalDeviceTimelineSemaphoreFeatures,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceTimelineSemaphoreFeaturesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceTimelineSemaphoreFeaturesBuilder<'a> {
        PhysicalDeviceTimelineSemaphoreFeaturesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn timeline_semaphore(mut self, timeline_semaphore: bool) -> Self {
        self.0.timeline_semaphore = timeline_semaphore as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceTimelineSemaphoreFeatures {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceTimelineSemaphoreFeaturesBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceTimelineSemaphoreFeaturesBuilder<'a> {
    type Target = PhysicalDeviceTimelineSemaphoreFeatures;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceTimelineSemaphoreFeaturesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceTimelineSemaphoreProperties.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceTimelineSemaphoreProperties {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub max_timeline_semaphore_value_difference: u64,
}
impl PhysicalDeviceTimelineSemaphoreProperties {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceTimelineSemaphoreProperties,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceTimelineSemaphorePropertiesBuilder<'a> {
        PhysicalDeviceTimelineSemaphorePropertiesBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceTimelineSemaphoreProperties {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceTimelineSemaphoreProperties")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "max_timeline_semaphore_value_difference",
                &self.max_timeline_semaphore_value_difference,
            )
            .finish()
    }
}
impl Default for PhysicalDeviceTimelineSemaphoreProperties {
    fn default() -> PhysicalDeviceTimelineSemaphoreProperties {
        PhysicalDeviceTimelineSemaphoreProperties {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES,
            p_next: std::ptr::null_mut(),
            max_timeline_semaphore_value_difference: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceTimelineSemaphoreProperties::extend`](struct.PhysicalDeviceTimelineSemaphoreProperties.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceTimelineSemaphoreProperties {}
impl ExtendableByPhysicalDeviceTimelineSemaphoreProperties
    for crate::vk1_1::PhysicalDeviceProperties2
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceTimelineSemaphoreProperties`](struct.PhysicalDeviceTimelineSemaphoreProperties.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceTimelineSemaphorePropertiesBuilder<'a>(
    PhysicalDeviceTimelineSemaphoreProperties,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceTimelineSemaphorePropertiesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceTimelineSemaphorePropertiesBuilder<'a> {
        PhysicalDeviceTimelineSemaphorePropertiesBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_timeline_semaphore_value_difference(
        mut self,
        max_timeline_semaphore_value_difference: u64,
    ) -> Self {
        self.0.max_timeline_semaphore_value_difference = max_timeline_semaphore_value_difference;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceTimelineSemaphoreProperties {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceTimelineSemaphorePropertiesBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceTimelineSemaphorePropertiesBuilder<'a> {
    type Target = PhysicalDeviceTimelineSemaphoreProperties;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceTimelineSemaphorePropertiesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreTypeCreateInfo.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SemaphoreTypeCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub semaphore_type: crate::vk1_2::SemaphoreType,
    pub initial_value: u64,
}
impl SemaphoreTypeCreateInfo {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableBySemaphoreTypeCreateInfo,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> SemaphoreTypeCreateInfoBuilder<'a> {
        SemaphoreTypeCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for SemaphoreTypeCreateInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("SemaphoreTypeCreateInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("semaphore_type", &self.semaphore_type)
            .field("initial_value", &self.initial_value)
            .finish()
    }
}
impl Default for SemaphoreTypeCreateInfo {
    fn default() -> SemaphoreTypeCreateInfo {
        SemaphoreTypeCreateInfo {
            s_type: crate::vk1_0::StructureType::SEMAPHORE_TYPE_CREATE_INFO,
            p_next: std::ptr::null(),
            semaphore_type: Default::default(),
            initial_value: Default::default(),
        }
    }
}
#[doc = "Used by [`SemaphoreTypeCreateInfo::extend`](struct.SemaphoreTypeCreateInfo.html#method.extend)"]
pub trait ExtendableBySemaphoreTypeCreateInfo {}
impl ExtendableBySemaphoreTypeCreateInfo for crate::vk1_0::SemaphoreCreateInfo {}
impl ExtendableBySemaphoreTypeCreateInfo for crate::vk1_1::PhysicalDeviceExternalSemaphoreInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`SemaphoreTypeCreateInfo`](struct.SemaphoreTypeCreateInfo.html)"]
#[repr(transparent)]
pub struct SemaphoreTypeCreateInfoBuilder<'a>(
    SemaphoreTypeCreateInfo,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> SemaphoreTypeCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> SemaphoreTypeCreateInfoBuilder<'a> {
        SemaphoreTypeCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn semaphore_type(mut self, semaphore_type: crate::vk1_2::SemaphoreType) -> Self {
        self.0.semaphore_type = semaphore_type;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn initial_value(mut self, initial_value: u64) -> Self {
        self.0.initial_value = initial_value;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> SemaphoreTypeCreateInfo {
        self.0
    }
}
impl<'a> std::fmt::Debug for SemaphoreTypeCreateInfoBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for SemaphoreTypeCreateInfoBuilder<'a> {
    type Target = SemaphoreTypeCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SemaphoreTypeCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkTimelineSemaphoreSubmitInfo.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TimelineSemaphoreSubmitInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub wait_semaphore_value_count: u32,
    pub p_wait_semaphore_values: *const u64,
    pub signal_semaphore_value_count: u32,
    pub p_signal_semaphore_values: *const u64,
}
impl TimelineSemaphoreSubmitInfo {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByTimelineSemaphoreSubmitInfo,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> TimelineSemaphoreSubmitInfoBuilder<'a> {
        TimelineSemaphoreSubmitInfoBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for TimelineSemaphoreSubmitInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("TimelineSemaphoreSubmitInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "wait_semaphore_value_count",
                &self.wait_semaphore_value_count,
            )
            .field("p_wait_semaphore_values", &self.p_wait_semaphore_values)
            .field(
                "signal_semaphore_value_count",
                &self.signal_semaphore_value_count,
            )
            .field("p_signal_semaphore_values", &self.p_signal_semaphore_values)
            .finish()
    }
}
impl Default for TimelineSemaphoreSubmitInfo {
    fn default() -> TimelineSemaphoreSubmitInfo {
        TimelineSemaphoreSubmitInfo {
            s_type: crate::vk1_0::StructureType::TIMELINE_SEMAPHORE_SUBMIT_INFO,
            p_next: std::ptr::null(),
            wait_semaphore_value_count: Default::default(),
            p_wait_semaphore_values: std::ptr::null(),
            signal_semaphore_value_count: Default::default(),
            p_signal_semaphore_values: std::ptr::null(),
        }
    }
}
#[doc = "Used by [`TimelineSemaphoreSubmitInfo::extend`](struct.TimelineSemaphoreSubmitInfo.html#method.extend)"]
pub trait ExtendableByTimelineSemaphoreSubmitInfo {}
impl ExtendableByTimelineSemaphoreSubmitInfo for crate::vk1_0::SubmitInfo {}
impl ExtendableByTimelineSemaphoreSubmitInfo for crate::vk1_0::BindSparseInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`TimelineSemaphoreSubmitInfo`](struct.TimelineSemaphoreSubmitInfo.html)"]
#[repr(transparent)]
pub struct TimelineSemaphoreSubmitInfoBuilder<'a>(
    TimelineSemaphoreSubmitInfo,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> TimelineSemaphoreSubmitInfoBuilder<'a> {
    #[inline]
    pub fn new() -> TimelineSemaphoreSubmitInfoBuilder<'a> {
        TimelineSemaphoreSubmitInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn wait_semaphore_values(mut self, wait_semaphore_values: &'a [u64]) -> Self {
        self.0.wait_semaphore_value_count = wait_semaphore_values.len() as _;
        self.0.p_wait_semaphore_values = wait_semaphore_values.as_ptr() as _;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn signal_semaphore_values(mut self, signal_semaphore_values: &'a [u64]) -> Self {
        self.0.signal_semaphore_value_count = signal_semaphore_values.len() as _;
        self.0.p_signal_semaphore_values = signal_semaphore_values.as_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> TimelineSemaphoreSubmitInfo {
        self.0
    }
}
impl<'a> std::fmt::Debug for TimelineSemaphoreSubmitInfoBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for TimelineSemaphoreSubmitInfoBuilder<'a> {
    type Target = TimelineSemaphoreSubmitInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for TimelineSemaphoreSubmitInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceBufferDeviceAddressFeatures.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceBufferDeviceAddressFeatures {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub buffer_device_address: crate::vk1_0::Bool32,
    pub buffer_device_address_capture_replay: crate::vk1_0::Bool32,
    pub buffer_device_address_multi_device: crate::vk1_0::Bool32,
}
impl PhysicalDeviceBufferDeviceAddressFeatures {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceBufferDeviceAddressFeatures,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceBufferDeviceAddressFeaturesBuilder<'a> {
        PhysicalDeviceBufferDeviceAddressFeaturesBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceBufferDeviceAddressFeatures {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceBufferDeviceAddressFeatures")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("buffer_device_address", &(self.buffer_device_address != 0))
            .field(
                "buffer_device_address_capture_replay",
                &(self.buffer_device_address_capture_replay != 0),
            )
            .field(
                "buffer_device_address_multi_device",
                &(self.buffer_device_address_multi_device != 0),
            )
            .finish()
    }
}
impl Default for PhysicalDeviceBufferDeviceAddressFeatures {
    fn default() -> PhysicalDeviceBufferDeviceAddressFeatures {
        PhysicalDeviceBufferDeviceAddressFeatures {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES,
            p_next: std::ptr::null_mut(),
            buffer_device_address: Default::default(),
            buffer_device_address_capture_replay: Default::default(),
            buffer_device_address_multi_device: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceBufferDeviceAddressFeatures::extend`](struct.PhysicalDeviceBufferDeviceAddressFeatures.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceBufferDeviceAddressFeatures {}
impl ExtendableByPhysicalDeviceBufferDeviceAddressFeatures
    for crate::vk1_1::PhysicalDeviceFeatures2
{
}
impl ExtendableByPhysicalDeviceBufferDeviceAddressFeatures for crate::vk1_0::DeviceCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceBufferDeviceAddressFeatures`](struct.PhysicalDeviceBufferDeviceAddressFeatures.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceBufferDeviceAddressFeaturesBuilder<'a>(
    PhysicalDeviceBufferDeviceAddressFeatures,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceBufferDeviceAddressFeaturesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceBufferDeviceAddressFeaturesBuilder<'a> {
        PhysicalDeviceBufferDeviceAddressFeaturesBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn buffer_device_address(mut self, buffer_device_address: bool) -> Self {
        self.0.buffer_device_address = buffer_device_address as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn buffer_device_address_capture_replay(
        mut self,
        buffer_device_address_capture_replay: bool,
    ) -> Self {
        self.0.buffer_device_address_capture_replay = buffer_device_address_capture_replay as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn buffer_device_address_multi_device(
        mut self,
        buffer_device_address_multi_device: bool,
    ) -> Self {
        self.0.buffer_device_address_multi_device = buffer_device_address_multi_device as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceBufferDeviceAddressFeatures {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceBufferDeviceAddressFeaturesBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceBufferDeviceAddressFeaturesBuilder<'a> {
    type Target = PhysicalDeviceBufferDeviceAddressFeatures;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceBufferDeviceAddressFeaturesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferOpaqueCaptureAddressCreateInfo.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BufferOpaqueCaptureAddressCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub opaque_capture_address: u64,
}
impl BufferOpaqueCaptureAddressCreateInfo {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByBufferOpaqueCaptureAddressCreateInfo,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> BufferOpaqueCaptureAddressCreateInfoBuilder<'a> {
        BufferOpaqueCaptureAddressCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for BufferOpaqueCaptureAddressCreateInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("BufferOpaqueCaptureAddressCreateInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("opaque_capture_address", &self.opaque_capture_address)
            .finish()
    }
}
impl Default for BufferOpaqueCaptureAddressCreateInfo {
    fn default() -> BufferOpaqueCaptureAddressCreateInfo {
        BufferOpaqueCaptureAddressCreateInfo {
            s_type: crate::vk1_0::StructureType::BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO,
            p_next: std::ptr::null(),
            opaque_capture_address: Default::default(),
        }
    }
}
#[doc = "Used by [`BufferOpaqueCaptureAddressCreateInfo::extend`](struct.BufferOpaqueCaptureAddressCreateInfo.html#method.extend)"]
pub trait ExtendableByBufferOpaqueCaptureAddressCreateInfo {}
impl ExtendableByBufferOpaqueCaptureAddressCreateInfo for crate::vk1_0::BufferCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`BufferOpaqueCaptureAddressCreateInfo`](struct.BufferOpaqueCaptureAddressCreateInfo.html)"]
#[repr(transparent)]
pub struct BufferOpaqueCaptureAddressCreateInfoBuilder<'a>(
    BufferOpaqueCaptureAddressCreateInfo,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> BufferOpaqueCaptureAddressCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> BufferOpaqueCaptureAddressCreateInfoBuilder<'a> {
        BufferOpaqueCaptureAddressCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn opaque_capture_address(mut self, opaque_capture_address: u64) -> Self {
        self.0.opaque_capture_address = opaque_capture_address;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> BufferOpaqueCaptureAddressCreateInfo {
        self.0
    }
}
impl<'a> std::fmt::Debug for BufferOpaqueCaptureAddressCreateInfoBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for BufferOpaqueCaptureAddressCreateInfoBuilder<'a> {
    type Target = BufferOpaqueCaptureAddressCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for BufferOpaqueCaptureAddressCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryOpaqueCaptureAddressAllocateInfo.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryOpaqueCaptureAddressAllocateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub opaque_capture_address: u64,
}
impl MemoryOpaqueCaptureAddressAllocateInfo {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByMemoryOpaqueCaptureAddressAllocateInfo,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> MemoryOpaqueCaptureAddressAllocateInfoBuilder<'a> {
        MemoryOpaqueCaptureAddressAllocateInfoBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for MemoryOpaqueCaptureAddressAllocateInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("MemoryOpaqueCaptureAddressAllocateInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("opaque_capture_address", &self.opaque_capture_address)
            .finish()
    }
}
impl Default for MemoryOpaqueCaptureAddressAllocateInfo {
    fn default() -> MemoryOpaqueCaptureAddressAllocateInfo {
        MemoryOpaqueCaptureAddressAllocateInfo {
            s_type: crate::vk1_0::StructureType::MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO,
            p_next: std::ptr::null(),
            opaque_capture_address: Default::default(),
        }
    }
}
#[doc = "Used by [`MemoryOpaqueCaptureAddressAllocateInfo::extend`](struct.MemoryOpaqueCaptureAddressAllocateInfo.html#method.extend)"]
pub trait ExtendableByMemoryOpaqueCaptureAddressAllocateInfo {}
impl ExtendableByMemoryOpaqueCaptureAddressAllocateInfo for crate::vk1_0::MemoryAllocateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`MemoryOpaqueCaptureAddressAllocateInfo`](struct.MemoryOpaqueCaptureAddressAllocateInfo.html)"]
#[repr(transparent)]
pub struct MemoryOpaqueCaptureAddressAllocateInfoBuilder<'a>(
    MemoryOpaqueCaptureAddressAllocateInfo,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> MemoryOpaqueCaptureAddressAllocateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> MemoryOpaqueCaptureAddressAllocateInfoBuilder<'a> {
        MemoryOpaqueCaptureAddressAllocateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn opaque_capture_address(mut self, opaque_capture_address: u64) -> Self {
        self.0.opaque_capture_address = opaque_capture_address;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> MemoryOpaqueCaptureAddressAllocateInfo {
        self.0
    }
}
impl<'a> std::fmt::Debug for MemoryOpaqueCaptureAddressAllocateInfoBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for MemoryOpaqueCaptureAddressAllocateInfoBuilder<'a> {
    type Target = MemoryOpaqueCaptureAddressAllocateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for MemoryOpaqueCaptureAddressAllocateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
