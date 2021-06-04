#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_YCBCR_2PLANE_444_FORMATS_SPEC_VERSION")]
pub const EXT_YCBCR_2PLANE_444_FORMATS_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_YCBCR_2PLANE_444_FORMATS_EXTENSION_NAME")]
pub const EXT_YCBCR_2PLANE_444_FORMATS_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_ycbcr_2plane_444_formats");
#[doc = "Provided by [`crate::extensions::ext_ycbcr_2plane_444_formats`]"]
impl crate::vk1_0::Format {
    pub const G8_B8R8_2PLANE_444_UNORM_EXT: Self = Self(1000330000);
    pub const G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16_EXT: Self = Self(1000330001);
    pub const G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16_EXT: Self = Self(1000330002);
    pub const G16_B16R16_2PLANE_444_UNORM_EXT: Self = Self(1000330003);
}
#[doc = "Provided by [`crate::extensions::ext_ycbcr_2plane_444_formats`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_YCBCR_2_PLANE_444_FORMATS_FEATURES_EXT: Self = Self(1000330000);
}
impl<'a> crate::ExtendableFromConst<'a, PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXTBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXTBuilder<'_>> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub ycbcr2plane444_formats: crate::vk1_0::Bool32,
}
impl Default for PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_YCBCR_2_PLANE_444_FORMATS_FEATURES_EXT, p_next: std::ptr::null_mut(), ycbcr2plane444_formats: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("ycbcr2plane444_formats", &(self.ycbcr2plane444_formats != 0)).finish()
    }
}
impl PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXTBuilder<'a> {
        PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT.html) · Builder of [`PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT`]"]
#[repr(transparent)]
pub struct PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXTBuilder<'a>(PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXTBuilder<'a> {
        PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn ycbcr2plane444_formats(mut self, ycbcr2plane444_formats: bool) -> Self {
        self.0.ycbcr2plane444_formats = ycbcr2plane444_formats as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXTBuilder<'a> {
    type Target = PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
