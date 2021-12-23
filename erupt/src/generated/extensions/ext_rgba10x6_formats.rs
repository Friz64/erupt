#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_RGBA10X6_FORMATS_SPEC_VERSION")]
pub const EXT_RGBA10X6_FORMATS_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_RGBA10X6_FORMATS_EXTENSION_NAME")]
pub const EXT_RGBA10X6_FORMATS_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_rgba10x6_formats");
#[doc = "Provided by [`crate::extensions::ext_rgba10x6_formats`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_RGBA10X6_FORMATS_FEATURES_EXT: Self = Self(1000344000);
}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceRGBA10X6FormatsFeaturesEXT> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceRGBA10X6FormatsFeaturesEXTBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceRGBA10X6FormatsFeaturesEXT> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceRGBA10X6FormatsFeaturesEXTBuilder<'_>> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceRGBA10X6FormatsFeaturesEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceRGBA10X6FormatsFeaturesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceRGBA10X6FormatsFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub format_rgba10x6_without_y_cb_cr_sampler: crate::vk1_0::Bool32,
}
impl PhysicalDeviceRGBA10X6FormatsFeaturesEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_RGBA10X6_FORMATS_FEATURES_EXT;
}
impl Default for PhysicalDeviceRGBA10X6FormatsFeaturesEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), format_rgba10x6_without_y_cb_cr_sampler: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceRGBA10X6FormatsFeaturesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceRGBA10X6FormatsFeaturesEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("format_rgba10x6_without_y_cb_cr_sampler", &(self.format_rgba10x6_without_y_cb_cr_sampler != 0)).finish()
    }
}
impl PhysicalDeviceRGBA10X6FormatsFeaturesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceRGBA10X6FormatsFeaturesEXTBuilder<'a> {
        PhysicalDeviceRGBA10X6FormatsFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceRGBA10X6FormatsFeaturesEXT.html) · Builder of [`PhysicalDeviceRGBA10X6FormatsFeaturesEXT`]"]
#[repr(transparent)]
pub struct PhysicalDeviceRGBA10X6FormatsFeaturesEXTBuilder<'a>(PhysicalDeviceRGBA10X6FormatsFeaturesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceRGBA10X6FormatsFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceRGBA10X6FormatsFeaturesEXTBuilder<'a> {
        PhysicalDeviceRGBA10X6FormatsFeaturesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn format_rgba10x6_without_y_cb_cr_sampler(mut self, format_rgba10x6_without_y_cb_cr_sampler: bool) -> Self {
        self.0.format_rgba10x6_without_y_cb_cr_sampler = format_rgba10x6_without_y_cb_cr_sampler as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> PhysicalDeviceRGBA10X6FormatsFeaturesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceRGBA10X6FormatsFeaturesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceRGBA10X6FormatsFeaturesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceRGBA10X6FormatsFeaturesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceRGBA10X6FormatsFeaturesEXTBuilder<'a> {
    type Target = PhysicalDeviceRGBA10X6FormatsFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceRGBA10X6FormatsFeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
