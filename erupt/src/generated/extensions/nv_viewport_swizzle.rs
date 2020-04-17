# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_NV_viewport_swizzle.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NV_VIEWPORT_SWIZZLE_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NV_VIEWPORT_SWIZZLE_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_NV_viewport_swizzle");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkViewportSwizzleNV.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ViewportSwizzleNV {
    pub x: crate::extensions::nv_viewport_swizzle::ViewportCoordinateSwizzleNV,
    pub y: crate::extensions::nv_viewport_swizzle::ViewportCoordinateSwizzleNV,
    pub z: crate::extensions::nv_viewport_swizzle::ViewportCoordinateSwizzleNV,
    pub w: crate::extensions::nv_viewport_swizzle::ViewportCoordinateSwizzleNV,
}
impl ViewportSwizzleNV {
    #[inline]
    pub fn builder<'a>(self) -> ViewportSwizzleNVBuilder<'a> {
        ViewportSwizzleNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ViewportSwizzleNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ViewportSwizzleNV")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .field("w", &self.w)
            .finish()
    }
}
impl Default for ViewportSwizzleNV {
    fn default() -> ViewportSwizzleNV {
        ViewportSwizzleNV {
            x: Default::default(),
            y: Default::default(),
            z: Default::default(),
            w: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`ViewportSwizzleNV`](struct.ViewportSwizzleNV.html)"]
#[repr(transparent)]
pub struct ViewportSwizzleNVBuilder<'a>(ViewportSwizzleNV, std::marker::PhantomData<&'a ()>);
impl<'a> ViewportSwizzleNVBuilder<'a> {
    #[inline]
    pub fn new() -> ViewportSwizzleNVBuilder<'a> {
        ViewportSwizzleNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn x(
        mut self,
        x: crate::extensions::nv_viewport_swizzle::ViewportCoordinateSwizzleNV,
    ) -> Self {
        self.0.x = x;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn y(
        mut self,
        y: crate::extensions::nv_viewport_swizzle::ViewportCoordinateSwizzleNV,
    ) -> Self {
        self.0.y = y;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn z(
        mut self,
        z: crate::extensions::nv_viewport_swizzle::ViewportCoordinateSwizzleNV,
    ) -> Self {
        self.0.z = z;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn w(
        mut self,
        w: crate::extensions::nv_viewport_swizzle::ViewportCoordinateSwizzleNV,
    ) -> Self {
        self.0.w = w;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> ViewportSwizzleNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for ViewportSwizzleNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkViewportCoordinateSwizzleNV.html) · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ViewportCoordinateSwizzleNV(pub i32);
#[doc = "[Part of `extensions::nv_viewport_swizzle`](../../extensions/nv_viewport_swizzle/index.html)"]
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
impl std::fmt::Debug for ViewportCoordinateSwizzleNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::POSITIVE_X_NV => "POSITIVE_X_NV",
            &Self::NEGATIVE_X_NV => "NEGATIVE_X_NV",
            &Self::POSITIVE_Y_NV => "POSITIVE_Y_NV",
            &Self::NEGATIVE_Y_NV => "NEGATIVE_Y_NV",
            &Self::POSITIVE_Z_NV => "POSITIVE_Z_NV",
            &Self::NEGATIVE_Z_NV => "NEGATIVE_Z_NV",
            &Self::POSITIVE_W_NV => "POSITIVE_W_NV",
            &Self::NEGATIVE_W_NV => "NEGATIVE_W_NV",
            _ => "(Unknown)",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineViewportSwizzleStateCreateInfoNV.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineViewportSwizzleStateCreateInfoNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::nv_viewport_swizzle::PipelineViewportSwizzleStateCreateFlagsNV,
    pub viewport_count: u32,
    pub p_viewport_swizzles: *const crate::extensions::nv_viewport_swizzle::ViewportSwizzleNV,
}
impl PipelineViewportSwizzleStateCreateInfoNV {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPipelineViewportSwizzleStateCreateInfoNV,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PipelineViewportSwizzleStateCreateInfoNVBuilder<'a> {
        PipelineViewportSwizzleStateCreateInfoNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PipelineViewportSwizzleStateCreateInfoNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PipelineViewportSwizzleStateCreateInfoNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("viewport_count", &self.viewport_count)
            .field("p_viewport_swizzles", &self.p_viewport_swizzles)
            .finish()
    }
}
impl Default for PipelineViewportSwizzleStateCreateInfoNV {
    fn default() -> PipelineViewportSwizzleStateCreateInfoNV {
        PipelineViewportSwizzleStateCreateInfoNV {
            s_type: crate::vk1_0::StructureType::PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV,
            p_next: std::ptr::null(),
            flags: Default::default(),
            viewport_count: Default::default(),
            p_viewport_swizzles: std::ptr::null(),
        }
    }
}
#[doc = "Used by [`PipelineViewportSwizzleStateCreateInfoNV::extend`](struct.PipelineViewportSwizzleStateCreateInfoNV.html#method.extend)"]
pub trait ExtendableByPipelineViewportSwizzleStateCreateInfoNV {}
impl ExtendableByPipelineViewportSwizzleStateCreateInfoNV
    for crate::vk1_0::PipelineViewportStateCreateInfo
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PipelineViewportSwizzleStateCreateInfoNV`](struct.PipelineViewportSwizzleStateCreateInfoNV.html)"]
#[repr(transparent)]
pub struct PipelineViewportSwizzleStateCreateInfoNVBuilder<'a>(
    PipelineViewportSwizzleStateCreateInfoNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PipelineViewportSwizzleStateCreateInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineViewportSwizzleStateCreateInfoNVBuilder<'a> {
        PipelineViewportSwizzleStateCreateInfoNVBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn flags(
        mut self,
        flags: crate::extensions::nv_viewport_swizzle::PipelineViewportSwizzleStateCreateFlagsNV,
    ) -> Self {
        self.0.flags = flags;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn viewport_swizzles(
        mut self,
        viewport_swizzles: &'a [crate::extensions::nv_viewport_swizzle::ViewportSwizzleNVBuilder],
    ) -> Self {
        self.0.viewport_count = viewport_swizzles.len() as _;
        self.0.p_viewport_swizzles = viewport_swizzles.as_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PipelineViewportSwizzleStateCreateInfoNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for PipelineViewportSwizzleStateCreateInfoNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "<s>Vulkan Manual Page</s> · Flag Bits of [`PipelineViewportSwizzleStateCreateFlagsNV`](struct.PipelineViewportSwizzleStateCreateFlagsNV.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
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
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            _ => "(Unknown)",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineViewportSwizzleStateCreateFlagsNV.html) · Flags of [`PipelineViewportSwizzleStateCreateFlagBitsNV`](struct.PipelineViewportSwizzleStateCreateFlagBitsNV.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct PipelineViewportSwizzleStateCreateFlagsNV : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
