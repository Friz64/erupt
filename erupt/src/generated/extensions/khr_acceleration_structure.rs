#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_ACCELERATION_STRUCTURE_SPEC_VERSION")]
pub const KHR_ACCELERATION_STRUCTURE_SPEC_VERSION: u32 = 11;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_ACCELERATION_STRUCTURE_EXTENSION_NAME")]
pub const KHR_ACCELERATION_STRUCTURE_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_acceleration_structure");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_DESTROY_ACCELERATION_STRUCTURE_KHR: *const std::os::raw::c_char = crate::cstr!("vkDestroyAccelerationStructureKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_COPY_ACCELERATION_STRUCTURE_KHR: *const std::os::raw::c_char = crate::cstr!("vkCmdCopyAccelerationStructureKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_COPY_ACCELERATION_STRUCTURE_KHR: *const std::os::raw::c_char = crate::cstr!("vkCopyAccelerationStructureKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_COPY_ACCELERATION_STRUCTURE_TO_MEMORY_KHR: *const std::os::raw::c_char = crate::cstr!("vkCmdCopyAccelerationStructureToMemoryKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_COPY_ACCELERATION_STRUCTURE_TO_MEMORY_KHR: *const std::os::raw::c_char = crate::cstr!("vkCopyAccelerationStructureToMemoryKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_COPY_MEMORY_TO_ACCELERATION_STRUCTURE_KHR: *const std::os::raw::c_char = crate::cstr!("vkCmdCopyMemoryToAccelerationStructureKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_COPY_MEMORY_TO_ACCELERATION_STRUCTURE_KHR: *const std::os::raw::c_char = crate::cstr!("vkCopyMemoryToAccelerationStructureKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_WRITE_ACCELERATION_STRUCTURES_PROPERTIES_KHR: *const std::os::raw::c_char = crate::cstr!("vkCmdWriteAccelerationStructuresPropertiesKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_WRITE_ACCELERATION_STRUCTURES_PROPERTIES_KHR: *const std::os::raw::c_char = crate::cstr!("vkWriteAccelerationStructuresPropertiesKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_DEVICE_ACCELERATION_STRUCTURE_COMPATIBILITY_KHR: *const std::os::raw::c_char = crate::cstr!("vkGetDeviceAccelerationStructureCompatibilityKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_ACCELERATION_STRUCTURE_KHR: *const std::os::raw::c_char = crate::cstr!("vkCreateAccelerationStructureKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_BUILD_ACCELERATION_STRUCTURES_KHR: *const std::os::raw::c_char = crate::cstr!("vkCmdBuildAccelerationStructuresKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_BUILD_ACCELERATION_STRUCTURES_INDIRECT_KHR: *const std::os::raw::c_char = crate::cstr!("vkCmdBuildAccelerationStructuresIndirectKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_BUILD_ACCELERATION_STRUCTURES_KHR: *const std::os::raw::c_char = crate::cstr!("vkBuildAccelerationStructuresKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_ACCELERATION_STRUCTURE_DEVICE_ADDRESS_KHR: *const std::os::raw::c_char = crate::cstr!("vkGetAccelerationStructureDeviceAddressKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_ACCELERATION_STRUCTURE_BUILD_SIZES_KHR: *const std::os::raw::c_char = crate::cstr!("vkGetAccelerationStructureBuildSizesKHR");
crate::non_dispatchable_handle!(AccelerationStructureKHR, ACCELERATION_STRUCTURE_KHR, "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureKHR.html) · Non-dispatchable Handle", "VkAccelerationStructureKHR");
#[doc = "Provided by [`crate::extensions::khr_acceleration_structure`]"]
impl crate::vk1_0::BufferUsageFlagBits {
    pub const ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR: Self = Self(524288);
    pub const ACCELERATION_STRUCTURE_STORAGE_KHR: Self = Self(1048576);
}
#[doc = "Provided by [`crate::extensions::khr_acceleration_structure`]"]
impl crate::vk1_0::DescriptorType {
    pub const ACCELERATION_STRUCTURE_KHR: Self = Self(1000150000);
}
#[doc = "Provided by [`crate::extensions::khr_acceleration_structure`]"]
impl crate::vk1_0::FormatFeatureFlagBits {
    pub const ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR: Self = Self(536870912);
}
#[doc = "Provided by [`crate::extensions::khr_acceleration_structure`]"]
impl crate::vk1_0::IndexType {
    pub const NONE_KHR: Self = Self(1000165000);
}
#[doc = "Provided by [`crate::extensions::khr_acceleration_structure`]"]
impl crate::vk1_0::AccessFlagBits {
    pub const ACCELERATION_STRUCTURE_READ_KHR: Self = Self(2097152);
    pub const ACCELERATION_STRUCTURE_WRITE_KHR: Self = Self(4194304);
}
#[doc = "Provided by [`crate::extensions::khr_acceleration_structure`]"]
impl crate::vk1_0::QueryType {
    pub const ACCELERATION_STRUCTURE_COMPACTED_SIZE_KHR: Self = Self(1000150000);
    pub const ACCELERATION_STRUCTURE_SERIALIZATION_SIZE_KHR: Self = Self(1000150001);
}
#[doc = "Provided by [`crate::extensions::khr_acceleration_structure`]"]
impl crate::vk1_0::StructureType {
    pub const WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_KHR: Self = Self(1000150007);
    pub const ACCELERATION_STRUCTURE_BUILD_GEOMETRY_INFO_KHR: Self = Self(1000150000);
    pub const ACCELERATION_STRUCTURE_DEVICE_ADDRESS_INFO_KHR: Self = Self(1000150002);
    pub const ACCELERATION_STRUCTURE_GEOMETRY_AABBS_DATA_KHR: Self = Self(1000150003);
    pub const ACCELERATION_STRUCTURE_GEOMETRY_INSTANCES_DATA_KHR: Self = Self(1000150004);
    pub const ACCELERATION_STRUCTURE_GEOMETRY_TRIANGLES_DATA_KHR: Self = Self(1000150005);
    pub const ACCELERATION_STRUCTURE_GEOMETRY_KHR: Self = Self(1000150006);
    pub const ACCELERATION_STRUCTURE_VERSION_INFO_KHR: Self = Self(1000150009);
    pub const COPY_ACCELERATION_STRUCTURE_INFO_KHR: Self = Self(1000150010);
    pub const COPY_ACCELERATION_STRUCTURE_TO_MEMORY_INFO_KHR: Self = Self(1000150011);
    pub const COPY_MEMORY_TO_ACCELERATION_STRUCTURE_INFO_KHR: Self = Self(1000150012);
    pub const PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR: Self = Self(1000150013);
    pub const PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_PROPERTIES_KHR: Self = Self(1000150014);
    pub const ACCELERATION_STRUCTURE_CREATE_INFO_KHR: Self = Self(1000150017);
    pub const ACCELERATION_STRUCTURE_BUILD_SIZES_INFO_KHR: Self = Self(1000150020);
}
#[doc = "Provided by [`crate::extensions::khr_acceleration_structure`]"]
impl crate::vk1_0::PipelineStageFlagBits {
    pub const ACCELERATION_STRUCTURE_BUILD_KHR: Self = Self(33554432);
}
#[doc = "Provided by [`crate::extensions::khr_acceleration_structure`]"]
impl crate::vk1_0::ObjectType {
    pub const ACCELERATION_STRUCTURE_KHR: Self = Self(1000150000);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeometryFlagsKHR.html) · Bitmask of [`GeometryFlagBitsKHR`]"] # [doc (alias = "VkGeometryFlagsKHR")] # [derive (Default)] # [repr (transparent)] pub struct GeometryFlagsKHR : u32 { const OPAQUE_KHR = GeometryFlagBitsKHR :: OPAQUE_KHR . 0 ; const NO_DUPLICATE_ANY_HIT_INVOCATION_KHR = GeometryFlagBitsKHR :: NO_DUPLICATE_ANY_HIT_INVOCATION_KHR . 0 ; const OPAQUE_NV = GeometryFlagBitsKHR :: OPAQUE_NV . 0 ; const NO_DUPLICATE_ANY_HIT_INVOCATION_NV = GeometryFlagBitsKHR :: NO_DUPLICATE_ANY_HIT_INVOCATION_NV . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeometryFlagBitsKHR.html) · Bits enum of [`GeometryFlagsKHR`]"]
#[doc(alias = "VkGeometryFlagBitsKHR")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct GeometryFlagBitsKHR(pub u32);
impl GeometryFlagBitsKHR {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> GeometryFlagsKHR {
        GeometryFlagsKHR::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for GeometryFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::OPAQUE_KHR => "OPAQUE_KHR",
            &Self::NO_DUPLICATE_ANY_HIT_INVOCATION_KHR => "NO_DUPLICATE_ANY_HIT_INVOCATION_KHR",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::khr_acceleration_structure`]"]
impl crate::extensions::khr_acceleration_structure::GeometryFlagBitsKHR {
    pub const OPAQUE_KHR: Self = Self(1);
    pub const NO_DUPLICATE_ANY_HIT_INVOCATION_KHR: Self = Self(2);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeometryInstanceFlagsKHR.html) · Bitmask of [`GeometryInstanceFlagBitsKHR`]"] # [doc (alias = "VkGeometryInstanceFlagsKHR")] # [derive (Default)] # [repr (transparent)] pub struct GeometryInstanceFlagsKHR : u32 { const TRIANGLE_FACING_CULL_DISABLE_KHR = GeometryInstanceFlagBitsKHR :: TRIANGLE_FACING_CULL_DISABLE_KHR . 0 ; const TRIANGLE_FRONT_COUNTERCLOCKWISE_KHR = GeometryInstanceFlagBitsKHR :: TRIANGLE_FRONT_COUNTERCLOCKWISE_KHR . 0 ; const FORCE_OPAQUE_KHR = GeometryInstanceFlagBitsKHR :: FORCE_OPAQUE_KHR . 0 ; const FORCE_NO_OPAQUE_KHR = GeometryInstanceFlagBitsKHR :: FORCE_NO_OPAQUE_KHR . 0 ; const TRIANGLE_CULL_DISABLE_NV = GeometryInstanceFlagBitsKHR :: TRIANGLE_CULL_DISABLE_NV . 0 ; const TRIANGLE_FRONT_COUNTERCLOCKWISE_NV = GeometryInstanceFlagBitsKHR :: TRIANGLE_FRONT_COUNTERCLOCKWISE_NV . 0 ; const FORCE_OPAQUE_NV = GeometryInstanceFlagBitsKHR :: FORCE_OPAQUE_NV . 0 ; const FORCE_NO_OPAQUE_NV = GeometryInstanceFlagBitsKHR :: FORCE_NO_OPAQUE_NV . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeometryInstanceFlagBitsKHR.html) · Bits enum of [`GeometryInstanceFlagsKHR`]"]
#[doc(alias = "VkGeometryInstanceFlagBitsKHR")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct GeometryInstanceFlagBitsKHR(pub u32);
impl GeometryInstanceFlagBitsKHR {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> GeometryInstanceFlagsKHR {
        GeometryInstanceFlagsKHR::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for GeometryInstanceFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::TRIANGLE_FACING_CULL_DISABLE_KHR => "TRIANGLE_FACING_CULL_DISABLE_KHR",
            &Self::TRIANGLE_FRONT_COUNTERCLOCKWISE_KHR => "TRIANGLE_FRONT_COUNTERCLOCKWISE_KHR",
            &Self::FORCE_OPAQUE_KHR => "FORCE_OPAQUE_KHR",
            &Self::FORCE_NO_OPAQUE_KHR => "FORCE_NO_OPAQUE_KHR",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::khr_acceleration_structure`]"]
impl crate::extensions::khr_acceleration_structure::GeometryInstanceFlagBitsKHR {
    pub const TRIANGLE_FACING_CULL_DISABLE_KHR: Self = Self(1);
    pub const TRIANGLE_FRONT_COUNTERCLOCKWISE_KHR: Self = Self(2);
    pub const FORCE_OPAQUE_KHR: Self = Self(4);
    pub const FORCE_NO_OPAQUE_KHR: Self = Self(8);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBuildAccelerationStructureFlagsKHR.html) · Bitmask of [`BuildAccelerationStructureFlagBitsKHR`]"] # [doc (alias = "VkBuildAccelerationStructureFlagsKHR")] # [derive (Default)] # [repr (transparent)] pub struct BuildAccelerationStructureFlagsKHR : u32 { const ALLOW_UPDATE_KHR = BuildAccelerationStructureFlagBitsKHR :: ALLOW_UPDATE_KHR . 0 ; const ALLOW_COMPACTION_KHR = BuildAccelerationStructureFlagBitsKHR :: ALLOW_COMPACTION_KHR . 0 ; const PREFER_FAST_TRACE_KHR = BuildAccelerationStructureFlagBitsKHR :: PREFER_FAST_TRACE_KHR . 0 ; const PREFER_FAST_BUILD_KHR = BuildAccelerationStructureFlagBitsKHR :: PREFER_FAST_BUILD_KHR . 0 ; const LOW_MEMORY_KHR = BuildAccelerationStructureFlagBitsKHR :: LOW_MEMORY_KHR . 0 ; const MOTION_NV = BuildAccelerationStructureFlagBitsKHR :: MOTION_NV . 0 ; const ALLOW_UPDATE_NV = BuildAccelerationStructureFlagBitsKHR :: ALLOW_UPDATE_NV . 0 ; const ALLOW_COMPACTION_NV = BuildAccelerationStructureFlagBitsKHR :: ALLOW_COMPACTION_NV . 0 ; const PREFER_FAST_TRACE_NV = BuildAccelerationStructureFlagBitsKHR :: PREFER_FAST_TRACE_NV . 0 ; const PREFER_FAST_BUILD_NV = BuildAccelerationStructureFlagBitsKHR :: PREFER_FAST_BUILD_NV . 0 ; const LOW_MEMORY_NV = BuildAccelerationStructureFlagBitsKHR :: LOW_MEMORY_NV . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBuildAccelerationStructureFlagBitsKHR.html) · Bits enum of [`BuildAccelerationStructureFlagsKHR`]"]
#[doc(alias = "VkBuildAccelerationStructureFlagBitsKHR")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct BuildAccelerationStructureFlagBitsKHR(pub u32);
impl BuildAccelerationStructureFlagBitsKHR {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> BuildAccelerationStructureFlagsKHR {
        BuildAccelerationStructureFlagsKHR::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for BuildAccelerationStructureFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::ALLOW_UPDATE_KHR => "ALLOW_UPDATE_KHR",
            &Self::ALLOW_COMPACTION_KHR => "ALLOW_COMPACTION_KHR",
            &Self::PREFER_FAST_TRACE_KHR => "PREFER_FAST_TRACE_KHR",
            &Self::PREFER_FAST_BUILD_KHR => "PREFER_FAST_BUILD_KHR",
            &Self::LOW_MEMORY_KHR => "LOW_MEMORY_KHR",
            &Self::MOTION_NV => "MOTION_NV",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::khr_acceleration_structure`]"]
impl crate::extensions::khr_acceleration_structure::BuildAccelerationStructureFlagBitsKHR {
    pub const ALLOW_UPDATE_KHR: Self = Self(1);
    pub const ALLOW_COMPACTION_KHR: Self = Self(2);
    pub const PREFER_FAST_TRACE_KHR: Self = Self(4);
    pub const PREFER_FAST_BUILD_KHR: Self = Self(8);
    pub const LOW_MEMORY_KHR: Self = Self(16);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureCreateFlagsKHR.html) · Bitmask of [`AccelerationStructureCreateFlagBitsKHR`]"] # [doc (alias = "VkAccelerationStructureCreateFlagsKHR")] # [derive (Default)] # [repr (transparent)] pub struct AccelerationStructureCreateFlagsKHR : u32 { const DEVICE_ADDRESS_CAPTURE_REPLAY_KHR = AccelerationStructureCreateFlagBitsKHR :: DEVICE_ADDRESS_CAPTURE_REPLAY_KHR . 0 ; const MOTION_NV = AccelerationStructureCreateFlagBitsKHR :: MOTION_NV . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureCreateFlagBitsKHR.html) · Bits enum of [`AccelerationStructureCreateFlagsKHR`]"]
#[doc(alias = "VkAccelerationStructureCreateFlagBitsKHR")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct AccelerationStructureCreateFlagBitsKHR(pub u32);
impl AccelerationStructureCreateFlagBitsKHR {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> AccelerationStructureCreateFlagsKHR {
        AccelerationStructureCreateFlagsKHR::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for AccelerationStructureCreateFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::DEVICE_ADDRESS_CAPTURE_REPLAY_KHR => "DEVICE_ADDRESS_CAPTURE_REPLAY_KHR",
            &Self::MOTION_NV => "MOTION_NV",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::khr_acceleration_structure`]"]
impl crate::extensions::khr_acceleration_structure::AccelerationStructureCreateFlagBitsKHR {
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY_KHR: Self = Self(1);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBuildAccelerationStructureModeKHR.html) · Enum"]
#[doc(alias = "VkBuildAccelerationStructureModeKHR")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct BuildAccelerationStructureModeKHR(pub i32);
impl std::fmt::Debug for BuildAccelerationStructureModeKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::BUILD_KHR => "BUILD_KHR",
            &Self::UPDATE_KHR => "UPDATE_KHR",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::khr_acceleration_structure`]"]
impl crate::extensions::khr_acceleration_structure::BuildAccelerationStructureModeKHR {
    pub const BUILD_KHR: Self = Self(0);
    pub const UPDATE_KHR: Self = Self(1);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCopyAccelerationStructureModeKHR.html) · Enum"]
#[doc(alias = "VkCopyAccelerationStructureModeKHR")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct CopyAccelerationStructureModeKHR(pub i32);
impl std::fmt::Debug for CopyAccelerationStructureModeKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::CLONE_KHR => "CLONE_KHR",
            &Self::COMPACT_KHR => "COMPACT_KHR",
            &Self::SERIALIZE_KHR => "SERIALIZE_KHR",
            &Self::DESERIALIZE_KHR => "DESERIALIZE_KHR",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::khr_acceleration_structure`]"]
impl crate::extensions::khr_acceleration_structure::CopyAccelerationStructureModeKHR {
    pub const CLONE_KHR: Self = Self(0);
    pub const COMPACT_KHR: Self = Self(1);
    pub const SERIALIZE_KHR: Self = Self(2);
    pub const DESERIALIZE_KHR: Self = Self(3);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureTypeKHR.html) · Enum"]
#[doc(alias = "VkAccelerationStructureTypeKHR")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct AccelerationStructureTypeKHR(pub i32);
impl std::fmt::Debug for AccelerationStructureTypeKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::TOP_LEVEL_KHR => "TOP_LEVEL_KHR",
            &Self::BOTTOM_LEVEL_KHR => "BOTTOM_LEVEL_KHR",
            &Self::GENERIC_KHR => "GENERIC_KHR",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::khr_acceleration_structure`]"]
impl crate::extensions::khr_acceleration_structure::AccelerationStructureTypeKHR {
    pub const TOP_LEVEL_KHR: Self = Self(0);
    pub const BOTTOM_LEVEL_KHR: Self = Self(1);
    pub const GENERIC_KHR: Self = Self(2);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeometryTypeKHR.html) · Enum"]
#[doc(alias = "VkGeometryTypeKHR")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct GeometryTypeKHR(pub i32);
impl std::fmt::Debug for GeometryTypeKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::TRIANGLES_KHR => "TRIANGLES_KHR",
            &Self::AABBS_KHR => "AABBS_KHR",
            &Self::INSTANCES_KHR => "INSTANCES_KHR",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::khr_acceleration_structure`]"]
impl crate::extensions::khr_acceleration_structure::GeometryTypeKHR {
    pub const TRIANGLES_KHR: Self = Self(0);
    pub const AABBS_KHR: Self = Self(1);
    pub const INSTANCES_KHR: Self = Self(2);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureBuildTypeKHR.html) · Enum"]
#[doc(alias = "VkAccelerationStructureBuildTypeKHR")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct AccelerationStructureBuildTypeKHR(pub i32);
impl std::fmt::Debug for AccelerationStructureBuildTypeKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::HOST_KHR => "HOST_KHR",
            &Self::DEVICE_KHR => "DEVICE_KHR",
            &Self::HOST_OR_DEVICE_KHR => "HOST_OR_DEVICE_KHR",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::khr_acceleration_structure`]"]
impl crate::extensions::khr_acceleration_structure::AccelerationStructureBuildTypeKHR {
    pub const HOST_KHR: Self = Self(0);
    pub const DEVICE_KHR: Self = Self(1);
    pub const HOST_OR_DEVICE_KHR: Self = Self(2);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureCompatibilityKHR.html) · Enum"]
#[doc(alias = "VkAccelerationStructureCompatibilityKHR")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct AccelerationStructureCompatibilityKHR(pub i32);
impl std::fmt::Debug for AccelerationStructureCompatibilityKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::COMPATIBLE_KHR => "COMPATIBLE_KHR",
            &Self::INCOMPATIBLE_KHR => "INCOMPATIBLE_KHR",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::khr_acceleration_structure`]"]
impl crate::extensions::khr_acceleration_structure::AccelerationStructureCompatibilityKHR {
    pub const COMPATIBLE_KHR: Self = Self(0);
    pub const INCOMPATIBLE_KHR: Self = Self(1);
}
#[doc = "Provided by [`crate::extensions::khr_acceleration_structure`]"]
impl crate::extensions::ext_debug_report::DebugReportObjectTypeEXT {
    pub const ACCELERATION_STRUCTURE_KHR_EXT: Self = Self(1000150000);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyAccelerationStructureKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyAccelerationStructureKHR = unsafe extern "system" fn(device: crate::vk1_0::Device, acceleration_structure: crate::extensions::khr_acceleration_structure::AccelerationStructureKHR, p_allocator: *const crate::vk1_0::AllocationCallbacks) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyAccelerationStructureKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyAccelerationStructureKHR = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, p_info: *const crate::extensions::khr_acceleration_structure::CopyAccelerationStructureInfoKHR) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCopyAccelerationStructureKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCopyAccelerationStructureKHR = unsafe extern "system" fn(device: crate::vk1_0::Device, deferred_operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR, p_info: *const crate::extensions::khr_acceleration_structure::CopyAccelerationStructureInfoKHR) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyAccelerationStructureToMemoryKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyAccelerationStructureToMemoryKHR = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, p_info: *const crate::extensions::khr_acceleration_structure::CopyAccelerationStructureToMemoryInfoKHR) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCopyAccelerationStructureToMemoryKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCopyAccelerationStructureToMemoryKHR = unsafe extern "system" fn(device: crate::vk1_0::Device, deferred_operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR, p_info: *const crate::extensions::khr_acceleration_structure::CopyAccelerationStructureToMemoryInfoKHR) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyMemoryToAccelerationStructureKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyMemoryToAccelerationStructureKHR = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, p_info: *const crate::extensions::khr_acceleration_structure::CopyMemoryToAccelerationStructureInfoKHR) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCopyMemoryToAccelerationStructureKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCopyMemoryToAccelerationStructureKHR = unsafe extern "system" fn(device: crate::vk1_0::Device, deferred_operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR, p_info: *const crate::extensions::khr_acceleration_structure::CopyMemoryToAccelerationStructureInfoKHR) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdWriteAccelerationStructuresPropertiesKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdWriteAccelerationStructuresPropertiesKHR = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, acceleration_structure_count: u32, p_acceleration_structures: *const crate::extensions::khr_acceleration_structure::AccelerationStructureKHR, query_type: crate::vk1_0::QueryType, query_pool: crate::vk1_0::QueryPool, first_query: u32) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkWriteAccelerationStructuresPropertiesKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkWriteAccelerationStructuresPropertiesKHR = unsafe extern "system" fn(device: crate::vk1_0::Device, acceleration_structure_count: u32, p_acceleration_structures: *const crate::extensions::khr_acceleration_structure::AccelerationStructureKHR, query_type: crate::vk1_0::QueryType, data_size: usize, p_data: *mut std::ffi::c_void, stride: usize) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceAccelerationStructureCompatibilityKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceAccelerationStructureCompatibilityKHR = unsafe extern "system" fn(device: crate::vk1_0::Device, p_version_info: *const crate::extensions::khr_acceleration_structure::AccelerationStructureVersionInfoKHR, p_compatibility: *mut crate::extensions::khr_acceleration_structure::AccelerationStructureCompatibilityKHR) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateAccelerationStructureKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateAccelerationStructureKHR = unsafe extern "system" fn(device: crate::vk1_0::Device, p_create_info: *const crate::extensions::khr_acceleration_structure::AccelerationStructureCreateInfoKHR, p_allocator: *const crate::vk1_0::AllocationCallbacks, p_acceleration_structure: *mut crate::extensions::khr_acceleration_structure::AccelerationStructureKHR) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBuildAccelerationStructuresKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBuildAccelerationStructuresKHR = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, info_count: u32, p_infos: *const crate::extensions::khr_acceleration_structure::AccelerationStructureBuildGeometryInfoKHR, pp_build_range_infos: *const *const crate::extensions::khr_acceleration_structure::AccelerationStructureBuildRangeInfoKHR) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBuildAccelerationStructuresIndirectKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBuildAccelerationStructuresIndirectKHR = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, info_count: u32, p_infos: *const crate::extensions::khr_acceleration_structure::AccelerationStructureBuildGeometryInfoKHR, p_indirect_device_addresses: *const crate::vk1_0::DeviceAddress, p_indirect_strides: *const u32, pp_max_primitive_counts: *const *const u32) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBuildAccelerationStructuresKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkBuildAccelerationStructuresKHR = unsafe extern "system" fn(device: crate::vk1_0::Device, deferred_operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR, info_count: u32, p_infos: *const crate::extensions::khr_acceleration_structure::AccelerationStructureBuildGeometryInfoKHR, pp_build_range_infos: *const *const crate::extensions::khr_acceleration_structure::AccelerationStructureBuildRangeInfoKHR) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetAccelerationStructureDeviceAddressKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetAccelerationStructureDeviceAddressKHR = unsafe extern "system" fn(device: crate::vk1_0::Device, p_info: *const crate::extensions::khr_acceleration_structure::AccelerationStructureDeviceAddressInfoKHR) -> crate::vk1_0::DeviceAddress;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetAccelerationStructureBuildSizesKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetAccelerationStructureBuildSizesKHR = unsafe extern "system" fn(device: crate::vk1_0::Device, build_type: crate::extensions::khr_acceleration_structure::AccelerationStructureBuildTypeKHR, p_build_info: *const crate::extensions::khr_acceleration_structure::AccelerationStructureBuildGeometryInfoKHR, p_max_primitive_counts: *const u32, p_size_info: *mut crate::extensions::khr_acceleration_structure::AccelerationStructureBuildSizesInfoKHR) -> ();
