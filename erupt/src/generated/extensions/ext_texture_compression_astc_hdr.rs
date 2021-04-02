#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_TEXTURE_COMPRESSION_ASTC_HDR_SPEC_VERSION")]
pub const EXT_TEXTURE_COMPRESSION_ASTC_HDR_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_TEXTURE_COMPRESSION_ASTC_HDR_EXTENSION_NAME")]
pub const EXT_TEXTURE_COMPRESSION_ASTC_HDR_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_texture_compression_astc_hdr");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceTextureCompressionASTCHDRFeaturesEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceTextureCompressionASTCHDRFeaturesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceTextureCompressionASTCHDRFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub texture_compression_astc_hdr: crate::vk1_0::Bool32,
}
impl Default for PhysicalDeviceTextureCompressionASTCHDRFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            texture_compression_astc_hdr: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceTextureCompressionASTCHDRFeaturesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceTextureCompressionASTCHDRFeaturesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("texture_compression_astc_hdr", &(self.texture_compression_astc_hdr != 0))
            .finish()
    }
}
impl PhysicalDeviceTextureCompressionASTCHDRFeaturesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceTextureCompressionASTCHDRFeaturesEXTBuilder<'a> {
        PhysicalDeviceTextureCompressionASTCHDRFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceTextureCompressionASTCHDRFeaturesEXT.html) · Builder of [`PhysicalDeviceTextureCompressionASTCHDRFeaturesEXT`]"]
#[repr(transparent)]
pub struct PhysicalDeviceTextureCompressionASTCHDRFeaturesEXTBuilder<'a>(PhysicalDeviceTextureCompressionASTCHDRFeaturesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceTextureCompressionASTCHDRFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceTextureCompressionASTCHDRFeaturesEXTBuilder<'a> {
        PhysicalDeviceTextureCompressionASTCHDRFeaturesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn texture_compression_astc_hdr(mut self, texture_compression_astc_hdr: bool) -> Self {
        self.0.texture_compression_astc_hdr = texture_compression_astc_hdr as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceTextureCompressionASTCHDRFeaturesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceTextureCompressionASTCHDRFeaturesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceTextureCompressionASTCHDRFeaturesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceTextureCompressionASTCHDRFeaturesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceTextureCompressionASTCHDRFeaturesEXTBuilder<'a> {
    type Target = PhysicalDeviceTextureCompressionASTCHDRFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceTextureCompressionASTCHDRFeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<PhysicalDeviceTextureCompressionASTCHDRFeaturesEXT> for PhysicalDeviceTextureCompressionASTCHDRFeaturesEXTBuilder<'_> {}
