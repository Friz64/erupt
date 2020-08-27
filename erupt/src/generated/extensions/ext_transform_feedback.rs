#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_TRANSFORM_FEEDBACK_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_TRANSFORM_FEEDBACK_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_transform_feedback");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_BIND_TRANSFORM_FEEDBACK_BUFFERS_EXT: *const std::os::raw::c_char =
    crate::cstr!("vkCmdBindTransformFeedbackBuffersEXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_BEGIN_TRANSFORM_FEEDBACK_EXT: *const std::os::raw::c_char =
    crate::cstr!("vkCmdBeginTransformFeedbackEXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_END_TRANSFORM_FEEDBACK_EXT: *const std::os::raw::c_char =
    crate::cstr!("vkCmdEndTransformFeedbackEXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_BEGIN_QUERY_INDEXED_EXT: *const std::os::raw::c_char =
    crate::cstr!("vkCmdBeginQueryIndexedEXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_END_QUERY_INDEXED_EXT: *const std::os::raw::c_char =
    crate::cstr!("vkCmdEndQueryIndexedEXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_DRAW_INDIRECT_BYTE_COUNT_EXT: *const std::os::raw::c_char =
    crate::cstr!("vkCmdDrawIndirectByteCountEXT");
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineRasterizationStateStreamCreateFlagsEXT.html) · Bitmask of [`PipelineRasterizationStateStreamCreateFlagBitsEXT`](./struct.PipelineRasterizationStateStreamCreateFlagBitsEXT.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct PipelineRasterizationStateStreamCreateFlagsEXT : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`PipelineRasterizationStateStreamCreateFlagsEXT`](./struct.PipelineRasterizationStateStreamCreateFlagsEXT.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct PipelineRasterizationStateStreamCreateFlagBitsEXT(pub u32);
impl PipelineRasterizationStateStreamCreateFlagBitsEXT {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> PipelineRasterizationStateStreamCreateFlagsEXT {
        PipelineRasterizationStateStreamCreateFlagsEXT::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for PipelineRasterizationStateStreamCreateFlagBitsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBindTransformFeedbackBuffersEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindTransformFeedbackBuffersEXT = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    first_binding: u32,
    binding_count: u32,
    p_buffers: *const crate::vk1_0::Buffer,
    p_offsets: *const crate::vk1_0::DeviceSize,
    p_sizes: *const crate::vk1_0::DeviceSize,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBeginTransformFeedbackEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBeginTransformFeedbackEXT = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    first_counter_buffer: u32,
    counter_buffer_count: u32,
    p_counter_buffers: *const crate::vk1_0::Buffer,
    p_counter_buffer_offsets: *const crate::vk1_0::DeviceSize,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdEndTransformFeedbackEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdEndTransformFeedbackEXT = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    first_counter_buffer: u32,
    counter_buffer_count: u32,
    p_counter_buffers: *const crate::vk1_0::Buffer,
    p_counter_buffer_offsets: *const crate::vk1_0::DeviceSize,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBeginQueryIndexedEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBeginQueryIndexedEXT = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    query_pool: crate::vk1_0::QueryPool,
    query: u32,
    flags: crate::vk1_0::QueryControlFlags,
    index: u32,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdEndQueryIndexedEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdEndQueryIndexedEXT = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    query_pool: crate::vk1_0::QueryPool,
    query: u32,
    index: u32,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawIndirectByteCountEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawIndirectByteCountEXT = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    instance_count: u32,
    first_instance: u32,
    counter_buffer: crate::vk1_0::Buffer,
    counter_buffer_offset: crate::vk1_0::DeviceSize,
    counter_offset: u32,
    vertex_stride: u32,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceTransformFeedbackFeaturesEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceTransformFeedbackFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub transform_feedback: crate::vk1_0::Bool32,
    pub geometry_streams: crate::vk1_0::Bool32,
}
impl Default for PhysicalDeviceTransformFeedbackFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            transform_feedback: Default::default(),
            geometry_streams: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceTransformFeedbackFeaturesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceTransformFeedbackFeaturesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("transform_feedback", &(self.transform_feedback != 0))
            .field("geometry_streams", &(self.geometry_streams != 0))
            .finish()
    }
}
impl PhysicalDeviceTransformFeedbackFeaturesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceTransformFeedbackFeaturesEXTBuilder<'a> {
        PhysicalDeviceTransformFeedbackFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceTransformFeedbackFeaturesEXT.html) · Builder of [`PhysicalDeviceTransformFeedbackFeaturesEXT`](struct.PhysicalDeviceTransformFeedbackFeaturesEXT.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceTransformFeedbackFeaturesEXTBuilder<'a>(
    PhysicalDeviceTransformFeedbackFeaturesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceTransformFeedbackFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceTransformFeedbackFeaturesEXTBuilder<'a> {
        PhysicalDeviceTransformFeedbackFeaturesEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[inline]
    pub fn transform_feedback(mut self, transform_feedback: bool) -> Self {
        self.0.transform_feedback = transform_feedback as _;
        self
    }
    #[inline]
    pub fn geometry_streams(mut self, geometry_streams: bool) -> Self {
        self.0.geometry_streams = geometry_streams as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceTransformFeedbackFeaturesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceTransformFeedbackFeaturesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceTransformFeedbackFeaturesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceTransformFeedbackFeaturesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceTransformFeedbackFeaturesEXTBuilder<'a> {
    type Target = PhysicalDeviceTransformFeedbackFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceTransformFeedbackFeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceTransformFeedbackPropertiesEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceTransformFeedbackPropertiesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub max_transform_feedback_streams: u32,
    pub max_transform_feedback_buffers: u32,
    pub max_transform_feedback_buffer_size: crate::vk1_0::DeviceSize,
    pub max_transform_feedback_stream_data_size: u32,
    pub max_transform_feedback_buffer_data_size: u32,
    pub max_transform_feedback_buffer_data_stride: u32,
    pub transform_feedback_queries: crate::vk1_0::Bool32,
    pub transform_feedback_streams_lines_triangles: crate::vk1_0::Bool32,
    pub transform_feedback_rasterization_stream_select: crate::vk1_0::Bool32,
    pub transform_feedback_draw: crate::vk1_0::Bool32,
}
impl Default for PhysicalDeviceTransformFeedbackPropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            max_transform_feedback_streams: Default::default(),
            max_transform_feedback_buffers: Default::default(),
            max_transform_feedback_buffer_size: Default::default(),
            max_transform_feedback_stream_data_size: Default::default(),
            max_transform_feedback_buffer_data_size: Default::default(),
            max_transform_feedback_buffer_data_stride: Default::default(),
            transform_feedback_queries: Default::default(),
            transform_feedback_streams_lines_triangles: Default::default(),
            transform_feedback_rasterization_stream_select: Default::default(),
            transform_feedback_draw: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceTransformFeedbackPropertiesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceTransformFeedbackPropertiesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "max_transform_feedback_streams",
                &self.max_transform_feedback_streams,
            )
            .field(
                "max_transform_feedback_buffers",
                &self.max_transform_feedback_buffers,
            )
            .field(
                "max_transform_feedback_buffer_size",
                &self.max_transform_feedback_buffer_size,
            )
            .field(
                "max_transform_feedback_stream_data_size",
                &self.max_transform_feedback_stream_data_size,
            )
            .field(
                "max_transform_feedback_buffer_data_size",
                &self.max_transform_feedback_buffer_data_size,
            )
            .field(
                "max_transform_feedback_buffer_data_stride",
                &self.max_transform_feedback_buffer_data_stride,
            )
            .field(
                "transform_feedback_queries",
                &(self.transform_feedback_queries != 0),
            )
            .field(
                "transform_feedback_streams_lines_triangles",
                &(self.transform_feedback_streams_lines_triangles != 0),
            )
            .field(
                "transform_feedback_rasterization_stream_select",
                &(self.transform_feedback_rasterization_stream_select != 0),
            )
            .field(
                "transform_feedback_draw",
                &(self.transform_feedback_draw != 0),
            )
            .finish()
    }
}
impl PhysicalDeviceTransformFeedbackPropertiesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceTransformFeedbackPropertiesEXTBuilder<'a> {
        PhysicalDeviceTransformFeedbackPropertiesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceTransformFeedbackPropertiesEXT.html) · Builder of [`PhysicalDeviceTransformFeedbackPropertiesEXT`](struct.PhysicalDeviceTransformFeedbackPropertiesEXT.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceTransformFeedbackPropertiesEXTBuilder<'a>(
    PhysicalDeviceTransformFeedbackPropertiesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceTransformFeedbackPropertiesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceTransformFeedbackPropertiesEXTBuilder<'a> {
        PhysicalDeviceTransformFeedbackPropertiesEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[inline]
    pub fn max_transform_feedback_streams(mut self, max_transform_feedback_streams: u32) -> Self {
        self.0.max_transform_feedback_streams = max_transform_feedback_streams as _;
        self
    }
    #[inline]
    pub fn max_transform_feedback_buffers(mut self, max_transform_feedback_buffers: u32) -> Self {
        self.0.max_transform_feedback_buffers = max_transform_feedback_buffers as _;
        self
    }
    #[inline]
    pub fn max_transform_feedback_buffer_size(
        mut self,
        max_transform_feedback_buffer_size: crate::vk1_0::DeviceSize,
    ) -> Self {
        self.0.max_transform_feedback_buffer_size = max_transform_feedback_buffer_size as _;
        self
    }
    #[inline]
    pub fn max_transform_feedback_stream_data_size(
        mut self,
        max_transform_feedback_stream_data_size: u32,
    ) -> Self {
        self.0.max_transform_feedback_stream_data_size =
            max_transform_feedback_stream_data_size as _;
        self
    }
    #[inline]
    pub fn max_transform_feedback_buffer_data_size(
        mut self,
        max_transform_feedback_buffer_data_size: u32,
    ) -> Self {
        self.0.max_transform_feedback_buffer_data_size =
            max_transform_feedback_buffer_data_size as _;
        self
    }
    #[inline]
    pub fn max_transform_feedback_buffer_data_stride(
        mut self,
        max_transform_feedback_buffer_data_stride: u32,
    ) -> Self {
        self.0.max_transform_feedback_buffer_data_stride =
            max_transform_feedback_buffer_data_stride as _;
        self
    }
    #[inline]
    pub fn transform_feedback_queries(mut self, transform_feedback_queries: bool) -> Self {
        self.0.transform_feedback_queries = transform_feedback_queries as _;
        self
    }
    #[inline]
    pub fn transform_feedback_streams_lines_triangles(
        mut self,
        transform_feedback_streams_lines_triangles: bool,
    ) -> Self {
        self.0.transform_feedback_streams_lines_triangles =
            transform_feedback_streams_lines_triangles as _;
        self
    }
    #[inline]
    pub fn transform_feedback_rasterization_stream_select(
        mut self,
        transform_feedback_rasterization_stream_select: bool,
    ) -> Self {
        self.0.transform_feedback_rasterization_stream_select =
            transform_feedback_rasterization_stream_select as _;
        self
    }
    #[inline]
    pub fn transform_feedback_draw(mut self, transform_feedback_draw: bool) -> Self {
        self.0.transform_feedback_draw = transform_feedback_draw as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceTransformFeedbackPropertiesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceTransformFeedbackPropertiesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceTransformFeedbackPropertiesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceTransformFeedbackPropertiesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceTransformFeedbackPropertiesEXTBuilder<'a> {
    type Target = PhysicalDeviceTransformFeedbackPropertiesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceTransformFeedbackPropertiesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineRasterizationStateStreamCreateInfoEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineRasterizationStateStreamCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags:
        crate::extensions::ext_transform_feedback::PipelineRasterizationStateStreamCreateFlagsEXT,
    pub rasterization_stream: u32,
}
impl Default for PipelineRasterizationStateStreamCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type:
                crate::vk1_0::StructureType::PIPELINE_RASTERIZATION_STATE_STREAM_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            flags: Default::default(),
            rasterization_stream: Default::default(),
        }
    }
}
impl std::fmt::Debug for PipelineRasterizationStateStreamCreateInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PipelineRasterizationStateStreamCreateInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("rasterization_stream", &self.rasterization_stream)
            .finish()
    }
}
impl PipelineRasterizationStateStreamCreateInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PipelineRasterizationStateStreamCreateInfoEXTBuilder<'a> {
        PipelineRasterizationStateStreamCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineRasterizationStateStreamCreateInfoEXT.html) · Builder of [`PipelineRasterizationStateStreamCreateInfoEXT`](struct.PipelineRasterizationStateStreamCreateInfoEXT.html)"]
