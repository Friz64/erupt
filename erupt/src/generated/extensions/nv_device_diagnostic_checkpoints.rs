#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_DEVICE_DIAGNOSTIC_CHECKPOINTS_SPEC_VERSION")]
pub const NV_DEVICE_DIAGNOSTIC_CHECKPOINTS_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_DEVICE_DIAGNOSTIC_CHECKPOINTS_EXTENSION_NAME")]
pub const NV_DEVICE_DIAGNOSTIC_CHECKPOINTS_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_NV_device_diagnostic_checkpoints");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_SET_CHECKPOINT_NV: *const std::os::raw::c_char = crate::cstr!("vkCmdSetCheckpointNV");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_QUEUE_CHECKPOINT_DATA_NV: *const std::os::raw::c_char = crate::cstr!("vkGetQueueCheckpointDataNV");
#[doc = "Provided by [`crate::extensions::nv_device_diagnostic_checkpoints`]"]
impl crate::vk1_0::StructureType {
    pub const CHECKPOINT_DATA_NV: Self = Self(1000206000);
    pub const QUEUE_FAMILY_CHECKPOINT_PROPERTIES_NV: Self = Self(1000206001);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetCheckpointNV.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetCheckpointNV = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, p_checkpoint_marker: *const std::ffi::c_void) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetQueueCheckpointDataNV.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetQueueCheckpointDataNV = unsafe extern "system" fn(queue: crate::vk1_0::Queue, p_checkpoint_data_count: *mut u32, p_checkpoint_data: *mut crate::extensions::nv_device_diagnostic_checkpoints::CheckpointDataNV) -> ();
impl<'a> crate::ExtendableFromMut<'a, QueueFamilyCheckpointPropertiesNV> for crate::vk1_1::QueueFamilyProperties2Builder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, QueueFamilyCheckpointPropertiesNVBuilder<'_>> for crate::vk1_1::QueueFamilyProperties2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueueFamilyCheckpointPropertiesNV.html) · Structure"]
#[doc(alias = "VkQueueFamilyCheckpointPropertiesNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct QueueFamilyCheckpointPropertiesNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub checkpoint_execution_stage_mask: crate::vk1_0::PipelineStageFlags,
}
impl Default for QueueFamilyCheckpointPropertiesNV {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::QUEUE_FAMILY_CHECKPOINT_PROPERTIES_NV, p_next: std::ptr::null_mut(), checkpoint_execution_stage_mask: Default::default() }
    }
}
impl std::fmt::Debug for QueueFamilyCheckpointPropertiesNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("QueueFamilyCheckpointPropertiesNV").field("s_type", &self.s_type).field("p_next", &self.p_next).field("checkpoint_execution_stage_mask", &self.checkpoint_execution_stage_mask).finish()
    }
}
impl QueueFamilyCheckpointPropertiesNV {
    #[inline]
    pub fn into_builder<'a>(self) -> QueueFamilyCheckpointPropertiesNVBuilder<'a> {
        QueueFamilyCheckpointPropertiesNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueueFamilyCheckpointPropertiesNV.html) · Builder of [`QueueFamilyCheckpointPropertiesNV`]"]
#[repr(transparent)]
pub struct QueueFamilyCheckpointPropertiesNVBuilder<'a>(QueueFamilyCheckpointPropertiesNV, std::marker::PhantomData<&'a ()>);
impl<'a> QueueFamilyCheckpointPropertiesNVBuilder<'a> {
    #[inline]
    pub fn new() -> QueueFamilyCheckpointPropertiesNVBuilder<'a> {
        QueueFamilyCheckpointPropertiesNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn checkpoint_execution_stage_mask(mut self, checkpoint_execution_stage_mask: crate::vk1_0::PipelineStageFlags) -> Self {
        self.0.checkpoint_execution_stage_mask = checkpoint_execution_stage_mask as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> QueueFamilyCheckpointPropertiesNV {
        self.0
    }
}
impl<'a> std::default::Default for QueueFamilyCheckpointPropertiesNVBuilder<'a> {
    fn default() -> QueueFamilyCheckpointPropertiesNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for QueueFamilyCheckpointPropertiesNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for QueueFamilyCheckpointPropertiesNVBuilder<'a> {
    type Target = QueueFamilyCheckpointPropertiesNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for QueueFamilyCheckpointPropertiesNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCheckpointDataNV.html) · Structure"]
#[doc(alias = "VkCheckpointDataNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CheckpointDataNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub stage: crate::vk1_0::PipelineStageFlagBits,
    pub p_checkpoint_marker: *mut std::ffi::c_void,
}
impl Default for CheckpointDataNV {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::CHECKPOINT_DATA_NV, p_next: std::ptr::null_mut(), stage: Default::default(), p_checkpoint_marker: std::ptr::null_mut() }
    }
}
impl std::fmt::Debug for CheckpointDataNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CheckpointDataNV").field("s_type", &self.s_type).field("p_next", &self.p_next).field("stage", &self.stage).field("p_checkpoint_marker", &self.p_checkpoint_marker).finish()
    }
}
impl CheckpointDataNV {
    #[inline]
    pub fn into_builder<'a>(self) -> CheckpointDataNVBuilder<'a> {
        CheckpointDataNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCheckpointDataNV.html) · Builder of [`CheckpointDataNV`]"]
#[repr(transparent)]
pub struct CheckpointDataNVBuilder<'a>(CheckpointDataNV, std::marker::PhantomData<&'a ()>);
impl<'a> CheckpointDataNVBuilder<'a> {
    #[inline]
    pub fn new() -> CheckpointDataNVBuilder<'a> {
        CheckpointDataNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn stage(mut self, stage: crate::vk1_0::PipelineStageFlagBits) -> Self {
        self.0.stage = stage as _;
        self
    }
    #[inline]
    pub fn checkpoint_marker(mut self, checkpoint_marker: *mut std::ffi::c_void) -> Self {
        self.0.p_checkpoint_marker = checkpoint_marker;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> CheckpointDataNV {
        self.0
    }
}
impl<'a> std::default::Default for CheckpointDataNVBuilder<'a> {
    fn default() -> CheckpointDataNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for CheckpointDataNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for CheckpointDataNVBuilder<'a> {
    type Target = CheckpointDataNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for CheckpointDataNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::nv_device_diagnostic_checkpoints`]"]
impl crate::DeviceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetCheckpointNV.html) · Function"]
    #[doc(alias = "vkCmdSetCheckpointNV")]
    pub unsafe fn cmd_set_checkpoint_nv(&self, command_buffer: crate::vk1_0::CommandBuffer, checkpoint_marker: *const std::ffi::c_void) -> () {
        let _function = self.cmd_set_checkpoint_nv.expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(command_buffer as _, checkpoint_marker);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetQueueCheckpointDataNV.html) · Function"]
    #[doc(alias = "vkGetQueueCheckpointDataNV")]
    pub unsafe fn get_queue_checkpoint_data_nv(&self, queue: crate::vk1_0::Queue, checkpoint_data_count: Option<u32>) -> Vec<crate::extensions::nv_device_diagnostic_checkpoints::CheckpointDataNV> {
        let _function = self.get_queue_checkpoint_data_nv.expect(crate::NOT_LOADED_MESSAGE);
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
