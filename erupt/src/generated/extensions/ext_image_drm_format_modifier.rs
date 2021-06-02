#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_IMAGE_DRM_FORMAT_MODIFIER_SPEC_VERSION")]
pub const EXT_IMAGE_DRM_FORMAT_MODIFIER_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_IMAGE_DRM_FORMAT_MODIFIER_EXTENSION_NAME")]
pub const EXT_IMAGE_DRM_FORMAT_MODIFIER_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_image_drm_format_modifier");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT: *const std::os::raw::c_char = crate::cstr!("vkGetImageDrmFormatModifierPropertiesEXT");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageDrmFormatModifierPropertiesEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageDrmFormatModifierPropertiesEXT = unsafe extern "system" fn(device: crate::vk1_0::Device, image: crate::vk1_0::Image, p_properties: *mut crate::extensions::ext_image_drm_format_modifier::ImageDrmFormatModifierPropertiesEXT) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDrmFormatModifierPropertiesListEXT.html) · Structure"]
#[doc(alias = "VkDrmFormatModifierPropertiesListEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DrmFormatModifierPropertiesListEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub drm_format_modifier_count: u32,
    pub p_drm_format_modifier_properties: *mut crate::extensions::ext_image_drm_format_modifier::DrmFormatModifierPropertiesEXT,
}
impl Default for DrmFormatModifierPropertiesListEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::DRM_FORMAT_MODIFIER_PROPERTIES_LIST_EXT, p_next: std::ptr::null_mut(), drm_format_modifier_count: Default::default(), p_drm_format_modifier_properties: std::ptr::null_mut() }
    }
}
impl std::fmt::Debug for DrmFormatModifierPropertiesListEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DrmFormatModifierPropertiesListEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("drm_format_modifier_count", &self.drm_format_modifier_count).field("p_drm_format_modifier_properties", &self.p_drm_format_modifier_properties).finish()
    }
}
impl DrmFormatModifierPropertiesListEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> DrmFormatModifierPropertiesListEXTBuilder<'a> {
        DrmFormatModifierPropertiesListEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDrmFormatModifierPropertiesListEXT.html) · Builder of [`DrmFormatModifierPropertiesListEXT`]"]
#[repr(transparent)]
pub struct DrmFormatModifierPropertiesListEXTBuilder<'a>(DrmFormatModifierPropertiesListEXT, std::marker::PhantomData<&'a ()>);
impl<'a> DrmFormatModifierPropertiesListEXTBuilder<'a> {
    #[inline]
    pub fn new() -> DrmFormatModifierPropertiesListEXTBuilder<'a> {
        DrmFormatModifierPropertiesListEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn drm_format_modifier_properties(mut self, drm_format_modifier_properties: &'a mut [crate::extensions::ext_image_drm_format_modifier::DrmFormatModifierPropertiesEXTBuilder]) -> Self {
        self.0.p_drm_format_modifier_properties = drm_format_modifier_properties.as_ptr() as _;
        self.0.drm_format_modifier_count = drm_format_modifier_properties.len() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DrmFormatModifierPropertiesListEXT {
        self.0
    }
}
impl<'a> std::default::Default for DrmFormatModifierPropertiesListEXTBuilder<'a> {
    fn default() -> DrmFormatModifierPropertiesListEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DrmFormatModifierPropertiesListEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DrmFormatModifierPropertiesListEXTBuilder<'a> {
    type Target = DrmFormatModifierPropertiesListEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DrmFormatModifierPropertiesListEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDrmFormatModifierPropertiesEXT.html) · Structure"]
#[doc(alias = "VkDrmFormatModifierPropertiesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DrmFormatModifierPropertiesEXT {
    pub drm_format_modifier: u64,
    pub drm_format_modifier_plane_count: u32,
    pub drm_format_modifier_tiling_features: crate::vk1_0::FormatFeatureFlags,
}
impl Default for DrmFormatModifierPropertiesEXT {
    fn default() -> Self {
        Self { drm_format_modifier: Default::default(), drm_format_modifier_plane_count: Default::default(), drm_format_modifier_tiling_features: Default::default() }
    }
}
impl std::fmt::Debug for DrmFormatModifierPropertiesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DrmFormatModifierPropertiesEXT").field("drm_format_modifier", &self.drm_format_modifier).field("drm_format_modifier_plane_count", &self.drm_format_modifier_plane_count).field("drm_format_modifier_tiling_features", &self.drm_format_modifier_tiling_features).finish()
    }
}
impl DrmFormatModifierPropertiesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> DrmFormatModifierPropertiesEXTBuilder<'a> {
        DrmFormatModifierPropertiesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDrmFormatModifierPropertiesEXT.html) · Builder of [`DrmFormatModifierPropertiesEXT`]"]
