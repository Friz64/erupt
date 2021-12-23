#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_INTEL_PERFORMANCE_QUERY_SPEC_VERSION")]
pub const INTEL_PERFORMANCE_QUERY_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_INTEL_PERFORMANCE_QUERY_EXTENSION_NAME")]
pub const INTEL_PERFORMANCE_QUERY_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_INTEL_performance_query");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_INITIALIZE_PERFORMANCE_API_INTEL: *const std::os::raw::c_char = crate::cstr!("vkInitializePerformanceApiINTEL");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_UNINITIALIZE_PERFORMANCE_API_INTEL: *const std::os::raw::c_char = crate::cstr!("vkUninitializePerformanceApiINTEL");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_SET_PERFORMANCE_MARKER_INTEL: *const std::os::raw::c_char = crate::cstr!("vkCmdSetPerformanceMarkerINTEL");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_SET_PERFORMANCE_STREAM_MARKER_INTEL: *const std::os::raw::c_char = crate::cstr!("vkCmdSetPerformanceStreamMarkerINTEL");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_SET_PERFORMANCE_OVERRIDE_INTEL: *const std::os::raw::c_char = crate::cstr!("vkCmdSetPerformanceOverrideINTEL");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_ACQUIRE_PERFORMANCE_CONFIGURATION_INTEL: *const std::os::raw::c_char = crate::cstr!("vkAcquirePerformanceConfigurationINTEL");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_RELEASE_PERFORMANCE_CONFIGURATION_INTEL: *const std::os::raw::c_char = crate::cstr!("vkReleasePerformanceConfigurationINTEL");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_QUEUE_SET_PERFORMANCE_CONFIGURATION_INTEL: *const std::os::raw::c_char = crate::cstr!("vkQueueSetPerformanceConfigurationINTEL");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_PERFORMANCE_PARAMETER_INTEL: *const std::os::raw::c_char = crate::cstr!("vkGetPerformanceParameterINTEL");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueryPoolCreateInfoINTEL.html) · Alias"]
#[doc(alias = "VkQueryPoolCreateInfoINTEL")]
#[allow(non_camel_case_types)]
pub type QueryPoolCreateInfoINTEL = crate::extensions::intel_performance_query::QueryPoolPerformanceQueryCreateInfoINTEL;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueryPoolCreateInfoINTEL.html) · Alias"]
#[doc(alias = "VkQueryPoolCreateInfoINTEL")]
#[allow(non_camel_case_types)]
pub type QueryPoolCreateInfoINTELBuilder<'a> = crate::extensions::intel_performance_query::QueryPoolPerformanceQueryCreateInfoINTELBuilder<'a>;
crate::non_dispatchable_handle!(PerformanceConfigurationINTEL, PERFORMANCE_CONFIGURATION_INTEL, "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceConfigurationINTEL.html) · Non-dispatchable Handle", "VkPerformanceConfigurationINTEL");
#[doc = "Provided by [`crate::extensions::intel_performance_query`]"]
impl crate::vk1_0::QueryType {
    pub const PERFORMANCE_QUERY_INTEL: Self = Self(1000210000);
}
#[doc = "Provided by [`crate::extensions::intel_performance_query`]"]
impl crate::vk1_0::StructureType {
    pub const QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL: Self = Self(1000210000);
    pub const INITIALIZE_PERFORMANCE_API_INFO_INTEL: Self = Self(1000210001);
    pub const PERFORMANCE_MARKER_INFO_INTEL: Self = Self(1000210002);
    pub const PERFORMANCE_STREAM_MARKER_INFO_INTEL: Self = Self(1000210003);
    pub const PERFORMANCE_OVERRIDE_INFO_INTEL: Self = Self(1000210004);
    pub const PERFORMANCE_CONFIGURATION_ACQUIRE_INFO_INTEL: Self = Self(1000210005);
    #[deprecated]
    pub const QUERY_POOL_CREATE_INFO_INTEL: Self = Self::QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL;
}
#[doc = "Provided by [`crate::extensions::intel_performance_query`]"]
impl crate::vk1_0::ObjectType {
    pub const PERFORMANCE_CONFIGURATION_INTEL: Self = Self(1000210000);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceConfigurationTypeINTEL.html) · Enum"]
