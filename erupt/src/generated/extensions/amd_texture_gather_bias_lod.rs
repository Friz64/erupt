#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_AMD_TEXTURE_GATHER_BIAS_LOD_SPEC_VERSION")]
pub const AMD_TEXTURE_GATHER_BIAS_LOD_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_AMD_TEXTURE_GATHER_BIAS_LOD_EXTENSION_NAME")]
pub const AMD_TEXTURE_GATHER_BIAS_LOD_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_AMD_texture_gather_bias_lod");
#[doc = "Provided by [`crate::extensions::amd_texture_gather_bias_lod`]"]
impl crate::vk1_0::StructureType {
    pub const TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD: Self = Self(1000041000);
}
impl<'a> crate::ExtendableFrom<'a, TextureLODGatherFormatPropertiesAMD> for crate::vk1_1::ImageFormatProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, TextureLODGatherFormatPropertiesAMDBuilder<'_>> for crate::vk1_1::ImageFormatProperties2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkTextureLODGatherFormatPropertiesAMD.html) · Structure"]
#[doc(alias = "VkTextureLODGatherFormatPropertiesAMD")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TextureLODGatherFormatPropertiesAMD {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub supports_texture_gather_lod_bias_amd: crate::vk1_0::Bool32,
}
impl TextureLODGatherFormatPropertiesAMD {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD;
}
impl Default for TextureLODGatherFormatPropertiesAMD {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), supports_texture_gather_lod_bias_amd: Default::default() }
    }
}
impl std::fmt::Debug for TextureLODGatherFormatPropertiesAMD {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TextureLODGatherFormatPropertiesAMD").field("s_type", &self.s_type).field("p_next", &self.p_next).field("supports_texture_gather_lod_bias_amd", &(self.supports_texture_gather_lod_bias_amd != 0)).finish()
    }
}
impl TextureLODGatherFormatPropertiesAMD {
    #[inline]
    pub fn into_builder<'a>(self) -> TextureLODGatherFormatPropertiesAMDBuilder<'a> {
        TextureLODGatherFormatPropertiesAMDBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkTextureLODGatherFormatPropertiesAMD.html) · Builder of [`TextureLODGatherFormatPropertiesAMD`]"]
#[repr(transparent)]
pub struct TextureLODGatherFormatPropertiesAMDBuilder<'a>(TextureLODGatherFormatPropertiesAMD, std::marker::PhantomData<&'a ()>);
impl<'a> TextureLODGatherFormatPropertiesAMDBuilder<'a> {
    #[inline]
    pub fn new() -> TextureLODGatherFormatPropertiesAMDBuilder<'a> {
        TextureLODGatherFormatPropertiesAMDBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn supports_texture_gather_lod_bias_amd(mut self, supports_texture_gather_lod_bias_amd: bool) -> Self {
        self.0.supports_texture_gather_lod_bias_amd = supports_texture_gather_lod_bias_amd as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> TextureLODGatherFormatPropertiesAMD {
        self.0
    }
}
impl<'a> std::default::Default for TextureLODGatherFormatPropertiesAMDBuilder<'a> {
    fn default() -> TextureLODGatherFormatPropertiesAMDBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for TextureLODGatherFormatPropertiesAMDBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for TextureLODGatherFormatPropertiesAMDBuilder<'a> {
    type Target = TextureLODGatherFormatPropertiesAMD;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for TextureLODGatherFormatPropertiesAMDBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
