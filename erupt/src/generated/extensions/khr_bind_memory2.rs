#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_BIND_MEMORY_2_SPEC_VERSION")]
pub const KHR_BIND_MEMORY_2_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_BIND_MEMORY_2_EXTENSION_NAME")]
pub const KHR_BIND_MEMORY_2_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_bind_memory2");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_BIND_BUFFER_MEMORY2_KHR: *const std::os::raw::c_char =
    crate::cstr!("vkBindBufferMemory2KHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_BIND_IMAGE_MEMORY2_KHR: *const std::os::raw::c_char =
    crate::cstr!("vkBindImageMemory2KHR");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindBufferMemoryInfoKHR.html) · Alias"]
#[doc(alias = "VkBindBufferMemoryInfoKHR")]
#[allow(non_camel_case_types)]
pub type BindBufferMemoryInfoKHR = crate::vk1_1::BindBufferMemoryInfo;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindBufferMemoryInfoKHR.html) · Alias"]
#[doc(alias = "VkBindBufferMemoryInfoKHR")]
#[allow(non_camel_case_types)]
pub type BindBufferMemoryInfoKHRBuilder<'a> = crate::vk1_1::BindBufferMemoryInfoBuilder<'a>;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindImageMemoryInfoKHR.html) · Alias"]
#[doc(alias = "VkBindImageMemoryInfoKHR")]
#[allow(non_camel_case_types)]
pub type BindImageMemoryInfoKHR = crate::vk1_1::BindImageMemoryInfo;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindImageMemoryInfoKHR.html) · Alias"]
#[doc(alias = "VkBindImageMemoryInfoKHR")]
#[allow(non_camel_case_types)]
pub type BindImageMemoryInfoKHRBuilder<'a> = crate::vk1_1::BindImageMemoryInfoBuilder<'a>;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBindBufferMemory2KHR.html) · Alias"]
#[allow(non_camel_case_types)]
pub type PFN_vkBindBufferMemory2KHR = crate::vk1_1::PFN_vkBindBufferMemory2;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBindImageMemory2KHR.html) · Alias"]
#[allow(non_camel_case_types)]
pub type PFN_vkBindImageMemory2KHR = crate::vk1_1::PFN_vkBindImageMemory2;
#[doc = "Provided by [`crate::extensions::khr_bind_memory2`]"]
impl crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBindBufferMemory2KHR.html) · Function"]
    #[doc(alias = "vkBindBufferMemory2KHR")]
    pub unsafe fn bind_buffer_memory2_khr(
        &self,
        bind_infos: &[crate::vk1_1::BindBufferMemoryInfoBuilder],
    ) -> crate::utils::VulkanResult<()> {
        let _function = self
            .bind_buffer_memory2_khr
            .expect("`bind_buffer_memory2_khr` is not loaded");
        let bind_info_count = bind_infos.len();
        let _return = _function(self.handle, bind_info_count as _, bind_infos.as_ptr() as _);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBindImageMemory2KHR.html) · Function"]
    #[doc(alias = "vkBindImageMemory2KHR")]
    pub unsafe fn bind_image_memory2_khr(
        &self,
        bind_infos: &[crate::vk1_1::BindImageMemoryInfoBuilder],
    ) -> crate::utils::VulkanResult<()> {
        let _function = self
            .bind_image_memory2_khr
            .expect("`bind_image_memory2_khr` is not loaded");
        let bind_info_count = bind_infos.len();
        let _return = _function(self.handle, bind_info_count as _, bind_infos.as_ptr() as _);
        crate::utils::VulkanResult::new(_return, ())
    }
}