impl<'a> crate::ExtendableFromConst<'a, PhysicalDeviceAccelerationStructureFeaturesKHR> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, PhysicalDeviceAccelerationStructureFeaturesKHRBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, WriteDescriptorSetAccelerationStructureKHR> for crate::vk1_0::WriteDescriptorSetBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, WriteDescriptorSetAccelerationStructureKHRBuilder<'_>> for crate::vk1_0::WriteDescriptorSetBuilder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceAccelerationStructureFeaturesKHR> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceAccelerationStructureFeaturesKHRBuilder<'_>> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceAccelerationStructurePropertiesKHR> for crate::vk1_1::PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceAccelerationStructurePropertiesKHRBuilder<'_>> for crate::vk1_1::PhysicalDeviceProperties2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkWriteDescriptorSetAccelerationStructureKHR.html) · Structure"]
#[doc(alias = "VkWriteDescriptorSetAccelerationStructureKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WriteDescriptorSetAccelerationStructureKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub acceleration_structure_count: u32,
    pub p_acceleration_structures: *const crate::extensions::khr_acceleration_structure::AccelerationStructureKHR,
}
impl WriteDescriptorSetAccelerationStructureKHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_KHR;
}
impl Default for WriteDescriptorSetAccelerationStructureKHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), acceleration_structure_count: Default::default(), p_acceleration_structures: std::ptr::null() }
    }
}
impl std::fmt::Debug for WriteDescriptorSetAccelerationStructureKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("WriteDescriptorSetAccelerationStructureKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("acceleration_structure_count", &self.acceleration_structure_count).field("p_acceleration_structures", &self.p_acceleration_structures).finish()
    }
}
impl WriteDescriptorSetAccelerationStructureKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> WriteDescriptorSetAccelerationStructureKHRBuilder<'a> {
        WriteDescriptorSetAccelerationStructureKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkWriteDescriptorSetAccelerationStructureKHR.html) · Builder of [`WriteDescriptorSetAccelerationStructureKHR`]"]
