// DO NOT EDIT: @generated by erupt's generator
///<s>Vulkan Manual Page</s> · Constant
#[doc(alias = "VK_KHR_DEPTH_STENCIL_RESOLVE_SPEC_VERSION")]
pub const KHR_DEPTH_STENCIL_RESOLVE_SPEC_VERSION: u32 = 1;
///<s>Vulkan Manual Page</s> · Constant
#[doc(alias = "VK_KHR_DEPTH_STENCIL_RESOLVE_EXTENSION_NAME")]
pub const KHR_DEPTH_STENCIL_RESOLVE_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!(
    "VK_KHR_depth_stencil_resolve"
);
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkResolveModeFlagsKHR.html) · Alias
#[doc(alias = "VkResolveModeFlagsKHR")]
#[allow(non_camel_case_types)]
pub type ResolveModeFlagsKHR = crate::vk1_2::ResolveModeFlags;
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkResolveModeFlagBitsKHR.html) · Alias
#[doc(alias = "VkResolveModeFlagBitsKHR")]
#[allow(non_camel_case_types)]
pub type ResolveModeFlagBitsKHR = crate::vk1_2::ResolveModeFlagBits;
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDepthStencilResolvePropertiesKHR.html) · Alias
#[doc(alias = "VkPhysicalDeviceDepthStencilResolvePropertiesKHR")]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceDepthStencilResolvePropertiesKHR = crate::vk1_2::PhysicalDeviceDepthStencilResolveProperties;
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDepthStencilResolvePropertiesKHR.html) · Alias
#[doc(alias = "VkPhysicalDeviceDepthStencilResolvePropertiesKHR")]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceDepthStencilResolvePropertiesKHRBuilder<'a> = crate::vk1_2::PhysicalDeviceDepthStencilResolvePropertiesBuilder<
    'a,
>;
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubpassDescriptionDepthStencilResolveKHR.html) · Alias
#[doc(alias = "VkSubpassDescriptionDepthStencilResolveKHR")]
#[allow(non_camel_case_types)]
pub type SubpassDescriptionDepthStencilResolveKHR = crate::vk1_2::SubpassDescriptionDepthStencilResolve;
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubpassDescriptionDepthStencilResolveKHR.html) · Alias
#[doc(alias = "VkSubpassDescriptionDepthStencilResolveKHR")]
#[allow(non_camel_case_types)]
pub type SubpassDescriptionDepthStencilResolveKHRBuilder<'a> = crate::vk1_2::SubpassDescriptionDepthStencilResolveBuilder<
    'a,
>;
///Provided by [`crate::extensions::khr_depth_stencil_resolve`]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES_KHR: Self = Self::PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES;
    pub const SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE_KHR: Self = Self::SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE;
}
///Provided by [`crate::extensions::khr_depth_stencil_resolve`]
impl crate::vk1_2::ResolveModeFlagBits {
    pub const NONE_KHR: Self = Self::NONE;
    pub const SAMPLE_ZERO_KHR: Self = Self::SAMPLE_ZERO;
    pub const AVERAGE_KHR: Self = Self::AVERAGE;
    pub const MIN_KHR: Self = Self::MIN;
    pub const MAX_KHR: Self = Self::MAX;
}
