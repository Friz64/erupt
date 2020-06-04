# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_KHR_bind_memory2.html)\n\n## Extends\n- [`ImageCreateFlagBits`](../../vk1_0/struct.ImageCreateFlagBits.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_BIND_MEMORY_2_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_BIND_MEMORY_2_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_bind_memory2");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBindBufferMemory2KHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkBindBufferMemory2KHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    bind_info_count: u32,
    p_bind_infos: *const crate::vk1_1::BindBufferMemoryInfo,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBindImageMemory2KHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkBindImageMemory2KHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    bind_info_count: u32,
    p_bind_infos: *const crate::vk1_1::BindImageMemoryInfo,
) -> crate::vk1_0::Result;
#[doc = "Provides Device Commands for [`KhrBindMemory2DeviceLoaderExt`](trait.KhrBindMemory2DeviceLoaderExt.html)"]
pub struct KhrBindMemory2DeviceCommands {
    pub bind_buffer_memory2_khr: Option<PFN_vkBindBufferMemory2KHR>,
    pub bind_image_memory2_khr: Option<PFN_vkBindImageMemory2KHR>,
}
impl KhrBindMemory2DeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<KhrBindMemory2DeviceCommands> {
        unsafe {
            let mut success = false;
            let table = KhrBindMemory2DeviceCommands {
                bind_buffer_memory2_khr: std::mem::transmute({
                    let symbol = loader.symbol("vkBindBufferMemory2KHR");
                    success |= symbol.is_some();
                    symbol
                }),
                bind_image_memory2_khr: std::mem::transmute({
                    let symbol = loader.symbol("vkBindImageMemory2KHR");
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
fn device_commands(loader: &crate::DeviceLoader) -> &KhrBindMemory2DeviceCommands {
    loader
        .khr_bind_memory2
        .as_ref()
        .expect("`khr_bind_memory2` not loaded")
}
#[doc = "Provides high level command wrappers for [`KhrBindMemory2DeviceCommands`](struct.KhrBindMemory2DeviceCommands.html)"]
pub trait KhrBindMemory2DeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBindBufferMemory2KHR.html) · Device Command"]
    unsafe fn bind_buffer_memory2_khr(
        &self,
        bind_infos: &[crate::vk1_1::BindBufferMemoryInfoBuilder],
    ) -> crate::utils::VulkanResult<()>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBindImageMemory2KHR.html) · Device Command"]
    unsafe fn bind_image_memory2_khr(
        &self,
        bind_infos: &[crate::vk1_1::BindImageMemoryInfoBuilder],
    ) -> crate::utils::VulkanResult<()>;
}
impl KhrBindMemory2DeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBindBufferMemory2KHR.html) · Device Command"]
    unsafe fn bind_buffer_memory2_khr(
        &self,
        bind_infos: &[crate::vk1_1::BindBufferMemoryInfoBuilder],
    ) -> crate::utils::VulkanResult<()> {
        let function = device_commands(self)
            .bind_buffer_memory2_khr
            .as_ref()
            .expect("`bind_buffer_memory2_khr` not available");
        let bind_info_count = bind_infos.len() as _;
        let _val = function(self.handle, bind_info_count, bind_infos.as_ptr() as _);
        crate::utils::VulkanResult::new(_val, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBindImageMemory2KHR.html) · Device Command"]
    unsafe fn bind_image_memory2_khr(
        &self,
        bind_infos: &[crate::vk1_1::BindImageMemoryInfoBuilder],
    ) -> crate::utils::VulkanResult<()> {
        let function = device_commands(self)
            .bind_image_memory2_khr
            .as_ref()
            .expect("`bind_image_memory2_khr` not available");
        let bind_info_count = bind_infos.len() as _;
        let _val = function(self.handle, bind_info_count, bind_infos.as_ptr() as _);
        crate::utils::VulkanResult::new(_val, ())
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindBufferMemoryInfoKHR.html) · Alias"]
pub type BindBufferMemoryInfoKHR = crate::vk1_1::BindBufferMemoryInfo;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindImageMemoryInfoKHR.html) · Alias"]
pub type BindImageMemoryInfoKHR = crate::vk1_1::BindImageMemoryInfo;
