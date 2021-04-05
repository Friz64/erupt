#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_SWAPCHAIN_SPEC_VERSION")]
pub const KHR_SWAPCHAIN_SPEC_VERSION: u32 = 70;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_SWAPCHAIN_EXTENSION_NAME")]
pub const KHR_SWAPCHAIN_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_swapchain");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_SWAPCHAIN_KHR: *const std::os::raw::c_char = crate::cstr!("vkCreateSwapchainKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_DESTROY_SWAPCHAIN_KHR: *const std::os::raw::c_char = crate::cstr!("vkDestroySwapchainKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_SWAPCHAIN_IMAGES_KHR: *const std::os::raw::c_char = crate::cstr!("vkGetSwapchainImagesKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_ACQUIRE_NEXT_IMAGE_KHR: *const std::os::raw::c_char = crate::cstr!("vkAcquireNextImageKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_QUEUE_PRESENT_KHR: *const std::os::raw::c_char = crate::cstr!("vkQueuePresentKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_DEVICE_GROUP_PRESENT_CAPABILITIES_KHR: *const std::os::raw::c_char = crate::cstr!("vkGetDeviceGroupPresentCapabilitiesKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_DEVICE_GROUP_SURFACE_PRESENT_MODES_KHR: *const std::os::raw::c_char = crate::cstr!("vkGetDeviceGroupSurfacePresentModesKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_ACQUIRE_NEXT_IMAGE2_KHR: *const std::os::raw::c_char = crate::cstr!("vkAcquireNextImage2KHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_PHYSICAL_DEVICE_PRESENT_RECTANGLES_KHR: *const std::os::raw::c_char = crate::cstr!("vkGetPhysicalDevicePresentRectanglesKHR");
crate::non_dispatchable_handle!(
    SwapchainKHR,
    SWAPCHAIN_KHR,
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSwapchainKHR.html) · Non-dispatchable Handle",
    "VkSwapchainKHR"
);
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceGroupPresentModeFlagsKHR.html) · Bitmask of [`DeviceGroupPresentModeFlagBitsKHR`]"] # [doc (alias = "VkDeviceGroupPresentModeFlagsKHR")] # [derive (Default)] # [repr (transparent)] pub struct DeviceGroupPresentModeFlagsKHR : u32 { const LOCAL_KHR = DeviceGroupPresentModeFlagBitsKHR :: LOCAL_KHR . 0 ; const REMOTE_KHR = DeviceGroupPresentModeFlagBitsKHR :: REMOTE_KHR . 0 ; const SUM_KHR = DeviceGroupPresentModeFlagBitsKHR :: SUM_KHR . 0 ; const LOCAL_MULTI_DEVICE_KHR = DeviceGroupPresentModeFlagBitsKHR :: LOCAL_MULTI_DEVICE_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceGroupPresentModeFlagBitsKHR.html) · Bits enum of [`DeviceGroupPresentModeFlagsKHR`]"]
#[doc(alias = "VkDeviceGroupPresentModeFlagBitsKHR")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct DeviceGroupPresentModeFlagBitsKHR(pub u32);
impl DeviceGroupPresentModeFlagBitsKHR {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> DeviceGroupPresentModeFlagsKHR {
        DeviceGroupPresentModeFlagsKHR::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for DeviceGroupPresentModeFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::LOCAL_KHR => "LOCAL_KHR",
            &Self::REMOTE_KHR => "REMOTE_KHR",
            &Self::SUM_KHR => "SUM_KHR",
            &Self::LOCAL_MULTI_DEVICE_KHR => "LOCAL_MULTI_DEVICE_KHR",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::khr_swapchain`]"]
impl DeviceGroupPresentModeFlagBitsKHR {
    pub const LOCAL_KHR: Self = Self(1);
    pub const REMOTE_KHR: Self = Self(2);
    pub const SUM_KHR: Self = Self(4);
    pub const LOCAL_MULTI_DEVICE_KHR: Self = Self(8);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSwapchainCreateFlagsKHR.html) · Bitmask of [`SwapchainCreateFlagBitsKHR`]"] # [doc (alias = "VkSwapchainCreateFlagsKHR")] # [derive (Default)] # [repr (transparent)] pub struct SwapchainCreateFlagsKHR : u32 { const SPLIT_INSTANCE_BIND_REGIONS_KHR = SwapchainCreateFlagBitsKHR :: SPLIT_INSTANCE_BIND_REGIONS_KHR . 0 ; const PROTECTED_KHR = SwapchainCreateFlagBitsKHR :: PROTECTED_KHR . 0 ; const MUTABLE_FORMAT_KHR = SwapchainCreateFlagBitsKHR :: MUTABLE_FORMAT_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSwapchainCreateFlagBitsKHR.html) · Bits enum of [`SwapchainCreateFlagsKHR`]"]
#[doc(alias = "VkSwapchainCreateFlagBitsKHR")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct SwapchainCreateFlagBitsKHR(pub u32);
impl SwapchainCreateFlagBitsKHR {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> SwapchainCreateFlagsKHR {
        SwapchainCreateFlagsKHR::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for SwapchainCreateFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::SPLIT_INSTANCE_BIND_REGIONS_KHR => "SPLIT_INSTANCE_BIND_REGIONS_KHR",
            &Self::PROTECTED_KHR => "PROTECTED_KHR",
            &Self::MUTABLE_FORMAT_KHR => "MUTABLE_FORMAT_KHR",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::khr_swapchain`]"]
impl SwapchainCreateFlagBitsKHR {
    pub const SPLIT_INSTANCE_BIND_REGIONS_KHR: Self = Self(1);
    pub const PROTECTED_KHR: Self = Self(2);
}
#[doc = "Provided by [`crate::extensions::khr_swapchain_mutable_format`]"]
impl SwapchainCreateFlagBitsKHR {
    pub const MUTABLE_FORMAT_KHR: Self = Self(4);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateSwapchainKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateSwapchainKHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_create_info: *const crate::extensions::khr_swapchain::SwapchainCreateInfoKHR,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
    p_swapchain: *mut crate::extensions::khr_swapchain::SwapchainKHR,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroySwapchainKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroySwapchainKHR =
    unsafe extern "system" fn(device: crate::vk1_0::Device, swapchain: crate::extensions::khr_swapchain::SwapchainKHR, p_allocator: *const crate::vk1_0::AllocationCallbacks) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSwapchainImagesKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetSwapchainImagesKHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
    p_swapchain_image_count: *mut u32,
    p_swapchain_images: *mut crate::vk1_0::Image,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAcquireNextImageKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkAcquireNextImageKHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
    timeout: u64,
    semaphore: crate::vk1_0::Semaphore,
    fence: crate::vk1_0::Fence,
    p_image_index: *mut u32,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueuePresentKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkQueuePresentKHR = unsafe extern "system" fn(queue: crate::vk1_0::Queue, p_present_info: *const crate::extensions::khr_swapchain::PresentInfoKHR) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceGroupPresentCapabilitiesKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceGroupPresentCapabilitiesKHR =
    unsafe extern "system" fn(device: crate::vk1_0::Device, p_device_group_present_capabilities: *mut crate::extensions::khr_swapchain::DeviceGroupPresentCapabilitiesKHR) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceGroupSurfacePresentModesKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceGroupSurfacePresentModesKHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    surface: crate::extensions::khr_surface::SurfaceKHR,
    p_modes: *mut crate::extensions::khr_swapchain::DeviceGroupPresentModeFlagsKHR,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAcquireNextImage2KHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkAcquireNextImage2KHR =
    unsafe extern "system" fn(device: crate::vk1_0::Device, p_acquire_info: *const crate::extensions::khr_swapchain::AcquireNextImageInfoKHR, p_image_index: *mut u32) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDevicePresentRectanglesKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDevicePresentRectanglesKHR = unsafe extern "system" fn(
    physical_device: crate::vk1_0::PhysicalDevice,
    surface: crate::extensions::khr_surface::SurfaceKHR,
    p_rect_count: *mut u32,
    p_rects: *mut crate::vk1_0::Rect2D,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSwapchainCreateInfoKHR.html) · Structure"]
#[doc(alias = "VkSwapchainCreateInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SwapchainCreateInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::khr_swapchain::SwapchainCreateFlagsKHR,
    pub surface: crate::extensions::khr_surface::SurfaceKHR,
    pub min_image_count: u32,
    pub image_format: crate::vk1_0::Format,
    pub image_color_space: crate::extensions::khr_surface::ColorSpaceKHR,
    pub image_extent: crate::vk1_0::Extent2D,
    pub image_array_layers: u32,
    pub image_usage: crate::vk1_0::ImageUsageFlags,
    pub image_sharing_mode: crate::vk1_0::SharingMode,
    pub queue_family_index_count: u32,
    pub p_queue_family_indices: *const u32,
    pub pre_transform: crate::extensions::khr_surface::SurfaceTransformFlagBitsKHR,
    pub composite_alpha: crate::extensions::khr_surface::CompositeAlphaFlagBitsKHR,
    pub present_mode: crate::extensions::khr_surface::PresentModeKHR,
    pub clipped: crate::vk1_0::Bool32,
    pub old_swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
}
impl Default for SwapchainCreateInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::SWAPCHAIN_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            flags: Default::default(),
            surface: Default::default(),
            min_image_count: Default::default(),
            image_format: Default::default(),
            image_color_space: Default::default(),
            image_extent: Default::default(),
            image_array_layers: Default::default(),
            image_usage: Default::default(),
            image_sharing_mode: Default::default(),
            queue_family_index_count: Default::default(),
            p_queue_family_indices: std::ptr::null(),
            pre_transform: Default::default(),
            composite_alpha: Default::default(),
            present_mode: Default::default(),
            clipped: Default::default(),
            old_swapchain: Default::default(),
        }
    }
}
impl std::fmt::Debug for SwapchainCreateInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SwapchainCreateInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("surface", &self.surface)
            .field("min_image_count", &self.min_image_count)
            .field("image_format", &self.image_format)
            .field("image_color_space", &self.image_color_space)
            .field("image_extent", &self.image_extent)
            .field("image_array_layers", &self.image_array_layers)
            .field("image_usage", &self.image_usage)
            .field("image_sharing_mode", &self.image_sharing_mode)
            .field("queue_family_index_count", &self.queue_family_index_count)
            .field("p_queue_family_indices", &self.p_queue_family_indices)
            .field("pre_transform", &self.pre_transform)
            .field("composite_alpha", &self.composite_alpha)
            .field("present_mode", &self.present_mode)
            .field("clipped", &(self.clipped != 0))
            .field("old_swapchain", &self.old_swapchain)
            .finish()
    }
}
impl SwapchainCreateInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> SwapchainCreateInfoKHRBuilder<'a> {
        SwapchainCreateInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_display_control::SwapchainCounterCreateInfoEXT> for SwapchainCreateInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_display_control::SwapchainCounterCreateInfoEXTBuilder<'_>> for SwapchainCreateInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::khr_swapchain::DeviceGroupSwapchainCreateInfoKHR> for SwapchainCreateInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::khr_swapchain::DeviceGroupSwapchainCreateInfoKHRBuilder<'_>> for SwapchainCreateInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::amd_display_native_hdr::SwapchainDisplayNativeHdrCreateInfoAMD> for SwapchainCreateInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::amd_display_native_hdr::SwapchainDisplayNativeHdrCreateInfoAMDBuilder<'_>> for SwapchainCreateInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::ImageFormatListCreateInfo> for SwapchainCreateInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::vk1_2::ImageFormatListCreateInfoBuilder<'_>> for SwapchainCreateInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_full_screen_exclusive::SurfaceFullScreenExclusiveInfoEXT> for SwapchainCreateInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_full_screen_exclusive::SurfaceFullScreenExclusiveInfoEXTBuilder<'_>> for SwapchainCreateInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_full_screen_exclusive::SurfaceFullScreenExclusiveWin32InfoEXT> for SwapchainCreateInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ext_full_screen_exclusive::SurfaceFullScreenExclusiveWin32InfoEXTBuilder<'_>> for SwapchainCreateInfoKHRBuilder<'a> {}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSwapchainCreateInfoKHR.html) · Builder of [`SwapchainCreateInfoKHR`]"]
