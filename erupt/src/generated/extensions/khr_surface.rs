#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_SURFACE_SPEC_VERSION")]
pub const KHR_SURFACE_SPEC_VERSION: u32 = 25;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_SURFACE_EXTENSION_NAME")]
pub const KHR_SURFACE_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_surface");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_DESTROY_SURFACE_KHR: *const std::os::raw::c_char = crate::cstr!("vkDestroySurfaceKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_PHYSICAL_DEVICE_SURFACE_SUPPORT_KHR: *const std::os::raw::c_char = crate::cstr!("vkGetPhysicalDeviceSurfaceSupportKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_PHYSICAL_DEVICE_SURFACE_CAPABILITIES_KHR: *const std::os::raw::c_char = crate::cstr!("vkGetPhysicalDeviceSurfaceCapabilitiesKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_PHYSICAL_DEVICE_SURFACE_FORMATS_KHR: *const std::os::raw::c_char = crate::cstr!("vkGetPhysicalDeviceSurfaceFormatsKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_PHYSICAL_DEVICE_SURFACE_PRESENT_MODES_KHR: *const std::os::raw::c_char = crate::cstr!("vkGetPhysicalDeviceSurfacePresentModesKHR");
crate::non_dispatchable_handle!(
    SurfaceKHR,
    SURFACE_KHR,
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceKHR.html) · Non-dispatchable Handle",
    "VkSurfaceKHR"
);
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkColorSpaceKHR.html) · Enum"]
#[doc(alias = "VkColorSpaceKHR")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct ColorSpaceKHR(pub i32);
impl std::fmt::Debug for ColorSpaceKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::SRGB_NONLINEAR_KHR => "SRGB_NONLINEAR_KHR",
            &Self::DISPLAY_P3_NONLINEAR_EXT => "DISPLAY_P3_NONLINEAR_EXT",
            &Self::EXTENDED_SRGB_LINEAR_EXT => "EXTENDED_SRGB_LINEAR_EXT",
            &Self::DISPLAY_P3_LINEAR_EXT => "DISPLAY_P3_LINEAR_EXT",
            &Self::DCI_P3_NONLINEAR_EXT => "DCI_P3_NONLINEAR_EXT",
            &Self::BT709_LINEAR_EXT => "BT709_LINEAR_EXT",
            &Self::BT709_NONLINEAR_EXT => "BT709_NONLINEAR_EXT",
            &Self::BT2020_LINEAR_EXT => "BT2020_LINEAR_EXT",
            &Self::HDR10_ST2084_EXT => "HDR10_ST2084_EXT",
            &Self::DOLBYVISION_EXT => "DOLBYVISION_EXT",
            &Self::HDR10_HLG_EXT => "HDR10_HLG_EXT",
            &Self::ADOBERGB_LINEAR_EXT => "ADOBERGB_LINEAR_EXT",
            &Self::ADOBERGB_NONLINEAR_EXT => "ADOBERGB_NONLINEAR_EXT",
            &Self::PASS_THROUGH_EXT => "PASS_THROUGH_EXT",
            &Self::EXTENDED_SRGB_NONLINEAR_EXT => "EXTENDED_SRGB_NONLINEAR_EXT",
            &Self::DISPLAY_NATIVE_AMD => "DISPLAY_NATIVE_AMD",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::khr_surface`]"]
impl ColorSpaceKHR {
    pub const SRGB_NONLINEAR_KHR: Self = Self(0);
    pub const COLORSPACE_SRGB_NONLINEAR_KHR: Self = Self::SRGB_NONLINEAR_KHR;
}
#[doc = "Provided by [`crate::extensions::ext_swapchain_colorspace`]"]
impl ColorSpaceKHR {
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
    pub const DCI_P3_LINEAR_EXT: Self = Self::DISPLAY_P3_LINEAR_EXT;
}
#[doc = "Provided by [`crate::extensions::amd_display_native_hdr`]"]
impl ColorSpaceKHR {
    pub const DISPLAY_NATIVE_AMD: Self = Self(1000213000);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCompositeAlphaFlagsKHR.html) · Bitmask of [`CompositeAlphaFlagBitsKHR`]"] # [doc (alias = "VkCompositeAlphaFlagsKHR")] # [derive (Default)] # [repr (transparent)] pub struct CompositeAlphaFlagsKHR : u32 { const OPAQUE_KHR = CompositeAlphaFlagBitsKHR :: OPAQUE_KHR . 0 ; const PRE_MULTIPLIED_KHR = CompositeAlphaFlagBitsKHR :: PRE_MULTIPLIED_KHR . 0 ; const POST_MULTIPLIED_KHR = CompositeAlphaFlagBitsKHR :: POST_MULTIPLIED_KHR . 0 ; const INHERIT_KHR = CompositeAlphaFlagBitsKHR :: INHERIT_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCompositeAlphaFlagBitsKHR.html) · Bits enum of [`CompositeAlphaFlagsKHR`]"]
#[doc(alias = "VkCompositeAlphaFlagBitsKHR")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct CompositeAlphaFlagBitsKHR(pub u32);
impl CompositeAlphaFlagBitsKHR {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> CompositeAlphaFlagsKHR {
        CompositeAlphaFlagsKHR::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for CompositeAlphaFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::OPAQUE_KHR => "OPAQUE_KHR",
            &Self::PRE_MULTIPLIED_KHR => "PRE_MULTIPLIED_KHR",
            &Self::POST_MULTIPLIED_KHR => "POST_MULTIPLIED_KHR",
            &Self::INHERIT_KHR => "INHERIT_KHR",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::khr_surface`]"]
