# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_KHR_display_swapchain.html)\n\n## Extends\n- [`Result`](../../vk1_0/struct.Result.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_DISPLAY_SWAPCHAIN_SPEC_VERSION: u32 = 10;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_DISPLAY_SWAPCHAIN_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_display_swapchain");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateSharedSwapchainsKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateSharedSwapchainsKHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    swapchain_count: u32,
    p_create_infos: *const crate::extensions::khr_swapchain::SwapchainCreateInfoKHR,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
    p_swapchains: *mut crate::extensions::khr_swapchain::SwapchainKHR,
) -> crate::vk1_0::Result;
#[doc = "Provides Device Commands for [`KhrDisplaySwapchainDeviceLoaderExt`](trait.KhrDisplaySwapchainDeviceLoaderExt.html)"]
pub struct KhrDisplaySwapchainDeviceCommands {
    pub create_shared_swapchains_khr: PFN_vkCreateSharedSwapchainsKHR,
}
impl KhrDisplaySwapchainDeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<KhrDisplaySwapchainDeviceCommands> {
        unsafe {
            Some(KhrDisplaySwapchainDeviceCommands {
                create_shared_swapchains_khr: std::mem::transmute(
                    loader.symbol("vkCreateSharedSwapchainsKHR")?,
                ),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`KhrDisplaySwapchainDeviceCommands`](struct.KhrDisplaySwapchainDeviceCommands.html)"]
pub trait KhrDisplaySwapchainDeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateSharedSwapchainsKHR.html) · Device Command"]
    unsafe fn create_shared_swapchains_khr(
        &self,
        create_infos: &[crate::extensions::khr_swapchain::SwapchainCreateInfoKHRBuilder],
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
    ) -> crate::utils::VulkanResult<Vec<crate::extensions::khr_swapchain::SwapchainKHR>>;
}
impl KhrDisplaySwapchainDeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateSharedSwapchainsKHR.html) · Device Command"]
    unsafe fn create_shared_swapchains_khr(
        &self,
        create_infos: &[crate::extensions::khr_swapchain::SwapchainCreateInfoKHRBuilder],
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
    ) -> crate::utils::VulkanResult<Vec<crate::extensions::khr_swapchain::SwapchainKHR>> {
        let function = self
            .khr_display_swapchain
            .as_ref()
            .expect("`khr_display_swapchain` not loaded")
            .create_shared_swapchains_khr;
        let swapchain_count = create_infos.len() as _;
        let mut swapchains = vec![Default::default(); swapchain_count as _];
        let _val = function(
            self.handle,
            swapchain_count,
            create_infos.as_ptr() as _,
            if let Some(allocator) = allocator {
                allocator
            } else {
                std::ptr::null()
            },
            swapchains.as_mut_ptr(),
        );
        crate::utils::VulkanResult::new(_val, swapchains)
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayPresentInfoKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DisplayPresentInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub src_rect: crate::vk1_0::Rect2D,
    pub dst_rect: crate::vk1_0::Rect2D,
    pub persistent: crate::vk1_0::Bool32,
}
impl DisplayPresentInfoKHR {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByDisplayPresentInfoKHR,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> DisplayPresentInfoKHRBuilder<'a> {
        DisplayPresentInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for DisplayPresentInfoKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("DisplayPresentInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("src_rect", &self.src_rect)
            .field("dst_rect", &self.dst_rect)
            .field("persistent", &(self.persistent != 0))
            .finish()
    }
}
impl Default for DisplayPresentInfoKHR {
    fn default() -> DisplayPresentInfoKHR {
        DisplayPresentInfoKHR {
            s_type: crate::vk1_0::StructureType::DISPLAY_PRESENT_INFO_KHR,
            p_next: std::ptr::null(),
            src_rect: Default::default(),
            dst_rect: Default::default(),
            persistent: Default::default(),
        }
    }
}
#[doc = "Used by [`DisplayPresentInfoKHR::extend`](struct.DisplayPresentInfoKHR.html#method.extend)"]
pub trait ExtendableByDisplayPresentInfoKHR {}
impl ExtendableByDisplayPresentInfoKHR for crate::extensions::khr_swapchain::PresentInfoKHR {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`DisplayPresentInfoKHR`](struct.DisplayPresentInfoKHR.html)"]
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
    #[allow(unused_mut)]
    #[inline]
    pub fn src_rect(mut self, src_rect: crate::vk1_0::Rect2D) -> Self {
        self.0.src_rect = src_rect;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn dst_rect(mut self, dst_rect: crate::vk1_0::Rect2D) -> Self {
        self.0.dst_rect = dst_rect;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn persistent(mut self, persistent: bool) -> Self {
        self.0.persistent = persistent as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> DisplayPresentInfoKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for DisplayPresentInfoKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
