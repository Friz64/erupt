//! Provides a basic Vulkan memory allocator, aiming to be *correct*
//!
//! ```rust ignore
//! let mut allocator = try_vk!(Allocator::new(
//!     &instance,
//!     physical_device,
//!     AllocatorCreateInfo::default()
//! ));
//!
//! let buffer = try_vk!(device.create_buffer(...));
//! let allocation = try_vk!(allocator.allocate(&device, buffer, MemoryTypeFinder::dynamic()));
//!
//! let mut map = try_vk!(buffer.map(&device, ..));
//! map.import(&[1, 2, 3]);
//! try_vk!(map.unmap(&device));
//!
//! allocator.free(&device, allocation);
//! ```
//!
//! # Note
//! This allocator has not been tested on a larger scale yet, there may be some bugs.
//!
//! If you encounter a bug, please report it on the [bug tracker](https://gitlab.com/Friz64/erupt/-/issues).
mod block;
mod mapped_memory;
mod memory_type;
mod object;
mod suballocator;

pub use block::*;
pub use mapped_memory::*;
pub use memory_type::*;
pub use object::*;
pub use suballocator::*;

use crate::{try_vk, utils::VulkanResult, vk1_0, DeviceLoader, InstanceLoader};
use std::{any::Any, ops::RangeBounds};

/// Align `addr` to `align` upwards
///
/// ```text
/// addr: 0, align: 16 -> 0
/// addr: 1, align: 16 -> 16
/// addr: 16, align: 16 -> 16
/// addr: 17, align: 16 -> 32
/// addr: 21, align: 16 -> 32
/// ```
#[inline]
pub fn align_up(addr: vk1_0::DeviceSize, align: vk1_0::DeviceSize) -> vk1_0::DeviceSize {
    (addr + align - 1) / align * align
}

/// A region of memory
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Region {
    /// Region start, inclusive
    pub start: vk1_0::DeviceSize,
    /// Region end, exclusive
    pub end: vk1_0::DeviceSize,
}

impl Region {
    /// Calculates the size of the region
    #[inline]
    pub fn size(&self) -> vk1_0::DeviceSize {
        self.end - self.start
    }

    /// Returns `true` if this region has a size of 0
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    /// Returns `true` if this region fits `mem_requirements`
    #[inline]
    pub fn fits(&self, mem_requirements: vk1_0::MemoryRequirements) -> bool {
        self.end - align_up(self.start, mem_requirements.alignment) >= mem_requirements.size
    }
}

/// A handle to an allocation, holding an inner allocation object
#[derive(Debug)]
pub struct Allocation<T> {
    object: T,
    block_idx: usize,
    region: Region,
    memory: vk1_0::DeviceMemory,
    host_coherent: bool,
}

impl<T> Allocation<T>
where
    T: AllocationObject,
{
    /// Returns the inner allocation object
    #[inline]
    pub fn object(&self) -> &T {
        &self.object
    }

    /// Returns the inner `DeviceMemory` handle where this is allocated
    #[inline]
    pub fn memory(&self) -> vk1_0::DeviceMemory {
        self.memory
    }

    /// Returns the allocated region of memory
    #[inline]
    pub fn region(&self) -> Region {
        self.region
    }

    /// Maps the specified range of memory
    #[inline]
    pub fn map(
        &self,
        device: &DeviceLoader,
        range: impl RangeBounds<vk1_0::DeviceSize>,
    ) -> VulkanResult<MappedMemory> {
        MappedMemory::map(device, self.memory, &self.region, self.host_coherent, range)
    }
}

/// Information used to create an `Allocator`
#[derive(Debug)]
pub struct AllocatorCreateInfo {
    /// Size of every allocation block (Default: 32 MiB)
    pub block_size: vk1_0::DeviceSize,
}

impl Default for AllocatorCreateInfo {
    #[inline]
    fn default() -> Self {
        AllocatorCreateInfo {
            block_size: 32 * 1024u64.pow(2),
        }
    }
}

/// A simple Vulkan memory allocator
#[derive(Debug)]
pub struct Allocator {
    info: AllocatorCreateInfo,
    dev_properties: vk1_0::PhysicalDeviceProperties,
    mem_properties: vk1_0::PhysicalDeviceMemoryProperties,
    blocks: Vec<Option<Block>>,
}

impl Allocator {
    /// Create a new memory allocator
    pub fn new(
        instance: &InstanceLoader,
        physical_device: vk1_0::PhysicalDevice,
        info: AllocatorCreateInfo,
    ) -> VulkanResult<Allocator> {
        let dev_properties =
            unsafe { instance.get_physical_device_properties(physical_device, None) };
        let mem_properties =
            unsafe { instance.get_physical_device_memory_properties(physical_device, None) };

        VulkanResult::new_ok(Allocator {
            info,
            dev_properties,
            mem_properties,
            blocks: Vec::new(),
        })
    }

    /// Allocate memory for `object`
    pub fn allocate<T>(
        &mut self,
        device: &DeviceLoader,
        object: T,
        finder: MemoryTypeFinder,
    ) -> VulkanResult<Allocation<T>>
    where
        T: AllocationObject + Any,
    {
        let mem_requirements = unsafe { object.memory_requirements(device) };
        let mem_type_idx = match finder.find(&self.mem_properties, &mem_requirements) {
            Some(mem_type_idx) => mem_type_idx,
            None => return VulkanResult::new_err(vk1_0::Result::ERROR_UNKNOWN),
        };

        let (block_idx, block, region) = match self
            .blocks
            .iter_mut()
            .enumerate()
            .filter_map(|(i, block)| block.as_mut().map(|block| (i, block)))
            .filter(|(_, block)| block.corresponds::<T>(mem_type_idx))
            .find_map(|(i, block)| block.allocate(mem_requirements).map(|r| (i, block, r)))
        {
            Some(block) => block,
            None => {
                let idx = self.blocks.len();
                self.blocks.push(Some(try_vk!(Block::new::<T>(
                    device,
                    self.info.block_size,
                    mem_type_idx,
                    &self.mem_properties,
                    &self.dev_properties.limits
                ))));

                let block = self.blocks[idx].as_mut().unwrap();
                if let Some(plan) = block.allocate(mem_requirements) {
                    (idx, block, plan)
                } else {
                    return VulkanResult::new_err(vk1_0::Result::ERROR_OUT_OF_DEVICE_MEMORY);
                }
            }
        };

        try_vk!(unsafe { object.bind_memory(device, block.memory(), region.start) });

        VulkanResult::new_ok(Allocation {
            object,
            block_idx,
            region,
            memory: block.memory(),
            host_coherent: block.host_coherent(),
        })
    }

    /// Free an allocation handle, destroying it's inner object
    pub fn free<T>(&mut self, device: &DeviceLoader, allocation: Allocation<T>)
    where
        T: AllocationObject,
    {
        Block::free(
            &mut self.blocks[allocation.block_idx],
            device,
            allocation.region(),
        );

        unsafe { allocation.object().destroy(device) };
    }
}
