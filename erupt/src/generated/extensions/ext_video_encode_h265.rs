//! ## Versioning Warning ⚠️
//!
//! This is a Vulkan **provisional/beta** extension and **must** be used with
//! caution. Its API/behaviour has not been finalized yet and _may_ therefore
//! change in ways that break backwards compatibility between revisions, and
//! before final release of a non-provisional version of this extension.
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_VIDEO_ENCODE_H265_SPEC_VERSION")]
pub const EXT_VIDEO_ENCODE_H265_SPEC_VERSION: u32 = 3;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_VIDEO_ENCODE_H265_EXTENSION_NAME")]
pub const EXT_VIDEO_ENCODE_H265_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_video_encode_h265");
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH265CapabilityFlagsEXT.html) · Bitmask of [`VideoEncodeH265CapabilityFlagBitsEXT`]"] # [doc (alias = "VkVideoEncodeH265CapabilityFlagsEXT")] # [derive (Default)] # [repr (transparent)] pub struct VideoEncodeH265CapabilityFlagsEXT : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`VideoEncodeH265CapabilityFlagsEXT`]"]
#[doc(alias = "VkVideoEncodeH265CapabilityFlagBitsEXT")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct VideoEncodeH265CapabilityFlagBitsEXT(pub u32);
impl VideoEncodeH265CapabilityFlagBitsEXT {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> VideoEncodeH265CapabilityFlagsEXT {
        VideoEncodeH265CapabilityFlagsEXT::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for VideoEncodeH265CapabilityFlagBitsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH265CreateFlagsEXT.html) · Bitmask of [`VideoEncodeH265CreateFlagBitsEXT`]"] # [doc (alias = "VkVideoEncodeH265CreateFlagsEXT")] # [derive (Default)] # [repr (transparent)] pub struct VideoEncodeH265CreateFlagsEXT : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`VideoEncodeH265CreateFlagsEXT`]"]
#[doc(alias = "VkVideoEncodeH265CreateFlagBitsEXT")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct VideoEncodeH265CreateFlagBitsEXT(pub u32);
impl VideoEncodeH265CreateFlagBitsEXT {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> VideoEncodeH265CreateFlagsEXT {
        VideoEncodeH265CreateFlagsEXT::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for VideoEncodeH265CreateFlagBitsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::ext_video_encode_h265`]"]
impl crate::vk1_0::StructureType {
    pub const VIDEO_ENCODE_H265_CAPABILITIES_EXT: Self = Self(1000039000);
    pub const VIDEO_ENCODE_H265_SESSION_CREATE_INFO_EXT: Self = Self(1000039001);
    pub const VIDEO_ENCODE_H265_SESSION_PARAMETERS_CREATE_INFO_EXT: Self = Self(1000039002);
    pub const VIDEO_ENCODE_H265_SESSION_PARAMETERS_ADD_INFO_EXT: Self = Self(1000039003);
    pub const VIDEO_ENCODE_H265_VCL_FRAME_INFO_EXT: Self = Self(1000039004);
    pub const VIDEO_ENCODE_H265_DPB_SLOT_INFO_EXT: Self = Self(1000039005);
    pub const VIDEO_ENCODE_H265_NALU_SLICE_EXT: Self = Self(1000039006);
    pub const VIDEO_ENCODE_H265_EMIT_PICTURE_PARAMETERS_EXT: Self = Self(1000039007);
    pub const VIDEO_ENCODE_H265_PROFILE_EXT: Self = Self(1000039008);
    pub const VIDEO_ENCODE_H265_REFERENCE_LISTS_EXT: Self = Self(1000039009);
    pub const VIDEO_ENCODE_H265_RATE_CONTROL_INFO_EXT: Self = Self(1000039010);
    pub const VIDEO_ENCODE_H265_RATE_CONTROL_LAYER_INFO_EXT: Self = Self(1000039011);
}
#[doc = "Provided by [`crate::extensions::ext_video_encode_h265`]"]
impl crate::extensions::khr_video_queue::VideoCodecOperationFlagBitsKHR {
    pub const ENCODE_H265_EXT: Self = Self(131072);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH265InputModeFlagsEXT.html) · Bitmask of [`VideoEncodeH265InputModeFlagBitsEXT`]"] # [doc (alias = "VkVideoEncodeH265InputModeFlagsEXT")] # [derive (Default)] # [repr (transparent)] pub struct VideoEncodeH265InputModeFlagsEXT : u32 { const FRAME_EXT = VideoEncodeH265InputModeFlagBitsEXT :: FRAME_EXT . 0 ; const SLICE_EXT = VideoEncodeH265InputModeFlagBitsEXT :: SLICE_EXT . 0 ; const NON_VCL_EXT = VideoEncodeH265InputModeFlagBitsEXT :: NON_VCL_EXT . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH265InputModeFlagBitsEXT.html) · Bits enum of [`VideoEncodeH265InputModeFlagsEXT`]"]
#[doc(alias = "VkVideoEncodeH265InputModeFlagBitsEXT")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct VideoEncodeH265InputModeFlagBitsEXT(pub u32);
impl VideoEncodeH265InputModeFlagBitsEXT {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> VideoEncodeH265InputModeFlagsEXT {
        VideoEncodeH265InputModeFlagsEXT::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for VideoEncodeH265InputModeFlagBitsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::FRAME_EXT => "FRAME_EXT",
            &Self::SLICE_EXT => "SLICE_EXT",
            &Self::NON_VCL_EXT => "NON_VCL_EXT",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::ext_video_encode_h265`]"]
impl crate::extensions::ext_video_encode_h265::VideoEncodeH265InputModeFlagBitsEXT {
    pub const FRAME_EXT: Self = Self(1);
    pub const SLICE_EXT: Self = Self(2);
    pub const NON_VCL_EXT: Self = Self(4);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH265OutputModeFlagsEXT.html) · Bitmask of [`VideoEncodeH265OutputModeFlagBitsEXT`]"] # [doc (alias = "VkVideoEncodeH265OutputModeFlagsEXT")] # [derive (Default)] # [repr (transparent)] pub struct VideoEncodeH265OutputModeFlagsEXT : u32 { const FRAME_EXT = VideoEncodeH265OutputModeFlagBitsEXT :: FRAME_EXT . 0 ; const SLICE_EXT = VideoEncodeH265OutputModeFlagBitsEXT :: SLICE_EXT . 0 ; const NON_VCL_EXT = VideoEncodeH265OutputModeFlagBitsEXT :: NON_VCL_EXT . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH265OutputModeFlagBitsEXT.html) · Bits enum of [`VideoEncodeH265OutputModeFlagsEXT`]"]
