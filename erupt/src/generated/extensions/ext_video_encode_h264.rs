//! ## Versioning Warning ⚠️
//!
//! This is a Vulkan **provisional/beta** extension and **must** be used with
//! caution. Its API/behaviour has not been finalized yet and _may_ therefore
//! change in ways that break backwards compatibility between revisions, and
//! before final release of a non-provisional version of this extension.
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_VIDEO_ENCODE_H264_SPEC_VERSION")]
pub const EXT_VIDEO_ENCODE_H264_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_VIDEO_ENCODE_H264_EXTENSION_NAME")]
pub const EXT_VIDEO_ENCODE_H264_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_video_encode_h264");
#[doc = "Provided by [`crate::extensions::ext_video_encode_h264`]"]
impl crate::vk1_0::StructureType {
    pub const VIDEO_ENCODE_H264_CAPABILITIES_EXT: Self = Self(1000038000);
    pub const VIDEO_ENCODE_H264_SESSION_CREATE_INFO_EXT: Self = Self(1000038001);
    pub const VIDEO_ENCODE_H264_SESSION_PARAMETERS_CREATE_INFO_EXT: Self = Self(1000038002);
    pub const VIDEO_ENCODE_H264_SESSION_PARAMETERS_ADD_INFO_EXT: Self = Self(1000038003);
    pub const VIDEO_ENCODE_H264_VCL_FRAME_INFO_EXT: Self = Self(1000038004);
    pub const VIDEO_ENCODE_H264_DPB_SLOT_INFO_EXT: Self = Self(1000038005);
    pub const VIDEO_ENCODE_H264_NALU_SLICE_EXT: Self = Self(1000038006);
    pub const VIDEO_ENCODE_H264_EMIT_PICTURE_PARAMETERS_EXT: Self = Self(1000038007);
    pub const VIDEO_ENCODE_H264_PROFILE_EXT: Self = Self(1000038008);
    pub const VIDEO_ENCODE_H264_RATE_CONTROL_INFO_EXT: Self = Self(1000038009);
    pub const VIDEO_ENCODE_H264_RATE_CONTROL_LAYER_INFO_EXT: Self = Self(1000038010);
}
#[doc = "Provided by [`crate::extensions::ext_video_encode_h264`]"]
impl crate::extensions::khr_video_queue::VideoCodecOperationFlagBitsKHR {
    pub const ENCODE_H264_EXT: Self = Self(65536);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH264CapabilityFlagsEXT.html) · Bitmask of [`VideoEncodeH264CapabilityFlagBitsEXT`]"] # [doc (alias = "VkVideoEncodeH264CapabilityFlagsEXT")] # [derive (Default)] # [repr (transparent)] pub struct VideoEncodeH264CapabilityFlagsEXT : u32 { const CABAC_EXT = VideoEncodeH264CapabilityFlagBitsEXT :: CABAC_EXT . 0 ; const CAVLC_EXT = VideoEncodeH264CapabilityFlagBitsEXT :: CAVLC_EXT . 0 ; const WEIGHTED_BI_PRED_IMPLICIT_EXT = VideoEncodeH264CapabilityFlagBitsEXT :: WEIGHTED_BI_PRED_IMPLICIT_EXT . 0 ; const TRANSFORM_8X8_EXT = VideoEncodeH264CapabilityFlagBitsEXT :: TRANSFORM_8X8_EXT . 0 ; const CHROMA_QP_OFFSET_EXT = VideoEncodeH264CapabilityFlagBitsEXT :: CHROMA_QP_OFFSET_EXT . 0 ; const SECOND_CHROMA_QP_OFFSET_EXT = VideoEncodeH264CapabilityFlagBitsEXT :: SECOND_CHROMA_QP_OFFSET_EXT . 0 ; const DEBLOCKING_FILTER_DISABLED_EXT = VideoEncodeH264CapabilityFlagBitsEXT :: DEBLOCKING_FILTER_DISABLED_EXT . 0 ; const DEBLOCKING_FILTER_ENABLED_EXT = VideoEncodeH264CapabilityFlagBitsEXT :: DEBLOCKING_FILTER_ENABLED_EXT . 0 ; const DEBLOCKING_FILTER_PARTIAL_EXT = VideoEncodeH264CapabilityFlagBitsEXT :: DEBLOCKING_FILTER_PARTIAL_EXT . 0 ; const MULTIPLE_SLICE_PER_FRAME_EXT = VideoEncodeH264CapabilityFlagBitsEXT :: MULTIPLE_SLICE_PER_FRAME_EXT . 0 ; const EVENLY_DISTRIBUTED_SLICE_SIZE_EXT = VideoEncodeH264CapabilityFlagBitsEXT :: EVENLY_DISTRIBUTED_SLICE_SIZE_EXT . 0 ; const OPTIONAL_RC_EXTENSION_STRUCT_EXT = VideoEncodeH264CapabilityFlagBitsEXT :: OPTIONAL_RC_EXTENSION_STRUCT_EXT . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH264CapabilityFlagBitsEXT.html) · Bits enum of [`VideoEncodeH264CapabilityFlagsEXT`]"]
