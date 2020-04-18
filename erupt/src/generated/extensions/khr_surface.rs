# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_KHR_surface.html)\n\n## Extends\n- [`ObjectType`](../../vk1_0/struct.ObjectType.html)\n- [`Result`](../../vk1_0/struct.Result.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_SURFACE_SPEC_VERSION: u32 = 25;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_SURFACE_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_surface");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroySurfaceKHR.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroySurfaceKHR = unsafe extern "system" fn(
    instance: crate::vk1_0::Instance,
    surface: crate::extensions::khr_surface::SurfaceKHR,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfaceSupportKHR.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSurfaceSupportKHR =
    unsafe extern "system" fn(
        physical_device: crate::vk1_0::PhysicalDevice,
        queue_family_index: u32,
        surface: crate::extensions::khr_surface::SurfaceKHR,
        p_supported: *mut crate::vk1_0::Bool32,
    ) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilitiesKHR.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR =
    unsafe extern "system" fn(
        physical_device: crate::vk1_0::PhysicalDevice,
        surface: crate::extensions::khr_surface::SurfaceKHR,
        p_surface_capabilities: *mut crate::extensions::khr_surface::SurfaceCapabilitiesKHR,
    ) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfaceFormatsKHR.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSurfaceFormatsKHR =
    unsafe extern "system" fn(
        physical_device: crate::vk1_0::PhysicalDevice,
        surface: crate::extensions::khr_surface::SurfaceKHR,
        p_surface_format_count: *mut u32,
        p_surface_formats: *mut crate::extensions::khr_surface::SurfaceFormatKHR,
    ) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfacePresentModesKHR.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSurfacePresentModesKHR =
    unsafe extern "system" fn(
        physical_device: crate::vk1_0::PhysicalDevice,
        surface: crate::extensions::khr_surface::SurfaceKHR,
        p_present_mode_count: *mut u32,
        p_present_modes: *mut crate::extensions::khr_surface::PresentModeKHR,
    ) -> crate::vk1_0::Result;
