//! Provides a basic Vulkan memory allocator (+parts), aiming to be *correct*
use crate::{
    try_vk,
    utils::VulkanResult,
    vk1_0::{self, *},
    DeviceLoader, InstanceLoader,
};
use std::{
    ffi::c_void,
    ops::{Bound, RangeBounds},
    ptr, slice,
};

/// Finds the wanted memory type
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MemoryTypeFinder<'a> {
    /// A list of memory properties and their associated impact
    pub impacts: &'a [(MemoryPropertyFlagBits, i32)],
}

impl<'a> MemoryTypeFinder<'a> {
    /// Fast access on the GPU, probably not mappable
    #[inline]
    pub fn gpu_only() -> Self {
        MemoryTypeFinder {
            impacts: &[
                (MemoryPropertyFlagBits::DEVICE_LOCAL, 10),
                (MemoryPropertyFlagBits::HOST_VISIBLE, -10),
            ],
        }
    }

    /// Designed for staging uploads to the GPU
    #[inline]
    pub fn upload() -> Self {
        MemoryTypeFinder {
            impacts: &[
                (MemoryPropertyFlagBits::HOST_VISIBLE, 10),
                (MemoryPropertyFlagBits::HOST_COHERENT, 7),
                (MemoryPropertyFlagBits::HOST_CACHED, -5),
                (MemoryPropertyFlagBits::DEVICE_LOCAL, -1),
            ],
        }
    }

    /// Designed for downloading from the GPU
    #[inline]
    pub fn download() -> Self {
        MemoryTypeFinder {
            impacts: &[
                (MemoryPropertyFlagBits::HOST_VISIBLE, 10),
                (MemoryPropertyFlagBits::HOST_COHERENT, 7),
                (MemoryPropertyFlagBits::HOST_CACHED, 5),
            ],
        }
    }

    /// Designed for dynamic write-once read-once resources
    #[inline]
    pub fn dynamic() -> Self {
        MemoryTypeFinder {
            impacts: &[
                (MemoryPropertyFlagBits::HOST_VISIBLE, 10),
                (MemoryPropertyFlagBits::DEVICE_LOCAL, 7),
                (MemoryPropertyFlagBits::HOST_COHERENT, 5),
                (MemoryPropertyFlagBits::HOST_CACHED, 1),
            ],
        }
    }

    /// Finds the memory type with the biggest impact sum
    pub fn find(
        self,
        mem_properties: &PhysicalDeviceMemoryProperties,
        mem_requirements: &MemoryRequirements,
    ) -> Option<u32> {
        mem_properties
            .memory_types
            .iter()
            .enumerate()
            .take(mem_properties.memory_type_count as usize)
            .filter(|(i, _)| (mem_requirements.memory_type_bits & (1 << i)) != 0)
            .max_by_key(|(_, ty)| -> i32 {
                self.impacts
                    .iter()
                    .filter(|(bits, _)| ty.property_flags.contains(bits.bitmask()))
                    .map(|(_, impact)| impact)
                    .sum()
            })
            .map(|(i, _)| i as u32)
    }
}

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
pub fn align_up(addr: DeviceSize, align: DeviceSize) -> DeviceSize {
    (addr + align - 1) / align * align
}

/// A region of memory
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Region {
    /// Region start, inclusive
    pub start: DeviceSize,
    /// Region end, exclusive
    pub end: DeviceSize,
}

impl Region {
    /// Calculates the size of the region
    #[inline]
    pub fn size(&self) -> DeviceSize {
        self.end - self.start
    }

    /// Returns `true` if this region has a size of 0
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    /// Returns `true` if this region fits `mem_requirements`
    #[inline]
    pub fn fits(&self, mem_requirements: MemoryRequirements) -> bool {
        self.end - align_up(self.start, mem_requirements.alignment) >= mem_requirements.size
    }
}

/// Simple suballocator
#[derive(Debug)]
pub struct Suballocator {
    free_regions: Vec<Region>,
    size: DeviceSize,
    align: Option<DeviceSize>,
}

impl Suballocator {
    /// Creates a new Suballocator with the maximum size of `size` and optional alignment of `align`
    pub fn new(size: DeviceSize, align: Option<DeviceSize>) -> Suballocator {
        if let Some(align) = align {
            assert!(align.is_power_of_two());
        }

        Suballocator {
            free_regions: vec![Region {
                start: 0,
                end: size,
            }],
            size,
            align,
        }
    }

