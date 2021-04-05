#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_VIEWPORT_SWIZZLE_SPEC_VERSION")]
pub const NV_VIEWPORT_SWIZZLE_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_VIEWPORT_SWIZZLE_EXTENSION_NAME")]
pub const NV_VIEWPORT_SWIZZLE_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_NV_viewport_swizzle");
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineViewportSwizzleStateCreateFlagsNV.html) · Bitmask of [`PipelineViewportSwizzleStateCreateFlagBitsNV`]"] # [doc (alias = "VkPipelineViewportSwizzleStateCreateFlagsNV")] # [derive (Default)] # [repr (transparent)] pub struct PipelineViewportSwizzleStateCreateFlagsNV : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`PipelineViewportSwizzleStateCreateFlagsNV`]"]
#[doc(alias = "VkPipelineViewportSwizzleStateCreateFlagBitsNV")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct PipelineViewportSwizzleStateCreateFlagBitsNV(pub u32);
impl PipelineViewportSwizzleStateCreateFlagBitsNV {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> PipelineViewportSwizzleStateCreateFlagsNV {
        PipelineViewportSwizzleStateCreateFlagsNV::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for PipelineViewportSwizzleStateCreateFlagBitsNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkViewportCoordinateSwizzleNV.html) · Enum"]
#[doc(alias = "VkViewportCoordinateSwizzleNV")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct ViewportCoordinateSwizzleNV(pub i32);
impl std::fmt::Debug for ViewportCoordinateSwizzleNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::POSITIVE_X_NV => "POSITIVE_X_NV",
            &Self::NEGATIVE_X_NV => "NEGATIVE_X_NV",
            &Self::POSITIVE_Y_NV => "POSITIVE_Y_NV",
            &Self::NEGATIVE_Y_NV => "NEGATIVE_Y_NV",
            &Self::POSITIVE_Z_NV => "POSITIVE_Z_NV",
            &Self::NEGATIVE_Z_NV => "NEGATIVE_Z_NV",
            &Self::POSITIVE_W_NV => "POSITIVE_W_NV",
            &Self::NEGATIVE_W_NV => "NEGATIVE_W_NV",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::nv_viewport_swizzle`]"]
impl ViewportCoordinateSwizzleNV {
    pub const POSITIVE_X_NV: Self = Self(0);
    pub const NEGATIVE_X_NV: Self = Self(1);
    pub const POSITIVE_Y_NV: Self = Self(2);
    pub const NEGATIVE_Y_NV: Self = Self(3);
    pub const POSITIVE_Z_NV: Self = Self(4);
    pub const NEGATIVE_Z_NV: Self = Self(5);
    pub const POSITIVE_W_NV: Self = Self(6);
    pub const NEGATIVE_W_NV: Self = Self(7);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkViewportSwizzleNV.html) · Structure"]
#[doc(alias = "VkViewportSwizzleNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ViewportSwizzleNV {
    pub x: crate::extensions::nv_viewport_swizzle::ViewportCoordinateSwizzleNV,
    pub y: crate::extensions::nv_viewport_swizzle::ViewportCoordinateSwizzleNV,
    pub z: crate::extensions::nv_viewport_swizzle::ViewportCoordinateSwizzleNV,
    pub w: crate::extensions::nv_viewport_swizzle::ViewportCoordinateSwizzleNV,
}
impl Default for ViewportSwizzleNV {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
            z: Default::default(),
            w: Default::default(),
        }
    }
}
impl std::fmt::Debug for ViewportSwizzleNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ViewportSwizzleNV").field("x", &self.x).field("y", &self.y).field("z", &self.z).field("w", &self.w).finish()
    }
}
impl ViewportSwizzleNV {
    #[inline]
    pub fn into_builder<'a>(self) -> ViewportSwizzleNVBuilder<'a> {
        ViewportSwizzleNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkViewportSwizzleNV.html) · Builder of [`ViewportSwizzleNV`]"]
#[repr(transparent)]
pub struct ViewportSwizzleNVBuilder<'a>(ViewportSwizzleNV, std::marker::PhantomData<&'a ()>);
impl<'a> ViewportSwizzleNVBuilder<'a> {
    #[inline]
    pub fn new() -> ViewportSwizzleNVBuilder<'a> {
        ViewportSwizzleNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn x(mut self, x: crate::extensions::nv_viewport_swizzle::ViewportCoordinateSwizzleNV) -> Self {
        self.0.x = x as _;
        self
    }
    #[inline]
    pub fn y(mut self, y: crate::extensions::nv_viewport_swizzle::ViewportCoordinateSwizzleNV) -> Self {
        self.0.y = y as _;
        self
    }
    #[inline]
    pub fn z(mut self, z: crate::extensions::nv_viewport_swizzle::ViewportCoordinateSwizzleNV) -> Self {
        self.0.z = z as _;
        self
    }
    #[inline]
    pub fn w(mut self, w: crate::extensions::nv_viewport_swizzle::ViewportCoordinateSwizzleNV) -> Self {
        self.0.w = w as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ViewportSwizzleNV {
        self.0
    }
}
impl<'a> std::default::Default for ViewportSwizzleNVBuilder<'a> {
    fn default() -> ViewportSwizzleNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ViewportSwizzleNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ViewportSwizzleNVBuilder<'a> {
    type Target = ViewportSwizzleNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ViewportSwizzleNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<ViewportSwizzleNV> for ViewportSwizzleNVBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineViewportSwizzleStateCreateInfoNV.html) · Structure"]
#[doc(alias = "VkPipelineViewportSwizzleStateCreateInfoNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineViewportSwizzleStateCreateInfoNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::nv_viewport_swizzle::PipelineViewportSwizzleStateCreateFlagsNV,
    pub viewport_count: u32,
    pub p_viewport_swizzles: *const crate::extensions::nv_viewport_swizzle::ViewportSwizzleNV,
}
impl Default for PipelineViewportSwizzleStateCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV,
            p_next: std::ptr::null(),
            flags: Default::default(),
            viewport_count: Default::default(),
            p_viewport_swizzles: std::ptr::null(),
        }
    }
}
impl std::fmt::Debug for PipelineViewportSwizzleStateCreateInfoNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PipelineViewportSwizzleStateCreateInfoNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("viewport_count", &self.viewport_count)
            .field("p_viewport_swizzles", &self.p_viewport_swizzles)
            .finish()
    }
}
impl PipelineViewportSwizzleStateCreateInfoNV {
    #[inline]
    pub fn into_builder<'a>(self) -> PipelineViewportSwizzleStateCreateInfoNVBuilder<'a> {
        PipelineViewportSwizzleStateCreateInfoNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineViewportSwizzleStateCreateInfoNV.html) · Builder of [`PipelineViewportSwizzleStateCreateInfoNV`]"]
#[repr(transparent)]
pub struct PipelineViewportSwizzleStateCreateInfoNVBuilder<'a>(PipelineViewportSwizzleStateCreateInfoNV, std::marker::PhantomData<&'a ()>);
impl<'a> PipelineViewportSwizzleStateCreateInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineViewportSwizzleStateCreateInfoNVBuilder<'a> {
        PipelineViewportSwizzleStateCreateInfoNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::nv_viewport_swizzle::PipelineViewportSwizzleStateCreateFlagsNV) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn viewport_swizzles(mut self, viewport_swizzles: &'a [impl crate::Repr<crate::extensions::nv_viewport_swizzle::ViewportSwizzleNV>]) -> Self {
        self.0.p_viewport_swizzles = viewport_swizzles.as_ptr() as _;
        self.0.viewport_count = viewport_swizzles.len() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PipelineViewportSwizzleStateCreateInfoNV {
        self.0
    }
}
impl<'a> std::default::Default for PipelineViewportSwizzleStateCreateInfoNVBuilder<'a> {
    fn default() -> PipelineViewportSwizzleStateCreateInfoNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PipelineViewportSwizzleStateCreateInfoNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PipelineViewportSwizzleStateCreateInfoNVBuilder<'a> {
    type Target = PipelineViewportSwizzleStateCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineViewportSwizzleStateCreateInfoNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<PipelineViewportSwizzleStateCreateInfoNV> for PipelineViewportSwizzleStateCreateInfoNVBuilder<'_> {}
