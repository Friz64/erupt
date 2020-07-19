#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_FRAGMENT_DENSITY_MAP_2_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_FRAGMENT_DENSITY_MAP_2_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_fragment_density_map2");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceFragmentDensityMap2FeaturesEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceFragmentDensityMap2FeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub fragment_density_map_deferred: crate::vk1_0::Bool32,
}
impl Default for PhysicalDeviceFragmentDensityMap2FeaturesEXT {
    fn default() -> Self {
        Self {
            s_type:
                crate::vk1_0::StructureType::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            fragment_density_map_deferred: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceFragmentDensityMap2FeaturesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceFragmentDensityMap2FeaturesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "fragment_density_map_deferred",
                &(self.fragment_density_map_deferred != 0),
            )
            .finish()
    }
}
impl PhysicalDeviceFragmentDensityMap2FeaturesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceFragmentDensityMap2FeaturesEXTBuilder<'a> {
        PhysicalDeviceFragmentDensityMap2FeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceFragmentDensityMap2FeaturesEXT.html) · Builder of [`PhysicalDeviceFragmentDensityMap2FeaturesEXT`](struct.PhysicalDeviceFragmentDensityMap2FeaturesEXT.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceFragmentDensityMap2FeaturesEXTBuilder<'a>(
    PhysicalDeviceFragmentDensityMap2FeaturesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceFragmentDensityMap2FeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceFragmentDensityMap2FeaturesEXTBuilder<'a> {
        PhysicalDeviceFragmentDensityMap2FeaturesEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[inline]
    pub fn fragment_density_map_deferred(mut self, fragment_density_map_deferred: bool) -> Self {
        self.0.fragment_density_map_deferred = fragment_density_map_deferred as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceFragmentDensityMap2FeaturesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceFragmentDensityMap2FeaturesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceFragmentDensityMap2FeaturesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceFragmentDensityMap2FeaturesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceFragmentDensityMap2FeaturesEXTBuilder<'a> {
    type Target = PhysicalDeviceFragmentDensityMap2FeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceFragmentDensityMap2FeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceFragmentDensityMap2PropertiesEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceFragmentDensityMap2PropertiesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub subsampled_loads: crate::vk1_0::Bool32,
    pub subsampled_coarse_reconstruction_early_access: crate::vk1_0::Bool32,
    pub max_subsampled_array_layers: u32,
    pub max_descriptor_set_subsampled_samplers: u32,
}
impl Default for PhysicalDeviceFragmentDensityMap2PropertiesEXT {
    fn default() -> Self {
        Self {
            s_type:
                crate::vk1_0::StructureType::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            subsampled_loads: Default::default(),
            subsampled_coarse_reconstruction_early_access: Default::default(),
            max_subsampled_array_layers: Default::default(),
            max_descriptor_set_subsampled_samplers: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceFragmentDensityMap2PropertiesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceFragmentDensityMap2PropertiesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("subsampled_loads", &(self.subsampled_loads != 0))
            .field(
                "subsampled_coarse_reconstruction_early_access",
                &(self.subsampled_coarse_reconstruction_early_access != 0),
            )
            .field(
                "max_subsampled_array_layers",
                &self.max_subsampled_array_layers,
            )
            .field(
                "max_descriptor_set_subsampled_samplers",
                &self.max_descriptor_set_subsampled_samplers,
            )
            .finish()
    }
}
impl PhysicalDeviceFragmentDensityMap2PropertiesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceFragmentDensityMap2PropertiesEXTBuilder<'a> {
        PhysicalDeviceFragmentDensityMap2PropertiesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceFragmentDensityMap2PropertiesEXT.html) · Builder of [`PhysicalDeviceFragmentDensityMap2PropertiesEXT`](struct.PhysicalDeviceFragmentDensityMap2PropertiesEXT.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceFragmentDensityMap2PropertiesEXTBuilder<'a>(
    PhysicalDeviceFragmentDensityMap2PropertiesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceFragmentDensityMap2PropertiesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceFragmentDensityMap2PropertiesEXTBuilder<'a> {
        PhysicalDeviceFragmentDensityMap2PropertiesEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[inline]
    pub fn subsampled_loads(mut self, subsampled_loads: bool) -> Self {
        self.0.subsampled_loads = subsampled_loads as _;
        self
    }
    #[inline]
    pub fn subsampled_coarse_reconstruction_early_access(
        mut self,
        subsampled_coarse_reconstruction_early_access: bool,
    ) -> Self {
        self.0.subsampled_coarse_reconstruction_early_access =
            subsampled_coarse_reconstruction_early_access as _;
        self
    }
    #[inline]
    pub fn max_subsampled_array_layers(mut self, max_subsampled_array_layers: u32) -> Self {
        self.0.max_subsampled_array_layers = max_subsampled_array_layers as _;
        self
    }
    #[inline]
    pub fn max_descriptor_set_subsampled_samplers(
        mut self,
        max_descriptor_set_subsampled_samplers: u32,
    ) -> Self {
        self.0.max_descriptor_set_subsampled_samplers = max_descriptor_set_subsampled_samplers as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceFragmentDensityMap2PropertiesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceFragmentDensityMap2PropertiesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceFragmentDensityMap2PropertiesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceFragmentDensityMap2PropertiesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceFragmentDensityMap2PropertiesEXTBuilder<'a> {
    type Target = PhysicalDeviceFragmentDensityMap2PropertiesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceFragmentDensityMap2PropertiesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
