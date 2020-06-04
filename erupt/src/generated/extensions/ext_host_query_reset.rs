# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_EXT_host_query_reset.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_HOST_QUERY_RESET_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_HOST_QUERY_RESET_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_host_query_reset");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkResetQueryPoolEXT.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkResetQueryPoolEXT = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    query_pool: crate::vk1_0::QueryPool,
    first_query: u32,
    query_count: u32,
) -> std::ffi::c_void;
#[doc = "Provides Device Commands for [`ExtHostQueryResetDeviceLoaderExt`](trait.ExtHostQueryResetDeviceLoaderExt.html)"]
pub struct ExtHostQueryResetDeviceCommands {
    pub reset_query_pool_ext: Option<PFN_vkResetQueryPoolEXT>,
}
impl ExtHostQueryResetDeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<ExtHostQueryResetDeviceCommands> {
        unsafe {
            let mut success = false;
            let table = ExtHostQueryResetDeviceCommands {
                reset_query_pool_ext: std::mem::transmute({
                    let symbol = loader.symbol("vkResetQueryPoolEXT");
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
fn device_commands(loader: &crate::DeviceLoader) -> &ExtHostQueryResetDeviceCommands {
    loader
        .ext_host_query_reset
        .as_ref()
        .expect("`ext_host_query_reset` not loaded")
}
#[doc = "Provides high level command wrappers for [`ExtHostQueryResetDeviceCommands`](struct.ExtHostQueryResetDeviceCommands.html)"]
pub trait ExtHostQueryResetDeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkResetQueryPoolEXT.html) · Device Command"]
    unsafe fn reset_query_pool_ext(
        &self,
        query_pool: crate::vk1_0::QueryPool,
        first_query: u32,
        query_count: u32,
    ) -> ();
}
impl ExtHostQueryResetDeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkResetQueryPoolEXT.html) · Device Command"]
    unsafe fn reset_query_pool_ext(
        &self,
        query_pool: crate::vk1_0::QueryPool,
        first_query: u32,
        query_count: u32,
    ) -> () {
        let function = device_commands(self)
            .reset_query_pool_ext
            .as_ref()
            .expect("`reset_query_pool_ext` not available");
        let _val = function(self.handle, query_pool, first_query, query_count);
        ()
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceHostQueryResetFeaturesEXT.html) · Alias"]
pub type PhysicalDeviceHostQueryResetFeaturesEXT =
    crate::vk1_2::PhysicalDeviceHostQueryResetFeatures;
