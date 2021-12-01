//! ## Versioning Warning ⚠️
//!
//! This is a Vulkan **provisional/beta** extension and **must** be used with
//! caution. Its API/behaviour has not been finalized yet and _may_ therefore
//! change in ways that break backwards compatibility between revisions, and
//! before final release of a non-provisional version of this extension.
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_VIDEO_ENCODE_QUEUE_SPEC_VERSION")]
pub const KHR_VIDEO_ENCODE_QUEUE_SPEC_VERSION: u32 = 3;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_VIDEO_ENCODE_QUEUE_EXTENSION_NAME")]
pub const KHR_VIDEO_ENCODE_QUEUE_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_video_encode_queue");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_ENCODE_VIDEO_KHR: *const std::os::raw::c_char = crate::cstr!("vkCmdEncodeVideoKHR");
#[doc = "Provided by [`crate::extensions::khr_video_encode_queue`]"]
impl crate::extensions::khr_synchronization2::AccessFlagBits2KHR {
    pub const VIDEO_ENCODE_READ_KHR: Self = Self(137438953472);
    pub const VIDEO_ENCODE_WRITE_KHR: Self = Self(274877906944);
}
#[doc = "Provided by [`crate::extensions::khr_video_encode_queue`]"]
impl crate::extensions::khr_synchronization2::PipelineStageFlagBits2KHR {
    pub const VIDEO_ENCODE_KHR: Self = Self(134217728);
}
#[doc = "Provided by [`crate::extensions::khr_video_encode_queue`]"]
impl crate::extensions::khr_format_feature_flags2::FormatFeatureFlagBits2KHR {
    pub const VIDEO_ENCODE_INPUT_KHR: Self = Self(134217728);
    pub const VIDEO_ENCODE_DPB_KHR: Self = Self(268435456);
}
#[doc = "Provided by [`crate::extensions::khr_video_encode_queue`]"]
impl crate::vk1_0::BufferUsageFlagBits {
    pub const VIDEO_ENCODE_DST_KHR: Self = Self(32768);
    pub const VIDEO_ENCODE_SRC_KHR: Self = Self(65536);
}
#[doc = "Provided by [`crate::extensions::khr_video_encode_queue`]"]
impl crate::vk1_0::FormatFeatureFlagBits {
    pub const VIDEO_ENCODE_INPUT_KHR: Self = Self(134217728);
    pub const VIDEO_ENCODE_DPB_KHR: Self = Self(268435456);
}
#[doc = "Provided by [`crate::extensions::khr_video_encode_queue`]"]
impl crate::vk1_0::ImageLayout {
    pub const VIDEO_ENCODE_DST_KHR: Self = Self(1000299000);
    pub const VIDEO_ENCODE_SRC_KHR: Self = Self(1000299001);
    pub const VIDEO_ENCODE_DPB_KHR: Self = Self(1000299002);
}
#[doc = "Provided by [`crate::extensions::khr_video_encode_queue`]"]
impl crate::vk1_0::ImageUsageFlagBits {
    pub const VIDEO_ENCODE_DST_KHR: Self = Self(8192);
    pub const VIDEO_ENCODE_SRC_KHR: Self = Self(16384);
    pub const VIDEO_ENCODE_DPB_KHR: Self = Self(32768);
}
#[doc = "Provided by [`crate::extensions::khr_video_encode_queue`]"]
impl crate::vk1_0::QueryType {
    pub const VIDEO_ENCODE_BITSTREAM_BUFFER_RANGE_KHR: Self = Self(1000299000);
}
#[doc = "Provided by [`crate::extensions::khr_video_encode_queue`]"]
impl crate::vk1_0::QueueFlagBits {
    pub const VIDEO_ENCODE_KHR: Self = Self(64);
}
#[doc = "Provided by [`crate::extensions::khr_video_encode_queue`]"]
impl crate::vk1_0::StructureType {
    pub const VIDEO_ENCODE_INFO_KHR: Self = Self(1000299000);
    pub const VIDEO_ENCODE_RATE_CONTROL_INFO_KHR: Self = Self(1000299001);
    pub const VIDEO_ENCODE_RATE_CONTROL_LAYER_INFO_KHR: Self = Self(1000299002);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeFlagsKHR.html) · Bitmask of [`VideoEncodeFlagBitsKHR`]"] # [doc (alias = "VkVideoEncodeFlagsKHR")] # [derive (Default)] # [repr (transparent)] pub struct VideoEncodeFlagsKHR : u32 { const DEFAULT_KHR = VideoEncodeFlagBitsKHR :: DEFAULT_KHR . 0 ; const RESERVED_0_KHR = VideoEncodeFlagBitsKHR :: RESERVED_0_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeFlagBitsKHR.html) · Bits enum of [`VideoEncodeFlagsKHR`]"]
