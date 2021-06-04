#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_MULTIVIEW_SPEC_VERSION")]
pub const KHR_MULTIVIEW_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_MULTIVIEW_EXTENSION_NAME")]
pub const KHR_MULTIVIEW_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_multiview");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMultiviewFeaturesKHR.html) · Alias"]
#[doc(alias = "VkPhysicalDeviceMultiviewFeaturesKHR")]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceMultiviewFeaturesKHR = crate::vk1_1::PhysicalDeviceMultiviewFeatures;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMultiviewFeaturesKHR.html) · Alias"]
#[doc(alias = "VkPhysicalDeviceMultiviewFeaturesKHR")]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceMultiviewFeaturesKHRBuilder<'a> = crate::vk1_1::PhysicalDeviceMultiviewFeaturesBuilder<'a>;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMultiviewPropertiesKHR.html) · Alias"]
#[doc(alias = "VkPhysicalDeviceMultiviewPropertiesKHR")]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceMultiviewPropertiesKHR = crate::vk1_1::PhysicalDeviceMultiviewProperties;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMultiviewPropertiesKHR.html) · Alias"]
#[doc(alias = "VkPhysicalDeviceMultiviewPropertiesKHR")]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceMultiviewPropertiesKHRBuilder<'a> = crate::vk1_1::PhysicalDeviceMultiviewPropertiesBuilder<'a>;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderPassMultiviewCreateInfoKHR.html) · Alias"]
#[doc(alias = "VkRenderPassMultiviewCreateInfoKHR")]
#[allow(non_camel_case_types)]
pub type RenderPassMultiviewCreateInfoKHR = crate::vk1_1::RenderPassMultiviewCreateInfo;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderPassMultiviewCreateInfoKHR.html) · Alias"]
#[doc(alias = "VkRenderPassMultiviewCreateInfoKHR")]
#[allow(non_camel_case_types)]
pub type RenderPassMultiviewCreateInfoKHRBuilder<'a> = crate::vk1_1::RenderPassMultiviewCreateInfoBuilder<'a>;
#[doc = "Provided by [`crate::extensions::khr_multiview`]"]
impl crate::vk1_0::StructureType {
    pub const RENDER_PASS_MULTIVIEW_CREATE_INFO_KHR: Self = Self::RENDER_PASS_MULTIVIEW_CREATE_INFO;
    pub const PHYSICAL_DEVICE_MULTIVIEW_FEATURES_KHR: Self = Self::PHYSICAL_DEVICE_MULTIVIEW_FEATURES;
    pub const PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES_KHR: Self = Self::PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES;
}
#[doc = "Provided by [`crate::extensions::khr_multiview`]"]
impl crate::vk1_0::DependencyFlagBits {
    pub const VIEW_LOCAL_KHR: Self = Self::VIEW_LOCAL;
}
