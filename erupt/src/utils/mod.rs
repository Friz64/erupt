#![warn(missing_docs)]

//! Utilities to aid your usage of this crate.

#[cfg(feature = "bytemuck")]
mod bytemuck;
mod decode_spv;
pub mod features;
#[cfg(feature = "loading")]
pub mod loading;
#[cfg(feature = "surface")]
pub mod surface;
mod vulkan_result;

pub use decode_spv::*;
use std::iter;
pub use vulkan_result::*;

/// Returns an iterator over all nodes in the given pointer chain.
///
/// ## Safety
/// Assumes all `p_next` pointers in the pointer chain are valid.
pub unsafe fn iterate_ptr_chain(
    mut node: *mut crate::vk1_0::BaseOutStructure,
) -> impl Iterator<Item = *mut crate::vk1_0::BaseOutStructure> {
    iter::from_fn(move || {
        if node.is_null() {
            None
        } else {
            let current_item = node;
            node = (*node).p_next;
            Some(current_item)
        }
    })
}