#[repr(transparent)]
pub struct SwapchainCreateInfoKHRBuilder<'a>(SwapchainCreateInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> SwapchainCreateInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> SwapchainCreateInfoKHRBuilder<'a> {
        SwapchainCreateInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::khr_swapchain::SwapchainCreateFlagsKHR) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn surface(mut self, surface: crate::extensions::khr_surface::SurfaceKHR) -> Self {
        self.0.surface = surface as _;
        self
    }
    #[inline]
    pub fn min_image_count(mut self, min_image_count: u32) -> Self {
        self.0.min_image_count = min_image_count as _;
        self
    }
    #[inline]
    pub fn image_format(mut self, image_format: crate::vk1_0::Format) -> Self {
        self.0.image_format = image_format as _;
        self
    }
    #[inline]
    pub fn image_color_space(mut self, image_color_space: crate::extensions::khr_surface::ColorSpaceKHR) -> Self {
        self.0.image_color_space = image_color_space as _;
        self
    }
    #[inline]
    pub fn image_extent(mut self, image_extent: crate::vk1_0::Extent2D) -> Self {
        self.0.image_extent = image_extent as _;
        self
    }
    #[inline]
    pub fn image_array_layers(mut self, image_array_layers: u32) -> Self {
        self.0.image_array_layers = image_array_layers as _;
        self
    }
    #[inline]
    pub fn image_usage(mut self, image_usage: crate::vk1_0::ImageUsageFlags) -> Self {
        self.0.image_usage = image_usage as _;
        self
    }
    #[inline]
    pub fn image_sharing_mode(mut self, image_sharing_mode: crate::vk1_0::SharingMode) -> Self {
        self.0.image_sharing_mode = image_sharing_mode as _;
        self
    }
    #[inline]
    pub fn queue_family_indices(mut self, queue_family_indices: &'a [u32]) -> Self {
        self.0.p_queue_family_indices = queue_family_indices.as_ptr() as _;
        self.0.queue_family_index_count = queue_family_indices.len() as _;
        self
    }
    #[inline]
    pub fn pre_transform(mut self, pre_transform: crate::extensions::khr_surface::SurfaceTransformFlagBitsKHR) -> Self {
        self.0.pre_transform = pre_transform as _;
        self
    }
    #[inline]
    pub fn composite_alpha(mut self, composite_alpha: crate::extensions::khr_surface::CompositeAlphaFlagBitsKHR) -> Self {
        self.0.composite_alpha = composite_alpha as _;
        self
    }
    #[inline]
    pub fn present_mode(mut self, present_mode: crate::extensions::khr_surface::PresentModeKHR) -> Self {
        self.0.present_mode = present_mode as _;
        self
    }
    #[inline]
    pub fn clipped(mut self, clipped: bool) -> Self {
        self.0.clipped = clipped as _;
        self
    }
    #[inline]
    pub fn old_swapchain(mut self, old_swapchain: crate::extensions::khr_swapchain::SwapchainKHR) -> Self {
        self.0.old_swapchain = old_swapchain as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> SwapchainCreateInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for SwapchainCreateInfoKHRBuilder<'a> {
    fn default() -> SwapchainCreateInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for SwapchainCreateInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for SwapchainCreateInfoKHRBuilder<'a> {
    type Target = SwapchainCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SwapchainCreateInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<SwapchainCreateInfoKHR> for SwapchainCreateInfoKHRBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPresentInfoKHR.html) · Structure"]
