#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_SHADING_RATE_IMAGE_SPEC_VERSION")]
pub const NV_SHADING_RATE_IMAGE_SPEC_VERSION: u32 = 3;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_SHADING_RATE_IMAGE_EXTENSION_NAME")]
pub const NV_SHADING_RATE_IMAGE_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_NV_shading_rate_image");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_BIND_SHADING_RATE_IMAGE_NV: *const std::os::raw::c_char = crate::cstr!("vkCmdBindShadingRateImageNV");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_SET_VIEWPORT_SHADING_RATE_PALETTE_NV: *const std::os::raw::c_char = crate::cstr!("vkCmdSetViewportShadingRatePaletteNV");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_SET_COARSE_SAMPLE_ORDER_NV: *const std::os::raw::c_char = crate::cstr!("vkCmdSetCoarseSampleOrderNV");
#[doc = "Provided by [`crate::extensions::nv_shading_rate_image`]"]
impl crate::vk1_0::DynamicState {
    pub const VIEWPORT_SHADING_RATE_PALETTE_NV: Self = Self(1000164004);
    pub const VIEWPORT_COARSE_SAMPLE_ORDER_NV: Self = Self(1000164006);
}
#[doc = "Provided by [`crate::extensions::nv_shading_rate_image`]"]
impl crate::vk1_0::ImageLayout {
    pub const SHADING_RATE_OPTIMAL_NV: Self = Self::FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR;
}
#[doc = "Provided by [`crate::extensions::nv_shading_rate_image`]"]
impl crate::vk1_0::ImageUsageFlagBits {
    pub const SHADING_RATE_IMAGE_NV: Self = Self::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR;
}
#[doc = "Provided by [`crate::extensions::nv_shading_rate_image`]"]
impl crate::vk1_0::AccessFlagBits {
    pub const SHADING_RATE_IMAGE_READ_NV: Self = Self::FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR;
}
#[doc = "Provided by [`crate::extensions::nv_shading_rate_image`]"]
impl crate::vk1_0::StructureType {
    pub const PIPELINE_VIEWPORT_SHADING_RATE_IMAGE_STATE_CREATE_INFO_NV: Self = Self(1000164000);
    pub const PHYSICAL_DEVICE_SHADING_RATE_IMAGE_FEATURES_NV: Self = Self(1000164001);
    pub const PHYSICAL_DEVICE_SHADING_RATE_IMAGE_PROPERTIES_NV: Self = Self(1000164002);
    pub const PIPELINE_VIEWPORT_COARSE_SAMPLE_ORDER_STATE_CREATE_INFO_NV: Self = Self(1000164005);
}
#[doc = "Provided by [`crate::extensions::nv_shading_rate_image`]"]
impl crate::vk1_0::PipelineStageFlagBits {
    pub const SHADING_RATE_IMAGE_NV: Self = Self::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR;
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkShadingRatePaletteEntryNV.html) · Enum"]
#[doc(alias = "VkShadingRatePaletteEntryNV")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct ShadingRatePaletteEntryNV(pub i32);
impl std::fmt::Debug for ShadingRatePaletteEntryNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::NO_INVOCATIONS_NV => "NO_INVOCATIONS_NV",
            &Self::_16_INVOCATIONS_PER_PIXEL_NV => "_16_INVOCATIONS_PER_PIXEL_NV",
            &Self::_8_INVOCATIONS_PER_PIXEL_NV => "_8_INVOCATIONS_PER_PIXEL_NV",
            &Self::_4_INVOCATIONS_PER_PIXEL_NV => "_4_INVOCATIONS_PER_PIXEL_NV",
            &Self::_2_INVOCATIONS_PER_PIXEL_NV => "_2_INVOCATIONS_PER_PIXEL_NV",
            &Self::_1_INVOCATION_PER_PIXEL_NV => "_1_INVOCATION_PER_PIXEL_NV",
            &Self::_1_INVOCATION_PER_2X1_PIXELS_NV => "_1_INVOCATION_PER_2X1_PIXELS_NV",
            &Self::_1_INVOCATION_PER_1X2_PIXELS_NV => "_1_INVOCATION_PER_1X2_PIXELS_NV",
            &Self::_1_INVOCATION_PER_2X2_PIXELS_NV => "_1_INVOCATION_PER_2X2_PIXELS_NV",
            &Self::_1_INVOCATION_PER_4X2_PIXELS_NV => "_1_INVOCATION_PER_4X2_PIXELS_NV",
            &Self::_1_INVOCATION_PER_2X4_PIXELS_NV => "_1_INVOCATION_PER_2X4_PIXELS_NV",
            &Self::_1_INVOCATION_PER_4X4_PIXELS_NV => "_1_INVOCATION_PER_4X4_PIXELS_NV",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::nv_shading_rate_image`]"]
impl crate::extensions::nv_shading_rate_image::ShadingRatePaletteEntryNV {
    pub const NO_INVOCATIONS_NV: Self = Self(0);
    pub const _16_INVOCATIONS_PER_PIXEL_NV: Self = Self(1);
    pub const _8_INVOCATIONS_PER_PIXEL_NV: Self = Self(2);
    pub const _4_INVOCATIONS_PER_PIXEL_NV: Self = Self(3);
    pub const _2_INVOCATIONS_PER_PIXEL_NV: Self = Self(4);
    pub const _1_INVOCATION_PER_PIXEL_NV: Self = Self(5);
    pub const _1_INVOCATION_PER_2X1_PIXELS_NV: Self = Self(6);
    pub const _1_INVOCATION_PER_1X2_PIXELS_NV: Self = Self(7);
    pub const _1_INVOCATION_PER_2X2_PIXELS_NV: Self = Self(8);
    pub const _1_INVOCATION_PER_4X2_PIXELS_NV: Self = Self(9);
    pub const _1_INVOCATION_PER_2X4_PIXELS_NV: Self = Self(10);
    pub const _1_INVOCATION_PER_4X4_PIXELS_NV: Self = Self(11);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCoarseSampleOrderTypeNV.html) · Enum"]
#[doc(alias = "VkCoarseSampleOrderTypeNV")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct CoarseSampleOrderTypeNV(pub i32);
impl std::fmt::Debug for CoarseSampleOrderTypeNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::DEFAULT_NV => "DEFAULT_NV",
            &Self::CUSTOM_NV => "CUSTOM_NV",
            &Self::PIXEL_MAJOR_NV => "PIXEL_MAJOR_NV",
            &Self::SAMPLE_MAJOR_NV => "SAMPLE_MAJOR_NV",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::nv_shading_rate_image`]"]
