# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_KHR_performance_query.html)\n\n## Extends\n- [`QueryType`](../../vk1_0/struct.QueryType.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_PERFORMANCE_QUERY_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_PERFORMANCE_QUERY_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_performance_query");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR = unsafe extern "system" fn ( physical_device : crate :: vk1_0 :: PhysicalDevice , queue_family_index : u32 , p_counter_count : * mut u32 , p_counters : * mut crate :: extensions :: khr_performance_query :: PerformanceCounterKHR , p_counter_descriptions : * mut crate :: extensions :: khr_performance_query :: PerformanceCounterDescriptionKHR , ) -> crate :: vk1_0 :: Result ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR = unsafe extern "system" fn ( physical_device : crate :: vk1_0 :: PhysicalDevice , p_performance_query_create_info : * const crate :: extensions :: khr_performance_query :: QueryPoolPerformanceCreateInfoKHR , p_num_passes : * mut u32 , ) -> std :: ffi :: c_void ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAcquireProfilingLockKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkAcquireProfilingLockKHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_info: *const crate::extensions::khr_performance_query::AcquireProfilingLockInfoKHR,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkReleaseProfilingLockKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkReleaseProfilingLockKHR =
    unsafe extern "system" fn(device: crate::vk1_0::Device) -> std::ffi::c_void;
