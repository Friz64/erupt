//! Provides a basic Vulkan memory allocator
use crate::{
    try_vk,
    utils::VulkanResult,
    vk1_0::{self, *},
    DeviceLoader, InstanceLoader,
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
    pub start: DeviceSize,
    pub end: DeviceSize,
}

impl Region {
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
}

impl Suballocator {
    /// Creates a new Suballocator with the maximum size of `size`
    pub fn new(size: DeviceSize) -> Suballocator {
        Suballocator {
            free_regions: vec![Region {
                start: 0,
                end: size,
            }],
        }
    }

    /// Returns `true` if this suballocator can fit `mem_requirements`
    pub fn can_allocate(&self, mem_requirements: MemoryRequirements) -> bool {
        self.free_regions
            .iter()
            .any(|region| region.fits(mem_requirements))
    }

    /// Makes a new allocation, returning the region of the new allocation
    ///
    /// Panics if `self.can_allocate` returns `false`
    pub fn allocate(&mut self, mem_requirements: MemoryRequirements) -> Region {
        let (i, free_region) = self
            .free_regions
            .iter()
            .enumerate()
            .find(|(_, region)| region.fits(mem_requirements))
            .expect("couldn't allocate, check `can_allocate` first");

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

        self.free_regions.remove(i);
        if !left.is_empty() {
            self.free_regions.push(left);
        }

        if !right.is_empty() {
            self.free_regions.push(right);
        }

        allocation
    }

    #[inline]
    /// Returns `true` if there are no active allocations
    pub fn is_empty(&self) -> bool {
        self.free_regions.len() == 1
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
    mem_type: u32,
    memory: DeviceMemory,
    suballocator: Suballocator,
}

impl Block {
    /// Create a new block, allocating `size` bytes on `mem_type`
    pub fn new(device: &DeviceLoader, size: DeviceSize, mem_type: u32) -> VulkanResult<Block> {
        let allocate_info = MemoryAllocateInfoBuilder::new()
            .allocation_size(size)
            .memory_type_index(mem_type);
        let memory = try_vk!(unsafe { device.allocate_memory(&allocate_info, None, None) });
        let suballocator = Suballocator::new(size);

        VulkanResult::new_ok(Block {
            mem_type,
            memory,
            suballocator,
        })
    }

    /// Returns the inner `DeviceMemory` handle
    #[inline]
    pub fn memory(&self) -> DeviceMemory {
        self.memory
    }

    /// Returns `true` if this block can fit `mem_requirements`
    #[inline]
    pub fn can_allocate(&self, mem_requirements: MemoryRequirements) -> bool {
        self.suballocator.can_allocate(mem_requirements)
    }

    /// Makes a new allocation, returning the region of the new allocation
    ///
    /// Panics if `self.can_allocate` returns `false`
    #[inline]
    pub fn allocate(&mut self, mem_requirements: MemoryRequirements) -> Region {
        self.suballocator.allocate(mem_requirements)
    }

    /// Frees an allocation
    #[inline]
    pub fn free(this: &mut Option<Block>, device: &DeviceLoader, allocation: Region) {
        if let Some(inner) = this {
            inner.suballocator.free(allocation);
            if inner.suballocator.is_empty() {
                this.take().unwrap().destroy(device);
            }
        }
    }

    /// Destroys this block, freeing the allocated Vulkan memory
    #[inline]
    pub fn destroy(self, device: &DeviceLoader) {
        unsafe { device.free_memory(self.memory, None) }
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

/// A handle to an allocation, holding an inner allocation object
#[derive(Debug)]
pub struct Allocation<T> {
    object: T,
    block_idx: usize,
    region: Region,
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
        let mem_properties =
            unsafe { instance.get_physical_device_memory_properties(physical_device, None) };

        VulkanResult::new_ok(Allocator {
            info,
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
        let mem_type = match finder.find(&self.mem_properties, &mem_requirements) {
            Some(mem_type) => mem_type,
            None => return VulkanResult::new_err(vk1_0::Result::ERROR_UNKNOWN),
        };

        let (block_idx, block) = match self
            .blocks
            .iter_mut()
            .enumerate()
            .filter_map(|(i, block)| block.as_mut().map(|block| (i, block)))
            .filter(|(_, block)| block.mem_type == mem_type && block.can_allocate(mem_requirements))
            .next()
        {
            Some(block) => block,
            None => {
                let idx = self.blocks.len();
                self.blocks.push(Some(try_vk!(Block::new(
                    device,
                    self.info.block_size,
                    mem_type
                ))));

                let block = self.blocks[idx].as_mut().unwrap();
                if block.can_allocate(mem_requirements) {
                    (idx, block)
                } else {
                    return VulkanResult::new_err(vk1_0::Result::ERROR_OUT_OF_DEVICE_MEMORY);
                }
            }
        };

        let region = block.allocate(mem_requirements);
        try_vk!(unsafe { object.bind_memory(device, block.memory, region.start) });

        VulkanResult::new_ok(Allocation {
            object,
            block_idx,
            region,
        })
    }

    /// Free an allocation handle
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
