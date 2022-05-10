// DO NOT EDIT: @generated by erupt's generator
///<s>Vulkan Manual Page</s> · Constant
#[doc(alias = "VK_EXT_GRAPHICS_PIPELINE_LIBRARY_SPEC_VERSION")]
pub const EXT_GRAPHICS_PIPELINE_LIBRARY_SPEC_VERSION: u32 = 1;
///<s>Vulkan Manual Page</s> · Constant
#[doc(alias = "VK_EXT_GRAPHICS_PIPELINE_LIBRARY_EXTENSION_NAME")]
pub const EXT_GRAPHICS_PIPELINE_LIBRARY_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!(
    "VK_EXT_graphics_pipeline_library"
);
///Provided by [`crate::extensions::ext_graphics_pipeline_library`]
impl crate::vk1_0::PipelineCreateFlagBits {
    pub const RETAIN_LINK_TIME_OPTIMIZATION_INFO_EXT: Self = Self(8388608);
    pub const LINK_TIME_OPTIMIZATION_EXT: Self = Self(1024);
}
///Provided by [`crate::extensions::ext_graphics_pipeline_library`]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_FEATURES_EXT: Self = Self(
        1000320000,
    );
    pub const PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_PROPERTIES_EXT: Self = Self(
        1000320001,
    );
    pub const GRAPHICS_PIPELINE_LIBRARY_CREATE_INFO_EXT: Self = Self(1000320002);
}
///Provided by [`crate::extensions::ext_graphics_pipeline_library`]
impl crate::vk1_0::PipelineLayoutCreateFlagBits {
    pub const INDEPENDENT_SETS_EXT: Self = Self(2);
}
bitflags::bitflags! {
    #[doc =
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGraphicsPipelineLibraryFlagsEXT.html) · Bitmask of [`GraphicsPipelineLibraryFlagBitsEXT`]"]
    #[doc(alias = "VkGraphicsPipelineLibraryFlagsEXT")] #[derive(Default)]
    #[repr(transparent)] pub struct GraphicsPipelineLibraryFlagsEXT : u32 { const
    VERTEX_INPUT_INTERFACE_EXT =
    GraphicsPipelineLibraryFlagBitsEXT::VERTEX_INPUT_INTERFACE_EXT.0; const
    PRE_RASTERIZATION_SHADERS_EXT =
    GraphicsPipelineLibraryFlagBitsEXT::PRE_RASTERIZATION_SHADERS_EXT.0; const
    FRAGMENT_SHADER_EXT = GraphicsPipelineLibraryFlagBitsEXT::FRAGMENT_SHADER_EXT.0;
    const FRAGMENT_OUTPUT_INTERFACE_EXT =
    GraphicsPipelineLibraryFlagBitsEXT::FRAGMENT_OUTPUT_INTERFACE_EXT.0; }
}
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGraphicsPipelineLibraryFlagBitsEXT.html) · Bits enum of [`GraphicsPipelineLibraryFlagsEXT`]
#[doc(alias = "VkGraphicsPipelineLibraryFlagBitsEXT")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct GraphicsPipelineLibraryFlagBitsEXT(pub u32);
impl GraphicsPipelineLibraryFlagBitsEXT {
    #[inline]
    ///Converts this enum variant to the corresponding bitmask
    pub const fn bitmask(&self) -> GraphicsPipelineLibraryFlagsEXT {
        GraphicsPipelineLibraryFlagsEXT::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for GraphicsPipelineLibraryFlagBitsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(
            match self {
                &Self::VERTEX_INPUT_INTERFACE_EXT => "VERTEX_INPUT_INTERFACE_EXT",
                &Self::PRE_RASTERIZATION_SHADERS_EXT => "PRE_RASTERIZATION_SHADERS_EXT",
                &Self::FRAGMENT_SHADER_EXT => "FRAGMENT_SHADER_EXT",
                &Self::FRAGMENT_OUTPUT_INTERFACE_EXT => "FRAGMENT_OUTPUT_INTERFACE_EXT",
                _ => "(unknown variant)",
            },
        )
    }
}
///Provided by [`crate::extensions::ext_graphics_pipeline_library`]
impl crate::extensions::ext_graphics_pipeline_library::GraphicsPipelineLibraryFlagBitsEXT {
    pub const VERTEX_INPUT_INTERFACE_EXT: Self = Self(1);
    pub const PRE_RASTERIZATION_SHADERS_EXT: Self = Self(2);
    pub const FRAGMENT_SHADER_EXT: Self = Self(4);
    pub const FRAGMENT_OUTPUT_INTERFACE_EXT: Self = Self(8);
}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceGraphicsPipelineLibraryFeaturesEXT>
for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<
    'a,
