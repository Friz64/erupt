#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE_SPEC_VERSION")]
pub const KHR_DESCRIPTOR_UPDATE_TEMPLATE_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE_EXTENSION_NAME")]
pub const KHR_DESCRIPTOR_UPDATE_TEMPLATE_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_descriptor_update_template");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_DESCRIPTOR_UPDATE_TEMPLATE_KHR: *const std::os::raw::c_char = crate::cstr!("vkCreateDescriptorUpdateTemplateKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_DESTROY_DESCRIPTOR_UPDATE_TEMPLATE_KHR: *const std::os::raw::c_char = crate::cstr!("vkDestroyDescriptorUpdateTemplateKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_UPDATE_DESCRIPTOR_SET_WITH_TEMPLATE_KHR: *const std::os::raw::c_char = crate::cstr!("vkUpdateDescriptorSetWithTemplateKHR");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorUpdateTemplateCreateFlagsKHR.html) · Alias"]
#[doc(alias = "VkDescriptorUpdateTemplateCreateFlagsKHR")]
#[allow(non_camel_case_types)]
pub type DescriptorUpdateTemplateCreateFlagsKHR = crate::vk1_1::DescriptorUpdateTemplateCreateFlags;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorUpdateTemplateKHR.html) · Alias"]
#[doc(alias = "VkDescriptorUpdateTemplateKHR")]
#[allow(non_camel_case_types)]
pub type DescriptorUpdateTemplateKHR = crate::vk1_1::DescriptorUpdateTemplate;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorUpdateTemplateTypeKHR.html) · Alias"]
#[doc(alias = "VkDescriptorUpdateTemplateTypeKHR")]
#[allow(non_camel_case_types)]
pub type DescriptorUpdateTemplateTypeKHR = crate::vk1_1::DescriptorUpdateTemplateType;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorUpdateTemplateEntryKHR.html) · Alias"]
#[doc(alias = "VkDescriptorUpdateTemplateEntryKHR")]
#[allow(non_camel_case_types)]
pub type DescriptorUpdateTemplateEntryKHR = crate::vk1_1::DescriptorUpdateTemplateEntry;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorUpdateTemplateEntryKHR.html) · Alias"]
#[doc(alias = "VkDescriptorUpdateTemplateEntryKHR")]
#[allow(non_camel_case_types)]
pub type DescriptorUpdateTemplateEntryKHRBuilder<'a> = crate::vk1_1::DescriptorUpdateTemplateEntryBuilder<'a>;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorUpdateTemplateCreateInfoKHR.html) · Alias"]
#[doc(alias = "VkDescriptorUpdateTemplateCreateInfoKHR")]
#[allow(non_camel_case_types)]
pub type DescriptorUpdateTemplateCreateInfoKHR = crate::vk1_1::DescriptorUpdateTemplateCreateInfo;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorUpdateTemplateCreateInfoKHR.html) · Alias"]
#[doc(alias = "VkDescriptorUpdateTemplateCreateInfoKHR")]
#[allow(non_camel_case_types)]
pub type DescriptorUpdateTemplateCreateInfoKHRBuilder<'a> = crate::vk1_1::DescriptorUpdateTemplateCreateInfoBuilder<'a>;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDescriptorUpdateTemplateKHR.html) · Alias"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateDescriptorUpdateTemplateKHR = crate::vk1_1::PFN_vkCreateDescriptorUpdateTemplate;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyDescriptorUpdateTemplateKHR.html) · Alias"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyDescriptorUpdateTemplateKHR = crate::vk1_1::PFN_vkDestroyDescriptorUpdateTemplate;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkUpdateDescriptorSetWithTemplateKHR.html) · Alias"]
#[allow(non_camel_case_types)]
pub type PFN_vkUpdateDescriptorSetWithTemplateKHR = crate::vk1_1::PFN_vkUpdateDescriptorSetWithTemplate;
#[doc = "Provided by [`crate::extensions::khr_descriptor_update_template`]"]
impl crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDescriptorUpdateTemplateKHR.html) · Function"]
    #[doc(alias = "vkCreateDescriptorUpdateTemplateKHR")]
    pub unsafe fn create_descriptor_update_template_khr(
        &self,
        create_info: &crate::vk1_1::DescriptorUpdateTemplateCreateInfo,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
    ) -> crate::utils::VulkanResult<crate::vk1_1::DescriptorUpdateTemplate> {
        let _function = self.create_descriptor_update_template_khr.expect("`create_descriptor_update_template_khr` is not loaded");
        let mut descriptor_update_template = Default::default();
        let _return = _function(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut descriptor_update_template,
        );
        crate::utils::VulkanResult::new(_return, descriptor_update_template)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyDescriptorUpdateTemplateKHR.html) · Function"]
    #[doc(alias = "vkDestroyDescriptorUpdateTemplateKHR")]
    pub unsafe fn destroy_descriptor_update_template_khr(
        &self,
        descriptor_update_template: Option<crate::vk1_1::DescriptorUpdateTemplate>,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
    ) -> () {
        let _function = self.destroy_descriptor_update_template_khr.expect("`destroy_descriptor_update_template_khr` is not loaded");
        let _return = _function(
            self.handle,
            match descriptor_update_template {
                Some(v) => v,
                None => Default::default(),
            },
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkUpdateDescriptorSetWithTemplateKHR.html) · Function"]
    #[doc(alias = "vkUpdateDescriptorSetWithTemplateKHR")]
    pub unsafe fn update_descriptor_set_with_template_khr(
        &self,
        descriptor_set: crate::vk1_0::DescriptorSet,
        descriptor_update_template: crate::vk1_1::DescriptorUpdateTemplate,
        data: *const std::ffi::c_void,
    ) -> () {
        let _function = self.update_descriptor_set_with_template_khr.expect("`update_descriptor_set_with_template_khr` is not loaded");
        let _return = _function(self.handle, descriptor_set as _, descriptor_update_template as _, data);
        ()
    }
}