#[doc = "Provides Instance Commands for [`KhrSurfaceInstanceLoaderExt`](trait.KhrSurfaceInstanceLoaderExt.html)"]
pub struct KhrSurfaceInstanceCommands {
    pub destroy_surface_khr: PFN_vkDestroySurfaceKHR,
    pub get_physical_device_surface_support_khr: PFN_vkGetPhysicalDeviceSurfaceSupportKHR,
    pub get_physical_device_surface_capabilities_khr: PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR,
    pub get_physical_device_surface_formats_khr: PFN_vkGetPhysicalDeviceSurfaceFormatsKHR,
    pub get_physical_device_surface_present_modes_khr:
        PFN_vkGetPhysicalDeviceSurfacePresentModesKHR,
}
impl KhrSurfaceInstanceCommands {
    #[inline]
    pub fn load(loader: &crate::InstanceLoader) -> Option<KhrSurfaceInstanceCommands> {
        unsafe {
            Some(KhrSurfaceInstanceCommands {
                destroy_surface_khr: std::mem::transmute(loader.symbol("vkDestroySurfaceKHR")?),
                get_physical_device_surface_support_khr: std::mem::transmute(
                    loader.symbol("vkGetPhysicalDeviceSurfaceSupportKHR")?,
                ),
                get_physical_device_surface_capabilities_khr: std::mem::transmute(
                    loader.symbol("vkGetPhysicalDeviceSurfaceCapabilitiesKHR")?,
                ),
                get_physical_device_surface_formats_khr: std::mem::transmute(
                    loader.symbol("vkGetPhysicalDeviceSurfaceFormatsKHR")?,
                ),
                get_physical_device_surface_present_modes_khr: std::mem::transmute(
                    loader.symbol("vkGetPhysicalDeviceSurfacePresentModesKHR")?,
                ),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`KhrSurfaceInstanceCommands`](struct.KhrSurfaceInstanceCommands.html)"]
pub trait KhrSurfaceInstanceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroySurfaceKHR.html) · Instance Command"]
    unsafe fn destroy_surface_khr(
        &self,
        surface: crate::extensions::khr_surface::SurfaceKHR,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfaceSupportKHR.html) · Instance Command"]
    unsafe fn get_physical_device_surface_support_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        queue_family_index: u32,
        surface: crate::extensions::khr_surface::SurfaceKHR,
        supported: Option<crate::vk1_0::Bool32>,
    ) -> crate::utils::VulkanResult<crate::vk1_0::Bool32>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilitiesKHR.html) · Instance Command"]
    unsafe fn get_physical_device_surface_capabilities_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        surface: crate::extensions::khr_surface::SurfaceKHR,
        surface_capabilities: Option<crate::extensions::khr_surface::SurfaceCapabilitiesKHR>,
    ) -> crate::utils::VulkanResult<crate::extensions::khr_surface::SurfaceCapabilitiesKHR>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfaceFormatsKHR.html) · Instance Command"]
    unsafe fn get_physical_device_surface_formats_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        surface: crate::extensions::khr_surface::SurfaceKHR,
        surface_format_count: Option<u32>,
    ) -> crate::utils::VulkanResult<Vec<crate::extensions::khr_surface::SurfaceFormatKHR>>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfacePresentModesKHR.html) · Instance Command"]
    unsafe fn get_physical_device_surface_present_modes_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        surface: crate::extensions::khr_surface::SurfaceKHR,
        present_mode_count: Option<u32>,
    ) -> crate::utils::VulkanResult<Vec<crate::extensions::khr_surface::PresentModeKHR>>;
}
impl KhrSurfaceInstanceLoaderExt for crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroySurfaceKHR.html) · Instance Command"]
    unsafe fn destroy_surface_khr(
        &self,
        surface: crate::extensions::khr_surface::SurfaceKHR,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
    ) -> () {
        let function = self
            .khr_surface
            .as_ref()
            .expect("`khr_surface` not loaded")
            .destroy_surface_khr;
        let _val = function(
            self.handle,
            surface,
            if let Some(allocator) = allocator {
                allocator
            } else {
                std::ptr::null()
            },
        );
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfaceSupportKHR.html) · Instance Command"]
    unsafe fn get_physical_device_surface_support_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        queue_family_index: u32,
        surface: crate::extensions::khr_surface::SurfaceKHR,
        supported: Option<crate::vk1_0::Bool32>,
    ) -> crate::utils::VulkanResult<crate::vk1_0::Bool32> {
        let function = self
            .khr_surface
            .as_ref()
            .expect("`khr_surface` not loaded")
            .get_physical_device_surface_support_khr;
        let mut supported = supported.unwrap_or_else(|| Default::default());
        let _val = function(physical_device, queue_family_index, surface, &mut supported);
        crate::utils::VulkanResult::new(_val, supported)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilitiesKHR.html) · Instance Command"]
    unsafe fn get_physical_device_surface_capabilities_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        surface: crate::extensions::khr_surface::SurfaceKHR,
        surface_capabilities: Option<crate::extensions::khr_surface::SurfaceCapabilitiesKHR>,
    ) -> crate::utils::VulkanResult<crate::extensions::khr_surface::SurfaceCapabilitiesKHR> {
        let function = self
            .khr_surface
            .as_ref()
            .expect("`khr_surface` not loaded")
            .get_physical_device_surface_capabilities_khr;
        let mut surface_capabilities = surface_capabilities.unwrap_or_else(|| Default::default());
        let _val = function(physical_device, surface, &mut surface_capabilities);
        crate::utils::VulkanResult::new(_val, surface_capabilities)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfaceFormatsKHR.html) · Instance Command"]
    unsafe fn get_physical_device_surface_formats_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        surface: crate::extensions::khr_surface::SurfaceKHR,
        surface_format_count: Option<u32>,
    ) -> crate::utils::VulkanResult<Vec<crate::extensions::khr_surface::SurfaceFormatKHR>> {
        let function = self
            .khr_surface
            .as_ref()
            .expect("`khr_surface` not loaded")
            .get_physical_device_surface_formats_khr;
        let mut surface_format_count = surface_format_count.unwrap_or_else(|| {
            let mut val = Default::default();
            function(physical_device, surface, &mut val, std::ptr::null_mut());
            val
        });
        let mut surface_formats = vec![Default::default(); surface_format_count as _];
        let _val = function(
            physical_device,
            surface,
            &mut surface_format_count,
            surface_formats.as_mut_ptr(),
        );
        crate::utils::VulkanResult::new(_val, surface_formats)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfacePresentModesKHR.html) · Instance Command"]
    unsafe fn get_physical_device_surface_present_modes_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        surface: crate::extensions::khr_surface::SurfaceKHR,
        present_mode_count: Option<u32>,
    ) -> crate::utils::VulkanResult<Vec<crate::extensions::khr_surface::PresentModeKHR>> {
        let function = self
            .khr_surface
            .as_ref()
            .expect("`khr_surface` not loaded")
            .get_physical_device_surface_present_modes_khr;
        let mut present_mode_count = present_mode_count.unwrap_or_else(|| {
            let mut val = Default::default();
            function(physical_device, surface, &mut val, std::ptr::null_mut());
            val
        });
        let mut present_modes = vec![Default::default(); present_mode_count as _];
        let _val = function(
            physical_device,
            surface,
            &mut present_mode_count,
            present_modes.as_mut_ptr(),
        );
        crate::utils::VulkanResult::new(_val, present_modes)
    }
}
crate :: non_dispatchable_handle ! ( SurfaceKHR , SURFACE_KHR , doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceKHR.html) · Non-dispatchable Handle" ) ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceCapabilitiesKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SurfaceCapabilitiesKHR {
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
}
impl SurfaceCapabilitiesKHR {
    #[inline]
    pub fn builder<'a>(self) -> SurfaceCapabilitiesKHRBuilder<'a> {
        SurfaceCapabilitiesKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for SurfaceCapabilitiesKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("SurfaceCapabilitiesKHR")
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
            .finish()
    }
}
impl Default for SurfaceCapabilitiesKHR {
    fn default() -> SurfaceCapabilitiesKHR {
        SurfaceCapabilitiesKHR {
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
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`SurfaceCapabilitiesKHR`](struct.SurfaceCapabilitiesKHR.html)"]
#[repr(transparent)]
pub struct SurfaceCapabilitiesKHRBuilder<'a>(
    SurfaceCapabilitiesKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> SurfaceCapabilitiesKHRBuilder<'a> {
    #[inline]
    pub fn new() -> SurfaceCapabilitiesKHRBuilder<'a> {
        SurfaceCapabilitiesKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn min_image_count(mut self, min_image_count: u32) -> Self {
        self.0.min_image_count = min_image_count;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_image_count(mut self, max_image_count: u32) -> Self {
        self.0.max_image_count = max_image_count;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn current_extent(mut self, current_extent: crate::vk1_0::Extent2D) -> Self {
        self.0.current_extent = current_extent;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn min_image_extent(mut self, min_image_extent: crate::vk1_0::Extent2D) -> Self {
        self.0.min_image_extent = min_image_extent;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_image_extent(mut self, max_image_extent: crate::vk1_0::Extent2D) -> Self {
        self.0.max_image_extent = max_image_extent;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_image_array_layers(mut self, max_image_array_layers: u32) -> Self {
        self.0.max_image_array_layers = max_image_array_layers;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn supported_transforms(
        mut self,
        supported_transforms: crate::extensions::khr_surface::SurfaceTransformFlagsKHR,
    ) -> Self {
        self.0.supported_transforms = supported_transforms;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn current_transform(
        mut self,
        current_transform: crate::extensions::khr_surface::SurfaceTransformFlagBitsKHR,
    ) -> Self {
        self.0.current_transform = current_transform;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn supported_composite_alpha(
        mut self,
        supported_composite_alpha: crate::extensions::khr_surface::CompositeAlphaFlagsKHR,
    ) -> Self {
        self.0.supported_composite_alpha = supported_composite_alpha;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn supported_usage_flags(
        mut self,
        supported_usage_flags: crate::vk1_0::ImageUsageFlags,
    ) -> Self {
        self.0.supported_usage_flags = supported_usage_flags;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> SurfaceCapabilitiesKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for SurfaceCapabilitiesKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for SurfaceCapabilitiesKHRBuilder<'a> {
    type Target = SurfaceCapabilitiesKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SurfaceCapabilitiesKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceTransformFlagBitsKHR.html) · Flag Bits of [`SurfaceTransformFlagsKHR`](struct.SurfaceTransformFlagsKHR.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct SurfaceTransformFlagBitsKHR(pub u32);
impl SurfaceTransformFlagBitsKHR {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> SurfaceTransformFlagsKHR {
        SurfaceTransformFlagsKHR::from_bits_truncate(self.0)
    }
}
#[doc = "[Part of `extensions::khr_surface`](../../extensions/khr_surface/index.html)"]
impl SurfaceTransformFlagBitsKHR {
    pub const IDENTITY_KHR: Self = Self(0x00000001);
    pub const ROTATE_90_KHR: Self = Self(0x00000002);
    pub const ROTATE_180_KHR: Self = Self(0x00000004);
    pub const ROTATE_270_KHR: Self = Self(0x00000008);
    pub const HORIZONTAL_MIRROR_KHR: Self = Self(0x00000010);
    pub const HORIZONTAL_MIRROR_ROTATE_90_KHR: Self = Self(0x00000020);
    pub const HORIZONTAL_MIRROR_ROTATE_180_KHR: Self = Self(0x00000040);
    pub const HORIZONTAL_MIRROR_ROTATE_270_KHR: Self = Self(0x00000080);
    pub const INHERIT_KHR: Self = Self(0x00000100);
}
impl std::fmt::Debug for SurfaceTransformFlagBitsKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::IDENTITY_KHR => "IDENTITY_KHR",
            &Self::ROTATE_90_KHR => "ROTATE_90_KHR",
            &Self::ROTATE_180_KHR => "ROTATE_180_KHR",
            &Self::ROTATE_270_KHR => "ROTATE_270_KHR",
            &Self::HORIZONTAL_MIRROR_KHR => "HORIZONTAL_MIRROR_KHR",
            &Self::HORIZONTAL_MIRROR_ROTATE_90_KHR => "HORIZONTAL_MIRROR_ROTATE_90_KHR",
            &Self::HORIZONTAL_MIRROR_ROTATE_180_KHR => "HORIZONTAL_MIRROR_ROTATE_180_KHR",
            &Self::HORIZONTAL_MIRROR_ROTATE_270_KHR => "HORIZONTAL_MIRROR_ROTATE_270_KHR",
            &Self::INHERIT_KHR => "INHERIT_KHR",
            _ => "(unknown)",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceTransformFlagsKHR.html) · Flags of [`SurfaceTransformFlagBitsKHR`](struct.SurfaceTransformFlagBitsKHR.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct SurfaceTransformFlagsKHR : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; const IDENTITY_KHR = SurfaceTransformFlagBitsKHR :: IDENTITY_KHR . 0 ; const ROTATE_90_KHR = SurfaceTransformFlagBitsKHR :: ROTATE_90_KHR . 0 ; const ROTATE_180_KHR = SurfaceTransformFlagBitsKHR :: ROTATE_180_KHR . 0 ; const ROTATE_270_KHR = SurfaceTransformFlagBitsKHR :: ROTATE_270_KHR . 0 ; const HORIZONTAL_MIRROR_KHR = SurfaceTransformFlagBitsKHR :: HORIZONTAL_MIRROR_KHR . 0 ; const HORIZONTAL_MIRROR_ROTATE_90_KHR = SurfaceTransformFlagBitsKHR :: HORIZONTAL_MIRROR_ROTATE_90_KHR . 0 ; const HORIZONTAL_MIRROR_ROTATE_180_KHR = SurfaceTransformFlagBitsKHR :: HORIZONTAL_MIRROR_ROTATE_180_KHR . 0 ; const HORIZONTAL_MIRROR_ROTATE_270_KHR = SurfaceTransformFlagBitsKHR :: HORIZONTAL_MIRROR_ROTATE_270_KHR . 0 ; const INHERIT_KHR = SurfaceTransformFlagBitsKHR :: INHERIT_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCompositeAlphaFlagBitsKHR.html) · Flag Bits of [`CompositeAlphaFlagsKHR`](struct.CompositeAlphaFlagsKHR.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct CompositeAlphaFlagBitsKHR(pub u32);
impl CompositeAlphaFlagBitsKHR {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> CompositeAlphaFlagsKHR {
        CompositeAlphaFlagsKHR::from_bits_truncate(self.0)
    }
}
#[doc = "[Part of `extensions::khr_surface`](../../extensions/khr_surface/index.html)"]
impl CompositeAlphaFlagBitsKHR {
    pub const OPAQUE_KHR: Self = Self(0x00000001);
    pub const PRE_MULTIPLIED_KHR: Self = Self(0x00000002);
    pub const POST_MULTIPLIED_KHR: Self = Self(0x00000004);
    pub const INHERIT_KHR: Self = Self(0x00000008);
}
impl std::fmt::Debug for CompositeAlphaFlagBitsKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::OPAQUE_KHR => "OPAQUE_KHR",
            &Self::PRE_MULTIPLIED_KHR => "PRE_MULTIPLIED_KHR",
            &Self::POST_MULTIPLIED_KHR => "POST_MULTIPLIED_KHR",
            &Self::INHERIT_KHR => "INHERIT_KHR",
            _ => "(unknown)",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCompositeAlphaFlagsKHR.html) · Flags of [`CompositeAlphaFlagBitsKHR`](struct.CompositeAlphaFlagBitsKHR.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct CompositeAlphaFlagsKHR : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; const OPAQUE_KHR = CompositeAlphaFlagBitsKHR :: OPAQUE_KHR . 0 ; const PRE_MULTIPLIED_KHR = CompositeAlphaFlagBitsKHR :: PRE_MULTIPLIED_KHR . 0 ; const POST_MULTIPLIED_KHR = CompositeAlphaFlagBitsKHR :: POST_MULTIPLIED_KHR . 0 ; const INHERIT_KHR = CompositeAlphaFlagBitsKHR :: INHERIT_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceFormatKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SurfaceFormatKHR {
    pub format: crate::vk1_0::Format,
    pub color_space: crate::extensions::khr_surface::ColorSpaceKHR,
}
impl SurfaceFormatKHR {
    #[inline]
    pub fn builder<'a>(self) -> SurfaceFormatKHRBuilder<'a> {
        SurfaceFormatKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for SurfaceFormatKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("SurfaceFormatKHR")
            .field("format", &self.format)
            .field("color_space", &self.color_space)
            .finish()
    }
}
impl Default for SurfaceFormatKHR {
    fn default() -> SurfaceFormatKHR {
        SurfaceFormatKHR {
            format: Default::default(),
            color_space: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`SurfaceFormatKHR`](struct.SurfaceFormatKHR.html)"]
#[repr(transparent)]
pub struct SurfaceFormatKHRBuilder<'a>(SurfaceFormatKHR, std::marker::PhantomData<&'a ()>);
impl<'a> SurfaceFormatKHRBuilder<'a> {
    #[inline]
    pub fn new() -> SurfaceFormatKHRBuilder<'a> {
        SurfaceFormatKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn format(mut self, format: crate::vk1_0::Format) -> Self {
        self.0.format = format;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn color_space(
        mut self,
        color_space: crate::extensions::khr_surface::ColorSpaceKHR,
    ) -> Self {
        self.0.color_space = color_space;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> SurfaceFormatKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for SurfaceFormatKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for SurfaceFormatKHRBuilder<'a> {
    type Target = SurfaceFormatKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SurfaceFormatKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkColorSpaceKHR.html) · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ColorSpaceKHR(pub i32);
#[doc = "[Part of `extensions::khr_surface`](../../extensions/khr_surface/index.html)"]
impl ColorSpaceKHR {
    pub const SRGB_NONLINEAR_KHR: Self = Self(0);
    pub const COLORSPACE_SRGB_NONLINEAR_KHR: Self = Self::SRGB_NONLINEAR_KHR;
}
#[doc = "[Part of `extensions::amd_display_native_hdr`](../../extensions/amd_display_native_hdr/index.html)"]
impl ColorSpaceKHR {
    pub const DISPLAY_NATIVE_AMD: Self = Self(1000213000);
}
#[doc = "[Part of `extensions::ext_swapchain_colorspace`](../../extensions/ext_swapchain_colorspace/index.html)"]
impl ColorSpaceKHR {
    pub const DISPLAY_P3_NONLINEAR_EXT: Self = Self(1000104001);
    pub const EXTENDED_SRGB_LINEAR_EXT: Self = Self(1000104002);
    pub const DISPLAY_P3_LINEAR_EXT: Self = Self(1000104003);
    pub const DCI_P3_NONLINEAR_EXT: Self = Self(1000104004);
    pub const BT709_LINEAR_EXT: Self = Self(1000104005);
    pub const BT709_NONLINEAR_EXT: Self = Self(1000104006);
    pub const BT2020_LINEAR_EXT: Self = Self(1000104007);
    pub const HDR10_ST2084_EXT: Self = Self(1000104008);
    pub const DOLBYVISION_EXT: Self = Self(1000104009);
    pub const HDR10_HLG_EXT: Self = Self(1000104010);
    pub const ADOBERGB_LINEAR_EXT: Self = Self(1000104011);
    pub const ADOBERGB_NONLINEAR_EXT: Self = Self(1000104012);
    pub const PASS_THROUGH_EXT: Self = Self(1000104013);
    pub const EXTENDED_SRGB_NONLINEAR_EXT: Self = Self(1000104014);
    pub const DCI_P3_LINEAR_EXT: Self = Self::DISPLAY_P3_LINEAR_EXT;
}
impl std::fmt::Debug for ColorSpaceKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::SRGB_NONLINEAR_KHR => "SRGB_NONLINEAR_KHR",
            &Self::DISPLAY_NATIVE_AMD => "DISPLAY_NATIVE_AMD",
            &Self::DISPLAY_P3_NONLINEAR_EXT => "DISPLAY_P3_NONLINEAR_EXT",
            &Self::EXTENDED_SRGB_LINEAR_EXT => "EXTENDED_SRGB_LINEAR_EXT",
            &Self::DISPLAY_P3_LINEAR_EXT => "DISPLAY_P3_LINEAR_EXT",
            &Self::DCI_P3_NONLINEAR_EXT => "DCI_P3_NONLINEAR_EXT",
            &Self::BT709_LINEAR_EXT => "BT709_LINEAR_EXT",
            &Self::BT709_NONLINEAR_EXT => "BT709_NONLINEAR_EXT",
            &Self::BT2020_LINEAR_EXT => "BT2020_LINEAR_EXT",
            &Self::HDR10_ST2084_EXT => "HDR10_ST2084_EXT",
            &Self::DOLBYVISION_EXT => "DOLBYVISION_EXT",
            &Self::HDR10_HLG_EXT => "HDR10_HLG_EXT",
            &Self::ADOBERGB_LINEAR_EXT => "ADOBERGB_LINEAR_EXT",
            &Self::ADOBERGB_NONLINEAR_EXT => "ADOBERGB_NONLINEAR_EXT",
            &Self::PASS_THROUGH_EXT => "PASS_THROUGH_EXT",
            &Self::EXTENDED_SRGB_NONLINEAR_EXT => "EXTENDED_SRGB_NONLINEAR_EXT",
            _ => "(unknown)",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPresentModeKHR.html) · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PresentModeKHR(pub i32);
#[doc = "[Part of `extensions::khr_surface`](../../extensions/khr_surface/index.html)"]
impl PresentModeKHR {
    pub const IMMEDIATE_KHR: Self = Self(0);
    pub const MAILBOX_KHR: Self = Self(1);
    pub const FIFO_KHR: Self = Self(2);
    pub const FIFO_RELAXED_KHR: Self = Self(3);
}
#[doc = "[Part of `extensions::khr_shared_presentable_image`](../../extensions/khr_shared_presentable_image/index.html)"]
impl PresentModeKHR {
    pub const SHARED_DEMAND_REFRESH_KHR: Self = Self(1000111000);
    pub const SHARED_CONTINUOUS_REFRESH_KHR: Self = Self(1000111001);
}
impl std::fmt::Debug for PresentModeKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::IMMEDIATE_KHR => "IMMEDIATE_KHR",
            &Self::MAILBOX_KHR => "MAILBOX_KHR",
            &Self::FIFO_KHR => "FIFO_KHR",
            &Self::FIFO_RELAXED_KHR => "FIFO_RELAXED_KHR",
            &Self::SHARED_DEMAND_REFRESH_KHR => "SHARED_DEMAND_REFRESH_KHR",
            &Self::SHARED_CONTINUOUS_REFRESH_KHR => "SHARED_CONTINUOUS_REFRESH_KHR",
            _ => "(unknown)",
        })
    }
}
