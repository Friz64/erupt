# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_KHR_create_renderpass2.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_CREATE_RENDERPASS_2_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_CREATE_RENDERPASS_2_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_create_renderpass2");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateRenderPass2KHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateRenderPass2KHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_create_info: *const crate::vk1_2::RenderPassCreateInfo2,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
    p_render_pass: *mut crate::vk1_0::RenderPass,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBeginRenderPass2KHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBeginRenderPass2KHR = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    p_render_pass_begin: *const crate::vk1_0::RenderPassBeginInfo,
    p_subpass_begin_info: *const crate::vk1_2::SubpassBeginInfo,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdNextSubpass2KHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdNextSubpass2KHR = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    p_subpass_begin_info: *const crate::vk1_2::SubpassBeginInfo,
    p_subpass_end_info: *const crate::vk1_2::SubpassEndInfo,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdEndRenderPass2KHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdEndRenderPass2KHR = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    p_subpass_end_info: *const crate::vk1_2::SubpassEndInfo,
) -> std::ffi::c_void;
#[doc = "Provides Device Commands for [`KhrCreateRenderpass2DeviceLoaderExt`](trait.KhrCreateRenderpass2DeviceLoaderExt.html)"]
pub struct KhrCreateRenderpass2DeviceCommands {
    pub create_render_pass2_khr: Option<PFN_vkCreateRenderPass2KHR>,
    pub cmd_begin_render_pass2_khr: Option<PFN_vkCmdBeginRenderPass2KHR>,
    pub cmd_next_subpass2_khr: Option<PFN_vkCmdNextSubpass2KHR>,
    pub cmd_end_render_pass2_khr: Option<PFN_vkCmdEndRenderPass2KHR>,
}
impl KhrCreateRenderpass2DeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<KhrCreateRenderpass2DeviceCommands> {
        unsafe {
            let mut success = false;
            let table = KhrCreateRenderpass2DeviceCommands {
                create_render_pass2_khr: std::mem::transmute({
                    let symbol = loader.symbol("vkCreateRenderPass2KHR");
                    success |= symbol.is_some();
                    symbol
                }),
                cmd_begin_render_pass2_khr: std::mem::transmute({
                    let symbol = loader.symbol("vkCmdBeginRenderPass2KHR");
                    success |= symbol.is_some();
                    symbol
                }),
                cmd_next_subpass2_khr: std::mem::transmute({
                    let symbol = loader.symbol("vkCmdNextSubpass2KHR");
                    success |= symbol.is_some();
                    symbol
                }),
                cmd_end_render_pass2_khr: std::mem::transmute({
                    let symbol = loader.symbol("vkCmdEndRenderPass2KHR");
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
fn device_commands(loader: &crate::DeviceLoader) -> &KhrCreateRenderpass2DeviceCommands {
    loader
        .khr_create_renderpass2
        .as_ref()
        .expect("`khr_create_renderpass2` not loaded")
}
#[doc = "Provides high level command wrappers for [`KhrCreateRenderpass2DeviceCommands`](struct.KhrCreateRenderpass2DeviceCommands.html)"]
pub trait KhrCreateRenderpass2DeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateRenderPass2KHR.html) · Device Command"]
    unsafe fn create_render_pass2_khr(
        &self,
        create_info: &crate::vk1_2::RenderPassCreateInfo2,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        render_pass: Option<crate::vk1_0::RenderPass>,
    ) -> crate::utils::VulkanResult<crate::vk1_0::RenderPass>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBeginRenderPass2KHR.html) · Device Command"]
    unsafe fn cmd_begin_render_pass2_khr(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        render_pass_begin: &crate::vk1_0::RenderPassBeginInfo,
        subpass_begin_info: &crate::vk1_2::SubpassBeginInfo,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdNextSubpass2KHR.html) · Device Command"]
    unsafe fn cmd_next_subpass2_khr(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        subpass_begin_info: &crate::vk1_2::SubpassBeginInfo,
        subpass_end_info: &crate::vk1_2::SubpassEndInfo,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdEndRenderPass2KHR.html) · Device Command"]
    unsafe fn cmd_end_render_pass2_khr(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        subpass_end_info: &crate::vk1_2::SubpassEndInfo,
    ) -> ();
}
impl KhrCreateRenderpass2DeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateRenderPass2KHR.html) · Device Command"]
    unsafe fn create_render_pass2_khr(
        &self,
        create_info: &crate::vk1_2::RenderPassCreateInfo2,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        render_pass: Option<crate::vk1_0::RenderPass>,
    ) -> crate::utils::VulkanResult<crate::vk1_0::RenderPass> {
        let function = device_commands(self)
            .create_render_pass2_khr
            .as_ref()
            .expect("`create_render_pass2_khr` not available");
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
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBeginRenderPass2KHR.html) · Device Command"]
    unsafe fn cmd_begin_render_pass2_khr(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        render_pass_begin: &crate::vk1_0::RenderPassBeginInfo,
        subpass_begin_info: &crate::vk1_2::SubpassBeginInfo,
    ) -> () {
        let function = device_commands(self)
            .cmd_begin_render_pass2_khr
            .as_ref()
            .expect("`cmd_begin_render_pass2_khr` not available");
        let _val = function(command_buffer, render_pass_begin, subpass_begin_info);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdNextSubpass2KHR.html) · Device Command"]
    unsafe fn cmd_next_subpass2_khr(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        subpass_begin_info: &crate::vk1_2::SubpassBeginInfo,
        subpass_end_info: &crate::vk1_2::SubpassEndInfo,
    ) -> () {
        let function = device_commands(self)
            .cmd_next_subpass2_khr
            .as_ref()
            .expect("`cmd_next_subpass2_khr` not available");
        let _val = function(command_buffer, subpass_begin_info, subpass_end_info);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdEndRenderPass2KHR.html) · Device Command"]
    unsafe fn cmd_end_render_pass2_khr(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        subpass_end_info: &crate::vk1_2::SubpassEndInfo,
    ) -> () {
        let function = device_commands(self)
            .cmd_end_render_pass2_khr
            .as_ref()
            .expect("`cmd_end_render_pass2_khr` not available");
        let _val = function(command_buffer, subpass_end_info);
        ()
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderPassCreateInfo2KHR.html) · Alias"]
pub type RenderPassCreateInfo2KHR = crate::vk1_2::RenderPassCreateInfo2;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAttachmentDescription2KHR.html) · Alias"]
pub type AttachmentDescription2KHR = crate::vk1_2::AttachmentDescription2;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAttachmentReference2KHR.html) · Alias"]
pub type AttachmentReference2KHR = crate::vk1_2::AttachmentReference2;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubpassDescription2KHR.html) · Alias"]
pub type SubpassDescription2KHR = crate::vk1_2::SubpassDescription2;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubpassDependency2KHR.html) · Alias"]
pub type SubpassDependency2KHR = crate::vk1_2::SubpassDependency2;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubpassBeginInfoKHR.html) · Alias"]
pub type SubpassBeginInfoKHR = crate::vk1_2::SubpassBeginInfo;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubpassEndInfoKHR.html) · Alias"]
pub type SubpassEndInfoKHR = crate::vk1_2::SubpassEndInfo;
