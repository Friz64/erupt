# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_NV_shading_rate_image.html)\n\n## Extends\n- [`AccessFlagBits`](../../vk1_0/struct.AccessFlagBits.html)\n- [`DynamicState`](../../vk1_0/struct.DynamicState.html)\n- [`ImageLayout`](../../vk1_0/struct.ImageLayout.html)\n- [`ImageUsageFlagBits`](../../vk1_0/struct.ImageUsageFlagBits.html)\n- [`PipelineStageFlagBits`](../../vk1_0/struct.PipelineStageFlagBits.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NV_SHADING_RATE_IMAGE_SPEC_VERSION: u32 = 3;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NV_SHADING_RATE_IMAGE_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_NV_shading_rate_image");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBindShadingRateImageNV.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindShadingRateImageNV = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    image_view: crate::vk1_0::ImageView,
    image_layout: crate::vk1_0::ImageLayout,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetViewportShadingRatePaletteNV.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetViewportShadingRatePaletteNV = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    first_viewport: u32,
    viewport_count: u32,
    p_shading_rate_palettes: *const crate::extensions::nv_shading_rate_image::ShadingRatePaletteNV,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetCoarseSampleOrderNV.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetCoarseSampleOrderNV = unsafe extern "system" fn ( command_buffer : crate :: vk1_0 :: CommandBuffer , sample_order_type : crate :: extensions :: nv_shading_rate_image :: CoarseSampleOrderTypeNV , custom_sample_order_count : u32 , p_custom_sample_orders : * const crate :: extensions :: nv_shading_rate_image :: CoarseSampleOrderCustomNV , ) -> std :: ffi :: c_void ;