#[doc(alias = "VkPresentInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PresentInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub wait_semaphore_count: u32,
    pub p_wait_semaphores: *const crate::vk1_0::Semaphore,
    pub swapchain_count: u32,
    pub p_swapchains: *const crate::extensions::khr_swapchain::SwapchainKHR,
    pub p_image_indices: *const u32,
    pub p_results: *mut crate::vk1_0::Result,
}
impl Default for PresentInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PRESENT_INFO_KHR,
            p_next: std::ptr::null(),
            wait_semaphore_count: Default::default(),
            p_wait_semaphores: std::ptr::null(),
            swapchain_count: Default::default(),
            p_swapchains: std::ptr::null(),
            p_image_indices: std::ptr::null(),
            p_results: std::ptr::null_mut(),
        }
    }
}
impl std::fmt::Debug for PresentInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PresentInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("wait_semaphore_count", &self.wait_semaphore_count)
            .field("p_wait_semaphores", &self.p_wait_semaphores)
            .field("swapchain_count", &self.swapchain_count)
            .field("p_swapchains", &self.p_swapchains)
            .field("p_image_indices", &self.p_image_indices)
            .field("p_results", &self.p_results)
            .finish()
    }
}
impl PresentInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> PresentInfoKHRBuilder<'a> {
        PresentInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::khr_display_swapchain::DisplayPresentInfoKHR> for PresentInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::khr_display_swapchain::DisplayPresentInfoKHRBuilder<'_>> for PresentInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::khr_incremental_present::PresentRegionsKHR> for PresentInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::khr_incremental_present::PresentRegionsKHRBuilder<'_>> for PresentInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::khr_swapchain::DeviceGroupPresentInfoKHR> for PresentInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::khr_swapchain::DeviceGroupPresentInfoKHRBuilder<'_>> for PresentInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::google_display_timing::PresentTimesInfoGOOGLE> for PresentInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::google_display_timing::PresentTimesInfoGOOGLEBuilder<'_>> for PresentInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ggp_frame_token::PresentFrameTokenGGP> for PresentInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::ggp_frame_token::PresentFrameTokenGGPBuilder<'_>> for PresentInfoKHRBuilder<'a> {}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPresentInfoKHR.html) · Builder of [`PresentInfoKHR`]"]
