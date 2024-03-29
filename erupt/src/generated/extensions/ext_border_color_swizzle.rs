// DO NOT EDIT: @generated by erupt's generator
///<s>Vulkan Manual Page</s> · Constant
#[doc(alias = "VK_EXT_BORDER_COLOR_SWIZZLE_SPEC_VERSION")]
pub const EXT_BORDER_COLOR_SWIZZLE_SPEC_VERSION: u32 = 1;
///<s>Vulkan Manual Page</s> · Constant
#[doc(alias = "VK_EXT_BORDER_COLOR_SWIZZLE_EXTENSION_NAME")]
pub const EXT_BORDER_COLOR_SWIZZLE_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!(
    "VK_EXT_border_color_swizzle"
);
///Provided by [`crate::extensions::ext_border_color_swizzle`]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_BORDER_COLOR_SWIZZLE_FEATURES_EXT: Self = Self(1000411000);
    pub const SAMPLER_BORDER_COLOR_COMPONENT_MAPPING_CREATE_INFO_EXT: Self = Self(
        1000411001,
    );
}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceBorderColorSwizzleFeaturesEXT>
for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<
    'a,
> crate::ExtendableFrom<'a, PhysicalDeviceBorderColorSwizzleFeaturesEXTBuilder<'_>>
for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, SamplerBorderColorComponentMappingCreateInfoEXT>
for crate::vk1_0::SamplerCreateInfoBuilder<'a> {}
impl<
    'a,
> crate::ExtendableFrom<'a, SamplerBorderColorComponentMappingCreateInfoEXTBuilder<'_>>
for crate::vk1_0::SamplerCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceBorderColorSwizzleFeaturesEXT>
for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<
    'a,
> crate::ExtendableFrom<'a, PhysicalDeviceBorderColorSwizzleFeaturesEXTBuilder<'_>>
for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerBorderColorComponentMappingCreateInfoEXT.html) · Structure
#[doc(alias = "VkSamplerBorderColorComponentMappingCreateInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SamplerBorderColorComponentMappingCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub components: crate::vk1_0::ComponentMapping,
    pub srgb: crate::vk1_0::Bool32,
}
impl SamplerBorderColorComponentMappingCreateInfoEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::SAMPLER_BORDER_COLOR_COMPONENT_MAPPING_CREATE_INFO_EXT;
}
impl Default for SamplerBorderColorComponentMappingCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: Self::STRUCTURE_TYPE,
            p_next: std::ptr::null(),
            components: Default::default(),
            srgb: Default::default(),
        }
    }
}
impl std::fmt::Debug for SamplerBorderColorComponentMappingCreateInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SamplerBorderColorComponentMappingCreateInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("components", &self.components)
            .field("srgb", &(self.srgb != 0))
            .finish()
    }
}
impl SamplerBorderColorComponentMappingCreateInfoEXT {
    #[inline]
    pub fn into_builder<'a>(
        self,
    ) -> SamplerBorderColorComponentMappingCreateInfoEXTBuilder<'a> {
        SamplerBorderColorComponentMappingCreateInfoEXTBuilder(
            self,
            std::marker::PhantomData,
        )
    }
}
#[derive(Copy, Clone)]
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerBorderColorComponentMappingCreateInfoEXT.html) · Builder of [`SamplerBorderColorComponentMappingCreateInfoEXT`]
#[repr(transparent)]
pub struct SamplerBorderColorComponentMappingCreateInfoEXTBuilder<'a>(
    SamplerBorderColorComponentMappingCreateInfoEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> SamplerBorderColorComponentMappingCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> SamplerBorderColorComponentMappingCreateInfoEXTBuilder<'a> {
        SamplerBorderColorComponentMappingCreateInfoEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[inline]
    #[must_use]
    pub fn components(mut self, components: crate::vk1_0::ComponentMapping) -> Self {
        self.0.components = components as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn srgb(mut self, srgb: bool) -> Self {
        self.0.srgb = srgb as _;
        self
    }
    #[inline]
    /// Discards all lifetime information.
    /// Use the `Deref` and `DerefMut` implementations if possible.
    pub fn build_dangling(self) -> SamplerBorderColorComponentMappingCreateInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default
for SamplerBorderColorComponentMappingCreateInfoEXTBuilder<'a> {
    fn default() -> SamplerBorderColorComponentMappingCreateInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for SamplerBorderColorComponentMappingCreateInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for SamplerBorderColorComponentMappingCreateInfoEXTBuilder<'a> {
    type Target = SamplerBorderColorComponentMappingCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut
for SamplerBorderColorComponentMappingCreateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceBorderColorSwizzleFeaturesEXT.html) · Structure
#[doc(alias = "VkPhysicalDeviceBorderColorSwizzleFeaturesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceBorderColorSwizzleFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub border_color_swizzle: crate::vk1_0::Bool32,
    pub border_color_swizzle_from_image: crate::vk1_0::Bool32,
}
impl PhysicalDeviceBorderColorSwizzleFeaturesEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_BORDER_COLOR_SWIZZLE_FEATURES_EXT;
}
impl Default for PhysicalDeviceBorderColorSwizzleFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: Self::STRUCTURE_TYPE,
            p_next: std::ptr::null_mut(),
            border_color_swizzle: Default::default(),
            border_color_swizzle_from_image: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceBorderColorSwizzleFeaturesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceBorderColorSwizzleFeaturesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("border_color_swizzle", &(self.border_color_swizzle != 0))
            .field(
                "border_color_swizzle_from_image",
                &(self.border_color_swizzle_from_image != 0),
            )
            .finish()
    }
}
impl PhysicalDeviceBorderColorSwizzleFeaturesEXT {
    #[inline]
    pub fn into_builder<'a>(
        self,
    ) -> PhysicalDeviceBorderColorSwizzleFeaturesEXTBuilder<'a> {
        PhysicalDeviceBorderColorSwizzleFeaturesEXTBuilder(
            self,
            std::marker::PhantomData,
        )
    }
}
#[derive(Copy, Clone)]
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceBorderColorSwizzleFeaturesEXT.html) · Builder of [`PhysicalDeviceBorderColorSwizzleFeaturesEXT`]
#[repr(transparent)]
pub struct PhysicalDeviceBorderColorSwizzleFeaturesEXTBuilder<'a>(
    PhysicalDeviceBorderColorSwizzleFeaturesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceBorderColorSwizzleFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceBorderColorSwizzleFeaturesEXTBuilder<'a> {
        PhysicalDeviceBorderColorSwizzleFeaturesEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[inline]
    #[must_use]
    pub fn border_color_swizzle(mut self, border_color_swizzle: bool) -> Self {
        self.0.border_color_swizzle = border_color_swizzle as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn border_color_swizzle_from_image(
        mut self,
        border_color_swizzle_from_image: bool,
    ) -> Self {
        self.0.border_color_swizzle_from_image = border_color_swizzle_from_image as _;
        self
    }
    #[inline]
    /// Discards all lifetime information.
    /// Use the `Deref` and `DerefMut` implementations if possible.
    pub fn build_dangling(self) -> PhysicalDeviceBorderColorSwizzleFeaturesEXT {
        self.0
    }
}
impl<'a> std::default::Default
for PhysicalDeviceBorderColorSwizzleFeaturesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceBorderColorSwizzleFeaturesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceBorderColorSwizzleFeaturesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceBorderColorSwizzleFeaturesEXTBuilder<'a> {
    type Target = PhysicalDeviceBorderColorSwizzleFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceBorderColorSwizzleFeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
