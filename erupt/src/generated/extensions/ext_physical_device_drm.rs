#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_PHYSICAL_DEVICE_DRM_SPEC_VERSION")]
pub const EXT_PHYSICAL_DEVICE_DRM_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_PHYSICAL_DEVICE_DRM_EXTENSION_NAME")]
pub const EXT_PHYSICAL_DEVICE_DRM_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_physical_device_drm");
#[doc = "Provided by [`crate::extensions::ext_physical_device_drm`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_DRM_PROPERTIES_EXT: Self = Self(1000353000);
}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceDrmPropertiesEXT> for crate::vk1_1::PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceDrmPropertiesEXTBuilder<'_>> for crate::vk1_1::PhysicalDeviceProperties2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDrmPropertiesEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceDrmPropertiesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceDrmPropertiesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub has_primary: crate::vk1_0::Bool32,
    pub has_render: crate::vk1_0::Bool32,
    pub primary_major: i64,
    pub primary_minor: i64,
    pub render_major: i64,
    pub render_minor: i64,
}
impl PhysicalDeviceDrmPropertiesEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_DRM_PROPERTIES_EXT;
}
impl Default for PhysicalDeviceDrmPropertiesEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_DRM_PROPERTIES_EXT, p_next: std::ptr::null_mut(), has_primary: Default::default(), has_render: Default::default(), primary_major: Default::default(), primary_minor: Default::default(), render_major: Default::default(), render_minor: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceDrmPropertiesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceDrmPropertiesEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("has_primary", &(self.has_primary != 0)).field("has_render", &(self.has_render != 0)).field("primary_major", &self.primary_major).field("primary_minor", &self.primary_minor).field("render_major", &self.render_major).field("render_minor", &self.render_minor).finish()
    }
}
impl PhysicalDeviceDrmPropertiesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceDrmPropertiesEXTBuilder<'a> {
        PhysicalDeviceDrmPropertiesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDrmPropertiesEXT.html) · Builder of [`PhysicalDeviceDrmPropertiesEXT`]"]
#[repr(transparent)]
pub struct PhysicalDeviceDrmPropertiesEXTBuilder<'a>(PhysicalDeviceDrmPropertiesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceDrmPropertiesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceDrmPropertiesEXTBuilder<'a> {
        PhysicalDeviceDrmPropertiesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn has_primary(mut self, has_primary: bool) -> Self {
        self.0.has_primary = has_primary as _;
        self
    }
    #[inline]
    pub fn has_render(mut self, has_render: bool) -> Self {
        self.0.has_render = has_render as _;
        self
    }
    #[inline]
    pub fn primary_major(mut self, primary_major: i64) -> Self {
        self.0.primary_major = primary_major as _;
        self
    }
    #[inline]
    pub fn primary_minor(mut self, primary_minor: i64) -> Self {
        self.0.primary_minor = primary_minor as _;
        self
    }
    #[inline]
    pub fn render_major(mut self, render_major: i64) -> Self {
        self.0.render_major = render_major as _;
        self
    }
    #[inline]
    pub fn render_minor(mut self, render_minor: i64) -> Self {
        self.0.render_minor = render_minor as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceDrmPropertiesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceDrmPropertiesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceDrmPropertiesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceDrmPropertiesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceDrmPropertiesEXTBuilder<'a> {
    type Target = PhysicalDeviceDrmPropertiesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceDrmPropertiesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
