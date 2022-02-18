// DO NOT EDIT: @generated by erupt's generator
///<s>Vulkan Manual Page</s> · Constant
#[doc(alias = "VK_EXT_TEXEL_BUFFER_ALIGNMENT_SPEC_VERSION")]
pub const EXT_TEXEL_BUFFER_ALIGNMENT_SPEC_VERSION: u32 = 1;
///<s>Vulkan Manual Page</s> · Constant
#[doc(alias = "VK_EXT_TEXEL_BUFFER_ALIGNMENT_EXTENSION_NAME")]
pub const EXT_TEXEL_BUFFER_ALIGNMENT_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!(
    "VK_EXT_texel_buffer_alignment"
);
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceTexelBufferAlignmentPropertiesEXT.html) · Alias
#[doc(alias = "VkPhysicalDeviceTexelBufferAlignmentPropertiesEXT")]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceTexelBufferAlignmentPropertiesEXT = crate::vk1_3::PhysicalDeviceTexelBufferAlignmentProperties;
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceTexelBufferAlignmentPropertiesEXT.html) · Alias
#[doc(alias = "VkPhysicalDeviceTexelBufferAlignmentPropertiesEXT")]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceTexelBufferAlignmentPropertiesEXTBuilder<'a> = crate::vk1_3::PhysicalDeviceTexelBufferAlignmentPropertiesBuilder<
    'a,
>;
///Provided by [`crate::extensions::ext_texel_buffer_alignment`]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_FEATURES_EXT: Self = Self(
        1000281000,
    );
    pub const PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES_EXT: Self = Self::PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES;
}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceTexelBufferAlignmentFeaturesEXT>
for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<
    'a,
> crate::ExtendableFrom<'a, PhysicalDeviceTexelBufferAlignmentFeaturesEXTBuilder<'_>>
for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceTexelBufferAlignmentFeaturesEXT>
for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<
    'a,
> crate::ExtendableFrom<'a, PhysicalDeviceTexelBufferAlignmentFeaturesEXTBuilder<'_>>
for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT.html) · Structure
#[doc(alias = "VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceTexelBufferAlignmentFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub texel_buffer_alignment: crate::vk1_0::Bool32,
}
impl PhysicalDeviceTexelBufferAlignmentFeaturesEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_FEATURES_EXT;
}
impl Default for PhysicalDeviceTexelBufferAlignmentFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: Self::STRUCTURE_TYPE,
            p_next: std::ptr::null_mut(),
            texel_buffer_alignment: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceTexelBufferAlignmentFeaturesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceTexelBufferAlignmentFeaturesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("texel_buffer_alignment", &(self.texel_buffer_alignment != 0))
            .finish()
    }
}
impl PhysicalDeviceTexelBufferAlignmentFeaturesEXT {
    #[inline]
    pub fn into_builder<'a>(
        self,
    ) -> PhysicalDeviceTexelBufferAlignmentFeaturesEXTBuilder<'a> {
        PhysicalDeviceTexelBufferAlignmentFeaturesEXTBuilder(
            self,
            std::marker::PhantomData,
        )
    }
}
#[derive(Copy, Clone)]
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT.html) · Builder of [`PhysicalDeviceTexelBufferAlignmentFeaturesEXT`]
#[repr(transparent)]
pub struct PhysicalDeviceTexelBufferAlignmentFeaturesEXTBuilder<'a>(
    PhysicalDeviceTexelBufferAlignmentFeaturesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceTexelBufferAlignmentFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceTexelBufferAlignmentFeaturesEXTBuilder<'a> {
        PhysicalDeviceTexelBufferAlignmentFeaturesEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[inline]
    #[must_use]
    pub fn texel_buffer_alignment(mut self, texel_buffer_alignment: bool) -> Self {
        self.0.texel_buffer_alignment = texel_buffer_alignment as _;
        self
    }
    #[inline]
    /// Discards all lifetime information.
    /// Use the `Deref` and `DerefMut` implementations if possible.
    pub fn build_dangling(self) -> PhysicalDeviceTexelBufferAlignmentFeaturesEXT {
        self.0
    }
}
impl<'a> std::default::Default
for PhysicalDeviceTexelBufferAlignmentFeaturesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceTexelBufferAlignmentFeaturesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceTexelBufferAlignmentFeaturesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceTexelBufferAlignmentFeaturesEXTBuilder<'a> {
    type Target = PhysicalDeviceTexelBufferAlignmentFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut
for PhysicalDeviceTexelBufferAlignmentFeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
