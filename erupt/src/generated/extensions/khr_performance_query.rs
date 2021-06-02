#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_PERFORMANCE_QUERY_SPEC_VERSION")]
pub const KHR_PERFORMANCE_QUERY_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_PERFORMANCE_QUERY_EXTENSION_NAME")]
pub const KHR_PERFORMANCE_QUERY_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_performance_query");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_ENUMERATE_PHYSICAL_DEVICE_QUEUE_FAMILY_PERFORMANCE_QUERY_COUNTERS_KHR: *const std::os::raw::c_char = crate::cstr!("vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_PHYSICAL_DEVICE_QUEUE_FAMILY_PERFORMANCE_QUERY_PASSES_KHR: *const std::os::raw::c_char = crate::cstr!("vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_ACQUIRE_PROFILING_LOCK_KHR: *const std::os::raw::c_char = crate::cstr!("vkAcquireProfilingLockKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_RELEASE_PROFILING_LOCK_KHR: *const std::os::raw::c_char = crate::cstr!("vkReleaseProfilingLockKHR");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceCounterScopeKHR.html) · Enum"]
#[doc(alias = "VkPerformanceCounterScopeKHR")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct PerformanceCounterScopeKHR(pub i32);
impl std::fmt::Debug for PerformanceCounterScopeKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::COMMAND_BUFFER_KHR => "COMMAND_BUFFER_KHR",
            &Self::RENDER_PASS_KHR => "RENDER_PASS_KHR",
            &Self::COMMAND_KHR => "COMMAND_KHR",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::khr_performance_query`]"]
impl PerformanceCounterScopeKHR {
    pub const COMMAND_BUFFER_KHR: Self = Self(0);
    pub const RENDER_PASS_KHR: Self = Self(1);
    pub const COMMAND_KHR: Self = Self(2);
    pub const QUERY_SCOPE_COMMAND_BUFFER_KHR: Self = Self::COMMAND_BUFFER_KHR;
    pub const QUERY_SCOPE_RENDER_PASS_KHR: Self = Self::RENDER_PASS_KHR;
    pub const QUERY_SCOPE_COMMAND_KHR: Self = Self::COMMAND_KHR;
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceCounterUnitKHR.html) · Enum"]
#[doc(alias = "VkPerformanceCounterUnitKHR")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct PerformanceCounterUnitKHR(pub i32);
impl std::fmt::Debug for PerformanceCounterUnitKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
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
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::khr_performance_query`]"]
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceCounterStorageKHR.html) · Enum"]
#[doc(alias = "VkPerformanceCounterStorageKHR")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct PerformanceCounterStorageKHR(pub i32);
impl std::fmt::Debug for PerformanceCounterStorageKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::INT32_KHR => "INT32_KHR",
            &Self::INT64_KHR => "INT64_KHR",
            &Self::UINT32_KHR => "UINT32_KHR",
            &Self::UINT64_KHR => "UINT64_KHR",
            &Self::FLOAT32_KHR => "FLOAT32_KHR",
            &Self::FLOAT64_KHR => "FLOAT64_KHR",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::khr_performance_query`]"]
impl PerformanceCounterStorageKHR {
    pub const INT32_KHR: Self = Self(0);
    pub const INT64_KHR: Self = Self(1);
    pub const UINT32_KHR: Self = Self(2);
    pub const UINT64_KHR: Self = Self(3);
    pub const FLOAT32_KHR: Self = Self(4);
    pub const FLOAT64_KHR: Self = Self(5);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceCounterDescriptionFlagsKHR.html) · Bitmask of [`PerformanceCounterDescriptionFlagBitsKHR`]"] # [doc (alias = "VkPerformanceCounterDescriptionFlagsKHR")] # [derive (Default)] # [repr (transparent)] pub struct PerformanceCounterDescriptionFlagsKHR : u32 { const PERFORMANCE_IMPACTING_KHR = PerformanceCounterDescriptionFlagBitsKHR :: PERFORMANCE_IMPACTING_KHR . 0 ; const CONCURRENTLY_IMPACTED_KHR = PerformanceCounterDescriptionFlagBitsKHR :: CONCURRENTLY_IMPACTED_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceCounterDescriptionFlagBitsKHR.html) · Bits enum of [`PerformanceCounterDescriptionFlagsKHR`]"]
#[doc(alias = "VkPerformanceCounterDescriptionFlagBitsKHR")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct PerformanceCounterDescriptionFlagBitsKHR(pub u32);
impl PerformanceCounterDescriptionFlagBitsKHR {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> PerformanceCounterDescriptionFlagsKHR {
        PerformanceCounterDescriptionFlagsKHR::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for PerformanceCounterDescriptionFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::PERFORMANCE_IMPACTING_KHR => "PERFORMANCE_IMPACTING_KHR",
            &Self::CONCURRENTLY_IMPACTED_KHR => "CONCURRENTLY_IMPACTED_KHR",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::khr_performance_query`]"]