#[repr(transparent)]
pub struct PresentInfoKHRBuilder<'a>(PresentInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> PresentInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PresentInfoKHRBuilder<'a> {
        PresentInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn wait_semaphores(mut self, wait_semaphores: &'a [crate::vk1_0::Semaphore]) -> Self {
        self.0.p_wait_semaphores = wait_semaphores.as_ptr() as _;
        self.0.wait_semaphore_count = wait_semaphores.len() as _;
        self
    }
    #[inline]
    pub fn swapchains(mut self, swapchains: &'a [crate::extensions::khr_swapchain::SwapchainKHR]) -> Self {
        self.0.p_swapchains = swapchains.as_ptr() as _;
        self.0.swapchain_count = swapchains.len() as _;
        self
    }
    #[inline]
    pub fn image_indices(mut self, image_indices: &'a [u32]) -> Self {
        self.0.p_image_indices = image_indices.as_ptr() as _;
        self.0.swapchain_count = image_indices.len() as _;
        self
    }
    #[inline]
    pub fn results(mut self, results: &'a mut [crate::vk1_0::Result]) -> Self {
        self.0.p_results = results.as_ptr() as _;
        self.0.swapchain_count = results.len() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PresentInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for PresentInfoKHRBuilder<'a> {
    fn default() -> PresentInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PresentInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PresentInfoKHRBuilder<'a> {
    type Target = PresentInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PresentInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<PresentInfoKHR> for PresentInfoKHRBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceGroupPresentCapabilitiesKHR.html) · Structure"]
#[doc(alias = "VkDeviceGroupPresentCapabilitiesKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DeviceGroupPresentCapabilitiesKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub present_mask: [u32; 32],
    pub modes: crate::extensions::khr_swapchain::DeviceGroupPresentModeFlagsKHR,
}
impl Default for DeviceGroupPresentCapabilitiesKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::DEVICE_GROUP_PRESENT_CAPABILITIES_KHR,
            p_next: std::ptr::null(),
            present_mask: unsafe { std::mem::zeroed() },
            modes: Default::default(),
        }
    }
}
impl std::fmt::Debug for DeviceGroupPresentCapabilitiesKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DeviceGroupPresentCapabilitiesKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("present_mask", &self.present_mask)
            .field("modes", &self.modes)
            .finish()
    }
}
impl DeviceGroupPresentCapabilitiesKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> DeviceGroupPresentCapabilitiesKHRBuilder<'a> {
        DeviceGroupPresentCapabilitiesKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceGroupPresentCapabilitiesKHR.html) · Builder of [`DeviceGroupPresentCapabilitiesKHR`]"]
