#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_LUID_SIZE")]
pub const LUID_SIZE: u32 = 8;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_QUEUE_FAMILY_EXTERNAL")]
pub const QUEUE_FAMILY_EXTERNAL: u32 = 4294967294;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_MAX_DEVICE_GROUP_SIZE")]
pub const MAX_DEVICE_GROUP_SIZE: u32 = 32;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_ENUMERATE_INSTANCE_VERSION: *const std::os::raw::c_char = crate::cstr!("vkEnumerateInstanceVersion");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_PHYSICAL_DEVICE_FEATURES2: *const std::os::raw::c_char = crate::cstr!("vkGetPhysicalDeviceFeatures2");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_PHYSICAL_DEVICE_PROPERTIES2: *const std::os::raw::c_char = crate::cstr!("vkGetPhysicalDeviceProperties2");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_PHYSICAL_DEVICE_FORMAT_PROPERTIES2: *const std::os::raw::c_char = crate::cstr!("vkGetPhysicalDeviceFormatProperties2");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_PHYSICAL_DEVICE_IMAGE_FORMAT_PROPERTIES2: *const std::os::raw::c_char = crate::cstr!("vkGetPhysicalDeviceImageFormatProperties2");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_PHYSICAL_DEVICE_QUEUE_FAMILY_PROPERTIES2: *const std::os::raw::c_char = crate::cstr!("vkGetPhysicalDeviceQueueFamilyProperties2");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_PHYSICAL_DEVICE_MEMORY_PROPERTIES2: *const std::os::raw::c_char = crate::cstr!("vkGetPhysicalDeviceMemoryProperties2");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_PROPERTIES2: *const std::os::raw::c_char = crate::cstr!("vkGetPhysicalDeviceSparseImageFormatProperties2");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_TRIM_COMMAND_POOL: *const std::os::raw::c_char = crate::cstr!("vkTrimCommandPool");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_PHYSICAL_DEVICE_EXTERNAL_BUFFER_PROPERTIES: *const std::os::raw::c_char = crate::cstr!("vkGetPhysicalDeviceExternalBufferProperties");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_PROPERTIES: *const std::os::raw::c_char = crate::cstr!("vkGetPhysicalDeviceExternalSemaphoreProperties");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_PHYSICAL_DEVICE_EXTERNAL_FENCE_PROPERTIES: *const std::os::raw::c_char = crate::cstr!("vkGetPhysicalDeviceExternalFenceProperties");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_ENUMERATE_PHYSICAL_DEVICE_GROUPS: *const std::os::raw::c_char = crate::cstr!("vkEnumeratePhysicalDeviceGroups");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_DEVICE_GROUP_PEER_MEMORY_FEATURES: *const std::os::raw::c_char = crate::cstr!("vkGetDeviceGroupPeerMemoryFeatures");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_BIND_BUFFER_MEMORY2: *const std::os::raw::c_char = crate::cstr!("vkBindBufferMemory2");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_BIND_IMAGE_MEMORY2: *const std::os::raw::c_char = crate::cstr!("vkBindImageMemory2");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_SET_DEVICE_MASK: *const std::os::raw::c_char = crate::cstr!("vkCmdSetDeviceMask");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_DISPATCH_BASE: *const std::os::raw::c_char = crate::cstr!("vkCmdDispatchBase");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_DESCRIPTOR_UPDATE_TEMPLATE: *const std::os::raw::c_char = crate::cstr!("vkCreateDescriptorUpdateTemplate");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_DESTROY_DESCRIPTOR_UPDATE_TEMPLATE: *const std::os::raw::c_char = crate::cstr!("vkDestroyDescriptorUpdateTemplate");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_UPDATE_DESCRIPTOR_SET_WITH_TEMPLATE: *const std::os::raw::c_char = crate::cstr!("vkUpdateDescriptorSetWithTemplate");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_BUFFER_MEMORY_REQUIREMENTS2: *const std::os::raw::c_char = crate::cstr!("vkGetBufferMemoryRequirements2");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_IMAGE_MEMORY_REQUIREMENTS2: *const std::os::raw::c_char = crate::cstr!("vkGetImageMemoryRequirements2");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_IMAGE_SPARSE_MEMORY_REQUIREMENTS2: *const std::os::raw::c_char = crate::cstr!("vkGetImageSparseMemoryRequirements2");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_SAMPLER_YCBCR_CONVERSION: *const std::os::raw::c_char = crate::cstr!("vkCreateSamplerYcbcrConversion");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_DESTROY_SAMPLER_YCBCR_CONVERSION: *const std::os::raw::c_char = crate::cstr!("vkDestroySamplerYcbcrConversion");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_DEVICE_QUEUE2: *const std::os::raw::c_char = crate::cstr!("vkGetDeviceQueue2");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_DESCRIPTOR_SET_LAYOUT_SUPPORT: *const std::os::raw::c_char = crate::cstr!("vkGetDescriptorSetLayoutSupport");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceVariablePointerFeatures.html) · Alias"]
#[doc(alias = "VkPhysicalDeviceVariablePointerFeatures")]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceVariablePointerFeatures = crate::vk1_1::PhysicalDeviceVariablePointersFeatures;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceVariablePointerFeatures.html) · Alias"]
#[doc(alias = "VkPhysicalDeviceVariablePointerFeatures")]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceVariablePointerFeaturesBuilder<'a> = crate::vk1_1::PhysicalDeviceVariablePointersFeaturesBuilder<'a>;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderDrawParameterFeatures.html) · Alias"]
#[doc(alias = "VkPhysicalDeviceShaderDrawParameterFeatures")]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceShaderDrawParameterFeatures = crate::vk1_1::PhysicalDeviceShaderDrawParametersFeatures;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderDrawParameterFeatures.html) · Alias"]
#[doc(alias = "VkPhysicalDeviceShaderDrawParameterFeatures")]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceShaderDrawParameterFeaturesBuilder<'a> = crate::vk1_1::PhysicalDeviceShaderDrawParametersFeaturesBuilder<'a>;
crate::non_dispatchable_handle!(
    DescriptorUpdateTemplate,
    DESCRIPTOR_UPDATE_TEMPLATE,
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorUpdateTemplate.html) · Non-dispatchable Handle",
    "VkDescriptorUpdateTemplate"
);
crate::non_dispatchable_handle!(
    SamplerYcbcrConversion,
    SAMPLER_YCBCR_CONVERSION,
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerYcbcrConversion.html) · Non-dispatchable Handle",
    "VkSamplerYcbcrConversion"
);
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorUpdateTemplateCreateFlags.html) · Bitmask of [`DescriptorUpdateTemplateCreateFlagBits`]"] # [doc (alias = "VkDescriptorUpdateTemplateCreateFlags")] # [derive (Default)] # [repr (transparent)] pub struct DescriptorUpdateTemplateCreateFlags : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`DescriptorUpdateTemplateCreateFlags`]"]
#[doc(alias = "VkDescriptorUpdateTemplateCreateFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct DescriptorUpdateTemplateCreateFlagBits(pub u32);
impl DescriptorUpdateTemplateCreateFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> DescriptorUpdateTemplateCreateFlags {
        DescriptorUpdateTemplateCreateFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for DescriptorUpdateTemplateCreateFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandPoolTrimFlags.html) · Bitmask of [`CommandPoolTrimFlagBits`]"] # [doc (alias = "VkCommandPoolTrimFlags")] # [derive (Default)] # [repr (transparent)] pub struct CommandPoolTrimFlags : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`CommandPoolTrimFlags`]"]
#[doc(alias = "VkCommandPoolTrimFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct CommandPoolTrimFlagBits(pub u32);
impl CommandPoolTrimFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> CommandPoolTrimFlags {
        CommandPoolTrimFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for CommandPoolTrimFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorUpdateTemplateType.html) · Enum"]
#[doc(alias = "VkDescriptorUpdateTemplateType")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct DescriptorUpdateTemplateType(pub i32);
impl std::fmt::Debug for DescriptorUpdateTemplateType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::DESCRIPTOR_SET => "DESCRIPTOR_SET",
            &Self::PUSH_DESCRIPTORS_KHR => "PUSH_DESCRIPTORS_KHR",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_1`]"]
impl DescriptorUpdateTemplateType {
    pub const DESCRIPTOR_SET: Self = Self(0);
}
#[doc = "Provided by [`crate::extensions::khr_push_descriptor`]"]
impl DescriptorUpdateTemplateType {
    pub const PUSH_DESCRIPTORS_KHR: Self = Self(1);
}
#[doc = "Provided by [`crate::extensions::khr_descriptor_update_template`]"]
impl DescriptorUpdateTemplateType {
    pub const DESCRIPTOR_SET_KHR: Self = Self::DESCRIPTOR_SET;
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPointClippingBehavior.html) · Enum"]
#[doc(alias = "VkPointClippingBehavior")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct PointClippingBehavior(pub i32);
impl std::fmt::Debug for PointClippingBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::ALL_CLIP_PLANES => "ALL_CLIP_PLANES",
            &Self::USER_CLIP_PLANES_ONLY => "USER_CLIP_PLANES_ONLY",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_1`]"]
impl PointClippingBehavior {
    pub const ALL_CLIP_PLANES: Self = Self(0);
    pub const USER_CLIP_PLANES_ONLY: Self = Self(1);
}
#[doc = "Provided by [`crate::extensions::khr_maintenance2`]"]
impl PointClippingBehavior {
    pub const ALL_CLIP_PLANES_KHR: Self = Self::ALL_CLIP_PLANES;
    pub const USER_CLIP_PLANES_ONLY_KHR: Self = Self::USER_CLIP_PLANES_ONLY;
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalMemoryHandleTypeFlags.html) · Bitmask of [`ExternalMemoryHandleTypeFlagBits`]"] # [doc (alias = "VkExternalMemoryHandleTypeFlags")] # [derive (Default)] # [repr (transparent)] pub struct ExternalMemoryHandleTypeFlags : u32 { const OPAQUE_FD = ExternalMemoryHandleTypeFlagBits :: OPAQUE_FD . 0 ; const OPAQUE_WIN32 = ExternalMemoryHandleTypeFlagBits :: OPAQUE_WIN32 . 0 ; const OPAQUE_WIN32_KMT = ExternalMemoryHandleTypeFlagBits :: OPAQUE_WIN32_KMT . 0 ; const D3D11_TEXTURE = ExternalMemoryHandleTypeFlagBits :: D3D11_TEXTURE . 0 ; const D3D11_TEXTURE_KMT = ExternalMemoryHandleTypeFlagBits :: D3D11_TEXTURE_KMT . 0 ; const D3D12_HEAP = ExternalMemoryHandleTypeFlagBits :: D3D12_HEAP . 0 ; const D3D12_RESOURCE = ExternalMemoryHandleTypeFlagBits :: D3D12_RESOURCE . 0 ; const DMA_BUF_EXT = ExternalMemoryHandleTypeFlagBits :: DMA_BUF_EXT . 0 ; const ANDROID_HARDWARE_BUFFER_ANDROID = ExternalMemoryHandleTypeFlagBits :: ANDROID_HARDWARE_BUFFER_ANDROID . 0 ; const HOST_ALLOCATION_EXT = ExternalMemoryHandleTypeFlagBits :: HOST_ALLOCATION_EXT . 0 ; const HOST_MAPPED_FOREIGN_MEMORY_EXT = ExternalMemoryHandleTypeFlagBits :: HOST_MAPPED_FOREIGN_MEMORY_EXT . 0 ; const ZIRCON_VMO_FUCHSIA = ExternalMemoryHandleTypeFlagBits :: ZIRCON_VMO_FUCHSIA . 0 ; const OPAQUE_FD_KHR = ExternalMemoryHandleTypeFlagBits :: OPAQUE_FD_KHR . 0 ; const OPAQUE_WIN32_KHR = ExternalMemoryHandleTypeFlagBits :: OPAQUE_WIN32_KHR . 0 ; const OPAQUE_WIN32_KMT_KHR = ExternalMemoryHandleTypeFlagBits :: OPAQUE_WIN32_KMT_KHR . 0 ; const D3D11_TEXTURE_KHR = ExternalMemoryHandleTypeFlagBits :: D3D11_TEXTURE_KHR . 0 ; const D3D11_TEXTURE_KMT_KHR = ExternalMemoryHandleTypeFlagBits :: D3D11_TEXTURE_KMT_KHR . 0 ; const D3D12_HEAP_KHR = ExternalMemoryHandleTypeFlagBits :: D3D12_HEAP_KHR . 0 ; const D3D12_RESOURCE_KHR = ExternalMemoryHandleTypeFlagBits :: D3D12_RESOURCE_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalMemoryHandleTypeFlagBits.html) · Bits enum of [`ExternalMemoryHandleTypeFlags`]"]
#[doc(alias = "VkExternalMemoryHandleTypeFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct ExternalMemoryHandleTypeFlagBits(pub u32);
impl ExternalMemoryHandleTypeFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> ExternalMemoryHandleTypeFlags {
        ExternalMemoryHandleTypeFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for ExternalMemoryHandleTypeFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::OPAQUE_FD => "OPAQUE_FD",
            &Self::OPAQUE_WIN32 => "OPAQUE_WIN32",
            &Self::OPAQUE_WIN32_KMT => "OPAQUE_WIN32_KMT",
            &Self::D3D11_TEXTURE => "D3D11_TEXTURE",
            &Self::D3D11_TEXTURE_KMT => "D3D11_TEXTURE_KMT",
            &Self::D3D12_HEAP => "D3D12_HEAP",
            &Self::D3D12_RESOURCE => "D3D12_RESOURCE",
            &Self::DMA_BUF_EXT => "DMA_BUF_EXT",
            &Self::ANDROID_HARDWARE_BUFFER_ANDROID => "ANDROID_HARDWARE_BUFFER_ANDROID",
            &Self::HOST_ALLOCATION_EXT => "HOST_ALLOCATION_EXT",
            &Self::HOST_MAPPED_FOREIGN_MEMORY_EXT => "HOST_MAPPED_FOREIGN_MEMORY_EXT",
            &Self::ZIRCON_VMO_FUCHSIA => "ZIRCON_VMO_FUCHSIA",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_1`]"]
impl ExternalMemoryHandleTypeFlagBits {
    pub const OPAQUE_FD: Self = Self(1);
    pub const OPAQUE_WIN32: Self = Self(2);
    pub const OPAQUE_WIN32_KMT: Self = Self(4);
    pub const D3D11_TEXTURE: Self = Self(8);
    pub const D3D11_TEXTURE_KMT: Self = Self(16);
    pub const D3D12_HEAP: Self = Self(32);
    pub const D3D12_RESOURCE: Self = Self(64);
}
#[doc = "Provided by [`crate::extensions::ext_external_memory_dma_buf`]"]
impl ExternalMemoryHandleTypeFlagBits {
    pub const DMA_BUF_EXT: Self = Self(512);
}
#[doc = "Provided by [`crate::extensions::android_external_memory_android_hardware_buffer`]"]
impl ExternalMemoryHandleTypeFlagBits {
    pub const ANDROID_HARDWARE_BUFFER_ANDROID: Self = Self(1024);
}
#[doc = "Provided by [`crate::extensions::ext_external_memory_host`]"]
impl ExternalMemoryHandleTypeFlagBits {
    pub const HOST_ALLOCATION_EXT: Self = Self(128);
    pub const HOST_MAPPED_FOREIGN_MEMORY_EXT: Self = Self(256);
}
#[doc = "Provided by [`crate::extensions::fuchsia_external_memory`]"]
impl ExternalMemoryHandleTypeFlagBits {
    pub const ZIRCON_VMO_FUCHSIA: Self = Self(2048);
}
#[doc = "Provided by [`crate::extensions::khr_external_memory_capabilities`]"]
impl ExternalMemoryHandleTypeFlagBits {
    pub const OPAQUE_FD_KHR: Self = Self::OPAQUE_FD;
    pub const OPAQUE_WIN32_KHR: Self = Self::OPAQUE_WIN32;
    pub const OPAQUE_WIN32_KMT_KHR: Self = Self::OPAQUE_WIN32_KMT;
    pub const D3D11_TEXTURE_KHR: Self = Self::D3D11_TEXTURE;
    pub const D3D11_TEXTURE_KMT_KHR: Self = Self::D3D11_TEXTURE_KMT;
    pub const D3D12_HEAP_KHR: Self = Self::D3D12_HEAP;
    pub const D3D12_RESOURCE_KHR: Self = Self::D3D12_RESOURCE;
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalMemoryFeatureFlags.html) · Bitmask of [`ExternalMemoryFeatureFlagBits`]"] # [doc (alias = "VkExternalMemoryFeatureFlags")] # [derive (Default)] # [repr (transparent)] pub struct ExternalMemoryFeatureFlags : u32 { const DEDICATED_ONLY = ExternalMemoryFeatureFlagBits :: DEDICATED_ONLY . 0 ; const EXPORTABLE = ExternalMemoryFeatureFlagBits :: EXPORTABLE . 0 ; const IMPORTABLE = ExternalMemoryFeatureFlagBits :: IMPORTABLE . 0 ; const DEDICATED_ONLY_KHR = ExternalMemoryFeatureFlagBits :: DEDICATED_ONLY_KHR . 0 ; const EXPORTABLE_KHR = ExternalMemoryFeatureFlagBits :: EXPORTABLE_KHR . 0 ; const IMPORTABLE_KHR = ExternalMemoryFeatureFlagBits :: IMPORTABLE_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalMemoryFeatureFlagBits.html) · Bits enum of [`ExternalMemoryFeatureFlags`]"]
#[doc(alias = "VkExternalMemoryFeatureFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct ExternalMemoryFeatureFlagBits(pub u32);
impl ExternalMemoryFeatureFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> ExternalMemoryFeatureFlags {
        ExternalMemoryFeatureFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for ExternalMemoryFeatureFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::DEDICATED_ONLY => "DEDICATED_ONLY",
            &Self::EXPORTABLE => "EXPORTABLE",
            &Self::IMPORTABLE => "IMPORTABLE",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_1`]"]
impl ExternalMemoryFeatureFlagBits {
    pub const DEDICATED_ONLY: Self = Self(1);
    pub const EXPORTABLE: Self = Self(2);
    pub const IMPORTABLE: Self = Self(4);
}
#[doc = "Provided by [`crate::extensions::khr_external_memory_capabilities`]"]
impl ExternalMemoryFeatureFlagBits {
    pub const DEDICATED_ONLY_KHR: Self = Self::DEDICATED_ONLY;
    pub const EXPORTABLE_KHR: Self = Self::EXPORTABLE;
    pub const IMPORTABLE_KHR: Self = Self::IMPORTABLE;
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalSemaphoreHandleTypeFlags.html) · Bitmask of [`ExternalSemaphoreHandleTypeFlagBits`]"] # [doc (alias = "VkExternalSemaphoreHandleTypeFlags")] # [derive (Default)] # [repr (transparent)] pub struct ExternalSemaphoreHandleTypeFlags : u32 { const OPAQUE_FD = ExternalSemaphoreHandleTypeFlagBits :: OPAQUE_FD . 0 ; const OPAQUE_WIN32 = ExternalSemaphoreHandleTypeFlagBits :: OPAQUE_WIN32 . 0 ; const OPAQUE_WIN32_KMT = ExternalSemaphoreHandleTypeFlagBits :: OPAQUE_WIN32_KMT . 0 ; const D3D12_FENCE = ExternalSemaphoreHandleTypeFlagBits :: D3D12_FENCE . 0 ; const SYNC_FD = ExternalSemaphoreHandleTypeFlagBits :: SYNC_FD . 0 ; const ZIRCON_EVENT_FUCHSIA = ExternalSemaphoreHandleTypeFlagBits :: ZIRCON_EVENT_FUCHSIA . 0 ; const D3D11_FENCE = ExternalSemaphoreHandleTypeFlagBits :: D3D11_FENCE . 0 ; const OPAQUE_FD_KHR = ExternalSemaphoreHandleTypeFlagBits :: OPAQUE_FD_KHR . 0 ; const OPAQUE_WIN32_KHR = ExternalSemaphoreHandleTypeFlagBits :: OPAQUE_WIN32_KHR . 0 ; const OPAQUE_WIN32_KMT_KHR = ExternalSemaphoreHandleTypeFlagBits :: OPAQUE_WIN32_KMT_KHR . 0 ; const D3D12_FENCE_KHR = ExternalSemaphoreHandleTypeFlagBits :: D3D12_FENCE_KHR . 0 ; const SYNC_FD_KHR = ExternalSemaphoreHandleTypeFlagBits :: SYNC_FD_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalSemaphoreHandleTypeFlagBits.html) · Bits enum of [`ExternalSemaphoreHandleTypeFlags`]"]
#[doc(alias = "VkExternalSemaphoreHandleTypeFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct ExternalSemaphoreHandleTypeFlagBits(pub u32);
impl ExternalSemaphoreHandleTypeFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> ExternalSemaphoreHandleTypeFlags {
        ExternalSemaphoreHandleTypeFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for ExternalSemaphoreHandleTypeFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::OPAQUE_FD => "OPAQUE_FD",
            &Self::OPAQUE_WIN32 => "OPAQUE_WIN32",
            &Self::OPAQUE_WIN32_KMT => "OPAQUE_WIN32_KMT",
            &Self::D3D12_FENCE => "D3D12_FENCE",
            &Self::SYNC_FD => "SYNC_FD",
            &Self::ZIRCON_EVENT_FUCHSIA => "ZIRCON_EVENT_FUCHSIA",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_1`]"]
impl ExternalSemaphoreHandleTypeFlagBits {
    pub const OPAQUE_FD: Self = Self(1);
    pub const OPAQUE_WIN32: Self = Self(2);
    pub const OPAQUE_WIN32_KMT: Self = Self(4);
    pub const D3D12_FENCE: Self = Self(8);
    pub const SYNC_FD: Self = Self(16);
    pub const D3D11_FENCE: Self = Self::D3D12_FENCE;
}
#[doc = "Provided by [`crate::extensions::fuchsia_external_semaphore`]"]
impl ExternalSemaphoreHandleTypeFlagBits {
    pub const ZIRCON_EVENT_FUCHSIA: Self = Self(128);
}
#[doc = "Provided by [`crate::extensions::khr_external_semaphore_capabilities`]"]
impl ExternalSemaphoreHandleTypeFlagBits {
    pub const OPAQUE_FD_KHR: Self = Self::OPAQUE_FD;
    pub const OPAQUE_WIN32_KHR: Self = Self::OPAQUE_WIN32;
    pub const OPAQUE_WIN32_KMT_KHR: Self = Self::OPAQUE_WIN32_KMT;
    pub const D3D12_FENCE_KHR: Self = Self::D3D12_FENCE;
    pub const SYNC_FD_KHR: Self = Self::SYNC_FD;
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalSemaphoreFeatureFlags.html) · Bitmask of [`ExternalSemaphoreFeatureFlagBits`]"] # [doc (alias = "VkExternalSemaphoreFeatureFlags")] # [derive (Default)] # [repr (transparent)] pub struct ExternalSemaphoreFeatureFlags : u32 { const EXPORTABLE = ExternalSemaphoreFeatureFlagBits :: EXPORTABLE . 0 ; const IMPORTABLE = ExternalSemaphoreFeatureFlagBits :: IMPORTABLE . 0 ; const EXPORTABLE_KHR = ExternalSemaphoreFeatureFlagBits :: EXPORTABLE_KHR . 0 ; const IMPORTABLE_KHR = ExternalSemaphoreFeatureFlagBits :: IMPORTABLE_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalSemaphoreFeatureFlagBits.html) · Bits enum of [`ExternalSemaphoreFeatureFlags`]"]
#[doc(alias = "VkExternalSemaphoreFeatureFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct ExternalSemaphoreFeatureFlagBits(pub u32);
impl ExternalSemaphoreFeatureFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> ExternalSemaphoreFeatureFlags {
        ExternalSemaphoreFeatureFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for ExternalSemaphoreFeatureFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::EXPORTABLE => "EXPORTABLE",
            &Self::IMPORTABLE => "IMPORTABLE",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_1`]"]
impl ExternalSemaphoreFeatureFlagBits {
    pub const EXPORTABLE: Self = Self(1);
    pub const IMPORTABLE: Self = Self(2);
}
#[doc = "Provided by [`crate::extensions::khr_external_semaphore_capabilities`]"]
impl ExternalSemaphoreFeatureFlagBits {
    pub const EXPORTABLE_KHR: Self = Self::EXPORTABLE;
    pub const IMPORTABLE_KHR: Self = Self::IMPORTABLE;
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreImportFlags.html) · Bitmask of [`SemaphoreImportFlagBits`]"] # [doc (alias = "VkSemaphoreImportFlags")] # [derive (Default)] # [repr (transparent)] pub struct SemaphoreImportFlags : u32 { const TEMPORARY = SemaphoreImportFlagBits :: TEMPORARY . 0 ; const TEMPORARY_KHR = SemaphoreImportFlagBits :: TEMPORARY_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreImportFlagBits.html) · Bits enum of [`SemaphoreImportFlags`]"]
#[doc(alias = "VkSemaphoreImportFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct SemaphoreImportFlagBits(pub u32);
impl SemaphoreImportFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> SemaphoreImportFlags {
        SemaphoreImportFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for SemaphoreImportFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::TEMPORARY => "TEMPORARY",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_1`]"]
impl SemaphoreImportFlagBits {
    pub const TEMPORARY: Self = Self(1);
}
#[doc = "Provided by [`crate::extensions::khr_external_semaphore`]"]
impl SemaphoreImportFlagBits {
    pub const TEMPORARY_KHR: Self = Self::TEMPORARY;
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalFenceHandleTypeFlags.html) · Bitmask of [`ExternalFenceHandleTypeFlagBits`]"] # [doc (alias = "VkExternalFenceHandleTypeFlags")] # [derive (Default)] # [repr (transparent)] pub struct ExternalFenceHandleTypeFlags : u32 { const OPAQUE_FD = ExternalFenceHandleTypeFlagBits :: OPAQUE_FD . 0 ; const OPAQUE_WIN32 = ExternalFenceHandleTypeFlagBits :: OPAQUE_WIN32 . 0 ; const OPAQUE_WIN32_KMT = ExternalFenceHandleTypeFlagBits :: OPAQUE_WIN32_KMT . 0 ; const SYNC_FD = ExternalFenceHandleTypeFlagBits :: SYNC_FD . 0 ; const OPAQUE_FD_KHR = ExternalFenceHandleTypeFlagBits :: OPAQUE_FD_KHR . 0 ; const OPAQUE_WIN32_KHR = ExternalFenceHandleTypeFlagBits :: OPAQUE_WIN32_KHR . 0 ; const OPAQUE_WIN32_KMT_KHR = ExternalFenceHandleTypeFlagBits :: OPAQUE_WIN32_KMT_KHR . 0 ; const SYNC_FD_KHR = ExternalFenceHandleTypeFlagBits :: SYNC_FD_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalFenceHandleTypeFlagBits.html) · Bits enum of [`ExternalFenceHandleTypeFlags`]"]
#[doc(alias = "VkExternalFenceHandleTypeFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct ExternalFenceHandleTypeFlagBits(pub u32);
impl ExternalFenceHandleTypeFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> ExternalFenceHandleTypeFlags {
        ExternalFenceHandleTypeFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for ExternalFenceHandleTypeFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::OPAQUE_FD => "OPAQUE_FD",
            &Self::OPAQUE_WIN32 => "OPAQUE_WIN32",
            &Self::OPAQUE_WIN32_KMT => "OPAQUE_WIN32_KMT",
            &Self::SYNC_FD => "SYNC_FD",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_1`]"]
impl ExternalFenceHandleTypeFlagBits {
    pub const OPAQUE_FD: Self = Self(1);
    pub const OPAQUE_WIN32: Self = Self(2);
    pub const OPAQUE_WIN32_KMT: Self = Self(4);
    pub const SYNC_FD: Self = Self(8);
}
#[doc = "Provided by [`crate::extensions::khr_external_fence_capabilities`]"]
impl ExternalFenceHandleTypeFlagBits {
    pub const OPAQUE_FD_KHR: Self = Self::OPAQUE_FD;
    pub const OPAQUE_WIN32_KHR: Self = Self::OPAQUE_WIN32;
    pub const OPAQUE_WIN32_KMT_KHR: Self = Self::OPAQUE_WIN32_KMT;
    pub const SYNC_FD_KHR: Self = Self::SYNC_FD;
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalFenceFeatureFlags.html) · Bitmask of [`ExternalFenceFeatureFlagBits`]"] # [doc (alias = "VkExternalFenceFeatureFlags")] # [derive (Default)] # [repr (transparent)] pub struct ExternalFenceFeatureFlags : u32 { const EXPORTABLE = ExternalFenceFeatureFlagBits :: EXPORTABLE . 0 ; const IMPORTABLE = ExternalFenceFeatureFlagBits :: IMPORTABLE . 0 ; const EXPORTABLE_KHR = ExternalFenceFeatureFlagBits :: EXPORTABLE_KHR . 0 ; const IMPORTABLE_KHR = ExternalFenceFeatureFlagBits :: IMPORTABLE_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalFenceFeatureFlagBits.html) · Bits enum of [`ExternalFenceFeatureFlags`]"]
#[doc(alias = "VkExternalFenceFeatureFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct ExternalFenceFeatureFlagBits(pub u32);
impl ExternalFenceFeatureFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> ExternalFenceFeatureFlags {
        ExternalFenceFeatureFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for ExternalFenceFeatureFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::EXPORTABLE => "EXPORTABLE",
            &Self::IMPORTABLE => "IMPORTABLE",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_1`]"]
impl ExternalFenceFeatureFlagBits {
    pub const EXPORTABLE: Self = Self(1);
    pub const IMPORTABLE: Self = Self(2);
}
#[doc = "Provided by [`crate::extensions::khr_external_fence_capabilities`]"]
impl ExternalFenceFeatureFlagBits {
    pub const EXPORTABLE_KHR: Self = Self::EXPORTABLE;
    pub const IMPORTABLE_KHR: Self = Self::IMPORTABLE;
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFenceImportFlags.html) · Bitmask of [`FenceImportFlagBits`]"] # [doc (alias = "VkFenceImportFlags")] # [derive (Default)] # [repr (transparent)] pub struct FenceImportFlags : u32 { const TEMPORARY = FenceImportFlagBits :: TEMPORARY . 0 ; const TEMPORARY_KHR = FenceImportFlagBits :: TEMPORARY_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFenceImportFlagBits.html) · Bits enum of [`FenceImportFlags`]"]
#[doc(alias = "VkFenceImportFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct FenceImportFlagBits(pub u32);
impl FenceImportFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> FenceImportFlags {
        FenceImportFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for FenceImportFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::TEMPORARY => "TEMPORARY",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_1`]"]
