# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_EXT_calibrated_timestamps.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_CALIBRATED_TIMESTAMPS_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_CALIBRATED_TIMESTAMPS_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_calibrated_timestamps");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceCalibrateableTimeDomainsEXT.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsEXT =
    unsafe extern "system" fn(
        physical_device: crate::vk1_0::PhysicalDevice,
        p_time_domain_count: *mut u32,
        p_time_domains: *mut crate::extensions::ext_calibrated_timestamps::TimeDomainEXT,
    ) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetCalibratedTimestampsEXT.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetCalibratedTimestampsEXT = unsafe extern "system" fn ( device : crate :: vk1_0 :: Device , timestamp_count : u32 , p_timestamp_infos : * const crate :: extensions :: ext_calibrated_timestamps :: CalibratedTimestampInfoEXT , p_timestamps : * mut u64 , p_max_deviation : * mut u64 , ) -> crate :: vk1_0 :: Result ;
#[doc = "Provides Instance Commands for [`ExtCalibratedTimestampsInstanceLoaderExt`](trait.ExtCalibratedTimestampsInstanceLoaderExt.html)"]
pub struct ExtCalibratedTimestampsInstanceCommands {
    pub get_physical_device_calibrateable_time_domains_ext:
        Option<PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsEXT>,
}
impl ExtCalibratedTimestampsInstanceCommands {
    #[inline]
    pub fn load(loader: &crate::InstanceLoader) -> Option<ExtCalibratedTimestampsInstanceCommands> {
        unsafe {
            let mut success = false;
            let table = ExtCalibratedTimestampsInstanceCommands {
                get_physical_device_calibrateable_time_domains_ext: std::mem::transmute({
                    let symbol = loader.symbol("vkGetPhysicalDeviceCalibrateableTimeDomainsEXT");
                    success |= symbol.is_some();
                    symbol
                }),
            };
            if success {
                Some(table)
            } else {
                None
            }
        }
    }
}
fn instance_commands(loader: &crate::InstanceLoader) -> &ExtCalibratedTimestampsInstanceCommands {
    loader
        .ext_calibrated_timestamps
        .as_ref()
        .expect("`ext_calibrated_timestamps` not loaded")
}
#[doc = "Provides high level command wrappers for [`ExtCalibratedTimestampsInstanceCommands`](struct.ExtCalibratedTimestampsInstanceCommands.html)"]
pub trait ExtCalibratedTimestampsInstanceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceCalibrateableTimeDomainsEXT.html) · Instance Command"]
    unsafe fn get_physical_device_calibrateable_time_domains_ext(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        time_domain_count: Option<u32>,
    ) -> crate::utils::VulkanResult<Vec<crate::extensions::ext_calibrated_timestamps::TimeDomainEXT>>;
}
impl ExtCalibratedTimestampsInstanceLoaderExt for crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceCalibrateableTimeDomainsEXT.html) · Instance Command"]
    unsafe fn get_physical_device_calibrateable_time_domains_ext(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        time_domain_count: Option<u32>,
    ) -> crate::utils::VulkanResult<Vec<crate::extensions::ext_calibrated_timestamps::TimeDomainEXT>>
    {
        let function = instance_commands(self)
            .get_physical_device_calibrateable_time_domains_ext
            .as_ref()
            .expect("`get_physical_device_calibrateable_time_domains_ext` not available");
        let mut time_domain_count = time_domain_count.unwrap_or_else(|| {
            let mut val = Default::default();
            function(physical_device, &mut val, std::ptr::null_mut());
            val
        });
        let mut time_domains = vec![Default::default(); time_domain_count as _];
        let _val = function(
            physical_device,
            &mut time_domain_count,
            time_domains.as_mut_ptr(),
        );
        crate::utils::VulkanResult::new(_val, time_domains)
    }
}
#[doc = "Provides Device Commands for [`ExtCalibratedTimestampsDeviceLoaderExt`](trait.ExtCalibratedTimestampsDeviceLoaderExt.html)"]
pub struct ExtCalibratedTimestampsDeviceCommands {
    pub get_calibrated_timestamps_ext: Option<PFN_vkGetCalibratedTimestampsEXT>,
}
impl ExtCalibratedTimestampsDeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<ExtCalibratedTimestampsDeviceCommands> {
        unsafe {
            let mut success = false;
            let table = ExtCalibratedTimestampsDeviceCommands {
                get_calibrated_timestamps_ext: std::mem::transmute({
                    let symbol = loader.symbol("vkGetCalibratedTimestampsEXT");
                    success |= symbol.is_some();
                    symbol
                }),
            };
            if success {
                Some(table)
            } else {
                None
            }
        }
    }
}
fn device_commands(loader: &crate::DeviceLoader) -> &ExtCalibratedTimestampsDeviceCommands {
    loader
        .ext_calibrated_timestamps
        .as_ref()
        .expect("`ext_calibrated_timestamps` not loaded")
}
#[doc = "Provides high level command wrappers for [`ExtCalibratedTimestampsDeviceCommands`](struct.ExtCalibratedTimestampsDeviceCommands.html)"]
pub trait ExtCalibratedTimestampsDeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetCalibratedTimestampsEXT.html) · Device Command"]
    unsafe fn get_calibrated_timestamps_ext(
        &self,
        timestamp_infos : & [ crate :: extensions :: ext_calibrated_timestamps :: CalibratedTimestampInfoEXTBuilder ],
        max_deviation: Option<u64>,
    ) -> crate::utils::VulkanResult<(Vec<u64>, u64)>;
}
impl ExtCalibratedTimestampsDeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetCalibratedTimestampsEXT.html) · Device Command"]
    unsafe fn get_calibrated_timestamps_ext(
        &self,
        timestamp_infos : & [ crate :: extensions :: ext_calibrated_timestamps :: CalibratedTimestampInfoEXTBuilder ],
        max_deviation: Option<u64>,
    ) -> crate::utils::VulkanResult<(Vec<u64>, u64)> {
        let function = device_commands(self)
            .get_calibrated_timestamps_ext
            .as_ref()
            .expect("`get_calibrated_timestamps_ext` not available");
        let timestamp_count = timestamp_infos.len() as _;
        let mut timestamps = vec![Default::default(); timestamp_count as _];
        let mut max_deviation = max_deviation.unwrap_or_else(|| Default::default());
        let _val = function(
            self.handle,
            timestamp_count,
            timestamp_infos.as_ptr() as _,
            timestamps.as_mut_ptr(),
            &mut max_deviation,
        );
        crate::utils::VulkanResult::new(_val, (timestamps, max_deviation))
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkTimeDomainEXT.html) · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct TimeDomainEXT(pub i32);
#[doc = "[Part of `extensions::ext_calibrated_timestamps`](../../extensions/ext_calibrated_timestamps/index.html)"]
impl TimeDomainEXT {
    pub const DEVICE_EXT: Self = Self(0);
    pub const CLOCK_MONOTONIC_EXT: Self = Self(1);
    pub const CLOCK_MONOTONIC_RAW_EXT: Self = Self(2);
    pub const QUERY_PERFORMANCE_COUNTER_EXT: Self = Self(3);
}
impl std::fmt::Debug for TimeDomainEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::DEVICE_EXT => "DEVICE_EXT",
            &Self::CLOCK_MONOTONIC_EXT => "CLOCK_MONOTONIC_EXT",
            &Self::CLOCK_MONOTONIC_RAW_EXT => "CLOCK_MONOTONIC_RAW_EXT",
            &Self::QUERY_PERFORMANCE_COUNTER_EXT => "QUERY_PERFORMANCE_COUNTER_EXT",
            _ => "(unknown)",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCalibratedTimestampInfoEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CalibratedTimestampInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub time_domain: crate::extensions::ext_calibrated_timestamps::TimeDomainEXT,
}
impl CalibratedTimestampInfoEXT {
    #[inline]
    pub fn builder<'a>(self) -> CalibratedTimestampInfoEXTBuilder<'a> {
        CalibratedTimestampInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for CalibratedTimestampInfoEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("CalibratedTimestampInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("time_domain", &self.time_domain)
            .finish()
    }
}
impl Default for CalibratedTimestampInfoEXT {
    fn default() -> CalibratedTimestampInfoEXT {
        CalibratedTimestampInfoEXT {
            s_type: crate::vk1_0::StructureType::CALIBRATED_TIMESTAMP_INFO_EXT,
            p_next: std::ptr::null(),
            time_domain: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCalibratedTimestampInfoEXT.html) · Builder of [`CalibratedTimestampInfoEXT`](struct.CalibratedTimestampInfoEXT.html)"]
#[repr(transparent)]
pub struct CalibratedTimestampInfoEXTBuilder<'a>(
    CalibratedTimestampInfoEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> CalibratedTimestampInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> CalibratedTimestampInfoEXTBuilder<'a> {
        CalibratedTimestampInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn time_domain(
        mut self,
        time_domain: crate::extensions::ext_calibrated_timestamps::TimeDomainEXT,
    ) -> Self {
        self.0.time_domain = time_domain;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> CalibratedTimestampInfoEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for CalibratedTimestampInfoEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