#[repr(transparent)]
pub struct DeviceGroupPresentCapabilitiesKHRBuilder<'a>(DeviceGroupPresentCapabilitiesKHR, std::marker::PhantomData<&'a ()>);
impl<'a> DeviceGroupPresentCapabilitiesKHRBuilder<'a> {
    #[inline]
    pub fn new() -> DeviceGroupPresentCapabilitiesKHRBuilder<'a> {
        DeviceGroupPresentCapabilitiesKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn present_mask(mut self, present_mask: [u32; 32]) -> Self {
        self.0.present_mask = present_mask as _;
        self
    }
    #[inline]
    pub fn modes(mut self, modes: crate::extensions::khr_swapchain::DeviceGroupPresentModeFlagsKHR) -> Self {
        self.0.modes = modes as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DeviceGroupPresentCapabilitiesKHR {
        self.0
    }
}
impl<'a> std::default::Default for DeviceGroupPresentCapabilitiesKHRBuilder<'a> {
    fn default() -> DeviceGroupPresentCapabilitiesKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DeviceGroupPresentCapabilitiesKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DeviceGroupPresentCapabilitiesKHRBuilder<'a> {
    type Target = DeviceGroupPresentCapabilitiesKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DeviceGroupPresentCapabilitiesKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<DeviceGroupPresentCapabilitiesKHR> for DeviceGroupPresentCapabilitiesKHRBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageSwapchainCreateInfoKHR.html) · Structure"]
#[doc(alias = "VkImageSwapchainCreateInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImageSwapchainCreateInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
}
impl Default for ImageSwapchainCreateInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::IMAGE_SWAPCHAIN_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            swapchain: Default::default(),
        }
    }
}
impl std::fmt::Debug for ImageSwapchainCreateInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ImageSwapchainCreateInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("swapchain", &self.swapchain)
            .finish()
    }
}
impl ImageSwapchainCreateInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> ImageSwapchainCreateInfoKHRBuilder<'a> {
        ImageSwapchainCreateInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageSwapchainCreateInfoKHR.html) · Builder of [`ImageSwapchainCreateInfoKHR`]"]
#[repr(transparent)]
pub struct ImageSwapchainCreateInfoKHRBuilder<'a>(ImageSwapchainCreateInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> ImageSwapchainCreateInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> ImageSwapchainCreateInfoKHRBuilder<'a> {
        ImageSwapchainCreateInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn swapchain(mut self, swapchain: crate::extensions::khr_swapchain::SwapchainKHR) -> Self {
        self.0.swapchain = swapchain as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ImageSwapchainCreateInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for ImageSwapchainCreateInfoKHRBuilder<'a> {
    fn default() -> ImageSwapchainCreateInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ImageSwapchainCreateInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ImageSwapchainCreateInfoKHRBuilder<'a> {
    type Target = ImageSwapchainCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ImageSwapchainCreateInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<ImageSwapchainCreateInfoKHR> for ImageSwapchainCreateInfoKHRBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindImageMemorySwapchainInfoKHR.html) · Structure"]
#[doc(alias = "VkBindImageMemorySwapchainInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BindImageMemorySwapchainInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
    pub image_index: u32,
}
impl Default for BindImageMemorySwapchainInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR,
            p_next: std::ptr::null(),
            swapchain: Default::default(),
            image_index: Default::default(),
        }
    }
}
impl std::fmt::Debug for BindImageMemorySwapchainInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BindImageMemorySwapchainInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("swapchain", &self.swapchain)
            .field("image_index", &self.image_index)
            .finish()
    }
}
impl BindImageMemorySwapchainInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> BindImageMemorySwapchainInfoKHRBuilder<'a> {
        BindImageMemorySwapchainInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindImageMemorySwapchainInfoKHR.html) · Builder of [`BindImageMemorySwapchainInfoKHR`]"]