impl FenceImportFlagBits {
    pub const TEMPORARY: Self = Self(1);
}
#[doc = "Provided by [`crate::extensions::khr_external_fence`]"]
impl FenceImportFlagBits {
    pub const TEMPORARY_KHR: Self = Self::TEMPORARY;
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPeerMemoryFeatureFlags.html) · Bitmask of [`PeerMemoryFeatureFlagBits`]"] # [doc (alias = "VkPeerMemoryFeatureFlags")] # [derive (Default)] # [repr (transparent)] pub struct PeerMemoryFeatureFlags : u32 { const COPY_SRC = PeerMemoryFeatureFlagBits :: COPY_SRC . 0 ; const COPY_DST = PeerMemoryFeatureFlagBits :: COPY_DST . 0 ; const GENERIC_SRC = PeerMemoryFeatureFlagBits :: GENERIC_SRC . 0 ; const GENERIC_DST = PeerMemoryFeatureFlagBits :: GENERIC_DST . 0 ; const COPY_SRC_KHR = PeerMemoryFeatureFlagBits :: COPY_SRC_KHR . 0 ; const COPY_DST_KHR = PeerMemoryFeatureFlagBits :: COPY_DST_KHR . 0 ; const GENERIC_SRC_KHR = PeerMemoryFeatureFlagBits :: GENERIC_SRC_KHR . 0 ; const GENERIC_DST_KHR = PeerMemoryFeatureFlagBits :: GENERIC_DST_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPeerMemoryFeatureFlagBits.html) · Bits enum of [`PeerMemoryFeatureFlags`]"]
#[doc(alias = "VkPeerMemoryFeatureFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct PeerMemoryFeatureFlagBits(pub u32);
impl PeerMemoryFeatureFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> PeerMemoryFeatureFlags {
        PeerMemoryFeatureFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for PeerMemoryFeatureFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::COPY_SRC => "COPY_SRC",
            &Self::COPY_DST => "COPY_DST",
            &Self::GENERIC_SRC => "GENERIC_SRC",
            &Self::GENERIC_DST => "GENERIC_DST",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_1`]"]
impl PeerMemoryFeatureFlagBits {
    pub const COPY_SRC: Self = Self(1);
    pub const COPY_DST: Self = Self(2);
    pub const GENERIC_SRC: Self = Self(4);
    pub const GENERIC_DST: Self = Self(8);
}
#[doc = "Provided by [`crate::extensions::khr_device_group`]"]
impl PeerMemoryFeatureFlagBits {
    pub const COPY_SRC_KHR: Self = Self::COPY_SRC;
    pub const COPY_DST_KHR: Self = Self::COPY_DST;
    pub const GENERIC_SRC_KHR: Self = Self::GENERIC_SRC;
    pub const GENERIC_DST_KHR: Self = Self::GENERIC_DST;
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryAllocateFlags.html) · Bitmask of [`MemoryAllocateFlagBits`]"] # [doc (alias = "VkMemoryAllocateFlags")] # [derive (Default)] # [repr (transparent)] pub struct MemoryAllocateFlags : u32 { const DEVICE_MASK = MemoryAllocateFlagBits :: DEVICE_MASK . 0 ; const DEVICE_ADDRESS = MemoryAllocateFlagBits :: DEVICE_ADDRESS . 0 ; const DEVICE_ADDRESS_CAPTURE_REPLAY = MemoryAllocateFlagBits :: DEVICE_ADDRESS_CAPTURE_REPLAY . 0 ; const DEVICE_MASK_KHR = MemoryAllocateFlagBits :: DEVICE_MASK_KHR . 0 ; const DEVICE_ADDRESS_KHR = MemoryAllocateFlagBits :: DEVICE_ADDRESS_KHR . 0 ; const DEVICE_ADDRESS_CAPTURE_REPLAY_KHR = MemoryAllocateFlagBits :: DEVICE_ADDRESS_CAPTURE_REPLAY_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryAllocateFlagBits.html) · Bits enum of [`MemoryAllocateFlags`]"]
#[doc(alias = "VkMemoryAllocateFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct MemoryAllocateFlagBits(pub u32);
impl MemoryAllocateFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> MemoryAllocateFlags {
        MemoryAllocateFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for MemoryAllocateFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::DEVICE_MASK => "DEVICE_MASK",
            &Self::DEVICE_ADDRESS => "DEVICE_ADDRESS",
            &Self::DEVICE_ADDRESS_CAPTURE_REPLAY => "DEVICE_ADDRESS_CAPTURE_REPLAY",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_1`]"]
impl MemoryAllocateFlagBits {
    pub const DEVICE_MASK: Self = Self(1);
}
#[doc = "Provided by [`crate::vk1_2`]"]
impl MemoryAllocateFlagBits {
    pub const DEVICE_ADDRESS: Self = Self(2);
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY: Self = Self(4);
}
#[doc = "Provided by [`crate::extensions::khr_device_group`]"]
impl MemoryAllocateFlagBits {
    pub const DEVICE_MASK_KHR: Self = Self::DEVICE_MASK;
}
#[doc = "Provided by [`crate::extensions::khr_buffer_device_address`]"]
impl MemoryAllocateFlagBits {
    pub const DEVICE_ADDRESS_KHR: Self = Self::DEVICE_ADDRESS;
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY_KHR: Self = Self::DEVICE_ADDRESS_CAPTURE_REPLAY;
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubgroupFeatureFlags.html) · Bitmask of [`SubgroupFeatureFlagBits`]"] # [doc (alias = "VkSubgroupFeatureFlags")] # [derive (Default)] # [repr (transparent)] pub struct SubgroupFeatureFlags : u32 { const BASIC = SubgroupFeatureFlagBits :: BASIC . 0 ; const VOTE = SubgroupFeatureFlagBits :: VOTE . 0 ; const ARITHMETIC = SubgroupFeatureFlagBits :: ARITHMETIC . 0 ; const BALLOT = SubgroupFeatureFlagBits :: BALLOT . 0 ; const SHUFFLE = SubgroupFeatureFlagBits :: SHUFFLE . 0 ; const SHUFFLE_RELATIVE = SubgroupFeatureFlagBits :: SHUFFLE_RELATIVE . 0 ; const CLUSTERED = SubgroupFeatureFlagBits :: CLUSTERED . 0 ; const QUAD = SubgroupFeatureFlagBits :: QUAD . 0 ; const PARTITIONED_NV = SubgroupFeatureFlagBits :: PARTITIONED_NV . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubgroupFeatureFlagBits.html) · Bits enum of [`SubgroupFeatureFlags`]"]
#[doc(alias = "VkSubgroupFeatureFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct SubgroupFeatureFlagBits(pub u32);
impl SubgroupFeatureFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> SubgroupFeatureFlags {
        SubgroupFeatureFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for SubgroupFeatureFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::BASIC => "BASIC",
            &Self::VOTE => "VOTE",
            &Self::ARITHMETIC => "ARITHMETIC",
            &Self::BALLOT => "BALLOT",
            &Self::SHUFFLE => "SHUFFLE",
            &Self::SHUFFLE_RELATIVE => "SHUFFLE_RELATIVE",
            &Self::CLUSTERED => "CLUSTERED",
            &Self::QUAD => "QUAD",
            &Self::PARTITIONED_NV => "PARTITIONED_NV",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_1`]"]
impl SubgroupFeatureFlagBits {
    pub const BASIC: Self = Self(1);
    pub const VOTE: Self = Self(2);
    pub const ARITHMETIC: Self = Self(4);
    pub const BALLOT: Self = Self(8);
    pub const SHUFFLE: Self = Self(16);
    pub const SHUFFLE_RELATIVE: Self = Self(32);
    pub const CLUSTERED: Self = Self(64);
    pub const QUAD: Self = Self(128);
}
#[doc = "Provided by [`crate::extensions::nv_shader_subgroup_partitioned`]"]
impl SubgroupFeatureFlagBits {
    pub const PARTITIONED_NV: Self = Self(256);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkTessellationDomainOrigin.html) · Enum"]
#[doc(alias = "VkTessellationDomainOrigin")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct TessellationDomainOrigin(pub i32);
impl std::fmt::Debug for TessellationDomainOrigin {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::UPPER_LEFT => "UPPER_LEFT",
            &Self::LOWER_LEFT => "LOWER_LEFT",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_1`]"]
impl TessellationDomainOrigin {
    pub const UPPER_LEFT: Self = Self(0);
    pub const LOWER_LEFT: Self = Self(1);
}
#[doc = "Provided by [`crate::extensions::khr_maintenance2`]"]
impl TessellationDomainOrigin {
    pub const UPPER_LEFT_KHR: Self = Self::UPPER_LEFT;
    pub const LOWER_LEFT_KHR: Self = Self::LOWER_LEFT;
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerYcbcrModelConversion.html) · Enum"]
#[doc(alias = "VkSamplerYcbcrModelConversion")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct SamplerYcbcrModelConversion(pub i32);
impl std::fmt::Debug for SamplerYcbcrModelConversion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::RGB_IDENTITY => "RGB_IDENTITY",
            &Self::YCBCR_IDENTITY => "YCBCR_IDENTITY",
            &Self::YCBCR_709 => "YCBCR_709",
            &Self::YCBCR_601 => "YCBCR_601",
            &Self::YCBCR_2020 => "YCBCR_2020",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_1`]"]
impl SamplerYcbcrModelConversion {
    pub const RGB_IDENTITY: Self = Self(0);
    pub const YCBCR_IDENTITY: Self = Self(1);
    pub const YCBCR_709: Self = Self(2);
    pub const YCBCR_601: Self = Self(3);
    pub const YCBCR_2020: Self = Self(4);
}
#[doc = "Provided by [`crate::extensions::khr_sampler_ycbcr_conversion`]"]
impl SamplerYcbcrModelConversion {
    pub const RGB_IDENTITY_KHR: Self = Self::RGB_IDENTITY;
    pub const YCBCR_IDENTITY_KHR: Self = Self::YCBCR_IDENTITY;
    pub const YCBCR_709_KHR: Self = Self::YCBCR_709;
    pub const YCBCR_601_KHR: Self = Self::YCBCR_601;
    pub const YCBCR_2020_KHR: Self = Self::YCBCR_2020;
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerYcbcrRange.html) · Enum"]
#[doc(alias = "VkSamplerYcbcrRange")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct SamplerYcbcrRange(pub i32);
impl std::fmt::Debug for SamplerYcbcrRange {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::ITU_FULL => "ITU_FULL",
            &Self::ITU_NARROW => "ITU_NARROW",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_1`]"]