#[doc(alias = "VkPerformanceConfigurationTypeINTEL")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct PerformanceConfigurationTypeINTEL(pub i32);
impl std::fmt::Debug for PerformanceConfigurationTypeINTEL {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::COMMAND_QUEUE_METRICS_DISCOVERY_ACTIVATED_INTEL => "COMMAND_QUEUE_METRICS_DISCOVERY_ACTIVATED_INTEL",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::intel_performance_query`]"]
impl crate::extensions::intel_performance_query::PerformanceConfigurationTypeINTEL {
    pub const COMMAND_QUEUE_METRICS_DISCOVERY_ACTIVATED_INTEL: Self = Self(0);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueryPoolSamplingModeINTEL.html) · Enum"]
#[doc(alias = "VkQueryPoolSamplingModeINTEL")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct QueryPoolSamplingModeINTEL(pub i32);
impl std::fmt::Debug for QueryPoolSamplingModeINTEL {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::MANUAL_INTEL => "MANUAL_INTEL",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::intel_performance_query`]"]
impl crate::extensions::intel_performance_query::QueryPoolSamplingModeINTEL {
    pub const MANUAL_INTEL: Self = Self(0);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceOverrideTypeINTEL.html) · Enum"]
#[doc(alias = "VkPerformanceOverrideTypeINTEL")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct PerformanceOverrideTypeINTEL(pub i32);
impl std::fmt::Debug for PerformanceOverrideTypeINTEL {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::NULL_HARDWARE_INTEL => "NULL_HARDWARE_INTEL",
            &Self::FLUSH_GPU_CACHES_INTEL => "FLUSH_GPU_CACHES_INTEL",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::intel_performance_query`]"]
impl crate::extensions::intel_performance_query::PerformanceOverrideTypeINTEL {
    pub const NULL_HARDWARE_INTEL: Self = Self(0);
    pub const FLUSH_GPU_CACHES_INTEL: Self = Self(1);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceParameterTypeINTEL.html) · Enum"]
#[doc(alias = "VkPerformanceParameterTypeINTEL")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct PerformanceParameterTypeINTEL(pub i32);
impl std::fmt::Debug for PerformanceParameterTypeINTEL {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::HW_COUNTERS_SUPPORTED_INTEL => "HW_COUNTERS_SUPPORTED_INTEL",
            &Self::STREAM_MARKER_VALID_BITS_INTEL => "STREAM_MARKER_VALID_BITS_INTEL",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::intel_performance_query`]"]
impl crate::extensions::intel_performance_query::PerformanceParameterTypeINTEL {
    pub const HW_COUNTERS_SUPPORTED_INTEL: Self = Self(0);
    pub const STREAM_MARKER_VALID_BITS_INTEL: Self = Self(1);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceValueTypeINTEL.html) · Enum"]
