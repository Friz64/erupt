//! ## Versioning Warning ⚠️
//!
//! This is a Vulkan **provisional/beta** extension and **must** be used with
//! caution. Its API/behaviour has not been finalized yet and _may_ therefore
//! change in ways that break backwards compatibility between revisions, and
//! before final release of a non-provisional version of this extension.
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_VIDEO_QUEUE_SPEC_VERSION")]
pub const KHR_VIDEO_QUEUE_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_VIDEO_QUEUE_EXTENSION_NAME")]
pub const KHR_VIDEO_QUEUE_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_video_queue");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_PHYSICAL_DEVICE_VIDEO_CAPABILITIES_KHR: *const std::os::raw::c_char = crate::cstr!("vkGetPhysicalDeviceVideoCapabilitiesKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_PHYSICAL_DEVICE_VIDEO_FORMAT_PROPERTIES_KHR: *const std::os::raw::c_char = crate::cstr!("vkGetPhysicalDeviceVideoFormatPropertiesKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_VIDEO_SESSION_KHR: *const std::os::raw::c_char = crate::cstr!("vkCreateVideoSessionKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_DESTROY_VIDEO_SESSION_KHR: *const std::os::raw::c_char = crate::cstr!("vkDestroyVideoSessionKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_VIDEO_SESSION_PARAMETERS_KHR: *const std::os::raw::c_char = crate::cstr!("vkCreateVideoSessionParametersKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_UPDATE_VIDEO_SESSION_PARAMETERS_KHR: *const std::os::raw::c_char = crate::cstr!("vkUpdateVideoSessionParametersKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_DESTROY_VIDEO_SESSION_PARAMETERS_KHR: *const std::os::raw::c_char = crate::cstr!("vkDestroyVideoSessionParametersKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_VIDEO_SESSION_MEMORY_REQUIREMENTS_KHR: *const std::os::raw::c_char = crate::cstr!("vkGetVideoSessionMemoryRequirementsKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_BIND_VIDEO_SESSION_MEMORY_KHR: *const std::os::raw::c_char = crate::cstr!("vkBindVideoSessionMemoryKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_BEGIN_VIDEO_CODING_KHR: *const std::os::raw::c_char = crate::cstr!("vkCmdBeginVideoCodingKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_CONTROL_VIDEO_CODING_KHR: *const std::os::raw::c_char = crate::cstr!("vkCmdControlVideoCodingKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_END_VIDEO_CODING_KHR: *const std::os::raw::c_char = crate::cstr!("vkCmdEndVideoCodingKHR");
crate::non_dispatchable_handle!(VideoSessionKHR, VIDEO_SESSION_KHR, "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoSessionKHR.html) · Non-dispatchable Handle", "VkVideoSessionKHR");
crate::non_dispatchable_handle!(VideoSessionParametersKHR, VIDEO_SESSION_PARAMETERS_KHR, "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoSessionParametersKHR.html) · Non-dispatchable Handle", "VkVideoSessionParametersKHR");
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoBeginCodingFlagsKHR.html) · Bitmask of [`VideoBeginCodingFlagBitsKHR`]"] # [doc (alias = "VkVideoBeginCodingFlagsKHR")] # [derive (Default)] # [repr (transparent)] pub struct VideoBeginCodingFlagsKHR : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`VideoBeginCodingFlagsKHR`]"]
#[doc(alias = "VkVideoBeginCodingFlagBitsKHR")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct VideoBeginCodingFlagBitsKHR(pub u32);
impl VideoBeginCodingFlagBitsKHR {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> VideoBeginCodingFlagsKHR {
        VideoBeginCodingFlagsKHR::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for VideoBeginCodingFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEndCodingFlagsKHR.html) · Bitmask of [`VideoEndCodingFlagBitsKHR`]"] # [doc (alias = "VkVideoEndCodingFlagsKHR")] # [derive (Default)] # [repr (transparent)] pub struct VideoEndCodingFlagsKHR : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`VideoEndCodingFlagsKHR`]"]
#[doc(alias = "VkVideoEndCodingFlagBitsKHR")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct VideoEndCodingFlagBitsKHR(pub u32);
impl VideoEndCodingFlagBitsKHR {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> VideoEndCodingFlagsKHR {
        VideoEndCodingFlagsKHR::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for VideoEndCodingFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoCodecOperationFlagsKHR.html) · Bitmask of [`VideoCodecOperationFlagBitsKHR`]"] # [doc (alias = "VkVideoCodecOperationFlagsKHR")] # [derive (Default)] # [repr (transparent)] pub struct VideoCodecOperationFlagsKHR : u32 { const INVALID_KHR = VideoCodecOperationFlagBitsKHR :: INVALID_KHR . 0 ; const ENCODE_H264_EXT = VideoCodecOperationFlagBitsKHR :: ENCODE_H264_EXT . 0 ; const DECODE_H264_EXT = VideoCodecOperationFlagBitsKHR :: DECODE_H264_EXT . 0 ; const DECODE_H265_EXT = VideoCodecOperationFlagBitsKHR :: DECODE_H265_EXT . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoCodecOperationFlagBitsKHR.html) · Bits enum of [`VideoCodecOperationFlagsKHR`]"]
#[doc(alias = "VkVideoCodecOperationFlagBitsKHR")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct VideoCodecOperationFlagBitsKHR(pub u32);
impl VideoCodecOperationFlagBitsKHR {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> VideoCodecOperationFlagsKHR {
        VideoCodecOperationFlagsKHR::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for VideoCodecOperationFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::INVALID_KHR => "INVALID_KHR",
            &Self::ENCODE_H264_EXT => "ENCODE_H264_EXT",
            &Self::DECODE_H264_EXT => "DECODE_H264_EXT",
            &Self::DECODE_H265_EXT => "DECODE_H265_EXT",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::khr_video_queue`]"]
impl VideoCodecOperationFlagBitsKHR {
    pub const INVALID_KHR: Self = Self(0);
}
#[doc = "Provided by [`crate::extensions::ext_video_encode_h264`]"]
impl VideoCodecOperationFlagBitsKHR {
    pub const ENCODE_H264_EXT: Self = Self(65536);
}
#[doc = "Provided by [`crate::extensions::ext_video_decode_h264`]"]
impl VideoCodecOperationFlagBitsKHR {
    pub const DECODE_H264_EXT: Self = Self(1);
}
#[doc = "Provided by [`crate::extensions::ext_video_decode_h265`]"]
impl VideoCodecOperationFlagBitsKHR {
    pub const DECODE_H265_EXT: Self = Self(2);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoChromaSubsamplingFlagsKHR.html) · Bitmask of [`VideoChromaSubsamplingFlagBitsKHR`]"] # [doc (alias = "VkVideoChromaSubsamplingFlagsKHR")] # [derive (Default)] # [repr (transparent)] pub struct VideoChromaSubsamplingFlagsKHR : u32 { const INVALID_KHR = VideoChromaSubsamplingFlagBitsKHR :: INVALID_KHR . 0 ; const MONOCHROME_KHR = VideoChromaSubsamplingFlagBitsKHR :: MONOCHROME_KHR . 0 ; const _420_KHR = VideoChromaSubsamplingFlagBitsKHR :: _420_KHR . 0 ; const _422_KHR = VideoChromaSubsamplingFlagBitsKHR :: _422_KHR . 0 ; const _444_KHR = VideoChromaSubsamplingFlagBitsKHR :: _444_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoChromaSubsamplingFlagBitsKHR.html) · Bits enum of [`VideoChromaSubsamplingFlagsKHR`]"]
#[doc(alias = "VkVideoChromaSubsamplingFlagBitsKHR")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct VideoChromaSubsamplingFlagBitsKHR(pub u32);
impl VideoChromaSubsamplingFlagBitsKHR {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> VideoChromaSubsamplingFlagsKHR {
        VideoChromaSubsamplingFlagsKHR::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for VideoChromaSubsamplingFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::INVALID_KHR => "INVALID_KHR",
            &Self::MONOCHROME_KHR => "MONOCHROME_KHR",
            &Self::_420_KHR => "_420_KHR",
            &Self::_422_KHR => "_422_KHR",
            &Self::_444_KHR => "_444_KHR",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::khr_video_queue`]"]
impl VideoChromaSubsamplingFlagBitsKHR {
    pub const INVALID_KHR: Self = Self(0);
    pub const MONOCHROME_KHR: Self = Self(1);
    pub const _420_KHR: Self = Self(2);
    pub const _422_KHR: Self = Self(4);
    pub const _444_KHR: Self = Self(8);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoComponentBitDepthFlagsKHR.html) · Bitmask of [`VideoComponentBitDepthFlagBitsKHR`]"] # [doc (alias = "VkVideoComponentBitDepthFlagsKHR")] # [derive (Default)] # [repr (transparent)] pub struct VideoComponentBitDepthFlagsKHR : u32 { const VIDEO_COMPONENT_DEPTH_INVALID_KHR = VideoComponentBitDepthFlagBitsKHR :: VIDEO_COMPONENT_DEPTH_INVALID_KHR . 0 ; const VIDEO_COMPONENT_DEPTH_8_KHR = VideoComponentBitDepthFlagBitsKHR :: VIDEO_COMPONENT_DEPTH_8_KHR . 0 ; const VIDEO_COMPONENT_DEPTH_10_KHR = VideoComponentBitDepthFlagBitsKHR :: VIDEO_COMPONENT_DEPTH_10_KHR . 0 ; const VIDEO_COMPONENT_DEPTH_12_KHR = VideoComponentBitDepthFlagBitsKHR :: VIDEO_COMPONENT_DEPTH_12_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoComponentBitDepthFlagBitsKHR.html) · Bits enum of [`VideoComponentBitDepthFlagsKHR`]"]
#[doc(alias = "VkVideoComponentBitDepthFlagBitsKHR")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct VideoComponentBitDepthFlagBitsKHR(pub u32);
impl VideoComponentBitDepthFlagBitsKHR {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> VideoComponentBitDepthFlagsKHR {
        VideoComponentBitDepthFlagsKHR::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for VideoComponentBitDepthFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::VIDEO_COMPONENT_DEPTH_INVALID_KHR => "VIDEO_COMPONENT_DEPTH_INVALID_KHR",
            &Self::VIDEO_COMPONENT_DEPTH_8_KHR => "VIDEO_COMPONENT_DEPTH_8_KHR",
            &Self::VIDEO_COMPONENT_DEPTH_10_KHR => "VIDEO_COMPONENT_DEPTH_10_KHR",
            &Self::VIDEO_COMPONENT_DEPTH_12_KHR => "VIDEO_COMPONENT_DEPTH_12_KHR",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::khr_video_queue`]"]
impl VideoComponentBitDepthFlagBitsKHR {
    pub const VIDEO_COMPONENT_DEPTH_INVALID_KHR: Self = Self(0);
    pub const VIDEO_COMPONENT_DEPTH_8_KHR: Self = Self(1);
    pub const VIDEO_COMPONENT_DEPTH_10_KHR: Self = Self(4);
    pub const VIDEO_COMPONENT_DEPTH_12_KHR: Self = Self(16);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoCapabilitiesFlagsKHR.html) · Bitmask of [`VideoCapabilitiesFlagBitsKHR`]"] # [doc (alias = "VkVideoCapabilitiesFlagsKHR")] # [derive (Default)] # [repr (transparent)] pub struct VideoCapabilitiesFlagsKHR : u32 { const PROTECTED_CONTENT_KHR = VideoCapabilitiesFlagBitsKHR :: PROTECTED_CONTENT_KHR . 0 ; const SEPARATE_REFERENCE_IMAGES_KHR = VideoCapabilitiesFlagBitsKHR :: SEPARATE_REFERENCE_IMAGES_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoCapabilitiesFlagBitsKHR.html) · Bits enum of [`VideoCapabilitiesFlagsKHR`]"]
#[doc(alias = "VkVideoCapabilitiesFlagBitsKHR")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct VideoCapabilitiesFlagBitsKHR(pub u32);
impl VideoCapabilitiesFlagBitsKHR {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> VideoCapabilitiesFlagsKHR {
        VideoCapabilitiesFlagsKHR::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for VideoCapabilitiesFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::PROTECTED_CONTENT_KHR => "PROTECTED_CONTENT_KHR",
            &Self::SEPARATE_REFERENCE_IMAGES_KHR => "SEPARATE_REFERENCE_IMAGES_KHR",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::khr_video_queue`]"]
impl VideoCapabilitiesFlagBitsKHR {
    pub const PROTECTED_CONTENT_KHR: Self = Self(1);
    pub const SEPARATE_REFERENCE_IMAGES_KHR: Self = Self(2);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoSessionCreateFlagsKHR.html) · Bitmask of [`VideoSessionCreateFlagBitsKHR`]"] # [doc (alias = "VkVideoSessionCreateFlagsKHR")] # [derive (Default)] # [repr (transparent)] pub struct VideoSessionCreateFlagsKHR : u32 { const DEFAULT_KHR = VideoSessionCreateFlagBitsKHR :: DEFAULT_KHR . 0 ; const PROTECTED_CONTENT_KHR = VideoSessionCreateFlagBitsKHR :: PROTECTED_CONTENT_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoSessionCreateFlagBitsKHR.html) · Bits enum of [`VideoSessionCreateFlagsKHR`]"]
#[doc(alias = "VkVideoSessionCreateFlagBitsKHR")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct VideoSessionCreateFlagBitsKHR(pub u32);
impl VideoSessionCreateFlagBitsKHR {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> VideoSessionCreateFlagsKHR {
        VideoSessionCreateFlagsKHR::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for VideoSessionCreateFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::DEFAULT_KHR => "DEFAULT_KHR",
            &Self::PROTECTED_CONTENT_KHR => "PROTECTED_CONTENT_KHR",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::khr_video_queue`]"]
impl VideoSessionCreateFlagBitsKHR {
    pub const DEFAULT_KHR: Self = Self(0);
    pub const PROTECTED_CONTENT_KHR: Self = Self(1);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoCodingQualityPresetFlagsKHR.html) · Bitmask of [`VideoCodingQualityPresetFlagBitsKHR`]"] # [doc (alias = "VkVideoCodingQualityPresetFlagsKHR")] # [derive (Default)] # [repr (transparent)] pub struct VideoCodingQualityPresetFlagsKHR : u32 { const DEFAULT_KHR = VideoCodingQualityPresetFlagBitsKHR :: DEFAULT_KHR . 0 ; const NORMAL_KHR = VideoCodingQualityPresetFlagBitsKHR :: NORMAL_KHR . 0 ; const POWER_KHR = VideoCodingQualityPresetFlagBitsKHR :: POWER_KHR . 0 ; const QUALITY_KHR = VideoCodingQualityPresetFlagBitsKHR :: QUALITY_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoCodingQualityPresetFlagBitsKHR.html) · Bits enum of [`VideoCodingQualityPresetFlagsKHR`]"]
#[doc(alias = "VkVideoCodingQualityPresetFlagBitsKHR")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct VideoCodingQualityPresetFlagBitsKHR(pub u32);
impl VideoCodingQualityPresetFlagBitsKHR {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> VideoCodingQualityPresetFlagsKHR {
        VideoCodingQualityPresetFlagsKHR::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for VideoCodingQualityPresetFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::DEFAULT_KHR => "DEFAULT_KHR",
            &Self::NORMAL_KHR => "NORMAL_KHR",
            &Self::POWER_KHR => "POWER_KHR",
            &Self::QUALITY_KHR => "QUALITY_KHR",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::khr_video_queue`]"]
impl VideoCodingQualityPresetFlagBitsKHR {
    pub const DEFAULT_KHR: Self = Self(0);
    pub const NORMAL_KHR: Self = Self(1);
    pub const POWER_KHR: Self = Self(2);
    pub const QUALITY_KHR: Self = Self(4);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoCodingControlFlagsKHR.html) · Bitmask of [`VideoCodingControlFlagBitsKHR`]"] # [doc (alias = "VkVideoCodingControlFlagsKHR")] # [derive (Default)] # [repr (transparent)] pub struct VideoCodingControlFlagsKHR : u32 { const DEFAULT_KHR = VideoCodingControlFlagBitsKHR :: DEFAULT_KHR . 0 ; const RESET_KHR = VideoCodingControlFlagBitsKHR :: RESET_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoCodingControlFlagBitsKHR.html) · Bits enum of [`VideoCodingControlFlagsKHR`]"]
#[doc(alias = "VkVideoCodingControlFlagBitsKHR")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct VideoCodingControlFlagBitsKHR(pub u32);
impl VideoCodingControlFlagBitsKHR {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> VideoCodingControlFlagsKHR {
        VideoCodingControlFlagsKHR::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for VideoCodingControlFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::DEFAULT_KHR => "DEFAULT_KHR",
            &Self::RESET_KHR => "RESET_KHR",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::khr_video_queue`]"]
impl VideoCodingControlFlagBitsKHR {
    pub const DEFAULT_KHR: Self = Self(0);
    pub const RESET_KHR: Self = Self(1);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueryResultStatusKHR.html) · Enum"]
#[doc(alias = "VkQueryResultStatusKHR")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct QueryResultStatusKHR(pub i32);
impl std::fmt::Debug for QueryResultStatusKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::ERROR_KHR => "ERROR_KHR",
            &Self::NOT_READY_KHR => "NOT_READY_KHR",
            &Self::COMPLETE_KHR => "COMPLETE_KHR",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::khr_video_queue`]"]
impl QueryResultStatusKHR {
    pub const ERROR_KHR: Self = Self(-1);
    pub const NOT_READY_KHR: Self = Self(0);
    pub const COMPLETE_KHR: Self = Self(1);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceVideoCapabilitiesKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceVideoCapabilitiesKHR = unsafe extern "system" fn(physical_device: crate::vk1_0::PhysicalDevice, p_video_profile: *const crate::extensions::khr_video_queue::VideoProfileKHR, p_capabilities: *mut crate::extensions::khr_video_queue::VideoCapabilitiesKHR) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceVideoFormatPropertiesKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceVideoFormatPropertiesKHR = unsafe extern "system" fn(physical_device: crate::vk1_0::PhysicalDevice, p_video_format_info: *const crate::extensions::khr_video_queue::PhysicalDeviceVideoFormatInfoKHR, p_video_format_property_count: *mut u32, p_video_format_properties: *mut crate::extensions::khr_video_queue::VideoFormatPropertiesKHR) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateVideoSessionKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateVideoSessionKHR = unsafe extern "system" fn(device: crate::vk1_0::Device, p_create_info: *const crate::extensions::khr_video_queue::VideoSessionCreateInfoKHR, p_allocator: *const crate::vk1_0::AllocationCallbacks, p_video_session: *mut crate::extensions::khr_video_queue::VideoSessionKHR) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyVideoSessionKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyVideoSessionKHR = unsafe extern "system" fn(device: crate::vk1_0::Device, video_session: crate::extensions::khr_video_queue::VideoSessionKHR, p_allocator: *const crate::vk1_0::AllocationCallbacks) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateVideoSessionParametersKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateVideoSessionParametersKHR = unsafe extern "system" fn(device: crate::vk1_0::Device, p_create_info: *const crate::extensions::khr_video_queue::VideoSessionParametersCreateInfoKHR, p_allocator: *const crate::vk1_0::AllocationCallbacks, p_video_session_parameters: *mut crate::extensions::khr_video_queue::VideoSessionParametersKHR) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkUpdateVideoSessionParametersKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkUpdateVideoSessionParametersKHR = unsafe extern "system" fn(device: crate::vk1_0::Device, video_session_parameters: crate::extensions::khr_video_queue::VideoSessionParametersKHR, p_update_info: *const crate::extensions::khr_video_queue::VideoSessionParametersUpdateInfoKHR) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyVideoSessionParametersKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyVideoSessionParametersKHR = unsafe extern "system" fn(device: crate::vk1_0::Device, video_session_parameters: crate::extensions::khr_video_queue::VideoSessionParametersKHR, p_allocator: *const crate::vk1_0::AllocationCallbacks) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetVideoSessionMemoryRequirementsKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetVideoSessionMemoryRequirementsKHR = unsafe extern "system" fn(device: crate::vk1_0::Device, video_session: crate::extensions::khr_video_queue::VideoSessionKHR, p_video_session_memory_requirements_count: *mut u32, p_video_session_memory_requirements: *mut crate::extensions::khr_video_queue::VideoGetMemoryPropertiesKHR) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBindVideoSessionMemoryKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkBindVideoSessionMemoryKHR = unsafe extern "system" fn(device: crate::vk1_0::Device, video_session: crate::extensions::khr_video_queue::VideoSessionKHR, video_session_bind_memory_count: u32, p_video_session_bind_memories: *const crate::extensions::khr_video_queue::VideoBindMemoryKHR) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBeginVideoCodingKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBeginVideoCodingKHR = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, p_begin_info: *const crate::extensions::khr_video_queue::VideoBeginCodingInfoKHR) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdControlVideoCodingKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdControlVideoCodingKHR = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, p_coding_control_info: *const crate::extensions::khr_video_queue::VideoCodingControlInfoKHR) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdEndVideoCodingKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdEndVideoCodingKHR = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, p_end_coding_info: *const crate::extensions::khr_video_queue::VideoEndCodingInfoKHR) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoQueueFamilyProperties2KHR.html) · Structure"]
#[doc(alias = "VkVideoQueueFamilyProperties2KHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoQueueFamilyProperties2KHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub video_codec_operations: crate::extensions::khr_video_queue::VideoCodecOperationFlagsKHR,
}
impl Default for VideoQueueFamilyProperties2KHR {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::VIDEO_QUEUE_FAMILY_PROPERTIES_2_KHR, p_next: std::ptr::null_mut(), video_codec_operations: Default::default() }
    }
}
impl std::fmt::Debug for VideoQueueFamilyProperties2KHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoQueueFamilyProperties2KHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("video_codec_operations", &self.video_codec_operations).finish()
    }
}
impl VideoQueueFamilyProperties2KHR {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoQueueFamilyProperties2KHRBuilder<'a> {
        VideoQueueFamilyProperties2KHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoQueueFamilyProperties2KHR.html) · Builder of [`VideoQueueFamilyProperties2KHR`]"]