impl SamplerYcbcrRange {
    pub const ITU_FULL: Self = Self(0);
    pub const ITU_NARROW: Self = Self(1);
}
#[doc = "Provided by [`crate::extensions::khr_sampler_ycbcr_conversion`]"]
impl SamplerYcbcrRange {
    pub const ITU_FULL_KHR: Self = Self::ITU_FULL;
    pub const ITU_NARROW_KHR: Self = Self::ITU_NARROW;
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkChromaLocation.html) · Enum"]
#[doc(alias = "VkChromaLocation")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct ChromaLocation(pub i32);
impl std::fmt::Debug for ChromaLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::COSITED_EVEN => "COSITED_EVEN",
            &Self::MIDPOINT => "MIDPOINT",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_1`]"]
impl ChromaLocation {
    pub const COSITED_EVEN: Self = Self(0);
    pub const MIDPOINT: Self = Self(1);
}
#[doc = "Provided by [`crate::extensions::khr_sampler_ycbcr_conversion`]"]
impl ChromaLocation {
    pub const COSITED_EVEN_KHR: Self = Self::COSITED_EVEN;
    pub const MIDPOINT_KHR: Self = Self::MIDPOINT;
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumerateInstanceVersion.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkEnumerateInstanceVersion = unsafe extern "system" fn(p_api_version: *mut u32) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceFeatures2.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceFeatures2 = unsafe extern "system" fn(physical_device: crate::vk1_0::PhysicalDevice, p_features: *mut crate::vk1_1::PhysicalDeviceFeatures2) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceProperties2.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceProperties2 = unsafe extern "system" fn(physical_device: crate::vk1_0::PhysicalDevice, p_properties: *mut crate::vk1_1::PhysicalDeviceProperties2) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceFormatProperties2.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceFormatProperties2 =
    unsafe extern "system" fn(physical_device: crate::vk1_0::PhysicalDevice, format: crate::vk1_0::Format, p_format_properties: *mut crate::vk1_1::FormatProperties2) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceImageFormatProperties2.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceImageFormatProperties2 = unsafe extern "system" fn(
    physical_device: crate::vk1_0::PhysicalDevice,
    p_image_format_info: *const crate::vk1_1::PhysicalDeviceImageFormatInfo2,
    p_image_format_properties: *mut crate::vk1_1::ImageFormatProperties2,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties2.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties2 =
    unsafe extern "system" fn(physical_device: crate::vk1_0::PhysicalDevice, p_queue_family_property_count: *mut u32, p_queue_family_properties: *mut crate::vk1_1::QueueFamilyProperties2) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceMemoryProperties2.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceMemoryProperties2 =
    unsafe extern "system" fn(physical_device: crate::vk1_0::PhysicalDevice, p_memory_properties: *mut crate::vk1_1::PhysicalDeviceMemoryProperties2) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSparseImageFormatProperties2.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties2 = unsafe extern "system" fn(
    physical_device: crate::vk1_0::PhysicalDevice,
    p_format_info: *const crate::vk1_1::PhysicalDeviceSparseImageFormatInfo2,
    p_property_count: *mut u32,
    p_properties: *mut crate::vk1_1::SparseImageFormatProperties2,
) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkTrimCommandPool.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkTrimCommandPool = unsafe extern "system" fn(device: crate::vk1_0::Device, command_pool: crate::vk1_0::CommandPool, flags: crate::vk1_1::CommandPoolTrimFlags) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceExternalBufferProperties.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceExternalBufferProperties = unsafe extern "system" fn(
    physical_device: crate::vk1_0::PhysicalDevice,
    p_external_buffer_info: *const crate::vk1_1::PhysicalDeviceExternalBufferInfo,
    p_external_buffer_properties: *mut crate::vk1_1::ExternalBufferProperties,
) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceExternalSemaphoreProperties.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceExternalSemaphoreProperties = unsafe extern "system" fn(
    physical_device: crate::vk1_0::PhysicalDevice,
    p_external_semaphore_info: *const crate::vk1_1::PhysicalDeviceExternalSemaphoreInfo,
    p_external_semaphore_properties: *mut crate::vk1_1::ExternalSemaphoreProperties,
) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceExternalFenceProperties.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceExternalFenceProperties = unsafe extern "system" fn(
    physical_device: crate::vk1_0::PhysicalDevice,
    p_external_fence_info: *const crate::vk1_1::PhysicalDeviceExternalFenceInfo,
    p_external_fence_properties: *mut crate::vk1_1::ExternalFenceProperties,
) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumeratePhysicalDeviceGroups.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkEnumeratePhysicalDeviceGroups = unsafe extern "system" fn(
    instance: crate::vk1_0::Instance,
    p_physical_device_group_count: *mut u32,
    p_physical_device_group_properties: *mut crate::vk1_1::PhysicalDeviceGroupProperties,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceGroupPeerMemoryFeatures.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceGroupPeerMemoryFeatures = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    heap_index: u32,
    local_device_index: u32,
    remote_device_index: u32,
    p_peer_memory_features: *mut crate::vk1_1::PeerMemoryFeatureFlags,
) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBindBufferMemory2.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkBindBufferMemory2 = unsafe extern "system" fn(device: crate::vk1_0::Device, bind_info_count: u32, p_bind_infos: *const crate::vk1_1::BindBufferMemoryInfo) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBindImageMemory2.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkBindImageMemory2 = unsafe extern "system" fn(device: crate::vk1_0::Device, bind_info_count: u32, p_bind_infos: *const crate::vk1_1::BindImageMemoryInfo) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDeviceMask.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDeviceMask = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, device_mask: u32) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDispatchBase.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDispatchBase =
    unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, base_group_x: u32, base_group_y: u32, base_group_z: u32, group_count_x: u32, group_count_y: u32, group_count_z: u32) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDescriptorUpdateTemplate.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateDescriptorUpdateTemplate = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_create_info: *const crate::vk1_1::DescriptorUpdateTemplateCreateInfo,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
    p_descriptor_update_template: *mut crate::vk1_1::DescriptorUpdateTemplate,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyDescriptorUpdateTemplate.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyDescriptorUpdateTemplate =
    unsafe extern "system" fn(device: crate::vk1_0::Device, descriptor_update_template: crate::vk1_1::DescriptorUpdateTemplate, p_allocator: *const crate::vk1_0::AllocationCallbacks) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkUpdateDescriptorSetWithTemplate.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkUpdateDescriptorSetWithTemplate = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    descriptor_set: crate::vk1_0::DescriptorSet,
    descriptor_update_template: crate::vk1_1::DescriptorUpdateTemplate,
    p_data: *const std::ffi::c_void,
) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetBufferMemoryRequirements2.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetBufferMemoryRequirements2 =
    unsafe extern "system" fn(device: crate::vk1_0::Device, p_info: *const crate::vk1_1::BufferMemoryRequirementsInfo2, p_memory_requirements: *mut crate::vk1_1::MemoryRequirements2) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageMemoryRequirements2.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageMemoryRequirements2 =
    unsafe extern "system" fn(device: crate::vk1_0::Device, p_info: *const crate::vk1_1::ImageMemoryRequirementsInfo2, p_memory_requirements: *mut crate::vk1_1::MemoryRequirements2) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageSparseMemoryRequirements2.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageSparseMemoryRequirements2 = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_info: *const crate::vk1_1::ImageSparseMemoryRequirementsInfo2,
    p_sparse_memory_requirement_count: *mut u32,
    p_sparse_memory_requirements: *mut crate::vk1_1::SparseImageMemoryRequirements2,
) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateSamplerYcbcrConversion.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateSamplerYcbcrConversion = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_create_info: *const crate::vk1_1::SamplerYcbcrConversionCreateInfo,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
    p_ycbcr_conversion: *mut crate::vk1_1::SamplerYcbcrConversion,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroySamplerYcbcrConversion.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroySamplerYcbcrConversion =
    unsafe extern "system" fn(device: crate::vk1_0::Device, ycbcr_conversion: crate::vk1_1::SamplerYcbcrConversion, p_allocator: *const crate::vk1_0::AllocationCallbacks) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceQueue2.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceQueue2 = unsafe extern "system" fn(device: crate::vk1_0::Device, p_queue_info: *const crate::vk1_1::DeviceQueueInfo2, p_queue: *mut crate::vk1_0::Queue) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDescriptorSetLayoutSupport.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDescriptorSetLayoutSupport =
    unsafe extern "system" fn(device: crate::vk1_0::Device, p_create_info: *const crate::vk1_0::DescriptorSetLayoutCreateInfo, p_support: *mut crate::vk1_1::DescriptorSetLayoutSupport) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceFeatures2.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceFeatures2")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceFeatures2 {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub features: crate::vk1_0::PhysicalDeviceFeatures,
}
impl Default for PhysicalDeviceFeatures2 {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_FEATURES_2,
            p_next: std::ptr::null_mut(),
            features: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceFeatures2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceFeatures2")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("features", &self.features)
            .finish()
    }
}
impl PhysicalDeviceFeatures2 {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceFeatures2Builder<'a> {
        PhysicalDeviceFeatures2Builder(self, std::marker::PhantomData)
    }
}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::nv_device_generated_commands::PhysicalDeviceDeviceGeneratedCommandsFeaturesNV> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::nv_device_generated_commands::PhysicalDeviceDeviceGeneratedCommandsFeaturesNVBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_private_data::PhysicalDevicePrivateDataFeaturesEXT> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_private_data::PhysicalDevicePrivateDataFeaturesEXTBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_1::PhysicalDeviceVariablePointersFeatures> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_1::PhysicalDeviceVariablePointersFeaturesBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_1::PhysicalDeviceMultiviewFeatures> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_1::PhysicalDeviceMultiviewFeaturesBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_1::PhysicalDevice16BitStorageFeatures> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_1::PhysicalDevice16BitStorageFeaturesBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::PhysicalDeviceShaderSubgroupExtendedTypesFeatures> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::PhysicalDeviceShaderSubgroupExtendedTypesFeaturesBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_1::PhysicalDeviceSamplerYcbcrConversionFeatures> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_1::PhysicalDeviceSamplerYcbcrConversionFeaturesBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_1::PhysicalDeviceProtectedMemoryFeatures> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_1::PhysicalDeviceProtectedMemoryFeaturesBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_blend_operation_advanced::PhysicalDeviceBlendOperationAdvancedFeaturesEXT> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_blend_operation_advanced::PhysicalDeviceBlendOperationAdvancedFeaturesEXTBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_inline_uniform_block::PhysicalDeviceInlineUniformBlockFeaturesEXT> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_inline_uniform_block::PhysicalDeviceInlineUniformBlockFeaturesEXTBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_1::PhysicalDeviceShaderDrawParametersFeatures> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_1::PhysicalDeviceShaderDrawParametersFeaturesBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::PhysicalDeviceShaderFloat16Int8Features> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::PhysicalDeviceShaderFloat16Int8FeaturesBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::PhysicalDeviceHostQueryResetFeatures> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::PhysicalDeviceHostQueryResetFeaturesBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_device_memory_report::PhysicalDeviceDeviceMemoryReportFeaturesEXT> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_device_memory_report::PhysicalDeviceDeviceMemoryReportFeaturesEXTBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::PhysicalDeviceDescriptorIndexingFeatures> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::PhysicalDeviceDescriptorIndexingFeaturesBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::PhysicalDeviceTimelineSemaphoreFeatures> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::PhysicalDeviceTimelineSemaphoreFeaturesBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::PhysicalDevice8BitStorageFeatures> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::PhysicalDevice8BitStorageFeaturesBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_conditional_rendering::PhysicalDeviceConditionalRenderingFeaturesEXT> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_conditional_rendering::PhysicalDeviceConditionalRenderingFeaturesEXTBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::PhysicalDeviceVulkanMemoryModelFeatures> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::PhysicalDeviceVulkanMemoryModelFeaturesBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::PhysicalDeviceShaderAtomicInt64Features> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::PhysicalDeviceShaderAtomicInt64FeaturesBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_shader_atomic_float::PhysicalDeviceShaderAtomicFloatFeaturesEXT> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_shader_atomic_float::PhysicalDeviceShaderAtomicFloatFeaturesEXTBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_vertex_attribute_divisor::PhysicalDeviceVertexAttributeDivisorFeaturesEXT> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_vertex_attribute_divisor::PhysicalDeviceVertexAttributeDivisorFeaturesEXTBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_astc_decode_mode::PhysicalDeviceASTCDecodeFeaturesEXT> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_astc_decode_mode::PhysicalDeviceASTCDecodeFeaturesEXTBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_transform_feedback::PhysicalDeviceTransformFeedbackFeaturesEXT> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_transform_feedback::PhysicalDeviceTransformFeedbackFeaturesEXTBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::nv_representative_fragment_test::PhysicalDeviceRepresentativeFragmentTestFeaturesNV> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::nv_representative_fragment_test::PhysicalDeviceRepresentativeFragmentTestFeaturesNVBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::nv_scissor_exclusive::PhysicalDeviceExclusiveScissorFeaturesNV> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::nv_scissor_exclusive::PhysicalDeviceExclusiveScissorFeaturesNVBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::nv_corner_sampled_image::PhysicalDeviceCornerSampledImageFeaturesNV> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::nv_corner_sampled_image::PhysicalDeviceCornerSampledImageFeaturesNVBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::nv_compute_shader_derivatives::PhysicalDeviceComputeShaderDerivativesFeaturesNV> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::nv_compute_shader_derivatives::PhysicalDeviceComputeShaderDerivativesFeaturesNVBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::nv_fragment_shader_barycentric::PhysicalDeviceFragmentShaderBarycentricFeaturesNV> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::nv_fragment_shader_barycentric::PhysicalDeviceFragmentShaderBarycentricFeaturesNVBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::nv_shader_image_footprint::PhysicalDeviceShaderImageFootprintFeaturesNV> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::nv_shader_image_footprint::PhysicalDeviceShaderImageFootprintFeaturesNVBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::nv_dedicated_allocation_image_aliasing::PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::nv_dedicated_allocation_image_aliasing::PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNVBuilder<'_>>
    for PhysicalDeviceFeatures2Builder<'a>
{
}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::nv_shading_rate_image::PhysicalDeviceShadingRateImageFeaturesNV> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::nv_shading_rate_image::PhysicalDeviceShadingRateImageFeaturesNVBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::nv_mesh_shader::PhysicalDeviceMeshShaderFeaturesNV> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::nv_mesh_shader::PhysicalDeviceMeshShaderFeaturesNVBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::khr_acceleration_structure::PhysicalDeviceAccelerationStructureFeaturesKHR> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::khr_acceleration_structure::PhysicalDeviceAccelerationStructureFeaturesKHRBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::khr_ray_tracing_pipeline::PhysicalDeviceRayTracingPipelineFeaturesKHR> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::khr_ray_tracing_pipeline::PhysicalDeviceRayTracingPipelineFeaturesKHRBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::khr_ray_query::PhysicalDeviceRayQueryFeaturesKHR> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::khr_ray_query::PhysicalDeviceRayQueryFeaturesKHRBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_fragment_density_map::PhysicalDeviceFragmentDensityMapFeaturesEXT> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_fragment_density_map::PhysicalDeviceFragmentDensityMapFeaturesEXTBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_fragment_density_map2::PhysicalDeviceFragmentDensityMap2FeaturesEXT> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_fragment_density_map2::PhysicalDeviceFragmentDensityMap2FeaturesEXTBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::PhysicalDeviceScalarBlockLayoutFeatures> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::PhysicalDeviceScalarBlockLayoutFeaturesBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::PhysicalDeviceUniformBufferStandardLayoutFeatures> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::PhysicalDeviceUniformBufferStandardLayoutFeaturesBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_depth_clip_enable::PhysicalDeviceDepthClipEnableFeaturesEXT> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_depth_clip_enable::PhysicalDeviceDepthClipEnableFeaturesEXTBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_memory_priority::PhysicalDeviceMemoryPriorityFeaturesEXT> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_memory_priority::PhysicalDeviceMemoryPriorityFeaturesEXTBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::PhysicalDeviceBufferDeviceAddressFeatures> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::PhysicalDeviceBufferDeviceAddressFeaturesBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_buffer_device_address::PhysicalDeviceBufferDeviceAddressFeaturesEXT> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_buffer_device_address::PhysicalDeviceBufferDeviceAddressFeaturesEXTBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::PhysicalDeviceImagelessFramebufferFeatures> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::PhysicalDeviceImagelessFramebufferFeaturesBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_texture_compression_astc_hdr::PhysicalDeviceTextureCompressionASTCHDRFeaturesEXT> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_texture_compression_astc_hdr::PhysicalDeviceTextureCompressionASTCHDRFeaturesEXTBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::nv_cooperative_matrix::PhysicalDeviceCooperativeMatrixFeaturesNV> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::nv_cooperative_matrix::PhysicalDeviceCooperativeMatrixFeaturesNVBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_ycbcr_image_arrays::PhysicalDeviceYcbcrImageArraysFeaturesEXT> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_ycbcr_image_arrays::PhysicalDeviceYcbcrImageArraysFeaturesEXTBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::khr_performance_query::PhysicalDevicePerformanceQueryFeaturesKHR> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::khr_performance_query::PhysicalDevicePerformanceQueryFeaturesKHRBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::nv_coverage_reduction_mode::PhysicalDeviceCoverageReductionModeFeaturesNV> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::nv_coverage_reduction_mode::PhysicalDeviceCoverageReductionModeFeaturesNVBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::intel_shader_integer_functions2::PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::intel_shader_integer_functions2::PhysicalDeviceShaderIntegerFunctions2FeaturesINTELBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::khr_shader_clock::PhysicalDeviceShaderClockFeaturesKHR> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::khr_shader_clock::PhysicalDeviceShaderClockFeaturesKHRBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_index_type_uint8::PhysicalDeviceIndexTypeUint8FeaturesEXT> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_index_type_uint8::PhysicalDeviceIndexTypeUint8FeaturesEXTBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::nv_shader_sm_builtins::PhysicalDeviceShaderSMBuiltinsFeaturesNV> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::nv_shader_sm_builtins::PhysicalDeviceShaderSMBuiltinsFeaturesNVBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_fragment_shader_interlock::PhysicalDeviceFragmentShaderInterlockFeaturesEXT> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_fragment_shader_interlock::PhysicalDeviceFragmentShaderInterlockFeaturesEXTBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::PhysicalDeviceSeparateDepthStencilLayoutsFeatures> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::PhysicalDeviceSeparateDepthStencilLayoutsFeaturesBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::khr_pipeline_executable_properties::PhysicalDevicePipelineExecutablePropertiesFeaturesKHR> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::khr_pipeline_executable_properties::PhysicalDevicePipelineExecutablePropertiesFeaturesKHRBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_shader_demote_to_helper_invocation::PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_shader_demote_to_helper_invocation::PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXTBuilder<'_>>
    for PhysicalDeviceFeatures2Builder<'a>
{
}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_texel_buffer_alignment::PhysicalDeviceTexelBufferAlignmentFeaturesEXT> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_texel_buffer_alignment::PhysicalDeviceTexelBufferAlignmentFeaturesEXTBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_subgroup_size_control::PhysicalDeviceSubgroupSizeControlFeaturesEXT> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_subgroup_size_control::PhysicalDeviceSubgroupSizeControlFeaturesEXTBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_line_rasterization::PhysicalDeviceLineRasterizationFeaturesEXT> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_line_rasterization::PhysicalDeviceLineRasterizationFeaturesEXTBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_pipeline_creation_cache_control::PhysicalDevicePipelineCreationCacheControlFeaturesEXT> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_pipeline_creation_cache_control::PhysicalDevicePipelineCreationCacheControlFeaturesEXTBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::PhysicalDeviceVulkan11Features> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::PhysicalDeviceVulkan11FeaturesBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::PhysicalDeviceVulkan12Features> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::PhysicalDeviceVulkan12FeaturesBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::amd_device_coherent_memory::PhysicalDeviceCoherentMemoryFeaturesAMD> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::amd_device_coherent_memory::PhysicalDeviceCoherentMemoryFeaturesAMDBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_custom_border_color::PhysicalDeviceCustomBorderColorFeaturesEXT> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_custom_border_color::PhysicalDeviceCustomBorderColorFeaturesEXTBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_extended_dynamic_state::PhysicalDeviceExtendedDynamicStateFeaturesEXT> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_extended_dynamic_state::PhysicalDeviceExtendedDynamicStateFeaturesEXTBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::nv_device_diagnostics_config::PhysicalDeviceDiagnosticsConfigFeaturesNV> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::nv_device_diagnostics_config::PhysicalDeviceDiagnosticsConfigFeaturesNVBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::khr_zero_initialize_workgroup_memory::PhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesKHR> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::khr_zero_initialize_workgroup_memory::PhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesKHRBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_robustness2::PhysicalDeviceRobustness2FeaturesEXT> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_robustness2::PhysicalDeviceRobustness2FeaturesEXTBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_image_robustness::PhysicalDeviceImageRobustnessFeaturesEXT> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_image_robustness::PhysicalDeviceImageRobustnessFeaturesEXTBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::khr_workgroup_memory_explicit_layout::PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::khr_workgroup_memory_explicit_layout::PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHRBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::khr_portability_subset::PhysicalDevicePortabilitySubsetFeaturesKHR> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::khr_portability_subset::PhysicalDevicePortabilitySubsetFeaturesKHRBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_4444_formats::PhysicalDevice4444FormatsFeaturesEXT> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_4444_formats::PhysicalDevice4444FormatsFeaturesEXTBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_shader_image_atomic_int64::PhysicalDeviceShaderImageAtomicInt64FeaturesEXT> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_shader_image_atomic_int64::PhysicalDeviceShaderImageAtomicInt64FeaturesEXTBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::khr_fragment_shading_rate::PhysicalDeviceFragmentShadingRateFeaturesKHR> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::khr_fragment_shading_rate::PhysicalDeviceFragmentShadingRateFeaturesKHRBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::khr_shader_terminate_invocation::PhysicalDeviceShaderTerminateInvocationFeaturesKHR> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::khr_shader_terminate_invocation::PhysicalDeviceShaderTerminateInvocationFeaturesKHRBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::nv_fragment_shading_rate_enums::PhysicalDeviceFragmentShadingRateEnumsFeaturesNV> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::nv_fragment_shading_rate_enums::PhysicalDeviceFragmentShadingRateEnumsFeaturesNVBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::valve_mutable_descriptor_type::PhysicalDeviceMutableDescriptorTypeFeaturesVALVE> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::valve_mutable_descriptor_type::PhysicalDeviceMutableDescriptorTypeFeaturesVALVEBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::khr_synchronization2::PhysicalDeviceSynchronization2FeaturesKHR> for PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::khr_synchronization2::PhysicalDeviceSynchronization2FeaturesKHRBuilder<'_>> for PhysicalDeviceFeatures2Builder<'a> {}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceFeatures2.html) · Builder of [`PhysicalDeviceFeatures2`]"]
#[repr(transparent)]
pub struct PhysicalDeviceFeatures2Builder<'a>(PhysicalDeviceFeatures2, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceFeatures2Builder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceFeatures2Builder<'a> {
        PhysicalDeviceFeatures2Builder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn features(mut self, features: crate::vk1_0::PhysicalDeviceFeatures) -> Self {
        self.0.features = features as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceFeatures2 {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceFeatures2Builder<'a> {
    fn default() -> PhysicalDeviceFeatures2Builder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceFeatures2Builder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceFeatures2Builder<'a> {
    type Target = PhysicalDeviceFeatures2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceFeatures2Builder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<PhysicalDeviceFeatures2> for PhysicalDeviceFeatures2Builder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceProperties2.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceProperties2")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceProperties2 {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub properties: crate::vk1_0::PhysicalDeviceProperties,
}
impl Default for PhysicalDeviceProperties2 {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_PROPERTIES_2,
            p_next: std::ptr::null_mut(),
            properties: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceProperties2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceProperties2")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("properties", &self.properties)
            .finish()
    }
}
impl PhysicalDeviceProperties2 {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceProperties2Builder<'a> {
        PhysicalDeviceProperties2Builder(self, std::marker::PhantomData)
    }
}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::nv_device_generated_commands::PhysicalDeviceDeviceGeneratedCommandsPropertiesNV> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::nv_device_generated_commands::PhysicalDeviceDeviceGeneratedCommandsPropertiesNVBuilder<'_>> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::khr_push_descriptor::PhysicalDevicePushDescriptorPropertiesKHR> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::khr_push_descriptor::PhysicalDevicePushDescriptorPropertiesKHRBuilder<'_>> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::PhysicalDeviceDriverProperties> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::PhysicalDeviceDriverPropertiesBuilder<'_>> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_1::PhysicalDeviceIDProperties> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_1::PhysicalDeviceIDPropertiesBuilder<'_>> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_1::PhysicalDeviceMultiviewProperties> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_1::PhysicalDeviceMultiviewPropertiesBuilder<'_>> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_discard_rectangles::PhysicalDeviceDiscardRectanglePropertiesEXT> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_discard_rectangles::PhysicalDeviceDiscardRectanglePropertiesEXTBuilder<'_>> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::nvx_multiview_per_view_attributes::PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::nvx_multiview_per_view_attributes::PhysicalDeviceMultiviewPerViewAttributesPropertiesNVXBuilder<'_>> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_1::PhysicalDeviceSubgroupProperties> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_1::PhysicalDeviceSubgroupPropertiesBuilder<'_>> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_1::PhysicalDevicePointClippingProperties> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_1::PhysicalDevicePointClippingPropertiesBuilder<'_>> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_1::PhysicalDeviceProtectedMemoryProperties> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_1::PhysicalDeviceProtectedMemoryPropertiesBuilder<'_>> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::PhysicalDeviceSamplerFilterMinmaxProperties> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::PhysicalDeviceSamplerFilterMinmaxPropertiesBuilder<'_>> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_sample_locations::PhysicalDeviceSampleLocationsPropertiesEXT> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_sample_locations::PhysicalDeviceSampleLocationsPropertiesEXTBuilder<'_>> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_blend_operation_advanced::PhysicalDeviceBlendOperationAdvancedPropertiesEXT> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_blend_operation_advanced::PhysicalDeviceBlendOperationAdvancedPropertiesEXTBuilder<'_>> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_inline_uniform_block::PhysicalDeviceInlineUniformBlockPropertiesEXT> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_inline_uniform_block::PhysicalDeviceInlineUniformBlockPropertiesEXTBuilder<'_>> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_1::PhysicalDeviceMaintenance3Properties> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_1::PhysicalDeviceMaintenance3PropertiesBuilder<'_>> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::PhysicalDeviceFloatControlsProperties> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::PhysicalDeviceFloatControlsPropertiesBuilder<'_>> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_external_memory_host::PhysicalDeviceExternalMemoryHostPropertiesEXT> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_external_memory_host::PhysicalDeviceExternalMemoryHostPropertiesEXTBuilder<'_>> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_conservative_rasterization::PhysicalDeviceConservativeRasterizationPropertiesEXT> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_conservative_rasterization::PhysicalDeviceConservativeRasterizationPropertiesEXTBuilder<'_>> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::amd_shader_core_properties::PhysicalDeviceShaderCorePropertiesAMD> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::amd_shader_core_properties::PhysicalDeviceShaderCorePropertiesAMDBuilder<'_>> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::amd_shader_core_properties2::PhysicalDeviceShaderCoreProperties2AMD> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::amd_shader_core_properties2::PhysicalDeviceShaderCoreProperties2AMDBuilder<'_>> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::PhysicalDeviceDescriptorIndexingProperties> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::PhysicalDeviceDescriptorIndexingPropertiesBuilder<'_>> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::PhysicalDeviceTimelineSemaphoreProperties> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::PhysicalDeviceTimelineSemaphorePropertiesBuilder<'_>> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_vertex_attribute_divisor::PhysicalDeviceVertexAttributeDivisorPropertiesEXT> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_vertex_attribute_divisor::PhysicalDeviceVertexAttributeDivisorPropertiesEXTBuilder<'_>> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_pci_bus_info::PhysicalDevicePCIBusInfoPropertiesEXT> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_pci_bus_info::PhysicalDevicePCIBusInfoPropertiesEXTBuilder<'_>> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::PhysicalDeviceDepthStencilResolveProperties> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::PhysicalDeviceDepthStencilResolvePropertiesBuilder<'_>> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_transform_feedback::PhysicalDeviceTransformFeedbackPropertiesEXT> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_transform_feedback::PhysicalDeviceTransformFeedbackPropertiesEXTBuilder<'_>> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::nv_shading_rate_image::PhysicalDeviceShadingRateImagePropertiesNV> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::nv_shading_rate_image::PhysicalDeviceShadingRateImagePropertiesNVBuilder<'_>> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::nv_mesh_shader::PhysicalDeviceMeshShaderPropertiesNV> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::nv_mesh_shader::PhysicalDeviceMeshShaderPropertiesNVBuilder<'_>> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::khr_acceleration_structure::PhysicalDeviceAccelerationStructurePropertiesKHR> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::khr_acceleration_structure::PhysicalDeviceAccelerationStructurePropertiesKHRBuilder<'_>> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::khr_ray_tracing_pipeline::PhysicalDeviceRayTracingPipelinePropertiesKHR> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::khr_ray_tracing_pipeline::PhysicalDeviceRayTracingPipelinePropertiesKHRBuilder<'_>> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::nv_ray_tracing::PhysicalDeviceRayTracingPropertiesNV> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::nv_ray_tracing::PhysicalDeviceRayTracingPropertiesNVBuilder<'_>> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_fragment_density_map::PhysicalDeviceFragmentDensityMapPropertiesEXT> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_fragment_density_map::PhysicalDeviceFragmentDensityMapPropertiesEXTBuilder<'_>> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_fragment_density_map2::PhysicalDeviceFragmentDensityMap2PropertiesEXT> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_fragment_density_map2::PhysicalDeviceFragmentDensityMap2PropertiesEXTBuilder<'_>> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::nv_cooperative_matrix::PhysicalDeviceCooperativeMatrixPropertiesNV> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::nv_cooperative_matrix::PhysicalDeviceCooperativeMatrixPropertiesNVBuilder<'_>> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::khr_performance_query::PhysicalDevicePerformanceQueryPropertiesKHR> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::khr_performance_query::PhysicalDevicePerformanceQueryPropertiesKHRBuilder<'_>> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::nv_shader_sm_builtins::PhysicalDeviceShaderSMBuiltinsPropertiesNV> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::nv_shader_sm_builtins::PhysicalDeviceShaderSMBuiltinsPropertiesNVBuilder<'_>> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_texel_buffer_alignment::PhysicalDeviceTexelBufferAlignmentPropertiesEXT> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_texel_buffer_alignment::PhysicalDeviceTexelBufferAlignmentPropertiesEXTBuilder<'_>> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_subgroup_size_control::PhysicalDeviceSubgroupSizeControlPropertiesEXT> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_subgroup_size_control::PhysicalDeviceSubgroupSizeControlPropertiesEXTBuilder<'_>> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_line_rasterization::PhysicalDeviceLineRasterizationPropertiesEXT> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_line_rasterization::PhysicalDeviceLineRasterizationPropertiesEXTBuilder<'_>> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::PhysicalDeviceVulkan11Properties> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::PhysicalDeviceVulkan11PropertiesBuilder<'_>> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::PhysicalDeviceVulkan12Properties> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::PhysicalDeviceVulkan12PropertiesBuilder<'_>> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_custom_border_color::PhysicalDeviceCustomBorderColorPropertiesEXT> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_custom_border_color::PhysicalDeviceCustomBorderColorPropertiesEXTBuilder<'_>> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_robustness2::PhysicalDeviceRobustness2PropertiesEXT> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_robustness2::PhysicalDeviceRobustness2PropertiesEXTBuilder<'_>> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::khr_portability_subset::PhysicalDevicePortabilitySubsetPropertiesKHR> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::khr_portability_subset::PhysicalDevicePortabilitySubsetPropertiesKHRBuilder<'_>> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::khr_fragment_shading_rate::PhysicalDeviceFragmentShadingRatePropertiesKHR> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::khr_fragment_shading_rate::PhysicalDeviceFragmentShadingRatePropertiesKHRBuilder<'_>> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::nv_fragment_shading_rate_enums::PhysicalDeviceFragmentShadingRateEnumsPropertiesNV> for PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::nv_fragment_shading_rate_enums::PhysicalDeviceFragmentShadingRateEnumsPropertiesNVBuilder<'_>> for PhysicalDeviceProperties2Builder<'a> {}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceProperties2.html) · Builder of [`PhysicalDeviceProperties2`]"]
#[repr(transparent)]
pub struct PhysicalDeviceProperties2Builder<'a>(PhysicalDeviceProperties2, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceProperties2Builder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceProperties2Builder<'a> {
        PhysicalDeviceProperties2Builder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn properties(mut self, properties: crate::vk1_0::PhysicalDeviceProperties) -> Self {
        self.0.properties = properties as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceProperties2 {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceProperties2Builder<'a> {
    fn default() -> PhysicalDeviceProperties2Builder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceProperties2Builder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceProperties2Builder<'a> {
    type Target = PhysicalDeviceProperties2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceProperties2Builder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<PhysicalDeviceProperties2> for PhysicalDeviceProperties2Builder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFormatProperties2.html) · Structure"]
#[doc(alias = "VkFormatProperties2")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FormatProperties2 {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub format_properties: crate::vk1_0::FormatProperties,
}
impl Default for FormatProperties2 {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::FORMAT_PROPERTIES_2,
            p_next: std::ptr::null_mut(),
            format_properties: Default::default(),
        }
    }
}
impl std::fmt::Debug for FormatProperties2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FormatProperties2")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("format_properties", &self.format_properties)
            .finish()
    }
}
impl FormatProperties2 {
    #[inline]
    pub fn into_builder<'a>(self) -> FormatProperties2Builder<'a> {
        FormatProperties2Builder(self, std::marker::PhantomData)
    }
}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_image_drm_format_modifier::DrmFormatModifierPropertiesListEXT> for FormatProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_image_drm_format_modifier::DrmFormatModifierPropertiesListEXTBuilder<'_>> for FormatProperties2Builder<'a> {}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFormatProperties2.html) · Builder of [`FormatProperties2`]"]
#[repr(transparent)]
pub struct FormatProperties2Builder<'a>(FormatProperties2, std::marker::PhantomData<&'a ()>);
impl<'a> FormatProperties2Builder<'a> {
    #[inline]
    pub fn new() -> FormatProperties2Builder<'a> {
        FormatProperties2Builder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn format_properties(mut self, format_properties: crate::vk1_0::FormatProperties) -> Self {
        self.0.format_properties = format_properties as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> FormatProperties2 {
        self.0
    }
}
impl<'a> std::default::Default for FormatProperties2Builder<'a> {
    fn default() -> FormatProperties2Builder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for FormatProperties2Builder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for FormatProperties2Builder<'a> {
    type Target = FormatProperties2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for FormatProperties2Builder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<FormatProperties2> for FormatProperties2Builder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageFormatProperties2.html) · Structure"]
#[doc(alias = "VkImageFormatProperties2")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImageFormatProperties2 {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub image_format_properties: crate::vk1_0::ImageFormatProperties,
}
impl Default for ImageFormatProperties2 {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::IMAGE_FORMAT_PROPERTIES_2,
            p_next: std::ptr::null_mut(),
            image_format_properties: Default::default(),
        }
    }
}
impl std::fmt::Debug for ImageFormatProperties2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ImageFormatProperties2")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("image_format_properties", &self.image_format_properties)
            .finish()
    }
}
impl ImageFormatProperties2 {
    #[inline]
    pub fn into_builder<'a>(self) -> ImageFormatProperties2Builder<'a> {
        ImageFormatProperties2Builder(self, std::marker::PhantomData)
    }
}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_1::ExternalImageFormatProperties> for ImageFormatProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_1::ExternalImageFormatPropertiesBuilder<'_>> for ImageFormatProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_1::SamplerYcbcrConversionImageFormatProperties> for ImageFormatProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_1::SamplerYcbcrConversionImageFormatPropertiesBuilder<'_>> for ImageFormatProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::amd_texture_gather_bias_lod::TextureLODGatherFormatPropertiesAMD> for ImageFormatProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::amd_texture_gather_bias_lod::TextureLODGatherFormatPropertiesAMDBuilder<'_>> for ImageFormatProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::android_external_memory_android_hardware_buffer::AndroidHardwareBufferUsageANDROID> for ImageFormatProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::android_external_memory_android_hardware_buffer::AndroidHardwareBufferUsageANDROIDBuilder<'_>> for ImageFormatProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_filter_cubic::FilterCubicImageViewImageFormatPropertiesEXT> for ImageFormatProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_filter_cubic::FilterCubicImageViewImageFormatPropertiesEXTBuilder<'_>> for ImageFormatProperties2Builder<'a> {}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageFormatProperties2.html) · Builder of [`ImageFormatProperties2`]"]
#[repr(transparent)]
pub struct ImageFormatProperties2Builder<'a>(ImageFormatProperties2, std::marker::PhantomData<&'a ()>);
impl<'a> ImageFormatProperties2Builder<'a> {
    #[inline]
    pub fn new() -> ImageFormatProperties2Builder<'a> {
        ImageFormatProperties2Builder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn image_format_properties(mut self, image_format_properties: crate::vk1_0::ImageFormatProperties) -> Self {
        self.0.image_format_properties = image_format_properties as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ImageFormatProperties2 {
        self.0
    }
}
impl<'a> std::default::Default for ImageFormatProperties2Builder<'a> {
    fn default() -> ImageFormatProperties2Builder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ImageFormatProperties2Builder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ImageFormatProperties2Builder<'a> {
    type Target = ImageFormatProperties2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ImageFormatProperties2Builder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<ImageFormatProperties2> for ImageFormatProperties2Builder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceImageFormatInfo2.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceImageFormatInfo2")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceImageFormatInfo2 {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub format: crate::vk1_0::Format,
    pub _type: crate::vk1_0::ImageType,
    pub tiling: crate::vk1_0::ImageTiling,
    pub usage: crate::vk1_0::ImageUsageFlags,
    pub flags: crate::vk1_0::ImageCreateFlags,
}
impl Default for PhysicalDeviceImageFormatInfo2 {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2,
            p_next: std::ptr::null(),
            format: Default::default(),
            _type: Default::default(),
            tiling: Default::default(),
            usage: Default::default(),
            flags: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceImageFormatInfo2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceImageFormatInfo2")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("format", &self.format)
            .field("_type", &self._type)
            .field("tiling", &self.tiling)
            .field("usage", &self.usage)
            .field("flags", &self.flags)
            .finish()
    }
}
impl PhysicalDeviceImageFormatInfo2 {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceImageFormatInfo2Builder<'a> {
        PhysicalDeviceImageFormatInfo2Builder(self, std::marker::PhantomData)
    }
}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_1::PhysicalDeviceExternalImageFormatInfo> for PhysicalDeviceImageFormatInfo2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_1::PhysicalDeviceExternalImageFormatInfoBuilder<'_>> for PhysicalDeviceImageFormatInfo2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::ImageFormatListCreateInfo> for PhysicalDeviceImageFormatInfo2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::ImageFormatListCreateInfoBuilder<'_>> for PhysicalDeviceImageFormatInfo2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_image_drm_format_modifier::PhysicalDeviceImageDrmFormatModifierInfoEXT> for PhysicalDeviceImageFormatInfo2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_image_drm_format_modifier::PhysicalDeviceImageDrmFormatModifierInfoEXTBuilder<'_>> for PhysicalDeviceImageFormatInfo2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::ImageStencilUsageCreateInfo> for PhysicalDeviceImageFormatInfo2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::ImageStencilUsageCreateInfoBuilder<'_>> for PhysicalDeviceImageFormatInfo2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_filter_cubic::PhysicalDeviceImageViewImageFormatInfoEXT> for PhysicalDeviceImageFormatInfo2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_filter_cubic::PhysicalDeviceImageViewImageFormatInfoEXTBuilder<'_>> for PhysicalDeviceImageFormatInfo2Builder<'a> {}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceImageFormatInfo2.html) · Builder of [`PhysicalDeviceImageFormatInfo2`]"]
#[repr(transparent)]
pub struct PhysicalDeviceImageFormatInfo2Builder<'a>(PhysicalDeviceImageFormatInfo2, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceImageFormatInfo2Builder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceImageFormatInfo2Builder<'a> {
        PhysicalDeviceImageFormatInfo2Builder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn format(mut self, format: crate::vk1_0::Format) -> Self {
        self.0.format = format as _;
        self
    }
    #[inline]
    pub fn _type(mut self, _type: crate::vk1_0::ImageType) -> Self {
        self.0._type = _type as _;
        self
    }
    #[inline]
    pub fn tiling(mut self, tiling: crate::vk1_0::ImageTiling) -> Self {
        self.0.tiling = tiling as _;
        self
    }
    #[inline]
    pub fn usage(mut self, usage: crate::vk1_0::ImageUsageFlags) -> Self {
        self.0.usage = usage as _;
        self
    }
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_0::ImageCreateFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceImageFormatInfo2 {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceImageFormatInfo2Builder<'a> {
    fn default() -> PhysicalDeviceImageFormatInfo2Builder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceImageFormatInfo2Builder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceImageFormatInfo2Builder<'a> {
    type Target = PhysicalDeviceImageFormatInfo2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceImageFormatInfo2Builder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<PhysicalDeviceImageFormatInfo2> for PhysicalDeviceImageFormatInfo2Builder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueueFamilyProperties2.html) · Structure"]
#[doc(alias = "VkQueueFamilyProperties2")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct QueueFamilyProperties2 {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub queue_family_properties: crate::vk1_0::QueueFamilyProperties,
}
impl Default for QueueFamilyProperties2 {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::QUEUE_FAMILY_PROPERTIES_2,
            p_next: std::ptr::null_mut(),
            queue_family_properties: Default::default(),
        }
    }
}
impl std::fmt::Debug for QueueFamilyProperties2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("QueueFamilyProperties2")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("queue_family_properties", &self.queue_family_properties)
            .finish()
    }
}
impl QueueFamilyProperties2 {
    #[inline]
    pub fn into_builder<'a>(self) -> QueueFamilyProperties2Builder<'a> {
        QueueFamilyProperties2Builder(self, std::marker::PhantomData)
    }
}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::nv_device_diagnostic_checkpoints::QueueFamilyCheckpointPropertiesNV> for QueueFamilyProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::nv_device_diagnostic_checkpoints::QueueFamilyCheckpointPropertiesNVBuilder<'_>> for QueueFamilyProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::khr_synchronization2::QueueFamilyCheckpointProperties2NV> for QueueFamilyProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::khr_synchronization2::QueueFamilyCheckpointProperties2NVBuilder<'_>> for QueueFamilyProperties2Builder<'a> {}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueueFamilyProperties2.html) · Builder of [`QueueFamilyProperties2`]"]
#[repr(transparent)]
pub struct QueueFamilyProperties2Builder<'a>(QueueFamilyProperties2, std::marker::PhantomData<&'a ()>);
impl<'a> QueueFamilyProperties2Builder<'a> {
    #[inline]
    pub fn new() -> QueueFamilyProperties2Builder<'a> {
        QueueFamilyProperties2Builder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn queue_family_properties(mut self, queue_family_properties: crate::vk1_0::QueueFamilyProperties) -> Self {
        self.0.queue_family_properties = queue_family_properties as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> QueueFamilyProperties2 {
        self.0
    }
}
impl<'a> std::default::Default for QueueFamilyProperties2Builder<'a> {
    fn default() -> QueueFamilyProperties2Builder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for QueueFamilyProperties2Builder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for QueueFamilyProperties2Builder<'a> {
    type Target = QueueFamilyProperties2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for QueueFamilyProperties2Builder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<QueueFamilyProperties2> for QueueFamilyProperties2Builder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMemoryProperties2.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceMemoryProperties2")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceMemoryProperties2 {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub memory_properties: crate::vk1_0::PhysicalDeviceMemoryProperties,
}
impl Default for PhysicalDeviceMemoryProperties2 {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_MEMORY_PROPERTIES_2,
            p_next: std::ptr::null_mut(),
            memory_properties: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceMemoryProperties2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceMemoryProperties2")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("memory_properties", &self.memory_properties)
            .finish()
    }
}
impl PhysicalDeviceMemoryProperties2 {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceMemoryProperties2Builder<'a> {
        PhysicalDeviceMemoryProperties2Builder(self, std::marker::PhantomData)
    }
}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_memory_budget::PhysicalDeviceMemoryBudgetPropertiesEXT> for PhysicalDeviceMemoryProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_memory_budget::PhysicalDeviceMemoryBudgetPropertiesEXTBuilder<'_>> for PhysicalDeviceMemoryProperties2Builder<'a> {}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMemoryProperties2.html) · Builder of [`PhysicalDeviceMemoryProperties2`]"]
#[repr(transparent)]
pub struct PhysicalDeviceMemoryProperties2Builder<'a>(PhysicalDeviceMemoryProperties2, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceMemoryProperties2Builder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceMemoryProperties2Builder<'a> {
        PhysicalDeviceMemoryProperties2Builder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn memory_properties(mut self, memory_properties: crate::vk1_0::PhysicalDeviceMemoryProperties) -> Self {
        self.0.memory_properties = memory_properties as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceMemoryProperties2 {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceMemoryProperties2Builder<'a> {
    fn default() -> PhysicalDeviceMemoryProperties2Builder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceMemoryProperties2Builder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceMemoryProperties2Builder<'a> {
    type Target = PhysicalDeviceMemoryProperties2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceMemoryProperties2Builder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<PhysicalDeviceMemoryProperties2> for PhysicalDeviceMemoryProperties2Builder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSparseImageFormatProperties2.html) · Structure"]
#[doc(alias = "VkSparseImageFormatProperties2")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SparseImageFormatProperties2 {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub properties: crate::vk1_0::SparseImageFormatProperties,
}
impl Default for SparseImageFormatProperties2 {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::SPARSE_IMAGE_FORMAT_PROPERTIES_2,
            p_next: std::ptr::null_mut(),
            properties: Default::default(),
        }
    }
}
impl std::fmt::Debug for SparseImageFormatProperties2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SparseImageFormatProperties2")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("properties", &self.properties)
            .finish()
    }
}
impl SparseImageFormatProperties2 {
    #[inline]
    pub fn into_builder<'a>(self) -> SparseImageFormatProperties2Builder<'a> {
        SparseImageFormatProperties2Builder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSparseImageFormatProperties2.html) · Builder of [`SparseImageFormatProperties2`]"]
#[repr(transparent)]
pub struct SparseImageFormatProperties2Builder<'a>(SparseImageFormatProperties2, std::marker::PhantomData<&'a ()>);
impl<'a> SparseImageFormatProperties2Builder<'a> {
    #[inline]
    pub fn new() -> SparseImageFormatProperties2Builder<'a> {
        SparseImageFormatProperties2Builder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn properties(mut self, properties: crate::vk1_0::SparseImageFormatProperties) -> Self {
        self.0.properties = properties as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> SparseImageFormatProperties2 {
        self.0
    }
}
impl<'a> std::default::Default for SparseImageFormatProperties2Builder<'a> {
    fn default() -> SparseImageFormatProperties2Builder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for SparseImageFormatProperties2Builder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for SparseImageFormatProperties2Builder<'a> {
    type Target = SparseImageFormatProperties2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SparseImageFormatProperties2Builder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<SparseImageFormatProperties2> for SparseImageFormatProperties2Builder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSparseImageFormatInfo2.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceSparseImageFormatInfo2")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceSparseImageFormatInfo2 {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub format: crate::vk1_0::Format,
    pub _type: crate::vk1_0::ImageType,
    pub samples: crate::vk1_0::SampleCountFlagBits,
    pub usage: crate::vk1_0::ImageUsageFlags,
    pub tiling: crate::vk1_0::ImageTiling,
}
impl Default for PhysicalDeviceSparseImageFormatInfo2 {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2,
            p_next: std::ptr::null(),
            format: Default::default(),
            _type: Default::default(),
            samples: Default::default(),
            usage: Default::default(),
            tiling: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceSparseImageFormatInfo2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceSparseImageFormatInfo2")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("format", &self.format)
            .field("_type", &self._type)
            .field("samples", &self.samples)
            .field("usage", &self.usage)
            .field("tiling", &self.tiling)
            .finish()
    }
}
impl PhysicalDeviceSparseImageFormatInfo2 {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceSparseImageFormatInfo2Builder<'a> {
        PhysicalDeviceSparseImageFormatInfo2Builder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSparseImageFormatInfo2.html) · Builder of [`PhysicalDeviceSparseImageFormatInfo2`]"]
#[repr(transparent)]
pub struct PhysicalDeviceSparseImageFormatInfo2Builder<'a>(PhysicalDeviceSparseImageFormatInfo2, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceSparseImageFormatInfo2Builder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceSparseImageFormatInfo2Builder<'a> {
        PhysicalDeviceSparseImageFormatInfo2Builder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn format(mut self, format: crate::vk1_0::Format) -> Self {
        self.0.format = format as _;
        self
    }
    #[inline]
    pub fn _type(mut self, _type: crate::vk1_0::ImageType) -> Self {
        self.0._type = _type as _;
        self
    }
    #[inline]
    pub fn samples(mut self, samples: crate::vk1_0::SampleCountFlagBits) -> Self {
        self.0.samples = samples as _;
        self
    }
    #[inline]
    pub fn usage(mut self, usage: crate::vk1_0::ImageUsageFlags) -> Self {
        self.0.usage = usage as _;
        self
    }
    #[inline]
    pub fn tiling(mut self, tiling: crate::vk1_0::ImageTiling) -> Self {
        self.0.tiling = tiling as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceSparseImageFormatInfo2 {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceSparseImageFormatInfo2Builder<'a> {
    fn default() -> PhysicalDeviceSparseImageFormatInfo2Builder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceSparseImageFormatInfo2Builder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceSparseImageFormatInfo2Builder<'a> {
    type Target = PhysicalDeviceSparseImageFormatInfo2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceSparseImageFormatInfo2Builder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<PhysicalDeviceSparseImageFormatInfo2> for PhysicalDeviceSparseImageFormatInfo2Builder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceVariablePointersFeatures.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceVariablePointersFeatures")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceVariablePointersFeatures {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub variable_pointers_storage_buffer: crate::vk1_0::Bool32,
    pub variable_pointers: crate::vk1_0::Bool32,
}
impl Default for PhysicalDeviceVariablePointersFeatures {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES,
            p_next: std::ptr::null_mut(),
            variable_pointers_storage_buffer: Default::default(),
            variable_pointers: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceVariablePointersFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceVariablePointersFeatures")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("variable_pointers_storage_buffer", &(self.variable_pointers_storage_buffer != 0))
            .field("variable_pointers", &(self.variable_pointers != 0))
            .finish()
    }
}
impl PhysicalDeviceVariablePointersFeatures {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceVariablePointersFeaturesBuilder<'a> {
        PhysicalDeviceVariablePointersFeaturesBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceVariablePointersFeatures.html) · Builder of [`PhysicalDeviceVariablePointersFeatures`]"]
#[repr(transparent)]
pub struct PhysicalDeviceVariablePointersFeaturesBuilder<'a>(PhysicalDeviceVariablePointersFeatures, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceVariablePointersFeaturesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceVariablePointersFeaturesBuilder<'a> {
        PhysicalDeviceVariablePointersFeaturesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn variable_pointers_storage_buffer(mut self, variable_pointers_storage_buffer: bool) -> Self {
        self.0.variable_pointers_storage_buffer = variable_pointers_storage_buffer as _;
        self
    }
    #[inline]
    pub fn variable_pointers(mut self, variable_pointers: bool) -> Self {
        self.0.variable_pointers = variable_pointers as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceVariablePointersFeatures {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceVariablePointersFeaturesBuilder<'a> {
    fn default() -> PhysicalDeviceVariablePointersFeaturesBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceVariablePointersFeaturesBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceVariablePointersFeaturesBuilder<'a> {
    type Target = PhysicalDeviceVariablePointersFeatures;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceVariablePointersFeaturesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<PhysicalDeviceVariablePointersFeatures> for PhysicalDeviceVariablePointersFeaturesBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalMemoryProperties.html) · Structure"]
#[doc(alias = "VkExternalMemoryProperties")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExternalMemoryProperties {
    pub external_memory_features: crate::vk1_1::ExternalMemoryFeatureFlags,
    pub export_from_imported_handle_types: crate::vk1_1::ExternalMemoryHandleTypeFlags,
    pub compatible_handle_types: crate::vk1_1::ExternalMemoryHandleTypeFlags,
}
impl Default for ExternalMemoryProperties {
    fn default() -> Self {
        Self {
            external_memory_features: Default::default(),
            export_from_imported_handle_types: Default::default(),
            compatible_handle_types: Default::default(),
        }
    }
}
impl std::fmt::Debug for ExternalMemoryProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ExternalMemoryProperties")
            .field("external_memory_features", &self.external_memory_features)
            .field("export_from_imported_handle_types", &self.export_from_imported_handle_types)
            .field("compatible_handle_types", &self.compatible_handle_types)
            .finish()
    }
}
impl ExternalMemoryProperties {
    #[inline]
    pub fn into_builder<'a>(self) -> ExternalMemoryPropertiesBuilder<'a> {
        ExternalMemoryPropertiesBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalMemoryProperties.html) · Builder of [`ExternalMemoryProperties`]"]
#[repr(transparent)]
pub struct ExternalMemoryPropertiesBuilder<'a>(ExternalMemoryProperties, std::marker::PhantomData<&'a ()>);
impl<'a> ExternalMemoryPropertiesBuilder<'a> {
    #[inline]
    pub fn new() -> ExternalMemoryPropertiesBuilder<'a> {
        ExternalMemoryPropertiesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn external_memory_features(mut self, external_memory_features: crate::vk1_1::ExternalMemoryFeatureFlags) -> Self {
        self.0.external_memory_features = external_memory_features as _;
        self
    }
    #[inline]
    pub fn export_from_imported_handle_types(mut self, export_from_imported_handle_types: crate::vk1_1::ExternalMemoryHandleTypeFlags) -> Self {
        self.0.export_from_imported_handle_types = export_from_imported_handle_types as _;
        self
    }
    #[inline]
    pub fn compatible_handle_types(mut self, compatible_handle_types: crate::vk1_1::ExternalMemoryHandleTypeFlags) -> Self {
        self.0.compatible_handle_types = compatible_handle_types as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ExternalMemoryProperties {
        self.0
    }
}
impl<'a> std::default::Default for ExternalMemoryPropertiesBuilder<'a> {
    fn default() -> ExternalMemoryPropertiesBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ExternalMemoryPropertiesBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ExternalMemoryPropertiesBuilder<'a> {
    type Target = ExternalMemoryProperties;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ExternalMemoryPropertiesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<ExternalMemoryProperties> for ExternalMemoryPropertiesBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceExternalImageFormatInfo.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceExternalImageFormatInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceExternalImageFormatInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub handle_type: crate::vk1_1::ExternalMemoryHandleTypeFlagBits,
}
impl Default for PhysicalDeviceExternalImageFormatInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO,
            p_next: std::ptr::null(),
            handle_type: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceExternalImageFormatInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceExternalImageFormatInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("handle_type", &self.handle_type)
            .finish()
    }
}
impl PhysicalDeviceExternalImageFormatInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceExternalImageFormatInfoBuilder<'a> {
        PhysicalDeviceExternalImageFormatInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceExternalImageFormatInfo.html) · Builder of [`PhysicalDeviceExternalImageFormatInfo`]"]
#[repr(transparent)]
pub struct PhysicalDeviceExternalImageFormatInfoBuilder<'a>(PhysicalDeviceExternalImageFormatInfo, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceExternalImageFormatInfoBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceExternalImageFormatInfoBuilder<'a> {
        PhysicalDeviceExternalImageFormatInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn handle_type(mut self, handle_type: crate::vk1_1::ExternalMemoryHandleTypeFlagBits) -> Self {
        self.0.handle_type = handle_type as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceExternalImageFormatInfo {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceExternalImageFormatInfoBuilder<'a> {
    fn default() -> PhysicalDeviceExternalImageFormatInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceExternalImageFormatInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceExternalImageFormatInfoBuilder<'a> {
    type Target = PhysicalDeviceExternalImageFormatInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceExternalImageFormatInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<PhysicalDeviceExternalImageFormatInfo> for PhysicalDeviceExternalImageFormatInfoBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalImageFormatProperties.html) · Structure"]
#[doc(alias = "VkExternalImageFormatProperties")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExternalImageFormatProperties {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub external_memory_properties: crate::vk1_1::ExternalMemoryProperties,
}
impl Default for ExternalImageFormatProperties {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::EXTERNAL_IMAGE_FORMAT_PROPERTIES,
            p_next: std::ptr::null_mut(),
            external_memory_properties: Default::default(),
        }
    }
}
impl std::fmt::Debug for ExternalImageFormatProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ExternalImageFormatProperties")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("external_memory_properties", &self.external_memory_properties)
            .finish()
    }
}
impl ExternalImageFormatProperties {
    #[inline]
    pub fn into_builder<'a>(self) -> ExternalImageFormatPropertiesBuilder<'a> {
        ExternalImageFormatPropertiesBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalImageFormatProperties.html) · Builder of [`ExternalImageFormatProperties`]"]
#[repr(transparent)]
pub struct ExternalImageFormatPropertiesBuilder<'a>(ExternalImageFormatProperties, std::marker::PhantomData<&'a ()>);
impl<'a> ExternalImageFormatPropertiesBuilder<'a> {
    #[inline]
    pub fn new() -> ExternalImageFormatPropertiesBuilder<'a> {
        ExternalImageFormatPropertiesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn external_memory_properties(mut self, external_memory_properties: crate::vk1_1::ExternalMemoryProperties) -> Self {
        self.0.external_memory_properties = external_memory_properties as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ExternalImageFormatProperties {
        self.0
    }
}
impl<'a> std::default::Default for ExternalImageFormatPropertiesBuilder<'a> {
    fn default() -> ExternalImageFormatPropertiesBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ExternalImageFormatPropertiesBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ExternalImageFormatPropertiesBuilder<'a> {
    type Target = ExternalImageFormatProperties;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ExternalImageFormatPropertiesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<ExternalImageFormatProperties> for ExternalImageFormatPropertiesBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceExternalBufferInfo.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceExternalBufferInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceExternalBufferInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::vk1_0::BufferCreateFlags,
    pub usage: crate::vk1_0::BufferUsageFlags,
    pub handle_type: crate::vk1_1::ExternalMemoryHandleTypeFlagBits,
}
impl Default for PhysicalDeviceExternalBufferInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            usage: Default::default(),
            handle_type: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceExternalBufferInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceExternalBufferInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("usage", &self.usage)
            .field("handle_type", &self.handle_type)
            .finish()
    }
}
impl PhysicalDeviceExternalBufferInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceExternalBufferInfoBuilder<'a> {
        PhysicalDeviceExternalBufferInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceExternalBufferInfo.html) · Builder of [`PhysicalDeviceExternalBufferInfo`]"]
#[repr(transparent)]
pub struct PhysicalDeviceExternalBufferInfoBuilder<'a>(PhysicalDeviceExternalBufferInfo, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceExternalBufferInfoBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceExternalBufferInfoBuilder<'a> {
        PhysicalDeviceExternalBufferInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_0::BufferCreateFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn usage(mut self, usage: crate::vk1_0::BufferUsageFlags) -> Self {
        self.0.usage = usage as _;
        self
    }
    #[inline]
    pub fn handle_type(mut self, handle_type: crate::vk1_1::ExternalMemoryHandleTypeFlagBits) -> Self {
        self.0.handle_type = handle_type as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceExternalBufferInfo {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceExternalBufferInfoBuilder<'a> {
    fn default() -> PhysicalDeviceExternalBufferInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceExternalBufferInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceExternalBufferInfoBuilder<'a> {
    type Target = PhysicalDeviceExternalBufferInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceExternalBufferInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<PhysicalDeviceExternalBufferInfo> for PhysicalDeviceExternalBufferInfoBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalBufferProperties.html) · Structure"]
#[doc(alias = "VkExternalBufferProperties")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExternalBufferProperties {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub external_memory_properties: crate::vk1_1::ExternalMemoryProperties,
}
impl Default for ExternalBufferProperties {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::EXTERNAL_BUFFER_PROPERTIES,
            p_next: std::ptr::null_mut(),
            external_memory_properties: Default::default(),
        }
    }
}
impl std::fmt::Debug for ExternalBufferProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ExternalBufferProperties")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("external_memory_properties", &self.external_memory_properties)
            .finish()
    }
}
impl ExternalBufferProperties {
    #[inline]
    pub fn into_builder<'a>(self) -> ExternalBufferPropertiesBuilder<'a> {
        ExternalBufferPropertiesBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalBufferProperties.html) · Builder of [`ExternalBufferProperties`]"]
#[repr(transparent)]
pub struct ExternalBufferPropertiesBuilder<'a>(ExternalBufferProperties, std::marker::PhantomData<&'a ()>);
impl<'a> ExternalBufferPropertiesBuilder<'a> {
    #[inline]
    pub fn new() -> ExternalBufferPropertiesBuilder<'a> {
        ExternalBufferPropertiesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn external_memory_properties(mut self, external_memory_properties: crate::vk1_1::ExternalMemoryProperties) -> Self {
        self.0.external_memory_properties = external_memory_properties as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ExternalBufferProperties {
        self.0
    }
}
impl<'a> std::default::Default for ExternalBufferPropertiesBuilder<'a> {
    fn default() -> ExternalBufferPropertiesBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ExternalBufferPropertiesBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ExternalBufferPropertiesBuilder<'a> {
    type Target = ExternalBufferProperties;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ExternalBufferPropertiesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<ExternalBufferProperties> for ExternalBufferPropertiesBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceIDProperties.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceIDProperties")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceIDProperties {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub device_uuid: [u8; 16],
    pub driver_uuid: [u8; 16],
    pub device_luid: [u8; 8],
    pub device_node_mask: u32,
    pub device_luid_valid: crate::vk1_0::Bool32,
}
impl Default for PhysicalDeviceIDProperties {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_ID_PROPERTIES,
            p_next: std::ptr::null_mut(),
            device_uuid: unsafe { std::mem::zeroed() },
            driver_uuid: unsafe { std::mem::zeroed() },
            device_luid: unsafe { std::mem::zeroed() },
            device_node_mask: Default::default(),
            device_luid_valid: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceIDProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceIDProperties")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("device_uuid", &self.device_uuid)
            .field("driver_uuid", &self.driver_uuid)
            .field("device_luid", &self.device_luid)
            .field("device_node_mask", &self.device_node_mask)
            .field("device_luid_valid", &(self.device_luid_valid != 0))
            .finish()
    }
}
impl PhysicalDeviceIDProperties {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceIDPropertiesBuilder<'a> {
        PhysicalDeviceIDPropertiesBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceIDProperties.html) · Builder of [`PhysicalDeviceIDProperties`]"]
#[repr(transparent)]
pub struct PhysicalDeviceIDPropertiesBuilder<'a>(PhysicalDeviceIDProperties, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceIDPropertiesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceIDPropertiesBuilder<'a> {
        PhysicalDeviceIDPropertiesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn device_uuid(mut self, device_uuid: [u8; 16]) -> Self {
        self.0.device_uuid = device_uuid as _;
        self
    }
    #[inline]
    pub fn driver_uuid(mut self, driver_uuid: [u8; 16]) -> Self {
        self.0.driver_uuid = driver_uuid as _;
        self
    }
    #[inline]
    pub fn device_luid(mut self, device_luid: [u8; 8]) -> Self {
        self.0.device_luid = device_luid as _;
        self
    }
    #[inline]
    pub fn device_node_mask(mut self, device_node_mask: u32) -> Self {
        self.0.device_node_mask = device_node_mask as _;
        self
    }
    #[inline]
    pub fn device_luid_valid(mut self, device_luid_valid: bool) -> Self {
        self.0.device_luid_valid = device_luid_valid as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceIDProperties {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceIDPropertiesBuilder<'a> {
    fn default() -> PhysicalDeviceIDPropertiesBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceIDPropertiesBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceIDPropertiesBuilder<'a> {
    type Target = PhysicalDeviceIDProperties;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceIDPropertiesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<PhysicalDeviceIDProperties> for PhysicalDeviceIDPropertiesBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalMemoryImageCreateInfo.html) · Structure"]
#[doc(alias = "VkExternalMemoryImageCreateInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExternalMemoryImageCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub handle_types: crate::vk1_1::ExternalMemoryHandleTypeFlags,
}
impl Default for ExternalMemoryImageCreateInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::EXTERNAL_MEMORY_IMAGE_CREATE_INFO,
            p_next: std::ptr::null(),
            handle_types: Default::default(),
        }
    }
}
impl std::fmt::Debug for ExternalMemoryImageCreateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ExternalMemoryImageCreateInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("handle_types", &self.handle_types)
            .finish()
    }
}
impl ExternalMemoryImageCreateInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> ExternalMemoryImageCreateInfoBuilder<'a> {
        ExternalMemoryImageCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalMemoryImageCreateInfo.html) · Builder of [`ExternalMemoryImageCreateInfo`]"]
#[repr(transparent)]
pub struct ExternalMemoryImageCreateInfoBuilder<'a>(ExternalMemoryImageCreateInfo, std::marker::PhantomData<&'a ()>);
impl<'a> ExternalMemoryImageCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> ExternalMemoryImageCreateInfoBuilder<'a> {
        ExternalMemoryImageCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn handle_types(mut self, handle_types: crate::vk1_1::ExternalMemoryHandleTypeFlags) -> Self {
        self.0.handle_types = handle_types as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ExternalMemoryImageCreateInfo {
        self.0
    }
}
impl<'a> std::default::Default for ExternalMemoryImageCreateInfoBuilder<'a> {
    fn default() -> ExternalMemoryImageCreateInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ExternalMemoryImageCreateInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ExternalMemoryImageCreateInfoBuilder<'a> {
    type Target = ExternalMemoryImageCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ExternalMemoryImageCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<ExternalMemoryImageCreateInfo> for ExternalMemoryImageCreateInfoBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalMemoryBufferCreateInfo.html) · Structure"]
#[doc(alias = "VkExternalMemoryBufferCreateInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExternalMemoryBufferCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub handle_types: crate::vk1_1::ExternalMemoryHandleTypeFlags,
}
impl Default for ExternalMemoryBufferCreateInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::EXTERNAL_MEMORY_BUFFER_CREATE_INFO,
            p_next: std::ptr::null(),
            handle_types: Default::default(),
        }
    }
}
impl std::fmt::Debug for ExternalMemoryBufferCreateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ExternalMemoryBufferCreateInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("handle_types", &self.handle_types)
            .finish()
    }
}
impl ExternalMemoryBufferCreateInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> ExternalMemoryBufferCreateInfoBuilder<'a> {
        ExternalMemoryBufferCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalMemoryBufferCreateInfo.html) · Builder of [`ExternalMemoryBufferCreateInfo`]"]
#[repr(transparent)]
pub struct ExternalMemoryBufferCreateInfoBuilder<'a>(ExternalMemoryBufferCreateInfo, std::marker::PhantomData<&'a ()>);
impl<'a> ExternalMemoryBufferCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> ExternalMemoryBufferCreateInfoBuilder<'a> {
        ExternalMemoryBufferCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn handle_types(mut self, handle_types: crate::vk1_1::ExternalMemoryHandleTypeFlags) -> Self {
        self.0.handle_types = handle_types as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ExternalMemoryBufferCreateInfo {
        self.0
    }
}
impl<'a> std::default::Default for ExternalMemoryBufferCreateInfoBuilder<'a> {
    fn default() -> ExternalMemoryBufferCreateInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ExternalMemoryBufferCreateInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ExternalMemoryBufferCreateInfoBuilder<'a> {
    type Target = ExternalMemoryBufferCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ExternalMemoryBufferCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<ExternalMemoryBufferCreateInfo> for ExternalMemoryBufferCreateInfoBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExportMemoryAllocateInfo.html) · Structure"]
#[doc(alias = "VkExportMemoryAllocateInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExportMemoryAllocateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub handle_types: crate::vk1_1::ExternalMemoryHandleTypeFlags,
}
impl Default for ExportMemoryAllocateInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::EXPORT_MEMORY_ALLOCATE_INFO,
            p_next: std::ptr::null(),
            handle_types: Default::default(),
        }
    }
}
impl std::fmt::Debug for ExportMemoryAllocateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ExportMemoryAllocateInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("handle_types", &self.handle_types)
            .finish()
    }
}
impl ExportMemoryAllocateInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> ExportMemoryAllocateInfoBuilder<'a> {
        ExportMemoryAllocateInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExportMemoryAllocateInfo.html) · Builder of [`ExportMemoryAllocateInfo`]"]
#[repr(transparent)]
pub struct ExportMemoryAllocateInfoBuilder<'a>(ExportMemoryAllocateInfo, std::marker::PhantomData<&'a ()>);
impl<'a> ExportMemoryAllocateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> ExportMemoryAllocateInfoBuilder<'a> {
        ExportMemoryAllocateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn handle_types(mut self, handle_types: crate::vk1_1::ExternalMemoryHandleTypeFlags) -> Self {
        self.0.handle_types = handle_types as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ExportMemoryAllocateInfo {
        self.0
    }
}
impl<'a> std::default::Default for ExportMemoryAllocateInfoBuilder<'a> {
    fn default() -> ExportMemoryAllocateInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ExportMemoryAllocateInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ExportMemoryAllocateInfoBuilder<'a> {
    type Target = ExportMemoryAllocateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ExportMemoryAllocateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<ExportMemoryAllocateInfo> for ExportMemoryAllocateInfoBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceExternalSemaphoreInfo.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceExternalSemaphoreInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceExternalSemaphoreInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub handle_type: crate::vk1_1::ExternalSemaphoreHandleTypeFlagBits,
}
impl Default for PhysicalDeviceExternalSemaphoreInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO,
            p_next: std::ptr::null(),
            handle_type: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceExternalSemaphoreInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceExternalSemaphoreInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("handle_type", &self.handle_type)
            .finish()
    }
}
impl PhysicalDeviceExternalSemaphoreInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceExternalSemaphoreInfoBuilder<'a> {
        PhysicalDeviceExternalSemaphoreInfoBuilder(self, std::marker::PhantomData)
    }
}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::SemaphoreTypeCreateInfo> for PhysicalDeviceExternalSemaphoreInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::SemaphoreTypeCreateInfoBuilder<'_>> for PhysicalDeviceExternalSemaphoreInfoBuilder<'a> {}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceExternalSemaphoreInfo.html) · Builder of [`PhysicalDeviceExternalSemaphoreInfo`]"]
#[repr(transparent)]
pub struct PhysicalDeviceExternalSemaphoreInfoBuilder<'a>(PhysicalDeviceExternalSemaphoreInfo, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceExternalSemaphoreInfoBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceExternalSemaphoreInfoBuilder<'a> {
        PhysicalDeviceExternalSemaphoreInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn handle_type(mut self, handle_type: crate::vk1_1::ExternalSemaphoreHandleTypeFlagBits) -> Self {
        self.0.handle_type = handle_type as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceExternalSemaphoreInfo {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceExternalSemaphoreInfoBuilder<'a> {
    fn default() -> PhysicalDeviceExternalSemaphoreInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceExternalSemaphoreInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceExternalSemaphoreInfoBuilder<'a> {
    type Target = PhysicalDeviceExternalSemaphoreInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceExternalSemaphoreInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<PhysicalDeviceExternalSemaphoreInfo> for PhysicalDeviceExternalSemaphoreInfoBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalSemaphoreProperties.html) · Structure"]
#[doc(alias = "VkExternalSemaphoreProperties")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExternalSemaphoreProperties {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub export_from_imported_handle_types: crate::vk1_1::ExternalSemaphoreHandleTypeFlags,
    pub compatible_handle_types: crate::vk1_1::ExternalSemaphoreHandleTypeFlags,
    pub external_semaphore_features: crate::vk1_1::ExternalSemaphoreFeatureFlags,
}
impl Default for ExternalSemaphoreProperties {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::EXTERNAL_SEMAPHORE_PROPERTIES,
            p_next: std::ptr::null_mut(),
            export_from_imported_handle_types: Default::default(),
            compatible_handle_types: Default::default(),
            external_semaphore_features: Default::default(),
        }
    }
}
impl std::fmt::Debug for ExternalSemaphoreProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ExternalSemaphoreProperties")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("export_from_imported_handle_types", &self.export_from_imported_handle_types)
            .field("compatible_handle_types", &self.compatible_handle_types)
            .field("external_semaphore_features", &self.external_semaphore_features)
            .finish()
    }
}
impl ExternalSemaphoreProperties {
    #[inline]
    pub fn into_builder<'a>(self) -> ExternalSemaphorePropertiesBuilder<'a> {
        ExternalSemaphorePropertiesBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalSemaphoreProperties.html) · Builder of [`ExternalSemaphoreProperties`]"]
#[repr(transparent)]
pub struct ExternalSemaphorePropertiesBuilder<'a>(ExternalSemaphoreProperties, std::marker::PhantomData<&'a ()>);
impl<'a> ExternalSemaphorePropertiesBuilder<'a> {
    #[inline]
    pub fn new() -> ExternalSemaphorePropertiesBuilder<'a> {
        ExternalSemaphorePropertiesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn export_from_imported_handle_types(mut self, export_from_imported_handle_types: crate::vk1_1::ExternalSemaphoreHandleTypeFlags) -> Self {
        self.0.export_from_imported_handle_types = export_from_imported_handle_types as _;
        self
    }
    #[inline]
    pub fn compatible_handle_types(mut self, compatible_handle_types: crate::vk1_1::ExternalSemaphoreHandleTypeFlags) -> Self {
        self.0.compatible_handle_types = compatible_handle_types as _;
        self
    }
    #[inline]
    pub fn external_semaphore_features(mut self, external_semaphore_features: crate::vk1_1::ExternalSemaphoreFeatureFlags) -> Self {
        self.0.external_semaphore_features = external_semaphore_features as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ExternalSemaphoreProperties {
        self.0
    }
}
impl<'a> std::default::Default for ExternalSemaphorePropertiesBuilder<'a> {
    fn default() -> ExternalSemaphorePropertiesBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ExternalSemaphorePropertiesBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ExternalSemaphorePropertiesBuilder<'a> {
    type Target = ExternalSemaphoreProperties;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ExternalSemaphorePropertiesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<ExternalSemaphoreProperties> for ExternalSemaphorePropertiesBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExportSemaphoreCreateInfo.html) · Structure"]
#[doc(alias = "VkExportSemaphoreCreateInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExportSemaphoreCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub handle_types: crate::vk1_1::ExternalSemaphoreHandleTypeFlags,
}
impl Default for ExportSemaphoreCreateInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::EXPORT_SEMAPHORE_CREATE_INFO,
            p_next: std::ptr::null(),
            handle_types: Default::default(),
        }
    }
}
impl std::fmt::Debug for ExportSemaphoreCreateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ExportSemaphoreCreateInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("handle_types", &self.handle_types)
            .finish()
    }
}
impl ExportSemaphoreCreateInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> ExportSemaphoreCreateInfoBuilder<'a> {
        ExportSemaphoreCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExportSemaphoreCreateInfo.html) · Builder of [`ExportSemaphoreCreateInfo`]"]
#[repr(transparent)]
pub struct ExportSemaphoreCreateInfoBuilder<'a>(ExportSemaphoreCreateInfo, std::marker::PhantomData<&'a ()>);
impl<'a> ExportSemaphoreCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> ExportSemaphoreCreateInfoBuilder<'a> {
        ExportSemaphoreCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn handle_types(mut self, handle_types: crate::vk1_1::ExternalSemaphoreHandleTypeFlags) -> Self {
        self.0.handle_types = handle_types as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ExportSemaphoreCreateInfo {
        self.0
    }
}
impl<'a> std::default::Default for ExportSemaphoreCreateInfoBuilder<'a> {
    fn default() -> ExportSemaphoreCreateInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ExportSemaphoreCreateInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ExportSemaphoreCreateInfoBuilder<'a> {
    type Target = ExportSemaphoreCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ExportSemaphoreCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<ExportSemaphoreCreateInfo> for ExportSemaphoreCreateInfoBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceExternalFenceInfo.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceExternalFenceInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceExternalFenceInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub handle_type: crate::vk1_1::ExternalFenceHandleTypeFlagBits,
}
impl Default for PhysicalDeviceExternalFenceInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO,
            p_next: std::ptr::null(),
            handle_type: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceExternalFenceInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceExternalFenceInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("handle_type", &self.handle_type)
            .finish()
    }
}
impl PhysicalDeviceExternalFenceInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceExternalFenceInfoBuilder<'a> {
        PhysicalDeviceExternalFenceInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceExternalFenceInfo.html) · Builder of [`PhysicalDeviceExternalFenceInfo`]"]
#[repr(transparent)]
pub struct PhysicalDeviceExternalFenceInfoBuilder<'a>(PhysicalDeviceExternalFenceInfo, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceExternalFenceInfoBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceExternalFenceInfoBuilder<'a> {
        PhysicalDeviceExternalFenceInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn handle_type(mut self, handle_type: crate::vk1_1::ExternalFenceHandleTypeFlagBits) -> Self {
        self.0.handle_type = handle_type as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceExternalFenceInfo {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceExternalFenceInfoBuilder<'a> {
    fn default() -> PhysicalDeviceExternalFenceInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceExternalFenceInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceExternalFenceInfoBuilder<'a> {
    type Target = PhysicalDeviceExternalFenceInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceExternalFenceInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<PhysicalDeviceExternalFenceInfo> for PhysicalDeviceExternalFenceInfoBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalFenceProperties.html) · Structure"]
#[doc(alias = "VkExternalFenceProperties")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExternalFenceProperties {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub export_from_imported_handle_types: crate::vk1_1::ExternalFenceHandleTypeFlags,
    pub compatible_handle_types: crate::vk1_1::ExternalFenceHandleTypeFlags,
    pub external_fence_features: crate::vk1_1::ExternalFenceFeatureFlags,
}
impl Default for ExternalFenceProperties {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::EXTERNAL_FENCE_PROPERTIES,
            p_next: std::ptr::null_mut(),
            export_from_imported_handle_types: Default::default(),
            compatible_handle_types: Default::default(),
            external_fence_features: Default::default(),
        }
    }
}
impl std::fmt::Debug for ExternalFenceProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ExternalFenceProperties")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("export_from_imported_handle_types", &self.export_from_imported_handle_types)
            .field("compatible_handle_types", &self.compatible_handle_types)
            .field("external_fence_features", &self.external_fence_features)
            .finish()
    }
}
impl ExternalFenceProperties {
    #[inline]
    pub fn into_builder<'a>(self) -> ExternalFencePropertiesBuilder<'a> {
        ExternalFencePropertiesBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalFenceProperties.html) · Builder of [`ExternalFenceProperties`]"]
#[repr(transparent)]
pub struct ExternalFencePropertiesBuilder<'a>(ExternalFenceProperties, std::marker::PhantomData<&'a ()>);
impl<'a> ExternalFencePropertiesBuilder<'a> {
    #[inline]
    pub fn new() -> ExternalFencePropertiesBuilder<'a> {
        ExternalFencePropertiesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn export_from_imported_handle_types(mut self, export_from_imported_handle_types: crate::vk1_1::ExternalFenceHandleTypeFlags) -> Self {
        self.0.export_from_imported_handle_types = export_from_imported_handle_types as _;
        self
    }
    #[inline]
    pub fn compatible_handle_types(mut self, compatible_handle_types: crate::vk1_1::ExternalFenceHandleTypeFlags) -> Self {
        self.0.compatible_handle_types = compatible_handle_types as _;
        self
    }
    #[inline]
    pub fn external_fence_features(mut self, external_fence_features: crate::vk1_1::ExternalFenceFeatureFlags) -> Self {
        self.0.external_fence_features = external_fence_features as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ExternalFenceProperties {
        self.0
    }
}
impl<'a> std::default::Default for ExternalFencePropertiesBuilder<'a> {
    fn default() -> ExternalFencePropertiesBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ExternalFencePropertiesBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ExternalFencePropertiesBuilder<'a> {
    type Target = ExternalFenceProperties;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ExternalFencePropertiesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<ExternalFenceProperties> for ExternalFencePropertiesBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExportFenceCreateInfo.html) · Structure"]
#[doc(alias = "VkExportFenceCreateInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExportFenceCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub handle_types: crate::vk1_1::ExternalFenceHandleTypeFlags,
}
impl Default for ExportFenceCreateInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::EXPORT_FENCE_CREATE_INFO,
            p_next: std::ptr::null(),
            handle_types: Default::default(),
        }
    }
}
impl std::fmt::Debug for ExportFenceCreateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ExportFenceCreateInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("handle_types", &self.handle_types)
            .finish()
    }
}
impl ExportFenceCreateInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> ExportFenceCreateInfoBuilder<'a> {
        ExportFenceCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExportFenceCreateInfo.html) · Builder of [`ExportFenceCreateInfo`]"]
#[repr(transparent)]
pub struct ExportFenceCreateInfoBuilder<'a>(ExportFenceCreateInfo, std::marker::PhantomData<&'a ()>);
impl<'a> ExportFenceCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> ExportFenceCreateInfoBuilder<'a> {
        ExportFenceCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn handle_types(mut self, handle_types: crate::vk1_1::ExternalFenceHandleTypeFlags) -> Self {
        self.0.handle_types = handle_types as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ExportFenceCreateInfo {
        self.0
    }
}
impl<'a> std::default::Default for ExportFenceCreateInfoBuilder<'a> {
    fn default() -> ExportFenceCreateInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ExportFenceCreateInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ExportFenceCreateInfoBuilder<'a> {
    type Target = ExportFenceCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ExportFenceCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<ExportFenceCreateInfo> for ExportFenceCreateInfoBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMultiviewFeatures.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceMultiviewFeatures")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceMultiviewFeatures {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub multiview: crate::vk1_0::Bool32,
    pub multiview_geometry_shader: crate::vk1_0::Bool32,
    pub multiview_tessellation_shader: crate::vk1_0::Bool32,
}
impl Default for PhysicalDeviceMultiviewFeatures {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_MULTIVIEW_FEATURES,
            p_next: std::ptr::null_mut(),
            multiview: Default::default(),
            multiview_geometry_shader: Default::default(),
            multiview_tessellation_shader: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceMultiviewFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceMultiviewFeatures")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("multiview", &(self.multiview != 0))
            .field("multiview_geometry_shader", &(self.multiview_geometry_shader != 0))
            .field("multiview_tessellation_shader", &(self.multiview_tessellation_shader != 0))
            .finish()
    }
}
impl PhysicalDeviceMultiviewFeatures {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceMultiviewFeaturesBuilder<'a> {
        PhysicalDeviceMultiviewFeaturesBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMultiviewFeatures.html) · Builder of [`PhysicalDeviceMultiviewFeatures`]"]
#[repr(transparent)]
pub struct PhysicalDeviceMultiviewFeaturesBuilder<'a>(PhysicalDeviceMultiviewFeatures, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceMultiviewFeaturesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceMultiviewFeaturesBuilder<'a> {
        PhysicalDeviceMultiviewFeaturesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn multiview(mut self, multiview: bool) -> Self {
        self.0.multiview = multiview as _;
        self
    }
    #[inline]
    pub fn multiview_geometry_shader(mut self, multiview_geometry_shader: bool) -> Self {
        self.0.multiview_geometry_shader = multiview_geometry_shader as _;
        self
    }
    #[inline]
    pub fn multiview_tessellation_shader(mut self, multiview_tessellation_shader: bool) -> Self {
        self.0.multiview_tessellation_shader = multiview_tessellation_shader as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceMultiviewFeatures {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceMultiviewFeaturesBuilder<'a> {
    fn default() -> PhysicalDeviceMultiviewFeaturesBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceMultiviewFeaturesBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceMultiviewFeaturesBuilder<'a> {
    type Target = PhysicalDeviceMultiviewFeatures;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceMultiviewFeaturesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<PhysicalDeviceMultiviewFeatures> for PhysicalDeviceMultiviewFeaturesBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMultiviewProperties.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceMultiviewProperties")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceMultiviewProperties {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub max_multiview_view_count: u32,
    pub max_multiview_instance_index: u32,
}
impl Default for PhysicalDeviceMultiviewProperties {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES,
            p_next: std::ptr::null_mut(),
            max_multiview_view_count: Default::default(),
            max_multiview_instance_index: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceMultiviewProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceMultiviewProperties")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("max_multiview_view_count", &self.max_multiview_view_count)
            .field("max_multiview_instance_index", &self.max_multiview_instance_index)
            .finish()
    }
}
impl PhysicalDeviceMultiviewProperties {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceMultiviewPropertiesBuilder<'a> {
        PhysicalDeviceMultiviewPropertiesBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMultiviewProperties.html) · Builder of [`PhysicalDeviceMultiviewProperties`]"]
#[repr(transparent)]
pub struct PhysicalDeviceMultiviewPropertiesBuilder<'a>(PhysicalDeviceMultiviewProperties, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceMultiviewPropertiesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceMultiviewPropertiesBuilder<'a> {
        PhysicalDeviceMultiviewPropertiesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn max_multiview_view_count(mut self, max_multiview_view_count: u32) -> Self {
        self.0.max_multiview_view_count = max_multiview_view_count as _;
        self
    }
    #[inline]
    pub fn max_multiview_instance_index(mut self, max_multiview_instance_index: u32) -> Self {
        self.0.max_multiview_instance_index = max_multiview_instance_index as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceMultiviewProperties {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceMultiviewPropertiesBuilder<'a> {
    fn default() -> PhysicalDeviceMultiviewPropertiesBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceMultiviewPropertiesBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceMultiviewPropertiesBuilder<'a> {
    type Target = PhysicalDeviceMultiviewProperties;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceMultiviewPropertiesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<PhysicalDeviceMultiviewProperties> for PhysicalDeviceMultiviewPropertiesBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderPassMultiviewCreateInfo.html) · Structure"]
#[doc(alias = "VkRenderPassMultiviewCreateInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RenderPassMultiviewCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub subpass_count: u32,
    pub p_view_masks: *const u32,
    pub dependency_count: u32,
    pub p_view_offsets: *const i32,
    pub correlation_mask_count: u32,
    pub p_correlation_masks: *const u32,
}
impl Default for RenderPassMultiviewCreateInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::RENDER_PASS_MULTIVIEW_CREATE_INFO,
            p_next: std::ptr::null(),
            subpass_count: Default::default(),
            p_view_masks: std::ptr::null(),
            dependency_count: Default::default(),
            p_view_offsets: std::ptr::null(),
            correlation_mask_count: Default::default(),
            p_correlation_masks: std::ptr::null(),
        }
    }
}
impl std::fmt::Debug for RenderPassMultiviewCreateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RenderPassMultiviewCreateInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("subpass_count", &self.subpass_count)
            .field("p_view_masks", &self.p_view_masks)
            .field("dependency_count", &self.dependency_count)
            .field("p_view_offsets", &self.p_view_offsets)
            .field("correlation_mask_count", &self.correlation_mask_count)
            .field("p_correlation_masks", &self.p_correlation_masks)
            .finish()
    }
}
impl RenderPassMultiviewCreateInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> RenderPassMultiviewCreateInfoBuilder<'a> {
        RenderPassMultiviewCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderPassMultiviewCreateInfo.html) · Builder of [`RenderPassMultiviewCreateInfo`]"]
#[repr(transparent)]
pub struct RenderPassMultiviewCreateInfoBuilder<'a>(RenderPassMultiviewCreateInfo, std::marker::PhantomData<&'a ()>);
impl<'a> RenderPassMultiviewCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> RenderPassMultiviewCreateInfoBuilder<'a> {
        RenderPassMultiviewCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn view_masks(mut self, view_masks: &'a [u32]) -> Self {
        self.0.p_view_masks = view_masks.as_ptr() as _;
        self.0.subpass_count = view_masks.len() as _;
        self
    }
    #[inline]
    pub fn view_offsets(mut self, view_offsets: &'a [i32]) -> Self {
        self.0.p_view_offsets = view_offsets.as_ptr() as _;
        self.0.dependency_count = view_offsets.len() as _;
        self
    }
    #[inline]
    pub fn correlation_masks(mut self, correlation_masks: &'a [u32]) -> Self {
        self.0.p_correlation_masks = correlation_masks.as_ptr() as _;
        self.0.correlation_mask_count = correlation_masks.len() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> RenderPassMultiviewCreateInfo {
        self.0
    }
}
impl<'a> std::default::Default for RenderPassMultiviewCreateInfoBuilder<'a> {
    fn default() -> RenderPassMultiviewCreateInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for RenderPassMultiviewCreateInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for RenderPassMultiviewCreateInfoBuilder<'a> {
    type Target = RenderPassMultiviewCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for RenderPassMultiviewCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<RenderPassMultiviewCreateInfo> for RenderPassMultiviewCreateInfoBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceGroupProperties.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceGroupProperties")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceGroupProperties {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub physical_device_count: u32,
    pub physical_devices: [crate::vk1_0::PhysicalDevice; 32],
    pub subset_allocation: crate::vk1_0::Bool32,
}
impl Default for PhysicalDeviceGroupProperties {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_GROUP_PROPERTIES,
            p_next: std::ptr::null_mut(),
            physical_device_count: Default::default(),
            physical_devices: unsafe { std::mem::zeroed() },
            subset_allocation: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceGroupProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceGroupProperties")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("physical_device_count", &self.physical_device_count)
            .field("physical_devices", &self.physical_devices)
            .field("subset_allocation", &(self.subset_allocation != 0))
            .finish()
    }
}
impl PhysicalDeviceGroupProperties {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceGroupPropertiesBuilder<'a> {
        PhysicalDeviceGroupPropertiesBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceGroupProperties.html) · Builder of [`PhysicalDeviceGroupProperties`]"]
#[repr(transparent)]
pub struct PhysicalDeviceGroupPropertiesBuilder<'a>(PhysicalDeviceGroupProperties, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceGroupPropertiesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceGroupPropertiesBuilder<'a> {
        PhysicalDeviceGroupPropertiesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn physical_device_count(mut self, physical_device_count: u32) -> Self {
        self.0.physical_device_count = physical_device_count as _;
        self
    }
    #[inline]
    pub fn physical_devices(mut self, physical_devices: [crate::vk1_0::PhysicalDevice; 32]) -> Self {
        self.0.physical_devices = physical_devices as _;
        self
    }
    #[inline]
    pub fn subset_allocation(mut self, subset_allocation: bool) -> Self {
        self.0.subset_allocation = subset_allocation as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceGroupProperties {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceGroupPropertiesBuilder<'a> {
    fn default() -> PhysicalDeviceGroupPropertiesBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceGroupPropertiesBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceGroupPropertiesBuilder<'a> {
    type Target = PhysicalDeviceGroupProperties;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceGroupPropertiesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<PhysicalDeviceGroupProperties> for PhysicalDeviceGroupPropertiesBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryAllocateFlagsInfo.html) · Structure"]
#[doc(alias = "VkMemoryAllocateFlagsInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryAllocateFlagsInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::vk1_1::MemoryAllocateFlags,
    pub device_mask: u32,
}
impl Default for MemoryAllocateFlagsInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::MEMORY_ALLOCATE_FLAGS_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            device_mask: Default::default(),
        }
    }
}
impl std::fmt::Debug for MemoryAllocateFlagsInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MemoryAllocateFlagsInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("device_mask", &self.device_mask)
            .finish()
    }
}
impl MemoryAllocateFlagsInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> MemoryAllocateFlagsInfoBuilder<'a> {
        MemoryAllocateFlagsInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryAllocateFlagsInfo.html) · Builder of [`MemoryAllocateFlagsInfo`]"]
#[repr(transparent)]
pub struct MemoryAllocateFlagsInfoBuilder<'a>(MemoryAllocateFlagsInfo, std::marker::PhantomData<&'a ()>);
impl<'a> MemoryAllocateFlagsInfoBuilder<'a> {
    #[inline]
    pub fn new() -> MemoryAllocateFlagsInfoBuilder<'a> {
        MemoryAllocateFlagsInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_1::MemoryAllocateFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn device_mask(mut self, device_mask: u32) -> Self {
        self.0.device_mask = device_mask as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> MemoryAllocateFlagsInfo {
        self.0
    }
}
impl<'a> std::default::Default for MemoryAllocateFlagsInfoBuilder<'a> {
    fn default() -> MemoryAllocateFlagsInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for MemoryAllocateFlagsInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for MemoryAllocateFlagsInfoBuilder<'a> {
    type Target = MemoryAllocateFlagsInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for MemoryAllocateFlagsInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<MemoryAllocateFlagsInfo> for MemoryAllocateFlagsInfoBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindBufferMemoryInfo.html) · Structure"]
#[doc(alias = "VkBindBufferMemoryInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BindBufferMemoryInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub buffer: crate::vk1_0::Buffer,
    pub memory: crate::vk1_0::DeviceMemory,
    pub memory_offset: crate::vk1_0::DeviceSize,
}
impl Default for BindBufferMemoryInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::BIND_BUFFER_MEMORY_INFO,
            p_next: std::ptr::null(),
            buffer: Default::default(),
            memory: Default::default(),
            memory_offset: Default::default(),
        }
    }
}
impl std::fmt::Debug for BindBufferMemoryInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BindBufferMemoryInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("buffer", &self.buffer)
            .field("memory", &self.memory)
            .field("memory_offset", &self.memory_offset)
            .finish()
    }
}
impl BindBufferMemoryInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> BindBufferMemoryInfoBuilder<'a> {
        BindBufferMemoryInfoBuilder(self, std::marker::PhantomData)
    }
}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_1::BindBufferMemoryDeviceGroupInfo> for BindBufferMemoryInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_1::BindBufferMemoryDeviceGroupInfoBuilder<'_>> for BindBufferMemoryInfoBuilder<'a> {}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindBufferMemoryInfo.html) · Builder of [`BindBufferMemoryInfo`]"]
#[repr(transparent)]
pub struct BindBufferMemoryInfoBuilder<'a>(BindBufferMemoryInfo, std::marker::PhantomData<&'a ()>);
impl<'a> BindBufferMemoryInfoBuilder<'a> {
    #[inline]
    pub fn new() -> BindBufferMemoryInfoBuilder<'a> {
        BindBufferMemoryInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn buffer(mut self, buffer: crate::vk1_0::Buffer) -> Self {
        self.0.buffer = buffer as _;
        self
    }
    #[inline]
    pub fn memory(mut self, memory: crate::vk1_0::DeviceMemory) -> Self {
        self.0.memory = memory as _;
        self
    }
    #[inline]
    pub fn memory_offset(mut self, memory_offset: crate::vk1_0::DeviceSize) -> Self {
        self.0.memory_offset = memory_offset as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> BindBufferMemoryInfo {
        self.0
    }
}
impl<'a> std::default::Default for BindBufferMemoryInfoBuilder<'a> {
    fn default() -> BindBufferMemoryInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for BindBufferMemoryInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for BindBufferMemoryInfoBuilder<'a> {
    type Target = BindBufferMemoryInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for BindBufferMemoryInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<BindBufferMemoryInfo> for BindBufferMemoryInfoBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindBufferMemoryDeviceGroupInfo.html) · Structure"]
#[doc(alias = "VkBindBufferMemoryDeviceGroupInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BindBufferMemoryDeviceGroupInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub device_index_count: u32,
    pub p_device_indices: *const u32,
}
impl Default for BindBufferMemoryDeviceGroupInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO,
            p_next: std::ptr::null(),
            device_index_count: Default::default(),
            p_device_indices: std::ptr::null(),
        }
    }
}
impl std::fmt::Debug for BindBufferMemoryDeviceGroupInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BindBufferMemoryDeviceGroupInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("device_index_count", &self.device_index_count)
            .field("p_device_indices", &self.p_device_indices)
            .finish()
    }
}
impl BindBufferMemoryDeviceGroupInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> BindBufferMemoryDeviceGroupInfoBuilder<'a> {
        BindBufferMemoryDeviceGroupInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindBufferMemoryDeviceGroupInfo.html) · Builder of [`BindBufferMemoryDeviceGroupInfo`]"]
#[repr(transparent)]
pub struct BindBufferMemoryDeviceGroupInfoBuilder<'a>(BindBufferMemoryDeviceGroupInfo, std::marker::PhantomData<&'a ()>);
impl<'a> BindBufferMemoryDeviceGroupInfoBuilder<'a> {
    #[inline]
    pub fn new() -> BindBufferMemoryDeviceGroupInfoBuilder<'a> {
        BindBufferMemoryDeviceGroupInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn device_indices(mut self, device_indices: &'a [u32]) -> Self {
        self.0.p_device_indices = device_indices.as_ptr() as _;
        self.0.device_index_count = device_indices.len() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> BindBufferMemoryDeviceGroupInfo {
        self.0
    }
}
impl<'a> std::default::Default for BindBufferMemoryDeviceGroupInfoBuilder<'a> {
    fn default() -> BindBufferMemoryDeviceGroupInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for BindBufferMemoryDeviceGroupInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for BindBufferMemoryDeviceGroupInfoBuilder<'a> {
    type Target = BindBufferMemoryDeviceGroupInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for BindBufferMemoryDeviceGroupInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<BindBufferMemoryDeviceGroupInfo> for BindBufferMemoryDeviceGroupInfoBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindImageMemoryInfo.html) · Structure"]
#[doc(alias = "VkBindImageMemoryInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BindImageMemoryInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub image: crate::vk1_0::Image,
    pub memory: crate::vk1_0::DeviceMemory,
    pub memory_offset: crate::vk1_0::DeviceSize,
}
impl Default for BindImageMemoryInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::BIND_IMAGE_MEMORY_INFO,
            p_next: std::ptr::null(),
            image: Default::default(),
            memory: Default::default(),
            memory_offset: Default::default(),
        }
    }
}
impl std::fmt::Debug for BindImageMemoryInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BindImageMemoryInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("image", &self.image)
            .field("memory", &self.memory)
            .field("memory_offset", &self.memory_offset)
            .finish()
    }
}
impl BindImageMemoryInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> BindImageMemoryInfoBuilder<'a> {
        BindImageMemoryInfoBuilder(self, std::marker::PhantomData)
    }
}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_1::BindImageMemoryDeviceGroupInfo> for BindImageMemoryInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_1::BindImageMemoryDeviceGroupInfoBuilder<'_>> for BindImageMemoryInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::khr_swapchain::BindImageMemorySwapchainInfoKHR> for BindImageMemoryInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::khr_swapchain::BindImageMemorySwapchainInfoKHRBuilder<'_>> for BindImageMemoryInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_1::BindImagePlaneMemoryInfo> for BindImageMemoryInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_1::BindImagePlaneMemoryInfoBuilder<'_>> for BindImageMemoryInfoBuilder<'a> {}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindImageMemoryInfo.html) · Builder of [`BindImageMemoryInfo`]"]
#[repr(transparent)]
pub struct BindImageMemoryInfoBuilder<'a>(BindImageMemoryInfo, std::marker::PhantomData<&'a ()>);
impl<'a> BindImageMemoryInfoBuilder<'a> {
    #[inline]
    pub fn new() -> BindImageMemoryInfoBuilder<'a> {
        BindImageMemoryInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn image(mut self, image: crate::vk1_0::Image) -> Self {
        self.0.image = image as _;
        self
    }
    #[inline]
    pub fn memory(mut self, memory: crate::vk1_0::DeviceMemory) -> Self {
        self.0.memory = memory as _;
        self
    }
    #[inline]
    pub fn memory_offset(mut self, memory_offset: crate::vk1_0::DeviceSize) -> Self {
        self.0.memory_offset = memory_offset as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> BindImageMemoryInfo {
        self.0
    }
}
impl<'a> std::default::Default for BindImageMemoryInfoBuilder<'a> {
    fn default() -> BindImageMemoryInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for BindImageMemoryInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for BindImageMemoryInfoBuilder<'a> {
    type Target = BindImageMemoryInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for BindImageMemoryInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<BindImageMemoryInfo> for BindImageMemoryInfoBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindImageMemoryDeviceGroupInfo.html) · Structure"]
#[doc(alias = "VkBindImageMemoryDeviceGroupInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BindImageMemoryDeviceGroupInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub device_index_count: u32,
    pub p_device_indices: *const u32,
    pub split_instance_bind_region_count: u32,
    pub p_split_instance_bind_regions: *const crate::vk1_0::Rect2D,
}
impl Default for BindImageMemoryDeviceGroupInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO,
            p_next: std::ptr::null(),
            device_index_count: Default::default(),
            p_device_indices: std::ptr::null(),
            split_instance_bind_region_count: Default::default(),
            p_split_instance_bind_regions: std::ptr::null(),
        }
    }
}
impl std::fmt::Debug for BindImageMemoryDeviceGroupInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BindImageMemoryDeviceGroupInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("device_index_count", &self.device_index_count)
            .field("p_device_indices", &self.p_device_indices)
            .field("split_instance_bind_region_count", &self.split_instance_bind_region_count)
            .field("p_split_instance_bind_regions", &self.p_split_instance_bind_regions)
            .finish()
    }
}
impl BindImageMemoryDeviceGroupInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> BindImageMemoryDeviceGroupInfoBuilder<'a> {
        BindImageMemoryDeviceGroupInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindImageMemoryDeviceGroupInfo.html) · Builder of [`BindImageMemoryDeviceGroupInfo`]"]
#[repr(transparent)]
pub struct BindImageMemoryDeviceGroupInfoBuilder<'a>(BindImageMemoryDeviceGroupInfo, std::marker::PhantomData<&'a ()>);
impl<'a> BindImageMemoryDeviceGroupInfoBuilder<'a> {
    #[inline]
    pub fn new() -> BindImageMemoryDeviceGroupInfoBuilder<'a> {
        BindImageMemoryDeviceGroupInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn device_indices(mut self, device_indices: &'a [u32]) -> Self {
        self.0.p_device_indices = device_indices.as_ptr() as _;
        self.0.device_index_count = device_indices.len() as _;
        self
    }
    #[inline]
    pub fn split_instance_bind_regions(mut self, split_instance_bind_regions: &'a [impl crate::Repr<crate::vk1_0::Rect2D>]) -> Self {
        self.0.p_split_instance_bind_regions = split_instance_bind_regions.as_ptr() as _;
        self.0.split_instance_bind_region_count = split_instance_bind_regions.len() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> BindImageMemoryDeviceGroupInfo {
        self.0
    }
}
impl<'a> std::default::Default for BindImageMemoryDeviceGroupInfoBuilder<'a> {
    fn default() -> BindImageMemoryDeviceGroupInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for BindImageMemoryDeviceGroupInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for BindImageMemoryDeviceGroupInfoBuilder<'a> {
    type Target = BindImageMemoryDeviceGroupInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for BindImageMemoryDeviceGroupInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<BindImageMemoryDeviceGroupInfo> for BindImageMemoryDeviceGroupInfoBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceGroupRenderPassBeginInfo.html) · Structure"]
#[doc(alias = "VkDeviceGroupRenderPassBeginInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DeviceGroupRenderPassBeginInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub device_mask: u32,
    pub device_render_area_count: u32,
    pub p_device_render_areas: *const crate::vk1_0::Rect2D,
}
impl Default for DeviceGroupRenderPassBeginInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::DEVICE_GROUP_RENDER_PASS_BEGIN_INFO,
            p_next: std::ptr::null(),
            device_mask: Default::default(),
            device_render_area_count: Default::default(),
            p_device_render_areas: std::ptr::null(),
        }
    }
}
impl std::fmt::Debug for DeviceGroupRenderPassBeginInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DeviceGroupRenderPassBeginInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("device_mask", &self.device_mask)
            .field("device_render_area_count", &self.device_render_area_count)
            .field("p_device_render_areas", &self.p_device_render_areas)
            .finish()
    }
}
impl DeviceGroupRenderPassBeginInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> DeviceGroupRenderPassBeginInfoBuilder<'a> {
        DeviceGroupRenderPassBeginInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceGroupRenderPassBeginInfo.html) · Builder of [`DeviceGroupRenderPassBeginInfo`]"]
#[repr(transparent)]
pub struct DeviceGroupRenderPassBeginInfoBuilder<'a>(DeviceGroupRenderPassBeginInfo, std::marker::PhantomData<&'a ()>);
impl<'a> DeviceGroupRenderPassBeginInfoBuilder<'a> {
    #[inline]
    pub fn new() -> DeviceGroupRenderPassBeginInfoBuilder<'a> {
        DeviceGroupRenderPassBeginInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn device_mask(mut self, device_mask: u32) -> Self {
        self.0.device_mask = device_mask as _;
        self
    }
    #[inline]
    pub fn device_render_areas(mut self, device_render_areas: &'a [impl crate::Repr<crate::vk1_0::Rect2D>]) -> Self {
        self.0.p_device_render_areas = device_render_areas.as_ptr() as _;
        self.0.device_render_area_count = device_render_areas.len() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DeviceGroupRenderPassBeginInfo {
        self.0
    }
}
impl<'a> std::default::Default for DeviceGroupRenderPassBeginInfoBuilder<'a> {
    fn default() -> DeviceGroupRenderPassBeginInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DeviceGroupRenderPassBeginInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DeviceGroupRenderPassBeginInfoBuilder<'a> {
    type Target = DeviceGroupRenderPassBeginInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DeviceGroupRenderPassBeginInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<DeviceGroupRenderPassBeginInfo> for DeviceGroupRenderPassBeginInfoBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceGroupCommandBufferBeginInfo.html) · Structure"]
#[doc(alias = "VkDeviceGroupCommandBufferBeginInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DeviceGroupCommandBufferBeginInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub device_mask: u32,
}
impl Default for DeviceGroupCommandBufferBeginInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO,
            p_next: std::ptr::null(),
            device_mask: Default::default(),
        }
    }
}
impl std::fmt::Debug for DeviceGroupCommandBufferBeginInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DeviceGroupCommandBufferBeginInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("device_mask", &self.device_mask)
            .finish()
    }
}
impl DeviceGroupCommandBufferBeginInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> DeviceGroupCommandBufferBeginInfoBuilder<'a> {
        DeviceGroupCommandBufferBeginInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceGroupCommandBufferBeginInfo.html) · Builder of [`DeviceGroupCommandBufferBeginInfo`]"]
#[repr(transparent)]
pub struct DeviceGroupCommandBufferBeginInfoBuilder<'a>(DeviceGroupCommandBufferBeginInfo, std::marker::PhantomData<&'a ()>);
impl<'a> DeviceGroupCommandBufferBeginInfoBuilder<'a> {
    #[inline]
    pub fn new() -> DeviceGroupCommandBufferBeginInfoBuilder<'a> {
        DeviceGroupCommandBufferBeginInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn device_mask(mut self, device_mask: u32) -> Self {
        self.0.device_mask = device_mask as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DeviceGroupCommandBufferBeginInfo {
        self.0
    }
}
impl<'a> std::default::Default for DeviceGroupCommandBufferBeginInfoBuilder<'a> {
    fn default() -> DeviceGroupCommandBufferBeginInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DeviceGroupCommandBufferBeginInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DeviceGroupCommandBufferBeginInfoBuilder<'a> {
    type Target = DeviceGroupCommandBufferBeginInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DeviceGroupCommandBufferBeginInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<DeviceGroupCommandBufferBeginInfo> for DeviceGroupCommandBufferBeginInfoBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceGroupSubmitInfo.html) · Structure"]
#[doc(alias = "VkDeviceGroupSubmitInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DeviceGroupSubmitInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub wait_semaphore_count: u32,
    pub p_wait_semaphore_device_indices: *const u32,
    pub command_buffer_count: u32,
    pub p_command_buffer_device_masks: *const u32,
    pub signal_semaphore_count: u32,
    pub p_signal_semaphore_device_indices: *const u32,
}
impl Default for DeviceGroupSubmitInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::DEVICE_GROUP_SUBMIT_INFO,
            p_next: std::ptr::null(),
            wait_semaphore_count: Default::default(),
            p_wait_semaphore_device_indices: std::ptr::null(),
            command_buffer_count: Default::default(),
            p_command_buffer_device_masks: std::ptr::null(),
            signal_semaphore_count: Default::default(),
            p_signal_semaphore_device_indices: std::ptr::null(),
        }
    }
}
impl std::fmt::Debug for DeviceGroupSubmitInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DeviceGroupSubmitInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("wait_semaphore_count", &self.wait_semaphore_count)
            .field("p_wait_semaphore_device_indices", &self.p_wait_semaphore_device_indices)
            .field("command_buffer_count", &self.command_buffer_count)
            .field("p_command_buffer_device_masks", &self.p_command_buffer_device_masks)
            .field("signal_semaphore_count", &self.signal_semaphore_count)
            .field("p_signal_semaphore_device_indices", &self.p_signal_semaphore_device_indices)
            .finish()
    }
}
impl DeviceGroupSubmitInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> DeviceGroupSubmitInfoBuilder<'a> {
        DeviceGroupSubmitInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceGroupSubmitInfo.html) · Builder of [`DeviceGroupSubmitInfo`]"]
#[repr(transparent)]
pub struct DeviceGroupSubmitInfoBuilder<'a>(DeviceGroupSubmitInfo, std::marker::PhantomData<&'a ()>);
impl<'a> DeviceGroupSubmitInfoBuilder<'a> {
    #[inline]
    pub fn new() -> DeviceGroupSubmitInfoBuilder<'a> {
        DeviceGroupSubmitInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn wait_semaphore_device_indices(mut self, wait_semaphore_device_indices: &'a [u32]) -> Self {
        self.0.p_wait_semaphore_device_indices = wait_semaphore_device_indices.as_ptr() as _;
        self.0.wait_semaphore_count = wait_semaphore_device_indices.len() as _;
        self
    }
    #[inline]
    pub fn command_buffer_device_masks(mut self, command_buffer_device_masks: &'a [u32]) -> Self {
        self.0.p_command_buffer_device_masks = command_buffer_device_masks.as_ptr() as _;
        self.0.command_buffer_count = command_buffer_device_masks.len() as _;
        self
    }
    #[inline]
    pub fn signal_semaphore_device_indices(mut self, signal_semaphore_device_indices: &'a [u32]) -> Self {
        self.0.p_signal_semaphore_device_indices = signal_semaphore_device_indices.as_ptr() as _;
        self.0.signal_semaphore_count = signal_semaphore_device_indices.len() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DeviceGroupSubmitInfo {
        self.0
    }
}
impl<'a> std::default::Default for DeviceGroupSubmitInfoBuilder<'a> {
    fn default() -> DeviceGroupSubmitInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DeviceGroupSubmitInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DeviceGroupSubmitInfoBuilder<'a> {
    type Target = DeviceGroupSubmitInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DeviceGroupSubmitInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<DeviceGroupSubmitInfo> for DeviceGroupSubmitInfoBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceGroupBindSparseInfo.html) · Structure"]
#[doc(alias = "VkDeviceGroupBindSparseInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DeviceGroupBindSparseInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub resource_device_index: u32,
    pub memory_device_index: u32,
}
impl Default for DeviceGroupBindSparseInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::DEVICE_GROUP_BIND_SPARSE_INFO,
            p_next: std::ptr::null(),
            resource_device_index: Default::default(),
            memory_device_index: Default::default(),
        }
    }
}
impl std::fmt::Debug for DeviceGroupBindSparseInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DeviceGroupBindSparseInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("resource_device_index", &self.resource_device_index)
            .field("memory_device_index", &self.memory_device_index)
            .finish()
    }
}
impl DeviceGroupBindSparseInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> DeviceGroupBindSparseInfoBuilder<'a> {
        DeviceGroupBindSparseInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceGroupBindSparseInfo.html) · Builder of [`DeviceGroupBindSparseInfo`]"]
#[repr(transparent)]
pub struct DeviceGroupBindSparseInfoBuilder<'a>(DeviceGroupBindSparseInfo, std::marker::PhantomData<&'a ()>);
impl<'a> DeviceGroupBindSparseInfoBuilder<'a> {
    #[inline]
    pub fn new() -> DeviceGroupBindSparseInfoBuilder<'a> {
        DeviceGroupBindSparseInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn resource_device_index(mut self, resource_device_index: u32) -> Self {
        self.0.resource_device_index = resource_device_index as _;
        self
    }
    #[inline]
    pub fn memory_device_index(mut self, memory_device_index: u32) -> Self {
        self.0.memory_device_index = memory_device_index as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DeviceGroupBindSparseInfo {
        self.0
    }
}
impl<'a> std::default::Default for DeviceGroupBindSparseInfoBuilder<'a> {
    fn default() -> DeviceGroupBindSparseInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DeviceGroupBindSparseInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DeviceGroupBindSparseInfoBuilder<'a> {
    type Target = DeviceGroupBindSparseInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DeviceGroupBindSparseInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<DeviceGroupBindSparseInfo> for DeviceGroupBindSparseInfoBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceGroupDeviceCreateInfo.html) · Structure"]
#[doc(alias = "VkDeviceGroupDeviceCreateInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DeviceGroupDeviceCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub physical_device_count: u32,
    pub p_physical_devices: *const crate::vk1_0::PhysicalDevice,
}
impl Default for DeviceGroupDeviceCreateInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::DEVICE_GROUP_DEVICE_CREATE_INFO,
            p_next: std::ptr::null(),
            physical_device_count: Default::default(),
            p_physical_devices: std::ptr::null(),
        }
    }
}
impl std::fmt::Debug for DeviceGroupDeviceCreateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DeviceGroupDeviceCreateInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("physical_device_count", &self.physical_device_count)
            .field("p_physical_devices", &self.p_physical_devices)
            .finish()
    }
}
impl DeviceGroupDeviceCreateInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> DeviceGroupDeviceCreateInfoBuilder<'a> {
        DeviceGroupDeviceCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceGroupDeviceCreateInfo.html) · Builder of [`DeviceGroupDeviceCreateInfo`]"]
#[repr(transparent)]
pub struct DeviceGroupDeviceCreateInfoBuilder<'a>(DeviceGroupDeviceCreateInfo, std::marker::PhantomData<&'a ()>);
impl<'a> DeviceGroupDeviceCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> DeviceGroupDeviceCreateInfoBuilder<'a> {
        DeviceGroupDeviceCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn physical_devices(mut self, physical_devices: &'a [crate::vk1_0::PhysicalDevice]) -> Self {
        self.0.p_physical_devices = physical_devices.as_ptr() as _;
        self.0.physical_device_count = physical_devices.len() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DeviceGroupDeviceCreateInfo {
        self.0
    }
}
impl<'a> std::default::Default for DeviceGroupDeviceCreateInfoBuilder<'a> {
    fn default() -> DeviceGroupDeviceCreateInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DeviceGroupDeviceCreateInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DeviceGroupDeviceCreateInfoBuilder<'a> {
    type Target = DeviceGroupDeviceCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DeviceGroupDeviceCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<DeviceGroupDeviceCreateInfo> for DeviceGroupDeviceCreateInfoBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorUpdateTemplateEntry.html) · Structure"]
#[doc(alias = "VkDescriptorUpdateTemplateEntry")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DescriptorUpdateTemplateEntry {
    pub dst_binding: u32,
    pub dst_array_element: u32,
    pub descriptor_count: u32,
    pub descriptor_type: crate::vk1_0::DescriptorType,
    pub offset: usize,
    pub stride: usize,
}
impl Default for DescriptorUpdateTemplateEntry {
    fn default() -> Self {
        Self {
            dst_binding: Default::default(),
            dst_array_element: Default::default(),
            descriptor_count: Default::default(),
            descriptor_type: Default::default(),
            offset: Default::default(),
            stride: Default::default(),
        }
    }
}
impl std::fmt::Debug for DescriptorUpdateTemplateEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DescriptorUpdateTemplateEntry")
            .field("dst_binding", &self.dst_binding)
            .field("dst_array_element", &self.dst_array_element)
            .field("descriptor_count", &self.descriptor_count)
            .field("descriptor_type", &self.descriptor_type)
            .field("offset", &self.offset)
            .field("stride", &self.stride)
            .finish()
    }
}
impl DescriptorUpdateTemplateEntry {
    #[inline]
    pub fn into_builder<'a>(self) -> DescriptorUpdateTemplateEntryBuilder<'a> {
        DescriptorUpdateTemplateEntryBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorUpdateTemplateEntry.html) · Builder of [`DescriptorUpdateTemplateEntry`]"]
#[repr(transparent)]
pub struct DescriptorUpdateTemplateEntryBuilder<'a>(DescriptorUpdateTemplateEntry, std::marker::PhantomData<&'a ()>);
impl<'a> DescriptorUpdateTemplateEntryBuilder<'a> {
    #[inline]
    pub fn new() -> DescriptorUpdateTemplateEntryBuilder<'a> {
        DescriptorUpdateTemplateEntryBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn dst_binding(mut self, dst_binding: u32) -> Self {
        self.0.dst_binding = dst_binding as _;
        self
    }
    #[inline]
    pub fn dst_array_element(mut self, dst_array_element: u32) -> Self {
        self.0.dst_array_element = dst_array_element as _;
        self
    }
    #[inline]
    pub fn descriptor_count(mut self, descriptor_count: u32) -> Self {
        self.0.descriptor_count = descriptor_count as _;
        self
    }
    #[inline]
    pub fn descriptor_type(mut self, descriptor_type: crate::vk1_0::DescriptorType) -> Self {
        self.0.descriptor_type = descriptor_type as _;
        self
    }
    #[inline]
    pub fn offset(mut self, offset: usize) -> Self {
        self.0.offset = offset as _;
        self
    }
    #[inline]
    pub fn stride(mut self, stride: usize) -> Self {
        self.0.stride = stride as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DescriptorUpdateTemplateEntry {
        self.0
    }
}
impl<'a> std::default::Default for DescriptorUpdateTemplateEntryBuilder<'a> {
    fn default() -> DescriptorUpdateTemplateEntryBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DescriptorUpdateTemplateEntryBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DescriptorUpdateTemplateEntryBuilder<'a> {
    type Target = DescriptorUpdateTemplateEntry;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DescriptorUpdateTemplateEntryBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<DescriptorUpdateTemplateEntry> for DescriptorUpdateTemplateEntryBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorUpdateTemplateCreateInfo.html) · Structure"]
#[doc(alias = "VkDescriptorUpdateTemplateCreateInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DescriptorUpdateTemplateCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::vk1_1::DescriptorUpdateTemplateCreateFlags,
    pub descriptor_update_entry_count: u32,
    pub p_descriptor_update_entries: *const crate::vk1_1::DescriptorUpdateTemplateEntry,
    pub template_type: crate::vk1_1::DescriptorUpdateTemplateType,
    pub descriptor_set_layout: crate::vk1_0::DescriptorSetLayout,
    pub pipeline_bind_point: crate::vk1_0::PipelineBindPoint,
    pub pipeline_layout: crate::vk1_0::PipelineLayout,
    pub set: u32,
}
impl Default for DescriptorUpdateTemplateCreateInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            descriptor_update_entry_count: Default::default(),
            p_descriptor_update_entries: std::ptr::null(),
            template_type: Default::default(),
            descriptor_set_layout: Default::default(),
            pipeline_bind_point: Default::default(),
            pipeline_layout: Default::default(),
            set: Default::default(),
        }
    }
}
impl std::fmt::Debug for DescriptorUpdateTemplateCreateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DescriptorUpdateTemplateCreateInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("descriptor_update_entry_count", &self.descriptor_update_entry_count)
            .field("p_descriptor_update_entries", &self.p_descriptor_update_entries)
            .field("template_type", &self.template_type)
            .field("descriptor_set_layout", &self.descriptor_set_layout)
            .field("pipeline_bind_point", &self.pipeline_bind_point)
            .field("pipeline_layout", &self.pipeline_layout)
            .field("set", &self.set)
            .finish()
    }
}
impl DescriptorUpdateTemplateCreateInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> DescriptorUpdateTemplateCreateInfoBuilder<'a> {
        DescriptorUpdateTemplateCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorUpdateTemplateCreateInfo.html) · Builder of [`DescriptorUpdateTemplateCreateInfo`]"]
#[repr(transparent)]
pub struct DescriptorUpdateTemplateCreateInfoBuilder<'a>(DescriptorUpdateTemplateCreateInfo, std::marker::PhantomData<&'a ()>);
impl<'a> DescriptorUpdateTemplateCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> DescriptorUpdateTemplateCreateInfoBuilder<'a> {
        DescriptorUpdateTemplateCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_1::DescriptorUpdateTemplateCreateFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn descriptor_update_entries(mut self, descriptor_update_entries: &'a [impl crate::Repr<crate::vk1_1::DescriptorUpdateTemplateEntry>]) -> Self {
        self.0.p_descriptor_update_entries = descriptor_update_entries.as_ptr() as _;
        self.0.descriptor_update_entry_count = descriptor_update_entries.len() as _;
        self
    }
    #[inline]
    pub fn template_type(mut self, template_type: crate::vk1_1::DescriptorUpdateTemplateType) -> Self {
        self.0.template_type = template_type as _;
        self
    }
    #[inline]
    pub fn descriptor_set_layout(mut self, descriptor_set_layout: crate::vk1_0::DescriptorSetLayout) -> Self {
        self.0.descriptor_set_layout = descriptor_set_layout as _;
        self
    }
    #[inline]
    pub fn pipeline_bind_point(mut self, pipeline_bind_point: crate::vk1_0::PipelineBindPoint) -> Self {
        self.0.pipeline_bind_point = pipeline_bind_point as _;
        self
    }
    #[inline]
    pub fn pipeline_layout(mut self, pipeline_layout: crate::vk1_0::PipelineLayout) -> Self {
        self.0.pipeline_layout = pipeline_layout as _;
        self
    }
    #[inline]
    pub fn set(mut self, set: u32) -> Self {
        self.0.set = set as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DescriptorUpdateTemplateCreateInfo {
        self.0
    }
}
impl<'a> std::default::Default for DescriptorUpdateTemplateCreateInfoBuilder<'a> {
    fn default() -> DescriptorUpdateTemplateCreateInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DescriptorUpdateTemplateCreateInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DescriptorUpdateTemplateCreateInfoBuilder<'a> {
    type Target = DescriptorUpdateTemplateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DescriptorUpdateTemplateCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<DescriptorUpdateTemplateCreateInfo> for DescriptorUpdateTemplateCreateInfoBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkInputAttachmentAspectReference.html) · Structure"]
#[doc(alias = "VkInputAttachmentAspectReference")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InputAttachmentAspectReference {
    pub subpass: u32,
    pub input_attachment_index: u32,
    pub aspect_mask: crate::vk1_0::ImageAspectFlags,
}
impl Default for InputAttachmentAspectReference {
    fn default() -> Self {
        Self {
            subpass: Default::default(),
            input_attachment_index: Default::default(),
            aspect_mask: Default::default(),
        }
    }
}
impl std::fmt::Debug for InputAttachmentAspectReference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InputAttachmentAspectReference")
            .field("subpass", &self.subpass)
            .field("input_attachment_index", &self.input_attachment_index)
            .field("aspect_mask", &self.aspect_mask)
            .finish()
    }
}
impl InputAttachmentAspectReference {
    #[inline]
    pub fn into_builder<'a>(self) -> InputAttachmentAspectReferenceBuilder<'a> {
        InputAttachmentAspectReferenceBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkInputAttachmentAspectReference.html) · Builder of [`InputAttachmentAspectReference`]"]
#[repr(transparent)]
pub struct InputAttachmentAspectReferenceBuilder<'a>(InputAttachmentAspectReference, std::marker::PhantomData<&'a ()>);
impl<'a> InputAttachmentAspectReferenceBuilder<'a> {
    #[inline]
    pub fn new() -> InputAttachmentAspectReferenceBuilder<'a> {
        InputAttachmentAspectReferenceBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn subpass(mut self, subpass: u32) -> Self {
        self.0.subpass = subpass as _;
        self
    }
    #[inline]
    pub fn input_attachment_index(mut self, input_attachment_index: u32) -> Self {
        self.0.input_attachment_index = input_attachment_index as _;
        self
    }
    #[inline]
    pub fn aspect_mask(mut self, aspect_mask: crate::vk1_0::ImageAspectFlags) -> Self {
        self.0.aspect_mask = aspect_mask as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> InputAttachmentAspectReference {
        self.0
    }
}
impl<'a> std::default::Default for InputAttachmentAspectReferenceBuilder<'a> {
    fn default() -> InputAttachmentAspectReferenceBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for InputAttachmentAspectReferenceBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for InputAttachmentAspectReferenceBuilder<'a> {
    type Target = InputAttachmentAspectReference;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for InputAttachmentAspectReferenceBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<InputAttachmentAspectReference> for InputAttachmentAspectReferenceBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderPassInputAttachmentAspectCreateInfo.html) · Structure"]
#[doc(alias = "VkRenderPassInputAttachmentAspectCreateInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RenderPassInputAttachmentAspectCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub aspect_reference_count: u32,
    pub p_aspect_references: *const crate::vk1_1::InputAttachmentAspectReference,
}
impl Default for RenderPassInputAttachmentAspectCreateInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO,
            p_next: std::ptr::null(),
            aspect_reference_count: Default::default(),
            p_aspect_references: std::ptr::null(),
        }
    }
}
impl std::fmt::Debug for RenderPassInputAttachmentAspectCreateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RenderPassInputAttachmentAspectCreateInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("aspect_reference_count", &self.aspect_reference_count)
            .field("p_aspect_references", &self.p_aspect_references)
            .finish()
    }
}
impl RenderPassInputAttachmentAspectCreateInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> RenderPassInputAttachmentAspectCreateInfoBuilder<'a> {
        RenderPassInputAttachmentAspectCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderPassInputAttachmentAspectCreateInfo.html) · Builder of [`RenderPassInputAttachmentAspectCreateInfo`]"]
#[repr(transparent)]
pub struct RenderPassInputAttachmentAspectCreateInfoBuilder<'a>(RenderPassInputAttachmentAspectCreateInfo, std::marker::PhantomData<&'a ()>);
impl<'a> RenderPassInputAttachmentAspectCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> RenderPassInputAttachmentAspectCreateInfoBuilder<'a> {
        RenderPassInputAttachmentAspectCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn aspect_references(mut self, aspect_references: &'a [impl crate::Repr<crate::vk1_1::InputAttachmentAspectReference>]) -> Self {
        self.0.p_aspect_references = aspect_references.as_ptr() as _;
        self.0.aspect_reference_count = aspect_references.len() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> RenderPassInputAttachmentAspectCreateInfo {
        self.0
    }
}
impl<'a> std::default::Default for RenderPassInputAttachmentAspectCreateInfoBuilder<'a> {
    fn default() -> RenderPassInputAttachmentAspectCreateInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for RenderPassInputAttachmentAspectCreateInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for RenderPassInputAttachmentAspectCreateInfoBuilder<'a> {
    type Target = RenderPassInputAttachmentAspectCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for RenderPassInputAttachmentAspectCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<RenderPassInputAttachmentAspectCreateInfo> for RenderPassInputAttachmentAspectCreateInfoBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevice16BitStorageFeatures.html) · Structure"]
#[doc(alias = "VkPhysicalDevice16BitStorageFeatures")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDevice16BitStorageFeatures {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub storage_buffer16_bit_access: crate::vk1_0::Bool32,
    pub uniform_and_storage_buffer16_bit_access: crate::vk1_0::Bool32,
    pub storage_push_constant16: crate::vk1_0::Bool32,
    pub storage_input_output16: crate::vk1_0::Bool32,
}
impl Default for PhysicalDevice16BitStorageFeatures {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES,
            p_next: std::ptr::null_mut(),
            storage_buffer16_bit_access: Default::default(),
            uniform_and_storage_buffer16_bit_access: Default::default(),
            storage_push_constant16: Default::default(),
            storage_input_output16: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDevice16BitStorageFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDevice16BitStorageFeatures")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("storage_buffer16_bit_access", &(self.storage_buffer16_bit_access != 0))
            .field("uniform_and_storage_buffer16_bit_access", &(self.uniform_and_storage_buffer16_bit_access != 0))
            .field("storage_push_constant16", &(self.storage_push_constant16 != 0))
            .field("storage_input_output16", &(self.storage_input_output16 != 0))
            .finish()
    }
}
impl PhysicalDevice16BitStorageFeatures {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDevice16BitStorageFeaturesBuilder<'a> {
        PhysicalDevice16BitStorageFeaturesBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevice16BitStorageFeatures.html) · Builder of [`PhysicalDevice16BitStorageFeatures`]"]
#[repr(transparent)]
pub struct PhysicalDevice16BitStorageFeaturesBuilder<'a>(PhysicalDevice16BitStorageFeatures, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDevice16BitStorageFeaturesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDevice16BitStorageFeaturesBuilder<'a> {
        PhysicalDevice16BitStorageFeaturesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn storage_buffer16_bit_access(mut self, storage_buffer16_bit_access: bool) -> Self {
        self.0.storage_buffer16_bit_access = storage_buffer16_bit_access as _;
        self
    }
    #[inline]
    pub fn uniform_and_storage_buffer16_bit_access(mut self, uniform_and_storage_buffer16_bit_access: bool) -> Self {
        self.0.uniform_and_storage_buffer16_bit_access = uniform_and_storage_buffer16_bit_access as _;
        self
    }
    #[inline]
    pub fn storage_push_constant16(mut self, storage_push_constant16: bool) -> Self {
        self.0.storage_push_constant16 = storage_push_constant16 as _;
        self
    }
    #[inline]
    pub fn storage_input_output16(mut self, storage_input_output16: bool) -> Self {
        self.0.storage_input_output16 = storage_input_output16 as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDevice16BitStorageFeatures {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDevice16BitStorageFeaturesBuilder<'a> {
    fn default() -> PhysicalDevice16BitStorageFeaturesBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDevice16BitStorageFeaturesBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDevice16BitStorageFeaturesBuilder<'a> {
    type Target = PhysicalDevice16BitStorageFeatures;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDevice16BitStorageFeaturesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<PhysicalDevice16BitStorageFeatures> for PhysicalDevice16BitStorageFeaturesBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSubgroupProperties.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceSubgroupProperties")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceSubgroupProperties {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub subgroup_size: u32,
    pub supported_stages: crate::vk1_0::ShaderStageFlags,
    pub supported_operations: crate::vk1_1::SubgroupFeatureFlags,
    pub quad_operations_in_all_stages: crate::vk1_0::Bool32,
}
impl Default for PhysicalDeviceSubgroupProperties {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_SUBGROUP_PROPERTIES,
            p_next: std::ptr::null_mut(),
            subgroup_size: Default::default(),
            supported_stages: Default::default(),
            supported_operations: Default::default(),
            quad_operations_in_all_stages: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceSubgroupProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceSubgroupProperties")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("subgroup_size", &self.subgroup_size)
            .field("supported_stages", &self.supported_stages)
            .field("supported_operations", &self.supported_operations)
            .field("quad_operations_in_all_stages", &(self.quad_operations_in_all_stages != 0))
            .finish()
    }
}
impl PhysicalDeviceSubgroupProperties {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceSubgroupPropertiesBuilder<'a> {
        PhysicalDeviceSubgroupPropertiesBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSubgroupProperties.html) · Builder of [`PhysicalDeviceSubgroupProperties`]"]
#[repr(transparent)]
pub struct PhysicalDeviceSubgroupPropertiesBuilder<'a>(PhysicalDeviceSubgroupProperties, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceSubgroupPropertiesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceSubgroupPropertiesBuilder<'a> {
        PhysicalDeviceSubgroupPropertiesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn subgroup_size(mut self, subgroup_size: u32) -> Self {
        self.0.subgroup_size = subgroup_size as _;
        self
    }
    #[inline]
    pub fn supported_stages(mut self, supported_stages: crate::vk1_0::ShaderStageFlags) -> Self {
        self.0.supported_stages = supported_stages as _;
        self
    }
    #[inline]
    pub fn supported_operations(mut self, supported_operations: crate::vk1_1::SubgroupFeatureFlags) -> Self {
        self.0.supported_operations = supported_operations as _;
        self
    }
    #[inline]
    pub fn quad_operations_in_all_stages(mut self, quad_operations_in_all_stages: bool) -> Self {
        self.0.quad_operations_in_all_stages = quad_operations_in_all_stages as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceSubgroupProperties {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceSubgroupPropertiesBuilder<'a> {
    fn default() -> PhysicalDeviceSubgroupPropertiesBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceSubgroupPropertiesBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceSubgroupPropertiesBuilder<'a> {
    type Target = PhysicalDeviceSubgroupProperties;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceSubgroupPropertiesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<PhysicalDeviceSubgroupProperties> for PhysicalDeviceSubgroupPropertiesBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferMemoryRequirementsInfo2.html) · Structure"]
#[doc(alias = "VkBufferMemoryRequirementsInfo2")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BufferMemoryRequirementsInfo2 {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub buffer: crate::vk1_0::Buffer,
}
impl Default for BufferMemoryRequirementsInfo2 {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::BUFFER_MEMORY_REQUIREMENTS_INFO_2,
            p_next: std::ptr::null(),
            buffer: Default::default(),
        }
    }
}
impl std::fmt::Debug for BufferMemoryRequirementsInfo2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BufferMemoryRequirementsInfo2")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("buffer", &self.buffer)
            .finish()
    }
}
impl BufferMemoryRequirementsInfo2 {
    #[inline]
    pub fn into_builder<'a>(self) -> BufferMemoryRequirementsInfo2Builder<'a> {
        BufferMemoryRequirementsInfo2Builder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferMemoryRequirementsInfo2.html) · Builder of [`BufferMemoryRequirementsInfo2`]"]
#[repr(transparent)]
pub struct BufferMemoryRequirementsInfo2Builder<'a>(BufferMemoryRequirementsInfo2, std::marker::PhantomData<&'a ()>);
impl<'a> BufferMemoryRequirementsInfo2Builder<'a> {
    #[inline]
    pub fn new() -> BufferMemoryRequirementsInfo2Builder<'a> {
        BufferMemoryRequirementsInfo2Builder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn buffer(mut self, buffer: crate::vk1_0::Buffer) -> Self {
        self.0.buffer = buffer as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> BufferMemoryRequirementsInfo2 {
        self.0
    }
}
impl<'a> std::default::Default for BufferMemoryRequirementsInfo2Builder<'a> {
    fn default() -> BufferMemoryRequirementsInfo2Builder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for BufferMemoryRequirementsInfo2Builder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for BufferMemoryRequirementsInfo2Builder<'a> {
    type Target = BufferMemoryRequirementsInfo2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for BufferMemoryRequirementsInfo2Builder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<BufferMemoryRequirementsInfo2> for BufferMemoryRequirementsInfo2Builder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageMemoryRequirementsInfo2.html) · Structure"]
#[doc(alias = "VkImageMemoryRequirementsInfo2")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImageMemoryRequirementsInfo2 {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub image: crate::vk1_0::Image,
}
impl Default for ImageMemoryRequirementsInfo2 {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::IMAGE_MEMORY_REQUIREMENTS_INFO_2,
            p_next: std::ptr::null(),
            image: Default::default(),
        }
    }
}
impl std::fmt::Debug for ImageMemoryRequirementsInfo2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ImageMemoryRequirementsInfo2")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("image", &self.image)
            .finish()
    }
}
impl ImageMemoryRequirementsInfo2 {
    #[inline]
    pub fn into_builder<'a>(self) -> ImageMemoryRequirementsInfo2Builder<'a> {
        ImageMemoryRequirementsInfo2Builder(self, std::marker::PhantomData)
    }
}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_1::ImagePlaneMemoryRequirementsInfo> for ImageMemoryRequirementsInfo2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_1::ImagePlaneMemoryRequirementsInfoBuilder<'_>> for ImageMemoryRequirementsInfo2Builder<'a> {}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageMemoryRequirementsInfo2.html) · Builder of [`ImageMemoryRequirementsInfo2`]"]
#[repr(transparent)]
pub struct ImageMemoryRequirementsInfo2Builder<'a>(ImageMemoryRequirementsInfo2, std::marker::PhantomData<&'a ()>);
impl<'a> ImageMemoryRequirementsInfo2Builder<'a> {
    #[inline]
    pub fn new() -> ImageMemoryRequirementsInfo2Builder<'a> {
        ImageMemoryRequirementsInfo2Builder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn image(mut self, image: crate::vk1_0::Image) -> Self {
        self.0.image = image as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ImageMemoryRequirementsInfo2 {
        self.0
    }
}
impl<'a> std::default::Default for ImageMemoryRequirementsInfo2Builder<'a> {
    fn default() -> ImageMemoryRequirementsInfo2Builder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ImageMemoryRequirementsInfo2Builder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ImageMemoryRequirementsInfo2Builder<'a> {
    type Target = ImageMemoryRequirementsInfo2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ImageMemoryRequirementsInfo2Builder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<ImageMemoryRequirementsInfo2> for ImageMemoryRequirementsInfo2Builder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageSparseMemoryRequirementsInfo2.html) · Structure"]
#[doc(alias = "VkImageSparseMemoryRequirementsInfo2")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImageSparseMemoryRequirementsInfo2 {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub image: crate::vk1_0::Image,
}
impl Default for ImageSparseMemoryRequirementsInfo2 {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2,
            p_next: std::ptr::null(),
            image: Default::default(),
        }
    }
}
impl std::fmt::Debug for ImageSparseMemoryRequirementsInfo2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ImageSparseMemoryRequirementsInfo2")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("image", &self.image)
            .finish()
    }
}
impl ImageSparseMemoryRequirementsInfo2 {
    #[inline]
    pub fn into_builder<'a>(self) -> ImageSparseMemoryRequirementsInfo2Builder<'a> {
        ImageSparseMemoryRequirementsInfo2Builder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageSparseMemoryRequirementsInfo2.html) · Builder of [`ImageSparseMemoryRequirementsInfo2`]"]
#[repr(transparent)]
pub struct ImageSparseMemoryRequirementsInfo2Builder<'a>(ImageSparseMemoryRequirementsInfo2, std::marker::PhantomData<&'a ()>);
impl<'a> ImageSparseMemoryRequirementsInfo2Builder<'a> {
    #[inline]
    pub fn new() -> ImageSparseMemoryRequirementsInfo2Builder<'a> {
        ImageSparseMemoryRequirementsInfo2Builder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn image(mut self, image: crate::vk1_0::Image) -> Self {
        self.0.image = image as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ImageSparseMemoryRequirementsInfo2 {
        self.0
    }
}
impl<'a> std::default::Default for ImageSparseMemoryRequirementsInfo2Builder<'a> {
    fn default() -> ImageSparseMemoryRequirementsInfo2Builder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ImageSparseMemoryRequirementsInfo2Builder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ImageSparseMemoryRequirementsInfo2Builder<'a> {
    type Target = ImageSparseMemoryRequirementsInfo2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ImageSparseMemoryRequirementsInfo2Builder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<ImageSparseMemoryRequirementsInfo2> for ImageSparseMemoryRequirementsInfo2Builder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryRequirements2.html) · Structure"]
#[doc(alias = "VkMemoryRequirements2")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryRequirements2 {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub memory_requirements: crate::vk1_0::MemoryRequirements,
}
impl Default for MemoryRequirements2 {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::MEMORY_REQUIREMENTS_2,
            p_next: std::ptr::null_mut(),
            memory_requirements: Default::default(),
        }
    }
}
impl std::fmt::Debug for MemoryRequirements2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MemoryRequirements2")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("memory_requirements", &self.memory_requirements)
            .finish()
    }
}
impl MemoryRequirements2 {
    #[inline]
    pub fn into_builder<'a>(self) -> MemoryRequirements2Builder<'a> {
        MemoryRequirements2Builder(self, std::marker::PhantomData)
    }
}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_1::MemoryDedicatedRequirements> for MemoryRequirements2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_1::MemoryDedicatedRequirementsBuilder<'_>> for MemoryRequirements2Builder<'a> {}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryRequirements2.html) · Builder of [`MemoryRequirements2`]"]
#[repr(transparent)]
pub struct MemoryRequirements2Builder<'a>(MemoryRequirements2, std::marker::PhantomData<&'a ()>);
impl<'a> MemoryRequirements2Builder<'a> {
    #[inline]
    pub fn new() -> MemoryRequirements2Builder<'a> {
        MemoryRequirements2Builder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn memory_requirements(mut self, memory_requirements: crate::vk1_0::MemoryRequirements) -> Self {
        self.0.memory_requirements = memory_requirements as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> MemoryRequirements2 {
        self.0
    }
}
impl<'a> std::default::Default for MemoryRequirements2Builder<'a> {
    fn default() -> MemoryRequirements2Builder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for MemoryRequirements2Builder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for MemoryRequirements2Builder<'a> {
    type Target = MemoryRequirements2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for MemoryRequirements2Builder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<MemoryRequirements2> for MemoryRequirements2Builder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSparseImageMemoryRequirements2.html) · Structure"]
#[doc(alias = "VkSparseImageMemoryRequirements2")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SparseImageMemoryRequirements2 {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub memory_requirements: crate::vk1_0::SparseImageMemoryRequirements,
}
impl Default for SparseImageMemoryRequirements2 {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::SPARSE_IMAGE_MEMORY_REQUIREMENTS_2,
            p_next: std::ptr::null_mut(),
            memory_requirements: Default::default(),
        }
    }
}
impl std::fmt::Debug for SparseImageMemoryRequirements2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SparseImageMemoryRequirements2")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("memory_requirements", &self.memory_requirements)
            .finish()
    }
}
impl SparseImageMemoryRequirements2 {
    #[inline]
    pub fn into_builder<'a>(self) -> SparseImageMemoryRequirements2Builder<'a> {
        SparseImageMemoryRequirements2Builder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSparseImageMemoryRequirements2.html) · Builder of [`SparseImageMemoryRequirements2`]"]
#[repr(transparent)]
pub struct SparseImageMemoryRequirements2Builder<'a>(SparseImageMemoryRequirements2, std::marker::PhantomData<&'a ()>);
impl<'a> SparseImageMemoryRequirements2Builder<'a> {
    #[inline]
    pub fn new() -> SparseImageMemoryRequirements2Builder<'a> {
        SparseImageMemoryRequirements2Builder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn memory_requirements(mut self, memory_requirements: crate::vk1_0::SparseImageMemoryRequirements) -> Self {
        self.0.memory_requirements = memory_requirements as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> SparseImageMemoryRequirements2 {
        self.0
    }
}
impl<'a> std::default::Default for SparseImageMemoryRequirements2Builder<'a> {
    fn default() -> SparseImageMemoryRequirements2Builder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for SparseImageMemoryRequirements2Builder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for SparseImageMemoryRequirements2Builder<'a> {
    type Target = SparseImageMemoryRequirements2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SparseImageMemoryRequirements2Builder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<SparseImageMemoryRequirements2> for SparseImageMemoryRequirements2Builder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevicePointClippingProperties.html) · Structure"]
#[doc(alias = "VkPhysicalDevicePointClippingProperties")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDevicePointClippingProperties {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub point_clipping_behavior: crate::vk1_1::PointClippingBehavior,
}
impl Default for PhysicalDevicePointClippingProperties {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES,
            p_next: std::ptr::null_mut(),
            point_clipping_behavior: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDevicePointClippingProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDevicePointClippingProperties")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("point_clipping_behavior", &self.point_clipping_behavior)
            .finish()
    }
}
impl PhysicalDevicePointClippingProperties {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDevicePointClippingPropertiesBuilder<'a> {
        PhysicalDevicePointClippingPropertiesBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevicePointClippingProperties.html) · Builder of [`PhysicalDevicePointClippingProperties`]"]
#[repr(transparent)]
pub struct PhysicalDevicePointClippingPropertiesBuilder<'a>(PhysicalDevicePointClippingProperties, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDevicePointClippingPropertiesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDevicePointClippingPropertiesBuilder<'a> {
        PhysicalDevicePointClippingPropertiesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn point_clipping_behavior(mut self, point_clipping_behavior: crate::vk1_1::PointClippingBehavior) -> Self {
        self.0.point_clipping_behavior = point_clipping_behavior as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDevicePointClippingProperties {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDevicePointClippingPropertiesBuilder<'a> {
    fn default() -> PhysicalDevicePointClippingPropertiesBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDevicePointClippingPropertiesBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDevicePointClippingPropertiesBuilder<'a> {
    type Target = PhysicalDevicePointClippingProperties;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDevicePointClippingPropertiesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<PhysicalDevicePointClippingProperties> for PhysicalDevicePointClippingPropertiesBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryDedicatedRequirements.html) · Structure"]
#[doc(alias = "VkMemoryDedicatedRequirements")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryDedicatedRequirements {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub prefers_dedicated_allocation: crate::vk1_0::Bool32,
    pub requires_dedicated_allocation: crate::vk1_0::Bool32,
}
impl Default for MemoryDedicatedRequirements {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::MEMORY_DEDICATED_REQUIREMENTS,
            p_next: std::ptr::null_mut(),
            prefers_dedicated_allocation: Default::default(),
            requires_dedicated_allocation: Default::default(),
        }
    }
}
impl std::fmt::Debug for MemoryDedicatedRequirements {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MemoryDedicatedRequirements")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("prefers_dedicated_allocation", &(self.prefers_dedicated_allocation != 0))
            .field("requires_dedicated_allocation", &(self.requires_dedicated_allocation != 0))
            .finish()
    }
}
impl MemoryDedicatedRequirements {
    #[inline]
    pub fn into_builder<'a>(self) -> MemoryDedicatedRequirementsBuilder<'a> {
        MemoryDedicatedRequirementsBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryDedicatedRequirements.html) · Builder of [`MemoryDedicatedRequirements`]"]
#[repr(transparent)]
pub struct MemoryDedicatedRequirementsBuilder<'a>(MemoryDedicatedRequirements, std::marker::PhantomData<&'a ()>);
impl<'a> MemoryDedicatedRequirementsBuilder<'a> {
    #[inline]
    pub fn new() -> MemoryDedicatedRequirementsBuilder<'a> {
        MemoryDedicatedRequirementsBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn prefers_dedicated_allocation(mut self, prefers_dedicated_allocation: bool) -> Self {
        self.0.prefers_dedicated_allocation = prefers_dedicated_allocation as _;
        self
    }
    #[inline]
    pub fn requires_dedicated_allocation(mut self, requires_dedicated_allocation: bool) -> Self {
        self.0.requires_dedicated_allocation = requires_dedicated_allocation as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> MemoryDedicatedRequirements {
        self.0
    }
}
impl<'a> std::default::Default for MemoryDedicatedRequirementsBuilder<'a> {
    fn default() -> MemoryDedicatedRequirementsBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for MemoryDedicatedRequirementsBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for MemoryDedicatedRequirementsBuilder<'a> {
    type Target = MemoryDedicatedRequirements;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for MemoryDedicatedRequirementsBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<MemoryDedicatedRequirements> for MemoryDedicatedRequirementsBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryDedicatedAllocateInfo.html) · Structure"]
#[doc(alias = "VkMemoryDedicatedAllocateInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryDedicatedAllocateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub image: crate::vk1_0::Image,
    pub buffer: crate::vk1_0::Buffer,
}
impl Default for MemoryDedicatedAllocateInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::MEMORY_DEDICATED_ALLOCATE_INFO,
            p_next: std::ptr::null(),
            image: Default::default(),
            buffer: Default::default(),
        }
    }
}
impl std::fmt::Debug for MemoryDedicatedAllocateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MemoryDedicatedAllocateInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("image", &self.image)
            .field("buffer", &self.buffer)
            .finish()
    }
}
impl MemoryDedicatedAllocateInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> MemoryDedicatedAllocateInfoBuilder<'a> {
        MemoryDedicatedAllocateInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryDedicatedAllocateInfo.html) · Builder of [`MemoryDedicatedAllocateInfo`]"]
#[repr(transparent)]
pub struct MemoryDedicatedAllocateInfoBuilder<'a>(MemoryDedicatedAllocateInfo, std::marker::PhantomData<&'a ()>);
impl<'a> MemoryDedicatedAllocateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> MemoryDedicatedAllocateInfoBuilder<'a> {
        MemoryDedicatedAllocateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn image(mut self, image: crate::vk1_0::Image) -> Self {
        self.0.image = image as _;
        self
    }
    #[inline]
    pub fn buffer(mut self, buffer: crate::vk1_0::Buffer) -> Self {
        self.0.buffer = buffer as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> MemoryDedicatedAllocateInfo {
        self.0
    }
}
impl<'a> std::default::Default for MemoryDedicatedAllocateInfoBuilder<'a> {
    fn default() -> MemoryDedicatedAllocateInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for MemoryDedicatedAllocateInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for MemoryDedicatedAllocateInfoBuilder<'a> {
    type Target = MemoryDedicatedAllocateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for MemoryDedicatedAllocateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<MemoryDedicatedAllocateInfo> for MemoryDedicatedAllocateInfoBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageViewUsageCreateInfo.html) · Structure"]
#[doc(alias = "VkImageViewUsageCreateInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImageViewUsageCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub usage: crate::vk1_0::ImageUsageFlags,
}
impl Default for ImageViewUsageCreateInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::IMAGE_VIEW_USAGE_CREATE_INFO,
            p_next: std::ptr::null(),
            usage: Default::default(),
        }
    }
}
impl std::fmt::Debug for ImageViewUsageCreateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ImageViewUsageCreateInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("usage", &self.usage)
            .finish()
    }
}
impl ImageViewUsageCreateInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> ImageViewUsageCreateInfoBuilder<'a> {
        ImageViewUsageCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageViewUsageCreateInfo.html) · Builder of [`ImageViewUsageCreateInfo`]"]
#[repr(transparent)]
pub struct ImageViewUsageCreateInfoBuilder<'a>(ImageViewUsageCreateInfo, std::marker::PhantomData<&'a ()>);
impl<'a> ImageViewUsageCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> ImageViewUsageCreateInfoBuilder<'a> {
        ImageViewUsageCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn usage(mut self, usage: crate::vk1_0::ImageUsageFlags) -> Self {
        self.0.usage = usage as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ImageViewUsageCreateInfo {
        self.0
    }
}
impl<'a> std::default::Default for ImageViewUsageCreateInfoBuilder<'a> {
    fn default() -> ImageViewUsageCreateInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ImageViewUsageCreateInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ImageViewUsageCreateInfoBuilder<'a> {
    type Target = ImageViewUsageCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ImageViewUsageCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<ImageViewUsageCreateInfo> for ImageViewUsageCreateInfoBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineTessellationDomainOriginStateCreateInfo.html) · Structure"]
#[doc(alias = "VkPipelineTessellationDomainOriginStateCreateInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineTessellationDomainOriginStateCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub domain_origin: crate::vk1_1::TessellationDomainOrigin,
}
impl Default for PipelineTessellationDomainOriginStateCreateInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO,
            p_next: std::ptr::null(),
            domain_origin: Default::default(),
        }
    }
}
impl std::fmt::Debug for PipelineTessellationDomainOriginStateCreateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PipelineTessellationDomainOriginStateCreateInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("domain_origin", &self.domain_origin)
            .finish()
    }
}
impl PipelineTessellationDomainOriginStateCreateInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> PipelineTessellationDomainOriginStateCreateInfoBuilder<'a> {
        PipelineTessellationDomainOriginStateCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineTessellationDomainOriginStateCreateInfo.html) · Builder of [`PipelineTessellationDomainOriginStateCreateInfo`]"]
#[repr(transparent)]
pub struct PipelineTessellationDomainOriginStateCreateInfoBuilder<'a>(PipelineTessellationDomainOriginStateCreateInfo, std::marker::PhantomData<&'a ()>);
impl<'a> PipelineTessellationDomainOriginStateCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineTessellationDomainOriginStateCreateInfoBuilder<'a> {
        PipelineTessellationDomainOriginStateCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn domain_origin(mut self, domain_origin: crate::vk1_1::TessellationDomainOrigin) -> Self {
        self.0.domain_origin = domain_origin as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PipelineTessellationDomainOriginStateCreateInfo {
        self.0
    }
}
impl<'a> std::default::Default for PipelineTessellationDomainOriginStateCreateInfoBuilder<'a> {
    fn default() -> PipelineTessellationDomainOriginStateCreateInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PipelineTessellationDomainOriginStateCreateInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PipelineTessellationDomainOriginStateCreateInfoBuilder<'a> {
    type Target = PipelineTessellationDomainOriginStateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineTessellationDomainOriginStateCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<PipelineTessellationDomainOriginStateCreateInfo> for PipelineTessellationDomainOriginStateCreateInfoBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerYcbcrConversionInfo.html) · Structure"]
#[doc(alias = "VkSamplerYcbcrConversionInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SamplerYcbcrConversionInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub conversion: crate::vk1_1::SamplerYcbcrConversion,
}
impl Default for SamplerYcbcrConversionInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::SAMPLER_YCBCR_CONVERSION_INFO,
            p_next: std::ptr::null(),
            conversion: Default::default(),
        }
    }
}
impl std::fmt::Debug for SamplerYcbcrConversionInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SamplerYcbcrConversionInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("conversion", &self.conversion)
            .finish()
    }
}
impl SamplerYcbcrConversionInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> SamplerYcbcrConversionInfoBuilder<'a> {
        SamplerYcbcrConversionInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerYcbcrConversionInfo.html) · Builder of [`SamplerYcbcrConversionInfo`]"]
#[repr(transparent)]
pub struct SamplerYcbcrConversionInfoBuilder<'a>(SamplerYcbcrConversionInfo, std::marker::PhantomData<&'a ()>);
impl<'a> SamplerYcbcrConversionInfoBuilder<'a> {
    #[inline]
    pub fn new() -> SamplerYcbcrConversionInfoBuilder<'a> {
        SamplerYcbcrConversionInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn conversion(mut self, conversion: crate::vk1_1::SamplerYcbcrConversion) -> Self {
        self.0.conversion = conversion as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> SamplerYcbcrConversionInfo {
        self.0
    }
}
impl<'a> std::default::Default for SamplerYcbcrConversionInfoBuilder<'a> {
    fn default() -> SamplerYcbcrConversionInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for SamplerYcbcrConversionInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for SamplerYcbcrConversionInfoBuilder<'a> {
    type Target = SamplerYcbcrConversionInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SamplerYcbcrConversionInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<SamplerYcbcrConversionInfo> for SamplerYcbcrConversionInfoBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerYcbcrConversionCreateInfo.html) · Structure"]
#[doc(alias = "VkSamplerYcbcrConversionCreateInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SamplerYcbcrConversionCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub format: crate::vk1_0::Format,
    pub ycbcr_model: crate::vk1_1::SamplerYcbcrModelConversion,
    pub ycbcr_range: crate::vk1_1::SamplerYcbcrRange,
    pub components: crate::vk1_0::ComponentMapping,
    pub x_chroma_offset: crate::vk1_1::ChromaLocation,
    pub y_chroma_offset: crate::vk1_1::ChromaLocation,
    pub chroma_filter: crate::vk1_0::Filter,
    pub force_explicit_reconstruction: crate::vk1_0::Bool32,
}
impl Default for SamplerYcbcrConversionCreateInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::SAMPLER_YCBCR_CONVERSION_CREATE_INFO,
            p_next: std::ptr::null(),
            format: Default::default(),
            ycbcr_model: Default::default(),
            ycbcr_range: Default::default(),
            components: Default::default(),
            x_chroma_offset: Default::default(),
            y_chroma_offset: Default::default(),
            chroma_filter: Default::default(),
            force_explicit_reconstruction: Default::default(),
        }
    }
}
impl std::fmt::Debug for SamplerYcbcrConversionCreateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SamplerYcbcrConversionCreateInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("format", &self.format)
            .field("ycbcr_model", &self.ycbcr_model)
            .field("ycbcr_range", &self.ycbcr_range)
            .field("components", &self.components)
            .field("x_chroma_offset", &self.x_chroma_offset)
            .field("y_chroma_offset", &self.y_chroma_offset)
            .field("chroma_filter", &self.chroma_filter)
            .field("force_explicit_reconstruction", &(self.force_explicit_reconstruction != 0))
            .finish()
    }
}
impl SamplerYcbcrConversionCreateInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> SamplerYcbcrConversionCreateInfoBuilder<'a> {
        SamplerYcbcrConversionCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::android_external_memory_android_hardware_buffer::ExternalFormatANDROID> for SamplerYcbcrConversionCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::android_external_memory_android_hardware_buffer::ExternalFormatANDROIDBuilder<'_>> for SamplerYcbcrConversionCreateInfoBuilder<'a> {}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerYcbcrConversionCreateInfo.html) · Builder of [`SamplerYcbcrConversionCreateInfo`]"]
#[repr(transparent)]
pub struct SamplerYcbcrConversionCreateInfoBuilder<'a>(SamplerYcbcrConversionCreateInfo, std::marker::PhantomData<&'a ()>);
impl<'a> SamplerYcbcrConversionCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> SamplerYcbcrConversionCreateInfoBuilder<'a> {
        SamplerYcbcrConversionCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn format(mut self, format: crate::vk1_0::Format) -> Self {
        self.0.format = format as _;
        self
    }
    #[inline]
    pub fn ycbcr_model(mut self, ycbcr_model: crate::vk1_1::SamplerYcbcrModelConversion) -> Self {
        self.0.ycbcr_model = ycbcr_model as _;
        self
    }
    #[inline]
    pub fn ycbcr_range(mut self, ycbcr_range: crate::vk1_1::SamplerYcbcrRange) -> Self {
        self.0.ycbcr_range = ycbcr_range as _;
        self
    }
    #[inline]
    pub fn components(mut self, components: crate::vk1_0::ComponentMapping) -> Self {
        self.0.components = components as _;
        self
    }
    #[inline]
    pub fn x_chroma_offset(mut self, x_chroma_offset: crate::vk1_1::ChromaLocation) -> Self {
        self.0.x_chroma_offset = x_chroma_offset as _;
        self
    }
    #[inline]
    pub fn y_chroma_offset(mut self, y_chroma_offset: crate::vk1_1::ChromaLocation) -> Self {
        self.0.y_chroma_offset = y_chroma_offset as _;
        self
    }
    #[inline]
    pub fn chroma_filter(mut self, chroma_filter: crate::vk1_0::Filter) -> Self {
        self.0.chroma_filter = chroma_filter as _;
        self
    }
    #[inline]
    pub fn force_explicit_reconstruction(mut self, force_explicit_reconstruction: bool) -> Self {
        self.0.force_explicit_reconstruction = force_explicit_reconstruction as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> SamplerYcbcrConversionCreateInfo {
        self.0
    }
}
impl<'a> std::default::Default for SamplerYcbcrConversionCreateInfoBuilder<'a> {
    fn default() -> SamplerYcbcrConversionCreateInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for SamplerYcbcrConversionCreateInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for SamplerYcbcrConversionCreateInfoBuilder<'a> {
    type Target = SamplerYcbcrConversionCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SamplerYcbcrConversionCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<SamplerYcbcrConversionCreateInfo> for SamplerYcbcrConversionCreateInfoBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindImagePlaneMemoryInfo.html) · Structure"]
#[doc(alias = "VkBindImagePlaneMemoryInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BindImagePlaneMemoryInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub plane_aspect: crate::vk1_0::ImageAspectFlagBits,
}
impl Default for BindImagePlaneMemoryInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::BIND_IMAGE_PLANE_MEMORY_INFO,
            p_next: std::ptr::null(),
            plane_aspect: Default::default(),
        }
    }
}
impl std::fmt::Debug for BindImagePlaneMemoryInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BindImagePlaneMemoryInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("plane_aspect", &self.plane_aspect)
            .finish()
    }
}
impl BindImagePlaneMemoryInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> BindImagePlaneMemoryInfoBuilder<'a> {
        BindImagePlaneMemoryInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindImagePlaneMemoryInfo.html) · Builder of [`BindImagePlaneMemoryInfo`]"]
#[repr(transparent)]
pub struct BindImagePlaneMemoryInfoBuilder<'a>(BindImagePlaneMemoryInfo, std::marker::PhantomData<&'a ()>);
impl<'a> BindImagePlaneMemoryInfoBuilder<'a> {
    #[inline]
    pub fn new() -> BindImagePlaneMemoryInfoBuilder<'a> {
        BindImagePlaneMemoryInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn plane_aspect(mut self, plane_aspect: crate::vk1_0::ImageAspectFlagBits) -> Self {
        self.0.plane_aspect = plane_aspect as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> BindImagePlaneMemoryInfo {
        self.0
    }
}
impl<'a> std::default::Default for BindImagePlaneMemoryInfoBuilder<'a> {
    fn default() -> BindImagePlaneMemoryInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for BindImagePlaneMemoryInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for BindImagePlaneMemoryInfoBuilder<'a> {
    type Target = BindImagePlaneMemoryInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for BindImagePlaneMemoryInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<BindImagePlaneMemoryInfo> for BindImagePlaneMemoryInfoBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImagePlaneMemoryRequirementsInfo.html) · Structure"]
#[doc(alias = "VkImagePlaneMemoryRequirementsInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImagePlaneMemoryRequirementsInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub plane_aspect: crate::vk1_0::ImageAspectFlagBits,
}
impl Default for ImagePlaneMemoryRequirementsInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO,
            p_next: std::ptr::null(),
            plane_aspect: Default::default(),
        }
    }
}
impl std::fmt::Debug for ImagePlaneMemoryRequirementsInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ImagePlaneMemoryRequirementsInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("plane_aspect", &self.plane_aspect)
            .finish()
    }
}
impl ImagePlaneMemoryRequirementsInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> ImagePlaneMemoryRequirementsInfoBuilder<'a> {
        ImagePlaneMemoryRequirementsInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImagePlaneMemoryRequirementsInfo.html) · Builder of [`ImagePlaneMemoryRequirementsInfo`]"]
#[repr(transparent)]
pub struct ImagePlaneMemoryRequirementsInfoBuilder<'a>(ImagePlaneMemoryRequirementsInfo, std::marker::PhantomData<&'a ()>);
impl<'a> ImagePlaneMemoryRequirementsInfoBuilder<'a> {
    #[inline]
    pub fn new() -> ImagePlaneMemoryRequirementsInfoBuilder<'a> {
        ImagePlaneMemoryRequirementsInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn plane_aspect(mut self, plane_aspect: crate::vk1_0::ImageAspectFlagBits) -> Self {
        self.0.plane_aspect = plane_aspect as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ImagePlaneMemoryRequirementsInfo {
        self.0
    }
}
impl<'a> std::default::Default for ImagePlaneMemoryRequirementsInfoBuilder<'a> {
    fn default() -> ImagePlaneMemoryRequirementsInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ImagePlaneMemoryRequirementsInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ImagePlaneMemoryRequirementsInfoBuilder<'a> {
    type Target = ImagePlaneMemoryRequirementsInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ImagePlaneMemoryRequirementsInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<ImagePlaneMemoryRequirementsInfo> for ImagePlaneMemoryRequirementsInfoBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSamplerYcbcrConversionFeatures.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceSamplerYcbcrConversionFeatures")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceSamplerYcbcrConversionFeatures {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub sampler_ycbcr_conversion: crate::vk1_0::Bool32,
}
impl Default for PhysicalDeviceSamplerYcbcrConversionFeatures {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES,
            p_next: std::ptr::null_mut(),
            sampler_ycbcr_conversion: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceSamplerYcbcrConversionFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceSamplerYcbcrConversionFeatures")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("sampler_ycbcr_conversion", &(self.sampler_ycbcr_conversion != 0))
            .finish()
    }
}
impl PhysicalDeviceSamplerYcbcrConversionFeatures {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceSamplerYcbcrConversionFeaturesBuilder<'a> {
        PhysicalDeviceSamplerYcbcrConversionFeaturesBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSamplerYcbcrConversionFeatures.html) · Builder of [`PhysicalDeviceSamplerYcbcrConversionFeatures`]"]
#[repr(transparent)]
pub struct PhysicalDeviceSamplerYcbcrConversionFeaturesBuilder<'a>(PhysicalDeviceSamplerYcbcrConversionFeatures, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceSamplerYcbcrConversionFeaturesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceSamplerYcbcrConversionFeaturesBuilder<'a> {
        PhysicalDeviceSamplerYcbcrConversionFeaturesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn sampler_ycbcr_conversion(mut self, sampler_ycbcr_conversion: bool) -> Self {
        self.0.sampler_ycbcr_conversion = sampler_ycbcr_conversion as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceSamplerYcbcrConversionFeatures {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceSamplerYcbcrConversionFeaturesBuilder<'a> {
    fn default() -> PhysicalDeviceSamplerYcbcrConversionFeaturesBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceSamplerYcbcrConversionFeaturesBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceSamplerYcbcrConversionFeaturesBuilder<'a> {
    type Target = PhysicalDeviceSamplerYcbcrConversionFeatures;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceSamplerYcbcrConversionFeaturesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<PhysicalDeviceSamplerYcbcrConversionFeatures> for PhysicalDeviceSamplerYcbcrConversionFeaturesBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerYcbcrConversionImageFormatProperties.html) · Structure"]
#[doc(alias = "VkSamplerYcbcrConversionImageFormatProperties")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SamplerYcbcrConversionImageFormatProperties {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub combined_image_sampler_descriptor_count: u32,
}
impl Default for SamplerYcbcrConversionImageFormatProperties {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES,
            p_next: std::ptr::null_mut(),
            combined_image_sampler_descriptor_count: Default::default(),
        }
    }
}
impl std::fmt::Debug for SamplerYcbcrConversionImageFormatProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SamplerYcbcrConversionImageFormatProperties")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("combined_image_sampler_descriptor_count", &self.combined_image_sampler_descriptor_count)
            .finish()
    }
}
impl SamplerYcbcrConversionImageFormatProperties {
    #[inline]
    pub fn into_builder<'a>(self) -> SamplerYcbcrConversionImageFormatPropertiesBuilder<'a> {
        SamplerYcbcrConversionImageFormatPropertiesBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerYcbcrConversionImageFormatProperties.html) · Builder of [`SamplerYcbcrConversionImageFormatProperties`]"]
#[repr(transparent)]
pub struct SamplerYcbcrConversionImageFormatPropertiesBuilder<'a>(SamplerYcbcrConversionImageFormatProperties, std::marker::PhantomData<&'a ()>);
impl<'a> SamplerYcbcrConversionImageFormatPropertiesBuilder<'a> {
    #[inline]
    pub fn new() -> SamplerYcbcrConversionImageFormatPropertiesBuilder<'a> {
        SamplerYcbcrConversionImageFormatPropertiesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn combined_image_sampler_descriptor_count(mut self, combined_image_sampler_descriptor_count: u32) -> Self {
        self.0.combined_image_sampler_descriptor_count = combined_image_sampler_descriptor_count as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> SamplerYcbcrConversionImageFormatProperties {
        self.0
    }
}
impl<'a> std::default::Default for SamplerYcbcrConversionImageFormatPropertiesBuilder<'a> {
    fn default() -> SamplerYcbcrConversionImageFormatPropertiesBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for SamplerYcbcrConversionImageFormatPropertiesBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for SamplerYcbcrConversionImageFormatPropertiesBuilder<'a> {
    type Target = SamplerYcbcrConversionImageFormatProperties;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SamplerYcbcrConversionImageFormatPropertiesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<SamplerYcbcrConversionImageFormatProperties> for SamplerYcbcrConversionImageFormatPropertiesBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkProtectedSubmitInfo.html) · Structure"]
#[doc(alias = "VkProtectedSubmitInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ProtectedSubmitInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub protected_submit: crate::vk1_0::Bool32,
}
impl Default for ProtectedSubmitInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PROTECTED_SUBMIT_INFO,
            p_next: std::ptr::null(),
            protected_submit: Default::default(),
        }
    }
}
impl std::fmt::Debug for ProtectedSubmitInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ProtectedSubmitInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("protected_submit", &(self.protected_submit != 0))
            .finish()
    }
}
impl ProtectedSubmitInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> ProtectedSubmitInfoBuilder<'a> {
        ProtectedSubmitInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkProtectedSubmitInfo.html) · Builder of [`ProtectedSubmitInfo`]"]
#[repr(transparent)]
pub struct ProtectedSubmitInfoBuilder<'a>(ProtectedSubmitInfo, std::marker::PhantomData<&'a ()>);
impl<'a> ProtectedSubmitInfoBuilder<'a> {
    #[inline]
    pub fn new() -> ProtectedSubmitInfoBuilder<'a> {
        ProtectedSubmitInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn protected_submit(mut self, protected_submit: bool) -> Self {
        self.0.protected_submit = protected_submit as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ProtectedSubmitInfo {
        self.0
    }
}
impl<'a> std::default::Default for ProtectedSubmitInfoBuilder<'a> {
    fn default() -> ProtectedSubmitInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ProtectedSubmitInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ProtectedSubmitInfoBuilder<'a> {
    type Target = ProtectedSubmitInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ProtectedSubmitInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<ProtectedSubmitInfo> for ProtectedSubmitInfoBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceProtectedMemoryFeatures.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceProtectedMemoryFeatures")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceProtectedMemoryFeatures {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub protected_memory: crate::vk1_0::Bool32,
}
impl Default for PhysicalDeviceProtectedMemoryFeatures {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES,
            p_next: std::ptr::null_mut(),
            protected_memory: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceProtectedMemoryFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceProtectedMemoryFeatures")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("protected_memory", &(self.protected_memory != 0))
            .finish()
    }
}
impl PhysicalDeviceProtectedMemoryFeatures {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceProtectedMemoryFeaturesBuilder<'a> {
        PhysicalDeviceProtectedMemoryFeaturesBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceProtectedMemoryFeatures.html) · Builder of [`PhysicalDeviceProtectedMemoryFeatures`]"]
#[repr(transparent)]
pub struct PhysicalDeviceProtectedMemoryFeaturesBuilder<'a>(PhysicalDeviceProtectedMemoryFeatures, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceProtectedMemoryFeaturesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceProtectedMemoryFeaturesBuilder<'a> {
        PhysicalDeviceProtectedMemoryFeaturesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn protected_memory(mut self, protected_memory: bool) -> Self {
        self.0.protected_memory = protected_memory as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceProtectedMemoryFeatures {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceProtectedMemoryFeaturesBuilder<'a> {
    fn default() -> PhysicalDeviceProtectedMemoryFeaturesBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceProtectedMemoryFeaturesBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceProtectedMemoryFeaturesBuilder<'a> {
    type Target = PhysicalDeviceProtectedMemoryFeatures;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceProtectedMemoryFeaturesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<PhysicalDeviceProtectedMemoryFeatures> for PhysicalDeviceProtectedMemoryFeaturesBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceProtectedMemoryProperties.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceProtectedMemoryProperties")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceProtectedMemoryProperties {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub protected_no_fault: crate::vk1_0::Bool32,
}
impl Default for PhysicalDeviceProtectedMemoryProperties {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES,
            p_next: std::ptr::null_mut(),
            protected_no_fault: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceProtectedMemoryProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceProtectedMemoryProperties")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("protected_no_fault", &(self.protected_no_fault != 0))
            .finish()
    }
}
impl PhysicalDeviceProtectedMemoryProperties {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceProtectedMemoryPropertiesBuilder<'a> {
        PhysicalDeviceProtectedMemoryPropertiesBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceProtectedMemoryProperties.html) · Builder of [`PhysicalDeviceProtectedMemoryProperties`]"]
#[repr(transparent)]
pub struct PhysicalDeviceProtectedMemoryPropertiesBuilder<'a>(PhysicalDeviceProtectedMemoryProperties, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceProtectedMemoryPropertiesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceProtectedMemoryPropertiesBuilder<'a> {
        PhysicalDeviceProtectedMemoryPropertiesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn protected_no_fault(mut self, protected_no_fault: bool) -> Self {
        self.0.protected_no_fault = protected_no_fault as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceProtectedMemoryProperties {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceProtectedMemoryPropertiesBuilder<'a> {
    fn default() -> PhysicalDeviceProtectedMemoryPropertiesBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceProtectedMemoryPropertiesBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceProtectedMemoryPropertiesBuilder<'a> {
    type Target = PhysicalDeviceProtectedMemoryProperties;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceProtectedMemoryPropertiesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<PhysicalDeviceProtectedMemoryProperties> for PhysicalDeviceProtectedMemoryPropertiesBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceQueueInfo2.html) · Structure"]
#[doc(alias = "VkDeviceQueueInfo2")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DeviceQueueInfo2 {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::vk1_0::DeviceQueueCreateFlags,
    pub queue_family_index: u32,
    pub queue_index: u32,
}
impl Default for DeviceQueueInfo2 {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::DEVICE_QUEUE_INFO_2,
            p_next: std::ptr::null(),
            flags: Default::default(),
            queue_family_index: Default::default(),
            queue_index: Default::default(),
        }
    }
}
impl std::fmt::Debug for DeviceQueueInfo2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DeviceQueueInfo2")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("queue_family_index", &self.queue_family_index)
            .field("queue_index", &self.queue_index)
            .finish()
    }
}
impl DeviceQueueInfo2 {
    #[inline]
    pub fn into_builder<'a>(self) -> DeviceQueueInfo2Builder<'a> {
        DeviceQueueInfo2Builder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceQueueInfo2.html) · Builder of [`DeviceQueueInfo2`]"]
#[repr(transparent)]
pub struct DeviceQueueInfo2Builder<'a>(DeviceQueueInfo2, std::marker::PhantomData<&'a ()>);
impl<'a> DeviceQueueInfo2Builder<'a> {
    #[inline]
    pub fn new() -> DeviceQueueInfo2Builder<'a> {
        DeviceQueueInfo2Builder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_0::DeviceQueueCreateFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn queue_family_index(mut self, queue_family_index: u32) -> Self {
        self.0.queue_family_index = queue_family_index as _;
        self
    }
    #[inline]
    pub fn queue_index(mut self, queue_index: u32) -> Self {
        self.0.queue_index = queue_index as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DeviceQueueInfo2 {
        self.0
    }
}
impl<'a> std::default::Default for DeviceQueueInfo2Builder<'a> {
    fn default() -> DeviceQueueInfo2Builder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DeviceQueueInfo2Builder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DeviceQueueInfo2Builder<'a> {
    type Target = DeviceQueueInfo2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DeviceQueueInfo2Builder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<DeviceQueueInfo2> for DeviceQueueInfo2Builder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMaintenance3Properties.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceMaintenance3Properties")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceMaintenance3Properties {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub max_per_set_descriptors: u32,
    pub max_memory_allocation_size: crate::vk1_0::DeviceSize,
}
impl Default for PhysicalDeviceMaintenance3Properties {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES,
            p_next: std::ptr::null_mut(),
            max_per_set_descriptors: Default::default(),
            max_memory_allocation_size: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceMaintenance3Properties {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceMaintenance3Properties")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("max_per_set_descriptors", &self.max_per_set_descriptors)
            .field("max_memory_allocation_size", &self.max_memory_allocation_size)
            .finish()
    }
}
impl PhysicalDeviceMaintenance3Properties {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceMaintenance3PropertiesBuilder<'a> {
        PhysicalDeviceMaintenance3PropertiesBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMaintenance3Properties.html) · Builder of [`PhysicalDeviceMaintenance3Properties`]"]
#[repr(transparent)]
pub struct PhysicalDeviceMaintenance3PropertiesBuilder<'a>(PhysicalDeviceMaintenance3Properties, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceMaintenance3PropertiesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceMaintenance3PropertiesBuilder<'a> {
        PhysicalDeviceMaintenance3PropertiesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn max_per_set_descriptors(mut self, max_per_set_descriptors: u32) -> Self {
        self.0.max_per_set_descriptors = max_per_set_descriptors as _;
        self
    }
    #[inline]
    pub fn max_memory_allocation_size(mut self, max_memory_allocation_size: crate::vk1_0::DeviceSize) -> Self {
        self.0.max_memory_allocation_size = max_memory_allocation_size as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceMaintenance3Properties {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceMaintenance3PropertiesBuilder<'a> {
    fn default() -> PhysicalDeviceMaintenance3PropertiesBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceMaintenance3PropertiesBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceMaintenance3PropertiesBuilder<'a> {
    type Target = PhysicalDeviceMaintenance3Properties;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceMaintenance3PropertiesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<PhysicalDeviceMaintenance3Properties> for PhysicalDeviceMaintenance3PropertiesBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorSetLayoutSupport.html) · Structure"]
#[doc(alias = "VkDescriptorSetLayoutSupport")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DescriptorSetLayoutSupport {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub supported: crate::vk1_0::Bool32,
}
impl Default for DescriptorSetLayoutSupport {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::DESCRIPTOR_SET_LAYOUT_SUPPORT,
            p_next: std::ptr::null_mut(),
            supported: Default::default(),
        }
    }
}
impl std::fmt::Debug for DescriptorSetLayoutSupport {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DescriptorSetLayoutSupport")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("supported", &(self.supported != 0))
            .finish()
    }
}
impl DescriptorSetLayoutSupport {
    #[inline]
    pub fn into_builder<'a>(self) -> DescriptorSetLayoutSupportBuilder<'a> {
        DescriptorSetLayoutSupportBuilder(self, std::marker::PhantomData)
    }
}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::DescriptorSetVariableDescriptorCountLayoutSupport> for DescriptorSetLayoutSupportBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::DescriptorSetVariableDescriptorCountLayoutSupportBuilder<'_>> for DescriptorSetLayoutSupportBuilder<'a> {}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorSetLayoutSupport.html) · Builder of [`DescriptorSetLayoutSupport`]"]
#[repr(transparent)]
pub struct DescriptorSetLayoutSupportBuilder<'a>(DescriptorSetLayoutSupport, std::marker::PhantomData<&'a ()>);
impl<'a> DescriptorSetLayoutSupportBuilder<'a> {
    #[inline]
    pub fn new() -> DescriptorSetLayoutSupportBuilder<'a> {
        DescriptorSetLayoutSupportBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn supported(mut self, supported: bool) -> Self {
        self.0.supported = supported as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DescriptorSetLayoutSupport {
        self.0
    }
}
impl<'a> std::default::Default for DescriptorSetLayoutSupportBuilder<'a> {
    fn default() -> DescriptorSetLayoutSupportBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DescriptorSetLayoutSupportBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DescriptorSetLayoutSupportBuilder<'a> {
    type Target = DescriptorSetLayoutSupport;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DescriptorSetLayoutSupportBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<DescriptorSetLayoutSupport> for DescriptorSetLayoutSupportBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderDrawParametersFeatures.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceShaderDrawParametersFeatures")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceShaderDrawParametersFeatures {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub shader_draw_parameters: crate::vk1_0::Bool32,
}
impl Default for PhysicalDeviceShaderDrawParametersFeatures {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES,
            p_next: std::ptr::null_mut(),
            shader_draw_parameters: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceShaderDrawParametersFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceShaderDrawParametersFeatures")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("shader_draw_parameters", &(self.shader_draw_parameters != 0))
            .finish()
    }
}
impl PhysicalDeviceShaderDrawParametersFeatures {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceShaderDrawParametersFeaturesBuilder<'a> {
        PhysicalDeviceShaderDrawParametersFeaturesBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderDrawParametersFeatures.html) · Builder of [`PhysicalDeviceShaderDrawParametersFeatures`]"]
#[repr(transparent)]
pub struct PhysicalDeviceShaderDrawParametersFeaturesBuilder<'a>(PhysicalDeviceShaderDrawParametersFeatures, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceShaderDrawParametersFeaturesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceShaderDrawParametersFeaturesBuilder<'a> {
        PhysicalDeviceShaderDrawParametersFeaturesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn shader_draw_parameters(mut self, shader_draw_parameters: bool) -> Self {
        self.0.shader_draw_parameters = shader_draw_parameters as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceShaderDrawParametersFeatures {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceShaderDrawParametersFeaturesBuilder<'a> {
    fn default() -> PhysicalDeviceShaderDrawParametersFeaturesBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceShaderDrawParametersFeaturesBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceShaderDrawParametersFeaturesBuilder<'a> {
    type Target = PhysicalDeviceShaderDrawParametersFeatures;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceShaderDrawParametersFeaturesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<PhysicalDeviceShaderDrawParametersFeatures> for PhysicalDeviceShaderDrawParametersFeaturesBuilder<'_> {}
#[doc = "Provided by [`crate::vk1_1`]"]
impl<T> crate::EntryLoader<T> {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumerateInstanceVersion.html) · Function"]
    #[doc(alias = "vkEnumerateInstanceVersion")]
    pub unsafe fn enumerate_instance_version(&self) -> crate::utils::VulkanResult<u32> {
        let _function = self.enumerate_instance_version.expect("`enumerate_instance_version` is not loaded");
        let mut api_version = Default::default();
        let _return = _function(&mut api_version);
        crate::utils::VulkanResult::new(_return, api_version)
    }
}
#[doc = "Provided by [`crate::vk1_1`]"]
impl crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceFeatures2.html) · Function"]
    #[doc(alias = "vkGetPhysicalDeviceFeatures2")]
    pub unsafe fn get_physical_device_features2(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        features: Option<crate::vk1_1::PhysicalDeviceFeatures2>,
    ) -> crate::vk1_1::PhysicalDeviceFeatures2 {
        let _function = self.get_physical_device_features2.expect("`get_physical_device_features2` is not loaded");
        let mut features = match features {
            Some(v) => v,
            None => Default::default(),
        };
        let _return = _function(physical_device as _, &mut features);
        features
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceProperties2.html) · Function"]
    #[doc(alias = "vkGetPhysicalDeviceProperties2")]
    pub unsafe fn get_physical_device_properties2(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        properties: Option<crate::vk1_1::PhysicalDeviceProperties2>,
    ) -> crate::vk1_1::PhysicalDeviceProperties2 {
        let _function = self.get_physical_device_properties2.expect("`get_physical_device_properties2` is not loaded");
        let mut properties = match properties {
            Some(v) => v,
            None => Default::default(),
        };
        let _return = _function(physical_device as _, &mut properties);
        properties
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceFormatProperties2.html) · Function"]
    #[doc(alias = "vkGetPhysicalDeviceFormatProperties2")]
    pub unsafe fn get_physical_device_format_properties2(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        format: crate::vk1_0::Format,
        format_properties: Option<crate::vk1_1::FormatProperties2>,
    ) -> crate::vk1_1::FormatProperties2 {
        let _function = self.get_physical_device_format_properties2.expect("`get_physical_device_format_properties2` is not loaded");
        let mut format_properties = match format_properties {
            Some(v) => v,
            None => Default::default(),
        };
        let _return = _function(physical_device as _, format as _, &mut format_properties);
        format_properties
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceImageFormatProperties2.html) · Function"]
    #[doc(alias = "vkGetPhysicalDeviceImageFormatProperties2")]
    pub unsafe fn get_physical_device_image_format_properties2(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        image_format_info: &crate::vk1_1::PhysicalDeviceImageFormatInfo2,
        image_format_properties: Option<crate::vk1_1::ImageFormatProperties2>,
    ) -> crate::utils::VulkanResult<crate::vk1_1::ImageFormatProperties2> {
        let _function = self.get_physical_device_image_format_properties2.expect("`get_physical_device_image_format_properties2` is not loaded");
        let mut image_format_properties = match image_format_properties {
            Some(v) => v,
            None => Default::default(),
        };
        let _return = _function(physical_device as _, image_format_info as _, &mut image_format_properties);
        crate::utils::VulkanResult::new(_return, image_format_properties)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties2.html) · Function"]
    #[doc(alias = "vkGetPhysicalDeviceQueueFamilyProperties2")]
    pub unsafe fn get_physical_device_queue_family_properties2(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        queue_family_property_count: Option<u32>,
    ) -> Vec<crate::vk1_1::QueueFamilyProperties2> {
        let _function = self.get_physical_device_queue_family_properties2.expect("`get_physical_device_queue_family_properties2` is not loaded");
        let mut queue_family_property_count = match queue_family_property_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                _function(physical_device as _, &mut v, std::ptr::null_mut());
                v
            }
        };
        let mut queue_family_properties = vec![Default::default(); queue_family_property_count as _];
        let _return = _function(physical_device as _, &mut queue_family_property_count, queue_family_properties.as_mut_ptr());
        queue_family_properties
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceMemoryProperties2.html) · Function"]
    #[doc(alias = "vkGetPhysicalDeviceMemoryProperties2")]
    pub unsafe fn get_physical_device_memory_properties2(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        memory_properties: Option<crate::vk1_1::PhysicalDeviceMemoryProperties2>,
    ) -> crate::vk1_1::PhysicalDeviceMemoryProperties2 {
        let _function = self.get_physical_device_memory_properties2.expect("`get_physical_device_memory_properties2` is not loaded");
        let mut memory_properties = match memory_properties {
            Some(v) => v,
            None => Default::default(),
        };
        let _return = _function(physical_device as _, &mut memory_properties);
        memory_properties
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSparseImageFormatProperties2.html) · Function"]
    #[doc(alias = "vkGetPhysicalDeviceSparseImageFormatProperties2")]
    pub unsafe fn get_physical_device_sparse_image_format_properties2(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        format_info: &crate::vk1_1::PhysicalDeviceSparseImageFormatInfo2,
        property_count: Option<u32>,
    ) -> Vec<crate::vk1_1::SparseImageFormatProperties2> {
        let _function = self
            .get_physical_device_sparse_image_format_properties2
            .expect("`get_physical_device_sparse_image_format_properties2` is not loaded");
        let mut property_count = match property_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                _function(physical_device as _, format_info as _, &mut v, std::ptr::null_mut());
                v
            }
        };
        let mut properties = vec![Default::default(); property_count as _];
        let _return = _function(physical_device as _, format_info as _, &mut property_count, properties.as_mut_ptr());
        properties
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceExternalBufferProperties.html) · Function"]
    #[doc(alias = "vkGetPhysicalDeviceExternalBufferProperties")]
    pub unsafe fn get_physical_device_external_buffer_properties(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        external_buffer_info: &crate::vk1_1::PhysicalDeviceExternalBufferInfo,
        external_buffer_properties: Option<crate::vk1_1::ExternalBufferProperties>,
    ) -> crate::vk1_1::ExternalBufferProperties {
        let _function = self
            .get_physical_device_external_buffer_properties
            .expect("`get_physical_device_external_buffer_properties` is not loaded");
        let mut external_buffer_properties = match external_buffer_properties {
            Some(v) => v,
            None => Default::default(),
        };
        let _return = _function(physical_device as _, external_buffer_info as _, &mut external_buffer_properties);
        external_buffer_properties
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceExternalSemaphoreProperties.html) · Function"]
    #[doc(alias = "vkGetPhysicalDeviceExternalSemaphoreProperties")]
    pub unsafe fn get_physical_device_external_semaphore_properties(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        external_semaphore_info: &crate::vk1_1::PhysicalDeviceExternalSemaphoreInfo,
        external_semaphore_properties: Option<crate::vk1_1::ExternalSemaphoreProperties>,
    ) -> crate::vk1_1::ExternalSemaphoreProperties {
        let _function = self
            .get_physical_device_external_semaphore_properties
            .expect("`get_physical_device_external_semaphore_properties` is not loaded");
        let mut external_semaphore_properties = match external_semaphore_properties {
            Some(v) => v,
            None => Default::default(),
        };
        let _return = _function(physical_device as _, external_semaphore_info as _, &mut external_semaphore_properties);
        external_semaphore_properties
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceExternalFenceProperties.html) · Function"]
    #[doc(alias = "vkGetPhysicalDeviceExternalFenceProperties")]
    pub unsafe fn get_physical_device_external_fence_properties(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        external_fence_info: &crate::vk1_1::PhysicalDeviceExternalFenceInfo,
        external_fence_properties: Option<crate::vk1_1::ExternalFenceProperties>,
    ) -> crate::vk1_1::ExternalFenceProperties {
        let _function = self
            .get_physical_device_external_fence_properties
            .expect("`get_physical_device_external_fence_properties` is not loaded");
        let mut external_fence_properties = match external_fence_properties {
            Some(v) => v,
            None => Default::default(),
        };
        let _return = _function(physical_device as _, external_fence_info as _, &mut external_fence_properties);
        external_fence_properties
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumeratePhysicalDeviceGroups.html) · Function"]
    #[doc(alias = "vkEnumeratePhysicalDeviceGroups")]
    pub unsafe fn enumerate_physical_device_groups(&self, physical_device_group_count: Option<u32>) -> crate::utils::VulkanResult<Vec<crate::vk1_1::PhysicalDeviceGroupProperties>> {
        let _function = self.enumerate_physical_device_groups.expect("`enumerate_physical_device_groups` is not loaded");
        let mut physical_device_group_count = match physical_device_group_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                _function(self.handle, &mut v, std::ptr::null_mut());
                v
            }
        };
        let mut physical_device_group_properties = vec![Default::default(); physical_device_group_count as _];
        let _return = _function(self.handle, &mut physical_device_group_count, physical_device_group_properties.as_mut_ptr());
        crate::utils::VulkanResult::new(_return, physical_device_group_properties)
    }
}
#[doc = "Provided by [`crate::vk1_1`]"]
impl crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkTrimCommandPool.html) · Function"]
    #[doc(alias = "vkTrimCommandPool")]
    pub unsafe fn trim_command_pool(&self, command_pool: crate::vk1_0::CommandPool, flags: Option<crate::vk1_1::CommandPoolTrimFlags>) -> () {
        let _function = self.trim_command_pool.expect("`trim_command_pool` is not loaded");
        let _return = _function(
            self.handle,
            command_pool as _,
            match flags {
                Some(v) => v,
                None => Default::default(),
            },
        );
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceGroupPeerMemoryFeatures.html) · Function"]
    #[doc(alias = "vkGetDeviceGroupPeerMemoryFeatures")]
    pub unsafe fn get_device_group_peer_memory_features(&self, heap_index: u32, local_device_index: u32, remote_device_index: u32) -> crate::vk1_1::PeerMemoryFeatureFlags {
        let _function = self.get_device_group_peer_memory_features.expect("`get_device_group_peer_memory_features` is not loaded");
        let mut peer_memory_features = Default::default();
        let _return = _function(self.handle, heap_index as _, local_device_index as _, remote_device_index as _, &mut peer_memory_features);
        peer_memory_features
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBindBufferMemory2.html) · Function"]
    #[doc(alias = "vkBindBufferMemory2")]
    pub unsafe fn bind_buffer_memory2(&self, bind_infos: &[impl crate::Repr<crate::vk1_1::BindBufferMemoryInfo>]) -> crate::utils::VulkanResult<()> {
        let _function = self.bind_buffer_memory2.expect("`bind_buffer_memory2` is not loaded");
        let bind_info_count = bind_infos.len();
        let _return = _function(self.handle, bind_info_count as _, bind_infos.as_ptr() as _);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBindImageMemory2.html) · Function"]
    #[doc(alias = "vkBindImageMemory2")]
    pub unsafe fn bind_image_memory2(&self, bind_infos: &[impl crate::Repr<crate::vk1_1::BindImageMemoryInfo>]) -> crate::utils::VulkanResult<()> {
        let _function = self.bind_image_memory2.expect("`bind_image_memory2` is not loaded");
        let bind_info_count = bind_infos.len();
        let _return = _function(self.handle, bind_info_count as _, bind_infos.as_ptr() as _);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDeviceMask.html) · Function"]
    #[doc(alias = "vkCmdSetDeviceMask")]
    pub unsafe fn cmd_set_device_mask(&self, command_buffer: crate::vk1_0::CommandBuffer, device_mask: u32) -> () {
        let _function = self.cmd_set_device_mask.expect("`cmd_set_device_mask` is not loaded");
        let _return = _function(command_buffer as _, device_mask as _);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDispatchBase.html) · Function"]
    #[doc(alias = "vkCmdDispatchBase")]
    pub unsafe fn cmd_dispatch_base(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        base_group_x: u32,
        base_group_y: u32,
        base_group_z: u32,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) -> () {
        let _function = self.cmd_dispatch_base.expect("`cmd_dispatch_base` is not loaded");
        let _return = _function(
            command_buffer as _,
            base_group_x as _,
            base_group_y as _,
            base_group_z as _,
            group_count_x as _,
            group_count_y as _,
            group_count_z as _,
        );
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDescriptorUpdateTemplate.html) · Function"]
    #[doc(alias = "vkCreateDescriptorUpdateTemplate")]
    pub unsafe fn create_descriptor_update_template(
        &self,
        create_info: &crate::vk1_1::DescriptorUpdateTemplateCreateInfo,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
    ) -> crate::utils::VulkanResult<crate::vk1_1::DescriptorUpdateTemplate> {
        let _function = self.create_descriptor_update_template.expect("`create_descriptor_update_template` is not loaded");
        let mut descriptor_update_template = Default::default();
        let _return = _function(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut descriptor_update_template,
        );
        crate::utils::VulkanResult::new(_return, descriptor_update_template)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyDescriptorUpdateTemplate.html) · Function"]
    #[doc(alias = "vkDestroyDescriptorUpdateTemplate")]
    pub unsafe fn destroy_descriptor_update_template(&self, descriptor_update_template: Option<crate::vk1_1::DescriptorUpdateTemplate>, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> () {
        let _function = self.destroy_descriptor_update_template.expect("`destroy_descriptor_update_template` is not loaded");
        let _return = _function(
            self.handle,
            match descriptor_update_template {
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
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkUpdateDescriptorSetWithTemplate.html) · Function"]
    #[doc(alias = "vkUpdateDescriptorSetWithTemplate")]
    pub unsafe fn update_descriptor_set_with_template(
        &self,
        descriptor_set: crate::vk1_0::DescriptorSet,
        descriptor_update_template: crate::vk1_1::DescriptorUpdateTemplate,
        data: *const std::ffi::c_void,
    ) -> () {
        let _function = self.update_descriptor_set_with_template.expect("`update_descriptor_set_with_template` is not loaded");
        let _return = _function(self.handle, descriptor_set as _, descriptor_update_template as _, data);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetBufferMemoryRequirements2.html) · Function"]
    #[doc(alias = "vkGetBufferMemoryRequirements2")]
    pub unsafe fn get_buffer_memory_requirements2(
        &self,
        info: &crate::vk1_1::BufferMemoryRequirementsInfo2,
        memory_requirements: Option<crate::vk1_1::MemoryRequirements2>,
    ) -> crate::vk1_1::MemoryRequirements2 {
        let _function = self.get_buffer_memory_requirements2.expect("`get_buffer_memory_requirements2` is not loaded");
        let mut memory_requirements = match memory_requirements {
            Some(v) => v,
            None => Default::default(),
        };
        let _return = _function(self.handle, info as _, &mut memory_requirements);
        memory_requirements
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageMemoryRequirements2.html) · Function"]
    #[doc(alias = "vkGetImageMemoryRequirements2")]
    pub unsafe fn get_image_memory_requirements2(
        &self,
        info: &crate::vk1_1::ImageMemoryRequirementsInfo2,
        memory_requirements: Option<crate::vk1_1::MemoryRequirements2>,
    ) -> crate::vk1_1::MemoryRequirements2 {
        let _function = self.get_image_memory_requirements2.expect("`get_image_memory_requirements2` is not loaded");
        let mut memory_requirements = match memory_requirements {
            Some(v) => v,
            None => Default::default(),
        };
        let _return = _function(self.handle, info as _, &mut memory_requirements);
        memory_requirements
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageSparseMemoryRequirements2.html) · Function"]
    #[doc(alias = "vkGetImageSparseMemoryRequirements2")]
    pub unsafe fn get_image_sparse_memory_requirements2(
        &self,
        info: &crate::vk1_1::ImageSparseMemoryRequirementsInfo2,
        sparse_memory_requirement_count: Option<u32>,
    ) -> Vec<crate::vk1_1::SparseImageMemoryRequirements2> {
        let _function = self.get_image_sparse_memory_requirements2.expect("`get_image_sparse_memory_requirements2` is not loaded");
        let mut sparse_memory_requirement_count = match sparse_memory_requirement_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                _function(self.handle, info as _, &mut v, std::ptr::null_mut());
                v
            }
        };
        let mut sparse_memory_requirements = vec![Default::default(); sparse_memory_requirement_count as _];
        let _return = _function(self.handle, info as _, &mut sparse_memory_requirement_count, sparse_memory_requirements.as_mut_ptr());
        sparse_memory_requirements
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateSamplerYcbcrConversion.html) · Function"]
    #[doc(alias = "vkCreateSamplerYcbcrConversion")]
    pub unsafe fn create_sampler_ycbcr_conversion(
        &self,
        create_info: &crate::vk1_1::SamplerYcbcrConversionCreateInfo,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
    ) -> crate::utils::VulkanResult<crate::vk1_1::SamplerYcbcrConversion> {
        let _function = self.create_sampler_ycbcr_conversion.expect("`create_sampler_ycbcr_conversion` is not loaded");
        let mut ycbcr_conversion = Default::default();
        let _return = _function(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut ycbcr_conversion,
        );
        crate::utils::VulkanResult::new(_return, ycbcr_conversion)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroySamplerYcbcrConversion.html) · Function"]
    #[doc(alias = "vkDestroySamplerYcbcrConversion")]
    pub unsafe fn destroy_sampler_ycbcr_conversion(&self, ycbcr_conversion: Option<crate::vk1_1::SamplerYcbcrConversion>, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> () {
        let _function = self.destroy_sampler_ycbcr_conversion.expect("`destroy_sampler_ycbcr_conversion` is not loaded");
        let _return = _function(
            self.handle,
            match ycbcr_conversion {
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
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceQueue2.html) · Function"]
    #[doc(alias = "vkGetDeviceQueue2")]
    pub unsafe fn get_device_queue2(&self, queue_info: &crate::vk1_1::DeviceQueueInfo2) -> crate::vk1_0::Queue {
        let _function = self.get_device_queue2.expect("`get_device_queue2` is not loaded");
        let mut queue = Default::default();
        let _return = _function(self.handle, queue_info as _, &mut queue);
        queue
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDescriptorSetLayoutSupport.html) · Function"]
    #[doc(alias = "vkGetDescriptorSetLayoutSupport")]
    pub unsafe fn get_descriptor_set_layout_support(
        &self,
        create_info: &crate::vk1_0::DescriptorSetLayoutCreateInfo,
        support: Option<crate::vk1_1::DescriptorSetLayoutSupport>,
    ) -> crate::vk1_1::DescriptorSetLayoutSupport {
        let _function = self.get_descriptor_set_layout_support.expect("`get_descriptor_set_layout_support` is not loaded");
        let mut support = match support {
            Some(v) => v,
            None => Default::default(),
        };
        let _return = _function(self.handle, create_info as _, &mut support);
        support
    }
}