#[doc(alias = "VkPerformanceValueTypeINTEL")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct PerformanceValueTypeINTEL(pub i32);
impl std::fmt::Debug for PerformanceValueTypeINTEL {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::UINT32_INTEL => "UINT32_INTEL",
            &Self::UINT64_INTEL => "UINT64_INTEL",
            &Self::FLOAT_INTEL => "FLOAT_INTEL",
            &Self::BOOL_INTEL => "BOOL_INTEL",
            &Self::STRING_INTEL => "STRING_INTEL",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::intel_performance_query`]"]
impl crate::extensions::intel_performance_query::PerformanceValueTypeINTEL {
    pub const UINT32_INTEL: Self = Self(0);
    pub const UINT64_INTEL: Self = Self(1);
    pub const FLOAT_INTEL: Self = Self(2);
    pub const BOOL_INTEL: Self = Self(3);
    pub const STRING_INTEL: Self = Self(4);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkInitializePerformanceApiINTEL.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkInitializePerformanceApiINTEL = unsafe extern "system" fn(device: crate::vk1_0::Device, p_initialize_info: *const crate::extensions::intel_performance_query::InitializePerformanceApiInfoINTEL) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkUninitializePerformanceApiINTEL.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkUninitializePerformanceApiINTEL = unsafe extern "system" fn(device: crate::vk1_0::Device) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetPerformanceMarkerINTEL.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetPerformanceMarkerINTEL = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, p_marker_info: *const crate::extensions::intel_performance_query::PerformanceMarkerInfoINTEL) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetPerformanceStreamMarkerINTEL.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetPerformanceStreamMarkerINTEL = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, p_marker_info: *const crate::extensions::intel_performance_query::PerformanceStreamMarkerInfoINTEL) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetPerformanceOverrideINTEL.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetPerformanceOverrideINTEL = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, p_override_info: *const crate::extensions::intel_performance_query::PerformanceOverrideInfoINTEL) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAcquirePerformanceConfigurationINTEL.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkAcquirePerformanceConfigurationINTEL = unsafe extern "system" fn(device: crate::vk1_0::Device, p_acquire_info: *const crate::extensions::intel_performance_query::PerformanceConfigurationAcquireInfoINTEL, p_configuration: *mut crate::extensions::intel_performance_query::PerformanceConfigurationINTEL) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkReleasePerformanceConfigurationINTEL.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkReleasePerformanceConfigurationINTEL = unsafe extern "system" fn(device: crate::vk1_0::Device, configuration: crate::extensions::intel_performance_query::PerformanceConfigurationINTEL) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueueSetPerformanceConfigurationINTEL.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkQueueSetPerformanceConfigurationINTEL = unsafe extern "system" fn(queue: crate::vk1_0::Queue, configuration: crate::extensions::intel_performance_query::PerformanceConfigurationINTEL) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPerformanceParameterINTEL.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPerformanceParameterINTEL = unsafe extern "system" fn(device: crate::vk1_0::Device, parameter: crate::extensions::intel_performance_query::PerformanceParameterTypeINTEL, p_value: *mut crate::extensions::intel_performance_query::PerformanceValueINTEL) -> crate::vk1_0::Result;
impl<'a> crate::ExtendableFrom<'a, QueryPoolPerformanceQueryCreateInfoINTEL> for crate::vk1_0::QueryPoolCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, QueryPoolPerformanceQueryCreateInfoINTELBuilder<'_>> for crate::vk1_0::QueryPoolCreateInfoBuilder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceValueDataINTEL.html) · Structure"]
#[doc(alias = "VkPerformanceValueDataINTEL")]
#[derive(Copy, Clone)]
#[repr(C)]
pub union PerformanceValueDataINTEL {
    pub value32: u32,
    pub value64: u64,
    pub value_float: std::os::raw::c_float,
    pub value_bool: crate::vk1_0::Bool32,
    pub value_string: *const std::os::raw::c_char,
}
impl Default for PerformanceValueDataINTEL {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for PerformanceValueDataINTEL {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PerformanceValueDataINTEL").finish()
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceValueINTEL.html) · Structure"]
#[doc(alias = "VkPerformanceValueINTEL")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PerformanceValueINTEL {
    pub _type: crate::extensions::intel_performance_query::PerformanceValueTypeINTEL,
    pub data: crate::extensions::intel_performance_query::PerformanceValueDataINTEL,
}
impl Default for PerformanceValueINTEL {
    fn default() -> Self {
        Self { _type: Default::default(), data: Default::default() }
    }
}
impl std::fmt::Debug for PerformanceValueINTEL {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PerformanceValueINTEL").field("_type", &self._type).field("data", &self.data).finish()
    }
}
impl PerformanceValueINTEL {
    #[inline]
    pub fn into_builder<'a>(self) -> PerformanceValueINTELBuilder<'a> {
        PerformanceValueINTELBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceValueINTEL.html) · Builder of [`PerformanceValueINTEL`]"]
#[repr(transparent)]
pub struct PerformanceValueINTELBuilder<'a>(PerformanceValueINTEL, std::marker::PhantomData<&'a ()>);
impl<'a> PerformanceValueINTELBuilder<'a> {
    #[inline]
    pub fn new() -> PerformanceValueINTELBuilder<'a> {
        PerformanceValueINTELBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn _type(mut self, _type: crate::extensions::intel_performance_query::PerformanceValueTypeINTEL) -> Self {
        self.0._type = _type as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn data(mut self, data: crate::extensions::intel_performance_query::PerformanceValueDataINTEL) -> Self {
        self.0.data = data as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> PerformanceValueINTEL {
        self.0
    }
}
impl<'a> std::default::Default for PerformanceValueINTELBuilder<'a> {
    fn default() -> PerformanceValueINTELBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PerformanceValueINTELBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PerformanceValueINTELBuilder<'a> {
    type Target = PerformanceValueINTEL;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PerformanceValueINTELBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkInitializePerformanceApiInfoINTEL.html) · Structure"]
#[doc(alias = "VkInitializePerformanceApiInfoINTEL")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InitializePerformanceApiInfoINTEL {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub p_user_data: *mut std::ffi::c_void,
}
impl InitializePerformanceApiInfoINTEL {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::INITIALIZE_PERFORMANCE_API_INFO_INTEL;
}
impl Default for InitializePerformanceApiInfoINTEL {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), p_user_data: std::ptr::null_mut() }
    }
}
impl std::fmt::Debug for InitializePerformanceApiInfoINTEL {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InitializePerformanceApiInfoINTEL").field("s_type", &self.s_type).field("p_next", &self.p_next).field("p_user_data", &self.p_user_data).finish()
    }
}
impl InitializePerformanceApiInfoINTEL {
    #[inline]
    pub fn into_builder<'a>(self) -> InitializePerformanceApiInfoINTELBuilder<'a> {
        InitializePerformanceApiInfoINTELBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkInitializePerformanceApiInfoINTEL.html) · Builder of [`InitializePerformanceApiInfoINTEL`]"]
#[repr(transparent)]
pub struct InitializePerformanceApiInfoINTELBuilder<'a>(InitializePerformanceApiInfoINTEL, std::marker::PhantomData<&'a ()>);
impl<'a> InitializePerformanceApiInfoINTELBuilder<'a> {
    #[inline]
    pub fn new() -> InitializePerformanceApiInfoINTELBuilder<'a> {
        InitializePerformanceApiInfoINTELBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn user_data(mut self, user_data: *mut std::ffi::c_void) -> Self {
        self.0.p_user_data = user_data;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> InitializePerformanceApiInfoINTEL {
        self.0
    }
}
impl<'a> std::default::Default for InitializePerformanceApiInfoINTELBuilder<'a> {
    fn default() -> InitializePerformanceApiInfoINTELBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for InitializePerformanceApiInfoINTELBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for InitializePerformanceApiInfoINTELBuilder<'a> {
    type Target = InitializePerformanceApiInfoINTEL;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for InitializePerformanceApiInfoINTELBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueryPoolPerformanceQueryCreateInfoINTEL.html) · Structure"]
#[doc(alias = "VkQueryPoolPerformanceQueryCreateInfoINTEL")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct QueryPoolPerformanceQueryCreateInfoINTEL {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub performance_counters_sampling: crate::extensions::intel_performance_query::QueryPoolSamplingModeINTEL,
}
impl QueryPoolPerformanceQueryCreateInfoINTEL {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL;
}
impl Default for QueryPoolPerformanceQueryCreateInfoINTEL {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), performance_counters_sampling: Default::default() }
    }
}
impl std::fmt::Debug for QueryPoolPerformanceQueryCreateInfoINTEL {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("QueryPoolPerformanceQueryCreateInfoINTEL").field("s_type", &self.s_type).field("p_next", &self.p_next).field("performance_counters_sampling", &self.performance_counters_sampling).finish()
    }
}
impl QueryPoolPerformanceQueryCreateInfoINTEL {
    #[inline]
    pub fn into_builder<'a>(self) -> QueryPoolPerformanceQueryCreateInfoINTELBuilder<'a> {
        QueryPoolPerformanceQueryCreateInfoINTELBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueryPoolPerformanceQueryCreateInfoINTEL.html) · Builder of [`QueryPoolPerformanceQueryCreateInfoINTEL`]"]
#[repr(transparent)]
pub struct QueryPoolPerformanceQueryCreateInfoINTELBuilder<'a>(QueryPoolPerformanceQueryCreateInfoINTEL, std::marker::PhantomData<&'a ()>);
impl<'a> QueryPoolPerformanceQueryCreateInfoINTELBuilder<'a> {
    #[inline]
    pub fn new() -> QueryPoolPerformanceQueryCreateInfoINTELBuilder<'a> {
        QueryPoolPerformanceQueryCreateInfoINTELBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn performance_counters_sampling(mut self, performance_counters_sampling: crate::extensions::intel_performance_query::QueryPoolSamplingModeINTEL) -> Self {
        self.0.performance_counters_sampling = performance_counters_sampling as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> QueryPoolPerformanceQueryCreateInfoINTEL {
        self.0
    }
}
impl<'a> std::default::Default for QueryPoolPerformanceQueryCreateInfoINTELBuilder<'a> {
    fn default() -> QueryPoolPerformanceQueryCreateInfoINTELBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for QueryPoolPerformanceQueryCreateInfoINTELBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for QueryPoolPerformanceQueryCreateInfoINTELBuilder<'a> {
    type Target = QueryPoolPerformanceQueryCreateInfoINTEL;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for QueryPoolPerformanceQueryCreateInfoINTELBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceMarkerInfoINTEL.html) · Structure"]
#[doc(alias = "VkPerformanceMarkerInfoINTEL")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PerformanceMarkerInfoINTEL {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub marker: u64,
}
impl PerformanceMarkerInfoINTEL {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PERFORMANCE_MARKER_INFO_INTEL;
}
impl Default for PerformanceMarkerInfoINTEL {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), marker: Default::default() }
    }
}
impl std::fmt::Debug for PerformanceMarkerInfoINTEL {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PerformanceMarkerInfoINTEL").field("s_type", &self.s_type).field("p_next", &self.p_next).field("marker", &self.marker).finish()
    }
}
impl PerformanceMarkerInfoINTEL {
    #[inline]
    pub fn into_builder<'a>(self) -> PerformanceMarkerInfoINTELBuilder<'a> {
        PerformanceMarkerInfoINTELBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceMarkerInfoINTEL.html) · Builder of [`PerformanceMarkerInfoINTEL`]"]
#[repr(transparent)]
pub struct PerformanceMarkerInfoINTELBuilder<'a>(PerformanceMarkerInfoINTEL, std::marker::PhantomData<&'a ()>);
impl<'a> PerformanceMarkerInfoINTELBuilder<'a> {
    #[inline]
    pub fn new() -> PerformanceMarkerInfoINTELBuilder<'a> {
        PerformanceMarkerInfoINTELBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn marker(mut self, marker: u64) -> Self {
        self.0.marker = marker as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> PerformanceMarkerInfoINTEL {
        self.0
    }
}
impl<'a> std::default::Default for PerformanceMarkerInfoINTELBuilder<'a> {
    fn default() -> PerformanceMarkerInfoINTELBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PerformanceMarkerInfoINTELBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PerformanceMarkerInfoINTELBuilder<'a> {
    type Target = PerformanceMarkerInfoINTEL;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PerformanceMarkerInfoINTELBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceStreamMarkerInfoINTEL.html) · Structure"]
#[doc(alias = "VkPerformanceStreamMarkerInfoINTEL")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PerformanceStreamMarkerInfoINTEL {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub marker: u32,
}
impl PerformanceStreamMarkerInfoINTEL {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PERFORMANCE_STREAM_MARKER_INFO_INTEL;
}
impl Default for PerformanceStreamMarkerInfoINTEL {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), marker: Default::default() }
    }
}
impl std::fmt::Debug for PerformanceStreamMarkerInfoINTEL {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PerformanceStreamMarkerInfoINTEL").field("s_type", &self.s_type).field("p_next", &self.p_next).field("marker", &self.marker).finish()
    }
}
impl PerformanceStreamMarkerInfoINTEL {
    #[inline]
    pub fn into_builder<'a>(self) -> PerformanceStreamMarkerInfoINTELBuilder<'a> {
        PerformanceStreamMarkerInfoINTELBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceStreamMarkerInfoINTEL.html) · Builder of [`PerformanceStreamMarkerInfoINTEL`]"]
#[repr(transparent)]
pub struct PerformanceStreamMarkerInfoINTELBuilder<'a>(PerformanceStreamMarkerInfoINTEL, std::marker::PhantomData<&'a ()>);
impl<'a> PerformanceStreamMarkerInfoINTELBuilder<'a> {
    #[inline]
    pub fn new() -> PerformanceStreamMarkerInfoINTELBuilder<'a> {
        PerformanceStreamMarkerInfoINTELBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn marker(mut self, marker: u32) -> Self {
        self.0.marker = marker as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> PerformanceStreamMarkerInfoINTEL {
        self.0
    }
}
impl<'a> std::default::Default for PerformanceStreamMarkerInfoINTELBuilder<'a> {
    fn default() -> PerformanceStreamMarkerInfoINTELBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PerformanceStreamMarkerInfoINTELBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PerformanceStreamMarkerInfoINTELBuilder<'a> {
    type Target = PerformanceStreamMarkerInfoINTEL;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PerformanceStreamMarkerInfoINTELBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceOverrideInfoINTEL.html) · Structure"]
#[doc(alias = "VkPerformanceOverrideInfoINTEL")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PerformanceOverrideInfoINTEL {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub _type: crate::extensions::intel_performance_query::PerformanceOverrideTypeINTEL,
    pub enable: crate::vk1_0::Bool32,
    pub parameter: u64,
}
impl PerformanceOverrideInfoINTEL {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PERFORMANCE_OVERRIDE_INFO_INTEL;
}
impl Default for PerformanceOverrideInfoINTEL {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), _type: Default::default(), enable: Default::default(), parameter: Default::default() }
    }
}
impl std::fmt::Debug for PerformanceOverrideInfoINTEL {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PerformanceOverrideInfoINTEL").field("s_type", &self.s_type).field("p_next", &self.p_next).field("_type", &self._type).field("enable", &(self.enable != 0)).field("parameter", &self.parameter).finish()
    }
}
impl PerformanceOverrideInfoINTEL {
    #[inline]
    pub fn into_builder<'a>(self) -> PerformanceOverrideInfoINTELBuilder<'a> {
        PerformanceOverrideInfoINTELBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceOverrideInfoINTEL.html) · Builder of [`PerformanceOverrideInfoINTEL`]"]
#[repr(transparent)]
pub struct PerformanceOverrideInfoINTELBuilder<'a>(PerformanceOverrideInfoINTEL, std::marker::PhantomData<&'a ()>);
impl<'a> PerformanceOverrideInfoINTELBuilder<'a> {
    #[inline]
    pub fn new() -> PerformanceOverrideInfoINTELBuilder<'a> {
        PerformanceOverrideInfoINTELBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn _type(mut self, _type: crate::extensions::intel_performance_query::PerformanceOverrideTypeINTEL) -> Self {
        self.0._type = _type as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn enable(mut self, enable: bool) -> Self {
        self.0.enable = enable as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn parameter(mut self, parameter: u64) -> Self {
        self.0.parameter = parameter as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> PerformanceOverrideInfoINTEL {
        self.0
    }
}
impl<'a> std::default::Default for PerformanceOverrideInfoINTELBuilder<'a> {
    fn default() -> PerformanceOverrideInfoINTELBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PerformanceOverrideInfoINTELBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PerformanceOverrideInfoINTELBuilder<'a> {
    type Target = PerformanceOverrideInfoINTEL;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PerformanceOverrideInfoINTELBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceConfigurationAcquireInfoINTEL.html) · Structure"]
#[doc(alias = "VkPerformanceConfigurationAcquireInfoINTEL")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PerformanceConfigurationAcquireInfoINTEL {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub _type: crate::extensions::intel_performance_query::PerformanceConfigurationTypeINTEL,
}
impl PerformanceConfigurationAcquireInfoINTEL {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PERFORMANCE_CONFIGURATION_ACQUIRE_INFO_INTEL;
}
impl Default for PerformanceConfigurationAcquireInfoINTEL {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), _type: Default::default() }
    }
}
impl std::fmt::Debug for PerformanceConfigurationAcquireInfoINTEL {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PerformanceConfigurationAcquireInfoINTEL").field("s_type", &self.s_type).field("p_next", &self.p_next).field("_type", &self._type).finish()
    }
}
impl PerformanceConfigurationAcquireInfoINTEL {
    #[inline]
    pub fn into_builder<'a>(self) -> PerformanceConfigurationAcquireInfoINTELBuilder<'a> {
        PerformanceConfigurationAcquireInfoINTELBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceConfigurationAcquireInfoINTEL.html) · Builder of [`PerformanceConfigurationAcquireInfoINTEL`]"]
#[repr(transparent)]
pub struct PerformanceConfigurationAcquireInfoINTELBuilder<'a>(PerformanceConfigurationAcquireInfoINTEL, std::marker::PhantomData<&'a ()>);
impl<'a> PerformanceConfigurationAcquireInfoINTELBuilder<'a> {
    #[inline]
    pub fn new() -> PerformanceConfigurationAcquireInfoINTELBuilder<'a> {
        PerformanceConfigurationAcquireInfoINTELBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn _type(mut self, _type: crate::extensions::intel_performance_query::PerformanceConfigurationTypeINTEL) -> Self {
        self.0._type = _type as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> PerformanceConfigurationAcquireInfoINTEL {
        self.0
    }
}
impl<'a> std::default::Default for PerformanceConfigurationAcquireInfoINTELBuilder<'a> {
    fn default() -> PerformanceConfigurationAcquireInfoINTELBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PerformanceConfigurationAcquireInfoINTELBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PerformanceConfigurationAcquireInfoINTELBuilder<'a> {
    type Target = PerformanceConfigurationAcquireInfoINTEL;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PerformanceConfigurationAcquireInfoINTELBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::intel_performance_query`]"]