#[doc = "Provides Instance Commands for [`KhrPerformanceQueryInstanceLoaderExt`](trait.KhrPerformanceQueryInstanceLoaderExt.html)"]
pub struct KhrPerformanceQueryInstanceCommands {
    pub enumerate_physical_device_queue_family_performance_query_counters_khr:
        PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR,
    pub get_physical_device_queue_family_performance_query_passes_khr:
        PFN_vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR,
}
impl KhrPerformanceQueryInstanceCommands {
    #[inline]
    pub fn load(loader: &crate::InstanceLoader) -> Option<KhrPerformanceQueryInstanceCommands> {
        unsafe {
            Some(KhrPerformanceQueryInstanceCommands {
                enumerate_physical_device_queue_family_performance_query_counters_khr:
                    std::mem::transmute(loader.symbol(
                        "vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR",
                    )?),
                get_physical_device_queue_family_performance_query_passes_khr: std::mem::transmute(
                    loader.symbol("vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR")?,
                ),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`KhrPerformanceQueryInstanceCommands`](struct.KhrPerformanceQueryInstanceCommands.html)"]
pub trait KhrPerformanceQueryInstanceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR.html) · Instance Command"]
    unsafe fn enumerate_physical_device_queue_family_performance_query_counters_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        queue_family_index: u32,
        counter_count: Option<u32>,
    ) -> crate::utils::VulkanResult<(
        Vec<crate::extensions::khr_performance_query::PerformanceCounterKHR>,
        Vec<crate::extensions::khr_performance_query::PerformanceCounterDescriptionKHR>,
    )>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR.html) · Instance Command"]
    unsafe fn get_physical_device_queue_family_performance_query_passes_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        performance_query_create_info : & crate :: extensions :: khr_performance_query :: QueryPoolPerformanceCreateInfoKHR,
        num_passes: Option<u32>,
    ) -> u32;
}
impl KhrPerformanceQueryInstanceLoaderExt for crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR.html) · Instance Command"]
    unsafe fn enumerate_physical_device_queue_family_performance_query_counters_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        queue_family_index: u32,
        counter_count: Option<u32>,
    ) -> crate::utils::VulkanResult<(
        Vec<crate::extensions::khr_performance_query::PerformanceCounterKHR>,
        Vec<crate::extensions::khr_performance_query::PerformanceCounterDescriptionKHR>,
    )> {
        let function = self
            .khr_performance_query
            .as_ref()
            .expect("`khr_performance_query` not loaded")
            .enumerate_physical_device_queue_family_performance_query_counters_khr;
        let mut counter_count = counter_count.unwrap_or_else(|| {
            let mut val = Default::default();
            function(
                physical_device,
                queue_family_index,
                &mut val,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            val
        });
        let mut counters = vec![Default::default(); counter_count as _];
        let mut counter_descriptions = vec![Default::default(); counter_count as _];
        let _val = function(
            physical_device,
            queue_family_index,
            &mut counter_count,
            counters.as_mut_ptr(),
            counter_descriptions.as_mut_ptr(),
        );
        crate::utils::VulkanResult::new(_val, (counters, counter_descriptions))
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR.html) · Instance Command"]
    unsafe fn get_physical_device_queue_family_performance_query_passes_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        performance_query_create_info : & crate :: extensions :: khr_performance_query :: QueryPoolPerformanceCreateInfoKHR,
        num_passes: Option<u32>,
    ) -> u32 {
        let function = self
            .khr_performance_query
            .as_ref()
            .expect("`khr_performance_query` not loaded")
            .get_physical_device_queue_family_performance_query_passes_khr;
        let mut num_passes = num_passes.unwrap_or_else(|| Default::default());
        let _val = function(
            physical_device,
            performance_query_create_info,
            &mut num_passes,
        );
        num_passes
    }
}
#[doc = "Provides Device Commands for [`KhrPerformanceQueryDeviceLoaderExt`](trait.KhrPerformanceQueryDeviceLoaderExt.html)"]
pub struct KhrPerformanceQueryDeviceCommands {
    pub acquire_profiling_lock_khr: PFN_vkAcquireProfilingLockKHR,
    pub release_profiling_lock_khr: PFN_vkReleaseProfilingLockKHR,
}
impl KhrPerformanceQueryDeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<KhrPerformanceQueryDeviceCommands> {
        unsafe {
            Some(KhrPerformanceQueryDeviceCommands {
                acquire_profiling_lock_khr: std::mem::transmute(
                    loader.symbol("vkAcquireProfilingLockKHR")?,
                ),
                release_profiling_lock_khr: std::mem::transmute(
                    loader.symbol("vkReleaseProfilingLockKHR")?,
                ),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`KhrPerformanceQueryDeviceCommands`](struct.KhrPerformanceQueryDeviceCommands.html)"]
pub trait KhrPerformanceQueryDeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAcquireProfilingLockKHR.html) · Device Command"]
    unsafe fn acquire_profiling_lock_khr(
        &self,
        info: &crate::extensions::khr_performance_query::AcquireProfilingLockInfoKHR,
    ) -> crate::utils::VulkanResult<()>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkReleaseProfilingLockKHR.html) · Device Command"]
    unsafe fn release_profiling_lock_khr(&self) -> ();
}
impl KhrPerformanceQueryDeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAcquireProfilingLockKHR.html) · Device Command"]
    unsafe fn acquire_profiling_lock_khr(
        &self,
        info: &crate::extensions::khr_performance_query::AcquireProfilingLockInfoKHR,
    ) -> crate::utils::VulkanResult<()> {
        let function = self
            .khr_performance_query
            .as_ref()
            .expect("`khr_performance_query` not loaded")
            .acquire_profiling_lock_khr;
        let _val = function(self.handle, info);
        crate::utils::VulkanResult::new(_val, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkReleaseProfilingLockKHR.html) · Device Command"]
    unsafe fn release_profiling_lock_khr(&self) -> () {
        let function = self
            .khr_performance_query
            .as_ref()
            .expect("`khr_performance_query` not loaded")
            .release_profiling_lock_khr;
        let _val = function(self.handle);
        ()
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceCounterKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PerformanceCounterKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub unit: crate::extensions::khr_performance_query::PerformanceCounterUnitKHR,
    pub scope: crate::extensions::khr_performance_query::PerformanceCounterScopeKHR,
    pub storage: crate::extensions::khr_performance_query::PerformanceCounterStorageKHR,
    pub uuid: [u8; crate::vk1_0::UUID_SIZE as usize],
}
impl PerformanceCounterKHR {
    #[inline]
    pub fn builder<'a>(self) -> PerformanceCounterKHRBuilder<'a> {
        PerformanceCounterKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PerformanceCounterKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PerformanceCounterKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("unit", &self.unit)
            .field("scope", &self.scope)
            .field("storage", &self.storage)
            .field("uuid", &self.uuid)
            .finish()
    }
}
impl Default for PerformanceCounterKHR {
    fn default() -> PerformanceCounterKHR {
        PerformanceCounterKHR {
            s_type: crate::vk1_0::StructureType::PERFORMANCE_COUNTER_KHR,
            p_next: std::ptr::null(),
            unit: Default::default(),
            scope: Default::default(),
            storage: Default::default(),
            uuid: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PerformanceCounterKHR`](struct.PerformanceCounterKHR.html)"]
#[repr(transparent)]
pub struct PerformanceCounterKHRBuilder<'a>(
    PerformanceCounterKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PerformanceCounterKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PerformanceCounterKHRBuilder<'a> {
        PerformanceCounterKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn unit(
        mut self,
        unit: crate::extensions::khr_performance_query::PerformanceCounterUnitKHR,
    ) -> Self {
        self.0.unit = unit;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn scope(
        mut self,
        scope: crate::extensions::khr_performance_query::PerformanceCounterScopeKHR,
    ) -> Self {
        self.0.scope = scope;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn storage(
        mut self,
        storage: crate::extensions::khr_performance_query::PerformanceCounterStorageKHR,
    ) -> Self {
        self.0.storage = storage;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn uuid(mut self, uuid: [u8; crate::vk1_0::UUID_SIZE as usize]) -> Self {
        self.0.uuid = uuid;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PerformanceCounterKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for PerformanceCounterKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PerformanceCounterKHRBuilder<'a> {
    type Target = PerformanceCounterKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PerformanceCounterKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceCounterUnitKHR.html) · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PerformanceCounterUnitKHR(pub i32);
#[doc = "[Part of `extensions::khr_performance_query`](../../extensions/khr_performance_query/index.html)"]
impl PerformanceCounterUnitKHR {
    pub const GENERIC_KHR: Self = Self(0);
    pub const PERCENTAGE_KHR: Self = Self(1);
    pub const NANOSECONDS_KHR: Self = Self(2);
    pub const BYTES_KHR: Self = Self(3);
    pub const BYTES_PER_SECOND_KHR: Self = Self(4);
    pub const KELVIN_KHR: Self = Self(5);
    pub const WATTS_KHR: Self = Self(6);
    pub const VOLTS_KHR: Self = Self(7);
    pub const AMPS_KHR: Self = Self(8);
    pub const HERTZ_KHR: Self = Self(9);
    pub const CYCLES_KHR: Self = Self(10);
}
impl std::fmt::Debug for PerformanceCounterUnitKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::GENERIC_KHR => "GENERIC_KHR",
            &Self::PERCENTAGE_KHR => "PERCENTAGE_KHR",
            &Self::NANOSECONDS_KHR => "NANOSECONDS_KHR",
            &Self::BYTES_KHR => "BYTES_KHR",
            &Self::BYTES_PER_SECOND_KHR => "BYTES_PER_SECOND_KHR",
            &Self::KELVIN_KHR => "KELVIN_KHR",
            &Self::WATTS_KHR => "WATTS_KHR",
            &Self::VOLTS_KHR => "VOLTS_KHR",
            &Self::AMPS_KHR => "AMPS_KHR",
            &Self::HERTZ_KHR => "HERTZ_KHR",
            &Self::CYCLES_KHR => "CYCLES_KHR",
            _ => "(unknown)",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceCounterScopeKHR.html) · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PerformanceCounterScopeKHR(pub i32);
#[doc = "[Part of `extensions::khr_performance_query`](../../extensions/khr_performance_query/index.html)"]
impl PerformanceCounterScopeKHR {
    pub const COMMAND_BUFFER_KHR: Self = Self(0);
    pub const RENDER_PASS_KHR: Self = Self(1);
    pub const COMMAND_KHR: Self = Self(2);
    pub const QUERY_SCOPE_COMMAND_BUFFER_KHR: Self = Self::COMMAND_BUFFER_KHR;
    pub const QUERY_SCOPE_RENDER_PASS_KHR: Self = Self::RENDER_PASS_KHR;
    pub const QUERY_SCOPE_COMMAND_KHR: Self = Self::COMMAND_KHR;
}
impl std::fmt::Debug for PerformanceCounterScopeKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::COMMAND_BUFFER_KHR => "COMMAND_BUFFER_KHR",
            &Self::RENDER_PASS_KHR => "RENDER_PASS_KHR",
            &Self::COMMAND_KHR => "COMMAND_KHR",
            _ => "(unknown)",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceCounterStorageKHR.html) · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PerformanceCounterStorageKHR(pub i32);
#[doc = "[Part of `extensions::khr_performance_query`](../../extensions/khr_performance_query/index.html)"]
impl PerformanceCounterStorageKHR {
    pub const INT32_KHR: Self = Self(0);
    pub const INT64_KHR: Self = Self(1);
    pub const UINT32_KHR: Self = Self(2);
    pub const UINT64_KHR: Self = Self(3);
    pub const FLOAT32_KHR: Self = Self(4);
    pub const FLOAT64_KHR: Self = Self(5);
}
impl std::fmt::Debug for PerformanceCounterStorageKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::INT32_KHR => "INT32_KHR",
            &Self::INT64_KHR => "INT64_KHR",
            &Self::UINT32_KHR => "UINT32_KHR",
            &Self::UINT64_KHR => "UINT64_KHR",
            &Self::FLOAT32_KHR => "FLOAT32_KHR",
            &Self::FLOAT64_KHR => "FLOAT64_KHR",
            _ => "(unknown)",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceCounterDescriptionKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PerformanceCounterDescriptionKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::khr_performance_query::PerformanceCounterDescriptionFlagsKHR,
    pub name: [std::os::raw::c_char; crate::vk1_0::MAX_DESCRIPTION_SIZE as usize],
    pub category: [std::os::raw::c_char; crate::vk1_0::MAX_DESCRIPTION_SIZE as usize],
    pub description: [std::os::raw::c_char; crate::vk1_0::MAX_DESCRIPTION_SIZE as usize],
}
impl PerformanceCounterDescriptionKHR {
    #[inline]
    pub fn builder<'a>(self) -> PerformanceCounterDescriptionKHRBuilder<'a> {
        PerformanceCounterDescriptionKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PerformanceCounterDescriptionKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PerformanceCounterDescriptionKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("name", &unsafe {
                std::ffi::CStr::from_ptr(self.name.as_ptr() as _)
            })
            .field("category", &unsafe {
                std::ffi::CStr::from_ptr(self.category.as_ptr() as _)
            })
            .field("description", &unsafe {
                std::ffi::CStr::from_ptr(self.description.as_ptr() as _)
            })
            .finish()
    }
}
impl Default for PerformanceCounterDescriptionKHR {
    fn default() -> PerformanceCounterDescriptionKHR {
        PerformanceCounterDescriptionKHR {
            s_type: crate::vk1_0::StructureType::PERFORMANCE_COUNTER_DESCRIPTION_KHR,
            p_next: std::ptr::null(),
            flags: Default::default(),
            name: unsafe { std::mem::zeroed() },
            category: unsafe { std::mem::zeroed() },
            description: unsafe { std::mem::zeroed() },
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PerformanceCounterDescriptionKHR`](struct.PerformanceCounterDescriptionKHR.html)"]
#[repr(transparent)]
pub struct PerformanceCounterDescriptionKHRBuilder<'a>(
    PerformanceCounterDescriptionKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PerformanceCounterDescriptionKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PerformanceCounterDescriptionKHRBuilder<'a> {
        PerformanceCounterDescriptionKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn flags(
        mut self,
        flags: crate::extensions::khr_performance_query::PerformanceCounterDescriptionFlagsKHR,
    ) -> Self {
        self.0.flags = flags;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn name(
        mut self,
        name: [std::os::raw::c_char; crate::vk1_0::MAX_DESCRIPTION_SIZE as usize],
    ) -> Self {
        self.0.name = name;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn category(
        mut self,
        category: [std::os::raw::c_char; crate::vk1_0::MAX_DESCRIPTION_SIZE as usize],
    ) -> Self {
        self.0.category = category;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn description(
        mut self,
        description: [std::os::raw::c_char; crate::vk1_0::MAX_DESCRIPTION_SIZE as usize],
    ) -> Self {
        self.0.description = description;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PerformanceCounterDescriptionKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for PerformanceCounterDescriptionKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PerformanceCounterDescriptionKHRBuilder<'a> {
    type Target = PerformanceCounterDescriptionKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PerformanceCounterDescriptionKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceCounterDescriptionFlagBitsKHR.html) · Flag Bits of [`PerformanceCounterDescriptionFlagsKHR`](struct.PerformanceCounterDescriptionFlagsKHR.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PerformanceCounterDescriptionFlagBitsKHR(pub u32);
impl PerformanceCounterDescriptionFlagBitsKHR {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> PerformanceCounterDescriptionFlagsKHR {
        PerformanceCounterDescriptionFlagsKHR::from_bits_truncate(self.0)
    }
}
#[doc = "[Part of `extensions::khr_performance_query`](../../extensions/khr_performance_query/index.html)"]
impl PerformanceCounterDescriptionFlagBitsKHR {
    pub const PERFORMANCE_IMPACTING_KHR: Self = Self(0x00000001);
    pub const CONCURRENTLY_IMPACTED_KHR: Self = Self(0x00000002);
}
impl std::fmt::Debug for PerformanceCounterDescriptionFlagBitsKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::PERFORMANCE_IMPACTING_KHR => "PERFORMANCE_IMPACTING_KHR",
            &Self::CONCURRENTLY_IMPACTED_KHR => "CONCURRENTLY_IMPACTED_KHR",
            _ => "(unknown)",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceCounterDescriptionFlagsKHR.html) · Flags of [`PerformanceCounterDescriptionFlagBitsKHR`](struct.PerformanceCounterDescriptionFlagBitsKHR.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct PerformanceCounterDescriptionFlagsKHR : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; const PERFORMANCE_IMPACTING_KHR = PerformanceCounterDescriptionFlagBitsKHR :: PERFORMANCE_IMPACTING_KHR . 0 ; const CONCURRENTLY_IMPACTED_KHR = PerformanceCounterDescriptionFlagBitsKHR :: CONCURRENTLY_IMPACTED_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueryPoolPerformanceCreateInfoKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct QueryPoolPerformanceCreateInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub queue_family_index: u32,
    pub counter_index_count: u32,
    pub p_counter_indices: *const u32,
}
impl QueryPoolPerformanceCreateInfoKHR {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByQueryPoolPerformanceCreateInfoKHR,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> QueryPoolPerformanceCreateInfoKHRBuilder<'a> {
        QueryPoolPerformanceCreateInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for QueryPoolPerformanceCreateInfoKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("QueryPoolPerformanceCreateInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("queue_family_index", &self.queue_family_index)
            .field("counter_index_count", &self.counter_index_count)
            .field("p_counter_indices", &self.p_counter_indices)
            .finish()
    }
}
impl Default for QueryPoolPerformanceCreateInfoKHR {
    fn default() -> QueryPoolPerformanceCreateInfoKHR {
        QueryPoolPerformanceCreateInfoKHR {
            s_type: crate::vk1_0::StructureType::QUERY_POOL_PERFORMANCE_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            queue_family_index: Default::default(),
            counter_index_count: Default::default(),
            p_counter_indices: std::ptr::null(),
        }
    }
}
#[doc = "Used by [`QueryPoolPerformanceCreateInfoKHR::extend`](struct.QueryPoolPerformanceCreateInfoKHR.html#method.extend)"]
pub trait ExtendableByQueryPoolPerformanceCreateInfoKHR {}
impl ExtendableByQueryPoolPerformanceCreateInfoKHR for crate::vk1_0::QueryPoolCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`QueryPoolPerformanceCreateInfoKHR`](struct.QueryPoolPerformanceCreateInfoKHR.html)"]
#[repr(transparent)]
pub struct QueryPoolPerformanceCreateInfoKHRBuilder<'a>(
    QueryPoolPerformanceCreateInfoKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> QueryPoolPerformanceCreateInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> QueryPoolPerformanceCreateInfoKHRBuilder<'a> {
        QueryPoolPerformanceCreateInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn queue_family_index(mut self, queue_family_index: u32) -> Self {
        self.0.queue_family_index = queue_family_index;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn counter_indices(mut self, counter_indices: &'a [u32]) -> Self {
        self.0.counter_index_count = counter_indices.len() as _;
        self.0.p_counter_indices = counter_indices.as_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> QueryPoolPerformanceCreateInfoKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for QueryPoolPerformanceCreateInfoKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for QueryPoolPerformanceCreateInfoKHRBuilder<'a> {
    type Target = QueryPoolPerformanceCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for QueryPoolPerformanceCreateInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAcquireProfilingLockInfoKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AcquireProfilingLockInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::khr_performance_query::AcquireProfilingLockFlagsKHR,
    pub timeout: u64,
}
impl AcquireProfilingLockInfoKHR {
    #[inline]
    pub fn builder<'a>(self) -> AcquireProfilingLockInfoKHRBuilder<'a> {
        AcquireProfilingLockInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for AcquireProfilingLockInfoKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("AcquireProfilingLockInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("timeout", &self.timeout)
            .finish()
    }
}
impl Default for AcquireProfilingLockInfoKHR {
    fn default() -> AcquireProfilingLockInfoKHR {
        AcquireProfilingLockInfoKHR {
            s_type: crate::vk1_0::StructureType::ACQUIRE_PROFILING_LOCK_INFO_KHR,
            p_next: std::ptr::null(),
            flags: Default::default(),
            timeout: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`AcquireProfilingLockInfoKHR`](struct.AcquireProfilingLockInfoKHR.html)"]
#[repr(transparent)]
pub struct AcquireProfilingLockInfoKHRBuilder<'a>(
    AcquireProfilingLockInfoKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> AcquireProfilingLockInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> AcquireProfilingLockInfoKHRBuilder<'a> {
        AcquireProfilingLockInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn flags(
        mut self,
        flags: crate::extensions::khr_performance_query::AcquireProfilingLockFlagsKHR,
    ) -> Self {
        self.0.flags = flags;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn timeout(mut self, timeout: u64) -> Self {
        self.0.timeout = timeout;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> AcquireProfilingLockInfoKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for AcquireProfilingLockInfoKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for AcquireProfilingLockInfoKHRBuilder<'a> {
    type Target = AcquireProfilingLockInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for AcquireProfilingLockInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Flag Bits of [`AcquireProfilingLockFlagsKHR`](struct.AcquireProfilingLockFlagsKHR.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct AcquireProfilingLockFlagBitsKHR(pub u32);
impl AcquireProfilingLockFlagBitsKHR {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> AcquireProfilingLockFlagsKHR {
        AcquireProfilingLockFlagsKHR::from_bits_truncate(self.0)
    }
}
#[doc = "[Part of `extensions::khr_performance_query`](../../extensions/khr_performance_query/index.html)"]
impl AcquireProfilingLockFlagBitsKHR {}
impl std::fmt::Debug for AcquireProfilingLockFlagBitsKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            _ => "(unknown)",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAcquireProfilingLockFlagsKHR.html) · Flags of [`AcquireProfilingLockFlagBitsKHR`](struct.AcquireProfilingLockFlagBitsKHR.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct AcquireProfilingLockFlagsKHR : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevicePerformanceQueryFeaturesKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDevicePerformanceQueryFeaturesKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub performance_counter_query_pools: crate::vk1_0::Bool32,
    pub performance_counter_multiple_query_pools: crate::vk1_0::Bool32,
}
impl PhysicalDevicePerformanceQueryFeaturesKHR {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDevicePerformanceQueryFeaturesKHR,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDevicePerformanceQueryFeaturesKHRBuilder<'a> {
        PhysicalDevicePerformanceQueryFeaturesKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDevicePerformanceQueryFeaturesKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDevicePerformanceQueryFeaturesKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "performance_counter_query_pools",
                &(self.performance_counter_query_pools != 0),
            )
            .field(
                "performance_counter_multiple_query_pools",
                &(self.performance_counter_multiple_query_pools != 0),
            )
            .finish()
    }
}
impl Default for PhysicalDevicePerformanceQueryFeaturesKHR {
    fn default() -> PhysicalDevicePerformanceQueryFeaturesKHR {
        PhysicalDevicePerformanceQueryFeaturesKHR {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_PERFORMANCE_QUERY_FEATURES_KHR,
            p_next: std::ptr::null_mut(),
            performance_counter_query_pools: Default::default(),
            performance_counter_multiple_query_pools: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDevicePerformanceQueryFeaturesKHR::extend`](struct.PhysicalDevicePerformanceQueryFeaturesKHR.html#method.extend)"]
pub trait ExtendableByPhysicalDevicePerformanceQueryFeaturesKHR {}
impl ExtendableByPhysicalDevicePerformanceQueryFeaturesKHR
    for crate::vk1_1::PhysicalDeviceFeatures2
{
}
impl ExtendableByPhysicalDevicePerformanceQueryFeaturesKHR for crate::vk1_0::DeviceCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDevicePerformanceQueryFeaturesKHR`](struct.PhysicalDevicePerformanceQueryFeaturesKHR.html)"]
#[repr(transparent)]
pub struct PhysicalDevicePerformanceQueryFeaturesKHRBuilder<'a>(
    PhysicalDevicePerformanceQueryFeaturesKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDevicePerformanceQueryFeaturesKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDevicePerformanceQueryFeaturesKHRBuilder<'a> {
        PhysicalDevicePerformanceQueryFeaturesKHRBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn performance_counter_query_pools(
        mut self,
        performance_counter_query_pools: bool,
    ) -> Self {
        self.0.performance_counter_query_pools = performance_counter_query_pools as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn performance_counter_multiple_query_pools(
        mut self,
        performance_counter_multiple_query_pools: bool,
    ) -> Self {
        self.0.performance_counter_multiple_query_pools =
            performance_counter_multiple_query_pools as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDevicePerformanceQueryFeaturesKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDevicePerformanceQueryFeaturesKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDevicePerformanceQueryFeaturesKHRBuilder<'a> {
    type Target = PhysicalDevicePerformanceQueryFeaturesKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDevicePerformanceQueryFeaturesKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevicePerformanceQueryPropertiesKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDevicePerformanceQueryPropertiesKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub allow_command_buffer_query_copies: crate::vk1_0::Bool32,
}
impl PhysicalDevicePerformanceQueryPropertiesKHR {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDevicePerformanceQueryPropertiesKHR,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDevicePerformanceQueryPropertiesKHRBuilder<'a> {
        PhysicalDevicePerformanceQueryPropertiesKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDevicePerformanceQueryPropertiesKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDevicePerformanceQueryPropertiesKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "allow_command_buffer_query_copies",
                &(self.allow_command_buffer_query_copies != 0),
            )
            .finish()
    }
}
impl Default for PhysicalDevicePerformanceQueryPropertiesKHR {
    fn default() -> PhysicalDevicePerformanceQueryPropertiesKHR {
        PhysicalDevicePerformanceQueryPropertiesKHR {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_PERFORMANCE_QUERY_PROPERTIES_KHR,
            p_next: std::ptr::null_mut(),
            allow_command_buffer_query_copies: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDevicePerformanceQueryPropertiesKHR::extend`](struct.PhysicalDevicePerformanceQueryPropertiesKHR.html#method.extend)"]
pub trait ExtendableByPhysicalDevicePerformanceQueryPropertiesKHR {}
impl ExtendableByPhysicalDevicePerformanceQueryPropertiesKHR
    for crate::vk1_1::PhysicalDeviceProperties2
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDevicePerformanceQueryPropertiesKHR`](struct.PhysicalDevicePerformanceQueryPropertiesKHR.html)"]
#[repr(transparent)]
pub struct PhysicalDevicePerformanceQueryPropertiesKHRBuilder<'a>(
    PhysicalDevicePerformanceQueryPropertiesKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDevicePerformanceQueryPropertiesKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDevicePerformanceQueryPropertiesKHRBuilder<'a> {
        PhysicalDevicePerformanceQueryPropertiesKHRBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn allow_command_buffer_query_copies(
        mut self,
        allow_command_buffer_query_copies: bool,
    ) -> Self {
        self.0.allow_command_buffer_query_copies = allow_command_buffer_query_copies as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDevicePerformanceQueryPropertiesKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDevicePerformanceQueryPropertiesKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDevicePerformanceQueryPropertiesKHRBuilder<'a> {
    type Target = PhysicalDevicePerformanceQueryPropertiesKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDevicePerformanceQueryPropertiesKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceCounterResultKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub union PerformanceCounterResultKHR {
    pub int32: i32,
    pub int64: i64,
    pub uint32: u32,
    pub uint64: u64,
    pub float32: f32,
    pub float64: f64,
}
impl std::fmt::Debug for PerformanceCounterResultKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PerformanceCounterResultKHR").finish()
    }
}
impl Default for PerformanceCounterResultKHR {
    fn default() -> PerformanceCounterResultKHR {
        unsafe { std::mem::zeroed() }
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceQuerySubmitInfoKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PerformanceQuerySubmitInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub counter_pass_index: u32,
}
impl PerformanceQuerySubmitInfoKHR {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPerformanceQuerySubmitInfoKHR,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PerformanceQuerySubmitInfoKHRBuilder<'a> {
        PerformanceQuerySubmitInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PerformanceQuerySubmitInfoKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PerformanceQuerySubmitInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("counter_pass_index", &self.counter_pass_index)
            .finish()
    }
}
impl Default for PerformanceQuerySubmitInfoKHR {
    fn default() -> PerformanceQuerySubmitInfoKHR {
        PerformanceQuerySubmitInfoKHR {
            s_type: crate::vk1_0::StructureType::PERFORMANCE_QUERY_SUBMIT_INFO_KHR,
            p_next: std::ptr::null(),
            counter_pass_index: Default::default(),
        }
    }
}
#[doc = "Used by [`PerformanceQuerySubmitInfoKHR::extend`](struct.PerformanceQuerySubmitInfoKHR.html#method.extend)"]
pub trait ExtendableByPerformanceQuerySubmitInfoKHR {}
impl ExtendableByPerformanceQuerySubmitInfoKHR for crate::vk1_0::SubmitInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PerformanceQuerySubmitInfoKHR`](struct.PerformanceQuerySubmitInfoKHR.html)"]
#[repr(transparent)]
pub struct PerformanceQuerySubmitInfoKHRBuilder<'a>(
    PerformanceQuerySubmitInfoKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PerformanceQuerySubmitInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PerformanceQuerySubmitInfoKHRBuilder<'a> {
        PerformanceQuerySubmitInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn counter_pass_index(mut self, counter_pass_index: u32) -> Self {
        self.0.counter_pass_index = counter_pass_index;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PerformanceQuerySubmitInfoKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for PerformanceQuerySubmitInfoKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PerformanceQuerySubmitInfoKHRBuilder<'a> {
    type Target = PerformanceQuerySubmitInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PerformanceQuerySubmitInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