#[repr(transparent)]
pub struct VideoQueueFamilyProperties2KHRBuilder<'a>(VideoQueueFamilyProperties2KHR, std::marker::PhantomData<&'a ()>);
impl<'a> VideoQueueFamilyProperties2KHRBuilder<'a> {
    #[inline]
    pub fn new() -> VideoQueueFamilyProperties2KHRBuilder<'a> {
        VideoQueueFamilyProperties2KHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn video_codec_operations(mut self, video_codec_operations: crate::extensions::khr_video_queue::VideoCodecOperationFlagsKHR) -> Self {
        self.0.video_codec_operations = video_codec_operations as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> VideoQueueFamilyProperties2KHR {
        self.0
    }
}
impl<'a> std::default::Default for VideoQueueFamilyProperties2KHRBuilder<'a> {
    fn default() -> VideoQueueFamilyProperties2KHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoQueueFamilyProperties2KHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoQueueFamilyProperties2KHRBuilder<'a> {
    type Target = VideoQueueFamilyProperties2KHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoQueueFamilyProperties2KHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoProfilesKHR.html) · Structure"]
#[doc(alias = "VkVideoProfilesKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoProfilesKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub profile_count: u32,
    pub p_profiles: *const crate::extensions::khr_video_queue::VideoProfileKHR,
}
impl Default for VideoProfilesKHR {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::VIDEO_PROFILES_KHR, p_next: std::ptr::null_mut(), profile_count: Default::default(), p_profiles: std::ptr::null() }
    }
}
impl std::fmt::Debug for VideoProfilesKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoProfilesKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("profile_count", &self.profile_count).field("p_profiles", &self.p_profiles).finish()
    }
}
impl VideoProfilesKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoProfilesKHRBuilder<'a> {
        VideoProfilesKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoProfilesKHR.html) · Builder of [`VideoProfilesKHR`]"]
