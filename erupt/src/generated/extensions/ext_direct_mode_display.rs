# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_EXT_direct_mode_display.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_DIRECT_MODE_DISPLAY_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_DIRECT_MODE_DISPLAY_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_direct_mode_display");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkReleaseDisplayEXT.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkReleaseDisplayEXT = unsafe extern "system" fn(
    physical_device: crate::vk1_0::PhysicalDevice,
    display: crate::extensions::khr_display::DisplayKHR,
) -> crate::vk1_0::Result;
#[doc = "Provides Instance Commands for [`ExtDirectModeDisplayInstanceLoaderExt`](trait.ExtDirectModeDisplayInstanceLoaderExt.html)"]
pub struct ExtDirectModeDisplayInstanceCommands {
    pub release_display_ext: Option<PFN_vkReleaseDisplayEXT>,
}
impl ExtDirectModeDisplayInstanceCommands {
    #[inline]
    pub fn load(loader: &crate::InstanceLoader) -> Option<ExtDirectModeDisplayInstanceCommands> {
        unsafe {
            let mut success = false;
            let table = ExtDirectModeDisplayInstanceCommands {
                release_display_ext: std::mem::transmute({
                    let symbol = loader.symbol("vkReleaseDisplayEXT");
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
#[inline]
fn instance_commands(loader: &crate::InstanceLoader) -> &ExtDirectModeDisplayInstanceCommands {
    loader
        .ext_direct_mode_display
        .as_ref()
        .expect("`ext_direct_mode_display` not loaded")
}
#[doc = "Provides high level command wrappers for [`ExtDirectModeDisplayInstanceCommands`](struct.ExtDirectModeDisplayInstanceCommands.html)"]
pub trait ExtDirectModeDisplayInstanceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkReleaseDisplayEXT.html) · Instance Command"]
    unsafe fn release_display_ext(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        display: crate::extensions::khr_display::DisplayKHR,
    ) -> crate::utils::VulkanResult<()>;
}
impl ExtDirectModeDisplayInstanceLoaderExt for crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkReleaseDisplayEXT.html) · Instance Command"]
    unsafe fn release_display_ext(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        display: crate::extensions::khr_display::DisplayKHR,
    ) -> crate::utils::VulkanResult<()> {
        let function = instance_commands(self)
            .release_display_ext
            .as_ref()
            .expect("`release_display_ext` not available");
        let _val = function(physical_device, display);
        crate::utils::VulkanResult::new(_val, ())
    }
}
