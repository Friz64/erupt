//! Utilities for working with
//! [`vk::PhysicalDeviceFeatures2`](crate::vk1_1::PhysicalDeviceFeatures2).

use crate::{vk, InstanceLoader};
use std::{
    alloc::{self, Layout},
    error::Error,
    fmt, ptr,
};

/// Returns the amount of consecutive
/// [`vk::Bool32`](crate::vk1_0::Bool32)s after the initial
/// [`vk::BaseOutStructure`](crate::vk1_0::BaseOutStructure) of
/// [`vk::PhysicalDeviceFeatures2`](crate::vk1_1::PhysicalDeviceFeatures2)
/// or any of the structs extending it.
pub fn features2_bool_count(structure_type: crate::vk1_0::StructureType) -> Option<usize> {
    crate::features2_bool_count(structure_type)
}

/// An abstract representation of any features struct extending
/// [`vk::PhysicalDeviceFeatures2`](crate::vk1_1::PhysicalDeviceFeatures2).
///
/// This is a dynamically sized type, which is why it can't be stored on the
/// stack. It is designed to be indistinguishable from the FFI type.
#[repr(C)]
#[derive(Debug)]
pub struct AbstractFeaturesNode {
    /// The base information, detailing the structure type and the pointer to
    /// the next node in the chain.
    pub base: vk::BaseOutStructure,
    /// A list of 4-byte Vulkan booleans for every feature. The length of this
    /// array is stored in the reference/pointer to this struct but can always
    /// be determined with [`features2_bool_count`].
    pub bools: [vk::Bool32],
}

/// An error that can occur when calling [`AbstractFeaturesNode`] methods.
#[derive(Debug)]
pub enum AbstractFeaturesNodeError {
    /// Structure is not a known features struct. The only valid structures are
    /// the ones which extend
    /// [`vk::PhysicalDeviceFeatures2`](crate::vk1_1::PhysicalDeviceFeatures2).
    UnknownFeaturesStruct,
    /// Failed to allocate memory.
    MemoryAllocationFailed,
}

impl fmt::Display for AbstractFeaturesNodeError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AbstractFeaturesNodeError::UnknownFeaturesStruct => {
                write!(fmt, "Structure is not a known features struct")
            }
            AbstractFeaturesNodeError::MemoryAllocationFailed => {
                write!(fmt, "Failed to allocate memory")
            }
        }
    }
}

impl Error for AbstractFeaturesNodeError {}

impl AbstractFeaturesNode {
    /// Converts a raw features struct pointer to a pointer to
    /// [`AbstractFeaturesNode`].
    ///
    /// Errors: [`AbstractFeaturesNodeError::UnknownFeaturesStruct`]
    pub unsafe fn from_raw(
        node: *mut vk::BaseOutStructure,
    ) -> Result<*mut AbstractFeaturesNode, AbstractFeaturesNodeError> {
        let bool_count = features2_bool_count((*node).s_type)
            .ok_or(AbstractFeaturesNodeError::UnknownFeaturesStruct)?;
        Ok(ptr::slice_from_raw_parts_mut(node as *mut vk::Bool32, bool_count) as _)
    }

    /// Casts this node into a generic raw pointer.
    pub fn as_raw(&mut self) -> *mut vk::BaseOutStructure {
        self as *mut _ as _
    }

    /// Returns the layout for the structure type along with its bool count.
    pub unsafe fn layout(
        s_type: vk::StructureType,
    ) -> Result<(Layout, usize), AbstractFeaturesNodeError> {
        let bool_count =
            features2_bool_count(s_type).ok_or(AbstractFeaturesNodeError::UnknownFeaturesStruct)?;
        let (layout, _bools_offset) = Layout::array::<vk::Bool32>(bool_count)
            .and_then(|bools| Layout::new::<vk::BaseOutStructure>().extend(bools))
            .unwrap();
        Ok((layout.pad_to_align(), bool_count))
    }