#[repr(transparent)]
pub struct BindImageMemorySwapchainInfoKHRBuilder<'a>(BindImageMemorySwapchainInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> BindImageMemorySwapchainInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> BindImageMemorySwapchainInfoKHRBuilder<'a> {
        BindImageMemorySwapchainInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn swapchain(mut self, swapchain: crate::extensions::khr_swapchain::SwapchainKHR) -> Self {
        self.0.swapchain = swapchain as _;
        self
    }
    #[inline]
    pub fn image_index(mut self, image_index: u32) -> Self {
        self.0.image_index = image_index as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> BindImageMemorySwapchainInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for BindImageMemorySwapchainInfoKHRBuilder<'a> {
    fn default() -> BindImageMemorySwapchainInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for BindImageMemorySwapchainInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for BindImageMemorySwapchainInfoKHRBuilder<'a> {
    type Target = BindImageMemorySwapchainInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for BindImageMemorySwapchainInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<BindImageMemorySwapchainInfoKHR> for BindImageMemorySwapchainInfoKHRBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAcquireNextImageInfoKHR.html) · Structure"]
#[doc(alias = "VkAcquireNextImageInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AcquireNextImageInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
    pub timeout: u64,
    pub semaphore: crate::vk1_0::Semaphore,
    pub fence: crate::vk1_0::Fence,
    pub device_mask: u32,
}
impl Default for AcquireNextImageInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::ACQUIRE_NEXT_IMAGE_INFO_KHR,
            p_next: std::ptr::null(),
            swapchain: Default::default(),
            timeout: Default::default(),
            semaphore: Default::default(),
            fence: Default::default(),
            device_mask: Default::default(),
        }
    }
}
impl std::fmt::Debug for AcquireNextImageInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AcquireNextImageInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("swapchain", &self.swapchain)
            .field("timeout", &self.timeout)
            .field("semaphore", &self.semaphore)
            .field("fence", &self.fence)
            .field("device_mask", &self.device_mask)
            .finish()
    }
}
impl AcquireNextImageInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> AcquireNextImageInfoKHRBuilder<'a> {
        AcquireNextImageInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAcquireNextImageInfoKHR.html) · Builder of [`AcquireNextImageInfoKHR`]"]
#[repr(transparent)]
pub struct AcquireNextImageInfoKHRBuilder<'a>(AcquireNextImageInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> AcquireNextImageInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> AcquireNextImageInfoKHRBuilder<'a> {
        AcquireNextImageInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn swapchain(mut self, swapchain: crate::extensions::khr_swapchain::SwapchainKHR) -> Self {
        self.0.swapchain = swapchain as _;
        self
    }
    #[inline]
    pub fn timeout(mut self, timeout: u64) -> Self {
        self.0.timeout = timeout as _;
        self
    }
    #[inline]
    pub fn semaphore(mut self, semaphore: crate::vk1_0::Semaphore) -> Self {
        self.0.semaphore = semaphore as _;
        self
    }
    #[inline]
    pub fn fence(mut self, fence: crate::vk1_0::Fence) -> Self {
        self.0.fence = fence as _;
        self
    }
    #[inline]
    pub fn device_mask(mut self, device_mask: u32) -> Self {
        self.0.device_mask = device_mask as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> AcquireNextImageInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for AcquireNextImageInfoKHRBuilder<'a> {
    fn default() -> AcquireNextImageInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for AcquireNextImageInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for AcquireNextImageInfoKHRBuilder<'a> {
    type Target = AcquireNextImageInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for AcquireNextImageInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<AcquireNextImageInfoKHR> for AcquireNextImageInfoKHRBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceGroupPresentInfoKHR.html) · Structure"]
#[doc(alias = "VkDeviceGroupPresentInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DeviceGroupPresentInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub swapchain_count: u32,
    pub p_device_masks: *const u32,
    pub mode: crate::extensions::khr_swapchain::DeviceGroupPresentModeFlagBitsKHR,
}
impl Default for DeviceGroupPresentInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::DEVICE_GROUP_PRESENT_INFO_KHR,
            p_next: std::ptr::null(),
            swapchain_count: Default::default(),
            p_device_masks: std::ptr::null(),
            mode: Default::default(),
        }
    }
}
impl std::fmt::Debug for DeviceGroupPresentInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DeviceGroupPresentInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("swapchain_count", &self.swapchain_count)
            .field("p_device_masks", &self.p_device_masks)
            .field("mode", &self.mode)
            .finish()
    }
}
impl DeviceGroupPresentInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> DeviceGroupPresentInfoKHRBuilder<'a> {
        DeviceGroupPresentInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceGroupPresentInfoKHR.html) · Builder of [`DeviceGroupPresentInfoKHR`]"]