#[repr(transparent)]
pub struct VideoProfilesKHRBuilder<'a>(VideoProfilesKHR, std::marker::PhantomData<&'a ()>);
impl<'a> VideoProfilesKHRBuilder<'a> {
    #[inline]
    pub fn new() -> VideoProfilesKHRBuilder<'a> {
        VideoProfilesKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn profile_count(mut self, profile_count: u32) -> Self {
        self.0.profile_count = profile_count as _;
        self
    }
    #[inline]
    pub fn profiles(mut self, profiles: &'a crate::extensions::khr_video_queue::VideoProfileKHR) -> Self {
        self.0.p_profiles = profiles as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> VideoProfilesKHR {
        self.0
    }
}
impl<'a> std::default::Default for VideoProfilesKHRBuilder<'a> {
    fn default() -> VideoProfilesKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoProfilesKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoProfilesKHRBuilder<'a> {
    type Target = VideoProfilesKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoProfilesKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceVideoFormatInfoKHR.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceVideoFormatInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceVideoFormatInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub image_usage: crate::vk1_0::ImageUsageFlags,
    pub p_video_profiles: *const crate::extensions::khr_video_queue::VideoProfilesKHR,
}
impl Default for PhysicalDeviceVideoFormatInfoKHR {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_VIDEO_FORMAT_INFO_KHR, p_next: std::ptr::null(), image_usage: Default::default(), p_video_profiles: std::ptr::null() }
    }
}
impl std::fmt::Debug for PhysicalDeviceVideoFormatInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceVideoFormatInfoKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("image_usage", &self.image_usage).field("p_video_profiles", &self.p_video_profiles).finish()
    }
}
impl PhysicalDeviceVideoFormatInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceVideoFormatInfoKHRBuilder<'a> {
        PhysicalDeviceVideoFormatInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceVideoFormatInfoKHR.html) · Builder of [`PhysicalDeviceVideoFormatInfoKHR`]"]
#[repr(transparent)]
pub struct PhysicalDeviceVideoFormatInfoKHRBuilder<'a>(PhysicalDeviceVideoFormatInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceVideoFormatInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceVideoFormatInfoKHRBuilder<'a> {
        PhysicalDeviceVideoFormatInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn image_usage(mut self, image_usage: crate::vk1_0::ImageUsageFlags) -> Self {
        self.0.image_usage = image_usage as _;
        self
    }
    #[inline]
    pub fn video_profiles(mut self, video_profiles: &'a crate::extensions::khr_video_queue::VideoProfilesKHR) -> Self {
        self.0.p_video_profiles = video_profiles as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceVideoFormatInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceVideoFormatInfoKHRBuilder<'a> {
    fn default() -> PhysicalDeviceVideoFormatInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceVideoFormatInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceVideoFormatInfoKHRBuilder<'a> {
    type Target = PhysicalDeviceVideoFormatInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceVideoFormatInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoFormatPropertiesKHR.html) · Structure"]
#[doc(alias = "VkVideoFormatPropertiesKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoFormatPropertiesKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub format: crate::vk1_0::Format,
}
impl Default for VideoFormatPropertiesKHR {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::VIDEO_FORMAT_PROPERTIES_KHR, p_next: std::ptr::null_mut(), format: Default::default() }
    }
}
impl std::fmt::Debug for VideoFormatPropertiesKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoFormatPropertiesKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("format", &self.format).finish()
    }
}
impl VideoFormatPropertiesKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoFormatPropertiesKHRBuilder<'a> {
        VideoFormatPropertiesKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoFormatPropertiesKHR.html) · Builder of [`VideoFormatPropertiesKHR`]"]
