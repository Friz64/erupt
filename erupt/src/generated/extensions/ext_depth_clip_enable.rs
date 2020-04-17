# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_EXT_depth_clip_enable.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_DEPTH_CLIP_ENABLE_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_DEPTH_CLIP_ENABLE_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_depth_clip_enable");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDepthClipEnableFeaturesEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceDepthClipEnableFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub depth_clip_enable: crate::vk1_0::Bool32,
}
impl PhysicalDeviceDepthClipEnableFeaturesEXT {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceDepthClipEnableFeaturesEXT,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceDepthClipEnableFeaturesEXTBuilder<'a> {
        PhysicalDeviceDepthClipEnableFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceDepthClipEnableFeaturesEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceDepthClipEnableFeaturesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("depth_clip_enable", &(self.depth_clip_enable != 0))
            .finish()
    }
}
impl Default for PhysicalDeviceDepthClipEnableFeaturesEXT {
    fn default() -> PhysicalDeviceDepthClipEnableFeaturesEXT {
        PhysicalDeviceDepthClipEnableFeaturesEXT {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_DEPTH_CLIP_ENABLE_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            depth_clip_enable: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceDepthClipEnableFeaturesEXT::extend`](struct.PhysicalDeviceDepthClipEnableFeaturesEXT.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceDepthClipEnableFeaturesEXT {}
impl ExtendableByPhysicalDeviceDepthClipEnableFeaturesEXT
    for crate::vk1_1::PhysicalDeviceFeatures2
{
}
impl ExtendableByPhysicalDeviceDepthClipEnableFeaturesEXT for crate::vk1_0::DeviceCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceDepthClipEnableFeaturesEXT`](struct.PhysicalDeviceDepthClipEnableFeaturesEXT.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceDepthClipEnableFeaturesEXTBuilder<'a>(
    PhysicalDeviceDepthClipEnableFeaturesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceDepthClipEnableFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceDepthClipEnableFeaturesEXTBuilder<'a> {
        PhysicalDeviceDepthClipEnableFeaturesEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn depth_clip_enable(mut self, depth_clip_enable: bool) -> Self {
        self.0.depth_clip_enable = depth_clip_enable as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceDepthClipEnableFeaturesEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceDepthClipEnableFeaturesEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceDepthClipEnableFeaturesEXTBuilder<'a> {
    type Target = PhysicalDeviceDepthClipEnableFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceDepthClipEnableFeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineRasterizationDepthClipStateCreateInfoEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineRasterizationDepthClipStateCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags:
        crate::extensions::ext_depth_clip_enable::PipelineRasterizationDepthClipStateCreateFlagsEXT,
    pub depth_clip_enable: crate::vk1_0::Bool32,
}
impl PipelineRasterizationDepthClipStateCreateInfoEXT {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPipelineRasterizationDepthClipStateCreateInfoEXT,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PipelineRasterizationDepthClipStateCreateInfoEXTBuilder<'a> {
        PipelineRasterizationDepthClipStateCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PipelineRasterizationDepthClipStateCreateInfoEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PipelineRasterizationDepthClipStateCreateInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("depth_clip_enable", &(self.depth_clip_enable != 0))
            .finish()
    }
}
impl Default for PipelineRasterizationDepthClipStateCreateInfoEXT {
    fn default() -> PipelineRasterizationDepthClipStateCreateInfoEXT {
        PipelineRasterizationDepthClipStateCreateInfoEXT {
            s_type:
                crate::vk1_0::StructureType::PIPELINE_RASTERIZATION_DEPTH_CLIP_STATE_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            flags: Default::default(),
            depth_clip_enable: Default::default(),
        }
    }
}
#[doc = "Used by [`PipelineRasterizationDepthClipStateCreateInfoEXT::extend`](struct.PipelineRasterizationDepthClipStateCreateInfoEXT.html#method.extend)"]
pub trait ExtendableByPipelineRasterizationDepthClipStateCreateInfoEXT {}
impl ExtendableByPipelineRasterizationDepthClipStateCreateInfoEXT
    for crate::vk1_0::PipelineRasterizationStateCreateInfo
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PipelineRasterizationDepthClipStateCreateInfoEXT`](struct.PipelineRasterizationDepthClipStateCreateInfoEXT.html)"]
#[repr(transparent)]
pub struct PipelineRasterizationDepthClipStateCreateInfoEXTBuilder<'a>(
    PipelineRasterizationDepthClipStateCreateInfoEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PipelineRasterizationDepthClipStateCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineRasterizationDepthClipStateCreateInfoEXTBuilder<'a> {
        PipelineRasterizationDepthClipStateCreateInfoEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn flags(
        mut self,
        flags : crate :: extensions :: ext_depth_clip_enable :: PipelineRasterizationDepthClipStateCreateFlagsEXT,
    ) -> Self {
        self.0.flags = flags;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn depth_clip_enable(mut self, depth_clip_enable: bool) -> Self {
        self.0.depth_clip_enable = depth_clip_enable as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PipelineRasterizationDepthClipStateCreateInfoEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for PipelineRasterizationDepthClipStateCreateInfoEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PipelineRasterizationDepthClipStateCreateInfoEXTBuilder<'a> {
    type Target = PipelineRasterizationDepthClipStateCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineRasterizationDepthClipStateCreateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Flag Bits of [`PipelineRasterizationDepthClipStateCreateFlagsEXT`](struct.PipelineRasterizationDepthClipStateCreateFlagsEXT.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PipelineRasterizationDepthClipStateCreateFlagBitsEXT(pub u32);
impl PipelineRasterizationDepthClipStateCreateFlagBitsEXT {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> PipelineRasterizationDepthClipStateCreateFlagsEXT {
        PipelineRasterizationDepthClipStateCreateFlagsEXT::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for PipelineRasterizationDepthClipStateCreateFlagBitsEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            _ => "(Unknown)",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineRasterizationDepthClipStateCreateFlagsEXT.html) · Flags of [`PipelineRasterizationDepthClipStateCreateFlagBitsEXT`](struct.PipelineRasterizationDepthClipStateCreateFlagBitsEXT.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct PipelineRasterizationDepthClipStateCreateFlagsEXT : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