impl crate::DeviceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkInitializePerformanceApiINTEL.html) · Function"]
    #[doc(alias = "vkInitializePerformanceApiINTEL")]
    pub unsafe fn initialize_performance_api_intel(&self, initialize_info: &crate::extensions::intel_performance_query::InitializePerformanceApiInfoINTEL) -> crate::utils::VulkanResult<()> {
        let _function = self.initialize_performance_api_intel.expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(self.handle, initialize_info as _);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkUninitializePerformanceApiINTEL.html) · Function"]
    #[doc(alias = "vkUninitializePerformanceApiINTEL")]
    pub unsafe fn uninitialize_performance_api_intel(&self) -> () {
        let _function = self.uninitialize_performance_api_intel.expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(self.handle);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetPerformanceMarkerINTEL.html) · Function"]
    #[doc(alias = "vkCmdSetPerformanceMarkerINTEL")]
    pub unsafe fn cmd_set_performance_marker_intel(&self, command_buffer: crate::vk1_0::CommandBuffer, marker_info: &crate::extensions::intel_performance_query::PerformanceMarkerInfoINTEL) -> crate::utils::VulkanResult<()> {
        let _function = self.cmd_set_performance_marker_intel.expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(command_buffer as _, marker_info as _);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetPerformanceStreamMarkerINTEL.html) · Function"]
    #[doc(alias = "vkCmdSetPerformanceStreamMarkerINTEL")]
    pub unsafe fn cmd_set_performance_stream_marker_intel(&self, command_buffer: crate::vk1_0::CommandBuffer, marker_info: &crate::extensions::intel_performance_query::PerformanceStreamMarkerInfoINTEL) -> crate::utils::VulkanResult<()> {
        let _function = self.cmd_set_performance_stream_marker_intel.expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(command_buffer as _, marker_info as _);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetPerformanceOverrideINTEL.html) · Function"]
    #[doc(alias = "vkCmdSetPerformanceOverrideINTEL")]
    pub unsafe fn cmd_set_performance_override_intel(&self, command_buffer: crate::vk1_0::CommandBuffer, override_info: &crate::extensions::intel_performance_query::PerformanceOverrideInfoINTEL) -> crate::utils::VulkanResult<()> {
        let _function = self.cmd_set_performance_override_intel.expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(command_buffer as _, override_info as _);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAcquirePerformanceConfigurationINTEL.html) · Function"]
    #[doc(alias = "vkAcquirePerformanceConfigurationINTEL")]
    pub unsafe fn acquire_performance_configuration_intel(&self, acquire_info: &crate::extensions::intel_performance_query::PerformanceConfigurationAcquireInfoINTEL) -> crate::utils::VulkanResult<crate::extensions::intel_performance_query::PerformanceConfigurationINTEL> {
        let _function = self.acquire_performance_configuration_intel.expect(crate::NOT_LOADED_MESSAGE);
        let mut configuration = Default::default();
        let _return = _function(self.handle, acquire_info as _, &mut configuration);
        crate::utils::VulkanResult::new(_return, configuration)
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkReleasePerformanceConfigurationINTEL.html) · Function"]
    #[doc(alias = "vkReleasePerformanceConfigurationINTEL")]
    pub unsafe fn release_performance_configuration_intel(&self, configuration: crate::extensions::intel_performance_query::PerformanceConfigurationINTEL) -> crate::utils::VulkanResult<()> {
        let _function = self.release_performance_configuration_intel.expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(self.handle, configuration as _);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueueSetPerformanceConfigurationINTEL.html) · Function"]
    #[doc(alias = "vkQueueSetPerformanceConfigurationINTEL")]
    pub unsafe fn queue_set_performance_configuration_intel(&self, queue: crate::vk1_0::Queue, configuration: crate::extensions::intel_performance_query::PerformanceConfigurationINTEL) -> crate::utils::VulkanResult<()> {
        let _function = self.queue_set_performance_configuration_intel.expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(queue as _, configuration as _);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPerformanceParameterINTEL.html) · Function"]
    #[doc(alias = "vkGetPerformanceParameterINTEL")]
    pub unsafe fn get_performance_parameter_intel(&self, parameter: crate::extensions::intel_performance_query::PerformanceParameterTypeINTEL) -> crate::utils::VulkanResult<crate::extensions::intel_performance_query::PerformanceValueINTEL> {
        let _function = self.get_performance_parameter_intel.expect(crate::NOT_LOADED_MESSAGE);
        let mut value = Default::default();
        let _return = _function(self.handle, parameter as _, &mut value);
        crate::utils::VulkanResult::new(_return, value)
    }
}
