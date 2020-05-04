use crate::vk1_0::*;

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
