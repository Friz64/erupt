# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_EXT_pipeline_creation_feedback.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_PIPELINE_CREATION_FEEDBACK_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_PIPELINE_CREATION_FEEDBACK_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_pipeline_creation_feedback");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCreationFeedbackFlagBitsEXT.html) · Flag Bits of [`PipelineCreationFeedbackFlagsEXT`](struct.PipelineCreationFeedbackFlagsEXT.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PipelineCreationFeedbackFlagBitsEXT(pub u32);
impl PipelineCreationFeedbackFlagBitsEXT {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> PipelineCreationFeedbackFlagsEXT {
        PipelineCreationFeedbackFlagsEXT::from_bits_truncate(self.0)
    }
}
#[doc = "[Part of `extensions::ext_pipeline_creation_feedback`](../../extensions/ext_pipeline_creation_feedback/index.html)"]
impl PipelineCreationFeedbackFlagBitsEXT {
    pub const VALID_EXT: Self = Self(0x00000001);
    pub const APPLICATION_PIPELINE_CACHE_HIT_EXT: Self = Self(0x00000002);
    pub const BASE_PIPELINE_ACCELERATION_EXT: Self = Self(0x00000004);
}
impl std::fmt::Debug for PipelineCreationFeedbackFlagBitsEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::VALID_EXT => "VALID_EXT",
            &Self::APPLICATION_PIPELINE_CACHE_HIT_EXT => "APPLICATION_PIPELINE_CACHE_HIT_EXT",
            &Self::BASE_PIPELINE_ACCELERATION_EXT => "BASE_PIPELINE_ACCELERATION_EXT",
            _ => "(unknown)",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCreationFeedbackFlagsEXT.html) · Flags of [`PipelineCreationFeedbackFlagBitsEXT`](struct.PipelineCreationFeedbackFlagBitsEXT.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct PipelineCreationFeedbackFlagsEXT : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; const VALID_EXT = PipelineCreationFeedbackFlagBitsEXT :: VALID_EXT . 0 ; const APPLICATION_PIPELINE_CACHE_HIT_EXT = PipelineCreationFeedbackFlagBitsEXT :: APPLICATION_PIPELINE_CACHE_HIT_EXT . 0 ; const BASE_PIPELINE_ACCELERATION_EXT = PipelineCreationFeedbackFlagBitsEXT :: BASE_PIPELINE_ACCELERATION_EXT . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCreationFeedbackCreateInfoEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineCreationFeedbackCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub p_pipeline_creation_feedback:
        *mut crate::extensions::ext_pipeline_creation_feedback::PipelineCreationFeedbackEXT,
    pub pipeline_stage_creation_feedback_count: u32,
    pub p_pipeline_stage_creation_feedbacks:
        *mut crate::extensions::ext_pipeline_creation_feedback::PipelineCreationFeedbackEXT,
}
impl PipelineCreationFeedbackCreateInfoEXT {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: crate::ExtendableBy<Self>,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PipelineCreationFeedbackCreateInfoEXTBuilder<'a> {
        PipelineCreationFeedbackCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PipelineCreationFeedbackCreateInfoEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PipelineCreationFeedbackCreateInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "p_pipeline_creation_feedback",
                &self.p_pipeline_creation_feedback,
            )
            .field(
                "pipeline_stage_creation_feedback_count",
                &self.pipeline_stage_creation_feedback_count,
            )
            .field(
                "p_pipeline_stage_creation_feedbacks",
                &self.p_pipeline_stage_creation_feedbacks,
            )
            .finish()
    }
}
impl Default for PipelineCreationFeedbackCreateInfoEXT {
    fn default() -> PipelineCreationFeedbackCreateInfoEXT {
        PipelineCreationFeedbackCreateInfoEXT {
            s_type: crate::vk1_0::StructureType::PIPELINE_CREATION_FEEDBACK_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            p_pipeline_creation_feedback: std::ptr::null_mut(),
            pipeline_stage_creation_feedback_count: Default::default(),
            p_pipeline_stage_creation_feedbacks: std::ptr::null_mut(),
        }
    }
}
impl crate::ExtendableBy<PipelineCreationFeedbackCreateInfoEXT>
    for crate::vk1_0::GraphicsPipelineCreateInfo
{
}
impl crate::ExtendableBy<PipelineCreationFeedbackCreateInfoEXT>
    for crate::vk1_0::ComputePipelineCreateInfo
{
}
impl crate::ExtendableBy<PipelineCreationFeedbackCreateInfoEXT>
    for crate::extensions::nv_ray_tracing::RayTracingPipelineCreateInfoNV
{
}
impl crate::ExtendableBy<PipelineCreationFeedbackCreateInfoEXT>
    for crate::extensions::khr_ray_tracing::RayTracingPipelineCreateInfoKHR
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PipelineCreationFeedbackCreateInfoEXT`](struct.PipelineCreationFeedbackCreateInfoEXT.html)"]
#[repr(transparent)]
pub struct PipelineCreationFeedbackCreateInfoEXTBuilder<'a>(
    PipelineCreationFeedbackCreateInfoEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PipelineCreationFeedbackCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineCreationFeedbackCreateInfoEXTBuilder<'a> {
        PipelineCreationFeedbackCreateInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn pipeline_creation_feedback(
        mut self,
        pipeline_creation_feedback : &'a mut crate :: extensions :: ext_pipeline_creation_feedback :: PipelineCreationFeedbackEXT,
    ) -> Self {
        self.0.p_pipeline_creation_feedback = pipeline_creation_feedback;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn pipeline_stage_creation_feedbacks(
        mut self,
        pipeline_stage_creation_feedbacks : &'a mut [ crate :: extensions :: ext_pipeline_creation_feedback :: PipelineCreationFeedbackEXTBuilder ],
    ) -> Self {
        self.0.pipeline_stage_creation_feedback_count =
            pipeline_stage_creation_feedbacks.len() as _;
        self.0.p_pipeline_stage_creation_feedbacks =
            pipeline_stage_creation_feedbacks.as_mut_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PipelineCreationFeedbackCreateInfoEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for PipelineCreationFeedbackCreateInfoEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCreationFeedbackEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineCreationFeedbackEXT {
    pub flags: crate::extensions::ext_pipeline_creation_feedback::PipelineCreationFeedbackFlagsEXT,
    pub duration: u64,
}
impl PipelineCreationFeedbackEXT {
    #[inline]
    pub fn builder<'a>(self) -> PipelineCreationFeedbackEXTBuilder<'a> {
        PipelineCreationFeedbackEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PipelineCreationFeedbackEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PipelineCreationFeedbackEXT")
            .field("flags", &self.flags)
            .field("duration", &self.duration)
            .finish()
    }
}
impl Default for PipelineCreationFeedbackEXT {
    fn default() -> PipelineCreationFeedbackEXT {
        PipelineCreationFeedbackEXT {
            flags: Default::default(),
            duration: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PipelineCreationFeedbackEXT`](struct.PipelineCreationFeedbackEXT.html)"]
#[repr(transparent)]
pub struct PipelineCreationFeedbackEXTBuilder<'a>(
    PipelineCreationFeedbackEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PipelineCreationFeedbackEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineCreationFeedbackEXTBuilder<'a> {
        PipelineCreationFeedbackEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn flags(
        mut self,
        flags: crate::extensions::ext_pipeline_creation_feedback::PipelineCreationFeedbackFlagsEXT,
    ) -> Self {
        self.0.flags = flags;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn duration(mut self, duration: u64) -> Self {
        self.0.duration = duration;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PipelineCreationFeedbackEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for PipelineCreationFeedbackEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
