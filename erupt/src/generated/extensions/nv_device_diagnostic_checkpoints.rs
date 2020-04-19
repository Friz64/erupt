# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_NV_device_diagnostic_checkpoints.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NV_DEVICE_DIAGNOSTIC_CHECKPOINTS_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NV_DEVICE_DIAGNOSTIC_CHECKPOINTS_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_NV_device_diagnostic_checkpoints");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetCheckpointNV.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetCheckpointNV = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    p_checkpoint_marker: *const std::ffi::c_void,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetQueueCheckpointDataNV.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetQueueCheckpointDataNV = unsafe extern "system" fn(
    queue: crate::vk1_0::Queue,
    p_checkpoint_data_count: *mut u32,
    p_checkpoint_data: *mut crate::extensions::nv_device_diagnostic_checkpoints::CheckpointDataNV,
) -> std::ffi::c_void;
#[doc = "Provides Device Commands for [`NvDeviceDiagnosticCheckpointsDeviceLoaderExt`](trait.NvDeviceDiagnosticCheckpointsDeviceLoaderExt.html)"]
pub struct NvDeviceDiagnosticCheckpointsDeviceCommands {
    pub cmd_set_checkpoint_nv: PFN_vkCmdSetCheckpointNV,
    pub get_queue_checkpoint_data_nv: PFN_vkGetQueueCheckpointDataNV,
}
impl NvDeviceDiagnosticCheckpointsDeviceCommands {
    #[inline]
    pub fn load(
        loader: &crate::DeviceLoader,
    ) -> Option<NvDeviceDiagnosticCheckpointsDeviceCommands> {
        unsafe {
            Some(NvDeviceDiagnosticCheckpointsDeviceCommands {
                cmd_set_checkpoint_nv: std::mem::transmute(loader.symbol("vkCmdSetCheckpointNV")?),
                get_queue_checkpoint_data_nv: std::mem::transmute(
                    loader.symbol("vkGetQueueCheckpointDataNV")?,
                ),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`NvDeviceDiagnosticCheckpointsDeviceCommands`](struct.NvDeviceDiagnosticCheckpointsDeviceCommands.html)"]
pub trait NvDeviceDiagnosticCheckpointsDeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetCheckpointNV.html) · Device Command"]
    unsafe fn cmd_set_checkpoint_nv(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        checkpoint_marker: *const std::ffi::c_void,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetQueueCheckpointDataNV.html) · Device Command"]
    unsafe fn get_queue_checkpoint_data_nv(
        &self,
        queue: crate::vk1_0::Queue,
        checkpoint_data_count: Option<u32>,
    ) -> Vec<crate::extensions::nv_device_diagnostic_checkpoints::CheckpointDataNV>;
}
impl NvDeviceDiagnosticCheckpointsDeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetCheckpointNV.html) · Device Command"]
    unsafe fn cmd_set_checkpoint_nv(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        checkpoint_marker: *const std::ffi::c_void,
    ) -> () {
        let function = self
            .nv_device_diagnostic_checkpoints
            .as_ref()
            .expect("`nv_device_diagnostic_checkpoints` not loaded")
            .cmd_set_checkpoint_nv;
        let _val = function(command_buffer, checkpoint_marker);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetQueueCheckpointDataNV.html) · Device Command"]
    unsafe fn get_queue_checkpoint_data_nv(
        &self,
        queue: crate::vk1_0::Queue,
        checkpoint_data_count: Option<u32>,
    ) -> Vec<crate::extensions::nv_device_diagnostic_checkpoints::CheckpointDataNV> {
        let function = self
            .nv_device_diagnostic_checkpoints
            .as_ref()
            .expect("`nv_device_diagnostic_checkpoints` not loaded")
            .get_queue_checkpoint_data_nv;
        let mut checkpoint_data_count = checkpoint_data_count.unwrap_or_else(|| {
            let mut val = Default::default();
            function(queue, &mut val, std::ptr::null_mut());
            val
        });
        let mut checkpoint_data = vec![Default::default(); checkpoint_data_count as _];
        let _val = function(
            queue,
            &mut checkpoint_data_count,
            checkpoint_data.as_mut_ptr(),
        );
        checkpoint_data
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCheckpointDataNV.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CheckpointDataNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub stage: crate::vk1_0::PipelineStageFlagBits,
    pub p_checkpoint_marker: *mut std::ffi::c_void,
}
impl CheckpointDataNV {
    #[inline]
    pub fn builder<'a>(self) -> CheckpointDataNVBuilder<'a> {
        CheckpointDataNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for CheckpointDataNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("CheckpointDataNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("stage", &self.stage)
            .field("p_checkpoint_marker", &self.p_checkpoint_marker)
            .finish()
    }
}
impl Default for CheckpointDataNV {
    fn default() -> CheckpointDataNV {
        CheckpointDataNV {
            s_type: crate::vk1_0::StructureType::CHECKPOINT_DATA_NV,
            p_next: std::ptr::null_mut(),
            stage: Default::default(),
            p_checkpoint_marker: std::ptr::null_mut(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`CheckpointDataNV`](struct.CheckpointDataNV.html)"]
#[repr(transparent)]
pub struct CheckpointDataNVBuilder<'a>(CheckpointDataNV, std::marker::PhantomData<&'a ()>);
impl<'a> CheckpointDataNVBuilder<'a> {
    #[inline]
    pub fn new() -> CheckpointDataNVBuilder<'a> {
        CheckpointDataNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn stage(mut self, stage: crate::vk1_0::PipelineStageFlagBits) -> Self {
        self.0.stage = stage;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn checkpoint_marker(mut self, checkpoint_marker: &'a mut std::ffi::c_void) -> Self {
        self.0.p_checkpoint_marker = checkpoint_marker;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> CheckpointDataNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for CheckpointDataNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueueFamilyCheckpointPropertiesNV.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct QueueFamilyCheckpointPropertiesNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub checkpoint_execution_stage_mask: crate::vk1_0::PipelineStageFlags,
}
impl QueueFamilyCheckpointPropertiesNV {
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
    pub fn builder<'a>(self) -> QueueFamilyCheckpointPropertiesNVBuilder<'a> {
        QueueFamilyCheckpointPropertiesNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for QueueFamilyCheckpointPropertiesNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("QueueFamilyCheckpointPropertiesNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "checkpoint_execution_stage_mask",
                &self.checkpoint_execution_stage_mask,
            )
            .finish()
    }
}
impl Default for QueueFamilyCheckpointPropertiesNV {
    fn default() -> QueueFamilyCheckpointPropertiesNV {
        QueueFamilyCheckpointPropertiesNV {
            s_type: crate::vk1_0::StructureType::QUEUE_FAMILY_CHECKPOINT_PROPERTIES_NV,
            p_next: std::ptr::null_mut(),
            checkpoint_execution_stage_mask: Default::default(),
        }
    }
}
impl crate::ExtendableBy<QueueFamilyCheckpointPropertiesNV>
    for crate::vk1_1::QueueFamilyProperties2
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`QueueFamilyCheckpointPropertiesNV`](struct.QueueFamilyCheckpointPropertiesNV.html)"]
#[repr(transparent)]
pub struct QueueFamilyCheckpointPropertiesNVBuilder<'a>(
    QueueFamilyCheckpointPropertiesNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> QueueFamilyCheckpointPropertiesNVBuilder<'a> {
    #[inline]
    pub fn new() -> QueueFamilyCheckpointPropertiesNVBuilder<'a> {
        QueueFamilyCheckpointPropertiesNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn checkpoint_execution_stage_mask(
        mut self,
        checkpoint_execution_stage_mask: crate::vk1_0::PipelineStageFlags,
    ) -> Self {
        self.0.checkpoint_execution_stage_mask = checkpoint_execution_stage_mask;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> QueueFamilyCheckpointPropertiesNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for QueueFamilyCheckpointPropertiesNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