impl CompositeAlphaFlagBitsKHR {
    pub const OPAQUE_KHR: Self = Self(1);
    pub const PRE_MULTIPLIED_KHR: Self = Self(2);
    pub const POST_MULTIPLIED_KHR: Self = Self(4);
    pub const INHERIT_KHR: Self = Self(8);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPresentModeKHR.html) · Enum"]
#[doc(alias = "VkPresentModeKHR")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct PresentModeKHR(pub i32);
impl std::fmt::Debug for PresentModeKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::IMMEDIATE_KHR => "IMMEDIATE_KHR",
            &Self::MAILBOX_KHR => "MAILBOX_KHR",
            &Self::FIFO_KHR => "FIFO_KHR",
            &Self::FIFO_RELAXED_KHR => "FIFO_RELAXED_KHR",
            &Self::SHARED_DEMAND_REFRESH_KHR => "SHARED_DEMAND_REFRESH_KHR",
            &Self::SHARED_CONTINUOUS_REFRESH_KHR => "SHARED_CONTINUOUS_REFRESH_KHR",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::khr_surface`]"]
impl PresentModeKHR {
    pub const IMMEDIATE_KHR: Self = Self(0);
    pub const MAILBOX_KHR: Self = Self(1);
    pub const FIFO_KHR: Self = Self(2);
    pub const FIFO_RELAXED_KHR: Self = Self(3);
}
#[doc = "Provided by [`crate::extensions::khr_shared_presentable_image`]"]
impl PresentModeKHR {
    pub const SHARED_DEMAND_REFRESH_KHR: Self = Self(1000111000);
    pub const SHARED_CONTINUOUS_REFRESH_KHR: Self = Self(1000111001);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceTransformFlagsKHR.html) · Bitmask of [`SurfaceTransformFlagBitsKHR`]"] # [doc (alias = "VkSurfaceTransformFlagsKHR")] # [derive (Default)] # [repr (transparent)] pub struct SurfaceTransformFlagsKHR : u32 { const IDENTITY_KHR = SurfaceTransformFlagBitsKHR :: IDENTITY_KHR . 0 ; const ROTATE_90_KHR = SurfaceTransformFlagBitsKHR :: ROTATE_90_KHR . 0 ; const ROTATE_180_KHR = SurfaceTransformFlagBitsKHR :: ROTATE_180_KHR . 0 ; const ROTATE_270_KHR = SurfaceTransformFlagBitsKHR :: ROTATE_270_KHR . 0 ; const HORIZONTAL_MIRROR_KHR = SurfaceTransformFlagBitsKHR :: HORIZONTAL_MIRROR_KHR . 0 ; const HORIZONTAL_MIRROR_ROTATE_90_KHR = SurfaceTransformFlagBitsKHR :: HORIZONTAL_MIRROR_ROTATE_90_KHR . 0 ; const HORIZONTAL_MIRROR_ROTATE_180_KHR = SurfaceTransformFlagBitsKHR :: HORIZONTAL_MIRROR_ROTATE_180_KHR . 0 ; const HORIZONTAL_MIRROR_ROTATE_270_KHR = SurfaceTransformFlagBitsKHR :: HORIZONTAL_MIRROR_ROTATE_270_KHR . 0 ; const INHERIT_KHR = SurfaceTransformFlagBitsKHR :: INHERIT_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceTransformFlagBitsKHR.html) · Bits enum of [`SurfaceTransformFlagsKHR`]"]
#[doc(alias = "VkSurfaceTransformFlagBitsKHR")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct SurfaceTransformFlagBitsKHR(pub u32);
impl SurfaceTransformFlagBitsKHR {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> SurfaceTransformFlagsKHR {
        SurfaceTransformFlagsKHR::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for SurfaceTransformFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::IDENTITY_KHR => "IDENTITY_KHR",
            &Self::ROTATE_90_KHR => "ROTATE_90_KHR",
            &Self::ROTATE_180_KHR => "ROTATE_180_KHR",
            &Self::ROTATE_270_KHR => "ROTATE_270_KHR",
            &Self::HORIZONTAL_MIRROR_KHR => "HORIZONTAL_MIRROR_KHR",
            &Self::HORIZONTAL_MIRROR_ROTATE_90_KHR => "HORIZONTAL_MIRROR_ROTATE_90_KHR",
            &Self::HORIZONTAL_MIRROR_ROTATE_180_KHR => "HORIZONTAL_MIRROR_ROTATE_180_KHR",
            &Self::HORIZONTAL_MIRROR_ROTATE_270_KHR => "HORIZONTAL_MIRROR_ROTATE_270_KHR",
            &Self::INHERIT_KHR => "INHERIT_KHR",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::khr_surface`]"]
