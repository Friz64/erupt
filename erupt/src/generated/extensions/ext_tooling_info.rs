#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_TOOLING_INFO_SPEC_VERSION")]
pub const EXT_TOOLING_INFO_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_TOOLING_INFO_EXTENSION_NAME")]
pub const EXT_TOOLING_INFO_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_tooling_info");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_PHYSICAL_DEVICE_TOOL_PROPERTIES_EXT: *const std::os::raw::c_char = crate::cstr!("vkGetPhysicalDeviceToolPropertiesEXT");
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkToolPurposeFlagsEXT.html) · Bitmask of [`ToolPurposeFlagBitsEXT`]"] # [doc (alias = "VkToolPurposeFlagsEXT")] # [derive (Default)] # [repr (transparent)] pub struct ToolPurposeFlagsEXT : u32 { const VALIDATION_EXT = ToolPurposeFlagBitsEXT :: VALIDATION_EXT . 0 ; const PROFILING_EXT = ToolPurposeFlagBitsEXT :: PROFILING_EXT . 0 ; const TRACING_EXT = ToolPurposeFlagBitsEXT :: TRACING_EXT . 0 ; const ADDITIONAL_FEATURES_EXT = ToolPurposeFlagBitsEXT :: ADDITIONAL_FEATURES_EXT . 0 ; const MODIFYING_FEATURES_EXT = ToolPurposeFlagBitsEXT :: MODIFYING_FEATURES_EXT . 0 ; const DEBUG_REPORTING_EXT = ToolPurposeFlagBitsEXT :: DEBUG_REPORTING_EXT . 0 ; const DEBUG_MARKERS_EXT = ToolPurposeFlagBitsEXT :: DEBUG_MARKERS_EXT . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkToolPurposeFlagBitsEXT.html) · Bits enum of [`ToolPurposeFlagsEXT`]"]
#[doc(alias = "VkToolPurposeFlagBitsEXT")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct ToolPurposeFlagBitsEXT(pub u32);
impl ToolPurposeFlagBitsEXT {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> ToolPurposeFlagsEXT {
        ToolPurposeFlagsEXT::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for ToolPurposeFlagBitsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::VALIDATION_EXT => "VALIDATION_EXT",
            &Self::PROFILING_EXT => "PROFILING_EXT",
            &Self::TRACING_EXT => "TRACING_EXT",
            &Self::ADDITIONAL_FEATURES_EXT => "ADDITIONAL_FEATURES_EXT",
            &Self::MODIFYING_FEATURES_EXT => "MODIFYING_FEATURES_EXT",
            &Self::DEBUG_REPORTING_EXT => "DEBUG_REPORTING_EXT",
            &Self::DEBUG_MARKERS_EXT => "DEBUG_MARKERS_EXT",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::ext_tooling_info`]"]
impl ToolPurposeFlagBitsEXT {
    pub const VALIDATION_EXT: Self = Self(1);
    pub const PROFILING_EXT: Self = Self(2);
    pub const TRACING_EXT: Self = Self(4);
    pub const ADDITIONAL_FEATURES_EXT: Self = Self(8);
    pub const MODIFYING_FEATURES_EXT: Self = Self(16);
    pub const DEBUG_REPORTING_EXT: Self = Self(32);
    pub const DEBUG_MARKERS_EXT: Self = Self(64);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceToolPropertiesEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceToolPropertiesEXT =
    unsafe extern "system" fn(physical_device: crate::vk1_0::PhysicalDevice, p_tool_count: *mut u32, p_tool_properties: *mut crate::extensions::ext_tooling_info::PhysicalDeviceToolPropertiesEXT) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceToolPropertiesEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceToolPropertiesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceToolPropertiesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub name: [std::os::raw::c_char; 256],
    pub version: [std::os::raw::c_char; 256],
    pub purposes: crate::extensions::ext_tooling_info::ToolPurposeFlagsEXT,
    pub description: [std::os::raw::c_char; 256],
    pub layer: [std::os::raw::c_char; 256],
}
impl Default for PhysicalDeviceToolPropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_TOOL_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            name: unsafe { std::mem::zeroed() },
            version: unsafe { std::mem::zeroed() },
            purposes: Default::default(),
            description: unsafe { std::mem::zeroed() },
            layer: unsafe { std::mem::zeroed() },
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceToolPropertiesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceToolPropertiesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("name", unsafe { &std::ffi::CStr::from_ptr(self.name.as_ptr()) })
            .field("version", unsafe { &std::ffi::CStr::from_ptr(self.version.as_ptr()) })
            .field("purposes", &self.purposes)
            .field("description", unsafe { &std::ffi::CStr::from_ptr(self.description.as_ptr()) })
            .field("layer", unsafe { &std::ffi::CStr::from_ptr(self.layer.as_ptr()) })
            .finish()
    }
}
impl PhysicalDeviceToolPropertiesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceToolPropertiesEXTBuilder<'a> {
        PhysicalDeviceToolPropertiesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceToolPropertiesEXT.html) · Builder of [`PhysicalDeviceToolPropertiesEXT`]"]
#[repr(transparent)]
pub struct PhysicalDeviceToolPropertiesEXTBuilder<'a>(PhysicalDeviceToolPropertiesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceToolPropertiesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceToolPropertiesEXTBuilder<'a> {
        PhysicalDeviceToolPropertiesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn name(mut self, name: [std::os::raw::c_char; 256]) -> Self {
        self.0.name = name as _;
        self
    }
    #[inline]
    pub fn version(mut self, version: [std::os::raw::c_char; 256]) -> Self {
        self.0.version = version as _;
        self
    }
    #[inline]
    pub fn purposes(mut self, purposes: crate::extensions::ext_tooling_info::ToolPurposeFlagsEXT) -> Self {
        self.0.purposes = purposes as _;
        self
    }
    #[inline]
    pub fn description(mut self, description: [std::os::raw::c_char; 256]) -> Self {
        self.0.description = description as _;
        self
    }
    #[inline]
    pub fn layer(mut self, layer: [std::os::raw::c_char; 256]) -> Self {
        self.0.layer = layer as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceToolPropertiesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceToolPropertiesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceToolPropertiesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceToolPropertiesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceToolPropertiesEXTBuilder<'a> {
    type Target = PhysicalDeviceToolPropertiesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceToolPropertiesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<PhysicalDeviceToolPropertiesEXT> for PhysicalDeviceToolPropertiesEXTBuilder<'_> {}
#[doc = "Provided by [`crate::extensions::ext_tooling_info`]"]
impl crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceToolPropertiesEXT.html) · Function"]
    #[doc(alias = "vkGetPhysicalDeviceToolPropertiesEXT")]
    pub unsafe fn get_physical_device_tool_properties_ext(&self, physical_device: crate::vk1_0::PhysicalDevice, tool_count: Option<u32>) -> crate::utils::VulkanResult<Vec<crate::extensions::ext_tooling_info::PhysicalDeviceToolPropertiesEXT>> {
        let _function = self.get_physical_device_tool_properties_ext.expect("`get_physical_device_tool_properties_ext` is not loaded");
        let mut tool_count = match tool_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                _function(physical_device as _, &mut v, std::ptr::null_mut());
                v
            }
        };
        let mut tool_properties = vec![Default::default(); tool_count as _];
        let _return = _function(physical_device as _, &mut tool_count, tool_properties.as_mut_ptr());
        crate::utils::VulkanResult::new(_return, tool_properties)
    }
}
