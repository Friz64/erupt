#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_SYNCHRONIZATION_2_SPEC_VERSION")]
pub const KHR_SYNCHRONIZATION_2_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_SYNCHRONIZATION_2_EXTENSION_NAME")]
pub const KHR_SYNCHRONIZATION_2_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_synchronization2");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_SET_EVENT2_KHR: *const std::os::raw::c_char = crate::cstr!("vkCmdSetEvent2KHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_RESET_EVENT2_KHR: *const std::os::raw::c_char = crate::cstr!("vkCmdResetEvent2KHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_WAIT_EVENTS2_KHR: *const std::os::raw::c_char = crate::cstr!("vkCmdWaitEvents2KHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_PIPELINE_BARRIER2_KHR: *const std::os::raw::c_char = crate::cstr!("vkCmdPipelineBarrier2KHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_QUEUE_SUBMIT2_KHR: *const std::os::raw::c_char = crate::cstr!("vkQueueSubmit2KHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_WRITE_TIMESTAMP2_KHR: *const std::os::raw::c_char = crate::cstr!("vkCmdWriteTimestamp2KHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_WRITE_BUFFER_MARKER2_AMD: *const std::os::raw::c_char = crate::cstr!("vkCmdWriteBufferMarker2AMD");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_QUEUE_CHECKPOINT_DATA2_NV: *const std::os::raw::c_char = crate::cstr!("vkGetQueueCheckpointData2NV");
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccessFlags2KHR.html) · Bitmask of [`AccessFlagBits2KHR`]"] # [doc (alias = "VkAccessFlags2KHR")] # [derive (Default)] # [repr (transparent)] pub struct AccessFlags2KHR : u64 { const NONE_KHR = AccessFlagBits2KHR :: NONE_KHR . 0 ; const INDIRECT_COMMAND_READ_KHR = AccessFlagBits2KHR :: INDIRECT_COMMAND_READ_KHR . 0 ; const INDEX_READ_KHR = AccessFlagBits2KHR :: INDEX_READ_KHR . 0 ; const VERTEX_ATTRIBUTE_READ_KHR = AccessFlagBits2KHR :: VERTEX_ATTRIBUTE_READ_KHR . 0 ; const UNIFORM_READ_KHR = AccessFlagBits2KHR :: UNIFORM_READ_KHR . 0 ; const INPUT_ATTACHMENT_READ_KHR = AccessFlagBits2KHR :: INPUT_ATTACHMENT_READ_KHR . 0 ; const SHADER_READ_KHR = AccessFlagBits2KHR :: SHADER_READ_KHR . 0 ; const SHADER_WRITE_KHR = AccessFlagBits2KHR :: SHADER_WRITE_KHR . 0 ; const COLOR_ATTACHMENT_READ_KHR = AccessFlagBits2KHR :: COLOR_ATTACHMENT_READ_KHR . 0 ; const COLOR_ATTACHMENT_WRITE_KHR = AccessFlagBits2KHR :: COLOR_ATTACHMENT_WRITE_KHR . 0 ; const DEPTH_STENCIL_ATTACHMENT_READ_KHR = AccessFlagBits2KHR :: DEPTH_STENCIL_ATTACHMENT_READ_KHR . 0 ; const DEPTH_STENCIL_ATTACHMENT_WRITE_KHR = AccessFlagBits2KHR :: DEPTH_STENCIL_ATTACHMENT_WRITE_KHR . 0 ; const TRANSFER_READ_KHR = AccessFlagBits2KHR :: TRANSFER_READ_KHR . 0 ; const TRANSFER_WRITE_KHR = AccessFlagBits2KHR :: TRANSFER_WRITE_KHR . 0 ; const HOST_READ_KHR = AccessFlagBits2KHR :: HOST_READ_KHR . 0 ; const HOST_WRITE_KHR = AccessFlagBits2KHR :: HOST_WRITE_KHR . 0 ; const MEMORY_READ_KHR = AccessFlagBits2KHR :: MEMORY_READ_KHR . 0 ; const MEMORY_WRITE_KHR = AccessFlagBits2KHR :: MEMORY_WRITE_KHR . 0 ; const SHADER_SAMPLED_READ_KHR = AccessFlagBits2KHR :: SHADER_SAMPLED_READ_KHR . 0 ; const SHADER_STORAGE_READ_KHR = AccessFlagBits2KHR :: SHADER_STORAGE_READ_KHR . 0 ; const SHADER_STORAGE_WRITE_KHR = AccessFlagBits2KHR :: SHADER_STORAGE_WRITE_KHR . 0 ; const RESERVED_READ_35_KHR = AccessFlagBits2KHR :: RESERVED_READ_35_KHR . 0 ; const RESERVED_WRITE_36_KHR = AccessFlagBits2KHR :: RESERVED_WRITE_36_KHR . 0 ; const RESERVED_READ_37_KHR = AccessFlagBits2KHR :: RESERVED_READ_37_KHR . 0 ; const RESERVED_WRITE_38_KHR = AccessFlagBits2KHR :: RESERVED_WRITE_38_KHR . 0 ; const TRANSFORM_FEEDBACK_WRITE_EXT = AccessFlagBits2KHR :: TRANSFORM_FEEDBACK_WRITE_EXT . 0 ; const TRANSFORM_FEEDBACK_COUNTER_READ_EXT = AccessFlagBits2KHR :: TRANSFORM_FEEDBACK_COUNTER_READ_EXT . 0 ; const TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT = AccessFlagBits2KHR :: TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT . 0 ; const CONDITIONAL_RENDERING_READ_EXT = AccessFlagBits2KHR :: CONDITIONAL_RENDERING_READ_EXT . 0 ; const COMMAND_PREPROCESS_READ_NV = AccessFlagBits2KHR :: COMMAND_PREPROCESS_READ_NV . 0 ; const COMMAND_PREPROCESS_WRITE_NV = AccessFlagBits2KHR :: COMMAND_PREPROCESS_WRITE_NV . 0 ; const FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR = AccessFlagBits2KHR :: FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR . 0 ; const SHADING_RATE_IMAGE_READ_NV = AccessFlagBits2KHR :: SHADING_RATE_IMAGE_READ_NV . 0 ; const ACCELERATION_STRUCTURE_READ_KHR = AccessFlagBits2KHR :: ACCELERATION_STRUCTURE_READ_KHR . 0 ; const ACCELERATION_STRUCTURE_WRITE_KHR = AccessFlagBits2KHR :: ACCELERATION_STRUCTURE_WRITE_KHR . 0 ; const ACCELERATION_STRUCTURE_READ_NV = AccessFlagBits2KHR :: ACCELERATION_STRUCTURE_READ_NV . 0 ; const ACCELERATION_STRUCTURE_WRITE_NV = AccessFlagBits2KHR :: ACCELERATION_STRUCTURE_WRITE_NV . 0 ; const FRAGMENT_DENSITY_MAP_READ_EXT = AccessFlagBits2KHR :: FRAGMENT_DENSITY_MAP_READ_EXT . 0 ; const COLOR_ATTACHMENT_READ_NONCOHERENT_EXT = AccessFlagBits2KHR :: COLOR_ATTACHMENT_READ_NONCOHERENT_EXT . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccessFlagBits2KHR.html) · Bits enum of [`AccessFlags2KHR`]"]
#[doc(alias = "VkAccessFlagBits2KHR")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct AccessFlagBits2KHR(pub u64);
impl AccessFlagBits2KHR {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> AccessFlags2KHR {
        AccessFlags2KHR::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for AccessFlagBits2KHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::NONE_KHR => "NONE_KHR",
            &Self::INDIRECT_COMMAND_READ_KHR => "INDIRECT_COMMAND_READ_KHR",
            &Self::INDEX_READ_KHR => "INDEX_READ_KHR",
            &Self::VERTEX_ATTRIBUTE_READ_KHR => "VERTEX_ATTRIBUTE_READ_KHR",
            &Self::UNIFORM_READ_KHR => "UNIFORM_READ_KHR",
            &Self::INPUT_ATTACHMENT_READ_KHR => "INPUT_ATTACHMENT_READ_KHR",
            &Self::SHADER_READ_KHR => "SHADER_READ_KHR",
            &Self::SHADER_WRITE_KHR => "SHADER_WRITE_KHR",
            &Self::COLOR_ATTACHMENT_READ_KHR => "COLOR_ATTACHMENT_READ_KHR",
            &Self::COLOR_ATTACHMENT_WRITE_KHR => "COLOR_ATTACHMENT_WRITE_KHR",
            &Self::DEPTH_STENCIL_ATTACHMENT_READ_KHR => "DEPTH_STENCIL_ATTACHMENT_READ_KHR",
            &Self::DEPTH_STENCIL_ATTACHMENT_WRITE_KHR => "DEPTH_STENCIL_ATTACHMENT_WRITE_KHR",
            &Self::TRANSFER_READ_KHR => "TRANSFER_READ_KHR",
            &Self::TRANSFER_WRITE_KHR => "TRANSFER_WRITE_KHR",
            &Self::HOST_READ_KHR => "HOST_READ_KHR",
            &Self::HOST_WRITE_KHR => "HOST_WRITE_KHR",
            &Self::MEMORY_READ_KHR => "MEMORY_READ_KHR",
            &Self::MEMORY_WRITE_KHR => "MEMORY_WRITE_KHR",
            &Self::SHADER_SAMPLED_READ_KHR => "SHADER_SAMPLED_READ_KHR",
            &Self::SHADER_STORAGE_READ_KHR => "SHADER_STORAGE_READ_KHR",
            &Self::SHADER_STORAGE_WRITE_KHR => "SHADER_STORAGE_WRITE_KHR",
            &Self::RESERVED_READ_35_KHR => "RESERVED_READ_35_KHR",
            &Self::RESERVED_WRITE_36_KHR => "RESERVED_WRITE_36_KHR",
            &Self::RESERVED_READ_37_KHR => "RESERVED_READ_37_KHR",
            &Self::RESERVED_WRITE_38_KHR => "RESERVED_WRITE_38_KHR",
            &Self::TRANSFORM_FEEDBACK_WRITE_EXT => "TRANSFORM_FEEDBACK_WRITE_EXT",
            &Self::TRANSFORM_FEEDBACK_COUNTER_READ_EXT => "TRANSFORM_FEEDBACK_COUNTER_READ_EXT",
            &Self::TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT => "TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT",
            &Self::CONDITIONAL_RENDERING_READ_EXT => "CONDITIONAL_RENDERING_READ_EXT",
            &Self::COMMAND_PREPROCESS_READ_NV => "COMMAND_PREPROCESS_READ_NV",
            &Self::COMMAND_PREPROCESS_WRITE_NV => "COMMAND_PREPROCESS_WRITE_NV",
            &Self::FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR => "FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR",
            &Self::SHADING_RATE_IMAGE_READ_NV => "SHADING_RATE_IMAGE_READ_NV",
            &Self::ACCELERATION_STRUCTURE_READ_KHR => "ACCELERATION_STRUCTURE_READ_KHR",
            &Self::ACCELERATION_STRUCTURE_WRITE_KHR => "ACCELERATION_STRUCTURE_WRITE_KHR",
            &Self::ACCELERATION_STRUCTURE_READ_NV => "ACCELERATION_STRUCTURE_READ_NV",
            &Self::ACCELERATION_STRUCTURE_WRITE_NV => "ACCELERATION_STRUCTURE_WRITE_NV",
            &Self::FRAGMENT_DENSITY_MAP_READ_EXT => "FRAGMENT_DENSITY_MAP_READ_EXT",
            &Self::COLOR_ATTACHMENT_READ_NONCOHERENT_EXT => "COLOR_ATTACHMENT_READ_NONCOHERENT_EXT",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::khr_synchronization2`]"]
impl AccessFlagBits2KHR {
    pub const NONE_KHR: Self = Self(0);
    pub const INDIRECT_COMMAND_READ_KHR: Self = Self(1);
    pub const INDEX_READ_KHR: Self = Self(2);
    pub const VERTEX_ATTRIBUTE_READ_KHR: Self = Self(4);
    pub const UNIFORM_READ_KHR: Self = Self(8);
    pub const INPUT_ATTACHMENT_READ_KHR: Self = Self(16);
    pub const SHADER_READ_KHR: Self = Self(32);
    pub const SHADER_WRITE_KHR: Self = Self(64);
    pub const COLOR_ATTACHMENT_READ_KHR: Self = Self(128);
    pub const COLOR_ATTACHMENT_WRITE_KHR: Self = Self(256);
    pub const DEPTH_STENCIL_ATTACHMENT_READ_KHR: Self = Self(512);
    pub const DEPTH_STENCIL_ATTACHMENT_WRITE_KHR: Self = Self(1024);
    pub const TRANSFER_READ_KHR: Self = Self(2048);
    pub const TRANSFER_WRITE_KHR: Self = Self(4096);
    pub const HOST_READ_KHR: Self = Self(8192);
    pub const HOST_WRITE_KHR: Self = Self(16384);
    pub const MEMORY_READ_KHR: Self = Self(32768);
    pub const MEMORY_WRITE_KHR: Self = Self(65536);
    pub const SHADER_SAMPLED_READ_KHR: Self = Self(4294967296);
    pub const SHADER_STORAGE_READ_KHR: Self = Self(8589934592);
    pub const SHADER_STORAGE_WRITE_KHR: Self = Self(17179869184);
    pub const RESERVED_READ_35_KHR: Self = Self(34359738368);
    pub const RESERVED_WRITE_36_KHR: Self = Self(68719476736);
    pub const RESERVED_READ_37_KHR: Self = Self(137438953472);
    pub const RESERVED_WRITE_38_KHR: Self = Self(274877906944);
    pub const TRANSFORM_FEEDBACK_WRITE_EXT: Self = Self(33554432);
    pub const TRANSFORM_FEEDBACK_COUNTER_READ_EXT: Self = Self(67108864);
    pub const TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT: Self = Self(134217728);
    pub const CONDITIONAL_RENDERING_READ_EXT: Self = Self(1048576);
    pub const COMMAND_PREPROCESS_READ_NV: Self = Self(131072);
    pub const COMMAND_PREPROCESS_WRITE_NV: Self = Self(262144);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR: Self = Self(8388608);
    pub const SHADING_RATE_IMAGE_READ_NV: Self = Self(8388608);
    pub const ACCELERATION_STRUCTURE_READ_KHR: Self = Self(2097152);
    pub const ACCELERATION_STRUCTURE_WRITE_KHR: Self = Self(4194304);
    pub const ACCELERATION_STRUCTURE_READ_NV: Self = Self(2097152);
    pub const ACCELERATION_STRUCTURE_WRITE_NV: Self = Self(4194304);
    pub const FRAGMENT_DENSITY_MAP_READ_EXT: Self = Self(16777216);
    pub const COLOR_ATTACHMENT_READ_NONCOHERENT_EXT: Self = Self(524288);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineStageFlags2KHR.html) · Bitmask of [`PipelineStageFlagBits2KHR`]"] # [doc (alias = "VkPipelineStageFlags2KHR")] # [derive (Default)] # [repr (transparent)] pub struct PipelineStageFlags2KHR : u64 { const NONE_KHR = PipelineStageFlagBits2KHR :: NONE_KHR . 0 ; const TOP_OF_PIPE_KHR = PipelineStageFlagBits2KHR :: TOP_OF_PIPE_KHR . 0 ; const DRAW_INDIRECT_KHR = PipelineStageFlagBits2KHR :: DRAW_INDIRECT_KHR . 0 ; const VERTEX_INPUT_KHR = PipelineStageFlagBits2KHR :: VERTEX_INPUT_KHR . 0 ; const VERTEX_SHADER_KHR = PipelineStageFlagBits2KHR :: VERTEX_SHADER_KHR . 0 ; const TESSELLATION_CONTROL_SHADER_KHR = PipelineStageFlagBits2KHR :: TESSELLATION_CONTROL_SHADER_KHR . 0 ; const TESSELLATION_EVALUATION_SHADER_KHR = PipelineStageFlagBits2KHR :: TESSELLATION_EVALUATION_SHADER_KHR . 0 ; const GEOMETRY_SHADER_KHR = PipelineStageFlagBits2KHR :: GEOMETRY_SHADER_KHR . 0 ; const FRAGMENT_SHADER_KHR = PipelineStageFlagBits2KHR :: FRAGMENT_SHADER_KHR . 0 ; const EARLY_FRAGMENT_TESTS_KHR = PipelineStageFlagBits2KHR :: EARLY_FRAGMENT_TESTS_KHR . 0 ; const LATE_FRAGMENT_TESTS_KHR = PipelineStageFlagBits2KHR :: LATE_FRAGMENT_TESTS_KHR . 0 ; const COLOR_ATTACHMENT_OUTPUT_KHR = PipelineStageFlagBits2KHR :: COLOR_ATTACHMENT_OUTPUT_KHR . 0 ; const COMPUTE_SHADER_KHR = PipelineStageFlagBits2KHR :: COMPUTE_SHADER_KHR . 0 ; const ALL_TRANSFER_KHR = PipelineStageFlagBits2KHR :: ALL_TRANSFER_KHR . 0 ; const TRANSFER_KHR = PipelineStageFlagBits2KHR :: TRANSFER_KHR . 0 ; const BOTTOM_OF_PIPE_KHR = PipelineStageFlagBits2KHR :: BOTTOM_OF_PIPE_KHR . 0 ; const HOST_KHR = PipelineStageFlagBits2KHR :: HOST_KHR . 0 ; const ALL_GRAPHICS_KHR = PipelineStageFlagBits2KHR :: ALL_GRAPHICS_KHR . 0 ; const ALL_COMMANDS_KHR = PipelineStageFlagBits2KHR :: ALL_COMMANDS_KHR . 0 ; const COPY_KHR = PipelineStageFlagBits2KHR :: COPY_KHR . 0 ; const RESOLVE_KHR = PipelineStageFlagBits2KHR :: RESOLVE_KHR . 0 ; const BLIT_KHR = PipelineStageFlagBits2KHR :: BLIT_KHR . 0 ; const CLEAR_KHR = PipelineStageFlagBits2KHR :: CLEAR_KHR . 0 ; const INDEX_INPUT_KHR = PipelineStageFlagBits2KHR :: INDEX_INPUT_KHR . 0 ; const VERTEX_ATTRIBUTE_INPUT_KHR = PipelineStageFlagBits2KHR :: VERTEX_ATTRIBUTE_INPUT_KHR . 0 ; const PRE_RASTERIZATION_SHADERS_KHR = PipelineStageFlagBits2KHR :: PRE_RASTERIZATION_SHADERS_KHR . 0 ; const RESERVED_26_KHR = PipelineStageFlagBits2KHR :: RESERVED_26_KHR . 0 ; const RESERVED_27_KHR = PipelineStageFlagBits2KHR :: RESERVED_27_KHR . 0 ; const TRANSFORM_FEEDBACK_EXT = PipelineStageFlagBits2KHR :: TRANSFORM_FEEDBACK_EXT . 0 ; const CONDITIONAL_RENDERING_EXT = PipelineStageFlagBits2KHR :: CONDITIONAL_RENDERING_EXT . 0 ; const COMMAND_PREPROCESS_NV = PipelineStageFlagBits2KHR :: COMMAND_PREPROCESS_NV . 0 ; const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR = PipelineStageFlagBits2KHR :: FRAGMENT_SHADING_RATE_ATTACHMENT_KHR . 0 ; const SHADING_RATE_IMAGE_NV = PipelineStageFlagBits2KHR :: SHADING_RATE_IMAGE_NV . 0 ; const ACCELERATION_STRUCTURE_BUILD_KHR = PipelineStageFlagBits2KHR :: ACCELERATION_STRUCTURE_BUILD_KHR . 0 ; const RAY_TRACING_SHADER_KHR = PipelineStageFlagBits2KHR :: RAY_TRACING_SHADER_KHR . 0 ; const RAY_TRACING_SHADER_NV = PipelineStageFlagBits2KHR :: RAY_TRACING_SHADER_NV . 0 ; const ACCELERATION_STRUCTURE_BUILD_NV = PipelineStageFlagBits2KHR :: ACCELERATION_STRUCTURE_BUILD_NV . 0 ; const FRAGMENT_DENSITY_PROCESS_EXT = PipelineStageFlagBits2KHR :: FRAGMENT_DENSITY_PROCESS_EXT . 0 ; const TASK_SHADER_NV = PipelineStageFlagBits2KHR :: TASK_SHADER_NV . 0 ; const MESH_SHADER_NV = PipelineStageFlagBits2KHR :: MESH_SHADER_NV . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineStageFlagBits2KHR.html) · Bits enum of [`PipelineStageFlags2KHR`]"]
#[doc(alias = "VkPipelineStageFlagBits2KHR")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct PipelineStageFlagBits2KHR(pub u64);
impl PipelineStageFlagBits2KHR {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> PipelineStageFlags2KHR {
        PipelineStageFlags2KHR::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for PipelineStageFlagBits2KHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::NONE_KHR => "NONE_KHR",
            &Self::TOP_OF_PIPE_KHR => "TOP_OF_PIPE_KHR",
            &Self::DRAW_INDIRECT_KHR => "DRAW_INDIRECT_KHR",
            &Self::VERTEX_INPUT_KHR => "VERTEX_INPUT_KHR",
            &Self::VERTEX_SHADER_KHR => "VERTEX_SHADER_KHR",
            &Self::TESSELLATION_CONTROL_SHADER_KHR => "TESSELLATION_CONTROL_SHADER_KHR",
            &Self::TESSELLATION_EVALUATION_SHADER_KHR => "TESSELLATION_EVALUATION_SHADER_KHR",
            &Self::GEOMETRY_SHADER_KHR => "GEOMETRY_SHADER_KHR",
            &Self::FRAGMENT_SHADER_KHR => "FRAGMENT_SHADER_KHR",
            &Self::EARLY_FRAGMENT_TESTS_KHR => "EARLY_FRAGMENT_TESTS_KHR",
            &Self::LATE_FRAGMENT_TESTS_KHR => "LATE_FRAGMENT_TESTS_KHR",
            &Self::COLOR_ATTACHMENT_OUTPUT_KHR => "COLOR_ATTACHMENT_OUTPUT_KHR",
            &Self::COMPUTE_SHADER_KHR => "COMPUTE_SHADER_KHR",
            &Self::ALL_TRANSFER_KHR => "ALL_TRANSFER_KHR",
            &Self::TRANSFER_KHR => "TRANSFER_KHR",
            &Self::BOTTOM_OF_PIPE_KHR => "BOTTOM_OF_PIPE_KHR",
            &Self::HOST_KHR => "HOST_KHR",
            &Self::ALL_GRAPHICS_KHR => "ALL_GRAPHICS_KHR",
            &Self::ALL_COMMANDS_KHR => "ALL_COMMANDS_KHR",
            &Self::COPY_KHR => "COPY_KHR",
            &Self::RESOLVE_KHR => "RESOLVE_KHR",
            &Self::BLIT_KHR => "BLIT_KHR",
            &Self::CLEAR_KHR => "CLEAR_KHR",
            &Self::INDEX_INPUT_KHR => "INDEX_INPUT_KHR",
            &Self::VERTEX_ATTRIBUTE_INPUT_KHR => "VERTEX_ATTRIBUTE_INPUT_KHR",
            &Self::PRE_RASTERIZATION_SHADERS_KHR => "PRE_RASTERIZATION_SHADERS_KHR",
            &Self::RESERVED_26_KHR => "RESERVED_26_KHR",
            &Self::RESERVED_27_KHR => "RESERVED_27_KHR",
            &Self::TRANSFORM_FEEDBACK_EXT => "TRANSFORM_FEEDBACK_EXT",
            &Self::CONDITIONAL_RENDERING_EXT => "CONDITIONAL_RENDERING_EXT",
            &Self::COMMAND_PREPROCESS_NV => "COMMAND_PREPROCESS_NV",
            &Self::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR => "FRAGMENT_SHADING_RATE_ATTACHMENT_KHR",
            &Self::SHADING_RATE_IMAGE_NV => "SHADING_RATE_IMAGE_NV",
            &Self::ACCELERATION_STRUCTURE_BUILD_KHR => "ACCELERATION_STRUCTURE_BUILD_KHR",
            &Self::RAY_TRACING_SHADER_KHR => "RAY_TRACING_SHADER_KHR",
            &Self::RAY_TRACING_SHADER_NV => "RAY_TRACING_SHADER_NV",
            &Self::ACCELERATION_STRUCTURE_BUILD_NV => "ACCELERATION_STRUCTURE_BUILD_NV",
            &Self::FRAGMENT_DENSITY_PROCESS_EXT => "FRAGMENT_DENSITY_PROCESS_EXT",
            &Self::TASK_SHADER_NV => "TASK_SHADER_NV",
            &Self::MESH_SHADER_NV => "MESH_SHADER_NV",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::khr_synchronization2`]"]
impl PipelineStageFlagBits2KHR {
    pub const NONE_KHR: Self = Self(0);
    pub const TOP_OF_PIPE_KHR: Self = Self(1);
    pub const DRAW_INDIRECT_KHR: Self = Self(2);
    pub const VERTEX_INPUT_KHR: Self = Self(4);
    pub const VERTEX_SHADER_KHR: Self = Self(8);
    pub const TESSELLATION_CONTROL_SHADER_KHR: Self = Self(16);
    pub const TESSELLATION_EVALUATION_SHADER_KHR: Self = Self(32);
    pub const GEOMETRY_SHADER_KHR: Self = Self(64);
    pub const FRAGMENT_SHADER_KHR: Self = Self(128);
    pub const EARLY_FRAGMENT_TESTS_KHR: Self = Self(256);
    pub const LATE_FRAGMENT_TESTS_KHR: Self = Self(512);
    pub const COLOR_ATTACHMENT_OUTPUT_KHR: Self = Self(1024);
    pub const COMPUTE_SHADER_KHR: Self = Self(2048);
    pub const ALL_TRANSFER_KHR: Self = Self(4096);
    pub const TRANSFER_KHR: Self = Self(4096);
    pub const BOTTOM_OF_PIPE_KHR: Self = Self(8192);
    pub const HOST_KHR: Self = Self(16384);
    pub const ALL_GRAPHICS_KHR: Self = Self(32768);
    pub const ALL_COMMANDS_KHR: Self = Self(65536);
    pub const COPY_KHR: Self = Self(4294967296);
    pub const RESOLVE_KHR: Self = Self(8589934592);
    pub const BLIT_KHR: Self = Self(17179869184);
    pub const CLEAR_KHR: Self = Self(34359738368);
    pub const INDEX_INPUT_KHR: Self = Self(68719476736);
    pub const VERTEX_ATTRIBUTE_INPUT_KHR: Self = Self(137438953472);
    pub const PRE_RASTERIZATION_SHADERS_KHR: Self = Self(274877906944);
    pub const RESERVED_26_KHR: Self = Self(67108864);
    pub const RESERVED_27_KHR: Self = Self(134217728);
    pub const TRANSFORM_FEEDBACK_EXT: Self = Self(16777216);
    pub const CONDITIONAL_RENDERING_EXT: Self = Self(262144);
    pub const COMMAND_PREPROCESS_NV: Self = Self(131072);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self = Self(4194304);
    pub const SHADING_RATE_IMAGE_NV: Self = Self(4194304);
    pub const ACCELERATION_STRUCTURE_BUILD_KHR: Self = Self(33554432);
    pub const RAY_TRACING_SHADER_KHR: Self = Self(2097152);
    pub const RAY_TRACING_SHADER_NV: Self = Self(2097152);
    pub const ACCELERATION_STRUCTURE_BUILD_NV: Self = Self(33554432);
    pub const FRAGMENT_DENSITY_PROCESS_EXT: Self = Self(8388608);
    pub const TASK_SHADER_NV: Self = Self(524288);
    pub const MESH_SHADER_NV: Self = Self(1048576);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubmitFlagsKHR.html) · Bitmask of [`SubmitFlagBitsKHR`]"] # [doc (alias = "VkSubmitFlagsKHR")] # [derive (Default)] # [repr (transparent)] pub struct SubmitFlagsKHR : u32 { const PROTECTED_KHR = SubmitFlagBitsKHR :: PROTECTED_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubmitFlagBitsKHR.html) · Bits enum of [`SubmitFlagsKHR`]"]
#[doc(alias = "VkSubmitFlagBitsKHR")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct SubmitFlagBitsKHR(pub u32);
impl SubmitFlagBitsKHR {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> SubmitFlagsKHR {
        SubmitFlagsKHR::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for SubmitFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::PROTECTED_KHR => "PROTECTED_KHR",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::khr_synchronization2`]"]
impl SubmitFlagBitsKHR {
    pub const PROTECTED_KHR: Self = Self(1);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetEvent2KHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetEvent2KHR =
    unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, event: crate::vk1_0::Event, p_dependency_info: *const crate::extensions::khr_synchronization2::DependencyInfoKHR) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdResetEvent2KHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdResetEvent2KHR =
    unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, event: crate::vk1_0::Event, stage_mask: crate::extensions::khr_synchronization2::PipelineStageFlags2KHR) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdWaitEvents2KHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdWaitEvents2KHR = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    event_count: u32,
    p_events: *const crate::vk1_0::Event,
    p_dependency_infos: *const crate::extensions::khr_synchronization2::DependencyInfoKHR,
) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdPipelineBarrier2KHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdPipelineBarrier2KHR =
    unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, p_dependency_info: *const crate::extensions::khr_synchronization2::DependencyInfoKHR) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueueSubmit2KHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkQueueSubmit2KHR = unsafe extern "system" fn(
    queue: crate::vk1_0::Queue,
    submit_count: u32,
    p_submits: *const crate::extensions::khr_synchronization2::SubmitInfo2KHR,
    fence: crate::vk1_0::Fence,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdWriteTimestamp2KHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdWriteTimestamp2KHR = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    stage: crate::extensions::khr_synchronization2::PipelineStageFlags2KHR,
    query_pool: crate::vk1_0::QueryPool,
    query: u32,
) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdWriteBufferMarker2AMD.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdWriteBufferMarker2AMD = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    stage: crate::extensions::khr_synchronization2::PipelineStageFlags2KHR,
    dst_buffer: crate::vk1_0::Buffer,
    dst_offset: crate::vk1_0::DeviceSize,
    marker: u32,
) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetQueueCheckpointData2NV.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetQueueCheckpointData2NV =
    unsafe extern "system" fn(queue: crate::vk1_0::Queue, p_checkpoint_data_count: *mut u32, p_checkpoint_data: *mut crate::extensions::khr_synchronization2::CheckpointData2NV) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryBarrier2KHR.html) · Structure"]
#[doc(alias = "VkMemoryBarrier2KHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryBarrier2KHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub src_stage_mask: crate::extensions::khr_synchronization2::PipelineStageFlags2KHR,
    pub src_access_mask: crate::extensions::khr_synchronization2::AccessFlags2KHR,
    pub dst_stage_mask: crate::extensions::khr_synchronization2::PipelineStageFlags2KHR,
    pub dst_access_mask: crate::extensions::khr_synchronization2::AccessFlags2KHR,
}
impl Default for MemoryBarrier2KHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::MEMORY_BARRIER_2_KHR,
            p_next: std::ptr::null(),
            src_stage_mask: Default::default(),
            src_access_mask: Default::default(),
            dst_stage_mask: Default::default(),
            dst_access_mask: Default::default(),
        }
    }
}
impl std::fmt::Debug for MemoryBarrier2KHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MemoryBarrier2KHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("src_stage_mask", &self.src_stage_mask)
            .field("src_access_mask", &self.src_access_mask)
            .field("dst_stage_mask", &self.dst_stage_mask)
            .field("dst_access_mask", &self.dst_access_mask)
            .finish()
    }
}
impl MemoryBarrier2KHR {
    #[inline]
    pub fn into_builder<'a>(self) -> MemoryBarrier2KHRBuilder<'a> {
        MemoryBarrier2KHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryBarrier2KHR.html) · Builder of [`MemoryBarrier2KHR`]"]
#[repr(transparent)]
pub struct MemoryBarrier2KHRBuilder<'a>(MemoryBarrier2KHR, std::marker::PhantomData<&'a ()>);
impl<'a> MemoryBarrier2KHRBuilder<'a> {
    #[inline]
    pub fn new() -> MemoryBarrier2KHRBuilder<'a> {
        MemoryBarrier2KHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn src_stage_mask(mut self, src_stage_mask: crate::extensions::khr_synchronization2::PipelineStageFlags2KHR) -> Self {
        self.0.src_stage_mask = src_stage_mask as _;
        self
    }
    #[inline]
    pub fn src_access_mask(mut self, src_access_mask: crate::extensions::khr_synchronization2::AccessFlags2KHR) -> Self {
        self.0.src_access_mask = src_access_mask as _;
        self
    }
    #[inline]
    pub fn dst_stage_mask(mut self, dst_stage_mask: crate::extensions::khr_synchronization2::PipelineStageFlags2KHR) -> Self {
        self.0.dst_stage_mask = dst_stage_mask as _;
        self
    }
    #[inline]
    pub fn dst_access_mask(mut self, dst_access_mask: crate::extensions::khr_synchronization2::AccessFlags2KHR) -> Self {
        self.0.dst_access_mask = dst_access_mask as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> MemoryBarrier2KHR {
        self.0
    }
}
impl<'a> std::default::Default for MemoryBarrier2KHRBuilder<'a> {
    fn default() -> MemoryBarrier2KHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for MemoryBarrier2KHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for MemoryBarrier2KHRBuilder<'a> {
    type Target = MemoryBarrier2KHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for MemoryBarrier2KHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageMemoryBarrier2KHR.html) · Structure"]
#[doc(alias = "VkImageMemoryBarrier2KHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImageMemoryBarrier2KHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub src_stage_mask: crate::extensions::khr_synchronization2::PipelineStageFlags2KHR,
    pub src_access_mask: crate::extensions::khr_synchronization2::AccessFlags2KHR,
    pub dst_stage_mask: crate::extensions::khr_synchronization2::PipelineStageFlags2KHR,
    pub dst_access_mask: crate::extensions::khr_synchronization2::AccessFlags2KHR,
    pub old_layout: crate::vk1_0::ImageLayout,
    pub new_layout: crate::vk1_0::ImageLayout,
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub image: crate::vk1_0::Image,
    pub subresource_range: crate::vk1_0::ImageSubresourceRange,
}
impl Default for ImageMemoryBarrier2KHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::IMAGE_MEMORY_BARRIER_2_KHR,
            p_next: std::ptr::null(),
            src_stage_mask: Default::default(),
            src_access_mask: Default::default(),
            dst_stage_mask: Default::default(),
            dst_access_mask: Default::default(),
            old_layout: Default::default(),
            new_layout: Default::default(),
            src_queue_family_index: Default::default(),
            dst_queue_family_index: Default::default(),
            image: Default::default(),
            subresource_range: Default::default(),
        }
    }
}
impl std::fmt::Debug for ImageMemoryBarrier2KHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ImageMemoryBarrier2KHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("src_stage_mask", &self.src_stage_mask)
            .field("src_access_mask", &self.src_access_mask)
            .field("dst_stage_mask", &self.dst_stage_mask)
            .field("dst_access_mask", &self.dst_access_mask)
            .field("old_layout", &self.old_layout)
            .field("new_layout", &self.new_layout)
            .field("src_queue_family_index", &self.src_queue_family_index)
            .field("dst_queue_family_index", &self.dst_queue_family_index)
            .field("image", &self.image)
            .field("subresource_range", &self.subresource_range)
            .finish()
    }
}
impl ImageMemoryBarrier2KHR {
    #[inline]
    pub fn into_builder<'a>(self) -> ImageMemoryBarrier2KHRBuilder<'a> {
        ImageMemoryBarrier2KHRBuilder(self, std::marker::PhantomData)
    }
}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_sample_locations::SampleLocationsInfoEXT> for ImageMemoryBarrier2KHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_sample_locations::SampleLocationsInfoEXTBuilder<'_>> for ImageMemoryBarrier2KHRBuilder<'a> {}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageMemoryBarrier2KHR.html) · Builder of [`ImageMemoryBarrier2KHR`]"]
#[repr(transparent)]
pub struct ImageMemoryBarrier2KHRBuilder<'a>(ImageMemoryBarrier2KHR, std::marker::PhantomData<&'a ()>);
impl<'a> ImageMemoryBarrier2KHRBuilder<'a> {
    #[inline]
    pub fn new() -> ImageMemoryBarrier2KHRBuilder<'a> {
        ImageMemoryBarrier2KHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn src_stage_mask(mut self, src_stage_mask: crate::extensions::khr_synchronization2::PipelineStageFlags2KHR) -> Self {
        self.0.src_stage_mask = src_stage_mask as _;
        self
    }
    #[inline]
    pub fn src_access_mask(mut self, src_access_mask: crate::extensions::khr_synchronization2::AccessFlags2KHR) -> Self {
        self.0.src_access_mask = src_access_mask as _;
        self
    }
    #[inline]
    pub fn dst_stage_mask(mut self, dst_stage_mask: crate::extensions::khr_synchronization2::PipelineStageFlags2KHR) -> Self {
        self.0.dst_stage_mask = dst_stage_mask as _;
        self
    }
    #[inline]
    pub fn dst_access_mask(mut self, dst_access_mask: crate::extensions::khr_synchronization2::AccessFlags2KHR) -> Self {
        self.0.dst_access_mask = dst_access_mask as _;
        self
    }
    #[inline]
    pub fn old_layout(mut self, old_layout: crate::vk1_0::ImageLayout) -> Self {
        self.0.old_layout = old_layout as _;
        self
    }
    #[inline]
    pub fn new_layout(mut self, new_layout: crate::vk1_0::ImageLayout) -> Self {
        self.0.new_layout = new_layout as _;
        self
    }
    #[inline]
    pub fn src_queue_family_index(mut self, src_queue_family_index: u32) -> Self {
        self.0.src_queue_family_index = src_queue_family_index as _;
        self
    }
    #[inline]
    pub fn dst_queue_family_index(mut self, dst_queue_family_index: u32) -> Self {
        self.0.dst_queue_family_index = dst_queue_family_index as _;
        self
    }
    #[inline]
    pub fn image(mut self, image: crate::vk1_0::Image) -> Self {
        self.0.image = image as _;
        self
    }
    #[inline]
    pub fn subresource_range(mut self, subresource_range: crate::vk1_0::ImageSubresourceRange) -> Self {
        self.0.subresource_range = subresource_range as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ImageMemoryBarrier2KHR {
        self.0
    }
}
impl<'a> std::default::Default for ImageMemoryBarrier2KHRBuilder<'a> {
    fn default() -> ImageMemoryBarrier2KHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ImageMemoryBarrier2KHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ImageMemoryBarrier2KHRBuilder<'a> {
    type Target = ImageMemoryBarrier2KHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ImageMemoryBarrier2KHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferMemoryBarrier2KHR.html) · Structure"]
#[doc(alias = "VkBufferMemoryBarrier2KHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BufferMemoryBarrier2KHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub src_stage_mask: crate::extensions::khr_synchronization2::PipelineStageFlags2KHR,
    pub src_access_mask: crate::extensions::khr_synchronization2::AccessFlags2KHR,
    pub dst_stage_mask: crate::extensions::khr_synchronization2::PipelineStageFlags2KHR,
    pub dst_access_mask: crate::extensions::khr_synchronization2::AccessFlags2KHR,
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub buffer: crate::vk1_0::Buffer,
    pub offset: crate::vk1_0::DeviceSize,
    pub size: crate::vk1_0::DeviceSize,
}
impl Default for BufferMemoryBarrier2KHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::BUFFER_MEMORY_BARRIER_2_KHR,
            p_next: std::ptr::null(),
            src_stage_mask: Default::default(),
            src_access_mask: Default::default(),
            dst_stage_mask: Default::default(),
            dst_access_mask: Default::default(),
            src_queue_family_index: Default::default(),
            dst_queue_family_index: Default::default(),
            buffer: Default::default(),
            offset: Default::default(),
            size: Default::default(),
        }
    }
}
impl std::fmt::Debug for BufferMemoryBarrier2KHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BufferMemoryBarrier2KHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("src_stage_mask", &self.src_stage_mask)
            .field("src_access_mask", &self.src_access_mask)
            .field("dst_stage_mask", &self.dst_stage_mask)
            .field("dst_access_mask", &self.dst_access_mask)
            .field("src_queue_family_index", &self.src_queue_family_index)
            .field("dst_queue_family_index", &self.dst_queue_family_index)
            .field("buffer", &self.buffer)
            .field("offset", &self.offset)
            .field("size", &self.size)
            .finish()
    }
}
impl BufferMemoryBarrier2KHR {
    #[inline]
    pub fn into_builder<'a>(self) -> BufferMemoryBarrier2KHRBuilder<'a> {
        BufferMemoryBarrier2KHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferMemoryBarrier2KHR.html) · Builder of [`BufferMemoryBarrier2KHR`]"]
#[repr(transparent)]
pub struct BufferMemoryBarrier2KHRBuilder<'a>(BufferMemoryBarrier2KHR, std::marker::PhantomData<&'a ()>);
impl<'a> BufferMemoryBarrier2KHRBuilder<'a> {
    #[inline]
    pub fn new() -> BufferMemoryBarrier2KHRBuilder<'a> {
        BufferMemoryBarrier2KHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn src_stage_mask(mut self, src_stage_mask: crate::extensions::khr_synchronization2::PipelineStageFlags2KHR) -> Self {
        self.0.src_stage_mask = src_stage_mask as _;
        self
    }
    #[inline]
    pub fn src_access_mask(mut self, src_access_mask: crate::extensions::khr_synchronization2::AccessFlags2KHR) -> Self {
        self.0.src_access_mask = src_access_mask as _;
        self
    }
    #[inline]
    pub fn dst_stage_mask(mut self, dst_stage_mask: crate::extensions::khr_synchronization2::PipelineStageFlags2KHR) -> Self {
        self.0.dst_stage_mask = dst_stage_mask as _;
        self
    }
    #[inline]
    pub fn dst_access_mask(mut self, dst_access_mask: crate::extensions::khr_synchronization2::AccessFlags2KHR) -> Self {
        self.0.dst_access_mask = dst_access_mask as _;
        self
    }
    #[inline]
    pub fn src_queue_family_index(mut self, src_queue_family_index: u32) -> Self {
        self.0.src_queue_family_index = src_queue_family_index as _;
        self
    }
    #[inline]
    pub fn dst_queue_family_index(mut self, dst_queue_family_index: u32) -> Self {
        self.0.dst_queue_family_index = dst_queue_family_index as _;
        self
    }
    #[inline]
    pub fn buffer(mut self, buffer: crate::vk1_0::Buffer) -> Self {
        self.0.buffer = buffer as _;
        self
    }
    #[inline]
    pub fn offset(mut self, offset: crate::vk1_0::DeviceSize) -> Self {
        self.0.offset = offset as _;
        self
    }
    #[inline]
    pub fn size(mut self, size: crate::vk1_0::DeviceSize) -> Self {
        self.0.size = size as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> BufferMemoryBarrier2KHR {
        self.0
    }
}
impl<'a> std::default::Default for BufferMemoryBarrier2KHRBuilder<'a> {
    fn default() -> BufferMemoryBarrier2KHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for BufferMemoryBarrier2KHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for BufferMemoryBarrier2KHRBuilder<'a> {
    type Target = BufferMemoryBarrier2KHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for BufferMemoryBarrier2KHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDependencyInfoKHR.html) · Structure"]
#[doc(alias = "VkDependencyInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DependencyInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub dependency_flags: crate::vk1_0::DependencyFlags,
    pub memory_barrier_count: u32,
    pub p_memory_barriers: *const crate::extensions::khr_synchronization2::MemoryBarrier2KHR,
    pub buffer_memory_barrier_count: u32,
    pub p_buffer_memory_barriers: *const crate::extensions::khr_synchronization2::BufferMemoryBarrier2KHR,
    pub image_memory_barrier_count: u32,
    pub p_image_memory_barriers: *const crate::extensions::khr_synchronization2::ImageMemoryBarrier2KHR,
}
impl Default for DependencyInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::DEPENDENCY_INFO_KHR,
            p_next: std::ptr::null(),
            dependency_flags: Default::default(),
            memory_barrier_count: Default::default(),
            p_memory_barriers: std::ptr::null(),
            buffer_memory_barrier_count: Default::default(),
            p_buffer_memory_barriers: std::ptr::null(),
            image_memory_barrier_count: Default::default(),
            p_image_memory_barriers: std::ptr::null(),
        }
    }
}
impl std::fmt::Debug for DependencyInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DependencyInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("dependency_flags", &self.dependency_flags)
            .field("memory_barrier_count", &self.memory_barrier_count)
            .field("p_memory_barriers", &self.p_memory_barriers)
            .field("buffer_memory_barrier_count", &self.buffer_memory_barrier_count)
            .field("p_buffer_memory_barriers", &self.p_buffer_memory_barriers)
            .field("image_memory_barrier_count", &self.image_memory_barrier_count)
            .field("p_image_memory_barriers", &self.p_image_memory_barriers)
            .finish()
    }
}
impl DependencyInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> DependencyInfoKHRBuilder<'a> {
        DependencyInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDependencyInfoKHR.html) · Builder of [`DependencyInfoKHR`]"]
#[repr(transparent)]
pub struct DependencyInfoKHRBuilder<'a>(DependencyInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> DependencyInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> DependencyInfoKHRBuilder<'a> {
        DependencyInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn dependency_flags(mut self, dependency_flags: crate::vk1_0::DependencyFlags) -> Self {
        self.0.dependency_flags = dependency_flags as _;
        self
    }
    #[inline]
    pub fn memory_barriers(mut self, memory_barriers: &'a [crate::extensions::khr_synchronization2::MemoryBarrier2KHRBuilder]) -> Self {
        self.0.p_memory_barriers = memory_barriers.as_ptr() as _;
        self.0.memory_barrier_count = memory_barriers.len() as _;
        self
    }
    #[inline]
    pub fn buffer_memory_barriers(mut self, buffer_memory_barriers: &'a [crate::extensions::khr_synchronization2::BufferMemoryBarrier2KHRBuilder]) -> Self {
        self.0.p_buffer_memory_barriers = buffer_memory_barriers.as_ptr() as _;
        self.0.buffer_memory_barrier_count = buffer_memory_barriers.len() as _;
        self
    }
    #[inline]
    pub fn image_memory_barriers(mut self, image_memory_barriers: &'a [crate::extensions::khr_synchronization2::ImageMemoryBarrier2KHRBuilder]) -> Self {
        self.0.p_image_memory_barriers = image_memory_barriers.as_ptr() as _;
        self.0.image_memory_barrier_count = image_memory_barriers.len() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DependencyInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for DependencyInfoKHRBuilder<'a> {
    fn default() -> DependencyInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DependencyInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DependencyInfoKHRBuilder<'a> {
    type Target = DependencyInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DependencyInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreSubmitInfoKHR.html) · Structure"]
#[doc(alias = "VkSemaphoreSubmitInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SemaphoreSubmitInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub semaphore: crate::vk1_0::Semaphore,
    pub value: u64,
    pub stage_mask: crate::extensions::khr_synchronization2::PipelineStageFlags2KHR,
    pub device_index: u32,
}
impl Default for SemaphoreSubmitInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::SEMAPHORE_SUBMIT_INFO_KHR,
            p_next: std::ptr::null(),
            semaphore: Default::default(),
            value: Default::default(),
            stage_mask: Default::default(),
            device_index: Default::default(),
        }
    }
}
impl std::fmt::Debug for SemaphoreSubmitInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SemaphoreSubmitInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("semaphore", &self.semaphore)
            .field("value", &self.value)
            .field("stage_mask", &self.stage_mask)
            .field("device_index", &self.device_index)
            .finish()
    }
}
impl SemaphoreSubmitInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> SemaphoreSubmitInfoKHRBuilder<'a> {
        SemaphoreSubmitInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreSubmitInfoKHR.html) · Builder of [`SemaphoreSubmitInfoKHR`]"]
#[repr(transparent)]
pub struct SemaphoreSubmitInfoKHRBuilder<'a>(SemaphoreSubmitInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> SemaphoreSubmitInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> SemaphoreSubmitInfoKHRBuilder<'a> {
        SemaphoreSubmitInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn semaphore(mut self, semaphore: crate::vk1_0::Semaphore) -> Self {
        self.0.semaphore = semaphore as _;
        self
    }
    #[inline]
    pub fn value(mut self, value: u64) -> Self {
        self.0.value = value as _;
        self
    }
    #[inline]
    pub fn stage_mask(mut self, stage_mask: crate::extensions::khr_synchronization2::PipelineStageFlags2KHR) -> Self {
        self.0.stage_mask = stage_mask as _;
        self
    }
    #[inline]
    pub fn device_index(mut self, device_index: u32) -> Self {
        self.0.device_index = device_index as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> SemaphoreSubmitInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for SemaphoreSubmitInfoKHRBuilder<'a> {
    fn default() -> SemaphoreSubmitInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for SemaphoreSubmitInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for SemaphoreSubmitInfoKHRBuilder<'a> {
    type Target = SemaphoreSubmitInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SemaphoreSubmitInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandBufferSubmitInfoKHR.html) · Structure"]
#[doc(alias = "VkCommandBufferSubmitInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CommandBufferSubmitInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub command_buffer: crate::vk1_0::CommandBuffer,
    pub device_mask: u32,
}
impl Default for CommandBufferSubmitInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::COMMAND_BUFFER_SUBMIT_INFO_KHR,
            p_next: std::ptr::null(),
            command_buffer: Default::default(),
            device_mask: Default::default(),
        }
    }
}
impl std::fmt::Debug for CommandBufferSubmitInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CommandBufferSubmitInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("command_buffer", &self.command_buffer)
            .field("device_mask", &self.device_mask)
            .finish()
    }
}
impl CommandBufferSubmitInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> CommandBufferSubmitInfoKHRBuilder<'a> {
        CommandBufferSubmitInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandBufferSubmitInfoKHR.html) · Builder of [`CommandBufferSubmitInfoKHR`]"]
#[repr(transparent)]
pub struct CommandBufferSubmitInfoKHRBuilder<'a>(CommandBufferSubmitInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> CommandBufferSubmitInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> CommandBufferSubmitInfoKHRBuilder<'a> {
        CommandBufferSubmitInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn command_buffer(mut self, command_buffer: crate::vk1_0::CommandBuffer) -> Self {
        self.0.command_buffer = command_buffer as _;
        self
    }
    #[inline]
    pub fn device_mask(mut self, device_mask: u32) -> Self {
        self.0.device_mask = device_mask as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> CommandBufferSubmitInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for CommandBufferSubmitInfoKHRBuilder<'a> {
    fn default() -> CommandBufferSubmitInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for CommandBufferSubmitInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for CommandBufferSubmitInfoKHRBuilder<'a> {
    type Target = CommandBufferSubmitInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for CommandBufferSubmitInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubmitInfo2KHR.html) · Structure"]
#[doc(alias = "VkSubmitInfo2KHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SubmitInfo2KHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::khr_synchronization2::SubmitFlagsKHR,
    pub wait_semaphore_info_count: u32,
    pub p_wait_semaphore_infos: *const crate::extensions::khr_synchronization2::SemaphoreSubmitInfoKHR,
    pub command_buffer_info_count: u32,
    pub p_command_buffer_infos: *const crate::extensions::khr_synchronization2::CommandBufferSubmitInfoKHR,
    pub signal_semaphore_info_count: u32,
    pub p_signal_semaphore_infos: *const crate::extensions::khr_synchronization2::SemaphoreSubmitInfoKHR,
}
impl Default for SubmitInfo2KHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::SUBMIT_INFO_2_KHR,
            p_next: std::ptr::null(),
            flags: Default::default(),
            wait_semaphore_info_count: Default::default(),
            p_wait_semaphore_infos: std::ptr::null(),
            command_buffer_info_count: Default::default(),
            p_command_buffer_infos: std::ptr::null(),
            signal_semaphore_info_count: Default::default(),
            p_signal_semaphore_infos: std::ptr::null(),
        }
    }
}
impl std::fmt::Debug for SubmitInfo2KHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubmitInfo2KHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("wait_semaphore_info_count", &self.wait_semaphore_info_count)
            .field("p_wait_semaphore_infos", &self.p_wait_semaphore_infos)
            .field("command_buffer_info_count", &self.command_buffer_info_count)
            .field("p_command_buffer_infos", &self.p_command_buffer_infos)
            .field("signal_semaphore_info_count", &self.signal_semaphore_info_count)
            .field("p_signal_semaphore_infos", &self.p_signal_semaphore_infos)
            .finish()
    }
}
impl SubmitInfo2KHR {
    #[inline]
    pub fn into_builder<'a>(self) -> SubmitInfo2KHRBuilder<'a> {
        SubmitInfo2KHRBuilder(self, std::marker::PhantomData)
    }
}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::nv_win32_keyed_mutex::Win32KeyedMutexAcquireReleaseInfoNV> for SubmitInfo2KHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::nv_win32_keyed_mutex::Win32KeyedMutexAcquireReleaseInfoNVBuilder<'_>> for SubmitInfo2KHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::khr_win32_keyed_mutex::Win32KeyedMutexAcquireReleaseInfoKHR> for SubmitInfo2KHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::khr_win32_keyed_mutex::Win32KeyedMutexAcquireReleaseInfoKHRBuilder<'_>> for SubmitInfo2KHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::khr_performance_query::PerformanceQuerySubmitInfoKHR> for SubmitInfo2KHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::khr_performance_query::PerformanceQuerySubmitInfoKHRBuilder<'_>> for SubmitInfo2KHRBuilder<'a> {}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubmitInfo2KHR.html) · Builder of [`SubmitInfo2KHR`]"]
#[repr(transparent)]
pub struct SubmitInfo2KHRBuilder<'a>(SubmitInfo2KHR, std::marker::PhantomData<&'a ()>);
impl<'a> SubmitInfo2KHRBuilder<'a> {
    #[inline]
    pub fn new() -> SubmitInfo2KHRBuilder<'a> {
        SubmitInfo2KHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::khr_synchronization2::SubmitFlagsKHR) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn wait_semaphore_infos(mut self, wait_semaphore_infos: &'a [crate::extensions::khr_synchronization2::SemaphoreSubmitInfoKHRBuilder]) -> Self {
        self.0.p_wait_semaphore_infos = wait_semaphore_infos.as_ptr() as _;
        self.0.wait_semaphore_info_count = wait_semaphore_infos.len() as _;
        self
    }
    #[inline]
    pub fn command_buffer_infos(mut self, command_buffer_infos: &'a [crate::extensions::khr_synchronization2::CommandBufferSubmitInfoKHRBuilder]) -> Self {
        self.0.p_command_buffer_infos = command_buffer_infos.as_ptr() as _;
        self.0.command_buffer_info_count = command_buffer_infos.len() as _;
        self
    }
    #[inline]
    pub fn signal_semaphore_infos(mut self, signal_semaphore_infos: &'a [crate::extensions::khr_synchronization2::SemaphoreSubmitInfoKHRBuilder]) -> Self {
        self.0.p_signal_semaphore_infos = signal_semaphore_infos.as_ptr() as _;
        self.0.signal_semaphore_info_count = signal_semaphore_infos.len() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> SubmitInfo2KHR {
        self.0
    }
}
impl<'a> std::default::Default for SubmitInfo2KHRBuilder<'a> {
    fn default() -> SubmitInfo2KHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for SubmitInfo2KHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for SubmitInfo2KHRBuilder<'a> {
    type Target = SubmitInfo2KHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SubmitInfo2KHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueueFamilyCheckpointProperties2NV.html) · Structure"]
#[doc(alias = "VkQueueFamilyCheckpointProperties2NV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct QueueFamilyCheckpointProperties2NV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub checkpoint_execution_stage_mask: crate::extensions::khr_synchronization2::PipelineStageFlags2KHR,
}
impl Default for QueueFamilyCheckpointProperties2NV {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::QUEUE_FAMILY_CHECKPOINT_PROPERTIES_2_NV,
            p_next: std::ptr::null_mut(),
            checkpoint_execution_stage_mask: Default::default(),
        }
    }
}
impl std::fmt::Debug for QueueFamilyCheckpointProperties2NV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("QueueFamilyCheckpointProperties2NV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("checkpoint_execution_stage_mask", &self.checkpoint_execution_stage_mask)
            .finish()
    }
}
impl QueueFamilyCheckpointProperties2NV {
    #[inline]
    pub fn into_builder<'a>(self) -> QueueFamilyCheckpointProperties2NVBuilder<'a> {
        QueueFamilyCheckpointProperties2NVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueueFamilyCheckpointProperties2NV.html) · Builder of [`QueueFamilyCheckpointProperties2NV`]"]
#[repr(transparent)]
pub struct QueueFamilyCheckpointProperties2NVBuilder<'a>(QueueFamilyCheckpointProperties2NV, std::marker::PhantomData<&'a ()>);
impl<'a> QueueFamilyCheckpointProperties2NVBuilder<'a> {
    #[inline]
    pub fn new() -> QueueFamilyCheckpointProperties2NVBuilder<'a> {
        QueueFamilyCheckpointProperties2NVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn checkpoint_execution_stage_mask(mut self, checkpoint_execution_stage_mask: crate::extensions::khr_synchronization2::PipelineStageFlags2KHR) -> Self {
        self.0.checkpoint_execution_stage_mask = checkpoint_execution_stage_mask as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> QueueFamilyCheckpointProperties2NV {
        self.0
    }
}
impl<'a> std::default::Default for QueueFamilyCheckpointProperties2NVBuilder<'a> {
    fn default() -> QueueFamilyCheckpointProperties2NVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for QueueFamilyCheckpointProperties2NVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for QueueFamilyCheckpointProperties2NVBuilder<'a> {
    type Target = QueueFamilyCheckpointProperties2NV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for QueueFamilyCheckpointProperties2NVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCheckpointData2NV.html) · Structure"]
#[doc(alias = "VkCheckpointData2NV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CheckpointData2NV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub stage: crate::extensions::khr_synchronization2::PipelineStageFlags2KHR,
    pub p_checkpoint_marker: *mut std::ffi::c_void,
}
impl Default for CheckpointData2NV {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::CHECKPOINT_DATA_2_NV,
            p_next: std::ptr::null_mut(),
            stage: Default::default(),
            p_checkpoint_marker: std::ptr::null_mut(),
        }
    }
}
impl std::fmt::Debug for CheckpointData2NV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CheckpointData2NV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("stage", &self.stage)
            .field("p_checkpoint_marker", &self.p_checkpoint_marker)
            .finish()
    }
}
impl CheckpointData2NV {
    #[inline]
    pub fn into_builder<'a>(self) -> CheckpointData2NVBuilder<'a> {
        CheckpointData2NVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCheckpointData2NV.html) · Builder of [`CheckpointData2NV`]"]
#[repr(transparent)]
pub struct CheckpointData2NVBuilder<'a>(CheckpointData2NV, std::marker::PhantomData<&'a ()>);
impl<'a> CheckpointData2NVBuilder<'a> {
    #[inline]
    pub fn new() -> CheckpointData2NVBuilder<'a> {
        CheckpointData2NVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn stage(mut self, stage: crate::extensions::khr_synchronization2::PipelineStageFlags2KHR) -> Self {
        self.0.stage = stage as _;
        self
    }
    #[inline]
    pub fn checkpoint_marker(mut self, checkpoint_marker: *mut std::ffi::c_void) -> Self {
        self.0.p_checkpoint_marker = checkpoint_marker;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> CheckpointData2NV {
        self.0
    }
}
impl<'a> std::default::Default for CheckpointData2NVBuilder<'a> {
    fn default() -> CheckpointData2NVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for CheckpointData2NVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for CheckpointData2NVBuilder<'a> {
    type Target = CheckpointData2NV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for CheckpointData2NVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSynchronization2FeaturesKHR.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceSynchronization2FeaturesKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceSynchronization2FeaturesKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub synchronization2: crate::vk1_0::Bool32,
}
impl Default for PhysicalDeviceSynchronization2FeaturesKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES_KHR,
            p_next: std::ptr::null_mut(),
            synchronization2: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceSynchronization2FeaturesKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceSynchronization2FeaturesKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("synchronization2", &(self.synchronization2 != 0))
            .finish()
    }
}
impl PhysicalDeviceSynchronization2FeaturesKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceSynchronization2FeaturesKHRBuilder<'a> {
        PhysicalDeviceSynchronization2FeaturesKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSynchronization2FeaturesKHR.html) · Builder of [`PhysicalDeviceSynchronization2FeaturesKHR`]"]
#[repr(transparent)]
pub struct PhysicalDeviceSynchronization2FeaturesKHRBuilder<'a>(PhysicalDeviceSynchronization2FeaturesKHR, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceSynchronization2FeaturesKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceSynchronization2FeaturesKHRBuilder<'a> {
        PhysicalDeviceSynchronization2FeaturesKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn synchronization2(mut self, synchronization2: bool) -> Self {
        self.0.synchronization2 = synchronization2 as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceSynchronization2FeaturesKHR {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceSynchronization2FeaturesKHRBuilder<'a> {
    fn default() -> PhysicalDeviceSynchronization2FeaturesKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceSynchronization2FeaturesKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceSynchronization2FeaturesKHRBuilder<'a> {
    type Target = PhysicalDeviceSynchronization2FeaturesKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceSynchronization2FeaturesKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::khr_synchronization2`]"]
impl crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetEvent2KHR.html) · Function"]
    #[doc(alias = "vkCmdSetEvent2KHR")]
    pub unsafe fn cmd_set_event2_khr(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        event: crate::vk1_0::Event,
        dependency_info: &crate::extensions::khr_synchronization2::DependencyInfoKHR,
    ) -> () {
        let _function = self.cmd_set_event2_khr.expect("`cmd_set_event2_khr` is not loaded");
        let _return = _function(command_buffer as _, event as _, dependency_info as _);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdResetEvent2KHR.html) · Function"]
    #[doc(alias = "vkCmdResetEvent2KHR")]
    pub unsafe fn cmd_reset_event2_khr(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        event: crate::vk1_0::Event,
        stage_mask: crate::extensions::khr_synchronization2::PipelineStageFlags2KHR,
    ) -> () {
        let _function = self.cmd_reset_event2_khr.expect("`cmd_reset_event2_khr` is not loaded");
        let _return = _function(command_buffer as _, event as _, stage_mask as _);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdWaitEvents2KHR.html) · Function"]
    #[doc(alias = "vkCmdWaitEvents2KHR")]
    pub unsafe fn cmd_wait_events2_khr(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        events: &[crate::vk1_0::Event],
        dependency_infos: &[crate::extensions::khr_synchronization2::DependencyInfoKHRBuilder],
    ) -> () {
        let _function = self.cmd_wait_events2_khr.expect("`cmd_wait_events2_khr` is not loaded");
        let event_count = events.len().min(dependency_infos.len());
        let _return = _function(command_buffer as _, event_count as _, events.as_ptr() as _, dependency_infos.as_ptr() as _);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdPipelineBarrier2KHR.html) · Function"]
    #[doc(alias = "vkCmdPipelineBarrier2KHR")]
    pub unsafe fn cmd_pipeline_barrier2_khr(&self, command_buffer: crate::vk1_0::CommandBuffer, dependency_info: &crate::extensions::khr_synchronization2::DependencyInfoKHR) -> () {
        let _function = self.cmd_pipeline_barrier2_khr.expect("`cmd_pipeline_barrier2_khr` is not loaded");
        let _return = _function(command_buffer as _, dependency_info as _);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueueSubmit2KHR.html) · Function"]
    #[doc(alias = "vkQueueSubmit2KHR")]
    pub unsafe fn queue_submit2_khr(
        &self,
        queue: crate::vk1_0::Queue,
        submits: &[crate::extensions::khr_synchronization2::SubmitInfo2KHRBuilder],
        fence: Option<crate::vk1_0::Fence>,
    ) -> crate::utils::VulkanResult<()> {
        let _function = self.queue_submit2_khr.expect("`queue_submit2_khr` is not loaded");
        let submit_count = submits.len();
        let _return = _function(
            queue as _,
            submit_count as _,
            submits.as_ptr() as _,
            match fence {
                Some(v) => v,
                None => Default::default(),
            },
        );
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdWriteTimestamp2KHR.html) · Function"]
    #[doc(alias = "vkCmdWriteTimestamp2KHR")]
    pub unsafe fn cmd_write_timestamp2_khr(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        stage: crate::extensions::khr_synchronization2::PipelineStageFlags2KHR,
        query_pool: crate::vk1_0::QueryPool,
        query: u32,
    ) -> () {
        let _function = self.cmd_write_timestamp2_khr.expect("`cmd_write_timestamp2_khr` is not loaded");
        let _return = _function(command_buffer as _, stage as _, query_pool as _, query as _);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdWriteBufferMarker2AMD.html) · Function"]
    #[doc(alias = "vkCmdWriteBufferMarker2AMD")]
    pub unsafe fn cmd_write_buffer_marker2_amd(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        stage: crate::extensions::khr_synchronization2::PipelineStageFlags2KHR,
        dst_buffer: crate::vk1_0::Buffer,
        dst_offset: crate::vk1_0::DeviceSize,
        marker: u32,
    ) -> () {
        let _function = self.cmd_write_buffer_marker2_amd.expect("`cmd_write_buffer_marker2_amd` is not loaded");
        let _return = _function(command_buffer as _, stage as _, dst_buffer as _, dst_offset as _, marker as _);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetQueueCheckpointData2NV.html) · Function"]
    #[doc(alias = "vkGetQueueCheckpointData2NV")]
    pub unsafe fn get_queue_checkpoint_data2_nv(&self, queue: crate::vk1_0::Queue, checkpoint_data_count: Option<u32>) -> Vec<crate::extensions::khr_synchronization2::CheckpointData2NV> {
        let _function = self.get_queue_checkpoint_data2_nv.expect("`get_queue_checkpoint_data2_nv` is not loaded");
        let mut checkpoint_data_count = match checkpoint_data_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                _function(queue as _, &mut v, std::ptr::null_mut());
                v
            }
        };
        let mut checkpoint_data = vec![Default::default(); checkpoint_data_count as _];
        let _return = _function(queue as _, &mut checkpoint_data_count, checkpoint_data.as_mut_ptr());
        checkpoint_data
    }
}
