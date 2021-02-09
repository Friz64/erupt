#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_GET_MEMORY_REQUIREMENTS_2_SPEC_VERSION")]
pub const KHR_GET_MEMORY_REQUIREMENTS_2_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_GET_MEMORY_REQUIREMENTS_2_EXTENSION_NAME")]
pub const KHR_GET_MEMORY_REQUIREMENTS_2_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_get_memory_requirements2");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_BUFFER_MEMORY_REQUIREMENTS2_KHR: *const std::os::raw::c_char =
    crate::cstr!("vkGetBufferMemoryRequirements2KHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_IMAGE_MEMORY_REQUIREMENTS2_KHR: *const std::os::raw::c_char =
    crate::cstr!("vkGetImageMemoryRequirements2KHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_IMAGE_SPARSE_MEMORY_REQUIREMENTS2_KHR: *const std::os::raw::c_char =
    crate::cstr!("vkGetImageSparseMemoryRequirements2KHR");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferMemoryRequirementsInfo2KHR.html) · Alias"]
#[doc(alias = "VkBufferMemoryRequirementsInfo2KHR")]
#[allow(non_camel_case_types)]
pub type BufferMemoryRequirementsInfo2KHR = crate::vk1_1::BufferMemoryRequirementsInfo2;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferMemoryRequirementsInfo2KHR.html) · Alias"]
#[doc(alias = "VkBufferMemoryRequirementsInfo2KHR")]
#[allow(non_camel_case_types)]
pub type BufferMemoryRequirementsInfo2KHRBuilder<'a> =
    crate::vk1_1::BufferMemoryRequirementsInfo2Builder<'a>;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageMemoryRequirementsInfo2KHR.html) · Alias"]
#[doc(alias = "VkImageMemoryRequirementsInfo2KHR")]
#[allow(non_camel_case_types)]
pub type ImageMemoryRequirementsInfo2KHR = crate::vk1_1::ImageMemoryRequirementsInfo2;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageMemoryRequirementsInfo2KHR.html) · Alias"]
#[doc(alias = "VkImageMemoryRequirementsInfo2KHR")]
#[allow(non_camel_case_types)]
pub type ImageMemoryRequirementsInfo2KHRBuilder<'a> =
    crate::vk1_1::ImageMemoryRequirementsInfo2Builder<'a>;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageSparseMemoryRequirementsInfo2KHR.html) · Alias"]
#[doc(alias = "VkImageSparseMemoryRequirementsInfo2KHR")]
#[allow(non_camel_case_types)]
pub type ImageSparseMemoryRequirementsInfo2KHR = crate::vk1_1::ImageSparseMemoryRequirementsInfo2;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageSparseMemoryRequirementsInfo2KHR.html) · Alias"]
#[doc(alias = "VkImageSparseMemoryRequirementsInfo2KHR")]
#[allow(non_camel_case_types)]
pub type ImageSparseMemoryRequirementsInfo2KHRBuilder<'a> =
    crate::vk1_1::ImageSparseMemoryRequirementsInfo2Builder<'a>;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryRequirements2KHR.html) · Alias"]
#[doc(alias = "VkMemoryRequirements2KHR")]
#[allow(non_camel_case_types)]
pub type MemoryRequirements2KHR = crate::vk1_1::MemoryRequirements2;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryRequirements2KHR.html) · Alias"]
#[doc(alias = "VkMemoryRequirements2KHR")]
#[allow(non_camel_case_types)]
pub type MemoryRequirements2KHRBuilder<'a> = crate::vk1_1::MemoryRequirements2Builder<'a>;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSparseImageMemoryRequirements2KHR.html) · Alias"]
#[doc(alias = "VkSparseImageMemoryRequirements2KHR")]
#[allow(non_camel_case_types)]
pub type SparseImageMemoryRequirements2KHR = crate::vk1_1::SparseImageMemoryRequirements2;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSparseImageMemoryRequirements2KHR.html) · Alias"]
#[doc(alias = "VkSparseImageMemoryRequirements2KHR")]
#[allow(non_camel_case_types)]
pub type SparseImageMemoryRequirements2KHRBuilder<'a> =
    crate::vk1_1::SparseImageMemoryRequirements2Builder<'a>;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetBufferMemoryRequirements2KHR.html) · Alias"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetBufferMemoryRequirements2KHR = crate::vk1_1::PFN_vkGetBufferMemoryRequirements2;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageMemoryRequirements2KHR.html) · Alias"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageMemoryRequirements2KHR = crate::vk1_1::PFN_vkGetImageMemoryRequirements2;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageSparseMemoryRequirements2KHR.html) · Alias"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageSparseMemoryRequirements2KHR =
    crate::vk1_1::PFN_vkGetImageSparseMemoryRequirements2;
#[doc = "Provided by [`crate::extensions::khr_get_memory_requirements2`]"]
impl crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetBufferMemoryRequirements2KHR.html) · Function"]
    #[doc(alias = "vkGetBufferMemoryRequirements2KHR")]
    pub unsafe fn get_buffer_memory_requirements2_khr(
        &self,
        info: &crate::vk1_1::BufferMemoryRequirementsInfo2,
        memory_requirements: Option<crate::vk1_1::MemoryRequirements2>,
    ) -> crate::vk1_1::MemoryRequirements2 {
        let _function = self
            .get_buffer_memory_requirements2_khr
            .expect("`get_buffer_memory_requirements2_khr` is not loaded");
        let mut memory_requirements = match memory_requirements {
            Some(v) => v,
            None => Default::default(),
        };
        let _return = _function(self.handle, info as _, &mut memory_requirements);
        memory_requirements
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageMemoryRequirements2KHR.html) · Function"]
    #[doc(alias = "vkGetImageMemoryRequirements2KHR")]
    pub unsafe fn get_image_memory_requirements2_khr(
        &self,
        info: &crate::vk1_1::ImageMemoryRequirementsInfo2,
        memory_requirements: Option<crate::vk1_1::MemoryRequirements2>,
    ) -> crate::vk1_1::MemoryRequirements2 {
        let _function = self
            .get_image_memory_requirements2_khr
            .expect("`get_image_memory_requirements2_khr` is not loaded");
        let mut memory_requirements = match memory_requirements {
            Some(v) => v,
            None => Default::default(),
        };
        let _return = _function(self.handle, info as _, &mut memory_requirements);
        memory_requirements
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageSparseMemoryRequirements2KHR.html) · Function"]
    #[doc(alias = "vkGetImageSparseMemoryRequirements2KHR")]
    pub unsafe fn get_image_sparse_memory_requirements2_khr(
        &self,
        info: &crate::vk1_1::ImageSparseMemoryRequirementsInfo2,
        sparse_memory_requirement_count: Option<u32>,
    ) -> Vec<crate::vk1_1::SparseImageMemoryRequirements2> {
        let _function = self
            .get_image_sparse_memory_requirements2_khr
            .expect("`get_image_sparse_memory_requirements2_khr` is not loaded");
        let mut sparse_memory_requirement_count = match sparse_memory_requirement_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                _function(self.handle, info as _, &mut v, std::ptr::null_mut());
                v
            }
        };
        let mut sparse_memory_requirements =
            vec![Default::default(); sparse_memory_requirement_count as _];
        let _return = _function(
            self.handle,
            info as _,
            &mut sparse_memory_requirement_count,
            sparse_memory_requirements.as_mut_ptr(),
        );
        sparse_memory_requirements
    }
}
