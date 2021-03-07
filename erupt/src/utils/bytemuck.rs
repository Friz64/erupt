use crate::vk;
use bytemuck::{Pod, Zeroable};

macro_rules! impl_pod {
    ($struct:ty) => {
        unsafe impl Pod for $struct {}
        unsafe impl Zeroable for $struct {}
    };
}

// vk1_0
impl_pod!(vk::DrawIndirectCommand);
impl_pod!(vk::DrawIndexedIndirectCommand);
impl_pod!(vk::DispatchIndirectCommand);

// nv_device_generated_commands
impl_pod!(vk::BindIndexBufferIndirectCommandNV);
impl_pod!(vk::BindShaderGroupIndirectCommandNV);
impl_pod!(vk::BindVertexBufferIndirectCommandNV);

// khr_ray_tracing_pipeline
impl_pod!(vk::TraceRaysIndirectCommandKHR);

// khr_acceleration_structure
// Disabled due to issue #10 (https://gitlab.com/Friz64/erupt/-/issues/10)
//impl_pod!(vk::AccelerationStructureInstanceKHR);
