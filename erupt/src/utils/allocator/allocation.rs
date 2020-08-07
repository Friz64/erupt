use super::Region;
use crate::{try_vk, utils::VulkanResult, vk1_0, DeviceLoader};
use std::{
    ffi::c_void,
    ops::{Bound, RangeBounds},
    ptr, slice,
};

/// A region of mapped memory
#[derive(Debug)]
pub struct MappedMemory {
    ptr: *mut c_void,
    host_coherent: bool,
    memory_range: vk1_0::MappedMemoryRange,
}

impl MappedMemory {
    /// Maps specified `range` on `memory`, which has region `region`
    ///
    /// Note: This function is usually called by `Allocation::map`
    pub fn map(
        device: &DeviceLoader,
        memory: vk1_0::DeviceMemory,
        region: &Region,
        host_coherent: bool,
        range: impl RangeBounds<vk1_0::DeviceSize>,
    ) -> VulkanResult<MappedMemory> {
        let start = match range.start_bound() {
            Bound::Excluded(start) => start + 1,
            Bound::Included(start) => *start,
            Bound::Unbounded => 0,
        };

        let end = match range.end_bound() {
            Bound::Included(end) => end + 1,
            Bound::Excluded(end) => *end,
            Bound::Unbounded => region.size(),
        };

        if !host_coherent && (start != 0 || end != region.size()) {
            panic!("Partial mapping on non host coherent memory is not supported");
        }

        let size = end - start;
        assert!(size > 0);

        let offset = start + region.start;
        let mut ptr = ptr::null_mut();
        try_vk!(unsafe { device.map_memory(memory, offset, size, None, &mut ptr) });

        let memory_range = vk1_0::MappedMemoryRange {
            memory,
            offset,
            size,
            ..Default::default()
        };

        let mapped = MappedMemory {
            ptr,
            host_coherent,
            memory_range,
        };

        try_vk!(mapped.invalidate(device));
        VulkanResult::new_ok(mapped)
    }

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
                device.invalidate_mapped_memory_ranges(&[self.memory_range.into_builder()])
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
            try_vk!(unsafe {
                device.flush_mapped_memory_ranges(&[self.memory_range.into_builder()])
            });
        }

        VulkanResult::new_ok(())
    }

    /// Unmap the memory
    #[inline]
    pub fn unmap(self, device: &DeviceLoader) -> VulkanResult<()> {
        try_vk!(self.flush(device));
        unsafe { device.unmap_memory(self.memory_range.memory) };
        VulkanResult::new_ok(())
    }
}
