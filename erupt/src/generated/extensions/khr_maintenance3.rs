# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_KHR_maintenance3.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_MAINTENANCE3_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_MAINTENANCE3_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_maintenance3");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDescriptorSetLayoutSupportKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDescriptorSetLayoutSupportKHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_create_info: *const crate::vk1_0::DescriptorSetLayoutCreateInfo,
    p_support: *mut crate::vk1_1::DescriptorSetLayoutSupport,
) -> std::ffi::c_void;
#[doc = "Provides Device Commands for [`KhrMaintenance3DeviceLoaderExt`](trait.KhrMaintenance3DeviceLoaderExt.html)"]
pub struct KhrMaintenance3DeviceCommands {
    pub get_descriptor_set_layout_support_khr: PFN_vkGetDescriptorSetLayoutSupportKHR,
}
impl KhrMaintenance3DeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<KhrMaintenance3DeviceCommands> {
        unsafe {
            Some(KhrMaintenance3DeviceCommands {
                get_descriptor_set_layout_support_khr: std::mem::transmute(
                    loader.symbol("vkGetDescriptorSetLayoutSupportKHR")?,
                ),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`KhrMaintenance3DeviceCommands`](struct.KhrMaintenance3DeviceCommands.html)"]
pub trait KhrMaintenance3DeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDescriptorSetLayoutSupportKHR.html) · Device Command"]
    unsafe fn get_descriptor_set_layout_support_khr(
        &self,
        create_info: &crate::vk1_0::DescriptorSetLayoutCreateInfo,
        support: Option<crate::vk1_1::DescriptorSetLayoutSupport>,
    ) -> crate::vk1_1::DescriptorSetLayoutSupport;
}
impl KhrMaintenance3DeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDescriptorSetLayoutSupportKHR.html) · Device Command"]
    unsafe fn get_descriptor_set_layout_support_khr(
        &self,
        create_info: &crate::vk1_0::DescriptorSetLayoutCreateInfo,
        support: Option<crate::vk1_1::DescriptorSetLayoutSupport>,
    ) -> crate::vk1_1::DescriptorSetLayoutSupport {
        let function = self
            .khr_maintenance3
            .as_ref()
            .expect("`khr_maintenance3` not loaded")
            .get_descriptor_set_layout_support_khr;
        let mut support = support.unwrap_or_else(|| Default::default());
        let _val = function(self.handle, create_info, &mut support);
        support
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMaintenance3PropertiesKHR.html) · Alias"]
pub type PhysicalDeviceMaintenance3PropertiesKHR =
    crate::vk1_1::PhysicalDeviceMaintenance3Properties;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorSetLayoutSupportKHR.html) · Alias"]
pub type DescriptorSetLayoutSupportKHR = crate::vk1_1::DescriptorSetLayoutSupport;
