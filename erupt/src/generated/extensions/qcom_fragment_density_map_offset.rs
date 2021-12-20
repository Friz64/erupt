#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_QCOM_FRAGMENT_DENSITY_MAP_OFFSET_SPEC_VERSION")]
pub const QCOM_FRAGMENT_DENSITY_MAP_OFFSET_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_QCOM_FRAGMENT_DENSITY_MAP_OFFSET_EXTENSION_NAME")]
pub const QCOM_FRAGMENT_DENSITY_MAP_OFFSET_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_QCOM_fragment_density_map_offset");
#[doc = "Provided by [`crate::extensions::qcom_fragment_density_map_offset`]"]
impl crate::vk1_0::ImageCreateFlagBits {
    pub const FRAGMENT_DENSITY_MAP_OFFSET_QCOM: Self = Self(32768);
}
#[doc = "Provided by [`crate::extensions::qcom_fragment_density_map_offset`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_FEATURES_QCOM: Self = Self(1000425000);
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_PROPERTIES_QCOM: Self = Self(1000425001);
    pub const SUBPASS_FRAGMENT_DENSITY_MAP_OFFSET_END_INFO_QCOM: Self = Self(1000425002);
}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOMBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOMBuilder<'_>> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM> for crate::vk1_1::PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOMBuilder<'_>> for crate::vk1_1::PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, SubpassFragmentDensityMapOffsetEndInfoQCOM> for crate::vk1_2::SubpassEndInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, SubpassFragmentDensityMapOffsetEndInfoQCOMBuilder<'_>> for crate::vk1_2::SubpassEndInfoBuilder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub fragment_density_map_offset: crate::vk1_0::Bool32,
}
impl PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_FEATURES_QCOM;
}
impl Default for PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), fragment_density_map_offset: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM").field("s_type", &self.s_type).field("p_next", &self.p_next).field("fragment_density_map_offset", &(self.fragment_density_map_offset != 0)).finish()
    }
}
impl PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOMBuilder<'a> {
        PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOMBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM.html) · Builder of [`PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM`]"]
#[repr(transparent)]
pub struct PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOMBuilder<'a>(PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOMBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOMBuilder<'a> {
        PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOMBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn fragment_density_map_offset(mut self, fragment_density_map_offset: bool) -> Self {
        self.0.fragment_density_map_offset = fragment_density_map_offset as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOMBuilder<'a> {
    fn default() -> PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOMBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOMBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOMBuilder<'a> {
    type Target = PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOMBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub fragment_density_offset_granularity: crate::vk1_0::Extent2D,
}
impl PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_PROPERTIES_QCOM;
}
impl Default for PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), fragment_density_offset_granularity: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM").field("s_type", &self.s_type).field("p_next", &self.p_next).field("fragment_density_offset_granularity", &self.fragment_density_offset_granularity).finish()
    }
}
impl PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOMBuilder<'a> {
        PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOMBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM.html) · Builder of [`PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM`]"]
#[repr(transparent)]
pub struct PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOMBuilder<'a>(PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOMBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOMBuilder<'a> {
        PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOMBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn fragment_density_offset_granularity(mut self, fragment_density_offset_granularity: crate::vk1_0::Extent2D) -> Self {
        self.0.fragment_density_offset_granularity = fragment_density_offset_granularity as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOMBuilder<'a> {
    fn default() -> PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOMBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOMBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOMBuilder<'a> {
    type Target = PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOMBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubpassFragmentDensityMapOffsetEndInfoQCOM.html) · Structure"]
#[doc(alias = "VkSubpassFragmentDensityMapOffsetEndInfoQCOM")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SubpassFragmentDensityMapOffsetEndInfoQCOM {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub fragment_density_offset_count: u32,
    pub p_fragment_density_offsets: *const crate::vk1_0::Offset2D,
}
impl SubpassFragmentDensityMapOffsetEndInfoQCOM {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::SUBPASS_FRAGMENT_DENSITY_MAP_OFFSET_END_INFO_QCOM;
}
impl Default for SubpassFragmentDensityMapOffsetEndInfoQCOM {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), fragment_density_offset_count: Default::default(), p_fragment_density_offsets: std::ptr::null() }
    }
}
impl std::fmt::Debug for SubpassFragmentDensityMapOffsetEndInfoQCOM {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubpassFragmentDensityMapOffsetEndInfoQCOM").field("s_type", &self.s_type).field("p_next", &self.p_next).field("fragment_density_offset_count", &self.fragment_density_offset_count).field("p_fragment_density_offsets", &self.p_fragment_density_offsets).finish()
    }
}
impl SubpassFragmentDensityMapOffsetEndInfoQCOM {
    #[inline]
    pub fn into_builder<'a>(self) -> SubpassFragmentDensityMapOffsetEndInfoQCOMBuilder<'a> {
        SubpassFragmentDensityMapOffsetEndInfoQCOMBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubpassFragmentDensityMapOffsetEndInfoQCOM.html) · Builder of [`SubpassFragmentDensityMapOffsetEndInfoQCOM`]"]
#[repr(transparent)]
pub struct SubpassFragmentDensityMapOffsetEndInfoQCOMBuilder<'a>(SubpassFragmentDensityMapOffsetEndInfoQCOM, std::marker::PhantomData<&'a ()>);
impl<'a> SubpassFragmentDensityMapOffsetEndInfoQCOMBuilder<'a> {
    #[inline]
    pub fn new() -> SubpassFragmentDensityMapOffsetEndInfoQCOMBuilder<'a> {
        SubpassFragmentDensityMapOffsetEndInfoQCOMBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn fragment_density_offsets(mut self, fragment_density_offsets: &'a [crate::vk1_0::Offset2DBuilder]) -> Self {
        self.0.p_fragment_density_offsets = fragment_density_offsets.as_ptr() as _;
        self.0.fragment_density_offset_count = fragment_density_offsets.len() as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> SubpassFragmentDensityMapOffsetEndInfoQCOM {
        self.0
    }
}
impl<'a> std::default::Default for SubpassFragmentDensityMapOffsetEndInfoQCOMBuilder<'a> {
    fn default() -> SubpassFragmentDensityMapOffsetEndInfoQCOMBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for SubpassFragmentDensityMapOffsetEndInfoQCOMBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for SubpassFragmentDensityMapOffsetEndInfoQCOMBuilder<'a> {
    type Target = SubpassFragmentDensityMapOffsetEndInfoQCOM;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SubpassFragmentDensityMapOffsetEndInfoQCOMBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