#[repr(transparent)]
pub struct WriteDescriptorSetAccelerationStructureKHRBuilder<'a>(WriteDescriptorSetAccelerationStructureKHR, std::marker::PhantomData<&'a ()>);
impl<'a> WriteDescriptorSetAccelerationStructureKHRBuilder<'a> {
    #[inline]
    pub fn new() -> WriteDescriptorSetAccelerationStructureKHRBuilder<'a> {
        WriteDescriptorSetAccelerationStructureKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn acceleration_structures(mut self, acceleration_structures: &'a [crate::extensions::khr_acceleration_structure::AccelerationStructureKHR]) -> Self {
        self.0.p_acceleration_structures = acceleration_structures.as_ptr() as _;
        self.0.acceleration_structure_count = acceleration_structures.len() as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> WriteDescriptorSetAccelerationStructureKHR {
        self.0
    }
}
impl<'a> std::default::Default for WriteDescriptorSetAccelerationStructureKHRBuilder<'a> {
    fn default() -> WriteDescriptorSetAccelerationStructureKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for WriteDescriptorSetAccelerationStructureKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for WriteDescriptorSetAccelerationStructureKHRBuilder<'a> {
    type Target = WriteDescriptorSetAccelerationStructureKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for WriteDescriptorSetAccelerationStructureKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceAccelerationStructureFeaturesKHR.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceAccelerationStructureFeaturesKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceAccelerationStructureFeaturesKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub acceleration_structure: crate::vk1_0::Bool32,
    pub acceleration_structure_capture_replay: crate::vk1_0::Bool32,
    pub acceleration_structure_indirect_build: crate::vk1_0::Bool32,
    pub acceleration_structure_host_commands: crate::vk1_0::Bool32,
    pub descriptor_binding_acceleration_structure_update_after_bind: crate::vk1_0::Bool32,
}
impl PhysicalDeviceAccelerationStructureFeaturesKHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR;
}
impl Default for PhysicalDeviceAccelerationStructureFeaturesKHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), acceleration_structure: Default::default(), acceleration_structure_capture_replay: Default::default(), acceleration_structure_indirect_build: Default::default(), acceleration_structure_host_commands: Default::default(), descriptor_binding_acceleration_structure_update_after_bind: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceAccelerationStructureFeaturesKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceAccelerationStructureFeaturesKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("acceleration_structure", &(self.acceleration_structure != 0)).field("acceleration_structure_capture_replay", &(self.acceleration_structure_capture_replay != 0)).field("acceleration_structure_indirect_build", &(self.acceleration_structure_indirect_build != 0)).field("acceleration_structure_host_commands", &(self.acceleration_structure_host_commands != 0)).field("descriptor_binding_acceleration_structure_update_after_bind", &(self.descriptor_binding_acceleration_structure_update_after_bind != 0)).finish()
    }
}
impl PhysicalDeviceAccelerationStructureFeaturesKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceAccelerationStructureFeaturesKHRBuilder<'a> {
        PhysicalDeviceAccelerationStructureFeaturesKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceAccelerationStructureFeaturesKHR.html) · Builder of [`PhysicalDeviceAccelerationStructureFeaturesKHR`]"]
#[repr(transparent)]
pub struct PhysicalDeviceAccelerationStructureFeaturesKHRBuilder<'a>(PhysicalDeviceAccelerationStructureFeaturesKHR, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceAccelerationStructureFeaturesKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceAccelerationStructureFeaturesKHRBuilder<'a> {
        PhysicalDeviceAccelerationStructureFeaturesKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn acceleration_structure(mut self, acceleration_structure: bool) -> Self {
        self.0.acceleration_structure = acceleration_structure as _;
        self
    }
    #[inline]
    pub fn acceleration_structure_capture_replay(mut self, acceleration_structure_capture_replay: bool) -> Self {
        self.0.acceleration_structure_capture_replay = acceleration_structure_capture_replay as _;
        self
    }
    #[inline]
    pub fn acceleration_structure_indirect_build(mut self, acceleration_structure_indirect_build: bool) -> Self {
        self.0.acceleration_structure_indirect_build = acceleration_structure_indirect_build as _;
        self
    }
    #[inline]
    pub fn acceleration_structure_host_commands(mut self, acceleration_structure_host_commands: bool) -> Self {
        self.0.acceleration_structure_host_commands = acceleration_structure_host_commands as _;
        self
    }
    #[inline]
    pub fn descriptor_binding_acceleration_structure_update_after_bind(mut self, descriptor_binding_acceleration_structure_update_after_bind: bool) -> Self {
        self.0.descriptor_binding_acceleration_structure_update_after_bind = descriptor_binding_acceleration_structure_update_after_bind as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceAccelerationStructureFeaturesKHR {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceAccelerationStructureFeaturesKHRBuilder<'a> {
    fn default() -> PhysicalDeviceAccelerationStructureFeaturesKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceAccelerationStructureFeaturesKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceAccelerationStructureFeaturesKHRBuilder<'a> {
    type Target = PhysicalDeviceAccelerationStructureFeaturesKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceAccelerationStructureFeaturesKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceAccelerationStructurePropertiesKHR.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceAccelerationStructurePropertiesKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceAccelerationStructurePropertiesKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub max_geometry_count: u64,
    pub max_instance_count: u64,
    pub max_primitive_count: u64,
    pub max_per_stage_descriptor_acceleration_structures: u32,
    pub max_per_stage_descriptor_update_after_bind_acceleration_structures: u32,
    pub max_descriptor_set_acceleration_structures: u32,
    pub max_descriptor_set_update_after_bind_acceleration_structures: u32,
    pub min_acceleration_structure_scratch_offset_alignment: u32,
}
impl PhysicalDeviceAccelerationStructurePropertiesKHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_PROPERTIES_KHR;
}
impl Default for PhysicalDeviceAccelerationStructurePropertiesKHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), max_geometry_count: Default::default(), max_instance_count: Default::default(), max_primitive_count: Default::default(), max_per_stage_descriptor_acceleration_structures: Default::default(), max_per_stage_descriptor_update_after_bind_acceleration_structures: Default::default(), max_descriptor_set_acceleration_structures: Default::default(), max_descriptor_set_update_after_bind_acceleration_structures: Default::default(), min_acceleration_structure_scratch_offset_alignment: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceAccelerationStructurePropertiesKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceAccelerationStructurePropertiesKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("max_geometry_count", &self.max_geometry_count).field("max_instance_count", &self.max_instance_count).field("max_primitive_count", &self.max_primitive_count).field("max_per_stage_descriptor_acceleration_structures", &self.max_per_stage_descriptor_acceleration_structures).field("max_per_stage_descriptor_update_after_bind_acceleration_structures", &self.max_per_stage_descriptor_update_after_bind_acceleration_structures).field("max_descriptor_set_acceleration_structures", &self.max_descriptor_set_acceleration_structures).field("max_descriptor_set_update_after_bind_acceleration_structures", &self.max_descriptor_set_update_after_bind_acceleration_structures).field("min_acceleration_structure_scratch_offset_alignment", &self.min_acceleration_structure_scratch_offset_alignment).finish()
    }
}
impl PhysicalDeviceAccelerationStructurePropertiesKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceAccelerationStructurePropertiesKHRBuilder<'a> {
        PhysicalDeviceAccelerationStructurePropertiesKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceAccelerationStructurePropertiesKHR.html) · Builder of [`PhysicalDeviceAccelerationStructurePropertiesKHR`]"]