#[doc(alias = "VkVideoEncodeFlagBitsKHR")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct VideoEncodeFlagBitsKHR(pub u32);
impl VideoEncodeFlagBitsKHR {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> VideoEncodeFlagsKHR {
        VideoEncodeFlagsKHR::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for VideoEncodeFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::DEFAULT_KHR => "DEFAULT_KHR",
            &Self::RESERVED_0_KHR => "RESERVED_0_KHR",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::khr_video_encode_queue`]"]
impl crate::extensions::khr_video_encode_queue::VideoEncodeFlagBitsKHR {
    pub const DEFAULT_KHR: Self = Self(0);
    pub const RESERVED_0_KHR: Self = Self(1);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeRateControlFlagsKHR.html) · Bitmask of [`VideoEncodeRateControlFlagBitsKHR`]"] # [doc (alias = "VkVideoEncodeRateControlFlagsKHR")] # [derive (Default)] # [repr (transparent)] pub struct VideoEncodeRateControlFlagsKHR : u32 { const DEFAULT_KHR = VideoEncodeRateControlFlagBitsKHR :: DEFAULT_KHR . 0 ; const RESERVED_0_KHR = VideoEncodeRateControlFlagBitsKHR :: RESERVED_0_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeRateControlFlagBitsKHR.html) · Bits enum of [`VideoEncodeRateControlFlagsKHR`]"]
#[doc(alias = "VkVideoEncodeRateControlFlagBitsKHR")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct VideoEncodeRateControlFlagBitsKHR(pub u32);
impl VideoEncodeRateControlFlagBitsKHR {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> VideoEncodeRateControlFlagsKHR {
        VideoEncodeRateControlFlagsKHR::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for VideoEncodeRateControlFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::DEFAULT_KHR => "DEFAULT_KHR",
            &Self::RESERVED_0_KHR => "RESERVED_0_KHR",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::khr_video_encode_queue`]"]
impl crate::extensions::khr_video_encode_queue::VideoEncodeRateControlFlagBitsKHR {
    pub const DEFAULT_KHR: Self = Self(0);
    pub const RESERVED_0_KHR: Self = Self(1);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeRateControlModeFlagsKHR.html) · Bitmask of [`VideoEncodeRateControlModeFlagBitsKHR`]"] # [doc (alias = "VkVideoEncodeRateControlModeFlagsKHR")] # [derive (Default)] # [repr (transparent)] pub struct VideoEncodeRateControlModeFlagsKHR : u32 { const NONE_KHR = VideoEncodeRateControlModeFlagBitsKHR :: NONE_KHR . 0 ; const CBR_KHR = VideoEncodeRateControlModeFlagBitsKHR :: CBR_KHR . 0 ; const VBR_KHR = VideoEncodeRateControlModeFlagBitsKHR :: VBR_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeRateControlModeFlagBitsKHR.html) · Bits enum of [`VideoEncodeRateControlModeFlagsKHR`]"]
