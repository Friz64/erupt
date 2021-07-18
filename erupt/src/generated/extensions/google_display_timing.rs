#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_GOOGLE_DISPLAY_TIMING_SPEC_VERSION")]
pub const GOOGLE_DISPLAY_TIMING_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_GOOGLE_DISPLAY_TIMING_EXTENSION_NAME")]
pub const GOOGLE_DISPLAY_TIMING_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_GOOGLE_display_timing");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_REFRESH_CYCLE_DURATION_GOOGLE: *const std::os::raw::c_char = crate::cstr!("vkGetRefreshCycleDurationGOOGLE");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_PAST_PRESENTATION_TIMING_GOOGLE: *const std::os::raw::c_char = crate::cstr!("vkGetPastPresentationTimingGOOGLE");
#[doc = "Provided by [`crate::extensions::google_display_timing`]"]
impl crate::vk1_0::StructureType {
    pub const PRESENT_TIMES_INFO_GOOGLE: Self = Self(1000092000);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetRefreshCycleDurationGOOGLE.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetRefreshCycleDurationGOOGLE = unsafe extern "system" fn(device: crate::vk1_0::Device, swapchain: crate::extensions::khr_swapchain::SwapchainKHR, p_display_timing_properties: *mut crate::extensions::google_display_timing::RefreshCycleDurationGOOGLE) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPastPresentationTimingGOOGLE.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPastPresentationTimingGOOGLE = unsafe extern "system" fn(device: crate::vk1_0::Device, swapchain: crate::extensions::khr_swapchain::SwapchainKHR, p_presentation_timing_count: *mut u32, p_presentation_timings: *mut crate::extensions::google_display_timing::PastPresentationTimingGOOGLE) -> crate::vk1_0::Result;
impl<'a> crate::ExtendableFromConst<'a, PresentTimesInfoGOOGLE> for crate::extensions::khr_swapchain::PresentInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, PresentTimesInfoGOOGLEBuilder<'_>> for crate::extensions::khr_swapchain::PresentInfoKHRBuilder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRefreshCycleDurationGOOGLE.html) · Structure"]
#[doc(alias = "VkRefreshCycleDurationGOOGLE")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RefreshCycleDurationGOOGLE {
    pub refresh_duration: u64,
}
impl Default for RefreshCycleDurationGOOGLE {
    fn default() -> Self {
        Self { refresh_duration: Default::default() }
    }
}
impl std::fmt::Debug for RefreshCycleDurationGOOGLE {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RefreshCycleDurationGOOGLE").field("refresh_duration", &self.refresh_duration).finish()
    }
}
impl RefreshCycleDurationGOOGLE {
    #[inline]
    pub fn into_builder<'a>(self) -> RefreshCycleDurationGOOGLEBuilder<'a> {
        RefreshCycleDurationGOOGLEBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRefreshCycleDurationGOOGLE.html) · Builder of [`RefreshCycleDurationGOOGLE`]"]
#[repr(transparent)]
pub struct RefreshCycleDurationGOOGLEBuilder<'a>(RefreshCycleDurationGOOGLE, std::marker::PhantomData<&'a ()>);
impl<'a> RefreshCycleDurationGOOGLEBuilder<'a> {
    #[inline]
    pub fn new() -> RefreshCycleDurationGOOGLEBuilder<'a> {
        RefreshCycleDurationGOOGLEBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn refresh_duration(mut self, refresh_duration: u64) -> Self {
        self.0.refresh_duration = refresh_duration as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> RefreshCycleDurationGOOGLE {
        self.0
    }
}
impl<'a> std::default::Default for RefreshCycleDurationGOOGLEBuilder<'a> {
    fn default() -> RefreshCycleDurationGOOGLEBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for RefreshCycleDurationGOOGLEBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for RefreshCycleDurationGOOGLEBuilder<'a> {
    type Target = RefreshCycleDurationGOOGLE;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for RefreshCycleDurationGOOGLEBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPastPresentationTimingGOOGLE.html) · Structure"]
#[doc(alias = "VkPastPresentationTimingGOOGLE")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PastPresentationTimingGOOGLE {
    pub present_id: u32,
    pub desired_present_time: u64,
    pub actual_present_time: u64,
    pub earliest_present_time: u64,
    pub present_margin: u64,
}
impl Default for PastPresentationTimingGOOGLE {
    fn default() -> Self {
        Self { present_id: Default::default(), desired_present_time: Default::default(), actual_present_time: Default::default(), earliest_present_time: Default::default(), present_margin: Default::default() }
    }
}
impl std::fmt::Debug for PastPresentationTimingGOOGLE {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PastPresentationTimingGOOGLE").field("present_id", &self.present_id).field("desired_present_time", &self.desired_present_time).field("actual_present_time", &self.actual_present_time).field("earliest_present_time", &self.earliest_present_time).field("present_margin", &self.present_margin).finish()
    }
}
impl PastPresentationTimingGOOGLE {
    #[inline]
    pub fn into_builder<'a>(self) -> PastPresentationTimingGOOGLEBuilder<'a> {
        PastPresentationTimingGOOGLEBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPastPresentationTimingGOOGLE.html) · Builder of [`PastPresentationTimingGOOGLE`]"]
#[repr(transparent)]
pub struct PastPresentationTimingGOOGLEBuilder<'a>(PastPresentationTimingGOOGLE, std::marker::PhantomData<&'a ()>);
impl<'a> PastPresentationTimingGOOGLEBuilder<'a> {
    #[inline]
    pub fn new() -> PastPresentationTimingGOOGLEBuilder<'a> {
        PastPresentationTimingGOOGLEBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn present_id(mut self, present_id: u32) -> Self {
        self.0.present_id = present_id as _;
        self
    }
    #[inline]
    pub fn desired_present_time(mut self, desired_present_time: u64) -> Self {
        self.0.desired_present_time = desired_present_time as _;
        self
    }
    #[inline]
    pub fn actual_present_time(mut self, actual_present_time: u64) -> Self {
        self.0.actual_present_time = actual_present_time as _;
        self
    }
    #[inline]
    pub fn earliest_present_time(mut self, earliest_present_time: u64) -> Self {
        self.0.earliest_present_time = earliest_present_time as _;
        self
    }
    #[inline]
    pub fn present_margin(mut self, present_margin: u64) -> Self {
        self.0.present_margin = present_margin as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PastPresentationTimingGOOGLE {
        self.0
    }
}
impl<'a> std::default::Default for PastPresentationTimingGOOGLEBuilder<'a> {
    fn default() -> PastPresentationTimingGOOGLEBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PastPresentationTimingGOOGLEBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PastPresentationTimingGOOGLEBuilder<'a> {
    type Target = PastPresentationTimingGOOGLE;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PastPresentationTimingGOOGLEBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPresentTimesInfoGOOGLE.html) · Structure"]
#[doc(alias = "VkPresentTimesInfoGOOGLE")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PresentTimesInfoGOOGLE {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub swapchain_count: u32,
    pub p_times: *const crate::extensions::google_display_timing::PresentTimeGOOGLE,
}
impl PresentTimesInfoGOOGLE {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PRESENT_TIMES_INFO_GOOGLE;
}
impl Default for PresentTimesInfoGOOGLE {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PRESENT_TIMES_INFO_GOOGLE, p_next: std::ptr::null(), swapchain_count: Default::default(), p_times: std::ptr::null() }
    }
}
impl std::fmt::Debug for PresentTimesInfoGOOGLE {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PresentTimesInfoGOOGLE").field("s_type", &self.s_type).field("p_next", &self.p_next).field("swapchain_count", &self.swapchain_count).field("p_times", &self.p_times).finish()
    }
}
impl PresentTimesInfoGOOGLE {
    #[inline]
    pub fn into_builder<'a>(self) -> PresentTimesInfoGOOGLEBuilder<'a> {
        PresentTimesInfoGOOGLEBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPresentTimesInfoGOOGLE.html) · Builder of [`PresentTimesInfoGOOGLE`]"]
#[repr(transparent)]
pub struct PresentTimesInfoGOOGLEBuilder<'a>(PresentTimesInfoGOOGLE, std::marker::PhantomData<&'a ()>);
impl<'a> PresentTimesInfoGOOGLEBuilder<'a> {
    #[inline]
    pub fn new() -> PresentTimesInfoGOOGLEBuilder<'a> {
        PresentTimesInfoGOOGLEBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn times(mut self, times: &'a [crate::extensions::google_display_timing::PresentTimeGOOGLEBuilder]) -> Self {
        self.0.p_times = times.as_ptr() as _;
        self.0.swapchain_count = times.len() as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PresentTimesInfoGOOGLE {
        self.0
    }
}
impl<'a> std::default::Default for PresentTimesInfoGOOGLEBuilder<'a> {
    fn default() -> PresentTimesInfoGOOGLEBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PresentTimesInfoGOOGLEBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PresentTimesInfoGOOGLEBuilder<'a> {
    type Target = PresentTimesInfoGOOGLE;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PresentTimesInfoGOOGLEBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPresentTimeGOOGLE.html) · Structure"]
#[doc(alias = "VkPresentTimeGOOGLE")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PresentTimeGOOGLE {
    pub present_id: u32,
    pub desired_present_time: u64,
}
impl Default for PresentTimeGOOGLE {
    fn default() -> Self {
        Self { present_id: Default::default(), desired_present_time: Default::default() }
    }
}
impl std::fmt::Debug for PresentTimeGOOGLE {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PresentTimeGOOGLE").field("present_id", &self.present_id).field("desired_present_time", &self.desired_present_time).finish()
    }
}
impl PresentTimeGOOGLE {
    #[inline]
    pub fn into_builder<'a>(self) -> PresentTimeGOOGLEBuilder<'a> {
        PresentTimeGOOGLEBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPresentTimeGOOGLE.html) · Builder of [`PresentTimeGOOGLE`]"]
#[repr(transparent)]
pub struct PresentTimeGOOGLEBuilder<'a>(PresentTimeGOOGLE, std::marker::PhantomData<&'a ()>);
impl<'a> PresentTimeGOOGLEBuilder<'a> {
    #[inline]
    pub fn new() -> PresentTimeGOOGLEBuilder<'a> {
        PresentTimeGOOGLEBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn present_id(mut self, present_id: u32) -> Self {
        self.0.present_id = present_id as _;
        self
    }
    #[inline]
    pub fn desired_present_time(mut self, desired_present_time: u64) -> Self {
        self.0.desired_present_time = desired_present_time as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PresentTimeGOOGLE {
        self.0
    }
}
impl<'a> std::default::Default for PresentTimeGOOGLEBuilder<'a> {
    fn default() -> PresentTimeGOOGLEBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PresentTimeGOOGLEBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PresentTimeGOOGLEBuilder<'a> {
    type Target = PresentTimeGOOGLE;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PresentTimeGOOGLEBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::google_display_timing`]"]
impl crate::DeviceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetRefreshCycleDurationGOOGLE.html) · Function"]
    #[doc(alias = "vkGetRefreshCycleDurationGOOGLE")]
    pub unsafe fn get_refresh_cycle_duration_google(&self, swapchain: crate::extensions::khr_swapchain::SwapchainKHR) -> crate::utils::VulkanResult<crate::extensions::google_display_timing::RefreshCycleDurationGOOGLE> {
        let _function = self.get_refresh_cycle_duration_google.expect(crate::NOT_LOADED_MESSAGE);
        let mut display_timing_properties = Default::default();
        let _return = _function(self.handle, swapchain as _, &mut display_timing_properties);
        crate::utils::VulkanResult::new(_return, display_timing_properties)
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPastPresentationTimingGOOGLE.html) · Function"]
    #[doc(alias = "vkGetPastPresentationTimingGOOGLE")]
    pub unsafe fn get_past_presentation_timing_google(&self, swapchain: crate::extensions::khr_swapchain::SwapchainKHR, presentation_timing_count: Option<u32>) -> crate::utils::VulkanResult<crate::SmallVec<crate::extensions::google_display_timing::PastPresentationTimingGOOGLE>> {
        let _function = self.get_past_presentation_timing_google.expect(crate::NOT_LOADED_MESSAGE);
        let mut presentation_timing_count = match presentation_timing_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                _function(self.handle, swapchain as _, &mut v, std::ptr::null_mut());
                v
            }
        };
        let mut presentation_timings = crate::SmallVec::from_elem(Default::default(), presentation_timing_count as _);
        let _return = _function(self.handle, swapchain as _, &mut presentation_timing_count, presentation_timings.as_mut_ptr());
        crate::utils::VulkanResult::new(_return, presentation_timings)
    }
}