impl PerformanceCounterDescriptionFlagBitsKHR {
    pub const PERFORMANCE_IMPACTING_KHR: Self = Self(1);
    pub const CONCURRENTLY_IMPACTED_KHR: Self = Self(2);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAcquireProfilingLockFlagsKHR.html) · Bitmask of [`AcquireProfilingLockFlagBitsKHR`]"] # [doc (alias = "VkAcquireProfilingLockFlagsKHR")] # [derive (Default)] # [repr (transparent)] pub struct AcquireProfilingLockFlagsKHR : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`AcquireProfilingLockFlagsKHR`]"]
#[doc(alias = "VkAcquireProfilingLockFlagBitsKHR")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct AcquireProfilingLockFlagBitsKHR(pub u32);
impl AcquireProfilingLockFlagBitsKHR {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> AcquireProfilingLockFlagsKHR {
        AcquireProfilingLockFlagsKHR::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for AcquireProfilingLockFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR = unsafe extern "system" fn(physical_device: crate::vk1_0::PhysicalDevice, queue_family_index: u32, p_counter_count: *mut u32, p_counters: *mut crate::extensions::khr_performance_query::PerformanceCounterKHR, p_counter_descriptions: *mut crate::extensions::khr_performance_query::PerformanceCounterDescriptionKHR) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR = unsafe extern "system" fn(physical_device: crate::vk1_0::PhysicalDevice, p_performance_query_create_info: *const crate::extensions::khr_performance_query::QueryPoolPerformanceCreateInfoKHR, p_num_passes: *mut u32) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAcquireProfilingLockKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkAcquireProfilingLockKHR = unsafe extern "system" fn(device: crate::vk1_0::Device, p_info: *const crate::extensions::khr_performance_query::AcquireProfilingLockInfoKHR) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkReleaseProfilingLockKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkReleaseProfilingLockKHR = unsafe extern "system" fn(device: crate::vk1_0::Device) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevicePerformanceQueryFeaturesKHR.html) · Structure"]
#[doc(alias = "VkPhysicalDevicePerformanceQueryFeaturesKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDevicePerformanceQueryFeaturesKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub performance_counter_query_pools: crate::vk1_0::Bool32,
    pub performance_counter_multiple_query_pools: crate::vk1_0::Bool32,
}
impl Default for PhysicalDevicePerformanceQueryFeaturesKHR {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_PERFORMANCE_QUERY_FEATURES_KHR, p_next: std::ptr::null_mut(), performance_counter_query_pools: Default::default(), performance_counter_multiple_query_pools: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDevicePerformanceQueryFeaturesKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDevicePerformanceQueryFeaturesKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("performance_counter_query_pools", &(self.performance_counter_query_pools != 0)).field("performance_counter_multiple_query_pools", &(self.performance_counter_multiple_query_pools != 0)).finish()
    }
}
impl PhysicalDevicePerformanceQueryFeaturesKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDevicePerformanceQueryFeaturesKHRBuilder<'a> {
        PhysicalDevicePerformanceQueryFeaturesKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevicePerformanceQueryFeaturesKHR.html) · Builder of [`PhysicalDevicePerformanceQueryFeaturesKHR`]"]