#[repr(transparent)]
pub struct VideoFormatPropertiesKHRBuilder<'a>(VideoFormatPropertiesKHR, std::marker::PhantomData<&'a ()>);
impl<'a> VideoFormatPropertiesKHRBuilder<'a> {
    #[inline]
    pub fn new() -> VideoFormatPropertiesKHRBuilder<'a> {
        VideoFormatPropertiesKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn format(mut self, format: crate::vk1_0::Format) -> Self {
        self.0.format = format as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> VideoFormatPropertiesKHR {
        self.0
    }
}
impl<'a> std::default::Default for VideoFormatPropertiesKHRBuilder<'a> {
    fn default() -> VideoFormatPropertiesKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoFormatPropertiesKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoFormatPropertiesKHRBuilder<'a> {
    type Target = VideoFormatPropertiesKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoFormatPropertiesKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoProfileKHR.html) · Structure"]
#[doc(alias = "VkVideoProfileKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoProfileKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub video_codec_operation: crate::extensions::khr_video_queue::VideoCodecOperationFlagBitsKHR,
    pub chroma_subsampling: crate::extensions::khr_video_queue::VideoChromaSubsamplingFlagsKHR,
    pub luma_bit_depth: crate::extensions::khr_video_queue::VideoComponentBitDepthFlagsKHR,
    pub chroma_bit_depth: crate::extensions::khr_video_queue::VideoComponentBitDepthFlagsKHR,
}
impl Default for VideoProfileKHR {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::VIDEO_PROFILE_KHR, p_next: std::ptr::null_mut(), video_codec_operation: Default::default(), chroma_subsampling: Default::default(), luma_bit_depth: Default::default(), chroma_bit_depth: Default::default() }
    }
}
impl std::fmt::Debug for VideoProfileKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoProfileKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("video_codec_operation", &self.video_codec_operation).field("chroma_subsampling", &self.chroma_subsampling).field("luma_bit_depth", &self.luma_bit_depth).field("chroma_bit_depth", &self.chroma_bit_depth).finish()
    }
}
impl VideoProfileKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoProfileKHRBuilder<'a> {
        VideoProfileKHRBuilder(self, std::marker::PhantomData)
    }
}
impl<'a> crate::ExtendableFromMut<'a, crate::extensions::ext_video_decode_h264::VideoDecodeH264ProfileEXT> for VideoProfileKHRBuilder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, crate::extensions::ext_video_decode_h264::VideoDecodeH264ProfileEXTBuilder<'_>> for VideoProfileKHRBuilder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, crate::extensions::ext_video_decode_h265::VideoDecodeH265ProfileEXT> for VideoProfileKHRBuilder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, crate::extensions::ext_video_decode_h265::VideoDecodeH265ProfileEXTBuilder<'_>> for VideoProfileKHRBuilder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, crate::extensions::ext_video_encode_h264::VideoEncodeH264ProfileEXT> for VideoProfileKHRBuilder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, crate::extensions::ext_video_encode_h264::VideoEncodeH264ProfileEXTBuilder<'_>> for VideoProfileKHRBuilder<'a> {}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoProfileKHR.html) · Builder of [`VideoProfileKHR`]"]
#[repr(transparent)]
pub struct VideoProfileKHRBuilder<'a>(VideoProfileKHR, std::marker::PhantomData<&'a ()>);
impl<'a> VideoProfileKHRBuilder<'a> {
    #[inline]
    pub fn new() -> VideoProfileKHRBuilder<'a> {
        VideoProfileKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn video_codec_operation(mut self, video_codec_operation: crate::extensions::khr_video_queue::VideoCodecOperationFlagBitsKHR) -> Self {
        self.0.video_codec_operation = video_codec_operation as _;
        self
    }
    #[inline]
    pub fn chroma_subsampling(mut self, chroma_subsampling: crate::extensions::khr_video_queue::VideoChromaSubsamplingFlagsKHR) -> Self {
        self.0.chroma_subsampling = chroma_subsampling as _;
        self
    }
    #[inline]
    pub fn luma_bit_depth(mut self, luma_bit_depth: crate::extensions::khr_video_queue::VideoComponentBitDepthFlagsKHR) -> Self {
        self.0.luma_bit_depth = luma_bit_depth as _;
        self
    }
    #[inline]
    pub fn chroma_bit_depth(mut self, chroma_bit_depth: crate::extensions::khr_video_queue::VideoComponentBitDepthFlagsKHR) -> Self {
        self.0.chroma_bit_depth = chroma_bit_depth as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> VideoProfileKHR {
        self.0
    }
}
impl<'a> std::default::Default for VideoProfileKHRBuilder<'a> {
    fn default() -> VideoProfileKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoProfileKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoProfileKHRBuilder<'a> {
    type Target = VideoProfileKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoProfileKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoCapabilitiesKHR.html) · Structure"]
#[doc(alias = "VkVideoCapabilitiesKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoCapabilitiesKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub capability_flags: crate::extensions::khr_video_queue::VideoCapabilitiesFlagsKHR,
    pub min_bitstream_buffer_offset_alignment: crate::vk1_0::DeviceSize,
    pub min_bitstream_buffer_size_alignment: crate::vk1_0::DeviceSize,
    pub video_picture_extent_granularity: crate::vk1_0::Extent2D,
    pub min_extent: crate::vk1_0::Extent2D,
    pub max_extent: crate::vk1_0::Extent2D,
    pub max_reference_pictures_slots_count: u32,
    pub max_reference_pictures_active_count: u32,
}
impl Default for VideoCapabilitiesKHR {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::VIDEO_CAPABILITIES_KHR, p_next: std::ptr::null_mut(), capability_flags: Default::default(), min_bitstream_buffer_offset_alignment: Default::default(), min_bitstream_buffer_size_alignment: Default::default(), video_picture_extent_granularity: Default::default(), min_extent: Default::default(), max_extent: Default::default(), max_reference_pictures_slots_count: Default::default(), max_reference_pictures_active_count: Default::default() }
    }
}
impl std::fmt::Debug for VideoCapabilitiesKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoCapabilitiesKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("capability_flags", &self.capability_flags).field("min_bitstream_buffer_offset_alignment", &self.min_bitstream_buffer_offset_alignment).field("min_bitstream_buffer_size_alignment", &self.min_bitstream_buffer_size_alignment).field("video_picture_extent_granularity", &self.video_picture_extent_granularity).field("min_extent", &self.min_extent).field("max_extent", &self.max_extent).field("max_reference_pictures_slots_count", &self.max_reference_pictures_slots_count).field("max_reference_pictures_active_count", &self.max_reference_pictures_active_count).finish()
    }
}
impl VideoCapabilitiesKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoCapabilitiesKHRBuilder<'a> {
        VideoCapabilitiesKHRBuilder(self, std::marker::PhantomData)
    }
}
impl<'a> crate::ExtendableFromMut<'a, crate::extensions::ext_video_decode_h264::VideoDecodeH264CapabilitiesEXT> for VideoCapabilitiesKHRBuilder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, crate::extensions::ext_video_decode_h264::VideoDecodeH264CapabilitiesEXTBuilder<'_>> for VideoCapabilitiesKHRBuilder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, crate::extensions::ext_video_decode_h265::VideoDecodeH265CapabilitiesEXT> for VideoCapabilitiesKHRBuilder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, crate::extensions::ext_video_decode_h265::VideoDecodeH265CapabilitiesEXTBuilder<'_>> for VideoCapabilitiesKHRBuilder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, crate::extensions::ext_video_encode_h264::VideoEncodeH264CapabilitiesEXT> for VideoCapabilitiesKHRBuilder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, crate::extensions::ext_video_encode_h264::VideoEncodeH264CapabilitiesEXTBuilder<'_>> for VideoCapabilitiesKHRBuilder<'a> {}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoCapabilitiesKHR.html) · Builder of [`VideoCapabilitiesKHR`]"]
#[repr(transparent)]
pub struct VideoCapabilitiesKHRBuilder<'a>(VideoCapabilitiesKHR, std::marker::PhantomData<&'a ()>);
impl<'a> VideoCapabilitiesKHRBuilder<'a> {
    #[inline]
    pub fn new() -> VideoCapabilitiesKHRBuilder<'a> {
        VideoCapabilitiesKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn capability_flags(mut self, capability_flags: crate::extensions::khr_video_queue::VideoCapabilitiesFlagsKHR) -> Self {
        self.0.capability_flags = capability_flags as _;
        self
    }
    #[inline]
    pub fn min_bitstream_buffer_offset_alignment(mut self, min_bitstream_buffer_offset_alignment: crate::vk1_0::DeviceSize) -> Self {
        self.0.min_bitstream_buffer_offset_alignment = min_bitstream_buffer_offset_alignment as _;
        self
    }
    #[inline]
    pub fn min_bitstream_buffer_size_alignment(mut self, min_bitstream_buffer_size_alignment: crate::vk1_0::DeviceSize) -> Self {
        self.0.min_bitstream_buffer_size_alignment = min_bitstream_buffer_size_alignment as _;
        self
    }
    #[inline]
    pub fn video_picture_extent_granularity(mut self, video_picture_extent_granularity: crate::vk1_0::Extent2D) -> Self {
        self.0.video_picture_extent_granularity = video_picture_extent_granularity as _;
        self
    }
    #[inline]
    pub fn min_extent(mut self, min_extent: crate::vk1_0::Extent2D) -> Self {
        self.0.min_extent = min_extent as _;
        self
    }
    #[inline]
    pub fn max_extent(mut self, max_extent: crate::vk1_0::Extent2D) -> Self {
        self.0.max_extent = max_extent as _;
        self
    }
    #[inline]
    pub fn max_reference_pictures_slots_count(mut self, max_reference_pictures_slots_count: u32) -> Self {
        self.0.max_reference_pictures_slots_count = max_reference_pictures_slots_count as _;
        self
    }
    #[inline]
    pub fn max_reference_pictures_active_count(mut self, max_reference_pictures_active_count: u32) -> Self {
        self.0.max_reference_pictures_active_count = max_reference_pictures_active_count as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> VideoCapabilitiesKHR {
        self.0
    }
}
impl<'a> std::default::Default for VideoCapabilitiesKHRBuilder<'a> {
    fn default() -> VideoCapabilitiesKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoCapabilitiesKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoCapabilitiesKHRBuilder<'a> {
    type Target = VideoCapabilitiesKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoCapabilitiesKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoGetMemoryPropertiesKHR.html) · Structure"]
#[doc(alias = "VkVideoGetMemoryPropertiesKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoGetMemoryPropertiesKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub memory_bind_index: u32,
    pub p_memory_requirements: *mut crate::vk1_1::MemoryRequirements2,
}
impl Default for VideoGetMemoryPropertiesKHR {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::VIDEO_GET_MEMORY_PROPERTIES_KHR, p_next: std::ptr::null(), memory_bind_index: Default::default(), p_memory_requirements: std::ptr::null_mut() }
    }
}
impl std::fmt::Debug for VideoGetMemoryPropertiesKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoGetMemoryPropertiesKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("memory_bind_index", &self.memory_bind_index).field("p_memory_requirements", &self.p_memory_requirements).finish()
    }
}
impl VideoGetMemoryPropertiesKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoGetMemoryPropertiesKHRBuilder<'a> {
        VideoGetMemoryPropertiesKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoGetMemoryPropertiesKHR.html) · Builder of [`VideoGetMemoryPropertiesKHR`]"]
