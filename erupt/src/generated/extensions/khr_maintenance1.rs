#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_MAINTENANCE1_SPEC_VERSION")]
pub const KHR_MAINTENANCE1_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_MAINTENANCE1_EXTENSION_NAME")]
pub const KHR_MAINTENANCE1_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_maintenance1");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_TRIM_COMMAND_POOL_KHR: *const std::os::raw::c_char = crate::cstr!("vkTrimCommandPoolKHR");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandPoolTrimFlagsKHR.html) · Alias"]
#[doc(alias = "VkCommandPoolTrimFlagsKHR")]
#[allow(non_camel_case_types)]
pub type CommandPoolTrimFlagsKHR = crate::vk1_1::CommandPoolTrimFlags;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkTrimCommandPoolKHR.html) · Alias"]
#[allow(non_camel_case_types)]
pub type PFN_vkTrimCommandPoolKHR = crate::vk1_1::PFN_vkTrimCommandPool;
#[doc = "Provided by [`crate::extensions::khr_maintenance1`]"]
impl crate::vk1_0::FormatFeatureFlagBits {
    pub const TRANSFER_SRC_KHR: Self = Self::TRANSFER_SRC;
    pub const TRANSFER_DST_KHR: Self = Self::TRANSFER_DST;
}
#[doc = "Provided by [`crate::extensions::khr_maintenance1`]"]
impl crate::vk1_0::ImageCreateFlagBits {
    pub const _2D_ARRAY_COMPATIBLE_KHR: Self = Self::_2D_ARRAY_COMPATIBLE;
}
#[doc = "Provided by [`crate::extensions::khr_maintenance1`]"]
impl crate::vk1_0::Result {
    pub const ERROR_OUT_OF_POOL_MEMORY_KHR: Self = Self::ERROR_OUT_OF_POOL_MEMORY;
}
#[doc = "Provided by [`crate::extensions::khr_maintenance1`]"]
impl crate::DeviceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkTrimCommandPoolKHR.html) · Function"]
    #[doc(alias = "vkTrimCommandPoolKHR")]
    pub unsafe fn trim_command_pool_khr(&self, command_pool: crate::vk1_0::CommandPool, flags: Option<crate::vk1_1::CommandPoolTrimFlags>) -> () {
        let _function = self.trim_command_pool_khr.expect("tried to call a function that isn't loaded");
        let _return = _function(
            self.handle,
            command_pool as _,
            match flags {
                Some(v) => v,
                None => Default::default(),
            },
        );
        ()
    }
}