impl crate::extensions::nv_shading_rate_image::CoarseSampleOrderTypeNV {
    pub const DEFAULT_NV: Self = Self(0);
    pub const CUSTOM_NV: Self = Self(1);
    pub const PIXEL_MAJOR_NV: Self = Self(2);
    pub const SAMPLE_MAJOR_NV: Self = Self(3);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBindShadingRateImageNV.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindShadingRateImageNV = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, image_view: crate::vk1_0::ImageView, image_layout: crate::vk1_0::ImageLayout) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetViewportShadingRatePaletteNV.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetViewportShadingRatePaletteNV = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, first_viewport: u32, viewport_count: u32, p_shading_rate_palettes: *const crate::extensions::nv_shading_rate_image::ShadingRatePaletteNV) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetCoarseSampleOrderNV.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetCoarseSampleOrderNV = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, sample_order_type: crate::extensions::nv_shading_rate_image::CoarseSampleOrderTypeNV, custom_sample_order_count: u32, p_custom_sample_orders: *const crate::extensions::nv_shading_rate_image::CoarseSampleOrderCustomNV) -> ();
impl<'a> crate::ExtendableFromConst<'a, PhysicalDeviceShadingRateImageFeaturesNV> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, PhysicalDeviceShadingRateImageFeaturesNVBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, PipelineViewportShadingRateImageStateCreateInfoNV> for crate::vk1_0::PipelineViewportStateCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, PipelineViewportShadingRateImageStateCreateInfoNVBuilder<'_>> for crate::vk1_0::PipelineViewportStateCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, PipelineViewportCoarseSampleOrderStateCreateInfoNV> for crate::vk1_0::PipelineViewportStateCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, PipelineViewportCoarseSampleOrderStateCreateInfoNVBuilder<'_>> for crate::vk1_0::PipelineViewportStateCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceShadingRateImageFeaturesNV> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceShadingRateImageFeaturesNVBuilder<'_>> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceShadingRateImagePropertiesNV> for crate::vk1_1::PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceShadingRateImagePropertiesNVBuilder<'_>> for crate::vk1_1::PhysicalDeviceProperties2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkShadingRatePaletteNV.html) · Structure"]
#[doc(alias = "VkShadingRatePaletteNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ShadingRatePaletteNV {
    pub shading_rate_palette_entry_count: u32,
    pub p_shading_rate_palette_entries: *const crate::extensions::nv_shading_rate_image::ShadingRatePaletteEntryNV,
}
impl Default for ShadingRatePaletteNV {
    fn default() -> Self {
        Self { shading_rate_palette_entry_count: Default::default(), p_shading_rate_palette_entries: std::ptr::null() }
    }
}
impl std::fmt::Debug for ShadingRatePaletteNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ShadingRatePaletteNV").field("shading_rate_palette_entry_count", &self.shading_rate_palette_entry_count).field("p_shading_rate_palette_entries", &self.p_shading_rate_palette_entries).finish()
    }
}
impl ShadingRatePaletteNV {
    #[inline]
    pub fn into_builder<'a>(self) -> ShadingRatePaletteNVBuilder<'a> {
        ShadingRatePaletteNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkShadingRatePaletteNV.html) · Builder of [`ShadingRatePaletteNV`]"]
#[repr(transparent)]
pub struct ShadingRatePaletteNVBuilder<'a>(ShadingRatePaletteNV, std::marker::PhantomData<&'a ()>);
impl<'a> ShadingRatePaletteNVBuilder<'a> {
    #[inline]
    pub fn new() -> ShadingRatePaletteNVBuilder<'a> {
        ShadingRatePaletteNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn shading_rate_palette_entries(mut self, shading_rate_palette_entries: &'a [crate::extensions::nv_shading_rate_image::ShadingRatePaletteEntryNV]) -> Self {
        self.0.p_shading_rate_palette_entries = shading_rate_palette_entries.as_ptr() as _;
        self.0.shading_rate_palette_entry_count = shading_rate_palette_entries.len() as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ShadingRatePaletteNV {
        self.0
    }
}
impl<'a> std::default::Default for ShadingRatePaletteNVBuilder<'a> {
    fn default() -> ShadingRatePaletteNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ShadingRatePaletteNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ShadingRatePaletteNVBuilder<'a> {
    type Target = ShadingRatePaletteNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ShadingRatePaletteNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineViewportShadingRateImageStateCreateInfoNV.html) · Structure"]
#[doc(alias = "VkPipelineViewportShadingRateImageStateCreateInfoNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineViewportShadingRateImageStateCreateInfoNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub shading_rate_image_enable: crate::vk1_0::Bool32,
    pub viewport_count: u32,
    pub p_shading_rate_palettes: *const crate::extensions::nv_shading_rate_image::ShadingRatePaletteNV,
}
impl PipelineViewportShadingRateImageStateCreateInfoNV {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PIPELINE_VIEWPORT_SHADING_RATE_IMAGE_STATE_CREATE_INFO_NV;
}
impl Default for PipelineViewportShadingRateImageStateCreateInfoNV {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), shading_rate_image_enable: Default::default(), viewport_count: Default::default(), p_shading_rate_palettes: std::ptr::null() }
    }
}
impl std::fmt::Debug for PipelineViewportShadingRateImageStateCreateInfoNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PipelineViewportShadingRateImageStateCreateInfoNV").field("s_type", &self.s_type).field("p_next", &self.p_next).field("shading_rate_image_enable", &(self.shading_rate_image_enable != 0)).field("viewport_count", &self.viewport_count).field("p_shading_rate_palettes", &self.p_shading_rate_palettes).finish()
    }
}
impl PipelineViewportShadingRateImageStateCreateInfoNV {
    #[inline]
    pub fn into_builder<'a>(self) -> PipelineViewportShadingRateImageStateCreateInfoNVBuilder<'a> {
        PipelineViewportShadingRateImageStateCreateInfoNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineViewportShadingRateImageStateCreateInfoNV.html) · Builder of [`PipelineViewportShadingRateImageStateCreateInfoNV`]"]
#[repr(transparent)]
pub struct PipelineViewportShadingRateImageStateCreateInfoNVBuilder<'a>(PipelineViewportShadingRateImageStateCreateInfoNV, std::marker::PhantomData<&'a ()>);
impl<'a> PipelineViewportShadingRateImageStateCreateInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineViewportShadingRateImageStateCreateInfoNVBuilder<'a> {
        PipelineViewportShadingRateImageStateCreateInfoNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn shading_rate_image_enable(mut self, shading_rate_image_enable: bool) -> Self {
        self.0.shading_rate_image_enable = shading_rate_image_enable as _;
        self
    }
    #[inline]
    pub fn shading_rate_palettes(mut self, shading_rate_palettes: &'a [crate::extensions::nv_shading_rate_image::ShadingRatePaletteNVBuilder]) -> Self {
        self.0.p_shading_rate_palettes = shading_rate_palettes.as_ptr() as _;
        self.0.viewport_count = shading_rate_palettes.len() as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PipelineViewportShadingRateImageStateCreateInfoNV {
        self.0
    }
}
impl<'a> std::default::Default for PipelineViewportShadingRateImageStateCreateInfoNVBuilder<'a> {
    fn default() -> PipelineViewportShadingRateImageStateCreateInfoNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PipelineViewportShadingRateImageStateCreateInfoNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PipelineViewportShadingRateImageStateCreateInfoNVBuilder<'a> {
    type Target = PipelineViewportShadingRateImageStateCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineViewportShadingRateImageStateCreateInfoNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShadingRateImageFeaturesNV.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceShadingRateImageFeaturesNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceShadingRateImageFeaturesNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub shading_rate_image: crate::vk1_0::Bool32,
    pub shading_rate_coarse_sample_order: crate::vk1_0::Bool32,
}
impl PhysicalDeviceShadingRateImageFeaturesNV {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_SHADING_RATE_IMAGE_FEATURES_NV;
}
impl Default for PhysicalDeviceShadingRateImageFeaturesNV {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), shading_rate_image: Default::default(), shading_rate_coarse_sample_order: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceShadingRateImageFeaturesNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceShadingRateImageFeaturesNV").field("s_type", &self.s_type).field("p_next", &self.p_next).field("shading_rate_image", &(self.shading_rate_image != 0)).field("shading_rate_coarse_sample_order", &(self.shading_rate_coarse_sample_order != 0)).finish()
    }
}
impl PhysicalDeviceShadingRateImageFeaturesNV {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceShadingRateImageFeaturesNVBuilder<'a> {
        PhysicalDeviceShadingRateImageFeaturesNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShadingRateImageFeaturesNV.html) · Builder of [`PhysicalDeviceShadingRateImageFeaturesNV`]"]
