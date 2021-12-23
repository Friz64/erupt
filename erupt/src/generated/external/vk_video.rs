#[doc = "<s>Vulkan Manual Page</s> · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct StdVideoH264ProfileIdc(pub i32);
impl std::fmt::Debug for StdVideoH264ProfileIdc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::BASELINE => "BASELINE",
            &Self::MAIN => "MAIN",
            &Self::HIGH => "HIGH",
            &Self::HIGH_444_PREDICTIVE => "HIGH_444_PREDICTIVE",
            &Self::INVALID => "INVALID",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::external::vk_video`]"]
impl crate::external::vk_video::StdVideoH264ProfileIdc {
    pub const BASELINE: Self = Self(66);
    pub const MAIN: Self = Self(77);
    pub const HIGH: Self = Self(100);
    pub const HIGH_444_PREDICTIVE: Self = Self(244);
    pub const INVALID: Self = Self(2147483647);
}
#[doc = "<s>Vulkan Manual Page</s> · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct StdVideoH264Level(pub i32);
impl std::fmt::Debug for StdVideoH264Level {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::_1_0 => "_1_0",
            &Self::_1_1 => "_1_1",
            &Self::_1_2 => "_1_2",
            &Self::_1_3 => "_1_3",
            &Self::_2_0 => "_2_0",
            &Self::_2_1 => "_2_1",
            &Self::_2_2 => "_2_2",
            &Self::_3_0 => "_3_0",
            &Self::_3_1 => "_3_1",
            &Self::_3_2 => "_3_2",
            &Self::_4_0 => "_4_0",
            &Self::_4_1 => "_4_1",
            &Self::_4_2 => "_4_2",
            &Self::_5_0 => "_5_0",
            &Self::_5_1 => "_5_1",
            &Self::_5_2 => "_5_2",
            &Self::_6_0 => "_6_0",
            &Self::_6_1 => "_6_1",
            &Self::_6_2 => "_6_2",
            &Self::INVALID => "INVALID",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::external::vk_video`]"]
impl crate::external::vk_video::StdVideoH264Level {
    pub const _1_0: Self = Self(0);
    pub const _1_1: Self = Self(1);
    pub const _1_2: Self = Self(2);
    pub const _1_3: Self = Self(3);
    pub const _2_0: Self = Self(4);
    pub const _2_1: Self = Self(5);
    pub const _2_2: Self = Self(6);
    pub const _3_0: Self = Self(7);
    pub const _3_1: Self = Self(8);
    pub const _3_2: Self = Self(9);
    pub const _4_0: Self = Self(10);
    pub const _4_1: Self = Self(11);
    pub const _4_2: Self = Self(12);
    pub const _5_0: Self = Self(13);
    pub const _5_1: Self = Self(14);
    pub const _5_2: Self = Self(15);
    pub const _6_0: Self = Self(16);
    pub const _6_1: Self = Self(17);
    pub const _6_2: Self = Self(18);
    pub const INVALID: Self = Self(2147483647);
}
#[doc = "<s>Vulkan Manual Page</s> · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct StdVideoH264ChromaFormatIdc(pub i32);
impl std::fmt::Debug for StdVideoH264ChromaFormatIdc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::MONOCHROME => "MONOCHROME",
            &Self::_420 => "_420",
            &Self::_422 => "_422",
            &Self::_444 => "_444",
            &Self::INVALID => "INVALID",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::external::vk_video`]"]
impl crate::external::vk_video::StdVideoH264ChromaFormatIdc {
    pub const MONOCHROME: Self = Self(0);
    pub const _420: Self = Self(1);
    pub const _422: Self = Self(2);
    pub const _444: Self = Self(3);
    pub const INVALID: Self = Self(2147483647);
}
#[doc = "<s>Vulkan Manual Page</s> · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct StdVideoH264PocType(pub i32);
impl std::fmt::Debug for StdVideoH264PocType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::_0 => "_0",
            &Self::_1 => "_1",
            &Self::_2 => "_2",
            &Self::INVALID => "INVALID",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::external::vk_video`]"]
impl crate::external::vk_video::StdVideoH264PocType {
    pub const _0: Self = Self(0);
    pub const _1: Self = Self(1);
    pub const _2: Self = Self(2);
    pub const INVALID: Self = Self(2147483647);
}
#[doc = "<s>Vulkan Manual Page</s> · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct StdVideoH264AspectRatioIdc(pub i32);
impl std::fmt::Debug for StdVideoH264AspectRatioIdc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::UNSPECIFIED => "UNSPECIFIED",
            &Self::SQUARE => "SQUARE",
            &Self::_12_11 => "_12_11",
            &Self::_10_11 => "_10_11",
            &Self::_16_11 => "_16_11",
            &Self::_40_33 => "_40_33",
            &Self::_24_11 => "_24_11",
            &Self::_20_11 => "_20_11",
            &Self::_32_11 => "_32_11",
            &Self::_80_33 => "_80_33",
            &Self::_18_11 => "_18_11",
            &Self::_15_11 => "_15_11",
            &Self::_64_33 => "_64_33",
            &Self::_160_99 => "_160_99",
            &Self::_4_3 => "_4_3",
            &Self::_3_2 => "_3_2",
            &Self::_2_1 => "_2_1",
            &Self::EXTENDED_SAR => "EXTENDED_SAR",
            &Self::INVALID => "INVALID",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::external::vk_video`]"]
impl crate::external::vk_video::StdVideoH264AspectRatioIdc {
    pub const UNSPECIFIED: Self = Self(0);
    pub const SQUARE: Self = Self(1);
    pub const _12_11: Self = Self(2);
    pub const _10_11: Self = Self(3);
    pub const _16_11: Self = Self(4);
    pub const _40_33: Self = Self(5);
    pub const _24_11: Self = Self(6);
    pub const _20_11: Self = Self(7);
    pub const _32_11: Self = Self(8);
    pub const _80_33: Self = Self(9);
    pub const _18_11: Self = Self(10);
    pub const _15_11: Self = Self(11);
    pub const _64_33: Self = Self(12);
    pub const _160_99: Self = Self(13);
    pub const _4_3: Self = Self(14);
    pub const _3_2: Self = Self(15);
    pub const _2_1: Self = Self(16);
    pub const EXTENDED_SAR: Self = Self(255);
    pub const INVALID: Self = Self(2147483647);
}
#[doc = "<s>Vulkan Manual Page</s> · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct StdVideoH264WeightedBipredIdc(pub i32);
impl std::fmt::Debug for StdVideoH264WeightedBipredIdc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::DEFAULT => "DEFAULT",
            &Self::EXPLICIT => "EXPLICIT",
            &Self::IMPLICIT => "IMPLICIT",
            &Self::INVALID => "INVALID",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::external::vk_video`]"]
impl crate::external::vk_video::StdVideoH264WeightedBipredIdc {
    pub const DEFAULT: Self = Self(0);
    pub const EXPLICIT: Self = Self(1);
    pub const IMPLICIT: Self = Self(2);
    pub const INVALID: Self = Self(2147483647);
}
#[doc = "<s>Vulkan Manual Page</s> · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct StdVideoH264SliceType(pub i32);
impl std::fmt::Debug for StdVideoH264SliceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::P => "P",
            &Self::B => "B",
            &Self::I => "I",
            &Self::INVALID => "INVALID",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::external::vk_video`]"]
impl crate::external::vk_video::StdVideoH264SliceType {
    pub const P: Self = Self(0);
    pub const B: Self = Self(1);
    pub const I: Self = Self(2);
    pub const INVALID: Self = Self(2147483647);
}
#[doc = "<s>Vulkan Manual Page</s> · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct StdVideoH264CabacInitIdc(pub i32);
impl std::fmt::Debug for StdVideoH264CabacInitIdc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::_0 => "_0",
            &Self::_1 => "_1",
            &Self::_2 => "_2",
            &Self::INVALID => "INVALID",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::external::vk_video`]"]
impl crate::external::vk_video::StdVideoH264CabacInitIdc {
    pub const _0: Self = Self(0);
    pub const _1: Self = Self(1);
    pub const _2: Self = Self(2);
    pub const INVALID: Self = Self(2147483647);
}
#[doc = "<s>Vulkan Manual Page</s> · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct StdVideoH264DisableDeblockingFilterIdc(pub i32);
impl std::fmt::Debug for StdVideoH264DisableDeblockingFilterIdc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::DISABLED => "DISABLED",
            &Self::ENABLED => "ENABLED",
            &Self::PARTIAL => "PARTIAL",
            &Self::INVALID => "INVALID",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::external::vk_video`]"]
impl crate::external::vk_video::StdVideoH264DisableDeblockingFilterIdc {
    pub const DISABLED: Self = Self(0);
    pub const ENABLED: Self = Self(1);
    pub const PARTIAL: Self = Self(2);
    pub const INVALID: Self = Self(2147483647);
}
#[doc = "<s>Vulkan Manual Page</s> · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct StdVideoH264PictureType(pub i32);
impl std::fmt::Debug for StdVideoH264PictureType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::P => "P",
            &Self::B => "B",
            &Self::I => "I",
            &Self::IDR => "IDR",
            &Self::INVALID => "INVALID",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::external::vk_video`]"]
impl crate::external::vk_video::StdVideoH264PictureType {
    pub const P: Self = Self(0);
    pub const B: Self = Self(1);
    pub const I: Self = Self(2);
    pub const IDR: Self = Self(5);
    pub const INVALID: Self = Self(2147483647);
}
#[doc = "<s>Vulkan Manual Page</s> · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct StdVideoH264ModificationOfPicNumsIdc(pub i32);
impl std::fmt::Debug for StdVideoH264ModificationOfPicNumsIdc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::SHORT_TERM_SUBTRACT => "SHORT_TERM_SUBTRACT",
            &Self::SHORT_TERM_ADD => "SHORT_TERM_ADD",
            &Self::LONG_TERM => "LONG_TERM",
            &Self::END => "END",
            &Self::INVALID => "INVALID",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::external::vk_video`]"]
impl crate::external::vk_video::StdVideoH264ModificationOfPicNumsIdc {
    pub const SHORT_TERM_SUBTRACT: Self = Self(0);
    pub const SHORT_TERM_ADD: Self = Self(1);
    pub const LONG_TERM: Self = Self(2);
    pub const END: Self = Self(3);
    pub const INVALID: Self = Self(2147483647);
}
#[doc = "<s>Vulkan Manual Page</s> · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct StdVideoH264MemMgmtControlOp(pub i32);
impl std::fmt::Debug for StdVideoH264MemMgmtControlOp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::END => "END",
            &Self::UNMARK_SHORT_TERM => "UNMARK_SHORT_TERM",
            &Self::UNMARK_LONG_TERM => "UNMARK_LONG_TERM",
            &Self::MARK_LONG_TERM => "MARK_LONG_TERM",
            &Self::SET_MAX_LONG_TERM_INDEX => "SET_MAX_LONG_TERM_INDEX",
            &Self::UNMARK_ALL => "UNMARK_ALL",
            &Self::MARK_CURRENT_AS_LONG_TERM => "MARK_CURRENT_AS_LONG_TERM",
            &Self::INVALID => "INVALID",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::external::vk_video`]"]
impl crate::external::vk_video::StdVideoH264MemMgmtControlOp {
    pub const END: Self = Self(0);
    pub const UNMARK_SHORT_TERM: Self = Self(1);
    pub const UNMARK_LONG_TERM: Self = Self(2);
    pub const MARK_LONG_TERM: Self = Self(3);
    pub const SET_MAX_LONG_TERM_INDEX: Self = Self(4);
    pub const UNMARK_ALL: Self = Self(5);
    pub const MARK_CURRENT_AS_LONG_TERM: Self = Self(6);
    pub const INVALID: Self = Self(2147483647);
}
#[doc = "<s>Vulkan Manual Page</s> · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct StdVideoH265ProfileIdc(pub i32);
impl std::fmt::Debug for StdVideoH265ProfileIdc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::MAIN => "MAIN",
            &Self::MAIN_10 => "MAIN_10",
            &Self::MAIN_STILL_PICTURE => "MAIN_STILL_PICTURE",
            &Self::FORMAT_RANGE_EXTENSIONS => "FORMAT_RANGE_EXTENSIONS",
            &Self::SCC_EXTENSIONS => "SCC_EXTENSIONS",
            &Self::INVALID => "INVALID",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::external::vk_video`]"]
impl crate::external::vk_video::StdVideoH265ProfileIdc {
    pub const MAIN: Self = Self(1);
    pub const MAIN_10: Self = Self(2);
    pub const MAIN_STILL_PICTURE: Self = Self(3);
    pub const FORMAT_RANGE_EXTENSIONS: Self = Self(4);
    pub const SCC_EXTENSIONS: Self = Self(9);
    pub const INVALID: Self = Self(2147483647);
}
#[doc = "<s>Vulkan Manual Page</s> · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct StdVideoH265Level(pub i32);
impl std::fmt::Debug for StdVideoH265Level {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::_1_0 => "_1_0",
            &Self::_2_0 => "_2_0",
            &Self::_2_1 => "_2_1",
            &Self::_3_0 => "_3_0",
            &Self::_3_1 => "_3_1",
            &Self::_4_0 => "_4_0",
            &Self::_4_1 => "_4_1",
            &Self::_5_0 => "_5_0",
            &Self::_5_1 => "_5_1",
            &Self::_5_2 => "_5_2",
            &Self::_6_0 => "_6_0",
            &Self::_6_1 => "_6_1",
            &Self::_6_2 => "_6_2",
            &Self::INVALID => "INVALID",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::external::vk_video`]"]
impl crate::external::vk_video::StdVideoH265Level {
    pub const _1_0: Self = Self(0);
    pub const _2_0: Self = Self(1);
    pub const _2_1: Self = Self(2);
    pub const _3_0: Self = Self(3);
    pub const _3_1: Self = Self(4);
    pub const _4_0: Self = Self(5);
    pub const _4_1: Self = Self(6);
    pub const _5_0: Self = Self(7);
    pub const _5_1: Self = Self(8);
    pub const _5_2: Self = Self(9);
    pub const _6_0: Self = Self(10);
    pub const _6_1: Self = Self(11);
    pub const _6_2: Self = Self(12);
    pub const INVALID: Self = Self(2147483647);
}
#[doc = "<s>Vulkan Manual Page</s> · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct StdVideoH265SliceType(pub i32);
impl std::fmt::Debug for StdVideoH265SliceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::B => "B",
            &Self::P => "P",
            &Self::I => "I",
            &Self::INVALID => "INVALID",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::external::vk_video`]"]
impl crate::external::vk_video::StdVideoH265SliceType {
    pub const B: Self = Self(0);
    pub const P: Self = Self(1);
    pub const I: Self = Self(2);
    pub const INVALID: Self = Self(2147483647);
}
#[doc = "<s>Vulkan Manual Page</s> · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct StdVideoH265PictureType(pub i32);
impl std::fmt::Debug for StdVideoH265PictureType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::P => "P",
            &Self::B => "B",
            &Self::I => "I",
            &Self::IDR => "IDR",
            &Self::INVALID => "INVALID",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::external::vk_video`]"]
