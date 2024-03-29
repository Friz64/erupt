// DO NOT EDIT: @generated by erupt's generator
///<s>Vulkan Manual Page</s> · Constant
#[doc(alias = "VK_EXT_FRAGMENT_DENSITY_MAP_SPEC_VERSION")]
pub const EXT_FRAGMENT_DENSITY_MAP_SPEC_VERSION: u32 = 2;
///<s>Vulkan Manual Page</s> · Constant
#[doc(alias = "VK_EXT_FRAGMENT_DENSITY_MAP_EXTENSION_NAME")]
pub const EXT_FRAGMENT_DENSITY_MAP_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!(
    "VK_EXT_fragment_density_map"
);
///Provided by [`crate::extensions::ext_fragment_density_map`]
impl crate::vk1_3::FormatFeatureFlagBits2 {
    pub const FRAGMENT_DENSITY_MAP_EXT: Self = Self(16777216);
}
///Provided by [`crate::extensions::ext_fragment_density_map`]
impl crate::vk1_0::SamplerCreateFlagBits {
    pub const SUBSAMPLED_EXT: Self = Self(1);
    pub const SUBSAMPLED_COARSE_RECONSTRUCTION_EXT: Self = Self(2);
}
///Provided by [`crate::extensions::ext_fragment_density_map`]
impl crate::vk1_0::FormatFeatureFlagBits {
    pub const FRAGMENT_DENSITY_MAP_EXT: Self = Self(16777216);
}
///Provided by [`crate::extensions::ext_fragment_density_map`]
impl crate::vk1_0::ImageCreateFlagBits {
    pub const SUBSAMPLED_EXT: Self = Self(16384);
}
///Provided by [`crate::extensions::ext_fragment_density_map`]
impl crate::vk1_0::ImageLayout {
    pub const FRAGMENT_DENSITY_MAP_OPTIMAL_EXT: Self = Self(1000218000);
}
///Provided by [`crate::extensions::ext_fragment_density_map`]
impl crate::vk1_0::ImageUsageFlagBits {
    pub const FRAGMENT_DENSITY_MAP_EXT: Self = Self(512);
}
///Provided by [`crate::extensions::ext_fragment_density_map`]
impl crate::vk1_0::ImageViewCreateFlagBits {
    pub const FRAGMENT_DENSITY_MAP_DYNAMIC_EXT: Self = Self(1);
}
///Provided by [`crate::extensions::ext_fragment_density_map`]
impl crate::vk1_0::AccessFlagBits {
    pub const FRAGMENT_DENSITY_MAP_READ_EXT: Self = Self(16777216);
}
///Provided by [`crate::extensions::ext_fragment_density_map`]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_FEATURES_EXT: Self = Self(1000218000);
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_PROPERTIES_EXT: Self = Self(
        1000218001,
    );
    pub const RENDER_PASS_FRAGMENT_DENSITY_MAP_CREATE_INFO_EXT: Self = Self(1000218002);
}
///Provided by [`crate::extensions::ext_fragment_density_map`]
impl crate::vk1_0::PipelineStageFlagBits {
    pub const FRAGMENT_DENSITY_PROCESS_EXT: Self = Self(8388608);
}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceFragmentDensityMapFeaturesEXT>
for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<
    'a,
> crate::ExtendableFrom<'a, PhysicalDeviceFragmentDensityMapFeaturesEXTBuilder<'_>>
for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, RenderPassFragmentDensityMapCreateInfoEXT>
for crate::vk1_0::RenderPassCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, RenderPassFragmentDensityMapCreateInfoEXTBuilder<'_>>
for crate::vk1_0::RenderPassCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceFragmentDensityMapFeaturesEXT>
for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<
    'a,
> crate::ExtendableFrom<'a, PhysicalDeviceFragmentDensityMapFeaturesEXTBuilder<'_>>
for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceFragmentDensityMapPropertiesEXT>
for crate::vk1_1::PhysicalDeviceProperties2Builder<'a> {}
impl<
    'a,
