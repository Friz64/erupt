#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_DEPTH_CLIP_CONTROL_SPEC_VERSION")]
pub const EXT_DEPTH_CLIP_CONTROL_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_DEPTH_CLIP_CONTROL_EXTENSION_NAME")]
pub const EXT_DEPTH_CLIP_CONTROL_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_depth_clip_control");
#[doc = "Provided by [`crate::extensions::ext_depth_clip_control`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_DEPTH_CLIP_CONTROL_FEATURES_EXT: Self = Self(1000355000);
    pub const PIPELINE_VIEWPORT_DEPTH_CLIP_CONTROL_CREATE_INFO_EXT: Self = Self(1000355001);
}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceDepthClipControlFeaturesEXT> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceDepthClipControlFeaturesEXTBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PipelineViewportDepthClipControlCreateInfoEXT> for crate::vk1_0::PipelineViewportStateCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PipelineViewportDepthClipControlCreateInfoEXTBuilder<'_>> for crate::vk1_0::PipelineViewportStateCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceDepthClipControlFeaturesEXT> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceDepthClipControlFeaturesEXTBuilder<'_>> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDepthClipControlFeaturesEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceDepthClipControlFeaturesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceDepthClipControlFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub depth_clip_control: crate::vk1_0::Bool32,
}
impl PhysicalDeviceDepthClipControlFeaturesEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_DEPTH_CLIP_CONTROL_FEATURES_EXT;
}
impl Default for PhysicalDeviceDepthClipControlFeaturesEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), depth_clip_control: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceDepthClipControlFeaturesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceDepthClipControlFeaturesEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("depth_clip_control", &(self.depth_clip_control != 0)).finish()
    }
}
impl PhysicalDeviceDepthClipControlFeaturesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceDepthClipControlFeaturesEXTBuilder<'a> {
        PhysicalDeviceDepthClipControlFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDepthClipControlFeaturesEXT.html) · Builder of [`PhysicalDeviceDepthClipControlFeaturesEXT`]"]
#[repr(transparent)]
pub struct PhysicalDeviceDepthClipControlFeaturesEXTBuilder<'a>(PhysicalDeviceDepthClipControlFeaturesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceDepthClipControlFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceDepthClipControlFeaturesEXTBuilder<'a> {
        PhysicalDeviceDepthClipControlFeaturesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn depth_clip_control(mut self, depth_clip_control: bool) -> Self {
        self.0.depth_clip_control = depth_clip_control as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> PhysicalDeviceDepthClipControlFeaturesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceDepthClipControlFeaturesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceDepthClipControlFeaturesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceDepthClipControlFeaturesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceDepthClipControlFeaturesEXTBuilder<'a> {
    type Target = PhysicalDeviceDepthClipControlFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceDepthClipControlFeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineViewportDepthClipControlCreateInfoEXT.html) · Structure"]
#[doc(alias = "VkPipelineViewportDepthClipControlCreateInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineViewportDepthClipControlCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub negative_one_to_one: crate::vk1_0::Bool32,
}
impl PipelineViewportDepthClipControlCreateInfoEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PIPELINE_VIEWPORT_DEPTH_CLIP_CONTROL_CREATE_INFO_EXT;
}
impl Default for PipelineViewportDepthClipControlCreateInfoEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), negative_one_to_one: Default::default() }
    }
}
impl std::fmt::Debug for PipelineViewportDepthClipControlCreateInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PipelineViewportDepthClipControlCreateInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("negative_one_to_one", &(self.negative_one_to_one != 0)).finish()
    }
}
impl PipelineViewportDepthClipControlCreateInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PipelineViewportDepthClipControlCreateInfoEXTBuilder<'a> {
        PipelineViewportDepthClipControlCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineViewportDepthClipControlCreateInfoEXT.html) · Builder of [`PipelineViewportDepthClipControlCreateInfoEXT`]"]
#[repr(transparent)]
pub struct PipelineViewportDepthClipControlCreateInfoEXTBuilder<'a>(PipelineViewportDepthClipControlCreateInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PipelineViewportDepthClipControlCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineViewportDepthClipControlCreateInfoEXTBuilder<'a> {
        PipelineViewportDepthClipControlCreateInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn negative_one_to_one(mut self, negative_one_to_one: bool) -> Self {
        self.0.negative_one_to_one = negative_one_to_one as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> PipelineViewportDepthClipControlCreateInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for PipelineViewportDepthClipControlCreateInfoEXTBuilder<'a> {
    fn default() -> PipelineViewportDepthClipControlCreateInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PipelineViewportDepthClipControlCreateInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PipelineViewportDepthClipControlCreateInfoEXTBuilder<'a> {
    type Target = PipelineViewportDepthClipControlCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineViewportDepthClipControlCreateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}