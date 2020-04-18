# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_EXT_tooling_info.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_TOOLING_INFO_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_TOOLING_INFO_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_tooling_info");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceToolPropertiesEXT.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceToolPropertiesEXT =
    unsafe extern "system" fn(
        physical_device: crate::vk1_0::PhysicalDevice,
        p_tool_count: *mut u32,
        p_tool_properties: *mut crate::extensions::ext_tooling_info::PhysicalDeviceToolPropertiesEXT,
    ) -> crate::vk1_0::Result;
#[doc = "Provides Instance Commands for [`ExtToolingInfoInstanceLoaderExt`](trait.ExtToolingInfoInstanceLoaderExt.html)"]
pub struct ExtToolingInfoInstanceCommands {
    pub get_physical_device_tool_properties_ext: PFN_vkGetPhysicalDeviceToolPropertiesEXT,
}
impl ExtToolingInfoInstanceCommands {
    #[inline]
    pub fn load(loader: &crate::InstanceLoader) -> Option<ExtToolingInfoInstanceCommands> {
        unsafe {
            Some(ExtToolingInfoInstanceCommands {
                get_physical_device_tool_properties_ext: std::mem::transmute(
                    loader.symbol("vkGetPhysicalDeviceToolPropertiesEXT")?,
                ),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`ExtToolingInfoInstanceCommands`](struct.ExtToolingInfoInstanceCommands.html)"]
pub trait ExtToolingInfoInstanceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceToolPropertiesEXT.html) · Instance Command"]
    unsafe fn get_physical_device_tool_properties_ext(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        tool_count: Option<u32>,
    ) -> crate::utils::VulkanResult<
        Vec<crate::extensions::ext_tooling_info::PhysicalDeviceToolPropertiesEXT>,
    >;
}
impl ExtToolingInfoInstanceLoaderExt for crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceToolPropertiesEXT.html) · Instance Command"]
    unsafe fn get_physical_device_tool_properties_ext(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        tool_count: Option<u32>,
    ) -> crate::utils::VulkanResult<
        Vec<crate::extensions::ext_tooling_info::PhysicalDeviceToolPropertiesEXT>,
    > {
        let function = self
            .ext_tooling_info
            .as_ref()
            .expect("`ext_tooling_info` not loaded")
            .get_physical_device_tool_properties_ext;
        let mut tool_count = tool_count.unwrap_or_else(|| {
            let mut val = Default::default();
            function(physical_device, &mut val, std::ptr::null_mut());
            val
        });
        let mut tool_properties = vec![Default::default(); tool_count as _];
        let _val = function(
            physical_device,
            &mut tool_count,
            tool_properties.as_mut_ptr(),
        );
        crate::utils::VulkanResult::new(_val, tool_properties)
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceToolPropertiesEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceToolPropertiesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub name: [std::os::raw::c_char; crate::vk1_0::MAX_EXTENSION_NAME_SIZE as usize],
    pub version: [std::os::raw::c_char; crate::vk1_0::MAX_EXTENSION_NAME_SIZE as usize],
    pub purposes: crate::extensions::ext_tooling_info::ToolPurposeFlagsEXT,
    pub description: [std::os::raw::c_char; crate::vk1_0::MAX_DESCRIPTION_SIZE as usize],
    pub layer: [std::os::raw::c_char; crate::vk1_0::MAX_EXTENSION_NAME_SIZE as usize],
}
impl PhysicalDeviceToolPropertiesEXT {
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceToolPropertiesEXTBuilder<'a> {
        PhysicalDeviceToolPropertiesEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceToolPropertiesEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceToolPropertiesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("name", &unsafe {
                std::ffi::CStr::from_ptr(self.name.as_ptr() as _)
            })
            .field("version", &unsafe {
                std::ffi::CStr::from_ptr(self.version.as_ptr() as _)
            })
            .field("purposes", &self.purposes)
            .field("description", &unsafe {
                std::ffi::CStr::from_ptr(self.description.as_ptr() as _)
            })
            .field("layer", &unsafe {
                std::ffi::CStr::from_ptr(self.layer.as_ptr() as _)
            })
            .finish()
    }
}
impl Default for PhysicalDeviceToolPropertiesEXT {
    fn default() -> PhysicalDeviceToolPropertiesEXT {
        PhysicalDeviceToolPropertiesEXT {
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
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceToolPropertiesEXT`](struct.PhysicalDeviceToolPropertiesEXT.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceToolPropertiesEXTBuilder<'a>(
    PhysicalDeviceToolPropertiesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceToolPropertiesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceToolPropertiesEXTBuilder<'a> {
        PhysicalDeviceToolPropertiesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn name(
        mut self,
        name: [std::os::raw::c_char; crate::vk1_0::MAX_EXTENSION_NAME_SIZE as usize],
    ) -> Self {
        self.0.name = name;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn version(
        mut self,
        version: [std::os::raw::c_char; crate::vk1_0::MAX_EXTENSION_NAME_SIZE as usize],
    ) -> Self {
        self.0.version = version;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn purposes(
        mut self,
        purposes: crate::extensions::ext_tooling_info::ToolPurposeFlagsEXT,
    ) -> Self {
        self.0.purposes = purposes;
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
    #[allow(unused_mut)]
    #[inline]
    pub fn layer(
        mut self,
        layer: [std::os::raw::c_char; crate::vk1_0::MAX_EXTENSION_NAME_SIZE as usize],
    ) -> Self {
        self.0.layer = layer;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceToolPropertiesEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceToolPropertiesEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkToolPurposeFlagBitsEXT.html) · Flag Bits of [`ToolPurposeFlagsEXT`](struct.ToolPurposeFlagsEXT.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ToolPurposeFlagBitsEXT(pub u32);
impl ToolPurposeFlagBitsEXT {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> ToolPurposeFlagsEXT {
        ToolPurposeFlagsEXT::from_bits_truncate(self.0)
    }
}
#[doc = "[Part of `extensions::ext_tooling_info`](../../extensions/ext_tooling_info/index.html)"]
impl ToolPurposeFlagBitsEXT {
    pub const VALIDATION_EXT: Self = Self(0x00000001);
    pub const PROFILING_EXT: Self = Self(0x00000002);
    pub const TRACING_EXT: Self = Self(0x00000004);
    pub const ADDITIONAL_FEATURES_EXT: Self = Self(0x00000008);
    pub const MODIFYING_FEATURES_EXT: Self = Self(0x00000010);
}
#[doc = "[Part of `extensions::ext_tooling_info`](../../extensions/ext_tooling_info/index.html)"]
impl ToolPurposeFlagBitsEXT {
    pub const DEBUG_REPORTING_EXT: Self = Self(0x00000020);
    pub const DEBUG_MARKERS_EXT: Self = Self(0x00000040);
}
impl std::fmt::Debug for ToolPurposeFlagBitsEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::VALIDATION_EXT => "VALIDATION_EXT",
            &Self::PROFILING_EXT => "PROFILING_EXT",
            &Self::TRACING_EXT => "TRACING_EXT",
            &Self::ADDITIONAL_FEATURES_EXT => "ADDITIONAL_FEATURES_EXT",
            &Self::MODIFYING_FEATURES_EXT => "MODIFYING_FEATURES_EXT",
            &Self::DEBUG_REPORTING_EXT => "DEBUG_REPORTING_EXT",
            &Self::DEBUG_MARKERS_EXT => "DEBUG_MARKERS_EXT",
            _ => "(unknown)",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkToolPurposeFlagsEXT.html) · Flags of [`ToolPurposeFlagBitsEXT`](struct.ToolPurposeFlagBitsEXT.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct ToolPurposeFlagsEXT : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; const VALIDATION_EXT = ToolPurposeFlagBitsEXT :: VALIDATION_EXT . 0 ; const PROFILING_EXT = ToolPurposeFlagBitsEXT :: PROFILING_EXT . 0 ; const TRACING_EXT = ToolPurposeFlagBitsEXT :: TRACING_EXT . 0 ; const ADDITIONAL_FEATURES_EXT = ToolPurposeFlagBitsEXT :: ADDITIONAL_FEATURES_EXT . 0 ; const MODIFYING_FEATURES_EXT = ToolPurposeFlagBitsEXT :: MODIFYING_FEATURES_EXT . 0 ; const DEBUG_REPORTING_EXT = ToolPurposeFlagBitsEXT :: DEBUG_REPORTING_EXT . 0 ; const DEBUG_MARKERS_EXT = ToolPurposeFlagBitsEXT :: DEBUG_MARKERS_EXT . 0 ; } }
