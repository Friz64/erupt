#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NV_DEVICE_GENERATED_COMMANDS_SPEC_VERSION: u32 = 3;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NV_DEVICE_GENERATED_COMMANDS_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_NV_device_generated_commands");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_EXECUTE_GENERATED_COMMANDS_NV: *const std::os::raw::c_char =
    crate::cstr!("vkCmdExecuteGeneratedCommandsNV");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_PREPROCESS_GENERATED_COMMANDS_NV: *const std::os::raw::c_char =
    crate::cstr!("vkCmdPreprocessGeneratedCommandsNV");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_BIND_PIPELINE_SHADER_GROUP_NV: *const std::os::raw::c_char =
    crate::cstr!("vkCmdBindPipelineShaderGroupNV");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_GENERATED_COMMANDS_MEMORY_REQUIREMENTS_NV: *const std::os::raw::c_char =
    crate::cstr!("vkGetGeneratedCommandsMemoryRequirementsNV");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_INDIRECT_COMMANDS_LAYOUT_NV: *const std::os::raw::c_char =
    crate::cstr!("vkCreateIndirectCommandsLayoutNV");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_DESTROY_INDIRECT_COMMANDS_LAYOUT_NV: *const std::os::raw::c_char =
    crate::cstr!("vkDestroyIndirectCommandsLayoutNV");
