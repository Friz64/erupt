#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION")]
pub const NV_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME")]
pub const NV_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_NV_external_memory_capabilities");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "FN_GET_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_PROPERTIES_NV")]
pub const FN_GET_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_PROPERTIES_NV: *const std::os::raw::c_char =
    crate::cstr!("vkGetPhysicalDeviceExternalImageFormatPropertiesNV");
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalMemoryHandleTypeFlagsNV.html) · Bitmask of [`ExternalMemoryHandleTypeFlagBitsNV`](./struct.ExternalMemoryHandleTypeFlagBitsNV.html)"] # [doc (alias = "VkExternalMemoryHandleTypeFlagsNV")] # [derive (Default)] # [repr (transparent)] pub struct ExternalMemoryHandleTypeFlagsNV : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; const OPAQUE_WIN32_NV = ExternalMemoryHandleTypeFlagBitsNV :: OPAQUE_WIN32_NV . 0 ; const OPAQUE_WIN32_KMT_NV = ExternalMemoryHandleTypeFlagBitsNV :: OPAQUE_WIN32_KMT_NV . 0 ; const D3D11_IMAGE_NV = ExternalMemoryHandleTypeFlagBitsNV :: D3D11_IMAGE_NV . 0 ; const D3D11_IMAGE_KMT_NV = ExternalMemoryHandleTypeFlagBitsNV :: D3D11_IMAGE_KMT_NV . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalMemoryHandleTypeFlagBitsNV.html) · Bits enum of [`ExternalMemoryHandleTypeFlagsNV`](./struct.ExternalMemoryHandleTypeFlagsNV.html)"]
#[doc(alias = "VkExternalMemoryHandleTypeFlagBitsNV")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct ExternalMemoryHandleTypeFlagBitsNV(pub u32);
impl ExternalMemoryHandleTypeFlagBitsNV {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> ExternalMemoryHandleTypeFlagsNV {
        ExternalMemoryHandleTypeFlagsNV::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for ExternalMemoryHandleTypeFlagBitsNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::OPAQUE_WIN32_NV => "OPAQUE_WIN32_NV",
            &Self::OPAQUE_WIN32_KMT_NV => "OPAQUE_WIN32_KMT_NV",
            &Self::D3D11_IMAGE_NV => "D3D11_IMAGE_NV",
            &Self::D3D11_IMAGE_KMT_NV => "D3D11_IMAGE_KMT_NV",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`extensions::nv_external_memory_capabilities`](./index.html)"]
impl ExternalMemoryHandleTypeFlagBitsNV {
    pub const OPAQUE_WIN32_NV: Self = Self(1);
    pub const OPAQUE_WIN32_KMT_NV: Self = Self(2);
    pub const D3D11_IMAGE_NV: Self = Self(4);
    pub const D3D11_IMAGE_KMT_NV: Self = Self(8);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalMemoryFeatureFlagsNV.html) · Bitmask of [`ExternalMemoryFeatureFlagBitsNV`](./struct.ExternalMemoryFeatureFlagBitsNV.html)"] # [doc (alias = "VkExternalMemoryFeatureFlagsNV")] # [derive (Default)] # [repr (transparent)] pub struct ExternalMemoryFeatureFlagsNV : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; const DEDICATED_ONLY_NV = ExternalMemoryFeatureFlagBitsNV :: DEDICATED_ONLY_NV . 0 ; const EXPORTABLE_NV = ExternalMemoryFeatureFlagBitsNV :: EXPORTABLE_NV . 0 ; const IMPORTABLE_NV = ExternalMemoryFeatureFlagBitsNV :: IMPORTABLE_NV . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalMemoryFeatureFlagBitsNV.html) · Bits enum of [`ExternalMemoryFeatureFlagsNV`](./struct.ExternalMemoryFeatureFlagsNV.html)"]
