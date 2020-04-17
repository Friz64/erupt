# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_AMD_texture_gather_bias_lod.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const AMD_TEXTURE_GATHER_BIAS_LOD_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const AMD_TEXTURE_GATHER_BIAS_LOD_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_AMD_texture_gather_bias_lod");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkTextureLODGatherFormatPropertiesAMD.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TextureLODGatherFormatPropertiesAMD {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub supports_texture_gather_lod_bias_amd: crate::vk1_0::Bool32,
}
impl TextureLODGatherFormatPropertiesAMD {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByTextureLODGatherFormatPropertiesAMD,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> TextureLODGatherFormatPropertiesAMDBuilder<'a> {
        TextureLODGatherFormatPropertiesAMDBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for TextureLODGatherFormatPropertiesAMD {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("TextureLODGatherFormatPropertiesAMD")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "supports_texture_gather_lod_bias_amd",
                &(self.supports_texture_gather_lod_bias_amd != 0),
            )
            .finish()
    }
}
impl Default for TextureLODGatherFormatPropertiesAMD {
    fn default() -> TextureLODGatherFormatPropertiesAMD {
        TextureLODGatherFormatPropertiesAMD {
            s_type: crate::vk1_0::StructureType::TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD,
            p_next: std::ptr::null_mut(),
            supports_texture_gather_lod_bias_amd: Default::default(),
        }
    }
}
#[doc = "Used by [`TextureLODGatherFormatPropertiesAMD::extend`](struct.TextureLODGatherFormatPropertiesAMD.html#method.extend)"]
pub trait ExtendableByTextureLODGatherFormatPropertiesAMD {}
impl ExtendableByTextureLODGatherFormatPropertiesAMD for crate::vk1_1::ImageFormatProperties2 {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`TextureLODGatherFormatPropertiesAMD`](struct.TextureLODGatherFormatPropertiesAMD.html)"]
#[repr(transparent)]
pub struct TextureLODGatherFormatPropertiesAMDBuilder<'a>(
    TextureLODGatherFormatPropertiesAMD,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> TextureLODGatherFormatPropertiesAMDBuilder<'a> {
    #[inline]
    pub fn new() -> TextureLODGatherFormatPropertiesAMDBuilder<'a> {
        TextureLODGatherFormatPropertiesAMDBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn supports_texture_gather_lod_bias_amd(
        mut self,
        supports_texture_gather_lod_bias_amd: bool,
    ) -> Self {
        self.0.supports_texture_gather_lod_bias_amd = supports_texture_gather_lod_bias_amd as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> TextureLODGatherFormatPropertiesAMD {
        self.0
    }
}
impl<'a> std::fmt::Debug for TextureLODGatherFormatPropertiesAMDBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
