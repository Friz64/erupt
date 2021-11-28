#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_SHADER_FLOAT_CONTROLS_SPEC_VERSION")]
pub const KHR_SHADER_FLOAT_CONTROLS_SPEC_VERSION: u32 = 4;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_SHADER_FLOAT_CONTROLS_EXTENSION_NAME")]
pub const KHR_SHADER_FLOAT_CONTROLS_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_shader_float_controls");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkShaderFloatControlsIndependenceKHR.html) · Alias"]
#[doc(alias = "VkShaderFloatControlsIndependenceKHR")]
#[allow(non_camel_case_types)]
pub type ShaderFloatControlsIndependenceKHR = crate::vk1_2::ShaderFloatControlsIndependence;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceFloatControlsPropertiesKHR.html) · Alias"]
#[doc(alias = "VkPhysicalDeviceFloatControlsPropertiesKHR")]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceFloatControlsPropertiesKHR = crate::vk1_2::PhysicalDeviceFloatControlsProperties;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceFloatControlsPropertiesKHR.html) · Alias"]
#[doc(alias = "VkPhysicalDeviceFloatControlsPropertiesKHR")]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceFloatControlsPropertiesKHRBuilder<'a> = crate::vk1_2::PhysicalDeviceFloatControlsPropertiesBuilder<'a>;
#[doc = "Provided by [`crate::extensions::khr_shader_float_controls`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES_KHR: Self = Self::PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES;
}
#[doc = "Provided by [`crate::extensions::khr_shader_float_controls`]"]
impl crate::vk1_2::ShaderFloatControlsIndependence {
    pub const _32_BIT_ONLY_KHR: Self = Self::_32_ONLY;
    pub const ALL_KHR: Self = Self::ALL;
    pub const NONE_KHR: Self = Self::NONE;
}