> crate::ExtendableFrom<'a, PhysicalDeviceGraphicsPipelineLibraryFeaturesEXTBuilder<'_>>
for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, GraphicsPipelineLibraryCreateInfoEXT>
for crate::vk1_0::GraphicsPipelineCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, GraphicsPipelineLibraryCreateInfoEXTBuilder<'_>>
for crate::vk1_0::GraphicsPipelineCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceGraphicsPipelineLibraryFeaturesEXT>
for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<
    'a,
> crate::ExtendableFrom<'a, PhysicalDeviceGraphicsPipelineLibraryFeaturesEXTBuilder<'_>>
for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceGraphicsPipelineLibraryPropertiesEXT>
for crate::vk1_1::PhysicalDeviceProperties2Builder<'a> {}
impl<
    'a,
> crate::ExtendableFrom<
    'a,
    PhysicalDeviceGraphicsPipelineLibraryPropertiesEXTBuilder<'_>,
> for crate::vk1_1::PhysicalDeviceProperties2Builder<'a> {}
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceGraphicsPipelineLibraryFeaturesEXT.html) · Structure
#[doc(alias = "VkPhysicalDeviceGraphicsPipelineLibraryFeaturesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceGraphicsPipelineLibraryFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub graphics_pipeline_library: crate::vk1_0::Bool32,
}
impl PhysicalDeviceGraphicsPipelineLibraryFeaturesEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_FEATURES_EXT;
}
impl Default for PhysicalDeviceGraphicsPipelineLibraryFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: Self::STRUCTURE_TYPE,
            p_next: std::ptr::null_mut(),
            graphics_pipeline_library: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceGraphicsPipelineLibraryFeaturesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceGraphicsPipelineLibraryFeaturesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("graphics_pipeline_library", &(self.graphics_pipeline_library != 0))
            .finish()
    }
}
impl PhysicalDeviceGraphicsPipelineLibraryFeaturesEXT {
    #[inline]
    pub fn into_builder<'a>(
        self,
    ) -> PhysicalDeviceGraphicsPipelineLibraryFeaturesEXTBuilder<'a> {
        PhysicalDeviceGraphicsPipelineLibraryFeaturesEXTBuilder(
            self,
            std::marker::PhantomData,
        )
    }
}
#[derive(Copy, Clone)]
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceGraphicsPipelineLibraryFeaturesEXT.html) · Builder of [`PhysicalDeviceGraphicsPipelineLibraryFeaturesEXT`]
#[repr(transparent)]
pub struct PhysicalDeviceGraphicsPipelineLibraryFeaturesEXTBuilder<'a>(
    PhysicalDeviceGraphicsPipelineLibraryFeaturesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceGraphicsPipelineLibraryFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceGraphicsPipelineLibraryFeaturesEXTBuilder<'a> {
        PhysicalDeviceGraphicsPipelineLibraryFeaturesEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[inline]
    #[must_use]
    pub fn graphics_pipeline_library(mut self, graphics_pipeline_library: bool) -> Self {
        self.0.graphics_pipeline_library = graphics_pipeline_library as _;
        self
    }
    #[inline]
    /// Discards all lifetime information.
    /// Use the `Deref` and `DerefMut` implementations if possible.
    pub fn build_dangling(self) -> PhysicalDeviceGraphicsPipelineLibraryFeaturesEXT {
        self.0
    }
}
impl<'a> std::default::Default
for PhysicalDeviceGraphicsPipelineLibraryFeaturesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceGraphicsPipelineLibraryFeaturesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug
for PhysicalDeviceGraphicsPipelineLibraryFeaturesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref
for PhysicalDeviceGraphicsPipelineLibraryFeaturesEXTBuilder<'a> {
    type Target = PhysicalDeviceGraphicsPipelineLibraryFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut
