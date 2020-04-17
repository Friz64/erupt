# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_KHR_shared_presentable_image.html)\n\n## Extends\n- [`ImageLayout`](../../vk1_0/struct.ImageLayout.html)\n- [`PresentModeKHR`](../../extensions/khr_surface/struct.PresentModeKHR.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_SHARED_PRESENTABLE_IMAGE_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_SHARED_PRESENTABLE_IMAGE_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_shared_presentable_image");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSwapchainStatusKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetSwapchainStatusKHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
) -> crate::vk1_0::Result;
#[doc = "Provides Device Commands for [`KhrSharedPresentableImageDeviceLoaderExt`](trait.KhrSharedPresentableImageDeviceLoaderExt.html)"]
pub struct KhrSharedPresentableImageDeviceCommands {
    pub get_swapchain_status_khr: PFN_vkGetSwapchainStatusKHR,
}
impl KhrSharedPresentableImageDeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<KhrSharedPresentableImageDeviceCommands> {
        unsafe {
            Some(KhrSharedPresentableImageDeviceCommands {
                get_swapchain_status_khr: std::mem::transmute(
                    loader.symbol("vkGetSwapchainStatusKHR")?,
                ),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`KhrSharedPresentableImageDeviceCommands`](struct.KhrSharedPresentableImageDeviceCommands.html)"]
pub trait KhrSharedPresentableImageDeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSwapchainStatusKHR.html) · Device Command"]
    unsafe fn get_swapchain_status_khr(
        &self,
        swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
    ) -> crate::utils::VulkanResult<()>;
}
impl KhrSharedPresentableImageDeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSwapchainStatusKHR.html) · Device Command"]
    unsafe fn get_swapchain_status_khr(
        &self,
        swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
    ) -> crate::utils::VulkanResult<()> {
        let function = self
            .khr_shared_presentable_image
            .as_ref()
            .expect("`khr_shared_presentable_image` not loaded")
            .get_swapchain_status_khr;
        let _val = function(self.handle, swapchain);
        crate::utils::VulkanResult::new(_val, ())
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSharedPresentSurfaceCapabilitiesKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SharedPresentSurfaceCapabilitiesKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub shared_present_supported_usage_flags: crate::vk1_0::ImageUsageFlags,
}
impl SharedPresentSurfaceCapabilitiesKHR {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableBySharedPresentSurfaceCapabilitiesKHR,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> SharedPresentSurfaceCapabilitiesKHRBuilder<'a> {
        SharedPresentSurfaceCapabilitiesKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for SharedPresentSurfaceCapabilitiesKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("SharedPresentSurfaceCapabilitiesKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "shared_present_supported_usage_flags",
                &self.shared_present_supported_usage_flags,
            )
            .finish()
    }
}
impl Default for SharedPresentSurfaceCapabilitiesKHR {
    fn default() -> SharedPresentSurfaceCapabilitiesKHR {
        SharedPresentSurfaceCapabilitiesKHR {
            s_type: crate::vk1_0::StructureType::SHARED_PRESENT_SURFACE_CAPABILITIES_KHR,
            p_next: std::ptr::null_mut(),
            shared_present_supported_usage_flags: Default::default(),
        }
    }
}
#[doc = "Used by [`SharedPresentSurfaceCapabilitiesKHR::extend`](struct.SharedPresentSurfaceCapabilitiesKHR.html#method.extend)"]
pub trait ExtendableBySharedPresentSurfaceCapabilitiesKHR {}
impl ExtendableBySharedPresentSurfaceCapabilitiesKHR
    for crate::extensions::khr_get_surface_capabilities2::SurfaceCapabilities2KHR
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`SharedPresentSurfaceCapabilitiesKHR`](struct.SharedPresentSurfaceCapabilitiesKHR.html)"]
#[repr(transparent)]
pub struct SharedPresentSurfaceCapabilitiesKHRBuilder<'a>(
    SharedPresentSurfaceCapabilitiesKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> SharedPresentSurfaceCapabilitiesKHRBuilder<'a> {
    #[inline]
    pub fn new() -> SharedPresentSurfaceCapabilitiesKHRBuilder<'a> {
        SharedPresentSurfaceCapabilitiesKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shared_present_supported_usage_flags(
        mut self,
        shared_present_supported_usage_flags: crate::vk1_0::ImageUsageFlags,
    ) -> Self {
        self.0.shared_present_supported_usage_flags = shared_present_supported_usage_flags;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> SharedPresentSurfaceCapabilitiesKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for SharedPresentSurfaceCapabilitiesKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
