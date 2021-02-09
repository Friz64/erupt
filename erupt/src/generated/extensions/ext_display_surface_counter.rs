#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_DISPLAY_SURFACE_COUNTER_SPEC_VERSION")]
pub const EXT_DISPLAY_SURFACE_COUNTER_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_DISPLAY_SURFACE_COUNTER_EXTENSION_NAME")]
pub const EXT_DISPLAY_SURFACE_COUNTER_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_display_surface_counter");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_PHYSICAL_DEVICE_SURFACE_CAPABILITIES2_EXT: *const std::os::raw::c_char =
    crate::cstr!("vkGetPhysicalDeviceSurfaceCapabilities2EXT");
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceCounterFlagsEXT.html) · Bitmask of [`SurfaceCounterFlagBitsEXT`]"] # [doc (alias = "VkSurfaceCounterFlagsEXT")] # [derive (Default)] # [repr (transparent)] pub struct SurfaceCounterFlagsEXT : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; const VBLANK_EXT = SurfaceCounterFlagBitsEXT :: VBLANK_EXT . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceCounterFlagBitsEXT.html) · Bits enum of [`SurfaceCounterFlagsEXT`]"]
#[doc(alias = "VkSurfaceCounterFlagBitsEXT")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct SurfaceCounterFlagBitsEXT(pub u32);
impl SurfaceCounterFlagBitsEXT {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> SurfaceCounterFlagsEXT {
        SurfaceCounterFlagsEXT::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for SurfaceCounterFlagBitsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::VBLANK_EXT => "VBLANK_EXT",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::ext_display_surface_counter`]"]
impl SurfaceCounterFlagBitsEXT {
    pub const VBLANK_EXT: Self = Self(1);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilities2EXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT = unsafe extern "system" fn (physical_device : crate :: vk1_0 :: PhysicalDevice , surface : crate :: extensions :: khr_surface :: SurfaceKHR , p_surface_capabilities : * mut crate :: extensions :: ext_display_surface_counter :: SurfaceCapabilities2EXT) -> crate :: vk1_0 :: Result ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceCapabilities2EXT.html) · Structure"]
#[doc(alias = "VkSurfaceCapabilities2EXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SurfaceCapabilities2EXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub min_image_count: u32,
    pub max_image_count: u32,
    pub current_extent: crate::vk1_0::Extent2D,
    pub min_image_extent: crate::vk1_0::Extent2D,
    pub max_image_extent: crate::vk1_0::Extent2D,
    pub max_image_array_layers: u32,
    pub supported_transforms: crate::extensions::khr_surface::SurfaceTransformFlagsKHR,
    pub current_transform: crate::extensions::khr_surface::SurfaceTransformFlagBitsKHR,
    pub supported_composite_alpha: crate::extensions::khr_surface::CompositeAlphaFlagsKHR,
    pub supported_usage_flags: crate::vk1_0::ImageUsageFlags,
    pub supported_surface_counters:
        crate::extensions::ext_display_surface_counter::SurfaceCounterFlagsEXT,
}
impl Default for SurfaceCapabilities2EXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::SURFACE_CAPABILITIES_2_EXT,
            p_next: std::ptr::null_mut(),
            min_image_count: Default::default(),
            max_image_count: Default::default(),
            current_extent: Default::default(),
            min_image_extent: Default::default(),
            max_image_extent: Default::default(),
            max_image_array_layers: Default::default(),
            supported_transforms: Default::default(),
            current_transform: Default::default(),
            supported_composite_alpha: Default::default(),
            supported_usage_flags: Default::default(),
            supported_surface_counters: Default::default(),
        }
    }
}
impl std::fmt::Debug for SurfaceCapabilities2EXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SurfaceCapabilities2EXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("min_image_count", &self.min_image_count)
            .field("max_image_count", &self.max_image_count)
            .field("current_extent", &self.current_extent)
            .field("min_image_extent", &self.min_image_extent)
            .field("max_image_extent", &self.max_image_extent)
            .field("max_image_array_layers", &self.max_image_array_layers)
            .field("supported_transforms", &self.supported_transforms)
            .field("current_transform", &self.current_transform)
            .field("supported_composite_alpha", &self.supported_composite_alpha)
            .field("supported_usage_flags", &self.supported_usage_flags)
            .field(
                "supported_surface_counters",
                &self.supported_surface_counters,
            )
            .finish()
    }
}
impl SurfaceCapabilities2EXT {
    #[inline]
    pub fn into_builder<'a>(self) -> SurfaceCapabilities2EXTBuilder<'a> {
        SurfaceCapabilities2EXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceCapabilities2EXT.html) · Builder of [`SurfaceCapabilities2EXT`]"]
#[repr(transparent)]
pub struct SurfaceCapabilities2EXTBuilder<'a>(
    SurfaceCapabilities2EXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> SurfaceCapabilities2EXTBuilder<'a> {
    #[inline]
    pub fn new() -> SurfaceCapabilities2EXTBuilder<'a> {
        SurfaceCapabilities2EXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn min_image_count(mut self, min_image_count: u32) -> Self {
        self.0.min_image_count = min_image_count as _;
        self
    }
    #[inline]
    pub fn max_image_count(mut self, max_image_count: u32) -> Self {
        self.0.max_image_count = max_image_count as _;
        self
    }
    #[inline]
    pub fn current_extent(mut self, current_extent: crate::vk1_0::Extent2D) -> Self {
        self.0.current_extent = current_extent as _;
        self
    }
    #[inline]
    pub fn min_image_extent(mut self, min_image_extent: crate::vk1_0::Extent2D) -> Self {
        self.0.min_image_extent = min_image_extent as _;
        self
    }
    #[inline]
    pub fn max_image_extent(mut self, max_image_extent: crate::vk1_0::Extent2D) -> Self {
        self.0.max_image_extent = max_image_extent as _;
        self
    }
    #[inline]
    pub fn max_image_array_layers(mut self, max_image_array_layers: u32) -> Self {
        self.0.max_image_array_layers = max_image_array_layers as _;
        self
    }
    #[inline]
    pub fn supported_transforms(
        mut self,
        supported_transforms: crate::extensions::khr_surface::SurfaceTransformFlagsKHR,
    ) -> Self {
        self.0.supported_transforms = supported_transforms as _;
        self
    }
    #[inline]
    pub fn current_transform(
        mut self,
        current_transform: crate::extensions::khr_surface::SurfaceTransformFlagBitsKHR,
    ) -> Self {
        self.0.current_transform = current_transform as _;
        self
    }
    #[inline]
    pub fn supported_composite_alpha(
        mut self,
        supported_composite_alpha: crate::extensions::khr_surface::CompositeAlphaFlagsKHR,
    ) -> Self {
        self.0.supported_composite_alpha = supported_composite_alpha as _;
        self
    }
    #[inline]
    pub fn supported_usage_flags(
        mut self,
        supported_usage_flags: crate::vk1_0::ImageUsageFlags,
    ) -> Self {
        self.0.supported_usage_flags = supported_usage_flags as _;
        self
    }
    #[inline]
    pub fn supported_surface_counters(
        mut self,
        supported_surface_counters : crate :: extensions :: ext_display_surface_counter :: SurfaceCounterFlagsEXT,
    ) -> Self {
        self.0.supported_surface_counters = supported_surface_counters as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> SurfaceCapabilities2EXT {
        self.0
    }
}
impl<'a> std::default::Default for SurfaceCapabilities2EXTBuilder<'a> {
    fn default() -> SurfaceCapabilities2EXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for SurfaceCapabilities2EXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for SurfaceCapabilities2EXTBuilder<'a> {
    type Target = SurfaceCapabilities2EXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SurfaceCapabilities2EXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::ext_display_surface_counter`]"]
impl crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilities2EXT.html) · Function"]
    #[doc(alias = "vkGetPhysicalDeviceSurfaceCapabilities2EXT")]
    pub unsafe fn get_physical_device_surface_capabilities2_ext(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        surface: crate::extensions::khr_surface::SurfaceKHR,
        surface_capabilities: Option<
            crate::extensions::ext_display_surface_counter::SurfaceCapabilities2EXT,
        >,
    ) -> crate::utils::VulkanResult<
        crate::extensions::ext_display_surface_counter::SurfaceCapabilities2EXT,
    > {
        let _function = self
            .get_physical_device_surface_capabilities2_ext
            .expect("`get_physical_device_surface_capabilities2_ext` is not loaded");
        let mut surface_capabilities = match surface_capabilities {
            Some(v) => v,
            None => Default::default(),
        };
        let _return = _function(
            physical_device as _,
            surface as _,
            &mut surface_capabilities,
        );
        crate::utils::VulkanResult::new(_return, surface_capabilities)
    }
}
