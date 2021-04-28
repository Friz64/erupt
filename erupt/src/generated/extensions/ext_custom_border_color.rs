#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_CUSTOM_BORDER_COLOR_SPEC_VERSION")]
pub const EXT_CUSTOM_BORDER_COLOR_SPEC_VERSION: u32 = 12;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_CUSTOM_BORDER_COLOR_EXTENSION_NAME")]
pub const EXT_CUSTOM_BORDER_COLOR_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_custom_border_color");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerCustomBorderColorCreateInfoEXT.html) · Structure"]
#[doc(alias = "VkSamplerCustomBorderColorCreateInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SamplerCustomBorderColorCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub custom_border_color: crate::vk1_0::ClearColorValue,
    pub format: crate::vk1_0::Format,
}
impl Default for SamplerCustomBorderColorCreateInfoEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::SAMPLER_CUSTOM_BORDER_COLOR_CREATE_INFO_EXT, p_next: std::ptr::null(), custom_border_color: Default::default(), format: Default::default() }
    }
}
impl std::fmt::Debug for SamplerCustomBorderColorCreateInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SamplerCustomBorderColorCreateInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("custom_border_color", &self.custom_border_color).field("format", &self.format).finish()
    }
}
impl SamplerCustomBorderColorCreateInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> SamplerCustomBorderColorCreateInfoEXTBuilder<'a> {
        SamplerCustomBorderColorCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerCustomBorderColorCreateInfoEXT.html) · Builder of [`SamplerCustomBorderColorCreateInfoEXT`]"]
#[repr(transparent)]
pub struct SamplerCustomBorderColorCreateInfoEXTBuilder<'a>(SamplerCustomBorderColorCreateInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> SamplerCustomBorderColorCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> SamplerCustomBorderColorCreateInfoEXTBuilder<'a> {
        SamplerCustomBorderColorCreateInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn custom_border_color(mut self, custom_border_color: crate::vk1_0::ClearColorValue) -> Self {
        self.0.custom_border_color = custom_border_color as _;
        self
    }
    #[inline]
    pub fn format(mut self, format: crate::vk1_0::Format) -> Self {
        self.0.format = format as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> SamplerCustomBorderColorCreateInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for SamplerCustomBorderColorCreateInfoEXTBuilder<'a> {
    fn default() -> SamplerCustomBorderColorCreateInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for SamplerCustomBorderColorCreateInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for SamplerCustomBorderColorCreateInfoEXTBuilder<'a> {
    type Target = SamplerCustomBorderColorCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SamplerCustomBorderColorCreateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceCustomBorderColorPropertiesEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceCustomBorderColorPropertiesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceCustomBorderColorPropertiesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub max_custom_border_color_samplers: u32,
}
impl Default for PhysicalDeviceCustomBorderColorPropertiesEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_PROPERTIES_EXT, p_next: std::ptr::null_mut(), max_custom_border_color_samplers: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceCustomBorderColorPropertiesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceCustomBorderColorPropertiesEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("max_custom_border_color_samplers", &self.max_custom_border_color_samplers).finish()
    }
}
impl PhysicalDeviceCustomBorderColorPropertiesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceCustomBorderColorPropertiesEXTBuilder<'a> {
        PhysicalDeviceCustomBorderColorPropertiesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceCustomBorderColorPropertiesEXT.html) · Builder of [`PhysicalDeviceCustomBorderColorPropertiesEXT`]"]
#[repr(transparent)]
pub struct PhysicalDeviceCustomBorderColorPropertiesEXTBuilder<'a>(PhysicalDeviceCustomBorderColorPropertiesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceCustomBorderColorPropertiesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceCustomBorderColorPropertiesEXTBuilder<'a> {
        PhysicalDeviceCustomBorderColorPropertiesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn max_custom_border_color_samplers(mut self, max_custom_border_color_samplers: u32) -> Self {
        self.0.max_custom_border_color_samplers = max_custom_border_color_samplers as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceCustomBorderColorPropertiesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceCustomBorderColorPropertiesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceCustomBorderColorPropertiesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceCustomBorderColorPropertiesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceCustomBorderColorPropertiesEXTBuilder<'a> {
    type Target = PhysicalDeviceCustomBorderColorPropertiesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceCustomBorderColorPropertiesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceCustomBorderColorFeaturesEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceCustomBorderColorFeaturesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceCustomBorderColorFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub custom_border_colors: crate::vk1_0::Bool32,
    pub custom_border_color_without_format: crate::vk1_0::Bool32,
}
impl Default for PhysicalDeviceCustomBorderColorFeaturesEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_FEATURES_EXT, p_next: std::ptr::null_mut(), custom_border_colors: Default::default(), custom_border_color_without_format: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceCustomBorderColorFeaturesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceCustomBorderColorFeaturesEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("custom_border_colors", &(self.custom_border_colors != 0)).field("custom_border_color_without_format", &(self.custom_border_color_without_format != 0)).finish()
    }
}
impl PhysicalDeviceCustomBorderColorFeaturesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceCustomBorderColorFeaturesEXTBuilder<'a> {
        PhysicalDeviceCustomBorderColorFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceCustomBorderColorFeaturesEXT.html) · Builder of [`PhysicalDeviceCustomBorderColorFeaturesEXT`]"]
#[repr(transparent)]
pub struct PhysicalDeviceCustomBorderColorFeaturesEXTBuilder<'a>(PhysicalDeviceCustomBorderColorFeaturesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceCustomBorderColorFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceCustomBorderColorFeaturesEXTBuilder<'a> {
        PhysicalDeviceCustomBorderColorFeaturesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn custom_border_colors(mut self, custom_border_colors: bool) -> Self {
        self.0.custom_border_colors = custom_border_colors as _;
        self
    }
    #[inline]
    pub fn custom_border_color_without_format(mut self, custom_border_color_without_format: bool) -> Self {
        self.0.custom_border_color_without_format = custom_border_color_without_format as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceCustomBorderColorFeaturesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceCustomBorderColorFeaturesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceCustomBorderColorFeaturesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceCustomBorderColorFeaturesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceCustomBorderColorFeaturesEXTBuilder<'a> {
    type Target = PhysicalDeviceCustomBorderColorFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceCustomBorderColorFeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