#[repr(transparent)]
pub struct VideoGetMemoryPropertiesKHRBuilder<'a>(VideoGetMemoryPropertiesKHR, std::marker::PhantomData<&'a ()>);
impl<'a> VideoGetMemoryPropertiesKHRBuilder<'a> {
    #[inline]
    pub fn new() -> VideoGetMemoryPropertiesKHRBuilder<'a> {
        VideoGetMemoryPropertiesKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn memory_bind_index(mut self, memory_bind_index: u32) -> Self {
        self.0.memory_bind_index = memory_bind_index as _;
        self
    }
    #[inline]
    pub fn memory_requirements(mut self, memory_requirements: &'a mut crate::vk1_1::MemoryRequirements2) -> Self {
        self.0.p_memory_requirements = memory_requirements as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> VideoGetMemoryPropertiesKHR {
        self.0
    }
}
impl<'a> std::default::Default for VideoGetMemoryPropertiesKHRBuilder<'a> {
    fn default() -> VideoGetMemoryPropertiesKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoGetMemoryPropertiesKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoGetMemoryPropertiesKHRBuilder<'a> {
    type Target = VideoGetMemoryPropertiesKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoGetMemoryPropertiesKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoBindMemoryKHR.html) · Structure"]
#[doc(alias = "VkVideoBindMemoryKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoBindMemoryKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub memory_bind_index: u32,
    pub memory: crate::vk1_0::DeviceMemory,
    pub memory_offset: crate::vk1_0::DeviceSize,
    pub memory_size: crate::vk1_0::DeviceSize,
}
impl Default for VideoBindMemoryKHR {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::VIDEO_BIND_MEMORY_KHR, p_next: std::ptr::null(), memory_bind_index: Default::default(), memory: Default::default(), memory_offset: Default::default(), memory_size: Default::default() }
    }
}
impl std::fmt::Debug for VideoBindMemoryKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoBindMemoryKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("memory_bind_index", &self.memory_bind_index).field("memory", &self.memory).field("memory_offset", &self.memory_offset).field("memory_size", &self.memory_size).finish()
    }
}
impl VideoBindMemoryKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoBindMemoryKHRBuilder<'a> {
        VideoBindMemoryKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoBindMemoryKHR.html) · Builder of [`VideoBindMemoryKHR`]"]
#[repr(transparent)]
pub struct VideoBindMemoryKHRBuilder<'a>(VideoBindMemoryKHR, std::marker::PhantomData<&'a ()>);
impl<'a> VideoBindMemoryKHRBuilder<'a> {
    #[inline]
    pub fn new() -> VideoBindMemoryKHRBuilder<'a> {
        VideoBindMemoryKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn memory_bind_index(mut self, memory_bind_index: u32) -> Self {
        self.0.memory_bind_index = memory_bind_index as _;
        self
    }
    #[inline]
    pub fn memory(mut self, memory: crate::vk1_0::DeviceMemory) -> Self {
        self.0.memory = memory as _;
        self
    }
    #[inline]
    pub fn memory_offset(mut self, memory_offset: crate::vk1_0::DeviceSize) -> Self {
        self.0.memory_offset = memory_offset as _;
        self
    }
    #[inline]
    pub fn memory_size(mut self, memory_size: crate::vk1_0::DeviceSize) -> Self {
        self.0.memory_size = memory_size as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> VideoBindMemoryKHR {
        self.0
    }
}
impl<'a> std::default::Default for VideoBindMemoryKHRBuilder<'a> {
    fn default() -> VideoBindMemoryKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoBindMemoryKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoBindMemoryKHRBuilder<'a> {
    type Target = VideoBindMemoryKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoBindMemoryKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoPictureResourceKHR.html) · Structure"]
#[doc(alias = "VkVideoPictureResourceKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoPictureResourceKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub coded_offset: crate::vk1_0::Offset2D,
    pub coded_extent: crate::vk1_0::Extent2D,
    pub base_array_layer: u32,
    pub image_view_binding: crate::vk1_0::ImageView,
}
impl Default for VideoPictureResourceKHR {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::VIDEO_PICTURE_RESOURCE_KHR, p_next: std::ptr::null(), coded_offset: Default::default(), coded_extent: Default::default(), base_array_layer: Default::default(), image_view_binding: Default::default() }
    }
}
impl std::fmt::Debug for VideoPictureResourceKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoPictureResourceKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("coded_offset", &self.coded_offset).field("coded_extent", &self.coded_extent).field("base_array_layer", &self.base_array_layer).field("image_view_binding", &self.image_view_binding).finish()
    }
}
impl VideoPictureResourceKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoPictureResourceKHRBuilder<'a> {
        VideoPictureResourceKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoPictureResourceKHR.html) · Builder of [`VideoPictureResourceKHR`]"]
