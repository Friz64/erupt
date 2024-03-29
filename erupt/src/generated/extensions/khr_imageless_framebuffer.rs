// DO NOT EDIT: @generated by erupt's generator
///<s>Vulkan Manual Page</s> · Constant
#[doc(alias = "VK_KHR_IMAGELESS_FRAMEBUFFER_SPEC_VERSION")]
pub const KHR_IMAGELESS_FRAMEBUFFER_SPEC_VERSION: u32 = 1;
///<s>Vulkan Manual Page</s> · Constant
#[doc(alias = "VK_KHR_IMAGELESS_FRAMEBUFFER_EXTENSION_NAME")]
pub const KHR_IMAGELESS_FRAMEBUFFER_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!(
    "VK_KHR_imageless_framebuffer"
);
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImagelessFramebufferFeaturesKHR.html) · Alias
#[doc(alias = "VkPhysicalDeviceImagelessFramebufferFeaturesKHR")]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceImagelessFramebufferFeaturesKHR = crate::vk1_2::PhysicalDeviceImagelessFramebufferFeatures;
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImagelessFramebufferFeaturesKHR.html) · Alias
#[doc(alias = "VkPhysicalDeviceImagelessFramebufferFeaturesKHR")]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceImagelessFramebufferFeaturesKHRBuilder<'a> = crate::vk1_2::PhysicalDeviceImagelessFramebufferFeaturesBuilder<
    'a,
>;
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFramebufferAttachmentsCreateInfoKHR.html) · Alias
#[doc(alias = "VkFramebufferAttachmentsCreateInfoKHR")]
#[allow(non_camel_case_types)]
pub type FramebufferAttachmentsCreateInfoKHR = crate::vk1_2::FramebufferAttachmentsCreateInfo;
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFramebufferAttachmentsCreateInfoKHR.html) · Alias
#[doc(alias = "VkFramebufferAttachmentsCreateInfoKHR")]
#[allow(non_camel_case_types)]
pub type FramebufferAttachmentsCreateInfoKHRBuilder<'a> = crate::vk1_2::FramebufferAttachmentsCreateInfoBuilder<
    'a,
>;
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFramebufferAttachmentImageInfoKHR.html) · Alias
#[doc(alias = "VkFramebufferAttachmentImageInfoKHR")]
#[allow(non_camel_case_types)]
pub type FramebufferAttachmentImageInfoKHR = crate::vk1_2::FramebufferAttachmentImageInfo;
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFramebufferAttachmentImageInfoKHR.html) · Alias
#[doc(alias = "VkFramebufferAttachmentImageInfoKHR")]
#[allow(non_camel_case_types)]
pub type FramebufferAttachmentImageInfoKHRBuilder<'a> = crate::vk1_2::FramebufferAttachmentImageInfoBuilder<
    'a,
>;
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderPassAttachmentBeginInfoKHR.html) · Alias
#[doc(alias = "VkRenderPassAttachmentBeginInfoKHR")]
#[allow(non_camel_case_types)]
pub type RenderPassAttachmentBeginInfoKHR = crate::vk1_2::RenderPassAttachmentBeginInfo;
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderPassAttachmentBeginInfoKHR.html) · Alias
#[doc(alias = "VkRenderPassAttachmentBeginInfoKHR")]
#[allow(non_camel_case_types)]
pub type RenderPassAttachmentBeginInfoKHRBuilder<'a> = crate::vk1_2::RenderPassAttachmentBeginInfoBuilder<
    'a,
>;
///Provided by [`crate::extensions::khr_imageless_framebuffer`]
impl crate::vk1_0::FramebufferCreateFlagBits {
    pub const IMAGELESS_KHR: Self = Self::IMAGELESS;
}
///Provided by [`crate::extensions::khr_imageless_framebuffer`]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES_KHR: Self = Self::PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES;
    pub const FRAMEBUFFER_ATTACHMENTS_CREATE_INFO_KHR: Self = Self::FRAMEBUFFER_ATTACHMENTS_CREATE_INFO;
    pub const FRAMEBUFFER_ATTACHMENT_IMAGE_INFO_KHR: Self = Self::FRAMEBUFFER_ATTACHMENT_IMAGE_INFO;
    pub const RENDER_PASS_ATTACHMENT_BEGIN_INFO_KHR: Self = Self::RENDER_PASS_ATTACHMENT_BEGIN_INFO;
}
