# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_EXT_astc_decode_mode.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_ASTC_DECODE_MODE_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_ASTC_DECODE_MODE_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_astc_decode_mode");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageViewASTCDecodeModeEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImageViewASTCDecodeModeEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub decode_mode: crate::vk1_0::Format,
}
impl ImageViewASTCDecodeModeEXT {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: crate::ExtendableBy<Self>,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> ImageViewASTCDecodeModeEXTBuilder<'a> {
        ImageViewASTCDecodeModeEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ImageViewASTCDecodeModeEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ImageViewASTCDecodeModeEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("decode_mode", &self.decode_mode)
            .finish()
    }
}
impl Default for ImageViewASTCDecodeModeEXT {
    fn default() -> ImageViewASTCDecodeModeEXT {
        ImageViewASTCDecodeModeEXT {
            s_type: crate::vk1_0::StructureType::IMAGE_VIEW_ASTC_DECODE_MODE_EXT,
            p_next: std::ptr::null(),
            decode_mode: Default::default(),
        }
    }
}
impl crate::ExtendableBy<ImageViewASTCDecodeModeEXT> for crate::vk1_0::ImageViewCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`ImageViewASTCDecodeModeEXT`](struct.ImageViewASTCDecodeModeEXT.html)"]
#[repr(transparent)]
pub struct ImageViewASTCDecodeModeEXTBuilder<'a>(
    ImageViewASTCDecodeModeEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> ImageViewASTCDecodeModeEXTBuilder<'a> {
    #[inline]
    pub fn new() -> ImageViewASTCDecodeModeEXTBuilder<'a> {
        ImageViewASTCDecodeModeEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn decode_mode(mut self, decode_mode: crate::vk1_0::Format) -> Self {
        self.0.decode_mode = decode_mode;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> ImageViewASTCDecodeModeEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for ImageViewASTCDecodeModeEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for ImageViewASTCDecodeModeEXTBuilder<'a> {
    type Target = ImageViewASTCDecodeModeEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ImageViewASTCDecodeModeEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceASTCDecodeFeaturesEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceASTCDecodeFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub decode_mode_shared_exponent: crate::vk1_0::Bool32,
}
impl PhysicalDeviceASTCDecodeFeaturesEXT {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: crate::ExtendableBy<Self>,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceASTCDecodeFeaturesEXTBuilder<'a> {
        PhysicalDeviceASTCDecodeFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceASTCDecodeFeaturesEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceASTCDecodeFeaturesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "decode_mode_shared_exponent",
                &(self.decode_mode_shared_exponent != 0),
            )
            .finish()
    }
}
impl Default for PhysicalDeviceASTCDecodeFeaturesEXT {
    fn default() -> PhysicalDeviceASTCDecodeFeaturesEXT {
        PhysicalDeviceASTCDecodeFeaturesEXT {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_ASTC_DECODE_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            decode_mode_shared_exponent: Default::default(),
        }
    }
}
impl crate::ExtendableBy<PhysicalDeviceASTCDecodeFeaturesEXT>
    for crate::vk1_1::PhysicalDeviceFeatures2
{
}
impl crate::ExtendableBy<PhysicalDeviceASTCDecodeFeaturesEXT> for crate::vk1_0::DeviceCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceASTCDecodeFeaturesEXT`](struct.PhysicalDeviceASTCDecodeFeaturesEXT.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceASTCDecodeFeaturesEXTBuilder<'a>(
    PhysicalDeviceASTCDecodeFeaturesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceASTCDecodeFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceASTCDecodeFeaturesEXTBuilder<'a> {
        PhysicalDeviceASTCDecodeFeaturesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn decode_mode_shared_exponent(mut self, decode_mode_shared_exponent: bool) -> Self {
        self.0.decode_mode_shared_exponent = decode_mode_shared_exponent as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceASTCDecodeFeaturesEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceASTCDecodeFeaturesEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceASTCDecodeFeaturesEXTBuilder<'a> {
    type Target = PhysicalDeviceASTCDecodeFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceASTCDecodeFeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