#[repr(transparent)]
pub struct VideoPictureResourceKHRBuilder<'a>(VideoPictureResourceKHR, std::marker::PhantomData<&'a ()>);
impl<'a> VideoPictureResourceKHRBuilder<'a> {
    #[inline]
    pub fn new() -> VideoPictureResourceKHRBuilder<'a> {
        VideoPictureResourceKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn coded_offset(mut self, coded_offset: crate::vk1_0::Offset2D) -> Self {
        self.0.coded_offset = coded_offset as _;
        self
    }
    #[inline]
    pub fn coded_extent(mut self, coded_extent: crate::vk1_0::Extent2D) -> Self {
        self.0.coded_extent = coded_extent as _;
        self
    }
    #[inline]
    pub fn base_array_layer(mut self, base_array_layer: u32) -> Self {
        self.0.base_array_layer = base_array_layer as _;
        self
    }
    #[inline]
    pub fn image_view_binding(mut self, image_view_binding: crate::vk1_0::ImageView) -> Self {
        self.0.image_view_binding = image_view_binding as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> VideoPictureResourceKHR {
        self.0
    }
}
impl<'a> std::default::Default for VideoPictureResourceKHRBuilder<'a> {
    fn default() -> VideoPictureResourceKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoPictureResourceKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoPictureResourceKHRBuilder<'a> {
    type Target = VideoPictureResourceKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoPictureResourceKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoReferenceSlotKHR.html) · Structure"]
#[doc(alias = "VkVideoReferenceSlotKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoReferenceSlotKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub slot_index: i8,
    pub p_picture_resource: *const crate::extensions::khr_video_queue::VideoPictureResourceKHR,
}
impl Default for VideoReferenceSlotKHR {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::VIDEO_REFERENCE_SLOT_KHR, p_next: std::ptr::null(), slot_index: Default::default(), p_picture_resource: std::ptr::null() }
    }
}
impl std::fmt::Debug for VideoReferenceSlotKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoReferenceSlotKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("slot_index", &self.slot_index).field("p_picture_resource", &self.p_picture_resource).finish()
    }
}
impl VideoReferenceSlotKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoReferenceSlotKHRBuilder<'a> {
        VideoReferenceSlotKHRBuilder(self, std::marker::PhantomData)
    }
}
impl<'a> crate::ExtendableFromConst<'a, crate::extensions::ext_video_decode_h264::VideoDecodeH264DpbSlotInfoEXT> for VideoReferenceSlotKHRBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, crate::extensions::ext_video_decode_h264::VideoDecodeH264DpbSlotInfoEXTBuilder<'_>> for VideoReferenceSlotKHRBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, crate::extensions::ext_video_decode_h265::VideoDecodeH265DpbSlotInfoEXT> for VideoReferenceSlotKHRBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, crate::extensions::ext_video_decode_h265::VideoDecodeH265DpbSlotInfoEXTBuilder<'_>> for VideoReferenceSlotKHRBuilder<'a> {}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoReferenceSlotKHR.html) · Builder of [`VideoReferenceSlotKHR`]"]
#[repr(transparent)]
pub struct VideoReferenceSlotKHRBuilder<'a>(VideoReferenceSlotKHR, std::marker::PhantomData<&'a ()>);
impl<'a> VideoReferenceSlotKHRBuilder<'a> {
    #[inline]
    pub fn new() -> VideoReferenceSlotKHRBuilder<'a> {
        VideoReferenceSlotKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn slot_index(mut self, slot_index: i8) -> Self {
        self.0.slot_index = slot_index as _;
        self
    }
    #[inline]
    pub fn picture_resource(mut self, picture_resource: &'a crate::extensions::khr_video_queue::VideoPictureResourceKHR) -> Self {
        self.0.p_picture_resource = picture_resource as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> VideoReferenceSlotKHR {
        self.0
    }
}
impl<'a> std::default::Default for VideoReferenceSlotKHRBuilder<'a> {
    fn default() -> VideoReferenceSlotKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoReferenceSlotKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoReferenceSlotKHRBuilder<'a> {
    type Target = VideoReferenceSlotKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoReferenceSlotKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoSessionCreateInfoKHR.html) · Structure"]
#[doc(alias = "VkVideoSessionCreateInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoSessionCreateInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub queue_family_index: u32,
    pub flags: crate::extensions::khr_video_queue::VideoSessionCreateFlagsKHR,
    pub p_video_profile: *const crate::extensions::khr_video_queue::VideoProfileKHR,
    pub picture_format: crate::vk1_0::Format,
    pub max_coded_extent: crate::vk1_0::Extent2D,
    pub reference_pictures_format: crate::vk1_0::Format,
    pub max_reference_pictures_slots_count: u32,
    pub max_reference_pictures_active_count: u32,
}
impl Default for VideoSessionCreateInfoKHR {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::VIDEO_SESSION_CREATE_INFO_KHR, p_next: std::ptr::null(), queue_family_index: Default::default(), flags: Default::default(), p_video_profile: std::ptr::null(), picture_format: Default::default(), max_coded_extent: Default::default(), reference_pictures_format: Default::default(), max_reference_pictures_slots_count: Default::default(), max_reference_pictures_active_count: Default::default() }
    }
}
impl std::fmt::Debug for VideoSessionCreateInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoSessionCreateInfoKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("queue_family_index", &self.queue_family_index).field("flags", &self.flags).field("p_video_profile", &self.p_video_profile).field("picture_format", &self.picture_format).field("max_coded_extent", &self.max_coded_extent).field("reference_pictures_format", &self.reference_pictures_format).field("max_reference_pictures_slots_count", &self.max_reference_pictures_slots_count).field("max_reference_pictures_active_count", &self.max_reference_pictures_active_count).finish()
    }
}
impl VideoSessionCreateInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoSessionCreateInfoKHRBuilder<'a> {
        VideoSessionCreateInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl<'a> crate::ExtendableFromConst<'a, crate::extensions::ext_video_decode_h264::VideoDecodeH264SessionCreateInfoEXT> for VideoSessionCreateInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, crate::extensions::ext_video_decode_h264::VideoDecodeH264SessionCreateInfoEXTBuilder<'_>> for VideoSessionCreateInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, crate::extensions::ext_video_decode_h265::VideoDecodeH265SessionCreateInfoEXT> for VideoSessionCreateInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, crate::extensions::ext_video_decode_h265::VideoDecodeH265SessionCreateInfoEXTBuilder<'_>> for VideoSessionCreateInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, crate::extensions::ext_video_encode_h264::VideoEncodeH264SessionCreateInfoEXT> for VideoSessionCreateInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, crate::extensions::ext_video_encode_h264::VideoEncodeH264SessionCreateInfoEXTBuilder<'_>> for VideoSessionCreateInfoKHRBuilder<'a> {}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoSessionCreateInfoKHR.html) · Builder of [`VideoSessionCreateInfoKHR`]"]
#[repr(transparent)]
pub struct VideoSessionCreateInfoKHRBuilder<'a>(VideoSessionCreateInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> VideoSessionCreateInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> VideoSessionCreateInfoKHRBuilder<'a> {
        VideoSessionCreateInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn queue_family_index(mut self, queue_family_index: u32) -> Self {
        self.0.queue_family_index = queue_family_index as _;
        self
    }
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::khr_video_queue::VideoSessionCreateFlagsKHR) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn video_profile(mut self, video_profile: &'a crate::extensions::khr_video_queue::VideoProfileKHR) -> Self {
        self.0.p_video_profile = video_profile as _;
        self
    }
    #[inline]
    pub fn picture_format(mut self, picture_format: crate::vk1_0::Format) -> Self {
        self.0.picture_format = picture_format as _;
        self
    }
    #[inline]
    pub fn max_coded_extent(mut self, max_coded_extent: crate::vk1_0::Extent2D) -> Self {
        self.0.max_coded_extent = max_coded_extent as _;
        self
    }
    #[inline]
    pub fn reference_pictures_format(mut self, reference_pictures_format: crate::vk1_0::Format) -> Self {
        self.0.reference_pictures_format = reference_pictures_format as _;
        self
    }
    #[inline]
    pub fn max_reference_pictures_slots_count(mut self, max_reference_pictures_slots_count: u32) -> Self {
        self.0.max_reference_pictures_slots_count = max_reference_pictures_slots_count as _;
        self
    }
    #[inline]
    pub fn max_reference_pictures_active_count(mut self, max_reference_pictures_active_count: u32) -> Self {
        self.0.max_reference_pictures_active_count = max_reference_pictures_active_count as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> VideoSessionCreateInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for VideoSessionCreateInfoKHRBuilder<'a> {
    fn default() -> VideoSessionCreateInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoSessionCreateInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoSessionCreateInfoKHRBuilder<'a> {
    type Target = VideoSessionCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoSessionCreateInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoSessionParametersCreateInfoKHR.html) · Structure"]
#[doc(alias = "VkVideoSessionParametersCreateInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoSessionParametersCreateInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub video_session_parameters_template: crate::extensions::khr_video_queue::VideoSessionParametersKHR,
    pub video_session: crate::extensions::khr_video_queue::VideoSessionKHR,
}
impl Default for VideoSessionParametersCreateInfoKHR {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::VIDEO_SESSION_PARAMETERS_CREATE_INFO_KHR, p_next: std::ptr::null(), video_session_parameters_template: Default::default(), video_session: Default::default() }
    }
}
impl std::fmt::Debug for VideoSessionParametersCreateInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoSessionParametersCreateInfoKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("video_session_parameters_template", &self.video_session_parameters_template).field("video_session", &self.video_session).finish()
    }
}
impl VideoSessionParametersCreateInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoSessionParametersCreateInfoKHRBuilder<'a> {
        VideoSessionParametersCreateInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl<'a> crate::ExtendableFromConst<'a, crate::extensions::ext_video_decode_h264::VideoDecodeH264SessionParametersCreateInfoEXT> for VideoSessionParametersCreateInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, crate::extensions::ext_video_decode_h264::VideoDecodeH264SessionParametersCreateInfoEXTBuilder<'_>> for VideoSessionParametersCreateInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, crate::extensions::ext_video_decode_h265::VideoDecodeH265SessionParametersCreateInfoEXT> for VideoSessionParametersCreateInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, crate::extensions::ext_video_decode_h265::VideoDecodeH265SessionParametersCreateInfoEXTBuilder<'_>> for VideoSessionParametersCreateInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, crate::extensions::ext_video_encode_h264::VideoEncodeH264SessionParametersCreateInfoEXT> for VideoSessionParametersCreateInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, crate::extensions::ext_video_encode_h264::VideoEncodeH264SessionParametersCreateInfoEXTBuilder<'_>> for VideoSessionParametersCreateInfoKHRBuilder<'a> {}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoSessionParametersCreateInfoKHR.html) · Builder of [`VideoSessionParametersCreateInfoKHR`]"]
