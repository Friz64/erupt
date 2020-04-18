# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_NV_device_diagnostics_config.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NV_DEVICE_DIAGNOSTICS_CONFIG_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NV_DEVICE_DIAGNOSTICS_CONFIG_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_NV_device_diagnostics_config");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDiagnosticsConfigFeaturesNV.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceDiagnosticsConfigFeaturesNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub diagnostics_config: crate::vk1_0::Bool32,
}
impl PhysicalDeviceDiagnosticsConfigFeaturesNV {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceDiagnosticsConfigFeaturesNV,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceDiagnosticsConfigFeaturesNVBuilder<'a> {
        PhysicalDeviceDiagnosticsConfigFeaturesNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceDiagnosticsConfigFeaturesNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceDiagnosticsConfigFeaturesNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("diagnostics_config", &(self.diagnostics_config != 0))
            .finish()
    }
}
impl Default for PhysicalDeviceDiagnosticsConfigFeaturesNV {
    fn default() -> PhysicalDeviceDiagnosticsConfigFeaturesNV {
        PhysicalDeviceDiagnosticsConfigFeaturesNV {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_DIAGNOSTICS_CONFIG_FEATURES_NV,
            p_next: std::ptr::null_mut(),
            diagnostics_config: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceDiagnosticsConfigFeaturesNV::extend`](struct.PhysicalDeviceDiagnosticsConfigFeaturesNV.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceDiagnosticsConfigFeaturesNV {}
impl ExtendableByPhysicalDeviceDiagnosticsConfigFeaturesNV
    for crate::vk1_1::PhysicalDeviceFeatures2
{
}
impl ExtendableByPhysicalDeviceDiagnosticsConfigFeaturesNV for crate::vk1_0::DeviceCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceDiagnosticsConfigFeaturesNV`](struct.PhysicalDeviceDiagnosticsConfigFeaturesNV.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceDiagnosticsConfigFeaturesNVBuilder<'a>(
    PhysicalDeviceDiagnosticsConfigFeaturesNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceDiagnosticsConfigFeaturesNVBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceDiagnosticsConfigFeaturesNVBuilder<'a> {
        PhysicalDeviceDiagnosticsConfigFeaturesNVBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn diagnostics_config(mut self, diagnostics_config: bool) -> Self {
        self.0.diagnostics_config = diagnostics_config as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceDiagnosticsConfigFeaturesNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceDiagnosticsConfigFeaturesNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DeviceDiagnosticsConfigCreateInfoNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::nv_device_diagnostics_config::DeviceDiagnosticsConfigFlagsNV,
}
impl DeviceDiagnosticsConfigCreateInfoNV {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByDeviceDiagnosticsConfigCreateInfoNV,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> DeviceDiagnosticsConfigCreateInfoNVBuilder<'a> {
        DeviceDiagnosticsConfigCreateInfoNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for DeviceDiagnosticsConfigCreateInfoNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("DeviceDiagnosticsConfigCreateInfoNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .finish()
    }
}
impl Default for DeviceDiagnosticsConfigCreateInfoNV {
    fn default() -> DeviceDiagnosticsConfigCreateInfoNV {
        DeviceDiagnosticsConfigCreateInfoNV {
            s_type: crate::vk1_0::StructureType::DEVICE_DIAGNOSTICS_CONFIG_CREATE_INFO_NV,
            p_next: std::ptr::null(),
            flags: Default::default(),
        }
    }
}
#[doc = "Used by [`DeviceDiagnosticsConfigCreateInfoNV::extend`](struct.DeviceDiagnosticsConfigCreateInfoNV.html#method.extend)"]
pub trait ExtendableByDeviceDiagnosticsConfigCreateInfoNV {}
impl ExtendableByDeviceDiagnosticsConfigCreateInfoNV for crate::vk1_0::DeviceCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`DeviceDiagnosticsConfigCreateInfoNV`](struct.DeviceDiagnosticsConfigCreateInfoNV.html)"]
#[repr(transparent)]
pub struct DeviceDiagnosticsConfigCreateInfoNVBuilder<'a>(
    DeviceDiagnosticsConfigCreateInfoNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> DeviceDiagnosticsConfigCreateInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> DeviceDiagnosticsConfigCreateInfoNVBuilder<'a> {
        DeviceDiagnosticsConfigCreateInfoNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn flags(
        mut self,
        flags: crate::extensions::nv_device_diagnostics_config::DeviceDiagnosticsConfigFlagsNV,
    ) -> Self {
        self.0.flags = flags;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> DeviceDiagnosticsConfigCreateInfoNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for DeviceDiagnosticsConfigCreateInfoNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceDiagnosticsConfigFlagBitsNV.html) · Flag Bits of [`DeviceDiagnosticsConfigFlagsNV`](struct.DeviceDiagnosticsConfigFlagsNV.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct DeviceDiagnosticsConfigFlagBitsNV(pub u32);
impl DeviceDiagnosticsConfigFlagBitsNV {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> DeviceDiagnosticsConfigFlagsNV {
        DeviceDiagnosticsConfigFlagsNV::from_bits_truncate(self.0)
    }
}
#[doc = "[Part of `extensions::nv_device_diagnostics_config`](../../extensions/nv_device_diagnostics_config/index.html)"]
impl DeviceDiagnosticsConfigFlagBitsNV {
    pub const ENABLE_SHADER_DEBUG_INFO_NV: Self = Self(0x00000001);
    pub const ENABLE_RESOURCE_TRACKING_NV: Self = Self(0x00000002);
    pub const ENABLE_AUTOMATIC_CHECKPOINTS_NV: Self = Self(0x00000004);
}
impl std::fmt::Debug for DeviceDiagnosticsConfigFlagBitsNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::ENABLE_SHADER_DEBUG_INFO_NV => "ENABLE_SHADER_DEBUG_INFO_NV",
            &Self::ENABLE_RESOURCE_TRACKING_NV => "ENABLE_RESOURCE_TRACKING_NV",
            &Self::ENABLE_AUTOMATIC_CHECKPOINTS_NV => "ENABLE_AUTOMATIC_CHECKPOINTS_NV",
            _ => "(unknown)",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceDiagnosticsConfigFlagsNV.html) · Flags of [`DeviceDiagnosticsConfigFlagBitsNV`](struct.DeviceDiagnosticsConfigFlagBitsNV.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct DeviceDiagnosticsConfigFlagsNV : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; const ENABLE_SHADER_DEBUG_INFO_NV = DeviceDiagnosticsConfigFlagBitsNV :: ENABLE_SHADER_DEBUG_INFO_NV . 0 ; const ENABLE_RESOURCE_TRACKING_NV = DeviceDiagnosticsConfigFlagBitsNV :: ENABLE_RESOURCE_TRACKING_NV . 0 ; const ENABLE_AUTOMATIC_CHECKPOINTS_NV = DeviceDiagnosticsConfigFlagBitsNV :: ENABLE_AUTOMATIC_CHECKPOINTS_NV . 0 ; } }
