use crate::{utils::VulkanResult, vk1_0, DeviceLoader};

/// A generic allocation object
pub trait AllocationObject {
    /// Returns this objects `MemoryRequirements`
    unsafe fn memory_requirements(&self, device: &DeviceLoader) -> vk1_0::MemoryRequirements;

    /// Binds `memory` at `offset` to this object
    unsafe fn bind_memory(
        &self,
        device: &DeviceLoader,
        memory: vk1_0::DeviceMemory,
        offset: vk1_0::DeviceSize,
    ) -> VulkanResult<()>;

    /// Destroys this object
    unsafe fn destroy(&self, device: &DeviceLoader);
}

impl AllocationObject for vk1_0::Buffer {
    #[inline]
    unsafe fn memory_requirements(&self, device: &DeviceLoader) -> vk1_0::MemoryRequirements {
        device.get_buffer_memory_requirements(*self, None)
    }

    #[inline]
    unsafe fn bind_memory(
        &self,
        device: &DeviceLoader,
        memory: vk1_0::DeviceMemory,
        offset: vk1_0::DeviceSize,
    ) -> VulkanResult<()> {
        device.bind_buffer_memory(*self, memory, offset)
    }

    #[inline]
    unsafe fn destroy(&self, device: &DeviceLoader) {
        device.destroy_buffer(Some(*self), None)
    }
}

impl AllocationObject for vk1_0::Image {
    #[inline]
    unsafe fn memory_requirements(&self, device: &DeviceLoader) -> vk1_0::MemoryRequirements {
        device.get_image_memory_requirements(*self, None)
    }

    #[inline]
    unsafe fn bind_memory(
        &self,
        device: &DeviceLoader,
        memory: vk1_0::DeviceMemory,
        offset: vk1_0::DeviceSize,
    ) -> VulkanResult<()> {
        device.bind_image_memory(*self, memory, offset)
    }

    #[inline]
    unsafe fn destroy(&self, device: &DeviceLoader) {
        device.destroy_image(Some(*self), None)
    }
}