    /// Makes a new allocation, returning the region of the new allocation if it was successful
    pub fn allocate(&mut self, mut mem_requirements: MemoryRequirements) -> Option<Region> {
        if let Some(align) = self.align {
            assert!(mem_requirements.alignment.is_power_of_two());
            mem_requirements.size = align_up(mem_requirements.size, align);
            mem_requirements.alignment = mem_requirements.alignment.max(align);
        }

        let (free_region_idx, free_region) = self
            .free_regions
            .iter()
            .enumerate()
            .find(|(_, region)| region.fits(mem_requirements))?;

        let allocation_start = align_up(free_region.start, mem_requirements.alignment);
        let allocation = Region {
            start: allocation_start,
            end: allocation_start + mem_requirements.size,
        };

        let left = Region {
            start: free_region.start,
            end: allocation.start,
        };

        let right = Region {
            start: allocation.end,
            end: free_region.end,
        };

        self.free_regions.swap_remove(free_region_idx);
        if !left.is_empty() {
            self.free_regions.push(left);
        }

        if !right.is_empty() {
            self.free_regions.push(right);
        }

        Some(allocation)
    }

    #[inline]
    /// Returns `true` if there are no active allocations
    pub fn is_empty(&self) -> bool {
        self.free_regions.len() == 1 && {
            let region = &self.free_regions[0];
            region.start == 0 && region.end == self.size
        }
    }

    /// Frees an allocation
    pub fn free(&mut self, allocation: Region) {
        let left = self
            .free_regions
            .iter()
            .enumerate()
            .find(|(_, region)| region.end == allocation.start)
            .map(|(i, _)| i);

        let right = self
            .free_regions
            .iter()
            .enumerate()
            .find(|(_, region)| region.start == allocation.end)
            .map(|(i, _)| i);

        match (left, right) {
            (Some(left), Some(right)) => {
                self.free_regions[left].end = self.free_regions[right].end;
                self.free_regions.remove(right);
            }
            (Some(left), None) => self.free_regions[left].end = allocation.end,
            (None, Some(right)) => self.free_regions[right].start = allocation.start,
            (None, None) => self.free_regions.push(allocation),
        }
    }
}

/// A block of Vulkan memory
#[derive(Debug)]
pub struct Block {
    mem_type_idx: u32,
    memory: DeviceMemory,
    suballocator: Suballocator,
    host_coherent: bool,
}

impl Block {
    /// Create a new block, allocating `size` bytes on `mem_type_idx`
    pub fn new(
        device: &DeviceLoader,
        size: DeviceSize,
        mem_type_idx: u32,
        mem_properties: &PhysicalDeviceMemoryProperties,
        limits: &PhysicalDeviceLimits,
    ) -> VulkanResult<Block> {
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
        })
    }

    /// Returns the inner `DeviceMemory` handle
    #[inline]
    pub fn memory(&self) -> DeviceMemory {
        self.memory
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
}

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

/// A region of mapped memory
#[derive(Debug)]
pub struct MappedMemory {
    ptr: *mut c_void,
    memory: DeviceMemory,
    host_coherent: bool,
    memory_range: MappedMemoryRange,
}

impl MappedMemory {
    /// Read the mapped memory
    #[inline]
    pub fn read(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.ptr as _, self.size()) }
    }

    /// Write to the mapped memory
    #[inline]
    pub fn write(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.ptr as _, self.size()) }
    }

    /// Copies data from a slice to the mapped memory
    ///
    /// # Panics
    /// This function will panic if `slice.len() > self.size()`
    #[inline]
    pub fn import(&mut self, slice: &[u8]) {
        let slice_len = slice.len();
        assert!(slice_len <= self.size());

        unsafe { ptr::copy_nonoverlapping(slice.as_ptr(), self.ptr as _, slice_len) }
    }

    /// Returns the raw pointer to the mapped memory
    #[inline]
    pub fn raw(&mut self) -> *mut c_void {
        self.ptr
    }

    /// Returns the size of the mapped memory
    #[inline]
    pub fn size(&self) -> usize {
        self.memory_range.size as usize
    }

    /// Invalidates host caches of this memory if necessary
    ///
    /// This is automatically called upon mapping
    #[inline]
    pub fn invalidate(&self, device: &DeviceLoader) -> VulkanResult<()> {
        if !self.host_coherent {
            try_vk!(unsafe {
                device.invalidate_mapped_memory_ranges(&[self.memory_range.builder()])
            });
        }

        VulkanResult::new_ok(())
    }

    /// Flush host writes to this memory if necessary
    ///
    /// This is automatically called upon unmapping
    #[inline]
    pub fn flush(&self, device: &DeviceLoader) -> VulkanResult<()> {
        if !self.host_coherent {
            try_vk!(unsafe { device.flush_mapped_memory_ranges(&[self.memory_range.builder()]) });
        }

        VulkanResult::new_ok(())
    }

    /// Unmap the memory
    #[inline]
    pub fn unmap(self, device: &DeviceLoader) -> VulkanResult<()> {
        try_vk!(self.flush(device));
        unsafe { device.unmap_memory(self.memory) };
        VulkanResult::new_ok(())
    }
}

