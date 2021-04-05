#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_PIPELINE_CREATION_FEEDBACK_SPEC_VERSION")]
pub const EXT_PIPELINE_CREATION_FEEDBACK_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_PIPELINE_CREATION_FEEDBACK_EXTENSION_NAME")]
pub const EXT_PIPELINE_CREATION_FEEDBACK_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_pipeline_creation_feedback");
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCreationFeedbackFlagsEXT.html) · Bitmask of [`PipelineCreationFeedbackFlagBitsEXT`]"] # [doc (alias = "VkPipelineCreationFeedbackFlagsEXT")] # [derive (Default)] # [repr (transparent)] pub struct PipelineCreationFeedbackFlagsEXT : u32 { const VALID_EXT = PipelineCreationFeedbackFlagBitsEXT :: VALID_EXT . 0 ; const APPLICATION_PIPELINE_CACHE_HIT_EXT = PipelineCreationFeedbackFlagBitsEXT :: APPLICATION_PIPELINE_CACHE_HIT_EXT . 0 ; const BASE_PIPELINE_ACCELERATION_EXT = PipelineCreationFeedbackFlagBitsEXT :: BASE_PIPELINE_ACCELERATION_EXT . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCreationFeedbackFlagBitsEXT.html) · Bits enum of [`PipelineCreationFeedbackFlagsEXT`]"]
#[doc(alias = "VkPipelineCreationFeedbackFlagBitsEXT")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct PipelineCreationFeedbackFlagBitsEXT(pub u32);
impl PipelineCreationFeedbackFlagBitsEXT {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> PipelineCreationFeedbackFlagsEXT {
        PipelineCreationFeedbackFlagsEXT::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for PipelineCreationFeedbackFlagBitsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::VALID_EXT => "VALID_EXT",
            &Self::APPLICATION_PIPELINE_CACHE_HIT_EXT => "APPLICATION_PIPELINE_CACHE_HIT_EXT",
            &Self::BASE_PIPELINE_ACCELERATION_EXT => "BASE_PIPELINE_ACCELERATION_EXT",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::ext_pipeline_creation_feedback`]"]
impl PipelineCreationFeedbackFlagBitsEXT {
    pub const VALID_EXT: Self = Self(1);
    pub const APPLICATION_PIPELINE_CACHE_HIT_EXT: Self = Self(2);
    pub const BASE_PIPELINE_ACCELERATION_EXT: Self = Self(4);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCreationFeedbackEXT.html) · Structure"]
#[doc(alias = "VkPipelineCreationFeedbackEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineCreationFeedbackEXT {
    pub flags: crate::extensions::ext_pipeline_creation_feedback::PipelineCreationFeedbackFlagsEXT,
    pub duration: u64,
}
impl Default for PipelineCreationFeedbackEXT {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            duration: Default::default(),
        }
    }
}
impl std::fmt::Debug for PipelineCreationFeedbackEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PipelineCreationFeedbackEXT").field("flags", &self.flags).field("duration", &self.duration).finish()
    }
}
impl PipelineCreationFeedbackEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PipelineCreationFeedbackEXTBuilder<'a> {
        PipelineCreationFeedbackEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCreationFeedbackEXT.html) · Builder of [`PipelineCreationFeedbackEXT`]"]
#[repr(transparent)]
pub struct PipelineCreationFeedbackEXTBuilder<'a>(PipelineCreationFeedbackEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PipelineCreationFeedbackEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineCreationFeedbackEXTBuilder<'a> {
        PipelineCreationFeedbackEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::ext_pipeline_creation_feedback::PipelineCreationFeedbackFlagsEXT) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn duration(mut self, duration: u64) -> Self {
        self.0.duration = duration as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PipelineCreationFeedbackEXT {
        self.0
    }
}
impl<'a> std::default::Default for PipelineCreationFeedbackEXTBuilder<'a> {
    fn default() -> PipelineCreationFeedbackEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PipelineCreationFeedbackEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PipelineCreationFeedbackEXTBuilder<'a> {
    type Target = PipelineCreationFeedbackEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineCreationFeedbackEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<PipelineCreationFeedbackEXT> for PipelineCreationFeedbackEXTBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCreationFeedbackCreateInfoEXT.html) · Structure"]
#[doc(alias = "VkPipelineCreationFeedbackCreateInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineCreationFeedbackCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub p_pipeline_creation_feedback: *mut crate::extensions::ext_pipeline_creation_feedback::PipelineCreationFeedbackEXT,
    pub pipeline_stage_creation_feedback_count: u32,
    pub p_pipeline_stage_creation_feedbacks: *mut crate::extensions::ext_pipeline_creation_feedback::PipelineCreationFeedbackEXT,
}
impl Default for PipelineCreationFeedbackCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PIPELINE_CREATION_FEEDBACK_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            p_pipeline_creation_feedback: std::ptr::null_mut(),
            pipeline_stage_creation_feedback_count: Default::default(),
            p_pipeline_stage_creation_feedbacks: std::ptr::null_mut(),
        }
    }
}
impl std::fmt::Debug for PipelineCreationFeedbackCreateInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PipelineCreationFeedbackCreateInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("p_pipeline_creation_feedback", &self.p_pipeline_creation_feedback)
            .field("pipeline_stage_creation_feedback_count", &self.pipeline_stage_creation_feedback_count)
            .field("p_pipeline_stage_creation_feedbacks", &self.p_pipeline_stage_creation_feedbacks)
            .finish()
    }
}
impl PipelineCreationFeedbackCreateInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PipelineCreationFeedbackCreateInfoEXTBuilder<'a> {
        PipelineCreationFeedbackCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCreationFeedbackCreateInfoEXT.html) · Builder of [`PipelineCreationFeedbackCreateInfoEXT`]"]
#[repr(transparent)]
pub struct PipelineCreationFeedbackCreateInfoEXTBuilder<'a>(PipelineCreationFeedbackCreateInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PipelineCreationFeedbackCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineCreationFeedbackCreateInfoEXTBuilder<'a> {
        PipelineCreationFeedbackCreateInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn pipeline_creation_feedback(mut self, pipeline_creation_feedback: &'a mut crate::extensions::ext_pipeline_creation_feedback::PipelineCreationFeedbackEXT) -> Self {
        self.0.p_pipeline_creation_feedback = pipeline_creation_feedback as _;
        self
    }
    #[inline]
    pub fn pipeline_stage_creation_feedbacks(mut self, pipeline_stage_creation_feedbacks: &'a mut [impl crate::Repr<crate::extensions::ext_pipeline_creation_feedback::PipelineCreationFeedbackEXT>]) -> Self {
        self.0.p_pipeline_stage_creation_feedbacks = pipeline_stage_creation_feedbacks.as_ptr() as _;
        self.0.pipeline_stage_creation_feedback_count = pipeline_stage_creation_feedbacks.len() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PipelineCreationFeedbackCreateInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for PipelineCreationFeedbackCreateInfoEXTBuilder<'a> {
    fn default() -> PipelineCreationFeedbackCreateInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PipelineCreationFeedbackCreateInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PipelineCreationFeedbackCreateInfoEXTBuilder<'a> {
    type Target = PipelineCreationFeedbackCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineCreationFeedbackCreateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<PipelineCreationFeedbackCreateInfoEXT> for PipelineCreationFeedbackCreateInfoEXTBuilder<'_> {}
