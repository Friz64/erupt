#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_ARM_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_SPEC_VERSION")]
pub const ARM_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_ARM_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_EXTENSION_NAME")]
pub const ARM_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_ARM_rasterization_order_attachment_access");
#[doc = "Provided by [`crate::extensions::arm_rasterization_order_attachment_access`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_ARM: Self = Self(1000342000);
}
#[doc = "Provided by [`crate::extensions::arm_rasterization_order_attachment_access`]"]
impl crate::vk1_0::SubpassDescriptionFlagBits {
    pub const RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_ARM: Self = Self(16);
    pub const RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_ARM: Self = Self(32);
    pub const RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_ARM: Self = Self(64);
}
#[doc = "Provided by [`crate::extensions::arm_rasterization_order_attachment_access`]"]
impl crate::vk1_0::PipelineDepthStencilStateCreateFlagBits {
    pub const RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_ARM: Self = Self(1);
    pub const RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_ARM: Self = Self(2);
}
#[doc = "Provided by [`crate::extensions::arm_rasterization_order_attachment_access`]"]
impl crate::vk1_0::PipelineColorBlendStateCreateFlagBits {
    pub const RASTERIZATION_ORDER_ATTACHMENT_ACCESS_ARM: Self = Self(1);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub rasterization_order_color_attachment_access: crate::vk1_0::Bool32,
    pub rasterization_order_depth_attachment_access: crate::vk1_0::Bool32,
    pub rasterization_order_stencil_attachment_access: crate::vk1_0::Bool32,
}
impl PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_ARM;
}
impl Default for PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), rasterization_order_color_attachment_access: Default::default(), rasterization_order_depth_attachment_access: Default::default(), rasterization_order_stencil_attachment_access: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM").field("s_type", &self.s_type).field("p_next", &self.p_next).field("rasterization_order_color_attachment_access", &(self.rasterization_order_color_attachment_access != 0)).field("rasterization_order_depth_attachment_access", &(self.rasterization_order_depth_attachment_access != 0)).field("rasterization_order_stencil_attachment_access", &(self.rasterization_order_stencil_attachment_access != 0)).finish()
    }
}
impl PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARMBuilder<'a> {
        PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARMBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM.html) · Builder of [`PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM`]"]
#[repr(transparent)]
pub struct PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARMBuilder<'a>(PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARMBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARMBuilder<'a> {
        PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARMBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn rasterization_order_color_attachment_access(mut self, rasterization_order_color_attachment_access: bool) -> Self {
        self.0.rasterization_order_color_attachment_access = rasterization_order_color_attachment_access as _;
        self
    }
    #[inline]
    pub fn rasterization_order_depth_attachment_access(mut self, rasterization_order_depth_attachment_access: bool) -> Self {
        self.0.rasterization_order_depth_attachment_access = rasterization_order_depth_attachment_access as _;
        self
    }
    #[inline]
    pub fn rasterization_order_stencil_attachment_access(mut self, rasterization_order_stencil_attachment_access: bool) -> Self {
        self.0.rasterization_order_stencil_attachment_access = rasterization_order_stencil_attachment_access as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARMBuilder<'a> {
    fn default() -> PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARMBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARMBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARMBuilder<'a> {
    type Target = PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARMBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