> crate::ExtendableFrom<'a, PhysicalDeviceFragmentDensityMapPropertiesEXTBuilder<'_>>
for crate::vk1_1::PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, RenderPassFragmentDensityMapCreateInfoEXT>
for crate::vk1_2::RenderPassCreateInfo2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, RenderPassFragmentDensityMapCreateInfoEXTBuilder<'_>>
for crate::vk1_2::RenderPassCreateInfo2Builder<'a> {}
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentDensityMapFeaturesEXT.html) · Structure
#[doc(alias = "VkPhysicalDeviceFragmentDensityMapFeaturesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceFragmentDensityMapFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub fragment_density_map: crate::vk1_0::Bool32,
    pub fragment_density_map_dynamic: crate::vk1_0::Bool32,
    pub fragment_density_map_non_subsampled_images: crate::vk1_0::Bool32,
}
impl PhysicalDeviceFragmentDensityMapFeaturesEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_FEATURES_EXT;
}
impl Default for PhysicalDeviceFragmentDensityMapFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: Self::STRUCTURE_TYPE,
            p_next: std::ptr::null_mut(),
            fragment_density_map: Default::default(),
            fragment_density_map_dynamic: Default::default(),
            fragment_density_map_non_subsampled_images: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceFragmentDensityMapFeaturesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceFragmentDensityMapFeaturesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("fragment_density_map", &(self.fragment_density_map != 0))
            .field(
                "fragment_density_map_dynamic",
                &(self.fragment_density_map_dynamic != 0),
            )
            .field(
                "fragment_density_map_non_subsampled_images",
                &(self.fragment_density_map_non_subsampled_images != 0),
            )
            .finish()
    }
}
impl PhysicalDeviceFragmentDensityMapFeaturesEXT {
    #[inline]
    pub fn into_builder<'a>(
        self,
    ) -> PhysicalDeviceFragmentDensityMapFeaturesEXTBuilder<'a> {
        PhysicalDeviceFragmentDensityMapFeaturesEXTBuilder(
            self,
            std::marker::PhantomData,
        )
    }
}
#[derive(Copy, Clone)]
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentDensityMapFeaturesEXT.html) · Builder of [`PhysicalDeviceFragmentDensityMapFeaturesEXT`]
#[repr(transparent)]
pub struct PhysicalDeviceFragmentDensityMapFeaturesEXTBuilder<'a>(
    PhysicalDeviceFragmentDensityMapFeaturesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceFragmentDensityMapFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceFragmentDensityMapFeaturesEXTBuilder<'a> {
        PhysicalDeviceFragmentDensityMapFeaturesEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[inline]
    #[must_use]
    pub fn fragment_density_map(mut self, fragment_density_map: bool) -> Self {
        self.0.fragment_density_map = fragment_density_map as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn fragment_density_map_dynamic(
        mut self,
        fragment_density_map_dynamic: bool,
    ) -> Self {
        self.0.fragment_density_map_dynamic = fragment_density_map_dynamic as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn fragment_density_map_non_subsampled_images(
        mut self,
        fragment_density_map_non_subsampled_images: bool,
    ) -> Self {
        self
            .0
            .fragment_density_map_non_subsampled_images = fragment_density_map_non_subsampled_images
            as _;
        self
    }
    #[inline]
    /// Discards all lifetime information.
    /// Use the `Deref` and `DerefMut` implementations if possible.
    pub fn build_dangling(self) -> PhysicalDeviceFragmentDensityMapFeaturesEXT {
        self.0
    }
}
impl<'a> std::default::Default
for PhysicalDeviceFragmentDensityMapFeaturesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceFragmentDensityMapFeaturesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceFragmentDensityMapFeaturesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceFragmentDensityMapFeaturesEXTBuilder<'a> {
    type Target = PhysicalDeviceFragmentDensityMapFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceFragmentDensityMapFeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentDensityMapPropertiesEXT.html) · Structure