impl SurfaceTransformFlagBitsKHR {
    pub const IDENTITY_KHR: Self = Self(1);
    pub const ROTATE_90_KHR: Self = Self(2);
    pub const ROTATE_180_KHR: Self = Self(4);
    pub const ROTATE_270_KHR: Self = Self(8);
    pub const HORIZONTAL_MIRROR_KHR: Self = Self(16);
    pub const HORIZONTAL_MIRROR_ROTATE_90_KHR: Self = Self(32);
    pub const HORIZONTAL_MIRROR_ROTATE_180_KHR: Self = Self(64);
    pub const HORIZONTAL_MIRROR_ROTATE_270_KHR: Self = Self(128);
    pub const INHERIT_KHR: Self = Self(256);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroySurfaceKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroySurfaceKHR =
    unsafe extern "system" fn(instance: crate::vk1_0::Instance, surface: crate::extensions::khr_surface::SurfaceKHR, p_allocator: *const crate::vk1_0::AllocationCallbacks) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfaceSupportKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSurfaceSupportKHR = unsafe extern "system" fn(
    physical_device: crate::vk1_0::PhysicalDevice,
    queue_family_index: u32,
    surface: crate::extensions::khr_surface::SurfaceKHR,
    p_supported: *mut crate::vk1_0::Bool32,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilitiesKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR = unsafe extern "system" fn(
    physical_device: crate::vk1_0::PhysicalDevice,
    surface: crate::extensions::khr_surface::SurfaceKHR,
    p_surface_capabilities: *mut crate::extensions::khr_surface::SurfaceCapabilitiesKHR,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfaceFormatsKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSurfaceFormatsKHR = unsafe extern "system" fn(
    physical_device: crate::vk1_0::PhysicalDevice,
    surface: crate::extensions::khr_surface::SurfaceKHR,
    p_surface_format_count: *mut u32,
    p_surface_formats: *mut crate::extensions::khr_surface::SurfaceFormatKHR,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfacePresentModesKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSurfacePresentModesKHR = unsafe extern "system" fn(
    physical_device: crate::vk1_0::PhysicalDevice,
    surface: crate::extensions::khr_surface::SurfaceKHR,
    p_present_mode_count: *mut u32,
    p_present_modes: *mut crate::extensions::khr_surface::PresentModeKHR,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceCapabilitiesKHR.html) · Structure"]
#[doc(alias = "VkSurfaceCapabilitiesKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SurfaceCapabilitiesKHR {
    pub min_image_count: u32,
    pub max_image_count: u32,
    pub current_extent: crate::vk1_0::Extent2D,
    pub min_image_extent: crate::vk1_0::Extent2D,
    pub max_image_extent: crate::vk1_0::Extent2D,
    pub max_image_array_layers: u32,
    pub supported_transforms: crate::extensions::khr_surface::SurfaceTransformFlagsKHR,
    pub current_transform: crate::extensions::khr_surface::SurfaceTransformFlagBitsKHR,
    pub supported_composite_alpha: crate::extensions::khr_surface::CompositeAlphaFlagsKHR,
    pub supported_usage_flags: crate::vk1_0::ImageUsageFlags,
}
impl Default for SurfaceCapabilitiesKHR {
    fn default() -> Self {
        Self {
            min_image_count: Default::default(),
            max_image_count: Default::default(),
            current_extent: Default::default(),
            min_image_extent: Default::default(),
            max_image_extent: Default::default(),
            max_image_array_layers: Default::default(),
            supported_transforms: Default::default(),
            current_transform: Default::default(),
            supported_composite_alpha: Default::default(),
            supported_usage_flags: Default::default(),
        }
    }
}
impl std::fmt::Debug for SurfaceCapabilitiesKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SurfaceCapabilitiesKHR")
            .field("min_image_count", &self.min_image_count)
            .field("max_image_count", &self.max_image_count)
            .field("current_extent", &self.current_extent)
            .field("min_image_extent", &self.min_image_extent)
            .field("max_image_extent", &self.max_image_extent)
            .field("max_image_array_layers", &self.max_image_array_layers)
            .field("supported_transforms", &self.supported_transforms)
            .field("current_transform", &self.current_transform)
            .field("supported_composite_alpha", &self.supported_composite_alpha)
            .field("supported_usage_flags", &self.supported_usage_flags)
            .finish()
    }
}
impl SurfaceCapabilitiesKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> SurfaceCapabilitiesKHRBuilder<'a> {
        SurfaceCapabilitiesKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceCapabilitiesKHR.html) · Builder of [`SurfaceCapabilitiesKHR`]"]
