#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_VIDEO_ENCODE_H264_SPEC_VERSION")]
pub const EXT_VIDEO_ENCODE_H264_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_VIDEO_ENCODE_H264_EXTENSION_NAME")]
pub const EXT_VIDEO_ENCODE_H264_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_video_encode_h264");
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH264CapabilitiesFlagsEXT.html) · Bitmask of [`VideoEncodeH264CapabilitiesFlagBitsEXT`]"] # [doc (alias = "VkVideoEncodeH264CapabilitiesFlagsEXT")] # [derive (Default)] # [repr (transparent)] pub struct VideoEncodeH264CapabilitiesFlagsEXT : u32 { const VIDEO_ENCODE_H264_CAPABILITY_CABAC_EXT = VideoEncodeH264CapabilitiesFlagBitsEXT :: VIDEO_ENCODE_H264_CAPABILITY_CABAC_EXT . 0 ; const VIDEO_ENCODE_H264_CAPABILITY_CAVLC_EXT = VideoEncodeH264CapabilitiesFlagBitsEXT :: VIDEO_ENCODE_H264_CAPABILITY_CAVLC_EXT . 0 ; const VIDEO_ENCODE_H264_CAPABILITY_WEIGHTED_BI_PRED_IMPLICIT_EXT = VideoEncodeH264CapabilitiesFlagBitsEXT :: VIDEO_ENCODE_H264_CAPABILITY_WEIGHTED_BI_PRED_IMPLICIT_EXT . 0 ; const VIDEO_ENCODE_H264_CAPABILITY_TRANSFORM_8X8_EXT = VideoEncodeH264CapabilitiesFlagBitsEXT :: VIDEO_ENCODE_H264_CAPABILITY_TRANSFORM_8X8_EXT . 0 ; const VIDEO_ENCODE_H264_CAPABILITY_CHROMA_QP_OFFSET_EXT = VideoEncodeH264CapabilitiesFlagBitsEXT :: VIDEO_ENCODE_H264_CAPABILITY_CHROMA_QP_OFFSET_EXT . 0 ; const VIDEO_ENCODE_H264_CAPABILITY_SECOND_CHROMA_QP_OFFSET_EXT = VideoEncodeH264CapabilitiesFlagBitsEXT :: VIDEO_ENCODE_H264_CAPABILITY_SECOND_CHROMA_QP_OFFSET_EXT . 0 ; const VIDEO_ENCODE_H264_CAPABILITY_DEBLOCKING_FILTER_DISABLED_EXT = VideoEncodeH264CapabilitiesFlagBitsEXT :: VIDEO_ENCODE_H264_CAPABILITY_DEBLOCKING_FILTER_DISABLED_EXT . 0 ; const VIDEO_ENCODE_H264_CAPABILITY_DEBLOCKING_FILTER_ENABLED_EXT = VideoEncodeH264CapabilitiesFlagBitsEXT :: VIDEO_ENCODE_H264_CAPABILITY_DEBLOCKING_FILTER_ENABLED_EXT . 0 ; const VIDEO_ENCODE_H264_CAPABILITY_DEBLOCKING_FILTER_PARTIAL_EXT = VideoEncodeH264CapabilitiesFlagBitsEXT :: VIDEO_ENCODE_H264_CAPABILITY_DEBLOCKING_FILTER_PARTIAL_EXT . 0 ; const VIDEO_ENCODE_H264_CAPABILITY_MULTIPLE_SLICE_PER_FRAME_EXT = VideoEncodeH264CapabilitiesFlagBitsEXT :: VIDEO_ENCODE_H264_CAPABILITY_MULTIPLE_SLICE_PER_FRAME_EXT . 0 ; const VIDEO_ENCODE_H264_CAPABILITY_EVENLY_DISTRIBUTED_SLICE_SIZE_EXT = VideoEncodeH264CapabilitiesFlagBitsEXT :: VIDEO_ENCODE_H264_CAPABILITY_EVENLY_DISTRIBUTED_SLICE_SIZE_EXT . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH264CapabilitiesFlagBitsEXT.html) · Bits enum of [`VideoEncodeH264CapabilitiesFlagsEXT`]"]
#[doc(alias = "VkVideoEncodeH264CapabilitiesFlagBitsEXT")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct VideoEncodeH264CapabilitiesFlagBitsEXT(pub u32);
impl VideoEncodeH264CapabilitiesFlagBitsEXT {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> VideoEncodeH264CapabilitiesFlagsEXT {
        VideoEncodeH264CapabilitiesFlagsEXT::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for VideoEncodeH264CapabilitiesFlagBitsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::VIDEO_ENCODE_H264_CAPABILITY_CABAC_EXT => "VIDEO_ENCODE_H264_CAPABILITY_CABAC_EXT",
            &Self::VIDEO_ENCODE_H264_CAPABILITY_CAVLC_EXT => "VIDEO_ENCODE_H264_CAPABILITY_CAVLC_EXT",
            &Self::VIDEO_ENCODE_H264_CAPABILITY_WEIGHTED_BI_PRED_IMPLICIT_EXT => "VIDEO_ENCODE_H264_CAPABILITY_WEIGHTED_BI_PRED_IMPLICIT_EXT",
            &Self::VIDEO_ENCODE_H264_CAPABILITY_TRANSFORM_8X8_EXT => "VIDEO_ENCODE_H264_CAPABILITY_TRANSFORM_8X8_EXT",
            &Self::VIDEO_ENCODE_H264_CAPABILITY_CHROMA_QP_OFFSET_EXT => "VIDEO_ENCODE_H264_CAPABILITY_CHROMA_QP_OFFSET_EXT",
            &Self::VIDEO_ENCODE_H264_CAPABILITY_SECOND_CHROMA_QP_OFFSET_EXT => "VIDEO_ENCODE_H264_CAPABILITY_SECOND_CHROMA_QP_OFFSET_EXT",
            &Self::VIDEO_ENCODE_H264_CAPABILITY_DEBLOCKING_FILTER_DISABLED_EXT => "VIDEO_ENCODE_H264_CAPABILITY_DEBLOCKING_FILTER_DISABLED_EXT",
            &Self::VIDEO_ENCODE_H264_CAPABILITY_DEBLOCKING_FILTER_ENABLED_EXT => "VIDEO_ENCODE_H264_CAPABILITY_DEBLOCKING_FILTER_ENABLED_EXT",
            &Self::VIDEO_ENCODE_H264_CAPABILITY_DEBLOCKING_FILTER_PARTIAL_EXT => "VIDEO_ENCODE_H264_CAPABILITY_DEBLOCKING_FILTER_PARTIAL_EXT",
            &Self::VIDEO_ENCODE_H264_CAPABILITY_MULTIPLE_SLICE_PER_FRAME_EXT => "VIDEO_ENCODE_H264_CAPABILITY_MULTIPLE_SLICE_PER_FRAME_EXT",
            &Self::VIDEO_ENCODE_H264_CAPABILITY_EVENLY_DISTRIBUTED_SLICE_SIZE_EXT => "VIDEO_ENCODE_H264_CAPABILITY_EVENLY_DISTRIBUTED_SLICE_SIZE_EXT",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::ext_video_encode_h264`]"]
impl VideoEncodeH264CapabilitiesFlagBitsEXT {
    pub const VIDEO_ENCODE_H264_CAPABILITY_CABAC_EXT: Self = Self(1);
    pub const VIDEO_ENCODE_H264_CAPABILITY_CAVLC_EXT: Self = Self(2);
    pub const VIDEO_ENCODE_H264_CAPABILITY_WEIGHTED_BI_PRED_IMPLICIT_EXT: Self = Self(4);
    pub const VIDEO_ENCODE_H264_CAPABILITY_TRANSFORM_8X8_EXT: Self = Self(8);
    pub const VIDEO_ENCODE_H264_CAPABILITY_CHROMA_QP_OFFSET_EXT: Self = Self(16);
    pub const VIDEO_ENCODE_H264_CAPABILITY_SECOND_CHROMA_QP_OFFSET_EXT: Self = Self(32);
    pub const VIDEO_ENCODE_H264_CAPABILITY_DEBLOCKING_FILTER_DISABLED_EXT: Self = Self(64);
    pub const VIDEO_ENCODE_H264_CAPABILITY_DEBLOCKING_FILTER_ENABLED_EXT: Self = Self(128);
    pub const VIDEO_ENCODE_H264_CAPABILITY_DEBLOCKING_FILTER_PARTIAL_EXT: Self = Self(256);
    pub const VIDEO_ENCODE_H264_CAPABILITY_MULTIPLE_SLICE_PER_FRAME_EXT: Self = Self(512);
    pub const VIDEO_ENCODE_H264_CAPABILITY_EVENLY_DISTRIBUTED_SLICE_SIZE_EXT: Self = Self(1024);
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
impl VideoEncodeH264InputModeFlagBitsEXT {
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
impl VideoEncodeH264OutputModeFlagBitsEXT {
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
impl VideoEncodeH264CreateFlagBitsEXT {
    pub const DEFAULT_EXT: Self = Self(0);
    pub const RESERVED_0_EXT: Self = Self(1);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeH264CapabilitiesEXT.html) · Structure"]
#[doc(alias = "VkVideoEncodeH264CapabilitiesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoEncodeH264CapabilitiesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::ext_video_encode_h264::VideoEncodeH264CapabilitiesFlagsEXT,
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
impl Default for VideoEncodeH264CapabilitiesEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::VIDEO_ENCODE_H264_CAPABILITIES_EXT, p_next: std::ptr::null(), flags: Default::default(), input_mode_flags: Default::default(), output_mode_flags: Default::default(), min_picture_size_in_mbs: Default::default(), max_picture_size_in_mbs: Default::default(), input_image_data_alignment: Default::default(), max_num_l0_reference_for_p: Default::default(), max_num_l0_reference_for_b: Default::default(), max_num_l1_reference: Default::default(), quality_level_count: Default::default(), std_extension_version: Default::default() }
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
    pub fn flags(mut self, flags: crate::extensions::ext_video_encode_h264::VideoEncodeH264CapabilitiesFlagsEXT) -> Self {
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
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> VideoEncodeH264CapabilitiesEXT {
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
impl Default for VideoEncodeH264SessionCreateInfoEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::VIDEO_ENCODE_H264_SESSION_CREATE_INFO_EXT, p_next: std::ptr::null(), flags: Default::default(), max_picture_size_in_mbs: Default::default(), p_std_extension_version: std::ptr::null() }
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
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> VideoEncodeH264SessionCreateInfoEXT {
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
impl Default for VideoEncodeH264SessionParametersAddInfoEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::VIDEO_ENCODE_H264_SESSION_PARAMETERS_ADD_INFO_EXT, p_next: std::ptr::null(), sps_std_count: Default::default(), p_sps_std: std::ptr::null(), pps_std_count: Default::default(), p_pps_std: std::ptr::null() }
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
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> VideoEncodeH264SessionParametersAddInfoEXT {
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
impl Default for VideoEncodeH264SessionParametersCreateInfoEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::VIDEO_ENCODE_H264_SESSION_PARAMETERS_CREATE_INFO_EXT, p_next: std::ptr::null(), max_sps_std_count: Default::default(), max_pps_std_count: Default::default(), p_parameters_add_info: std::ptr::null() }
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
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> VideoEncodeH264SessionParametersCreateInfoEXT {
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
impl Default for VideoEncodeH264DpbSlotInfoEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::VIDEO_ENCODE_H264_DPB_SLOT_INFO_EXT, p_next: std::ptr::null(), slot_index: Default::default(), p_std_picture_info: std::ptr::null() }
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
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> VideoEncodeH264DpbSlotInfoEXT {
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
impl Default for VideoEncodeH264VclFrameInfoEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::VIDEO_ENCODE_H264_VCL_FRAME_INFO_EXT, p_next: std::ptr::null(), ref_default_final_list0_entry_count: Default::default(), p_ref_default_final_list0_entries: std::ptr::null(), ref_default_final_list1_entry_count: Default::default(), p_ref_default_final_list1_entries: std::ptr::null(), nalu_slice_entry_count: Default::default(), p_nalu_slice_entries: std::ptr::null(), p_current_picture_info: std::ptr::null() }
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
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> VideoEncodeH264VclFrameInfoEXT {
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
impl Default for VideoEncodeH264EmitPictureParametersEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::VIDEO_ENCODE_H264_EMIT_PICTURE_PARAMETERS_EXT, p_next: std::ptr::null(), sps_id: Default::default(), emit_sps_enable: Default::default(), pps_id_entry_count: Default::default(), pps_id_entries: std::ptr::null() }
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
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> VideoEncodeH264EmitPictureParametersEXT {
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
impl Default for VideoEncodeH264ProfileEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::VIDEO_ENCODE_H264_PROFILE_EXT, p_next: std::ptr::null(), std_profile_idc: Default::default() }
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
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> VideoEncodeH264ProfileEXT {
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
    pub preceding_nalu_bytes: u32,
    pub min_qp: u8,
    pub max_qp: u8,
}
impl Default for VideoEncodeH264NaluSliceEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::VIDEO_ENCODE_H264_NALU_SLICE_EXT, p_next: std::ptr::null(), p_slice_header_std: std::ptr::null(), mb_count: Default::default(), ref_final_list0_entry_count: Default::default(), p_ref_final_list0_entries: std::ptr::null(), ref_final_list1_entry_count: Default::default(), p_ref_final_list1_entries: std::ptr::null(), preceding_nalu_bytes: Default::default(), min_qp: Default::default(), max_qp: Default::default() }
    }
}
impl std::fmt::Debug for VideoEncodeH264NaluSliceEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoEncodeH264NaluSliceEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("p_slice_header_std", &self.p_slice_header_std).field("mb_count", &self.mb_count).field("ref_final_list0_entry_count", &self.ref_final_list0_entry_count).field("p_ref_final_list0_entries", &self.p_ref_final_list0_entries).field("ref_final_list1_entry_count", &self.ref_final_list1_entry_count).field("p_ref_final_list1_entries", &self.p_ref_final_list1_entries).field("preceding_nalu_bytes", &self.preceding_nalu_bytes).field("min_qp", &self.min_qp).field("max_qp", &self.max_qp).finish()
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
    pub fn preceding_nalu_bytes(mut self, preceding_nalu_bytes: u32) -> Self {
        self.0.preceding_nalu_bytes = preceding_nalu_bytes as _;
        self
    }
    #[inline]
    pub fn min_qp(mut self, min_qp: u8) -> Self {
        self.0.min_qp = min_qp as _;
        self
    }
    #[inline]
    pub fn max_qp(mut self, max_qp: u8) -> Self {
        self.0.max_qp = max_qp as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> VideoEncodeH264NaluSliceEXT {
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
