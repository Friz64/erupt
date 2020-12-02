#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_DISPLAY_SWAPCHAIN_SPEC_VERSION")]
pub const KHR_DISPLAY_SWAPCHAIN_SPEC_VERSION: u32 = 10;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_DISPLAY_SWAPCHAIN_EXTENSION_NAME")]
pub const KHR_DISPLAY_SWAPCHAIN_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_display_swapchain");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "FN_CREATE_SHARED_SWAPCHAINS_KHR")]
pub const FN_CREATE_SHARED_SWAPCHAINS_KHR: *const std::os::raw::c_char =
    crate::cstr!("vkCreateSharedSwapchainsKHR");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateSharedSwapchainsKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateSharedSwapchainsKHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    swapchain_count: u32,
    p_create_infos: *const crate::extensions::khr_swapchain::SwapchainCreateInfoKHR,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
    p_swapchains: *mut crate::extensions::khr_swapchain::SwapchainKHR,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayPresentInfoKHR.html) · Structure"]
#[doc(alias = "VkDisplayPresentInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DisplayPresentInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub src_rect: crate::vk1_0::Rect2D,
    pub dst_rect: crate::vk1_0::Rect2D,
    pub persistent: crate::vk1_0::Bool32,
}
impl Default for DisplayPresentInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::DISPLAY_PRESENT_INFO_KHR,
            p_next: std::ptr::null(),
            src_rect: Default::default(),
            dst_rect: Default::default(),
            persistent: Default::default(),
        }
    }
}
impl std::fmt::Debug for DisplayPresentInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DisplayPresentInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("src_rect", &self.src_rect)
            .field("dst_rect", &self.dst_rect)
            .field("persistent", &(self.persistent != 0))
            .finish()
    }
}
impl DisplayPresentInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> DisplayPresentInfoKHRBuilder<'a> {
        DisplayPresentInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayPresentInfoKHR.html) · Builder of [`DisplayPresentInfoKHR`](struct.DisplayPresentInfoKHR.html)"]
#[repr(transparent)]
pub struct DisplayPresentInfoKHRBuilder<'a>(
    DisplayPresentInfoKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> DisplayPresentInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> DisplayPresentInfoKHRBuilder<'a> {
        DisplayPresentInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn src_rect(mut self, src_rect: crate::vk1_0::Rect2D) -> Self {
        self.0.src_rect = src_rect as _;
        self
    }
    #[inline]
    pub fn dst_rect(mut self, dst_rect: crate::vk1_0::Rect2D) -> Self {
        self.0.dst_rect = dst_rect as _;
        self
    }
    #[inline]
    pub fn persistent(mut self, persistent: bool) -> Self {
        self.0.persistent = persistent as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DisplayPresentInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for DisplayPresentInfoKHRBuilder<'a> {
    fn default() -> DisplayPresentInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DisplayPresentInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DisplayPresentInfoKHRBuilder<'a> {
    type Target = DisplayPresentInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DisplayPresentInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`extensions::khr_display_swapchain`](extensions/khr_display_swapchain/index.html)"]
impl crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateSharedSwapchainsKHR.html) · Function"]
    #[doc(alias = "vkCreateSharedSwapchainsKHR")]
    pub unsafe fn create_shared_swapchains_khr(
        &self,
        create_infos: &[crate::extensions::khr_swapchain::SwapchainCreateInfoKHRBuilder],
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
    ) -> crate::utils::VulkanResult<Vec<crate::extensions::khr_swapchain::SwapchainKHR>> {
        let _function = self
            .create_shared_swapchains_khr
            .expect("`create_shared_swapchains_khr` is not loaded");
        let swapchain_count = create_infos.len();
        let mut swapchains = vec![Default::default(); swapchain_count as _];
        let _return = _function(
            self.handle,
            swapchain_count as _,
            create_infos.as_ptr() as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            swapchains.as_mut_ptr(),
        );
        crate::utils::VulkanResult::new(_return, swapchains)
    }
}