#[repr(transparent)]
pub struct PhysicalDeviceAccelerationStructurePropertiesKHRBuilder<'a>(PhysicalDeviceAccelerationStructurePropertiesKHR, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceAccelerationStructurePropertiesKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceAccelerationStructurePropertiesKHRBuilder<'a> {
        PhysicalDeviceAccelerationStructurePropertiesKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn max_geometry_count(mut self, max_geometry_count: u64) -> Self {
        self.0.max_geometry_count = max_geometry_count as _;
        self
    }
    #[inline]
    pub fn max_instance_count(mut self, max_instance_count: u64) -> Self {
        self.0.max_instance_count = max_instance_count as _;
        self
    }
    #[inline]
    pub fn max_primitive_count(mut self, max_primitive_count: u64) -> Self {
        self.0.max_primitive_count = max_primitive_count as _;
        self
    }
    #[inline]
    pub fn max_per_stage_descriptor_acceleration_structures(mut self, max_per_stage_descriptor_acceleration_structures: u32) -> Self {
        self.0.max_per_stage_descriptor_acceleration_structures = max_per_stage_descriptor_acceleration_structures as _;
        self
    }
    #[inline]
    pub fn max_per_stage_descriptor_update_after_bind_acceleration_structures(mut self, max_per_stage_descriptor_update_after_bind_acceleration_structures: u32) -> Self {
        self.0.max_per_stage_descriptor_update_after_bind_acceleration_structures = max_per_stage_descriptor_update_after_bind_acceleration_structures as _;
        self
    }
    #[inline]
    pub fn max_descriptor_set_acceleration_structures(mut self, max_descriptor_set_acceleration_structures: u32) -> Self {
        self.0.max_descriptor_set_acceleration_structures = max_descriptor_set_acceleration_structures as _;
        self
    }
    #[inline]
    pub fn max_descriptor_set_update_after_bind_acceleration_structures(mut self, max_descriptor_set_update_after_bind_acceleration_structures: u32) -> Self {
        self.0.max_descriptor_set_update_after_bind_acceleration_structures = max_descriptor_set_update_after_bind_acceleration_structures as _;
        self
    }
    #[inline]
    pub fn min_acceleration_structure_scratch_offset_alignment(mut self, min_acceleration_structure_scratch_offset_alignment: u32) -> Self {
        self.0.min_acceleration_structure_scratch_offset_alignment = min_acceleration_structure_scratch_offset_alignment as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceAccelerationStructurePropertiesKHR {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceAccelerationStructurePropertiesKHRBuilder<'a> {
    fn default() -> PhysicalDeviceAccelerationStructurePropertiesKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceAccelerationStructurePropertiesKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceAccelerationStructurePropertiesKHRBuilder<'a> {
    type Target = PhysicalDeviceAccelerationStructurePropertiesKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceAccelerationStructurePropertiesKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceOrHostAddressKHR.html) · Structure"]
#[doc(alias = "VkDeviceOrHostAddressKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub union DeviceOrHostAddressKHR {
    pub device_address: crate::vk1_0::DeviceAddress,
    pub host_address: *mut std::ffi::c_void,
}
impl Default for DeviceOrHostAddressKHR {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for DeviceOrHostAddressKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DeviceOrHostAddressKHR").finish()
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceOrHostAddressConstKHR.html) · Structure"]
#[doc(alias = "VkDeviceOrHostAddressConstKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub union DeviceOrHostAddressConstKHR {
    pub device_address: crate::vk1_0::DeviceAddress,
    pub host_address: *const std::ffi::c_void,
}
impl Default for DeviceOrHostAddressConstKHR {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for DeviceOrHostAddressConstKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DeviceOrHostAddressConstKHR").finish()
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureGeometryTrianglesDataKHR.html) · Structure"]
#[doc(alias = "VkAccelerationStructureGeometryTrianglesDataKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AccelerationStructureGeometryTrianglesDataKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub vertex_format: crate::vk1_0::Format,
    pub vertex_data: crate::extensions::khr_acceleration_structure::DeviceOrHostAddressConstKHR,
    pub vertex_stride: crate::vk1_0::DeviceSize,
    pub max_vertex: u32,
    pub index_type: crate::vk1_0::IndexType,
    pub index_data: crate::extensions::khr_acceleration_structure::DeviceOrHostAddressConstKHR,
    pub transform_data: crate::extensions::khr_acceleration_structure::DeviceOrHostAddressConstKHR,
}
impl AccelerationStructureGeometryTrianglesDataKHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::ACCELERATION_STRUCTURE_GEOMETRY_TRIANGLES_DATA_KHR;
}
impl Default for AccelerationStructureGeometryTrianglesDataKHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), vertex_format: Default::default(), vertex_data: Default::default(), vertex_stride: Default::default(), max_vertex: Default::default(), index_type: Default::default(), index_data: Default::default(), transform_data: Default::default() }
    }
}
impl std::fmt::Debug for AccelerationStructureGeometryTrianglesDataKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AccelerationStructureGeometryTrianglesDataKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("vertex_format", &self.vertex_format).field("vertex_data", &self.vertex_data).field("vertex_stride", &self.vertex_stride).field("max_vertex", &self.max_vertex).field("index_type", &self.index_type).field("index_data", &self.index_data).field("transform_data", &self.transform_data).finish()
    }
}
impl AccelerationStructureGeometryTrianglesDataKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> AccelerationStructureGeometryTrianglesDataKHRBuilder<'a> {
        AccelerationStructureGeometryTrianglesDataKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureGeometryTrianglesDataKHR.html) · Builder of [`AccelerationStructureGeometryTrianglesDataKHR`]"]
#[repr(transparent)]
pub struct AccelerationStructureGeometryTrianglesDataKHRBuilder<'a>(AccelerationStructureGeometryTrianglesDataKHR, std::marker::PhantomData<&'a ()>);
impl<'a> AccelerationStructureGeometryTrianglesDataKHRBuilder<'a> {
    #[inline]
    pub fn new() -> AccelerationStructureGeometryTrianglesDataKHRBuilder<'a> {
        AccelerationStructureGeometryTrianglesDataKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn vertex_format(mut self, vertex_format: crate::vk1_0::Format) -> Self {
        self.0.vertex_format = vertex_format as _;
        self
    }
    #[inline]
    pub fn vertex_data(mut self, vertex_data: crate::extensions::khr_acceleration_structure::DeviceOrHostAddressConstKHR) -> Self {
        self.0.vertex_data = vertex_data as _;
        self
    }
    #[inline]
    pub fn vertex_stride(mut self, vertex_stride: crate::vk1_0::DeviceSize) -> Self {
        self.0.vertex_stride = vertex_stride as _;
        self
    }
    #[inline]
    pub fn max_vertex(mut self, max_vertex: u32) -> Self {
        self.0.max_vertex = max_vertex as _;
        self
    }
    #[inline]
    pub fn index_type(mut self, index_type: crate::vk1_0::IndexType) -> Self {
        self.0.index_type = index_type as _;
        self
    }
    #[inline]
    pub fn index_data(mut self, index_data: crate::extensions::khr_acceleration_structure::DeviceOrHostAddressConstKHR) -> Self {
        self.0.index_data = index_data as _;
        self
    }
    #[inline]
    pub fn transform_data(mut self, transform_data: crate::extensions::khr_acceleration_structure::DeviceOrHostAddressConstKHR) -> Self {
        self.0.transform_data = transform_data as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> AccelerationStructureGeometryTrianglesDataKHR {
        self.0
    }
}
impl<'a> std::default::Default for AccelerationStructureGeometryTrianglesDataKHRBuilder<'a> {
    fn default() -> AccelerationStructureGeometryTrianglesDataKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for AccelerationStructureGeometryTrianglesDataKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for AccelerationStructureGeometryTrianglesDataKHRBuilder<'a> {
    type Target = AccelerationStructureGeometryTrianglesDataKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for AccelerationStructureGeometryTrianglesDataKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureGeometryAabbsDataKHR.html) · Structure"]
#[doc(alias = "VkAccelerationStructureGeometryAabbsDataKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AccelerationStructureGeometryAabbsDataKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub data: crate::extensions::khr_acceleration_structure::DeviceOrHostAddressConstKHR,
    pub stride: crate::vk1_0::DeviceSize,
}
impl AccelerationStructureGeometryAabbsDataKHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::ACCELERATION_STRUCTURE_GEOMETRY_AABBS_DATA_KHR;
}
impl Default for AccelerationStructureGeometryAabbsDataKHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), data: Default::default(), stride: Default::default() }
    }
}
impl std::fmt::Debug for AccelerationStructureGeometryAabbsDataKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AccelerationStructureGeometryAabbsDataKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("data", &self.data).field("stride", &self.stride).finish()
    }
}
impl AccelerationStructureGeometryAabbsDataKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> AccelerationStructureGeometryAabbsDataKHRBuilder<'a> {
        AccelerationStructureGeometryAabbsDataKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureGeometryAabbsDataKHR.html) · Builder of [`AccelerationStructureGeometryAabbsDataKHR`]"]
#[repr(transparent)]
pub struct AccelerationStructureGeometryAabbsDataKHRBuilder<'a>(AccelerationStructureGeometryAabbsDataKHR, std::marker::PhantomData<&'a ()>);
impl<'a> AccelerationStructureGeometryAabbsDataKHRBuilder<'a> {
    #[inline]
    pub fn new() -> AccelerationStructureGeometryAabbsDataKHRBuilder<'a> {
        AccelerationStructureGeometryAabbsDataKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn data(mut self, data: crate::extensions::khr_acceleration_structure::DeviceOrHostAddressConstKHR) -> Self {
        self.0.data = data as _;
        self
    }
    #[inline]
    pub fn stride(mut self, stride: crate::vk1_0::DeviceSize) -> Self {
        self.0.stride = stride as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> AccelerationStructureGeometryAabbsDataKHR {
        self.0
    }
}
impl<'a> std::default::Default for AccelerationStructureGeometryAabbsDataKHRBuilder<'a> {
    fn default() -> AccelerationStructureGeometryAabbsDataKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for AccelerationStructureGeometryAabbsDataKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for AccelerationStructureGeometryAabbsDataKHRBuilder<'a> {
    type Target = AccelerationStructureGeometryAabbsDataKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for AccelerationStructureGeometryAabbsDataKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureGeometryInstancesDataKHR.html) · Structure"]
#[doc(alias = "VkAccelerationStructureGeometryInstancesDataKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AccelerationStructureGeometryInstancesDataKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub array_of_pointers: crate::vk1_0::Bool32,
    pub data: crate::extensions::khr_acceleration_structure::DeviceOrHostAddressConstKHR,
}
impl AccelerationStructureGeometryInstancesDataKHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::ACCELERATION_STRUCTURE_GEOMETRY_INSTANCES_DATA_KHR;
}
impl Default for AccelerationStructureGeometryInstancesDataKHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), array_of_pointers: Default::default(), data: Default::default() }
    }
}
impl std::fmt::Debug for AccelerationStructureGeometryInstancesDataKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AccelerationStructureGeometryInstancesDataKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("array_of_pointers", &(self.array_of_pointers != 0)).field("data", &self.data).finish()
    }
}
impl AccelerationStructureGeometryInstancesDataKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> AccelerationStructureGeometryInstancesDataKHRBuilder<'a> {
        AccelerationStructureGeometryInstancesDataKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureGeometryInstancesDataKHR.html) · Builder of [`AccelerationStructureGeometryInstancesDataKHR`]"]