#[repr(transparent)]
pub struct PhysicalDeviceShadingRateImageFeaturesNVBuilder<'a>(PhysicalDeviceShadingRateImageFeaturesNV, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceShadingRateImageFeaturesNVBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceShadingRateImageFeaturesNVBuilder<'a> {
        PhysicalDeviceShadingRateImageFeaturesNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn shading_rate_image(mut self, shading_rate_image: bool) -> Self {
        self.0.shading_rate_image = shading_rate_image as _;
        self
    }
    #[inline]
    pub fn shading_rate_coarse_sample_order(mut self, shading_rate_coarse_sample_order: bool) -> Self {
        self.0.shading_rate_coarse_sample_order = shading_rate_coarse_sample_order as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceShadingRateImageFeaturesNV {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceShadingRateImageFeaturesNVBuilder<'a> {
    fn default() -> PhysicalDeviceShadingRateImageFeaturesNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceShadingRateImageFeaturesNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceShadingRateImageFeaturesNVBuilder<'a> {
    type Target = PhysicalDeviceShadingRateImageFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceShadingRateImageFeaturesNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShadingRateImagePropertiesNV.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceShadingRateImagePropertiesNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceShadingRateImagePropertiesNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub shading_rate_texel_size: crate::vk1_0::Extent2D,
    pub shading_rate_palette_size: u32,
    pub shading_rate_max_coarse_samples: u32,
}
impl PhysicalDeviceShadingRateImagePropertiesNV {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_SHADING_RATE_IMAGE_PROPERTIES_NV;
}
impl Default for PhysicalDeviceShadingRateImagePropertiesNV {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), shading_rate_texel_size: Default::default(), shading_rate_palette_size: Default::default(), shading_rate_max_coarse_samples: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceShadingRateImagePropertiesNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceShadingRateImagePropertiesNV").field("s_type", &self.s_type).field("p_next", &self.p_next).field("shading_rate_texel_size", &self.shading_rate_texel_size).field("shading_rate_palette_size", &self.shading_rate_palette_size).field("shading_rate_max_coarse_samples", &self.shading_rate_max_coarse_samples).finish()
    }
}
impl PhysicalDeviceShadingRateImagePropertiesNV {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceShadingRateImagePropertiesNVBuilder<'a> {
        PhysicalDeviceShadingRateImagePropertiesNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShadingRateImagePropertiesNV.html) · Builder of [`PhysicalDeviceShadingRateImagePropertiesNV`]"]
