// DO NOT EDIT: @generated by erupt's generator
///<s>Vulkan Manual Page</s> · Constant
#[doc(alias = "VK_EXT_PIPELINE_PROPERTIES_SPEC_VERSION")]
pub const EXT_PIPELINE_PROPERTIES_SPEC_VERSION: u32 = 1;
///<s>Vulkan Manual Page</s> · Constant
#[doc(alias = "VK_EXT_PIPELINE_PROPERTIES_EXTENSION_NAME")]
pub const EXT_PIPELINE_PROPERTIES_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!(
    "VK_EXT_pipeline_properties"
);
///<s>Vulkan Manual Page</s> · Constant
pub const FN_GET_PIPELINE_PROPERTIES_EXT: *const std::os::raw::c_char = crate::cstr!(
    "vkGetPipelinePropertiesEXT"
);
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineInfoEXT.html) · Alias
#[doc(alias = "VkPipelineInfoEXT")]
#[allow(non_camel_case_types)]
pub type PipelineInfoEXT = crate::extensions::khr_pipeline_executable_properties::PipelineInfoKHR;
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineInfoEXT.html) · Alias
#[doc(alias = "VkPipelineInfoEXT")]
#[allow(non_camel_case_types)]
pub type PipelineInfoEXTBuilder<'a> = crate::extensions::khr_pipeline_executable_properties::PipelineInfoKHRBuilder<
    'a,
>;
///Provided by [`crate::extensions::ext_pipeline_properties`]
impl crate::vk1_0::StructureType {
    pub const PIPELINE_PROPERTIES_IDENTIFIER_EXT: Self = Self(1000372000);
    pub const PHYSICAL_DEVICE_PIPELINE_PROPERTIES_FEATURES_EXT: Self = Self(1000372001);
    pub const PIPELINE_INFO_EXT: Self = Self::PIPELINE_INFO_KHR;
}
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPipelinePropertiesEXT.html) · Function
#[allow(non_camel_case_types)]
pub type PFN_vkGetPipelinePropertiesEXT = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_pipeline_info: *const crate::extensions::ext_pipeline_properties::PipelineInfoEXT,
    p_pipeline_properties: *mut crate::vk1_0::BaseOutStructure,
) -> crate::vk1_0::Result;
impl<'a> crate::ExtendableFrom<'a, PhysicalDevicePipelinePropertiesFeaturesEXT>
for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<
    'a,
> crate::ExtendableFrom<'a, PhysicalDevicePipelinePropertiesFeaturesEXTBuilder<'_>>
for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDevicePipelinePropertiesFeaturesEXT>
for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<
    'a,
> crate::ExtendableFrom<'a, PhysicalDevicePipelinePropertiesFeaturesEXTBuilder<'_>>
for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelinePropertiesIdentifierEXT.html) · Structure
#[doc(alias = "VkPipelinePropertiesIdentifierEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelinePropertiesIdentifierEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub pipeline_identifier: [u8; 16],
}
impl PipelinePropertiesIdentifierEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PIPELINE_PROPERTIES_IDENTIFIER_EXT;
}
impl Default for PipelinePropertiesIdentifierEXT {
    fn default() -> Self {
        Self {
            s_type: Self::STRUCTURE_TYPE,
            p_next: std::ptr::null_mut(),
            pipeline_identifier: unsafe { std::mem::zeroed() },
        }
    }
}
impl std::fmt::Debug for PipelinePropertiesIdentifierEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PipelinePropertiesIdentifierEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("pipeline_identifier", &self.pipeline_identifier)
            .finish()
    }
}
impl PipelinePropertiesIdentifierEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PipelinePropertiesIdentifierEXTBuilder<'a> {
        PipelinePropertiesIdentifierEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelinePropertiesIdentifierEXT.html) · Builder of [`PipelinePropertiesIdentifierEXT`]