#[repr(transparent)]
pub struct SurfaceCapabilitiesKHRBuilder<'a>(SurfaceCapabilitiesKHR, std::marker::PhantomData<&'a ()>);
impl<'a> SurfaceCapabilitiesKHRBuilder<'a> {
    #[inline]
    pub fn new() -> SurfaceCapabilitiesKHRBuilder<'a> {
        SurfaceCapabilitiesKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn min_image_count(mut self, min_image_count: u32) -> Self {
        self.0.min_image_count = min_image_count as _;
        self
    }
    #[inline]
    pub fn max_image_count(mut self, max_image_count: u32) -> Self {
        self.0.max_image_count = max_image_count as _;
        self
    }
    #[inline]
    pub fn current_extent(mut self, current_extent: crate::vk1_0::Extent2D) -> Self {
        self.0.current_extent = current_extent as _;
        self
    }
    #[inline]
    pub fn min_image_extent(mut self, min_image_extent: crate::vk1_0::Extent2D) -> Self {
        self.0.min_image_extent = min_image_extent as _;
        self
    }
    #[inline]
    pub fn max_image_extent(mut self, max_image_extent: crate::vk1_0::Extent2D) -> Self {
        self.0.max_image_extent = max_image_extent as _;
        self
    }
    #[inline]
    pub fn max_image_array_layers(mut self, max_image_array_layers: u32) -> Self {
        self.0.max_image_array_layers = max_image_array_layers as _;
        self
    }
    #[inline]
    pub fn supported_transforms(mut self, supported_transforms: crate::extensions::khr_surface::SurfaceTransformFlagsKHR) -> Self {
        self.0.supported_transforms = supported_transforms as _;
        self
    }
    #[inline]
    pub fn current_transform(mut self, current_transform: crate::extensions::khr_surface::SurfaceTransformFlagBitsKHR) -> Self {
        self.0.current_transform = current_transform as _;
        self
    }
    #[inline]
    pub fn supported_composite_alpha(mut self, supported_composite_alpha: crate::extensions::khr_surface::CompositeAlphaFlagsKHR) -> Self {
        self.0.supported_composite_alpha = supported_composite_alpha as _;
        self
    }
    #[inline]
    pub fn supported_usage_flags(mut self, supported_usage_flags: crate::vk1_0::ImageUsageFlags) -> Self {
        self.0.supported_usage_flags = supported_usage_flags as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> SurfaceCapabilitiesKHR {
        self.0
    }
}
impl<'a> std::default::Default for SurfaceCapabilitiesKHRBuilder<'a> {
    fn default() -> SurfaceCapabilitiesKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for SurfaceCapabilitiesKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for SurfaceCapabilitiesKHRBuilder<'a> {
    type Target = SurfaceCapabilitiesKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SurfaceCapabilitiesKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceFormatKHR.html) · Structure"]
#[doc(alias = "VkSurfaceFormatKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SurfaceFormatKHR {
    pub format: crate::vk1_0::Format,
    pub color_space: crate::extensions::khr_surface::ColorSpaceKHR,
}
impl Default for SurfaceFormatKHR {
    fn default() -> Self {
        Self {
            format: Default::default(),
            color_space: Default::default(),
        }
    }
}
impl std::fmt::Debug for SurfaceFormatKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SurfaceFormatKHR").field("format", &self.format).field("color_space", &self.color_space).finish()
    }
}
impl SurfaceFormatKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> SurfaceFormatKHRBuilder<'a> {
        SurfaceFormatKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceFormatKHR.html) · Builder of [`SurfaceFormatKHR`]"]