#[repr(transparent)]
pub struct PhysicalDevicePerformanceQueryFeaturesKHRBuilder<'a>(PhysicalDevicePerformanceQueryFeaturesKHR, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDevicePerformanceQueryFeaturesKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDevicePerformanceQueryFeaturesKHRBuilder<'a> {
        PhysicalDevicePerformanceQueryFeaturesKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn performance_counter_query_pools(mut self, performance_counter_query_pools: bool) -> Self {
        self.0.performance_counter_query_pools = performance_counter_query_pools as _;
        self
    }
    #[inline]
    pub fn performance_counter_multiple_query_pools(mut self, performance_counter_multiple_query_pools: bool) -> Self {
        self.0.performance_counter_multiple_query_pools = performance_counter_multiple_query_pools as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDevicePerformanceQueryFeaturesKHR {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDevicePerformanceQueryFeaturesKHRBuilder<'a> {
    fn default() -> PhysicalDevicePerformanceQueryFeaturesKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDevicePerformanceQueryFeaturesKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
#[doc(alias = "VkPhysicalDevicePerformanceQueryPropertiesKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDevicePerformanceQueryPropertiesKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub allow_command_buffer_query_copies: crate::vk1_0::Bool32,
}
impl Default for PhysicalDevicePerformanceQueryPropertiesKHR {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_PERFORMANCE_QUERY_PROPERTIES_KHR, p_next: std::ptr::null_mut(), allow_command_buffer_query_copies: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDevicePerformanceQueryPropertiesKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDevicePerformanceQueryPropertiesKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("allow_command_buffer_query_copies", &(self.allow_command_buffer_query_copies != 0)).finish()
    }
}
impl PhysicalDevicePerformanceQueryPropertiesKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDevicePerformanceQueryPropertiesKHRBuilder<'a> {
        PhysicalDevicePerformanceQueryPropertiesKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevicePerformanceQueryPropertiesKHR.html) · Builder of [`PhysicalDevicePerformanceQueryPropertiesKHR`]"]
#[repr(transparent)]
pub struct PhysicalDevicePerformanceQueryPropertiesKHRBuilder<'a>(PhysicalDevicePerformanceQueryPropertiesKHR, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDevicePerformanceQueryPropertiesKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDevicePerformanceQueryPropertiesKHRBuilder<'a> {
        PhysicalDevicePerformanceQueryPropertiesKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn allow_command_buffer_query_copies(mut self, allow_command_buffer_query_copies: bool) -> Self {
        self.0.allow_command_buffer_query_copies = allow_command_buffer_query_copies as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDevicePerformanceQueryPropertiesKHR {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDevicePerformanceQueryPropertiesKHRBuilder<'a> {
    fn default() -> PhysicalDevicePerformanceQueryPropertiesKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDevicePerformanceQueryPropertiesKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceCounterKHR.html) · Structure"]
#[doc(alias = "VkPerformanceCounterKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PerformanceCounterKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub unit: crate::extensions::khr_performance_query::PerformanceCounterUnitKHR,
    pub scope: crate::extensions::khr_performance_query::PerformanceCounterScopeKHR,
    pub storage: crate::extensions::khr_performance_query::PerformanceCounterStorageKHR,
    pub uuid: [u8; 16],
}
impl Default for PerformanceCounterKHR {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PERFORMANCE_COUNTER_KHR, p_next: std::ptr::null(), unit: Default::default(), scope: Default::default(), storage: Default::default(), uuid: unsafe { std::mem::zeroed() } }
    }
}
impl std::fmt::Debug for PerformanceCounterKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PerformanceCounterKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("unit", &self.unit).field("scope", &self.scope).field("storage", &self.storage).field("uuid", &self.uuid).finish()
    }
}
impl PerformanceCounterKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> PerformanceCounterKHRBuilder<'a> {
        PerformanceCounterKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceCounterKHR.html) · Builder of [`PerformanceCounterKHR`]"]
