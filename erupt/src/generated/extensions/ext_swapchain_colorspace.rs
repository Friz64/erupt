#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_SWAPCHAIN_COLOR_SPACE_SPEC_VERSION")]
pub const EXT_SWAPCHAIN_COLOR_SPACE_SPEC_VERSION: u32 = 4;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_SWAPCHAIN_COLOR_SPACE_EXTENSION_NAME")]
pub const EXT_SWAPCHAIN_COLOR_SPACE_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_swapchain_colorspace");
#[doc = "Provided by [`crate::extensions::ext_swapchain_colorspace`]"]
impl crate::extensions::khr_surface::ColorSpaceKHR {
    pub const DISPLAY_P3_NONLINEAR_EXT: Self = Self(1000104001);
    pub const EXTENDED_SRGB_LINEAR_EXT: Self = Self(1000104002);
    pub const DISPLAY_P3_LINEAR_EXT: Self = Self(1000104003);
    pub const DCI_P3_NONLINEAR_EXT: Self = Self(1000104004);
    pub const BT709_LINEAR_EXT: Self = Self(1000104005);
    pub const BT709_NONLINEAR_EXT: Self = Self(1000104006);
    pub const BT2020_LINEAR_EXT: Self = Self(1000104007);
    pub const HDR10_ST2084_EXT: Self = Self(1000104008);
    pub const DOLBYVISION_EXT: Self = Self(1000104009);
    pub const HDR10_HLG_EXT: Self = Self(1000104010);
    pub const ADOBERGB_LINEAR_EXT: Self = Self(1000104011);
    pub const ADOBERGB_NONLINEAR_EXT: Self = Self(1000104012);
    pub const PASS_THROUGH_EXT: Self = Self(1000104013);
    pub const EXTENDED_SRGB_NONLINEAR_EXT: Self = Self(1000104014);
    #[deprecated]
    pub const DCI_P3_LINEAR_EXT: Self = Self::DISPLAY_P3_LINEAR_EXT;
}