#[repr(transparent)]
pub struct AccelerationStructureGeometryInstancesDataKHRBuilder<'a>(AccelerationStructureGeometryInstancesDataKHR, std::marker::PhantomData<&'a ()>);
impl<'a> AccelerationStructureGeometryInstancesDataKHRBuilder<'a> {
    #[inline]
    pub fn new() -> AccelerationStructureGeometryInstancesDataKHRBuilder<'a> {
        AccelerationStructureGeometryInstancesDataKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn array_of_pointers(mut self, array_of_pointers: bool) -> Self {
        self.0.array_of_pointers = array_of_pointers as _;
        self
    }
    #[inline]
    pub fn data(mut self, data: crate::extensions::khr_acceleration_structure::DeviceOrHostAddressConstKHR) -> Self {
        self.0.data = data as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> AccelerationStructureGeometryInstancesDataKHR {
        self.0
    }
}
impl<'a> std::default::Default for AccelerationStructureGeometryInstancesDataKHRBuilder<'a> {
    fn default() -> AccelerationStructureGeometryInstancesDataKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for AccelerationStructureGeometryInstancesDataKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for AccelerationStructureGeometryInstancesDataKHRBuilder<'a> {
    type Target = AccelerationStructureGeometryInstancesDataKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for AccelerationStructureGeometryInstancesDataKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureGeometryDataKHR.html) · Structure"]
#[doc(alias = "VkAccelerationStructureGeometryDataKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub union AccelerationStructureGeometryDataKHR {
    pub triangles: crate::extensions::khr_acceleration_structure::AccelerationStructureGeometryTrianglesDataKHR,
    pub aabbs: crate::extensions::khr_acceleration_structure::AccelerationStructureGeometryAabbsDataKHR,
    pub instances: crate::extensions::khr_acceleration_structure::AccelerationStructureGeometryInstancesDataKHR,
}
impl Default for AccelerationStructureGeometryDataKHR {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for AccelerationStructureGeometryDataKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AccelerationStructureGeometryDataKHR").finish()
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureGeometryKHR.html) · Structure"]
#[doc(alias = "VkAccelerationStructureGeometryKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AccelerationStructureGeometryKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub geometry_type: crate::extensions::khr_acceleration_structure::GeometryTypeKHR,
    pub geometry: crate::extensions::khr_acceleration_structure::AccelerationStructureGeometryDataKHR,
    pub flags: crate::extensions::khr_acceleration_structure::GeometryFlagsKHR,
}
impl AccelerationStructureGeometryKHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::ACCELERATION_STRUCTURE_GEOMETRY_KHR;
}
impl Default for AccelerationStructureGeometryKHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), geometry_type: Default::default(), geometry: Default::default(), flags: Default::default() }
    }
}
impl std::fmt::Debug for AccelerationStructureGeometryKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AccelerationStructureGeometryKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("geometry_type", &self.geometry_type).field("geometry", &self.geometry).field("flags", &self.flags).finish()
    }
}
impl AccelerationStructureGeometryKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> AccelerationStructureGeometryKHRBuilder<'a> {
        AccelerationStructureGeometryKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureGeometryKHR.html) · Builder of [`AccelerationStructureGeometryKHR`]"]
#[repr(transparent)]
pub struct AccelerationStructureGeometryKHRBuilder<'a>(AccelerationStructureGeometryKHR, std::marker::PhantomData<&'a ()>);
impl<'a> AccelerationStructureGeometryKHRBuilder<'a> {
    #[inline]
    pub fn new() -> AccelerationStructureGeometryKHRBuilder<'a> {
        AccelerationStructureGeometryKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn geometry_type(mut self, geometry_type: crate::extensions::khr_acceleration_structure::GeometryTypeKHR) -> Self {
        self.0.geometry_type = geometry_type as _;
        self
    }
    #[inline]
    pub fn geometry(mut self, geometry: crate::extensions::khr_acceleration_structure::AccelerationStructureGeometryDataKHR) -> Self {
        self.0.geometry = geometry as _;
        self
    }
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::khr_acceleration_structure::GeometryFlagsKHR) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> AccelerationStructureGeometryKHR {
        self.0
    }
}
impl<'a> std::default::Default for AccelerationStructureGeometryKHRBuilder<'a> {
    fn default() -> AccelerationStructureGeometryKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for AccelerationStructureGeometryKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for AccelerationStructureGeometryKHRBuilder<'a> {
    type Target = AccelerationStructureGeometryKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for AccelerationStructureGeometryKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureBuildGeometryInfoKHR.html) · Structure"]
#[doc(alias = "VkAccelerationStructureBuildGeometryInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AccelerationStructureBuildGeometryInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub _type: crate::extensions::khr_acceleration_structure::AccelerationStructureTypeKHR,
    pub flags: crate::extensions::khr_acceleration_structure::BuildAccelerationStructureFlagsKHR,
    pub mode: crate::extensions::khr_acceleration_structure::BuildAccelerationStructureModeKHR,
    pub src_acceleration_structure: crate::extensions::khr_acceleration_structure::AccelerationStructureKHR,
    pub dst_acceleration_structure: crate::extensions::khr_acceleration_structure::AccelerationStructureKHR,
    pub geometry_count: u32,
    pub p_geometries: *const crate::extensions::khr_acceleration_structure::AccelerationStructureGeometryKHR,
    pub pp_geometries: *const *const crate::extensions::khr_acceleration_structure::AccelerationStructureGeometryKHR,
    pub scratch_data: crate::extensions::khr_acceleration_structure::DeviceOrHostAddressKHR,
}
impl AccelerationStructureBuildGeometryInfoKHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::ACCELERATION_STRUCTURE_BUILD_GEOMETRY_INFO_KHR;
}
impl Default for AccelerationStructureBuildGeometryInfoKHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), _type: Default::default(), flags: Default::default(), mode: Default::default(), src_acceleration_structure: Default::default(), dst_acceleration_structure: Default::default(), geometry_count: Default::default(), p_geometries: std::ptr::null(), pp_geometries: std::ptr::null(), scratch_data: Default::default() }
    }
}
impl std::fmt::Debug for AccelerationStructureBuildGeometryInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AccelerationStructureBuildGeometryInfoKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("_type", &self._type).field("flags", &self.flags).field("mode", &self.mode).field("src_acceleration_structure", &self.src_acceleration_structure).field("dst_acceleration_structure", &self.dst_acceleration_structure).field("geometry_count", &self.geometry_count).field("p_geometries", &self.p_geometries).field("pp_geometries", &self.pp_geometries).field("scratch_data", &self.scratch_data).finish()
    }
}
impl AccelerationStructureBuildGeometryInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> AccelerationStructureBuildGeometryInfoKHRBuilder<'a> {
        AccelerationStructureBuildGeometryInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureBuildGeometryInfoKHR.html) · Builder of [`AccelerationStructureBuildGeometryInfoKHR`]"]
#[repr(transparent)]
pub struct AccelerationStructureBuildGeometryInfoKHRBuilder<'a>(AccelerationStructureBuildGeometryInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> AccelerationStructureBuildGeometryInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> AccelerationStructureBuildGeometryInfoKHRBuilder<'a> {
        AccelerationStructureBuildGeometryInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn _type(mut self, _type: crate::extensions::khr_acceleration_structure::AccelerationStructureTypeKHR) -> Self {
        self.0._type = _type as _;
        self
    }
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::khr_acceleration_structure::BuildAccelerationStructureFlagsKHR) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn mode(mut self, mode: crate::extensions::khr_acceleration_structure::BuildAccelerationStructureModeKHR) -> Self {
        self.0.mode = mode as _;
        self
    }
    #[inline]
    pub fn src_acceleration_structure(mut self, src_acceleration_structure: crate::extensions::khr_acceleration_structure::AccelerationStructureKHR) -> Self {
        self.0.src_acceleration_structure = src_acceleration_structure as _;
        self
    }
    #[inline]
    pub fn dst_acceleration_structure(mut self, dst_acceleration_structure: crate::extensions::khr_acceleration_structure::AccelerationStructureKHR) -> Self {
        self.0.dst_acceleration_structure = dst_acceleration_structure as _;
        self
    }
    #[inline]
    pub fn geometries(mut self, geometries: &'a [crate::extensions::khr_acceleration_structure::AccelerationStructureGeometryKHRBuilder]) -> Self {
        self.0.p_geometries = geometries.as_ptr() as _;
        self.0.geometry_count = geometries.len() as _;
        self
    }
    #[inline]
    pub fn scratch_data(mut self, scratch_data: crate::extensions::khr_acceleration_structure::DeviceOrHostAddressKHR) -> Self {
        self.0.scratch_data = scratch_data as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> AccelerationStructureBuildGeometryInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for AccelerationStructureBuildGeometryInfoKHRBuilder<'a> {
    fn default() -> AccelerationStructureBuildGeometryInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for AccelerationStructureBuildGeometryInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for AccelerationStructureBuildGeometryInfoKHRBuilder<'a> {
    type Target = AccelerationStructureBuildGeometryInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for AccelerationStructureBuildGeometryInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureBuildRangeInfoKHR.html) · Structure"]
#[doc(alias = "VkAccelerationStructureBuildRangeInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AccelerationStructureBuildRangeInfoKHR {
    pub primitive_count: u32,
    pub primitive_offset: u32,
    pub first_vertex: u32,
    pub transform_offset: u32,
}
impl Default for AccelerationStructureBuildRangeInfoKHR {
    fn default() -> Self {
        Self { primitive_count: Default::default(), primitive_offset: Default::default(), first_vertex: Default::default(), transform_offset: Default::default() }
    }
}
impl std::fmt::Debug for AccelerationStructureBuildRangeInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AccelerationStructureBuildRangeInfoKHR").field("primitive_count", &self.primitive_count).field("primitive_offset", &self.primitive_offset).field("first_vertex", &self.first_vertex).field("transform_offset", &self.transform_offset).finish()
    }
}
impl AccelerationStructureBuildRangeInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> AccelerationStructureBuildRangeInfoKHRBuilder<'a> {
        AccelerationStructureBuildRangeInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureBuildRangeInfoKHR.html) · Builder of [`AccelerationStructureBuildRangeInfoKHR`]"]
#[repr(transparent)]
pub struct AccelerationStructureBuildRangeInfoKHRBuilder<'a>(AccelerationStructureBuildRangeInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> AccelerationStructureBuildRangeInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> AccelerationStructureBuildRangeInfoKHRBuilder<'a> {
        AccelerationStructureBuildRangeInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn primitive_count(mut self, primitive_count: u32) -> Self {
        self.0.primitive_count = primitive_count as _;
        self
    }
    #[inline]
    pub fn primitive_offset(mut self, primitive_offset: u32) -> Self {
        self.0.primitive_offset = primitive_offset as _;
        self
    }
    #[inline]
    pub fn first_vertex(mut self, first_vertex: u32) -> Self {
        self.0.first_vertex = first_vertex as _;
        self
    }
    #[inline]
    pub fn transform_offset(mut self, transform_offset: u32) -> Self {
        self.0.transform_offset = transform_offset as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> AccelerationStructureBuildRangeInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for AccelerationStructureBuildRangeInfoKHRBuilder<'a> {
    fn default() -> AccelerationStructureBuildRangeInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for AccelerationStructureBuildRangeInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for AccelerationStructureBuildRangeInfoKHRBuilder<'a> {
    type Target = AccelerationStructureBuildRangeInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for AccelerationStructureBuildRangeInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureCreateInfoKHR.html) · Structure"]
#[doc(alias = "VkAccelerationStructureCreateInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AccelerationStructureCreateInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub create_flags: crate::extensions::khr_acceleration_structure::AccelerationStructureCreateFlagsKHR,
    pub buffer: crate::vk1_0::Buffer,
    pub offset: crate::vk1_0::DeviceSize,
    pub size: crate::vk1_0::DeviceSize,
    pub _type: crate::extensions::khr_acceleration_structure::AccelerationStructureTypeKHR,
    pub device_address: crate::vk1_0::DeviceAddress,
}
impl AccelerationStructureCreateInfoKHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::ACCELERATION_STRUCTURE_CREATE_INFO_KHR;
}
impl Default for AccelerationStructureCreateInfoKHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), create_flags: Default::default(), buffer: Default::default(), offset: Default::default(), size: Default::default(), _type: Default::default(), device_address: Default::default() }
    }
}
impl std::fmt::Debug for AccelerationStructureCreateInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AccelerationStructureCreateInfoKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("create_flags", &self.create_flags).field("buffer", &self.buffer).field("offset", &self.offset).field("size", &self.size).field("_type", &self._type).field("device_address", &self.device_address).finish()
    }
}
impl AccelerationStructureCreateInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> AccelerationStructureCreateInfoKHRBuilder<'a> {
        AccelerationStructureCreateInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureCreateInfoKHR.html) · Builder of [`AccelerationStructureCreateInfoKHR`]"]