#[doc(alias = "VkVideoEncodeRateControlModeFlagBitsKHR")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct VideoEncodeRateControlModeFlagBitsKHR(pub u32);
impl VideoEncodeRateControlModeFlagBitsKHR {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> VideoEncodeRateControlModeFlagsKHR {
        VideoEncodeRateControlModeFlagsKHR::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for VideoEncodeRateControlModeFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::NONE_KHR => "NONE_KHR",
            &Self::CBR_KHR => "CBR_KHR",
            &Self::VBR_KHR => "VBR_KHR",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::khr_video_encode_queue`]"]
impl crate::extensions::khr_video_encode_queue::VideoEncodeRateControlModeFlagBitsKHR {
    pub const NONE_KHR: Self = Self(0);
    pub const CBR_KHR: Self = Self(1);
    pub const VBR_KHR: Self = Self(2);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdEncodeVideoKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdEncodeVideoKHR = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, p_encode_info: *const crate::extensions::khr_video_encode_queue::VideoEncodeInfoKHR) -> ();
impl<'a> crate::ExtendableFrom<'a, VideoEncodeRateControlInfoKHR> for crate::extensions::khr_video_queue::VideoCodingControlInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeRateControlInfoKHRBuilder<'_>> for crate::extensions::khr_video_queue::VideoCodingControlInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeRateControlLayerInfoKHR> for crate::extensions::khr_video_queue::VideoCodingControlInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoEncodeRateControlLayerInfoKHRBuilder<'_>> for crate::extensions::khr_video_queue::VideoCodingControlInfoKHRBuilder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeInfoKHR.html) · Structure"]
#[doc(alias = "VkVideoEncodeInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoEncodeInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::khr_video_encode_queue::VideoEncodeFlagsKHR,
    pub quality_level: u32,
    pub coded_extent: crate::vk1_0::Extent2D,
    pub dst_bitstream_buffer: crate::vk1_0::Buffer,
    pub dst_bitstream_buffer_offset: crate::vk1_0::DeviceSize,
    pub dst_bitstream_buffer_max_range: crate::vk1_0::DeviceSize,
    pub src_picture_resource: crate::extensions::khr_video_queue::VideoPictureResourceKHR,
    pub p_setup_reference_slot: *const crate::extensions::khr_video_queue::VideoReferenceSlotKHR,
    pub reference_slot_count: u32,
    pub p_reference_slots: *const crate::extensions::khr_video_queue::VideoReferenceSlotKHR,
    pub preceding_externally_encoded_bytes: u32,
}
impl VideoEncodeInfoKHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::VIDEO_ENCODE_INFO_KHR;
}
impl Default for VideoEncodeInfoKHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), flags: Default::default(), quality_level: Default::default(), coded_extent: Default::default(), dst_bitstream_buffer: Default::default(), dst_bitstream_buffer_offset: Default::default(), dst_bitstream_buffer_max_range: Default::default(), src_picture_resource: Default::default(), p_setup_reference_slot: std::ptr::null(), reference_slot_count: Default::default(), p_reference_slots: std::ptr::null(), preceding_externally_encoded_bytes: Default::default() }
    }
}
impl std::fmt::Debug for VideoEncodeInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoEncodeInfoKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("quality_level", &self.quality_level).field("coded_extent", &self.coded_extent).field("dst_bitstream_buffer", &self.dst_bitstream_buffer).field("dst_bitstream_buffer_offset", &self.dst_bitstream_buffer_offset).field("dst_bitstream_buffer_max_range", &self.dst_bitstream_buffer_max_range).field("src_picture_resource", &self.src_picture_resource).field("p_setup_reference_slot", &self.p_setup_reference_slot).field("reference_slot_count", &self.reference_slot_count).field("p_reference_slots", &self.p_reference_slots).field("preceding_externally_encoded_bytes", &self.preceding_externally_encoded_bytes).finish()
    }
}
impl VideoEncodeInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoEncodeInfoKHRBuilder<'a> {
        VideoEncodeInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeInfoKHR.html) · Builder of [`VideoEncodeInfoKHR`]"]
#[repr(transparent)]
pub struct VideoEncodeInfoKHRBuilder<'a>(VideoEncodeInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> VideoEncodeInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> VideoEncodeInfoKHRBuilder<'a> {
        VideoEncodeInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::khr_video_encode_queue::VideoEncodeFlagsKHR) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn quality_level(mut self, quality_level: u32) -> Self {
        self.0.quality_level = quality_level as _;
        self
    }
    #[inline]
    pub fn coded_extent(mut self, coded_extent: crate::vk1_0::Extent2D) -> Self {
        self.0.coded_extent = coded_extent as _;
        self
    }
    #[inline]
    pub fn dst_bitstream_buffer(mut self, dst_bitstream_buffer: crate::vk1_0::Buffer) -> Self {
        self.0.dst_bitstream_buffer = dst_bitstream_buffer as _;
        self
    }
    #[inline]
    pub fn dst_bitstream_buffer_offset(mut self, dst_bitstream_buffer_offset: crate::vk1_0::DeviceSize) -> Self {
        self.0.dst_bitstream_buffer_offset = dst_bitstream_buffer_offset as _;
        self
    }
    #[inline]
    pub fn dst_bitstream_buffer_max_range(mut self, dst_bitstream_buffer_max_range: crate::vk1_0::DeviceSize) -> Self {
        self.0.dst_bitstream_buffer_max_range = dst_bitstream_buffer_max_range as _;
        self
    }
    #[inline]
    pub fn src_picture_resource(mut self, src_picture_resource: crate::extensions::khr_video_queue::VideoPictureResourceKHR) -> Self {
        self.0.src_picture_resource = src_picture_resource as _;
        self
    }
    #[inline]
    pub fn setup_reference_slot(mut self, setup_reference_slot: &'a crate::extensions::khr_video_queue::VideoReferenceSlotKHR) -> Self {
        self.0.p_setup_reference_slot = setup_reference_slot as _;
        self
    }
    #[inline]
    pub fn reference_slots(mut self, reference_slots: &'a [crate::extensions::khr_video_queue::VideoReferenceSlotKHRBuilder]) -> Self {
        self.0.p_reference_slots = reference_slots.as_ptr() as _;
        self.0.reference_slot_count = reference_slots.len() as _;
        self
    }
    #[inline]
    pub fn preceding_externally_encoded_bytes(mut self, preceding_externally_encoded_bytes: u32) -> Self {
        self.0.preceding_externally_encoded_bytes = preceding_externally_encoded_bytes as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> VideoEncodeInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for VideoEncodeInfoKHRBuilder<'a> {
    fn default() -> VideoEncodeInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoEncodeInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoEncodeInfoKHRBuilder<'a> {
    type Target = VideoEncodeInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoEncodeInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeRateControlInfoKHR.html) · Structure"]
#[doc(alias = "VkVideoEncodeRateControlInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoEncodeRateControlInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::khr_video_encode_queue::VideoEncodeRateControlFlagsKHR,
    pub rate_control_mode: crate::extensions::khr_video_encode_queue::VideoEncodeRateControlModeFlagBitsKHR,
    pub layer_count: u8,
    pub p_layer_configs: *const crate::extensions::khr_video_encode_queue::VideoEncodeRateControlLayerInfoKHR,
}
impl VideoEncodeRateControlInfoKHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::VIDEO_ENCODE_RATE_CONTROL_INFO_KHR;
}
impl Default for VideoEncodeRateControlInfoKHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), flags: Default::default(), rate_control_mode: Default::default(), layer_count: Default::default(), p_layer_configs: std::ptr::null() }
    }
}
impl std::fmt::Debug for VideoEncodeRateControlInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoEncodeRateControlInfoKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("rate_control_mode", &self.rate_control_mode).field("layer_count", &self.layer_count).field("p_layer_configs", &self.p_layer_configs).finish()
    }
}
impl VideoEncodeRateControlInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoEncodeRateControlInfoKHRBuilder<'a> {
        VideoEncodeRateControlInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeRateControlInfoKHR.html) · Builder of [`VideoEncodeRateControlInfoKHR`]"]
#[repr(transparent)]
pub struct VideoEncodeRateControlInfoKHRBuilder<'a>(VideoEncodeRateControlInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> VideoEncodeRateControlInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> VideoEncodeRateControlInfoKHRBuilder<'a> {
        VideoEncodeRateControlInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::khr_video_encode_queue::VideoEncodeRateControlFlagsKHR) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn rate_control_mode(mut self, rate_control_mode: crate::extensions::khr_video_encode_queue::VideoEncodeRateControlModeFlagBitsKHR) -> Self {
        self.0.rate_control_mode = rate_control_mode as _;
        self
    }
    #[inline]
    pub fn layer_configs(mut self, layer_configs: &'a [crate::extensions::khr_video_encode_queue::VideoEncodeRateControlLayerInfoKHRBuilder]) -> Self {
        self.0.p_layer_configs = layer_configs.as_ptr() as _;
        self.0.layer_count = layer_configs.len() as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> VideoEncodeRateControlInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for VideoEncodeRateControlInfoKHRBuilder<'a> {
    fn default() -> VideoEncodeRateControlInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoEncodeRateControlInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoEncodeRateControlInfoKHRBuilder<'a> {
    type Target = VideoEncodeRateControlInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoEncodeRateControlInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeRateControlLayerInfoKHR.html) · Structure"]
#[doc(alias = "VkVideoEncodeRateControlLayerInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoEncodeRateControlLayerInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub average_bitrate: u32,
    pub max_bitrate: u32,
    pub frame_rate_numerator: u32,
    pub frame_rate_denominator: u32,
    pub virtual_buffer_size_in_ms: u32,
    pub initial_virtual_buffer_size_in_ms: u32,
}
impl VideoEncodeRateControlLayerInfoKHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::VIDEO_ENCODE_RATE_CONTROL_LAYER_INFO_KHR;
}
impl Default for VideoEncodeRateControlLayerInfoKHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), average_bitrate: Default::default(), max_bitrate: Default::default(), frame_rate_numerator: Default::default(), frame_rate_denominator: Default::default(), virtual_buffer_size_in_ms: Default::default(), initial_virtual_buffer_size_in_ms: Default::default() }
    }
}
impl std::fmt::Debug for VideoEncodeRateControlLayerInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoEncodeRateControlLayerInfoKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("average_bitrate", &self.average_bitrate).field("max_bitrate", &self.max_bitrate).field("frame_rate_numerator", &self.frame_rate_numerator).field("frame_rate_denominator", &self.frame_rate_denominator).field("virtual_buffer_size_in_ms", &self.virtual_buffer_size_in_ms).field("initial_virtual_buffer_size_in_ms", &self.initial_virtual_buffer_size_in_ms).finish()
    }
}
impl VideoEncodeRateControlLayerInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoEncodeRateControlLayerInfoKHRBuilder<'a> {
        VideoEncodeRateControlLayerInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoEncodeRateControlLayerInfoKHR.html) · Builder of [`VideoEncodeRateControlLayerInfoKHR`]"]
#[repr(transparent)]
pub struct VideoEncodeRateControlLayerInfoKHRBuilder<'a>(VideoEncodeRateControlLayerInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> VideoEncodeRateControlLayerInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> VideoEncodeRateControlLayerInfoKHRBuilder<'a> {
        VideoEncodeRateControlLayerInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn average_bitrate(mut self, average_bitrate: u32) -> Self {
        self.0.average_bitrate = average_bitrate as _;
        self
    }
    #[inline]
    pub fn max_bitrate(mut self, max_bitrate: u32) -> Self {
        self.0.max_bitrate = max_bitrate as _;
        self
    }
    #[inline]
    pub fn frame_rate_numerator(mut self, frame_rate_numerator: u32) -> Self {
        self.0.frame_rate_numerator = frame_rate_numerator as _;
        self
    }
    #[inline]
    pub fn frame_rate_denominator(mut self, frame_rate_denominator: u32) -> Self {
        self.0.frame_rate_denominator = frame_rate_denominator as _;
        self
    }
    #[inline]
    pub fn virtual_buffer_size_in_ms(mut self, virtual_buffer_size_in_ms: u32) -> Self {
        self.0.virtual_buffer_size_in_ms = virtual_buffer_size_in_ms as _;
        self
    }
    #[inline]
    pub fn initial_virtual_buffer_size_in_ms(mut self, initial_virtual_buffer_size_in_ms: u32) -> Self {
        self.0.initial_virtual_buffer_size_in_ms = initial_virtual_buffer_size_in_ms as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> VideoEncodeRateControlLayerInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for VideoEncodeRateControlLayerInfoKHRBuilder<'a> {
    fn default() -> VideoEncodeRateControlLayerInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoEncodeRateControlLayerInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoEncodeRateControlLayerInfoKHRBuilder<'a> {
    type Target = VideoEncodeRateControlLayerInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoEncodeRateControlLayerInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::khr_video_encode_queue`]"]
impl crate::DeviceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdEncodeVideoKHR.html) · Function"]
    #[doc(alias = "vkCmdEncodeVideoKHR")]
    pub unsafe fn cmd_encode_video_khr(&self, command_buffer: crate::vk1_0::CommandBuffer, encode_info: &crate::extensions::khr_video_encode_queue::VideoEncodeInfoKHR) -> () {
        let _function = self.cmd_encode_video_khr.expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(command_buffer as _, encode_info as _);
        ()
    }
}
