use super::{object::AllocationObject, suballocator::Suballocator, Region};
use crate::{try_vk, utils::VulkanResult, vk1_0::*, DeviceLoader};
use std::any::{Any, TypeId};

/// A block of Vulkan memory
#[derive(Debug)]
pub struct Block {
    mem_type_idx: u32,
    memory: DeviceMemory,
    suballocator: Suballocator,
    host_coherent: bool,
    object_type: TypeId,
}

impl Block {
    /// Create a new block, allocating `size` bytes on `mem_type_idx` for object `T`
    pub fn new<T>(
        device: &DeviceLoader,
        size: DeviceSize,
        mem_type_idx: u32,
        mem_properties: &PhysicalDeviceMemoryProperties,
        limits: &PhysicalDeviceLimits,
    ) -> VulkanResult<Block>
    where
        T: AllocationObject + Any,
    {
        let allocate_info = MemoryAllocateInfoBuilder::new()
            .allocation_size(size)
            .memory_type_index(mem_type_idx);
        let memory = try_vk!(unsafe { device.allocate_memory(&allocate_info, None, None) });

        let host_coherent = mem_properties.memory_types[mem_type_idx as usize]
            .property_flags
            .contains(MemoryPropertyFlagBits::HOST_COHERENT.bitmask());

        let suballocator = Suballocator::new(
            size,
            if host_coherent {
                None
            } else {
                Some(limits.non_coherent_atom_size)
            },
        );

        VulkanResult::new_ok(Block {
            mem_type_idx,
            memory,
            suballocator,
            host_coherent,
            object_type: TypeId::of::<T>(),
        })
    }

    /// Makes a new allocation, returning the region of the new allocation if it was successful
    #[inline]
    pub fn allocate(&mut self, mem_requirements: MemoryRequirements) -> Option<Region> {
        self.suballocator.allocate(mem_requirements)
    }

    /// Frees an allocation, destroying this block if it's now empty
    #[inline]
    pub fn free(this: &mut Option<Self>, device: &DeviceLoader, allocation: Region) {
        if let Some(inner) = this {
            inner.suballocator.free(allocation);

            if inner.suballocator.is_empty() {
                unsafe { device.free_memory(inner.memory, None) };
                this.take();
            }
        }
    }

    /// Returns the inner `DeviceMemory` handle
    #[inline]
    pub fn memory(&self) -> DeviceMemory {
        self.memory
    }

    /// Returns true if this block corresponds to the provided parameters
    #[inline]
    pub fn corresponds<T>(&self, mem_type_idx: u32) -> bool
    where
        T: AllocationObject + Any,
    {
        self.mem_type_idx == mem_type_idx && self.object_type == TypeId::of::<T>()
    }

    /// Returns true if this block is allocated on host coherent memory
    #[inline]
    pub fn host_coherent(&self) -> bool {
        self.host_coherent
    }
}
