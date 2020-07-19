use super::Region;
use crate::vk1_0;

/// Simple suballocator
#[derive(Debug)]
pub struct Suballocator {
    free_regions: Vec<Region>,
    size: vk1_0::DeviceSize,
    align: Option<vk1_0::DeviceSize>,
}

impl Suballocator {
    /// Creates a new Suballocator with the maximum size of `size` and optional alignment of `align`
    pub fn new(size: vk1_0::DeviceSize, align: Option<vk1_0::DeviceSize>) -> Suballocator {
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
    ///
    /// The `memory_type_bits` of `mem_requirements` is ignored
    pub fn allocate(&mut self, mut mem_requirements: vk1_0::MemoryRequirements) -> Option<Region> {
        if let Some(align) = self.align {
            assert!(mem_requirements.alignment.is_power_of_two());
            mem_requirements.size = super::align_up(mem_requirements.size, align);
            mem_requirements.alignment = mem_requirements.alignment.max(align);
        }

        let (free_region_idx, free_region) = self
            .free_regions
            .iter()
            .enumerate()
            .find(|(_, region)| region.fits(mem_requirements))?;

        let allocation_start = super::align_up(free_region.start, mem_requirements.alignment);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suballoc() {
        let mut suballoc = Suballocator::new(128, None);
        assert!(suballoc.is_empty());

        let r1 = suballoc
            .allocate(vk1_0::MemoryRequirements {
                size: 13,
                alignment: 1,
                memory_type_bits: u32::MAX,
            })
            .unwrap();
        dbg!(&suballoc.free_regions);
        let r2 = suballoc
            .allocate(vk1_0::MemoryRequirements {
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
            .allocate(vk1_0::MemoryRequirements {
                size: 13,
                alignment: 1,
                memory_type_bits: u32::MAX,
            })
            .unwrap();
        dbg!(&suballoc.free_regions);
        let r2 = suballoc
            .allocate(vk1_0::MemoryRequirements {
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