#[doc = "Provides Device Commands for [`NvShadingRateImageDeviceLoaderExt`](trait.NvShadingRateImageDeviceLoaderExt.html)"]
pub struct NvShadingRateImageDeviceCommands {
    pub cmd_bind_shading_rate_image_nv: PFN_vkCmdBindShadingRateImageNV,
    pub cmd_set_viewport_shading_rate_palette_nv: PFN_vkCmdSetViewportShadingRatePaletteNV,
    pub cmd_set_coarse_sample_order_nv: PFN_vkCmdSetCoarseSampleOrderNV,
}
impl NvShadingRateImageDeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<NvShadingRateImageDeviceCommands> {
        unsafe {
            Some(NvShadingRateImageDeviceCommands {
                cmd_bind_shading_rate_image_nv: std::mem::transmute(
                    loader.symbol("vkCmdBindShadingRateImageNV")?,
                ),
                cmd_set_viewport_shading_rate_palette_nv: std::mem::transmute(
                    loader.symbol("vkCmdSetViewportShadingRatePaletteNV")?,
                ),
                cmd_set_coarse_sample_order_nv: std::mem::transmute(
                    loader.symbol("vkCmdSetCoarseSampleOrderNV")?,
                ),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`NvShadingRateImageDeviceCommands`](struct.NvShadingRateImageDeviceCommands.html)"]
pub trait NvShadingRateImageDeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBindShadingRateImageNV.html) · Device Command"]
    unsafe fn cmd_bind_shading_rate_image_nv(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        image_view: crate::vk1_0::ImageView,
        image_layout: crate::vk1_0::ImageLayout,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetViewportShadingRatePaletteNV.html) · Device Command"]
    unsafe fn cmd_set_viewport_shading_rate_palette_nv(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        first_viewport: u32,
        shading_rate_palettes : & [ crate :: extensions :: nv_shading_rate_image :: ShadingRatePaletteNVBuilder ],
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetCoarseSampleOrderNV.html) · Device Command"]
    unsafe fn cmd_set_coarse_sample_order_nv(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        sample_order_type: crate::extensions::nv_shading_rate_image::CoarseSampleOrderTypeNV,
        custom_sample_orders : & [ crate :: extensions :: nv_shading_rate_image :: CoarseSampleOrderCustomNVBuilder ],
    ) -> ();
}
impl NvShadingRateImageDeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBindShadingRateImageNV.html) · Device Command"]
    unsafe fn cmd_bind_shading_rate_image_nv(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        image_view: crate::vk1_0::ImageView,
        image_layout: crate::vk1_0::ImageLayout,
    ) -> () {
        let function = self
            .nv_shading_rate_image
            .as_ref()
            .expect("`nv_shading_rate_image` not loaded")
            .cmd_bind_shading_rate_image_nv;
        let _val = function(command_buffer, image_view, image_layout);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetViewportShadingRatePaletteNV.html) · Device Command"]
    unsafe fn cmd_set_viewport_shading_rate_palette_nv(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        first_viewport: u32,
        shading_rate_palettes : & [ crate :: extensions :: nv_shading_rate_image :: ShadingRatePaletteNVBuilder ],
    ) -> () {
        let function = self
            .nv_shading_rate_image
            .as_ref()
            .expect("`nv_shading_rate_image` not loaded")
            .cmd_set_viewport_shading_rate_palette_nv;
        let viewport_count = shading_rate_palettes.len() as _;
        let _val = function(
            command_buffer,
            first_viewport,
            viewport_count,
            shading_rate_palettes.as_ptr() as _,
        );
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetCoarseSampleOrderNV.html) · Device Command"]
    unsafe fn cmd_set_coarse_sample_order_nv(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        sample_order_type: crate::extensions::nv_shading_rate_image::CoarseSampleOrderTypeNV,
        custom_sample_orders : & [ crate :: extensions :: nv_shading_rate_image :: CoarseSampleOrderCustomNVBuilder ],
    ) -> () {
        let function = self
            .nv_shading_rate_image
            .as_ref()
            .expect("`nv_shading_rate_image` not loaded")
            .cmd_set_coarse_sample_order_nv;
        let custom_sample_order_count = custom_sample_orders.len() as _;
        let _val = function(
            command_buffer,
            sample_order_type,
            custom_sample_order_count,
            custom_sample_orders.as_ptr() as _,
        );
        ()
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkShadingRatePaletteNV.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ShadingRatePaletteNV {
    pub shading_rate_palette_entry_count: u32,
    pub p_shading_rate_palette_entries:
        *const crate::extensions::nv_shading_rate_image::ShadingRatePaletteEntryNV,
}
impl ShadingRatePaletteNV {
    #[inline]
    pub fn builder<'a>(self) -> ShadingRatePaletteNVBuilder<'a> {
        ShadingRatePaletteNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ShadingRatePaletteNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ShadingRatePaletteNV")
            .field(
                "shading_rate_palette_entry_count",
                &self.shading_rate_palette_entry_count,
            )
            .field(
                "p_shading_rate_palette_entries",
                &self.p_shading_rate_palette_entries,
            )
            .finish()
    }
}
impl Default for ShadingRatePaletteNV {
    fn default() -> ShadingRatePaletteNV {
        ShadingRatePaletteNV {
            shading_rate_palette_entry_count: Default::default(),
            p_shading_rate_palette_entries: std::ptr::null(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`ShadingRatePaletteNV`](struct.ShadingRatePaletteNV.html)"]
#[repr(transparent)]
pub struct ShadingRatePaletteNVBuilder<'a>(ShadingRatePaletteNV, std::marker::PhantomData<&'a ()>);
impl<'a> ShadingRatePaletteNVBuilder<'a> {
    #[inline]
    pub fn new() -> ShadingRatePaletteNVBuilder<'a> {
        ShadingRatePaletteNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shading_rate_palette_entries(
        mut self,
        shading_rate_palette_entries : &'a [ crate :: extensions :: nv_shading_rate_image :: ShadingRatePaletteEntryNV ],
    ) -> Self {
        self.0.shading_rate_palette_entry_count = shading_rate_palette_entries.len() as _;
        self.0.p_shading_rate_palette_entries = shading_rate_palette_entries.as_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> ShadingRatePaletteNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for ShadingRatePaletteNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkShadingRatePaletteEntryNV.html) · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ShadingRatePaletteEntryNV(pub i32);
#[doc = "[Part of `extensions::nv_shading_rate_image`](../../extensions/nv_shading_rate_image/index.html)"]
impl ShadingRatePaletteEntryNV {
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
impl std::fmt::Debug for ShadingRatePaletteEntryNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::NO_INVOCATIONS_NV => "NO_INVOCATIONS_NV",
            &Self::_16_INVOCATIONS_PER_PIXEL_NV => "16_INVOCATIONS_PER_PIXEL_NV",
            &Self::_8_INVOCATIONS_PER_PIXEL_NV => "8_INVOCATIONS_PER_PIXEL_NV",
            &Self::_4_INVOCATIONS_PER_PIXEL_NV => "4_INVOCATIONS_PER_PIXEL_NV",
            &Self::_2_INVOCATIONS_PER_PIXEL_NV => "2_INVOCATIONS_PER_PIXEL_NV",
            &Self::_1_INVOCATION_PER_PIXEL_NV => "1_INVOCATION_PER_PIXEL_NV",
            &Self::_1_INVOCATION_PER_2X1_PIXELS_NV => "1_INVOCATION_PER_2X1_PIXELS_NV",
            &Self::_1_INVOCATION_PER_1X2_PIXELS_NV => "1_INVOCATION_PER_1X2_PIXELS_NV",
            &Self::_1_INVOCATION_PER_2X2_PIXELS_NV => "1_INVOCATION_PER_2X2_PIXELS_NV",
            &Self::_1_INVOCATION_PER_4X2_PIXELS_NV => "1_INVOCATION_PER_4X2_PIXELS_NV",
            &Self::_1_INVOCATION_PER_2X4_PIXELS_NV => "1_INVOCATION_PER_2X4_PIXELS_NV",
            &Self::_1_INVOCATION_PER_4X4_PIXELS_NV => "1_INVOCATION_PER_4X4_PIXELS_NV",
            _ => "(unknown)",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCoarseSampleOrderTypeNV.html) · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct CoarseSampleOrderTypeNV(pub i32);
#[doc = "[Part of `extensions::nv_shading_rate_image`](../../extensions/nv_shading_rate_image/index.html)"]
impl CoarseSampleOrderTypeNV {
    pub const DEFAULT_NV: Self = Self(0);
    pub const CUSTOM_NV: Self = Self(1);
    pub const PIXEL_MAJOR_NV: Self = Self(2);
    pub const SAMPLE_MAJOR_NV: Self = Self(3);
}
impl std::fmt::Debug for CoarseSampleOrderTypeNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::DEFAULT_NV => "DEFAULT_NV",
            &Self::CUSTOM_NV => "CUSTOM_NV",
            &Self::PIXEL_MAJOR_NV => "PIXEL_MAJOR_NV",
            &Self::SAMPLE_MAJOR_NV => "SAMPLE_MAJOR_NV",
            _ => "(unknown)",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCoarseSampleOrderCustomNV.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CoarseSampleOrderCustomNV {
    pub shading_rate: crate::extensions::nv_shading_rate_image::ShadingRatePaletteEntryNV,
    pub sample_count: u32,
    pub sample_location_count: u32,
    pub p_sample_locations: *const crate::extensions::nv_shading_rate_image::CoarseSampleLocationNV,
}
impl CoarseSampleOrderCustomNV {
    #[inline]
    pub fn builder<'a>(self) -> CoarseSampleOrderCustomNVBuilder<'a> {
        CoarseSampleOrderCustomNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for CoarseSampleOrderCustomNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("CoarseSampleOrderCustomNV")
            .field("shading_rate", &self.shading_rate)
            .field("sample_count", &self.sample_count)
            .field("sample_location_count", &self.sample_location_count)
            .field("p_sample_locations", &self.p_sample_locations)
            .finish()
    }
}
impl Default for CoarseSampleOrderCustomNV {
    fn default() -> CoarseSampleOrderCustomNV {
        CoarseSampleOrderCustomNV {
            shading_rate: Default::default(),
            sample_count: Default::default(),
            sample_location_count: Default::default(),
            p_sample_locations: std::ptr::null(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`CoarseSampleOrderCustomNV`](struct.CoarseSampleOrderCustomNV.html)"]
#[repr(transparent)]
pub struct CoarseSampleOrderCustomNVBuilder<'a>(
    CoarseSampleOrderCustomNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> CoarseSampleOrderCustomNVBuilder<'a> {
    #[inline]
    pub fn new() -> CoarseSampleOrderCustomNVBuilder<'a> {
        CoarseSampleOrderCustomNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shading_rate(
        mut self,
        shading_rate: crate::extensions::nv_shading_rate_image::ShadingRatePaletteEntryNV,
    ) -> Self {
        self.0.shading_rate = shading_rate;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn sample_count(mut self, sample_count: u32) -> Self {
        self.0.sample_count = sample_count;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn sample_locations(
        mut self,
        sample_locations : &'a [ crate :: extensions :: nv_shading_rate_image :: CoarseSampleLocationNVBuilder ],
    ) -> Self {
        self.0.sample_location_count = sample_locations.len() as _;
        self.0.p_sample_locations = sample_locations.as_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> CoarseSampleOrderCustomNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for CoarseSampleOrderCustomNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCoarseSampleLocationNV.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CoarseSampleLocationNV {
    pub pixel_x: u32,
    pub pixel_y: u32,
    pub sample: u32,
}
impl CoarseSampleLocationNV {
    #[inline]
    pub fn builder<'a>(self) -> CoarseSampleLocationNVBuilder<'a> {
        CoarseSampleLocationNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for CoarseSampleLocationNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("CoarseSampleLocationNV")
            .field("pixel_x", &self.pixel_x)
            .field("pixel_y", &self.pixel_y)
            .field("sample", &self.sample)
            .finish()
    }
}
impl Default for CoarseSampleLocationNV {
    fn default() -> CoarseSampleLocationNV {
        CoarseSampleLocationNV {
            pixel_x: Default::default(),
            pixel_y: Default::default(),
            sample: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`CoarseSampleLocationNV`](struct.CoarseSampleLocationNV.html)"]
#[repr(transparent)]
pub struct CoarseSampleLocationNVBuilder<'a>(
    CoarseSampleLocationNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> CoarseSampleLocationNVBuilder<'a> {
    #[inline]
    pub fn new() -> CoarseSampleLocationNVBuilder<'a> {
        CoarseSampleLocationNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn pixel_x(mut self, pixel_x: u32) -> Self {
        self.0.pixel_x = pixel_x;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn pixel_y(mut self, pixel_y: u32) -> Self {
        self.0.pixel_y = pixel_y;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn sample(mut self, sample: u32) -> Self {
        self.0.sample = sample;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> CoarseSampleLocationNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for CoarseSampleLocationNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineViewportShadingRateImageStateCreateInfoNV.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineViewportShadingRateImageStateCreateInfoNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub shading_rate_image_enable: crate::vk1_0::Bool32,
    pub viewport_count: u32,
    pub p_shading_rate_palettes:
        *const crate::extensions::nv_shading_rate_image::ShadingRatePaletteNV,
}
impl PipelineViewportShadingRateImageStateCreateInfoNV {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPipelineViewportShadingRateImageStateCreateInfoNV,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PipelineViewportShadingRateImageStateCreateInfoNVBuilder<'a> {
        PipelineViewportShadingRateImageStateCreateInfoNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PipelineViewportShadingRateImageStateCreateInfoNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PipelineViewportShadingRateImageStateCreateInfoNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "shading_rate_image_enable",
                &(self.shading_rate_image_enable != 0),
            )
            .field("viewport_count", &self.viewport_count)
            .field("p_shading_rate_palettes", &self.p_shading_rate_palettes)
            .finish()
    }
}
impl Default for PipelineViewportShadingRateImageStateCreateInfoNV {
    fn default() -> PipelineViewportShadingRateImageStateCreateInfoNV {
        PipelineViewportShadingRateImageStateCreateInfoNV { s_type : crate :: vk1_0 :: StructureType :: PIPELINE_VIEWPORT_SHADING_RATE_IMAGE_STATE_CREATE_INFO_NV , p_next : std :: ptr :: null ( ) , shading_rate_image_enable : Default :: default ( ) , viewport_count : Default :: default ( ) , p_shading_rate_palettes : std :: ptr :: null ( ) , }
    }
}
#[doc = "Used by [`PipelineViewportShadingRateImageStateCreateInfoNV::extend`](struct.PipelineViewportShadingRateImageStateCreateInfoNV.html#method.extend)"]
pub trait ExtendableByPipelineViewportShadingRateImageStateCreateInfoNV {}
impl ExtendableByPipelineViewportShadingRateImageStateCreateInfoNV
    for crate::vk1_0::PipelineViewportStateCreateInfo
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PipelineViewportShadingRateImageStateCreateInfoNV`](struct.PipelineViewportShadingRateImageStateCreateInfoNV.html)"]
#[repr(transparent)]
pub struct PipelineViewportShadingRateImageStateCreateInfoNVBuilder<'a>(
    PipelineViewportShadingRateImageStateCreateInfoNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PipelineViewportShadingRateImageStateCreateInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineViewportShadingRateImageStateCreateInfoNVBuilder<'a> {
        PipelineViewportShadingRateImageStateCreateInfoNVBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shading_rate_image_enable(mut self, shading_rate_image_enable: bool) -> Self {
        self.0.shading_rate_image_enable = shading_rate_image_enable as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shading_rate_palettes(
        mut self,
        shading_rate_palettes : &'a [ crate :: extensions :: nv_shading_rate_image :: ShadingRatePaletteNVBuilder ],
    ) -> Self {
        self.0.viewport_count = shading_rate_palettes.len() as _;
        self.0.p_shading_rate_palettes = shading_rate_palettes.as_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PipelineViewportShadingRateImageStateCreateInfoNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for PipelineViewportShadingRateImageStateCreateInfoNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceShadingRateImageFeaturesNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub shading_rate_image: crate::vk1_0::Bool32,
    pub shading_rate_coarse_sample_order: crate::vk1_0::Bool32,
}
impl PhysicalDeviceShadingRateImageFeaturesNV {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceShadingRateImageFeaturesNV,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceShadingRateImageFeaturesNVBuilder<'a> {
        PhysicalDeviceShadingRateImageFeaturesNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceShadingRateImageFeaturesNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceShadingRateImageFeaturesNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("shading_rate_image", &(self.shading_rate_image != 0))
            .field(
                "shading_rate_coarse_sample_order",
                &(self.shading_rate_coarse_sample_order != 0),
            )
            .finish()
    }
}
impl Default for PhysicalDeviceShadingRateImageFeaturesNV {
    fn default() -> PhysicalDeviceShadingRateImageFeaturesNV {
        PhysicalDeviceShadingRateImageFeaturesNV {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_SHADING_RATE_IMAGE_FEATURES_NV,
            p_next: std::ptr::null_mut(),
            shading_rate_image: Default::default(),
            shading_rate_coarse_sample_order: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceShadingRateImageFeaturesNV::extend`](struct.PhysicalDeviceShadingRateImageFeaturesNV.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceShadingRateImageFeaturesNV {}
impl ExtendableByPhysicalDeviceShadingRateImageFeaturesNV
    for crate::vk1_1::PhysicalDeviceFeatures2
{
}
impl ExtendableByPhysicalDeviceShadingRateImageFeaturesNV for crate::vk1_0::DeviceCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceShadingRateImageFeaturesNV`](struct.PhysicalDeviceShadingRateImageFeaturesNV.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceShadingRateImageFeaturesNVBuilder<'a>(
    PhysicalDeviceShadingRateImageFeaturesNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceShadingRateImageFeaturesNVBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceShadingRateImageFeaturesNVBuilder<'a> {
        PhysicalDeviceShadingRateImageFeaturesNVBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shading_rate_image(mut self, shading_rate_image: bool) -> Self {
        self.0.shading_rate_image = shading_rate_image as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shading_rate_coarse_sample_order(
        mut self,
        shading_rate_coarse_sample_order: bool,
    ) -> Self {
        self.0.shading_rate_coarse_sample_order = shading_rate_coarse_sample_order as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceShadingRateImageFeaturesNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceShadingRateImageFeaturesNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceShadingRateImagePropertiesNV,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceShadingRateImagePropertiesNVBuilder<'a> {
        PhysicalDeviceShadingRateImagePropertiesNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceShadingRateImagePropertiesNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceShadingRateImagePropertiesNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("shading_rate_texel_size", &self.shading_rate_texel_size)
            .field("shading_rate_palette_size", &self.shading_rate_palette_size)
            .field(
                "shading_rate_max_coarse_samples",
                &self.shading_rate_max_coarse_samples,
            )
            .finish()
    }
}
impl Default for PhysicalDeviceShadingRateImagePropertiesNV {
    fn default() -> PhysicalDeviceShadingRateImagePropertiesNV {
        PhysicalDeviceShadingRateImagePropertiesNV {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_SHADING_RATE_IMAGE_PROPERTIES_NV,
            p_next: std::ptr::null_mut(),
            shading_rate_texel_size: Default::default(),
            shading_rate_palette_size: Default::default(),
            shading_rate_max_coarse_samples: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceShadingRateImagePropertiesNV::extend`](struct.PhysicalDeviceShadingRateImagePropertiesNV.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceShadingRateImagePropertiesNV {}
impl ExtendableByPhysicalDeviceShadingRateImagePropertiesNV
    for crate::vk1_1::PhysicalDeviceProperties2
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceShadingRateImagePropertiesNV`](struct.PhysicalDeviceShadingRateImagePropertiesNV.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceShadingRateImagePropertiesNVBuilder<'a>(
    PhysicalDeviceShadingRateImagePropertiesNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceShadingRateImagePropertiesNVBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceShadingRateImagePropertiesNVBuilder<'a> {
        PhysicalDeviceShadingRateImagePropertiesNVBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shading_rate_texel_size(
        mut self,
        shading_rate_texel_size: crate::vk1_0::Extent2D,
    ) -> Self {
        self.0.shading_rate_texel_size = shading_rate_texel_size;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shading_rate_palette_size(mut self, shading_rate_palette_size: u32) -> Self {
        self.0.shading_rate_palette_size = shading_rate_palette_size;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shading_rate_max_coarse_samples(mut self, shading_rate_max_coarse_samples: u32) -> Self {
        self.0.shading_rate_max_coarse_samples = shading_rate_max_coarse_samples;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceShadingRateImagePropertiesNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceShadingRateImagePropertiesNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineViewportCoarseSampleOrderStateCreateInfoNV.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineViewportCoarseSampleOrderStateCreateInfoNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub sample_order_type: crate::extensions::nv_shading_rate_image::CoarseSampleOrderTypeNV,
    pub custom_sample_order_count: u32,
    pub p_custom_sample_orders:
        *const crate::extensions::nv_shading_rate_image::CoarseSampleOrderCustomNV,
}
impl PipelineViewportCoarseSampleOrderStateCreateInfoNV {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPipelineViewportCoarseSampleOrderStateCreateInfoNV,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PipelineViewportCoarseSampleOrderStateCreateInfoNVBuilder<'a> {
        PipelineViewportCoarseSampleOrderStateCreateInfoNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PipelineViewportCoarseSampleOrderStateCreateInfoNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PipelineViewportCoarseSampleOrderStateCreateInfoNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("sample_order_type", &self.sample_order_type)
            .field("custom_sample_order_count", &self.custom_sample_order_count)
            .field("p_custom_sample_orders", &self.p_custom_sample_orders)
            .finish()
    }
}
impl Default for PipelineViewportCoarseSampleOrderStateCreateInfoNV {
    fn default() -> PipelineViewportCoarseSampleOrderStateCreateInfoNV {
        PipelineViewportCoarseSampleOrderStateCreateInfoNV { s_type : crate :: vk1_0 :: StructureType :: PIPELINE_VIEWPORT_COARSE_SAMPLE_ORDER_STATE_CREATE_INFO_NV , p_next : std :: ptr :: null ( ) , sample_order_type : Default :: default ( ) , custom_sample_order_count : Default :: default ( ) , p_custom_sample_orders : std :: ptr :: null ( ) , }
    }
}
#[doc = "Used by [`PipelineViewportCoarseSampleOrderStateCreateInfoNV::extend`](struct.PipelineViewportCoarseSampleOrderStateCreateInfoNV.html#method.extend)"]
pub trait ExtendableByPipelineViewportCoarseSampleOrderStateCreateInfoNV {}
impl ExtendableByPipelineViewportCoarseSampleOrderStateCreateInfoNV
    for crate::vk1_0::PipelineViewportStateCreateInfo
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PipelineViewportCoarseSampleOrderStateCreateInfoNV`](struct.PipelineViewportCoarseSampleOrderStateCreateInfoNV.html)"]
#[repr(transparent)]
pub struct PipelineViewportCoarseSampleOrderStateCreateInfoNVBuilder<'a>(
    PipelineViewportCoarseSampleOrderStateCreateInfoNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PipelineViewportCoarseSampleOrderStateCreateInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineViewportCoarseSampleOrderStateCreateInfoNVBuilder<'a> {
        PipelineViewportCoarseSampleOrderStateCreateInfoNVBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn sample_order_type(
        mut self,
        sample_order_type: crate::extensions::nv_shading_rate_image::CoarseSampleOrderTypeNV,
    ) -> Self {
        self.0.sample_order_type = sample_order_type;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn custom_sample_orders(
        mut self,
        custom_sample_orders : &'a [ crate :: extensions :: nv_shading_rate_image :: CoarseSampleOrderCustomNVBuilder ],
    ) -> Self {
        self.0.custom_sample_order_count = custom_sample_orders.len() as _;
        self.0.p_custom_sample_orders = custom_sample_orders.as_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PipelineViewportCoarseSampleOrderStateCreateInfoNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for PipelineViewportCoarseSampleOrderStateCreateInfoNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
