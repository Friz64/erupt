# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_EXT_image_drm_format_modifier.html)\n\n## Extends\n- [`ImageAspectFlagBits`](../../vk1_0/struct.ImageAspectFlagBits.html)\n- [`ImageTiling`](../../vk1_0/struct.ImageTiling.html)\n- [`Result`](../../vk1_0/struct.Result.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_IMAGE_DRM_FORMAT_MODIFIER_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_IMAGE_DRM_FORMAT_MODIFIER_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_image_drm_format_modifier");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageDrmFormatModifierPropertiesEXT.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageDrmFormatModifierPropertiesEXT = unsafe extern "system" fn ( device : crate :: vk1_0 :: Device , image : crate :: vk1_0 :: Image , p_properties : * mut crate :: extensions :: ext_image_drm_format_modifier :: ImageDrmFormatModifierPropertiesEXT , ) -> crate :: vk1_0 :: Result ;
#[doc = "Provides Device Commands for [`ExtImageDrmFormatModifierDeviceLoaderExt`](trait.ExtImageDrmFormatModifierDeviceLoaderExt.html)"]
pub struct ExtImageDrmFormatModifierDeviceCommands {
    pub get_image_drm_format_modifier_properties_ext: PFN_vkGetImageDrmFormatModifierPropertiesEXT,
}
impl ExtImageDrmFormatModifierDeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<ExtImageDrmFormatModifierDeviceCommands> {
        unsafe {
            Some(ExtImageDrmFormatModifierDeviceCommands {
                get_image_drm_format_modifier_properties_ext: std::mem::transmute(
                    loader.symbol("vkGetImageDrmFormatModifierPropertiesEXT")?,
                ),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`ExtImageDrmFormatModifierDeviceCommands`](struct.ExtImageDrmFormatModifierDeviceCommands.html)"]
pub trait ExtImageDrmFormatModifierDeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageDrmFormatModifierPropertiesEXT.html) · Device Command"]
    unsafe fn get_image_drm_format_modifier_properties_ext(
        &self,
        image: crate::vk1_0::Image,
        properties: Option<
            crate::extensions::ext_image_drm_format_modifier::ImageDrmFormatModifierPropertiesEXT,
        >,
    ) -> crate::utils::VulkanResult<
        crate::extensions::ext_image_drm_format_modifier::ImageDrmFormatModifierPropertiesEXT,
    >;
}
impl ExtImageDrmFormatModifierDeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageDrmFormatModifierPropertiesEXT.html) · Device Command"]
    unsafe fn get_image_drm_format_modifier_properties_ext(
        &self,
        image: crate::vk1_0::Image,
        properties: Option<
            crate::extensions::ext_image_drm_format_modifier::ImageDrmFormatModifierPropertiesEXT,
        >,
    ) -> crate::utils::VulkanResult<
        crate::extensions::ext_image_drm_format_modifier::ImageDrmFormatModifierPropertiesEXT,
    > {
        let function = self
            .ext_image_drm_format_modifier
            .as_ref()
            .expect("`ext_image_drm_format_modifier` not loaded")
            .get_image_drm_format_modifier_properties_ext;
        let mut properties = properties.unwrap_or_else(|| Default::default());
        let _val = function(self.handle, image, &mut properties);
        crate::utils::VulkanResult::new(_val, properties)
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageDrmFormatModifierPropertiesEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImageDrmFormatModifierPropertiesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub drm_format_modifier: u64,
}
impl ImageDrmFormatModifierPropertiesEXT {
    #[inline]
    pub fn builder<'a>(self) -> ImageDrmFormatModifierPropertiesEXTBuilder<'a> {
        ImageDrmFormatModifierPropertiesEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ImageDrmFormatModifierPropertiesEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ImageDrmFormatModifierPropertiesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("drm_format_modifier", &self.drm_format_modifier)
            .finish()
    }
}
impl Default for ImageDrmFormatModifierPropertiesEXT {
    fn default() -> ImageDrmFormatModifierPropertiesEXT {
        ImageDrmFormatModifierPropertiesEXT {
            s_type: crate::vk1_0::StructureType::IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            drm_format_modifier: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`ImageDrmFormatModifierPropertiesEXT`](struct.ImageDrmFormatModifierPropertiesEXT.html)"]
#[repr(transparent)]
pub struct ImageDrmFormatModifierPropertiesEXTBuilder<'a>(
    ImageDrmFormatModifierPropertiesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> ImageDrmFormatModifierPropertiesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> ImageDrmFormatModifierPropertiesEXTBuilder<'a> {
        ImageDrmFormatModifierPropertiesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn drm_format_modifier(mut self, drm_format_modifier: u64) -> Self {
        self.0.drm_format_modifier = drm_format_modifier;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> ImageDrmFormatModifierPropertiesEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for ImageDrmFormatModifierPropertiesEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDrmFormatModifierPropertiesListEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DrmFormatModifierPropertiesListEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub drm_format_modifier_count: u32,
    pub p_drm_format_modifier_properties:
        *mut crate::extensions::ext_image_drm_format_modifier::DrmFormatModifierPropertiesEXT,
}
impl DrmFormatModifierPropertiesListEXT {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByDrmFormatModifierPropertiesListEXT,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> DrmFormatModifierPropertiesListEXTBuilder<'a> {
        DrmFormatModifierPropertiesListEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for DrmFormatModifierPropertiesListEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("DrmFormatModifierPropertiesListEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("drm_format_modifier_count", &self.drm_format_modifier_count)
            .field(
                "p_drm_format_modifier_properties",
                &self.p_drm_format_modifier_properties,
            )
            .finish()
    }
}
impl Default for DrmFormatModifierPropertiesListEXT {
    fn default() -> DrmFormatModifierPropertiesListEXT {
        DrmFormatModifierPropertiesListEXT {
            s_type: crate::vk1_0::StructureType::DRM_FORMAT_MODIFIER_PROPERTIES_LIST_EXT,
            p_next: std::ptr::null_mut(),
            drm_format_modifier_count: Default::default(),
            p_drm_format_modifier_properties: std::ptr::null_mut(),
        }
    }
}
#[doc = "Used by [`DrmFormatModifierPropertiesListEXT::extend`](struct.DrmFormatModifierPropertiesListEXT.html#method.extend)"]
pub trait ExtendableByDrmFormatModifierPropertiesListEXT {}
impl ExtendableByDrmFormatModifierPropertiesListEXT for crate::vk1_1::FormatProperties2 {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`DrmFormatModifierPropertiesListEXT`](struct.DrmFormatModifierPropertiesListEXT.html)"]
#[repr(transparent)]
pub struct DrmFormatModifierPropertiesListEXTBuilder<'a>(
    DrmFormatModifierPropertiesListEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> DrmFormatModifierPropertiesListEXTBuilder<'a> {
    #[inline]
    pub fn new() -> DrmFormatModifierPropertiesListEXTBuilder<'a> {
        DrmFormatModifierPropertiesListEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn drm_format_modifier_properties(
        mut self,
        drm_format_modifier_properties : &'a mut [ crate :: extensions :: ext_image_drm_format_modifier :: DrmFormatModifierPropertiesEXTBuilder ],
    ) -> Self {
        self.0.drm_format_modifier_count = drm_format_modifier_properties.len() as _;
        self.0.p_drm_format_modifier_properties = drm_format_modifier_properties.as_mut_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> DrmFormatModifierPropertiesListEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for DrmFormatModifierPropertiesListEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DrmFormatModifierPropertiesEXT {
    pub drm_format_modifier: u64,
    pub drm_format_modifier_plane_count: u32,
    pub drm_format_modifier_tiling_features: crate::vk1_0::FormatFeatureFlags,
}
impl DrmFormatModifierPropertiesEXT {
    #[inline]
    pub fn builder<'a>(self) -> DrmFormatModifierPropertiesEXTBuilder<'a> {
        DrmFormatModifierPropertiesEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for DrmFormatModifierPropertiesEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("DrmFormatModifierPropertiesEXT")
            .field("drm_format_modifier", &self.drm_format_modifier)
            .field(
                "drm_format_modifier_plane_count",
                &self.drm_format_modifier_plane_count,
            )
            .field(
                "drm_format_modifier_tiling_features",
                &self.drm_format_modifier_tiling_features,
            )
            .finish()
    }
}
impl Default for DrmFormatModifierPropertiesEXT {
    fn default() -> DrmFormatModifierPropertiesEXT {
        DrmFormatModifierPropertiesEXT {
            drm_format_modifier: Default::default(),
            drm_format_modifier_plane_count: Default::default(),
            drm_format_modifier_tiling_features: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`DrmFormatModifierPropertiesEXT`](struct.DrmFormatModifierPropertiesEXT.html)"]
#[repr(transparent)]
pub struct DrmFormatModifierPropertiesEXTBuilder<'a>(
    DrmFormatModifierPropertiesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> DrmFormatModifierPropertiesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> DrmFormatModifierPropertiesEXTBuilder<'a> {
        DrmFormatModifierPropertiesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn drm_format_modifier(mut self, drm_format_modifier: u64) -> Self {
        self.0.drm_format_modifier = drm_format_modifier;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn drm_format_modifier_plane_count(mut self, drm_format_modifier_plane_count: u32) -> Self {
        self.0.drm_format_modifier_plane_count = drm_format_modifier_plane_count;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn drm_format_modifier_tiling_features(
        mut self,
        drm_format_modifier_tiling_features: crate::vk1_0::FormatFeatureFlags,
    ) -> Self {
        self.0.drm_format_modifier_tiling_features = drm_format_modifier_tiling_features;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> DrmFormatModifierPropertiesEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for DrmFormatModifierPropertiesEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
impl PhysicalDeviceImageDrmFormatModifierInfoEXT {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceImageDrmFormatModifierInfoEXT,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceImageDrmFormatModifierInfoEXTBuilder<'a> {
        PhysicalDeviceImageDrmFormatModifierInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceImageDrmFormatModifierInfoEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceImageDrmFormatModifierInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("drm_format_modifier", &self.drm_format_modifier)
            .field("sharing_mode", &self.sharing_mode)
            .field("queue_family_index_count", &self.queue_family_index_count)
            .field("p_queue_family_indices", &self.p_queue_family_indices)
            .finish()
    }
}
impl Default for PhysicalDeviceImageDrmFormatModifierInfoEXT {
    fn default() -> PhysicalDeviceImageDrmFormatModifierInfoEXT {
        PhysicalDeviceImageDrmFormatModifierInfoEXT {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_IMAGE_DRM_FORMAT_MODIFIER_INFO_EXT,
            p_next: std::ptr::null(),
            drm_format_modifier: Default::default(),
            sharing_mode: Default::default(),
            queue_family_index_count: Default::default(),
            p_queue_family_indices: std::ptr::null(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceImageDrmFormatModifierInfoEXT::extend`](struct.PhysicalDeviceImageDrmFormatModifierInfoEXT.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceImageDrmFormatModifierInfoEXT {}
impl ExtendableByPhysicalDeviceImageDrmFormatModifierInfoEXT
    for crate::vk1_1::PhysicalDeviceImageFormatInfo2
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceImageDrmFormatModifierInfoEXT`](struct.PhysicalDeviceImageDrmFormatModifierInfoEXT.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceImageDrmFormatModifierInfoEXTBuilder<'a>(
    PhysicalDeviceImageDrmFormatModifierInfoEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceImageDrmFormatModifierInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceImageDrmFormatModifierInfoEXTBuilder<'a> {
        PhysicalDeviceImageDrmFormatModifierInfoEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn drm_format_modifier(mut self, drm_format_modifier: u64) -> Self {
        self.0.drm_format_modifier = drm_format_modifier;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn sharing_mode(mut self, sharing_mode: crate::vk1_0::SharingMode) -> Self {
        self.0.sharing_mode = sharing_mode;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn queue_family_indices(mut self, queue_family_indices: &'a [u32]) -> Self {
        self.0.queue_family_index_count = queue_family_indices.len() as _;
        self.0.p_queue_family_indices = queue_family_indices.as_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceImageDrmFormatModifierInfoEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceImageDrmFormatModifierInfoEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImageDrmFormatModifierListCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub drm_format_modifier_count: u32,
    pub p_drm_format_modifiers: *const u64,
}
impl ImageDrmFormatModifierListCreateInfoEXT {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByImageDrmFormatModifierListCreateInfoEXT,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> ImageDrmFormatModifierListCreateInfoEXTBuilder<'a> {
        ImageDrmFormatModifierListCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ImageDrmFormatModifierListCreateInfoEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ImageDrmFormatModifierListCreateInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("drm_format_modifier_count", &self.drm_format_modifier_count)
            .field("p_drm_format_modifiers", &self.p_drm_format_modifiers)
            .finish()
    }
}
impl Default for ImageDrmFormatModifierListCreateInfoEXT {
    fn default() -> ImageDrmFormatModifierListCreateInfoEXT {
        ImageDrmFormatModifierListCreateInfoEXT {
            s_type: crate::vk1_0::StructureType::IMAGE_DRM_FORMAT_MODIFIER_LIST_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            drm_format_modifier_count: Default::default(),
            p_drm_format_modifiers: std::ptr::null(),
        }
    }
}
#[doc = "Used by [`ImageDrmFormatModifierListCreateInfoEXT::extend`](struct.ImageDrmFormatModifierListCreateInfoEXT.html#method.extend)"]
pub trait ExtendableByImageDrmFormatModifierListCreateInfoEXT {}
impl ExtendableByImageDrmFormatModifierListCreateInfoEXT for crate::vk1_0::ImageCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`ImageDrmFormatModifierListCreateInfoEXT`](struct.ImageDrmFormatModifierListCreateInfoEXT.html)"]
#[repr(transparent)]
pub struct ImageDrmFormatModifierListCreateInfoEXTBuilder<'a>(
    ImageDrmFormatModifierListCreateInfoEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> ImageDrmFormatModifierListCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> ImageDrmFormatModifierListCreateInfoEXTBuilder<'a> {
        ImageDrmFormatModifierListCreateInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn drm_format_modifiers(mut self, drm_format_modifiers: &'a [u64]) -> Self {
        self.0.drm_format_modifier_count = drm_format_modifiers.len() as _;
        self.0.p_drm_format_modifiers = drm_format_modifiers.as_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> ImageDrmFormatModifierListCreateInfoEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for ImageDrmFormatModifierListCreateInfoEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImageDrmFormatModifierExplicitCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub drm_format_modifier: u64,
    pub drm_format_modifier_plane_count: u32,
    pub p_plane_layouts: *const crate::vk1_0::SubresourceLayout,
}
impl ImageDrmFormatModifierExplicitCreateInfoEXT {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByImageDrmFormatModifierExplicitCreateInfoEXT,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> ImageDrmFormatModifierExplicitCreateInfoEXTBuilder<'a> {
        ImageDrmFormatModifierExplicitCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ImageDrmFormatModifierExplicitCreateInfoEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ImageDrmFormatModifierExplicitCreateInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("drm_format_modifier", &self.drm_format_modifier)
            .field(
                "drm_format_modifier_plane_count",
                &self.drm_format_modifier_plane_count,
            )
            .field("p_plane_layouts", &self.p_plane_layouts)
            .finish()
    }
}
impl Default for ImageDrmFormatModifierExplicitCreateInfoEXT {
    fn default() -> ImageDrmFormatModifierExplicitCreateInfoEXT {
        ImageDrmFormatModifierExplicitCreateInfoEXT {
            s_type: crate::vk1_0::StructureType::IMAGE_DRM_FORMAT_MODIFIER_EXPLICIT_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            drm_format_modifier: Default::default(),
            drm_format_modifier_plane_count: Default::default(),
            p_plane_layouts: std::ptr::null(),
        }
    }
}
#[doc = "Used by [`ImageDrmFormatModifierExplicitCreateInfoEXT::extend`](struct.ImageDrmFormatModifierExplicitCreateInfoEXT.html#method.extend)"]
pub trait ExtendableByImageDrmFormatModifierExplicitCreateInfoEXT {}
impl ExtendableByImageDrmFormatModifierExplicitCreateInfoEXT for crate::vk1_0::ImageCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`ImageDrmFormatModifierExplicitCreateInfoEXT`](struct.ImageDrmFormatModifierExplicitCreateInfoEXT.html)"]
#[repr(transparent)]
pub struct ImageDrmFormatModifierExplicitCreateInfoEXTBuilder<'a>(
    ImageDrmFormatModifierExplicitCreateInfoEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> ImageDrmFormatModifierExplicitCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> ImageDrmFormatModifierExplicitCreateInfoEXTBuilder<'a> {
        ImageDrmFormatModifierExplicitCreateInfoEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn drm_format_modifier(mut self, drm_format_modifier: u64) -> Self {
        self.0.drm_format_modifier = drm_format_modifier;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn plane_layouts(
        mut self,
        plane_layouts: &'a [crate::vk1_0::SubresourceLayoutBuilder],
    ) -> Self {
        self.0.drm_format_modifier_plane_count = plane_layouts.len() as _;
        self.0.p_plane_layouts = plane_layouts.as_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> ImageDrmFormatModifierExplicitCreateInfoEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for ImageDrmFormatModifierExplicitCreateInfoEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
