#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_HOST_QUERY_RESET_SPEC_VERSION")]
pub const EXT_HOST_QUERY_RESET_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_HOST_QUERY_RESET_EXTENSION_NAME")]
pub const EXT_HOST_QUERY_RESET_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_host_query_reset");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_RESET_QUERY_POOL_EXT: *const std::os::raw::c_char = crate::cstr!("vkResetQueryPoolEXT");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceHostQueryResetFeaturesEXT.html) · Alias"]
#[doc(alias = "VkPhysicalDeviceHostQueryResetFeaturesEXT")]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceHostQueryResetFeaturesEXT = crate::vk1_2::PhysicalDeviceHostQueryResetFeatures;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceHostQueryResetFeaturesEXT.html) · Alias"]
#[doc(alias = "VkPhysicalDeviceHostQueryResetFeaturesEXT")]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceHostQueryResetFeaturesEXTBuilder<'a> = crate::vk1_2::PhysicalDeviceHostQueryResetFeaturesBuilder<'a>;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkResetQueryPoolEXT.html) · Alias"]
#[allow(non_camel_case_types)]
pub type PFN_vkResetQueryPoolEXT = crate::vk1_2::PFN_vkResetQueryPool;
#[doc = "Provided by [`crate::extensions::ext_host_query_reset`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES_EXT: Self = Self::PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES;
}
#[doc = "Provided by [`crate::extensions::ext_host_query_reset`]"]
impl crate::DeviceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkResetQueryPoolEXT.html) · Function"]
    #[doc(alias = "vkResetQueryPoolEXT")]
    pub unsafe fn reset_query_pool_ext(&self, query_pool: crate::vk1_0::QueryPool, first_query: u32, query_count: u32) -> () {
        let _function = self.reset_query_pool_ext.expect("tried to call a function that isn't loaded");
        let _return = _function(self.handle, query_pool as _, first_query as _, query_count as _);
        ()
    }
}