/// A handle to an allocation, holding an inner allocation object
#[derive(Debug)]
pub struct Allocation<T> {
    object: T,
    block_idx: usize,
    region: Region,
    memory: DeviceMemory,
    host_coherent: bool,
}

impl<T> Allocation<T>
where
    T: AllocationObject + Clone,
{
    /// Returns the inner allocation object
    #[inline]
    pub fn object(&self) -> T {
        self.object.clone()
    }

    /// Returns the allocated region of memory
    #[inline]
    pub fn region(&self) -> Region {
        self.region
    }

    /// Maps the specified range of memory
    pub fn map(
        &self,
        device: &DeviceLoader,
        range: impl RangeBounds<DeviceSize>,
    ) -> VulkanResult<MappedMemory> {
        let start = match range.start_bound() {
            Bound::Excluded(start) => start + 1,
            Bound::Included(start) => *start,
            Bound::Unbounded => 0,
        } + self.region.start;

        let size = match range.end_bound() {
            Bound::Included(end) => end + 1,
            Bound::Excluded(end) => *end,
            Bound::Unbounded => self.region.size(),
        } - start;

        assert!(size > 0);

        if !self.host_coherent && start != self.region.start && size != self.region.size() {
            panic!("Bounded mapping on non host coherent memory is not supported");
        }

        let mut ptr = std::ptr::null_mut();
        try_vk!(unsafe {
            device.map_memory(self.memory, start, size, MemoryMapFlags::empty(), &mut ptr)
        });

        let memory_range = MappedMemoryRange {
            memory: self.memory,
            offset: start,
            size,
            ..Default::default()
        };

        let mapped = MappedMemory {
            ptr,
            memory: self.memory,
            host_coherent: self.host_coherent,
            memory_range,
        };

        try_vk!(mapped.invalidate(device));
        VulkanResult::new_ok(mapped)
    }
}

/// Information used to create an `Allocator`
#[derive(Debug)]
pub struct AllocatorCreateInfo {
    /// Size of every allocation block (Default: 64 MiB)
    pub block_size: DeviceSize,
}

impl Default for AllocatorCreateInfo {
    #[inline]
    fn default() -> Self {
        AllocatorCreateInfo {
            block_size: 64 * 1024u64.pow(2),
        }
    }
}

/// A simple Vulkan memory allocator
#[derive(Debug)]
pub struct Allocator {
    info: AllocatorCreateInfo,
    dev_properties: PhysicalDeviceProperties,
    mem_properties: PhysicalDeviceMemoryProperties,
    blocks: Vec<Option<Block>>,
}

impl Allocator {
    /// Create a new memory allocator
    pub fn new(
        instance: &InstanceLoader,
        physical_device: PhysicalDevice,
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
        T: AllocationObject,
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
            .filter(|(_, block)| block.mem_type_idx == mem_type_idx)
            .find_map(|(i, block)| block.allocate(mem_requirements).map(|r| (i, block, r)))
        {
            Some(block) => block,
            None => {
                let idx = self.blocks.len();
                self.blocks.push(Some(try_vk!(Block::new(
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

        try_vk!(unsafe { object.bind_memory(device, block.memory, region.start) });

        VulkanResult::new_ok(Allocation {
            object,
            block_idx,
            region,
            memory: block.memory,
            host_coherent: block.host_coherent,
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
            allocation.region,
        );

        unsafe { allocation.object.destroy(device) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suballoc() {
        let mut suballoc = Suballocator::new(128, None);
        assert!(suballoc.is_empty());

        let r1 = suballoc
            .allocate(MemoryRequirements {
                size: 13,
                alignment: 1,
                memory_type_bits: u32::MAX,
            })
            .unwrap();
        dbg!(&suballoc.free_regions);
        let r2 = suballoc
            .allocate(MemoryRequirements {
                size: 4,
                alignment: 16,
                memory_type_bits: u32::MAX,
            })
            .unwrap();
        dbg!(&suballoc.free_regions);

        assert!(!suballoc.is_empty());

        suballoc.free(r1);
        suballoc.free(r2);

        assert!(suballoc.is_empty());
    }

    #[test]
    fn suballoc_align() {
        let mut suballoc = Suballocator::new(128, Some(64));
        assert!(suballoc.is_empty());

        let r1 = suballoc
            .allocate(MemoryRequirements {
                size: 13,
                alignment: 1,
                memory_type_bits: u32::MAX,
            })
            .unwrap();
        dbg!(&suballoc.free_regions);
        let r2 = suballoc
            .allocate(MemoryRequirements {
                size: 4,
                alignment: 16,
                memory_type_bits: u32::MAX,
            })
            .unwrap();
        dbg!(&suballoc.free_regions);

        assert!(!suballoc.is_empty());

        suballoc.free(r1);
        suballoc.free(r2);

        assert!(suballoc.is_empty());
    }
}
