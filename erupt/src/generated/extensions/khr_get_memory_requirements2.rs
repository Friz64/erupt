# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_KHR_get_memory_requirements2.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_GET_MEMORY_REQUIREMENTS_2_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_GET_MEMORY_REQUIREMENTS_2_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_get_memory_requirements2");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageMemoryRequirements2KHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageMemoryRequirements2KHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_info: *const crate::vk1_1::ImageMemoryRequirementsInfo2,
    p_memory_requirements: *mut crate::vk1_1::MemoryRequirements2,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetBufferMemoryRequirements2KHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetBufferMemoryRequirements2KHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_info: *const crate::vk1_1::BufferMemoryRequirementsInfo2,
    p_memory_requirements: *mut crate::vk1_1::MemoryRequirements2,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageSparseMemoryRequirements2KHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageSparseMemoryRequirements2KHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_info: *const crate::vk1_1::ImageSparseMemoryRequirementsInfo2,
    p_sparse_memory_requirement_count: *mut u32,
    p_sparse_memory_requirements: *mut crate::vk1_1::SparseImageMemoryRequirements2,
)
    -> std::ffi::c_void;
#[doc = "Provides Device Commands for [`KhrGetMemoryRequirements2DeviceLoaderExt`](trait.KhrGetMemoryRequirements2DeviceLoaderExt.html)"]
pub struct KhrGetMemoryRequirements2DeviceCommands {
    pub get_image_memory_requirements2_khr: PFN_vkGetImageMemoryRequirements2KHR,
    pub get_buffer_memory_requirements2_khr: PFN_vkGetBufferMemoryRequirements2KHR,
    pub get_image_sparse_memory_requirements2_khr: PFN_vkGetImageSparseMemoryRequirements2KHR,
}
impl KhrGetMemoryRequirements2DeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<KhrGetMemoryRequirements2DeviceCommands> {
        unsafe {
            Some(KhrGetMemoryRequirements2DeviceCommands {
                get_image_memory_requirements2_khr: std::mem::transmute(
                    loader.symbol("vkGetImageMemoryRequirements2KHR")?,
                ),
                get_buffer_memory_requirements2_khr: std::mem::transmute(
                    loader.symbol("vkGetBufferMemoryRequirements2KHR")?,
                ),
                get_image_sparse_memory_requirements2_khr: std::mem::transmute(
                    loader.symbol("vkGetImageSparseMemoryRequirements2KHR")?,
                ),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`KhrGetMemoryRequirements2DeviceCommands`](struct.KhrGetMemoryRequirements2DeviceCommands.html)"]
pub trait KhrGetMemoryRequirements2DeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageMemoryRequirements2KHR.html) · Device Command"]
    unsafe fn get_image_memory_requirements2_khr(
        &self,
        info: &crate::vk1_1::ImageMemoryRequirementsInfo2,
        memory_requirements: Option<crate::vk1_1::MemoryRequirements2>,
    ) -> crate::vk1_1::MemoryRequirements2;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetBufferMemoryRequirements2KHR.html) · Device Command"]
    unsafe fn get_buffer_memory_requirements2_khr(
        &self,
        info: &crate::vk1_1::BufferMemoryRequirementsInfo2,
        memory_requirements: Option<crate::vk1_1::MemoryRequirements2>,
    ) -> crate::vk1_1::MemoryRequirements2;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageSparseMemoryRequirements2KHR.html) · Device Command"]
    unsafe fn get_image_sparse_memory_requirements2_khr(
        &self,
        info: &crate::vk1_1::ImageSparseMemoryRequirementsInfo2,
        sparse_memory_requirement_count: Option<u32>,
    ) -> Vec<crate::vk1_1::SparseImageMemoryRequirements2>;
}
impl KhrGetMemoryRequirements2DeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageMemoryRequirements2KHR.html) · Device Command"]
    unsafe fn get_image_memory_requirements2_khr(
        &self,
        info: &crate::vk1_1::ImageMemoryRequirementsInfo2,
        memory_requirements: Option<crate::vk1_1::MemoryRequirements2>,
    ) -> crate::vk1_1::MemoryRequirements2 {
        let function = self
            .khr_get_memory_requirements2
            .as_ref()
            .expect("`khr_get_memory_requirements2` not loaded")
            .get_image_memory_requirements2_khr;
        let mut memory_requirements = memory_requirements.unwrap_or_else(|| Default::default());
        let _val = function(self.handle, info, &mut memory_requirements);
        memory_requirements
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetBufferMemoryRequirements2KHR.html) · Device Command"]
    unsafe fn get_buffer_memory_requirements2_khr(
        &self,
        info: &crate::vk1_1::BufferMemoryRequirementsInfo2,
        memory_requirements: Option<crate::vk1_1::MemoryRequirements2>,
    ) -> crate::vk1_1::MemoryRequirements2 {
        let function = self
            .khr_get_memory_requirements2
            .as_ref()
            .expect("`khr_get_memory_requirements2` not loaded")
            .get_buffer_memory_requirements2_khr;
        let mut memory_requirements = memory_requirements.unwrap_or_else(|| Default::default());
        let _val = function(self.handle, info, &mut memory_requirements);
        memory_requirements
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageSparseMemoryRequirements2KHR.html) · Device Command"]
    unsafe fn get_image_sparse_memory_requirements2_khr(
        &self,
        info: &crate::vk1_1::ImageSparseMemoryRequirementsInfo2,
        sparse_memory_requirement_count: Option<u32>,
    ) -> Vec<crate::vk1_1::SparseImageMemoryRequirements2> {
        let function = self
            .khr_get_memory_requirements2
            .as_ref()
            .expect("`khr_get_memory_requirements2` not loaded")
            .get_image_sparse_memory_requirements2_khr;
        let mut sparse_memory_requirement_count =
            sparse_memory_requirement_count.unwrap_or_else(|| {
                let mut val = Default::default();
                function(self.handle, info, &mut val, std::ptr::null_mut());
                val
            });
        let mut sparse_memory_requirements =
            vec![Default::default(); sparse_memory_requirement_count as _];
        let _val = function(
            self.handle,
            info,
            &mut sparse_memory_requirement_count,
            sparse_memory_requirements.as_mut_ptr(),
        );
        sparse_memory_requirements
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferMemoryRequirementsInfo2KHR.html) · Alias"]
pub type BufferMemoryRequirementsInfo2KHR = crate::vk1_1::BufferMemoryRequirementsInfo2;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageMemoryRequirementsInfo2KHR.html) · Alias"]
pub type ImageMemoryRequirementsInfo2KHR = crate::vk1_1::ImageMemoryRequirementsInfo2;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageSparseMemoryRequirementsInfo2KHR.html) · Alias"]
pub type ImageSparseMemoryRequirementsInfo2KHR = crate::vk1_1::ImageSparseMemoryRequirementsInfo2;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSparseImageMemoryRequirements2KHR.html) · Alias"]
pub type SparseImageMemoryRequirements2KHR = crate::vk1_1::SparseImageMemoryRequirements2;
