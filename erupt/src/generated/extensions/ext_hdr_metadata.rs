# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_EXT_hdr_metadata.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_HDR_METADATA_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_HDR_METADATA_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_hdr_metadata");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSetHdrMetadataEXT.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkSetHdrMetadataEXT = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    swapchain_count: u32,
    p_swapchains: *const crate::extensions::khr_swapchain::SwapchainKHR,
    p_metadata: *const crate::extensions::ext_hdr_metadata::HdrMetadataEXT,
) -> std::ffi::c_void;
#[doc = "Provides Device Commands for [`ExtHdrMetadataDeviceLoaderExt`](trait.ExtHdrMetadataDeviceLoaderExt.html)"]
pub struct ExtHdrMetadataDeviceCommands {
    pub set_hdr_metadata_ext: Option<PFN_vkSetHdrMetadataEXT>,
}
impl ExtHdrMetadataDeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<ExtHdrMetadataDeviceCommands> {
        unsafe {
            let mut success = false;
            let table = ExtHdrMetadataDeviceCommands {
                set_hdr_metadata_ext: std::mem::transmute({
                    let symbol = loader.symbol("vkSetHdrMetadataEXT");
                    success |= symbol.is_some();
                    symbol
                }),
            };
            if success {
                Some(table)
            } else {
                None
            }
        }
    }
}
fn device_commands(loader: &crate::DeviceLoader) -> &ExtHdrMetadataDeviceCommands {
    loader
        .ext_hdr_metadata
        .as_ref()
        .expect("`ext_hdr_metadata` not loaded")
}
#[doc = "Provides high level command wrappers for [`ExtHdrMetadataDeviceCommands`](struct.ExtHdrMetadataDeviceCommands.html)"]
pub trait ExtHdrMetadataDeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSetHdrMetadataEXT.html) · Device Command"]
    unsafe fn set_hdr_metadata_ext(
        &self,
        swapchains: &[crate::extensions::khr_swapchain::SwapchainKHR],
        metadata: &[crate::extensions::ext_hdr_metadata::HdrMetadataEXTBuilder],
    ) -> ();
}
impl ExtHdrMetadataDeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSetHdrMetadataEXT.html) · Device Command"]
    unsafe fn set_hdr_metadata_ext(
        &self,
        swapchains: &[crate::extensions::khr_swapchain::SwapchainKHR],
        metadata: &[crate::extensions::ext_hdr_metadata::HdrMetadataEXTBuilder],
    ) -> () {
        let function = device_commands(self)
            .set_hdr_metadata_ext
            .as_ref()
            .expect("`set_hdr_metadata_ext` not available");
        let swapchain_count = swapchains.len().min(metadata.len()) as _;
        let _val = function(
            self.handle,
            swapchain_count,
            swapchains.as_ptr() as _,
            metadata.as_ptr() as _,
        );
        ()
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkHdrMetadataEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HdrMetadataEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub display_primary_red: crate::extensions::ext_hdr_metadata::XYColorEXT,
    pub display_primary_green: crate::extensions::ext_hdr_metadata::XYColorEXT,
    pub display_primary_blue: crate::extensions::ext_hdr_metadata::XYColorEXT,
    pub white_point: crate::extensions::ext_hdr_metadata::XYColorEXT,
    pub max_luminance: f32,
    pub min_luminance: f32,
    pub max_content_light_level: f32,
    pub max_frame_average_light_level: f32,
}
impl HdrMetadataEXT {
    #[inline]
    pub fn builder<'a>(self) -> HdrMetadataEXTBuilder<'a> {
        HdrMetadataEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for HdrMetadataEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("HdrMetadataEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("display_primary_red", &self.display_primary_red)
            .field("display_primary_green", &self.display_primary_green)
            .field("display_primary_blue", &self.display_primary_blue)
            .field("white_point", &self.white_point)
            .field("max_luminance", &self.max_luminance)
            .field("min_luminance", &self.min_luminance)
            .field("max_content_light_level", &self.max_content_light_level)
            .field(
                "max_frame_average_light_level",
                &self.max_frame_average_light_level,
            )
            .finish()
    }
}
impl Default for HdrMetadataEXT {
    fn default() -> HdrMetadataEXT {
        HdrMetadataEXT {
            s_type: crate::vk1_0::StructureType::HDR_METADATA_EXT,
            p_next: std::ptr::null(),
            display_primary_red: Default::default(),
            display_primary_green: Default::default(),
            display_primary_blue: Default::default(),
            white_point: Default::default(),
            max_luminance: Default::default(),
            min_luminance: Default::default(),
            max_content_light_level: Default::default(),
            max_frame_average_light_level: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkHdrMetadataEXT.html) · Builder of [`HdrMetadataEXT`](struct.HdrMetadataEXT.html)"]
#[repr(transparent)]
pub struct HdrMetadataEXTBuilder<'a>(HdrMetadataEXT, std::marker::PhantomData<&'a ()>);
impl<'a> HdrMetadataEXTBuilder<'a> {
    #[inline]
    pub fn new() -> HdrMetadataEXTBuilder<'a> {
        HdrMetadataEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn display_primary_red(
        mut self,
        display_primary_red: crate::extensions::ext_hdr_metadata::XYColorEXT,
    ) -> Self {
        self.0.display_primary_red = display_primary_red;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn display_primary_green(
        mut self,
        display_primary_green: crate::extensions::ext_hdr_metadata::XYColorEXT,
    ) -> Self {
        self.0.display_primary_green = display_primary_green;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn display_primary_blue(
        mut self,
        display_primary_blue: crate::extensions::ext_hdr_metadata::XYColorEXT,
    ) -> Self {
        self.0.display_primary_blue = display_primary_blue;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn white_point(
        mut self,
        white_point: crate::extensions::ext_hdr_metadata::XYColorEXT,
    ) -> Self {
        self.0.white_point = white_point;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_luminance(mut self, max_luminance: f32) -> Self {
        self.0.max_luminance = max_luminance;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn min_luminance(mut self, min_luminance: f32) -> Self {
        self.0.min_luminance = min_luminance;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_content_light_level(mut self, max_content_light_level: f32) -> Self {
        self.0.max_content_light_level = max_content_light_level;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_frame_average_light_level(mut self, max_frame_average_light_level: f32) -> Self {
        self.0.max_frame_average_light_level = max_frame_average_light_level;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> HdrMetadataEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for HdrMetadataEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for HdrMetadataEXTBuilder<'a> {
    type Target = HdrMetadataEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for HdrMetadataEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkXYColorEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XYColorEXT {
    pub x: f32,
    pub y: f32,
}
impl XYColorEXT {
    #[inline]
    pub fn builder<'a>(self) -> XYColorEXTBuilder<'a> {
        XYColorEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for XYColorEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("XYColorEXT")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}
impl Default for XYColorEXT {
    fn default() -> XYColorEXT {
        XYColorEXT {
            x: Default::default(),
            y: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkXYColorEXT.html) · Builder of [`XYColorEXT`](struct.XYColorEXT.html)"]
#[repr(transparent)]
pub struct XYColorEXTBuilder<'a>(XYColorEXT, std::marker::PhantomData<&'a ()>);
impl<'a> XYColorEXTBuilder<'a> {
    #[inline]
    pub fn new() -> XYColorEXTBuilder<'a> {
        XYColorEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn x(mut self, x: f32) -> Self {
        self.0.x = x;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn y(mut self, y: f32) -> Self {
        self.0.y = y;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> XYColorEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for XYColorEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for XYColorEXTBuilder<'a> {
    type Target = XYColorEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for XYColorEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