#[repr(transparent)]
pub struct PerformanceCounterKHRBuilder<'a>(PerformanceCounterKHR, std::marker::PhantomData<&'a ()>);
impl<'a> PerformanceCounterKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PerformanceCounterKHRBuilder<'a> {
        PerformanceCounterKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn unit(mut self, unit: crate::extensions::khr_performance_query::PerformanceCounterUnitKHR) -> Self {
        self.0.unit = unit as _;
        self
    }
    #[inline]
    pub fn scope(mut self, scope: crate::extensions::khr_performance_query::PerformanceCounterScopeKHR) -> Self {
        self.0.scope = scope as _;
        self
    }
    #[inline]
    pub fn storage(mut self, storage: crate::extensions::khr_performance_query::PerformanceCounterStorageKHR) -> Self {
        self.0.storage = storage as _;
        self
    }
    #[inline]
    pub fn uuid(mut self, uuid: [u8; 16]) -> Self {
        self.0.uuid = uuid as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PerformanceCounterKHR {
        self.0
    }
}
impl<'a> std::default::Default for PerformanceCounterKHRBuilder<'a> {
    fn default() -> PerformanceCounterKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PerformanceCounterKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceCounterDescriptionKHR.html) · Structure"]
#[doc(alias = "VkPerformanceCounterDescriptionKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PerformanceCounterDescriptionKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::khr_performance_query::PerformanceCounterDescriptionFlagsKHR,
    pub name: [std::os::raw::c_char; 256],
    pub category: [std::os::raw::c_char; 256],
    pub description: [std::os::raw::c_char; 256],
}
impl Default for PerformanceCounterDescriptionKHR {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PERFORMANCE_COUNTER_DESCRIPTION_KHR, p_next: std::ptr::null(), flags: Default::default(), name: unsafe { std::mem::zeroed() }, category: unsafe { std::mem::zeroed() }, description: unsafe { std::mem::zeroed() } }
    }
}
impl std::fmt::Debug for PerformanceCounterDescriptionKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PerformanceCounterDescriptionKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("name", unsafe { &std::ffi::CStr::from_ptr(self.name.as_ptr()) }).field("category", unsafe { &std::ffi::CStr::from_ptr(self.category.as_ptr()) }).field("description", unsafe { &std::ffi::CStr::from_ptr(self.description.as_ptr()) }).finish()
    }
}
impl PerformanceCounterDescriptionKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> PerformanceCounterDescriptionKHRBuilder<'a> {
        PerformanceCounterDescriptionKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceCounterDescriptionKHR.html) · Builder of [`PerformanceCounterDescriptionKHR`]"]
#[repr(transparent)]
pub struct PerformanceCounterDescriptionKHRBuilder<'a>(PerformanceCounterDescriptionKHR, std::marker::PhantomData<&'a ()>);
impl<'a> PerformanceCounterDescriptionKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PerformanceCounterDescriptionKHRBuilder<'a> {
        PerformanceCounterDescriptionKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::khr_performance_query::PerformanceCounterDescriptionFlagsKHR) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn name(mut self, name: [std::os::raw::c_char; 256]) -> Self {
        self.0.name = name as _;
        self
    }
    #[inline]
    pub fn category(mut self, category: [std::os::raw::c_char; 256]) -> Self {
        self.0.category = category as _;
        self
    }
    #[inline]
    pub fn description(mut self, description: [std::os::raw::c_char; 256]) -> Self {
        self.0.description = description as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PerformanceCounterDescriptionKHR {
        self.0
    }
}
impl<'a> std::default::Default for PerformanceCounterDescriptionKHRBuilder<'a> {
    fn default() -> PerformanceCounterDescriptionKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PerformanceCounterDescriptionKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueryPoolPerformanceCreateInfoKHR.html) · Structure"]
