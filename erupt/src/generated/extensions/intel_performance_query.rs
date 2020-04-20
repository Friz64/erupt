# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_INTEL_performance_query.html)\n\n## Extends\n- [`ObjectType`](../../vk1_0/struct.ObjectType.html)\n- [`QueryType`](../../vk1_0/struct.QueryType.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const INTEL_PERFORMANCE_QUERY_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const INTEL_PERFORMANCE_QUERY_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_INTEL_performance_query");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkInitializePerformanceApiINTEL.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkInitializePerformanceApiINTEL = unsafe extern "system" fn ( device : crate :: vk1_0 :: Device , p_initialize_info : * const crate :: extensions :: intel_performance_query :: InitializePerformanceApiInfoINTEL , ) -> crate :: vk1_0 :: Result ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkUninitializePerformanceApiINTEL.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkUninitializePerformanceApiINTEL =
    unsafe extern "system" fn(device: crate::vk1_0::Device) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetPerformanceMarkerINTEL.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetPerformanceMarkerINTEL = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    p_marker_info: *const crate::extensions::intel_performance_query::PerformanceMarkerInfoINTEL,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetPerformanceStreamMarkerINTEL.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetPerformanceStreamMarkerINTEL = unsafe extern "system" fn ( command_buffer : crate :: vk1_0 :: CommandBuffer , p_marker_info : * const crate :: extensions :: intel_performance_query :: PerformanceStreamMarkerInfoINTEL , ) -> crate :: vk1_0 :: Result ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetPerformanceOverrideINTEL.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetPerformanceOverrideINTEL = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    p_override_info: *const crate::extensions::intel_performance_query::PerformanceOverrideInfoINTEL,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAcquirePerformanceConfigurationINTEL.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkAcquirePerformanceConfigurationINTEL = unsafe extern "system" fn ( device : crate :: vk1_0 :: Device , p_acquire_info : * const crate :: extensions :: intel_performance_query :: PerformanceConfigurationAcquireInfoINTEL , p_configuration : * mut crate :: extensions :: intel_performance_query :: PerformanceConfigurationINTEL , ) -> crate :: vk1_0 :: Result ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkReleasePerformanceConfigurationINTEL.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkReleasePerformanceConfigurationINTEL =
    unsafe extern "system" fn(
        device: crate::vk1_0::Device,
        configuration: crate::extensions::intel_performance_query::PerformanceConfigurationINTEL,
    ) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueueSetPerformanceConfigurationINTEL.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkQueueSetPerformanceConfigurationINTEL =
    unsafe extern "system" fn(
        queue: crate::vk1_0::Queue,
        configuration: crate::extensions::intel_performance_query::PerformanceConfigurationINTEL,
    ) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPerformanceParameterINTEL.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPerformanceParameterINTEL = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    parameter: crate::extensions::intel_performance_query::PerformanceParameterTypeINTEL,
    p_value: *mut crate::extensions::intel_performance_query::PerformanceValueINTEL,
) -> crate::vk1_0::Result;
#[doc = "Provides Device Commands for [`IntelPerformanceQueryDeviceLoaderExt`](trait.IntelPerformanceQueryDeviceLoaderExt.html)"]
pub struct IntelPerformanceQueryDeviceCommands {
    pub initialize_performance_api_intel: PFN_vkInitializePerformanceApiINTEL,
    pub uninitialize_performance_api_intel: PFN_vkUninitializePerformanceApiINTEL,
    pub cmd_set_performance_marker_intel: PFN_vkCmdSetPerformanceMarkerINTEL,
    pub cmd_set_performance_stream_marker_intel: PFN_vkCmdSetPerformanceStreamMarkerINTEL,
    pub cmd_set_performance_override_intel: PFN_vkCmdSetPerformanceOverrideINTEL,
    pub acquire_performance_configuration_intel: PFN_vkAcquirePerformanceConfigurationINTEL,
    pub release_performance_configuration_intel: PFN_vkReleasePerformanceConfigurationINTEL,
    pub queue_set_performance_configuration_intel: PFN_vkQueueSetPerformanceConfigurationINTEL,
    pub get_performance_parameter_intel: PFN_vkGetPerformanceParameterINTEL,
}
impl IntelPerformanceQueryDeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<IntelPerformanceQueryDeviceCommands> {
        unsafe {
            Some(IntelPerformanceQueryDeviceCommands {
                initialize_performance_api_intel: std::mem::transmute(
                    loader.symbol("vkInitializePerformanceApiINTEL")?,
                ),
                uninitialize_performance_api_intel: std::mem::transmute(
                    loader.symbol("vkUninitializePerformanceApiINTEL")?,
                ),
                cmd_set_performance_marker_intel: std::mem::transmute(
                    loader.symbol("vkCmdSetPerformanceMarkerINTEL")?,
                ),
                cmd_set_performance_stream_marker_intel: std::mem::transmute(
                    loader.symbol("vkCmdSetPerformanceStreamMarkerINTEL")?,
                ),
                cmd_set_performance_override_intel: std::mem::transmute(
                    loader.symbol("vkCmdSetPerformanceOverrideINTEL")?,
                ),
                acquire_performance_configuration_intel: std::mem::transmute(
                    loader.symbol("vkAcquirePerformanceConfigurationINTEL")?,
                ),
                release_performance_configuration_intel: std::mem::transmute(
                    loader.symbol("vkReleasePerformanceConfigurationINTEL")?,
                ),
                queue_set_performance_configuration_intel: std::mem::transmute(
                    loader.symbol("vkQueueSetPerformanceConfigurationINTEL")?,
                ),
                get_performance_parameter_intel: std::mem::transmute(
                    loader.symbol("vkGetPerformanceParameterINTEL")?,
                ),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`IntelPerformanceQueryDeviceCommands`](struct.IntelPerformanceQueryDeviceCommands.html)"]
pub trait IntelPerformanceQueryDeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkInitializePerformanceApiINTEL.html) · Device Command"]
    unsafe fn initialize_performance_api_intel(
        &self,
        initialize_info : & crate :: extensions :: intel_performance_query :: InitializePerformanceApiInfoINTEL,
    ) -> crate::utils::VulkanResult<()>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkUninitializePerformanceApiINTEL.html) · Device Command"]
    unsafe fn uninitialize_performance_api_intel(&self) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetPerformanceMarkerINTEL.html) · Device Command"]
    unsafe fn cmd_set_performance_marker_intel(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        marker_info: &crate::extensions::intel_performance_query::PerformanceMarkerInfoINTEL,
    ) -> crate::utils::VulkanResult<()>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetPerformanceStreamMarkerINTEL.html) · Device Command"]
    unsafe fn cmd_set_performance_stream_marker_intel(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        marker_info: &crate::extensions::intel_performance_query::PerformanceStreamMarkerInfoINTEL,
    ) -> crate::utils::VulkanResult<()>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetPerformanceOverrideINTEL.html) · Device Command"]
    unsafe fn cmd_set_performance_override_intel(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        override_info: &crate::extensions::intel_performance_query::PerformanceOverrideInfoINTEL,
    ) -> crate::utils::VulkanResult<()>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAcquirePerformanceConfigurationINTEL.html) · Device Command"]
    unsafe fn acquire_performance_configuration_intel(
        &self,
        acquire_info : & crate :: extensions :: intel_performance_query :: PerformanceConfigurationAcquireInfoINTEL,
        configuration: Option<
            crate::extensions::intel_performance_query::PerformanceConfigurationINTEL,
        >,
    ) -> crate::utils::VulkanResult<
        crate::extensions::intel_performance_query::PerformanceConfigurationINTEL,
    >;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkReleasePerformanceConfigurationINTEL.html) · Device Command"]
    unsafe fn release_performance_configuration_intel(
        &self,
        configuration: crate::extensions::intel_performance_query::PerformanceConfigurationINTEL,
    ) -> crate::utils::VulkanResult<()>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueueSetPerformanceConfigurationINTEL.html) · Device Command"]
    unsafe fn queue_set_performance_configuration_intel(
        &self,
        queue: crate::vk1_0::Queue,
        configuration: crate::extensions::intel_performance_query::PerformanceConfigurationINTEL,
    ) -> crate::utils::VulkanResult<()>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPerformanceParameterINTEL.html) · Device Command"]
    unsafe fn get_performance_parameter_intel(
        &self,
        parameter: crate::extensions::intel_performance_query::PerformanceParameterTypeINTEL,
        value: Option<crate::extensions::intel_performance_query::PerformanceValueINTEL>,
    ) -> crate::utils::VulkanResult<crate::extensions::intel_performance_query::PerformanceValueINTEL>;
}
impl IntelPerformanceQueryDeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkInitializePerformanceApiINTEL.html) · Device Command"]
    unsafe fn initialize_performance_api_intel(
        &self,
        initialize_info : & crate :: extensions :: intel_performance_query :: InitializePerformanceApiInfoINTEL,
    ) -> crate::utils::VulkanResult<()> {
        let function = self
            .intel_performance_query
            .as_ref()
            .expect("`intel_performance_query` not loaded")
            .initialize_performance_api_intel;
        let _val = function(self.handle, initialize_info);
        crate::utils::VulkanResult::new(_val, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkUninitializePerformanceApiINTEL.html) · Device Command"]
    unsafe fn uninitialize_performance_api_intel(&self) -> () {
        let function = self
            .intel_performance_query
            .as_ref()
            .expect("`intel_performance_query` not loaded")
            .uninitialize_performance_api_intel;
        let _val = function(self.handle);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetPerformanceMarkerINTEL.html) · Device Command"]
    unsafe fn cmd_set_performance_marker_intel(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        marker_info: &crate::extensions::intel_performance_query::PerformanceMarkerInfoINTEL,
    ) -> crate::utils::VulkanResult<()> {
        let function = self
            .intel_performance_query
            .as_ref()
            .expect("`intel_performance_query` not loaded")
            .cmd_set_performance_marker_intel;
        let _val = function(command_buffer, marker_info);
        crate::utils::VulkanResult::new(_val, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetPerformanceStreamMarkerINTEL.html) · Device Command"]
    unsafe fn cmd_set_performance_stream_marker_intel(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        marker_info: &crate::extensions::intel_performance_query::PerformanceStreamMarkerInfoINTEL,
    ) -> crate::utils::VulkanResult<()> {
        let function = self
            .intel_performance_query
            .as_ref()
            .expect("`intel_performance_query` not loaded")
            .cmd_set_performance_stream_marker_intel;
        let _val = function(command_buffer, marker_info);
        crate::utils::VulkanResult::new(_val, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetPerformanceOverrideINTEL.html) · Device Command"]
    unsafe fn cmd_set_performance_override_intel(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        override_info: &crate::extensions::intel_performance_query::PerformanceOverrideInfoINTEL,
    ) -> crate::utils::VulkanResult<()> {
        let function = self
            .intel_performance_query
            .as_ref()
            .expect("`intel_performance_query` not loaded")
            .cmd_set_performance_override_intel;
        let _val = function(command_buffer, override_info);
        crate::utils::VulkanResult::new(_val, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAcquirePerformanceConfigurationINTEL.html) · Device Command"]
    unsafe fn acquire_performance_configuration_intel(
        &self,
        acquire_info : & crate :: extensions :: intel_performance_query :: PerformanceConfigurationAcquireInfoINTEL,
        configuration: Option<
            crate::extensions::intel_performance_query::PerformanceConfigurationINTEL,
        >,
    ) -> crate::utils::VulkanResult<
        crate::extensions::intel_performance_query::PerformanceConfigurationINTEL,
    > {
        let function = self
            .intel_performance_query
            .as_ref()
            .expect("`intel_performance_query` not loaded")
            .acquire_performance_configuration_intel;
        let mut configuration = configuration.unwrap_or_else(|| Default::default());
        let _val = function(self.handle, acquire_info, &mut configuration);
        crate::utils::VulkanResult::new(_val, configuration)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkReleasePerformanceConfigurationINTEL.html) · Device Command"]
    unsafe fn release_performance_configuration_intel(
        &self,
        configuration: crate::extensions::intel_performance_query::PerformanceConfigurationINTEL,
    ) -> crate::utils::VulkanResult<()> {
        let function = self
            .intel_performance_query
            .as_ref()
            .expect("`intel_performance_query` not loaded")
            .release_performance_configuration_intel;
        let _val = function(self.handle, configuration);
        crate::utils::VulkanResult::new(_val, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueueSetPerformanceConfigurationINTEL.html) · Device Command"]
    unsafe fn queue_set_performance_configuration_intel(
        &self,
        queue: crate::vk1_0::Queue,
        configuration: crate::extensions::intel_performance_query::PerformanceConfigurationINTEL,
    ) -> crate::utils::VulkanResult<()> {
        let function = self
            .intel_performance_query
            .as_ref()
            .expect("`intel_performance_query` not loaded")
            .queue_set_performance_configuration_intel;
        let _val = function(queue, configuration);
        crate::utils::VulkanResult::new(_val, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPerformanceParameterINTEL.html) · Device Command"]
    unsafe fn get_performance_parameter_intel(
        &self,
        parameter: crate::extensions::intel_performance_query::PerformanceParameterTypeINTEL,
        value: Option<crate::extensions::intel_performance_query::PerformanceValueINTEL>,
    ) -> crate::utils::VulkanResult<crate::extensions::intel_performance_query::PerformanceValueINTEL>
    {
        let function = self
            .intel_performance_query
            .as_ref()
            .expect("`intel_performance_query` not loaded")
            .get_performance_parameter_intel;
        let mut value = value.unwrap_or_else(|| Default::default());
        let _val = function(self.handle, parameter, &mut value);
        crate::utils::VulkanResult::new(_val, value)
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkInitializePerformanceApiInfoINTEL.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InitializePerformanceApiInfoINTEL {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub p_user_data: *mut std::ffi::c_void,
}
impl InitializePerformanceApiInfoINTEL {
    #[inline]
    pub fn builder<'a>(self) -> InitializePerformanceApiInfoINTELBuilder<'a> {
        InitializePerformanceApiInfoINTELBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for InitializePerformanceApiInfoINTEL {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("InitializePerformanceApiInfoINTEL")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("p_user_data", &self.p_user_data)
            .finish()
    }
}
impl Default for InitializePerformanceApiInfoINTEL {
    fn default() -> InitializePerformanceApiInfoINTEL {
        InitializePerformanceApiInfoINTEL {
            s_type: crate::vk1_0::StructureType::INITIALIZE_PERFORMANCE_API_INFO_INTEL,
            p_next: std::ptr::null(),
            p_user_data: std::ptr::null_mut(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkInitializePerformanceApiInfoINTEL.html) · Builder of [`InitializePerformanceApiInfoINTEL`](struct.InitializePerformanceApiInfoINTEL.html)"]
#[repr(transparent)]
pub struct InitializePerformanceApiInfoINTELBuilder<'a>(
    InitializePerformanceApiInfoINTEL,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> InitializePerformanceApiInfoINTELBuilder<'a> {
    #[inline]
    pub fn new() -> InitializePerformanceApiInfoINTELBuilder<'a> {
        InitializePerformanceApiInfoINTELBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn user_data(mut self, user_data: &'a mut std::ffi::c_void) -> Self {
        self.0.p_user_data = user_data;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> InitializePerformanceApiInfoINTEL {
        self.0
    }
}
impl<'a> std::fmt::Debug for InitializePerformanceApiInfoINTELBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceMarkerInfoINTEL.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PerformanceMarkerInfoINTEL {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub marker: u64,
}
impl PerformanceMarkerInfoINTEL {
    #[inline]
    pub fn builder<'a>(self) -> PerformanceMarkerInfoINTELBuilder<'a> {
        PerformanceMarkerInfoINTELBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PerformanceMarkerInfoINTEL {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PerformanceMarkerInfoINTEL")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("marker", &self.marker)
            .finish()
    }
}
impl Default for PerformanceMarkerInfoINTEL {
    fn default() -> PerformanceMarkerInfoINTEL {
        PerformanceMarkerInfoINTEL {
            s_type: crate::vk1_0::StructureType::PERFORMANCE_MARKER_INFO_INTEL,
            p_next: std::ptr::null(),
            marker: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceMarkerInfoINTEL.html) · Builder of [`PerformanceMarkerInfoINTEL`](struct.PerformanceMarkerInfoINTEL.html)"]
#[repr(transparent)]
pub struct PerformanceMarkerInfoINTELBuilder<'a>(
    PerformanceMarkerInfoINTEL,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PerformanceMarkerInfoINTELBuilder<'a> {
    #[inline]
    pub fn new() -> PerformanceMarkerInfoINTELBuilder<'a> {
        PerformanceMarkerInfoINTELBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn marker(mut self, marker: u64) -> Self {
        self.0.marker = marker;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PerformanceMarkerInfoINTEL {
        self.0
    }
}
impl<'a> std::fmt::Debug for PerformanceMarkerInfoINTELBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PerformanceStreamMarkerInfoINTEL {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub marker: u32,
}
impl PerformanceStreamMarkerInfoINTEL {
    #[inline]
    pub fn builder<'a>(self) -> PerformanceStreamMarkerInfoINTELBuilder<'a> {
        PerformanceStreamMarkerInfoINTELBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PerformanceStreamMarkerInfoINTEL {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PerformanceStreamMarkerInfoINTEL")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("marker", &self.marker)
            .finish()
    }
}
impl Default for PerformanceStreamMarkerInfoINTEL {
    fn default() -> PerformanceStreamMarkerInfoINTEL {
        PerformanceStreamMarkerInfoINTEL {
            s_type: crate::vk1_0::StructureType::PERFORMANCE_STREAM_MARKER_INFO_INTEL,
            p_next: std::ptr::null(),
            marker: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceStreamMarkerInfoINTEL.html) · Builder of [`PerformanceStreamMarkerInfoINTEL`](struct.PerformanceStreamMarkerInfoINTEL.html)"]
#[repr(transparent)]
pub struct PerformanceStreamMarkerInfoINTELBuilder<'a>(
    PerformanceStreamMarkerInfoINTEL,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PerformanceStreamMarkerInfoINTELBuilder<'a> {
    #[inline]
    pub fn new() -> PerformanceStreamMarkerInfoINTELBuilder<'a> {
        PerformanceStreamMarkerInfoINTELBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn marker(mut self, marker: u32) -> Self {
        self.0.marker = marker;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PerformanceStreamMarkerInfoINTEL {
        self.0
    }
}
impl<'a> std::fmt::Debug for PerformanceStreamMarkerInfoINTELBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
    #[inline]
    pub fn builder<'a>(self) -> PerformanceOverrideInfoINTELBuilder<'a> {
        PerformanceOverrideInfoINTELBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PerformanceOverrideInfoINTEL {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PerformanceOverrideInfoINTEL")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("_type", &self._type)
            .field("enable", &(self.enable != 0))
            .field("parameter", &self.parameter)
            .finish()
    }
}
impl Default for PerformanceOverrideInfoINTEL {
    fn default() -> PerformanceOverrideInfoINTEL {
        PerformanceOverrideInfoINTEL {
            s_type: crate::vk1_0::StructureType::PERFORMANCE_OVERRIDE_INFO_INTEL,
            p_next: std::ptr::null(),
            _type: Default::default(),
            enable: Default::default(),
            parameter: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceOverrideInfoINTEL.html) · Builder of [`PerformanceOverrideInfoINTEL`](struct.PerformanceOverrideInfoINTEL.html)"]
#[repr(transparent)]
pub struct PerformanceOverrideInfoINTELBuilder<'a>(
    PerformanceOverrideInfoINTEL,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PerformanceOverrideInfoINTELBuilder<'a> {
    #[inline]
    pub fn new() -> PerformanceOverrideInfoINTELBuilder<'a> {
        PerformanceOverrideInfoINTELBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn _type(
        mut self,
        _type: crate::extensions::intel_performance_query::PerformanceOverrideTypeINTEL,
    ) -> Self {
        self.0._type = _type;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn enable(mut self, enable: bool) -> Self {
        self.0.enable = enable as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn parameter(mut self, parameter: u64) -> Self {
        self.0.parameter = parameter;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PerformanceOverrideInfoINTEL {
        self.0
    }
}
impl<'a> std::fmt::Debug for PerformanceOverrideInfoINTELBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceOverrideTypeINTEL.html) · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PerformanceOverrideTypeINTEL(pub i32);
#[doc = "[Part of `extensions::intel_performance_query`](../../extensions/intel_performance_query/index.html)"]
impl PerformanceOverrideTypeINTEL {
    pub const NULL_HARDWARE_INTEL: Self = Self(0);
    pub const FLUSH_GPU_CACHES_INTEL: Self = Self(1);
}
impl std::fmt::Debug for PerformanceOverrideTypeINTEL {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::NULL_HARDWARE_INTEL => "NULL_HARDWARE_INTEL",
            &Self::FLUSH_GPU_CACHES_INTEL => "FLUSH_GPU_CACHES_INTEL",
            _ => "(unknown)",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceConfigurationAcquireInfoINTEL.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PerformanceConfigurationAcquireInfoINTEL {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub _type: crate::extensions::intel_performance_query::PerformanceConfigurationTypeINTEL,
}
impl PerformanceConfigurationAcquireInfoINTEL {
    #[inline]
    pub fn builder<'a>(self) -> PerformanceConfigurationAcquireInfoINTELBuilder<'a> {
        PerformanceConfigurationAcquireInfoINTELBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PerformanceConfigurationAcquireInfoINTEL {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PerformanceConfigurationAcquireInfoINTEL")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("_type", &self._type)
            .finish()
    }
}
impl Default for PerformanceConfigurationAcquireInfoINTEL {
    fn default() -> PerformanceConfigurationAcquireInfoINTEL {
        PerformanceConfigurationAcquireInfoINTEL {
            s_type: crate::vk1_0::StructureType::PERFORMANCE_CONFIGURATION_ACQUIRE_INFO_INTEL,
            p_next: std::ptr::null(),
            _type: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceConfigurationAcquireInfoINTEL.html) · Builder of [`PerformanceConfigurationAcquireInfoINTEL`](struct.PerformanceConfigurationAcquireInfoINTEL.html)"]
#[repr(transparent)]
pub struct PerformanceConfigurationAcquireInfoINTELBuilder<'a>(
    PerformanceConfigurationAcquireInfoINTEL,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PerformanceConfigurationAcquireInfoINTELBuilder<'a> {
    #[inline]
    pub fn new() -> PerformanceConfigurationAcquireInfoINTELBuilder<'a> {
        PerformanceConfigurationAcquireInfoINTELBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn _type(
        mut self,
        _type: crate::extensions::intel_performance_query::PerformanceConfigurationTypeINTEL,
    ) -> Self {
        self.0._type = _type;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PerformanceConfigurationAcquireInfoINTEL {
        self.0
    }
}
impl<'a> std::fmt::Debug for PerformanceConfigurationAcquireInfoINTELBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceConfigurationTypeINTEL.html) · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PerformanceConfigurationTypeINTEL(pub i32);
#[doc = "[Part of `extensions::intel_performance_query`](../../extensions/intel_performance_query/index.html)"]
impl PerformanceConfigurationTypeINTEL {
    pub const COMMAND_QUEUE_METRICS_DISCOVERY_ACTIVATED_INTEL: Self = Self(0);
}
impl std::fmt::Debug for PerformanceConfigurationTypeINTEL {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::COMMAND_QUEUE_METRICS_DISCOVERY_ACTIVATED_INTEL => {
                "COMMAND_QUEUE_METRICS_DISCOVERY_ACTIVATED_INTEL"
            }
            _ => "(unknown)",
        })
    }
}
crate :: non_dispatchable_handle ! ( PerformanceConfigurationINTEL , PERFORMANCE_CONFIGURATION_INTEL , doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceConfigurationINTEL.html) · Non-dispatchable Handle" ) ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceParameterTypeINTEL.html) · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PerformanceParameterTypeINTEL(pub i32);
#[doc = "[Part of `extensions::intel_performance_query`](../../extensions/intel_performance_query/index.html)"]
impl PerformanceParameterTypeINTEL {
    pub const HW_COUNTERS_SUPPORTED_INTEL: Self = Self(0);
    pub const STREAM_MARKER_VALIDS_INTEL: Self = Self(1);
}
impl std::fmt::Debug for PerformanceParameterTypeINTEL {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::HW_COUNTERS_SUPPORTED_INTEL => "HW_COUNTERS_SUPPORTED_INTEL",
            &Self::STREAM_MARKER_VALIDS_INTEL => "STREAM_MARKER_VALIDS_INTEL",
            _ => "(unknown)",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceValueINTEL.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PerformanceValueINTEL {
    pub _type: crate::extensions::intel_performance_query::PerformanceValueTypeINTEL,
    pub data: crate::extensions::intel_performance_query::PerformanceValueDataINTEL,
}
impl PerformanceValueINTEL {
    #[inline]
    pub fn builder<'a>(self) -> PerformanceValueINTELBuilder<'a> {
        PerformanceValueINTELBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PerformanceValueINTEL {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PerformanceValueINTEL")
            .field("_type", &self._type)
            .field("data", &self.data)
            .finish()
    }
}
impl Default for PerformanceValueINTEL {
    fn default() -> PerformanceValueINTEL {
        PerformanceValueINTEL {
            _type: Default::default(),
            data: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceValueINTEL.html) · Builder of [`PerformanceValueINTEL`](struct.PerformanceValueINTEL.html)"]
#[repr(transparent)]
pub struct PerformanceValueINTELBuilder<'a>(
    PerformanceValueINTEL,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PerformanceValueINTELBuilder<'a> {
    #[inline]
    pub fn new() -> PerformanceValueINTELBuilder<'a> {
        PerformanceValueINTELBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn _type(
        mut self,
        _type: crate::extensions::intel_performance_query::PerformanceValueTypeINTEL,
    ) -> Self {
        self.0._type = _type;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn data(
        mut self,
        data: crate::extensions::intel_performance_query::PerformanceValueDataINTEL,
    ) -> Self {
        self.0.data = data;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PerformanceValueINTEL {
        self.0
    }
}
impl<'a> std::fmt::Debug for PerformanceValueINTELBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceValueTypeINTEL.html) · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PerformanceValueTypeINTEL(pub i32);
#[doc = "[Part of `extensions::intel_performance_query`](../../extensions/intel_performance_query/index.html)"]
impl PerformanceValueTypeINTEL {
    pub const UINT32_INTEL: Self = Self(0);
    pub const UINT64_INTEL: Self = Self(1);
    pub const FLOAT_INTEL: Self = Self(2);
    pub const BOOL_INTEL: Self = Self(3);
    pub const STRING_INTEL: Self = Self(4);
}
impl std::fmt::Debug for PerformanceValueTypeINTEL {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::UINT32_INTEL => "UINT32_INTEL",
            &Self::UINT64_INTEL => "UINT64_INTEL",
            &Self::FLOAT_INTEL => "FLOAT_INTEL",
            &Self::BOOL_INTEL => "BOOL_INTEL",
            &Self::STRING_INTEL => "STRING_INTEL",
            _ => "(unknown)",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceValueDataINTEL.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub union PerformanceValueDataINTEL {
    pub value32: u32,
    pub value64: u64,
    pub value_float: f32,
    pub value_bool: crate::vk1_0::Bool32,
    pub value_string: *const std::os::raw::c_char,
}
impl std::fmt::Debug for PerformanceValueDataINTEL {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PerformanceValueDataINTEL").finish()
    }
}
impl Default for PerformanceValueDataINTEL {
    fn default() -> PerformanceValueDataINTEL {
        unsafe { std::mem::zeroed() }
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueryPoolSamplingModeINTEL.html) · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct QueryPoolSamplingModeINTEL(pub i32);
#[doc = "[Part of `extensions::intel_performance_query`](../../extensions/intel_performance_query/index.html)"]
impl QueryPoolSamplingModeINTEL {
    pub const MANUAL_INTEL: Self = Self(0);
}
impl std::fmt::Debug for QueryPoolSamplingModeINTEL {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::MANUAL_INTEL => "MANUAL_INTEL",
            _ => "(unknown)",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueryPoolCreateInfoINTEL.html) · Alias"]
pub type QueryPoolCreateInfoINTEL =
    crate::extensions::intel_performance_query::QueryPoolPerformanceQueryCreateInfoINTEL;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueryPoolPerformanceQueryCreateInfoINTEL.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct QueryPoolPerformanceQueryCreateInfoINTEL {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub performance_counters_sampling:
        crate::extensions::intel_performance_query::QueryPoolSamplingModeINTEL,
}
impl QueryPoolPerformanceQueryCreateInfoINTEL {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: crate::ExtendableBy<Self>,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> QueryPoolPerformanceQueryCreateInfoINTELBuilder<'a> {
        QueryPoolPerformanceQueryCreateInfoINTELBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for QueryPoolPerformanceQueryCreateInfoINTEL {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("QueryPoolPerformanceQueryCreateInfoINTEL")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "performance_counters_sampling",
                &self.performance_counters_sampling,
            )
            .finish()
    }
}
impl Default for QueryPoolPerformanceQueryCreateInfoINTEL {
    fn default() -> QueryPoolPerformanceQueryCreateInfoINTEL {
        QueryPoolPerformanceQueryCreateInfoINTEL {
            s_type: crate::vk1_0::StructureType::QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL,
            p_next: std::ptr::null(),
            performance_counters_sampling: Default::default(),
        }
    }
}
impl crate::ExtendableBy<QueryPoolPerformanceQueryCreateInfoINTEL>
    for crate::vk1_0::QueryPoolCreateInfo
{
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueryPoolPerformanceQueryCreateInfoINTEL.html) · Builder of [`QueryPoolPerformanceQueryCreateInfoINTEL`](struct.QueryPoolPerformanceQueryCreateInfoINTEL.html)"]
#[repr(transparent)]
pub struct QueryPoolPerformanceQueryCreateInfoINTELBuilder<'a>(
    QueryPoolPerformanceQueryCreateInfoINTEL,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> QueryPoolPerformanceQueryCreateInfoINTELBuilder<'a> {
    #[inline]
    pub fn new() -> QueryPoolPerformanceQueryCreateInfoINTELBuilder<'a> {
        QueryPoolPerformanceQueryCreateInfoINTELBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn performance_counters_sampling(
        mut self,
        performance_counters_sampling : crate :: extensions :: intel_performance_query :: QueryPoolSamplingModeINTEL,
    ) -> Self {
        self.0.performance_counters_sampling = performance_counters_sampling;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> QueryPoolPerformanceQueryCreateInfoINTEL {
        self.0
    }
}
impl<'a> std::fmt::Debug for QueryPoolPerformanceQueryCreateInfoINTELBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
