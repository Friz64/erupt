//! ## Versioning Warning ⚠️
//!
//! This is a Vulkan **provisional/beta** extension and **must** be used with
//! caution. Its API/behaviour has not been finalized yet and _may_ therefore
//! change in ways that break backwards compatibility between revisions, and
//! before final release of a non-provisional version of this extension.
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_VIDEO_DECODE_H265_SPEC_VERSION")]
pub const EXT_VIDEO_DECODE_H265_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_VIDEO_DECODE_H265_EXTENSION_NAME")]
pub const EXT_VIDEO_DECODE_H265_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_video_decode_h265");
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoDecodeH265CreateFlagsEXT.html) · Bitmask of [`VideoDecodeH265CreateFlagBitsEXT`]"] # [doc (alias = "VkVideoDecodeH265CreateFlagsEXT")] # [derive (Default)] # [repr (transparent)] pub struct VideoDecodeH265CreateFlagsEXT : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`VideoDecodeH265CreateFlagsEXT`]"]
#[doc(alias = "VkVideoDecodeH265CreateFlagBitsEXT")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct VideoDecodeH265CreateFlagBitsEXT(pub u32);
impl VideoDecodeH265CreateFlagBitsEXT {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> VideoDecodeH265CreateFlagsEXT {
        VideoDecodeH265CreateFlagsEXT::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for VideoDecodeH265CreateFlagBitsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::ext_video_decode_h265`]"]
impl crate::vk1_0::StructureType {
    pub const VIDEO_DECODE_H265_CAPABILITIES_EXT: Self = Self(1000187000);
    pub const VIDEO_DECODE_H265_SESSION_CREATE_INFO_EXT: Self = Self(1000187001);
    pub const VIDEO_DECODE_H265_SESSION_PARAMETERS_CREATE_INFO_EXT: Self = Self(1000187002);
    pub const VIDEO_DECODE_H265_SESSION_PARAMETERS_ADD_INFO_EXT: Self = Self(1000187003);
    pub const VIDEO_DECODE_H265_PROFILE_EXT: Self = Self(1000187004);
    pub const VIDEO_DECODE_H265_PICTURE_INFO_EXT: Self = Self(1000187005);
    pub const VIDEO_DECODE_H265_DPB_SLOT_INFO_EXT: Self = Self(1000187006);
}
#[doc = "Provided by [`crate::extensions::ext_video_decode_h265`]"]
impl crate::extensions::khr_video_queue::VideoCodecOperationFlagBitsKHR {
    pub const DECODE_H265_EXT: Self = Self(2);
}
#[cfg(feature = "khr_video_queue")]
impl<'a> crate::ExtendableFromMut<'a, VideoDecodeH265ProfileEXT> for crate::extensions::khr_video_queue::VideoProfileKHRBuilder<'a> {}
#[cfg(feature = "khr_video_queue")]
impl<'a> crate::ExtendableFromMut<'a, VideoDecodeH265ProfileEXTBuilder<'_>> for crate::extensions::khr_video_queue::VideoProfileKHRBuilder<'a> {}
#[cfg(feature = "khr_video_queue")]
impl<'a> crate::ExtendableFromMut<'a, VideoDecodeH265CapabilitiesEXT> for crate::extensions::khr_video_queue::VideoCapabilitiesKHRBuilder<'a> {}
#[cfg(feature = "khr_video_queue")]
impl<'a> crate::ExtendableFromMut<'a, VideoDecodeH265CapabilitiesEXTBuilder<'_>> for crate::extensions::khr_video_queue::VideoCapabilitiesKHRBuilder<'a> {}
#[cfg(feature = "khr_video_queue")]
impl<'a> crate::ExtendableFromConst<'a, VideoDecodeH265DpbSlotInfoEXT> for crate::extensions::khr_video_queue::VideoReferenceSlotKHRBuilder<'a> {}
#[cfg(feature = "khr_video_queue")]
impl<'a> crate::ExtendableFromConst<'a, VideoDecodeH265DpbSlotInfoEXTBuilder<'_>> for crate::extensions::khr_video_queue::VideoReferenceSlotKHRBuilder<'a> {}
#[cfg(feature = "khr_video_decode_queue")]
impl<'a> crate::ExtendableFromConst<'a, VideoDecodeH265PictureInfoEXT> for crate::extensions::khr_video_decode_queue::VideoDecodeInfoKHRBuilder<'a> {}
#[cfg(feature = "khr_video_decode_queue")]
impl<'a> crate::ExtendableFromConst<'a, VideoDecodeH265PictureInfoEXTBuilder<'_>> for crate::extensions::khr_video_decode_queue::VideoDecodeInfoKHRBuilder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoDecodeH265ProfileEXT.html) · Structure"]
#[doc(alias = "VkVideoDecodeH265ProfileEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoDecodeH265ProfileEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub std_profile_idc: crate::external::vk_video::StdVideoH265ProfileIdc,
}
impl Default for VideoDecodeH265ProfileEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::VIDEO_DECODE_H265_PROFILE_EXT, p_next: std::ptr::null(), std_profile_idc: Default::default() }
    }
}
impl std::fmt::Debug for VideoDecodeH265ProfileEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoDecodeH265ProfileEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("std_profile_idc", &self.std_profile_idc).finish()
    }
}
impl VideoDecodeH265ProfileEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoDecodeH265ProfileEXTBuilder<'a> {
        VideoDecodeH265ProfileEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoDecodeH265ProfileEXT.html) · Builder of [`VideoDecodeH265ProfileEXT`]"]
#[repr(transparent)]
pub struct VideoDecodeH265ProfileEXTBuilder<'a>(VideoDecodeH265ProfileEXT, std::marker::PhantomData<&'a ()>);
impl<'a> VideoDecodeH265ProfileEXTBuilder<'a> {
    #[inline]
    pub fn new() -> VideoDecodeH265ProfileEXTBuilder<'a> {
        VideoDecodeH265ProfileEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn std_profile_idc(mut self, std_profile_idc: crate::external::vk_video::StdVideoH265ProfileIdc) -> Self {
        self.0.std_profile_idc = std_profile_idc as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> VideoDecodeH265ProfileEXT {
        self.0
    }
}
impl<'a> std::default::Default for VideoDecodeH265ProfileEXTBuilder<'a> {
    fn default() -> VideoDecodeH265ProfileEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoDecodeH265ProfileEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoDecodeH265ProfileEXTBuilder<'a> {
    type Target = VideoDecodeH265ProfileEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoDecodeH265ProfileEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoDecodeH265CapabilitiesEXT.html) · Structure"]
#[doc(alias = "VkVideoDecodeH265CapabilitiesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoDecodeH265CapabilitiesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub max_level: u32,
    pub std_extension_version: crate::vk1_0::ExtensionProperties,
}
impl Default for VideoDecodeH265CapabilitiesEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::VIDEO_DECODE_H265_CAPABILITIES_EXT, p_next: std::ptr::null_mut(), max_level: Default::default(), std_extension_version: Default::default() }
    }
}
impl std::fmt::Debug for VideoDecodeH265CapabilitiesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoDecodeH265CapabilitiesEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("max_level", &self.max_level).field("std_extension_version", &self.std_extension_version).finish()
    }
}
impl VideoDecodeH265CapabilitiesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoDecodeH265CapabilitiesEXTBuilder<'a> {
        VideoDecodeH265CapabilitiesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoDecodeH265CapabilitiesEXT.html) · Builder of [`VideoDecodeH265CapabilitiesEXT`]"]
#[repr(transparent)]
pub struct VideoDecodeH265CapabilitiesEXTBuilder<'a>(VideoDecodeH265CapabilitiesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> VideoDecodeH265CapabilitiesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> VideoDecodeH265CapabilitiesEXTBuilder<'a> {
        VideoDecodeH265CapabilitiesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn max_level(mut self, max_level: u32) -> Self {
        self.0.max_level = max_level as _;
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
    pub fn build(self) -> VideoDecodeH265CapabilitiesEXT {
        self.0
    }
}
impl<'a> std::default::Default for VideoDecodeH265CapabilitiesEXTBuilder<'a> {
    fn default() -> VideoDecodeH265CapabilitiesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoDecodeH265CapabilitiesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoDecodeH265CapabilitiesEXTBuilder<'a> {
    type Target = VideoDecodeH265CapabilitiesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoDecodeH265CapabilitiesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoDecodeH265SessionCreateInfoEXT.html) · Structure"]
#[doc(alias = "VkVideoDecodeH265SessionCreateInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoDecodeH265SessionCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::ext_video_decode_h265::VideoDecodeH265CreateFlagsEXT,
    pub p_std_extension_version: *const crate::vk1_0::ExtensionProperties,
}
impl Default for VideoDecodeH265SessionCreateInfoEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::VIDEO_DECODE_H265_SESSION_CREATE_INFO_EXT, p_next: std::ptr::null(), flags: Default::default(), p_std_extension_version: std::ptr::null() }
    }
}
impl std::fmt::Debug for VideoDecodeH265SessionCreateInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoDecodeH265SessionCreateInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("p_std_extension_version", &self.p_std_extension_version).finish()
    }
}
impl VideoDecodeH265SessionCreateInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoDecodeH265SessionCreateInfoEXTBuilder<'a> {
        VideoDecodeH265SessionCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoDecodeH265SessionCreateInfoEXT.html) · Builder of [`VideoDecodeH265SessionCreateInfoEXT`]"]
#[repr(transparent)]
pub struct VideoDecodeH265SessionCreateInfoEXTBuilder<'a>(VideoDecodeH265SessionCreateInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> VideoDecodeH265SessionCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> VideoDecodeH265SessionCreateInfoEXTBuilder<'a> {
        VideoDecodeH265SessionCreateInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::ext_video_decode_h265::VideoDecodeH265CreateFlagsEXT) -> Self {
        self.0.flags = flags as _;
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
    pub fn build(self) -> VideoDecodeH265SessionCreateInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for VideoDecodeH265SessionCreateInfoEXTBuilder<'a> {
    fn default() -> VideoDecodeH265SessionCreateInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoDecodeH265SessionCreateInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoDecodeH265SessionCreateInfoEXTBuilder<'a> {
    type Target = VideoDecodeH265SessionCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoDecodeH265SessionCreateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoDecodeH265SessionParametersAddInfoEXT.html) · Structure"]
#[doc(alias = "VkVideoDecodeH265SessionParametersAddInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoDecodeH265SessionParametersAddInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub sps_std_count: u32,
    pub p_sps_std: *const crate::external::vk_video::StdVideoH265SequenceParameterSet,
    pub pps_std_count: u32,
    pub p_pps_std: *const crate::external::vk_video::StdVideoH265PictureParameterSet,
}
impl Default for VideoDecodeH265SessionParametersAddInfoEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::VIDEO_DECODE_H265_SESSION_PARAMETERS_ADD_INFO_EXT, p_next: std::ptr::null(), sps_std_count: Default::default(), p_sps_std: std::ptr::null(), pps_std_count: Default::default(), p_pps_std: std::ptr::null() }
    }
}
impl std::fmt::Debug for VideoDecodeH265SessionParametersAddInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoDecodeH265SessionParametersAddInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("sps_std_count", &self.sps_std_count).field("p_sps_std", &self.p_sps_std).field("pps_std_count", &self.pps_std_count).field("p_pps_std", &self.p_pps_std).finish()
    }
}
impl VideoDecodeH265SessionParametersAddInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoDecodeH265SessionParametersAddInfoEXTBuilder<'a> {
        VideoDecodeH265SessionParametersAddInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoDecodeH265SessionParametersAddInfoEXT.html) · Builder of [`VideoDecodeH265SessionParametersAddInfoEXT`]"]
#[repr(transparent)]
pub struct VideoDecodeH265SessionParametersAddInfoEXTBuilder<'a>(VideoDecodeH265SessionParametersAddInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> VideoDecodeH265SessionParametersAddInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> VideoDecodeH265SessionParametersAddInfoEXTBuilder<'a> {
        VideoDecodeH265SessionParametersAddInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn sps_std(mut self, sps_std: &'a [crate::external::vk_video::StdVideoH265SequenceParameterSetBuilder]) -> Self {
        self.0.p_sps_std = sps_std.as_ptr() as _;
        self.0.sps_std_count = sps_std.len() as _;
        self
    }
    #[inline]
    pub fn pps_std(mut self, pps_std: &'a [crate::external::vk_video::StdVideoH265PictureParameterSetBuilder]) -> Self {
        self.0.p_pps_std = pps_std.as_ptr() as _;
        self.0.pps_std_count = pps_std.len() as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> VideoDecodeH265SessionParametersAddInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for VideoDecodeH265SessionParametersAddInfoEXTBuilder<'a> {
    fn default() -> VideoDecodeH265SessionParametersAddInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoDecodeH265SessionParametersAddInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoDecodeH265SessionParametersAddInfoEXTBuilder<'a> {
    type Target = VideoDecodeH265SessionParametersAddInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoDecodeH265SessionParametersAddInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoDecodeH265SessionParametersCreateInfoEXT.html) · Structure"]
#[doc(alias = "VkVideoDecodeH265SessionParametersCreateInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoDecodeH265SessionParametersCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub max_sps_std_count: u32,
    pub max_pps_std_count: u32,
    pub p_parameters_add_info: *const crate::extensions::ext_video_decode_h265::VideoDecodeH265SessionParametersAddInfoEXT,
}
impl Default for VideoDecodeH265SessionParametersCreateInfoEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::VIDEO_DECODE_H265_SESSION_PARAMETERS_CREATE_INFO_EXT, p_next: std::ptr::null(), max_sps_std_count: Default::default(), max_pps_std_count: Default::default(), p_parameters_add_info: std::ptr::null() }
    }
}
impl std::fmt::Debug for VideoDecodeH265SessionParametersCreateInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoDecodeH265SessionParametersCreateInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("max_sps_std_count", &self.max_sps_std_count).field("max_pps_std_count", &self.max_pps_std_count).field("p_parameters_add_info", &self.p_parameters_add_info).finish()
    }
}
impl VideoDecodeH265SessionParametersCreateInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoDecodeH265SessionParametersCreateInfoEXTBuilder<'a> {
        VideoDecodeH265SessionParametersCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoDecodeH265SessionParametersCreateInfoEXT.html) · Builder of [`VideoDecodeH265SessionParametersCreateInfoEXT`]"]
#[repr(transparent)]
pub struct VideoDecodeH265SessionParametersCreateInfoEXTBuilder<'a>(VideoDecodeH265SessionParametersCreateInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> VideoDecodeH265SessionParametersCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> VideoDecodeH265SessionParametersCreateInfoEXTBuilder<'a> {
        VideoDecodeH265SessionParametersCreateInfoEXTBuilder(Default::default(), std::marker::PhantomData)
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
    pub fn parameters_add_info(mut self, parameters_add_info: &'a crate::extensions::ext_video_decode_h265::VideoDecodeH265SessionParametersAddInfoEXT) -> Self {
        self.0.p_parameters_add_info = parameters_add_info as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> VideoDecodeH265SessionParametersCreateInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for VideoDecodeH265SessionParametersCreateInfoEXTBuilder<'a> {
    fn default() -> VideoDecodeH265SessionParametersCreateInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoDecodeH265SessionParametersCreateInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoDecodeH265SessionParametersCreateInfoEXTBuilder<'a> {
    type Target = VideoDecodeH265SessionParametersCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoDecodeH265SessionParametersCreateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoDecodeH265PictureInfoEXT.html) · Structure"]
#[doc(alias = "VkVideoDecodeH265PictureInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoDecodeH265PictureInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub p_std_picture_info: *mut crate::external::vk_video::StdVideoDecodeH265PictureInfo,
    pub slices_count: u32,
    pub p_slices_data_offsets: *const u32,
}
impl Default for VideoDecodeH265PictureInfoEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::VIDEO_DECODE_H265_PICTURE_INFO_EXT, p_next: std::ptr::null(), p_std_picture_info: std::ptr::null_mut(), slices_count: Default::default(), p_slices_data_offsets: std::ptr::null() }
    }
}
impl std::fmt::Debug for VideoDecodeH265PictureInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoDecodeH265PictureInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("p_std_picture_info", &self.p_std_picture_info).field("slices_count", &self.slices_count).field("p_slices_data_offsets", &self.p_slices_data_offsets).finish()
    }
}
impl VideoDecodeH265PictureInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoDecodeH265PictureInfoEXTBuilder<'a> {
        VideoDecodeH265PictureInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoDecodeH265PictureInfoEXT.html) · Builder of [`VideoDecodeH265PictureInfoEXT`]"]
#[repr(transparent)]
pub struct VideoDecodeH265PictureInfoEXTBuilder<'a>(VideoDecodeH265PictureInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> VideoDecodeH265PictureInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> VideoDecodeH265PictureInfoEXTBuilder<'a> {
        VideoDecodeH265PictureInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn std_picture_info(mut self, std_picture_info: &'a mut crate::external::vk_video::StdVideoDecodeH265PictureInfo) -> Self {
        self.0.p_std_picture_info = std_picture_info as _;
        self
    }
    #[inline]
    pub fn slices_data_offsets(mut self, slices_data_offsets: &'a [u32]) -> Self {
        self.0.p_slices_data_offsets = slices_data_offsets.as_ptr() as _;
        self.0.slices_count = slices_data_offsets.len() as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> VideoDecodeH265PictureInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for VideoDecodeH265PictureInfoEXTBuilder<'a> {
    fn default() -> VideoDecodeH265PictureInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoDecodeH265PictureInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoDecodeH265PictureInfoEXTBuilder<'a> {
    type Target = VideoDecodeH265PictureInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoDecodeH265PictureInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoDecodeH265DpbSlotInfoEXT.html) · Structure"]
#[doc(alias = "VkVideoDecodeH265DpbSlotInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoDecodeH265DpbSlotInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub p_std_reference_info: *const crate::external::vk_video::StdVideoDecodeH265ReferenceInfo,
}
impl Default for VideoDecodeH265DpbSlotInfoEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::VIDEO_DECODE_H265_DPB_SLOT_INFO_EXT, p_next: std::ptr::null(), p_std_reference_info: std::ptr::null() }
    }
}
impl std::fmt::Debug for VideoDecodeH265DpbSlotInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoDecodeH265DpbSlotInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("p_std_reference_info", &self.p_std_reference_info).finish()
    }
}
impl VideoDecodeH265DpbSlotInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoDecodeH265DpbSlotInfoEXTBuilder<'a> {
        VideoDecodeH265DpbSlotInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoDecodeH265DpbSlotInfoEXT.html) · Builder of [`VideoDecodeH265DpbSlotInfoEXT`]"]
#[repr(transparent)]
pub struct VideoDecodeH265DpbSlotInfoEXTBuilder<'a>(VideoDecodeH265DpbSlotInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> VideoDecodeH265DpbSlotInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> VideoDecodeH265DpbSlotInfoEXTBuilder<'a> {
        VideoDecodeH265DpbSlotInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn std_reference_info(mut self, std_reference_info: &'a crate::external::vk_video::StdVideoDecodeH265ReferenceInfo) -> Self {
        self.0.p_std_reference_info = std_reference_info as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> VideoDecodeH265DpbSlotInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for VideoDecodeH265DpbSlotInfoEXTBuilder<'a> {
    fn default() -> VideoDecodeH265DpbSlotInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoDecodeH265DpbSlotInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoDecodeH265DpbSlotInfoEXTBuilder<'a> {
    type Target = VideoDecodeH265DpbSlotInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoDecodeH265DpbSlotInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[cfg(feature = "khr_video_queue")]
impl<'a> crate::ExtendableFromConst<'a, VideoDecodeH265SessionCreateInfoEXT> for crate::extensions::khr_video_queue::VideoSessionCreateInfoKHRBuilder<'a> {}
#[cfg(feature = "khr_video_queue")]
impl<'a> crate::ExtendableFromConst<'a, VideoDecodeH265SessionCreateInfoEXTBuilder<'_>> for crate::extensions::khr_video_queue::VideoSessionCreateInfoKHRBuilder<'a> {}
#[cfg(feature = "khr_video_queue")]
impl<'a> crate::ExtendableFromConst<'a, VideoDecodeH265SessionParametersCreateInfoEXT> for crate::extensions::khr_video_queue::VideoSessionParametersCreateInfoKHRBuilder<'a> {}
#[cfg(feature = "khr_video_queue")]
impl<'a> crate::ExtendableFromConst<'a, VideoDecodeH265SessionParametersCreateInfoEXTBuilder<'_>> for crate::extensions::khr_video_queue::VideoSessionParametersCreateInfoKHRBuilder<'a> {}
#[cfg(feature = "khr_video_queue")]
impl<'a> crate::ExtendableFromConst<'a, VideoDecodeH265SessionParametersAddInfoEXT> for crate::extensions::khr_video_queue::VideoSessionParametersUpdateInfoKHRBuilder<'a> {}
#[cfg(feature = "khr_video_queue")]
impl<'a> crate::ExtendableFromConst<'a, VideoDecodeH265SessionParametersAddInfoEXTBuilder<'_>> for crate::extensions::khr_video_queue::VideoSessionParametersUpdateInfoKHRBuilder<'a> {}
