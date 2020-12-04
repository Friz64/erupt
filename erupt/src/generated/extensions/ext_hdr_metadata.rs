#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_HDR_METADATA_SPEC_VERSION")]
pub const EXT_HDR_METADATA_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_HDR_METADATA_EXTENSION_NAME")]
pub const EXT_HDR_METADATA_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_hdr_metadata");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "FN_SET_HDR_METADATA_EXT")]
pub const FN_SET_HDR_METADATA_EXT: *const std::os::raw::c_char =
    crate::cstr!("vkSetHdrMetadataEXT");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSetHdrMetadataEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkSetHdrMetadataEXT = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    swapchain_count: u32,
    p_swapchains: *const crate::extensions::khr_swapchain::SwapchainKHR,
    p_metadata: *const crate::extensions::ext_hdr_metadata::HdrMetadataEXT,
) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkXYColorEXT.html) · Structure"]
#[doc(alias = "VkXYColorEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XYColorEXT {
    pub x: std::os::raw::c_float,
    pub y: std::os::raw::c_float,
}
impl Default for XYColorEXT {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
        }
    }
}
impl std::fmt::Debug for XYColorEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("XYColorEXT")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}
impl XYColorEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> XYColorEXTBuilder<'a> {
        XYColorEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkXYColorEXT.html) · Builder of [`XYColorEXT`]"]
#[repr(transparent)]
pub struct XYColorEXTBuilder<'a>(XYColorEXT, std::marker::PhantomData<&'a ()>);
impl<'a> XYColorEXTBuilder<'a> {
    #[inline]
    pub fn new() -> XYColorEXTBuilder<'a> {
        XYColorEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn x(mut self, x: std::os::raw::c_float) -> Self {
        self.0.x = x as _;
        self
    }
    #[inline]
    pub fn y(mut self, y: std::os::raw::c_float) -> Self {
        self.0.y = y as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> XYColorEXT {
        self.0
    }
}
impl<'a> std::default::Default for XYColorEXTBuilder<'a> {
    fn default() -> XYColorEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for XYColorEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkHdrMetadataEXT.html) · Structure"]
#[doc(alias = "VkHdrMetadataEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HdrMetadataEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub display_primary_red: crate::extensions::ext_hdr_metadata::XYColorEXT,
    pub display_primary_green: crate::extensions::ext_hdr_metadata::XYColorEXT,
    pub display_primary_blue: crate::extensions::ext_hdr_metadata::XYColorEXT,
    pub white_point: crate::extensions::ext_hdr_metadata::XYColorEXT,
    pub max_luminance: std::os::raw::c_float,
    pub min_luminance: std::os::raw::c_float,
    pub max_content_light_level: std::os::raw::c_float,
    pub max_frame_average_light_level: std::os::raw::c_float,
}
impl Default for HdrMetadataEXT {
    fn default() -> Self {
        Self {
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
impl std::fmt::Debug for HdrMetadataEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("HdrMetadataEXT")
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
impl HdrMetadataEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> HdrMetadataEXTBuilder<'a> {
        HdrMetadataEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkHdrMetadataEXT.html) · Builder of [`HdrMetadataEXT`]"]
#[repr(transparent)]
pub struct HdrMetadataEXTBuilder<'a>(HdrMetadataEXT, std::marker::PhantomData<&'a ()>);
impl<'a> HdrMetadataEXTBuilder<'a> {
    #[inline]
    pub fn new() -> HdrMetadataEXTBuilder<'a> {
        HdrMetadataEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn display_primary_red(
        mut self,
        display_primary_red: crate::extensions::ext_hdr_metadata::XYColorEXT,
    ) -> Self {
        self.0.display_primary_red = display_primary_red as _;
        self
    }
    #[inline]
    pub fn display_primary_green(
        mut self,
        display_primary_green: crate::extensions::ext_hdr_metadata::XYColorEXT,
    ) -> Self {
        self.0.display_primary_green = display_primary_green as _;
        self
    }
    #[inline]
    pub fn display_primary_blue(
        mut self,
        display_primary_blue: crate::extensions::ext_hdr_metadata::XYColorEXT,
    ) -> Self {
        self.0.display_primary_blue = display_primary_blue as _;
        self
    }
    #[inline]
    pub fn white_point(
        mut self,
        white_point: crate::extensions::ext_hdr_metadata::XYColorEXT,
    ) -> Self {
        self.0.white_point = white_point as _;
        self
    }
    #[inline]
    pub fn max_luminance(mut self, max_luminance: std::os::raw::c_float) -> Self {
        self.0.max_luminance = max_luminance as _;
        self
    }
    #[inline]
    pub fn min_luminance(mut self, min_luminance: std::os::raw::c_float) -> Self {
        self.0.min_luminance = min_luminance as _;
        self
    }
    #[inline]
    pub fn max_content_light_level(
        mut self,
        max_content_light_level: std::os::raw::c_float,
    ) -> Self {
        self.0.max_content_light_level = max_content_light_level as _;
        self
    }
    #[inline]
    pub fn max_frame_average_light_level(
        mut self,
        max_frame_average_light_level: std::os::raw::c_float,
    ) -> Self {
        self.0.max_frame_average_light_level = max_frame_average_light_level as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> HdrMetadataEXT {
        self.0
    }
}
impl<'a> std::default::Default for HdrMetadataEXTBuilder<'a> {
    fn default() -> HdrMetadataEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for HdrMetadataEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
#[doc = "Provided by [`crate::extensions::ext_hdr_metadata`]"]
impl crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSetHdrMetadataEXT.html) · Function"]
    #[doc(alias = "vkSetHdrMetadataEXT")]
    pub unsafe fn set_hdr_metadata_ext(
        &self,
        swapchains: &[crate::extensions::khr_swapchain::SwapchainKHR],
        metadata: &[crate::extensions::ext_hdr_metadata::HdrMetadataEXTBuilder],
    ) -> () {
        let _function = self
            .set_hdr_metadata_ext
            .expect("`set_hdr_metadata_ext` is not loaded");
        let swapchain_count = swapchains.len().min(metadata.len());
        let _return = _function(
            self.handle,
            swapchain_count as _,
            swapchains.as_ptr() as _,
            metadata.as_ptr() as _,
        );
        ()
    }
}