#[repr(transparent)]
pub struct VideoSessionParametersCreateInfoKHRBuilder<'a>(VideoSessionParametersCreateInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> VideoSessionParametersCreateInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> VideoSessionParametersCreateInfoKHRBuilder<'a> {
        VideoSessionParametersCreateInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn video_session_parameters_template(mut self, video_session_parameters_template: crate::extensions::khr_video_queue::VideoSessionParametersKHR) -> Self {
        self.0.video_session_parameters_template = video_session_parameters_template as _;
        self
    }
    #[inline]
    pub fn video_session(mut self, video_session: crate::extensions::khr_video_queue::VideoSessionKHR) -> Self {
        self.0.video_session = video_session as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> VideoSessionParametersCreateInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for VideoSessionParametersCreateInfoKHRBuilder<'a> {
    fn default() -> VideoSessionParametersCreateInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoSessionParametersCreateInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoSessionParametersCreateInfoKHRBuilder<'a> {
    type Target = VideoSessionParametersCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoSessionParametersCreateInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoSessionParametersUpdateInfoKHR.html) · Structure"]
#[doc(alias = "VkVideoSessionParametersUpdateInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoSessionParametersUpdateInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub update_sequence_count: u32,
}
impl Default for VideoSessionParametersUpdateInfoKHR {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::VIDEO_SESSION_PARAMETERS_UPDATE_INFO_KHR, p_next: std::ptr::null(), update_sequence_count: Default::default() }
    }
}
impl std::fmt::Debug for VideoSessionParametersUpdateInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoSessionParametersUpdateInfoKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("update_sequence_count", &self.update_sequence_count).finish()
    }
}
impl VideoSessionParametersUpdateInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoSessionParametersUpdateInfoKHRBuilder<'a> {
        VideoSessionParametersUpdateInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl<'a> crate::ExtendableFromConst<'a, crate::extensions::ext_video_decode_h264::VideoDecodeH264SessionParametersAddInfoEXT> for VideoSessionParametersUpdateInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, crate::extensions::ext_video_decode_h264::VideoDecodeH264SessionParametersAddInfoEXTBuilder<'_>> for VideoSessionParametersUpdateInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, crate::extensions::ext_video_decode_h265::VideoDecodeH265SessionParametersAddInfoEXT> for VideoSessionParametersUpdateInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, crate::extensions::ext_video_decode_h265::VideoDecodeH265SessionParametersAddInfoEXTBuilder<'_>> for VideoSessionParametersUpdateInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, crate::extensions::ext_video_encode_h264::VideoEncodeH264SessionParametersAddInfoEXT> for VideoSessionParametersUpdateInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, crate::extensions::ext_video_encode_h264::VideoEncodeH264SessionParametersAddInfoEXTBuilder<'_>> for VideoSessionParametersUpdateInfoKHRBuilder<'a> {}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoSessionParametersUpdateInfoKHR.html) · Builder of [`VideoSessionParametersUpdateInfoKHR`]"]
#[repr(transparent)]
pub struct VideoSessionParametersUpdateInfoKHRBuilder<'a>(VideoSessionParametersUpdateInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> VideoSessionParametersUpdateInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> VideoSessionParametersUpdateInfoKHRBuilder<'a> {
        VideoSessionParametersUpdateInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn update_sequence_count(mut self, update_sequence_count: u32) -> Self {
        self.0.update_sequence_count = update_sequence_count as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> VideoSessionParametersUpdateInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for VideoSessionParametersUpdateInfoKHRBuilder<'a> {
    fn default() -> VideoSessionParametersUpdateInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoSessionParametersUpdateInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoSessionParametersUpdateInfoKHRBuilder<'a> {
    type Target = VideoSessionParametersUpdateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoSessionParametersUpdateInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoBeginCodingInfoKHR.html) · Structure"]
#[doc(alias = "VkVideoBeginCodingInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoBeginCodingInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::khr_video_queue::VideoBeginCodingFlagsKHR,
    pub codec_quality_preset: crate::extensions::khr_video_queue::VideoCodingQualityPresetFlagsKHR,
    pub video_session: crate::extensions::khr_video_queue::VideoSessionKHR,
    pub video_session_parameters: crate::extensions::khr_video_queue::VideoSessionParametersKHR,
    pub reference_slot_count: u32,
    pub p_reference_slots: *const crate::extensions::khr_video_queue::VideoReferenceSlotKHR,
}
impl Default for VideoBeginCodingInfoKHR {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::VIDEO_BEGIN_CODING_INFO_KHR, p_next: std::ptr::null(), flags: Default::default(), codec_quality_preset: Default::default(), video_session: Default::default(), video_session_parameters: Default::default(), reference_slot_count: Default::default(), p_reference_slots: std::ptr::null() }
    }
}
impl std::fmt::Debug for VideoBeginCodingInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoBeginCodingInfoKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("codec_quality_preset", &self.codec_quality_preset).field("video_session", &self.video_session).field("video_session_parameters", &self.video_session_parameters).field("reference_slot_count", &self.reference_slot_count).field("p_reference_slots", &self.p_reference_slots).finish()
    }
}
impl VideoBeginCodingInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoBeginCodingInfoKHRBuilder<'a> {
        VideoBeginCodingInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoBeginCodingInfoKHR.html) · Builder of [`VideoBeginCodingInfoKHR`]"]
#[repr(transparent)]
pub struct VideoBeginCodingInfoKHRBuilder<'a>(VideoBeginCodingInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> VideoBeginCodingInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> VideoBeginCodingInfoKHRBuilder<'a> {
        VideoBeginCodingInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::khr_video_queue::VideoBeginCodingFlagsKHR) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn codec_quality_preset(mut self, codec_quality_preset: crate::extensions::khr_video_queue::VideoCodingQualityPresetFlagsKHR) -> Self {
        self.0.codec_quality_preset = codec_quality_preset as _;
        self
    }
    #[inline]
    pub fn video_session(mut self, video_session: crate::extensions::khr_video_queue::VideoSessionKHR) -> Self {
        self.0.video_session = video_session as _;
        self
    }
    #[inline]
    pub fn video_session_parameters(mut self, video_session_parameters: crate::extensions::khr_video_queue::VideoSessionParametersKHR) -> Self {
        self.0.video_session_parameters = video_session_parameters as _;
        self
    }
    #[inline]
    pub fn reference_slots(mut self, reference_slots: &'a [crate::extensions::khr_video_queue::VideoReferenceSlotKHRBuilder]) -> Self {
        self.0.p_reference_slots = reference_slots.as_ptr() as _;
        self.0.reference_slot_count = reference_slots.len() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> VideoBeginCodingInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for VideoBeginCodingInfoKHRBuilder<'a> {
    fn default() -> VideoBeginCodingInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoBeginCodingInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoBeginCodingInfoKHRBuilder<'a> {
    type Target = VideoBeginCodingInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoBeginCodingInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEndCodingInfoKHR.html) · Structure"]
#[doc(alias = "VkVideoEndCodingInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoEndCodingInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::khr_video_queue::VideoEndCodingFlagsKHR,
}
impl Default for VideoEndCodingInfoKHR {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::VIDEO_END_CODING_INFO_KHR, p_next: std::ptr::null(), flags: Default::default() }
    }
}
impl std::fmt::Debug for VideoEndCodingInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoEndCodingInfoKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).finish()
    }
}
impl VideoEndCodingInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoEndCodingInfoKHRBuilder<'a> {
        VideoEndCodingInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEndCodingInfoKHR.html) · Builder of [`VideoEndCodingInfoKHR`]"]
#[repr(transparent)]
pub struct VideoEndCodingInfoKHRBuilder<'a>(VideoEndCodingInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> VideoEndCodingInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> VideoEndCodingInfoKHRBuilder<'a> {
        VideoEndCodingInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::khr_video_queue::VideoEndCodingFlagsKHR) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> VideoEndCodingInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for VideoEndCodingInfoKHRBuilder<'a> {
    fn default() -> VideoEndCodingInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoEndCodingInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoEndCodingInfoKHRBuilder<'a> {
    type Target = VideoEndCodingInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoEndCodingInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoCodingControlInfoKHR.html) · Structure"]
#[doc(alias = "VkVideoCodingControlInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoCodingControlInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::khr_video_queue::VideoCodingControlFlagsKHR,
}
impl Default for VideoCodingControlInfoKHR {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::VIDEO_CODING_CONTROL_INFO_KHR, p_next: std::ptr::null(), flags: Default::default() }
    }
}
impl std::fmt::Debug for VideoCodingControlInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoCodingControlInfoKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).finish()
    }
}
impl VideoCodingControlInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoCodingControlInfoKHRBuilder<'a> {
        VideoCodingControlInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl<'a> crate::ExtendableFromConst<'a, crate::extensions::khr_video_encode_queue::VideoEncodeRateControlInfoKHR> for VideoCodingControlInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, crate::extensions::khr_video_encode_queue::VideoEncodeRateControlInfoKHRBuilder<'_>> for VideoCodingControlInfoKHRBuilder<'a> {}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoCodingControlInfoKHR.html) · Builder of [`VideoCodingControlInfoKHR`]"]