#[repr(transparent)]
pub struct PipelineRasterizationStateStreamCreateInfoEXTBuilder<'a>(
    PipelineRasterizationStateStreamCreateInfoEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PipelineRasterizationStateStreamCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineRasterizationStateStreamCreateInfoEXTBuilder<'a> {
        PipelineRasterizationStateStreamCreateInfoEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[inline]
    pub fn flags(
        mut self,
        flags : crate :: extensions :: ext_transform_feedback :: PipelineRasterizationStateStreamCreateFlagsEXT,
    ) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn rasterization_stream(mut self, rasterization_stream: u32) -> Self {
        self.0.rasterization_stream = rasterization_stream as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PipelineRasterizationStateStreamCreateInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for PipelineRasterizationStateStreamCreateInfoEXTBuilder<'a> {
    fn default() -> PipelineRasterizationStateStreamCreateInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PipelineRasterizationStateStreamCreateInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PipelineRasterizationStateStreamCreateInfoEXTBuilder<'a> {
    type Target = PipelineRasterizationStateStreamCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineRasterizationStateStreamCreateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`extensions::ext_transform_feedback`](extensions/ext_transform_feedback/index.html)"]
impl crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBindTransformFeedbackBuffersEXT.html) · Function"]
    pub unsafe fn cmd_bind_transform_feedback_buffers_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        first_binding: u32,
        buffers: &[crate::vk1_0::Buffer],
        offsets: &[crate::vk1_0::DeviceSize],
        sizes: &[crate::vk1_0::DeviceSize],
    ) -> () {
        let _function = self
            .cmd_bind_transform_feedback_buffers_ext
            .expect("`cmd_bind_transform_feedback_buffers_ext` is not loaded");
        let binding_count = buffers.len().min(offsets.len()).min(sizes.len());
        let _return = _function(
            command_buffer as _,
            first_binding as _,
            binding_count as _,
            buffers.as_ptr() as _,
            offsets.as_ptr() as _,
            sizes.as_ptr() as _,
        );
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBeginTransformFeedbackEXT.html) · Function"]
    pub unsafe fn cmd_begin_transform_feedback_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        first_counter_buffer: u32,
        counter_buffers: &[crate::vk1_0::Buffer],
        counter_buffer_offsets: &[crate::vk1_0::DeviceSize],
    ) -> () {
        let _function = self
            .cmd_begin_transform_feedback_ext
            .expect("`cmd_begin_transform_feedback_ext` is not loaded");
        let counter_buffer_count = counter_buffers.len().min(counter_buffer_offsets.len());
        let _return = _function(
            command_buffer as _,
            first_counter_buffer as _,
            counter_buffer_count as _,
            counter_buffers.as_ptr() as _,
            counter_buffer_offsets.as_ptr() as _,
        );
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdEndTransformFeedbackEXT.html) · Function"]
    pub unsafe fn cmd_end_transform_feedback_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        first_counter_buffer: u32,
        counter_buffers: &[crate::vk1_0::Buffer],
        counter_buffer_offsets: &[crate::vk1_0::DeviceSize],
    ) -> () {
        let _function = self
            .cmd_end_transform_feedback_ext
            .expect("`cmd_end_transform_feedback_ext` is not loaded");
        let counter_buffer_count = counter_buffers.len().min(counter_buffer_offsets.len());
        let _return = _function(
            command_buffer as _,
            first_counter_buffer as _,
            counter_buffer_count as _,
            counter_buffers.as_ptr() as _,
            counter_buffer_offsets.as_ptr() as _,
        );
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBeginQueryIndexedEXT.html) · Function"]
    pub unsafe fn cmd_begin_query_indexed_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        query_pool: crate::vk1_0::QueryPool,
        query: u32,
        flags: Option<crate::vk1_0::QueryControlFlags>,
        index: u32,
    ) -> () {
        let _function = self
            .cmd_begin_query_indexed_ext
            .expect("`cmd_begin_query_indexed_ext` is not loaded");
        let _return = _function(
            command_buffer as _,
            query_pool as _,
            query as _,
            match flags {
                Some(v) => v,
                None => Default::default(),
            },
            index as _,
        );
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdEndQueryIndexedEXT.html) · Function"]
    pub unsafe fn cmd_end_query_indexed_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        query_pool: crate::vk1_0::QueryPool,
        query: u32,
        index: u32,
    ) -> () {
        let _function = self
            .cmd_end_query_indexed_ext
            .expect("`cmd_end_query_indexed_ext` is not loaded");
        let _return = _function(command_buffer as _, query_pool as _, query as _, index as _);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawIndirectByteCountEXT.html) · Function"]
    pub unsafe fn cmd_draw_indirect_byte_count_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        instance_count: u32,
        first_instance: u32,
        counter_buffer: crate::vk1_0::Buffer,
        counter_buffer_offset: crate::vk1_0::DeviceSize,
        counter_offset: u32,
        vertex_stride: u32,
    ) -> () {
        let _function = self
            .cmd_draw_indirect_byte_count_ext
            .expect("`cmd_draw_indirect_byte_count_ext` is not loaded");
        let _return = _function(
            command_buffer as _,
            instance_count as _,
            first_instance as _,
            counter_buffer as _,
            counter_buffer_offset as _,
            counter_offset as _,
            vertex_stride as _,
        );
        ()
    }
}
