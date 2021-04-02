#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_DEPTH_CLIP_ENABLE_SPEC_VERSION")]
pub const EXT_DEPTH_CLIP_ENABLE_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_DEPTH_CLIP_ENABLE_EXTENSION_NAME")]
pub const EXT_DEPTH_CLIP_ENABLE_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_depth_clip_enable");
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineRasterizationDepthClipStateCreateFlagsEXT.html) · Bitmask of [`PipelineRasterizationDepthClipStateCreateFlagBitsEXT`]"] # [doc (alias = "VkPipelineRasterizationDepthClipStateCreateFlagsEXT")] # [derive (Default)] # [repr (transparent)] pub struct PipelineRasterizationDepthClipStateCreateFlagsEXT : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`PipelineRasterizationDepthClipStateCreateFlagsEXT`]"]
#[doc(alias = "VkPipelineRasterizationDepthClipStateCreateFlagBitsEXT")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
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
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDepthClipEnableFeaturesEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceDepthClipEnableFeaturesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceDepthClipEnableFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub depth_clip_enable: crate::vk1_0::Bool32,
}
impl Default for PhysicalDeviceDepthClipEnableFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_DEPTH_CLIP_ENABLE_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            depth_clip_enable: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceDepthClipEnableFeaturesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceDepthClipEnableFeaturesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("depth_clip_enable", &(self.depth_clip_enable != 0))
            .finish()
    }
}
impl PhysicalDeviceDepthClipEnableFeaturesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceDepthClipEnableFeaturesEXTBuilder<'a> {
        PhysicalDeviceDepthClipEnableFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDepthClipEnableFeaturesEXT.html) · Builder of [`PhysicalDeviceDepthClipEnableFeaturesEXT`]"]
#[repr(transparent)]
pub struct PhysicalDeviceDepthClipEnableFeaturesEXTBuilder<'a>(PhysicalDeviceDepthClipEnableFeaturesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceDepthClipEnableFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceDepthClipEnableFeaturesEXTBuilder<'a> {
        PhysicalDeviceDepthClipEnableFeaturesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn depth_clip_enable(mut self, depth_clip_enable: bool) -> Self {
        self.0.depth_clip_enable = depth_clip_enable as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceDepthClipEnableFeaturesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceDepthClipEnableFeaturesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceDepthClipEnableFeaturesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceDepthClipEnableFeaturesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
unsafe impl crate::Repr<PhysicalDeviceDepthClipEnableFeaturesEXT> for PhysicalDeviceDepthClipEnableFeaturesEXTBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineRasterizationDepthClipStateCreateInfoEXT.html) · Structure"]
#[doc(alias = "VkPipelineRasterizationDepthClipStateCreateInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineRasterizationDepthClipStateCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::ext_depth_clip_enable::PipelineRasterizationDepthClipStateCreateFlagsEXT,
    pub depth_clip_enable: crate::vk1_0::Bool32,
}
impl Default for PipelineRasterizationDepthClipStateCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PIPELINE_RASTERIZATION_DEPTH_CLIP_STATE_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            flags: Default::default(),
            depth_clip_enable: Default::default(),
        }
    }
}
impl std::fmt::Debug for PipelineRasterizationDepthClipStateCreateInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PipelineRasterizationDepthClipStateCreateInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("depth_clip_enable", &(self.depth_clip_enable != 0))
            .finish()
    }
}
impl PipelineRasterizationDepthClipStateCreateInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PipelineRasterizationDepthClipStateCreateInfoEXTBuilder<'a> {
        PipelineRasterizationDepthClipStateCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineRasterizationDepthClipStateCreateInfoEXT.html) · Builder of [`PipelineRasterizationDepthClipStateCreateInfoEXT`]"]
#[repr(transparent)]
pub struct PipelineRasterizationDepthClipStateCreateInfoEXTBuilder<'a>(PipelineRasterizationDepthClipStateCreateInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PipelineRasterizationDepthClipStateCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineRasterizationDepthClipStateCreateInfoEXTBuilder<'a> {
        PipelineRasterizationDepthClipStateCreateInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::ext_depth_clip_enable::PipelineRasterizationDepthClipStateCreateFlagsEXT) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn depth_clip_enable(mut self, depth_clip_enable: bool) -> Self {
        self.0.depth_clip_enable = depth_clip_enable as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PipelineRasterizationDepthClipStateCreateInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for PipelineRasterizationDepthClipStateCreateInfoEXTBuilder<'a> {
    fn default() -> PipelineRasterizationDepthClipStateCreateInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PipelineRasterizationDepthClipStateCreateInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
unsafe impl crate::Repr<PipelineRasterizationDepthClipStateCreateInfoEXT> for PipelineRasterizationDepthClipStateCreateInfoEXTBuilder<'_> {}