#[doc(alias = "VkQueryPoolPerformanceCreateInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct QueryPoolPerformanceCreateInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub queue_family_index: u32,
    pub counter_index_count: u32,
    pub p_counter_indices: *const u32,
}
impl Default for QueryPoolPerformanceCreateInfoKHR {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::QUERY_POOL_PERFORMANCE_CREATE_INFO_KHR, p_next: std::ptr::null(), queue_family_index: Default::default(), counter_index_count: Default::default(), p_counter_indices: std::ptr::null() }
    }
}
impl std::fmt::Debug for QueryPoolPerformanceCreateInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("QueryPoolPerformanceCreateInfoKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("queue_family_index", &self.queue_family_index).field("counter_index_count", &self.counter_index_count).field("p_counter_indices", &self.p_counter_indices).finish()
    }
}
impl QueryPoolPerformanceCreateInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> QueryPoolPerformanceCreateInfoKHRBuilder<'a> {
        QueryPoolPerformanceCreateInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueryPoolPerformanceCreateInfoKHR.html) · Builder of [`QueryPoolPerformanceCreateInfoKHR`]"]
#[repr(transparent)]
pub struct QueryPoolPerformanceCreateInfoKHRBuilder<'a>(QueryPoolPerformanceCreateInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> QueryPoolPerformanceCreateInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> QueryPoolPerformanceCreateInfoKHRBuilder<'a> {
        QueryPoolPerformanceCreateInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn queue_family_index(mut self, queue_family_index: u32) -> Self {
        self.0.queue_family_index = queue_family_index as _;
        self
    }
    #[inline]
    pub fn counter_indices(mut self, counter_indices: &'a [u32]) -> Self {
        self.0.p_counter_indices = counter_indices.as_ptr() as _;
        self.0.counter_index_count = counter_indices.len() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> QueryPoolPerformanceCreateInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for QueryPoolPerformanceCreateInfoKHRBuilder<'a> {
    fn default() -> QueryPoolPerformanceCreateInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for QueryPoolPerformanceCreateInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceCounterResultKHR.html) · Structure"]
#[doc(alias = "VkPerformanceCounterResultKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub union PerformanceCounterResultKHR {
    pub int32: i32,
    pub int64: i64,
    pub uint32: u32,
    pub uint64: u64,
    pub float32: std::os::raw::c_float,
    pub float64: std::os::raw::c_double,
}
impl Default for PerformanceCounterResultKHR {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for PerformanceCounterResultKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PerformanceCounterResultKHR").finish()
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAcquireProfilingLockInfoKHR.html) · Structure"]
#[doc(alias = "VkAcquireProfilingLockInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AcquireProfilingLockInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::khr_performance_query::AcquireProfilingLockFlagsKHR,
    pub timeout: u64,
}
impl Default for AcquireProfilingLockInfoKHR {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::ACQUIRE_PROFILING_LOCK_INFO_KHR, p_next: std::ptr::null(), flags: Default::default(), timeout: Default::default() }
    }
}
impl std::fmt::Debug for AcquireProfilingLockInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AcquireProfilingLockInfoKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("timeout", &self.timeout).finish()
    }
}
impl AcquireProfilingLockInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> AcquireProfilingLockInfoKHRBuilder<'a> {
        AcquireProfilingLockInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAcquireProfilingLockInfoKHR.html) · Builder of [`AcquireProfilingLockInfoKHR`]"]
#[repr(transparent)]
pub struct AcquireProfilingLockInfoKHRBuilder<'a>(AcquireProfilingLockInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> AcquireProfilingLockInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> AcquireProfilingLockInfoKHRBuilder<'a> {
        AcquireProfilingLockInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::khr_performance_query::AcquireProfilingLockFlagsKHR) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn timeout(mut self, timeout: u64) -> Self {
        self.0.timeout = timeout as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> AcquireProfilingLockInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for AcquireProfilingLockInfoKHRBuilder<'a> {
    fn default() -> AcquireProfilingLockInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for AcquireProfilingLockInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceQuerySubmitInfoKHR.html) · Structure"]
