# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_EXT_texture_compression_astc_hdr.html)\n\n## Extends\n- [`Format`](../../vk1_0/struct.Format.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_TEXTURE_COMPRESSION_ASTC_HDR_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_TEXTURE_COMPRESSION_ASTC_HDR_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_texture_compression_astc_hdr");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceTextureCompressionASTCHDRFeaturesEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceTextureCompressionASTCHDRFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub texture_compression_astc_hdr: crate::vk1_0::Bool32,
}
impl PhysicalDeviceTextureCompressionASTCHDRFeaturesEXT {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceTextureCompressionASTCHDRFeaturesEXT,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceTextureCompressionASTCHDRFeaturesEXTBuilder<'a> {
        PhysicalDeviceTextureCompressionASTCHDRFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceTextureCompressionASTCHDRFeaturesEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceTextureCompressionASTCHDRFeaturesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "texture_compression_astc_hdr",
                &(self.texture_compression_astc_hdr != 0),
            )
            .finish()
    }
}
impl Default for PhysicalDeviceTextureCompressionASTCHDRFeaturesEXT {
    fn default() -> PhysicalDeviceTextureCompressionASTCHDRFeaturesEXT {
        PhysicalDeviceTextureCompressionASTCHDRFeaturesEXT { s_type : crate :: vk1_0 :: StructureType :: PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES_EXT , p_next : std :: ptr :: null_mut ( ) , texture_compression_astc_hdr : Default :: default ( ) , }
    }
}
#[doc = "Used by [`PhysicalDeviceTextureCompressionASTCHDRFeaturesEXT::extend`](struct.PhysicalDeviceTextureCompressionASTCHDRFeaturesEXT.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceTextureCompressionASTCHDRFeaturesEXT {}
impl ExtendableByPhysicalDeviceTextureCompressionASTCHDRFeaturesEXT
    for crate::vk1_1::PhysicalDeviceFeatures2
{
}
impl ExtendableByPhysicalDeviceTextureCompressionASTCHDRFeaturesEXT
    for crate::vk1_0::DeviceCreateInfo
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceTextureCompressionASTCHDRFeaturesEXT`](struct.PhysicalDeviceTextureCompressionASTCHDRFeaturesEXT.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceTextureCompressionASTCHDRFeaturesEXTBuilder<'a>(
    PhysicalDeviceTextureCompressionASTCHDRFeaturesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceTextureCompressionASTCHDRFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceTextureCompressionASTCHDRFeaturesEXTBuilder<'a> {
        PhysicalDeviceTextureCompressionASTCHDRFeaturesEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn texture_compression_astc_hdr(mut self, texture_compression_astc_hdr: bool) -> Self {
        self.0.texture_compression_astc_hdr = texture_compression_astc_hdr as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceTextureCompressionASTCHDRFeaturesEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceTextureCompressionASTCHDRFeaturesEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
