use crate::{utils::VulkanResult, vk1_0::*, DeviceLoader};

/// A generic allocation object
pub trait AllocationObject {
    /// Returns this objects `MemoryRequirements`
    unsafe fn memory_requirements(&self, device: &DeviceLoader) -> MemoryRequirements;

    /// Binds `memory` at `offset` to this object
    unsafe fn bind_memory(
        &self,
        device: &DeviceLoader,
        memory: DeviceMemory,
        offset: DeviceSize,
    ) -> VulkanResult<()>;

    /// Destroys this object
    unsafe fn destroy(&self, device: &DeviceLoader);
}

impl AllocationObject for Buffer {
    #[inline]
    unsafe fn memory_requirements(&self, device: &DeviceLoader) -> MemoryRequirements {
        device.get_buffer_memory_requirements(*self, None)
    }

    #[inline]
    unsafe fn bind_memory(
        &self,
        device: &DeviceLoader,
        memory: DeviceMemory,
        offset: DeviceSize,
    ) -> VulkanResult<()> {
        device.bind_buffer_memory(*self, memory, offset)
    }

    #[inline]
    unsafe fn destroy(&self, device: &DeviceLoader) {
        device.destroy_buffer(*self, None)
    }
}

impl AllocationObject for Image {
    #[inline]
    unsafe fn memory_requirements(&self, device: &DeviceLoader) -> MemoryRequirements {
        device.get_image_memory_requirements(*self, None)
    }

    #[inline]
    unsafe fn bind_memory(
        &self,
        device: &DeviceLoader,
        memory: DeviceMemory,
        offset: DeviceSize,
    ) -> VulkanResult<()> {
        device.bind_image_memory(*self, memory, offset)
    }

    #[inline]
    unsafe fn destroy(&self, device: &DeviceLoader) {
        device.destroy_image(*self, None)
    }
}