#[repr(transparent)]
pub struct PipelinePropertiesIdentifierEXTBuilder<'a>(
    PipelinePropertiesIdentifierEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PipelinePropertiesIdentifierEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PipelinePropertiesIdentifierEXTBuilder<'a> {
        PipelinePropertiesIdentifierEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[inline]
    #[must_use]
    pub fn pipeline_identifier(mut self, pipeline_identifier: [u8; 16]) -> Self {
        self.0.pipeline_identifier = pipeline_identifier as _;
        self
    }
    #[inline]
    /// Discards all lifetime information.
    /// Use the `Deref` and `DerefMut` implementations if possible.
    pub fn build_dangling(self) -> PipelinePropertiesIdentifierEXT {
        self.0
    }
}
impl<'a> std::default::Default for PipelinePropertiesIdentifierEXTBuilder<'a> {
    fn default() -> PipelinePropertiesIdentifierEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PipelinePropertiesIdentifierEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PipelinePropertiesIdentifierEXTBuilder<'a> {
    type Target = PipelinePropertiesIdentifierEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelinePropertiesIdentifierEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePipelinePropertiesFeaturesEXT.html) · Structure
#[doc(alias = "VkPhysicalDevicePipelinePropertiesFeaturesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDevicePipelinePropertiesFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub pipeline_properties_identifier: crate::vk1_0::Bool32,
}
impl PhysicalDevicePipelinePropertiesFeaturesEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_PIPELINE_PROPERTIES_FEATURES_EXT;
}
impl Default for PhysicalDevicePipelinePropertiesFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: Self::STRUCTURE_TYPE,
            p_next: std::ptr::null_mut(),
            pipeline_properties_identifier: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDevicePipelinePropertiesFeaturesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDevicePipelinePropertiesFeaturesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "pipeline_properties_identifier",
                &(self.pipeline_properties_identifier != 0),
            )
            .finish()
    }
}
impl PhysicalDevicePipelinePropertiesFeaturesEXT {
    #[inline]
    pub fn into_builder<'a>(
        self,
    ) -> PhysicalDevicePipelinePropertiesFeaturesEXTBuilder<'a> {
        PhysicalDevicePipelinePropertiesFeaturesEXTBuilder(
            self,
            std::marker::PhantomData,
        )
    }
}
#[derive(Copy, Clone)]
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePipelinePropertiesFeaturesEXT.html) · Builder of [`PhysicalDevicePipelinePropertiesFeaturesEXT`]
#[repr(transparent)]
pub struct PhysicalDevicePipelinePropertiesFeaturesEXTBuilder<'a>(
    PhysicalDevicePipelinePropertiesFeaturesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDevicePipelinePropertiesFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDevicePipelinePropertiesFeaturesEXTBuilder<'a> {
        PhysicalDevicePipelinePropertiesFeaturesEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[inline]
    #[must_use]
    pub fn pipeline_properties_identifier(
        mut self,
        pipeline_properties_identifier: bool,
    ) -> Self {
        self.0.pipeline_properties_identifier = pipeline_properties_identifier as _;
        self
    }
    #[inline]
    /// Discards all lifetime information.
    /// Use the `Deref` and `DerefMut` implementations if possible.
    pub fn build_dangling(self) -> PhysicalDevicePipelinePropertiesFeaturesEXT {
        self.0
    }
}
impl<'a> std::default::Default
for PhysicalDevicePipelinePropertiesFeaturesEXTBuilder<'a> {
    fn default() -> PhysicalDevicePipelinePropertiesFeaturesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDevicePipelinePropertiesFeaturesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDevicePipelinePropertiesFeaturesEXTBuilder<'a> {
    type Target = PhysicalDevicePipelinePropertiesFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDevicePipelinePropertiesFeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
///Provided by [`crate::extensions::ext_pipeline_properties`]
impl crate::DeviceLoader {
    #[inline]
    #[track_caller]
    ///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPipelinePropertiesEXT.html) · Function
    #[doc(alias = "vkGetPipelinePropertiesEXT")]
    pub unsafe fn get_pipeline_properties_ext(
        &self,
        pipeline_info: &crate::extensions::ext_pipeline_properties::PipelineInfoEXT,
    ) -> crate::utils::VulkanResult<crate::vk1_0::BaseOutStructure> {
        let _function = self
            .get_pipeline_properties_ext
            .expect(crate::NOT_LOADED_MESSAGE);
        let mut pipeline_properties = Default::default();
        let _return = _function(
            self.handle,
            pipeline_info as _,
            &mut pipeline_properties,
        );
        crate::utils::VulkanResult::new(_return, pipeline_properties)
    }
}