// DO NOT EDIT: @generated by erupt's generator
///<s>Vulkan Manual Page</s> · Constant
#[doc(alias = "VK_KHR_BIND_MEMORY_2_SPEC_VERSION")]
pub const KHR_BIND_MEMORY_2_SPEC_VERSION: u32 = 1;
///<s>Vulkan Manual Page</s> · Constant
#[doc(alias = "VK_KHR_BIND_MEMORY_2_EXTENSION_NAME")]
pub const KHR_BIND_MEMORY_2_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!(
    "VK_KHR_bind_memory2"
);
///<s>Vulkan Manual Page</s> · Constant
pub const FN_BIND_BUFFER_MEMORY2_KHR: *const std::os::raw::c_char = crate::cstr!(
    "vkBindBufferMemory2KHR"
);
///<s>Vulkan Manual Page</s> · Constant
pub const FN_BIND_IMAGE_MEMORY2_KHR: *const std::os::raw::c_char = crate::cstr!(
    "vkBindImageMemory2KHR"
);
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBindBufferMemoryInfoKHR.html) · Alias
#[doc(alias = "VkBindBufferMemoryInfoKHR")]
#[allow(non_camel_case_types)]
pub type BindBufferMemoryInfoKHR = crate::vk1_1::BindBufferMemoryInfo;
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBindBufferMemoryInfoKHR.html) · Alias
#[doc(alias = "VkBindBufferMemoryInfoKHR")]
#[allow(non_camel_case_types)]
pub type BindBufferMemoryInfoKHRBuilder<'a> = crate::vk1_1::BindBufferMemoryInfoBuilder<
    'a,
>;
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBindImageMemoryInfoKHR.html) · Alias
#[doc(alias = "VkBindImageMemoryInfoKHR")]
#[allow(non_camel_case_types)]
pub type BindImageMemoryInfoKHR = crate::vk1_1::BindImageMemoryInfo;
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBindImageMemoryInfoKHR.html) · Alias
#[doc(alias = "VkBindImageMemoryInfoKHR")]
#[allow(non_camel_case_types)]
pub type BindImageMemoryInfoKHRBuilder<'a> = crate::vk1_1::BindImageMemoryInfoBuilder<
    'a,
>;
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindBufferMemory2KHR.html) · Alias
#[allow(non_camel_case_types)]
pub type PFN_vkBindBufferMemory2KHR = crate::vk1_1::PFN_vkBindBufferMemory2;
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindImageMemory2KHR.html) · Alias
#[allow(non_camel_case_types)]
pub type PFN_vkBindImageMemory2KHR = crate::vk1_1::PFN_vkBindImageMemory2;
///Provided by [`crate::extensions::khr_bind_memory2`]
impl crate::vk1_0::ImageCreateFlagBits {
    pub const ALIAS_KHR: Self = Self::ALIAS;
}
///Provided by [`crate::extensions::khr_bind_memory2`]
impl crate::vk1_0::StructureType {
    pub const BIND_BUFFER_MEMORY_INFO_KHR: Self = Self::BIND_BUFFER_MEMORY_INFO;
    pub const BIND_IMAGE_MEMORY_INFO_KHR: Self = Self::BIND_IMAGE_MEMORY_INFO;
}
///Provided by [`crate::extensions::khr_bind_memory2`]
impl crate::DeviceLoader {
    #[inline]
    #[track_caller]
    ///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindBufferMemory2KHR.html) · Function
    #[doc(alias = "vkBindBufferMemory2KHR")]
    pub unsafe fn bind_buffer_memory2_khr(
        &self,
        bind_infos: &[crate::vk1_1::BindBufferMemoryInfoBuilder],
    ) -> crate::utils::VulkanResult<()> {
        let _function = self.bind_buffer_memory2_khr.expect(crate::NOT_LOADED_MESSAGE);
        let bind_info_count = bind_infos.len();
        let _return = _function(
            self.handle,
            bind_info_count as _,
            bind_infos.as_ptr() as _,
        );
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[track_caller]
    ///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindImageMemory2KHR.html) · Function
    #[doc(alias = "vkBindImageMemory2KHR")]
    pub unsafe fn bind_image_memory2_khr(
        &self,
        bind_infos: &[crate::vk1_1::BindImageMemoryInfoBuilder],
    ) -> crate::utils::VulkanResult<()> {
        let _function = self.bind_image_memory2_khr.expect(crate::NOT_LOADED_MESSAGE);
        let bind_info_count = bind_infos.len();
        let _return = _function(
            self.handle,
            bind_info_count as _,
            bind_infos.as_ptr() as _,
        );
        crate::utils::VulkanResult::new(_return, ())
    }
}