#[doc(alias = "VkExternalMemoryFeatureFlagBitsNV")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct ExternalMemoryFeatureFlagBitsNV(pub u32);
impl ExternalMemoryFeatureFlagBitsNV {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> ExternalMemoryFeatureFlagsNV {
        ExternalMemoryFeatureFlagsNV::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for ExternalMemoryFeatureFlagBitsNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::DEDICATED_ONLY_NV => "DEDICATED_ONLY_NV",
            &Self::EXPORTABLE_NV => "EXPORTABLE_NV",
            &Self::IMPORTABLE_NV => "IMPORTABLE_NV",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`extensions::nv_external_memory_capabilities`](./index.html)"]
impl ExternalMemoryFeatureFlagBitsNV {
    pub const DEDICATED_ONLY_NV: Self = Self(1);
    pub const EXPORTABLE_NV: Self = Self(2);
    pub const IMPORTABLE_NV: Self = Self(4);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceExternalImageFormatPropertiesNV.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceExternalImageFormatPropertiesNV = unsafe extern "system" fn (physical_device : crate :: vk1_0 :: PhysicalDevice , format : crate :: vk1_0 :: Format , _type : crate :: vk1_0 :: ImageType , tiling : crate :: vk1_0 :: ImageTiling , usage : crate :: vk1_0 :: ImageUsageFlags , flags : crate :: vk1_0 :: ImageCreateFlags , external_handle_type : crate :: extensions :: nv_external_memory_capabilities :: ExternalMemoryHandleTypeFlagsNV , p_external_image_format_properties : * mut crate :: extensions :: nv_external_memory_capabilities :: ExternalImageFormatPropertiesNV) -> crate :: vk1_0 :: Result ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalImageFormatPropertiesNV.html) · Structure"]
#[doc(alias = "VkExternalImageFormatPropertiesNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExternalImageFormatPropertiesNV {
    pub image_format_properties: crate::vk1_0::ImageFormatProperties,
    pub external_memory_features:
        crate::extensions::nv_external_memory_capabilities::ExternalMemoryFeatureFlagsNV,
    pub export_from_imported_handle_types:
        crate::extensions::nv_external_memory_capabilities::ExternalMemoryHandleTypeFlagsNV,
    pub compatible_handle_types:
        crate::extensions::nv_external_memory_capabilities::ExternalMemoryHandleTypeFlagsNV,
}
impl Default for ExternalImageFormatPropertiesNV {
    fn default() -> Self {
        Self {
            image_format_properties: Default::default(),
            external_memory_features: Default::default(),
            export_from_imported_handle_types: Default::default(),
            compatible_handle_types: Default::default(),
        }
    }
}
impl std::fmt::Debug for ExternalImageFormatPropertiesNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ExternalImageFormatPropertiesNV")
            .field("image_format_properties", &self.image_format_properties)
            .field("external_memory_features", &self.external_memory_features)
            .field(
                "export_from_imported_handle_types",
                &self.export_from_imported_handle_types,
            )
            .field("compatible_handle_types", &self.compatible_handle_types)
            .finish()
    }
}
impl ExternalImageFormatPropertiesNV {
    #[inline]
    pub fn into_builder<'a>(self) -> ExternalImageFormatPropertiesNVBuilder<'a> {
        ExternalImageFormatPropertiesNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalImageFormatPropertiesNV.html) · Builder of [`ExternalImageFormatPropertiesNV`](struct.ExternalImageFormatPropertiesNV.html)"]
#[repr(transparent)]
pub struct ExternalImageFormatPropertiesNVBuilder<'a>(
    ExternalImageFormatPropertiesNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> ExternalImageFormatPropertiesNVBuilder<'a> {
    #[inline]
    pub fn new() -> ExternalImageFormatPropertiesNVBuilder<'a> {
        ExternalImageFormatPropertiesNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn image_format_properties(
        mut self,
        image_format_properties: crate::vk1_0::ImageFormatProperties,
    ) -> Self {
        self.0.image_format_properties = image_format_properties as _;
        self
    }
    #[inline]
    pub fn external_memory_features(
        mut self,
        external_memory_features : crate :: extensions :: nv_external_memory_capabilities :: ExternalMemoryFeatureFlagsNV,
    ) -> Self {
        self.0.external_memory_features = external_memory_features as _;
        self
    }
    #[inline]
    pub fn export_from_imported_handle_types(
        mut self,
        export_from_imported_handle_types : crate :: extensions :: nv_external_memory_capabilities :: ExternalMemoryHandleTypeFlagsNV,
    ) -> Self {
        self.0.export_from_imported_handle_types = export_from_imported_handle_types as _;
        self
    }
    #[inline]
    pub fn compatible_handle_types(
        mut self,
        compatible_handle_types : crate :: extensions :: nv_external_memory_capabilities :: ExternalMemoryHandleTypeFlagsNV,
    ) -> Self {
        self.0.compatible_handle_types = compatible_handle_types as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ExternalImageFormatPropertiesNV {
        self.0
    }
}
impl<'a> std::default::Default for ExternalImageFormatPropertiesNVBuilder<'a> {
    fn default() -> ExternalImageFormatPropertiesNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ExternalImageFormatPropertiesNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ExternalImageFormatPropertiesNVBuilder<'a> {
    type Target = ExternalImageFormatPropertiesNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ExternalImageFormatPropertiesNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`extensions::nv_external_memory_capabilities`](extensions/nv_external_memory_capabilities/index.html)"]
impl crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceExternalImageFormatPropertiesNV.html) · Function"]
    #[doc(alias = "vkGetPhysicalDeviceExternalImageFormatPropertiesNV")]
    pub unsafe fn get_physical_device_external_image_format_properties_nv(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        format: crate::vk1_0::Format,
        _type: crate::vk1_0::ImageType,
        tiling: crate::vk1_0::ImageTiling,
        usage: crate::vk1_0::ImageUsageFlags,
        flags: Option<crate::vk1_0::ImageCreateFlags>,
        external_handle_type: Option<
            crate::extensions::nv_external_memory_capabilities::ExternalMemoryHandleTypeFlagsNV,
        >,
        external_image_format_properties: Option<
            crate::extensions::nv_external_memory_capabilities::ExternalImageFormatPropertiesNV,
        >,
    ) -> crate::utils::VulkanResult<
        crate::extensions::nv_external_memory_capabilities::ExternalImageFormatPropertiesNV,
    > {
        let _function = self
            .get_physical_device_external_image_format_properties_nv
            .expect("`get_physical_device_external_image_format_properties_nv` is not loaded");
        let mut external_image_format_properties = match external_image_format_properties {
            Some(v) => v,
            None => Default::default(),
        };
        let _return = _function(
            physical_device as _,
            format as _,
            _type as _,
            tiling as _,
            usage as _,
            match flags {
                Some(v) => v,
                None => Default::default(),
            },
            match external_handle_type {
                Some(v) => v,
                None => Default::default(),
            },
            &mut external_image_format_properties,
        );
        crate::utils::VulkanResult::new(_return, external_image_format_properties)
    }
}
