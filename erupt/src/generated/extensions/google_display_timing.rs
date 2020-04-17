# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_GOOGLE_display_timing.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const GOOGLE_DISPLAY_TIMING_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const GOOGLE_DISPLAY_TIMING_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_GOOGLE_display_timing");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetRefreshCycleDurationGOOGLE.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetRefreshCycleDurationGOOGLE = unsafe extern "system" fn ( device : crate :: vk1_0 :: Device , swapchain : crate :: extensions :: khr_swapchain :: SwapchainKHR , p_display_timing_properties : * mut crate :: extensions :: google_display_timing :: RefreshCycleDurationGOOGLE , ) -> crate :: vk1_0 :: Result ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPastPresentationTimingGOOGLE.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPastPresentationTimingGOOGLE = unsafe extern "system" fn ( device : crate :: vk1_0 :: Device , swapchain : crate :: extensions :: khr_swapchain :: SwapchainKHR , p_presentation_timing_count : * mut u32 , p_presentation_timings : * mut crate :: extensions :: google_display_timing :: PastPresentationTimingGOOGLE , ) -> crate :: vk1_0 :: Result ;
#[doc = "Provides Device Commands for [`GoogleDisplayTimingDeviceLoaderExt`](trait.GoogleDisplayTimingDeviceLoaderExt.html)"]
pub struct GoogleDisplayTimingDeviceCommands {
    pub get_refresh_cycle_duration_google: PFN_vkGetRefreshCycleDurationGOOGLE,
    pub get_past_presentation_timing_google: PFN_vkGetPastPresentationTimingGOOGLE,
}
impl GoogleDisplayTimingDeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<GoogleDisplayTimingDeviceCommands> {
        unsafe {
            Some(GoogleDisplayTimingDeviceCommands {
                get_refresh_cycle_duration_google: std::mem::transmute(
                    loader.symbol("vkGetRefreshCycleDurationGOOGLE")?,
                ),
                get_past_presentation_timing_google: std::mem::transmute(
                    loader.symbol("vkGetPastPresentationTimingGOOGLE")?,
                ),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`GoogleDisplayTimingDeviceCommands`](struct.GoogleDisplayTimingDeviceCommands.html)"]
pub trait GoogleDisplayTimingDeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetRefreshCycleDurationGOOGLE.html) · Device Command"]
    unsafe fn get_refresh_cycle_duration_google(
        &self,
        swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
        display_timing_properties: Option<
            crate::extensions::google_display_timing::RefreshCycleDurationGOOGLE,
        >,
    ) -> crate::utils::VulkanResult<
        crate::extensions::google_display_timing::RefreshCycleDurationGOOGLE,
    >;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPastPresentationTimingGOOGLE.html) · Device Command"]
    unsafe fn get_past_presentation_timing_google(
        &self,
        swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
        presentation_timing_count: Option<u32>,
    ) -> crate::utils::VulkanResult<
        Vec<crate::extensions::google_display_timing::PastPresentationTimingGOOGLE>,
    >;
}
impl GoogleDisplayTimingDeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetRefreshCycleDurationGOOGLE.html) · Device Command"]
    unsafe fn get_refresh_cycle_duration_google(
        &self,
        swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
        display_timing_properties: Option<
            crate::extensions::google_display_timing::RefreshCycleDurationGOOGLE,
        >,
    ) -> crate::utils::VulkanResult<
        crate::extensions::google_display_timing::RefreshCycleDurationGOOGLE,
    > {
        let function = self
            .google_display_timing
            .as_ref()
            .expect("`google_display_timing` not loaded")
            .get_refresh_cycle_duration_google;
        let mut display_timing_properties =
            display_timing_properties.unwrap_or_else(|| Default::default());
        let _val = function(self.handle, swapchain, &mut display_timing_properties);
        crate::utils::VulkanResult::new(_val, display_timing_properties)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPastPresentationTimingGOOGLE.html) · Device Command"]
    unsafe fn get_past_presentation_timing_google(
        &self,
        swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
        presentation_timing_count: Option<u32>,
    ) -> crate::utils::VulkanResult<
        Vec<crate::extensions::google_display_timing::PastPresentationTimingGOOGLE>,
    > {
        let function = self
            .google_display_timing
            .as_ref()
            .expect("`google_display_timing` not loaded")
            .get_past_presentation_timing_google;
        let mut presentation_timing_count = presentation_timing_count.unwrap_or_else(|| {
            let mut val = Default::default();
            function(self.handle, swapchain, &mut val, std::ptr::null_mut());
            val
        });
        let mut presentation_timings = vec![Default::default(); presentation_timing_count as _];
        let _val = function(
            self.handle,
            swapchain,
            &mut presentation_timing_count,
            presentation_timings.as_mut_ptr(),
        );
        crate::utils::VulkanResult::new(_val, presentation_timings)
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRefreshCycleDurationGOOGLE.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RefreshCycleDurationGOOGLE {
    pub refresh_duration: u64,
}
impl RefreshCycleDurationGOOGLE {
    #[inline]
    pub fn builder<'a>(self) -> RefreshCycleDurationGOOGLEBuilder<'a> {
        RefreshCycleDurationGOOGLEBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for RefreshCycleDurationGOOGLE {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("RefreshCycleDurationGOOGLE")
            .field("refresh_duration", &self.refresh_duration)
            .finish()
    }
}
impl Default for RefreshCycleDurationGOOGLE {
    fn default() -> RefreshCycleDurationGOOGLE {
        RefreshCycleDurationGOOGLE {
            refresh_duration: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`RefreshCycleDurationGOOGLE`](struct.RefreshCycleDurationGOOGLE.html)"]
#[repr(transparent)]
pub struct RefreshCycleDurationGOOGLEBuilder<'a>(
    RefreshCycleDurationGOOGLE,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> RefreshCycleDurationGOOGLEBuilder<'a> {
    #[inline]
    pub fn new() -> RefreshCycleDurationGOOGLEBuilder<'a> {
        RefreshCycleDurationGOOGLEBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn refresh_duration(mut self, refresh_duration: u64) -> Self {
        self.0.refresh_duration = refresh_duration;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> RefreshCycleDurationGOOGLE {
        self.0
    }
}
impl<'a> std::fmt::Debug for RefreshCycleDurationGOOGLEBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PastPresentationTimingGOOGLE {
    pub present_id: u32,
    pub desired_present_time: u64,
    pub actual_present_time: u64,
    pub earliest_present_time: u64,
    pub present_margin: u64,
}
impl PastPresentationTimingGOOGLE {
    #[inline]
    pub fn builder<'a>(self) -> PastPresentationTimingGOOGLEBuilder<'a> {
        PastPresentationTimingGOOGLEBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PastPresentationTimingGOOGLE {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PastPresentationTimingGOOGLE")
            .field("present_id", &self.present_id)
            .field("desired_present_time", &self.desired_present_time)
            .field("actual_present_time", &self.actual_present_time)
            .field("earliest_present_time", &self.earliest_present_time)
            .field("present_margin", &self.present_margin)
            .finish()
    }
}
impl Default for PastPresentationTimingGOOGLE {
    fn default() -> PastPresentationTimingGOOGLE {
        PastPresentationTimingGOOGLE {
            present_id: Default::default(),
            desired_present_time: Default::default(),
            actual_present_time: Default::default(),
            earliest_present_time: Default::default(),
            present_margin: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PastPresentationTimingGOOGLE`](struct.PastPresentationTimingGOOGLE.html)"]
#[repr(transparent)]
pub struct PastPresentationTimingGOOGLEBuilder<'a>(
    PastPresentationTimingGOOGLE,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PastPresentationTimingGOOGLEBuilder<'a> {
    #[inline]
    pub fn new() -> PastPresentationTimingGOOGLEBuilder<'a> {
        PastPresentationTimingGOOGLEBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn present_id(mut self, present_id: u32) -> Self {
        self.0.present_id = present_id;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn desired_present_time(mut self, desired_present_time: u64) -> Self {
        self.0.desired_present_time = desired_present_time;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn actual_present_time(mut self, actual_present_time: u64) -> Self {
        self.0.actual_present_time = actual_present_time;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn earliest_present_time(mut self, earliest_present_time: u64) -> Self {
        self.0.earliest_present_time = earliest_present_time;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn present_margin(mut self, present_margin: u64) -> Self {
        self.0.present_margin = present_margin;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PastPresentationTimingGOOGLE {
        self.0
    }
}
impl<'a> std::fmt::Debug for PastPresentationTimingGOOGLEBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PresentTimesInfoGOOGLE {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub swapchain_count: u32,
    pub p_times: *const crate::extensions::google_display_timing::PresentTimeGOOGLE,
}
impl PresentTimesInfoGOOGLE {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPresentTimesInfoGOOGLE,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PresentTimesInfoGOOGLEBuilder<'a> {
        PresentTimesInfoGOOGLEBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PresentTimesInfoGOOGLE {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PresentTimesInfoGOOGLE")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("swapchain_count", &self.swapchain_count)
            .field("p_times", &self.p_times)
            .finish()
    }
}
impl Default for PresentTimesInfoGOOGLE {
    fn default() -> PresentTimesInfoGOOGLE {
        PresentTimesInfoGOOGLE {
            s_type: crate::vk1_0::StructureType::PRESENT_TIMES_INFO_GOOGLE,
            p_next: std::ptr::null(),
            swapchain_count: Default::default(),
            p_times: std::ptr::null(),
        }
    }
}
#[doc = "Used by [`PresentTimesInfoGOOGLE::extend`](struct.PresentTimesInfoGOOGLE.html#method.extend)"]
pub trait ExtendableByPresentTimesInfoGOOGLE {}
impl ExtendableByPresentTimesInfoGOOGLE for crate::extensions::khr_swapchain::PresentInfoKHR {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PresentTimesInfoGOOGLE`](struct.PresentTimesInfoGOOGLE.html)"]
#[repr(transparent)]
pub struct PresentTimesInfoGOOGLEBuilder<'a>(
    PresentTimesInfoGOOGLE,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PresentTimesInfoGOOGLEBuilder<'a> {
    #[inline]
    pub fn new() -> PresentTimesInfoGOOGLEBuilder<'a> {
        PresentTimesInfoGOOGLEBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn times(
        mut self,
        times: &'a [crate::extensions::google_display_timing::PresentTimeGOOGLEBuilder],
    ) -> Self {
        self.0.swapchain_count = times.len() as _;
        self.0.p_times = times.as_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PresentTimesInfoGOOGLE {
        self.0
    }
}
impl<'a> std::fmt::Debug for PresentTimesInfoGOOGLEBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PresentTimeGOOGLE {
    pub present_id: u32,
    pub desired_present_time: u64,
}
impl PresentTimeGOOGLE {
    #[inline]
    pub fn builder<'a>(self) -> PresentTimeGOOGLEBuilder<'a> {
        PresentTimeGOOGLEBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PresentTimeGOOGLE {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PresentTimeGOOGLE")
            .field("present_id", &self.present_id)
            .field("desired_present_time", &self.desired_present_time)
            .finish()
    }
}
impl Default for PresentTimeGOOGLE {
    fn default() -> PresentTimeGOOGLE {
        PresentTimeGOOGLE {
            present_id: Default::default(),
            desired_present_time: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PresentTimeGOOGLE`](struct.PresentTimeGOOGLE.html)"]
#[repr(transparent)]
pub struct PresentTimeGOOGLEBuilder<'a>(PresentTimeGOOGLE, std::marker::PhantomData<&'a ()>);
impl<'a> PresentTimeGOOGLEBuilder<'a> {
    #[inline]
    pub fn new() -> PresentTimeGOOGLEBuilder<'a> {
        PresentTimeGOOGLEBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn present_id(mut self, present_id: u32) -> Self {
        self.0.present_id = present_id;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn desired_present_time(mut self, desired_present_time: u64) -> Self {
        self.0.desired_present_time = desired_present_time;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PresentTimeGOOGLE {
        self.0
    }
}
impl<'a> std::fmt::Debug for PresentTimeGOOGLEBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
