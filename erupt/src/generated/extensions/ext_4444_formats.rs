#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_4444_FORMATS_SPEC_VERSION")]
pub const EXT_4444_FORMATS_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_4444_FORMATS_EXTENSION_NAME")]
pub const EXT_4444_FORMATS_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_4444_formats");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevice4444FormatsFeaturesEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDevice4444FormatsFeaturesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDevice4444FormatsFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub format_a4r4g4b4: crate::vk1_0::Bool32,
    pub format_a4b4g4r4: crate::vk1_0::Bool32,
}
impl Default for PhysicalDevice4444FormatsFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_4444_FORMATS_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            format_a4r4g4b4: Default::default(),
            format_a4b4g4r4: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDevice4444FormatsFeaturesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDevice4444FormatsFeaturesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("format_a4r4g4b4", &(self.format_a4r4g4b4 != 0))
            .field("format_a4b4g4r4", &(self.format_a4b4g4r4 != 0))
            .finish()
    }
}
impl PhysicalDevice4444FormatsFeaturesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDevice4444FormatsFeaturesEXTBuilder<'a> {
        PhysicalDevice4444FormatsFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevice4444FormatsFeaturesEXT.html) · Builder of [`PhysicalDevice4444FormatsFeaturesEXT`]"]
#[repr(transparent)]
pub struct PhysicalDevice4444FormatsFeaturesEXTBuilder<'a>(
    PhysicalDevice4444FormatsFeaturesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDevice4444FormatsFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDevice4444FormatsFeaturesEXTBuilder<'a> {
        PhysicalDevice4444FormatsFeaturesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn format_a4r4g4b4(mut self, format_a4r4g4b4: bool) -> Self {
        self.0.format_a4r4g4b4 = format_a4r4g4b4 as _;
        self
    }
    #[inline]
    pub fn format_a4b4g4r4(mut self, format_a4b4g4r4: bool) -> Self {
        self.0.format_a4b4g4r4 = format_a4b4g4r4 as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDevice4444FormatsFeaturesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDevice4444FormatsFeaturesEXTBuilder<'a> {
    fn default() -> PhysicalDevice4444FormatsFeaturesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDevice4444FormatsFeaturesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDevice4444FormatsFeaturesEXTBuilder<'a> {
    type Target = PhysicalDevice4444FormatsFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDevice4444FormatsFeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