#[repr(transparent)]
pub struct SurfaceFormatKHRBuilder<'a>(SurfaceFormatKHR, std::marker::PhantomData<&'a ()>);
impl<'a> SurfaceFormatKHRBuilder<'a> {
    #[inline]
    pub fn new() -> SurfaceFormatKHRBuilder<'a> {
        SurfaceFormatKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn format(mut self, format: crate::vk1_0::Format) -> Self {
        self.0.format = format as _;
        self
    }
    #[inline]
    pub fn color_space(mut self, color_space: crate::extensions::khr_surface::ColorSpaceKHR) -> Self {
        self.0.color_space = color_space as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> SurfaceFormatKHR {
        self.0
    }
}
impl<'a> std::default::Default for SurfaceFormatKHRBuilder<'a> {
    fn default() -> SurfaceFormatKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for SurfaceFormatKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for SurfaceFormatKHRBuilder<'a> {
    type Target = SurfaceFormatKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SurfaceFormatKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::khr_surface`]"]
impl crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroySurfaceKHR.html) · Function"]
    #[doc(alias = "vkDestroySurfaceKHR")]
    pub unsafe fn destroy_surface_khr(&self, surface: Option<crate::extensions::khr_surface::SurfaceKHR>, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> () {
        let _function = self.destroy_surface_khr.expect("`destroy_surface_khr` is not loaded");
        let _return = _function(
            self.handle,
            match surface {
                Some(v) => v,
                None => Default::default(),
            },
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfaceSupportKHR.html) · Function"]
    #[doc(alias = "vkGetPhysicalDeviceSurfaceSupportKHR")]
    pub unsafe fn get_physical_device_surface_support_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        queue_family_index: u32,
        surface: crate::extensions::khr_surface::SurfaceKHR,
        supported: Option<crate::vk1_0::Bool32>,
    ) -> crate::utils::VulkanResult<bool> {
        let _function = self.get_physical_device_surface_support_khr.expect("`get_physical_device_surface_support_khr` is not loaded");
        let mut supported = match supported {
            Some(v) => v,
            None => Default::default(),
        };
        let _return = _function(physical_device as _, queue_family_index as _, surface as _, &mut supported);
        crate::utils::VulkanResult::new(_return, supported != 0)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilitiesKHR.html) · Function"]
    #[doc(alias = "vkGetPhysicalDeviceSurfaceCapabilitiesKHR")]
    pub unsafe fn get_physical_device_surface_capabilities_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        surface: crate::extensions::khr_surface::SurfaceKHR,
        surface_capabilities: Option<crate::extensions::khr_surface::SurfaceCapabilitiesKHR>,
    ) -> crate::utils::VulkanResult<crate::extensions::khr_surface::SurfaceCapabilitiesKHR> {
        let _function = self.get_physical_device_surface_capabilities_khr.expect("`get_physical_device_surface_capabilities_khr` is not loaded");
        let mut surface_capabilities = match surface_capabilities {
            Some(v) => v,
            None => Default::default(),
        };
        let _return = _function(physical_device as _, surface as _, &mut surface_capabilities);
        crate::utils::VulkanResult::new(_return, surface_capabilities)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfaceFormatsKHR.html) · Function"]
    #[doc(alias = "vkGetPhysicalDeviceSurfaceFormatsKHR")]
    pub unsafe fn get_physical_device_surface_formats_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        surface: crate::extensions::khr_surface::SurfaceKHR,
        surface_format_count: Option<u32>,
    ) -> crate::utils::VulkanResult<Vec<crate::extensions::khr_surface::SurfaceFormatKHR>> {
        let _function = self.get_physical_device_surface_formats_khr.expect("`get_physical_device_surface_formats_khr` is not loaded");
        let mut surface_format_count = match surface_format_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                _function(physical_device as _, surface as _, &mut v, std::ptr::null_mut());
                v
            }
        };
        let mut surface_formats = vec![Default::default(); surface_format_count as _];
        let _return = _function(physical_device as _, surface as _, &mut surface_format_count, surface_formats.as_mut_ptr());
        crate::utils::VulkanResult::new(_return, surface_formats)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfacePresentModesKHR.html) · Function"]
    #[doc(alias = "vkGetPhysicalDeviceSurfacePresentModesKHR")]
    pub unsafe fn get_physical_device_surface_present_modes_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        surface: crate::extensions::khr_surface::SurfaceKHR,
        present_mode_count: Option<u32>,
    ) -> crate::utils::VulkanResult<Vec<crate::extensions::khr_surface::PresentModeKHR>> {
        let _function = self
            .get_physical_device_surface_present_modes_khr
            .expect("`get_physical_device_surface_present_modes_khr` is not loaded");
        let mut present_mode_count = match present_mode_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                _function(physical_device as _, surface as _, &mut v, std::ptr::null_mut());
                v
            }
        };
        let mut present_modes = vec![Default::default(); present_mode_count as _];
        let _return = _function(physical_device as _, surface as _, &mut present_mode_count, present_modes.as_mut_ptr());
        crate::utils::VulkanResult::new(_return, present_modes)
    }
}