#[doc(alias = "VkPerformanceQuerySubmitInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PerformanceQuerySubmitInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub counter_pass_index: u32,
}
impl Default for PerformanceQuerySubmitInfoKHR {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PERFORMANCE_QUERY_SUBMIT_INFO_KHR, p_next: std::ptr::null(), counter_pass_index: Default::default() }
    }
}
impl std::fmt::Debug for PerformanceQuerySubmitInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PerformanceQuerySubmitInfoKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("counter_pass_index", &self.counter_pass_index).finish()
    }
}
impl PerformanceQuerySubmitInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> PerformanceQuerySubmitInfoKHRBuilder<'a> {
        PerformanceQuerySubmitInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceQuerySubmitInfoKHR.html) · Builder of [`PerformanceQuerySubmitInfoKHR`]"]
#[repr(transparent)]
pub struct PerformanceQuerySubmitInfoKHRBuilder<'a>(PerformanceQuerySubmitInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> PerformanceQuerySubmitInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PerformanceQuerySubmitInfoKHRBuilder<'a> {
        PerformanceQuerySubmitInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn counter_pass_index(mut self, counter_pass_index: u32) -> Self {
        self.0.counter_pass_index = counter_pass_index as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PerformanceQuerySubmitInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for PerformanceQuerySubmitInfoKHRBuilder<'a> {
    fn default() -> PerformanceQuerySubmitInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PerformanceQuerySubmitInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
#[doc = "Provided by [`crate::extensions::khr_performance_query`]"]
impl crate::InstanceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR.html) · Function"]
    #[doc(alias = "vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR")]
    pub unsafe fn enumerate_physical_device_queue_family_performance_query_counters_khr(&self, physical_device: crate::vk1_0::PhysicalDevice, queue_family_index: u32, counter_count: Option<u32>) -> crate::utils::VulkanResult<(Vec<crate::extensions::khr_performance_query::PerformanceCounterKHR>, Vec<crate::extensions::khr_performance_query::PerformanceCounterDescriptionKHR>)> {
        let _function = self.enumerate_physical_device_queue_family_performance_query_counters_khr.expect("tried to call a function that isn't loaded");
        let mut counter_count = match counter_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                _function(physical_device as _, queue_family_index as _, &mut v, std::ptr::null_mut(), std::ptr::null_mut());
                v
            }
        };
        let mut counters = vec![Default::default(); counter_count as _];
        let mut counter_descriptions = vec![Default::default(); counter_count as _];
        let _return = _function(physical_device as _, queue_family_index as _, &mut counter_count, counters.as_mut_ptr(), counter_descriptions.as_mut_ptr());
        crate::utils::VulkanResult::new(_return, (counters, counter_descriptions))
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR.html) · Function"]
    #[doc(alias = "vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR")]
    pub unsafe fn get_physical_device_queue_family_performance_query_passes_khr(&self, physical_device: crate::vk1_0::PhysicalDevice, performance_query_create_info: &crate::extensions::khr_performance_query::QueryPoolPerformanceCreateInfoKHR) -> u32 {
        let _function = self.get_physical_device_queue_family_performance_query_passes_khr.expect("tried to call a function that isn't loaded");
        let mut num_passes = Default::default();
        let _return = _function(physical_device as _, performance_query_create_info as _, &mut num_passes);
        num_passes
    }
}
#[doc = "Provided by [`crate::extensions::khr_performance_query`]"]
impl crate::DeviceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAcquireProfilingLockKHR.html) · Function"]
    #[doc(alias = "vkAcquireProfilingLockKHR")]
    pub unsafe fn acquire_profiling_lock_khr(&self, info: &crate::extensions::khr_performance_query::AcquireProfilingLockInfoKHR) -> crate::utils::VulkanResult<()> {
        let _function = self.acquire_profiling_lock_khr.expect("tried to call a function that isn't loaded");
        let _return = _function(self.handle, info as _);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkReleaseProfilingLockKHR.html) · Function"]
    #[doc(alias = "vkReleaseProfilingLockKHR")]
    pub unsafe fn release_profiling_lock_khr(&self) -> () {
        let _function = self.release_profiling_lock_khr.expect("tried to call a function that isn't loaded");
        let _return = _function(self.handle);
        ()
    }
}