crate :: non_dispatchable_handle ! ( IndirectCommandsLayoutNV , INDIRECT_COMMANDS_LAYOUT_NV , doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkIndirectCommandsLayoutNV.html) · Non-dispatchable Handle" ) ;
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkIndirectCommandsLayoutUsageFlagsNV.html) · Bitmask of [`IndirectCommandsLayoutUsageFlagBitsNV`](./struct.IndirectCommandsLayoutUsageFlagBitsNV.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct IndirectCommandsLayoutUsageFlagsNV : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; const EXPLICIT_PREPROCESS_NV = IndirectCommandsLayoutUsageFlagBitsNV :: EXPLICIT_PREPROCESS_NV . 0 ; const INDEXED_SEQUENCES_NV = IndirectCommandsLayoutUsageFlagBitsNV :: INDEXED_SEQUENCES_NV . 0 ; const UNORDERED_SEQUENCES_NV = IndirectCommandsLayoutUsageFlagBitsNV :: UNORDERED_SEQUENCES_NV . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkIndirectCommandsLayoutUsageFlagBitsNV.html) · Bits enum of [`IndirectCommandsLayoutUsageFlagsNV`](./struct.IndirectCommandsLayoutUsageFlagsNV.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct IndirectCommandsLayoutUsageFlagBitsNV(pub u32);
impl IndirectCommandsLayoutUsageFlagBitsNV {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> IndirectCommandsLayoutUsageFlagsNV {
        IndirectCommandsLayoutUsageFlagsNV::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for IndirectCommandsLayoutUsageFlagBitsNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::EXPLICIT_PREPROCESS_NV => "EXPLICIT_PREPROCESS_NV",
            &Self::INDEXED_SEQUENCES_NV => "INDEXED_SEQUENCES_NV",
            &Self::UNORDERED_SEQUENCES_NV => "UNORDERED_SEQUENCES_NV",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`extensions::nv_device_generated_commands`](./index.html)"]
impl IndirectCommandsLayoutUsageFlagBitsNV {
    pub const EXPLICIT_PREPROCESS_NV: Self = Self(1);
    pub const INDEXED_SEQUENCES_NV: Self = Self(2);
    pub const UNORDERED_SEQUENCES_NV: Self = Self(4);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkIndirectCommandsTokenTypeNV.html) · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct IndirectCommandsTokenTypeNV(pub i32);
impl std::fmt::Debug for IndirectCommandsTokenTypeNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::SHADER_GROUP_NV => "SHADER_GROUP_NV",
            &Self::STATE_FLAGS_NV => "STATE_FLAGS_NV",
            &Self::INDEX_BUFFER_NV => "INDEX_BUFFER_NV",
            &Self::VERTEX_BUFFER_NV => "VERTEX_BUFFER_NV",
            &Self::PUSH_CONSTANT_NV => "PUSH_CONSTANT_NV",
            &Self::DRAW_INDEXED_NV => "DRAW_INDEXED_NV",
            &Self::DRAW_NV => "DRAW_NV",
            &Self::DRAW_TASKS_NV => "DRAW_TASKS_NV",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`extensions::nv_device_generated_commands`](./index.html)"]
impl IndirectCommandsTokenTypeNV {
    pub const SHADER_GROUP_NV: Self = Self(0);
    pub const STATE_FLAGS_NV: Self = Self(1);
    pub const INDEX_BUFFER_NV: Self = Self(2);
    pub const VERTEX_BUFFER_NV: Self = Self(3);
    pub const PUSH_CONSTANT_NV: Self = Self(4);
    pub const DRAW_INDEXED_NV: Self = Self(5);
    pub const DRAW_NV: Self = Self(6);
    pub const DRAW_TASKS_NV: Self = Self(7);
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkIndirectStateFlagsNV.html) · Bitmask of [`IndirectStateFlagBitsNV`](./struct.IndirectStateFlagBitsNV.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct IndirectStateFlagsNV : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; const FLAG_FRONTFACE_NV = IndirectStateFlagBitsNV :: FLAG_FRONTFACE_NV . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkIndirectStateFlagBitsNV.html) · Bits enum of [`IndirectStateFlagsNV`](./struct.IndirectStateFlagsNV.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct IndirectStateFlagBitsNV(pub u32);
impl IndirectStateFlagBitsNV {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> IndirectStateFlagsNV {
        IndirectStateFlagsNV::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for IndirectStateFlagBitsNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::FLAG_FRONTFACE_NV => "FLAG_FRONTFACE_NV",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`extensions::nv_device_generated_commands`](./index.html)"]
impl IndirectStateFlagBitsNV {
    pub const FLAG_FRONTFACE_NV: Self = Self(1);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdExecuteGeneratedCommandsNV.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdExecuteGeneratedCommandsNV = unsafe extern "system" fn ( command_buffer : crate :: vk1_0 :: CommandBuffer , is_preprocessed : crate :: vk1_0 :: Bool32 , p_generated_commands_info : * const crate :: extensions :: nv_device_generated_commands :: GeneratedCommandsInfoNV ) -> std :: ffi :: c_void ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdPreprocessGeneratedCommandsNV.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdPreprocessGeneratedCommandsNV = unsafe extern "system" fn ( command_buffer : crate :: vk1_0 :: CommandBuffer , p_generated_commands_info : * const crate :: extensions :: nv_device_generated_commands :: GeneratedCommandsInfoNV ) -> std :: ffi :: c_void ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBindPipelineShaderGroupNV.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindPipelineShaderGroupNV = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    pipeline_bind_point: crate::vk1_0::PipelineBindPoint,
    pipeline: crate::vk1_0::Pipeline,
    group_index: u32,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetGeneratedCommandsMemoryRequirementsNV.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetGeneratedCommandsMemoryRequirementsNV = unsafe extern "system" fn ( device : crate :: vk1_0 :: Device , p_info : * const crate :: extensions :: nv_device_generated_commands :: GeneratedCommandsMemoryRequirementsInfoNV , p_memory_requirements : * mut crate :: vk1_1 :: MemoryRequirements2 ) -> std :: ffi :: c_void ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateIndirectCommandsLayoutNV.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateIndirectCommandsLayoutNV = unsafe extern "system" fn ( device : crate :: vk1_0 :: Device , p_create_info : * const crate :: extensions :: nv_device_generated_commands :: IndirectCommandsLayoutCreateInfoNV , p_allocator : * const crate :: vk1_0 :: AllocationCallbacks , p_indirect_commands_layout : * mut crate :: extensions :: nv_device_generated_commands :: IndirectCommandsLayoutNV ) -> crate :: vk1_0 :: Result ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyIndirectCommandsLayoutNV.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyIndirectCommandsLayoutNV = unsafe extern "system" fn ( device : crate :: vk1_0 :: Device , indirect_commands_layout : crate :: extensions :: nv_device_generated_commands :: IndirectCommandsLayoutNV , p_allocator : * const crate :: vk1_0 :: AllocationCallbacks ) -> std :: ffi :: c_void ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceDeviceGeneratedCommandsFeaturesNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub device_generated_commands: crate::vk1_0::Bool32,
}
impl Default for PhysicalDeviceDeviceGeneratedCommandsFeaturesNV {
    fn default() -> Self {
        Self {
            s_type:
                crate::vk1_0::StructureType::PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_NV,
            p_next: std::ptr::null_mut(),
            device_generated_commands: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceDeviceGeneratedCommandsFeaturesNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceDeviceGeneratedCommandsFeaturesNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "device_generated_commands",
                &(self.device_generated_commands != 0),
            )
            .finish()
    }
}
impl PhysicalDeviceDeviceGeneratedCommandsFeaturesNV {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceDeviceGeneratedCommandsFeaturesNVBuilder<'a> {
        PhysicalDeviceDeviceGeneratedCommandsFeaturesNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV.html) · Builder of [`PhysicalDeviceDeviceGeneratedCommandsFeaturesNV`](struct.PhysicalDeviceDeviceGeneratedCommandsFeaturesNV.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceDeviceGeneratedCommandsFeaturesNVBuilder<'a>(
    PhysicalDeviceDeviceGeneratedCommandsFeaturesNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceDeviceGeneratedCommandsFeaturesNVBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceDeviceGeneratedCommandsFeaturesNVBuilder<'a> {
        PhysicalDeviceDeviceGeneratedCommandsFeaturesNVBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[inline]
    pub fn device_generated_commands(mut self, device_generated_commands: bool) -> Self {
        self.0.device_generated_commands = device_generated_commands as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceDeviceGeneratedCommandsFeaturesNV {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceDeviceGeneratedCommandsFeaturesNVBuilder<'a> {
    fn default() -> PhysicalDeviceDeviceGeneratedCommandsFeaturesNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceDeviceGeneratedCommandsFeaturesNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceDeviceGeneratedCommandsFeaturesNVBuilder<'a> {
    type Target = PhysicalDeviceDeviceGeneratedCommandsFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceDeviceGeneratedCommandsFeaturesNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceDeviceGeneratedCommandsPropertiesNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub max_graphics_shader_group_count: u32,
    pub max_indirect_sequence_count: u32,
    pub max_indirect_commands_token_count: u32,
    pub max_indirect_commands_stream_count: u32,
    pub max_indirect_commands_token_offset: u32,
    pub max_indirect_commands_stream_stride: u32,
    pub min_sequences_count_buffer_offset_alignment: u32,
    pub min_sequences_index_buffer_offset_alignment: u32,
    pub min_indirect_commands_buffer_offset_alignment: u32,
}
impl Default for PhysicalDeviceDeviceGeneratedCommandsPropertiesNV {
    fn default() -> Self {
        Self {
            s_type:
                crate::vk1_0::StructureType::PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_NV,
            p_next: std::ptr::null_mut(),
            max_graphics_shader_group_count: Default::default(),
            max_indirect_sequence_count: Default::default(),
            max_indirect_commands_token_count: Default::default(),
            max_indirect_commands_stream_count: Default::default(),
            max_indirect_commands_token_offset: Default::default(),
            max_indirect_commands_stream_stride: Default::default(),
            min_sequences_count_buffer_offset_alignment: Default::default(),
            min_sequences_index_buffer_offset_alignment: Default::default(),
            min_indirect_commands_buffer_offset_alignment: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceDeviceGeneratedCommandsPropertiesNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceDeviceGeneratedCommandsPropertiesNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "max_graphics_shader_group_count",
                &self.max_graphics_shader_group_count,
            )
            .field(
                "max_indirect_sequence_count",
                &self.max_indirect_sequence_count,
            )
            .field(
                "max_indirect_commands_token_count",
                &self.max_indirect_commands_token_count,
            )
            .field(
                "max_indirect_commands_stream_count",
                &self.max_indirect_commands_stream_count,
            )
            .field(
                "max_indirect_commands_token_offset",
                &self.max_indirect_commands_token_offset,
            )
            .field(
                "max_indirect_commands_stream_stride",
                &self.max_indirect_commands_stream_stride,
            )
            .field(
                "min_sequences_count_buffer_offset_alignment",
                &self.min_sequences_count_buffer_offset_alignment,
            )
            .field(
                "min_sequences_index_buffer_offset_alignment",
                &self.min_sequences_index_buffer_offset_alignment,
            )
            .field(
                "min_indirect_commands_buffer_offset_alignment",
                &self.min_indirect_commands_buffer_offset_alignment,
            )
            .finish()
    }
}
impl PhysicalDeviceDeviceGeneratedCommandsPropertiesNV {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceDeviceGeneratedCommandsPropertiesNVBuilder<'a> {
        PhysicalDeviceDeviceGeneratedCommandsPropertiesNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV.html) · Builder of [`PhysicalDeviceDeviceGeneratedCommandsPropertiesNV`](struct.PhysicalDeviceDeviceGeneratedCommandsPropertiesNV.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceDeviceGeneratedCommandsPropertiesNVBuilder<'a>(
    PhysicalDeviceDeviceGeneratedCommandsPropertiesNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceDeviceGeneratedCommandsPropertiesNVBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceDeviceGeneratedCommandsPropertiesNVBuilder<'a> {
        PhysicalDeviceDeviceGeneratedCommandsPropertiesNVBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[inline]
    pub fn max_graphics_shader_group_count(mut self, max_graphics_shader_group_count: u32) -> Self {
        self.0.max_graphics_shader_group_count = max_graphics_shader_group_count as _;
        self
    }
    #[inline]
    pub fn max_indirect_sequence_count(mut self, max_indirect_sequence_count: u32) -> Self {
        self.0.max_indirect_sequence_count = max_indirect_sequence_count as _;
        self
    }
    #[inline]
    pub fn max_indirect_commands_token_count(
        mut self,
        max_indirect_commands_token_count: u32,
    ) -> Self {
        self.0.max_indirect_commands_token_count = max_indirect_commands_token_count as _;
        self
    }
    #[inline]
    pub fn max_indirect_commands_stream_count(
        mut self,
        max_indirect_commands_stream_count: u32,
    ) -> Self {
        self.0.max_indirect_commands_stream_count = max_indirect_commands_stream_count as _;
        self
    }
    #[inline]
    pub fn max_indirect_commands_token_offset(
        mut self,
        max_indirect_commands_token_offset: u32,
    ) -> Self {
        self.0.max_indirect_commands_token_offset = max_indirect_commands_token_offset as _;
        self
    }
    #[inline]
    pub fn max_indirect_commands_stream_stride(
        mut self,
        max_indirect_commands_stream_stride: u32,
    ) -> Self {
        self.0.max_indirect_commands_stream_stride = max_indirect_commands_stream_stride as _;
        self
    }
    #[inline]
    pub fn min_sequences_count_buffer_offset_alignment(
        mut self,
        min_sequences_count_buffer_offset_alignment: u32,
    ) -> Self {
        self.0.min_sequences_count_buffer_offset_alignment =
            min_sequences_count_buffer_offset_alignment as _;
        self
    }
    #[inline]
    pub fn min_sequences_index_buffer_offset_alignment(
        mut self,
        min_sequences_index_buffer_offset_alignment: u32,
    ) -> Self {
        self.0.min_sequences_index_buffer_offset_alignment =
            min_sequences_index_buffer_offset_alignment as _;
        self
    }
    #[inline]
    pub fn min_indirect_commands_buffer_offset_alignment(
        mut self,
        min_indirect_commands_buffer_offset_alignment: u32,
    ) -> Self {
        self.0.min_indirect_commands_buffer_offset_alignment =
            min_indirect_commands_buffer_offset_alignment as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceDeviceGeneratedCommandsPropertiesNV {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceDeviceGeneratedCommandsPropertiesNVBuilder<'a> {
    fn default() -> PhysicalDeviceDeviceGeneratedCommandsPropertiesNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceDeviceGeneratedCommandsPropertiesNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceDeviceGeneratedCommandsPropertiesNVBuilder<'a> {
    type Target = PhysicalDeviceDeviceGeneratedCommandsPropertiesNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceDeviceGeneratedCommandsPropertiesNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGraphicsShaderGroupCreateInfoNV.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GraphicsShaderGroupCreateInfoNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub stage_count: u32,
    pub p_stages: *const crate::vk1_0::PipelineShaderStageCreateInfo,
    pub p_vertex_input_state: *const crate::vk1_0::PipelineVertexInputStateCreateInfo,
    pub p_tessellation_state: *const crate::vk1_0::PipelineTessellationStateCreateInfo,
}
impl Default for GraphicsShaderGroupCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::GRAPHICS_SHADER_GROUP_CREATE_INFO_NV,
            p_next: std::ptr::null(),
            stage_count: Default::default(),
            p_stages: std::ptr::null(),
            p_vertex_input_state: std::ptr::null(),
            p_tessellation_state: std::ptr::null(),
        }
    }
}
impl std::fmt::Debug for GraphicsShaderGroupCreateInfoNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("GraphicsShaderGroupCreateInfoNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("stage_count", &self.stage_count)
            .field("p_stages", &self.p_stages)
            .field("p_vertex_input_state", &self.p_vertex_input_state)
            .field("p_tessellation_state", &self.p_tessellation_state)
            .finish()
    }
}
impl GraphicsShaderGroupCreateInfoNV {
    #[inline]
    pub fn into_builder<'a>(self) -> GraphicsShaderGroupCreateInfoNVBuilder<'a> {
        GraphicsShaderGroupCreateInfoNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGraphicsShaderGroupCreateInfoNV.html) · Builder of [`GraphicsShaderGroupCreateInfoNV`](struct.GraphicsShaderGroupCreateInfoNV.html)"]
#[repr(transparent)]
pub struct GraphicsShaderGroupCreateInfoNVBuilder<'a>(
    GraphicsShaderGroupCreateInfoNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> GraphicsShaderGroupCreateInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> GraphicsShaderGroupCreateInfoNVBuilder<'a> {
        GraphicsShaderGroupCreateInfoNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn stages(
        mut self,
        stages: &'a [crate::vk1_0::PipelineShaderStageCreateInfoBuilder],
    ) -> Self {
        self.0.p_stages = stages.as_ptr() as _;
        self.0.stage_count = stages.len() as _;
        self
    }
    #[inline]
    pub fn vertex_input_state(
        mut self,
        vertex_input_state: &'a crate::vk1_0::PipelineVertexInputStateCreateInfo,
    ) -> Self {
        self.0.p_vertex_input_state = vertex_input_state as _;
        self
    }
    #[inline]
    pub fn tessellation_state(
        mut self,
        tessellation_state: &'a crate::vk1_0::PipelineTessellationStateCreateInfo,
    ) -> Self {
        self.0.p_tessellation_state = tessellation_state as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> GraphicsShaderGroupCreateInfoNV {
        self.0
    }
}
impl<'a> std::default::Default for GraphicsShaderGroupCreateInfoNVBuilder<'a> {
    fn default() -> GraphicsShaderGroupCreateInfoNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for GraphicsShaderGroupCreateInfoNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for GraphicsShaderGroupCreateInfoNVBuilder<'a> {
    type Target = GraphicsShaderGroupCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for GraphicsShaderGroupCreateInfoNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGraphicsPipelineShaderGroupsCreateInfoNV.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GraphicsPipelineShaderGroupsCreateInfoNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub group_count: u32,
    pub p_groups:
        *const crate::extensions::nv_device_generated_commands::GraphicsShaderGroupCreateInfoNV,
    pub pipeline_count: u32,
    pub p_pipelines: *const crate::vk1_0::Pipeline,
}
impl Default for GraphicsPipelineShaderGroupsCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::GRAPHICS_PIPELINE_SHADER_GROUPS_CREATE_INFO_NV,
            p_next: std::ptr::null(),
            group_count: Default::default(),
            p_groups: std::ptr::null(),
            pipeline_count: Default::default(),
            p_pipelines: std::ptr::null(),
        }
    }
}
impl std::fmt::Debug for GraphicsPipelineShaderGroupsCreateInfoNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("GraphicsPipelineShaderGroupsCreateInfoNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("group_count", &self.group_count)
            .field("p_groups", &self.p_groups)
            .field("pipeline_count", &self.pipeline_count)
            .field("p_pipelines", &self.p_pipelines)
            .finish()
    }
}
impl GraphicsPipelineShaderGroupsCreateInfoNV {
    #[inline]
    pub fn into_builder<'a>(self) -> GraphicsPipelineShaderGroupsCreateInfoNVBuilder<'a> {
        GraphicsPipelineShaderGroupsCreateInfoNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGraphicsPipelineShaderGroupsCreateInfoNV.html) · Builder of [`GraphicsPipelineShaderGroupsCreateInfoNV`](struct.GraphicsPipelineShaderGroupsCreateInfoNV.html)"]
#[repr(transparent)]
pub struct GraphicsPipelineShaderGroupsCreateInfoNVBuilder<'a>(
    GraphicsPipelineShaderGroupsCreateInfoNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> GraphicsPipelineShaderGroupsCreateInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> GraphicsPipelineShaderGroupsCreateInfoNVBuilder<'a> {
        GraphicsPipelineShaderGroupsCreateInfoNVBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[inline]
    pub fn groups(
        mut self,
        groups : & 'a [ crate :: extensions :: nv_device_generated_commands :: GraphicsShaderGroupCreateInfoNVBuilder ],
    ) -> Self {
        self.0.p_groups = groups.as_ptr() as _;
        self.0.group_count = groups.len() as _;
        self
    }
    #[inline]
    pub fn pipelines(mut self, pipelines: &'a [crate::vk1_0::Pipeline]) -> Self {
        self.0.p_pipelines = pipelines.as_ptr() as _;
        self.0.pipeline_count = pipelines.len() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> GraphicsPipelineShaderGroupsCreateInfoNV {
        self.0
    }
}
impl<'a> std::default::Default for GraphicsPipelineShaderGroupsCreateInfoNVBuilder<'a> {
    fn default() -> GraphicsPipelineShaderGroupsCreateInfoNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for GraphicsPipelineShaderGroupsCreateInfoNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for GraphicsPipelineShaderGroupsCreateInfoNVBuilder<'a> {
    type Target = GraphicsPipelineShaderGroupsCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for GraphicsPipelineShaderGroupsCreateInfoNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindShaderGroupIndirectCommandNV.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BindShaderGroupIndirectCommandNV {
    pub group_index: u32,
}
impl Default for BindShaderGroupIndirectCommandNV {
    fn default() -> Self {
        Self {
            group_index: Default::default(),
        }
    }
}
impl std::fmt::Debug for BindShaderGroupIndirectCommandNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BindShaderGroupIndirectCommandNV")
            .field("group_index", &self.group_index)
            .finish()
    }
}
impl BindShaderGroupIndirectCommandNV {
    #[inline]
    pub fn into_builder<'a>(self) -> BindShaderGroupIndirectCommandNVBuilder<'a> {
        BindShaderGroupIndirectCommandNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindShaderGroupIndirectCommandNV.html) · Builder of [`BindShaderGroupIndirectCommandNV`](struct.BindShaderGroupIndirectCommandNV.html)"]
#[repr(transparent)]
pub struct BindShaderGroupIndirectCommandNVBuilder<'a>(
    BindShaderGroupIndirectCommandNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> BindShaderGroupIndirectCommandNVBuilder<'a> {
    #[inline]
    pub fn new() -> BindShaderGroupIndirectCommandNVBuilder<'a> {
        BindShaderGroupIndirectCommandNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn group_index(mut self, group_index: u32) -> Self {
        self.0.group_index = group_index as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> BindShaderGroupIndirectCommandNV {
        self.0
    }
}
impl<'a> std::default::Default for BindShaderGroupIndirectCommandNVBuilder<'a> {
    fn default() -> BindShaderGroupIndirectCommandNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for BindShaderGroupIndirectCommandNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for BindShaderGroupIndirectCommandNVBuilder<'a> {
    type Target = BindShaderGroupIndirectCommandNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for BindShaderGroupIndirectCommandNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindIndexBufferIndirectCommandNV.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BindIndexBufferIndirectCommandNV {
    pub buffer_address: crate::vk1_0::DeviceAddress,
    pub size: u32,
    pub index_type: crate::vk1_0::IndexType,
}
impl Default for BindIndexBufferIndirectCommandNV {
    fn default() -> Self {
        Self {
            buffer_address: Default::default(),
            size: Default::default(),
            index_type: Default::default(),
        }
    }
}
impl std::fmt::Debug for BindIndexBufferIndirectCommandNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BindIndexBufferIndirectCommandNV")
            .field("buffer_address", &self.buffer_address)
            .field("size", &self.size)
            .field("index_type", &self.index_type)
            .finish()
    }
}
impl BindIndexBufferIndirectCommandNV {
    #[inline]
    pub fn into_builder<'a>(self) -> BindIndexBufferIndirectCommandNVBuilder<'a> {
        BindIndexBufferIndirectCommandNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindIndexBufferIndirectCommandNV.html) · Builder of [`BindIndexBufferIndirectCommandNV`](struct.BindIndexBufferIndirectCommandNV.html)"]
#[repr(transparent)]
pub struct BindIndexBufferIndirectCommandNVBuilder<'a>(
    BindIndexBufferIndirectCommandNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> BindIndexBufferIndirectCommandNVBuilder<'a> {
    #[inline]
    pub fn new() -> BindIndexBufferIndirectCommandNVBuilder<'a> {
        BindIndexBufferIndirectCommandNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn buffer_address(mut self, buffer_address: crate::vk1_0::DeviceAddress) -> Self {
        self.0.buffer_address = buffer_address as _;
        self
    }
    #[inline]
    pub fn size(mut self, size: u32) -> Self {
        self.0.size = size as _;
        self
    }
    #[inline]
    pub fn index_type(mut self, index_type: crate::vk1_0::IndexType) -> Self {
        self.0.index_type = index_type as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> BindIndexBufferIndirectCommandNV {
        self.0
    }
}
impl<'a> std::default::Default for BindIndexBufferIndirectCommandNVBuilder<'a> {
    fn default() -> BindIndexBufferIndirectCommandNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for BindIndexBufferIndirectCommandNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for BindIndexBufferIndirectCommandNVBuilder<'a> {
    type Target = BindIndexBufferIndirectCommandNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for BindIndexBufferIndirectCommandNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindVertexBufferIndirectCommandNV.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BindVertexBufferIndirectCommandNV {
    pub buffer_address: crate::vk1_0::DeviceAddress,
    pub size: u32,
    pub stride: u32,
}
impl Default for BindVertexBufferIndirectCommandNV {
    fn default() -> Self {
        Self {
            buffer_address: Default::default(),
            size: Default::default(),
            stride: Default::default(),
        }
    }
}
impl std::fmt::Debug for BindVertexBufferIndirectCommandNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BindVertexBufferIndirectCommandNV")
            .field("buffer_address", &self.buffer_address)
            .field("size", &self.size)
            .field("stride", &self.stride)
            .finish()
    }
}
impl BindVertexBufferIndirectCommandNV {
    #[inline]
    pub fn into_builder<'a>(self) -> BindVertexBufferIndirectCommandNVBuilder<'a> {
        BindVertexBufferIndirectCommandNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindVertexBufferIndirectCommandNV.html) · Builder of [`BindVertexBufferIndirectCommandNV`](struct.BindVertexBufferIndirectCommandNV.html)"]
#[repr(transparent)]
pub struct BindVertexBufferIndirectCommandNVBuilder<'a>(
    BindVertexBufferIndirectCommandNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> BindVertexBufferIndirectCommandNVBuilder<'a> {
    #[inline]
    pub fn new() -> BindVertexBufferIndirectCommandNVBuilder<'a> {
        BindVertexBufferIndirectCommandNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn buffer_address(mut self, buffer_address: crate::vk1_0::DeviceAddress) -> Self {
        self.0.buffer_address = buffer_address as _;
        self
    }
    #[inline]
    pub fn size(mut self, size: u32) -> Self {
        self.0.size = size as _;
        self
    }
    #[inline]
    pub fn stride(mut self, stride: u32) -> Self {
        self.0.stride = stride as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> BindVertexBufferIndirectCommandNV {
        self.0
    }
}
impl<'a> std::default::Default for BindVertexBufferIndirectCommandNVBuilder<'a> {
    fn default() -> BindVertexBufferIndirectCommandNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for BindVertexBufferIndirectCommandNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for BindVertexBufferIndirectCommandNVBuilder<'a> {
    type Target = BindVertexBufferIndirectCommandNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for BindVertexBufferIndirectCommandNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSetStateFlagsIndirectCommandNV.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SetStateFlagsIndirectCommandNV {
    pub data: u32,
}
impl Default for SetStateFlagsIndirectCommandNV {
    fn default() -> Self {
        Self {
            data: Default::default(),
        }
    }
}
impl std::fmt::Debug for SetStateFlagsIndirectCommandNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SetStateFlagsIndirectCommandNV")
            .field("data", &self.data)
            .finish()
    }
}
impl SetStateFlagsIndirectCommandNV {
    #[inline]
    pub fn into_builder<'a>(self) -> SetStateFlagsIndirectCommandNVBuilder<'a> {
        SetStateFlagsIndirectCommandNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSetStateFlagsIndirectCommandNV.html) · Builder of [`SetStateFlagsIndirectCommandNV`](struct.SetStateFlagsIndirectCommandNV.html)"]
#[repr(transparent)]
pub struct SetStateFlagsIndirectCommandNVBuilder<'a>(
    SetStateFlagsIndirectCommandNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> SetStateFlagsIndirectCommandNVBuilder<'a> {
    #[inline]
    pub fn new() -> SetStateFlagsIndirectCommandNVBuilder<'a> {
        SetStateFlagsIndirectCommandNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn data(mut self, data: u32) -> Self {
        self.0.data = data as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> SetStateFlagsIndirectCommandNV {
        self.0
    }
}
impl<'a> std::default::Default for SetStateFlagsIndirectCommandNVBuilder<'a> {
    fn default() -> SetStateFlagsIndirectCommandNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for SetStateFlagsIndirectCommandNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for SetStateFlagsIndirectCommandNVBuilder<'a> {
    type Target = SetStateFlagsIndirectCommandNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SetStateFlagsIndirectCommandNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkIndirectCommandsStreamNV.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IndirectCommandsStreamNV {
    pub buffer: crate::vk1_0::Buffer,
    pub offset: crate::vk1_0::DeviceSize,
}
impl Default for IndirectCommandsStreamNV {
    fn default() -> Self {
        Self {
            buffer: Default::default(),
            offset: Default::default(),
        }
    }
}
impl std::fmt::Debug for IndirectCommandsStreamNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IndirectCommandsStreamNV")
            .field("buffer", &self.buffer)
            .field("offset", &self.offset)
            .finish()
    }
}
impl IndirectCommandsStreamNV {
    #[inline]
    pub fn into_builder<'a>(self) -> IndirectCommandsStreamNVBuilder<'a> {
        IndirectCommandsStreamNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkIndirectCommandsStreamNV.html) · Builder of [`IndirectCommandsStreamNV`](struct.IndirectCommandsStreamNV.html)"]
#[repr(transparent)]
pub struct IndirectCommandsStreamNVBuilder<'a>(
    IndirectCommandsStreamNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> IndirectCommandsStreamNVBuilder<'a> {
    #[inline]
    pub fn new() -> IndirectCommandsStreamNVBuilder<'a> {
        IndirectCommandsStreamNVBuilder(Default::default(), std::marker::PhantomData)
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
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> IndirectCommandsStreamNV {
        self.0
    }
}
impl<'a> std::default::Default for IndirectCommandsStreamNVBuilder<'a> {
    fn default() -> IndirectCommandsStreamNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for IndirectCommandsStreamNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for IndirectCommandsStreamNVBuilder<'a> {
    type Target = IndirectCommandsStreamNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for IndirectCommandsStreamNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkIndirectCommandsLayoutTokenNV.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IndirectCommandsLayoutTokenNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub token_type: crate::extensions::nv_device_generated_commands::IndirectCommandsTokenTypeNV,
    pub stream: u32,
    pub offset: u32,
    pub vertex_binding_unit: u32,
    pub vertex_dynamic_stride: crate::vk1_0::Bool32,
    pub pushconstant_pipeline_layout: crate::vk1_0::PipelineLayout,
    pub pushconstant_shader_stage_flags: crate::vk1_0::ShaderStageFlags,
    pub pushconstant_offset: u32,
    pub pushconstant_size: u32,
    pub indirect_state_flags: crate::extensions::nv_device_generated_commands::IndirectStateFlagsNV,
    pub index_type_count: u32,
    pub p_index_types: *const crate::vk1_0::IndexType,
    pub p_index_type_values: *const u32,
}
impl Default for IndirectCommandsLayoutTokenNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::INDIRECT_COMMANDS_LAYOUT_TOKEN_NV,
            p_next: std::ptr::null(),
            token_type: Default::default(),
            stream: Default::default(),
            offset: Default::default(),
            vertex_binding_unit: Default::default(),
            vertex_dynamic_stride: Default::default(),
            pushconstant_pipeline_layout: Default::default(),
            pushconstant_shader_stage_flags: Default::default(),
            pushconstant_offset: Default::default(),
            pushconstant_size: Default::default(),
            indirect_state_flags: Default::default(),
            index_type_count: Default::default(),
            p_index_types: std::ptr::null(),
            p_index_type_values: std::ptr::null(),
        }
    }
}
impl std::fmt::Debug for IndirectCommandsLayoutTokenNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IndirectCommandsLayoutTokenNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("token_type", &self.token_type)
            .field("stream", &self.stream)
            .field("offset", &self.offset)
            .field("vertex_binding_unit", &self.vertex_binding_unit)
            .field("vertex_dynamic_stride", &(self.vertex_dynamic_stride != 0))
            .field(
                "pushconstant_pipeline_layout",
                &self.pushconstant_pipeline_layout,
            )
            .field(
                "pushconstant_shader_stage_flags",
                &self.pushconstant_shader_stage_flags,
            )
            .field("pushconstant_offset", &self.pushconstant_offset)
            .field("pushconstant_size", &self.pushconstant_size)
            .field("indirect_state_flags", &self.indirect_state_flags)
            .field("index_type_count", &self.index_type_count)
            .field("p_index_types", &self.p_index_types)
            .field("p_index_type_values", &self.p_index_type_values)
            .finish()
    }
}
impl IndirectCommandsLayoutTokenNV {
    #[inline]
    pub fn into_builder<'a>(self) -> IndirectCommandsLayoutTokenNVBuilder<'a> {
        IndirectCommandsLayoutTokenNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkIndirectCommandsLayoutTokenNV.html) · Builder of [`IndirectCommandsLayoutTokenNV`](struct.IndirectCommandsLayoutTokenNV.html)"]
#[repr(transparent)]
pub struct IndirectCommandsLayoutTokenNVBuilder<'a>(
    IndirectCommandsLayoutTokenNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> IndirectCommandsLayoutTokenNVBuilder<'a> {
    #[inline]
    pub fn new() -> IndirectCommandsLayoutTokenNVBuilder<'a> {
        IndirectCommandsLayoutTokenNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn token_type(
        mut self,
        token_type: crate::extensions::nv_device_generated_commands::IndirectCommandsTokenTypeNV,
    ) -> Self {
        self.0.token_type = token_type as _;
        self
    }
    #[inline]
    pub fn stream(mut self, stream: u32) -> Self {
        self.0.stream = stream as _;
        self
    }
    #[inline]
    pub fn offset(mut self, offset: u32) -> Self {
        self.0.offset = offset as _;
        self
    }
    #[inline]
    pub fn vertex_binding_unit(mut self, vertex_binding_unit: u32) -> Self {
        self.0.vertex_binding_unit = vertex_binding_unit as _;
        self
    }
    #[inline]
    pub fn vertex_dynamic_stride(mut self, vertex_dynamic_stride: bool) -> Self {
        self.0.vertex_dynamic_stride = vertex_dynamic_stride as _;
        self
    }
    #[inline]
    pub fn pushconstant_pipeline_layout(
        mut self,
        pushconstant_pipeline_layout: crate::vk1_0::PipelineLayout,
    ) -> Self {
        self.0.pushconstant_pipeline_layout = pushconstant_pipeline_layout as _;
        self
    }
    #[inline]
    pub fn pushconstant_shader_stage_flags(
        mut self,
        pushconstant_shader_stage_flags: crate::vk1_0::ShaderStageFlags,
    ) -> Self {
        self.0.pushconstant_shader_stage_flags = pushconstant_shader_stage_flags as _;
        self
    }
    #[inline]
    pub fn pushconstant_offset(mut self, pushconstant_offset: u32) -> Self {
        self.0.pushconstant_offset = pushconstant_offset as _;
        self
    }
    #[inline]
    pub fn pushconstant_size(mut self, pushconstant_size: u32) -> Self {
        self.0.pushconstant_size = pushconstant_size as _;
        self
    }
    #[inline]
    pub fn indirect_state_flags(
        mut self,
        indirect_state_flags: crate::extensions::nv_device_generated_commands::IndirectStateFlagsNV,
    ) -> Self {
        self.0.indirect_state_flags = indirect_state_flags as _;
        self
    }
    #[inline]
    pub fn index_types(mut self, index_types: &'a [crate::vk1_0::IndexType]) -> Self {
        self.0.p_index_types = index_types.as_ptr() as _;
        self.0.index_type_count = index_types.len() as _;
        self
    }
    #[inline]
    pub fn index_type_values(mut self, index_type_values: &'a [u32]) -> Self {
        self.0.p_index_type_values = index_type_values.as_ptr() as _;
        self.0.index_type_count = index_type_values.len() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> IndirectCommandsLayoutTokenNV {
        self.0
    }
}
impl<'a> std::default::Default for IndirectCommandsLayoutTokenNVBuilder<'a> {
    fn default() -> IndirectCommandsLayoutTokenNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for IndirectCommandsLayoutTokenNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for IndirectCommandsLayoutTokenNVBuilder<'a> {
    type Target = IndirectCommandsLayoutTokenNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for IndirectCommandsLayoutTokenNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkIndirectCommandsLayoutCreateInfoNV.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IndirectCommandsLayoutCreateInfoNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::nv_device_generated_commands::IndirectCommandsLayoutUsageFlagsNV,
    pub pipeline_bind_point: crate::vk1_0::PipelineBindPoint,
    pub token_count: u32,
    pub p_tokens:
        *const crate::extensions::nv_device_generated_commands::IndirectCommandsLayoutTokenNV,
    pub stream_count: u32,
    pub p_stream_strides: *const u32,
}
impl Default for IndirectCommandsLayoutCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NV,
            p_next: std::ptr::null(),
            flags: Default::default(),
            pipeline_bind_point: Default::default(),
            token_count: Default::default(),
            p_tokens: std::ptr::null(),
            stream_count: Default::default(),
            p_stream_strides: std::ptr::null(),
        }
    }
}
impl std::fmt::Debug for IndirectCommandsLayoutCreateInfoNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IndirectCommandsLayoutCreateInfoNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("pipeline_bind_point", &self.pipeline_bind_point)
            .field("token_count", &self.token_count)
            .field("p_tokens", &self.p_tokens)
            .field("stream_count", &self.stream_count)
            .field("p_stream_strides", &self.p_stream_strides)
            .finish()
    }
}
impl IndirectCommandsLayoutCreateInfoNV {
    #[inline]
    pub fn into_builder<'a>(self) -> IndirectCommandsLayoutCreateInfoNVBuilder<'a> {
        IndirectCommandsLayoutCreateInfoNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkIndirectCommandsLayoutCreateInfoNV.html) · Builder of [`IndirectCommandsLayoutCreateInfoNV`](struct.IndirectCommandsLayoutCreateInfoNV.html)"]
#[repr(transparent)]
pub struct IndirectCommandsLayoutCreateInfoNVBuilder<'a>(
    IndirectCommandsLayoutCreateInfoNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> IndirectCommandsLayoutCreateInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> IndirectCommandsLayoutCreateInfoNVBuilder<'a> {
        IndirectCommandsLayoutCreateInfoNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(
        mut self,
        flags: crate::extensions::nv_device_generated_commands::IndirectCommandsLayoutUsageFlagsNV,
    ) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn pipeline_bind_point(
        mut self,
        pipeline_bind_point: crate::vk1_0::PipelineBindPoint,
    ) -> Self {
        self.0.pipeline_bind_point = pipeline_bind_point as _;
        self
    }
    #[inline]
    pub fn tokens(
        mut self,
        tokens : & 'a [ crate :: extensions :: nv_device_generated_commands :: IndirectCommandsLayoutTokenNVBuilder ],
    ) -> Self {
        self.0.p_tokens = tokens.as_ptr() as _;
        self.0.token_count = tokens.len() as _;
        self
    }
    #[inline]
    pub fn stream_strides(mut self, stream_strides: &'a [u32]) -> Self {
        self.0.p_stream_strides = stream_strides.as_ptr() as _;
        self.0.stream_count = stream_strides.len() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> IndirectCommandsLayoutCreateInfoNV {
        self.0
    }
}
impl<'a> std::default::Default for IndirectCommandsLayoutCreateInfoNVBuilder<'a> {
    fn default() -> IndirectCommandsLayoutCreateInfoNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for IndirectCommandsLayoutCreateInfoNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for IndirectCommandsLayoutCreateInfoNVBuilder<'a> {
    type Target = IndirectCommandsLayoutCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for IndirectCommandsLayoutCreateInfoNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeneratedCommandsInfoNV.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GeneratedCommandsInfoNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub pipeline_bind_point: crate::vk1_0::PipelineBindPoint,
    pub pipeline: crate::vk1_0::Pipeline,
    pub indirect_commands_layout:
        crate::extensions::nv_device_generated_commands::IndirectCommandsLayoutNV,
    pub stream_count: u32,
    pub p_streams: *const crate::extensions::nv_device_generated_commands::IndirectCommandsStreamNV,
    pub sequences_count: u32,
    pub preprocess_buffer: crate::vk1_0::Buffer,
    pub preprocess_offset: crate::vk1_0::DeviceSize,
    pub preprocess_size: crate::vk1_0::DeviceSize,
    pub sequences_count_buffer: crate::vk1_0::Buffer,
    pub sequences_count_offset: crate::vk1_0::DeviceSize,
    pub sequences_index_buffer: crate::vk1_0::Buffer,
    pub sequences_index_offset: crate::vk1_0::DeviceSize,
}
impl Default for GeneratedCommandsInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::GENERATED_COMMANDS_INFO_NV,
            p_next: std::ptr::null(),
            pipeline_bind_point: Default::default(),
            pipeline: Default::default(),
            indirect_commands_layout: Default::default(),
            stream_count: Default::default(),
            p_streams: std::ptr::null(),
            sequences_count: Default::default(),
            preprocess_buffer: Default::default(),
            preprocess_offset: Default::default(),
            preprocess_size: Default::default(),
            sequences_count_buffer: Default::default(),
            sequences_count_offset: Default::default(),
            sequences_index_buffer: Default::default(),
            sequences_index_offset: Default::default(),
        }
    }
}
impl std::fmt::Debug for GeneratedCommandsInfoNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("GeneratedCommandsInfoNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("pipeline_bind_point", &self.pipeline_bind_point)
            .field("pipeline", &self.pipeline)
            .field("indirect_commands_layout", &self.indirect_commands_layout)
            .field("stream_count", &self.stream_count)
            .field("p_streams", &self.p_streams)
            .field("sequences_count", &self.sequences_count)
            .field("preprocess_buffer", &self.preprocess_buffer)
            .field("preprocess_offset", &self.preprocess_offset)
            .field("preprocess_size", &self.preprocess_size)
            .field("sequences_count_buffer", &self.sequences_count_buffer)
            .field("sequences_count_offset", &self.sequences_count_offset)
            .field("sequences_index_buffer", &self.sequences_index_buffer)
            .field("sequences_index_offset", &self.sequences_index_offset)
            .finish()
    }
}
impl GeneratedCommandsInfoNV {
    #[inline]
    pub fn into_builder<'a>(self) -> GeneratedCommandsInfoNVBuilder<'a> {
        GeneratedCommandsInfoNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeneratedCommandsInfoNV.html) · Builder of [`GeneratedCommandsInfoNV`](struct.GeneratedCommandsInfoNV.html)"]
#[repr(transparent)]
pub struct GeneratedCommandsInfoNVBuilder<'a>(
    GeneratedCommandsInfoNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> GeneratedCommandsInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> GeneratedCommandsInfoNVBuilder<'a> {
        GeneratedCommandsInfoNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn pipeline_bind_point(
        mut self,
        pipeline_bind_point: crate::vk1_0::PipelineBindPoint,
    ) -> Self {
        self.0.pipeline_bind_point = pipeline_bind_point as _;
        self
    }
    #[inline]
    pub fn pipeline(mut self, pipeline: crate::vk1_0::Pipeline) -> Self {
        self.0.pipeline = pipeline as _;
        self
    }
    #[inline]
    pub fn indirect_commands_layout(
        mut self,
        indirect_commands_layout : crate :: extensions :: nv_device_generated_commands :: IndirectCommandsLayoutNV,
    ) -> Self {
        self.0.indirect_commands_layout = indirect_commands_layout as _;
        self
    }
    #[inline]
    pub fn streams(
        mut self,
        streams : & 'a [ crate :: extensions :: nv_device_generated_commands :: IndirectCommandsStreamNVBuilder ],
    ) -> Self {
        self.0.p_streams = streams.as_ptr() as _;
        self.0.stream_count = streams.len() as _;
        self
    }
    #[inline]
    pub fn sequences_count(mut self, sequences_count: u32) -> Self {
        self.0.sequences_count = sequences_count as _;
        self
    }
    #[inline]
    pub fn preprocess_buffer(mut self, preprocess_buffer: crate::vk1_0::Buffer) -> Self {
        self.0.preprocess_buffer = preprocess_buffer as _;
        self
    }
    #[inline]
    pub fn preprocess_offset(mut self, preprocess_offset: crate::vk1_0::DeviceSize) -> Self {
        self.0.preprocess_offset = preprocess_offset as _;
        self
    }
    #[inline]
    pub fn preprocess_size(mut self, preprocess_size: crate::vk1_0::DeviceSize) -> Self {
        self.0.preprocess_size = preprocess_size as _;
        self
    }
    #[inline]
    pub fn sequences_count_buffer(mut self, sequences_count_buffer: crate::vk1_0::Buffer) -> Self {
        self.0.sequences_count_buffer = sequences_count_buffer as _;
        self
    }
    #[inline]
    pub fn sequences_count_offset(
        mut self,
        sequences_count_offset: crate::vk1_0::DeviceSize,
    ) -> Self {
        self.0.sequences_count_offset = sequences_count_offset as _;
        self
    }
    #[inline]
    pub fn sequences_index_buffer(mut self, sequences_index_buffer: crate::vk1_0::Buffer) -> Self {
        self.0.sequences_index_buffer = sequences_index_buffer as _;
        self
    }
    #[inline]
    pub fn sequences_index_offset(
        mut self,
        sequences_index_offset: crate::vk1_0::DeviceSize,
    ) -> Self {
        self.0.sequences_index_offset = sequences_index_offset as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> GeneratedCommandsInfoNV {
        self.0
    }
}
impl<'a> std::default::Default for GeneratedCommandsInfoNVBuilder<'a> {
    fn default() -> GeneratedCommandsInfoNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for GeneratedCommandsInfoNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for GeneratedCommandsInfoNVBuilder<'a> {
    type Target = GeneratedCommandsInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for GeneratedCommandsInfoNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeneratedCommandsMemoryRequirementsInfoNV.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GeneratedCommandsMemoryRequirementsInfoNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub pipeline_bind_point: crate::vk1_0::PipelineBindPoint,
    pub pipeline: crate::vk1_0::Pipeline,
    pub indirect_commands_layout:
        crate::extensions::nv_device_generated_commands::IndirectCommandsLayoutNV,
    pub max_sequences_count: u32,
}
impl Default for GeneratedCommandsMemoryRequirementsInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_NV,
            p_next: std::ptr::null(),
            pipeline_bind_point: Default::default(),
            pipeline: Default::default(),
            indirect_commands_layout: Default::default(),
            max_sequences_count: Default::default(),
        }
    }
}
impl std::fmt::Debug for GeneratedCommandsMemoryRequirementsInfoNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("GeneratedCommandsMemoryRequirementsInfoNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("pipeline_bind_point", &self.pipeline_bind_point)
            .field("pipeline", &self.pipeline)
            .field("indirect_commands_layout", &self.indirect_commands_layout)
            .field("max_sequences_count", &self.max_sequences_count)
            .finish()
    }
}
impl GeneratedCommandsMemoryRequirementsInfoNV {
    #[inline]
    pub fn into_builder<'a>(self) -> GeneratedCommandsMemoryRequirementsInfoNVBuilder<'a> {
        GeneratedCommandsMemoryRequirementsInfoNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeneratedCommandsMemoryRequirementsInfoNV.html) · Builder of [`GeneratedCommandsMemoryRequirementsInfoNV`](struct.GeneratedCommandsMemoryRequirementsInfoNV.html)"]
#[repr(transparent)]
pub struct GeneratedCommandsMemoryRequirementsInfoNVBuilder<'a>(
    GeneratedCommandsMemoryRequirementsInfoNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> GeneratedCommandsMemoryRequirementsInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> GeneratedCommandsMemoryRequirementsInfoNVBuilder<'a> {
        GeneratedCommandsMemoryRequirementsInfoNVBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[inline]
    pub fn pipeline_bind_point(
        mut self,
        pipeline_bind_point: crate::vk1_0::PipelineBindPoint,
    ) -> Self {
        self.0.pipeline_bind_point = pipeline_bind_point as _;
        self
    }
    #[inline]
    pub fn pipeline(mut self, pipeline: crate::vk1_0::Pipeline) -> Self {
        self.0.pipeline = pipeline as _;
        self
    }
    #[inline]
    pub fn indirect_commands_layout(
        mut self,
        indirect_commands_layout : crate :: extensions :: nv_device_generated_commands :: IndirectCommandsLayoutNV,
    ) -> Self {
        self.0.indirect_commands_layout = indirect_commands_layout as _;
        self
    }
    #[inline]
    pub fn max_sequences_count(mut self, max_sequences_count: u32) -> Self {
        self.0.max_sequences_count = max_sequences_count as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> GeneratedCommandsMemoryRequirementsInfoNV {
        self.0
    }
}
impl<'a> std::default::Default for GeneratedCommandsMemoryRequirementsInfoNVBuilder<'a> {
    fn default() -> GeneratedCommandsMemoryRequirementsInfoNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for GeneratedCommandsMemoryRequirementsInfoNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for GeneratedCommandsMemoryRequirementsInfoNVBuilder<'a> {
    type Target = GeneratedCommandsMemoryRequirementsInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for GeneratedCommandsMemoryRequirementsInfoNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`extensions::nv_device_generated_commands`](extensions/nv_device_generated_commands/index.html)"]
impl crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdExecuteGeneratedCommandsNV.html) · Function"]
    pub unsafe fn cmd_execute_generated_commands_nv(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        is_preprocessed: bool,
        generated_commands_info : & crate :: extensions :: nv_device_generated_commands :: GeneratedCommandsInfoNV,
    ) -> () {
        let _function = self
            .cmd_execute_generated_commands_nv
            .expect("`cmd_execute_generated_commands_nv` is not loaded");
        let _return = _function(
            command_buffer as _,
            is_preprocessed as _,
            generated_commands_info as _,
        );
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdPreprocessGeneratedCommandsNV.html) · Function"]
    pub unsafe fn cmd_preprocess_generated_commands_nv(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        generated_commands_info : & crate :: extensions :: nv_device_generated_commands :: GeneratedCommandsInfoNV,
    ) -> () {
        let _function = self
            .cmd_preprocess_generated_commands_nv
            .expect("`cmd_preprocess_generated_commands_nv` is not loaded");
        let _return = _function(command_buffer as _, generated_commands_info as _);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBindPipelineShaderGroupNV.html) · Function"]
    pub unsafe fn cmd_bind_pipeline_shader_group_nv(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        pipeline_bind_point: crate::vk1_0::PipelineBindPoint,
        pipeline: crate::vk1_0::Pipeline,
        group_index: u32,
    ) -> () {
        let _function = self
            .cmd_bind_pipeline_shader_group_nv
            .expect("`cmd_bind_pipeline_shader_group_nv` is not loaded");
        let _return = _function(
            command_buffer as _,
            pipeline_bind_point as _,
            pipeline as _,
            group_index as _,
        );
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetGeneratedCommandsMemoryRequirementsNV.html) · Function"]
    pub unsafe fn get_generated_commands_memory_requirements_nv(
        &self,
        info : & crate :: extensions :: nv_device_generated_commands :: GeneratedCommandsMemoryRequirementsInfoNV,
        memory_requirements: Option<crate::vk1_1::MemoryRequirements2>,
    ) -> crate::vk1_1::MemoryRequirements2 {
        let _function = self
            .get_generated_commands_memory_requirements_nv
            .expect("`get_generated_commands_memory_requirements_nv` is not loaded");
        let mut memory_requirements = match memory_requirements {
            Some(v) => v,
            None => Default::default(),
        };
        let _return = _function(self.handle, info as _, &mut memory_requirements);
        memory_requirements
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateIndirectCommandsLayoutNV.html) · Function"]
    pub unsafe fn create_indirect_commands_layout_nv(
        &self,
        create_info : & crate :: extensions :: nv_device_generated_commands :: IndirectCommandsLayoutCreateInfoNV,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        indirect_commands_layout: Option<
            crate::extensions::nv_device_generated_commands::IndirectCommandsLayoutNV,
        >,
    ) -> crate::utils::VulkanResult<
        crate::extensions::nv_device_generated_commands::IndirectCommandsLayoutNV,
    > {
        let _function = self
            .create_indirect_commands_layout_nv
            .expect("`create_indirect_commands_layout_nv` is not loaded");
        let mut indirect_commands_layout = match indirect_commands_layout {
            Some(v) => v,
            None => Default::default(),
        };
        let _return = _function(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut indirect_commands_layout,
        );
        crate::utils::VulkanResult::new(_return, indirect_commands_layout)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyIndirectCommandsLayoutNV.html) · Function"]
    pub unsafe fn destroy_indirect_commands_layout_nv(
        &self,
        indirect_commands_layout: Option<
            crate::extensions::nv_device_generated_commands::IndirectCommandsLayoutNV,
        >,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
    ) -> () {
        let _function = self
            .destroy_indirect_commands_layout_nv
            .expect("`destroy_indirect_commands_layout_nv` is not loaded");
        let _return = _function(
            self.handle,
            match indirect_commands_layout {
                Some(v) => v,
                None => Default::default(),
            },
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
        ()
    }
}