#[repr(transparent)]
pub struct PhysicalDeviceShadingRateImagePropertiesNVBuilder<'a>(PhysicalDeviceShadingRateImagePropertiesNV, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceShadingRateImagePropertiesNVBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceShadingRateImagePropertiesNVBuilder<'a> {
        PhysicalDeviceShadingRateImagePropertiesNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn shading_rate_texel_size(mut self, shading_rate_texel_size: crate::vk1_0::Extent2D) -> Self {
        self.0.shading_rate_texel_size = shading_rate_texel_size as _;
        self
    }
    #[inline]
    pub fn shading_rate_palette_size(mut self, shading_rate_palette_size: u32) -> Self {
        self.0.shading_rate_palette_size = shading_rate_palette_size as _;
        self
    }
    #[inline]
    pub fn shading_rate_max_coarse_samples(mut self, shading_rate_max_coarse_samples: u32) -> Self {
        self.0.shading_rate_max_coarse_samples = shading_rate_max_coarse_samples as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceShadingRateImagePropertiesNV {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceShadingRateImagePropertiesNVBuilder<'a> {
    fn default() -> PhysicalDeviceShadingRateImagePropertiesNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceShadingRateImagePropertiesNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceShadingRateImagePropertiesNVBuilder<'a> {
    type Target = PhysicalDeviceShadingRateImagePropertiesNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceShadingRateImagePropertiesNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCoarseSampleLocationNV.html) · Structure"]
#[doc(alias = "VkCoarseSampleLocationNV")]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct CoarseSampleLocationNV {
    pub pixel_x: u32,
    pub pixel_y: u32,
    pub sample: u32,
}
impl Default for CoarseSampleLocationNV {
    fn default() -> Self {
        Self { pixel_x: Default::default(), pixel_y: Default::default(), sample: Default::default() }
    }
}
impl std::fmt::Debug for CoarseSampleLocationNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CoarseSampleLocationNV").field("pixel_x", &self.pixel_x).field("pixel_y", &self.pixel_y).field("sample", &self.sample).finish()
    }
}
impl CoarseSampleLocationNV {
    #[inline]
    pub fn into_builder<'a>(self) -> CoarseSampleLocationNVBuilder<'a> {
        CoarseSampleLocationNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCoarseSampleLocationNV.html) · Builder of [`CoarseSampleLocationNV`]"]
#[repr(transparent)]
pub struct CoarseSampleLocationNVBuilder<'a>(CoarseSampleLocationNV, std::marker::PhantomData<&'a ()>);
impl<'a> CoarseSampleLocationNVBuilder<'a> {
    #[inline]
    pub fn new() -> CoarseSampleLocationNVBuilder<'a> {
        CoarseSampleLocationNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn pixel_x(mut self, pixel_x: u32) -> Self {
        self.0.pixel_x = pixel_x as _;
        self
    }
    #[inline]
    pub fn pixel_y(mut self, pixel_y: u32) -> Self {
        self.0.pixel_y = pixel_y as _;
        self
    }
    #[inline]
    pub fn sample(mut self, sample: u32) -> Self {
        self.0.sample = sample as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> CoarseSampleLocationNV {
        self.0
    }
}
impl<'a> std::default::Default for CoarseSampleLocationNVBuilder<'a> {
    fn default() -> CoarseSampleLocationNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for CoarseSampleLocationNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for CoarseSampleLocationNVBuilder<'a> {
    type Target = CoarseSampleLocationNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for CoarseSampleLocationNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCoarseSampleOrderCustomNV.html) · Structure"]
#[doc(alias = "VkCoarseSampleOrderCustomNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CoarseSampleOrderCustomNV {
    pub shading_rate: crate::extensions::nv_shading_rate_image::ShadingRatePaletteEntryNV,
    pub sample_count: u32,
    pub sample_location_count: u32,
    pub p_sample_locations: *const crate::extensions::nv_shading_rate_image::CoarseSampleLocationNV,
}
impl Default for CoarseSampleOrderCustomNV {
    fn default() -> Self {
        Self { shading_rate: Default::default(), sample_count: Default::default(), sample_location_count: Default::default(), p_sample_locations: std::ptr::null() }
    }
}
impl std::fmt::Debug for CoarseSampleOrderCustomNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CoarseSampleOrderCustomNV").field("shading_rate", &self.shading_rate).field("sample_count", &self.sample_count).field("sample_location_count", &self.sample_location_count).field("p_sample_locations", &self.p_sample_locations).finish()
    }
}
impl CoarseSampleOrderCustomNV {
    #[inline]
    pub fn into_builder<'a>(self) -> CoarseSampleOrderCustomNVBuilder<'a> {
        CoarseSampleOrderCustomNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCoarseSampleOrderCustomNV.html) · Builder of [`CoarseSampleOrderCustomNV`]"]
#[repr(transparent)]
pub struct CoarseSampleOrderCustomNVBuilder<'a>(CoarseSampleOrderCustomNV, std::marker::PhantomData<&'a ()>);
impl<'a> CoarseSampleOrderCustomNVBuilder<'a> {
    #[inline]
    pub fn new() -> CoarseSampleOrderCustomNVBuilder<'a> {
        CoarseSampleOrderCustomNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn shading_rate(mut self, shading_rate: crate::extensions::nv_shading_rate_image::ShadingRatePaletteEntryNV) -> Self {
        self.0.shading_rate = shading_rate as _;
        self
    }
    #[inline]
    pub fn sample_count(mut self, sample_count: u32) -> Self {
        self.0.sample_count = sample_count as _;
        self
    }
    #[inline]
    pub fn sample_locations(mut self, sample_locations: &'a [crate::extensions::nv_shading_rate_image::CoarseSampleLocationNVBuilder]) -> Self {
        self.0.p_sample_locations = sample_locations.as_ptr() as _;
        self.0.sample_location_count = sample_locations.len() as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> CoarseSampleOrderCustomNV {
        self.0
    }
}
impl<'a> std::default::Default for CoarseSampleOrderCustomNVBuilder<'a> {
    fn default() -> CoarseSampleOrderCustomNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for CoarseSampleOrderCustomNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for CoarseSampleOrderCustomNVBuilder<'a> {
    type Target = CoarseSampleOrderCustomNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for CoarseSampleOrderCustomNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineViewportCoarseSampleOrderStateCreateInfoNV.html) · Structure"]
#[doc(alias = "VkPipelineViewportCoarseSampleOrderStateCreateInfoNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineViewportCoarseSampleOrderStateCreateInfoNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub sample_order_type: crate::extensions::nv_shading_rate_image::CoarseSampleOrderTypeNV,
    pub custom_sample_order_count: u32,
    pub p_custom_sample_orders: *const crate::extensions::nv_shading_rate_image::CoarseSampleOrderCustomNV,
}
impl PipelineViewportCoarseSampleOrderStateCreateInfoNV {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PIPELINE_VIEWPORT_COARSE_SAMPLE_ORDER_STATE_CREATE_INFO_NV;
}
impl Default for PipelineViewportCoarseSampleOrderStateCreateInfoNV {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), sample_order_type: Default::default(), custom_sample_order_count: Default::default(), p_custom_sample_orders: std::ptr::null() }
    }
}
impl std::fmt::Debug for PipelineViewportCoarseSampleOrderStateCreateInfoNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PipelineViewportCoarseSampleOrderStateCreateInfoNV").field("s_type", &self.s_type).field("p_next", &self.p_next).field("sample_order_type", &self.sample_order_type).field("custom_sample_order_count", &self.custom_sample_order_count).field("p_custom_sample_orders", &self.p_custom_sample_orders).finish()
    }
}
impl PipelineViewportCoarseSampleOrderStateCreateInfoNV {
    #[inline]
    pub fn into_builder<'a>(self) -> PipelineViewportCoarseSampleOrderStateCreateInfoNVBuilder<'a> {
        PipelineViewportCoarseSampleOrderStateCreateInfoNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineViewportCoarseSampleOrderStateCreateInfoNV.html) · Builder of [`PipelineViewportCoarseSampleOrderStateCreateInfoNV`]"]
#[repr(transparent)]
pub struct PipelineViewportCoarseSampleOrderStateCreateInfoNVBuilder<'a>(PipelineViewportCoarseSampleOrderStateCreateInfoNV, std::marker::PhantomData<&'a ()>);
impl<'a> PipelineViewportCoarseSampleOrderStateCreateInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineViewportCoarseSampleOrderStateCreateInfoNVBuilder<'a> {
        PipelineViewportCoarseSampleOrderStateCreateInfoNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn sample_order_type(mut self, sample_order_type: crate::extensions::nv_shading_rate_image::CoarseSampleOrderTypeNV) -> Self {
        self.0.sample_order_type = sample_order_type as _;
        self
    }
    #[inline]
    pub fn custom_sample_orders(mut self, custom_sample_orders: &'a [crate::extensions::nv_shading_rate_image::CoarseSampleOrderCustomNVBuilder]) -> Self {
        self.0.p_custom_sample_orders = custom_sample_orders.as_ptr() as _;
        self.0.custom_sample_order_count = custom_sample_orders.len() as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PipelineViewportCoarseSampleOrderStateCreateInfoNV {
        self.0
    }
}
impl<'a> std::default::Default for PipelineViewportCoarseSampleOrderStateCreateInfoNVBuilder<'a> {
    fn default() -> PipelineViewportCoarseSampleOrderStateCreateInfoNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PipelineViewportCoarseSampleOrderStateCreateInfoNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PipelineViewportCoarseSampleOrderStateCreateInfoNVBuilder<'a> {
    type Target = PipelineViewportCoarseSampleOrderStateCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineViewportCoarseSampleOrderStateCreateInfoNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::nv_shading_rate_image`]"]
impl crate::DeviceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBindShadingRateImageNV.html) · Function"]
    #[doc(alias = "vkCmdBindShadingRateImageNV")]
    pub unsafe fn cmd_bind_shading_rate_image_nv(&self, command_buffer: crate::vk1_0::CommandBuffer, image_view: Option<crate::vk1_0::ImageView>, image_layout: crate::vk1_0::ImageLayout) -> () {
        let _function = self.cmd_bind_shading_rate_image_nv.expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(
            command_buffer as _,
            match image_view {
                Some(v) => v,
                None => Default::default(),
            },
            image_layout as _,
        );
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetViewportShadingRatePaletteNV.html) · Function"]
    #[doc(alias = "vkCmdSetViewportShadingRatePaletteNV")]
    pub unsafe fn cmd_set_viewport_shading_rate_palette_nv(&self, command_buffer: crate::vk1_0::CommandBuffer, first_viewport: u32, shading_rate_palettes: &[crate::extensions::nv_shading_rate_image::ShadingRatePaletteNVBuilder]) -> () {
        let _function = self.cmd_set_viewport_shading_rate_palette_nv.expect(crate::NOT_LOADED_MESSAGE);
        let viewport_count = shading_rate_palettes.len();
        let _return = _function(command_buffer as _, first_viewport as _, viewport_count as _, shading_rate_palettes.as_ptr() as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetCoarseSampleOrderNV.html) · Function"]
    #[doc(alias = "vkCmdSetCoarseSampleOrderNV")]
    pub unsafe fn cmd_set_coarse_sample_order_nv(&self, command_buffer: crate::vk1_0::CommandBuffer, sample_order_type: crate::extensions::nv_shading_rate_image::CoarseSampleOrderTypeNV, custom_sample_orders: &[crate::extensions::nv_shading_rate_image::CoarseSampleOrderCustomNVBuilder]) -> () {
        let _function = self.cmd_set_coarse_sample_order_nv.expect(crate::NOT_LOADED_MESSAGE);
        let custom_sample_order_count = custom_sample_orders.len();
        let _return = _function(command_buffer as _, sample_order_type as _, custom_sample_order_count as _, custom_sample_orders.as_ptr() as _);
        ()
    }
}