#[repr(transparent)]
pub struct DrmFormatModifierPropertiesEXTBuilder<'a>(DrmFormatModifierPropertiesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> DrmFormatModifierPropertiesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> DrmFormatModifierPropertiesEXTBuilder<'a> {
        DrmFormatModifierPropertiesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn drm_format_modifier(mut self, drm_format_modifier: u64) -> Self {
        self.0.drm_format_modifier = drm_format_modifier as _;
        self
    }
    #[inline]
    pub fn drm_format_modifier_plane_count(mut self, drm_format_modifier_plane_count: u32) -> Self {
        self.0.drm_format_modifier_plane_count = drm_format_modifier_plane_count as _;
        self
    }
    #[inline]
    pub fn drm_format_modifier_tiling_features(mut self, drm_format_modifier_tiling_features: crate::vk1_0::FormatFeatureFlags) -> Self {
        self.0.drm_format_modifier_tiling_features = drm_format_modifier_tiling_features as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DrmFormatModifierPropertiesEXT {
        self.0
    }
}
impl<'a> std::default::Default for DrmFormatModifierPropertiesEXTBuilder<'a> {
    fn default() -> DrmFormatModifierPropertiesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DrmFormatModifierPropertiesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DrmFormatModifierPropertiesEXTBuilder<'a> {
    type Target = DrmFormatModifierPropertiesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DrmFormatModifierPropertiesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceImageDrmFormatModifierInfoEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceImageDrmFormatModifierInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceImageDrmFormatModifierInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub drm_format_modifier: u64,
    pub sharing_mode: crate::vk1_0::SharingMode,
    pub queue_family_index_count: u32,
    pub p_queue_family_indices: *const u32,
}
impl Default for PhysicalDeviceImageDrmFormatModifierInfoEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_IMAGE_DRM_FORMAT_MODIFIER_INFO_EXT, p_next: std::ptr::null(), drm_format_modifier: Default::default(), sharing_mode: Default::default(), queue_family_index_count: Default::default(), p_queue_family_indices: std::ptr::null() }
    }
}
impl std::fmt::Debug for PhysicalDeviceImageDrmFormatModifierInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceImageDrmFormatModifierInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("drm_format_modifier", &self.drm_format_modifier).field("sharing_mode", &self.sharing_mode).field("queue_family_index_count", &self.queue_family_index_count).field("p_queue_family_indices", &self.p_queue_family_indices).finish()
    }
}
impl PhysicalDeviceImageDrmFormatModifierInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceImageDrmFormatModifierInfoEXTBuilder<'a> {
        PhysicalDeviceImageDrmFormatModifierInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceImageDrmFormatModifierInfoEXT.html) · Builder of [`PhysicalDeviceImageDrmFormatModifierInfoEXT`]"]