#[repr(transparent)]
pub struct AccelerationStructureCreateInfoKHRBuilder<'a>(AccelerationStructureCreateInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> AccelerationStructureCreateInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> AccelerationStructureCreateInfoKHRBuilder<'a> {
        AccelerationStructureCreateInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn create_flags(mut self, create_flags: crate::extensions::khr_acceleration_structure::AccelerationStructureCreateFlagsKHR) -> Self {
        self.0.create_flags = create_flags as _;
        self
    }
    #[inline]
    pub fn buffer(mut self, buffer: crate::vk1_0::Buffer) -> Self {
        self.0.buffer = buffer as _;
        self
    }
    #[inline]
    pub fn offset(mut self, offset: crate::vk1_0::DeviceSize) -> Self {
        self.0.offset = offset as _;
        self
    }
    #[inline]
    pub fn size(mut self, size: crate::vk1_0::DeviceSize) -> Self {
        self.0.size = size as _;
        self
    }
    #[inline]
    pub fn _type(mut self, _type: crate::extensions::khr_acceleration_structure::AccelerationStructureTypeKHR) -> Self {
        self.0._type = _type as _;
        self
    }
    #[inline]
    pub fn device_address(mut self, device_address: crate::vk1_0::DeviceAddress) -> Self {
        self.0.device_address = device_address as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> AccelerationStructureCreateInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for AccelerationStructureCreateInfoKHRBuilder<'a> {
    fn default() -> AccelerationStructureCreateInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for AccelerationStructureCreateInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for AccelerationStructureCreateInfoKHRBuilder<'a> {
    type Target = AccelerationStructureCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for AccelerationStructureCreateInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAabbPositionsKHR.html) · Structure"]
#[doc(alias = "VkAabbPositionsKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AabbPositionsKHR {
    pub min_x: std::os::raw::c_float,
    pub min_y: std::os::raw::c_float,
    pub min_z: std::os::raw::c_float,
    pub max_x: std::os::raw::c_float,
    pub max_y: std::os::raw::c_float,
    pub max_z: std::os::raw::c_float,
}
impl Default for AabbPositionsKHR {
    fn default() -> Self {
        Self { min_x: Default::default(), min_y: Default::default(), min_z: Default::default(), max_x: Default::default(), max_y: Default::default(), max_z: Default::default() }
    }
}
impl std::fmt::Debug for AabbPositionsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AabbPositionsKHR").field("min_x", &self.min_x).field("min_y", &self.min_y).field("min_z", &self.min_z).field("max_x", &self.max_x).field("max_y", &self.max_y).field("max_z", &self.max_z).finish()
    }
}
impl AabbPositionsKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> AabbPositionsKHRBuilder<'a> {
        AabbPositionsKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAabbPositionsKHR.html) · Builder of [`AabbPositionsKHR`]"]
#[repr(transparent)]
pub struct AabbPositionsKHRBuilder<'a>(AabbPositionsKHR, std::marker::PhantomData<&'a ()>);
impl<'a> AabbPositionsKHRBuilder<'a> {
    #[inline]
    pub fn new() -> AabbPositionsKHRBuilder<'a> {
        AabbPositionsKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn min_x(mut self, min_x: std::os::raw::c_float) -> Self {
        self.0.min_x = min_x as _;
        self
    }
    #[inline]
    pub fn min_y(mut self, min_y: std::os::raw::c_float) -> Self {
        self.0.min_y = min_y as _;
        self
    }
    #[inline]
    pub fn min_z(mut self, min_z: std::os::raw::c_float) -> Self {
        self.0.min_z = min_z as _;
        self
    }
    #[inline]
    pub fn max_x(mut self, max_x: std::os::raw::c_float) -> Self {
        self.0.max_x = max_x as _;
        self
    }
    #[inline]
    pub fn max_y(mut self, max_y: std::os::raw::c_float) -> Self {
        self.0.max_y = max_y as _;
        self
    }
    #[inline]
    pub fn max_z(mut self, max_z: std::os::raw::c_float) -> Self {
        self.0.max_z = max_z as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> AabbPositionsKHR {
        self.0
    }
}
impl<'a> std::default::Default for AabbPositionsKHRBuilder<'a> {
    fn default() -> AabbPositionsKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for AabbPositionsKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for AabbPositionsKHRBuilder<'a> {
    type Target = AabbPositionsKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for AabbPositionsKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkTransformMatrixKHR.html) · Structure"]
#[doc(alias = "VkTransformMatrixKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TransformMatrixKHR {
    pub matrix: [[std::os::raw::c_float; 4]; 3],
}
impl Default for TransformMatrixKHR {
    fn default() -> Self {
        Self { matrix: unsafe { std::mem::zeroed() } }
    }
}
impl std::fmt::Debug for TransformMatrixKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TransformMatrixKHR").field("matrix", &self.matrix).finish()
    }
}
impl TransformMatrixKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> TransformMatrixKHRBuilder<'a> {
        TransformMatrixKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkTransformMatrixKHR.html) · Builder of [`TransformMatrixKHR`]"]
#[repr(transparent)]
pub struct TransformMatrixKHRBuilder<'a>(TransformMatrixKHR, std::marker::PhantomData<&'a ()>);
impl<'a> TransformMatrixKHRBuilder<'a> {
    #[inline]
    pub fn new() -> TransformMatrixKHRBuilder<'a> {
        TransformMatrixKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn matrix(mut self, matrix: [[std::os::raw::c_float; 4]; 3]) -> Self {
        self.0.matrix = matrix as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> TransformMatrixKHR {
        self.0
    }
}
impl<'a> std::default::Default for TransformMatrixKHRBuilder<'a> {
    fn default() -> TransformMatrixKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for TransformMatrixKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for TransformMatrixKHRBuilder<'a> {
    type Target = TransformMatrixKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for TransformMatrixKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureInstanceKHR.html) · Structure"]
#[doc(alias = "VkAccelerationStructureInstanceKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AccelerationStructureInstanceKHR {
    pub transform: crate::extensions::khr_acceleration_structure::TransformMatrixKHR,
    pub instance_custom_index_and_mask: u32,
    pub instance_shader_binding_table_record_offset_and_flags: u32,
    pub acceleration_structure_reference: u64,
}
impl Default for AccelerationStructureInstanceKHR {
    fn default() -> Self {
        Self { transform: Default::default(), instance_custom_index_and_mask: Default::default(), instance_shader_binding_table_record_offset_and_flags: Default::default(), acceleration_structure_reference: Default::default() }
    }
}
impl std::fmt::Debug for AccelerationStructureInstanceKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AccelerationStructureInstanceKHR").field("transform", &self.transform).field("instance_custom_index_and_mask", &format!("{:#b}", &self.instance_custom_index_and_mask)).field("instance_shader_binding_table_record_offset_and_flags", &format!("{:#b}", &self.instance_shader_binding_table_record_offset_and_flags)).field("acceleration_structure_reference", &self.acceleration_structure_reference).finish()
    }
}
impl AccelerationStructureInstanceKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> AccelerationStructureInstanceKHRBuilder<'a> {
        AccelerationStructureInstanceKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureInstanceKHR.html) · Builder of [`AccelerationStructureInstanceKHR`]"]
#[repr(transparent)]
pub struct AccelerationStructureInstanceKHRBuilder<'a>(AccelerationStructureInstanceKHR, std::marker::PhantomData<&'a ()>);
impl<'a> AccelerationStructureInstanceKHRBuilder<'a> {
    #[inline]
    pub fn new() -> AccelerationStructureInstanceKHRBuilder<'a> {
        AccelerationStructureInstanceKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn transform(mut self, transform: crate::extensions::khr_acceleration_structure::TransformMatrixKHR) -> Self {
        self.0.transform = transform as _;
        self
    }
    #[inline]
    pub fn instance_custom_index(mut self, instance_custom_index: u32) -> Self {
        self.0.instance_custom_index_and_mask = crate::bits_copy!(self.0.instance_custom_index_and_mask, instance_custom_index, 0usize, 23usize);
        self
    }
    #[inline]
    pub fn mask(mut self, mask: u32) -> Self {
        self.0.instance_custom_index_and_mask = crate::bits_copy!(self.0.instance_custom_index_and_mask, mask, 24usize, 31usize);
        self
    }
    #[inline]
    pub fn instance_shader_binding_table_record_offset(mut self, instance_shader_binding_table_record_offset: u32) -> Self {
        self.0.instance_shader_binding_table_record_offset_and_flags = crate::bits_copy!(self.0.instance_shader_binding_table_record_offset_and_flags, instance_shader_binding_table_record_offset, 0usize, 23usize);
        self
    }
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::khr_acceleration_structure::GeometryInstanceFlagsKHR) -> Self {
        self.0.instance_shader_binding_table_record_offset_and_flags = crate::bits_copy!(self.0.instance_shader_binding_table_record_offset_and_flags, flags.bits(), 24usize, 31usize);
        self
    }
    #[inline]
    pub fn acceleration_structure_reference(mut self, acceleration_structure_reference: u64) -> Self {
        self.0.acceleration_structure_reference = acceleration_structure_reference as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> AccelerationStructureInstanceKHR {
        self.0
    }
}
impl<'a> std::default::Default for AccelerationStructureInstanceKHRBuilder<'a> {
    fn default() -> AccelerationStructureInstanceKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for AccelerationStructureInstanceKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for AccelerationStructureInstanceKHRBuilder<'a> {
    type Target = AccelerationStructureInstanceKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for AccelerationStructureInstanceKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureDeviceAddressInfoKHR.html) · Structure"]
#[doc(alias = "VkAccelerationStructureDeviceAddressInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AccelerationStructureDeviceAddressInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub acceleration_structure: crate::extensions::khr_acceleration_structure::AccelerationStructureKHR,
}
impl AccelerationStructureDeviceAddressInfoKHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::ACCELERATION_STRUCTURE_DEVICE_ADDRESS_INFO_KHR;
}
impl Default for AccelerationStructureDeviceAddressInfoKHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), acceleration_structure: Default::default() }
    }
}
impl std::fmt::Debug for AccelerationStructureDeviceAddressInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AccelerationStructureDeviceAddressInfoKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("acceleration_structure", &self.acceleration_structure).finish()
    }
}
impl AccelerationStructureDeviceAddressInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> AccelerationStructureDeviceAddressInfoKHRBuilder<'a> {
        AccelerationStructureDeviceAddressInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureDeviceAddressInfoKHR.html) · Builder of [`AccelerationStructureDeviceAddressInfoKHR`]"]
