# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_KHR_maintenance1.html)\n\n## Extends\n- [`FormatFeatureFlagBits`](../../vk1_0/struct.FormatFeatureFlagBits.html)\n- [`ImageCreateFlagBits`](../../vk1_0/struct.ImageCreateFlagBits.html)\n- [`Result`](../../vk1_0/struct.Result.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_MAINTENANCE1_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_MAINTENANCE1_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_maintenance1");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkTrimCommandPoolKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkTrimCommandPoolKHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    command_pool: crate::vk1_0::CommandPool,
    flags: crate::vk1_1::CommandPoolTrimFlags,
) -> std::ffi::c_void;
#[doc = "Provides Device Commands for [`KhrMaintenance1DeviceLoaderExt`](trait.KhrMaintenance1DeviceLoaderExt.html)"]
pub struct KhrMaintenance1DeviceCommands {
    pub trim_command_pool_khr: PFN_vkTrimCommandPoolKHR,
}
impl KhrMaintenance1DeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<KhrMaintenance1DeviceCommands> {
        unsafe {
            Some(KhrMaintenance1DeviceCommands {
                trim_command_pool_khr: std::mem::transmute(loader.symbol("vkTrimCommandPoolKHR")?),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`KhrMaintenance1DeviceCommands`](struct.KhrMaintenance1DeviceCommands.html)"]
pub trait KhrMaintenance1DeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkTrimCommandPoolKHR.html) · Device Command"]
    unsafe fn trim_command_pool_khr(
        &self,
        command_pool: crate::vk1_0::CommandPool,
        flags: crate::vk1_1::CommandPoolTrimFlags,
    ) -> ();
}
impl KhrMaintenance1DeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkTrimCommandPoolKHR.html) · Device Command"]
    unsafe fn trim_command_pool_khr(
        &self,
        command_pool: crate::vk1_0::CommandPool,
        flags: crate::vk1_1::CommandPoolTrimFlags,
    ) -> () {
        let function = self
            .khr_maintenance1
            .as_ref()
            .expect("`khr_maintenance1` not loaded")
            .trim_command_pool_khr;
        let _val = function(self.handle, command_pool, flags);
        ()
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandPoolTrimFlagsKHR.html) · Alias"]
pub type CommandPoolTrimFlagsKHR = crate::vk1_1::CommandPoolTrimFlags;