#[repr(transparent)]
pub struct PhysicalDeviceImageDrmFormatModifierInfoEXTBuilder<'a>(PhysicalDeviceImageDrmFormatModifierInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceImageDrmFormatModifierInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceImageDrmFormatModifierInfoEXTBuilder<'a> {
        PhysicalDeviceImageDrmFormatModifierInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn drm_format_modifier(mut self, drm_format_modifier: u64) -> Self {
        self.0.drm_format_modifier = drm_format_modifier as _;
        self
    }
    #[inline]
    pub fn sharing_mode(mut self, sharing_mode: crate::vk1_0::SharingMode) -> Self {
        self.0.sharing_mode = sharing_mode as _;
        self
    }
    #[inline]
    pub fn queue_family_indices(mut self, queue_family_indices: &'a [u32]) -> Self {
        self.0.p_queue_family_indices = queue_family_indices.as_ptr() as _;
        self.0.queue_family_index_count = queue_family_indices.len() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceImageDrmFormatModifierInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceImageDrmFormatModifierInfoEXTBuilder<'a> {
    fn default() -> PhysicalDeviceImageDrmFormatModifierInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceImageDrmFormatModifierInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceImageDrmFormatModifierInfoEXTBuilder<'a> {
    type Target = PhysicalDeviceImageDrmFormatModifierInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceImageDrmFormatModifierInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageDrmFormatModifierListCreateInfoEXT.html) · Structure"]
#[doc(alias = "VkImageDrmFormatModifierListCreateInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImageDrmFormatModifierListCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub drm_format_modifier_count: u32,
    pub p_drm_format_modifiers: *const u64,
}
impl Default for ImageDrmFormatModifierListCreateInfoEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::IMAGE_DRM_FORMAT_MODIFIER_LIST_CREATE_INFO_EXT, p_next: std::ptr::null(), drm_format_modifier_count: Default::default(), p_drm_format_modifiers: std::ptr::null() }
    }
}
impl std::fmt::Debug for ImageDrmFormatModifierListCreateInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ImageDrmFormatModifierListCreateInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("drm_format_modifier_count", &self.drm_format_modifier_count).field("p_drm_format_modifiers", &self.p_drm_format_modifiers).finish()
    }
}
impl ImageDrmFormatModifierListCreateInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> ImageDrmFormatModifierListCreateInfoEXTBuilder<'a> {
        ImageDrmFormatModifierListCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageDrmFormatModifierListCreateInfoEXT.html) · Builder of [`ImageDrmFormatModifierListCreateInfoEXT`]"]
#[repr(transparent)]
pub struct ImageDrmFormatModifierListCreateInfoEXTBuilder<'a>(ImageDrmFormatModifierListCreateInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> ImageDrmFormatModifierListCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> ImageDrmFormatModifierListCreateInfoEXTBuilder<'a> {
        ImageDrmFormatModifierListCreateInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn drm_format_modifiers(mut self, drm_format_modifiers: &'a [u64]) -> Self {
        self.0.p_drm_format_modifiers = drm_format_modifiers.as_ptr() as _;
        self.0.drm_format_modifier_count = drm_format_modifiers.len() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ImageDrmFormatModifierListCreateInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for ImageDrmFormatModifierListCreateInfoEXTBuilder<'a> {
    fn default() -> ImageDrmFormatModifierListCreateInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ImageDrmFormatModifierListCreateInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ImageDrmFormatModifierListCreateInfoEXTBuilder<'a> {
    type Target = ImageDrmFormatModifierListCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ImageDrmFormatModifierListCreateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageDrmFormatModifierExplicitCreateInfoEXT.html) · Structure"]
#[doc(alias = "VkImageDrmFormatModifierExplicitCreateInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImageDrmFormatModifierExplicitCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub drm_format_modifier: u64,
    pub drm_format_modifier_plane_count: u32,
    pub p_plane_layouts: *const crate::vk1_0::SubresourceLayout,
}
impl Default for ImageDrmFormatModifierExplicitCreateInfoEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::IMAGE_DRM_FORMAT_MODIFIER_EXPLICIT_CREATE_INFO_EXT, p_next: std::ptr::null(), drm_format_modifier: Default::default(), drm_format_modifier_plane_count: Default::default(), p_plane_layouts: std::ptr::null() }
    }
}
impl std::fmt::Debug for ImageDrmFormatModifierExplicitCreateInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ImageDrmFormatModifierExplicitCreateInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("drm_format_modifier", &self.drm_format_modifier).field("drm_format_modifier_plane_count", &self.drm_format_modifier_plane_count).field("p_plane_layouts", &self.p_plane_layouts).finish()
    }
}
impl ImageDrmFormatModifierExplicitCreateInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> ImageDrmFormatModifierExplicitCreateInfoEXTBuilder<'a> {
        ImageDrmFormatModifierExplicitCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageDrmFormatModifierExplicitCreateInfoEXT.html) · Builder of [`ImageDrmFormatModifierExplicitCreateInfoEXT`]"]