#[repr(transparent)]
pub struct AccelerationStructureDeviceAddressInfoKHRBuilder<'a>(AccelerationStructureDeviceAddressInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> AccelerationStructureDeviceAddressInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> AccelerationStructureDeviceAddressInfoKHRBuilder<'a> {
        AccelerationStructureDeviceAddressInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn acceleration_structure(mut self, acceleration_structure: crate::extensions::khr_acceleration_structure::AccelerationStructureKHR) -> Self {
        self.0.acceleration_structure = acceleration_structure as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> AccelerationStructureDeviceAddressInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for AccelerationStructureDeviceAddressInfoKHRBuilder<'a> {
    fn default() -> AccelerationStructureDeviceAddressInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for AccelerationStructureDeviceAddressInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for AccelerationStructureDeviceAddressInfoKHRBuilder<'a> {
    type Target = AccelerationStructureDeviceAddressInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for AccelerationStructureDeviceAddressInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureVersionInfoKHR.html) · Structure"]
#[doc(alias = "VkAccelerationStructureVersionInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AccelerationStructureVersionInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub p_version_data: *const u8,
}
impl AccelerationStructureVersionInfoKHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::ACCELERATION_STRUCTURE_VERSION_INFO_KHR;
}
impl Default for AccelerationStructureVersionInfoKHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), p_version_data: std::ptr::null() }
    }
}
impl std::fmt::Debug for AccelerationStructureVersionInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AccelerationStructureVersionInfoKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("p_version_data", &self.p_version_data).finish()
    }
}
impl AccelerationStructureVersionInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> AccelerationStructureVersionInfoKHRBuilder<'a> {
        AccelerationStructureVersionInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureVersionInfoKHR.html) · Builder of [`AccelerationStructureVersionInfoKHR`]"]
#[repr(transparent)]
pub struct AccelerationStructureVersionInfoKHRBuilder<'a>(AccelerationStructureVersionInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> AccelerationStructureVersionInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> AccelerationStructureVersionInfoKHRBuilder<'a> {
        AccelerationStructureVersionInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn version_data(mut self, version_data: &'a [u8; 32]) -> Self {
        self.0.p_version_data = version_data.as_ptr() as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> AccelerationStructureVersionInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for AccelerationStructureVersionInfoKHRBuilder<'a> {
    fn default() -> AccelerationStructureVersionInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for AccelerationStructureVersionInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for AccelerationStructureVersionInfoKHRBuilder<'a> {
    type Target = AccelerationStructureVersionInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for AccelerationStructureVersionInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCopyAccelerationStructureInfoKHR.html) · Structure"]
#[doc(alias = "VkCopyAccelerationStructureInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CopyAccelerationStructureInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub src: crate::extensions::khr_acceleration_structure::AccelerationStructureKHR,
    pub dst: crate::extensions::khr_acceleration_structure::AccelerationStructureKHR,
    pub mode: crate::extensions::khr_acceleration_structure::CopyAccelerationStructureModeKHR,
}
impl CopyAccelerationStructureInfoKHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::COPY_ACCELERATION_STRUCTURE_INFO_KHR;
}
impl Default for CopyAccelerationStructureInfoKHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), src: Default::default(), dst: Default::default(), mode: Default::default() }
    }
}
impl std::fmt::Debug for CopyAccelerationStructureInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CopyAccelerationStructureInfoKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("src", &self.src).field("dst", &self.dst).field("mode", &self.mode).finish()
    }
}
impl CopyAccelerationStructureInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> CopyAccelerationStructureInfoKHRBuilder<'a> {
        CopyAccelerationStructureInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCopyAccelerationStructureInfoKHR.html) · Builder of [`CopyAccelerationStructureInfoKHR`]"]
#[repr(transparent)]
pub struct CopyAccelerationStructureInfoKHRBuilder<'a>(CopyAccelerationStructureInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> CopyAccelerationStructureInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> CopyAccelerationStructureInfoKHRBuilder<'a> {
        CopyAccelerationStructureInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn src(mut self, src: crate::extensions::khr_acceleration_structure::AccelerationStructureKHR) -> Self {
        self.0.src = src as _;
        self
    }
    #[inline]
    pub fn dst(mut self, dst: crate::extensions::khr_acceleration_structure::AccelerationStructureKHR) -> Self {
        self.0.dst = dst as _;
        self
    }
    #[inline]
    pub fn mode(mut self, mode: crate::extensions::khr_acceleration_structure::CopyAccelerationStructureModeKHR) -> Self {
        self.0.mode = mode as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> CopyAccelerationStructureInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for CopyAccelerationStructureInfoKHRBuilder<'a> {
    fn default() -> CopyAccelerationStructureInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for CopyAccelerationStructureInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for CopyAccelerationStructureInfoKHRBuilder<'a> {
    type Target = CopyAccelerationStructureInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for CopyAccelerationStructureInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCopyAccelerationStructureToMemoryInfoKHR.html) · Structure"]
#[doc(alias = "VkCopyAccelerationStructureToMemoryInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CopyAccelerationStructureToMemoryInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub src: crate::extensions::khr_acceleration_structure::AccelerationStructureKHR,
    pub dst: crate::extensions::khr_acceleration_structure::DeviceOrHostAddressKHR,
    pub mode: crate::extensions::khr_acceleration_structure::CopyAccelerationStructureModeKHR,
}
impl CopyAccelerationStructureToMemoryInfoKHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::COPY_ACCELERATION_STRUCTURE_TO_MEMORY_INFO_KHR;
}
impl Default for CopyAccelerationStructureToMemoryInfoKHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), src: Default::default(), dst: Default::default(), mode: Default::default() }
    }
}
impl std::fmt::Debug for CopyAccelerationStructureToMemoryInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CopyAccelerationStructureToMemoryInfoKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("src", &self.src).field("dst", &self.dst).field("mode", &self.mode).finish()
    }
}
impl CopyAccelerationStructureToMemoryInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> CopyAccelerationStructureToMemoryInfoKHRBuilder<'a> {
        CopyAccelerationStructureToMemoryInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCopyAccelerationStructureToMemoryInfoKHR.html) · Builder of [`CopyAccelerationStructureToMemoryInfoKHR`]"]
#[repr(transparent)]
pub struct CopyAccelerationStructureToMemoryInfoKHRBuilder<'a>(CopyAccelerationStructureToMemoryInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> CopyAccelerationStructureToMemoryInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> CopyAccelerationStructureToMemoryInfoKHRBuilder<'a> {
        CopyAccelerationStructureToMemoryInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn src(mut self, src: crate::extensions::khr_acceleration_structure::AccelerationStructureKHR) -> Self {
        self.0.src = src as _;
        self
    }
    #[inline]
    pub fn dst(mut self, dst: crate::extensions::khr_acceleration_structure::DeviceOrHostAddressKHR) -> Self {
        self.0.dst = dst as _;
        self
    }
    #[inline]
    pub fn mode(mut self, mode: crate::extensions::khr_acceleration_structure::CopyAccelerationStructureModeKHR) -> Self {
        self.0.mode = mode as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> CopyAccelerationStructureToMemoryInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for CopyAccelerationStructureToMemoryInfoKHRBuilder<'a> {
    fn default() -> CopyAccelerationStructureToMemoryInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for CopyAccelerationStructureToMemoryInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for CopyAccelerationStructureToMemoryInfoKHRBuilder<'a> {
    type Target = CopyAccelerationStructureToMemoryInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for CopyAccelerationStructureToMemoryInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCopyMemoryToAccelerationStructureInfoKHR.html) · Structure"]
#[doc(alias = "VkCopyMemoryToAccelerationStructureInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CopyMemoryToAccelerationStructureInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub src: crate::extensions::khr_acceleration_structure::DeviceOrHostAddressConstKHR,
    pub dst: crate::extensions::khr_acceleration_structure::AccelerationStructureKHR,
    pub mode: crate::extensions::khr_acceleration_structure::CopyAccelerationStructureModeKHR,
}
impl CopyMemoryToAccelerationStructureInfoKHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::COPY_MEMORY_TO_ACCELERATION_STRUCTURE_INFO_KHR;
}
impl Default for CopyMemoryToAccelerationStructureInfoKHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), src: Default::default(), dst: Default::default(), mode: Default::default() }
    }
}
impl std::fmt::Debug for CopyMemoryToAccelerationStructureInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CopyMemoryToAccelerationStructureInfoKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("src", &self.src).field("dst", &self.dst).field("mode", &self.mode).finish()
    }
}
impl CopyMemoryToAccelerationStructureInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> CopyMemoryToAccelerationStructureInfoKHRBuilder<'a> {
        CopyMemoryToAccelerationStructureInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCopyMemoryToAccelerationStructureInfoKHR.html) · Builder of [`CopyMemoryToAccelerationStructureInfoKHR`]"]
#[repr(transparent)]
pub struct CopyMemoryToAccelerationStructureInfoKHRBuilder<'a>(CopyMemoryToAccelerationStructureInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> CopyMemoryToAccelerationStructureInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> CopyMemoryToAccelerationStructureInfoKHRBuilder<'a> {
        CopyMemoryToAccelerationStructureInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn src(mut self, src: crate::extensions::khr_acceleration_structure::DeviceOrHostAddressConstKHR) -> Self {
        self.0.src = src as _;
        self
    }
    #[inline]
    pub fn dst(mut self, dst: crate::extensions::khr_acceleration_structure::AccelerationStructureKHR) -> Self {
        self.0.dst = dst as _;
        self
    }
    #[inline]
    pub fn mode(mut self, mode: crate::extensions::khr_acceleration_structure::CopyAccelerationStructureModeKHR) -> Self {
        self.0.mode = mode as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> CopyMemoryToAccelerationStructureInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for CopyMemoryToAccelerationStructureInfoKHRBuilder<'a> {
    fn default() -> CopyMemoryToAccelerationStructureInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for CopyMemoryToAccelerationStructureInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for CopyMemoryToAccelerationStructureInfoKHRBuilder<'a> {
    type Target = CopyMemoryToAccelerationStructureInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for CopyMemoryToAccelerationStructureInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureBuildSizesInfoKHR.html) · Structure"]
#[doc(alias = "VkAccelerationStructureBuildSizesInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AccelerationStructureBuildSizesInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub acceleration_structure_size: crate::vk1_0::DeviceSize,
    pub update_scratch_size: crate::vk1_0::DeviceSize,
    pub build_scratch_size: crate::vk1_0::DeviceSize,
}
impl AccelerationStructureBuildSizesInfoKHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::ACCELERATION_STRUCTURE_BUILD_SIZES_INFO_KHR;
}
impl Default for AccelerationStructureBuildSizesInfoKHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), acceleration_structure_size: Default::default(), update_scratch_size: Default::default(), build_scratch_size: Default::default() }
    }
}
impl std::fmt::Debug for AccelerationStructureBuildSizesInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AccelerationStructureBuildSizesInfoKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("acceleration_structure_size", &self.acceleration_structure_size).field("update_scratch_size", &self.update_scratch_size).field("build_scratch_size", &self.build_scratch_size).finish()
    }
}
impl AccelerationStructureBuildSizesInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> AccelerationStructureBuildSizesInfoKHRBuilder<'a> {
        AccelerationStructureBuildSizesInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureBuildSizesInfoKHR.html) · Builder of [`AccelerationStructureBuildSizesInfoKHR`]"]