impl crate::external::vk_video::StdVideoH265PictureType {
    pub const P: Self = Self(0);
    pub const B: Self = Self(1);
    pub const I: Self = Self(2);
    pub const IDR: Self = Self(3);
    pub const INVALID: Self = Self(2147483647);
}
#[doc = "<s>Vulkan Manual Page</s> · Structure"]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct StdVideoH264SpsFlags {
    pub constraint_set0_flag_and_more_bitfield: u32,
}
impl Default for StdVideoH264SpsFlags {
    fn default() -> Self {
        Self { constraint_set0_flag_and_more_bitfield: Default::default() }
    }
}
impl std::fmt::Debug for StdVideoH264SpsFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StdVideoH264SpsFlags").field("constraint_set0_flag_and_more_bitfield", &format!("{:#b}", &self.constraint_set0_flag_and_more_bitfield)).finish()
    }
}
impl StdVideoH264SpsFlags {
    #[inline]
    pub fn into_builder<'a>(self) -> StdVideoH264SpsFlagsBuilder<'a> {
        StdVideoH264SpsFlagsBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "<s>Vulkan Manual Page</s> · Builder of [`StdVideoH264SpsFlags`]"]
#[repr(transparent)]
pub struct StdVideoH264SpsFlagsBuilder<'a>(StdVideoH264SpsFlags, std::marker::PhantomData<&'a ()>);
impl<'a> StdVideoH264SpsFlagsBuilder<'a> {
    #[inline]
    pub fn new() -> StdVideoH264SpsFlagsBuilder<'a> {
        StdVideoH264SpsFlagsBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn constraint_set0_flag(mut self, constraint_set0_flag: u32) -> Self {
        self.0.constraint_set0_flag_and_more_bitfield = crate::bits_copy!(self.0.constraint_set0_flag_and_more_bitfield, constraint_set0_flag, 0usize, 0usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn constraint_set1_flag(mut self, constraint_set1_flag: u32) -> Self {
        self.0.constraint_set0_flag_and_more_bitfield = crate::bits_copy!(self.0.constraint_set0_flag_and_more_bitfield, constraint_set1_flag, 1usize, 1usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn constraint_set2_flag(mut self, constraint_set2_flag: u32) -> Self {
        self.0.constraint_set0_flag_and_more_bitfield = crate::bits_copy!(self.0.constraint_set0_flag_and_more_bitfield, constraint_set2_flag, 2usize, 2usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn constraint_set3_flag(mut self, constraint_set3_flag: u32) -> Self {
        self.0.constraint_set0_flag_and_more_bitfield = crate::bits_copy!(self.0.constraint_set0_flag_and_more_bitfield, constraint_set3_flag, 3usize, 3usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn constraint_set4_flag(mut self, constraint_set4_flag: u32) -> Self {
        self.0.constraint_set0_flag_and_more_bitfield = crate::bits_copy!(self.0.constraint_set0_flag_and_more_bitfield, constraint_set4_flag, 4usize, 4usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn constraint_set5_flag(mut self, constraint_set5_flag: u32) -> Self {
        self.0.constraint_set0_flag_and_more_bitfield = crate::bits_copy!(self.0.constraint_set0_flag_and_more_bitfield, constraint_set5_flag, 5usize, 5usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn direct_8x8_inference_flag(mut self, direct_8x8_inference_flag: u32) -> Self {
        self.0.constraint_set0_flag_and_more_bitfield = crate::bits_copy!(self.0.constraint_set0_flag_and_more_bitfield, direct_8x8_inference_flag, 6usize, 6usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn mb_adaptive_frame_field_flag(mut self, mb_adaptive_frame_field_flag: u32) -> Self {
        self.0.constraint_set0_flag_and_more_bitfield = crate::bits_copy!(self.0.constraint_set0_flag_and_more_bitfield, mb_adaptive_frame_field_flag, 7usize, 7usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn frame_mbs_only_flag(mut self, frame_mbs_only_flag: u32) -> Self {
        self.0.constraint_set0_flag_and_more_bitfield = crate::bits_copy!(self.0.constraint_set0_flag_and_more_bitfield, frame_mbs_only_flag, 8usize, 8usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn delta_pic_order_always_zero_flag(mut self, delta_pic_order_always_zero_flag: u32) -> Self {
        self.0.constraint_set0_flag_and_more_bitfield = crate::bits_copy!(self.0.constraint_set0_flag_and_more_bitfield, delta_pic_order_always_zero_flag, 9usize, 9usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn separate_colour_plane_flag(mut self, separate_colour_plane_flag: u32) -> Self {
        self.0.constraint_set0_flag_and_more_bitfield = crate::bits_copy!(self.0.constraint_set0_flag_and_more_bitfield, separate_colour_plane_flag, 10usize, 10usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn gaps_in_frame_num_value_allowed_flag(mut self, gaps_in_frame_num_value_allowed_flag: u32) -> Self {
        self.0.constraint_set0_flag_and_more_bitfield = crate::bits_copy!(self.0.constraint_set0_flag_and_more_bitfield, gaps_in_frame_num_value_allowed_flag, 11usize, 11usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn qpprime_y_zero_transform_bypass_flag(mut self, qpprime_y_zero_transform_bypass_flag: u32) -> Self {
        self.0.constraint_set0_flag_and_more_bitfield = crate::bits_copy!(self.0.constraint_set0_flag_and_more_bitfield, qpprime_y_zero_transform_bypass_flag, 12usize, 12usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn frame_cropping_flag(mut self, frame_cropping_flag: u32) -> Self {
        self.0.constraint_set0_flag_and_more_bitfield = crate::bits_copy!(self.0.constraint_set0_flag_and_more_bitfield, frame_cropping_flag, 13usize, 13usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn seq_scaling_matrix_present_flag(mut self, seq_scaling_matrix_present_flag: u32) -> Self {
        self.0.constraint_set0_flag_and_more_bitfield = crate::bits_copy!(self.0.constraint_set0_flag_and_more_bitfield, seq_scaling_matrix_present_flag, 14usize, 14usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn vui_parameters_present_flag(mut self, vui_parameters_present_flag: u32) -> Self {
        self.0.constraint_set0_flag_and_more_bitfield = crate::bits_copy!(self.0.constraint_set0_flag_and_more_bitfield, vui_parameters_present_flag, 15usize, 15usize);
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> StdVideoH264SpsFlags {
        self.0
    }
}
impl<'a> std::default::Default for StdVideoH264SpsFlagsBuilder<'a> {
    fn default() -> StdVideoH264SpsFlagsBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for StdVideoH264SpsFlagsBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for StdVideoH264SpsFlagsBuilder<'a> {
    type Target = StdVideoH264SpsFlags;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StdVideoH264SpsFlagsBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Structure"]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct StdVideoH264ScalingLists {
    pub scaling_list_present_mask: u8,
    pub use_default_scaling_matrix_mask: u8,
    pub scaling_list4x4: [[u8; 16]; 6],
    pub scaling_list8x8: [[u8; 64]; 2],
}
impl Default for StdVideoH264ScalingLists {
    fn default() -> Self {
        Self { scaling_list_present_mask: Default::default(), use_default_scaling_matrix_mask: Default::default(), scaling_list4x4: unsafe { std::mem::zeroed() }, scaling_list8x8: unsafe { std::mem::zeroed() } }
    }
}
impl std::fmt::Debug for StdVideoH264ScalingLists {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StdVideoH264ScalingLists").field("scaling_list_present_mask", &self.scaling_list_present_mask).field("use_default_scaling_matrix_mask", &self.use_default_scaling_matrix_mask).field("scaling_list4x4", &self.scaling_list4x4).field("scaling_list8x8", &self.scaling_list8x8).finish()
    }
}
impl StdVideoH264ScalingLists {
    #[inline]
    pub fn into_builder<'a>(self) -> StdVideoH264ScalingListsBuilder<'a> {
        StdVideoH264ScalingListsBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "<s>Vulkan Manual Page</s> · Builder of [`StdVideoH264ScalingLists`]"]
#[repr(transparent)]
pub struct StdVideoH264ScalingListsBuilder<'a>(StdVideoH264ScalingLists, std::marker::PhantomData<&'a ()>);
impl<'a> StdVideoH264ScalingListsBuilder<'a> {
    #[inline]
    pub fn new() -> StdVideoH264ScalingListsBuilder<'a> {
        StdVideoH264ScalingListsBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn scaling_list_present_mask(mut self, scaling_list_present_mask: u8) -> Self {
        self.0.scaling_list_present_mask = scaling_list_present_mask as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn use_default_scaling_matrix_mask(mut self, use_default_scaling_matrix_mask: u8) -> Self {
        self.0.use_default_scaling_matrix_mask = use_default_scaling_matrix_mask as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn scaling_list4x4(mut self, scaling_list4x4: [[u8; 16]; 6]) -> Self {
        self.0.scaling_list4x4 = scaling_list4x4 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn scaling_list8x8(mut self, scaling_list8x8: [[u8; 64]; 2]) -> Self {
        self.0.scaling_list8x8 = scaling_list8x8 as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> StdVideoH264ScalingLists {
        self.0
    }
}
impl<'a> std::default::Default for StdVideoH264ScalingListsBuilder<'a> {
    fn default() -> StdVideoH264ScalingListsBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for StdVideoH264ScalingListsBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for StdVideoH264ScalingListsBuilder<'a> {
    type Target = StdVideoH264ScalingLists;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StdVideoH264ScalingListsBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StdVideoH264SequenceParameterSetVui {
    pub aspect_ratio_idc: crate::external::vk_video::StdVideoH264AspectRatioIdc,
    pub sar_width: u16,
    pub sar_height: u16,
    pub video_format: u8,
    pub color_primaries: u8,
    pub transfer_characteristics: u8,
    pub matrix_coefficients: u8,
    pub num_units_in_tick: u32,
    pub time_scale: u32,
    pub p_hrd_parameters: *mut crate::external::vk_video::StdVideoH264HrdParameters,
    pub max_num_reorder_frames: u8,
    pub max_dec_frame_buffering: u8,
    pub flags: crate::external::vk_video::StdVideoH264SpsVuiFlags,
}
impl Default for StdVideoH264SequenceParameterSetVui {
    fn default() -> Self {
        Self { aspect_ratio_idc: Default::default(), sar_width: Default::default(), sar_height: Default::default(), video_format: Default::default(), color_primaries: Default::default(), transfer_characteristics: Default::default(), matrix_coefficients: Default::default(), num_units_in_tick: Default::default(), time_scale: Default::default(), p_hrd_parameters: std::ptr::null_mut(), max_num_reorder_frames: Default::default(), max_dec_frame_buffering: Default::default(), flags: Default::default() }
    }
}
impl std::fmt::Debug for StdVideoH264SequenceParameterSetVui {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StdVideoH264SequenceParameterSetVui").field("aspect_ratio_idc", &self.aspect_ratio_idc).field("sar_width", &self.sar_width).field("sar_height", &self.sar_height).field("video_format", &self.video_format).field("color_primaries", &self.color_primaries).field("transfer_characteristics", &self.transfer_characteristics).field("matrix_coefficients", &self.matrix_coefficients).field("num_units_in_tick", &self.num_units_in_tick).field("time_scale", &self.time_scale).field("p_hrd_parameters", &self.p_hrd_parameters).field("max_num_reorder_frames", &self.max_num_reorder_frames).field("max_dec_frame_buffering", &self.max_dec_frame_buffering).field("flags", &self.flags).finish()
    }
}
impl StdVideoH264SequenceParameterSetVui {
    #[inline]
    pub fn into_builder<'a>(self) -> StdVideoH264SequenceParameterSetVuiBuilder<'a> {
        StdVideoH264SequenceParameterSetVuiBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "<s>Vulkan Manual Page</s> · Builder of [`StdVideoH264SequenceParameterSetVui`]"]
#[repr(transparent)]
pub struct StdVideoH264SequenceParameterSetVuiBuilder<'a>(StdVideoH264SequenceParameterSetVui, std::marker::PhantomData<&'a ()>);
impl<'a> StdVideoH264SequenceParameterSetVuiBuilder<'a> {
    #[inline]
    pub fn new() -> StdVideoH264SequenceParameterSetVuiBuilder<'a> {
        StdVideoH264SequenceParameterSetVuiBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn aspect_ratio_idc(mut self, aspect_ratio_idc: crate::external::vk_video::StdVideoH264AspectRatioIdc) -> Self {
        self.0.aspect_ratio_idc = aspect_ratio_idc as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn sar_width(mut self, sar_width: u16) -> Self {
        self.0.sar_width = sar_width as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn sar_height(mut self, sar_height: u16) -> Self {
        self.0.sar_height = sar_height as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn video_format(mut self, video_format: u8) -> Self {
        self.0.video_format = video_format as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn color_primaries(mut self, color_primaries: u8) -> Self {
        self.0.color_primaries = color_primaries as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn transfer_characteristics(mut self, transfer_characteristics: u8) -> Self {
        self.0.transfer_characteristics = transfer_characteristics as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn matrix_coefficients(mut self, matrix_coefficients: u8) -> Self {
        self.0.matrix_coefficients = matrix_coefficients as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn num_units_in_tick(mut self, num_units_in_tick: u32) -> Self {
        self.0.num_units_in_tick = num_units_in_tick as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn time_scale(mut self, time_scale: u32) -> Self {
        self.0.time_scale = time_scale as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn hrd_parameters(mut self, hrd_parameters: &'a mut crate::external::vk_video::StdVideoH264HrdParameters) -> Self {
        self.0.p_hrd_parameters = hrd_parameters as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn max_num_reorder_frames(mut self, max_num_reorder_frames: u8) -> Self {
        self.0.max_num_reorder_frames = max_num_reorder_frames as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn max_dec_frame_buffering(mut self, max_dec_frame_buffering: u8) -> Self {
        self.0.max_dec_frame_buffering = max_dec_frame_buffering as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn flags(mut self, flags: crate::external::vk_video::StdVideoH264SpsVuiFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> StdVideoH264SequenceParameterSetVui {
        self.0
    }
}
impl<'a> std::default::Default for StdVideoH264SequenceParameterSetVuiBuilder<'a> {
    fn default() -> StdVideoH264SequenceParameterSetVuiBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for StdVideoH264SequenceParameterSetVuiBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for StdVideoH264SequenceParameterSetVuiBuilder<'a> {
    type Target = StdVideoH264SequenceParameterSetVui;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StdVideoH264SequenceParameterSetVuiBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Structure"]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct StdVideoH264HrdParameters {
    pub cpb_cnt_minus1: u8,
    pub bit_rate_scale: u8,
    pub cpb_size_scale: u8,
    pub bit_rate_value_minus1: [u32; 32],
    pub cpb_size_value_minus1: [u32; 32],
    pub cbr_flag: [u8; 32],
    pub initial_cpb_removal_delay_length_minus1: u32,
    pub cpb_removal_delay_length_minus1: u32,
    pub dpb_output_delay_length_minus1: u32,
    pub time_offset_length: u32,
}
impl Default for StdVideoH264HrdParameters {
    fn default() -> Self {
        Self { cpb_cnt_minus1: Default::default(), bit_rate_scale: Default::default(), cpb_size_scale: Default::default(), bit_rate_value_minus1: unsafe { std::mem::zeroed() }, cpb_size_value_minus1: unsafe { std::mem::zeroed() }, cbr_flag: unsafe { std::mem::zeroed() }, initial_cpb_removal_delay_length_minus1: Default::default(), cpb_removal_delay_length_minus1: Default::default(), dpb_output_delay_length_minus1: Default::default(), time_offset_length: Default::default() }
    }
}
impl std::fmt::Debug for StdVideoH264HrdParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StdVideoH264HrdParameters").field("cpb_cnt_minus1", &self.cpb_cnt_minus1).field("bit_rate_scale", &self.bit_rate_scale).field("cpb_size_scale", &self.cpb_size_scale).field("bit_rate_value_minus1", &self.bit_rate_value_minus1).field("cpb_size_value_minus1", &self.cpb_size_value_minus1).field("cbr_flag", &self.cbr_flag).field("initial_cpb_removal_delay_length_minus1", &self.initial_cpb_removal_delay_length_minus1).field("cpb_removal_delay_length_minus1", &self.cpb_removal_delay_length_minus1).field("dpb_output_delay_length_minus1", &self.dpb_output_delay_length_minus1).field("time_offset_length", &self.time_offset_length).finish()
    }
}
impl StdVideoH264HrdParameters {
    #[inline]
    pub fn into_builder<'a>(self) -> StdVideoH264HrdParametersBuilder<'a> {
        StdVideoH264HrdParametersBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "<s>Vulkan Manual Page</s> · Builder of [`StdVideoH264HrdParameters`]"]
#[repr(transparent)]
pub struct StdVideoH264HrdParametersBuilder<'a>(StdVideoH264HrdParameters, std::marker::PhantomData<&'a ()>);
impl<'a> StdVideoH264HrdParametersBuilder<'a> {
    #[inline]
    pub fn new() -> StdVideoH264HrdParametersBuilder<'a> {
        StdVideoH264HrdParametersBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn cpb_cnt_minus1(mut self, cpb_cnt_minus1: u8) -> Self {
        self.0.cpb_cnt_minus1 = cpb_cnt_minus1 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn bit_rate_scale(mut self, bit_rate_scale: u8) -> Self {
        self.0.bit_rate_scale = bit_rate_scale as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn cpb_size_scale(mut self, cpb_size_scale: u8) -> Self {
        self.0.cpb_size_scale = cpb_size_scale as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn bit_rate_value_minus1(mut self, bit_rate_value_minus1: [u32; 32]) -> Self {
        self.0.bit_rate_value_minus1 = bit_rate_value_minus1 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn cpb_size_value_minus1(mut self, cpb_size_value_minus1: [u32; 32]) -> Self {
        self.0.cpb_size_value_minus1 = cpb_size_value_minus1 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn cbr_flag(mut self, cbr_flag: [u8; 32]) -> Self {
        self.0.cbr_flag = cbr_flag as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn initial_cpb_removal_delay_length_minus1(mut self, initial_cpb_removal_delay_length_minus1: u32) -> Self {
        self.0.initial_cpb_removal_delay_length_minus1 = initial_cpb_removal_delay_length_minus1 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn cpb_removal_delay_length_minus1(mut self, cpb_removal_delay_length_minus1: u32) -> Self {
        self.0.cpb_removal_delay_length_minus1 = cpb_removal_delay_length_minus1 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn dpb_output_delay_length_minus1(mut self, dpb_output_delay_length_minus1: u32) -> Self {
        self.0.dpb_output_delay_length_minus1 = dpb_output_delay_length_minus1 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn time_offset_length(mut self, time_offset_length: u32) -> Self {
        self.0.time_offset_length = time_offset_length as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> StdVideoH264HrdParameters {
        self.0
    }
}
impl<'a> std::default::Default for StdVideoH264HrdParametersBuilder<'a> {
    fn default() -> StdVideoH264HrdParametersBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for StdVideoH264HrdParametersBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for StdVideoH264HrdParametersBuilder<'a> {
    type Target = StdVideoH264HrdParameters;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StdVideoH264HrdParametersBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Structure"]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct StdVideoH264SpsVuiFlags {
    pub aspect_ratio_info_present_flag_and_more_bitfield: u32,
}
impl Default for StdVideoH264SpsVuiFlags {
    fn default() -> Self {
        Self { aspect_ratio_info_present_flag_and_more_bitfield: Default::default() }
    }
}
impl std::fmt::Debug for StdVideoH264SpsVuiFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StdVideoH264SpsVuiFlags").field("aspect_ratio_info_present_flag_and_more_bitfield", &format!("{:#b}", &self.aspect_ratio_info_present_flag_and_more_bitfield)).finish()
    }
}
impl StdVideoH264SpsVuiFlags {
    #[inline]
    pub fn into_builder<'a>(self) -> StdVideoH264SpsVuiFlagsBuilder<'a> {
        StdVideoH264SpsVuiFlagsBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "<s>Vulkan Manual Page</s> · Builder of [`StdVideoH264SpsVuiFlags`]"]
#[repr(transparent)]
pub struct StdVideoH264SpsVuiFlagsBuilder<'a>(StdVideoH264SpsVuiFlags, std::marker::PhantomData<&'a ()>);
impl<'a> StdVideoH264SpsVuiFlagsBuilder<'a> {
    #[inline]
    pub fn new() -> StdVideoH264SpsVuiFlagsBuilder<'a> {
        StdVideoH264SpsVuiFlagsBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn aspect_ratio_info_present_flag(mut self, aspect_ratio_info_present_flag: u32) -> Self {
        self.0.aspect_ratio_info_present_flag_and_more_bitfield = crate::bits_copy!(self.0.aspect_ratio_info_present_flag_and_more_bitfield, aspect_ratio_info_present_flag, 0usize, 0usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn overscan_info_present_flag(mut self, overscan_info_present_flag: u32) -> Self {
        self.0.aspect_ratio_info_present_flag_and_more_bitfield = crate::bits_copy!(self.0.aspect_ratio_info_present_flag_and_more_bitfield, overscan_info_present_flag, 1usize, 1usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn overscan_appropriate_flag(mut self, overscan_appropriate_flag: u32) -> Self {
        self.0.aspect_ratio_info_present_flag_and_more_bitfield = crate::bits_copy!(self.0.aspect_ratio_info_present_flag_and_more_bitfield, overscan_appropriate_flag, 2usize, 2usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn video_signal_type_present_flag(mut self, video_signal_type_present_flag: u32) -> Self {
        self.0.aspect_ratio_info_present_flag_and_more_bitfield = crate::bits_copy!(self.0.aspect_ratio_info_present_flag_and_more_bitfield, video_signal_type_present_flag, 3usize, 3usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn video_full_range_flag(mut self, video_full_range_flag: u32) -> Self {
        self.0.aspect_ratio_info_present_flag_and_more_bitfield = crate::bits_copy!(self.0.aspect_ratio_info_present_flag_and_more_bitfield, video_full_range_flag, 4usize, 4usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn color_description_present_flag(mut self, color_description_present_flag: u32) -> Self {
        self.0.aspect_ratio_info_present_flag_and_more_bitfield = crate::bits_copy!(self.0.aspect_ratio_info_present_flag_and_more_bitfield, color_description_present_flag, 5usize, 5usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn chroma_loc_info_present_flag(mut self, chroma_loc_info_present_flag: u32) -> Self {
        self.0.aspect_ratio_info_present_flag_and_more_bitfield = crate::bits_copy!(self.0.aspect_ratio_info_present_flag_and_more_bitfield, chroma_loc_info_present_flag, 6usize, 6usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn timing_info_present_flag(mut self, timing_info_present_flag: u32) -> Self {
        self.0.aspect_ratio_info_present_flag_and_more_bitfield = crate::bits_copy!(self.0.aspect_ratio_info_present_flag_and_more_bitfield, timing_info_present_flag, 7usize, 7usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn fixed_frame_rate_flag(mut self, fixed_frame_rate_flag: u32) -> Self {
        self.0.aspect_ratio_info_present_flag_and_more_bitfield = crate::bits_copy!(self.0.aspect_ratio_info_present_flag_and_more_bitfield, fixed_frame_rate_flag, 8usize, 8usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn bitstream_restriction_flag(mut self, bitstream_restriction_flag: u32) -> Self {
        self.0.aspect_ratio_info_present_flag_and_more_bitfield = crate::bits_copy!(self.0.aspect_ratio_info_present_flag_and_more_bitfield, bitstream_restriction_flag, 9usize, 9usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn nal_hrd_parameters_present_flag(mut self, nal_hrd_parameters_present_flag: u32) -> Self {
        self.0.aspect_ratio_info_present_flag_and_more_bitfield = crate::bits_copy!(self.0.aspect_ratio_info_present_flag_and_more_bitfield, nal_hrd_parameters_present_flag, 10usize, 10usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn vcl_hrd_parameters_present_flag(mut self, vcl_hrd_parameters_present_flag: u32) -> Self {
        self.0.aspect_ratio_info_present_flag_and_more_bitfield = crate::bits_copy!(self.0.aspect_ratio_info_present_flag_and_more_bitfield, vcl_hrd_parameters_present_flag, 11usize, 11usize);
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> StdVideoH264SpsVuiFlags {
        self.0
    }
}
impl<'a> std::default::Default for StdVideoH264SpsVuiFlagsBuilder<'a> {
    fn default() -> StdVideoH264SpsVuiFlagsBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for StdVideoH264SpsVuiFlagsBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for StdVideoH264SpsVuiFlagsBuilder<'a> {
    type Target = StdVideoH264SpsVuiFlags;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StdVideoH264SpsVuiFlagsBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Structure"]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct StdVideoH264PpsFlags {
    pub transform_8x8_mode_flag_and_more_bitfield: u32,
}
impl Default for StdVideoH264PpsFlags {
    fn default() -> Self {
        Self { transform_8x8_mode_flag_and_more_bitfield: Default::default() }
    }
}
impl std::fmt::Debug for StdVideoH264PpsFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StdVideoH264PpsFlags").field("transform_8x8_mode_flag_and_more_bitfield", &format!("{:#b}", &self.transform_8x8_mode_flag_and_more_bitfield)).finish()
    }
}
impl StdVideoH264PpsFlags {
    #[inline]
    pub fn into_builder<'a>(self) -> StdVideoH264PpsFlagsBuilder<'a> {
        StdVideoH264PpsFlagsBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "<s>Vulkan Manual Page</s> · Builder of [`StdVideoH264PpsFlags`]"]
#[repr(transparent)]
pub struct StdVideoH264PpsFlagsBuilder<'a>(StdVideoH264PpsFlags, std::marker::PhantomData<&'a ()>);
impl<'a> StdVideoH264PpsFlagsBuilder<'a> {
    #[inline]
    pub fn new() -> StdVideoH264PpsFlagsBuilder<'a> {
        StdVideoH264PpsFlagsBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn transform_8x8_mode_flag(mut self, transform_8x8_mode_flag: u32) -> Self {
        self.0.transform_8x8_mode_flag_and_more_bitfield = crate::bits_copy!(self.0.transform_8x8_mode_flag_and_more_bitfield, transform_8x8_mode_flag, 0usize, 0usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn redundant_pic_cnt_present_flag(mut self, redundant_pic_cnt_present_flag: u32) -> Self {
        self.0.transform_8x8_mode_flag_and_more_bitfield = crate::bits_copy!(self.0.transform_8x8_mode_flag_and_more_bitfield, redundant_pic_cnt_present_flag, 1usize, 1usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn constrained_intra_pred_flag(mut self, constrained_intra_pred_flag: u32) -> Self {
        self.0.transform_8x8_mode_flag_and_more_bitfield = crate::bits_copy!(self.0.transform_8x8_mode_flag_and_more_bitfield, constrained_intra_pred_flag, 2usize, 2usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn deblocking_filter_control_present_flag(mut self, deblocking_filter_control_present_flag: u32) -> Self {
        self.0.transform_8x8_mode_flag_and_more_bitfield = crate::bits_copy!(self.0.transform_8x8_mode_flag_and_more_bitfield, deblocking_filter_control_present_flag, 3usize, 3usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn weighted_bipred_idc_flag(mut self, weighted_bipred_idc_flag: u32) -> Self {
        self.0.transform_8x8_mode_flag_and_more_bitfield = crate::bits_copy!(self.0.transform_8x8_mode_flag_and_more_bitfield, weighted_bipred_idc_flag, 4usize, 4usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn weighted_pred_flag(mut self, weighted_pred_flag: u32) -> Self {
        self.0.transform_8x8_mode_flag_and_more_bitfield = crate::bits_copy!(self.0.transform_8x8_mode_flag_and_more_bitfield, weighted_pred_flag, 5usize, 5usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn pic_order_present_flag(mut self, pic_order_present_flag: u32) -> Self {
        self.0.transform_8x8_mode_flag_and_more_bitfield = crate::bits_copy!(self.0.transform_8x8_mode_flag_and_more_bitfield, pic_order_present_flag, 6usize, 6usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn entropy_coding_mode_flag(mut self, entropy_coding_mode_flag: u32) -> Self {
        self.0.transform_8x8_mode_flag_and_more_bitfield = crate::bits_copy!(self.0.transform_8x8_mode_flag_and_more_bitfield, entropy_coding_mode_flag, 7usize, 7usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn pic_scaling_matrix_present_flag(mut self, pic_scaling_matrix_present_flag: u32) -> Self {
        self.0.transform_8x8_mode_flag_and_more_bitfield = crate::bits_copy!(self.0.transform_8x8_mode_flag_and_more_bitfield, pic_scaling_matrix_present_flag, 8usize, 8usize);
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> StdVideoH264PpsFlags {
        self.0
    }
}
impl<'a> std::default::Default for StdVideoH264PpsFlagsBuilder<'a> {
    fn default() -> StdVideoH264PpsFlagsBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for StdVideoH264PpsFlagsBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for StdVideoH264PpsFlagsBuilder<'a> {
    type Target = StdVideoH264PpsFlags;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StdVideoH264PpsFlagsBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Structure"]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct StdVideoDecodeH264PictureInfo {
    pub seq_parameter_set_id: u8,
    pub pic_parameter_set_id: u8,
    pub reserved: u16,
    pub frame_num: u16,
    pub idr_pic_id: u16,
    pub pic_order_cnt: [i32; 2],
    pub flags: crate::external::vk_video::StdVideoDecodeH264PictureInfoFlags,
}
impl Default for StdVideoDecodeH264PictureInfo {
    fn default() -> Self {
        Self { seq_parameter_set_id: Default::default(), pic_parameter_set_id: Default::default(), reserved: Default::default(), frame_num: Default::default(), idr_pic_id: Default::default(), pic_order_cnt: unsafe { std::mem::zeroed() }, flags: Default::default() }
    }
}
impl std::fmt::Debug for StdVideoDecodeH264PictureInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StdVideoDecodeH264PictureInfo").field("seq_parameter_set_id", &self.seq_parameter_set_id).field("pic_parameter_set_id", &self.pic_parameter_set_id).field("reserved", &self.reserved).field("frame_num", &self.frame_num).field("idr_pic_id", &self.idr_pic_id).field("pic_order_cnt", &self.pic_order_cnt).field("flags", &self.flags).finish()
    }
}
impl StdVideoDecodeH264PictureInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> StdVideoDecodeH264PictureInfoBuilder<'a> {
        StdVideoDecodeH264PictureInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "<s>Vulkan Manual Page</s> · Builder of [`StdVideoDecodeH264PictureInfo`]"]
#[repr(transparent)]
pub struct StdVideoDecodeH264PictureInfoBuilder<'a>(StdVideoDecodeH264PictureInfo, std::marker::PhantomData<&'a ()>);
impl<'a> StdVideoDecodeH264PictureInfoBuilder<'a> {
    #[inline]
    pub fn new() -> StdVideoDecodeH264PictureInfoBuilder<'a> {
        StdVideoDecodeH264PictureInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn seq_parameter_set_id(mut self, seq_parameter_set_id: u8) -> Self {
        self.0.seq_parameter_set_id = seq_parameter_set_id as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn pic_parameter_set_id(mut self, pic_parameter_set_id: u8) -> Self {
        self.0.pic_parameter_set_id = pic_parameter_set_id as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn reserved(mut self, reserved: u16) -> Self {
        self.0.reserved = reserved as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn frame_num(mut self, frame_num: u16) -> Self {
        self.0.frame_num = frame_num as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn idr_pic_id(mut self, idr_pic_id: u16) -> Self {
        self.0.idr_pic_id = idr_pic_id as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn pic_order_cnt(mut self, pic_order_cnt: [i32; 2]) -> Self {
        self.0.pic_order_cnt = pic_order_cnt as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn flags(mut self, flags: crate::external::vk_video::StdVideoDecodeH264PictureInfoFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> StdVideoDecodeH264PictureInfo {
        self.0
    }
}
impl<'a> std::default::Default for StdVideoDecodeH264PictureInfoBuilder<'a> {
    fn default() -> StdVideoDecodeH264PictureInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for StdVideoDecodeH264PictureInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for StdVideoDecodeH264PictureInfoBuilder<'a> {
    type Target = StdVideoDecodeH264PictureInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StdVideoDecodeH264PictureInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Structure"]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct StdVideoDecodeH264ReferenceInfo {
    pub frame_num: u16,
    pub reserved: u16,
    pub pic_order_cnt: [i32; 2],
    pub flags: crate::external::vk_video::StdVideoDecodeH264ReferenceInfoFlags,
}
impl Default for StdVideoDecodeH264ReferenceInfo {
    fn default() -> Self {
        Self { frame_num: Default::default(), reserved: Default::default(), pic_order_cnt: unsafe { std::mem::zeroed() }, flags: Default::default() }
    }
}
impl std::fmt::Debug for StdVideoDecodeH264ReferenceInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StdVideoDecodeH264ReferenceInfo").field("frame_num", &self.frame_num).field("reserved", &self.reserved).field("pic_order_cnt", &self.pic_order_cnt).field("flags", &self.flags).finish()
    }
}
impl StdVideoDecodeH264ReferenceInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> StdVideoDecodeH264ReferenceInfoBuilder<'a> {
        StdVideoDecodeH264ReferenceInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "<s>Vulkan Manual Page</s> · Builder of [`StdVideoDecodeH264ReferenceInfo`]"]
#[repr(transparent)]
pub struct StdVideoDecodeH264ReferenceInfoBuilder<'a>(StdVideoDecodeH264ReferenceInfo, std::marker::PhantomData<&'a ()>);
impl<'a> StdVideoDecodeH264ReferenceInfoBuilder<'a> {
    #[inline]
    pub fn new() -> StdVideoDecodeH264ReferenceInfoBuilder<'a> {
        StdVideoDecodeH264ReferenceInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn frame_num(mut self, frame_num: u16) -> Self {
        self.0.frame_num = frame_num as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn reserved(mut self, reserved: u16) -> Self {
        self.0.reserved = reserved as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn pic_order_cnt(mut self, pic_order_cnt: [i32; 2]) -> Self {
        self.0.pic_order_cnt = pic_order_cnt as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn flags(mut self, flags: crate::external::vk_video::StdVideoDecodeH264ReferenceInfoFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> StdVideoDecodeH264ReferenceInfo {
        self.0
    }
}
impl<'a> std::default::Default for StdVideoDecodeH264ReferenceInfoBuilder<'a> {
    fn default() -> StdVideoDecodeH264ReferenceInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for StdVideoDecodeH264ReferenceInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for StdVideoDecodeH264ReferenceInfoBuilder<'a> {
    type Target = StdVideoDecodeH264ReferenceInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StdVideoDecodeH264ReferenceInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StdVideoDecodeH264Mvc {
    pub view_id0: u32,
    pub mvc_element_count: u32,
    pub p_mvc_elements: *mut crate::external::vk_video::StdVideoDecodeH264MvcElement,
}
impl Default for StdVideoDecodeH264Mvc {
    fn default() -> Self {
        Self { view_id0: Default::default(), mvc_element_count: Default::default(), p_mvc_elements: std::ptr::null_mut() }
    }
}
impl std::fmt::Debug for StdVideoDecodeH264Mvc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StdVideoDecodeH264Mvc").field("view_id0", &self.view_id0).field("mvc_element_count", &self.mvc_element_count).field("p_mvc_elements", &self.p_mvc_elements).finish()
    }
}
impl StdVideoDecodeH264Mvc {
    #[inline]
    pub fn into_builder<'a>(self) -> StdVideoDecodeH264MvcBuilder<'a> {
        StdVideoDecodeH264MvcBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "<s>Vulkan Manual Page</s> · Builder of [`StdVideoDecodeH264Mvc`]"]
#[repr(transparent)]
pub struct StdVideoDecodeH264MvcBuilder<'a>(StdVideoDecodeH264Mvc, std::marker::PhantomData<&'a ()>);
impl<'a> StdVideoDecodeH264MvcBuilder<'a> {
    #[inline]
    pub fn new() -> StdVideoDecodeH264MvcBuilder<'a> {
        StdVideoDecodeH264MvcBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn view_id0(mut self, view_id0: u32) -> Self {
        self.0.view_id0 = view_id0 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn mvc_element_count(mut self, mvc_element_count: u32) -> Self {
        self.0.mvc_element_count = mvc_element_count as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn mvc_elements(mut self, mvc_elements: &'a mut crate::external::vk_video::StdVideoDecodeH264MvcElement) -> Self {
        self.0.p_mvc_elements = mvc_elements as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> StdVideoDecodeH264Mvc {
        self.0
    }
}
impl<'a> std::default::Default for StdVideoDecodeH264MvcBuilder<'a> {
    fn default() -> StdVideoDecodeH264MvcBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for StdVideoDecodeH264MvcBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for StdVideoDecodeH264MvcBuilder<'a> {
    type Target = StdVideoDecodeH264Mvc;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StdVideoDecodeH264MvcBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Structure"]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct StdVideoDecodeH264PictureInfoFlags {
    pub field_pic_flag_and_more_bitfield: u32,
}
impl Default for StdVideoDecodeH264PictureInfoFlags {
    fn default() -> Self {
        Self { field_pic_flag_and_more_bitfield: Default::default() }
    }
}
impl std::fmt::Debug for StdVideoDecodeH264PictureInfoFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StdVideoDecodeH264PictureInfoFlags").field("field_pic_flag_and_more_bitfield", &format!("{:#b}", &self.field_pic_flag_and_more_bitfield)).finish()
    }
}
impl StdVideoDecodeH264PictureInfoFlags {
    #[inline]
    pub fn into_builder<'a>(self) -> StdVideoDecodeH264PictureInfoFlagsBuilder<'a> {
        StdVideoDecodeH264PictureInfoFlagsBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "<s>Vulkan Manual Page</s> · Builder of [`StdVideoDecodeH264PictureInfoFlags`]"]
#[repr(transparent)]
pub struct StdVideoDecodeH264PictureInfoFlagsBuilder<'a>(StdVideoDecodeH264PictureInfoFlags, std::marker::PhantomData<&'a ()>);
impl<'a> StdVideoDecodeH264PictureInfoFlagsBuilder<'a> {
    #[inline]
    pub fn new() -> StdVideoDecodeH264PictureInfoFlagsBuilder<'a> {
        StdVideoDecodeH264PictureInfoFlagsBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn field_pic_flag(mut self, field_pic_flag: u32) -> Self {
        self.0.field_pic_flag_and_more_bitfield = crate::bits_copy!(self.0.field_pic_flag_and_more_bitfield, field_pic_flag, 0usize, 0usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn is_intra(mut self, is_intra: u32) -> Self {
        self.0.field_pic_flag_and_more_bitfield = crate::bits_copy!(self.0.field_pic_flag_and_more_bitfield, is_intra, 1usize, 1usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn idr_pic_flag(mut self, idr_pic_flag: u32) -> Self {
        self.0.field_pic_flag_and_more_bitfield = crate::bits_copy!(self.0.field_pic_flag_and_more_bitfield, idr_pic_flag, 2usize, 2usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn bottom_field_flag(mut self, bottom_field_flag: u32) -> Self {
        self.0.field_pic_flag_and_more_bitfield = crate::bits_copy!(self.0.field_pic_flag_and_more_bitfield, bottom_field_flag, 3usize, 3usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn is_reference(mut self, is_reference: u32) -> Self {
        self.0.field_pic_flag_and_more_bitfield = crate::bits_copy!(self.0.field_pic_flag_and_more_bitfield, is_reference, 4usize, 4usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn complementary_field_pair(mut self, complementary_field_pair: u32) -> Self {
        self.0.field_pic_flag_and_more_bitfield = crate::bits_copy!(self.0.field_pic_flag_and_more_bitfield, complementary_field_pair, 5usize, 5usize);
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> StdVideoDecodeH264PictureInfoFlags {
        self.0
    }
}
impl<'a> std::default::Default for StdVideoDecodeH264PictureInfoFlagsBuilder<'a> {
    fn default() -> StdVideoDecodeH264PictureInfoFlagsBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for StdVideoDecodeH264PictureInfoFlagsBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for StdVideoDecodeH264PictureInfoFlagsBuilder<'a> {
    type Target = StdVideoDecodeH264PictureInfoFlags;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StdVideoDecodeH264PictureInfoFlagsBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Structure"]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct StdVideoDecodeH264ReferenceInfoFlags {
    pub top_field_flag_and_more_bitfield: u32,
}
impl Default for StdVideoDecodeH264ReferenceInfoFlags {
    fn default() -> Self {
        Self { top_field_flag_and_more_bitfield: Default::default() }
    }
}
impl std::fmt::Debug for StdVideoDecodeH264ReferenceInfoFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StdVideoDecodeH264ReferenceInfoFlags").field("top_field_flag_and_more_bitfield", &format!("{:#b}", &self.top_field_flag_and_more_bitfield)).finish()
    }
}
impl StdVideoDecodeH264ReferenceInfoFlags {
    #[inline]
    pub fn into_builder<'a>(self) -> StdVideoDecodeH264ReferenceInfoFlagsBuilder<'a> {
        StdVideoDecodeH264ReferenceInfoFlagsBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "<s>Vulkan Manual Page</s> · Builder of [`StdVideoDecodeH264ReferenceInfoFlags`]"]
#[repr(transparent)]
pub struct StdVideoDecodeH264ReferenceInfoFlagsBuilder<'a>(StdVideoDecodeH264ReferenceInfoFlags, std::marker::PhantomData<&'a ()>);
impl<'a> StdVideoDecodeH264ReferenceInfoFlagsBuilder<'a> {
    #[inline]
    pub fn new() -> StdVideoDecodeH264ReferenceInfoFlagsBuilder<'a> {
        StdVideoDecodeH264ReferenceInfoFlagsBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn top_field_flag(mut self, top_field_flag: u32) -> Self {
        self.0.top_field_flag_and_more_bitfield = crate::bits_copy!(self.0.top_field_flag_and_more_bitfield, top_field_flag, 0usize, 0usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn bottom_field_flag(mut self, bottom_field_flag: u32) -> Self {
        self.0.top_field_flag_and_more_bitfield = crate::bits_copy!(self.0.top_field_flag_and_more_bitfield, bottom_field_flag, 1usize, 1usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn is_long_term(mut self, is_long_term: u32) -> Self {
        self.0.top_field_flag_and_more_bitfield = crate::bits_copy!(self.0.top_field_flag_and_more_bitfield, is_long_term, 2usize, 2usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn is_non_existing(mut self, is_non_existing: u32) -> Self {
        self.0.top_field_flag_and_more_bitfield = crate::bits_copy!(self.0.top_field_flag_and_more_bitfield, is_non_existing, 3usize, 3usize);
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> StdVideoDecodeH264ReferenceInfoFlags {
        self.0
    }
}
impl<'a> std::default::Default for StdVideoDecodeH264ReferenceInfoFlagsBuilder<'a> {
    fn default() -> StdVideoDecodeH264ReferenceInfoFlagsBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for StdVideoDecodeH264ReferenceInfoFlagsBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for StdVideoDecodeH264ReferenceInfoFlagsBuilder<'a> {
    type Target = StdVideoDecodeH264ReferenceInfoFlags;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StdVideoDecodeH264ReferenceInfoFlagsBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Structure"]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct StdVideoDecodeH264MvcElement {
    pub flags: crate::external::vk_video::StdVideoDecodeH264MvcElementFlags,
    pub view_order_index: u16,
    pub view_id: u16,
    pub temporal_id: u16,
    pub priority_id: u16,
    pub num_of_anchor_refs_in_l0: u16,
    pub view_id_of_anchor_refs_in_l0: [u16; 15],
    pub num_of_anchor_refs_in_l1: u16,
    pub view_id_of_anchor_refs_in_l1: [u16; 15],
    pub num_of_non_anchor_refs_in_l0: u16,
    pub view_id_of_non_anchor_refs_in_l0: [u16; 15],
    pub num_of_non_anchor_refs_in_l1: u16,
    pub view_id_of_non_anchor_refs_in_l1: [u16; 15],
}
impl Default for StdVideoDecodeH264MvcElement {
    fn default() -> Self {
        Self { flags: Default::default(), view_order_index: Default::default(), view_id: Default::default(), temporal_id: Default::default(), priority_id: Default::default(), num_of_anchor_refs_in_l0: Default::default(), view_id_of_anchor_refs_in_l0: unsafe { std::mem::zeroed() }, num_of_anchor_refs_in_l1: Default::default(), view_id_of_anchor_refs_in_l1: unsafe { std::mem::zeroed() }, num_of_non_anchor_refs_in_l0: Default::default(), view_id_of_non_anchor_refs_in_l0: unsafe { std::mem::zeroed() }, num_of_non_anchor_refs_in_l1: Default::default(), view_id_of_non_anchor_refs_in_l1: unsafe { std::mem::zeroed() } }
    }
}
impl std::fmt::Debug for StdVideoDecodeH264MvcElement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StdVideoDecodeH264MvcElement").field("flags", &self.flags).field("view_order_index", &self.view_order_index).field("view_id", &self.view_id).field("temporal_id", &self.temporal_id).field("priority_id", &self.priority_id).field("num_of_anchor_refs_in_l0", &self.num_of_anchor_refs_in_l0).field("view_id_of_anchor_refs_in_l0", &self.view_id_of_anchor_refs_in_l0).field("num_of_anchor_refs_in_l1", &self.num_of_anchor_refs_in_l1).field("view_id_of_anchor_refs_in_l1", &self.view_id_of_anchor_refs_in_l1).field("num_of_non_anchor_refs_in_l0", &self.num_of_non_anchor_refs_in_l0).field("view_id_of_non_anchor_refs_in_l0", &self.view_id_of_non_anchor_refs_in_l0).field("num_of_non_anchor_refs_in_l1", &self.num_of_non_anchor_refs_in_l1).field("view_id_of_non_anchor_refs_in_l1", &self.view_id_of_non_anchor_refs_in_l1).finish()
    }
}
impl StdVideoDecodeH264MvcElement {
    #[inline]
    pub fn into_builder<'a>(self) -> StdVideoDecodeH264MvcElementBuilder<'a> {
        StdVideoDecodeH264MvcElementBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "<s>Vulkan Manual Page</s> · Builder of [`StdVideoDecodeH264MvcElement`]"]
#[repr(transparent)]
pub struct StdVideoDecodeH264MvcElementBuilder<'a>(StdVideoDecodeH264MvcElement, std::marker::PhantomData<&'a ()>);
impl<'a> StdVideoDecodeH264MvcElementBuilder<'a> {
    #[inline]
    pub fn new() -> StdVideoDecodeH264MvcElementBuilder<'a> {
        StdVideoDecodeH264MvcElementBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn flags(mut self, flags: crate::external::vk_video::StdVideoDecodeH264MvcElementFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn view_order_index(mut self, view_order_index: u16) -> Self {
        self.0.view_order_index = view_order_index as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn view_id(mut self, view_id: u16) -> Self {
        self.0.view_id = view_id as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn temporal_id(mut self, temporal_id: u16) -> Self {
        self.0.temporal_id = temporal_id as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn priority_id(mut self, priority_id: u16) -> Self {
        self.0.priority_id = priority_id as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn num_of_anchor_refs_in_l0(mut self, num_of_anchor_refs_in_l0: u16) -> Self {
        self.0.num_of_anchor_refs_in_l0 = num_of_anchor_refs_in_l0 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn view_id_of_anchor_refs_in_l0(mut self, view_id_of_anchor_refs_in_l0: [u16; 15]) -> Self {
        self.0.view_id_of_anchor_refs_in_l0 = view_id_of_anchor_refs_in_l0 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn num_of_anchor_refs_in_l1(mut self, num_of_anchor_refs_in_l1: u16) -> Self {
        self.0.num_of_anchor_refs_in_l1 = num_of_anchor_refs_in_l1 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn view_id_of_anchor_refs_in_l1(mut self, view_id_of_anchor_refs_in_l1: [u16; 15]) -> Self {
        self.0.view_id_of_anchor_refs_in_l1 = view_id_of_anchor_refs_in_l1 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn num_of_non_anchor_refs_in_l0(mut self, num_of_non_anchor_refs_in_l0: u16) -> Self {
        self.0.num_of_non_anchor_refs_in_l0 = num_of_non_anchor_refs_in_l0 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn view_id_of_non_anchor_refs_in_l0(mut self, view_id_of_non_anchor_refs_in_l0: [u16; 15]) -> Self {
        self.0.view_id_of_non_anchor_refs_in_l0 = view_id_of_non_anchor_refs_in_l0 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn num_of_non_anchor_refs_in_l1(mut self, num_of_non_anchor_refs_in_l1: u16) -> Self {
        self.0.num_of_non_anchor_refs_in_l1 = num_of_non_anchor_refs_in_l1 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn view_id_of_non_anchor_refs_in_l1(mut self, view_id_of_non_anchor_refs_in_l1: [u16; 15]) -> Self {
        self.0.view_id_of_non_anchor_refs_in_l1 = view_id_of_non_anchor_refs_in_l1 as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> StdVideoDecodeH264MvcElement {
        self.0
    }
}
impl<'a> std::default::Default for StdVideoDecodeH264MvcElementBuilder<'a> {
    fn default() -> StdVideoDecodeH264MvcElementBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for StdVideoDecodeH264MvcElementBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for StdVideoDecodeH264MvcElementBuilder<'a> {
    type Target = StdVideoDecodeH264MvcElement;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StdVideoDecodeH264MvcElementBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Structure"]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct StdVideoDecodeH264MvcElementFlags {
    pub non_idr_and_more_bitfield: u32,
}
impl Default for StdVideoDecodeH264MvcElementFlags {
    fn default() -> Self {
        Self { non_idr_and_more_bitfield: Default::default() }
    }
}
impl std::fmt::Debug for StdVideoDecodeH264MvcElementFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StdVideoDecodeH264MvcElementFlags").field("non_idr_and_more_bitfield", &format!("{:#b}", &self.non_idr_and_more_bitfield)).finish()
    }
}
impl StdVideoDecodeH264MvcElementFlags {
    #[inline]
    pub fn into_builder<'a>(self) -> StdVideoDecodeH264MvcElementFlagsBuilder<'a> {
        StdVideoDecodeH264MvcElementFlagsBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "<s>Vulkan Manual Page</s> · Builder of [`StdVideoDecodeH264MvcElementFlags`]"]
#[repr(transparent)]
pub struct StdVideoDecodeH264MvcElementFlagsBuilder<'a>(StdVideoDecodeH264MvcElementFlags, std::marker::PhantomData<&'a ()>);
impl<'a> StdVideoDecodeH264MvcElementFlagsBuilder<'a> {
    #[inline]
    pub fn new() -> StdVideoDecodeH264MvcElementFlagsBuilder<'a> {
        StdVideoDecodeH264MvcElementFlagsBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn non_idr(mut self, non_idr: u32) -> Self {
        self.0.non_idr_and_more_bitfield = crate::bits_copy!(self.0.non_idr_and_more_bitfield, non_idr, 0usize, 0usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn anchor_pic(mut self, anchor_pic: u32) -> Self {
        self.0.non_idr_and_more_bitfield = crate::bits_copy!(self.0.non_idr_and_more_bitfield, anchor_pic, 1usize, 1usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn inter_view(mut self, inter_view: u32) -> Self {
        self.0.non_idr_and_more_bitfield = crate::bits_copy!(self.0.non_idr_and_more_bitfield, inter_view, 2usize, 2usize);
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> StdVideoDecodeH264MvcElementFlags {
        self.0
    }
}
impl<'a> std::default::Default for StdVideoDecodeH264MvcElementFlagsBuilder<'a> {
    fn default() -> StdVideoDecodeH264MvcElementFlagsBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for StdVideoDecodeH264MvcElementFlagsBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for StdVideoDecodeH264MvcElementFlagsBuilder<'a> {
    type Target = StdVideoDecodeH264MvcElementFlags;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StdVideoDecodeH264MvcElementFlagsBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StdVideoH264SequenceParameterSet {
    pub profile_idc: crate::external::vk_video::StdVideoH264ProfileIdc,
    pub level_idc: crate::external::vk_video::StdVideoH264Level,
    pub seq_parameter_set_id: u8,
    pub chroma_format_idc: crate::external::vk_video::StdVideoH264ChromaFormatIdc,
    pub bit_depth_luma_minus8: u8,
    pub bit_depth_chroma_minus8: u8,
    pub log2_max_frame_num_minus4: u8,
    pub pic_order_cnt_type: crate::external::vk_video::StdVideoH264PocType,
    pub log2_max_pic_order_cnt_lsb_minus4: u8,
    pub offset_for_non_ref_pic: i32,
    pub offset_for_top_to_bottom_field: i32,
    pub num_ref_frames_in_pic_order_cnt_cycle: u8,
    pub max_num_ref_frames: u8,
    pub pic_width_in_mbs_minus1: u32,
    pub pic_height_in_map_units_minus1: u32,
    pub frame_crop_left_offset: u32,
    pub frame_crop_right_offset: u32,
    pub frame_crop_top_offset: u32,
    pub frame_crop_bottom_offset: u32,
    pub flags: crate::external::vk_video::StdVideoH264SpsFlags,
    pub p_offset_for_ref_frame: *mut i32,
    pub p_scaling_lists: *mut crate::external::vk_video::StdVideoH264ScalingLists,
    pub p_sequence_parameter_set_vui: *mut crate::external::vk_video::StdVideoH264SequenceParameterSetVui,
}
impl Default for StdVideoH264SequenceParameterSet {
    fn default() -> Self {
        Self { profile_idc: Default::default(), level_idc: Default::default(), seq_parameter_set_id: Default::default(), chroma_format_idc: Default::default(), bit_depth_luma_minus8: Default::default(), bit_depth_chroma_minus8: Default::default(), log2_max_frame_num_minus4: Default::default(), pic_order_cnt_type: Default::default(), log2_max_pic_order_cnt_lsb_minus4: Default::default(), offset_for_non_ref_pic: Default::default(), offset_for_top_to_bottom_field: Default::default(), num_ref_frames_in_pic_order_cnt_cycle: Default::default(), max_num_ref_frames: Default::default(), pic_width_in_mbs_minus1: Default::default(), pic_height_in_map_units_minus1: Default::default(), frame_crop_left_offset: Default::default(), frame_crop_right_offset: Default::default(), frame_crop_top_offset: Default::default(), frame_crop_bottom_offset: Default::default(), flags: Default::default(), p_offset_for_ref_frame: std::ptr::null_mut(), p_scaling_lists: std::ptr::null_mut(), p_sequence_parameter_set_vui: std::ptr::null_mut() }
    }
}
impl std::fmt::Debug for StdVideoH264SequenceParameterSet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StdVideoH264SequenceParameterSet").field("profile_idc", &self.profile_idc).field("level_idc", &self.level_idc).field("seq_parameter_set_id", &self.seq_parameter_set_id).field("chroma_format_idc", &self.chroma_format_idc).field("bit_depth_luma_minus8", &self.bit_depth_luma_minus8).field("bit_depth_chroma_minus8", &self.bit_depth_chroma_minus8).field("log2_max_frame_num_minus4", &self.log2_max_frame_num_minus4).field("pic_order_cnt_type", &self.pic_order_cnt_type).field("log2_max_pic_order_cnt_lsb_minus4", &self.log2_max_pic_order_cnt_lsb_minus4).field("offset_for_non_ref_pic", &self.offset_for_non_ref_pic).field("offset_for_top_to_bottom_field", &self.offset_for_top_to_bottom_field).field("num_ref_frames_in_pic_order_cnt_cycle", &self.num_ref_frames_in_pic_order_cnt_cycle).field("max_num_ref_frames", &self.max_num_ref_frames).field("pic_width_in_mbs_minus1", &self.pic_width_in_mbs_minus1).field("pic_height_in_map_units_minus1", &self.pic_height_in_map_units_minus1).field("frame_crop_left_offset", &self.frame_crop_left_offset).field("frame_crop_right_offset", &self.frame_crop_right_offset).field("frame_crop_top_offset", &self.frame_crop_top_offset).field("frame_crop_bottom_offset", &self.frame_crop_bottom_offset).field("flags", &self.flags).field("p_offset_for_ref_frame", &self.p_offset_for_ref_frame).field("p_scaling_lists", &self.p_scaling_lists).field("p_sequence_parameter_set_vui", &self.p_sequence_parameter_set_vui).finish()
    }
}
impl StdVideoH264SequenceParameterSet {
    #[inline]
    pub fn into_builder<'a>(self) -> StdVideoH264SequenceParameterSetBuilder<'a> {
        StdVideoH264SequenceParameterSetBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "<s>Vulkan Manual Page</s> · Builder of [`StdVideoH264SequenceParameterSet`]"]
#[repr(transparent)]
pub struct StdVideoH264SequenceParameterSetBuilder<'a>(StdVideoH264SequenceParameterSet, std::marker::PhantomData<&'a ()>);
impl<'a> StdVideoH264SequenceParameterSetBuilder<'a> {
    #[inline]
    pub fn new() -> StdVideoH264SequenceParameterSetBuilder<'a> {
        StdVideoH264SequenceParameterSetBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn profile_idc(mut self, profile_idc: crate::external::vk_video::StdVideoH264ProfileIdc) -> Self {
        self.0.profile_idc = profile_idc as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn level_idc(mut self, level_idc: crate::external::vk_video::StdVideoH264Level) -> Self {
        self.0.level_idc = level_idc as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn seq_parameter_set_id(mut self, seq_parameter_set_id: u8) -> Self {
        self.0.seq_parameter_set_id = seq_parameter_set_id as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn chroma_format_idc(mut self, chroma_format_idc: crate::external::vk_video::StdVideoH264ChromaFormatIdc) -> Self {
        self.0.chroma_format_idc = chroma_format_idc as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn bit_depth_luma_minus8(mut self, bit_depth_luma_minus8: u8) -> Self {
        self.0.bit_depth_luma_minus8 = bit_depth_luma_minus8 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn bit_depth_chroma_minus8(mut self, bit_depth_chroma_minus8: u8) -> Self {
        self.0.bit_depth_chroma_minus8 = bit_depth_chroma_minus8 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn log2_max_frame_num_minus4(mut self, log2_max_frame_num_minus4: u8) -> Self {
        self.0.log2_max_frame_num_minus4 = log2_max_frame_num_minus4 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn pic_order_cnt_type(mut self, pic_order_cnt_type: crate::external::vk_video::StdVideoH264PocType) -> Self {
        self.0.pic_order_cnt_type = pic_order_cnt_type as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn log2_max_pic_order_cnt_lsb_minus4(mut self, log2_max_pic_order_cnt_lsb_minus4: u8) -> Self {
        self.0.log2_max_pic_order_cnt_lsb_minus4 = log2_max_pic_order_cnt_lsb_minus4 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn offset_for_non_ref_pic(mut self, offset_for_non_ref_pic: i32) -> Self {
        self.0.offset_for_non_ref_pic = offset_for_non_ref_pic as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn offset_for_top_to_bottom_field(mut self, offset_for_top_to_bottom_field: i32) -> Self {
        self.0.offset_for_top_to_bottom_field = offset_for_top_to_bottom_field as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn num_ref_frames_in_pic_order_cnt_cycle(mut self, num_ref_frames_in_pic_order_cnt_cycle: u8) -> Self {
        self.0.num_ref_frames_in_pic_order_cnt_cycle = num_ref_frames_in_pic_order_cnt_cycle as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn max_num_ref_frames(mut self, max_num_ref_frames: u8) -> Self {
        self.0.max_num_ref_frames = max_num_ref_frames as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn pic_width_in_mbs_minus1(mut self, pic_width_in_mbs_minus1: u32) -> Self {
        self.0.pic_width_in_mbs_minus1 = pic_width_in_mbs_minus1 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn pic_height_in_map_units_minus1(mut self, pic_height_in_map_units_minus1: u32) -> Self {
        self.0.pic_height_in_map_units_minus1 = pic_height_in_map_units_minus1 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn frame_crop_left_offset(mut self, frame_crop_left_offset: u32) -> Self {
        self.0.frame_crop_left_offset = frame_crop_left_offset as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn frame_crop_right_offset(mut self, frame_crop_right_offset: u32) -> Self {
        self.0.frame_crop_right_offset = frame_crop_right_offset as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn frame_crop_top_offset(mut self, frame_crop_top_offset: u32) -> Self {
        self.0.frame_crop_top_offset = frame_crop_top_offset as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn frame_crop_bottom_offset(mut self, frame_crop_bottom_offset: u32) -> Self {
        self.0.frame_crop_bottom_offset = frame_crop_bottom_offset as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn flags(mut self, flags: crate::external::vk_video::StdVideoH264SpsFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn offset_for_ref_frame(mut self, offset_for_ref_frame: &'a mut [i32]) -> Self {
        self.0.p_offset_for_ref_frame = offset_for_ref_frame.as_ptr() as _;
        self.0.num_ref_frames_in_pic_order_cnt_cycle = offset_for_ref_frame.len() as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn scaling_lists(mut self, scaling_lists: &'a mut crate::external::vk_video::StdVideoH264ScalingLists) -> Self {
        self.0.p_scaling_lists = scaling_lists as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn sequence_parameter_set_vui(mut self, sequence_parameter_set_vui: &'a mut crate::external::vk_video::StdVideoH264SequenceParameterSetVui) -> Self {
        self.0.p_sequence_parameter_set_vui = sequence_parameter_set_vui as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> StdVideoH264SequenceParameterSet {
        self.0
    }
}
impl<'a> std::default::Default for StdVideoH264SequenceParameterSetBuilder<'a> {
    fn default() -> StdVideoH264SequenceParameterSetBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for StdVideoH264SequenceParameterSetBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for StdVideoH264SequenceParameterSetBuilder<'a> {
    type Target = StdVideoH264SequenceParameterSet;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StdVideoH264SequenceParameterSetBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StdVideoH264PictureParameterSet {
    pub seq_parameter_set_id: u8,
    pub pic_parameter_set_id: u8,
    pub num_ref_idx_l0_default_active_minus1: u8,
    pub num_ref_idx_l1_default_active_minus1: u8,
    pub weighted_bipred_idc: crate::external::vk_video::StdVideoH264WeightedBipredIdc,
    pub pic_init_qp_minus26: i8,
    pub pic_init_qs_minus26: i8,
    pub chroma_qp_index_offset: i8,
    pub second_chroma_qp_index_offset: i8,
    pub flags: crate::external::vk_video::StdVideoH264PpsFlags,
    pub p_scaling_lists: *mut crate::external::vk_video::StdVideoH264ScalingLists,
}
impl Default for StdVideoH264PictureParameterSet {
    fn default() -> Self {
        Self { seq_parameter_set_id: Default::default(), pic_parameter_set_id: Default::default(), num_ref_idx_l0_default_active_minus1: Default::default(), num_ref_idx_l1_default_active_minus1: Default::default(), weighted_bipred_idc: Default::default(), pic_init_qp_minus26: Default::default(), pic_init_qs_minus26: Default::default(), chroma_qp_index_offset: Default::default(), second_chroma_qp_index_offset: Default::default(), flags: Default::default(), p_scaling_lists: std::ptr::null_mut() }
    }
}
impl std::fmt::Debug for StdVideoH264PictureParameterSet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StdVideoH264PictureParameterSet").field("seq_parameter_set_id", &self.seq_parameter_set_id).field("pic_parameter_set_id", &self.pic_parameter_set_id).field("num_ref_idx_l0_default_active_minus1", &self.num_ref_idx_l0_default_active_minus1).field("num_ref_idx_l1_default_active_minus1", &self.num_ref_idx_l1_default_active_minus1).field("weighted_bipred_idc", &self.weighted_bipred_idc).field("pic_init_qp_minus26", &self.pic_init_qp_minus26).field("pic_init_qs_minus26", &self.pic_init_qs_minus26).field("chroma_qp_index_offset", &self.chroma_qp_index_offset).field("second_chroma_qp_index_offset", &self.second_chroma_qp_index_offset).field("flags", &self.flags).field("p_scaling_lists", &self.p_scaling_lists).finish()
    }
}
impl StdVideoH264PictureParameterSet {
    #[inline]
    pub fn into_builder<'a>(self) -> StdVideoH264PictureParameterSetBuilder<'a> {
        StdVideoH264PictureParameterSetBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "<s>Vulkan Manual Page</s> · Builder of [`StdVideoH264PictureParameterSet`]"]
#[repr(transparent)]
pub struct StdVideoH264PictureParameterSetBuilder<'a>(StdVideoH264PictureParameterSet, std::marker::PhantomData<&'a ()>);
impl<'a> StdVideoH264PictureParameterSetBuilder<'a> {
    #[inline]
    pub fn new() -> StdVideoH264PictureParameterSetBuilder<'a> {
        StdVideoH264PictureParameterSetBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn seq_parameter_set_id(mut self, seq_parameter_set_id: u8) -> Self {
        self.0.seq_parameter_set_id = seq_parameter_set_id as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn pic_parameter_set_id(mut self, pic_parameter_set_id: u8) -> Self {
        self.0.pic_parameter_set_id = pic_parameter_set_id as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn num_ref_idx_l0_default_active_minus1(mut self, num_ref_idx_l0_default_active_minus1: u8) -> Self {
        self.0.num_ref_idx_l0_default_active_minus1 = num_ref_idx_l0_default_active_minus1 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn num_ref_idx_l1_default_active_minus1(mut self, num_ref_idx_l1_default_active_minus1: u8) -> Self {
        self.0.num_ref_idx_l1_default_active_minus1 = num_ref_idx_l1_default_active_minus1 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn weighted_bipred_idc(mut self, weighted_bipred_idc: crate::external::vk_video::StdVideoH264WeightedBipredIdc) -> Self {
        self.0.weighted_bipred_idc = weighted_bipred_idc as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn pic_init_qp_minus26(mut self, pic_init_qp_minus26: i8) -> Self {
        self.0.pic_init_qp_minus26 = pic_init_qp_minus26 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn pic_init_qs_minus26(mut self, pic_init_qs_minus26: i8) -> Self {
        self.0.pic_init_qs_minus26 = pic_init_qs_minus26 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn chroma_qp_index_offset(mut self, chroma_qp_index_offset: i8) -> Self {
        self.0.chroma_qp_index_offset = chroma_qp_index_offset as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn second_chroma_qp_index_offset(mut self, second_chroma_qp_index_offset: i8) -> Self {
        self.0.second_chroma_qp_index_offset = second_chroma_qp_index_offset as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn flags(mut self, flags: crate::external::vk_video::StdVideoH264PpsFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn scaling_lists(mut self, scaling_lists: &'a mut crate::external::vk_video::StdVideoH264ScalingLists) -> Self {
        self.0.p_scaling_lists = scaling_lists as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> StdVideoH264PictureParameterSet {
        self.0
    }
}
impl<'a> std::default::Default for StdVideoH264PictureParameterSetBuilder<'a> {
    fn default() -> StdVideoH264PictureParameterSetBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for StdVideoH264PictureParameterSetBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for StdVideoH264PictureParameterSetBuilder<'a> {
    type Target = StdVideoH264PictureParameterSet;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StdVideoH264PictureParameterSetBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StdVideoH265VideoParameterSet {
    pub vps_video_parameter_set_id: u8,
    pub vps_max_sub_layers_minus1: u8,
    pub vps_num_units_in_tick: u32,
    pub vps_time_scale: u32,
    pub vps_num_ticks_poc_diff_one_minus1: u32,
    pub p_dec_pic_buf_mgr: *mut crate::external::vk_video::StdVideoH265DecPicBufMgr,
    pub p_hrd_parameters: *mut crate::external::vk_video::StdVideoH265HrdParameters,
    pub flags: crate::external::vk_video::StdVideoH265VpsFlags,
}
impl Default for StdVideoH265VideoParameterSet {
    fn default() -> Self {
        Self { vps_video_parameter_set_id: Default::default(), vps_max_sub_layers_minus1: Default::default(), vps_num_units_in_tick: Default::default(), vps_time_scale: Default::default(), vps_num_ticks_poc_diff_one_minus1: Default::default(), p_dec_pic_buf_mgr: std::ptr::null_mut(), p_hrd_parameters: std::ptr::null_mut(), flags: Default::default() }
    }
}
impl std::fmt::Debug for StdVideoH265VideoParameterSet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StdVideoH265VideoParameterSet").field("vps_video_parameter_set_id", &self.vps_video_parameter_set_id).field("vps_max_sub_layers_minus1", &self.vps_max_sub_layers_minus1).field("vps_num_units_in_tick", &self.vps_num_units_in_tick).field("vps_time_scale", &self.vps_time_scale).field("vps_num_ticks_poc_diff_one_minus1", &self.vps_num_ticks_poc_diff_one_minus1).field("p_dec_pic_buf_mgr", &self.p_dec_pic_buf_mgr).field("p_hrd_parameters", &self.p_hrd_parameters).field("flags", &self.flags).finish()
    }
}
impl StdVideoH265VideoParameterSet {
    #[inline]
    pub fn into_builder<'a>(self) -> StdVideoH265VideoParameterSetBuilder<'a> {
        StdVideoH265VideoParameterSetBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "<s>Vulkan Manual Page</s> · Builder of [`StdVideoH265VideoParameterSet`]"]
#[repr(transparent)]
pub struct StdVideoH265VideoParameterSetBuilder<'a>(StdVideoH265VideoParameterSet, std::marker::PhantomData<&'a ()>);
impl<'a> StdVideoH265VideoParameterSetBuilder<'a> {
    #[inline]
    pub fn new() -> StdVideoH265VideoParameterSetBuilder<'a> {
        StdVideoH265VideoParameterSetBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn vps_video_parameter_set_id(mut self, vps_video_parameter_set_id: u8) -> Self {
        self.0.vps_video_parameter_set_id = vps_video_parameter_set_id as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn vps_max_sub_layers_minus1(mut self, vps_max_sub_layers_minus1: u8) -> Self {
        self.0.vps_max_sub_layers_minus1 = vps_max_sub_layers_minus1 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn vps_num_units_in_tick(mut self, vps_num_units_in_tick: u32) -> Self {
        self.0.vps_num_units_in_tick = vps_num_units_in_tick as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn vps_time_scale(mut self, vps_time_scale: u32) -> Self {
        self.0.vps_time_scale = vps_time_scale as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn vps_num_ticks_poc_diff_one_minus1(mut self, vps_num_ticks_poc_diff_one_minus1: u32) -> Self {
        self.0.vps_num_ticks_poc_diff_one_minus1 = vps_num_ticks_poc_diff_one_minus1 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn dec_pic_buf_mgr(mut self, dec_pic_buf_mgr: &'a mut crate::external::vk_video::StdVideoH265DecPicBufMgr) -> Self {
        self.0.p_dec_pic_buf_mgr = dec_pic_buf_mgr as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn hrd_parameters(mut self, hrd_parameters: &'a mut crate::external::vk_video::StdVideoH265HrdParameters) -> Self {
        self.0.p_hrd_parameters = hrd_parameters as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn flags(mut self, flags: crate::external::vk_video::StdVideoH265VpsFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> StdVideoH265VideoParameterSet {
        self.0
    }
}
impl<'a> std::default::Default for StdVideoH265VideoParameterSetBuilder<'a> {
    fn default() -> StdVideoH265VideoParameterSetBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for StdVideoH265VideoParameterSetBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for StdVideoH265VideoParameterSetBuilder<'a> {
    type Target = StdVideoH265VideoParameterSet;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StdVideoH265VideoParameterSetBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StdVideoH265SequenceParameterSet {
    pub profile_idc: crate::external::vk_video::StdVideoH265ProfileIdc,
    pub level_idc: crate::external::vk_video::StdVideoH265Level,
    pub pic_width_in_luma_samples: u32,
    pub pic_height_in_luma_samples: u32,
    pub sps_video_parameter_set_id: u8,
    pub sps_max_sub_layers_minus1: u8,
    pub sps_seq_parameter_set_id: u8,
    pub chroma_format_idc: u8,
    pub bit_depth_luma_minus8: u8,
    pub bit_depth_chroma_minus8: u8,
    pub log2_max_pic_order_cnt_lsb_minus4: u8,
    pub sps_max_dec_pic_buffering_minus1: u8,
    pub log2_min_luma_coding_block_size_minus3: u8,
    pub log2_diff_max_min_luma_coding_block_size: u8,
    pub log2_min_luma_transform_block_size_minus2: u8,
    pub log2_diff_max_min_luma_transform_block_size: u8,
    pub max_transform_hierarchy_depth_inter: u8,
    pub max_transform_hierarchy_depth_intra: u8,
    pub num_short_term_ref_pic_sets: u8,
    pub num_long_term_ref_pics_sps: u8,
    pub pcm_sample_bit_depth_luma_minus1: u8,
    pub pcm_sample_bit_depth_chroma_minus1: u8,
    pub log2_min_pcm_luma_coding_block_size_minus3: u8,
    pub log2_diff_max_min_pcm_luma_coding_block_size: u8,
    pub conf_win_left_offset: u32,
    pub conf_win_right_offset: u32,
    pub conf_win_top_offset: u32,
    pub conf_win_bottom_offset: u32,
    pub p_dec_pic_buf_mgr: *mut crate::external::vk_video::StdVideoH265DecPicBufMgr,
    pub flags: crate::external::vk_video::StdVideoH265SpsFlags,
    pub p_scaling_lists: *mut crate::external::vk_video::StdVideoH265ScalingLists,
    pub p_sequence_parameter_set_vui: *mut crate::external::vk_video::StdVideoH265SequenceParameterSetVui,
    pub palette_max_size: u8,
    pub delta_palette_max_predictor_size: u8,
    pub motion_vector_resolution_control_idc: u8,
    pub sps_num_palette_predictor_initializer_minus1: u8,
    pub p_predictor_palette_entries: *mut crate::external::vk_video::StdVideoH265PredictorPaletteEntries,
}
impl Default for StdVideoH265SequenceParameterSet {
    fn default() -> Self {
        Self {
            profile_idc: Default::default(),
            level_idc: Default::default(),
            pic_width_in_luma_samples: Default::default(),
            pic_height_in_luma_samples: Default::default(),
            sps_video_parameter_set_id: Default::default(),
            sps_max_sub_layers_minus1: Default::default(),
            sps_seq_parameter_set_id: Default::default(),
            chroma_format_idc: Default::default(),
            bit_depth_luma_minus8: Default::default(),
            bit_depth_chroma_minus8: Default::default(),
            log2_max_pic_order_cnt_lsb_minus4: Default::default(),
            sps_max_dec_pic_buffering_minus1: Default::default(),
            log2_min_luma_coding_block_size_minus3: Default::default(),
            log2_diff_max_min_luma_coding_block_size: Default::default(),
            log2_min_luma_transform_block_size_minus2: Default::default(),
            log2_diff_max_min_luma_transform_block_size: Default::default(),
            max_transform_hierarchy_depth_inter: Default::default(),
            max_transform_hierarchy_depth_intra: Default::default(),
            num_short_term_ref_pic_sets: Default::default(),
            num_long_term_ref_pics_sps: Default::default(),
            pcm_sample_bit_depth_luma_minus1: Default::default(),
            pcm_sample_bit_depth_chroma_minus1: Default::default(),
            log2_min_pcm_luma_coding_block_size_minus3: Default::default(),
            log2_diff_max_min_pcm_luma_coding_block_size: Default::default(),
            conf_win_left_offset: Default::default(),
            conf_win_right_offset: Default::default(),
            conf_win_top_offset: Default::default(),
            conf_win_bottom_offset: Default::default(),
            p_dec_pic_buf_mgr: std::ptr::null_mut(),
            flags: Default::default(),
            p_scaling_lists: std::ptr::null_mut(),
            p_sequence_parameter_set_vui: std::ptr::null_mut(),
            palette_max_size: Default::default(),
            delta_palette_max_predictor_size: Default::default(),
            motion_vector_resolution_control_idc: Default::default(),
            sps_num_palette_predictor_initializer_minus1: Default::default(),
            p_predictor_palette_entries: std::ptr::null_mut(),
        }
    }
}
impl std::fmt::Debug for StdVideoH265SequenceParameterSet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StdVideoH265SequenceParameterSet").field("profile_idc", &self.profile_idc).field("level_idc", &self.level_idc).field("pic_width_in_luma_samples", &self.pic_width_in_luma_samples).field("pic_height_in_luma_samples", &self.pic_height_in_luma_samples).field("sps_video_parameter_set_id", &self.sps_video_parameter_set_id).field("sps_max_sub_layers_minus1", &self.sps_max_sub_layers_minus1).field("sps_seq_parameter_set_id", &self.sps_seq_parameter_set_id).field("chroma_format_idc", &self.chroma_format_idc).field("bit_depth_luma_minus8", &self.bit_depth_luma_minus8).field("bit_depth_chroma_minus8", &self.bit_depth_chroma_minus8).field("log2_max_pic_order_cnt_lsb_minus4", &self.log2_max_pic_order_cnt_lsb_minus4).field("sps_max_dec_pic_buffering_minus1", &self.sps_max_dec_pic_buffering_minus1).field("log2_min_luma_coding_block_size_minus3", &self.log2_min_luma_coding_block_size_minus3).field("log2_diff_max_min_luma_coding_block_size", &self.log2_diff_max_min_luma_coding_block_size).field("log2_min_luma_transform_block_size_minus2", &self.log2_min_luma_transform_block_size_minus2).field("log2_diff_max_min_luma_transform_block_size", &self.log2_diff_max_min_luma_transform_block_size).field("max_transform_hierarchy_depth_inter", &self.max_transform_hierarchy_depth_inter).field("max_transform_hierarchy_depth_intra", &self.max_transform_hierarchy_depth_intra).field("num_short_term_ref_pic_sets", &self.num_short_term_ref_pic_sets).field("num_long_term_ref_pics_sps", &self.num_long_term_ref_pics_sps).field("pcm_sample_bit_depth_luma_minus1", &self.pcm_sample_bit_depth_luma_minus1).field("pcm_sample_bit_depth_chroma_minus1", &self.pcm_sample_bit_depth_chroma_minus1).field("log2_min_pcm_luma_coding_block_size_minus3", &self.log2_min_pcm_luma_coding_block_size_minus3).field("log2_diff_max_min_pcm_luma_coding_block_size", &self.log2_diff_max_min_pcm_luma_coding_block_size).field("conf_win_left_offset", &self.conf_win_left_offset).field("conf_win_right_offset", &self.conf_win_right_offset).field("conf_win_top_offset", &self.conf_win_top_offset).field("conf_win_bottom_offset", &self.conf_win_bottom_offset).field("p_dec_pic_buf_mgr", &self.p_dec_pic_buf_mgr).field("flags", &self.flags).field("p_scaling_lists", &self.p_scaling_lists).field("p_sequence_parameter_set_vui", &self.p_sequence_parameter_set_vui).field("palette_max_size", &self.palette_max_size).field("delta_palette_max_predictor_size", &self.delta_palette_max_predictor_size).field("motion_vector_resolution_control_idc", &self.motion_vector_resolution_control_idc).field("sps_num_palette_predictor_initializer_minus1", &self.sps_num_palette_predictor_initializer_minus1).field("p_predictor_palette_entries", &self.p_predictor_palette_entries).finish()
    }
}
impl StdVideoH265SequenceParameterSet {
    #[inline]
    pub fn into_builder<'a>(self) -> StdVideoH265SequenceParameterSetBuilder<'a> {
        StdVideoH265SequenceParameterSetBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "<s>Vulkan Manual Page</s> · Builder of [`StdVideoH265SequenceParameterSet`]"]
#[repr(transparent)]
pub struct StdVideoH265SequenceParameterSetBuilder<'a>(StdVideoH265SequenceParameterSet, std::marker::PhantomData<&'a ()>);
impl<'a> StdVideoH265SequenceParameterSetBuilder<'a> {
    #[inline]
    pub fn new() -> StdVideoH265SequenceParameterSetBuilder<'a> {
        StdVideoH265SequenceParameterSetBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn profile_idc(mut self, profile_idc: crate::external::vk_video::StdVideoH265ProfileIdc) -> Self {
        self.0.profile_idc = profile_idc as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn level_idc(mut self, level_idc: crate::external::vk_video::StdVideoH265Level) -> Self {
        self.0.level_idc = level_idc as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn pic_width_in_luma_samples(mut self, pic_width_in_luma_samples: u32) -> Self {
        self.0.pic_width_in_luma_samples = pic_width_in_luma_samples as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn pic_height_in_luma_samples(mut self, pic_height_in_luma_samples: u32) -> Self {
        self.0.pic_height_in_luma_samples = pic_height_in_luma_samples as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn sps_video_parameter_set_id(mut self, sps_video_parameter_set_id: u8) -> Self {
        self.0.sps_video_parameter_set_id = sps_video_parameter_set_id as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn sps_max_sub_layers_minus1(mut self, sps_max_sub_layers_minus1: u8) -> Self {
        self.0.sps_max_sub_layers_minus1 = sps_max_sub_layers_minus1 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn sps_seq_parameter_set_id(mut self, sps_seq_parameter_set_id: u8) -> Self {
        self.0.sps_seq_parameter_set_id = sps_seq_parameter_set_id as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn chroma_format_idc(mut self, chroma_format_idc: u8) -> Self {
        self.0.chroma_format_idc = chroma_format_idc as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn bit_depth_luma_minus8(mut self, bit_depth_luma_minus8: u8) -> Self {
        self.0.bit_depth_luma_minus8 = bit_depth_luma_minus8 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn bit_depth_chroma_minus8(mut self, bit_depth_chroma_minus8: u8) -> Self {
        self.0.bit_depth_chroma_minus8 = bit_depth_chroma_minus8 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn log2_max_pic_order_cnt_lsb_minus4(mut self, log2_max_pic_order_cnt_lsb_minus4: u8) -> Self {
        self.0.log2_max_pic_order_cnt_lsb_minus4 = log2_max_pic_order_cnt_lsb_minus4 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn sps_max_dec_pic_buffering_minus1(mut self, sps_max_dec_pic_buffering_minus1: u8) -> Self {
        self.0.sps_max_dec_pic_buffering_minus1 = sps_max_dec_pic_buffering_minus1 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn log2_min_luma_coding_block_size_minus3(mut self, log2_min_luma_coding_block_size_minus3: u8) -> Self {
        self.0.log2_min_luma_coding_block_size_minus3 = log2_min_luma_coding_block_size_minus3 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn log2_diff_max_min_luma_coding_block_size(mut self, log2_diff_max_min_luma_coding_block_size: u8) -> Self {
        self.0.log2_diff_max_min_luma_coding_block_size = log2_diff_max_min_luma_coding_block_size as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn log2_min_luma_transform_block_size_minus2(mut self, log2_min_luma_transform_block_size_minus2: u8) -> Self {
        self.0.log2_min_luma_transform_block_size_minus2 = log2_min_luma_transform_block_size_minus2 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn log2_diff_max_min_luma_transform_block_size(mut self, log2_diff_max_min_luma_transform_block_size: u8) -> Self {
        self.0.log2_diff_max_min_luma_transform_block_size = log2_diff_max_min_luma_transform_block_size as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn max_transform_hierarchy_depth_inter(mut self, max_transform_hierarchy_depth_inter: u8) -> Self {
        self.0.max_transform_hierarchy_depth_inter = max_transform_hierarchy_depth_inter as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn max_transform_hierarchy_depth_intra(mut self, max_transform_hierarchy_depth_intra: u8) -> Self {
        self.0.max_transform_hierarchy_depth_intra = max_transform_hierarchy_depth_intra as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn num_short_term_ref_pic_sets(mut self, num_short_term_ref_pic_sets: u8) -> Self {
        self.0.num_short_term_ref_pic_sets = num_short_term_ref_pic_sets as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn num_long_term_ref_pics_sps(mut self, num_long_term_ref_pics_sps: u8) -> Self {
        self.0.num_long_term_ref_pics_sps = num_long_term_ref_pics_sps as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn pcm_sample_bit_depth_luma_minus1(mut self, pcm_sample_bit_depth_luma_minus1: u8) -> Self {
        self.0.pcm_sample_bit_depth_luma_minus1 = pcm_sample_bit_depth_luma_minus1 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn pcm_sample_bit_depth_chroma_minus1(mut self, pcm_sample_bit_depth_chroma_minus1: u8) -> Self {
        self.0.pcm_sample_bit_depth_chroma_minus1 = pcm_sample_bit_depth_chroma_minus1 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn log2_min_pcm_luma_coding_block_size_minus3(mut self, log2_min_pcm_luma_coding_block_size_minus3: u8) -> Self {
        self.0.log2_min_pcm_luma_coding_block_size_minus3 = log2_min_pcm_luma_coding_block_size_minus3 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn log2_diff_max_min_pcm_luma_coding_block_size(mut self, log2_diff_max_min_pcm_luma_coding_block_size: u8) -> Self {
        self.0.log2_diff_max_min_pcm_luma_coding_block_size = log2_diff_max_min_pcm_luma_coding_block_size as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn conf_win_left_offset(mut self, conf_win_left_offset: u32) -> Self {
        self.0.conf_win_left_offset = conf_win_left_offset as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn conf_win_right_offset(mut self, conf_win_right_offset: u32) -> Self {
        self.0.conf_win_right_offset = conf_win_right_offset as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn conf_win_top_offset(mut self, conf_win_top_offset: u32) -> Self {
        self.0.conf_win_top_offset = conf_win_top_offset as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn conf_win_bottom_offset(mut self, conf_win_bottom_offset: u32) -> Self {
        self.0.conf_win_bottom_offset = conf_win_bottom_offset as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn dec_pic_buf_mgr(mut self, dec_pic_buf_mgr: &'a mut crate::external::vk_video::StdVideoH265DecPicBufMgr) -> Self {
        self.0.p_dec_pic_buf_mgr = dec_pic_buf_mgr as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn flags(mut self, flags: crate::external::vk_video::StdVideoH265SpsFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn scaling_lists(mut self, scaling_lists: &'a mut crate::external::vk_video::StdVideoH265ScalingLists) -> Self {
        self.0.p_scaling_lists = scaling_lists as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn sequence_parameter_set_vui(mut self, sequence_parameter_set_vui: &'a mut crate::external::vk_video::StdVideoH265SequenceParameterSetVui) -> Self {
        self.0.p_sequence_parameter_set_vui = sequence_parameter_set_vui as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn palette_max_size(mut self, palette_max_size: u8) -> Self {
        self.0.palette_max_size = palette_max_size as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn delta_palette_max_predictor_size(mut self, delta_palette_max_predictor_size: u8) -> Self {
        self.0.delta_palette_max_predictor_size = delta_palette_max_predictor_size as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn motion_vector_resolution_control_idc(mut self, motion_vector_resolution_control_idc: u8) -> Self {
        self.0.motion_vector_resolution_control_idc = motion_vector_resolution_control_idc as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn sps_num_palette_predictor_initializer_minus1(mut self, sps_num_palette_predictor_initializer_minus1: u8) -> Self {
        self.0.sps_num_palette_predictor_initializer_minus1 = sps_num_palette_predictor_initializer_minus1 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn predictor_palette_entries(mut self, predictor_palette_entries: &'a mut crate::external::vk_video::StdVideoH265PredictorPaletteEntries) -> Self {
        self.0.p_predictor_palette_entries = predictor_palette_entries as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> StdVideoH265SequenceParameterSet {
        self.0
    }
}
impl<'a> std::default::Default for StdVideoH265SequenceParameterSetBuilder<'a> {
    fn default() -> StdVideoH265SequenceParameterSetBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for StdVideoH265SequenceParameterSetBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for StdVideoH265SequenceParameterSetBuilder<'a> {
    type Target = StdVideoH265SequenceParameterSet;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StdVideoH265SequenceParameterSetBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StdVideoH265PictureParameterSet {
    pub pps_pic_parameter_set_id: u8,
    pub pps_seq_parameter_set_id: u8,
    pub num_extra_slice_header_bits: u8,
    pub num_ref_idx_l0_default_active_minus1: u8,
    pub num_ref_idx_l1_default_active_minus1: u8,
    pub init_qp_minus26: i8,
    pub diff_cu_qp_delta_depth: u8,
    pub pps_cb_qp_offset: i8,
    pub pps_cr_qp_offset: i8,
    pub num_tile_columns_minus1: u8,
    pub num_tile_rows_minus1: u8,
    pub column_width_minus1: [u16; 19],
    pub row_height_minus1: [u16; 21],
    pub pps_beta_offset_div2: i8,
    pub pps_tc_offset_div2: i8,
    pub log2_parallel_merge_level_minus2: u8,
    pub flags: crate::external::vk_video::StdVideoH265PpsFlags,
    pub p_scaling_lists: *mut crate::external::vk_video::StdVideoH265ScalingLists,
    pub log2_max_transform_skip_block_size_minus2: u8,
    pub diff_cu_chroma_qp_offset_depth: u8,
    pub chroma_qp_offset_list_len_minus1: u8,
    pub cb_qp_offset_list: [i8; 6],
    pub cr_qp_offset_list: [i8; 6],
    pub log2_sao_offset_scale_luma: u8,
    pub log2_sao_offset_scale_chroma: u8,
    pub pps_act_y_qp_offset_plus5: i8,
    pub pps_act_cb_qp_offset_plus5: i8,
    pub pps_act_cr_qp_offset_plus5: i8,
    pub pps_num_palette_predictor_initializer: u8,
    pub luma_bit_depth_entry_minus8: u8,
    pub chroma_bit_depth_entry_minus8: u8,
    pub p_predictor_palette_entries: *mut crate::external::vk_video::StdVideoH265PredictorPaletteEntries,
}
impl Default for StdVideoH265PictureParameterSet {
    fn default() -> Self {
        Self { pps_pic_parameter_set_id: Default::default(), pps_seq_parameter_set_id: Default::default(), num_extra_slice_header_bits: Default::default(), num_ref_idx_l0_default_active_minus1: Default::default(), num_ref_idx_l1_default_active_minus1: Default::default(), init_qp_minus26: Default::default(), diff_cu_qp_delta_depth: Default::default(), pps_cb_qp_offset: Default::default(), pps_cr_qp_offset: Default::default(), num_tile_columns_minus1: Default::default(), num_tile_rows_minus1: Default::default(), column_width_minus1: unsafe { std::mem::zeroed() }, row_height_minus1: unsafe { std::mem::zeroed() }, pps_beta_offset_div2: Default::default(), pps_tc_offset_div2: Default::default(), log2_parallel_merge_level_minus2: Default::default(), flags: Default::default(), p_scaling_lists: std::ptr::null_mut(), log2_max_transform_skip_block_size_minus2: Default::default(), diff_cu_chroma_qp_offset_depth: Default::default(), chroma_qp_offset_list_len_minus1: Default::default(), cb_qp_offset_list: unsafe { std::mem::zeroed() }, cr_qp_offset_list: unsafe { std::mem::zeroed() }, log2_sao_offset_scale_luma: Default::default(), log2_sao_offset_scale_chroma: Default::default(), pps_act_y_qp_offset_plus5: Default::default(), pps_act_cb_qp_offset_plus5: Default::default(), pps_act_cr_qp_offset_plus5: Default::default(), pps_num_palette_predictor_initializer: Default::default(), luma_bit_depth_entry_minus8: Default::default(), chroma_bit_depth_entry_minus8: Default::default(), p_predictor_palette_entries: std::ptr::null_mut() }
    }
}
impl std::fmt::Debug for StdVideoH265PictureParameterSet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StdVideoH265PictureParameterSet").field("pps_pic_parameter_set_id", &self.pps_pic_parameter_set_id).field("pps_seq_parameter_set_id", &self.pps_seq_parameter_set_id).field("num_extra_slice_header_bits", &self.num_extra_slice_header_bits).field("num_ref_idx_l0_default_active_minus1", &self.num_ref_idx_l0_default_active_minus1).field("num_ref_idx_l1_default_active_minus1", &self.num_ref_idx_l1_default_active_minus1).field("init_qp_minus26", &self.init_qp_minus26).field("diff_cu_qp_delta_depth", &self.diff_cu_qp_delta_depth).field("pps_cb_qp_offset", &self.pps_cb_qp_offset).field("pps_cr_qp_offset", &self.pps_cr_qp_offset).field("num_tile_columns_minus1", &self.num_tile_columns_minus1).field("num_tile_rows_minus1", &self.num_tile_rows_minus1).field("column_width_minus1", &self.column_width_minus1).field("row_height_minus1", &self.row_height_minus1).field("pps_beta_offset_div2", &self.pps_beta_offset_div2).field("pps_tc_offset_div2", &self.pps_tc_offset_div2).field("log2_parallel_merge_level_minus2", &self.log2_parallel_merge_level_minus2).field("flags", &self.flags).field("p_scaling_lists", &self.p_scaling_lists).field("log2_max_transform_skip_block_size_minus2", &self.log2_max_transform_skip_block_size_minus2).field("diff_cu_chroma_qp_offset_depth", &self.diff_cu_chroma_qp_offset_depth).field("chroma_qp_offset_list_len_minus1", &self.chroma_qp_offset_list_len_minus1).field("cb_qp_offset_list", &self.cb_qp_offset_list).field("cr_qp_offset_list", &self.cr_qp_offset_list).field("log2_sao_offset_scale_luma", &self.log2_sao_offset_scale_luma).field("log2_sao_offset_scale_chroma", &self.log2_sao_offset_scale_chroma).field("pps_act_y_qp_offset_plus5", &self.pps_act_y_qp_offset_plus5).field("pps_act_cb_qp_offset_plus5", &self.pps_act_cb_qp_offset_plus5).field("pps_act_cr_qp_offset_plus5", &self.pps_act_cr_qp_offset_plus5).field("pps_num_palette_predictor_initializer", &self.pps_num_palette_predictor_initializer).field("luma_bit_depth_entry_minus8", &self.luma_bit_depth_entry_minus8).field("chroma_bit_depth_entry_minus8", &self.chroma_bit_depth_entry_minus8).field("p_predictor_palette_entries", &self.p_predictor_palette_entries).finish()
    }
}
impl StdVideoH265PictureParameterSet {
    #[inline]
    pub fn into_builder<'a>(self) -> StdVideoH265PictureParameterSetBuilder<'a> {
        StdVideoH265PictureParameterSetBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "<s>Vulkan Manual Page</s> · Builder of [`StdVideoH265PictureParameterSet`]"]
#[repr(transparent)]
pub struct StdVideoH265PictureParameterSetBuilder<'a>(StdVideoH265PictureParameterSet, std::marker::PhantomData<&'a ()>);
impl<'a> StdVideoH265PictureParameterSetBuilder<'a> {
    #[inline]
    pub fn new() -> StdVideoH265PictureParameterSetBuilder<'a> {
        StdVideoH265PictureParameterSetBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn pps_pic_parameter_set_id(mut self, pps_pic_parameter_set_id: u8) -> Self {
        self.0.pps_pic_parameter_set_id = pps_pic_parameter_set_id as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn pps_seq_parameter_set_id(mut self, pps_seq_parameter_set_id: u8) -> Self {
        self.0.pps_seq_parameter_set_id = pps_seq_parameter_set_id as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn num_extra_slice_header_bits(mut self, num_extra_slice_header_bits: u8) -> Self {
        self.0.num_extra_slice_header_bits = num_extra_slice_header_bits as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn num_ref_idx_l0_default_active_minus1(mut self, num_ref_idx_l0_default_active_minus1: u8) -> Self {
        self.0.num_ref_idx_l0_default_active_minus1 = num_ref_idx_l0_default_active_minus1 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn num_ref_idx_l1_default_active_minus1(mut self, num_ref_idx_l1_default_active_minus1: u8) -> Self {
        self.0.num_ref_idx_l1_default_active_minus1 = num_ref_idx_l1_default_active_minus1 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn init_qp_minus26(mut self, init_qp_minus26: i8) -> Self {
        self.0.init_qp_minus26 = init_qp_minus26 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn diff_cu_qp_delta_depth(mut self, diff_cu_qp_delta_depth: u8) -> Self {
        self.0.diff_cu_qp_delta_depth = diff_cu_qp_delta_depth as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn pps_cb_qp_offset(mut self, pps_cb_qp_offset: i8) -> Self {
        self.0.pps_cb_qp_offset = pps_cb_qp_offset as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn pps_cr_qp_offset(mut self, pps_cr_qp_offset: i8) -> Self {
        self.0.pps_cr_qp_offset = pps_cr_qp_offset as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn num_tile_columns_minus1(mut self, num_tile_columns_minus1: u8) -> Self {
        self.0.num_tile_columns_minus1 = num_tile_columns_minus1 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn num_tile_rows_minus1(mut self, num_tile_rows_minus1: u8) -> Self {
        self.0.num_tile_rows_minus1 = num_tile_rows_minus1 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn column_width_minus1(mut self, column_width_minus1: [u16; 19]) -> Self {
        self.0.column_width_minus1 = column_width_minus1 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn row_height_minus1(mut self, row_height_minus1: [u16; 21]) -> Self {
        self.0.row_height_minus1 = row_height_minus1 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn pps_beta_offset_div2(mut self, pps_beta_offset_div2: i8) -> Self {
        self.0.pps_beta_offset_div2 = pps_beta_offset_div2 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn pps_tc_offset_div2(mut self, pps_tc_offset_div2: i8) -> Self {
        self.0.pps_tc_offset_div2 = pps_tc_offset_div2 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn log2_parallel_merge_level_minus2(mut self, log2_parallel_merge_level_minus2: u8) -> Self {
        self.0.log2_parallel_merge_level_minus2 = log2_parallel_merge_level_minus2 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn flags(mut self, flags: crate::external::vk_video::StdVideoH265PpsFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn scaling_lists(mut self, scaling_lists: &'a mut crate::external::vk_video::StdVideoH265ScalingLists) -> Self {
        self.0.p_scaling_lists = scaling_lists as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn log2_max_transform_skip_block_size_minus2(mut self, log2_max_transform_skip_block_size_minus2: u8) -> Self {
        self.0.log2_max_transform_skip_block_size_minus2 = log2_max_transform_skip_block_size_minus2 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn diff_cu_chroma_qp_offset_depth(mut self, diff_cu_chroma_qp_offset_depth: u8) -> Self {
        self.0.diff_cu_chroma_qp_offset_depth = diff_cu_chroma_qp_offset_depth as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn chroma_qp_offset_list_len_minus1(mut self, chroma_qp_offset_list_len_minus1: u8) -> Self {
        self.0.chroma_qp_offset_list_len_minus1 = chroma_qp_offset_list_len_minus1 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn cb_qp_offset_list(mut self, cb_qp_offset_list: [i8; 6]) -> Self {
        self.0.cb_qp_offset_list = cb_qp_offset_list as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn cr_qp_offset_list(mut self, cr_qp_offset_list: [i8; 6]) -> Self {
        self.0.cr_qp_offset_list = cr_qp_offset_list as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn log2_sao_offset_scale_luma(mut self, log2_sao_offset_scale_luma: u8) -> Self {
        self.0.log2_sao_offset_scale_luma = log2_sao_offset_scale_luma as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn log2_sao_offset_scale_chroma(mut self, log2_sao_offset_scale_chroma: u8) -> Self {
        self.0.log2_sao_offset_scale_chroma = log2_sao_offset_scale_chroma as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn pps_act_y_qp_offset_plus5(mut self, pps_act_y_qp_offset_plus5: i8) -> Self {
        self.0.pps_act_y_qp_offset_plus5 = pps_act_y_qp_offset_plus5 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn pps_act_cb_qp_offset_plus5(mut self, pps_act_cb_qp_offset_plus5: i8) -> Self {
        self.0.pps_act_cb_qp_offset_plus5 = pps_act_cb_qp_offset_plus5 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn pps_act_cr_qp_offset_plus5(mut self, pps_act_cr_qp_offset_plus5: i8) -> Self {
        self.0.pps_act_cr_qp_offset_plus5 = pps_act_cr_qp_offset_plus5 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn pps_num_palette_predictor_initializer(mut self, pps_num_palette_predictor_initializer: u8) -> Self {
        self.0.pps_num_palette_predictor_initializer = pps_num_palette_predictor_initializer as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn luma_bit_depth_entry_minus8(mut self, luma_bit_depth_entry_minus8: u8) -> Self {
        self.0.luma_bit_depth_entry_minus8 = luma_bit_depth_entry_minus8 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn chroma_bit_depth_entry_minus8(mut self, chroma_bit_depth_entry_minus8: u8) -> Self {
        self.0.chroma_bit_depth_entry_minus8 = chroma_bit_depth_entry_minus8 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn predictor_palette_entries(mut self, predictor_palette_entries: &'a mut crate::external::vk_video::StdVideoH265PredictorPaletteEntries) -> Self {
        self.0.p_predictor_palette_entries = predictor_palette_entries as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> StdVideoH265PictureParameterSet {
        self.0
    }
}
impl<'a> std::default::Default for StdVideoH265PictureParameterSetBuilder<'a> {
    fn default() -> StdVideoH265PictureParameterSetBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for StdVideoH265PictureParameterSetBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for StdVideoH265PictureParameterSetBuilder<'a> {
    type Target = StdVideoH265PictureParameterSet;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StdVideoH265PictureParameterSetBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Structure"]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct StdVideoH265DecPicBufMgr {
    pub max_latency_increase_plus1: [u32; 7],
    pub max_dec_pic_buffering_minus1: [u8; 7],
    pub max_num_reorder_pics: [u8; 7],
}
impl Default for StdVideoH265DecPicBufMgr {
    fn default() -> Self {
        Self { max_latency_increase_plus1: unsafe { std::mem::zeroed() }, max_dec_pic_buffering_minus1: unsafe { std::mem::zeroed() }, max_num_reorder_pics: unsafe { std::mem::zeroed() } }
    }
}
impl std::fmt::Debug for StdVideoH265DecPicBufMgr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StdVideoH265DecPicBufMgr").field("max_latency_increase_plus1", &self.max_latency_increase_plus1).field("max_dec_pic_buffering_minus1", &self.max_dec_pic_buffering_minus1).field("max_num_reorder_pics", &self.max_num_reorder_pics).finish()
    }
}
impl StdVideoH265DecPicBufMgr {
    #[inline]
    pub fn into_builder<'a>(self) -> StdVideoH265DecPicBufMgrBuilder<'a> {
        StdVideoH265DecPicBufMgrBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "<s>Vulkan Manual Page</s> · Builder of [`StdVideoH265DecPicBufMgr`]"]
#[repr(transparent)]
pub struct StdVideoH265DecPicBufMgrBuilder<'a>(StdVideoH265DecPicBufMgr, std::marker::PhantomData<&'a ()>);
impl<'a> StdVideoH265DecPicBufMgrBuilder<'a> {
    #[inline]
    pub fn new() -> StdVideoH265DecPicBufMgrBuilder<'a> {
        StdVideoH265DecPicBufMgrBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn max_latency_increase_plus1(mut self, max_latency_increase_plus1: [u32; 7]) -> Self {
        self.0.max_latency_increase_plus1 = max_latency_increase_plus1 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn max_dec_pic_buffering_minus1(mut self, max_dec_pic_buffering_minus1: [u8; 7]) -> Self {
        self.0.max_dec_pic_buffering_minus1 = max_dec_pic_buffering_minus1 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn max_num_reorder_pics(mut self, max_num_reorder_pics: [u8; 7]) -> Self {
        self.0.max_num_reorder_pics = max_num_reorder_pics as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> StdVideoH265DecPicBufMgr {
        self.0
    }
}
impl<'a> std::default::Default for StdVideoH265DecPicBufMgrBuilder<'a> {
    fn default() -> StdVideoH265DecPicBufMgrBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for StdVideoH265DecPicBufMgrBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for StdVideoH265DecPicBufMgrBuilder<'a> {
    type Target = StdVideoH265DecPicBufMgr;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StdVideoH265DecPicBufMgrBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StdVideoH265HrdParameters {
    pub tick_divisor_minus2: u8,
    pub du_cpb_removal_delay_increment_length_minus1: u8,
    pub dpb_output_delay_du_length_minus1: u8,
    pub bit_rate_scale: u8,
    pub cpb_size_scale: u8,
    pub cpb_size_du_scale: u8,
    pub initial_cpb_removal_delay_length_minus1: u8,
    pub au_cpb_removal_delay_length_minus1: u8,
    pub dpb_output_delay_length_minus1: u8,
    pub cpb_cnt_minus1: [u8; 7],
    pub elemental_duration_in_tc_minus1: [u16; 7],
    pub p_sub_layer_hrd_parameters_nal: [*mut crate::external::vk_video::StdVideoH265SubLayerHrdParameters; 7],
    pub p_sub_layer_hrd_parameters_vcl: [*mut crate::external::vk_video::StdVideoH265SubLayerHrdParameters; 7],
    pub flags: crate::external::vk_video::StdVideoH265HrdFlags,
}
impl Default for StdVideoH265HrdParameters {
    fn default() -> Self {
        Self { tick_divisor_minus2: Default::default(), du_cpb_removal_delay_increment_length_minus1: Default::default(), dpb_output_delay_du_length_minus1: Default::default(), bit_rate_scale: Default::default(), cpb_size_scale: Default::default(), cpb_size_du_scale: Default::default(), initial_cpb_removal_delay_length_minus1: Default::default(), au_cpb_removal_delay_length_minus1: Default::default(), dpb_output_delay_length_minus1: Default::default(), cpb_cnt_minus1: unsafe { std::mem::zeroed() }, elemental_duration_in_tc_minus1: unsafe { std::mem::zeroed() }, p_sub_layer_hrd_parameters_nal: unsafe { std::mem::zeroed() }, p_sub_layer_hrd_parameters_vcl: unsafe { std::mem::zeroed() }, flags: Default::default() }
    }
}
impl std::fmt::Debug for StdVideoH265HrdParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StdVideoH265HrdParameters").field("tick_divisor_minus2", &self.tick_divisor_minus2).field("du_cpb_removal_delay_increment_length_minus1", &self.du_cpb_removal_delay_increment_length_minus1).field("dpb_output_delay_du_length_minus1", &self.dpb_output_delay_du_length_minus1).field("bit_rate_scale", &self.bit_rate_scale).field("cpb_size_scale", &self.cpb_size_scale).field("cpb_size_du_scale", &self.cpb_size_du_scale).field("initial_cpb_removal_delay_length_minus1", &self.initial_cpb_removal_delay_length_minus1).field("au_cpb_removal_delay_length_minus1", &self.au_cpb_removal_delay_length_minus1).field("dpb_output_delay_length_minus1", &self.dpb_output_delay_length_minus1).field("cpb_cnt_minus1", &self.cpb_cnt_minus1).field("elemental_duration_in_tc_minus1", &self.elemental_duration_in_tc_minus1).field("p_sub_layer_hrd_parameters_nal", &self.p_sub_layer_hrd_parameters_nal).field("p_sub_layer_hrd_parameters_vcl", &self.p_sub_layer_hrd_parameters_vcl).field("flags", &self.flags).finish()
    }
}
impl StdVideoH265HrdParameters {
    #[inline]
    pub fn into_builder<'a>(self) -> StdVideoH265HrdParametersBuilder<'a> {
        StdVideoH265HrdParametersBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "<s>Vulkan Manual Page</s> · Builder of [`StdVideoH265HrdParameters`]"]
#[repr(transparent)]
pub struct StdVideoH265HrdParametersBuilder<'a>(StdVideoH265HrdParameters, std::marker::PhantomData<&'a ()>);
impl<'a> StdVideoH265HrdParametersBuilder<'a> {
    #[inline]
    pub fn new() -> StdVideoH265HrdParametersBuilder<'a> {
        StdVideoH265HrdParametersBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn tick_divisor_minus2(mut self, tick_divisor_minus2: u8) -> Self {
        self.0.tick_divisor_minus2 = tick_divisor_minus2 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn du_cpb_removal_delay_increment_length_minus1(mut self, du_cpb_removal_delay_increment_length_minus1: u8) -> Self {
        self.0.du_cpb_removal_delay_increment_length_minus1 = du_cpb_removal_delay_increment_length_minus1 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn dpb_output_delay_du_length_minus1(mut self, dpb_output_delay_du_length_minus1: u8) -> Self {
        self.0.dpb_output_delay_du_length_minus1 = dpb_output_delay_du_length_minus1 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn bit_rate_scale(mut self, bit_rate_scale: u8) -> Self {
        self.0.bit_rate_scale = bit_rate_scale as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn cpb_size_scale(mut self, cpb_size_scale: u8) -> Self {
        self.0.cpb_size_scale = cpb_size_scale as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn cpb_size_du_scale(mut self, cpb_size_du_scale: u8) -> Self {
        self.0.cpb_size_du_scale = cpb_size_du_scale as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn initial_cpb_removal_delay_length_minus1(mut self, initial_cpb_removal_delay_length_minus1: u8) -> Self {
        self.0.initial_cpb_removal_delay_length_minus1 = initial_cpb_removal_delay_length_minus1 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn au_cpb_removal_delay_length_minus1(mut self, au_cpb_removal_delay_length_minus1: u8) -> Self {
        self.0.au_cpb_removal_delay_length_minus1 = au_cpb_removal_delay_length_minus1 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn dpb_output_delay_length_minus1(mut self, dpb_output_delay_length_minus1: u8) -> Self {
        self.0.dpb_output_delay_length_minus1 = dpb_output_delay_length_minus1 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn cpb_cnt_minus1(mut self, cpb_cnt_minus1: [u8; 7]) -> Self {
        self.0.cpb_cnt_minus1 = cpb_cnt_minus1 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn elemental_duration_in_tc_minus1(mut self, elemental_duration_in_tc_minus1: [u16; 7]) -> Self {
        self.0.elemental_duration_in_tc_minus1 = elemental_duration_in_tc_minus1 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn sub_layer_hrd_parameters_nal(mut self, sub_layer_hrd_parameters_nal: [*mut crate::external::vk_video::StdVideoH265SubLayerHrdParameters; 7]) -> Self {
        self.0.p_sub_layer_hrd_parameters_nal = sub_layer_hrd_parameters_nal as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn sub_layer_hrd_parameters_vcl(mut self, sub_layer_hrd_parameters_vcl: [*mut crate::external::vk_video::StdVideoH265SubLayerHrdParameters; 7]) -> Self {
        self.0.p_sub_layer_hrd_parameters_vcl = sub_layer_hrd_parameters_vcl as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn flags(mut self, flags: crate::external::vk_video::StdVideoH265HrdFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> StdVideoH265HrdParameters {
        self.0
    }
}
impl<'a> std::default::Default for StdVideoH265HrdParametersBuilder<'a> {
    fn default() -> StdVideoH265HrdParametersBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for StdVideoH265HrdParametersBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for StdVideoH265HrdParametersBuilder<'a> {
    type Target = StdVideoH265HrdParameters;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StdVideoH265HrdParametersBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Structure"]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct StdVideoH265VpsFlags {
    pub vps_temporal_id_nesting_flag_and_more_bitfield: u32,
}
impl Default for StdVideoH265VpsFlags {
    fn default() -> Self {
        Self { vps_temporal_id_nesting_flag_and_more_bitfield: Default::default() }
    }
}
impl std::fmt::Debug for StdVideoH265VpsFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StdVideoH265VpsFlags").field("vps_temporal_id_nesting_flag_and_more_bitfield", &format!("{:#b}", &self.vps_temporal_id_nesting_flag_and_more_bitfield)).finish()
    }
}
impl StdVideoH265VpsFlags {
    #[inline]
    pub fn into_builder<'a>(self) -> StdVideoH265VpsFlagsBuilder<'a> {
        StdVideoH265VpsFlagsBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "<s>Vulkan Manual Page</s> · Builder of [`StdVideoH265VpsFlags`]"]
#[repr(transparent)]
pub struct StdVideoH265VpsFlagsBuilder<'a>(StdVideoH265VpsFlags, std::marker::PhantomData<&'a ()>);
impl<'a> StdVideoH265VpsFlagsBuilder<'a> {
    #[inline]
    pub fn new() -> StdVideoH265VpsFlagsBuilder<'a> {
        StdVideoH265VpsFlagsBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn vps_temporal_id_nesting_flag(mut self, vps_temporal_id_nesting_flag: u32) -> Self {
        self.0.vps_temporal_id_nesting_flag_and_more_bitfield = crate::bits_copy!(self.0.vps_temporal_id_nesting_flag_and_more_bitfield, vps_temporal_id_nesting_flag, 0usize, 0usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn vps_sub_layer_ordering_info_present_flag(mut self, vps_sub_layer_ordering_info_present_flag: u32) -> Self {
        self.0.vps_temporal_id_nesting_flag_and_more_bitfield = crate::bits_copy!(self.0.vps_temporal_id_nesting_flag_and_more_bitfield, vps_sub_layer_ordering_info_present_flag, 1usize, 1usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn vps_timing_info_present_flag(mut self, vps_timing_info_present_flag: u32) -> Self {
        self.0.vps_temporal_id_nesting_flag_and_more_bitfield = crate::bits_copy!(self.0.vps_temporal_id_nesting_flag_and_more_bitfield, vps_timing_info_present_flag, 2usize, 2usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn vps_poc_proportional_to_timing_flag(mut self, vps_poc_proportional_to_timing_flag: u32) -> Self {
        self.0.vps_temporal_id_nesting_flag_and_more_bitfield = crate::bits_copy!(self.0.vps_temporal_id_nesting_flag_and_more_bitfield, vps_poc_proportional_to_timing_flag, 3usize, 3usize);
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> StdVideoH265VpsFlags {
        self.0
    }
}
impl<'a> std::default::Default for StdVideoH265VpsFlagsBuilder<'a> {
    fn default() -> StdVideoH265VpsFlagsBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for StdVideoH265VpsFlagsBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for StdVideoH265VpsFlagsBuilder<'a> {
    type Target = StdVideoH265VpsFlags;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StdVideoH265VpsFlagsBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Structure"]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct StdVideoH265SpsFlags {
    pub sps_temporal_id_nesting_flag_and_more_bitfield: u32,
}
impl Default for StdVideoH265SpsFlags {
    fn default() -> Self {
        Self { sps_temporal_id_nesting_flag_and_more_bitfield: Default::default() }
    }
}
impl std::fmt::Debug for StdVideoH265SpsFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StdVideoH265SpsFlags").field("sps_temporal_id_nesting_flag_and_more_bitfield", &format!("{:#b}", &self.sps_temporal_id_nesting_flag_and_more_bitfield)).finish()
    }
}
impl StdVideoH265SpsFlags {
    #[inline]
    pub fn into_builder<'a>(self) -> StdVideoH265SpsFlagsBuilder<'a> {
        StdVideoH265SpsFlagsBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "<s>Vulkan Manual Page</s> · Builder of [`StdVideoH265SpsFlags`]"]
#[repr(transparent)]
pub struct StdVideoH265SpsFlagsBuilder<'a>(StdVideoH265SpsFlags, std::marker::PhantomData<&'a ()>);
impl<'a> StdVideoH265SpsFlagsBuilder<'a> {
    #[inline]
    pub fn new() -> StdVideoH265SpsFlagsBuilder<'a> {
        StdVideoH265SpsFlagsBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn sps_temporal_id_nesting_flag(mut self, sps_temporal_id_nesting_flag: u32) -> Self {
        self.0.sps_temporal_id_nesting_flag_and_more_bitfield = crate::bits_copy!(self.0.sps_temporal_id_nesting_flag_and_more_bitfield, sps_temporal_id_nesting_flag, 0usize, 0usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn separate_colour_plane_flag(mut self, separate_colour_plane_flag: u32) -> Self {
        self.0.sps_temporal_id_nesting_flag_and_more_bitfield = crate::bits_copy!(self.0.sps_temporal_id_nesting_flag_and_more_bitfield, separate_colour_plane_flag, 1usize, 1usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn scaling_list_enabled_flag(mut self, scaling_list_enabled_flag: u32) -> Self {
        self.0.sps_temporal_id_nesting_flag_and_more_bitfield = crate::bits_copy!(self.0.sps_temporal_id_nesting_flag_and_more_bitfield, scaling_list_enabled_flag, 2usize, 2usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn sps_scaling_list_data_present_flag(mut self, sps_scaling_list_data_present_flag: u32) -> Self {
        self.0.sps_temporal_id_nesting_flag_and_more_bitfield = crate::bits_copy!(self.0.sps_temporal_id_nesting_flag_and_more_bitfield, sps_scaling_list_data_present_flag, 3usize, 3usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn amp_enabled_flag(mut self, amp_enabled_flag: u32) -> Self {
        self.0.sps_temporal_id_nesting_flag_and_more_bitfield = crate::bits_copy!(self.0.sps_temporal_id_nesting_flag_and_more_bitfield, amp_enabled_flag, 4usize, 4usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn sample_adaptive_offset_enabled_flag(mut self, sample_adaptive_offset_enabled_flag: u32) -> Self {
        self.0.sps_temporal_id_nesting_flag_and_more_bitfield = crate::bits_copy!(self.0.sps_temporal_id_nesting_flag_and_more_bitfield, sample_adaptive_offset_enabled_flag, 5usize, 5usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn pcm_enabled_flag(mut self, pcm_enabled_flag: u32) -> Self {
        self.0.sps_temporal_id_nesting_flag_and_more_bitfield = crate::bits_copy!(self.0.sps_temporal_id_nesting_flag_and_more_bitfield, pcm_enabled_flag, 6usize, 6usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn pcm_loop_filter_disabled_flag(mut self, pcm_loop_filter_disabled_flag: u32) -> Self {
        self.0.sps_temporal_id_nesting_flag_and_more_bitfield = crate::bits_copy!(self.0.sps_temporal_id_nesting_flag_and_more_bitfield, pcm_loop_filter_disabled_flag, 7usize, 7usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn long_term_ref_pics_present_flag(mut self, long_term_ref_pics_present_flag: u32) -> Self {
        self.0.sps_temporal_id_nesting_flag_and_more_bitfield = crate::bits_copy!(self.0.sps_temporal_id_nesting_flag_and_more_bitfield, long_term_ref_pics_present_flag, 8usize, 8usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn sps_temporal_mvp_enabled_flag(mut self, sps_temporal_mvp_enabled_flag: u32) -> Self {
        self.0.sps_temporal_id_nesting_flag_and_more_bitfield = crate::bits_copy!(self.0.sps_temporal_id_nesting_flag_and_more_bitfield, sps_temporal_mvp_enabled_flag, 9usize, 9usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn strong_intra_smoothing_enabled_flag(mut self, strong_intra_smoothing_enabled_flag: u32) -> Self {
        self.0.sps_temporal_id_nesting_flag_and_more_bitfield = crate::bits_copy!(self.0.sps_temporal_id_nesting_flag_and_more_bitfield, strong_intra_smoothing_enabled_flag, 10usize, 10usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn vui_parameters_present_flag(mut self, vui_parameters_present_flag: u32) -> Self {
        self.0.sps_temporal_id_nesting_flag_and_more_bitfield = crate::bits_copy!(self.0.sps_temporal_id_nesting_flag_and_more_bitfield, vui_parameters_present_flag, 11usize, 11usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn sps_extension_present_flag(mut self, sps_extension_present_flag: u32) -> Self {
        self.0.sps_temporal_id_nesting_flag_and_more_bitfield = crate::bits_copy!(self.0.sps_temporal_id_nesting_flag_and_more_bitfield, sps_extension_present_flag, 12usize, 12usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn sps_range_extension_flag(mut self, sps_range_extension_flag: u32) -> Self {
        self.0.sps_temporal_id_nesting_flag_and_more_bitfield = crate::bits_copy!(self.0.sps_temporal_id_nesting_flag_and_more_bitfield, sps_range_extension_flag, 13usize, 13usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn transform_skip_rotation_enabled_flag(mut self, transform_skip_rotation_enabled_flag: u32) -> Self {
        self.0.sps_temporal_id_nesting_flag_and_more_bitfield = crate::bits_copy!(self.0.sps_temporal_id_nesting_flag_and_more_bitfield, transform_skip_rotation_enabled_flag, 14usize, 14usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn transform_skip_context_enabled_flag(mut self, transform_skip_context_enabled_flag: u32) -> Self {
        self.0.sps_temporal_id_nesting_flag_and_more_bitfield = crate::bits_copy!(self.0.sps_temporal_id_nesting_flag_and_more_bitfield, transform_skip_context_enabled_flag, 15usize, 15usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn implicit_rdpcm_enabled_flag(mut self, implicit_rdpcm_enabled_flag: u32) -> Self {
        self.0.sps_temporal_id_nesting_flag_and_more_bitfield = crate::bits_copy!(self.0.sps_temporal_id_nesting_flag_and_more_bitfield, implicit_rdpcm_enabled_flag, 16usize, 16usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn explicit_rdpcm_enabled_flag(mut self, explicit_rdpcm_enabled_flag: u32) -> Self {
        self.0.sps_temporal_id_nesting_flag_and_more_bitfield = crate::bits_copy!(self.0.sps_temporal_id_nesting_flag_and_more_bitfield, explicit_rdpcm_enabled_flag, 17usize, 17usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn extended_precision_processing_flag(mut self, extended_precision_processing_flag: u32) -> Self {
        self.0.sps_temporal_id_nesting_flag_and_more_bitfield = crate::bits_copy!(self.0.sps_temporal_id_nesting_flag_and_more_bitfield, extended_precision_processing_flag, 18usize, 18usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn intra_smoothing_disabled_flag(mut self, intra_smoothing_disabled_flag: u32) -> Self {
        self.0.sps_temporal_id_nesting_flag_and_more_bitfield = crate::bits_copy!(self.0.sps_temporal_id_nesting_flag_and_more_bitfield, intra_smoothing_disabled_flag, 19usize, 19usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn high_precision_offsets_enabled_flag(mut self, high_precision_offsets_enabled_flag: u32) -> Self {
        self.0.sps_temporal_id_nesting_flag_and_more_bitfield = crate::bits_copy!(self.0.sps_temporal_id_nesting_flag_and_more_bitfield, high_precision_offsets_enabled_flag, 20usize, 20usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn persistent_rice_adaptation_enabled_flag(mut self, persistent_rice_adaptation_enabled_flag: u32) -> Self {
        self.0.sps_temporal_id_nesting_flag_and_more_bitfield = crate::bits_copy!(self.0.sps_temporal_id_nesting_flag_and_more_bitfield, persistent_rice_adaptation_enabled_flag, 21usize, 21usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn cabac_bypass_alignment_enabled_flag(mut self, cabac_bypass_alignment_enabled_flag: u32) -> Self {
        self.0.sps_temporal_id_nesting_flag_and_more_bitfield = crate::bits_copy!(self.0.sps_temporal_id_nesting_flag_and_more_bitfield, cabac_bypass_alignment_enabled_flag, 22usize, 22usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn sps_curr_pic_ref_enabled_flag(mut self, sps_curr_pic_ref_enabled_flag: u32) -> Self {
        self.0.sps_temporal_id_nesting_flag_and_more_bitfield = crate::bits_copy!(self.0.sps_temporal_id_nesting_flag_and_more_bitfield, sps_curr_pic_ref_enabled_flag, 23usize, 23usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn palette_mode_enabled_flag(mut self, palette_mode_enabled_flag: u32) -> Self {
        self.0.sps_temporal_id_nesting_flag_and_more_bitfield = crate::bits_copy!(self.0.sps_temporal_id_nesting_flag_and_more_bitfield, palette_mode_enabled_flag, 24usize, 24usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn sps_palette_predictor_initializer_present_flag(mut self, sps_palette_predictor_initializer_present_flag: u32) -> Self {
        self.0.sps_temporal_id_nesting_flag_and_more_bitfield = crate::bits_copy!(self.0.sps_temporal_id_nesting_flag_and_more_bitfield, sps_palette_predictor_initializer_present_flag, 25usize, 25usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn intra_boundary_filtering_disabled_flag(mut self, intra_boundary_filtering_disabled_flag: u32) -> Self {
        self.0.sps_temporal_id_nesting_flag_and_more_bitfield = crate::bits_copy!(self.0.sps_temporal_id_nesting_flag_and_more_bitfield, intra_boundary_filtering_disabled_flag, 26usize, 26usize);
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> StdVideoH265SpsFlags {
        self.0
    }
}
impl<'a> std::default::Default for StdVideoH265SpsFlagsBuilder<'a> {
    fn default() -> StdVideoH265SpsFlagsBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for StdVideoH265SpsFlagsBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for StdVideoH265SpsFlagsBuilder<'a> {
    type Target = StdVideoH265SpsFlags;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StdVideoH265SpsFlagsBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Structure"]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct StdVideoH265ScalingLists {
    pub scaling_list4x4: [[u8; 16]; 6],
    pub scaling_list8x8: [[u8; 64]; 6],
    pub scaling_list16x16: [[u8; 64]; 6],
    pub scaling_list32x32: [[u8; 64]; 2],
    pub scaling_list_dc_coef16x16: [u8; 6],
    pub scaling_list_dc_coef32x32: [u8; 2],
}
impl Default for StdVideoH265ScalingLists {
    fn default() -> Self {
        Self { scaling_list4x4: unsafe { std::mem::zeroed() }, scaling_list8x8: unsafe { std::mem::zeroed() }, scaling_list16x16: unsafe { std::mem::zeroed() }, scaling_list32x32: unsafe { std::mem::zeroed() }, scaling_list_dc_coef16x16: unsafe { std::mem::zeroed() }, scaling_list_dc_coef32x32: unsafe { std::mem::zeroed() } }
    }
}
impl std::fmt::Debug for StdVideoH265ScalingLists {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StdVideoH265ScalingLists").field("scaling_list4x4", &self.scaling_list4x4).field("scaling_list8x8", &self.scaling_list8x8).field("scaling_list16x16", &self.scaling_list16x16).field("scaling_list32x32", &self.scaling_list32x32).field("scaling_list_dc_coef16x16", &self.scaling_list_dc_coef16x16).field("scaling_list_dc_coef32x32", &self.scaling_list_dc_coef32x32).finish()
    }
}
impl StdVideoH265ScalingLists {
    #[inline]
    pub fn into_builder<'a>(self) -> StdVideoH265ScalingListsBuilder<'a> {
        StdVideoH265ScalingListsBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "<s>Vulkan Manual Page</s> · Builder of [`StdVideoH265ScalingLists`]"]
#[repr(transparent)]
pub struct StdVideoH265ScalingListsBuilder<'a>(StdVideoH265ScalingLists, std::marker::PhantomData<&'a ()>);
impl<'a> StdVideoH265ScalingListsBuilder<'a> {
    #[inline]
    pub fn new() -> StdVideoH265ScalingListsBuilder<'a> {
        StdVideoH265ScalingListsBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn scaling_list4x4(mut self, scaling_list4x4: [[u8; 16]; 6]) -> Self {
        self.0.scaling_list4x4 = scaling_list4x4 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn scaling_list8x8(mut self, scaling_list8x8: [[u8; 64]; 6]) -> Self {
        self.0.scaling_list8x8 = scaling_list8x8 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn scaling_list16x16(mut self, scaling_list16x16: [[u8; 64]; 6]) -> Self {
        self.0.scaling_list16x16 = scaling_list16x16 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn scaling_list32x32(mut self, scaling_list32x32: [[u8; 64]; 2]) -> Self {
        self.0.scaling_list32x32 = scaling_list32x32 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn scaling_list_dc_coef16x16(mut self, scaling_list_dc_coef16x16: [u8; 6]) -> Self {
        self.0.scaling_list_dc_coef16x16 = scaling_list_dc_coef16x16 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn scaling_list_dc_coef32x32(mut self, scaling_list_dc_coef32x32: [u8; 2]) -> Self {
        self.0.scaling_list_dc_coef32x32 = scaling_list_dc_coef32x32 as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> StdVideoH265ScalingLists {
        self.0
    }
}
impl<'a> std::default::Default for StdVideoH265ScalingListsBuilder<'a> {
    fn default() -> StdVideoH265ScalingListsBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for StdVideoH265ScalingListsBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for StdVideoH265ScalingListsBuilder<'a> {
    type Target = StdVideoH265ScalingLists;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StdVideoH265ScalingListsBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StdVideoH265SequenceParameterSetVui {
    pub aspect_ratio_idc: u8,
    pub sar_width: u16,
    pub sar_height: u16,
    pub video_format: u8,
    pub colour_primaries: u8,
    pub transfer_characteristics: u8,
    pub matrix_coeffs: u8,
    pub chroma_sample_loc_type_top_field: u8,
    pub chroma_sample_loc_type_bottom_field: u8,
    pub def_disp_win_left_offset: u16,
    pub def_disp_win_right_offset: u16,
    pub def_disp_win_top_offset: u16,
    pub def_disp_win_bottom_offset: u16,
    pub vui_num_units_in_tick: u32,
    pub vui_time_scale: u32,
    pub vui_num_ticks_poc_diff_one_minus1: u32,
    pub p_hrd_parameters: *mut crate::external::vk_video::StdVideoH265HrdParameters,
    pub min_spatial_segmentation_idc: u16,
    pub max_bytes_per_pic_denom: u8,
    pub max_bits_per_min_cu_denom: u8,
    pub log2_max_mv_length_horizontal: u8,
    pub log2_max_mv_length_vertical: u8,
    pub flags: crate::external::vk_video::StdVideoH265SpsVuiFlags,
}
impl Default for StdVideoH265SequenceParameterSetVui {
    fn default() -> Self {
        Self { aspect_ratio_idc: Default::default(), sar_width: Default::default(), sar_height: Default::default(), video_format: Default::default(), colour_primaries: Default::default(), transfer_characteristics: Default::default(), matrix_coeffs: Default::default(), chroma_sample_loc_type_top_field: Default::default(), chroma_sample_loc_type_bottom_field: Default::default(), def_disp_win_left_offset: Default::default(), def_disp_win_right_offset: Default::default(), def_disp_win_top_offset: Default::default(), def_disp_win_bottom_offset: Default::default(), vui_num_units_in_tick: Default::default(), vui_time_scale: Default::default(), vui_num_ticks_poc_diff_one_minus1: Default::default(), p_hrd_parameters: std::ptr::null_mut(), min_spatial_segmentation_idc: Default::default(), max_bytes_per_pic_denom: Default::default(), max_bits_per_min_cu_denom: Default::default(), log2_max_mv_length_horizontal: Default::default(), log2_max_mv_length_vertical: Default::default(), flags: Default::default() }
    }
}
impl std::fmt::Debug for StdVideoH265SequenceParameterSetVui {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StdVideoH265SequenceParameterSetVui").field("aspect_ratio_idc", &self.aspect_ratio_idc).field("sar_width", &self.sar_width).field("sar_height", &self.sar_height).field("video_format", &self.video_format).field("colour_primaries", &self.colour_primaries).field("transfer_characteristics", &self.transfer_characteristics).field("matrix_coeffs", &self.matrix_coeffs).field("chroma_sample_loc_type_top_field", &self.chroma_sample_loc_type_top_field).field("chroma_sample_loc_type_bottom_field", &self.chroma_sample_loc_type_bottom_field).field("def_disp_win_left_offset", &self.def_disp_win_left_offset).field("def_disp_win_right_offset", &self.def_disp_win_right_offset).field("def_disp_win_top_offset", &self.def_disp_win_top_offset).field("def_disp_win_bottom_offset", &self.def_disp_win_bottom_offset).field("vui_num_units_in_tick", &self.vui_num_units_in_tick).field("vui_time_scale", &self.vui_time_scale).field("vui_num_ticks_poc_diff_one_minus1", &self.vui_num_ticks_poc_diff_one_minus1).field("p_hrd_parameters", &self.p_hrd_parameters).field("min_spatial_segmentation_idc", &self.min_spatial_segmentation_idc).field("max_bytes_per_pic_denom", &self.max_bytes_per_pic_denom).field("max_bits_per_min_cu_denom", &self.max_bits_per_min_cu_denom).field("log2_max_mv_length_horizontal", &self.log2_max_mv_length_horizontal).field("log2_max_mv_length_vertical", &self.log2_max_mv_length_vertical).field("flags", &self.flags).finish()
    }
}
impl StdVideoH265SequenceParameterSetVui {
    #[inline]
    pub fn into_builder<'a>(self) -> StdVideoH265SequenceParameterSetVuiBuilder<'a> {
        StdVideoH265SequenceParameterSetVuiBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "<s>Vulkan Manual Page</s> · Builder of [`StdVideoH265SequenceParameterSetVui`]"]
#[repr(transparent)]
pub struct StdVideoH265SequenceParameterSetVuiBuilder<'a>(StdVideoH265SequenceParameterSetVui, std::marker::PhantomData<&'a ()>);
impl<'a> StdVideoH265SequenceParameterSetVuiBuilder<'a> {
    #[inline]
    pub fn new() -> StdVideoH265SequenceParameterSetVuiBuilder<'a> {
        StdVideoH265SequenceParameterSetVuiBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn aspect_ratio_idc(mut self, aspect_ratio_idc: u8) -> Self {
        self.0.aspect_ratio_idc = aspect_ratio_idc as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn sar_width(mut self, sar_width: u16) -> Self {
        self.0.sar_width = sar_width as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn sar_height(mut self, sar_height: u16) -> Self {
        self.0.sar_height = sar_height as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn video_format(mut self, video_format: u8) -> Self {
        self.0.video_format = video_format as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn colour_primaries(mut self, colour_primaries: u8) -> Self {
        self.0.colour_primaries = colour_primaries as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn transfer_characteristics(mut self, transfer_characteristics: u8) -> Self {
        self.0.transfer_characteristics = transfer_characteristics as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn matrix_coeffs(mut self, matrix_coeffs: u8) -> Self {
        self.0.matrix_coeffs = matrix_coeffs as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn chroma_sample_loc_type_top_field(mut self, chroma_sample_loc_type_top_field: u8) -> Self {
        self.0.chroma_sample_loc_type_top_field = chroma_sample_loc_type_top_field as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn chroma_sample_loc_type_bottom_field(mut self, chroma_sample_loc_type_bottom_field: u8) -> Self {
        self.0.chroma_sample_loc_type_bottom_field = chroma_sample_loc_type_bottom_field as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn def_disp_win_left_offset(mut self, def_disp_win_left_offset: u16) -> Self {
        self.0.def_disp_win_left_offset = def_disp_win_left_offset as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn def_disp_win_right_offset(mut self, def_disp_win_right_offset: u16) -> Self {
        self.0.def_disp_win_right_offset = def_disp_win_right_offset as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn def_disp_win_top_offset(mut self, def_disp_win_top_offset: u16) -> Self {
        self.0.def_disp_win_top_offset = def_disp_win_top_offset as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn def_disp_win_bottom_offset(mut self, def_disp_win_bottom_offset: u16) -> Self {
        self.0.def_disp_win_bottom_offset = def_disp_win_bottom_offset as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn vui_num_units_in_tick(mut self, vui_num_units_in_tick: u32) -> Self {
        self.0.vui_num_units_in_tick = vui_num_units_in_tick as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn vui_time_scale(mut self, vui_time_scale: u32) -> Self {
        self.0.vui_time_scale = vui_time_scale as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn vui_num_ticks_poc_diff_one_minus1(mut self, vui_num_ticks_poc_diff_one_minus1: u32) -> Self {
        self.0.vui_num_ticks_poc_diff_one_minus1 = vui_num_ticks_poc_diff_one_minus1 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn hrd_parameters(mut self, hrd_parameters: &'a mut crate::external::vk_video::StdVideoH265HrdParameters) -> Self {
        self.0.p_hrd_parameters = hrd_parameters as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn min_spatial_segmentation_idc(mut self, min_spatial_segmentation_idc: u16) -> Self {
        self.0.min_spatial_segmentation_idc = min_spatial_segmentation_idc as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn max_bytes_per_pic_denom(mut self, max_bytes_per_pic_denom: u8) -> Self {
        self.0.max_bytes_per_pic_denom = max_bytes_per_pic_denom as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn max_bits_per_min_cu_denom(mut self, max_bits_per_min_cu_denom: u8) -> Self {
        self.0.max_bits_per_min_cu_denom = max_bits_per_min_cu_denom as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn log2_max_mv_length_horizontal(mut self, log2_max_mv_length_horizontal: u8) -> Self {
        self.0.log2_max_mv_length_horizontal = log2_max_mv_length_horizontal as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn log2_max_mv_length_vertical(mut self, log2_max_mv_length_vertical: u8) -> Self {
        self.0.log2_max_mv_length_vertical = log2_max_mv_length_vertical as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn flags(mut self, flags: crate::external::vk_video::StdVideoH265SpsVuiFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> StdVideoH265SequenceParameterSetVui {
        self.0
    }
}
impl<'a> std::default::Default for StdVideoH265SequenceParameterSetVuiBuilder<'a> {
    fn default() -> StdVideoH265SequenceParameterSetVuiBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for StdVideoH265SequenceParameterSetVuiBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for StdVideoH265SequenceParameterSetVuiBuilder<'a> {
    type Target = StdVideoH265SequenceParameterSetVui;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StdVideoH265SequenceParameterSetVuiBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Structure"]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct StdVideoH265PredictorPaletteEntries {
    pub predictor_palette_entries: [[u16; 128]; 3],
}
impl Default for StdVideoH265PredictorPaletteEntries {
    fn default() -> Self {
        Self { predictor_palette_entries: unsafe { std::mem::zeroed() } }
    }
}
impl std::fmt::Debug for StdVideoH265PredictorPaletteEntries {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StdVideoH265PredictorPaletteEntries").field("predictor_palette_entries", &self.predictor_palette_entries).finish()
    }
}
impl StdVideoH265PredictorPaletteEntries {
    #[inline]
    pub fn into_builder<'a>(self) -> StdVideoH265PredictorPaletteEntriesBuilder<'a> {
        StdVideoH265PredictorPaletteEntriesBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "<s>Vulkan Manual Page</s> · Builder of [`StdVideoH265PredictorPaletteEntries`]"]
#[repr(transparent)]
pub struct StdVideoH265PredictorPaletteEntriesBuilder<'a>(StdVideoH265PredictorPaletteEntries, std::marker::PhantomData<&'a ()>);
impl<'a> StdVideoH265PredictorPaletteEntriesBuilder<'a> {
    #[inline]
    pub fn new() -> StdVideoH265PredictorPaletteEntriesBuilder<'a> {
        StdVideoH265PredictorPaletteEntriesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn predictor_palette_entries(mut self, predictor_palette_entries: [[u16; 128]; 3]) -> Self {
        self.0.predictor_palette_entries = predictor_palette_entries as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> StdVideoH265PredictorPaletteEntries {
        self.0
    }
}
impl<'a> std::default::Default for StdVideoH265PredictorPaletteEntriesBuilder<'a> {
    fn default() -> StdVideoH265PredictorPaletteEntriesBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for StdVideoH265PredictorPaletteEntriesBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for StdVideoH265PredictorPaletteEntriesBuilder<'a> {
    type Target = StdVideoH265PredictorPaletteEntries;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StdVideoH265PredictorPaletteEntriesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Structure"]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct StdVideoH265PpsFlags {
    pub dependent_slice_segments_enabled_flag_and_more_bitfield: u32,
}
impl Default for StdVideoH265PpsFlags {
    fn default() -> Self {
        Self { dependent_slice_segments_enabled_flag_and_more_bitfield: Default::default() }
    }
}
impl std::fmt::Debug for StdVideoH265PpsFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StdVideoH265PpsFlags").field("dependent_slice_segments_enabled_flag_and_more_bitfield", &format!("{:#b}", &self.dependent_slice_segments_enabled_flag_and_more_bitfield)).finish()
    }
}
impl StdVideoH265PpsFlags {
    #[inline]
    pub fn into_builder<'a>(self) -> StdVideoH265PpsFlagsBuilder<'a> {
        StdVideoH265PpsFlagsBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "<s>Vulkan Manual Page</s> · Builder of [`StdVideoH265PpsFlags`]"]
#[repr(transparent)]
pub struct StdVideoH265PpsFlagsBuilder<'a>(StdVideoH265PpsFlags, std::marker::PhantomData<&'a ()>);
impl<'a> StdVideoH265PpsFlagsBuilder<'a> {
    #[inline]
    pub fn new() -> StdVideoH265PpsFlagsBuilder<'a> {
        StdVideoH265PpsFlagsBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn dependent_slice_segments_enabled_flag(mut self, dependent_slice_segments_enabled_flag: u32) -> Self {
        self.0.dependent_slice_segments_enabled_flag_and_more_bitfield = crate::bits_copy!(self.0.dependent_slice_segments_enabled_flag_and_more_bitfield, dependent_slice_segments_enabled_flag, 0usize, 0usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn output_flag_present_flag(mut self, output_flag_present_flag: u32) -> Self {
        self.0.dependent_slice_segments_enabled_flag_and_more_bitfield = crate::bits_copy!(self.0.dependent_slice_segments_enabled_flag_and_more_bitfield, output_flag_present_flag, 1usize, 1usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn sign_data_hiding_enabled_flag(mut self, sign_data_hiding_enabled_flag: u32) -> Self {
        self.0.dependent_slice_segments_enabled_flag_and_more_bitfield = crate::bits_copy!(self.0.dependent_slice_segments_enabled_flag_and_more_bitfield, sign_data_hiding_enabled_flag, 2usize, 2usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn cabac_init_present_flag(mut self, cabac_init_present_flag: u32) -> Self {
        self.0.dependent_slice_segments_enabled_flag_and_more_bitfield = crate::bits_copy!(self.0.dependent_slice_segments_enabled_flag_and_more_bitfield, cabac_init_present_flag, 3usize, 3usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn constrained_intra_pred_flag(mut self, constrained_intra_pred_flag: u32) -> Self {
        self.0.dependent_slice_segments_enabled_flag_and_more_bitfield = crate::bits_copy!(self.0.dependent_slice_segments_enabled_flag_and_more_bitfield, constrained_intra_pred_flag, 4usize, 4usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn transform_skip_enabled_flag(mut self, transform_skip_enabled_flag: u32) -> Self {
        self.0.dependent_slice_segments_enabled_flag_and_more_bitfield = crate::bits_copy!(self.0.dependent_slice_segments_enabled_flag_and_more_bitfield, transform_skip_enabled_flag, 5usize, 5usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn cu_qp_delta_enabled_flag(mut self, cu_qp_delta_enabled_flag: u32) -> Self {
        self.0.dependent_slice_segments_enabled_flag_and_more_bitfield = crate::bits_copy!(self.0.dependent_slice_segments_enabled_flag_and_more_bitfield, cu_qp_delta_enabled_flag, 6usize, 6usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn pps_slice_chroma_qp_offsets_present_flag(mut self, pps_slice_chroma_qp_offsets_present_flag: u32) -> Self {
        self.0.dependent_slice_segments_enabled_flag_and_more_bitfield = crate::bits_copy!(self.0.dependent_slice_segments_enabled_flag_and_more_bitfield, pps_slice_chroma_qp_offsets_present_flag, 7usize, 7usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn weighted_pred_flag(mut self, weighted_pred_flag: u32) -> Self {
        self.0.dependent_slice_segments_enabled_flag_and_more_bitfield = crate::bits_copy!(self.0.dependent_slice_segments_enabled_flag_and_more_bitfield, weighted_pred_flag, 8usize, 8usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn weighted_bipred_flag(mut self, weighted_bipred_flag: u32) -> Self {
        self.0.dependent_slice_segments_enabled_flag_and_more_bitfield = crate::bits_copy!(self.0.dependent_slice_segments_enabled_flag_and_more_bitfield, weighted_bipred_flag, 9usize, 9usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn transquant_bypass_enabled_flag(mut self, transquant_bypass_enabled_flag: u32) -> Self {
        self.0.dependent_slice_segments_enabled_flag_and_more_bitfield = crate::bits_copy!(self.0.dependent_slice_segments_enabled_flag_and_more_bitfield, transquant_bypass_enabled_flag, 10usize, 10usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn tiles_enabled_flag(mut self, tiles_enabled_flag: u32) -> Self {
        self.0.dependent_slice_segments_enabled_flag_and_more_bitfield = crate::bits_copy!(self.0.dependent_slice_segments_enabled_flag_and_more_bitfield, tiles_enabled_flag, 11usize, 11usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn entropy_coding_sync_enabled_flag(mut self, entropy_coding_sync_enabled_flag: u32) -> Self {
        self.0.dependent_slice_segments_enabled_flag_and_more_bitfield = crate::bits_copy!(self.0.dependent_slice_segments_enabled_flag_and_more_bitfield, entropy_coding_sync_enabled_flag, 12usize, 12usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn uniform_spacing_flag(mut self, uniform_spacing_flag: u32) -> Self {
        self.0.dependent_slice_segments_enabled_flag_and_more_bitfield = crate::bits_copy!(self.0.dependent_slice_segments_enabled_flag_and_more_bitfield, uniform_spacing_flag, 13usize, 13usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn loop_filter_across_tiles_enabled_flag(mut self, loop_filter_across_tiles_enabled_flag: u32) -> Self {
        self.0.dependent_slice_segments_enabled_flag_and_more_bitfield = crate::bits_copy!(self.0.dependent_slice_segments_enabled_flag_and_more_bitfield, loop_filter_across_tiles_enabled_flag, 14usize, 14usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn pps_loop_filter_across_slices_enabled_flag(mut self, pps_loop_filter_across_slices_enabled_flag: u32) -> Self {
        self.0.dependent_slice_segments_enabled_flag_and_more_bitfield = crate::bits_copy!(self.0.dependent_slice_segments_enabled_flag_and_more_bitfield, pps_loop_filter_across_slices_enabled_flag, 15usize, 15usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn deblocking_filter_control_present_flag(mut self, deblocking_filter_control_present_flag: u32) -> Self {
        self.0.dependent_slice_segments_enabled_flag_and_more_bitfield = crate::bits_copy!(self.0.dependent_slice_segments_enabled_flag_and_more_bitfield, deblocking_filter_control_present_flag, 16usize, 16usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn deblocking_filter_override_enabled_flag(mut self, deblocking_filter_override_enabled_flag: u32) -> Self {
        self.0.dependent_slice_segments_enabled_flag_and_more_bitfield = crate::bits_copy!(self.0.dependent_slice_segments_enabled_flag_and_more_bitfield, deblocking_filter_override_enabled_flag, 17usize, 17usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn pps_deblocking_filter_disabled_flag(mut self, pps_deblocking_filter_disabled_flag: u32) -> Self {
        self.0.dependent_slice_segments_enabled_flag_and_more_bitfield = crate::bits_copy!(self.0.dependent_slice_segments_enabled_flag_and_more_bitfield, pps_deblocking_filter_disabled_flag, 18usize, 18usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn pps_scaling_list_data_present_flag(mut self, pps_scaling_list_data_present_flag: u32) -> Self {
        self.0.dependent_slice_segments_enabled_flag_and_more_bitfield = crate::bits_copy!(self.0.dependent_slice_segments_enabled_flag_and_more_bitfield, pps_scaling_list_data_present_flag, 19usize, 19usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn lists_modification_present_flag(mut self, lists_modification_present_flag: u32) -> Self {
        self.0.dependent_slice_segments_enabled_flag_and_more_bitfield = crate::bits_copy!(self.0.dependent_slice_segments_enabled_flag_and_more_bitfield, lists_modification_present_flag, 20usize, 20usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn slice_segment_header_extension_present_flag(mut self, slice_segment_header_extension_present_flag: u32) -> Self {
        self.0.dependent_slice_segments_enabled_flag_and_more_bitfield = crate::bits_copy!(self.0.dependent_slice_segments_enabled_flag_and_more_bitfield, slice_segment_header_extension_present_flag, 21usize, 21usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn pps_extension_present_flag(mut self, pps_extension_present_flag: u32) -> Self {
        self.0.dependent_slice_segments_enabled_flag_and_more_bitfield = crate::bits_copy!(self.0.dependent_slice_segments_enabled_flag_and_more_bitfield, pps_extension_present_flag, 22usize, 22usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn cross_component_prediction_enabled_flag(mut self, cross_component_prediction_enabled_flag: u32) -> Self {
        self.0.dependent_slice_segments_enabled_flag_and_more_bitfield = crate::bits_copy!(self.0.dependent_slice_segments_enabled_flag_and_more_bitfield, cross_component_prediction_enabled_flag, 23usize, 23usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn chroma_qp_offset_list_enabled_flag(mut self, chroma_qp_offset_list_enabled_flag: u32) -> Self {
        self.0.dependent_slice_segments_enabled_flag_and_more_bitfield = crate::bits_copy!(self.0.dependent_slice_segments_enabled_flag_and_more_bitfield, chroma_qp_offset_list_enabled_flag, 24usize, 24usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn pps_curr_pic_ref_enabled_flag(mut self, pps_curr_pic_ref_enabled_flag: u32) -> Self {
        self.0.dependent_slice_segments_enabled_flag_and_more_bitfield = crate::bits_copy!(self.0.dependent_slice_segments_enabled_flag_and_more_bitfield, pps_curr_pic_ref_enabled_flag, 25usize, 25usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn residual_adaptive_colour_transform_enabled_flag(mut self, residual_adaptive_colour_transform_enabled_flag: u32) -> Self {
        self.0.dependent_slice_segments_enabled_flag_and_more_bitfield = crate::bits_copy!(self.0.dependent_slice_segments_enabled_flag_and_more_bitfield, residual_adaptive_colour_transform_enabled_flag, 26usize, 26usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn pps_slice_act_qp_offsets_present_flag(mut self, pps_slice_act_qp_offsets_present_flag: u32) -> Self {
        self.0.dependent_slice_segments_enabled_flag_and_more_bitfield = crate::bits_copy!(self.0.dependent_slice_segments_enabled_flag_and_more_bitfield, pps_slice_act_qp_offsets_present_flag, 27usize, 27usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn pps_palette_predictor_initializer_present_flag(mut self, pps_palette_predictor_initializer_present_flag: u32) -> Self {
        self.0.dependent_slice_segments_enabled_flag_and_more_bitfield = crate::bits_copy!(self.0.dependent_slice_segments_enabled_flag_and_more_bitfield, pps_palette_predictor_initializer_present_flag, 28usize, 28usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn monochrome_palette_flag(mut self, monochrome_palette_flag: u32) -> Self {
        self.0.dependent_slice_segments_enabled_flag_and_more_bitfield = crate::bits_copy!(self.0.dependent_slice_segments_enabled_flag_and_more_bitfield, monochrome_palette_flag, 29usize, 29usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn pps_range_extension_flag(mut self, pps_range_extension_flag: u32) -> Self {
        self.0.dependent_slice_segments_enabled_flag_and_more_bitfield = crate::bits_copy!(self.0.dependent_slice_segments_enabled_flag_and_more_bitfield, pps_range_extension_flag, 30usize, 30usize);
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> StdVideoH265PpsFlags {
        self.0
    }
}
impl<'a> std::default::Default for StdVideoH265PpsFlagsBuilder<'a> {
    fn default() -> StdVideoH265PpsFlagsBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for StdVideoH265PpsFlagsBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for StdVideoH265PpsFlagsBuilder<'a> {
    type Target = StdVideoH265PpsFlags;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StdVideoH265PpsFlagsBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Structure"]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct StdVideoH265SubLayerHrdParameters {
    pub bit_rate_value_minus1: [u32; 32],
    pub cpb_size_value_minus1: [u32; 32],
    pub cpb_size_du_value_minus1: [u32; 32],
    pub bit_rate_du_value_minus1: [u32; 32],
    pub cbr_flag: u32,
}
impl Default for StdVideoH265SubLayerHrdParameters {
    fn default() -> Self {
        Self { bit_rate_value_minus1: unsafe { std::mem::zeroed() }, cpb_size_value_minus1: unsafe { std::mem::zeroed() }, cpb_size_du_value_minus1: unsafe { std::mem::zeroed() }, bit_rate_du_value_minus1: unsafe { std::mem::zeroed() }, cbr_flag: Default::default() }
    }
}
impl std::fmt::Debug for StdVideoH265SubLayerHrdParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StdVideoH265SubLayerHrdParameters").field("bit_rate_value_minus1", &self.bit_rate_value_minus1).field("cpb_size_value_minus1", &self.cpb_size_value_minus1).field("cpb_size_du_value_minus1", &self.cpb_size_du_value_minus1).field("bit_rate_du_value_minus1", &self.bit_rate_du_value_minus1).field("cbr_flag", &self.cbr_flag).finish()
    }
}
impl StdVideoH265SubLayerHrdParameters {
    #[inline]
    pub fn into_builder<'a>(self) -> StdVideoH265SubLayerHrdParametersBuilder<'a> {
        StdVideoH265SubLayerHrdParametersBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "<s>Vulkan Manual Page</s> · Builder of [`StdVideoH265SubLayerHrdParameters`]"]
#[repr(transparent)]
pub struct StdVideoH265SubLayerHrdParametersBuilder<'a>(StdVideoH265SubLayerHrdParameters, std::marker::PhantomData<&'a ()>);
impl<'a> StdVideoH265SubLayerHrdParametersBuilder<'a> {
    #[inline]
    pub fn new() -> StdVideoH265SubLayerHrdParametersBuilder<'a> {
        StdVideoH265SubLayerHrdParametersBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn bit_rate_value_minus1(mut self, bit_rate_value_minus1: [u32; 32]) -> Self {
        self.0.bit_rate_value_minus1 = bit_rate_value_minus1 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn cpb_size_value_minus1(mut self, cpb_size_value_minus1: [u32; 32]) -> Self {
        self.0.cpb_size_value_minus1 = cpb_size_value_minus1 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn cpb_size_du_value_minus1(mut self, cpb_size_du_value_minus1: [u32; 32]) -> Self {
        self.0.cpb_size_du_value_minus1 = cpb_size_du_value_minus1 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn bit_rate_du_value_minus1(mut self, bit_rate_du_value_minus1: [u32; 32]) -> Self {
        self.0.bit_rate_du_value_minus1 = bit_rate_du_value_minus1 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn cbr_flag(mut self, cbr_flag: u32) -> Self {
        self.0.cbr_flag = cbr_flag as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> StdVideoH265SubLayerHrdParameters {
        self.0
    }
}
impl<'a> std::default::Default for StdVideoH265SubLayerHrdParametersBuilder<'a> {
    fn default() -> StdVideoH265SubLayerHrdParametersBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for StdVideoH265SubLayerHrdParametersBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for StdVideoH265SubLayerHrdParametersBuilder<'a> {
    type Target = StdVideoH265SubLayerHrdParameters;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StdVideoH265SubLayerHrdParametersBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Structure"]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct StdVideoH265HrdFlags {
    pub nal_hrd_parameters_present_flag_and_more_bitfield: u32,
}
impl Default for StdVideoH265HrdFlags {
    fn default() -> Self {
        Self { nal_hrd_parameters_present_flag_and_more_bitfield: Default::default() }
    }
}
impl std::fmt::Debug for StdVideoH265HrdFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StdVideoH265HrdFlags").field("nal_hrd_parameters_present_flag_and_more_bitfield", &format!("{:#b}", &self.nal_hrd_parameters_present_flag_and_more_bitfield)).finish()
    }
}
impl StdVideoH265HrdFlags {
    #[inline]
    pub fn into_builder<'a>(self) -> StdVideoH265HrdFlagsBuilder<'a> {
        StdVideoH265HrdFlagsBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "<s>Vulkan Manual Page</s> · Builder of [`StdVideoH265HrdFlags`]"]
#[repr(transparent)]
pub struct StdVideoH265HrdFlagsBuilder<'a>(StdVideoH265HrdFlags, std::marker::PhantomData<&'a ()>);
impl<'a> StdVideoH265HrdFlagsBuilder<'a> {
    #[inline]
    pub fn new() -> StdVideoH265HrdFlagsBuilder<'a> {
        StdVideoH265HrdFlagsBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn nal_hrd_parameters_present_flag(mut self, nal_hrd_parameters_present_flag: u32) -> Self {
        self.0.nal_hrd_parameters_present_flag_and_more_bitfield = crate::bits_copy!(self.0.nal_hrd_parameters_present_flag_and_more_bitfield, nal_hrd_parameters_present_flag, 0usize, 0usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn vcl_hrd_parameters_present_flag(mut self, vcl_hrd_parameters_present_flag: u32) -> Self {
        self.0.nal_hrd_parameters_present_flag_and_more_bitfield = crate::bits_copy!(self.0.nal_hrd_parameters_present_flag_and_more_bitfield, vcl_hrd_parameters_present_flag, 1usize, 1usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn sub_pic_hrd_params_present_flag(mut self, sub_pic_hrd_params_present_flag: u32) -> Self {
        self.0.nal_hrd_parameters_present_flag_and_more_bitfield = crate::bits_copy!(self.0.nal_hrd_parameters_present_flag_and_more_bitfield, sub_pic_hrd_params_present_flag, 2usize, 2usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn sub_pic_cpb_params_in_pic_timing_sei_flag(mut self, sub_pic_cpb_params_in_pic_timing_sei_flag: u32) -> Self {
        self.0.nal_hrd_parameters_present_flag_and_more_bitfield = crate::bits_copy!(self.0.nal_hrd_parameters_present_flag_and_more_bitfield, sub_pic_cpb_params_in_pic_timing_sei_flag, 3usize, 3usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn fixed_pic_rate_general_flag(mut self, fixed_pic_rate_general_flag: u32) -> Self {
        self.0.nal_hrd_parameters_present_flag_and_more_bitfield = crate::bits_copy!(self.0.nal_hrd_parameters_present_flag_and_more_bitfield, fixed_pic_rate_general_flag, 4usize, 11usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn fixed_pic_rate_within_cvs_flag(mut self, fixed_pic_rate_within_cvs_flag: u32) -> Self {
        self.0.nal_hrd_parameters_present_flag_and_more_bitfield = crate::bits_copy!(self.0.nal_hrd_parameters_present_flag_and_more_bitfield, fixed_pic_rate_within_cvs_flag, 12usize, 19usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn low_delay_hrd_flag(mut self, low_delay_hrd_flag: u32) -> Self {
        self.0.nal_hrd_parameters_present_flag_and_more_bitfield = crate::bits_copy!(self.0.nal_hrd_parameters_present_flag_and_more_bitfield, low_delay_hrd_flag, 20usize, 27usize);
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> StdVideoH265HrdFlags {
        self.0
    }
}
impl<'a> std::default::Default for StdVideoH265HrdFlagsBuilder<'a> {
    fn default() -> StdVideoH265HrdFlagsBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for StdVideoH265HrdFlagsBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for StdVideoH265HrdFlagsBuilder<'a> {
    type Target = StdVideoH265HrdFlags;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StdVideoH265HrdFlagsBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Structure"]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct StdVideoH265SpsVuiFlags {
    pub aspect_ratio_info_present_flag_and_more_bitfield: u32,
}
impl Default for StdVideoH265SpsVuiFlags {
    fn default() -> Self {
        Self { aspect_ratio_info_present_flag_and_more_bitfield: Default::default() }
    }
}
impl std::fmt::Debug for StdVideoH265SpsVuiFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StdVideoH265SpsVuiFlags").field("aspect_ratio_info_present_flag_and_more_bitfield", &format!("{:#b}", &self.aspect_ratio_info_present_flag_and_more_bitfield)).finish()
    }
}
impl StdVideoH265SpsVuiFlags {
    #[inline]
    pub fn into_builder<'a>(self) -> StdVideoH265SpsVuiFlagsBuilder<'a> {
        StdVideoH265SpsVuiFlagsBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "<s>Vulkan Manual Page</s> · Builder of [`StdVideoH265SpsVuiFlags`]"]
#[repr(transparent)]
pub struct StdVideoH265SpsVuiFlagsBuilder<'a>(StdVideoH265SpsVuiFlags, std::marker::PhantomData<&'a ()>);
impl<'a> StdVideoH265SpsVuiFlagsBuilder<'a> {
    #[inline]
    pub fn new() -> StdVideoH265SpsVuiFlagsBuilder<'a> {
        StdVideoH265SpsVuiFlagsBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn aspect_ratio_info_present_flag(mut self, aspect_ratio_info_present_flag: u32) -> Self {
        self.0.aspect_ratio_info_present_flag_and_more_bitfield = crate::bits_copy!(self.0.aspect_ratio_info_present_flag_and_more_bitfield, aspect_ratio_info_present_flag, 0usize, 0usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn overscan_info_present_flag(mut self, overscan_info_present_flag: u32) -> Self {
        self.0.aspect_ratio_info_present_flag_and_more_bitfield = crate::bits_copy!(self.0.aspect_ratio_info_present_flag_and_more_bitfield, overscan_info_present_flag, 1usize, 1usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn overscan_appropriate_flag(mut self, overscan_appropriate_flag: u32) -> Self {
        self.0.aspect_ratio_info_present_flag_and_more_bitfield = crate::bits_copy!(self.0.aspect_ratio_info_present_flag_and_more_bitfield, overscan_appropriate_flag, 2usize, 2usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn video_signal_type_present_flag(mut self, video_signal_type_present_flag: u32) -> Self {
        self.0.aspect_ratio_info_present_flag_and_more_bitfield = crate::bits_copy!(self.0.aspect_ratio_info_present_flag_and_more_bitfield, video_signal_type_present_flag, 3usize, 3usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn video_full_range_flag(mut self, video_full_range_flag: u32) -> Self {
        self.0.aspect_ratio_info_present_flag_and_more_bitfield = crate::bits_copy!(self.0.aspect_ratio_info_present_flag_and_more_bitfield, video_full_range_flag, 4usize, 4usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn colour_description_present_flag(mut self, colour_description_present_flag: u32) -> Self {
        self.0.aspect_ratio_info_present_flag_and_more_bitfield = crate::bits_copy!(self.0.aspect_ratio_info_present_flag_and_more_bitfield, colour_description_present_flag, 5usize, 5usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn chroma_loc_info_present_flag(mut self, chroma_loc_info_present_flag: u32) -> Self {
        self.0.aspect_ratio_info_present_flag_and_more_bitfield = crate::bits_copy!(self.0.aspect_ratio_info_present_flag_and_more_bitfield, chroma_loc_info_present_flag, 6usize, 6usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn neutral_chroma_indication_flag(mut self, neutral_chroma_indication_flag: u32) -> Self {
        self.0.aspect_ratio_info_present_flag_and_more_bitfield = crate::bits_copy!(self.0.aspect_ratio_info_present_flag_and_more_bitfield, neutral_chroma_indication_flag, 7usize, 7usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn field_seq_flag(mut self, field_seq_flag: u32) -> Self {
        self.0.aspect_ratio_info_present_flag_and_more_bitfield = crate::bits_copy!(self.0.aspect_ratio_info_present_flag_and_more_bitfield, field_seq_flag, 8usize, 8usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn frame_field_info_present_flag(mut self, frame_field_info_present_flag: u32) -> Self {
        self.0.aspect_ratio_info_present_flag_and_more_bitfield = crate::bits_copy!(self.0.aspect_ratio_info_present_flag_and_more_bitfield, frame_field_info_present_flag, 9usize, 9usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn default_display_window_flag(mut self, default_display_window_flag: u32) -> Self {
        self.0.aspect_ratio_info_present_flag_and_more_bitfield = crate::bits_copy!(self.0.aspect_ratio_info_present_flag_and_more_bitfield, default_display_window_flag, 10usize, 10usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn vui_timing_info_present_flag(mut self, vui_timing_info_present_flag: u32) -> Self {
        self.0.aspect_ratio_info_present_flag_and_more_bitfield = crate::bits_copy!(self.0.aspect_ratio_info_present_flag_and_more_bitfield, vui_timing_info_present_flag, 11usize, 11usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn vui_poc_proportional_to_timing_flag(mut self, vui_poc_proportional_to_timing_flag: u32) -> Self {
        self.0.aspect_ratio_info_present_flag_and_more_bitfield = crate::bits_copy!(self.0.aspect_ratio_info_present_flag_and_more_bitfield, vui_poc_proportional_to_timing_flag, 12usize, 12usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn vui_hrd_parameters_present_flag(mut self, vui_hrd_parameters_present_flag: u32) -> Self {
        self.0.aspect_ratio_info_present_flag_and_more_bitfield = crate::bits_copy!(self.0.aspect_ratio_info_present_flag_and_more_bitfield, vui_hrd_parameters_present_flag, 13usize, 13usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn bitstream_restriction_flag(mut self, bitstream_restriction_flag: u32) -> Self {
        self.0.aspect_ratio_info_present_flag_and_more_bitfield = crate::bits_copy!(self.0.aspect_ratio_info_present_flag_and_more_bitfield, bitstream_restriction_flag, 14usize, 14usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn tiles_fixed_structure_flag(mut self, tiles_fixed_structure_flag: u32) -> Self {
        self.0.aspect_ratio_info_present_flag_and_more_bitfield = crate::bits_copy!(self.0.aspect_ratio_info_present_flag_and_more_bitfield, tiles_fixed_structure_flag, 15usize, 15usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn motion_vectors_over_pic_boundaries_flag(mut self, motion_vectors_over_pic_boundaries_flag: u32) -> Self {
        self.0.aspect_ratio_info_present_flag_and_more_bitfield = crate::bits_copy!(self.0.aspect_ratio_info_present_flag_and_more_bitfield, motion_vectors_over_pic_boundaries_flag, 16usize, 16usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn restricted_ref_pic_lists_flag(mut self, restricted_ref_pic_lists_flag: u32) -> Self {
        self.0.aspect_ratio_info_present_flag_and_more_bitfield = crate::bits_copy!(self.0.aspect_ratio_info_present_flag_and_more_bitfield, restricted_ref_pic_lists_flag, 17usize, 17usize);
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> StdVideoH265SpsVuiFlags {
        self.0
    }
}
impl<'a> std::default::Default for StdVideoH265SpsVuiFlagsBuilder<'a> {
    fn default() -> StdVideoH265SpsVuiFlagsBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for StdVideoH265SpsVuiFlagsBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for StdVideoH265SpsVuiFlagsBuilder<'a> {
    type Target = StdVideoH265SpsVuiFlags;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StdVideoH265SpsVuiFlagsBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Structure"]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct StdVideoDecodeH265PictureInfo {
    pub vps_video_parameter_set_id: u8,
    pub sps_seq_parameter_set_id: u8,
    pub pps_pic_parameter_set_id: u8,
    pub num_short_term_ref_pic_sets: u8,
    pub pic_order_cnt_val: i32,
    pub num_bits_for_st_ref_pic_set_in_slice: u16,
    pub num_delta_pocs_of_ref_rps_idx: u8,
    pub ref_pic_set_st_curr_before: [u8; 8],
    pub ref_pic_set_st_curr_after: [u8; 8],
    pub ref_pic_set_lt_curr: [u8; 8],
    pub flags: crate::external::vk_video::StdVideoDecodeH265PictureInfoFlags,
}
impl Default for StdVideoDecodeH265PictureInfo {
    fn default() -> Self {
        Self { vps_video_parameter_set_id: Default::default(), sps_seq_parameter_set_id: Default::default(), pps_pic_parameter_set_id: Default::default(), num_short_term_ref_pic_sets: Default::default(), pic_order_cnt_val: Default::default(), num_bits_for_st_ref_pic_set_in_slice: Default::default(), num_delta_pocs_of_ref_rps_idx: Default::default(), ref_pic_set_st_curr_before: unsafe { std::mem::zeroed() }, ref_pic_set_st_curr_after: unsafe { std::mem::zeroed() }, ref_pic_set_lt_curr: unsafe { std::mem::zeroed() }, flags: Default::default() }
    }
}
impl std::fmt::Debug for StdVideoDecodeH265PictureInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StdVideoDecodeH265PictureInfo").field("vps_video_parameter_set_id", &self.vps_video_parameter_set_id).field("sps_seq_parameter_set_id", &self.sps_seq_parameter_set_id).field("pps_pic_parameter_set_id", &self.pps_pic_parameter_set_id).field("num_short_term_ref_pic_sets", &self.num_short_term_ref_pic_sets).field("pic_order_cnt_val", &self.pic_order_cnt_val).field("num_bits_for_st_ref_pic_set_in_slice", &self.num_bits_for_st_ref_pic_set_in_slice).field("num_delta_pocs_of_ref_rps_idx", &self.num_delta_pocs_of_ref_rps_idx).field("ref_pic_set_st_curr_before", &self.ref_pic_set_st_curr_before).field("ref_pic_set_st_curr_after", &self.ref_pic_set_st_curr_after).field("ref_pic_set_lt_curr", &self.ref_pic_set_lt_curr).field("flags", &self.flags).finish()
    }
}
impl StdVideoDecodeH265PictureInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> StdVideoDecodeH265PictureInfoBuilder<'a> {
        StdVideoDecodeH265PictureInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "<s>Vulkan Manual Page</s> · Builder of [`StdVideoDecodeH265PictureInfo`]"]
#[repr(transparent)]
pub struct StdVideoDecodeH265PictureInfoBuilder<'a>(StdVideoDecodeH265PictureInfo, std::marker::PhantomData<&'a ()>);
impl<'a> StdVideoDecodeH265PictureInfoBuilder<'a> {
    #[inline]
    pub fn new() -> StdVideoDecodeH265PictureInfoBuilder<'a> {
        StdVideoDecodeH265PictureInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn vps_video_parameter_set_id(mut self, vps_video_parameter_set_id: u8) -> Self {
        self.0.vps_video_parameter_set_id = vps_video_parameter_set_id as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn sps_seq_parameter_set_id(mut self, sps_seq_parameter_set_id: u8) -> Self {
        self.0.sps_seq_parameter_set_id = sps_seq_parameter_set_id as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn pps_pic_parameter_set_id(mut self, pps_pic_parameter_set_id: u8) -> Self {
        self.0.pps_pic_parameter_set_id = pps_pic_parameter_set_id as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn num_short_term_ref_pic_sets(mut self, num_short_term_ref_pic_sets: u8) -> Self {
        self.0.num_short_term_ref_pic_sets = num_short_term_ref_pic_sets as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn pic_order_cnt_val(mut self, pic_order_cnt_val: i32) -> Self {
        self.0.pic_order_cnt_val = pic_order_cnt_val as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn num_bits_for_st_ref_pic_set_in_slice(mut self, num_bits_for_st_ref_pic_set_in_slice: u16) -> Self {
        self.0.num_bits_for_st_ref_pic_set_in_slice = num_bits_for_st_ref_pic_set_in_slice as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn num_delta_pocs_of_ref_rps_idx(mut self, num_delta_pocs_of_ref_rps_idx: u8) -> Self {
        self.0.num_delta_pocs_of_ref_rps_idx = num_delta_pocs_of_ref_rps_idx as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn ref_pic_set_st_curr_before(mut self, ref_pic_set_st_curr_before: [u8; 8]) -> Self {
        self.0.ref_pic_set_st_curr_before = ref_pic_set_st_curr_before as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn ref_pic_set_st_curr_after(mut self, ref_pic_set_st_curr_after: [u8; 8]) -> Self {
        self.0.ref_pic_set_st_curr_after = ref_pic_set_st_curr_after as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn ref_pic_set_lt_curr(mut self, ref_pic_set_lt_curr: [u8; 8]) -> Self {
        self.0.ref_pic_set_lt_curr = ref_pic_set_lt_curr as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn flags(mut self, flags: crate::external::vk_video::StdVideoDecodeH265PictureInfoFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> StdVideoDecodeH265PictureInfo {
        self.0
    }
}
impl<'a> std::default::Default for StdVideoDecodeH265PictureInfoBuilder<'a> {
    fn default() -> StdVideoDecodeH265PictureInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for StdVideoDecodeH265PictureInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for StdVideoDecodeH265PictureInfoBuilder<'a> {
    type Target = StdVideoDecodeH265PictureInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StdVideoDecodeH265PictureInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Structure"]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct StdVideoDecodeH265ReferenceInfo {
    pub pic_order_cnt_val: i32,
    pub flags: crate::external::vk_video::StdVideoDecodeH265ReferenceInfoFlags,
}
impl Default for StdVideoDecodeH265ReferenceInfo {
    fn default() -> Self {
        Self { pic_order_cnt_val: Default::default(), flags: Default::default() }
    }
}
impl std::fmt::Debug for StdVideoDecodeH265ReferenceInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StdVideoDecodeH265ReferenceInfo").field("pic_order_cnt_val", &self.pic_order_cnt_val).field("flags", &self.flags).finish()
    }
}
impl StdVideoDecodeH265ReferenceInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> StdVideoDecodeH265ReferenceInfoBuilder<'a> {
        StdVideoDecodeH265ReferenceInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "<s>Vulkan Manual Page</s> · Builder of [`StdVideoDecodeH265ReferenceInfo`]"]
#[repr(transparent)]
pub struct StdVideoDecodeH265ReferenceInfoBuilder<'a>(StdVideoDecodeH265ReferenceInfo, std::marker::PhantomData<&'a ()>);
impl<'a> StdVideoDecodeH265ReferenceInfoBuilder<'a> {
    #[inline]
    pub fn new() -> StdVideoDecodeH265ReferenceInfoBuilder<'a> {
        StdVideoDecodeH265ReferenceInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn pic_order_cnt_val(mut self, pic_order_cnt_val: i32) -> Self {
        self.0.pic_order_cnt_val = pic_order_cnt_val as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn flags(mut self, flags: crate::external::vk_video::StdVideoDecodeH265ReferenceInfoFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> StdVideoDecodeH265ReferenceInfo {
        self.0
    }
}
impl<'a> std::default::Default for StdVideoDecodeH265ReferenceInfoBuilder<'a> {
    fn default() -> StdVideoDecodeH265ReferenceInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for StdVideoDecodeH265ReferenceInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for StdVideoDecodeH265ReferenceInfoBuilder<'a> {
    type Target = StdVideoDecodeH265ReferenceInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StdVideoDecodeH265ReferenceInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Structure"]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct StdVideoDecodeH265PictureInfoFlags {
    pub irap_pic_flag_and_more_bitfield: u32,
}
impl Default for StdVideoDecodeH265PictureInfoFlags {
    fn default() -> Self {
        Self { irap_pic_flag_and_more_bitfield: Default::default() }
    }
}
impl std::fmt::Debug for StdVideoDecodeH265PictureInfoFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StdVideoDecodeH265PictureInfoFlags").field("irap_pic_flag_and_more_bitfield", &format!("{:#b}", &self.irap_pic_flag_and_more_bitfield)).finish()
    }
}
impl StdVideoDecodeH265PictureInfoFlags {
    #[inline]
    pub fn into_builder<'a>(self) -> StdVideoDecodeH265PictureInfoFlagsBuilder<'a> {
        StdVideoDecodeH265PictureInfoFlagsBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "<s>Vulkan Manual Page</s> · Builder of [`StdVideoDecodeH265PictureInfoFlags`]"]
#[repr(transparent)]
pub struct StdVideoDecodeH265PictureInfoFlagsBuilder<'a>(StdVideoDecodeH265PictureInfoFlags, std::marker::PhantomData<&'a ()>);
impl<'a> StdVideoDecodeH265PictureInfoFlagsBuilder<'a> {
    #[inline]
    pub fn new() -> StdVideoDecodeH265PictureInfoFlagsBuilder<'a> {
        StdVideoDecodeH265PictureInfoFlagsBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn irap_pic_flag(mut self, irap_pic_flag: u32) -> Self {
        self.0.irap_pic_flag_and_more_bitfield = crate::bits_copy!(self.0.irap_pic_flag_and_more_bitfield, irap_pic_flag, 0usize, 0usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn idr_pic_flag(mut self, idr_pic_flag: u32) -> Self {
        self.0.irap_pic_flag_and_more_bitfield = crate::bits_copy!(self.0.irap_pic_flag_and_more_bitfield, idr_pic_flag, 1usize, 1usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn is_reference(mut self, is_reference: u32) -> Self {
        self.0.irap_pic_flag_and_more_bitfield = crate::bits_copy!(self.0.irap_pic_flag_and_more_bitfield, is_reference, 2usize, 2usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn short_term_ref_pic_set_sps_flag(mut self, short_term_ref_pic_set_sps_flag: u32) -> Self {
        self.0.irap_pic_flag_and_more_bitfield = crate::bits_copy!(self.0.irap_pic_flag_and_more_bitfield, short_term_ref_pic_set_sps_flag, 3usize, 3usize);
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> StdVideoDecodeH265PictureInfoFlags {
        self.0
    }
}
impl<'a> std::default::Default for StdVideoDecodeH265PictureInfoFlagsBuilder<'a> {
    fn default() -> StdVideoDecodeH265PictureInfoFlagsBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for StdVideoDecodeH265PictureInfoFlagsBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for StdVideoDecodeH265PictureInfoFlagsBuilder<'a> {
    type Target = StdVideoDecodeH265PictureInfoFlags;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StdVideoDecodeH265PictureInfoFlagsBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Structure"]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct StdVideoDecodeH265ReferenceInfoFlags {
    pub is_long_term_and_is_non_existing: u32,
}
impl Default for StdVideoDecodeH265ReferenceInfoFlags {
    fn default() -> Self {
        Self { is_long_term_and_is_non_existing: Default::default() }
    }
}
impl std::fmt::Debug for StdVideoDecodeH265ReferenceInfoFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StdVideoDecodeH265ReferenceInfoFlags").field("is_long_term_and_is_non_existing", &format!("{:#b}", &self.is_long_term_and_is_non_existing)).finish()
    }
}
impl StdVideoDecodeH265ReferenceInfoFlags {
    #[inline]
    pub fn into_builder<'a>(self) -> StdVideoDecodeH265ReferenceInfoFlagsBuilder<'a> {
        StdVideoDecodeH265ReferenceInfoFlagsBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "<s>Vulkan Manual Page</s> · Builder of [`StdVideoDecodeH265ReferenceInfoFlags`]"]
#[repr(transparent)]
pub struct StdVideoDecodeH265ReferenceInfoFlagsBuilder<'a>(StdVideoDecodeH265ReferenceInfoFlags, std::marker::PhantomData<&'a ()>);
impl<'a> StdVideoDecodeH265ReferenceInfoFlagsBuilder<'a> {
    #[inline]
    pub fn new() -> StdVideoDecodeH265ReferenceInfoFlagsBuilder<'a> {
        StdVideoDecodeH265ReferenceInfoFlagsBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn is_long_term(mut self, is_long_term: u32) -> Self {
        self.0.is_long_term_and_is_non_existing = crate::bits_copy!(self.0.is_long_term_and_is_non_existing, is_long_term, 0usize, 0usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn is_non_existing(mut self, is_non_existing: u32) -> Self {
        self.0.is_long_term_and_is_non_existing = crate::bits_copy!(self.0.is_long_term_and_is_non_existing, is_non_existing, 1usize, 1usize);
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> StdVideoDecodeH265ReferenceInfoFlags {
        self.0
    }
}
impl<'a> std::default::Default for StdVideoDecodeH265ReferenceInfoFlagsBuilder<'a> {
    fn default() -> StdVideoDecodeH265ReferenceInfoFlagsBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for StdVideoDecodeH265ReferenceInfoFlagsBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for StdVideoDecodeH265ReferenceInfoFlagsBuilder<'a> {
    type Target = StdVideoDecodeH265ReferenceInfoFlags;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StdVideoDecodeH265ReferenceInfoFlagsBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StdVideoEncodeH264SliceHeader {
    pub flags: crate::external::vk_video::StdVideoEncodeH264SliceHeaderFlags,
    pub slice_type: crate::external::vk_video::StdVideoH264SliceType,
    pub seq_parameter_set_id: u8,
    pub pic_parameter_set_id: u8,
    pub idr_pic_id: u16,
    pub num_ref_idx_l0_active_minus1: u8,
    pub num_ref_idx_l1_active_minus1: u8,
    pub cabac_init_idc: crate::external::vk_video::StdVideoH264CabacInitIdc,
    pub disable_deblocking_filter_idc: crate::external::vk_video::StdVideoH264DisableDeblockingFilterIdc,
    pub slice_alpha_c0_offset_div2: i8,
    pub slice_beta_offset_div2: i8,
    pub p_mem_mgmt_ctrl_operations: *mut crate::external::vk_video::StdVideoEncodeH264RefMemMgmtCtrlOperations,
}
impl Default for StdVideoEncodeH264SliceHeader {
    fn default() -> Self {
        Self { flags: Default::default(), slice_type: Default::default(), seq_parameter_set_id: Default::default(), pic_parameter_set_id: Default::default(), idr_pic_id: Default::default(), num_ref_idx_l0_active_minus1: Default::default(), num_ref_idx_l1_active_minus1: Default::default(), cabac_init_idc: Default::default(), disable_deblocking_filter_idc: Default::default(), slice_alpha_c0_offset_div2: Default::default(), slice_beta_offset_div2: Default::default(), p_mem_mgmt_ctrl_operations: std::ptr::null_mut() }
    }
}
impl std::fmt::Debug for StdVideoEncodeH264SliceHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StdVideoEncodeH264SliceHeader").field("flags", &self.flags).field("slice_type", &self.slice_type).field("seq_parameter_set_id", &self.seq_parameter_set_id).field("pic_parameter_set_id", &self.pic_parameter_set_id).field("idr_pic_id", &self.idr_pic_id).field("num_ref_idx_l0_active_minus1", &self.num_ref_idx_l0_active_minus1).field("num_ref_idx_l1_active_minus1", &self.num_ref_idx_l1_active_minus1).field("cabac_init_idc", &self.cabac_init_idc).field("disable_deblocking_filter_idc", &self.disable_deblocking_filter_idc).field("slice_alpha_c0_offset_div2", &self.slice_alpha_c0_offset_div2).field("slice_beta_offset_div2", &self.slice_beta_offset_div2).field("p_mem_mgmt_ctrl_operations", &self.p_mem_mgmt_ctrl_operations).finish()
    }
}
impl StdVideoEncodeH264SliceHeader {
    #[inline]
    pub fn into_builder<'a>(self) -> StdVideoEncodeH264SliceHeaderBuilder<'a> {
        StdVideoEncodeH264SliceHeaderBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "<s>Vulkan Manual Page</s> · Builder of [`StdVideoEncodeH264SliceHeader`]"]
#[repr(transparent)]
pub struct StdVideoEncodeH264SliceHeaderBuilder<'a>(StdVideoEncodeH264SliceHeader, std::marker::PhantomData<&'a ()>);
impl<'a> StdVideoEncodeH264SliceHeaderBuilder<'a> {
    #[inline]
    pub fn new() -> StdVideoEncodeH264SliceHeaderBuilder<'a> {
        StdVideoEncodeH264SliceHeaderBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn flags(mut self, flags: crate::external::vk_video::StdVideoEncodeH264SliceHeaderFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn slice_type(mut self, slice_type: crate::external::vk_video::StdVideoH264SliceType) -> Self {
        self.0.slice_type = slice_type as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn seq_parameter_set_id(mut self, seq_parameter_set_id: u8) -> Self {
        self.0.seq_parameter_set_id = seq_parameter_set_id as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn pic_parameter_set_id(mut self, pic_parameter_set_id: u8) -> Self {
        self.0.pic_parameter_set_id = pic_parameter_set_id as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn idr_pic_id(mut self, idr_pic_id: u16) -> Self {
        self.0.idr_pic_id = idr_pic_id as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn num_ref_idx_l0_active_minus1(mut self, num_ref_idx_l0_active_minus1: u8) -> Self {
        self.0.num_ref_idx_l0_active_minus1 = num_ref_idx_l0_active_minus1 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn num_ref_idx_l1_active_minus1(mut self, num_ref_idx_l1_active_minus1: u8) -> Self {
        self.0.num_ref_idx_l1_active_minus1 = num_ref_idx_l1_active_minus1 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn cabac_init_idc(mut self, cabac_init_idc: crate::external::vk_video::StdVideoH264CabacInitIdc) -> Self {
        self.0.cabac_init_idc = cabac_init_idc as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn disable_deblocking_filter_idc(mut self, disable_deblocking_filter_idc: crate::external::vk_video::StdVideoH264DisableDeblockingFilterIdc) -> Self {
        self.0.disable_deblocking_filter_idc = disable_deblocking_filter_idc as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn slice_alpha_c0_offset_div2(mut self, slice_alpha_c0_offset_div2: i8) -> Self {
        self.0.slice_alpha_c0_offset_div2 = slice_alpha_c0_offset_div2 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn slice_beta_offset_div2(mut self, slice_beta_offset_div2: i8) -> Self {
        self.0.slice_beta_offset_div2 = slice_beta_offset_div2 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn mem_mgmt_ctrl_operations(mut self, mem_mgmt_ctrl_operations: &'a mut crate::external::vk_video::StdVideoEncodeH264RefMemMgmtCtrlOperations) -> Self {
        self.0.p_mem_mgmt_ctrl_operations = mem_mgmt_ctrl_operations as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> StdVideoEncodeH264SliceHeader {
        self.0
    }
}
impl<'a> std::default::Default for StdVideoEncodeH264SliceHeaderBuilder<'a> {
    fn default() -> StdVideoEncodeH264SliceHeaderBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for StdVideoEncodeH264SliceHeaderBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for StdVideoEncodeH264SliceHeaderBuilder<'a> {
    type Target = StdVideoEncodeH264SliceHeader;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StdVideoEncodeH264SliceHeaderBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Structure"]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct StdVideoEncodeH264PictureInfo {
    pub flags: crate::external::vk_video::StdVideoEncodeH264PictureInfoFlags,
    pub picture_type: crate::external::vk_video::StdVideoH264PictureType,
    pub frame_num: u32,
    pub picture_order_count: u32,
    pub long_term_pic_num: u16,
    pub long_term_frame_idx: u16,
}
impl Default for StdVideoEncodeH264PictureInfo {
    fn default() -> Self {
        Self { flags: Default::default(), picture_type: Default::default(), frame_num: Default::default(), picture_order_count: Default::default(), long_term_pic_num: Default::default(), long_term_frame_idx: Default::default() }
    }
}
impl std::fmt::Debug for StdVideoEncodeH264PictureInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StdVideoEncodeH264PictureInfo").field("flags", &self.flags).field("picture_type", &self.picture_type).field("frame_num", &self.frame_num).field("picture_order_count", &self.picture_order_count).field("long_term_pic_num", &self.long_term_pic_num).field("long_term_frame_idx", &self.long_term_frame_idx).finish()
    }
}
impl StdVideoEncodeH264PictureInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> StdVideoEncodeH264PictureInfoBuilder<'a> {
        StdVideoEncodeH264PictureInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "<s>Vulkan Manual Page</s> · Builder of [`StdVideoEncodeH264PictureInfo`]"]
#[repr(transparent)]
pub struct StdVideoEncodeH264PictureInfoBuilder<'a>(StdVideoEncodeH264PictureInfo, std::marker::PhantomData<&'a ()>);
impl<'a> StdVideoEncodeH264PictureInfoBuilder<'a> {
    #[inline]
    pub fn new() -> StdVideoEncodeH264PictureInfoBuilder<'a> {
        StdVideoEncodeH264PictureInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn flags(mut self, flags: crate::external::vk_video::StdVideoEncodeH264PictureInfoFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn picture_type(mut self, picture_type: crate::external::vk_video::StdVideoH264PictureType) -> Self {
        self.0.picture_type = picture_type as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn frame_num(mut self, frame_num: u32) -> Self {
        self.0.frame_num = frame_num as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn picture_order_count(mut self, picture_order_count: u32) -> Self {
        self.0.picture_order_count = picture_order_count as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn long_term_pic_num(mut self, long_term_pic_num: u16) -> Self {
        self.0.long_term_pic_num = long_term_pic_num as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn long_term_frame_idx(mut self, long_term_frame_idx: u16) -> Self {
        self.0.long_term_frame_idx = long_term_frame_idx as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> StdVideoEncodeH264PictureInfo {
        self.0
    }
}
impl<'a> std::default::Default for StdVideoEncodeH264PictureInfoBuilder<'a> {
    fn default() -> StdVideoEncodeH264PictureInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for StdVideoEncodeH264PictureInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for StdVideoEncodeH264PictureInfoBuilder<'a> {
    type Target = StdVideoEncodeH264PictureInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StdVideoEncodeH264PictureInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Structure"]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct StdVideoEncodeH264SliceHeaderFlags {
    pub idr_flag_and_more_bitfield: u32,
}
impl Default for StdVideoEncodeH264SliceHeaderFlags {
    fn default() -> Self {
        Self { idr_flag_and_more_bitfield: Default::default() }
    }
}
impl std::fmt::Debug for StdVideoEncodeH264SliceHeaderFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StdVideoEncodeH264SliceHeaderFlags").field("idr_flag_and_more_bitfield", &format!("{:#b}", &self.idr_flag_and_more_bitfield)).finish()
    }
}
impl StdVideoEncodeH264SliceHeaderFlags {
    #[inline]
    pub fn into_builder<'a>(self) -> StdVideoEncodeH264SliceHeaderFlagsBuilder<'a> {
        StdVideoEncodeH264SliceHeaderFlagsBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "<s>Vulkan Manual Page</s> · Builder of [`StdVideoEncodeH264SliceHeaderFlags`]"]
#[repr(transparent)]
pub struct StdVideoEncodeH264SliceHeaderFlagsBuilder<'a>(StdVideoEncodeH264SliceHeaderFlags, std::marker::PhantomData<&'a ()>);
impl<'a> StdVideoEncodeH264SliceHeaderFlagsBuilder<'a> {
    #[inline]
    pub fn new() -> StdVideoEncodeH264SliceHeaderFlagsBuilder<'a> {
        StdVideoEncodeH264SliceHeaderFlagsBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn idr_flag(mut self, idr_flag: u32) -> Self {
        self.0.idr_flag_and_more_bitfield = crate::bits_copy!(self.0.idr_flag_and_more_bitfield, idr_flag, 0usize, 0usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn is_reference_flag(mut self, is_reference_flag: u32) -> Self {
        self.0.idr_flag_and_more_bitfield = crate::bits_copy!(self.0.idr_flag_and_more_bitfield, is_reference_flag, 1usize, 1usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn num_ref_idx_active_override_flag(mut self, num_ref_idx_active_override_flag: u32) -> Self {
        self.0.idr_flag_and_more_bitfield = crate::bits_copy!(self.0.idr_flag_and_more_bitfield, num_ref_idx_active_override_flag, 2usize, 2usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn no_output_of_prior_pics_flag(mut self, no_output_of_prior_pics_flag: u32) -> Self {
        self.0.idr_flag_and_more_bitfield = crate::bits_copy!(self.0.idr_flag_and_more_bitfield, no_output_of_prior_pics_flag, 3usize, 3usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn long_term_reference_flag(mut self, long_term_reference_flag: u32) -> Self {
        self.0.idr_flag_and_more_bitfield = crate::bits_copy!(self.0.idr_flag_and_more_bitfield, long_term_reference_flag, 4usize, 4usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn adaptive_ref_pic_marking_mode_flag(mut self, adaptive_ref_pic_marking_mode_flag: u32) -> Self {
        self.0.idr_flag_and_more_bitfield = crate::bits_copy!(self.0.idr_flag_and_more_bitfield, adaptive_ref_pic_marking_mode_flag, 5usize, 5usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn no_prior_references_available_flag(mut self, no_prior_references_available_flag: u32) -> Self {
        self.0.idr_flag_and_more_bitfield = crate::bits_copy!(self.0.idr_flag_and_more_bitfield, no_prior_references_available_flag, 6usize, 6usize);
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> StdVideoEncodeH264SliceHeaderFlags {
        self.0
    }
}
impl<'a> std::default::Default for StdVideoEncodeH264SliceHeaderFlagsBuilder<'a> {
    fn default() -> StdVideoEncodeH264SliceHeaderFlagsBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for StdVideoEncodeH264SliceHeaderFlagsBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for StdVideoEncodeH264SliceHeaderFlagsBuilder<'a> {
    type Target = StdVideoEncodeH264SliceHeaderFlags;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StdVideoEncodeH264SliceHeaderFlagsBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StdVideoEncodeH264RefMemMgmtCtrlOperations {
    pub flags: crate::external::vk_video::StdVideoEncodeH264RefMgmtFlags,
    pub ref_list0_mod_op_count: u8,
    pub p_ref_list0_mod_operations: *mut crate::external::vk_video::StdVideoEncodeH264RefListModEntry,
    pub ref_list1_mod_op_count: u8,
    pub p_ref_list1_mod_operations: *mut crate::external::vk_video::StdVideoEncodeH264RefListModEntry,
    pub ref_pic_marking_op_count: u8,
    pub p_ref_pic_marking_operations: *mut crate::external::vk_video::StdVideoEncodeH264RefPicMarkingEntry,
}
impl Default for StdVideoEncodeH264RefMemMgmtCtrlOperations {
    fn default() -> Self {
        Self { flags: Default::default(), ref_list0_mod_op_count: Default::default(), p_ref_list0_mod_operations: std::ptr::null_mut(), ref_list1_mod_op_count: Default::default(), p_ref_list1_mod_operations: std::ptr::null_mut(), ref_pic_marking_op_count: Default::default(), p_ref_pic_marking_operations: std::ptr::null_mut() }
    }
}
impl std::fmt::Debug for StdVideoEncodeH264RefMemMgmtCtrlOperations {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StdVideoEncodeH264RefMemMgmtCtrlOperations").field("flags", &self.flags).field("ref_list0_mod_op_count", &self.ref_list0_mod_op_count).field("p_ref_list0_mod_operations", &self.p_ref_list0_mod_operations).field("ref_list1_mod_op_count", &self.ref_list1_mod_op_count).field("p_ref_list1_mod_operations", &self.p_ref_list1_mod_operations).field("ref_pic_marking_op_count", &self.ref_pic_marking_op_count).field("p_ref_pic_marking_operations", &self.p_ref_pic_marking_operations).finish()
    }
}
impl StdVideoEncodeH264RefMemMgmtCtrlOperations {
    #[inline]
    pub fn into_builder<'a>(self) -> StdVideoEncodeH264RefMemMgmtCtrlOperationsBuilder<'a> {
        StdVideoEncodeH264RefMemMgmtCtrlOperationsBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "<s>Vulkan Manual Page</s> · Builder of [`StdVideoEncodeH264RefMemMgmtCtrlOperations`]"]
#[repr(transparent)]
pub struct StdVideoEncodeH264RefMemMgmtCtrlOperationsBuilder<'a>(StdVideoEncodeH264RefMemMgmtCtrlOperations, std::marker::PhantomData<&'a ()>);
impl<'a> StdVideoEncodeH264RefMemMgmtCtrlOperationsBuilder<'a> {
    #[inline]
    pub fn new() -> StdVideoEncodeH264RefMemMgmtCtrlOperationsBuilder<'a> {
        StdVideoEncodeH264RefMemMgmtCtrlOperationsBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn flags(mut self, flags: crate::external::vk_video::StdVideoEncodeH264RefMgmtFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn ref_list0_mod_op_count(mut self, ref_list0_mod_op_count: u8) -> Self {
        self.0.ref_list0_mod_op_count = ref_list0_mod_op_count as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn ref_list0_mod_operations(mut self, ref_list0_mod_operations: &'a mut crate::external::vk_video::StdVideoEncodeH264RefListModEntry) -> Self {
        self.0.p_ref_list0_mod_operations = ref_list0_mod_operations as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn ref_list1_mod_op_count(mut self, ref_list1_mod_op_count: u8) -> Self {
        self.0.ref_list1_mod_op_count = ref_list1_mod_op_count as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn ref_list1_mod_operations(mut self, ref_list1_mod_operations: &'a mut crate::external::vk_video::StdVideoEncodeH264RefListModEntry) -> Self {
        self.0.p_ref_list1_mod_operations = ref_list1_mod_operations as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn ref_pic_marking_op_count(mut self, ref_pic_marking_op_count: u8) -> Self {
        self.0.ref_pic_marking_op_count = ref_pic_marking_op_count as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn ref_pic_marking_operations(mut self, ref_pic_marking_operations: &'a mut crate::external::vk_video::StdVideoEncodeH264RefPicMarkingEntry) -> Self {
        self.0.p_ref_pic_marking_operations = ref_pic_marking_operations as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> StdVideoEncodeH264RefMemMgmtCtrlOperations {
        self.0
    }
}
impl<'a> std::default::Default for StdVideoEncodeH264RefMemMgmtCtrlOperationsBuilder<'a> {
    fn default() -> StdVideoEncodeH264RefMemMgmtCtrlOperationsBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for StdVideoEncodeH264RefMemMgmtCtrlOperationsBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for StdVideoEncodeH264RefMemMgmtCtrlOperationsBuilder<'a> {
    type Target = StdVideoEncodeH264RefMemMgmtCtrlOperations;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StdVideoEncodeH264RefMemMgmtCtrlOperationsBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Structure"]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct StdVideoEncodeH264PictureInfoFlags {
    pub idr_flag_and_more_bitfield: u32,
}
impl Default for StdVideoEncodeH264PictureInfoFlags {
    fn default() -> Self {
        Self { idr_flag_and_more_bitfield: Default::default() }
    }
}
impl std::fmt::Debug for StdVideoEncodeH264PictureInfoFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StdVideoEncodeH264PictureInfoFlags").field("idr_flag_and_more_bitfield", &format!("{:#b}", &self.idr_flag_and_more_bitfield)).finish()
    }
}
impl StdVideoEncodeH264PictureInfoFlags {
    #[inline]
    pub fn into_builder<'a>(self) -> StdVideoEncodeH264PictureInfoFlagsBuilder<'a> {
        StdVideoEncodeH264PictureInfoFlagsBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "<s>Vulkan Manual Page</s> · Builder of [`StdVideoEncodeH264PictureInfoFlags`]"]
#[repr(transparent)]
pub struct StdVideoEncodeH264PictureInfoFlagsBuilder<'a>(StdVideoEncodeH264PictureInfoFlags, std::marker::PhantomData<&'a ()>);
impl<'a> StdVideoEncodeH264PictureInfoFlagsBuilder<'a> {
    #[inline]
    pub fn new() -> StdVideoEncodeH264PictureInfoFlagsBuilder<'a> {
        StdVideoEncodeH264PictureInfoFlagsBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn idr_flag(mut self, idr_flag: u32) -> Self {
        self.0.idr_flag_and_more_bitfield = crate::bits_copy!(self.0.idr_flag_and_more_bitfield, idr_flag, 0usize, 0usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn is_reference_flag(mut self, is_reference_flag: u32) -> Self {
        self.0.idr_flag_and_more_bitfield = crate::bits_copy!(self.0.idr_flag_and_more_bitfield, is_reference_flag, 1usize, 1usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn long_term_reference_flag(mut self, long_term_reference_flag: u32) -> Self {
        self.0.idr_flag_and_more_bitfield = crate::bits_copy!(self.0.idr_flag_and_more_bitfield, long_term_reference_flag, 2usize, 2usize);
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> StdVideoEncodeH264PictureInfoFlags {
        self.0
    }
}
impl<'a> std::default::Default for StdVideoEncodeH264PictureInfoFlagsBuilder<'a> {
    fn default() -> StdVideoEncodeH264PictureInfoFlagsBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for StdVideoEncodeH264PictureInfoFlagsBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for StdVideoEncodeH264PictureInfoFlagsBuilder<'a> {
    type Target = StdVideoEncodeH264PictureInfoFlags;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StdVideoEncodeH264PictureInfoFlagsBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Structure"]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct StdVideoEncodeH264RefMgmtFlags {
    pub ref_pic_list_modification_l0_flag_and_ref_pic_list_modification_l1_flag: u32,
}
impl Default for StdVideoEncodeH264RefMgmtFlags {
    fn default() -> Self {
        Self { ref_pic_list_modification_l0_flag_and_ref_pic_list_modification_l1_flag: Default::default() }
    }
}
impl std::fmt::Debug for StdVideoEncodeH264RefMgmtFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StdVideoEncodeH264RefMgmtFlags").field("ref_pic_list_modification_l0_flag_and_ref_pic_list_modification_l1_flag", &format!("{:#b}", &self.ref_pic_list_modification_l0_flag_and_ref_pic_list_modification_l1_flag)).finish()
    }
}
impl StdVideoEncodeH264RefMgmtFlags {
    #[inline]
    pub fn into_builder<'a>(self) -> StdVideoEncodeH264RefMgmtFlagsBuilder<'a> {
        StdVideoEncodeH264RefMgmtFlagsBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "<s>Vulkan Manual Page</s> · Builder of [`StdVideoEncodeH264RefMgmtFlags`]"]
#[repr(transparent)]
pub struct StdVideoEncodeH264RefMgmtFlagsBuilder<'a>(StdVideoEncodeH264RefMgmtFlags, std::marker::PhantomData<&'a ()>);
impl<'a> StdVideoEncodeH264RefMgmtFlagsBuilder<'a> {
    #[inline]
    pub fn new() -> StdVideoEncodeH264RefMgmtFlagsBuilder<'a> {
        StdVideoEncodeH264RefMgmtFlagsBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn ref_pic_list_modification_l0_flag(mut self, ref_pic_list_modification_l0_flag: u32) -> Self {
        self.0.ref_pic_list_modification_l0_flag_and_ref_pic_list_modification_l1_flag = crate::bits_copy!(self.0.ref_pic_list_modification_l0_flag_and_ref_pic_list_modification_l1_flag, ref_pic_list_modification_l0_flag, 0usize, 0usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn ref_pic_list_modification_l1_flag(mut self, ref_pic_list_modification_l1_flag: u32) -> Self {
        self.0.ref_pic_list_modification_l0_flag_and_ref_pic_list_modification_l1_flag = crate::bits_copy!(self.0.ref_pic_list_modification_l0_flag_and_ref_pic_list_modification_l1_flag, ref_pic_list_modification_l1_flag, 1usize, 1usize);
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> StdVideoEncodeH264RefMgmtFlags {
        self.0
    }
}
impl<'a> std::default::Default for StdVideoEncodeH264RefMgmtFlagsBuilder<'a> {
    fn default() -> StdVideoEncodeH264RefMgmtFlagsBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for StdVideoEncodeH264RefMgmtFlagsBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for StdVideoEncodeH264RefMgmtFlagsBuilder<'a> {
    type Target = StdVideoEncodeH264RefMgmtFlags;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StdVideoEncodeH264RefMgmtFlagsBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Structure"]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct StdVideoEncodeH264RefListModEntry {
    pub modification_of_pic_nums_idc: crate::external::vk_video::StdVideoH264ModificationOfPicNumsIdc,
    pub abs_diff_pic_num_minus1: u16,
    pub long_term_pic_num: u16,
}
impl Default for StdVideoEncodeH264RefListModEntry {
    fn default() -> Self {
        Self { modification_of_pic_nums_idc: Default::default(), abs_diff_pic_num_minus1: Default::default(), long_term_pic_num: Default::default() }
    }
}
impl std::fmt::Debug for StdVideoEncodeH264RefListModEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StdVideoEncodeH264RefListModEntry").field("modification_of_pic_nums_idc", &self.modification_of_pic_nums_idc).field("abs_diff_pic_num_minus1", &self.abs_diff_pic_num_minus1).field("long_term_pic_num", &self.long_term_pic_num).finish()
    }
}
impl StdVideoEncodeH264RefListModEntry {
    #[inline]
    pub fn into_builder<'a>(self) -> StdVideoEncodeH264RefListModEntryBuilder<'a> {
        StdVideoEncodeH264RefListModEntryBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "<s>Vulkan Manual Page</s> · Builder of [`StdVideoEncodeH264RefListModEntry`]"]
#[repr(transparent)]
pub struct StdVideoEncodeH264RefListModEntryBuilder<'a>(StdVideoEncodeH264RefListModEntry, std::marker::PhantomData<&'a ()>);
impl<'a> StdVideoEncodeH264RefListModEntryBuilder<'a> {
    #[inline]
    pub fn new() -> StdVideoEncodeH264RefListModEntryBuilder<'a> {
        StdVideoEncodeH264RefListModEntryBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn modification_of_pic_nums_idc(mut self, modification_of_pic_nums_idc: crate::external::vk_video::StdVideoH264ModificationOfPicNumsIdc) -> Self {
        self.0.modification_of_pic_nums_idc = modification_of_pic_nums_idc as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn abs_diff_pic_num_minus1(mut self, abs_diff_pic_num_minus1: u16) -> Self {
        self.0.abs_diff_pic_num_minus1 = abs_diff_pic_num_minus1 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn long_term_pic_num(mut self, long_term_pic_num: u16) -> Self {
        self.0.long_term_pic_num = long_term_pic_num as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> StdVideoEncodeH264RefListModEntry {
        self.0
    }
}
impl<'a> std::default::Default for StdVideoEncodeH264RefListModEntryBuilder<'a> {
    fn default() -> StdVideoEncodeH264RefListModEntryBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for StdVideoEncodeH264RefListModEntryBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for StdVideoEncodeH264RefListModEntryBuilder<'a> {
    type Target = StdVideoEncodeH264RefListModEntry;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StdVideoEncodeH264RefListModEntryBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Structure"]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct StdVideoEncodeH264RefPicMarkingEntry {
    pub operation: crate::external::vk_video::StdVideoH264MemMgmtControlOp,
    pub difference_of_pic_nums_minus1: u16,
    pub long_term_pic_num: u16,
    pub long_term_frame_idx: u16,
    pub max_long_term_frame_idx_plus1: u16,
}
impl Default for StdVideoEncodeH264RefPicMarkingEntry {
    fn default() -> Self {
        Self { operation: Default::default(), difference_of_pic_nums_minus1: Default::default(), long_term_pic_num: Default::default(), long_term_frame_idx: Default::default(), max_long_term_frame_idx_plus1: Default::default() }
    }
}
impl std::fmt::Debug for StdVideoEncodeH264RefPicMarkingEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StdVideoEncodeH264RefPicMarkingEntry").field("operation", &self.operation).field("difference_of_pic_nums_minus1", &self.difference_of_pic_nums_minus1).field("long_term_pic_num", &self.long_term_pic_num).field("long_term_frame_idx", &self.long_term_frame_idx).field("max_long_term_frame_idx_plus1", &self.max_long_term_frame_idx_plus1).finish()
    }
}
impl StdVideoEncodeH264RefPicMarkingEntry {
    #[inline]
    pub fn into_builder<'a>(self) -> StdVideoEncodeH264RefPicMarkingEntryBuilder<'a> {
        StdVideoEncodeH264RefPicMarkingEntryBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "<s>Vulkan Manual Page</s> · Builder of [`StdVideoEncodeH264RefPicMarkingEntry`]"]
#[repr(transparent)]
pub struct StdVideoEncodeH264RefPicMarkingEntryBuilder<'a>(StdVideoEncodeH264RefPicMarkingEntry, std::marker::PhantomData<&'a ()>);
impl<'a> StdVideoEncodeH264RefPicMarkingEntryBuilder<'a> {
    #[inline]
    pub fn new() -> StdVideoEncodeH264RefPicMarkingEntryBuilder<'a> {
        StdVideoEncodeH264RefPicMarkingEntryBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn operation(mut self, operation: crate::external::vk_video::StdVideoH264MemMgmtControlOp) -> Self {
        self.0.operation = operation as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn difference_of_pic_nums_minus1(mut self, difference_of_pic_nums_minus1: u16) -> Self {
        self.0.difference_of_pic_nums_minus1 = difference_of_pic_nums_minus1 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn long_term_pic_num(mut self, long_term_pic_num: u16) -> Self {
        self.0.long_term_pic_num = long_term_pic_num as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn long_term_frame_idx(mut self, long_term_frame_idx: u16) -> Self {
        self.0.long_term_frame_idx = long_term_frame_idx as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn max_long_term_frame_idx_plus1(mut self, max_long_term_frame_idx_plus1: u16) -> Self {
        self.0.max_long_term_frame_idx_plus1 = max_long_term_frame_idx_plus1 as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> StdVideoEncodeH264RefPicMarkingEntry {
        self.0
    }
}
impl<'a> std::default::Default for StdVideoEncodeH264RefPicMarkingEntryBuilder<'a> {
    fn default() -> StdVideoEncodeH264RefPicMarkingEntryBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for StdVideoEncodeH264RefPicMarkingEntryBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for StdVideoEncodeH264RefPicMarkingEntryBuilder<'a> {
    type Target = StdVideoEncodeH264RefPicMarkingEntry;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StdVideoEncodeH264RefPicMarkingEntryBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Structure"]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct StdVideoEncodeH265PictureInfoFlags {
    pub is_reference_flag_and_more_bitfield: u32,
}
impl Default for StdVideoEncodeH265PictureInfoFlags {
    fn default() -> Self {
        Self { is_reference_flag_and_more_bitfield: Default::default() }
    }
}
impl std::fmt::Debug for StdVideoEncodeH265PictureInfoFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StdVideoEncodeH265PictureInfoFlags").field("is_reference_flag_and_more_bitfield", &format!("{:#b}", &self.is_reference_flag_and_more_bitfield)).finish()
    }
}
impl StdVideoEncodeH265PictureInfoFlags {
    #[inline]
    pub fn into_builder<'a>(self) -> StdVideoEncodeH265PictureInfoFlagsBuilder<'a> {
        StdVideoEncodeH265PictureInfoFlagsBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "<s>Vulkan Manual Page</s> · Builder of [`StdVideoEncodeH265PictureInfoFlags`]"]
#[repr(transparent)]
pub struct StdVideoEncodeH265PictureInfoFlagsBuilder<'a>(StdVideoEncodeH265PictureInfoFlags, std::marker::PhantomData<&'a ()>);
impl<'a> StdVideoEncodeH265PictureInfoFlagsBuilder<'a> {
    #[inline]
    pub fn new() -> StdVideoEncodeH265PictureInfoFlagsBuilder<'a> {
        StdVideoEncodeH265PictureInfoFlagsBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn is_reference_flag(mut self, is_reference_flag: u32) -> Self {
        self.0.is_reference_flag_and_more_bitfield = crate::bits_copy!(self.0.is_reference_flag_and_more_bitfield, is_reference_flag, 0usize, 0usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn irap_pic_flag(mut self, irap_pic_flag: u32) -> Self {
        self.0.is_reference_flag_and_more_bitfield = crate::bits_copy!(self.0.is_reference_flag_and_more_bitfield, irap_pic_flag, 1usize, 1usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn long_term_flag(mut self, long_term_flag: u32) -> Self {
        self.0.is_reference_flag_and_more_bitfield = crate::bits_copy!(self.0.is_reference_flag_and_more_bitfield, long_term_flag, 2usize, 2usize);
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> StdVideoEncodeH265PictureInfoFlags {
        self.0
    }
}
impl<'a> std::default::Default for StdVideoEncodeH265PictureInfoFlagsBuilder<'a> {
    fn default() -> StdVideoEncodeH265PictureInfoFlagsBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for StdVideoEncodeH265PictureInfoFlagsBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for StdVideoEncodeH265PictureInfoFlagsBuilder<'a> {
    type Target = StdVideoEncodeH265PictureInfoFlags;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StdVideoEncodeH265PictureInfoFlagsBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Structure"]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct StdVideoEncodeH265PictureInfo {
    pub picture_type: crate::external::vk_video::StdVideoH265PictureType,
    pub sps_video_parameter_set_id: u8,
    pub pps_seq_parameter_set_id: u8,
    pub pic_order_cnt_val: i32,
    pub temporal_id: u8,
    pub flags: crate::external::vk_video::StdVideoEncodeH265PictureInfoFlags,
}
impl Default for StdVideoEncodeH265PictureInfo {
    fn default() -> Self {
        Self { picture_type: Default::default(), sps_video_parameter_set_id: Default::default(), pps_seq_parameter_set_id: Default::default(), pic_order_cnt_val: Default::default(), temporal_id: Default::default(), flags: Default::default() }
    }
}
impl std::fmt::Debug for StdVideoEncodeH265PictureInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StdVideoEncodeH265PictureInfo").field("picture_type", &self.picture_type).field("sps_video_parameter_set_id", &self.sps_video_parameter_set_id).field("pps_seq_parameter_set_id", &self.pps_seq_parameter_set_id).field("pic_order_cnt_val", &self.pic_order_cnt_val).field("temporal_id", &self.temporal_id).field("flags", &self.flags).finish()
    }
}
impl StdVideoEncodeH265PictureInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> StdVideoEncodeH265PictureInfoBuilder<'a> {
        StdVideoEncodeH265PictureInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "<s>Vulkan Manual Page</s> · Builder of [`StdVideoEncodeH265PictureInfo`]"]
#[repr(transparent)]
pub struct StdVideoEncodeH265PictureInfoBuilder<'a>(StdVideoEncodeH265PictureInfo, std::marker::PhantomData<&'a ()>);
impl<'a> StdVideoEncodeH265PictureInfoBuilder<'a> {
    #[inline]
    pub fn new() -> StdVideoEncodeH265PictureInfoBuilder<'a> {
        StdVideoEncodeH265PictureInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn picture_type(mut self, picture_type: crate::external::vk_video::StdVideoH265PictureType) -> Self {
        self.0.picture_type = picture_type as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn sps_video_parameter_set_id(mut self, sps_video_parameter_set_id: u8) -> Self {
        self.0.sps_video_parameter_set_id = sps_video_parameter_set_id as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn pps_seq_parameter_set_id(mut self, pps_seq_parameter_set_id: u8) -> Self {
        self.0.pps_seq_parameter_set_id = pps_seq_parameter_set_id as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn pic_order_cnt_val(mut self, pic_order_cnt_val: i32) -> Self {
        self.0.pic_order_cnt_val = pic_order_cnt_val as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn temporal_id(mut self, temporal_id: u8) -> Self {
        self.0.temporal_id = temporal_id as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn flags(mut self, flags: crate::external::vk_video::StdVideoEncodeH265PictureInfoFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> StdVideoEncodeH265PictureInfo {
        self.0
    }
}
impl<'a> std::default::Default for StdVideoEncodeH265PictureInfoBuilder<'a> {
    fn default() -> StdVideoEncodeH265PictureInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for StdVideoEncodeH265PictureInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for StdVideoEncodeH265PictureInfoBuilder<'a> {
    type Target = StdVideoEncodeH265PictureInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StdVideoEncodeH265PictureInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Structure"]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct StdVideoEncodeH265SliceHeader {
    pub slice_type: crate::external::vk_video::StdVideoH265SliceType,
    pub slice_pic_parameter_set_id: u8,
    pub num_short_term_ref_pic_sets: u8,
    pub slice_segment_address: u32,
    pub short_term_ref_pic_set_idx: u8,
    pub num_long_term_sps: u8,
    pub num_long_term_pics: u8,
    pub collocated_ref_idx: u8,
    pub num_ref_idx_l0_active_minus1: u8,
    pub num_ref_idx_l1_active_minus1: u8,
    pub luma_log2_weight_denom: u8,
    pub delta_chroma_log2_weight_denom: i8,
    pub delta_luma_weight_l0: [i8; 15],
    pub luma_offset_l0: [i8; 15],
    pub delta_chroma_weight_l0: [[i8; 2]; 15],
    pub delta_chroma_offset_l0: [[i8; 2]; 15],
    pub delta_luma_weight_l1: [i8; 15],
    pub luma_offset_l1: [i8; 15],
    pub delta_chroma_weight_l1: [[i8; 2]; 15],
    pub delta_chroma_offset_l1: [[i8; 2]; 15],
    pub max_num_merge_cand: u8,
    pub slice_qp_delta: i8,
    pub slice_cb_qp_offset: i8,
    pub slice_cr_qp_offset: i8,
    pub slice_beta_offset_div2: i8,
    pub slice_tc_offset_div2: i8,
    pub slice_act_y_qp_offset: i8,
    pub slice_act_cb_qp_offset: i8,
    pub slice_act_cr_qp_offset: i8,
    pub flags: crate::external::vk_video::StdVideoEncodeH265SliceHeaderFlags,
}
impl Default for StdVideoEncodeH265SliceHeader {
    fn default() -> Self {
        Self { slice_type: Default::default(), slice_pic_parameter_set_id: Default::default(), num_short_term_ref_pic_sets: Default::default(), slice_segment_address: Default::default(), short_term_ref_pic_set_idx: Default::default(), num_long_term_sps: Default::default(), num_long_term_pics: Default::default(), collocated_ref_idx: Default::default(), num_ref_idx_l0_active_minus1: Default::default(), num_ref_idx_l1_active_minus1: Default::default(), luma_log2_weight_denom: Default::default(), delta_chroma_log2_weight_denom: Default::default(), delta_luma_weight_l0: unsafe { std::mem::zeroed() }, luma_offset_l0: unsafe { std::mem::zeroed() }, delta_chroma_weight_l0: unsafe { std::mem::zeroed() }, delta_chroma_offset_l0: unsafe { std::mem::zeroed() }, delta_luma_weight_l1: unsafe { std::mem::zeroed() }, luma_offset_l1: unsafe { std::mem::zeroed() }, delta_chroma_weight_l1: unsafe { std::mem::zeroed() }, delta_chroma_offset_l1: unsafe { std::mem::zeroed() }, max_num_merge_cand: Default::default(), slice_qp_delta: Default::default(), slice_cb_qp_offset: Default::default(), slice_cr_qp_offset: Default::default(), slice_beta_offset_div2: Default::default(), slice_tc_offset_div2: Default::default(), slice_act_y_qp_offset: Default::default(), slice_act_cb_qp_offset: Default::default(), slice_act_cr_qp_offset: Default::default(), flags: Default::default() }
    }
}
impl std::fmt::Debug for StdVideoEncodeH265SliceHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StdVideoEncodeH265SliceHeader").field("slice_type", &self.slice_type).field("slice_pic_parameter_set_id", &self.slice_pic_parameter_set_id).field("num_short_term_ref_pic_sets", &self.num_short_term_ref_pic_sets).field("slice_segment_address", &self.slice_segment_address).field("short_term_ref_pic_set_idx", &self.short_term_ref_pic_set_idx).field("num_long_term_sps", &self.num_long_term_sps).field("num_long_term_pics", &self.num_long_term_pics).field("collocated_ref_idx", &self.collocated_ref_idx).field("num_ref_idx_l0_active_minus1", &self.num_ref_idx_l0_active_minus1).field("num_ref_idx_l1_active_minus1", &self.num_ref_idx_l1_active_minus1).field("luma_log2_weight_denom", &self.luma_log2_weight_denom).field("delta_chroma_log2_weight_denom", &self.delta_chroma_log2_weight_denom).field("delta_luma_weight_l0", &self.delta_luma_weight_l0).field("luma_offset_l0", &self.luma_offset_l0).field("delta_chroma_weight_l0", &self.delta_chroma_weight_l0).field("delta_chroma_offset_l0", &self.delta_chroma_offset_l0).field("delta_luma_weight_l1", &self.delta_luma_weight_l1).field("luma_offset_l1", &self.luma_offset_l1).field("delta_chroma_weight_l1", &self.delta_chroma_weight_l1).field("delta_chroma_offset_l1", &self.delta_chroma_offset_l1).field("max_num_merge_cand", &self.max_num_merge_cand).field("slice_qp_delta", &self.slice_qp_delta).field("slice_cb_qp_offset", &self.slice_cb_qp_offset).field("slice_cr_qp_offset", &self.slice_cr_qp_offset).field("slice_beta_offset_div2", &self.slice_beta_offset_div2).field("slice_tc_offset_div2", &self.slice_tc_offset_div2).field("slice_act_y_qp_offset", &self.slice_act_y_qp_offset).field("slice_act_cb_qp_offset", &self.slice_act_cb_qp_offset).field("slice_act_cr_qp_offset", &self.slice_act_cr_qp_offset).field("flags", &self.flags).finish()
    }
}
impl StdVideoEncodeH265SliceHeader {
    #[inline]
    pub fn into_builder<'a>(self) -> StdVideoEncodeH265SliceHeaderBuilder<'a> {
        StdVideoEncodeH265SliceHeaderBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "<s>Vulkan Manual Page</s> · Builder of [`StdVideoEncodeH265SliceHeader`]"]
#[repr(transparent)]
pub struct StdVideoEncodeH265SliceHeaderBuilder<'a>(StdVideoEncodeH265SliceHeader, std::marker::PhantomData<&'a ()>);
impl<'a> StdVideoEncodeH265SliceHeaderBuilder<'a> {
    #[inline]
    pub fn new() -> StdVideoEncodeH265SliceHeaderBuilder<'a> {
        StdVideoEncodeH265SliceHeaderBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn slice_type(mut self, slice_type: crate::external::vk_video::StdVideoH265SliceType) -> Self {
        self.0.slice_type = slice_type as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn slice_pic_parameter_set_id(mut self, slice_pic_parameter_set_id: u8) -> Self {
        self.0.slice_pic_parameter_set_id = slice_pic_parameter_set_id as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn num_short_term_ref_pic_sets(mut self, num_short_term_ref_pic_sets: u8) -> Self {
        self.0.num_short_term_ref_pic_sets = num_short_term_ref_pic_sets as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn slice_segment_address(mut self, slice_segment_address: u32) -> Self {
        self.0.slice_segment_address = slice_segment_address as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn short_term_ref_pic_set_idx(mut self, short_term_ref_pic_set_idx: u8) -> Self {
        self.0.short_term_ref_pic_set_idx = short_term_ref_pic_set_idx as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn num_long_term_sps(mut self, num_long_term_sps: u8) -> Self {
        self.0.num_long_term_sps = num_long_term_sps as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn num_long_term_pics(mut self, num_long_term_pics: u8) -> Self {
        self.0.num_long_term_pics = num_long_term_pics as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn collocated_ref_idx(mut self, collocated_ref_idx: u8) -> Self {
        self.0.collocated_ref_idx = collocated_ref_idx as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn num_ref_idx_l0_active_minus1(mut self, num_ref_idx_l0_active_minus1: u8) -> Self {
        self.0.num_ref_idx_l0_active_minus1 = num_ref_idx_l0_active_minus1 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn num_ref_idx_l1_active_minus1(mut self, num_ref_idx_l1_active_minus1: u8) -> Self {
        self.0.num_ref_idx_l1_active_minus1 = num_ref_idx_l1_active_minus1 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn luma_log2_weight_denom(mut self, luma_log2_weight_denom: u8) -> Self {
        self.0.luma_log2_weight_denom = luma_log2_weight_denom as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn delta_chroma_log2_weight_denom(mut self, delta_chroma_log2_weight_denom: i8) -> Self {
        self.0.delta_chroma_log2_weight_denom = delta_chroma_log2_weight_denom as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn delta_luma_weight_l0(mut self, delta_luma_weight_l0: [i8; 15]) -> Self {
        self.0.delta_luma_weight_l0 = delta_luma_weight_l0 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn luma_offset_l0(mut self, luma_offset_l0: [i8; 15]) -> Self {
        self.0.luma_offset_l0 = luma_offset_l0 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn delta_chroma_weight_l0(mut self, delta_chroma_weight_l0: [[i8; 2]; 15]) -> Self {
        self.0.delta_chroma_weight_l0 = delta_chroma_weight_l0 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn delta_chroma_offset_l0(mut self, delta_chroma_offset_l0: [[i8; 2]; 15]) -> Self {
        self.0.delta_chroma_offset_l0 = delta_chroma_offset_l0 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn delta_luma_weight_l1(mut self, delta_luma_weight_l1: [i8; 15]) -> Self {
        self.0.delta_luma_weight_l1 = delta_luma_weight_l1 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn luma_offset_l1(mut self, luma_offset_l1: [i8; 15]) -> Self {
        self.0.luma_offset_l1 = luma_offset_l1 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn delta_chroma_weight_l1(mut self, delta_chroma_weight_l1: [[i8; 2]; 15]) -> Self {
        self.0.delta_chroma_weight_l1 = delta_chroma_weight_l1 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn delta_chroma_offset_l1(mut self, delta_chroma_offset_l1: [[i8; 2]; 15]) -> Self {
        self.0.delta_chroma_offset_l1 = delta_chroma_offset_l1 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn max_num_merge_cand(mut self, max_num_merge_cand: u8) -> Self {
        self.0.max_num_merge_cand = max_num_merge_cand as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn slice_qp_delta(mut self, slice_qp_delta: i8) -> Self {
        self.0.slice_qp_delta = slice_qp_delta as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn slice_cb_qp_offset(mut self, slice_cb_qp_offset: i8) -> Self {
        self.0.slice_cb_qp_offset = slice_cb_qp_offset as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn slice_cr_qp_offset(mut self, slice_cr_qp_offset: i8) -> Self {
        self.0.slice_cr_qp_offset = slice_cr_qp_offset as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn slice_beta_offset_div2(mut self, slice_beta_offset_div2: i8) -> Self {
        self.0.slice_beta_offset_div2 = slice_beta_offset_div2 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn slice_tc_offset_div2(mut self, slice_tc_offset_div2: i8) -> Self {
        self.0.slice_tc_offset_div2 = slice_tc_offset_div2 as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn slice_act_y_qp_offset(mut self, slice_act_y_qp_offset: i8) -> Self {
        self.0.slice_act_y_qp_offset = slice_act_y_qp_offset as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn slice_act_cb_qp_offset(mut self, slice_act_cb_qp_offset: i8) -> Self {
        self.0.slice_act_cb_qp_offset = slice_act_cb_qp_offset as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn slice_act_cr_qp_offset(mut self, slice_act_cr_qp_offset: i8) -> Self {
        self.0.slice_act_cr_qp_offset = slice_act_cr_qp_offset as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn flags(mut self, flags: crate::external::vk_video::StdVideoEncodeH265SliceHeaderFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> StdVideoEncodeH265SliceHeader {
        self.0
    }
}
impl<'a> std::default::Default for StdVideoEncodeH265SliceHeaderBuilder<'a> {
    fn default() -> StdVideoEncodeH265SliceHeaderBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for StdVideoEncodeH265SliceHeaderBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for StdVideoEncodeH265SliceHeaderBuilder<'a> {
    type Target = StdVideoEncodeH265SliceHeader;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StdVideoEncodeH265SliceHeaderBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Structure"]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct StdVideoEncodeH265ReferenceInfo {
    pub pic_order_cnt_val: i32,
    pub temporal_id: u8,
    pub flags: crate::external::vk_video::StdVideoEncodeH265ReferenceInfoFlags,
}
impl Default for StdVideoEncodeH265ReferenceInfo {
    fn default() -> Self {
        Self { pic_order_cnt_val: Default::default(), temporal_id: Default::default(), flags: Default::default() }
    }
}
impl std::fmt::Debug for StdVideoEncodeH265ReferenceInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StdVideoEncodeH265ReferenceInfo").field("pic_order_cnt_val", &self.pic_order_cnt_val).field("temporal_id", &self.temporal_id).field("flags", &self.flags).finish()
    }
}
impl StdVideoEncodeH265ReferenceInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> StdVideoEncodeH265ReferenceInfoBuilder<'a> {
        StdVideoEncodeH265ReferenceInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "<s>Vulkan Manual Page</s> · Builder of [`StdVideoEncodeH265ReferenceInfo`]"]
#[repr(transparent)]
pub struct StdVideoEncodeH265ReferenceInfoBuilder<'a>(StdVideoEncodeH265ReferenceInfo, std::marker::PhantomData<&'a ()>);
impl<'a> StdVideoEncodeH265ReferenceInfoBuilder<'a> {
    #[inline]
    pub fn new() -> StdVideoEncodeH265ReferenceInfoBuilder<'a> {
        StdVideoEncodeH265ReferenceInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn pic_order_cnt_val(mut self, pic_order_cnt_val: i32) -> Self {
        self.0.pic_order_cnt_val = pic_order_cnt_val as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn temporal_id(mut self, temporal_id: u8) -> Self {
        self.0.temporal_id = temporal_id as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn flags(mut self, flags: crate::external::vk_video::StdVideoEncodeH265ReferenceInfoFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> StdVideoEncodeH265ReferenceInfo {
        self.0
    }
}
impl<'a> std::default::Default for StdVideoEncodeH265ReferenceInfoBuilder<'a> {
    fn default() -> StdVideoEncodeH265ReferenceInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for StdVideoEncodeH265ReferenceInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for StdVideoEncodeH265ReferenceInfoBuilder<'a> {
    type Target = StdVideoEncodeH265ReferenceInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StdVideoEncodeH265ReferenceInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StdVideoEncodeH265ReferenceModifications {
    pub flags: crate::external::vk_video::StdVideoEncodeH265ReferenceModificationFlags,
    pub reference_list0_modifications_count: u8,
    pub p_reference_list0_modifications: *mut u8,
    pub reference_list1_modifications_count: u8,
    pub p_reference_list1_modifications: *mut u8,
}
impl Default for StdVideoEncodeH265ReferenceModifications {
    fn default() -> Self {
        Self { flags: Default::default(), reference_list0_modifications_count: Default::default(), p_reference_list0_modifications: std::ptr::null_mut(), reference_list1_modifications_count: Default::default(), p_reference_list1_modifications: std::ptr::null_mut() }
    }
}
impl std::fmt::Debug for StdVideoEncodeH265ReferenceModifications {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StdVideoEncodeH265ReferenceModifications").field("flags", &self.flags).field("reference_list0_modifications_count", &self.reference_list0_modifications_count).field("p_reference_list0_modifications", &self.p_reference_list0_modifications).field("reference_list1_modifications_count", &self.reference_list1_modifications_count).field("p_reference_list1_modifications", &self.p_reference_list1_modifications).finish()
    }
}
impl StdVideoEncodeH265ReferenceModifications {
    #[inline]
    pub fn into_builder<'a>(self) -> StdVideoEncodeH265ReferenceModificationsBuilder<'a> {
        StdVideoEncodeH265ReferenceModificationsBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "<s>Vulkan Manual Page</s> · Builder of [`StdVideoEncodeH265ReferenceModifications`]"]
#[repr(transparent)]
pub struct StdVideoEncodeH265ReferenceModificationsBuilder<'a>(StdVideoEncodeH265ReferenceModifications, std::marker::PhantomData<&'a ()>);
impl<'a> StdVideoEncodeH265ReferenceModificationsBuilder<'a> {
    #[inline]
    pub fn new() -> StdVideoEncodeH265ReferenceModificationsBuilder<'a> {
        StdVideoEncodeH265ReferenceModificationsBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn flags(mut self, flags: crate::external::vk_video::StdVideoEncodeH265ReferenceModificationFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn reference_list0_modifications_count(mut self, reference_list0_modifications_count: u8) -> Self {
        self.0.reference_list0_modifications_count = reference_list0_modifications_count as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn reference_list0_modifications(mut self, reference_list0_modifications: &'a mut u8) -> Self {
        self.0.p_reference_list0_modifications = reference_list0_modifications as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn reference_list1_modifications_count(mut self, reference_list1_modifications_count: u8) -> Self {
        self.0.reference_list1_modifications_count = reference_list1_modifications_count as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn reference_list1_modifications(mut self, reference_list1_modifications: &'a mut u8) -> Self {
        self.0.p_reference_list1_modifications = reference_list1_modifications as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> StdVideoEncodeH265ReferenceModifications {
        self.0
    }
}
impl<'a> std::default::Default for StdVideoEncodeH265ReferenceModificationsBuilder<'a> {
    fn default() -> StdVideoEncodeH265ReferenceModificationsBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for StdVideoEncodeH265ReferenceModificationsBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for StdVideoEncodeH265ReferenceModificationsBuilder<'a> {
    type Target = StdVideoEncodeH265ReferenceModifications;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StdVideoEncodeH265ReferenceModificationsBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Structure"]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct StdVideoEncodeH265SliceHeaderFlags {
    pub first_slice_segment_in_pic_flag_and_more_bitfield: u32,
    pub luma_weight_l0_flag: u16,
    pub chroma_weight_l0_flag: u16,
    pub luma_weight_l1_flag: u16,
    pub chroma_weight_l1_flag: u16,
}
impl Default for StdVideoEncodeH265SliceHeaderFlags {
    fn default() -> Self {
        Self { first_slice_segment_in_pic_flag_and_more_bitfield: Default::default(), luma_weight_l0_flag: Default::default(), chroma_weight_l0_flag: Default::default(), luma_weight_l1_flag: Default::default(), chroma_weight_l1_flag: Default::default() }
    }
}
impl std::fmt::Debug for StdVideoEncodeH265SliceHeaderFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StdVideoEncodeH265SliceHeaderFlags").field("first_slice_segment_in_pic_flag_and_more_bitfield", &format!("{:#b}", &self.first_slice_segment_in_pic_flag_and_more_bitfield)).field("luma_weight_l0_flag", &self.luma_weight_l0_flag).field("chroma_weight_l0_flag", &self.chroma_weight_l0_flag).field("luma_weight_l1_flag", &self.luma_weight_l1_flag).field("chroma_weight_l1_flag", &self.chroma_weight_l1_flag).finish()
    }
}
impl StdVideoEncodeH265SliceHeaderFlags {
    #[inline]
    pub fn into_builder<'a>(self) -> StdVideoEncodeH265SliceHeaderFlagsBuilder<'a> {
        StdVideoEncodeH265SliceHeaderFlagsBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "<s>Vulkan Manual Page</s> · Builder of [`StdVideoEncodeH265SliceHeaderFlags`]"]
#[repr(transparent)]
pub struct StdVideoEncodeH265SliceHeaderFlagsBuilder<'a>(StdVideoEncodeH265SliceHeaderFlags, std::marker::PhantomData<&'a ()>);
impl<'a> StdVideoEncodeH265SliceHeaderFlagsBuilder<'a> {
    #[inline]
    pub fn new() -> StdVideoEncodeH265SliceHeaderFlagsBuilder<'a> {
        StdVideoEncodeH265SliceHeaderFlagsBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn first_slice_segment_in_pic_flag(mut self, first_slice_segment_in_pic_flag: u32) -> Self {
        self.0.first_slice_segment_in_pic_flag_and_more_bitfield = crate::bits_copy!(self.0.first_slice_segment_in_pic_flag_and_more_bitfield, first_slice_segment_in_pic_flag, 0usize, 0usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn no_output_of_prior_pics_flag(mut self, no_output_of_prior_pics_flag: u32) -> Self {
        self.0.first_slice_segment_in_pic_flag_and_more_bitfield = crate::bits_copy!(self.0.first_slice_segment_in_pic_flag_and_more_bitfield, no_output_of_prior_pics_flag, 1usize, 1usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn dependent_slice_segment_flag(mut self, dependent_slice_segment_flag: u32) -> Self {
        self.0.first_slice_segment_in_pic_flag_and_more_bitfield = crate::bits_copy!(self.0.first_slice_segment_in_pic_flag_and_more_bitfield, dependent_slice_segment_flag, 2usize, 2usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn short_term_ref_pic_set_sps_flag(mut self, short_term_ref_pic_set_sps_flag: u32) -> Self {
        self.0.first_slice_segment_in_pic_flag_and_more_bitfield = crate::bits_copy!(self.0.first_slice_segment_in_pic_flag_and_more_bitfield, short_term_ref_pic_set_sps_flag, 3usize, 3usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn slice_temporal_mvp_enable_flag(mut self, slice_temporal_mvp_enable_flag: u32) -> Self {
        self.0.first_slice_segment_in_pic_flag_and_more_bitfield = crate::bits_copy!(self.0.first_slice_segment_in_pic_flag_and_more_bitfield, slice_temporal_mvp_enable_flag, 4usize, 4usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn slice_sao_luma_flag(mut self, slice_sao_luma_flag: u32) -> Self {
        self.0.first_slice_segment_in_pic_flag_and_more_bitfield = crate::bits_copy!(self.0.first_slice_segment_in_pic_flag_and_more_bitfield, slice_sao_luma_flag, 5usize, 5usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn slice_sao_chroma_flag(mut self, slice_sao_chroma_flag: u32) -> Self {
        self.0.first_slice_segment_in_pic_flag_and_more_bitfield = crate::bits_copy!(self.0.first_slice_segment_in_pic_flag_and_more_bitfield, slice_sao_chroma_flag, 6usize, 6usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn num_ref_idx_active_override_flag(mut self, num_ref_idx_active_override_flag: u32) -> Self {
        self.0.first_slice_segment_in_pic_flag_and_more_bitfield = crate::bits_copy!(self.0.first_slice_segment_in_pic_flag_and_more_bitfield, num_ref_idx_active_override_flag, 7usize, 7usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn mvd_l1_zero_flag(mut self, mvd_l1_zero_flag: u32) -> Self {
        self.0.first_slice_segment_in_pic_flag_and_more_bitfield = crate::bits_copy!(self.0.first_slice_segment_in_pic_flag_and_more_bitfield, mvd_l1_zero_flag, 8usize, 8usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn cabac_init_flag(mut self, cabac_init_flag: u32) -> Self {
        self.0.first_slice_segment_in_pic_flag_and_more_bitfield = crate::bits_copy!(self.0.first_slice_segment_in_pic_flag_and_more_bitfield, cabac_init_flag, 9usize, 9usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn slice_deblocking_filter_disable_flag(mut self, slice_deblocking_filter_disable_flag: u32) -> Self {
        self.0.first_slice_segment_in_pic_flag_and_more_bitfield = crate::bits_copy!(self.0.first_slice_segment_in_pic_flag_and_more_bitfield, slice_deblocking_filter_disable_flag, 10usize, 10usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn collocated_from_l0_flag(mut self, collocated_from_l0_flag: u32) -> Self {
        self.0.first_slice_segment_in_pic_flag_and_more_bitfield = crate::bits_copy!(self.0.first_slice_segment_in_pic_flag_and_more_bitfield, collocated_from_l0_flag, 11usize, 11usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn slice_loop_filter_across_slices_enabled_flag(mut self, slice_loop_filter_across_slices_enabled_flag: u32) -> Self {
        self.0.first_slice_segment_in_pic_flag_and_more_bitfield = crate::bits_copy!(self.0.first_slice_segment_in_pic_flag_and_more_bitfield, slice_loop_filter_across_slices_enabled_flag, 12usize, 12usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn b_last_slice_in_pic(mut self, b_last_slice_in_pic: u32) -> Self {
        self.0.first_slice_segment_in_pic_flag_and_more_bitfield = crate::bits_copy!(self.0.first_slice_segment_in_pic_flag_and_more_bitfield, b_last_slice_in_pic, 13usize, 13usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn reserved_bits(mut self, reserved_bits: u32) -> Self {
        self.0.first_slice_segment_in_pic_flag_and_more_bitfield = crate::bits_copy!(self.0.first_slice_segment_in_pic_flag_and_more_bitfield, reserved_bits, 14usize, 31usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn luma_weight_l0_flag(mut self, luma_weight_l0_flag: u16) -> Self {
        self.0.luma_weight_l0_flag = luma_weight_l0_flag as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn chroma_weight_l0_flag(mut self, chroma_weight_l0_flag: u16) -> Self {
        self.0.chroma_weight_l0_flag = chroma_weight_l0_flag as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn luma_weight_l1_flag(mut self, luma_weight_l1_flag: u16) -> Self {
        self.0.luma_weight_l1_flag = luma_weight_l1_flag as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn chroma_weight_l1_flag(mut self, chroma_weight_l1_flag: u16) -> Self {
        self.0.chroma_weight_l1_flag = chroma_weight_l1_flag as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> StdVideoEncodeH265SliceHeaderFlags {
        self.0
    }
}
impl<'a> std::default::Default for StdVideoEncodeH265SliceHeaderFlagsBuilder<'a> {
    fn default() -> StdVideoEncodeH265SliceHeaderFlagsBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for StdVideoEncodeH265SliceHeaderFlagsBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for StdVideoEncodeH265SliceHeaderFlagsBuilder<'a> {
    type Target = StdVideoEncodeH265SliceHeaderFlags;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StdVideoEncodeH265SliceHeaderFlagsBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Structure"]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct StdVideoEncodeH265ReferenceInfoFlags {
    pub is_long_term_and_is_used_flag: u32,
}
impl Default for StdVideoEncodeH265ReferenceInfoFlags {
    fn default() -> Self {
        Self { is_long_term_and_is_used_flag: Default::default() }
    }
}
impl std::fmt::Debug for StdVideoEncodeH265ReferenceInfoFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StdVideoEncodeH265ReferenceInfoFlags").field("is_long_term_and_is_used_flag", &format!("{:#b}", &self.is_long_term_and_is_used_flag)).finish()
    }
}
impl StdVideoEncodeH265ReferenceInfoFlags {
    #[inline]
    pub fn into_builder<'a>(self) -> StdVideoEncodeH265ReferenceInfoFlagsBuilder<'a> {
        StdVideoEncodeH265ReferenceInfoFlagsBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "<s>Vulkan Manual Page</s> · Builder of [`StdVideoEncodeH265ReferenceInfoFlags`]"]
#[repr(transparent)]
pub struct StdVideoEncodeH265ReferenceInfoFlagsBuilder<'a>(StdVideoEncodeH265ReferenceInfoFlags, std::marker::PhantomData<&'a ()>);
impl<'a> StdVideoEncodeH265ReferenceInfoFlagsBuilder<'a> {
    #[inline]
    pub fn new() -> StdVideoEncodeH265ReferenceInfoFlagsBuilder<'a> {
        StdVideoEncodeH265ReferenceInfoFlagsBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn is_long_term(mut self, is_long_term: u32) -> Self {
        self.0.is_long_term_and_is_used_flag = crate::bits_copy!(self.0.is_long_term_and_is_used_flag, is_long_term, 0usize, 0usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn is_used_flag(mut self, is_used_flag: u32) -> Self {
        self.0.is_long_term_and_is_used_flag = crate::bits_copy!(self.0.is_long_term_and_is_used_flag, is_used_flag, 1usize, 1usize);
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> StdVideoEncodeH265ReferenceInfoFlags {
        self.0
    }
}
impl<'a> std::default::Default for StdVideoEncodeH265ReferenceInfoFlagsBuilder<'a> {
    fn default() -> StdVideoEncodeH265ReferenceInfoFlagsBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for StdVideoEncodeH265ReferenceInfoFlagsBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for StdVideoEncodeH265ReferenceInfoFlagsBuilder<'a> {
    type Target = StdVideoEncodeH265ReferenceInfoFlags;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StdVideoEncodeH265ReferenceInfoFlagsBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Structure"]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct StdVideoEncodeH265ReferenceModificationFlags {
    pub ref_pic_list_modification_flag_l0_and_ref_pic_list_modification_flag_l1: u32,
}
impl Default for StdVideoEncodeH265ReferenceModificationFlags {
    fn default() -> Self {
        Self { ref_pic_list_modification_flag_l0_and_ref_pic_list_modification_flag_l1: Default::default() }
    }
}
impl std::fmt::Debug for StdVideoEncodeH265ReferenceModificationFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StdVideoEncodeH265ReferenceModificationFlags").field("ref_pic_list_modification_flag_l0_and_ref_pic_list_modification_flag_l1", &format!("{:#b}", &self.ref_pic_list_modification_flag_l0_and_ref_pic_list_modification_flag_l1)).finish()
    }
}
impl StdVideoEncodeH265ReferenceModificationFlags {
    #[inline]
    pub fn into_builder<'a>(self) -> StdVideoEncodeH265ReferenceModificationFlagsBuilder<'a> {
        StdVideoEncodeH265ReferenceModificationFlagsBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "<s>Vulkan Manual Page</s> · Builder of [`StdVideoEncodeH265ReferenceModificationFlags`]"]
#[repr(transparent)]
pub struct StdVideoEncodeH265ReferenceModificationFlagsBuilder<'a>(StdVideoEncodeH265ReferenceModificationFlags, std::marker::PhantomData<&'a ()>);
impl<'a> StdVideoEncodeH265ReferenceModificationFlagsBuilder<'a> {
    #[inline]
    pub fn new() -> StdVideoEncodeH265ReferenceModificationFlagsBuilder<'a> {
        StdVideoEncodeH265ReferenceModificationFlagsBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn ref_pic_list_modification_flag_l0(mut self, ref_pic_list_modification_flag_l0: u32) -> Self {
        self.0.ref_pic_list_modification_flag_l0_and_ref_pic_list_modification_flag_l1 = crate::bits_copy!(self.0.ref_pic_list_modification_flag_l0_and_ref_pic_list_modification_flag_l1, ref_pic_list_modification_flag_l0, 0usize, 0usize);
        self
    }
    #[inline]
    #[must_use]
    pub fn ref_pic_list_modification_flag_l1(mut self, ref_pic_list_modification_flag_l1: u32) -> Self {
        self.0.ref_pic_list_modification_flag_l0_and_ref_pic_list_modification_flag_l1 = crate::bits_copy!(self.0.ref_pic_list_modification_flag_l0_and_ref_pic_list_modification_flag_l1, ref_pic_list_modification_flag_l1, 1usize, 1usize);
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> StdVideoEncodeH265ReferenceModificationFlags {
        self.0
    }
}
impl<'a> std::default::Default for StdVideoEncodeH265ReferenceModificationFlagsBuilder<'a> {
    fn default() -> StdVideoEncodeH265ReferenceModificationFlagsBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for StdVideoEncodeH265ReferenceModificationFlagsBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for StdVideoEncodeH265ReferenceModificationFlagsBuilder<'a> {
    type Target = StdVideoEncodeH265ReferenceModificationFlags;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StdVideoEncodeH265ReferenceModificationFlagsBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