#[doc(alias = "VkVideoEncodeH264CapabilityFlagBitsEXT")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct VideoEncodeH264CapabilityFlagBitsEXT(pub u32);
impl VideoEncodeH264CapabilityFlagBitsEXT {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> VideoEncodeH264CapabilityFlagsEXT {
        VideoEncodeH264CapabilityFlagsEXT::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for VideoEncodeH264CapabilityFlagBitsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::CABAC_EXT => "CABAC_EXT",
            &Self::CAVLC_EXT => "CAVLC_EXT",
            &Self::WEIGHTED_BI_PRED_IMPLICIT_EXT => "WEIGHTED_BI_PRED_IMPLICIT_EXT",
            &Self::TRANSFORM_8X8_EXT => "TRANSFORM_8X8_EXT",
            &Self::CHROMA_QP_OFFSET_EXT => "CHROMA_QP_OFFSET_EXT",
            &Self::SECOND_CHROMA_QP_OFFSET_EXT => "SECOND_CHROMA_QP_OFFSET_EXT",
            &Self::DEBLOCKING_FILTER_DISABLED_EXT => "DEBLOCKING_FILTER_DISABLED_EXT",
            &Self::DEBLOCKING_FILTER_ENABLED_EXT => "DEBLOCKING_FILTER_ENABLED_EXT",
            &Self::DEBLOCKING_FILTER_PARTIAL_EXT => "DEBLOCKING_FILTER_PARTIAL_EXT",
            &Self::MULTIPLE_SLICE_PER_FRAME_EXT => "MULTIPLE_SLICE_PER_FRAME_EXT",
            &Self::EVENLY_DISTRIBUTED_SLICE_SIZE_EXT => "EVENLY_DISTRIBUTED_SLICE_SIZE_EXT",
            &Self::OPTIONAL_RC_EXTENSION_STRUCT_EXT => "OPTIONAL_RC_EXTENSION_STRUCT_EXT",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::ext_video_encode_h264`]"]
impl crate::extensions::ext_video_encode_h264::VideoEncodeH264CapabilityFlagBitsEXT {
    pub const CABAC_EXT: Self = Self(1);
    pub const CAVLC_EXT: Self = Self(2);
    pub const WEIGHTED_BI_PRED_IMPLICIT_EXT: Self = Self(4);
    pub const TRANSFORM_8X8_EXT: Self = Self(8);
    pub const CHROMA_QP_OFFSET_EXT: Self = Self(16);
    pub const SECOND_CHROMA_QP_OFFSET_EXT: Self = Self(32);
    pub const DEBLOCKING_FILTER_DISABLED_EXT: Self = Self(64);
    pub const DEBLOCKING_FILTER_ENABLED_EXT: Self = Self(128);
    pub const DEBLOCKING_FILTER_PARTIAL_EXT: Self = Self(256);
    pub const MULTIPLE_SLICE_PER_FRAME_EXT: Self = Self(512);
    pub const EVENLY_DISTRIBUTED_SLICE_SIZE_EXT: Self = Self(1024);
    pub const OPTIONAL_RC_EXTENSION_STRUCT_EXT: Self = Self(2048);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH264InputModeFlagsEXT.html) · Bitmask of [`VideoEncodeH264InputModeFlagBitsEXT`]"] # [doc (alias = "VkVideoEncodeH264InputModeFlagsEXT")] # [derive (Default)] # [repr (transparent)] pub struct VideoEncodeH264InputModeFlagsEXT : u32 { const FRAME_EXT = VideoEncodeH264InputModeFlagBitsEXT :: FRAME_EXT . 0 ; const SLICE_EXT = VideoEncodeH264InputModeFlagBitsEXT :: SLICE_EXT . 0 ; const NON_VCL_EXT = VideoEncodeH264InputModeFlagBitsEXT :: NON_VCL_EXT . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH264InputModeFlagBitsEXT.html) · Bits enum of [`VideoEncodeH264InputModeFlagsEXT`]"]
#[doc(alias = "VkVideoEncodeH264InputModeFlagBitsEXT")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct VideoEncodeH264InputModeFlagBitsEXT(pub u32);
impl VideoEncodeH264InputModeFlagBitsEXT {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> VideoEncodeH264InputModeFlagsEXT {
        VideoEncodeH264InputModeFlagsEXT::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for VideoEncodeH264InputModeFlagBitsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::FRAME_EXT => "FRAME_EXT",
            &Self::SLICE_EXT => "SLICE_EXT",
            &Self::NON_VCL_EXT => "NON_VCL_EXT",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::ext_video_encode_h264`]"]
impl crate::extensions::ext_video_encode_h264::VideoEncodeH264InputModeFlagBitsEXT {
    pub const FRAME_EXT: Self = Self(1);
    pub const SLICE_EXT: Self = Self(2);
    pub const NON_VCL_EXT: Self = Self(4);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH264OutputModeFlagsEXT.html) · Bitmask of [`VideoEncodeH264OutputModeFlagBitsEXT`]"] # [doc (alias = "VkVideoEncodeH264OutputModeFlagsEXT")] # [derive (Default)] # [repr (transparent)] pub struct VideoEncodeH264OutputModeFlagsEXT : u32 { const FRAME_EXT = VideoEncodeH264OutputModeFlagBitsEXT :: FRAME_EXT . 0 ; const SLICE_EXT = VideoEncodeH264OutputModeFlagBitsEXT :: SLICE_EXT . 0 ; const NON_VCL_EXT = VideoEncodeH264OutputModeFlagBitsEXT :: NON_VCL_EXT . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH264OutputModeFlagBitsEXT.html) · Bits enum of [`VideoEncodeH264OutputModeFlagsEXT`]"]
#[doc(alias = "VkVideoEncodeH264OutputModeFlagBitsEXT")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct VideoEncodeH264OutputModeFlagBitsEXT(pub u32);
impl VideoEncodeH264OutputModeFlagBitsEXT {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> VideoEncodeH264OutputModeFlagsEXT {
        VideoEncodeH264OutputModeFlagsEXT::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for VideoEncodeH264OutputModeFlagBitsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::FRAME_EXT => "FRAME_EXT",
            &Self::SLICE_EXT => "SLICE_EXT",
            &Self::NON_VCL_EXT => "NON_VCL_EXT",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::ext_video_encode_h264`]"]
impl crate::extensions::ext_video_encode_h264::VideoEncodeH264OutputModeFlagBitsEXT {
    pub const FRAME_EXT: Self = Self(1);
    pub const SLICE_EXT: Self = Self(2);
    pub const NON_VCL_EXT: Self = Self(4);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH264CreateFlagsEXT.html) · Bitmask of [`VideoEncodeH264CreateFlagBitsEXT`]"] # [doc (alias = "VkVideoEncodeH264CreateFlagsEXT")] # [derive (Default)] # [repr (transparent)] pub struct VideoEncodeH264CreateFlagsEXT : u32 { const DEFAULT_EXT = VideoEncodeH264CreateFlagBitsEXT :: DEFAULT_EXT . 0 ; const RESERVED_0_EXT = VideoEncodeH264CreateFlagBitsEXT :: RESERVED_0_EXT . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH264CreateFlagBitsEXT.html) · Bits enum of [`VideoEncodeH264CreateFlagsEXT`]"]
#[doc(alias = "VkVideoEncodeH264CreateFlagBitsEXT")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct VideoEncodeH264CreateFlagBitsEXT(pub u32);
impl VideoEncodeH264CreateFlagBitsEXT {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> VideoEncodeH264CreateFlagsEXT {
        VideoEncodeH264CreateFlagsEXT::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for VideoEncodeH264CreateFlagBitsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::DEFAULT_EXT => "DEFAULT_EXT",
            &Self::RESERVED_0_EXT => "RESERVED_0_EXT",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::ext_video_encode_h264`]"]
impl crate::extensions::ext_video_encode_h264::VideoEncodeH264CreateFlagBitsEXT {
    pub const DEFAULT_EXT: Self = Self(0);
    pub const RESERVED_0_EXT: Self = Self(1);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH264RateControlStructureFlagsEXT.html) · Bitmask of [`VideoEncodeH264RateControlStructureFlagBitsEXT`]"] # [doc (alias = "VkVideoEncodeH264RateControlStructureFlagsEXT")] # [derive (Default)] # [repr (transparent)] pub struct VideoEncodeH264RateControlStructureFlagsEXT : u32 { const UNKNOWN_EXT = VideoEncodeH264RateControlStructureFlagBitsEXT :: UNKNOWN_EXT . 0 ; const FLAT_EXT = VideoEncodeH264RateControlStructureFlagBitsEXT :: FLAT_EXT . 0 ; const DYADIC_EXT = VideoEncodeH264RateControlStructureFlagBitsEXT :: DYADIC_EXT . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH264RateControlStructureFlagBitsEXT.html) · Bits enum of [`VideoEncodeH264RateControlStructureFlagsEXT`]"]