#[doc(alias = "VkVideoEncodeH265OutputModeFlagBitsEXT")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct VideoEncodeH265OutputModeFlagBitsEXT(pub u32);
impl VideoEncodeH265OutputModeFlagBitsEXT {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> VideoEncodeH265OutputModeFlagsEXT {
        VideoEncodeH265OutputModeFlagsEXT::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for VideoEncodeH265OutputModeFlagBitsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::FRAME_EXT => "FRAME_EXT",
            &Self::SLICE_EXT => "SLICE_EXT",
            &Self::NON_VCL_EXT => "NON_VCL_EXT",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::ext_video_encode_h265`]"]
impl crate::extensions::ext_video_encode_h265::VideoEncodeH265OutputModeFlagBitsEXT {
    pub const FRAME_EXT: Self = Self(1);
    pub const SLICE_EXT: Self = Self(2);
    pub const NON_VCL_EXT: Self = Self(4);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH265CtbSizeFlagsEXT.html) · Bitmask of [`VideoEncodeH265CtbSizeFlagBitsEXT`]"] # [doc (alias = "VkVideoEncodeH265CtbSizeFlagsEXT")] # [derive (Default)] # [repr (transparent)] pub struct VideoEncodeH265CtbSizeFlagsEXT : u32 { const _8_EXT = VideoEncodeH265CtbSizeFlagBitsEXT :: _8_EXT . 0 ; const _16_EXT = VideoEncodeH265CtbSizeFlagBitsEXT :: _16_EXT . 0 ; const _32_EXT = VideoEncodeH265CtbSizeFlagBitsEXT :: _32_EXT . 0 ; const _64_EXT = VideoEncodeH265CtbSizeFlagBitsEXT :: _64_EXT . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH265CtbSizeFlagBitsEXT.html) · Bits enum of [`VideoEncodeH265CtbSizeFlagsEXT`]"]
#[doc(alias = "VkVideoEncodeH265CtbSizeFlagBitsEXT")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct VideoEncodeH265CtbSizeFlagBitsEXT(pub u32);
impl VideoEncodeH265CtbSizeFlagBitsEXT {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> VideoEncodeH265CtbSizeFlagsEXT {
        VideoEncodeH265CtbSizeFlagsEXT::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for VideoEncodeH265CtbSizeFlagBitsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::_8_EXT => "8_EXT",
            &Self::_16_EXT => "16_EXT",
            &Self::_32_EXT => "32_EXT",
            &Self::_64_EXT => "64_EXT",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::ext_video_encode_h265`]"]
impl crate::extensions::ext_video_encode_h265::VideoEncodeH265CtbSizeFlagBitsEXT {
    pub const _8_EXT: Self = Self(1);
    pub const _16_EXT: Self = Self(2);
    pub const _32_EXT: Self = Self(4);
    pub const _64_EXT: Self = Self(8);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH265RateControlStructureFlagsEXT.html) · Bitmask of [`VideoEncodeH265RateControlStructureFlagBitsEXT`]"] # [doc (alias = "VkVideoEncodeH265RateControlStructureFlagsEXT")] # [derive (Default)] # [repr (transparent)] pub struct VideoEncodeH265RateControlStructureFlagsEXT : u32 { const UNKNOWN_EXT = VideoEncodeH265RateControlStructureFlagBitsEXT :: UNKNOWN_EXT . 0 ; const FLAT_EXT = VideoEncodeH265RateControlStructureFlagBitsEXT :: FLAT_EXT . 0 ; const DYADIC_EXT = VideoEncodeH265RateControlStructureFlagBitsEXT :: DYADIC_EXT . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH265RateControlStructureFlagBitsEXT.html) · Bits enum of [`VideoEncodeH265RateControlStructureFlagsEXT`]"]
