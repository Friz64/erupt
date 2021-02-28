#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_DEVICE_DIAGNOSTICS_CONFIG_SPEC_VERSION")]
pub const NV_DEVICE_DIAGNOSTICS_CONFIG_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_DEVICE_DIAGNOSTICS_CONFIG_EXTENSION_NAME")]
pub const NV_DEVICE_DIAGNOSTICS_CONFIG_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_NV_device_diagnostics_config");
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceDiagnosticsConfigFlagsNV.html) · Bitmask of [`DeviceDiagnosticsConfigFlagBitsNV`]"] # [doc (alias = "VkDeviceDiagnosticsConfigFlagsNV")] # [derive (Default)] # [repr (transparent)] pub struct DeviceDiagnosticsConfigFlagsNV : u32 { const ENABLE_SHADER_DEBUG_INFO_NV = DeviceDiagnosticsConfigFlagBitsNV :: ENABLE_SHADER_DEBUG_INFO_NV . 0 ; const ENABLE_RESOURCE_TRACKING_NV = DeviceDiagnosticsConfigFlagBitsNV :: ENABLE_RESOURCE_TRACKING_NV . 0 ; const ENABLE_AUTOMATIC_CHECKPOINTS_NV = DeviceDiagnosticsConfigFlagBitsNV :: ENABLE_AUTOMATIC_CHECKPOINTS_NV . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceDiagnosticsConfigFlagBitsNV.html) · Bits enum of [`DeviceDiagnosticsConfigFlagsNV`]"]
#[doc(alias = "VkDeviceDiagnosticsConfigFlagBitsNV")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct DeviceDiagnosticsConfigFlagBitsNV(pub u32);
impl DeviceDiagnosticsConfigFlagBitsNV {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> DeviceDiagnosticsConfigFlagsNV {
        DeviceDiagnosticsConfigFlagsNV::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for DeviceDiagnosticsConfigFlagBitsNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::ENABLE_SHADER_DEBUG_INFO_NV => "ENABLE_SHADER_DEBUG_INFO_NV",
            &Self::ENABLE_RESOURCE_TRACKING_NV => "ENABLE_RESOURCE_TRACKING_NV",
            &Self::ENABLE_AUTOMATIC_CHECKPOINTS_NV => "ENABLE_AUTOMATIC_CHECKPOINTS_NV",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::nv_device_diagnostics_config`]"]
impl DeviceDiagnosticsConfigFlagBitsNV {
    pub const ENABLE_SHADER_DEBUG_INFO_NV: Self = Self(1);
    pub const ENABLE_RESOURCE_TRACKING_NV: Self = Self(2);
    pub const ENABLE_AUTOMATIC_CHECKPOINTS_NV: Self = Self(4);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDiagnosticsConfigFeaturesNV.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceDiagnosticsConfigFeaturesNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceDiagnosticsConfigFeaturesNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub diagnostics_config: crate::vk1_0::Bool32,
}
impl Default for PhysicalDeviceDiagnosticsConfigFeaturesNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_DIAGNOSTICS_CONFIG_FEATURES_NV,
            p_next: std::ptr::null_mut(),
            diagnostics_config: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceDiagnosticsConfigFeaturesNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceDiagnosticsConfigFeaturesNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("diagnostics_config", &(self.diagnostics_config != 0))
            .finish()
    }
}
impl PhysicalDeviceDiagnosticsConfigFeaturesNV {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceDiagnosticsConfigFeaturesNVBuilder<'a> {
        PhysicalDeviceDiagnosticsConfigFeaturesNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDiagnosticsConfigFeaturesNV.html) · Builder of [`PhysicalDeviceDiagnosticsConfigFeaturesNV`]"]
#[repr(transparent)]
pub struct PhysicalDeviceDiagnosticsConfigFeaturesNVBuilder<'a>(PhysicalDeviceDiagnosticsConfigFeaturesNV, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceDiagnosticsConfigFeaturesNVBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceDiagnosticsConfigFeaturesNVBuilder<'a> {
        PhysicalDeviceDiagnosticsConfigFeaturesNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn diagnostics_config(mut self, diagnostics_config: bool) -> Self {
        self.0.diagnostics_config = diagnostics_config as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceDiagnosticsConfigFeaturesNV {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceDiagnosticsConfigFeaturesNVBuilder<'a> {
    fn default() -> PhysicalDeviceDiagnosticsConfigFeaturesNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceDiagnosticsConfigFeaturesNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceDiagnosticsConfigFeaturesNVBuilder<'a> {
    type Target = PhysicalDeviceDiagnosticsConfigFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceDiagnosticsConfigFeaturesNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceDiagnosticsConfigCreateInfoNV.html) · Structure"]
#[doc(alias = "VkDeviceDiagnosticsConfigCreateInfoNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DeviceDiagnosticsConfigCreateInfoNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::nv_device_diagnostics_config::DeviceDiagnosticsConfigFlagsNV,
}
impl Default for DeviceDiagnosticsConfigCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::DEVICE_DIAGNOSTICS_CONFIG_CREATE_INFO_NV,
            p_next: std::ptr::null(),
            flags: Default::default(),
        }
    }
}
impl std::fmt::Debug for DeviceDiagnosticsConfigCreateInfoNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DeviceDiagnosticsConfigCreateInfoNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .finish()
    }
}
impl DeviceDiagnosticsConfigCreateInfoNV {
    #[inline]
    pub fn into_builder<'a>(self) -> DeviceDiagnosticsConfigCreateInfoNVBuilder<'a> {
        DeviceDiagnosticsConfigCreateInfoNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceDiagnosticsConfigCreateInfoNV.html) · Builder of [`DeviceDiagnosticsConfigCreateInfoNV`]"]
#[repr(transparent)]
pub struct DeviceDiagnosticsConfigCreateInfoNVBuilder<'a>(DeviceDiagnosticsConfigCreateInfoNV, std::marker::PhantomData<&'a ()>);
impl<'a> DeviceDiagnosticsConfigCreateInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> DeviceDiagnosticsConfigCreateInfoNVBuilder<'a> {
        DeviceDiagnosticsConfigCreateInfoNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::nv_device_diagnostics_config::DeviceDiagnosticsConfigFlagsNV) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DeviceDiagnosticsConfigCreateInfoNV {
        self.0
    }
}
impl<'a> std::default::Default for DeviceDiagnosticsConfigCreateInfoNVBuilder<'a> {
    fn default() -> DeviceDiagnosticsConfigCreateInfoNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DeviceDiagnosticsConfigCreateInfoNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DeviceDiagnosticsConfigCreateInfoNVBuilder<'a> {
    type Target = DeviceDiagnosticsConfigCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DeviceDiagnosticsConfigCreateInfoNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