#[doc(alias = "VkVideoEncodeH264RateControlStructureFlagBitsEXT")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct VideoEncodeH264RateControlStructureFlagBitsEXT(pub u32);
impl VideoEncodeH264RateControlStructureFlagBitsEXT {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> VideoEncodeH264RateControlStructureFlagsEXT {
        VideoEncodeH264RateControlStructureFlagsEXT::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for VideoEncodeH264RateControlStructureFlagBitsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::UNKNOWN_EXT => "UNKNOWN_EXT",
            &Self::FLAT_EXT => "FLAT_EXT",
            &Self::DYADIC_EXT => "DYADIC_EXT",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::ext_video_encode_h264`]"]
impl crate::extensions::ext_video_encode_h264::VideoEncodeH264RateControlStructureFlagBitsEXT {
    pub const UNKNOWN_EXT: Self = Self(0);
    pub const FLAT_EXT: Self = Self(1);
    pub const DYADIC_EXT: Self = Self(2);
}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH264ProfileEXT> for crate::vk1_0::BufferCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH264ProfileEXTBuilder<'_>> for crate::vk1_0::BufferCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH264ProfileEXT> for crate::vk1_0::ImageCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH264ProfileEXTBuilder<'_>> for crate::vk1_0::ImageCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH264ProfileEXT> for crate::vk1_0::ImageViewCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH264ProfileEXTBuilder<'_>> for crate::vk1_0::ImageViewCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH264ProfileEXT> for crate::vk1_0::QueryPoolCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH264ProfileEXTBuilder<'_>> for crate::vk1_0::QueryPoolCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH264ProfileEXT> for crate::vk1_1::FormatProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH264ProfileEXTBuilder<'_>> for crate::vk1_1::FormatProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH264ProfileEXT> for crate::extensions::khr_video_queue::VideoProfileKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH264ProfileEXTBuilder<'_>> for crate::extensions::khr_video_queue::VideoProfileKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH264CapabilitiesEXT> for crate::extensions::khr_video_queue::VideoCapabilitiesKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH264CapabilitiesEXTBuilder<'_>> for crate::extensions::khr_video_queue::VideoCapabilitiesKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH264SessionCreateInfoEXT> for crate::extensions::khr_video_queue::VideoSessionCreateInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH264SessionCreateInfoEXTBuilder<'_>> for crate::extensions::khr_video_queue::VideoSessionCreateInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH264SessionParametersCreateInfoEXT> for crate::extensions::khr_video_queue::VideoSessionParametersCreateInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH264SessionParametersCreateInfoEXTBuilder<'_>> for crate::extensions::khr_video_queue::VideoSessionParametersCreateInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH264SessionParametersAddInfoEXT> for crate::extensions::khr_video_queue::VideoSessionParametersUpdateInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH264SessionParametersAddInfoEXTBuilder<'_>> for crate::extensions::khr_video_queue::VideoSessionParametersUpdateInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH264VclFrameInfoEXT> for crate::extensions::khr_video_encode_queue::VideoEncodeInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH264VclFrameInfoEXTBuilder<'_>> for crate::extensions::khr_video_encode_queue::VideoEncodeInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH264EmitPictureParametersEXT> for crate::extensions::khr_video_encode_queue::VideoEncodeInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH264EmitPictureParametersEXTBuilder<'_>> for crate::extensions::khr_video_encode_queue::VideoEncodeInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH264RateControlInfoEXT> for crate::extensions::khr_video_encode_queue::VideoEncodeRateControlInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH264RateControlInfoEXTBuilder<'_>> for crate::extensions::khr_video_encode_queue::VideoEncodeRateControlInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH264RateControlLayerInfoEXT> for crate::extensions::khr_video_encode_queue::VideoEncodeRateControlLayerInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeH264RateControlLayerInfoEXTBuilder<'_>> for crate::extensions::khr_video_encode_queue::VideoEncodeRateControlLayerInfoKHRBuilder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH264CapabilitiesEXT.html) · Structure"]
#[doc(alias = "VkVideoEncodeH264CapabilitiesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoEncodeH264CapabilitiesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::ext_video_encode_h264::VideoEncodeH264CapabilityFlagsEXT,
    pub input_mode_flags: crate::extensions::ext_video_encode_h264::VideoEncodeH264InputModeFlagsEXT,
    pub output_mode_flags: crate::extensions::ext_video_encode_h264::VideoEncodeH264OutputModeFlagsEXT,
    pub min_picture_size_in_mbs: crate::vk1_0::Extent2D,
    pub max_picture_size_in_mbs: crate::vk1_0::Extent2D,
    pub input_image_data_alignment: crate::vk1_0::Extent2D,
    pub max_num_l0_reference_for_p: u8,
    pub max_num_l0_reference_for_b: u8,
    pub max_num_l1_reference: u8,
    pub quality_level_count: u8,
    pub std_extension_version: crate::vk1_0::ExtensionProperties,
}
impl VideoEncodeH264CapabilitiesEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::VIDEO_ENCODE_H264_CAPABILITIES_EXT;
}
impl Default for VideoEncodeH264CapabilitiesEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), flags: Default::default(), input_mode_flags: Default::default(), output_mode_flags: Default::default(), min_picture_size_in_mbs: Default::default(), max_picture_size_in_mbs: Default::default(), input_image_data_alignment: Default::default(), max_num_l0_reference_for_p: Default::default(), max_num_l0_reference_for_b: Default::default(), max_num_l1_reference: Default::default(), quality_level_count: Default::default(), std_extension_version: Default::default() }
    }
}
impl std::fmt::Debug for VideoEncodeH264CapabilitiesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoEncodeH264CapabilitiesEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("input_mode_flags", &self.input_mode_flags).field("output_mode_flags", &self.output_mode_flags).field("min_picture_size_in_mbs", &self.min_picture_size_in_mbs).field("max_picture_size_in_mbs", &self.max_picture_size_in_mbs).field("input_image_data_alignment", &self.input_image_data_alignment).field("max_num_l0_reference_for_p", &self.max_num_l0_reference_for_p).field("max_num_l0_reference_for_b", &self.max_num_l0_reference_for_b).field("max_num_l1_reference", &self.max_num_l1_reference).field("quality_level_count", &self.quality_level_count).field("std_extension_version", &self.std_extension_version).finish()
    }
}
impl VideoEncodeH264CapabilitiesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoEncodeH264CapabilitiesEXTBuilder<'a> {
        VideoEncodeH264CapabilitiesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH264CapabilitiesEXT.html) · Builder of [`VideoEncodeH264CapabilitiesEXT`]"]
#[repr(transparent)]
pub struct VideoEncodeH264CapabilitiesEXTBuilder<'a>(VideoEncodeH264CapabilitiesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> VideoEncodeH264CapabilitiesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> VideoEncodeH264CapabilitiesEXTBuilder<'a> {
        VideoEncodeH264CapabilitiesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::ext_video_encode_h264::VideoEncodeH264CapabilityFlagsEXT) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn input_mode_flags(mut self, input_mode_flags: crate::extensions::ext_video_encode_h264::VideoEncodeH264InputModeFlagsEXT) -> Self {
        self.0.input_mode_flags = input_mode_flags as _;
        self
    }
    #[inline]
    pub fn output_mode_flags(mut self, output_mode_flags: crate::extensions::ext_video_encode_h264::VideoEncodeH264OutputModeFlagsEXT) -> Self {
        self.0.output_mode_flags = output_mode_flags as _;
        self
    }
    #[inline]
    pub fn min_picture_size_in_mbs(mut self, min_picture_size_in_mbs: crate::vk1_0::Extent2D) -> Self {
        self.0.min_picture_size_in_mbs = min_picture_size_in_mbs as _;
        self
    }
    #[inline]
    pub fn max_picture_size_in_mbs(mut self, max_picture_size_in_mbs: crate::vk1_0::Extent2D) -> Self {
        self.0.max_picture_size_in_mbs = max_picture_size_in_mbs as _;
        self
    }
    #[inline]
    pub fn input_image_data_alignment(mut self, input_image_data_alignment: crate::vk1_0::Extent2D) -> Self {
        self.0.input_image_data_alignment = input_image_data_alignment as _;
        self
    }
    #[inline]
    pub fn max_num_l0_reference_for_p(mut self, max_num_l0_reference_for_p: u8) -> Self {
        self.0.max_num_l0_reference_for_p = max_num_l0_reference_for_p as _;
        self
    }
    #[inline]
    pub fn max_num_l0_reference_for_b(mut self, max_num_l0_reference_for_b: u8) -> Self {
        self.0.max_num_l0_reference_for_b = max_num_l0_reference_for_b as _;
        self
    }
    #[inline]
    pub fn max_num_l1_reference(mut self, max_num_l1_reference: u8) -> Self {
        self.0.max_num_l1_reference = max_num_l1_reference as _;
        self
    }
    #[inline]
    pub fn quality_level_count(mut self, quality_level_count: u8) -> Self {
        self.0.quality_level_count = quality_level_count as _;
        self
    }
    #[inline]
    pub fn std_extension_version(mut self, std_extension_version: crate::vk1_0::ExtensionProperties) -> Self {
        self.0.std_extension_version = std_extension_version as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> VideoEncodeH264CapabilitiesEXT {
        self.0
    }
}
impl<'a> std::default::Default for VideoEncodeH264CapabilitiesEXTBuilder<'a> {
    fn default() -> VideoEncodeH264CapabilitiesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoEncodeH264CapabilitiesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoEncodeH264CapabilitiesEXTBuilder<'a> {
    type Target = VideoEncodeH264CapabilitiesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoEncodeH264CapabilitiesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH264SessionCreateInfoEXT.html) · Structure"]
#[doc(alias = "VkVideoEncodeH264SessionCreateInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoEncodeH264SessionCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::ext_video_encode_h264::VideoEncodeH264CreateFlagsEXT,
    pub max_picture_size_in_mbs: crate::vk1_0::Extent2D,
    pub p_std_extension_version: *const crate::vk1_0::ExtensionProperties,
}
impl VideoEncodeH264SessionCreateInfoEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::VIDEO_ENCODE_H264_SESSION_CREATE_INFO_EXT;
}
impl Default for VideoEncodeH264SessionCreateInfoEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), flags: Default::default(), max_picture_size_in_mbs: Default::default(), p_std_extension_version: std::ptr::null() }
    }
}
impl std::fmt::Debug for VideoEncodeH264SessionCreateInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoEncodeH264SessionCreateInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("max_picture_size_in_mbs", &self.max_picture_size_in_mbs).field("p_std_extension_version", &self.p_std_extension_version).finish()
    }
}
impl VideoEncodeH264SessionCreateInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoEncodeH264SessionCreateInfoEXTBuilder<'a> {
        VideoEncodeH264SessionCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH264SessionCreateInfoEXT.html) · Builder of [`VideoEncodeH264SessionCreateInfoEXT`]"]
#[repr(transparent)]
pub struct VideoEncodeH264SessionCreateInfoEXTBuilder<'a>(VideoEncodeH264SessionCreateInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> VideoEncodeH264SessionCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> VideoEncodeH264SessionCreateInfoEXTBuilder<'a> {
        VideoEncodeH264SessionCreateInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::ext_video_encode_h264::VideoEncodeH264CreateFlagsEXT) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn max_picture_size_in_mbs(mut self, max_picture_size_in_mbs: crate::vk1_0::Extent2D) -> Self {
        self.0.max_picture_size_in_mbs = max_picture_size_in_mbs as _;
        self
    }
    #[inline]
    pub fn std_extension_version(mut self, std_extension_version: &'a crate::vk1_0::ExtensionProperties) -> Self {
        self.0.p_std_extension_version = std_extension_version as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> VideoEncodeH264SessionCreateInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for VideoEncodeH264SessionCreateInfoEXTBuilder<'a> {
    fn default() -> VideoEncodeH264SessionCreateInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoEncodeH264SessionCreateInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoEncodeH264SessionCreateInfoEXTBuilder<'a> {
    type Target = VideoEncodeH264SessionCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoEncodeH264SessionCreateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH264SessionParametersAddInfoEXT.html) · Structure"]
#[doc(alias = "VkVideoEncodeH264SessionParametersAddInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoEncodeH264SessionParametersAddInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub sps_std_count: u32,
    pub p_sps_std: *const crate::external::vk_video::StdVideoH264SequenceParameterSet,
    pub pps_std_count: u32,
    pub p_pps_std: *const crate::external::vk_video::StdVideoH264PictureParameterSet,
}
impl VideoEncodeH264SessionParametersAddInfoEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::VIDEO_ENCODE_H264_SESSION_PARAMETERS_ADD_INFO_EXT;
}
impl Default for VideoEncodeH264SessionParametersAddInfoEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), sps_std_count: Default::default(), p_sps_std: std::ptr::null(), pps_std_count: Default::default(), p_pps_std: std::ptr::null() }
    }
}
impl std::fmt::Debug for VideoEncodeH264SessionParametersAddInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoEncodeH264SessionParametersAddInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("sps_std_count", &self.sps_std_count).field("p_sps_std", &self.p_sps_std).field("pps_std_count", &self.pps_std_count).field("p_pps_std", &self.p_pps_std).finish()
    }
}
impl VideoEncodeH264SessionParametersAddInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoEncodeH264SessionParametersAddInfoEXTBuilder<'a> {
        VideoEncodeH264SessionParametersAddInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH264SessionParametersAddInfoEXT.html) · Builder of [`VideoEncodeH264SessionParametersAddInfoEXT`]"]
#[repr(transparent)]
pub struct VideoEncodeH264SessionParametersAddInfoEXTBuilder<'a>(VideoEncodeH264SessionParametersAddInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> VideoEncodeH264SessionParametersAddInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> VideoEncodeH264SessionParametersAddInfoEXTBuilder<'a> {
        VideoEncodeH264SessionParametersAddInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn sps_std(mut self, sps_std: &'a [crate::external::vk_video::StdVideoH264SequenceParameterSetBuilder]) -> Self {
        self.0.p_sps_std = sps_std.as_ptr() as _;
        self.0.sps_std_count = sps_std.len() as _;
        self
    }
    #[inline]
    pub fn pps_std(mut self, pps_std: &'a [crate::external::vk_video::StdVideoH264PictureParameterSetBuilder]) -> Self {
        self.0.p_pps_std = pps_std.as_ptr() as _;
        self.0.pps_std_count = pps_std.len() as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> VideoEncodeH264SessionParametersAddInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for VideoEncodeH264SessionParametersAddInfoEXTBuilder<'a> {
    fn default() -> VideoEncodeH264SessionParametersAddInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoEncodeH264SessionParametersAddInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoEncodeH264SessionParametersAddInfoEXTBuilder<'a> {
    type Target = VideoEncodeH264SessionParametersAddInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoEncodeH264SessionParametersAddInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH264SessionParametersCreateInfoEXT.html) · Structure"]
#[doc(alias = "VkVideoEncodeH264SessionParametersCreateInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoEncodeH264SessionParametersCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub max_sps_std_count: u32,
    pub max_pps_std_count: u32,
    pub p_parameters_add_info: *const crate::extensions::ext_video_encode_h264::VideoEncodeH264SessionParametersAddInfoEXT,
}
impl VideoEncodeH264SessionParametersCreateInfoEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::VIDEO_ENCODE_H264_SESSION_PARAMETERS_CREATE_INFO_EXT;
}
impl Default for VideoEncodeH264SessionParametersCreateInfoEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), max_sps_std_count: Default::default(), max_pps_std_count: Default::default(), p_parameters_add_info: std::ptr::null() }
    }
}
impl std::fmt::Debug for VideoEncodeH264SessionParametersCreateInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoEncodeH264SessionParametersCreateInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("max_sps_std_count", &self.max_sps_std_count).field("max_pps_std_count", &self.max_pps_std_count).field("p_parameters_add_info", &self.p_parameters_add_info).finish()
    }
}
impl VideoEncodeH264SessionParametersCreateInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoEncodeH264SessionParametersCreateInfoEXTBuilder<'a> {
        VideoEncodeH264SessionParametersCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH264SessionParametersCreateInfoEXT.html) · Builder of [`VideoEncodeH264SessionParametersCreateInfoEXT`]"]
#[repr(transparent)]
pub struct VideoEncodeH264SessionParametersCreateInfoEXTBuilder<'a>(VideoEncodeH264SessionParametersCreateInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> VideoEncodeH264SessionParametersCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> VideoEncodeH264SessionParametersCreateInfoEXTBuilder<'a> {
        VideoEncodeH264SessionParametersCreateInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn max_sps_std_count(mut self, max_sps_std_count: u32) -> Self {
        self.0.max_sps_std_count = max_sps_std_count as _;
        self
    }
    #[inline]
    pub fn max_pps_std_count(mut self, max_pps_std_count: u32) -> Self {
        self.0.max_pps_std_count = max_pps_std_count as _;
        self
    }
    #[inline]
    pub fn parameters_add_info(mut self, parameters_add_info: &'a crate::extensions::ext_video_encode_h264::VideoEncodeH264SessionParametersAddInfoEXT) -> Self {
        self.0.p_parameters_add_info = parameters_add_info as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> VideoEncodeH264SessionParametersCreateInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for VideoEncodeH264SessionParametersCreateInfoEXTBuilder<'a> {
    fn default() -> VideoEncodeH264SessionParametersCreateInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoEncodeH264SessionParametersCreateInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoEncodeH264SessionParametersCreateInfoEXTBuilder<'a> {
    type Target = VideoEncodeH264SessionParametersCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoEncodeH264SessionParametersCreateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH264DpbSlotInfoEXT.html) · Structure"]
#[doc(alias = "VkVideoEncodeH264DpbSlotInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoEncodeH264DpbSlotInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub slot_index: i8,
    pub p_std_picture_info: *const crate::external::vk_video::StdVideoEncodeH264PictureInfo,
}
impl VideoEncodeH264DpbSlotInfoEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::VIDEO_ENCODE_H264_DPB_SLOT_INFO_EXT;
}
impl Default for VideoEncodeH264DpbSlotInfoEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), slot_index: Default::default(), p_std_picture_info: std::ptr::null() }
    }
}
impl std::fmt::Debug for VideoEncodeH264DpbSlotInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoEncodeH264DpbSlotInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("slot_index", &self.slot_index).field("p_std_picture_info", &self.p_std_picture_info).finish()
    }
}
impl VideoEncodeH264DpbSlotInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoEncodeH264DpbSlotInfoEXTBuilder<'a> {
        VideoEncodeH264DpbSlotInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH264DpbSlotInfoEXT.html) · Builder of [`VideoEncodeH264DpbSlotInfoEXT`]"]
#[repr(transparent)]
pub struct VideoEncodeH264DpbSlotInfoEXTBuilder<'a>(VideoEncodeH264DpbSlotInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> VideoEncodeH264DpbSlotInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> VideoEncodeH264DpbSlotInfoEXTBuilder<'a> {
        VideoEncodeH264DpbSlotInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn slot_index(mut self, slot_index: i8) -> Self {
        self.0.slot_index = slot_index as _;
        self
    }
    #[inline]
    pub fn std_picture_info(mut self, std_picture_info: &'a crate::external::vk_video::StdVideoEncodeH264PictureInfo) -> Self {
        self.0.p_std_picture_info = std_picture_info as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> VideoEncodeH264DpbSlotInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for VideoEncodeH264DpbSlotInfoEXTBuilder<'a> {
    fn default() -> VideoEncodeH264DpbSlotInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoEncodeH264DpbSlotInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoEncodeH264DpbSlotInfoEXTBuilder<'a> {
    type Target = VideoEncodeH264DpbSlotInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoEncodeH264DpbSlotInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH264VclFrameInfoEXT.html) · Structure"]
#[doc(alias = "VkVideoEncodeH264VclFrameInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoEncodeH264VclFrameInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub ref_default_final_list0_entry_count: u8,
    pub p_ref_default_final_list0_entries: *const crate::extensions::ext_video_encode_h264::VideoEncodeH264DpbSlotInfoEXT,
    pub ref_default_final_list1_entry_count: u8,
    pub p_ref_default_final_list1_entries: *const crate::extensions::ext_video_encode_h264::VideoEncodeH264DpbSlotInfoEXT,
    pub nalu_slice_entry_count: u32,
    pub p_nalu_slice_entries: *const crate::extensions::ext_video_encode_h264::VideoEncodeH264NaluSliceEXT,
    pub p_current_picture_info: *const crate::extensions::ext_video_encode_h264::VideoEncodeH264DpbSlotInfoEXT,
}
impl VideoEncodeH264VclFrameInfoEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::VIDEO_ENCODE_H264_VCL_FRAME_INFO_EXT;
}
impl Default for VideoEncodeH264VclFrameInfoEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), ref_default_final_list0_entry_count: Default::default(), p_ref_default_final_list0_entries: std::ptr::null(), ref_default_final_list1_entry_count: Default::default(), p_ref_default_final_list1_entries: std::ptr::null(), nalu_slice_entry_count: Default::default(), p_nalu_slice_entries: std::ptr::null(), p_current_picture_info: std::ptr::null() }
    }
}
impl std::fmt::Debug for VideoEncodeH264VclFrameInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoEncodeH264VclFrameInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("ref_default_final_list0_entry_count", &self.ref_default_final_list0_entry_count).field("p_ref_default_final_list0_entries", &self.p_ref_default_final_list0_entries).field("ref_default_final_list1_entry_count", &self.ref_default_final_list1_entry_count).field("p_ref_default_final_list1_entries", &self.p_ref_default_final_list1_entries).field("nalu_slice_entry_count", &self.nalu_slice_entry_count).field("p_nalu_slice_entries", &self.p_nalu_slice_entries).field("p_current_picture_info", &self.p_current_picture_info).finish()
    }
}
impl VideoEncodeH264VclFrameInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoEncodeH264VclFrameInfoEXTBuilder<'a> {
        VideoEncodeH264VclFrameInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH264VclFrameInfoEXT.html) · Builder of [`VideoEncodeH264VclFrameInfoEXT`]"]
#[repr(transparent)]
pub struct VideoEncodeH264VclFrameInfoEXTBuilder<'a>(VideoEncodeH264VclFrameInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> VideoEncodeH264VclFrameInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> VideoEncodeH264VclFrameInfoEXTBuilder<'a> {
        VideoEncodeH264VclFrameInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn ref_default_final_list0_entries(mut self, ref_default_final_list0_entries: &'a [crate::extensions::ext_video_encode_h264::VideoEncodeH264DpbSlotInfoEXTBuilder]) -> Self {
        self.0.p_ref_default_final_list0_entries = ref_default_final_list0_entries.as_ptr() as _;
        self.0.ref_default_final_list0_entry_count = ref_default_final_list0_entries.len() as _;
        self
    }
    #[inline]
    pub fn ref_default_final_list1_entries(mut self, ref_default_final_list1_entries: &'a [crate::extensions::ext_video_encode_h264::VideoEncodeH264DpbSlotInfoEXTBuilder]) -> Self {
        self.0.p_ref_default_final_list1_entries = ref_default_final_list1_entries.as_ptr() as _;
        self.0.ref_default_final_list1_entry_count = ref_default_final_list1_entries.len() as _;
        self
    }
    #[inline]
    pub fn nalu_slice_entries(mut self, nalu_slice_entries: &'a [crate::extensions::ext_video_encode_h264::VideoEncodeH264NaluSliceEXTBuilder]) -> Self {
        self.0.p_nalu_slice_entries = nalu_slice_entries.as_ptr() as _;
        self.0.nalu_slice_entry_count = nalu_slice_entries.len() as _;
        self
    }
    #[inline]
    pub fn current_picture_info(mut self, current_picture_info: &'a crate::extensions::ext_video_encode_h264::VideoEncodeH264DpbSlotInfoEXT) -> Self {
        self.0.p_current_picture_info = current_picture_info as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> VideoEncodeH264VclFrameInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for VideoEncodeH264VclFrameInfoEXTBuilder<'a> {
    fn default() -> VideoEncodeH264VclFrameInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoEncodeH264VclFrameInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoEncodeH264VclFrameInfoEXTBuilder<'a> {
    type Target = VideoEncodeH264VclFrameInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoEncodeH264VclFrameInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH264EmitPictureParametersEXT.html) · Structure"]
#[doc(alias = "VkVideoEncodeH264EmitPictureParametersEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoEncodeH264EmitPictureParametersEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub sps_id: u8,
    pub emit_sps_enable: crate::vk1_0::Bool32,
    pub pps_id_entry_count: u32,
    pub pps_id_entries: *const u8,
}
impl VideoEncodeH264EmitPictureParametersEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::VIDEO_ENCODE_H264_EMIT_PICTURE_PARAMETERS_EXT;
}
impl Default for VideoEncodeH264EmitPictureParametersEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), sps_id: Default::default(), emit_sps_enable: Default::default(), pps_id_entry_count: Default::default(), pps_id_entries: std::ptr::null() }
    }
}
impl std::fmt::Debug for VideoEncodeH264EmitPictureParametersEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoEncodeH264EmitPictureParametersEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("sps_id", &self.sps_id).field("emit_sps_enable", &(self.emit_sps_enable != 0)).field("pps_id_entry_count", &self.pps_id_entry_count).field("pps_id_entries", &self.pps_id_entries).finish()
    }
}
impl VideoEncodeH264EmitPictureParametersEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoEncodeH264EmitPictureParametersEXTBuilder<'a> {
        VideoEncodeH264EmitPictureParametersEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH264EmitPictureParametersEXT.html) · Builder of [`VideoEncodeH264EmitPictureParametersEXT`]"]
#[repr(transparent)]
pub struct VideoEncodeH264EmitPictureParametersEXTBuilder<'a>(VideoEncodeH264EmitPictureParametersEXT, std::marker::PhantomData<&'a ()>);
impl<'a> VideoEncodeH264EmitPictureParametersEXTBuilder<'a> {
    #[inline]
    pub fn new() -> VideoEncodeH264EmitPictureParametersEXTBuilder<'a> {
        VideoEncodeH264EmitPictureParametersEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn sps_id(mut self, sps_id: u8) -> Self {
        self.0.sps_id = sps_id as _;
        self
    }
    #[inline]
    pub fn emit_sps_enable(mut self, emit_sps_enable: bool) -> Self {
        self.0.emit_sps_enable = emit_sps_enable as _;
        self
    }
    #[inline]
    pub fn pps_id_entries(mut self, pps_id_entries: &'a [u8]) -> Self {
        self.0.pps_id_entries = pps_id_entries.as_ptr() as _;
        self.0.pps_id_entry_count = pps_id_entries.len() as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> VideoEncodeH264EmitPictureParametersEXT {
        self.0
    }
}
impl<'a> std::default::Default for VideoEncodeH264EmitPictureParametersEXTBuilder<'a> {
    fn default() -> VideoEncodeH264EmitPictureParametersEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoEncodeH264EmitPictureParametersEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoEncodeH264EmitPictureParametersEXTBuilder<'a> {
    type Target = VideoEncodeH264EmitPictureParametersEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoEncodeH264EmitPictureParametersEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH264ProfileEXT.html) · Structure"]
#[doc(alias = "VkVideoEncodeH264ProfileEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoEncodeH264ProfileEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub std_profile_idc: crate::external::vk_video::StdVideoH264ProfileIdc,
}
impl VideoEncodeH264ProfileEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::VIDEO_ENCODE_H264_PROFILE_EXT;
}
impl Default for VideoEncodeH264ProfileEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), std_profile_idc: Default::default() }
    }
}
impl std::fmt::Debug for VideoEncodeH264ProfileEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoEncodeH264ProfileEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("std_profile_idc", &self.std_profile_idc).finish()
    }
}
impl VideoEncodeH264ProfileEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoEncodeH264ProfileEXTBuilder<'a> {
        VideoEncodeH264ProfileEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH264ProfileEXT.html) · Builder of [`VideoEncodeH264ProfileEXT`]"]
#[repr(transparent)]
pub struct VideoEncodeH264ProfileEXTBuilder<'a>(VideoEncodeH264ProfileEXT, std::marker::PhantomData<&'a ()>);
impl<'a> VideoEncodeH264ProfileEXTBuilder<'a> {
    #[inline]
    pub fn new() -> VideoEncodeH264ProfileEXTBuilder<'a> {
        VideoEncodeH264ProfileEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn std_profile_idc(mut self, std_profile_idc: crate::external::vk_video::StdVideoH264ProfileIdc) -> Self {
        self.0.std_profile_idc = std_profile_idc as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> VideoEncodeH264ProfileEXT {
        self.0
    }
}
impl<'a> std::default::Default for VideoEncodeH264ProfileEXTBuilder<'a> {
    fn default() -> VideoEncodeH264ProfileEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoEncodeH264ProfileEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoEncodeH264ProfileEXTBuilder<'a> {
    type Target = VideoEncodeH264ProfileEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoEncodeH264ProfileEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH264NaluSliceEXT.html) · Structure"]
#[doc(alias = "VkVideoEncodeH264NaluSliceEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoEncodeH264NaluSliceEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub p_slice_header_std: *const crate::external::vk_video::StdVideoEncodeH264SliceHeader,
    pub mb_count: u32,
    pub ref_final_list0_entry_count: u8,
    pub p_ref_final_list0_entries: *const crate::extensions::ext_video_encode_h264::VideoEncodeH264DpbSlotInfoEXT,
    pub ref_final_list1_entry_count: u8,
    pub p_ref_final_list1_entries: *const crate::extensions::ext_video_encode_h264::VideoEncodeH264DpbSlotInfoEXT,
}
impl VideoEncodeH264NaluSliceEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::VIDEO_ENCODE_H264_NALU_SLICE_EXT;
}
impl Default for VideoEncodeH264NaluSliceEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), p_slice_header_std: std::ptr::null(), mb_count: Default::default(), ref_final_list0_entry_count: Default::default(), p_ref_final_list0_entries: std::ptr::null(), ref_final_list1_entry_count: Default::default(), p_ref_final_list1_entries: std::ptr::null() }
    }
}
impl std::fmt::Debug for VideoEncodeH264NaluSliceEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoEncodeH264NaluSliceEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("p_slice_header_std", &self.p_slice_header_std).field("mb_count", &self.mb_count).field("ref_final_list0_entry_count", &self.ref_final_list0_entry_count).field("p_ref_final_list0_entries", &self.p_ref_final_list0_entries).field("ref_final_list1_entry_count", &self.ref_final_list1_entry_count).field("p_ref_final_list1_entries", &self.p_ref_final_list1_entries).finish()
    }
}
impl VideoEncodeH264NaluSliceEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoEncodeH264NaluSliceEXTBuilder<'a> {
        VideoEncodeH264NaluSliceEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH264NaluSliceEXT.html) · Builder of [`VideoEncodeH264NaluSliceEXT`]"]
#[repr(transparent)]
pub struct VideoEncodeH264NaluSliceEXTBuilder<'a>(VideoEncodeH264NaluSliceEXT, std::marker::PhantomData<&'a ()>);
impl<'a> VideoEncodeH264NaluSliceEXTBuilder<'a> {
    #[inline]
    pub fn new() -> VideoEncodeH264NaluSliceEXTBuilder<'a> {
        VideoEncodeH264NaluSliceEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn slice_header_std(mut self, slice_header_std: &'a crate::external::vk_video::StdVideoEncodeH264SliceHeader) -> Self {
        self.0.p_slice_header_std = slice_header_std as _;
        self
    }
    #[inline]
    pub fn mb_count(mut self, mb_count: u32) -> Self {
        self.0.mb_count = mb_count as _;
        self
    }
    #[inline]
    pub fn ref_final_list0_entries(mut self, ref_final_list0_entries: &'a [crate::extensions::ext_video_encode_h264::VideoEncodeH264DpbSlotInfoEXTBuilder]) -> Self {
        self.0.p_ref_final_list0_entries = ref_final_list0_entries.as_ptr() as _;
        self.0.ref_final_list0_entry_count = ref_final_list0_entries.len() as _;
        self
    }
    #[inline]
    pub fn ref_final_list1_entries(mut self, ref_final_list1_entries: &'a [crate::extensions::ext_video_encode_h264::VideoEncodeH264DpbSlotInfoEXTBuilder]) -> Self {
        self.0.p_ref_final_list1_entries = ref_final_list1_entries.as_ptr() as _;
        self.0.ref_final_list1_entry_count = ref_final_list1_entries.len() as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> VideoEncodeH264NaluSliceEXT {
        self.0
    }
}
impl<'a> std::default::Default for VideoEncodeH264NaluSliceEXTBuilder<'a> {
    fn default() -> VideoEncodeH264NaluSliceEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoEncodeH264NaluSliceEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoEncodeH264NaluSliceEXTBuilder<'a> {
    type Target = VideoEncodeH264NaluSliceEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoEncodeH264NaluSliceEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH264RateControlInfoEXT.html) · Structure"]
#[doc(alias = "VkVideoEncodeH264RateControlInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoEncodeH264RateControlInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub gop_frame_count: u32,
    pub idr_period: u32,
    pub consecutive_b_frame_count: u32,
    pub rate_control_structure: crate::extensions::ext_video_encode_h264::VideoEncodeH264RateControlStructureFlagBitsEXT,
}
impl VideoEncodeH264RateControlInfoEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::VIDEO_ENCODE_H264_RATE_CONTROL_INFO_EXT;
}
impl Default for VideoEncodeH264RateControlInfoEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), gop_frame_count: Default::default(), idr_period: Default::default(), consecutive_b_frame_count: Default::default(), rate_control_structure: Default::default() }
    }
}
impl std::fmt::Debug for VideoEncodeH264RateControlInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoEncodeH264RateControlInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("gop_frame_count", &self.gop_frame_count).field("idr_period", &self.idr_period).field("consecutive_b_frame_count", &self.consecutive_b_frame_count).field("rate_control_structure", &self.rate_control_structure).finish()
    }
}
impl VideoEncodeH264RateControlInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoEncodeH264RateControlInfoEXTBuilder<'a> {
        VideoEncodeH264RateControlInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH264RateControlInfoEXT.html) · Builder of [`VideoEncodeH264RateControlInfoEXT`]"]
#[repr(transparent)]
pub struct VideoEncodeH264RateControlInfoEXTBuilder<'a>(VideoEncodeH264RateControlInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> VideoEncodeH264RateControlInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> VideoEncodeH264RateControlInfoEXTBuilder<'a> {
        VideoEncodeH264RateControlInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn gop_frame_count(mut self, gop_frame_count: u32) -> Self {
        self.0.gop_frame_count = gop_frame_count as _;
        self
    }
    #[inline]
    pub fn idr_period(mut self, idr_period: u32) -> Self {
        self.0.idr_period = idr_period as _;
        self
    }
    #[inline]
    pub fn consecutive_b_frame_count(mut self, consecutive_b_frame_count: u32) -> Self {
        self.0.consecutive_b_frame_count = consecutive_b_frame_count as _;
        self
    }
    #[inline]
    pub fn rate_control_structure(mut self, rate_control_structure: crate::extensions::ext_video_encode_h264::VideoEncodeH264RateControlStructureFlagBitsEXT) -> Self {
        self.0.rate_control_structure = rate_control_structure as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> VideoEncodeH264RateControlInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for VideoEncodeH264RateControlInfoEXTBuilder<'a> {
    fn default() -> VideoEncodeH264RateControlInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoEncodeH264RateControlInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoEncodeH264RateControlInfoEXTBuilder<'a> {
    type Target = VideoEncodeH264RateControlInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoEncodeH264RateControlInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH264QpEXT.html) · Structure"]
#[doc(alias = "VkVideoEncodeH264QpEXT")]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct VideoEncodeH264QpEXT {
    pub qp_i: i32,
    pub qp_p: i32,
    pub qp_b: i32,
}
impl Default for VideoEncodeH264QpEXT {
    fn default() -> Self {
        Self { qp_i: Default::default(), qp_p: Default::default(), qp_b: Default::default() }
    }
}
impl std::fmt::Debug for VideoEncodeH264QpEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoEncodeH264QpEXT").field("qp_i", &self.qp_i).field("qp_p", &self.qp_p).field("qp_b", &self.qp_b).finish()
    }
}
impl VideoEncodeH264QpEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoEncodeH264QpEXTBuilder<'a> {
        VideoEncodeH264QpEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH264QpEXT.html) · Builder of [`VideoEncodeH264QpEXT`]"]
#[repr(transparent)]
pub struct VideoEncodeH264QpEXTBuilder<'a>(VideoEncodeH264QpEXT, std::marker::PhantomData<&'a ()>);
impl<'a> VideoEncodeH264QpEXTBuilder<'a> {
    #[inline]
    pub fn new() -> VideoEncodeH264QpEXTBuilder<'a> {
        VideoEncodeH264QpEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn qp_i(mut self, qp_i: i32) -> Self {
        self.0.qp_i = qp_i as _;
        self
    }
    #[inline]
    pub fn qp_p(mut self, qp_p: i32) -> Self {
        self.0.qp_p = qp_p as _;
        self
    }
    #[inline]
    pub fn qp_b(mut self, qp_b: i32) -> Self {
        self.0.qp_b = qp_b as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> VideoEncodeH264QpEXT {
        self.0
    }
}
impl<'a> std::default::Default for VideoEncodeH264QpEXTBuilder<'a> {
    fn default() -> VideoEncodeH264QpEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoEncodeH264QpEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoEncodeH264QpEXTBuilder<'a> {
    type Target = VideoEncodeH264QpEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoEncodeH264QpEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH264FrameSizeEXT.html) · Structure"]
#[doc(alias = "VkVideoEncodeH264FrameSizeEXT")]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct VideoEncodeH264FrameSizeEXT {
    pub frame_i_size: u32,
    pub frame_p_size: u32,
    pub frame_b_size: u32,
}
impl Default for VideoEncodeH264FrameSizeEXT {
    fn default() -> Self {
        Self { frame_i_size: Default::default(), frame_p_size: Default::default(), frame_b_size: Default::default() }
    }
}
impl std::fmt::Debug for VideoEncodeH264FrameSizeEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoEncodeH264FrameSizeEXT").field("frame_i_size", &self.frame_i_size).field("frame_p_size", &self.frame_p_size).field("frame_b_size", &self.frame_b_size).finish()
    }
}
impl VideoEncodeH264FrameSizeEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoEncodeH264FrameSizeEXTBuilder<'a> {
        VideoEncodeH264FrameSizeEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH264FrameSizeEXT.html) · Builder of [`VideoEncodeH264FrameSizeEXT`]"]
#[repr(transparent)]
pub struct VideoEncodeH264FrameSizeEXTBuilder<'a>(VideoEncodeH264FrameSizeEXT, std::marker::PhantomData<&'a ()>);
impl<'a> VideoEncodeH264FrameSizeEXTBuilder<'a> {
    #[inline]
    pub fn new() -> VideoEncodeH264FrameSizeEXTBuilder<'a> {
        VideoEncodeH264FrameSizeEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn frame_i_size(mut self, frame_i_size: u32) -> Self {
        self.0.frame_i_size = frame_i_size as _;
        self
    }
    #[inline]
    pub fn frame_p_size(mut self, frame_p_size: u32) -> Self {
        self.0.frame_p_size = frame_p_size as _;
        self
    }
    #[inline]
    pub fn frame_b_size(mut self, frame_b_size: u32) -> Self {
        self.0.frame_b_size = frame_b_size as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> VideoEncodeH264FrameSizeEXT {
        self.0
    }
}
impl<'a> std::default::Default for VideoEncodeH264FrameSizeEXTBuilder<'a> {
    fn default() -> VideoEncodeH264FrameSizeEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoEncodeH264FrameSizeEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoEncodeH264FrameSizeEXTBuilder<'a> {
    type Target = VideoEncodeH264FrameSizeEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoEncodeH264FrameSizeEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH264RateControlLayerInfoEXT.html) · Structure"]
#[doc(alias = "VkVideoEncodeH264RateControlLayerInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoEncodeH264RateControlLayerInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub temporal_layer_id: u8,
    pub use_initial_rc_qp: crate::vk1_0::Bool32,
    pub initial_rc_qp: crate::extensions::ext_video_encode_h264::VideoEncodeH264QpEXT,
    pub use_min_qp: crate::vk1_0::Bool32,
    pub min_qp: crate::extensions::ext_video_encode_h264::VideoEncodeH264QpEXT,
    pub use_max_qp: crate::vk1_0::Bool32,
    pub max_qp: crate::extensions::ext_video_encode_h264::VideoEncodeH264QpEXT,
    pub use_max_frame_size: crate::vk1_0::Bool32,
    pub max_frame_size: crate::extensions::ext_video_encode_h264::VideoEncodeH264FrameSizeEXT,
}
impl VideoEncodeH264RateControlLayerInfoEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::VIDEO_ENCODE_H264_RATE_CONTROL_LAYER_INFO_EXT;
}
impl Default for VideoEncodeH264RateControlLayerInfoEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), temporal_layer_id: Default::default(), use_initial_rc_qp: Default::default(), initial_rc_qp: Default::default(), use_min_qp: Default::default(), min_qp: Default::default(), use_max_qp: Default::default(), max_qp: Default::default(), use_max_frame_size: Default::default(), max_frame_size: Default::default() }
    }
}
impl std::fmt::Debug for VideoEncodeH264RateControlLayerInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoEncodeH264RateControlLayerInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("temporal_layer_id", &self.temporal_layer_id).field("use_initial_rc_qp", &(self.use_initial_rc_qp != 0)).field("initial_rc_qp", &self.initial_rc_qp).field("use_min_qp", &(self.use_min_qp != 0)).field("min_qp", &self.min_qp).field("use_max_qp", &(self.use_max_qp != 0)).field("max_qp", &self.max_qp).field("use_max_frame_size", &(self.use_max_frame_size != 0)).field("max_frame_size", &self.max_frame_size).finish()
    }
}
impl VideoEncodeH264RateControlLayerInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoEncodeH264RateControlLayerInfoEXTBuilder<'a> {
        VideoEncodeH264RateControlLayerInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH264RateControlLayerInfoEXT.html) · Builder of [`VideoEncodeH264RateControlLayerInfoEXT`]"]
#[repr(transparent)]
pub struct VideoEncodeH264RateControlLayerInfoEXTBuilder<'a>(VideoEncodeH264RateControlLayerInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> VideoEncodeH264RateControlLayerInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> VideoEncodeH264RateControlLayerInfoEXTBuilder<'a> {
        VideoEncodeH264RateControlLayerInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn temporal_layer_id(mut self, temporal_layer_id: u8) -> Self {
        self.0.temporal_layer_id = temporal_layer_id as _;
        self
    }
    #[inline]
    pub fn use_initial_rc_qp(mut self, use_initial_rc_qp: bool) -> Self {
        self.0.use_initial_rc_qp = use_initial_rc_qp as _;
        self
    }
    #[inline]
    pub fn initial_rc_qp(mut self, initial_rc_qp: crate::extensions::ext_video_encode_h264::VideoEncodeH264QpEXT) -> Self {
        self.0.initial_rc_qp = initial_rc_qp as _;
        self
    }
    #[inline]
    pub fn use_min_qp(mut self, use_min_qp: bool) -> Self {
        self.0.use_min_qp = use_min_qp as _;
        self
    }
    #[inline]
    pub fn min_qp(mut self, min_qp: crate::extensions::ext_video_encode_h264::VideoEncodeH264QpEXT) -> Self {
        self.0.min_qp = min_qp as _;
        self
    }
    #[inline]
    pub fn use_max_qp(mut self, use_max_qp: bool) -> Self {
        self.0.use_max_qp = use_max_qp as _;
        self
    }
    #[inline]
    pub fn max_qp(mut self, max_qp: crate::extensions::ext_video_encode_h264::VideoEncodeH264QpEXT) -> Self {
        self.0.max_qp = max_qp as _;
        self
    }
    #[inline]
    pub fn use_max_frame_size(mut self, use_max_frame_size: bool) -> Self {
        self.0.use_max_frame_size = use_max_frame_size as _;
        self
    }
    #[inline]
    pub fn max_frame_size(mut self, max_frame_size: crate::extensions::ext_video_encode_h264::VideoEncodeH264FrameSizeEXT) -> Self {
        self.0.max_frame_size = max_frame_size as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> VideoEncodeH264RateControlLayerInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for VideoEncodeH264RateControlLayerInfoEXTBuilder<'a> {
    fn default() -> VideoEncodeH264RateControlLayerInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoEncodeH264RateControlLayerInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoEncodeH264RateControlLayerInfoEXTBuilder<'a> {
    type Target = VideoEncodeH264RateControlLayerInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoEncodeH264RateControlLayerInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