#[repr(transparent)]
pub struct DeviceGroupPresentInfoKHRBuilder<'a>(DeviceGroupPresentInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> DeviceGroupPresentInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> DeviceGroupPresentInfoKHRBuilder<'a> {
        DeviceGroupPresentInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn device_masks(mut self, device_masks: &'a [u32]) -> Self {
        self.0.p_device_masks = device_masks.as_ptr() as _;
        self.0.swapchain_count = device_masks.len() as _;
        self
    }
    #[inline]
    pub fn mode(mut self, mode: crate::extensions::khr_swapchain::DeviceGroupPresentModeFlagBitsKHR) -> Self {
        self.0.mode = mode as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DeviceGroupPresentInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for DeviceGroupPresentInfoKHRBuilder<'a> {
    fn default() -> DeviceGroupPresentInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DeviceGroupPresentInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DeviceGroupPresentInfoKHRBuilder<'a> {
    type Target = DeviceGroupPresentInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DeviceGroupPresentInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<DeviceGroupPresentInfoKHR> for DeviceGroupPresentInfoKHRBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceGroupSwapchainCreateInfoKHR.html) · Structure"]
#[doc(alias = "VkDeviceGroupSwapchainCreateInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DeviceGroupSwapchainCreateInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub modes: crate::extensions::khr_swapchain::DeviceGroupPresentModeFlagsKHR,
}
impl Default for DeviceGroupSwapchainCreateInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            modes: Default::default(),
        }
    }
}
impl std::fmt::Debug for DeviceGroupSwapchainCreateInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DeviceGroupSwapchainCreateInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("modes", &self.modes)
            .finish()
    }
}
impl DeviceGroupSwapchainCreateInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> DeviceGroupSwapchainCreateInfoKHRBuilder<'a> {
        DeviceGroupSwapchainCreateInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceGroupSwapchainCreateInfoKHR.html) · Builder of [`DeviceGroupSwapchainCreateInfoKHR`]"]