#[repr(transparent)]
pub struct VideoCodingControlInfoKHRBuilder<'a>(VideoCodingControlInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> VideoCodingControlInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> VideoCodingControlInfoKHRBuilder<'a> {
        VideoCodingControlInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::khr_video_queue::VideoCodingControlFlagsKHR) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> VideoCodingControlInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for VideoCodingControlInfoKHRBuilder<'a> {
    fn default() -> VideoCodingControlInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoCodingControlInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoCodingControlInfoKHRBuilder<'a> {
    type Target = VideoCodingControlInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoCodingControlInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::khr_video_queue`]"]
impl crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceVideoCapabilitiesKHR.html) · Function"]
    #[doc(alias = "vkGetPhysicalDeviceVideoCapabilitiesKHR")]
    pub unsafe fn get_physical_device_video_capabilities_khr(&self, physical_device: crate::vk1_0::PhysicalDevice, video_profile: &crate::extensions::khr_video_queue::VideoProfileKHR, capabilities: Option<crate::extensions::khr_video_queue::VideoCapabilitiesKHR>) -> crate::utils::VulkanResult<crate::extensions::khr_video_queue::VideoCapabilitiesKHR> {
        let _function = self.get_physical_device_video_capabilities_khr.expect("`get_physical_device_video_capabilities_khr` is not loaded");
        let mut capabilities = match capabilities {
            Some(v) => v,
            None => Default::default(),
        };
        let _return = _function(physical_device as _, video_profile as _, &mut capabilities);
        crate::utils::VulkanResult::new(_return, capabilities)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceVideoFormatPropertiesKHR.html) · Function"]
    #[doc(alias = "vkGetPhysicalDeviceVideoFormatPropertiesKHR")]
    pub unsafe fn get_physical_device_video_format_properties_khr(&self, physical_device: crate::vk1_0::PhysicalDevice, video_format_info: &crate::extensions::khr_video_queue::PhysicalDeviceVideoFormatInfoKHR, video_format_property_count: Option<u32>) -> crate::utils::VulkanResult<Vec<crate::extensions::khr_video_queue::VideoFormatPropertiesKHR>> {
        let _function = self.get_physical_device_video_format_properties_khr.expect("`get_physical_device_video_format_properties_khr` is not loaded");
        let mut video_format_property_count = match video_format_property_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                _function(physical_device as _, video_format_info as _, &mut v, std::ptr::null_mut());
                v
            }
        };
        let mut video_format_properties = vec![Default::default(); video_format_property_count as _];
        let _return = _function(physical_device as _, video_format_info as _, &mut video_format_property_count, video_format_properties.as_mut_ptr());
        crate::utils::VulkanResult::new(_return, video_format_properties)
    }
}
#[doc = "Provided by [`crate::extensions::khr_video_queue`]"]
impl crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateVideoSessionKHR.html) · Function"]
    #[doc(alias = "vkCreateVideoSessionKHR")]
    pub unsafe fn create_video_session_khr(&self, create_info: &crate::extensions::khr_video_queue::VideoSessionCreateInfoKHR, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> crate::utils::VulkanResult<crate::extensions::khr_video_queue::VideoSessionKHR> {
        let _function = self.create_video_session_khr.expect("`create_video_session_khr` is not loaded");
        let mut video_session = Default::default();
        let _return = _function(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut video_session,
        );
        crate::utils::VulkanResult::new(_return, video_session)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyVideoSessionKHR.html) · Function"]
    #[doc(alias = "vkDestroyVideoSessionKHR")]
    pub unsafe fn destroy_video_session_khr(&self, video_session: crate::extensions::khr_video_queue::VideoSessionKHR, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> () {
        let _function = self.destroy_video_session_khr.expect("`destroy_video_session_khr` is not loaded");
        let _return = _function(
            self.handle,
            video_session as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateVideoSessionParametersKHR.html) · Function"]
    #[doc(alias = "vkCreateVideoSessionParametersKHR")]
    pub unsafe fn create_video_session_parameters_khr(&self, create_info: &crate::extensions::khr_video_queue::VideoSessionParametersCreateInfoKHR, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> crate::utils::VulkanResult<crate::extensions::khr_video_queue::VideoSessionParametersKHR> {
        let _function = self.create_video_session_parameters_khr.expect("`create_video_session_parameters_khr` is not loaded");
        let mut video_session_parameters = Default::default();
        let _return = _function(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut video_session_parameters,
        );
        crate::utils::VulkanResult::new(_return, video_session_parameters)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkUpdateVideoSessionParametersKHR.html) · Function"]
    #[doc(alias = "vkUpdateVideoSessionParametersKHR")]
    pub unsafe fn update_video_session_parameters_khr(&self, video_session_parameters: crate::extensions::khr_video_queue::VideoSessionParametersKHR, update_info: &crate::extensions::khr_video_queue::VideoSessionParametersUpdateInfoKHR) -> crate::utils::VulkanResult<()> {
        let _function = self.update_video_session_parameters_khr.expect("`update_video_session_parameters_khr` is not loaded");
        let _return = _function(self.handle, video_session_parameters as _, update_info as _);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyVideoSessionParametersKHR.html) · Function"]
    #[doc(alias = "vkDestroyVideoSessionParametersKHR")]
    pub unsafe fn destroy_video_session_parameters_khr(&self, video_session_parameters: crate::extensions::khr_video_queue::VideoSessionParametersKHR, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> () {
        let _function = self.destroy_video_session_parameters_khr.expect("`destroy_video_session_parameters_khr` is not loaded");
        let _return = _function(
            self.handle,
            video_session_parameters as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetVideoSessionMemoryRequirementsKHR.html) · Function"]
    #[doc(alias = "vkGetVideoSessionMemoryRequirementsKHR")]
    pub unsafe fn get_video_session_memory_requirements_khr(&self, video_session: crate::extensions::khr_video_queue::VideoSessionKHR, video_session_memory_requirements_count: Option<u32>) -> crate::utils::VulkanResult<Vec<crate::extensions::khr_video_queue::VideoGetMemoryPropertiesKHR>> {
        let _function = self.get_video_session_memory_requirements_khr.expect("`get_video_session_memory_requirements_khr` is not loaded");
        let mut video_session_memory_requirements_count = match video_session_memory_requirements_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                _function(self.handle, video_session as _, &mut v, std::ptr::null_mut());
                v
            }
        };
        let mut video_session_memory_requirements = vec![Default::default(); video_session_memory_requirements_count as _];
        let _return = _function(self.handle, video_session as _, &mut video_session_memory_requirements_count, video_session_memory_requirements.as_mut_ptr());
        crate::utils::VulkanResult::new(_return, video_session_memory_requirements)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBindVideoSessionMemoryKHR.html) · Function"]
    #[doc(alias = "vkBindVideoSessionMemoryKHR")]
    pub unsafe fn bind_video_session_memory_khr(&self, video_session: crate::extensions::khr_video_queue::VideoSessionKHR, video_session_bind_memories: &[crate::extensions::khr_video_queue::VideoBindMemoryKHRBuilder]) -> crate::utils::VulkanResult<()> {
        let _function = self.bind_video_session_memory_khr.expect("`bind_video_session_memory_khr` is not loaded");
        let video_session_bind_memory_count = video_session_bind_memories.len();
        let _return = _function(self.handle, video_session as _, video_session_bind_memory_count as _, video_session_bind_memories.as_ptr() as _);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBeginVideoCodingKHR.html) · Function"]
    #[doc(alias = "vkCmdBeginVideoCodingKHR")]
    pub unsafe fn cmd_begin_video_coding_khr(&self, command_buffer: crate::vk1_0::CommandBuffer, begin_info: &crate::extensions::khr_video_queue::VideoBeginCodingInfoKHR) -> () {
        let _function = self.cmd_begin_video_coding_khr.expect("`cmd_begin_video_coding_khr` is not loaded");
        let _return = _function(command_buffer as _, begin_info as _);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdControlVideoCodingKHR.html) · Function"]
    #[doc(alias = "vkCmdControlVideoCodingKHR")]
    pub unsafe fn cmd_control_video_coding_khr(&self, command_buffer: crate::vk1_0::CommandBuffer, coding_control_info: &crate::extensions::khr_video_queue::VideoCodingControlInfoKHR) -> () {
        let _function = self.cmd_control_video_coding_khr.expect("`cmd_control_video_coding_khr` is not loaded");
        let _return = _function(command_buffer as _, coding_control_info as _);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdEndVideoCodingKHR.html) · Function"]
    #[doc(alias = "vkCmdEndVideoCodingKHR")]
    pub unsafe fn cmd_end_video_coding_khr(&self, command_buffer: crate::vk1_0::CommandBuffer, end_coding_info: &crate::extensions::khr_video_queue::VideoEndCodingInfoKHR) -> () {
        let _function = self.cmd_end_video_coding_khr.expect("`cmd_end_video_coding_khr` is not loaded");
        let _return = _function(command_buffer as _, end_coding_info as _);
        ()
    }
}
