#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_PRESENT_WAIT_SPEC_VERSION")]
pub const KHR_PRESENT_WAIT_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_PRESENT_WAIT_EXTENSION_NAME")]
pub const KHR_PRESENT_WAIT_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_present_wait");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_WAIT_FOR_PRESENT_KHR: *const std::os::raw::c_char = crate::cstr!("vkWaitForPresentKHR");
#[doc = "Provided by [`crate::extensions::khr_present_wait`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_PRESENT_WAIT_FEATURES_KHR: Self = Self(1000248000);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkWaitForPresentKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkWaitForPresentKHR = unsafe extern "system" fn(device: crate::vk1_0::Device, swapchain: crate::extensions::khr_swapchain::SwapchainKHR, present_id: u64, timeout: u64) -> crate::vk1_0::Result;
impl<'a> crate::ExtendableFrom<'a, PhysicalDevicePresentWaitFeaturesKHR> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDevicePresentWaitFeaturesKHRBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDevicePresentWaitFeaturesKHR> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDevicePresentWaitFeaturesKHRBuilder<'_>> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevicePresentWaitFeaturesKHR.html) · Structure"]
#[doc(alias = "VkPhysicalDevicePresentWaitFeaturesKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDevicePresentWaitFeaturesKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub present_wait: crate::vk1_0::Bool32,
}
impl PhysicalDevicePresentWaitFeaturesKHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_PRESENT_WAIT_FEATURES_KHR;
}
impl Default for PhysicalDevicePresentWaitFeaturesKHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), present_wait: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDevicePresentWaitFeaturesKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDevicePresentWaitFeaturesKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("present_wait", &(self.present_wait != 0)).finish()
    }
}
impl PhysicalDevicePresentWaitFeaturesKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDevicePresentWaitFeaturesKHRBuilder<'a> {
        PhysicalDevicePresentWaitFeaturesKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevicePresentWaitFeaturesKHR.html) · Builder of [`PhysicalDevicePresentWaitFeaturesKHR`]"]
#[repr(transparent)]
pub struct PhysicalDevicePresentWaitFeaturesKHRBuilder<'a>(PhysicalDevicePresentWaitFeaturesKHR, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDevicePresentWaitFeaturesKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDevicePresentWaitFeaturesKHRBuilder<'a> {
        PhysicalDevicePresentWaitFeaturesKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn present_wait(mut self, present_wait: bool) -> Self {
        self.0.present_wait = present_wait as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDevicePresentWaitFeaturesKHR {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDevicePresentWaitFeaturesKHRBuilder<'a> {
    fn default() -> PhysicalDevicePresentWaitFeaturesKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDevicePresentWaitFeaturesKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDevicePresentWaitFeaturesKHRBuilder<'a> {
    type Target = PhysicalDevicePresentWaitFeaturesKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDevicePresentWaitFeaturesKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::khr_present_wait`]"]
impl crate::DeviceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkWaitForPresentKHR.html) · Function"]
    #[doc(alias = "vkWaitForPresentKHR")]
    pub unsafe fn wait_for_present_khr(&self, swapchain: crate::extensions::khr_swapchain::SwapchainKHR, present_id: u64, timeout: u64) -> crate::utils::VulkanResult<()> {
        let _function = self.wait_for_present_khr.expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(self.handle, swapchain as _, present_id as _, timeout as _);
        crate::utils::VulkanResult::new(_return, ())
    }
}