for PhysicalDeviceGraphicsPipelineLibraryFeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceGraphicsPipelineLibraryPropertiesEXT.html) · Structure
#[doc(alias = "VkPhysicalDeviceGraphicsPipelineLibraryPropertiesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceGraphicsPipelineLibraryPropertiesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub graphics_pipeline_library_fast_linking: crate::vk1_0::Bool32,
    pub graphics_pipeline_library_independent_interpolation_decoration: crate::vk1_0::Bool32,
}
impl PhysicalDeviceGraphicsPipelineLibraryPropertiesEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_PROPERTIES_EXT;
}
impl Default for PhysicalDeviceGraphicsPipelineLibraryPropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: Self::STRUCTURE_TYPE,
            p_next: std::ptr::null_mut(),
            graphics_pipeline_library_fast_linking: Default::default(),
            graphics_pipeline_library_independent_interpolation_decoration: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceGraphicsPipelineLibraryPropertiesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceGraphicsPipelineLibraryPropertiesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "graphics_pipeline_library_fast_linking",
                &(self.graphics_pipeline_library_fast_linking != 0),
            )
            .field(
                "graphics_pipeline_library_independent_interpolation_decoration",
                &(self.graphics_pipeline_library_independent_interpolation_decoration
                    != 0),
            )
            .finish()
    }
}
impl PhysicalDeviceGraphicsPipelineLibraryPropertiesEXT {
    #[inline]
    pub fn into_builder<'a>(
        self,
    ) -> PhysicalDeviceGraphicsPipelineLibraryPropertiesEXTBuilder<'a> {
        PhysicalDeviceGraphicsPipelineLibraryPropertiesEXTBuilder(
            self,
            std::marker::PhantomData,
        )
    }
}
#[derive(Copy, Clone)]
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceGraphicsPipelineLibraryPropertiesEXT.html) · Builder of [`PhysicalDeviceGraphicsPipelineLibraryPropertiesEXT`]
#[repr(transparent)]
pub struct PhysicalDeviceGraphicsPipelineLibraryPropertiesEXTBuilder<'a>(
    PhysicalDeviceGraphicsPipelineLibraryPropertiesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceGraphicsPipelineLibraryPropertiesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceGraphicsPipelineLibraryPropertiesEXTBuilder<'a> {
        PhysicalDeviceGraphicsPipelineLibraryPropertiesEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[inline]
    #[must_use]
    pub fn graphics_pipeline_library_fast_linking(
        mut self,
        graphics_pipeline_library_fast_linking: bool,
    ) -> Self {
        self
            .0
            .graphics_pipeline_library_fast_linking = graphics_pipeline_library_fast_linking
            as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn graphics_pipeline_library_independent_interpolation_decoration(
        mut self,
        graphics_pipeline_library_independent_interpolation_decoration: bool,
    ) -> Self {
        self
            .0
            .graphics_pipeline_library_independent_interpolation_decoration = graphics_pipeline_library_independent_interpolation_decoration
            as _;
        self
    }
    #[inline]
    /// Discards all lifetime information.
    /// Use the `Deref` and `DerefMut` implementations if possible.
    pub fn build_dangling(self) -> PhysicalDeviceGraphicsPipelineLibraryPropertiesEXT {
        self.0
    }
}
impl<'a> std::default::Default
for PhysicalDeviceGraphicsPipelineLibraryPropertiesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceGraphicsPipelineLibraryPropertiesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug
for PhysicalDeviceGraphicsPipelineLibraryPropertiesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref
for PhysicalDeviceGraphicsPipelineLibraryPropertiesEXTBuilder<'a> {
    type Target = PhysicalDeviceGraphicsPipelineLibraryPropertiesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut
for PhysicalDeviceGraphicsPipelineLibraryPropertiesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGraphicsPipelineLibraryCreateInfoEXT.html) · Structure
#[doc(alias = "VkGraphicsPipelineLibraryCreateInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GraphicsPipelineLibraryCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub flags: crate::extensions::ext_graphics_pipeline_library::GraphicsPipelineLibraryFlagsEXT,
}
impl GraphicsPipelineLibraryCreateInfoEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::GRAPHICS_PIPELINE_LIBRARY_CREATE_INFO_EXT;
}
impl Default for GraphicsPipelineLibraryCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: Self::STRUCTURE_TYPE,
            p_next: std::ptr::null_mut(),
            flags: Default::default(),
        }
    }
}
impl std::fmt::Debug for GraphicsPipelineLibraryCreateInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("GraphicsPipelineLibraryCreateInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .finish()
    }
}
impl GraphicsPipelineLibraryCreateInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> GraphicsPipelineLibraryCreateInfoEXTBuilder<'a> {
        GraphicsPipelineLibraryCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGraphicsPipelineLibraryCreateInfoEXT.html) · Builder of [`GraphicsPipelineLibraryCreateInfoEXT`]
#[repr(transparent)]
pub struct GraphicsPipelineLibraryCreateInfoEXTBuilder<'a>(
    GraphicsPipelineLibraryCreateInfoEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> GraphicsPipelineLibraryCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> GraphicsPipelineLibraryCreateInfoEXTBuilder<'a> {
        GraphicsPipelineLibraryCreateInfoEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[inline]
    #[must_use]
    pub fn flags(
        mut self,
        flags: crate::extensions::ext_graphics_pipeline_library::GraphicsPipelineLibraryFlagsEXT,
    ) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    /// Discards all lifetime information.
    /// Use the `Deref` and `DerefMut` implementations if possible.
    pub fn build_dangling(self) -> GraphicsPipelineLibraryCreateInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for GraphicsPipelineLibraryCreateInfoEXTBuilder<'a> {
    fn default() -> GraphicsPipelineLibraryCreateInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for GraphicsPipelineLibraryCreateInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for GraphicsPipelineLibraryCreateInfoEXTBuilder<'a> {
    type Target = GraphicsPipelineLibraryCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for GraphicsPipelineLibraryCreateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}