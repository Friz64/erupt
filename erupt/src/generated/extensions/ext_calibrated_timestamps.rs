#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_CALIBRATED_TIMESTAMPS_SPEC_VERSION")]
pub const EXT_CALIBRATED_TIMESTAMPS_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_CALIBRATED_TIMESTAMPS_EXTENSION_NAME")]
pub const EXT_CALIBRATED_TIMESTAMPS_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_calibrated_timestamps");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_PHYSICAL_DEVICE_CALIBRATEABLE_TIME_DOMAINS_EXT: *const std::os::raw::c_char = crate::cstr!("vkGetPhysicalDeviceCalibrateableTimeDomainsEXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_CALIBRATED_TIMESTAMPS_EXT: *const std::os::raw::c_char = crate::cstr!("vkGetCalibratedTimestampsEXT");
#[doc = "Provided by [`crate::extensions::ext_calibrated_timestamps`]"]
impl crate::vk1_0::StructureType {
    pub const CALIBRATED_TIMESTAMP_INFO_EXT: Self = Self(1000184000);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkTimeDomainEXT.html) · Enum"]
#[doc(alias = "VkTimeDomainEXT")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct TimeDomainEXT(pub i32);
impl std::fmt::Debug for TimeDomainEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            #[cfg(feature = "ext_calibrated_timestamps")]
            &Self::DEVICE_EXT => "DEVICE_EXT",
            #[cfg(feature = "ext_calibrated_timestamps")]
            &Self::CLOCK_MONOTONIC_EXT => "CLOCK_MONOTONIC_EXT",
            #[cfg(feature = "ext_calibrated_timestamps")]
            &Self::CLOCK_MONOTONIC_RAW_EXT => "CLOCK_MONOTONIC_RAW_EXT",
            #[cfg(feature = "ext_calibrated_timestamps")]
            &Self::QUERY_PERFORMANCE_COUNTER_EXT => "QUERY_PERFORMANCE_COUNTER_EXT",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::ext_calibrated_timestamps`]"]
impl crate::extensions::ext_calibrated_timestamps::TimeDomainEXT {
    pub const DEVICE_EXT: Self = Self(0);
    pub const CLOCK_MONOTONIC_EXT: Self = Self(1);
    pub const CLOCK_MONOTONIC_RAW_EXT: Self = Self(2);
    pub const QUERY_PERFORMANCE_COUNTER_EXT: Self = Self(3);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceCalibrateableTimeDomainsEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsEXT = unsafe extern "system" fn(physical_device: crate::vk1_0::PhysicalDevice, p_time_domain_count: *mut u32, p_time_domains: *mut crate::extensions::ext_calibrated_timestamps::TimeDomainEXT) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetCalibratedTimestampsEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetCalibratedTimestampsEXT = unsafe extern "system" fn(device: crate::vk1_0::Device, timestamp_count: u32, p_timestamp_infos: *const crate::extensions::ext_calibrated_timestamps::CalibratedTimestampInfoEXT, p_timestamps: *mut u64, p_max_deviation: *mut u64) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCalibratedTimestampInfoEXT.html) · Structure"]
#[doc(alias = "VkCalibratedTimestampInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CalibratedTimestampInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub time_domain: crate::extensions::ext_calibrated_timestamps::TimeDomainEXT,
}
impl Default for CalibratedTimestampInfoEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::CALIBRATED_TIMESTAMP_INFO_EXT, p_next: std::ptr::null(), time_domain: Default::default() }
    }
}
impl std::fmt::Debug for CalibratedTimestampInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CalibratedTimestampInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("time_domain", &self.time_domain).finish()
    }
}
impl CalibratedTimestampInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> CalibratedTimestampInfoEXTBuilder<'a> {
        CalibratedTimestampInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCalibratedTimestampInfoEXT.html) · Builder of [`CalibratedTimestampInfoEXT`]"]
#[repr(transparent)]
pub struct CalibratedTimestampInfoEXTBuilder<'a>(CalibratedTimestampInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> CalibratedTimestampInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> CalibratedTimestampInfoEXTBuilder<'a> {
        CalibratedTimestampInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn time_domain(mut self, time_domain: crate::extensions::ext_calibrated_timestamps::TimeDomainEXT) -> Self {
        self.0.time_domain = time_domain as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> CalibratedTimestampInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for CalibratedTimestampInfoEXTBuilder<'a> {
    fn default() -> CalibratedTimestampInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for CalibratedTimestampInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for CalibratedTimestampInfoEXTBuilder<'a> {
    type Target = CalibratedTimestampInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for CalibratedTimestampInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::ext_calibrated_timestamps`]"]
impl crate::InstanceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceCalibrateableTimeDomainsEXT.html) · Function"]
    #[doc(alias = "vkGetPhysicalDeviceCalibrateableTimeDomainsEXT")]
    pub unsafe fn get_physical_device_calibrateable_time_domains_ext(&self, physical_device: crate::vk1_0::PhysicalDevice, time_domain_count: Option<u32>) -> crate::utils::VulkanResult<Vec<crate::extensions::ext_calibrated_timestamps::TimeDomainEXT>> {
        let _function = self.get_physical_device_calibrateable_time_domains_ext.expect("tried to call a function that isn't loaded");
        let mut time_domain_count = match time_domain_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                _function(physical_device as _, &mut v, std::ptr::null_mut());
                v
            }
        };
        let mut time_domains = vec![Default::default(); time_domain_count as _];
        let _return = _function(physical_device as _, &mut time_domain_count, time_domains.as_mut_ptr());
        crate::utils::VulkanResult::new(_return, time_domains)
    }
}
#[doc = "Provided by [`crate::extensions::ext_calibrated_timestamps`]"]
impl crate::DeviceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetCalibratedTimestampsEXT.html) · Function"]
    #[doc(alias = "vkGetCalibratedTimestampsEXT")]
    pub unsafe fn get_calibrated_timestamps_ext(&self, timestamp_infos: &[crate::extensions::ext_calibrated_timestamps::CalibratedTimestampInfoEXTBuilder]) -> crate::utils::VulkanResult<(Vec<u64>, u64)> {
        let _function = self.get_calibrated_timestamps_ext.expect("tried to call a function that isn't loaded");
        let timestamp_count = timestamp_infos.len();
        let mut timestamps = vec![Default::default(); timestamp_count as _];
        let mut max_deviation = Default::default();
        let _return = _function(self.handle, timestamp_count as _, timestamp_infos.as_ptr() as _, timestamps.as_mut_ptr(), &mut max_deviation);
        crate::utils::VulkanResult::new(_return, (timestamps, max_deviation))
    }
}
