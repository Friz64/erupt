#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_DYNAMIC_RENDERING_SPEC_VERSION")]
pub const KHR_DYNAMIC_RENDERING_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_DYNAMIC_RENDERING_EXTENSION_NAME")]
pub const KHR_DYNAMIC_RENDERING_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_dynamic_rendering");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_BEGIN_RENDERING_KHR: *const std::os::raw::c_char = crate::cstr!("vkCmdBeginRenderingKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_END_RENDERING_KHR: *const std::os::raw::c_char = crate::cstr!("vkCmdEndRenderingKHR");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAttachmentSampleCountInfoNV.html) · Alias"]
#[doc(alias = "VkAttachmentSampleCountInfoNV")]
#[allow(non_camel_case_types)]
pub type AttachmentSampleCountInfoNV = crate::extensions::khr_dynamic_rendering::AttachmentSampleCountInfoAMD;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAttachmentSampleCountInfoNV.html) · Alias"]
#[doc(alias = "VkAttachmentSampleCountInfoNV")]
#[allow(non_camel_case_types)]
pub type AttachmentSampleCountInfoNVBuilder<'a> = crate::extensions::khr_dynamic_rendering::AttachmentSampleCountInfoAMDBuilder<'a>;
#[doc = "Provided by [`crate::extensions::khr_dynamic_rendering`]"]
impl crate::vk1_0::AttachmentStoreOp {
    pub const NONE_KHR: Self = Self(1000301000);
}
#[doc = "Provided by [`crate::extensions::khr_dynamic_rendering`]"]
impl crate::vk1_0::PipelineCreateFlagBits {
    pub const RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self = Self(2097152);
    pub const RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT: Self = Self(4194304);
    #[deprecated]
    pub const PIPELINE_RASTERIZATION_STATE_CREATE_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self = Self::RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR;
    #[deprecated]
    pub const PIPELINE_RASTERIZATION_STATE_CREATE_FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT: Self = Self::RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT;
}
#[doc = "Provided by [`crate::extensions::khr_dynamic_rendering`]"]
impl crate::vk1_0::StructureType {
    pub const RENDERING_INFO_KHR: Self = Self(1000044000);
    pub const RENDERING_ATTACHMENT_INFO_KHR: Self = Self(1000044001);
    pub const PIPELINE_RENDERING_CREATE_INFO_KHR: Self = Self(1000044002);
    pub const PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES_KHR: Self = Self(1000044003);
    pub const COMMAND_BUFFER_INHERITANCE_RENDERING_INFO_KHR: Self = Self(1000044004);
    pub const RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR: Self = Self(1000044006);
    pub const RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_INFO_EXT: Self = Self(1000044007);
    pub const ATTACHMENT_SAMPLE_COUNT_INFO_AMD: Self = Self(1000044008);
    pub const MULTIVIEW_PER_VIEW_ATTRIBUTES_INFO_NVX: Self = Self(1000044009);
    pub const ATTACHMENT_SAMPLE_COUNT_INFO_NV: Self = Self::ATTACHMENT_SAMPLE_COUNT_INFO_AMD;
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderingFlagsKHR.html) · Bitmask of [`RenderingFlagBitsKHR`]"] # [doc (alias = "VkRenderingFlagsKHR")] # [derive (Default)] # [repr (transparent)] pub struct RenderingFlagsKHR : u32 { const CONTENTS_SECONDARY_COMMAND_BUFFERS_KHR = RenderingFlagBitsKHR :: CONTENTS_SECONDARY_COMMAND_BUFFERS_KHR . 0 ; const SUSPENDING_KHR = RenderingFlagBitsKHR :: SUSPENDING_KHR . 0 ; const RESUMING_KHR = RenderingFlagBitsKHR :: RESUMING_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderingFlagBitsKHR.html) · Bits enum of [`RenderingFlagsKHR`]"]
#[doc(alias = "VkRenderingFlagBitsKHR")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct RenderingFlagBitsKHR(pub u32);
impl RenderingFlagBitsKHR {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> RenderingFlagsKHR {
        RenderingFlagsKHR::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for RenderingFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::CONTENTS_SECONDARY_COMMAND_BUFFERS_KHR => "CONTENTS_SECONDARY_COMMAND_BUFFERS_KHR",
            &Self::SUSPENDING_KHR => "SUSPENDING_KHR",
            &Self::RESUMING_KHR => "RESUMING_KHR",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::khr_dynamic_rendering`]"]
impl crate::extensions::khr_dynamic_rendering::RenderingFlagBitsKHR {
    pub const CONTENTS_SECONDARY_COMMAND_BUFFERS_KHR: Self = Self(1);
    pub const SUSPENDING_KHR: Self = Self(2);
    pub const RESUMING_KHR: Self = Self(4);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBeginRenderingKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBeginRenderingKHR = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, p_rendering_info: *const crate::extensions::khr_dynamic_rendering::RenderingInfoKHR) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdEndRenderingKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdEndRenderingKHR = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer) -> ();
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceDynamicRenderingFeaturesKHR> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceDynamicRenderingFeaturesKHRBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PipelineRenderingCreateInfoKHR> for crate::vk1_0::GraphicsPipelineCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PipelineRenderingCreateInfoKHRBuilder<'_>> for crate::vk1_0::GraphicsPipelineCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, AttachmentSampleCountInfoAMD> for crate::vk1_0::GraphicsPipelineCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, AttachmentSampleCountInfoAMDBuilder<'_>> for crate::vk1_0::GraphicsPipelineCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, MultiviewPerViewAttributesInfoNVX> for crate::vk1_0::GraphicsPipelineCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, MultiviewPerViewAttributesInfoNVXBuilder<'_>> for crate::vk1_0::GraphicsPipelineCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, CommandBufferInheritanceRenderingInfoKHR> for crate::vk1_0::CommandBufferInheritanceInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, CommandBufferInheritanceRenderingInfoKHRBuilder<'_>> for crate::vk1_0::CommandBufferInheritanceInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, AttachmentSampleCountInfoAMD> for crate::vk1_0::CommandBufferInheritanceInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, AttachmentSampleCountInfoAMDBuilder<'_>> for crate::vk1_0::CommandBufferInheritanceInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, MultiviewPerViewAttributesInfoNVX> for crate::vk1_0::CommandBufferInheritanceInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, MultiviewPerViewAttributesInfoNVXBuilder<'_>> for crate::vk1_0::CommandBufferInheritanceInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceDynamicRenderingFeaturesKHR> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceDynamicRenderingFeaturesKHRBuilder<'_>> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineRenderingCreateInfoKHR.html) · Structure"]
#[doc(alias = "VkPipelineRenderingCreateInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineRenderingCreateInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub view_mask: u32,
    pub color_attachment_count: u32,
    pub p_color_attachment_formats: *const crate::vk1_0::Format,
    pub depth_attachment_format: crate::vk1_0::Format,
    pub stencil_attachment_format: crate::vk1_0::Format,
}
impl PipelineRenderingCreateInfoKHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PIPELINE_RENDERING_CREATE_INFO_KHR;
}
impl Default for PipelineRenderingCreateInfoKHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), view_mask: Default::default(), color_attachment_count: Default::default(), p_color_attachment_formats: std::ptr::null(), depth_attachment_format: Default::default(), stencil_attachment_format: Default::default() }
    }
}
impl std::fmt::Debug for PipelineRenderingCreateInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PipelineRenderingCreateInfoKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("view_mask", &self.view_mask).field("color_attachment_count", &self.color_attachment_count).field("p_color_attachment_formats", &self.p_color_attachment_formats).field("depth_attachment_format", &self.depth_attachment_format).field("stencil_attachment_format", &self.stencil_attachment_format).finish()
    }
}
impl PipelineRenderingCreateInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> PipelineRenderingCreateInfoKHRBuilder<'a> {
        PipelineRenderingCreateInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineRenderingCreateInfoKHR.html) · Builder of [`PipelineRenderingCreateInfoKHR`]"]
#[repr(transparent)]
pub struct PipelineRenderingCreateInfoKHRBuilder<'a>(PipelineRenderingCreateInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> PipelineRenderingCreateInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineRenderingCreateInfoKHRBuilder<'a> {
        PipelineRenderingCreateInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn view_mask(mut self, view_mask: u32) -> Self {
        self.0.view_mask = view_mask as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn color_attachment_formats(mut self, color_attachment_formats: &'a [crate::vk1_0::Format]) -> Self {
        self.0.p_color_attachment_formats = color_attachment_formats.as_ptr() as _;
        self.0.color_attachment_count = color_attachment_formats.len() as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn depth_attachment_format(mut self, depth_attachment_format: crate::vk1_0::Format) -> Self {
        self.0.depth_attachment_format = depth_attachment_format as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn stencil_attachment_format(mut self, stencil_attachment_format: crate::vk1_0::Format) -> Self {
        self.0.stencil_attachment_format = stencil_attachment_format as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> PipelineRenderingCreateInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for PipelineRenderingCreateInfoKHRBuilder<'a> {
    fn default() -> PipelineRenderingCreateInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PipelineRenderingCreateInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PipelineRenderingCreateInfoKHRBuilder<'a> {
    type Target = PipelineRenderingCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineRenderingCreateInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderingInfoKHR.html) · Structure"]
#[doc(alias = "VkRenderingInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RenderingInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::khr_dynamic_rendering::RenderingFlagsKHR,
    pub render_area: crate::vk1_0::Rect2D,
    pub layer_count: u32,
    pub view_mask: u32,
    pub color_attachment_count: u32,
    pub p_color_attachments: *const crate::extensions::khr_dynamic_rendering::RenderingAttachmentInfoKHR,
    pub p_depth_attachment: *const crate::extensions::khr_dynamic_rendering::RenderingAttachmentInfoKHR,
    pub p_stencil_attachment: *const crate::extensions::khr_dynamic_rendering::RenderingAttachmentInfoKHR,
}
impl RenderingInfoKHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::RENDERING_INFO_KHR;
}
impl Default for RenderingInfoKHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), flags: Default::default(), render_area: Default::default(), layer_count: Default::default(), view_mask: Default::default(), color_attachment_count: Default::default(), p_color_attachments: std::ptr::null(), p_depth_attachment: std::ptr::null(), p_stencil_attachment: std::ptr::null() }
    }
}
impl std::fmt::Debug for RenderingInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RenderingInfoKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("render_area", &self.render_area).field("layer_count", &self.layer_count).field("view_mask", &self.view_mask).field("color_attachment_count", &self.color_attachment_count).field("p_color_attachments", &self.p_color_attachments).field("p_depth_attachment", &self.p_depth_attachment).field("p_stencil_attachment", &self.p_stencil_attachment).finish()
    }
}
impl RenderingInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> RenderingInfoKHRBuilder<'a> {
        RenderingInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl<'a> crate::ExtendableFrom<'a, RenderingFragmentShadingRateAttachmentInfoKHR> for crate::extensions::khr_dynamic_rendering::RenderingInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, RenderingFragmentShadingRateAttachmentInfoKHRBuilder<'_>> for crate::extensions::khr_dynamic_rendering::RenderingInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, RenderingFragmentDensityMapAttachmentInfoEXT> for crate::extensions::khr_dynamic_rendering::RenderingInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, RenderingFragmentDensityMapAttachmentInfoEXTBuilder<'_>> for crate::extensions::khr_dynamic_rendering::RenderingInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, MultiviewPerViewAttributesInfoNVX> for crate::extensions::khr_dynamic_rendering::RenderingInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, MultiviewPerViewAttributesInfoNVXBuilder<'_>> for crate::extensions::khr_dynamic_rendering::RenderingInfoKHRBuilder<'a> {}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderingInfoKHR.html) · Builder of [`RenderingInfoKHR`]"]
#[repr(transparent)]
pub struct RenderingInfoKHRBuilder<'a>(RenderingInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> RenderingInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> RenderingInfoKHRBuilder<'a> {
        RenderingInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn flags(mut self, flags: crate::extensions::khr_dynamic_rendering::RenderingFlagsKHR) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn render_area(mut self, render_area: crate::vk1_0::Rect2D) -> Self {
        self.0.render_area = render_area as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn layer_count(mut self, layer_count: u32) -> Self {
        self.0.layer_count = layer_count as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn view_mask(mut self, view_mask: u32) -> Self {
        self.0.view_mask = view_mask as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn color_attachments(mut self, color_attachments: &'a [crate::extensions::khr_dynamic_rendering::RenderingAttachmentInfoKHRBuilder]) -> Self {
        self.0.p_color_attachments = color_attachments.as_ptr() as _;
        self.0.color_attachment_count = color_attachments.len() as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn depth_attachment(mut self, depth_attachment: &'a crate::extensions::khr_dynamic_rendering::RenderingAttachmentInfoKHR) -> Self {
        self.0.p_depth_attachment = depth_attachment as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn stencil_attachment(mut self, stencil_attachment: &'a crate::extensions::khr_dynamic_rendering::RenderingAttachmentInfoKHR) -> Self {
        self.0.p_stencil_attachment = stencil_attachment as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> RenderingInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for RenderingInfoKHRBuilder<'a> {
    fn default() -> RenderingInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for RenderingInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for RenderingInfoKHRBuilder<'a> {
    type Target = RenderingInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for RenderingInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderingAttachmentInfoKHR.html) · Structure"]
#[doc(alias = "VkRenderingAttachmentInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RenderingAttachmentInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub image_view: crate::vk1_0::ImageView,
    pub image_layout: crate::vk1_0::ImageLayout,
    pub resolve_mode: crate::vk1_2::ResolveModeFlagBits,
    pub resolve_image_view: crate::vk1_0::ImageView,
    pub resolve_image_layout: crate::vk1_0::ImageLayout,
    pub load_op: crate::vk1_0::AttachmentLoadOp,
    pub store_op: crate::vk1_0::AttachmentStoreOp,
    pub clear_value: crate::vk1_0::ClearValue,
}
impl RenderingAttachmentInfoKHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::RENDERING_ATTACHMENT_INFO_KHR;
}
impl Default for RenderingAttachmentInfoKHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), image_view: Default::default(), image_layout: Default::default(), resolve_mode: Default::default(), resolve_image_view: Default::default(), resolve_image_layout: Default::default(), load_op: Default::default(), store_op: Default::default(), clear_value: Default::default() }
    }
}
impl std::fmt::Debug for RenderingAttachmentInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RenderingAttachmentInfoKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("image_view", &self.image_view).field("image_layout", &self.image_layout).field("resolve_mode", &self.resolve_mode).field("resolve_image_view", &self.resolve_image_view).field("resolve_image_layout", &self.resolve_image_layout).field("load_op", &self.load_op).field("store_op", &self.store_op).field("clear_value", &self.clear_value).finish()
    }
}
impl RenderingAttachmentInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> RenderingAttachmentInfoKHRBuilder<'a> {
        RenderingAttachmentInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderingAttachmentInfoKHR.html) · Builder of [`RenderingAttachmentInfoKHR`]"]
#[repr(transparent)]
pub struct RenderingAttachmentInfoKHRBuilder<'a>(RenderingAttachmentInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> RenderingAttachmentInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> RenderingAttachmentInfoKHRBuilder<'a> {
        RenderingAttachmentInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn image_view(mut self, image_view: crate::vk1_0::ImageView) -> Self {
        self.0.image_view = image_view as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn image_layout(mut self, image_layout: crate::vk1_0::ImageLayout) -> Self {
        self.0.image_layout = image_layout as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn resolve_mode(mut self, resolve_mode: crate::vk1_2::ResolveModeFlagBits) -> Self {
        self.0.resolve_mode = resolve_mode as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn resolve_image_view(mut self, resolve_image_view: crate::vk1_0::ImageView) -> Self {
        self.0.resolve_image_view = resolve_image_view as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn resolve_image_layout(mut self, resolve_image_layout: crate::vk1_0::ImageLayout) -> Self {
        self.0.resolve_image_layout = resolve_image_layout as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn load_op(mut self, load_op: crate::vk1_0::AttachmentLoadOp) -> Self {
        self.0.load_op = load_op as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn store_op(mut self, store_op: crate::vk1_0::AttachmentStoreOp) -> Self {
        self.0.store_op = store_op as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn clear_value(mut self, clear_value: crate::vk1_0::ClearValue) -> Self {
        self.0.clear_value = clear_value as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> RenderingAttachmentInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for RenderingAttachmentInfoKHRBuilder<'a> {
    fn default() -> RenderingAttachmentInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for RenderingAttachmentInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for RenderingAttachmentInfoKHRBuilder<'a> {
    type Target = RenderingAttachmentInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for RenderingAttachmentInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderingFragmentShadingRateAttachmentInfoKHR.html) · Structure"]
#[doc(alias = "VkRenderingFragmentShadingRateAttachmentInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RenderingFragmentShadingRateAttachmentInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub image_view: crate::vk1_0::ImageView,
    pub image_layout: crate::vk1_0::ImageLayout,
    pub shading_rate_attachment_texel_size: crate::vk1_0::Extent2D,
}
impl RenderingFragmentShadingRateAttachmentInfoKHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR;
}
impl Default for RenderingFragmentShadingRateAttachmentInfoKHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), image_view: Default::default(), image_layout: Default::default(), shading_rate_attachment_texel_size: Default::default() }
    }
}
impl std::fmt::Debug for RenderingFragmentShadingRateAttachmentInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RenderingFragmentShadingRateAttachmentInfoKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("image_view", &self.image_view).field("image_layout", &self.image_layout).field("shading_rate_attachment_texel_size", &self.shading_rate_attachment_texel_size).finish()
    }
}
impl RenderingFragmentShadingRateAttachmentInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> RenderingFragmentShadingRateAttachmentInfoKHRBuilder<'a> {
        RenderingFragmentShadingRateAttachmentInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderingFragmentShadingRateAttachmentInfoKHR.html) · Builder of [`RenderingFragmentShadingRateAttachmentInfoKHR`]"]
#[repr(transparent)]
pub struct RenderingFragmentShadingRateAttachmentInfoKHRBuilder<'a>(RenderingFragmentShadingRateAttachmentInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> RenderingFragmentShadingRateAttachmentInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> RenderingFragmentShadingRateAttachmentInfoKHRBuilder<'a> {
        RenderingFragmentShadingRateAttachmentInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn image_view(mut self, image_view: crate::vk1_0::ImageView) -> Self {
        self.0.image_view = image_view as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn image_layout(mut self, image_layout: crate::vk1_0::ImageLayout) -> Self {
        self.0.image_layout = image_layout as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn shading_rate_attachment_texel_size(mut self, shading_rate_attachment_texel_size: crate::vk1_0::Extent2D) -> Self {
        self.0.shading_rate_attachment_texel_size = shading_rate_attachment_texel_size as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> RenderingFragmentShadingRateAttachmentInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for RenderingFragmentShadingRateAttachmentInfoKHRBuilder<'a> {
    fn default() -> RenderingFragmentShadingRateAttachmentInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for RenderingFragmentShadingRateAttachmentInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for RenderingFragmentShadingRateAttachmentInfoKHRBuilder<'a> {
    type Target = RenderingFragmentShadingRateAttachmentInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for RenderingFragmentShadingRateAttachmentInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderingFragmentDensityMapAttachmentInfoEXT.html) · Structure"]
#[doc(alias = "VkRenderingFragmentDensityMapAttachmentInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RenderingFragmentDensityMapAttachmentInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub image_view: crate::vk1_0::ImageView,
    pub image_layout: crate::vk1_0::ImageLayout,
}
impl RenderingFragmentDensityMapAttachmentInfoEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_INFO_EXT;
}
impl Default for RenderingFragmentDensityMapAttachmentInfoEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), image_view: Default::default(), image_layout: Default::default() }
    }
}
impl std::fmt::Debug for RenderingFragmentDensityMapAttachmentInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RenderingFragmentDensityMapAttachmentInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("image_view", &self.image_view).field("image_layout", &self.image_layout).finish()
    }
}
impl RenderingFragmentDensityMapAttachmentInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> RenderingFragmentDensityMapAttachmentInfoEXTBuilder<'a> {
        RenderingFragmentDensityMapAttachmentInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderingFragmentDensityMapAttachmentInfoEXT.html) · Builder of [`RenderingFragmentDensityMapAttachmentInfoEXT`]"]
#[repr(transparent)]
pub struct RenderingFragmentDensityMapAttachmentInfoEXTBuilder<'a>(RenderingFragmentDensityMapAttachmentInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> RenderingFragmentDensityMapAttachmentInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> RenderingFragmentDensityMapAttachmentInfoEXTBuilder<'a> {
        RenderingFragmentDensityMapAttachmentInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn image_view(mut self, image_view: crate::vk1_0::ImageView) -> Self {
        self.0.image_view = image_view as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn image_layout(mut self, image_layout: crate::vk1_0::ImageLayout) -> Self {
        self.0.image_layout = image_layout as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> RenderingFragmentDensityMapAttachmentInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for RenderingFragmentDensityMapAttachmentInfoEXTBuilder<'a> {
    fn default() -> RenderingFragmentDensityMapAttachmentInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for RenderingFragmentDensityMapAttachmentInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for RenderingFragmentDensityMapAttachmentInfoEXTBuilder<'a> {
    type Target = RenderingFragmentDensityMapAttachmentInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for RenderingFragmentDensityMapAttachmentInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDynamicRenderingFeaturesKHR.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceDynamicRenderingFeaturesKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceDynamicRenderingFeaturesKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub dynamic_rendering: crate::vk1_0::Bool32,
}
impl PhysicalDeviceDynamicRenderingFeaturesKHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES_KHR;
}
impl Default for PhysicalDeviceDynamicRenderingFeaturesKHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), dynamic_rendering: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceDynamicRenderingFeaturesKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceDynamicRenderingFeaturesKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("dynamic_rendering", &(self.dynamic_rendering != 0)).finish()
    }
}
impl PhysicalDeviceDynamicRenderingFeaturesKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceDynamicRenderingFeaturesKHRBuilder<'a> {
        PhysicalDeviceDynamicRenderingFeaturesKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDynamicRenderingFeaturesKHR.html) · Builder of [`PhysicalDeviceDynamicRenderingFeaturesKHR`]"]
#[repr(transparent)]
pub struct PhysicalDeviceDynamicRenderingFeaturesKHRBuilder<'a>(PhysicalDeviceDynamicRenderingFeaturesKHR, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceDynamicRenderingFeaturesKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceDynamicRenderingFeaturesKHRBuilder<'a> {
        PhysicalDeviceDynamicRenderingFeaturesKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn dynamic_rendering(mut self, dynamic_rendering: bool) -> Self {
        self.0.dynamic_rendering = dynamic_rendering as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> PhysicalDeviceDynamicRenderingFeaturesKHR {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceDynamicRenderingFeaturesKHRBuilder<'a> {
    fn default() -> PhysicalDeviceDynamicRenderingFeaturesKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceDynamicRenderingFeaturesKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceDynamicRenderingFeaturesKHRBuilder<'a> {
    type Target = PhysicalDeviceDynamicRenderingFeaturesKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceDynamicRenderingFeaturesKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandBufferInheritanceRenderingInfoKHR.html) · Structure"]
#[doc(alias = "VkCommandBufferInheritanceRenderingInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CommandBufferInheritanceRenderingInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::khr_dynamic_rendering::RenderingFlagsKHR,
    pub view_mask: u32,
    pub color_attachment_count: u32,
    pub p_color_attachment_formats: *const crate::vk1_0::Format,
    pub depth_attachment_format: crate::vk1_0::Format,
    pub stencil_attachment_format: crate::vk1_0::Format,
    pub rasterization_samples: crate::vk1_0::SampleCountFlagBits,
}
impl CommandBufferInheritanceRenderingInfoKHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::COMMAND_BUFFER_INHERITANCE_RENDERING_INFO_KHR;
}
impl Default for CommandBufferInheritanceRenderingInfoKHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), flags: Default::default(), view_mask: Default::default(), color_attachment_count: Default::default(), p_color_attachment_formats: std::ptr::null(), depth_attachment_format: Default::default(), stencil_attachment_format: Default::default(), rasterization_samples: Default::default() }
    }
}
impl std::fmt::Debug for CommandBufferInheritanceRenderingInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CommandBufferInheritanceRenderingInfoKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("view_mask", &self.view_mask).field("color_attachment_count", &self.color_attachment_count).field("p_color_attachment_formats", &self.p_color_attachment_formats).field("depth_attachment_format", &self.depth_attachment_format).field("stencil_attachment_format", &self.stencil_attachment_format).field("rasterization_samples", &self.rasterization_samples).finish()
    }
}
impl CommandBufferInheritanceRenderingInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> CommandBufferInheritanceRenderingInfoKHRBuilder<'a> {
        CommandBufferInheritanceRenderingInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandBufferInheritanceRenderingInfoKHR.html) · Builder of [`CommandBufferInheritanceRenderingInfoKHR`]"]
#[repr(transparent)]
pub struct CommandBufferInheritanceRenderingInfoKHRBuilder<'a>(CommandBufferInheritanceRenderingInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> CommandBufferInheritanceRenderingInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> CommandBufferInheritanceRenderingInfoKHRBuilder<'a> {
        CommandBufferInheritanceRenderingInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn flags(mut self, flags: crate::extensions::khr_dynamic_rendering::RenderingFlagsKHR) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn view_mask(mut self, view_mask: u32) -> Self {
        self.0.view_mask = view_mask as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn color_attachment_formats(mut self, color_attachment_formats: &'a [crate::vk1_0::Format]) -> Self {
        self.0.p_color_attachment_formats = color_attachment_formats.as_ptr() as _;
        self.0.color_attachment_count = color_attachment_formats.len() as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn depth_attachment_format(mut self, depth_attachment_format: crate::vk1_0::Format) -> Self {
        self.0.depth_attachment_format = depth_attachment_format as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn stencil_attachment_format(mut self, stencil_attachment_format: crate::vk1_0::Format) -> Self {
        self.0.stencil_attachment_format = stencil_attachment_format as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn rasterization_samples(mut self, rasterization_samples: crate::vk1_0::SampleCountFlagBits) -> Self {
        self.0.rasterization_samples = rasterization_samples as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> CommandBufferInheritanceRenderingInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for CommandBufferInheritanceRenderingInfoKHRBuilder<'a> {
    fn default() -> CommandBufferInheritanceRenderingInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for CommandBufferInheritanceRenderingInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for CommandBufferInheritanceRenderingInfoKHRBuilder<'a> {
    type Target = CommandBufferInheritanceRenderingInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for CommandBufferInheritanceRenderingInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAttachmentSampleCountInfoAMD.html) · Structure"]
#[doc(alias = "VkAttachmentSampleCountInfoAMD")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AttachmentSampleCountInfoAMD {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub color_attachment_count: u32,
    pub p_color_attachment_samples: *const crate::vk1_0::SampleCountFlagBits,
    pub depth_stencil_attachment_samples: crate::vk1_0::SampleCountFlagBits,
}
impl AttachmentSampleCountInfoAMD {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::ATTACHMENT_SAMPLE_COUNT_INFO_AMD;
}
impl Default for AttachmentSampleCountInfoAMD {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), color_attachment_count: Default::default(), p_color_attachment_samples: std::ptr::null(), depth_stencil_attachment_samples: Default::default() }
    }
}
impl std::fmt::Debug for AttachmentSampleCountInfoAMD {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AttachmentSampleCountInfoAMD").field("s_type", &self.s_type).field("p_next", &self.p_next).field("color_attachment_count", &self.color_attachment_count).field("p_color_attachment_samples", &self.p_color_attachment_samples).field("depth_stencil_attachment_samples", &self.depth_stencil_attachment_samples).finish()
    }
}
impl AttachmentSampleCountInfoAMD {
    #[inline]
    pub fn into_builder<'a>(self) -> AttachmentSampleCountInfoAMDBuilder<'a> {
        AttachmentSampleCountInfoAMDBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAttachmentSampleCountInfoAMD.html) · Builder of [`AttachmentSampleCountInfoAMD`]"]
#[repr(transparent)]
pub struct AttachmentSampleCountInfoAMDBuilder<'a>(AttachmentSampleCountInfoAMD, std::marker::PhantomData<&'a ()>);
impl<'a> AttachmentSampleCountInfoAMDBuilder<'a> {
    #[inline]
    pub fn new() -> AttachmentSampleCountInfoAMDBuilder<'a> {
        AttachmentSampleCountInfoAMDBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn color_attachment_samples(mut self, color_attachment_samples: &'a [crate::vk1_0::SampleCountFlagBits]) -> Self {
        self.0.p_color_attachment_samples = color_attachment_samples.as_ptr() as _;
        self.0.color_attachment_count = color_attachment_samples.len() as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn depth_stencil_attachment_samples(mut self, depth_stencil_attachment_samples: crate::vk1_0::SampleCountFlagBits) -> Self {
        self.0.depth_stencil_attachment_samples = depth_stencil_attachment_samples as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> AttachmentSampleCountInfoAMD {
        self.0
    }
}
impl<'a> std::default::Default for AttachmentSampleCountInfoAMDBuilder<'a> {
    fn default() -> AttachmentSampleCountInfoAMDBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for AttachmentSampleCountInfoAMDBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for AttachmentSampleCountInfoAMDBuilder<'a> {
    type Target = AttachmentSampleCountInfoAMD;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for AttachmentSampleCountInfoAMDBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMultiviewPerViewAttributesInfoNVX.html) · Structure"]
#[doc(alias = "VkMultiviewPerViewAttributesInfoNVX")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MultiviewPerViewAttributesInfoNVX {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub per_view_attributes: crate::vk1_0::Bool32,
    pub per_view_attributes_position_x_only: crate::vk1_0::Bool32,
}
impl MultiviewPerViewAttributesInfoNVX {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::MULTIVIEW_PER_VIEW_ATTRIBUTES_INFO_NVX;
}
impl Default for MultiviewPerViewAttributesInfoNVX {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), per_view_attributes: Default::default(), per_view_attributes_position_x_only: Default::default() }
    }
}
impl std::fmt::Debug for MultiviewPerViewAttributesInfoNVX {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MultiviewPerViewAttributesInfoNVX").field("s_type", &self.s_type).field("p_next", &self.p_next).field("per_view_attributes", &(self.per_view_attributes != 0)).field("per_view_attributes_position_x_only", &(self.per_view_attributes_position_x_only != 0)).finish()
    }
}
impl MultiviewPerViewAttributesInfoNVX {
    #[inline]
    pub fn into_builder<'a>(self) -> MultiviewPerViewAttributesInfoNVXBuilder<'a> {
        MultiviewPerViewAttributesInfoNVXBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMultiviewPerViewAttributesInfoNVX.html) · Builder of [`MultiviewPerViewAttributesInfoNVX`]"]
#[repr(transparent)]
pub struct MultiviewPerViewAttributesInfoNVXBuilder<'a>(MultiviewPerViewAttributesInfoNVX, std::marker::PhantomData<&'a ()>);
impl<'a> MultiviewPerViewAttributesInfoNVXBuilder<'a> {
    #[inline]
    pub fn new() -> MultiviewPerViewAttributesInfoNVXBuilder<'a> {
        MultiviewPerViewAttributesInfoNVXBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn per_view_attributes(mut self, per_view_attributes: bool) -> Self {
        self.0.per_view_attributes = per_view_attributes as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn per_view_attributes_position_x_only(mut self, per_view_attributes_position_x_only: bool) -> Self {
        self.0.per_view_attributes_position_x_only = per_view_attributes_position_x_only as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> MultiviewPerViewAttributesInfoNVX {
        self.0
    }
}
impl<'a> std::default::Default for MultiviewPerViewAttributesInfoNVXBuilder<'a> {
    fn default() -> MultiviewPerViewAttributesInfoNVXBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for MultiviewPerViewAttributesInfoNVXBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for MultiviewPerViewAttributesInfoNVXBuilder<'a> {
    type Target = MultiviewPerViewAttributesInfoNVX;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for MultiviewPerViewAttributesInfoNVXBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::khr_dynamic_rendering`]"]
impl crate::DeviceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBeginRenderingKHR.html) · Function"]
    #[doc(alias = "vkCmdBeginRenderingKHR")]
    pub unsafe fn cmd_begin_rendering_khr(&self, command_buffer: crate::vk1_0::CommandBuffer, rendering_info: &crate::extensions::khr_dynamic_rendering::RenderingInfoKHR) -> () {
        let _function = self.cmd_begin_rendering_khr.expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(command_buffer as _, rendering_info as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdEndRenderingKHR.html) · Function"]
    #[doc(alias = "vkCmdEndRenderingKHR")]
    pub unsafe fn cmd_end_rendering_khr(&self, command_buffer: crate::vk1_0::CommandBuffer) -> () {
        let _function = self.cmd_end_rendering_khr.expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(command_buffer as _);
        ()
    }
}
