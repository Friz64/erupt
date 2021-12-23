#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_LINEAR_COLOR_ATTACHMENT_SPEC_VERSION")]
pub const NV_LINEAR_COLOR_ATTACHMENT_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_LINEAR_COLOR_ATTACHMENT_EXTENSION_NAME")]
pub const NV_LINEAR_COLOR_ATTACHMENT_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_NV_linear_color_attachment");
#[doc = "Provided by [`crate::extensions::nv_linear_color_attachment`]"]
impl crate::extensions::khr_format_feature_flags2::FormatFeatureFlagBits2KHR {
    pub const LINEAR_COLOR_ATTACHMENT_NV: Self = Self(274877906944);
}
#[doc = "Provided by [`crate::extensions::nv_linear_color_attachment`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_LINEAR_COLOR_ATTACHMENT_FEATURES_NV: Self = Self(1000430000);
}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceLinearColorAttachmentFeaturesNV> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceLinearColorAttachmentFeaturesNVBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceLinearColorAttachmentFeaturesNV> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceLinearColorAttachmentFeaturesNVBuilder<'_>> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceLinearColorAttachmentFeaturesNV.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceLinearColorAttachmentFeaturesNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceLinearColorAttachmentFeaturesNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub linear_color_attachment: crate::vk1_0::Bool32,
}
impl PhysicalDeviceLinearColorAttachmentFeaturesNV {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_LINEAR_COLOR_ATTACHMENT_FEATURES_NV;
}
impl Default for PhysicalDeviceLinearColorAttachmentFeaturesNV {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), linear_color_attachment: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceLinearColorAttachmentFeaturesNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceLinearColorAttachmentFeaturesNV").field("s_type", &self.s_type).field("p_next", &self.p_next).field("linear_color_attachment", &(self.linear_color_attachment != 0)).finish()
    }
}
impl PhysicalDeviceLinearColorAttachmentFeaturesNV {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceLinearColorAttachmentFeaturesNVBuilder<'a> {
        PhysicalDeviceLinearColorAttachmentFeaturesNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceLinearColorAttachmentFeaturesNV.html) · Builder of [`PhysicalDeviceLinearColorAttachmentFeaturesNV`]"]
#[repr(transparent)]
pub struct PhysicalDeviceLinearColorAttachmentFeaturesNVBuilder<'a>(PhysicalDeviceLinearColorAttachmentFeaturesNV, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceLinearColorAttachmentFeaturesNVBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceLinearColorAttachmentFeaturesNVBuilder<'a> {
        PhysicalDeviceLinearColorAttachmentFeaturesNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn linear_color_attachment(mut self, linear_color_attachment: bool) -> Self {
        self.0.linear_color_attachment = linear_color_attachment as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> PhysicalDeviceLinearColorAttachmentFeaturesNV {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceLinearColorAttachmentFeaturesNVBuilder<'a> {
    fn default() -> PhysicalDeviceLinearColorAttachmentFeaturesNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceLinearColorAttachmentFeaturesNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceLinearColorAttachmentFeaturesNVBuilder<'a> {
    type Target = PhysicalDeviceLinearColorAttachmentFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceLinearColorAttachmentFeaturesNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
