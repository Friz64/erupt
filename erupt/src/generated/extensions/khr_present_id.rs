#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_PRESENT_ID_SPEC_VERSION")]
pub const KHR_PRESENT_ID_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_PRESENT_ID_EXTENSION_NAME")]
pub const KHR_PRESENT_ID_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_present_id");
#[doc = "Provided by [`crate::extensions::khr_present_id`]"]
impl crate::vk1_0::StructureType {
    pub const PRESENT_ID_KHR: Self = Self(1000294000);
    pub const PHYSICAL_DEVICE_PRESENT_ID_FEATURES_KHR: Self = Self(1000294001);
}
impl<'a> crate::ExtendableFrom<'a, PhysicalDevicePresentIdFeaturesKHR> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDevicePresentIdFeaturesKHRBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PresentIdKHR> for crate::extensions::khr_swapchain::PresentInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PresentIdKHRBuilder<'_>> for crate::extensions::khr_swapchain::PresentInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDevicePresentIdFeaturesKHR> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDevicePresentIdFeaturesKHRBuilder<'_>> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevicePresentIdFeaturesKHR.html) · Structure"]
#[doc(alias = "VkPhysicalDevicePresentIdFeaturesKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDevicePresentIdFeaturesKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub present_id: crate::vk1_0::Bool32,
}
impl PhysicalDevicePresentIdFeaturesKHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_PRESENT_ID_FEATURES_KHR;
}
impl Default for PhysicalDevicePresentIdFeaturesKHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), present_id: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDevicePresentIdFeaturesKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDevicePresentIdFeaturesKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("present_id", &(self.present_id != 0)).finish()
    }
}
impl PhysicalDevicePresentIdFeaturesKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDevicePresentIdFeaturesKHRBuilder<'a> {
        PhysicalDevicePresentIdFeaturesKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevicePresentIdFeaturesKHR.html) · Builder of [`PhysicalDevicePresentIdFeaturesKHR`]"]
#[repr(transparent)]
pub struct PhysicalDevicePresentIdFeaturesKHRBuilder<'a>(PhysicalDevicePresentIdFeaturesKHR, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDevicePresentIdFeaturesKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDevicePresentIdFeaturesKHRBuilder<'a> {
        PhysicalDevicePresentIdFeaturesKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn present_id(mut self, present_id: bool) -> Self {
        self.0.present_id = present_id as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> PhysicalDevicePresentIdFeaturesKHR {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDevicePresentIdFeaturesKHRBuilder<'a> {
    fn default() -> PhysicalDevicePresentIdFeaturesKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDevicePresentIdFeaturesKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDevicePresentIdFeaturesKHRBuilder<'a> {
    type Target = PhysicalDevicePresentIdFeaturesKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDevicePresentIdFeaturesKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPresentIdKHR.html) · Structure"]
#[doc(alias = "VkPresentIdKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PresentIdKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub swapchain_count: u32,
    pub p_present_ids: *const u64,
}
impl PresentIdKHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PRESENT_ID_KHR;
}
impl Default for PresentIdKHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), swapchain_count: Default::default(), p_present_ids: std::ptr::null() }
    }
}
impl std::fmt::Debug for PresentIdKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PresentIdKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("swapchain_count", &self.swapchain_count).field("p_present_ids", &self.p_present_ids).finish()
    }
}
impl PresentIdKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> PresentIdKHRBuilder<'a> {
        PresentIdKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPresentIdKHR.html) · Builder of [`PresentIdKHR`]"]
#[repr(transparent)]
pub struct PresentIdKHRBuilder<'a>(PresentIdKHR, std::marker::PhantomData<&'a ()>);
impl<'a> PresentIdKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PresentIdKHRBuilder<'a> {
        PresentIdKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn present_ids(mut self, present_ids: &'a [u64]) -> Self {
        self.0.p_present_ids = present_ids.as_ptr() as _;
        self.0.swapchain_count = present_ids.len() as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> PresentIdKHR {
        self.0
    }
}
impl<'a> std::default::Default for PresentIdKHRBuilder<'a> {
    fn default() -> PresentIdKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PresentIdKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PresentIdKHRBuilder<'a> {
    type Target = PresentIdKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PresentIdKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
