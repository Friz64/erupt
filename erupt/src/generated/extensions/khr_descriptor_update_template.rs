# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_KHR_descriptor_update_template.html)\n\n## Extends\n- [`DebugReportObjectTypeEXT`](../../extensions/ext_debug_report/struct.DebugReportObjectTypeEXT.html)\n- [`DescriptorUpdateTemplateType`](../../vk1_1/struct.DescriptorUpdateTemplateType.html)\n- [`ObjectType`](../../vk1_0/struct.ObjectType.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_DESCRIPTOR_UPDATE_TEMPLATE_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_DESCRIPTOR_UPDATE_TEMPLATE_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_descriptor_update_template");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDescriptorUpdateTemplateKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateDescriptorUpdateTemplateKHR =
    unsafe extern "system" fn(
        device: crate::vk1_0::Device,
        p_create_info: *const crate::vk1_1::DescriptorUpdateTemplateCreateInfo,
        p_allocator: *const crate::vk1_0::AllocationCallbacks,
        p_descriptor_update_template: *mut crate::vk1_1::DescriptorUpdateTemplate,
    ) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyDescriptorUpdateTemplateKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyDescriptorUpdateTemplateKHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    descriptor_update_template: crate::vk1_1::DescriptorUpdateTemplate,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkUpdateDescriptorSetWithTemplateKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkUpdateDescriptorSetWithTemplateKHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    descriptor_set: crate::vk1_0::DescriptorSet,
    descriptor_update_template: crate::vk1_1::DescriptorUpdateTemplate,
    p_data: *const std::ffi::c_void,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdPushDescriptorSetWithTemplateKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdPushDescriptorSetWithTemplateKHR = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    descriptor_update_template: crate::vk1_1::DescriptorUpdateTemplate,
    layout: crate::vk1_0::PipelineLayout,
    set: u32,
    p_data: *const std::ffi::c_void,
) -> std::ffi::c_void;
#[doc = "Provides Device Commands for [`KhrDescriptorUpdateTemplateDeviceLoaderExt`](trait.KhrDescriptorUpdateTemplateDeviceLoaderExt.html)"]
pub struct KhrDescriptorUpdateTemplateDeviceCommands {
    pub create_descriptor_update_template_khr: Option<PFN_vkCreateDescriptorUpdateTemplateKHR>,
    pub destroy_descriptor_update_template_khr: Option<PFN_vkDestroyDescriptorUpdateTemplateKHR>,
    pub update_descriptor_set_with_template_khr: Option<PFN_vkUpdateDescriptorSetWithTemplateKHR>,
    pub cmd_push_descriptor_set_with_template_khr:
        Option<PFN_vkCmdPushDescriptorSetWithTemplateKHR>,
}
impl KhrDescriptorUpdateTemplateDeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<KhrDescriptorUpdateTemplateDeviceCommands> {
        unsafe {
            let mut success = false;
            let table = KhrDescriptorUpdateTemplateDeviceCommands {
                create_descriptor_update_template_khr: std::mem::transmute({
                    let symbol = loader.symbol("vkCreateDescriptorUpdateTemplateKHR");
                    success |= symbol.is_some();
                    symbol
                }),
                destroy_descriptor_update_template_khr: std::mem::transmute({
                    let symbol = loader.symbol("vkDestroyDescriptorUpdateTemplateKHR");
                    success |= symbol.is_some();
                    symbol
                }),
                update_descriptor_set_with_template_khr: std::mem::transmute({
                    let symbol = loader.symbol("vkUpdateDescriptorSetWithTemplateKHR");
                    success |= symbol.is_some();
                    symbol
                }),
                cmd_push_descriptor_set_with_template_khr: std::mem::transmute({
                    let symbol = loader.symbol("vkCmdPushDescriptorSetWithTemplateKHR");
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
fn device_commands(loader: &crate::DeviceLoader) -> &KhrDescriptorUpdateTemplateDeviceCommands {
    loader
        .khr_descriptor_update_template
        .as_ref()
        .expect("`khr_descriptor_update_template` not loaded")
}
#[doc = "Provides high level command wrappers for [`KhrDescriptorUpdateTemplateDeviceCommands`](struct.KhrDescriptorUpdateTemplateDeviceCommands.html)"]
pub trait KhrDescriptorUpdateTemplateDeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDescriptorUpdateTemplateKHR.html) · Device Command"]
    unsafe fn create_descriptor_update_template_khr(
        &self,
        create_info: &crate::vk1_1::DescriptorUpdateTemplateCreateInfo,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        descriptor_update_template: Option<crate::vk1_1::DescriptorUpdateTemplate>,
    ) -> crate::utils::VulkanResult<crate::vk1_1::DescriptorUpdateTemplate>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyDescriptorUpdateTemplateKHR.html) · Device Command"]
    unsafe fn destroy_descriptor_update_template_khr(
        &self,
        descriptor_update_template: crate::vk1_1::DescriptorUpdateTemplate,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkUpdateDescriptorSetWithTemplateKHR.html) · Device Command"]
    unsafe fn update_descriptor_set_with_template_khr(
        &self,
        descriptor_set: crate::vk1_0::DescriptorSet,
        descriptor_update_template: crate::vk1_1::DescriptorUpdateTemplate,
        data: *const std::ffi::c_void,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdPushDescriptorSetWithTemplateKHR.html) · Device Command"]
    unsafe fn cmd_push_descriptor_set_with_template_khr(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        descriptor_update_template: crate::vk1_1::DescriptorUpdateTemplate,
        layout: crate::vk1_0::PipelineLayout,
        set: u32,
        data: *const std::ffi::c_void,
    ) -> ();
}
impl KhrDescriptorUpdateTemplateDeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDescriptorUpdateTemplateKHR.html) · Device Command"]
    unsafe fn create_descriptor_update_template_khr(
        &self,
        create_info: &crate::vk1_1::DescriptorUpdateTemplateCreateInfo,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        descriptor_update_template: Option<crate::vk1_1::DescriptorUpdateTemplate>,
    ) -> crate::utils::VulkanResult<crate::vk1_1::DescriptorUpdateTemplate> {
        let function = device_commands(self)
            .create_descriptor_update_template_khr
            .as_ref()
            .expect("`create_descriptor_update_template_khr` not available");
        let mut descriptor_update_template =
            descriptor_update_template.unwrap_or_else(|| Default::default());
        let _val = function(
            self.handle,
            create_info,
            if let Some(allocator) = allocator {
                allocator
            } else {
                std::ptr::null()
            },
            &mut descriptor_update_template,
        );
        crate::utils::VulkanResult::new(_val, descriptor_update_template)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyDescriptorUpdateTemplateKHR.html) · Device Command"]
    unsafe fn destroy_descriptor_update_template_khr(
        &self,
        descriptor_update_template: crate::vk1_1::DescriptorUpdateTemplate,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
    ) -> () {
        let function = device_commands(self)
            .destroy_descriptor_update_template_khr
            .as_ref()
            .expect("`destroy_descriptor_update_template_khr` not available");
        let _val = function(
            self.handle,
            descriptor_update_template,
            if let Some(allocator) = allocator {
                allocator
            } else {
                std::ptr::null()
            },
        );
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkUpdateDescriptorSetWithTemplateKHR.html) · Device Command"]
    unsafe fn update_descriptor_set_with_template_khr(
        &self,
        descriptor_set: crate::vk1_0::DescriptorSet,
        descriptor_update_template: crate::vk1_1::DescriptorUpdateTemplate,
        data: *const std::ffi::c_void,
    ) -> () {
        let function = device_commands(self)
            .update_descriptor_set_with_template_khr
            .as_ref()
            .expect("`update_descriptor_set_with_template_khr` not available");
        let _val = function(
            self.handle,
            descriptor_set,
            descriptor_update_template,
            data,
        );
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdPushDescriptorSetWithTemplateKHR.html) · Device Command"]
    unsafe fn cmd_push_descriptor_set_with_template_khr(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        descriptor_update_template: crate::vk1_1::DescriptorUpdateTemplate,
        layout: crate::vk1_0::PipelineLayout,
        set: u32,
        data: *const std::ffi::c_void,
    ) -> () {
        let function = device_commands(self)
            .cmd_push_descriptor_set_with_template_khr
            .as_ref()
            .expect("`cmd_push_descriptor_set_with_template_khr` not available");
        let _val = function(
            command_buffer,
            descriptor_update_template,
            layout,
            set,
            data,
        );
        ()
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorUpdateTemplateKHR.html) · Alias"]
pub type DescriptorUpdateTemplateKHR = crate::vk1_1::DescriptorUpdateTemplate;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorUpdateTemplateCreateFlagsKHR.html) · Alias"]
pub type DescriptorUpdateTemplateCreateFlagsKHR = crate::vk1_1::DescriptorUpdateTemplateCreateFlags;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorUpdateTemplateTypeKHR.html) · Alias"]
pub type DescriptorUpdateTemplateTypeKHR = crate::vk1_1::DescriptorUpdateTemplateType;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorUpdateTemplateEntryKHR.html) · Alias"]
pub type DescriptorUpdateTemplateEntryKHR = crate::vk1_1::DescriptorUpdateTemplateEntry;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorUpdateTemplateCreateInfoKHR.html) · Alias"]
pub type DescriptorUpdateTemplateCreateInfoKHR = crate::vk1_1::DescriptorUpdateTemplateCreateInfo;