#[repr(transparent)]
pub struct ImageDrmFormatModifierExplicitCreateInfoEXTBuilder<'a>(ImageDrmFormatModifierExplicitCreateInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> ImageDrmFormatModifierExplicitCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> ImageDrmFormatModifierExplicitCreateInfoEXTBuilder<'a> {
        ImageDrmFormatModifierExplicitCreateInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn drm_format_modifier(mut self, drm_format_modifier: u64) -> Self {
        self.0.drm_format_modifier = drm_format_modifier as _;
        self
    }
    #[inline]
    pub fn plane_layouts(mut self, plane_layouts: &'a [crate::vk1_0::SubresourceLayoutBuilder]) -> Self {
        self.0.p_plane_layouts = plane_layouts.as_ptr() as _;
        self.0.drm_format_modifier_plane_count = plane_layouts.len() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ImageDrmFormatModifierExplicitCreateInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for ImageDrmFormatModifierExplicitCreateInfoEXTBuilder<'a> {
    fn default() -> ImageDrmFormatModifierExplicitCreateInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ImageDrmFormatModifierExplicitCreateInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ImageDrmFormatModifierExplicitCreateInfoEXTBuilder<'a> {
    type Target = ImageDrmFormatModifierExplicitCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ImageDrmFormatModifierExplicitCreateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageDrmFormatModifierPropertiesEXT.html) · Structure"]
#[doc(alias = "VkImageDrmFormatModifierPropertiesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImageDrmFormatModifierPropertiesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub drm_format_modifier: u64,
}
impl Default for ImageDrmFormatModifierPropertiesEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT, p_next: std::ptr::null_mut(), drm_format_modifier: Default::default() }
    }
}
impl std::fmt::Debug for ImageDrmFormatModifierPropertiesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ImageDrmFormatModifierPropertiesEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("drm_format_modifier", &self.drm_format_modifier).finish()
    }
}
impl ImageDrmFormatModifierPropertiesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> ImageDrmFormatModifierPropertiesEXTBuilder<'a> {
        ImageDrmFormatModifierPropertiesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageDrmFormatModifierPropertiesEXT.html) · Builder of [`ImageDrmFormatModifierPropertiesEXT`]"]
#[repr(transparent)]
pub struct ImageDrmFormatModifierPropertiesEXTBuilder<'a>(ImageDrmFormatModifierPropertiesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> ImageDrmFormatModifierPropertiesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> ImageDrmFormatModifierPropertiesEXTBuilder<'a> {
        ImageDrmFormatModifierPropertiesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn drm_format_modifier(mut self, drm_format_modifier: u64) -> Self {
        self.0.drm_format_modifier = drm_format_modifier as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ImageDrmFormatModifierPropertiesEXT {
        self.0
    }
}
impl<'a> std::default::Default for ImageDrmFormatModifierPropertiesEXTBuilder<'a> {
    fn default() -> ImageDrmFormatModifierPropertiesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ImageDrmFormatModifierPropertiesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ImageDrmFormatModifierPropertiesEXTBuilder<'a> {
    type Target = ImageDrmFormatModifierPropertiesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ImageDrmFormatModifierPropertiesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::ext_image_drm_format_modifier`]"]
impl crate::DeviceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageDrmFormatModifierPropertiesEXT.html) · Function"]
    #[doc(alias = "vkGetImageDrmFormatModifierPropertiesEXT")]
    pub unsafe fn get_image_drm_format_modifier_properties_ext(&self, image: crate::vk1_0::Image, properties: Option<crate::extensions::ext_image_drm_format_modifier::ImageDrmFormatModifierPropertiesEXT>) -> crate::utils::VulkanResult<crate::extensions::ext_image_drm_format_modifier::ImageDrmFormatModifierPropertiesEXT> {
        let _function = self.get_image_drm_format_modifier_properties_ext.expect("tried to call a function that isn't loaded");
        let mut properties = match properties {
            Some(v) => v,
            None => Default::default(),
        };
        let _return = _function(self.handle, image as _, &mut properties);
        crate::utils::VulkanResult::new(_return, properties)
    }
}