#[doc(alias = "VkVideoEncodeH265RateControlStructureFlagBitsEXT")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct VideoEncodeH265RateControlStructureFlagBitsEXT(pub u32);
impl VideoEncodeH265RateControlStructureFlagBitsEXT {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> VideoEncodeH265RateControlStructureFlagsEXT {
        VideoEncodeH265RateControlStructureFlagsEXT::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for VideoEncodeH265RateControlStructureFlagBitsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::UNKNOWN_EXT => "UNKNOWN_EXT",
            &Self::FLAT_EXT => "FLAT_EXT",
            &Self::DYADIC_EXT => "DYADIC_EXT",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::ext_video_encode_h265`]"]
impl crate::extensions::ext_video_encode_h265::VideoEncodeH265RateControlStructureFlagBitsEXT {
    pub const UNKNOWN_EXT: Self = Self(0);
    pub const FLAT_EXT: Self = Self(1);
    pub const DYADIC_EXT: Self = Self(2);
}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH265ProfileEXT> for crate::vk1_0::BufferCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH265ProfileEXTBuilder<'_>> for crate::vk1_0::BufferCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH265ProfileEXT> for crate::vk1_0::ImageCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH265ProfileEXTBuilder<'_>> for crate::vk1_0::ImageCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH265ProfileEXT> for crate::vk1_0::ImageViewCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH265ProfileEXTBuilder<'_>> for crate::vk1_0::ImageViewCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH265ProfileEXT> for crate::vk1_0::QueryPoolCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH265ProfileEXTBuilder<'_>> for crate::vk1_0::QueryPoolCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH265ProfileEXT> for crate::vk1_1::FormatProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH265ProfileEXTBuilder<'_>> for crate::vk1_1::FormatProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH265ProfileEXT> for crate::extensions::khr_video_queue::VideoProfileKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH265ProfileEXTBuilder<'_>> for crate::extensions::khr_video_queue::VideoProfileKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH265CapabilitiesEXT> for crate::extensions::khr_video_queue::VideoCapabilitiesKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH265CapabilitiesEXTBuilder<'_>> for crate::extensions::khr_video_queue::VideoCapabilitiesKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH265SessionCreateInfoEXT> for crate::extensions::khr_video_queue::VideoSessionCreateInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH265SessionCreateInfoEXTBuilder<'_>> for crate::extensions::khr_video_queue::VideoSessionCreateInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH265SessionParametersCreateInfoEXT> for crate::extensions::khr_video_queue::VideoSessionParametersCreateInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH265SessionParametersCreateInfoEXTBuilder<'_>> for crate::extensions::khr_video_queue::VideoSessionParametersCreateInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH265SessionParametersAddInfoEXT> for crate::extensions::khr_video_queue::VideoSessionParametersUpdateInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH265SessionParametersAddInfoEXTBuilder<'_>> for crate::extensions::khr_video_queue::VideoSessionParametersUpdateInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH265VclFrameInfoEXT> for crate::extensions::khr_video_encode_queue::VideoEncodeInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH265VclFrameInfoEXTBuilder<'_>> for crate::extensions::khr_video_encode_queue::VideoEncodeInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH265EmitPictureParametersEXT> for crate::extensions::khr_video_encode_queue::VideoEncodeInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH265EmitPictureParametersEXTBuilder<'_>> for crate::extensions::khr_video_encode_queue::VideoEncodeInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH265RateControlInfoEXT> for crate::extensions::khr_video_encode_queue::VideoEncodeRateControlInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH265RateControlInfoEXTBuilder<'_>> for crate::extensions::khr_video_encode_queue::VideoEncodeRateControlInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH265RateControlLayerInfoEXT> for crate::extensions::khr_video_encode_queue::VideoEncodeRateControlLayerInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH265RateControlLayerInfoEXTBuilder<'_>> for crate::extensions::khr_video_encode_queue::VideoEncodeRateControlLayerInfoKHRBuilder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH265CapabilitiesEXT.html) · Structure"]
#[doc(alias = "VkVideoEncodeH265CapabilitiesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoEncodeH265CapabilitiesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::ext_video_encode_h265::VideoEncodeH265CapabilityFlagsEXT,
    pub input_mode_flags: crate::extensions::ext_video_encode_h265::VideoEncodeH265InputModeFlagsEXT,
    pub output_mode_flags: crate::extensions::ext_video_encode_h265::VideoEncodeH265OutputModeFlagsEXT,
    pub ctb_sizes: crate::extensions::ext_video_encode_h265::VideoEncodeH265CtbSizeFlagsEXT,
    pub input_image_data_alignment: crate::vk1_0::Extent2D,
    pub max_num_l0_reference_for_p: u8,
    pub max_num_l0_reference_for_b: u8,
    pub max_num_l1_reference: u8,
    pub max_num_sub_layers: u8,
    pub quality_level_count: u8,
    pub std_extension_version: crate::vk1_0::ExtensionProperties,
}
impl VideoEncodeH265CapabilitiesEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::VIDEO_ENCODE_H265_CAPABILITIES_EXT;
}
impl Default for VideoEncodeH265CapabilitiesEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), flags: Default::default(), input_mode_flags: Default::default(), output_mode_flags: Default::default(), ctb_sizes: Default::default(), input_image_data_alignment: Default::default(), max_num_l0_reference_for_p: Default::default(), max_num_l0_reference_for_b: Default::default(), max_num_l1_reference: Default::default(), max_num_sub_layers: Default::default(), quality_level_count: Default::default(), std_extension_version: Default::default() }
    }
}
impl std::fmt::Debug for VideoEncodeH265CapabilitiesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoEncodeH265CapabilitiesEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("input_mode_flags", &self.input_mode_flags).field("output_mode_flags", &self.output_mode_flags).field("ctb_sizes", &self.ctb_sizes).field("input_image_data_alignment", &self.input_image_data_alignment).field("max_num_l0_reference_for_p", &self.max_num_l0_reference_for_p).field("max_num_l0_reference_for_b", &self.max_num_l0_reference_for_b).field("max_num_l1_reference", &self.max_num_l1_reference).field("max_num_sub_layers", &self.max_num_sub_layers).field("quality_level_count", &self.quality_level_count).field("std_extension_version", &self.std_extension_version).finish()
    }
}
impl VideoEncodeH265CapabilitiesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoEncodeH265CapabilitiesEXTBuilder<'a> {
        VideoEncodeH265CapabilitiesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH265CapabilitiesEXT.html) · Builder of [`VideoEncodeH265CapabilitiesEXT`]"]
#[repr(transparent)]
pub struct VideoEncodeH265CapabilitiesEXTBuilder<'a>(VideoEncodeH265CapabilitiesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> VideoEncodeH265CapabilitiesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> VideoEncodeH265CapabilitiesEXTBuilder<'a> {
        VideoEncodeH265CapabilitiesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn flags(mut self, flags: crate::extensions::ext_video_encode_h265::VideoEncodeH265CapabilityFlagsEXT) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn input_mode_flags(mut self, input_mode_flags: crate::extensions::ext_video_encode_h265::VideoEncodeH265InputModeFlagsEXT) -> Self {
        self.0.input_mode_flags = input_mode_flags as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn output_mode_flags(mut self, output_mode_flags: crate::extensions::ext_video_encode_h265::VideoEncodeH265OutputModeFlagsEXT) -> Self {
        self.0.output_mode_flags = output_mode_flags as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn ctb_sizes(mut self, ctb_sizes: crate::extensions::ext_video_encode_h265::VideoEncodeH265CtbSizeFlagsEXT) -> Self {
        self.0.ctb_sizes = ctb_sizes as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn input_image_data_alignment(mut self, input_image_data_alignment: crate::vk1_0::Extent2D) -> Self {
        self.0.input_image_data_alignment = input_image_data_alignment as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn max_num_l0_reference_for_p(mut self, max_num_l0_reference_for_p: u8) -> Self {
        self.0.max_num_l0_reference_for_p = max_num_l0_reference_for_p as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn max_num_l0_reference_for_b(mut self, max_num_l0_reference_for_b: u8) -> Self {
        self.0.max_num_l0_reference_for_b = max_num_l0_reference_for_b as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn max_num_l1_reference(mut self, max_num_l1_reference: u8) -> Self {
        self.0.max_num_l1_reference = max_num_l1_reference as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn max_num_sub_layers(mut self, max_num_sub_layers: u8) -> Self {
        self.0.max_num_sub_layers = max_num_sub_layers as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn quality_level_count(mut self, quality_level_count: u8) -> Self {
        self.0.quality_level_count = quality_level_count as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn std_extension_version(mut self, std_extension_version: crate::vk1_0::ExtensionProperties) -> Self {
        self.0.std_extension_version = std_extension_version as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> VideoEncodeH265CapabilitiesEXT {
        self.0
    }
}
impl<'a> std::default::Default for VideoEncodeH265CapabilitiesEXTBuilder<'a> {
    fn default() -> VideoEncodeH265CapabilitiesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoEncodeH265CapabilitiesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoEncodeH265CapabilitiesEXTBuilder<'a> {
    type Target = VideoEncodeH265CapabilitiesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoEncodeH265CapabilitiesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH265SessionCreateInfoEXT.html) · Structure"]
#[doc(alias = "VkVideoEncodeH265SessionCreateInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoEncodeH265SessionCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::ext_video_encode_h265::VideoEncodeH265CreateFlagsEXT,
    pub p_std_extension_version: *const crate::vk1_0::ExtensionProperties,
}
impl VideoEncodeH265SessionCreateInfoEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::VIDEO_ENCODE_H265_SESSION_CREATE_INFO_EXT;
}
impl Default for VideoEncodeH265SessionCreateInfoEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), flags: Default::default(), p_std_extension_version: std::ptr::null() }
    }
}
impl std::fmt::Debug for VideoEncodeH265SessionCreateInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoEncodeH265SessionCreateInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("p_std_extension_version", &self.p_std_extension_version).finish()
    }
}
impl VideoEncodeH265SessionCreateInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoEncodeH265SessionCreateInfoEXTBuilder<'a> {
        VideoEncodeH265SessionCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH265SessionCreateInfoEXT.html) · Builder of [`VideoEncodeH265SessionCreateInfoEXT`]"]
#[repr(transparent)]
pub struct VideoEncodeH265SessionCreateInfoEXTBuilder<'a>(VideoEncodeH265SessionCreateInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> VideoEncodeH265SessionCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> VideoEncodeH265SessionCreateInfoEXTBuilder<'a> {
        VideoEncodeH265SessionCreateInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn flags(mut self, flags: crate::extensions::ext_video_encode_h265::VideoEncodeH265CreateFlagsEXT) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn std_extension_version(mut self, std_extension_version: &'a crate::vk1_0::ExtensionProperties) -> Self {
        self.0.p_std_extension_version = std_extension_version as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> VideoEncodeH265SessionCreateInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for VideoEncodeH265SessionCreateInfoEXTBuilder<'a> {
    fn default() -> VideoEncodeH265SessionCreateInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoEncodeH265SessionCreateInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoEncodeH265SessionCreateInfoEXTBuilder<'a> {
    type Target = VideoEncodeH265SessionCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoEncodeH265SessionCreateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH265SessionParametersAddInfoEXT.html) · Structure"]
#[doc(alias = "VkVideoEncodeH265SessionParametersAddInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoEncodeH265SessionParametersAddInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub vps_std_count: u32,
    pub p_vps_std: *const crate::external::vk_video::StdVideoH265VideoParameterSet,
    pub sps_std_count: u32,
    pub p_sps_std: *const crate::external::vk_video::StdVideoH265SequenceParameterSet,
    pub pps_std_count: u32,
    pub p_pps_std: *const crate::external::vk_video::StdVideoH265PictureParameterSet,
}
impl VideoEncodeH265SessionParametersAddInfoEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::VIDEO_ENCODE_H265_SESSION_PARAMETERS_ADD_INFO_EXT;
}
impl Default for VideoEncodeH265SessionParametersAddInfoEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), vps_std_count: Default::default(), p_vps_std: std::ptr::null(), sps_std_count: Default::default(), p_sps_std: std::ptr::null(), pps_std_count: Default::default(), p_pps_std: std::ptr::null() }
    }
}
impl std::fmt::Debug for VideoEncodeH265SessionParametersAddInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoEncodeH265SessionParametersAddInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("vps_std_count", &self.vps_std_count).field("p_vps_std", &self.p_vps_std).field("sps_std_count", &self.sps_std_count).field("p_sps_std", &self.p_sps_std).field("pps_std_count", &self.pps_std_count).field("p_pps_std", &self.p_pps_std).finish()
    }
}
impl VideoEncodeH265SessionParametersAddInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoEncodeH265SessionParametersAddInfoEXTBuilder<'a> {
        VideoEncodeH265SessionParametersAddInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH265SessionParametersAddInfoEXT.html) · Builder of [`VideoEncodeH265SessionParametersAddInfoEXT`]"]
#[repr(transparent)]
pub struct VideoEncodeH265SessionParametersAddInfoEXTBuilder<'a>(VideoEncodeH265SessionParametersAddInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> VideoEncodeH265SessionParametersAddInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> VideoEncodeH265SessionParametersAddInfoEXTBuilder<'a> {
        VideoEncodeH265SessionParametersAddInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn vps_std(mut self, vps_std: &'a [crate::external::vk_video::StdVideoH265VideoParameterSetBuilder]) -> Self {
        self.0.p_vps_std = vps_std.as_ptr() as _;
        self.0.vps_std_count = vps_std.len() as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn sps_std(mut self, sps_std: &'a [crate::external::vk_video::StdVideoH265SequenceParameterSetBuilder]) -> Self {
        self.0.p_sps_std = sps_std.as_ptr() as _;
        self.0.sps_std_count = sps_std.len() as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn pps_std(mut self, pps_std: &'a [crate::external::vk_video::StdVideoH265PictureParameterSetBuilder]) -> Self {
        self.0.p_pps_std = pps_std.as_ptr() as _;
        self.0.pps_std_count = pps_std.len() as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> VideoEncodeH265SessionParametersAddInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for VideoEncodeH265SessionParametersAddInfoEXTBuilder<'a> {
    fn default() -> VideoEncodeH265SessionParametersAddInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoEncodeH265SessionParametersAddInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoEncodeH265SessionParametersAddInfoEXTBuilder<'a> {
    type Target = VideoEncodeH265SessionParametersAddInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoEncodeH265SessionParametersAddInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH265SessionParametersCreateInfoEXT.html) · Structure"]
#[doc(alias = "VkVideoEncodeH265SessionParametersCreateInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoEncodeH265SessionParametersCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub max_vps_std_count: u32,
    pub max_sps_std_count: u32,
    pub max_pps_std_count: u32,
    pub p_parameters_add_info: *const crate::extensions::ext_video_encode_h265::VideoEncodeH265SessionParametersAddInfoEXT,
}
impl VideoEncodeH265SessionParametersCreateInfoEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::VIDEO_ENCODE_H265_SESSION_PARAMETERS_CREATE_INFO_EXT;
}
impl Default for VideoEncodeH265SessionParametersCreateInfoEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), max_vps_std_count: Default::default(), max_sps_std_count: Default::default(), max_pps_std_count: Default::default(), p_parameters_add_info: std::ptr::null() }
    }
}
impl std::fmt::Debug for VideoEncodeH265SessionParametersCreateInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoEncodeH265SessionParametersCreateInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("max_vps_std_count", &self.max_vps_std_count).field("max_sps_std_count", &self.max_sps_std_count).field("max_pps_std_count", &self.max_pps_std_count).field("p_parameters_add_info", &self.p_parameters_add_info).finish()
    }
}
impl VideoEncodeH265SessionParametersCreateInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoEncodeH265SessionParametersCreateInfoEXTBuilder<'a> {
        VideoEncodeH265SessionParametersCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH265SessionParametersCreateInfoEXT.html) · Builder of [`VideoEncodeH265SessionParametersCreateInfoEXT`]"]
#[repr(transparent)]
pub struct VideoEncodeH265SessionParametersCreateInfoEXTBuilder<'a>(VideoEncodeH265SessionParametersCreateInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> VideoEncodeH265SessionParametersCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> VideoEncodeH265SessionParametersCreateInfoEXTBuilder<'a> {
        VideoEncodeH265SessionParametersCreateInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn max_vps_std_count(mut self, max_vps_std_count: u32) -> Self {
        self.0.max_vps_std_count = max_vps_std_count as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn max_sps_std_count(mut self, max_sps_std_count: u32) -> Self {
        self.0.max_sps_std_count = max_sps_std_count as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn max_pps_std_count(mut self, max_pps_std_count: u32) -> Self {
        self.0.max_pps_std_count = max_pps_std_count as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn parameters_add_info(mut self, parameters_add_info: &'a crate::extensions::ext_video_encode_h265::VideoEncodeH265SessionParametersAddInfoEXT) -> Self {
        self.0.p_parameters_add_info = parameters_add_info as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> VideoEncodeH265SessionParametersCreateInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for VideoEncodeH265SessionParametersCreateInfoEXTBuilder<'a> {
    fn default() -> VideoEncodeH265SessionParametersCreateInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoEncodeH265SessionParametersCreateInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoEncodeH265SessionParametersCreateInfoEXTBuilder<'a> {
    type Target = VideoEncodeH265SessionParametersCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoEncodeH265SessionParametersCreateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH265VclFrameInfoEXT.html) · Structure"]
#[doc(alias = "VkVideoEncodeH265VclFrameInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoEncodeH265VclFrameInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub p_reference_final_lists: *const crate::extensions::ext_video_encode_h265::VideoEncodeH265ReferenceListsEXT,
    pub nalu_slice_entry_count: u32,
    pub p_nalu_slice_entries: *const crate::extensions::ext_video_encode_h265::VideoEncodeH265NaluSliceEXT,
    pub p_current_picture_info: *const crate::external::vk_video::StdVideoEncodeH265PictureInfo,
}
impl VideoEncodeH265VclFrameInfoEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::VIDEO_ENCODE_H265_VCL_FRAME_INFO_EXT;
}
impl Default for VideoEncodeH265VclFrameInfoEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), p_reference_final_lists: std::ptr::null(), nalu_slice_entry_count: Default::default(), p_nalu_slice_entries: std::ptr::null(), p_current_picture_info: std::ptr::null() }
    }
}
impl std::fmt::Debug for VideoEncodeH265VclFrameInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoEncodeH265VclFrameInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("p_reference_final_lists", &self.p_reference_final_lists).field("nalu_slice_entry_count", &self.nalu_slice_entry_count).field("p_nalu_slice_entries", &self.p_nalu_slice_entries).field("p_current_picture_info", &self.p_current_picture_info).finish()
    }
}
impl VideoEncodeH265VclFrameInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoEncodeH265VclFrameInfoEXTBuilder<'a> {
        VideoEncodeH265VclFrameInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH265VclFrameInfoEXT.html) · Builder of [`VideoEncodeH265VclFrameInfoEXT`]"]
#[repr(transparent)]
pub struct VideoEncodeH265VclFrameInfoEXTBuilder<'a>(VideoEncodeH265VclFrameInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> VideoEncodeH265VclFrameInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> VideoEncodeH265VclFrameInfoEXTBuilder<'a> {
        VideoEncodeH265VclFrameInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn reference_final_lists(mut self, reference_final_lists: &'a crate::extensions::ext_video_encode_h265::VideoEncodeH265ReferenceListsEXT) -> Self {
        self.0.p_reference_final_lists = reference_final_lists as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn nalu_slice_entries(mut self, nalu_slice_entries: &'a [crate::extensions::ext_video_encode_h265::VideoEncodeH265NaluSliceEXTBuilder]) -> Self {
        self.0.p_nalu_slice_entries = nalu_slice_entries.as_ptr() as _;
        self.0.nalu_slice_entry_count = nalu_slice_entries.len() as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn current_picture_info(mut self, current_picture_info: &'a crate::external::vk_video::StdVideoEncodeH265PictureInfo) -> Self {
        self.0.p_current_picture_info = current_picture_info as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> VideoEncodeH265VclFrameInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for VideoEncodeH265VclFrameInfoEXTBuilder<'a> {
    fn default() -> VideoEncodeH265VclFrameInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoEncodeH265VclFrameInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoEncodeH265VclFrameInfoEXTBuilder<'a> {
    type Target = VideoEncodeH265VclFrameInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoEncodeH265VclFrameInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH265EmitPictureParametersEXT.html) · Structure"]
#[doc(alias = "VkVideoEncodeH265EmitPictureParametersEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoEncodeH265EmitPictureParametersEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub vps_id: u8,
    pub sps_id: u8,
    pub emit_vps_enable: crate::vk1_0::Bool32,
    pub emit_sps_enable: crate::vk1_0::Bool32,
    pub pps_id_entry_count: u32,
    pub pps_id_entries: *const u8,
}
impl VideoEncodeH265EmitPictureParametersEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::VIDEO_ENCODE_H265_EMIT_PICTURE_PARAMETERS_EXT;
}
impl Default for VideoEncodeH265EmitPictureParametersEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), vps_id: Default::default(), sps_id: Default::default(), emit_vps_enable: Default::default(), emit_sps_enable: Default::default(), pps_id_entry_count: Default::default(), pps_id_entries: std::ptr::null() }
    }
}
impl std::fmt::Debug for VideoEncodeH265EmitPictureParametersEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoEncodeH265EmitPictureParametersEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("vps_id", &self.vps_id).field("sps_id", &self.sps_id).field("emit_vps_enable", &(self.emit_vps_enable != 0)).field("emit_sps_enable", &(self.emit_sps_enable != 0)).field("pps_id_entry_count", &self.pps_id_entry_count).field("pps_id_entries", &self.pps_id_entries).finish()
    }
}
impl VideoEncodeH265EmitPictureParametersEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoEncodeH265EmitPictureParametersEXTBuilder<'a> {
        VideoEncodeH265EmitPictureParametersEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH265EmitPictureParametersEXT.html) · Builder of [`VideoEncodeH265EmitPictureParametersEXT`]"]
#[repr(transparent)]
pub struct VideoEncodeH265EmitPictureParametersEXTBuilder<'a>(VideoEncodeH265EmitPictureParametersEXT, std::marker::PhantomData<&'a ()>);
impl<'a> VideoEncodeH265EmitPictureParametersEXTBuilder<'a> {
    #[inline]
    pub fn new() -> VideoEncodeH265EmitPictureParametersEXTBuilder<'a> {
        VideoEncodeH265EmitPictureParametersEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn vps_id(mut self, vps_id: u8) -> Self {
        self.0.vps_id = vps_id as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn sps_id(mut self, sps_id: u8) -> Self {
        self.0.sps_id = sps_id as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn emit_vps_enable(mut self, emit_vps_enable: bool) -> Self {
        self.0.emit_vps_enable = emit_vps_enable as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn emit_sps_enable(mut self, emit_sps_enable: bool) -> Self {
        self.0.emit_sps_enable = emit_sps_enable as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn pps_id_entries(mut self, pps_id_entries: &'a [u8]) -> Self {
        self.0.pps_id_entries = pps_id_entries.as_ptr() as _;
        self.0.pps_id_entry_count = pps_id_entries.len() as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> VideoEncodeH265EmitPictureParametersEXT {
        self.0
    }
}
impl<'a> std::default::Default for VideoEncodeH265EmitPictureParametersEXTBuilder<'a> {
    fn default() -> VideoEncodeH265EmitPictureParametersEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoEncodeH265EmitPictureParametersEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoEncodeH265EmitPictureParametersEXTBuilder<'a> {
    type Target = VideoEncodeH265EmitPictureParametersEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoEncodeH265EmitPictureParametersEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH265NaluSliceEXT.html) · Structure"]
#[doc(alias = "VkVideoEncodeH265NaluSliceEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoEncodeH265NaluSliceEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub ctb_count: u32,
    pub p_reference_final_lists: *const crate::extensions::ext_video_encode_h265::VideoEncodeH265ReferenceListsEXT,
    pub p_slice_header_std: *const crate::external::vk_video::StdVideoEncodeH265SliceHeader,
}
impl VideoEncodeH265NaluSliceEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::VIDEO_ENCODE_H265_NALU_SLICE_EXT;
}
impl Default for VideoEncodeH265NaluSliceEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), ctb_count: Default::default(), p_reference_final_lists: std::ptr::null(), p_slice_header_std: std::ptr::null() }
    }
}
impl std::fmt::Debug for VideoEncodeH265NaluSliceEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoEncodeH265NaluSliceEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("ctb_count", &self.ctb_count).field("p_reference_final_lists", &self.p_reference_final_lists).field("p_slice_header_std", &self.p_slice_header_std).finish()
    }
}
impl VideoEncodeH265NaluSliceEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoEncodeH265NaluSliceEXTBuilder<'a> {
        VideoEncodeH265NaluSliceEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH265NaluSliceEXT.html) · Builder of [`VideoEncodeH265NaluSliceEXT`]"]
#[repr(transparent)]
pub struct VideoEncodeH265NaluSliceEXTBuilder<'a>(VideoEncodeH265NaluSliceEXT, std::marker::PhantomData<&'a ()>);
impl<'a> VideoEncodeH265NaluSliceEXTBuilder<'a> {
    #[inline]
    pub fn new() -> VideoEncodeH265NaluSliceEXTBuilder<'a> {
        VideoEncodeH265NaluSliceEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn ctb_count(mut self, ctb_count: u32) -> Self {
        self.0.ctb_count = ctb_count as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn reference_final_lists(mut self, reference_final_lists: &'a crate::extensions::ext_video_encode_h265::VideoEncodeH265ReferenceListsEXT) -> Self {
        self.0.p_reference_final_lists = reference_final_lists as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn slice_header_std(mut self, slice_header_std: &'a crate::external::vk_video::StdVideoEncodeH265SliceHeader) -> Self {
        self.0.p_slice_header_std = slice_header_std as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> VideoEncodeH265NaluSliceEXT {
        self.0
    }
}
impl<'a> std::default::Default for VideoEncodeH265NaluSliceEXTBuilder<'a> {
    fn default() -> VideoEncodeH265NaluSliceEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoEncodeH265NaluSliceEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoEncodeH265NaluSliceEXTBuilder<'a> {
    type Target = VideoEncodeH265NaluSliceEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoEncodeH265NaluSliceEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH265RateControlInfoEXT.html) · Structure"]
#[doc(alias = "VkVideoEncodeH265RateControlInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoEncodeH265RateControlInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub gop_frame_count: u32,
    pub idr_period: u32,
    pub consecutive_b_frame_count: u32,
    pub rate_control_structure: crate::extensions::ext_video_encode_h265::VideoEncodeH265RateControlStructureFlagBitsEXT,
    pub sub_layer_count: u8,
}
impl VideoEncodeH265RateControlInfoEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::VIDEO_ENCODE_H265_RATE_CONTROL_INFO_EXT;
}
impl Default for VideoEncodeH265RateControlInfoEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), gop_frame_count: Default::default(), idr_period: Default::default(), consecutive_b_frame_count: Default::default(), rate_control_structure: Default::default(), sub_layer_count: Default::default() }
    }
}
impl std::fmt::Debug for VideoEncodeH265RateControlInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoEncodeH265RateControlInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("gop_frame_count", &self.gop_frame_count).field("idr_period", &self.idr_period).field("consecutive_b_frame_count", &self.consecutive_b_frame_count).field("rate_control_structure", &self.rate_control_structure).field("sub_layer_count", &self.sub_layer_count).finish()
    }
}
impl VideoEncodeH265RateControlInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoEncodeH265RateControlInfoEXTBuilder<'a> {
        VideoEncodeH265RateControlInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH265RateControlInfoEXT.html) · Builder of [`VideoEncodeH265RateControlInfoEXT`]"]
#[repr(transparent)]
pub struct VideoEncodeH265RateControlInfoEXTBuilder<'a>(VideoEncodeH265RateControlInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> VideoEncodeH265RateControlInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> VideoEncodeH265RateControlInfoEXTBuilder<'a> {
        VideoEncodeH265RateControlInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn gop_frame_count(mut self, gop_frame_count: u32) -> Self {
        self.0.gop_frame_count = gop_frame_count as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn idr_period(mut self, idr_period: u32) -> Self {
        self.0.idr_period = idr_period as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn consecutive_b_frame_count(mut self, consecutive_b_frame_count: u32) -> Self {
        self.0.consecutive_b_frame_count = consecutive_b_frame_count as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn rate_control_structure(mut self, rate_control_structure: crate::extensions::ext_video_encode_h265::VideoEncodeH265RateControlStructureFlagBitsEXT) -> Self {
        self.0.rate_control_structure = rate_control_structure as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn sub_layer_count(mut self, sub_layer_count: u8) -> Self {
        self.0.sub_layer_count = sub_layer_count as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> VideoEncodeH265RateControlInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for VideoEncodeH265RateControlInfoEXTBuilder<'a> {
    fn default() -> VideoEncodeH265RateControlInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoEncodeH265RateControlInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoEncodeH265RateControlInfoEXTBuilder<'a> {
    type Target = VideoEncodeH265RateControlInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoEncodeH265RateControlInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH265QpEXT.html) · Structure"]
#[doc(alias = "VkVideoEncodeH265QpEXT")]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct VideoEncodeH265QpEXT {
    pub qp_i: i32,
    pub qp_p: i32,
    pub qp_b: i32,
}
impl Default for VideoEncodeH265QpEXT {
    fn default() -> Self {
        Self { qp_i: Default::default(), qp_p: Default::default(), qp_b: Default::default() }
    }
}
impl std::fmt::Debug for VideoEncodeH265QpEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoEncodeH265QpEXT").field("qp_i", &self.qp_i).field("qp_p", &self.qp_p).field("qp_b", &self.qp_b).finish()
    }
}
impl VideoEncodeH265QpEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoEncodeH265QpEXTBuilder<'a> {
        VideoEncodeH265QpEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH265QpEXT.html) · Builder of [`VideoEncodeH265QpEXT`]"]
#[repr(transparent)]
pub struct VideoEncodeH265QpEXTBuilder<'a>(VideoEncodeH265QpEXT, std::marker::PhantomData<&'a ()>);
impl<'a> VideoEncodeH265QpEXTBuilder<'a> {
    #[inline]
    pub fn new() -> VideoEncodeH265QpEXTBuilder<'a> {
        VideoEncodeH265QpEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn qp_i(mut self, qp_i: i32) -> Self {
        self.0.qp_i = qp_i as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn qp_p(mut self, qp_p: i32) -> Self {
        self.0.qp_p = qp_p as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn qp_b(mut self, qp_b: i32) -> Self {
        self.0.qp_b = qp_b as _;
        self
    }
    #[inline]
    pub fn build(self) -> VideoEncodeH265QpEXT {
        self.0
    }
}
impl<'a> std::default::Default for VideoEncodeH265QpEXTBuilder<'a> {
    fn default() -> VideoEncodeH265QpEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoEncodeH265QpEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoEncodeH265QpEXTBuilder<'a> {
    type Target = VideoEncodeH265QpEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoEncodeH265QpEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH265FrameSizeEXT.html) · Structure"]
#[doc(alias = "VkVideoEncodeH265FrameSizeEXT")]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct VideoEncodeH265FrameSizeEXT {
    pub frame_i_size: u32,
    pub frame_p_size: u32,
    pub frame_b_size: u32,
}
impl Default for VideoEncodeH265FrameSizeEXT {
    fn default() -> Self {
        Self { frame_i_size: Default::default(), frame_p_size: Default::default(), frame_b_size: Default::default() }
    }
}
impl std::fmt::Debug for VideoEncodeH265FrameSizeEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoEncodeH265FrameSizeEXT").field("frame_i_size", &self.frame_i_size).field("frame_p_size", &self.frame_p_size).field("frame_b_size", &self.frame_b_size).finish()
    }
}
impl VideoEncodeH265FrameSizeEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoEncodeH265FrameSizeEXTBuilder<'a> {
        VideoEncodeH265FrameSizeEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH265FrameSizeEXT.html) · Builder of [`VideoEncodeH265FrameSizeEXT`]"]
#[repr(transparent)]
pub struct VideoEncodeH265FrameSizeEXTBuilder<'a>(VideoEncodeH265FrameSizeEXT, std::marker::PhantomData<&'a ()>);
impl<'a> VideoEncodeH265FrameSizeEXTBuilder<'a> {
    #[inline]
    pub fn new() -> VideoEncodeH265FrameSizeEXTBuilder<'a> {
        VideoEncodeH265FrameSizeEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn frame_i_size(mut self, frame_i_size: u32) -> Self {
        self.0.frame_i_size = frame_i_size as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn frame_p_size(mut self, frame_p_size: u32) -> Self {
        self.0.frame_p_size = frame_p_size as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn frame_b_size(mut self, frame_b_size: u32) -> Self {
        self.0.frame_b_size = frame_b_size as _;
        self
    }
    #[inline]
    pub fn build(self) -> VideoEncodeH265FrameSizeEXT {
        self.0
    }
}
impl<'a> std::default::Default for VideoEncodeH265FrameSizeEXTBuilder<'a> {
    fn default() -> VideoEncodeH265FrameSizeEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoEncodeH265FrameSizeEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoEncodeH265FrameSizeEXTBuilder<'a> {
    type Target = VideoEncodeH265FrameSizeEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoEncodeH265FrameSizeEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH265RateControlLayerInfoEXT.html) · Structure"]
#[doc(alias = "VkVideoEncodeH265RateControlLayerInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoEncodeH265RateControlLayerInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub temporal_id: u8,
    pub use_initial_rc_qp: crate::vk1_0::Bool32,
    pub initial_rc_qp: crate::extensions::ext_video_encode_h265::VideoEncodeH265QpEXT,
    pub use_min_qp: crate::vk1_0::Bool32,
    pub min_qp: crate::extensions::ext_video_encode_h265::VideoEncodeH265QpEXT,
    pub use_max_qp: crate::vk1_0::Bool32,
    pub max_qp: crate::extensions::ext_video_encode_h265::VideoEncodeH265QpEXT,
    pub use_max_frame_size: crate::vk1_0::Bool32,
    pub max_frame_size: crate::extensions::ext_video_encode_h265::VideoEncodeH265FrameSizeEXT,
}
impl VideoEncodeH265RateControlLayerInfoEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::VIDEO_ENCODE_H265_RATE_CONTROL_LAYER_INFO_EXT;
}
impl Default for VideoEncodeH265RateControlLayerInfoEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), temporal_id: Default::default(), use_initial_rc_qp: Default::default(), initial_rc_qp: Default::default(), use_min_qp: Default::default(), min_qp: Default::default(), use_max_qp: Default::default(), max_qp: Default::default(), use_max_frame_size: Default::default(), max_frame_size: Default::default() }
    }
}
impl std::fmt::Debug for VideoEncodeH265RateControlLayerInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoEncodeH265RateControlLayerInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("temporal_id", &self.temporal_id).field("use_initial_rc_qp", &(self.use_initial_rc_qp != 0)).field("initial_rc_qp", &self.initial_rc_qp).field("use_min_qp", &(self.use_min_qp != 0)).field("min_qp", &self.min_qp).field("use_max_qp", &(self.use_max_qp != 0)).field("max_qp", &self.max_qp).field("use_max_frame_size", &(self.use_max_frame_size != 0)).field("max_frame_size", &self.max_frame_size).finish()
    }
}
impl VideoEncodeH265RateControlLayerInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoEncodeH265RateControlLayerInfoEXTBuilder<'a> {
        VideoEncodeH265RateControlLayerInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH265RateControlLayerInfoEXT.html) · Builder of [`VideoEncodeH265RateControlLayerInfoEXT`]"]
#[repr(transparent)]
pub struct VideoEncodeH265RateControlLayerInfoEXTBuilder<'a>(VideoEncodeH265RateControlLayerInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> VideoEncodeH265RateControlLayerInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> VideoEncodeH265RateControlLayerInfoEXTBuilder<'a> {
        VideoEncodeH265RateControlLayerInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn temporal_id(mut self, temporal_id: u8) -> Self {
        self.0.temporal_id = temporal_id as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn use_initial_rc_qp(mut self, use_initial_rc_qp: bool) -> Self {
        self.0.use_initial_rc_qp = use_initial_rc_qp as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn initial_rc_qp(mut self, initial_rc_qp: crate::extensions::ext_video_encode_h265::VideoEncodeH265QpEXT) -> Self {
        self.0.initial_rc_qp = initial_rc_qp as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn use_min_qp(mut self, use_min_qp: bool) -> Self {
        self.0.use_min_qp = use_min_qp as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn min_qp(mut self, min_qp: crate::extensions::ext_video_encode_h265::VideoEncodeH265QpEXT) -> Self {
        self.0.min_qp = min_qp as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn use_max_qp(mut self, use_max_qp: bool) -> Self {
        self.0.use_max_qp = use_max_qp as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn max_qp(mut self, max_qp: crate::extensions::ext_video_encode_h265::VideoEncodeH265QpEXT) -> Self {
        self.0.max_qp = max_qp as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn use_max_frame_size(mut self, use_max_frame_size: bool) -> Self {
        self.0.use_max_frame_size = use_max_frame_size as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn max_frame_size(mut self, max_frame_size: crate::extensions::ext_video_encode_h265::VideoEncodeH265FrameSizeEXT) -> Self {
        self.0.max_frame_size = max_frame_size as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> VideoEncodeH265RateControlLayerInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for VideoEncodeH265RateControlLayerInfoEXTBuilder<'a> {
    fn default() -> VideoEncodeH265RateControlLayerInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoEncodeH265RateControlLayerInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoEncodeH265RateControlLayerInfoEXTBuilder<'a> {
    type Target = VideoEncodeH265RateControlLayerInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoEncodeH265RateControlLayerInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH265ProfileEXT.html) · Structure"]
#[doc(alias = "VkVideoEncodeH265ProfileEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoEncodeH265ProfileEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub std_profile_idc: crate::external::vk_video::StdVideoH265ProfileIdc,
}
impl VideoEncodeH265ProfileEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::VIDEO_ENCODE_H265_PROFILE_EXT;
}
impl Default for VideoEncodeH265ProfileEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), std_profile_idc: Default::default() }
    }
}
impl std::fmt::Debug for VideoEncodeH265ProfileEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoEncodeH265ProfileEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("std_profile_idc", &self.std_profile_idc).finish()
    }
}
impl VideoEncodeH265ProfileEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoEncodeH265ProfileEXTBuilder<'a> {
        VideoEncodeH265ProfileEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH265ProfileEXT.html) · Builder of [`VideoEncodeH265ProfileEXT`]"]
#[repr(transparent)]
pub struct VideoEncodeH265ProfileEXTBuilder<'a>(VideoEncodeH265ProfileEXT, std::marker::PhantomData<&'a ()>);
impl<'a> VideoEncodeH265ProfileEXTBuilder<'a> {
    #[inline]
    pub fn new() -> VideoEncodeH265ProfileEXTBuilder<'a> {
        VideoEncodeH265ProfileEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn std_profile_idc(mut self, std_profile_idc: crate::external::vk_video::StdVideoH265ProfileIdc) -> Self {
        self.0.std_profile_idc = std_profile_idc as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> VideoEncodeH265ProfileEXT {
        self.0
    }
}
impl<'a> std::default::Default for VideoEncodeH265ProfileEXTBuilder<'a> {
    fn default() -> VideoEncodeH265ProfileEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoEncodeH265ProfileEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoEncodeH265ProfileEXTBuilder<'a> {
    type Target = VideoEncodeH265ProfileEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoEncodeH265ProfileEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH265DpbSlotInfoEXT.html) · Structure"]
#[doc(alias = "VkVideoEncodeH265DpbSlotInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoEncodeH265DpbSlotInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub slot_index: i8,
    pub p_std_reference_info: *const crate::external::vk_video::StdVideoEncodeH265ReferenceInfo,
}
impl VideoEncodeH265DpbSlotInfoEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::VIDEO_ENCODE_H265_DPB_SLOT_INFO_EXT;
}
impl Default for VideoEncodeH265DpbSlotInfoEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), slot_index: Default::default(), p_std_reference_info: std::ptr::null() }
    }
}
impl std::fmt::Debug for VideoEncodeH265DpbSlotInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoEncodeH265DpbSlotInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("slot_index", &self.slot_index).field("p_std_reference_info", &self.p_std_reference_info).finish()
    }
}
impl VideoEncodeH265DpbSlotInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoEncodeH265DpbSlotInfoEXTBuilder<'a> {
        VideoEncodeH265DpbSlotInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH265DpbSlotInfoEXT.html) · Builder of [`VideoEncodeH265DpbSlotInfoEXT`]"]
#[repr(transparent)]
pub struct VideoEncodeH265DpbSlotInfoEXTBuilder<'a>(VideoEncodeH265DpbSlotInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> VideoEncodeH265DpbSlotInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> VideoEncodeH265DpbSlotInfoEXTBuilder<'a> {
        VideoEncodeH265DpbSlotInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn slot_index(mut self, slot_index: i8) -> Self {
        self.0.slot_index = slot_index as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn std_reference_info(mut self, std_reference_info: &'a crate::external::vk_video::StdVideoEncodeH265ReferenceInfo) -> Self {
        self.0.p_std_reference_info = std_reference_info as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> VideoEncodeH265DpbSlotInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for VideoEncodeH265DpbSlotInfoEXTBuilder<'a> {
    fn default() -> VideoEncodeH265DpbSlotInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoEncodeH265DpbSlotInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoEncodeH265DpbSlotInfoEXTBuilder<'a> {
    type Target = VideoEncodeH265DpbSlotInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoEncodeH265DpbSlotInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH265ReferenceListsEXT.html) · Structure"]
#[doc(alias = "VkVideoEncodeH265ReferenceListsEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoEncodeH265ReferenceListsEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub reference_list0_entry_count: u8,
    pub p_reference_list0_entries: *const crate::extensions::ext_video_encode_h265::VideoEncodeH265DpbSlotInfoEXT,
    pub reference_list1_entry_count: u8,
    pub p_reference_list1_entries: *const crate::extensions::ext_video_encode_h265::VideoEncodeH265DpbSlotInfoEXT,
    pub p_reference_modifications: *const crate::external::vk_video::StdVideoEncodeH265ReferenceModifications,
}
impl VideoEncodeH265ReferenceListsEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::VIDEO_ENCODE_H265_REFERENCE_LISTS_EXT;
}
impl Default for VideoEncodeH265ReferenceListsEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), reference_list0_entry_count: Default::default(), p_reference_list0_entries: std::ptr::null(), reference_list1_entry_count: Default::default(), p_reference_list1_entries: std::ptr::null(), p_reference_modifications: std::ptr::null() }
    }
}
impl std::fmt::Debug for VideoEncodeH265ReferenceListsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoEncodeH265ReferenceListsEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("reference_list0_entry_count", &self.reference_list0_entry_count).field("p_reference_list0_entries", &self.p_reference_list0_entries).field("reference_list1_entry_count", &self.reference_list1_entry_count).field("p_reference_list1_entries", &self.p_reference_list1_entries).field("p_reference_modifications", &self.p_reference_modifications).finish()
    }
}
impl VideoEncodeH265ReferenceListsEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoEncodeH265ReferenceListsEXTBuilder<'a> {
        VideoEncodeH265ReferenceListsEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH265ReferenceListsEXT.html) · Builder of [`VideoEncodeH265ReferenceListsEXT`]"]
#[repr(transparent)]
pub struct VideoEncodeH265ReferenceListsEXTBuilder<'a>(VideoEncodeH265ReferenceListsEXT, std::marker::PhantomData<&'a ()>);
impl<'a> VideoEncodeH265ReferenceListsEXTBuilder<'a> {
    #[inline]
    pub fn new() -> VideoEncodeH265ReferenceListsEXTBuilder<'a> {
        VideoEncodeH265ReferenceListsEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn reference_list0_entries(mut self, reference_list0_entries: &'a [crate::extensions::ext_video_encode_h265::VideoEncodeH265DpbSlotInfoEXTBuilder]) -> Self {
        self.0.p_reference_list0_entries = reference_list0_entries.as_ptr() as _;
        self.0.reference_list0_entry_count = reference_list0_entries.len() as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn reference_list1_entries(mut self, reference_list1_entries: &'a [crate::extensions::ext_video_encode_h265::VideoEncodeH265DpbSlotInfoEXTBuilder]) -> Self {
        self.0.p_reference_list1_entries = reference_list1_entries.as_ptr() as _;
        self.0.reference_list1_entry_count = reference_list1_entries.len() as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn reference_modifications(mut self, reference_modifications: &'a crate::external::vk_video::StdVideoEncodeH265ReferenceModifications) -> Self {
        self.0.p_reference_modifications = reference_modifications as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> VideoEncodeH265ReferenceListsEXT {
        self.0
    }
}
impl<'a> std::default::Default for VideoEncodeH265ReferenceListsEXTBuilder<'a> {
    fn default() -> VideoEncodeH265ReferenceListsEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoEncodeH265ReferenceListsEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoEncodeH265ReferenceListsEXTBuilder<'a> {
    type Target = VideoEncodeH265ReferenceListsEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoEncodeH265ReferenceListsEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