    /// Copies a single node (**not** including its tail) into a new allocation of
    /// [`AbstractFeaturesNode`].
    ///
    /// Errors: [`AbstractFeaturesNodeError::UnknownFeaturesStruct`],
    /// [`AbstractFeaturesNodeError::MemoryAllocationFailed`]
    pub unsafe fn copy_single(
        node: *const vk::BaseOutStructure,
    ) -> Result<Box<AbstractFeaturesNode>, AbstractFeaturesNodeError> {
        let (layout, bool_count) = AbstractFeaturesNode::layout((*node).s_type)?;
        let allocation = alloc::alloc(layout);
        if allocation.is_null() {
            return Err(AbstractFeaturesNodeError::MemoryAllocationFailed);
        }

        ptr::copy_nonoverlapping(node.cast::<u8>(), allocation, layout.size());

        let fat_ptr = ptr::slice_from_raw_parts_mut(allocation as *mut vk::Bool32, bool_count);
        Ok(Box::from_raw(fat_ptr as *mut _))
    }

    /// Copies a node (along with its tail) into new allocations of
    /// [`AbstractFeaturesNode`]. You are responsible for managing the memory
    /// of all the nodes in the pointer chain yourself. You can free the entire
    /// returned pointer chain with [`AbstractFeaturesNode::free_chain`].
    ///
    /// Errors: [`AbstractFeaturesNodeError::UnknownFeaturesStruct`],
    /// [`AbstractFeaturesNodeError::MemoryAllocationFailed`]
    pub unsafe fn copy_chain(
        node: *mut vk::BaseOutStructure,
    ) -> Result<*mut AbstractFeaturesNode, AbstractFeaturesNodeError> {
        let mut first_node: Option<*mut AbstractFeaturesNode> = None;
        let mut last_node: Option<*mut AbstractFeaturesNode> = None;
        for node in super::iterate_ptr_chain(node) {
            let abstract_node = Box::into_raw(AbstractFeaturesNode::copy_single(node)?);
            if let Some(last_node) = last_node.or(first_node) {
                (*last_node).base.p_next = abstract_node as _;
            }

            if first_node.is_none() {
                first_node = Some(abstract_node);
            } else {
                last_node = Some(abstract_node);
            }
        }

        Ok(first_node.unwrap())
    }

    /// Frees a node (along with its tail). This is useful when you are
    /// responsible for managing the memory of the pointer chain nodes by
    /// yourself, for example through [`AbstractFeaturesNode::copy_chain`].
    ///
    /// Errors: [`AbstractFeaturesNodeError::UnknownFeaturesStruct`]
    pub unsafe fn free_chain(
        node: *mut AbstractFeaturesNode,
    ) -> Result<(), AbstractFeaturesNodeError> {
        let mut node = node as *mut vk::BaseOutStructure;
        while !node.is_null() {
            let vk::BaseOutStructure { s_type, p_next } = *node;
            let (layout, _bool_count) = AbstractFeaturesNode::layout(s_type)?;
            alloc::dealloc(node as _, layout);
            node = p_next;
        }

        Ok(())
    }
}

/// Returns `true` if all enabled features in the `query` pointer chain are
/// supported on `physical_device`.
///
/// `physical_device` must have been retrieved from `instance`.
///
/// Errors: [`AbstractFeaturesNodeError::UnknownFeaturesStruct`],
/// [`AbstractFeaturesNodeError::MemoryAllocationFailed`]
pub unsafe fn supported(
    instance: &InstanceLoader,
    physical_device: vk::PhysicalDevice,
    query: &vk::PhysicalDeviceFeatures2Builder,
) -> Result<bool, AbstractFeaturesNodeError> {
    let copied = AbstractFeaturesNode::copy_chain(query as *const _ as _)?;
    instance.get_physical_device_features2(
        physical_device,
        &mut *(copied as *mut vk::PhysicalDeviceFeatures2),
    );

    for (query_node, supported_node) in
        super::iterate_ptr_chain(query as *const _ as _).zip(super::iterate_ptr_chain(copied as _))
    {
        let query_node = AbstractFeaturesNode::from_raw(query_node as _)?;
        let supported_node = AbstractFeaturesNode::from_raw(supported_node as _)?;
        if !((*query_node).bools.iter())
            .zip((*supported_node).bools.iter())
            .all(|(query, supported)| *query == 0 || *supported != 0)
        {
            return Ok(false);
        }
    }

    AbstractFeaturesNode::free_chain(copied)?;
    Ok(true)
}