#[repr(transparent)]
pub struct DeviceGroupSwapchainCreateInfoKHRBuilder<'a>(DeviceGroupSwapchainCreateInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> DeviceGroupSwapchainCreateInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> DeviceGroupSwapchainCreateInfoKHRBuilder<'a> {
        DeviceGroupSwapchainCreateInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn modes(mut self, modes: crate::extensions::khr_swapchain::DeviceGroupPresentModeFlagsKHR) -> Self {
        self.0.modes = modes as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DeviceGroupSwapchainCreateInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for DeviceGroupSwapchainCreateInfoKHRBuilder<'a> {
    fn default() -> DeviceGroupSwapchainCreateInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DeviceGroupSwapchainCreateInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DeviceGroupSwapchainCreateInfoKHRBuilder<'a> {
    type Target = DeviceGroupSwapchainCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DeviceGroupSwapchainCreateInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<DeviceGroupSwapchainCreateInfoKHR> for DeviceGroupSwapchainCreateInfoKHRBuilder<'_> {}
#[doc = "Provided by [`crate::extensions::khr_swapchain`]"]
impl crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateSwapchainKHR.html) · Function"]
    #[doc(alias = "vkCreateSwapchainKHR")]
    pub unsafe fn create_swapchain_khr(
        &self,
        create_info: &crate::extensions::khr_swapchain::SwapchainCreateInfoKHR,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
    ) -> crate::utils::VulkanResult<crate::extensions::khr_swapchain::SwapchainKHR> {
        let _function = self.create_swapchain_khr.expect("`create_swapchain_khr` is not loaded");
        let mut swapchain = Default::default();
        let _return = _function(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut swapchain,
        );
        crate::utils::VulkanResult::new(_return, swapchain)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroySwapchainKHR.html) · Function"]
    #[doc(alias = "vkDestroySwapchainKHR")]
    pub unsafe fn destroy_swapchain_khr(&self, swapchain: Option<crate::extensions::khr_swapchain::SwapchainKHR>, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> () {
        let _function = self.destroy_swapchain_khr.expect("`destroy_swapchain_khr` is not loaded");
        let _return = _function(
            self.handle,
            match swapchain {
                Some(v) => v,
                None => Default::default(),
            },
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSwapchainImagesKHR.html) · Function"]
    #[doc(alias = "vkGetSwapchainImagesKHR")]
    pub unsafe fn get_swapchain_images_khr(
        &self,
        swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
        swapchain_image_count: Option<u32>,
    ) -> crate::utils::VulkanResult<Vec<crate::vk1_0::Image>> {
        let _function = self.get_swapchain_images_khr.expect("`get_swapchain_images_khr` is not loaded");
        let mut swapchain_image_count = match swapchain_image_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                _function(self.handle, swapchain as _, &mut v, std::ptr::null_mut());
                v
            }
        };
        let mut swapchain_images = vec![Default::default(); swapchain_image_count as _];
        let _return = _function(self.handle, swapchain as _, &mut swapchain_image_count, swapchain_images.as_mut_ptr());
        crate::utils::VulkanResult::new(_return, swapchain_images)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAcquireNextImageKHR.html) · Function"]
    #[doc(alias = "vkAcquireNextImageKHR")]
    pub unsafe fn acquire_next_image_khr(
        &self,
        swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
        timeout: u64,
        semaphore: Option<crate::vk1_0::Semaphore>,
        fence: Option<crate::vk1_0::Fence>,
    ) -> crate::utils::VulkanResult<u32> {
        let _function = self.acquire_next_image_khr.expect("`acquire_next_image_khr` is not loaded");
        let mut image_index = Default::default();
        let _return = _function(
            self.handle,
            swapchain as _,
            timeout as _,
            match semaphore {
                Some(v) => v,
                None => Default::default(),
            },
            match fence {
                Some(v) => v,
                None => Default::default(),
            },
            &mut image_index,
        );
        crate::utils::VulkanResult::new(_return, image_index)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueuePresentKHR.html) · Function"]
    #[doc(alias = "vkQueuePresentKHR")]
    pub unsafe fn queue_present_khr(&self, queue: crate::vk1_0::Queue, present_info: &crate::extensions::khr_swapchain::PresentInfoKHR) -> crate::utils::VulkanResult<()> {
        let _function = self.queue_present_khr.expect("`queue_present_khr` is not loaded");
        let _return = _function(queue as _, present_info as _);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceGroupPresentCapabilitiesKHR.html) · Function"]
    #[doc(alias = "vkGetDeviceGroupPresentCapabilitiesKHR")]
    pub unsafe fn get_device_group_present_capabilities_khr(&self) -> crate::utils::VulkanResult<crate::extensions::khr_swapchain::DeviceGroupPresentCapabilitiesKHR> {
        let _function = self.get_device_group_present_capabilities_khr.expect("`get_device_group_present_capabilities_khr` is not loaded");
        let mut device_group_present_capabilities = Default::default();
        let _return = _function(self.handle, &mut device_group_present_capabilities);
        crate::utils::VulkanResult::new(_return, device_group_present_capabilities)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceGroupSurfacePresentModesKHR.html) · Function"]
    #[doc(alias = "vkGetDeviceGroupSurfacePresentModesKHR")]
    pub unsafe fn get_device_group_surface_present_modes_khr(
        &self,
        surface: crate::extensions::khr_surface::SurfaceKHR,
        modes: &mut crate::extensions::khr_swapchain::DeviceGroupPresentModeFlagsKHR,
    ) -> crate::utils::VulkanResult<()> {
        let _function = self.get_device_group_surface_present_modes_khr.expect("`get_device_group_surface_present_modes_khr` is not loaded");
        let _return = _function(self.handle, surface as _, modes as _);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAcquireNextImage2KHR.html) · Function"]
    #[doc(alias = "vkAcquireNextImage2KHR")]
    pub unsafe fn acquire_next_image2_khr(&self, acquire_info: &crate::extensions::khr_swapchain::AcquireNextImageInfoKHR) -> crate::utils::VulkanResult<u32> {
        let _function = self.acquire_next_image2_khr.expect("`acquire_next_image2_khr` is not loaded");
        let mut image_index = Default::default();
        let _return = _function(self.handle, acquire_info as _, &mut image_index);
        crate::utils::VulkanResult::new(_return, image_index)
    }
}
#[doc = "Provided by [`crate::extensions::khr_swapchain`]"]
impl crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDevicePresentRectanglesKHR.html) · Function"]
    #[doc(alias = "vkGetPhysicalDevicePresentRectanglesKHR")]
    pub unsafe fn get_physical_device_present_rectangles_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        surface: crate::extensions::khr_surface::SurfaceKHR,
        rect_count: Option<u32>,
    ) -> crate::utils::VulkanResult<Vec<crate::vk1_0::Rect2D>> {
        let _function = self.get_physical_device_present_rectangles_khr.expect("`get_physical_device_present_rectangles_khr` is not loaded");
        let mut rect_count = match rect_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                _function(physical_device as _, surface as _, &mut v, std::ptr::null_mut());
                v
            }
        };
        let mut rects = vec![Default::default(); rect_count as _];
        let _return = _function(physical_device as _, surface as _, &mut rect_count, rects.as_mut_ptr());
        crate::utils::VulkanResult::new(_return, rects)
    }
}
