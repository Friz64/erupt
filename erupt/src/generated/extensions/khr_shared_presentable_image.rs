#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_SHARED_PRESENTABLE_IMAGE_SPEC_VERSION")]
pub const KHR_SHARED_PRESENTABLE_IMAGE_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_SHARED_PRESENTABLE_IMAGE_EXTENSION_NAME")]
pub const KHR_SHARED_PRESENTABLE_IMAGE_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_shared_presentable_image");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_SWAPCHAIN_STATUS_KHR: *const std::os::raw::c_char = crate::cstr!("vkGetSwapchainStatusKHR");
#[doc = "Provided by [`crate::extensions::khr_shared_presentable_image`]"]
impl crate::vk1_0::ImageLayout {
    pub const SHARED_PRESENT_KHR: Self = Self(1000111000);
}
#[doc = "Provided by [`crate::extensions::khr_shared_presentable_image`]"]
impl crate::vk1_0::StructureType {
    pub const SHARED_PRESENT_SURFACE_CAPABILITIES_KHR: Self = Self(1000111000);
}
#[doc = "Provided by [`crate::extensions::khr_shared_presentable_image`]"]
impl crate::extensions::khr_surface::PresentModeKHR {
    pub const SHARED_DEMAND_REFRESH_KHR: Self = Self(1000111000);
    pub const SHARED_CONTINUOUS_REFRESH_KHR: Self = Self(1000111001);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSwapchainStatusKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetSwapchainStatusKHR = unsafe extern "system" fn(device: crate::vk1_0::Device, swapchain: crate::extensions::khr_swapchain::SwapchainKHR) -> crate::vk1_0::Result;
impl<'a> crate::ExtendableFromMut<'a, SharedPresentSurfaceCapabilitiesKHR> for crate::extensions::khr_get_surface_capabilities2::SurfaceCapabilities2KHRBuilder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, SharedPresentSurfaceCapabilitiesKHRBuilder<'_>> for crate::extensions::khr_get_surface_capabilities2::SurfaceCapabilities2KHRBuilder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSharedPresentSurfaceCapabilitiesKHR.html) · Structure"]
#[doc(alias = "VkSharedPresentSurfaceCapabilitiesKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SharedPresentSurfaceCapabilitiesKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub shared_present_supported_usage_flags: crate::vk1_0::ImageUsageFlags,
}
impl SharedPresentSurfaceCapabilitiesKHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::SHARED_PRESENT_SURFACE_CAPABILITIES_KHR;
}
impl Default for SharedPresentSurfaceCapabilitiesKHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), shared_present_supported_usage_flags: Default::default() }
    }
}
impl std::fmt::Debug for SharedPresentSurfaceCapabilitiesKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SharedPresentSurfaceCapabilitiesKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("shared_present_supported_usage_flags", &self.shared_present_supported_usage_flags).finish()
    }
}
impl SharedPresentSurfaceCapabilitiesKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> SharedPresentSurfaceCapabilitiesKHRBuilder<'a> {
        SharedPresentSurfaceCapabilitiesKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSharedPresentSurfaceCapabilitiesKHR.html) · Builder of [`SharedPresentSurfaceCapabilitiesKHR`]"]
#[repr(transparent)]
pub struct SharedPresentSurfaceCapabilitiesKHRBuilder<'a>(SharedPresentSurfaceCapabilitiesKHR, std::marker::PhantomData<&'a ()>);
impl<'a> SharedPresentSurfaceCapabilitiesKHRBuilder<'a> {
    #[inline]
    pub fn new() -> SharedPresentSurfaceCapabilitiesKHRBuilder<'a> {
        SharedPresentSurfaceCapabilitiesKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn shared_present_supported_usage_flags(mut self, shared_present_supported_usage_flags: crate::vk1_0::ImageUsageFlags) -> Self {
        self.0.shared_present_supported_usage_flags = shared_present_supported_usage_flags as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> SharedPresentSurfaceCapabilitiesKHR {
        self.0
    }
}
impl<'a> std::default::Default for SharedPresentSurfaceCapabilitiesKHRBuilder<'a> {
    fn default() -> SharedPresentSurfaceCapabilitiesKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for SharedPresentSurfaceCapabilitiesKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for SharedPresentSurfaceCapabilitiesKHRBuilder<'a> {
    type Target = SharedPresentSurfaceCapabilitiesKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SharedPresentSurfaceCapabilitiesKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::khr_shared_presentable_image`]"]
impl crate::DeviceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSwapchainStatusKHR.html) · Function"]
    #[doc(alias = "vkGetSwapchainStatusKHR")]
    pub unsafe fn get_swapchain_status_khr(&self, swapchain: crate::extensions::khr_swapchain::SwapchainKHR) -> crate::utils::VulkanResult<()> {
        let _function = self.get_swapchain_status_khr.expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(self.handle, swapchain as _);
        crate::utils::VulkanResult::new(_return, ())
    }
}