#[repr(transparent)]
pub struct AccelerationStructureBuildSizesInfoKHRBuilder<'a>(AccelerationStructureBuildSizesInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> AccelerationStructureBuildSizesInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> AccelerationStructureBuildSizesInfoKHRBuilder<'a> {
        AccelerationStructureBuildSizesInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn acceleration_structure_size(mut self, acceleration_structure_size: crate::vk1_0::DeviceSize) -> Self {
        self.0.acceleration_structure_size = acceleration_structure_size as _;
        self
    }
    #[inline]
    pub fn update_scratch_size(mut self, update_scratch_size: crate::vk1_0::DeviceSize) -> Self {
        self.0.update_scratch_size = update_scratch_size as _;
        self
    }
    #[inline]
    pub fn build_scratch_size(mut self, build_scratch_size: crate::vk1_0::DeviceSize) -> Self {
        self.0.build_scratch_size = build_scratch_size as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> AccelerationStructureBuildSizesInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for AccelerationStructureBuildSizesInfoKHRBuilder<'a> {
    fn default() -> AccelerationStructureBuildSizesInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for AccelerationStructureBuildSizesInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for AccelerationStructureBuildSizesInfoKHRBuilder<'a> {
    type Target = AccelerationStructureBuildSizesInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for AccelerationStructureBuildSizesInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::khr_acceleration_structure`]"]
impl crate::DeviceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyAccelerationStructureKHR.html) · Function"]
    #[doc(alias = "vkDestroyAccelerationStructureKHR")]
    pub unsafe fn destroy_acceleration_structure_khr(&self, acceleration_structure: Option<crate::extensions::khr_acceleration_structure::AccelerationStructureKHR>, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> () {
        let _function = self.destroy_acceleration_structure_khr.expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(
            self.handle,
            match acceleration_structure {
                Some(v) => v,
                None => Default::default(),
            },
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyAccelerationStructureKHR.html) · Function"]
    #[doc(alias = "vkCmdCopyAccelerationStructureKHR")]
    pub unsafe fn cmd_copy_acceleration_structure_khr(&self, command_buffer: crate::vk1_0::CommandBuffer, info: &crate::extensions::khr_acceleration_structure::CopyAccelerationStructureInfoKHR) -> () {
        let _function = self.cmd_copy_acceleration_structure_khr.expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(command_buffer as _, info as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCopyAccelerationStructureKHR.html) · Function"]
    #[doc(alias = "vkCopyAccelerationStructureKHR")]
    pub unsafe fn copy_acceleration_structure_khr(&self, deferred_operation: Option<crate::extensions::khr_deferred_host_operations::DeferredOperationKHR>, info: &crate::extensions::khr_acceleration_structure::CopyAccelerationStructureInfoKHR) -> crate::utils::VulkanResult<()> {
        let _function = self.copy_acceleration_structure_khr.expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(
            self.handle,
            match deferred_operation {
                Some(v) => v,
                None => Default::default(),
            },
            info as _,
        );
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyAccelerationStructureToMemoryKHR.html) · Function"]
    #[doc(alias = "vkCmdCopyAccelerationStructureToMemoryKHR")]
    pub unsafe fn cmd_copy_acceleration_structure_to_memory_khr(&self, command_buffer: crate::vk1_0::CommandBuffer, info: &crate::extensions::khr_acceleration_structure::CopyAccelerationStructureToMemoryInfoKHR) -> () {
        let _function = self.cmd_copy_acceleration_structure_to_memory_khr.expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(command_buffer as _, info as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCopyAccelerationStructureToMemoryKHR.html) · Function"]
    #[doc(alias = "vkCopyAccelerationStructureToMemoryKHR")]
    pub unsafe fn copy_acceleration_structure_to_memory_khr(&self, deferred_operation: Option<crate::extensions::khr_deferred_host_operations::DeferredOperationKHR>, info: &crate::extensions::khr_acceleration_structure::CopyAccelerationStructureToMemoryInfoKHR) -> crate::utils::VulkanResult<()> {
        let _function = self.copy_acceleration_structure_to_memory_khr.expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(
            self.handle,
            match deferred_operation {
                Some(v) => v,
                None => Default::default(),
            },
            info as _,
        );
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyMemoryToAccelerationStructureKHR.html) · Function"]
    #[doc(alias = "vkCmdCopyMemoryToAccelerationStructureKHR")]
    pub unsafe fn cmd_copy_memory_to_acceleration_structure_khr(&self, command_buffer: crate::vk1_0::CommandBuffer, info: &crate::extensions::khr_acceleration_structure::CopyMemoryToAccelerationStructureInfoKHR) -> () {
        let _function = self.cmd_copy_memory_to_acceleration_structure_khr.expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(command_buffer as _, info as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCopyMemoryToAccelerationStructureKHR.html) · Function"]
    #[doc(alias = "vkCopyMemoryToAccelerationStructureKHR")]
    pub unsafe fn copy_memory_to_acceleration_structure_khr(&self, deferred_operation: Option<crate::extensions::khr_deferred_host_operations::DeferredOperationKHR>, info: &crate::extensions::khr_acceleration_structure::CopyMemoryToAccelerationStructureInfoKHR) -> crate::utils::VulkanResult<()> {
        let _function = self.copy_memory_to_acceleration_structure_khr.expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(
            self.handle,
            match deferred_operation {
                Some(v) => v,
                None => Default::default(),
            },
            info as _,
        );
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdWriteAccelerationStructuresPropertiesKHR.html) · Function"]
    #[doc(alias = "vkCmdWriteAccelerationStructuresPropertiesKHR")]
    pub unsafe fn cmd_write_acceleration_structures_properties_khr(&self, command_buffer: crate::vk1_0::CommandBuffer, acceleration_structures: &[crate::extensions::khr_acceleration_structure::AccelerationStructureKHR], query_type: crate::vk1_0::QueryType, query_pool: crate::vk1_0::QueryPool, first_query: u32) -> () {
        let _function = self.cmd_write_acceleration_structures_properties_khr.expect(crate::NOT_LOADED_MESSAGE);
        let acceleration_structure_count = acceleration_structures.len();
        let _return = _function(command_buffer as _, acceleration_structure_count as _, acceleration_structures.as_ptr() as _, query_type as _, query_pool as _, first_query as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkWriteAccelerationStructuresPropertiesKHR.html) · Function"]
    #[doc(alias = "vkWriteAccelerationStructuresPropertiesKHR")]
    pub unsafe fn write_acceleration_structures_properties_khr(&self, acceleration_structures: &[crate::extensions::khr_acceleration_structure::AccelerationStructureKHR], query_type: crate::vk1_0::QueryType, data_size: usize, data: *mut std::ffi::c_void, stride: usize) -> crate::utils::VulkanResult<()> {
        let _function = self.write_acceleration_structures_properties_khr.expect(crate::NOT_LOADED_MESSAGE);
        let acceleration_structure_count = acceleration_structures.len();
        let _return = _function(self.handle, acceleration_structure_count as _, acceleration_structures.as_ptr() as _, query_type as _, data_size, data, stride as _);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceAccelerationStructureCompatibilityKHR.html) · Function"]
    #[doc(alias = "vkGetDeviceAccelerationStructureCompatibilityKHR")]
    pub unsafe fn get_device_acceleration_structure_compatibility_khr(&self, version_info: &crate::extensions::khr_acceleration_structure::AccelerationStructureVersionInfoKHR) -> crate::extensions::khr_acceleration_structure::AccelerationStructureCompatibilityKHR {
        let _function = self.get_device_acceleration_structure_compatibility_khr.expect(crate::NOT_LOADED_MESSAGE);
        let mut compatibility = Default::default();
        let _return = _function(self.handle, version_info as _, &mut compatibility);
        compatibility
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateAccelerationStructureKHR.html) · Function"]
    #[doc(alias = "vkCreateAccelerationStructureKHR")]
    pub unsafe fn create_acceleration_structure_khr(&self, create_info: &crate::extensions::khr_acceleration_structure::AccelerationStructureCreateInfoKHR, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> crate::utils::VulkanResult<crate::extensions::khr_acceleration_structure::AccelerationStructureKHR> {
        let _function = self.create_acceleration_structure_khr.expect(crate::NOT_LOADED_MESSAGE);
        let mut acceleration_structure = Default::default();
        let _return = _function(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut acceleration_structure,
        );
        crate::utils::VulkanResult::new(_return, acceleration_structure)
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBuildAccelerationStructuresKHR.html) · Function"]
    #[doc(alias = "vkCmdBuildAccelerationStructuresKHR")]
    pub unsafe fn cmd_build_acceleration_structures_khr(&self, command_buffer: crate::vk1_0::CommandBuffer, infos: &[crate::extensions::khr_acceleration_structure::AccelerationStructureBuildGeometryInfoKHRBuilder], build_range_infos: &[*const crate::extensions::khr_acceleration_structure::AccelerationStructureBuildRangeInfoKHR]) -> () {
        let _function = self.cmd_build_acceleration_structures_khr.expect(crate::NOT_LOADED_MESSAGE);
        let info_count = infos.len().min(build_range_infos.len());
        let _return = _function(command_buffer as _, info_count as _, infos.as_ptr() as _, build_range_infos.as_ptr() as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBuildAccelerationStructuresIndirectKHR.html) · Function"]
    #[doc(alias = "vkCmdBuildAccelerationStructuresIndirectKHR")]
    pub unsafe fn cmd_build_acceleration_structures_indirect_khr(&self, command_buffer: crate::vk1_0::CommandBuffer, infos: &[crate::extensions::khr_acceleration_structure::AccelerationStructureBuildGeometryInfoKHRBuilder], indirect_device_addresses: &[crate::vk1_0::DeviceAddress], indirect_strides: &[u32], max_primitive_counts: &[*const u32]) -> () {
        let _function = self.cmd_build_acceleration_structures_indirect_khr.expect(crate::NOT_LOADED_MESSAGE);
        let info_count = infos.len().min(indirect_device_addresses.len()).min(indirect_strides.len()).min(max_primitive_counts.len());
        let _return = _function(command_buffer as _, info_count as _, infos.as_ptr() as _, indirect_device_addresses.as_ptr() as _, indirect_strides.as_ptr() as _, max_primitive_counts.as_ptr() as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBuildAccelerationStructuresKHR.html) · Function"]
    #[doc(alias = "vkBuildAccelerationStructuresKHR")]
    pub unsafe fn build_acceleration_structures_khr(&self, deferred_operation: Option<crate::extensions::khr_deferred_host_operations::DeferredOperationKHR>, infos: &[crate::extensions::khr_acceleration_structure::AccelerationStructureBuildGeometryInfoKHRBuilder], build_range_infos: &[*const crate::extensions::khr_acceleration_structure::AccelerationStructureBuildRangeInfoKHR]) -> crate::utils::VulkanResult<()> {
        let _function = self.build_acceleration_structures_khr.expect(crate::NOT_LOADED_MESSAGE);
        let info_count = infos.len().min(build_range_infos.len());
        let _return = _function(
            self.handle,
            match deferred_operation {
                Some(v) => v,
                None => Default::default(),
            },
            info_count as _,
            infos.as_ptr() as _,
            build_range_infos.as_ptr() as _,
        );
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetAccelerationStructureDeviceAddressKHR.html) · Function"]
    #[doc(alias = "vkGetAccelerationStructureDeviceAddressKHR")]
    pub unsafe fn get_acceleration_structure_device_address_khr(&self, info: &crate::extensions::khr_acceleration_structure::AccelerationStructureDeviceAddressInfoKHR) -> crate::vk1_0::DeviceAddress {
        let _function = self.get_acceleration_structure_device_address_khr.expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(self.handle, info as _);
        _return
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetAccelerationStructureBuildSizesKHR.html) · Function"]
    #[doc(alias = "vkGetAccelerationStructureBuildSizesKHR")]
    pub unsafe fn get_acceleration_structure_build_sizes_khr(&self, build_type: crate::extensions::khr_acceleration_structure::AccelerationStructureBuildTypeKHR, build_info: &crate::extensions::khr_acceleration_structure::AccelerationStructureBuildGeometryInfoKHR, max_primitive_counts: &[u32]) -> crate::extensions::khr_acceleration_structure::AccelerationStructureBuildSizesInfoKHR {
        let _function = self.get_acceleration_structure_build_sizes_khr.expect(crate::NOT_LOADED_MESSAGE);
        let mut size_info = Default::default();
        let _return = _function(self.handle, build_type as _, build_info as _, max_primitive_counts.as_ptr() as _, &mut size_info);
        size_info
    }
}