#[doc(alias = "VkPhysicalDeviceFragmentDensityMapPropertiesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceFragmentDensityMapPropertiesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub min_fragment_density_texel_size: crate::vk1_0::Extent2D,
    pub max_fragment_density_texel_size: crate::vk1_0::Extent2D,
    pub fragment_density_invocations: crate::vk1_0::Bool32,
}
impl PhysicalDeviceFragmentDensityMapPropertiesEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_PROPERTIES_EXT;
}
impl Default for PhysicalDeviceFragmentDensityMapPropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: Self::STRUCTURE_TYPE,
            p_next: std::ptr::null_mut(),
            min_fragment_density_texel_size: Default::default(),
            max_fragment_density_texel_size: Default::default(),
            fragment_density_invocations: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceFragmentDensityMapPropertiesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceFragmentDensityMapPropertiesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "min_fragment_density_texel_size",
                &self.min_fragment_density_texel_size,
            )
            .field(
                "max_fragment_density_texel_size",
                &self.max_fragment_density_texel_size,
            )
            .field(
                "fragment_density_invocations",
                &(self.fragment_density_invocations != 0),
            )
            .finish()
    }
}
impl PhysicalDeviceFragmentDensityMapPropertiesEXT {
    #[inline]
    pub fn into_builder<'a>(
        self,
    ) -> PhysicalDeviceFragmentDensityMapPropertiesEXTBuilder<'a> {
        PhysicalDeviceFragmentDensityMapPropertiesEXTBuilder(
            self,
            std::marker::PhantomData,
        )
    }
}
#[derive(Copy, Clone)]
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentDensityMapPropertiesEXT.html) · Builder of [`PhysicalDeviceFragmentDensityMapPropertiesEXT`]
#[repr(transparent)]
pub struct PhysicalDeviceFragmentDensityMapPropertiesEXTBuilder<'a>(
    PhysicalDeviceFragmentDensityMapPropertiesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceFragmentDensityMapPropertiesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceFragmentDensityMapPropertiesEXTBuilder<'a> {
        PhysicalDeviceFragmentDensityMapPropertiesEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[inline]
    #[must_use]
    pub fn min_fragment_density_texel_size(
        mut self,
        min_fragment_density_texel_size: crate::vk1_0::Extent2D,
    ) -> Self {
        self.0.min_fragment_density_texel_size = min_fragment_density_texel_size as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn max_fragment_density_texel_size(
        mut self,
        max_fragment_density_texel_size: crate::vk1_0::Extent2D,
    ) -> Self {
        self.0.max_fragment_density_texel_size = max_fragment_density_texel_size as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn fragment_density_invocations(
        mut self,
        fragment_density_invocations: bool,
    ) -> Self {
        self.0.fragment_density_invocations = fragment_density_invocations as _;
        self
    }
    #[inline]
    /// Discards all lifetime information.
    /// Use the `Deref` and `DerefMut` implementations if possible.
    pub fn build_dangling(self) -> PhysicalDeviceFragmentDensityMapPropertiesEXT {
        self.0
    }
}
impl<'a> std::default::Default
for PhysicalDeviceFragmentDensityMapPropertiesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceFragmentDensityMapPropertiesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceFragmentDensityMapPropertiesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceFragmentDensityMapPropertiesEXTBuilder<'a> {
    type Target = PhysicalDeviceFragmentDensityMapPropertiesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut
for PhysicalDeviceFragmentDensityMapPropertiesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderPassFragmentDensityMapCreateInfoEXT.html) · Structure
#[doc(alias = "VkRenderPassFragmentDensityMapCreateInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RenderPassFragmentDensityMapCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub fragment_density_map_attachment: crate::vk1_0::AttachmentReference,
}
impl RenderPassFragmentDensityMapCreateInfoEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::RENDER_PASS_FRAGMENT_DENSITY_MAP_CREATE_INFO_EXT;
}
impl Default for RenderPassFragmentDensityMapCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: Self::STRUCTURE_TYPE,
            p_next: std::ptr::null(),
            fragment_density_map_attachment: Default::default(),
        }
    }
}
impl std::fmt::Debug for RenderPassFragmentDensityMapCreateInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RenderPassFragmentDensityMapCreateInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "fragment_density_map_attachment",
                &self.fragment_density_map_attachment,
            )
            .finish()
    }
}
impl RenderPassFragmentDensityMapCreateInfoEXT {
    #[inline]
    pub fn into_builder<'a>(
        self,
    ) -> RenderPassFragmentDensityMapCreateInfoEXTBuilder<'a> {
        RenderPassFragmentDensityMapCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderPassFragmentDensityMapCreateInfoEXT.html) · Builder of [`RenderPassFragmentDensityMapCreateInfoEXT`]
#[repr(transparent)]
pub struct RenderPassFragmentDensityMapCreateInfoEXTBuilder<'a>(
    RenderPassFragmentDensityMapCreateInfoEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> RenderPassFragmentDensityMapCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> RenderPassFragmentDensityMapCreateInfoEXTBuilder<'a> {
        RenderPassFragmentDensityMapCreateInfoEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[inline]
    #[must_use]
    pub fn fragment_density_map_attachment(
        mut self,
        fragment_density_map_attachment: crate::vk1_0::AttachmentReference,
    ) -> Self {
        self.0.fragment_density_map_attachment = fragment_density_map_attachment as _;
        self
    }
    #[inline]
    /// Discards all lifetime information.
    /// Use the `Deref` and `DerefMut` implementations if possible.
    pub fn build_dangling(self) -> RenderPassFragmentDensityMapCreateInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for RenderPassFragmentDensityMapCreateInfoEXTBuilder<'a> {
    fn default() -> RenderPassFragmentDensityMapCreateInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for RenderPassFragmentDensityMapCreateInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for RenderPassFragmentDensityMapCreateInfoEXTBuilder<'a> {
    type Target = RenderPassFragmentDensityMapCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for RenderPassFragmentDensityMapCreateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
