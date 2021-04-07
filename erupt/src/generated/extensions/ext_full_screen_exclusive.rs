#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_FULL_SCREEN_EXCLUSIVE_SPEC_VERSION")]
pub const EXT_FULL_SCREEN_EXCLUSIVE_SPEC_VERSION: u32 = 4;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_FULL_SCREEN_EXCLUSIVE_EXTENSION_NAME")]
pub const EXT_FULL_SCREEN_EXCLUSIVE_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_full_screen_exclusive");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_PHYSICAL_DEVICE_SURFACE_PRESENT_MODES2_EXT: *const std::os::raw::c_char = crate::cstr!("vkGetPhysicalDeviceSurfacePresentModes2EXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_DEVICE_GROUP_SURFACE_PRESENT_MODES2_EXT: *const std::os::raw::c_char = crate::cstr!("vkGetDeviceGroupSurfacePresentModes2EXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_ACQUIRE_FULL_SCREEN_EXCLUSIVE_MODE_EXT: *const std::os::raw::c_char = crate::cstr!("vkAcquireFullScreenExclusiveModeEXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_RELEASE_FULL_SCREEN_EXCLUSIVE_MODE_EXT: *const std::os::raw::c_char = crate::cstr!("vkReleaseFullScreenExclusiveModeEXT");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFullScreenExclusiveEXT.html) · Enum"]
#[doc(alias = "VkFullScreenExclusiveEXT")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct FullScreenExclusiveEXT(pub i32);
impl std::fmt::Debug for FullScreenExclusiveEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::DEFAULT_EXT => "DEFAULT_EXT",
            &Self::ALLOWED_EXT => "ALLOWED_EXT",
            &Self::DISALLOWED_EXT => "DISALLOWED_EXT",
            &Self::APPLICATION_CONTROLLED_EXT => "APPLICATION_CONTROLLED_EXT",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::ext_full_screen_exclusive`]"]
impl FullScreenExclusiveEXT {
    pub const DEFAULT_EXT: Self = Self(0);
    pub const ALLOWED_EXT: Self = Self(1);
    pub const DISALLOWED_EXT: Self = Self(2);
    pub const APPLICATION_CONTROLLED_EXT: Self = Self(3);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfacePresentModes2EXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT = unsafe extern "system" fn(
    physical_device: crate::vk1_0::PhysicalDevice,
    p_surface_info: *const crate::extensions::khr_get_surface_capabilities2::PhysicalDeviceSurfaceInfo2KHR,
    p_present_mode_count: *mut u32,
    p_present_modes: *mut crate::extensions::khr_surface::PresentModeKHR,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceGroupSurfacePresentModes2EXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceGroupSurfacePresentModes2EXT = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_surface_info: *const crate::extensions::khr_get_surface_capabilities2::PhysicalDeviceSurfaceInfo2KHR,
    p_modes: *mut crate::extensions::khr_swapchain::DeviceGroupPresentModeFlagsKHR,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAcquireFullScreenExclusiveModeEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkAcquireFullScreenExclusiveModeEXT = unsafe extern "system" fn(device: crate::vk1_0::Device, swapchain: crate::extensions::khr_swapchain::SwapchainKHR) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkReleaseFullScreenExclusiveModeEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkReleaseFullScreenExclusiveModeEXT = unsafe extern "system" fn(device: crate::vk1_0::Device, swapchain: crate::extensions::khr_swapchain::SwapchainKHR) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceFullScreenExclusiveInfoEXT.html) · Structure"]
#[doc(alias = "VkSurfaceFullScreenExclusiveInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SurfaceFullScreenExclusiveInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub full_screen_exclusive: crate::extensions::ext_full_screen_exclusive::FullScreenExclusiveEXT,
}
impl Default for SurfaceFullScreenExclusiveInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT,
            p_next: std::ptr::null_mut(),
            full_screen_exclusive: Default::default(),
        }
    }
}
impl std::fmt::Debug for SurfaceFullScreenExclusiveInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SurfaceFullScreenExclusiveInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("full_screen_exclusive", &self.full_screen_exclusive)
            .finish()
    }
}
impl SurfaceFullScreenExclusiveInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> SurfaceFullScreenExclusiveInfoEXTBuilder<'a> {
        SurfaceFullScreenExclusiveInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceFullScreenExclusiveInfoEXT.html) · Builder of [`SurfaceFullScreenExclusiveInfoEXT`]"]
#[repr(transparent)]
pub struct SurfaceFullScreenExclusiveInfoEXTBuilder<'a>(SurfaceFullScreenExclusiveInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> SurfaceFullScreenExclusiveInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> SurfaceFullScreenExclusiveInfoEXTBuilder<'a> {
        SurfaceFullScreenExclusiveInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn full_screen_exclusive(mut self, full_screen_exclusive: crate::extensions::ext_full_screen_exclusive::FullScreenExclusiveEXT) -> Self {
        self.0.full_screen_exclusive = full_screen_exclusive as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> SurfaceFullScreenExclusiveInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for SurfaceFullScreenExclusiveInfoEXTBuilder<'a> {
    fn default() -> SurfaceFullScreenExclusiveInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for SurfaceFullScreenExclusiveInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for SurfaceFullScreenExclusiveInfoEXTBuilder<'a> {
    type Target = SurfaceFullScreenExclusiveInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SurfaceFullScreenExclusiveInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceFullScreenExclusiveWin32InfoEXT.html) · Structure"]
#[doc(alias = "VkSurfaceFullScreenExclusiveWin32InfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SurfaceFullScreenExclusiveWin32InfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub hmonitor: *mut std::ffi::c_void,
}
impl Default for SurfaceFullScreenExclusiveWin32InfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT,
            p_next: std::ptr::null(),
            hmonitor: std::ptr::null_mut(),
        }
    }
}
impl std::fmt::Debug for SurfaceFullScreenExclusiveWin32InfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SurfaceFullScreenExclusiveWin32InfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("hmonitor", &self.hmonitor)
            .finish()
    }
}
impl SurfaceFullScreenExclusiveWin32InfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> SurfaceFullScreenExclusiveWin32InfoEXTBuilder<'a> {
        SurfaceFullScreenExclusiveWin32InfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceFullScreenExclusiveWin32InfoEXT.html) · Builder of [`SurfaceFullScreenExclusiveWin32InfoEXT`]"]
#[repr(transparent)]
pub struct SurfaceFullScreenExclusiveWin32InfoEXTBuilder<'a>(SurfaceFullScreenExclusiveWin32InfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> SurfaceFullScreenExclusiveWin32InfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> SurfaceFullScreenExclusiveWin32InfoEXTBuilder<'a> {
        SurfaceFullScreenExclusiveWin32InfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn hmonitor(mut self, hmonitor: *mut std::ffi::c_void) -> Self {
        self.0.hmonitor = hmonitor;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> SurfaceFullScreenExclusiveWin32InfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for SurfaceFullScreenExclusiveWin32InfoEXTBuilder<'a> {
    fn default() -> SurfaceFullScreenExclusiveWin32InfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for SurfaceFullScreenExclusiveWin32InfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for SurfaceFullScreenExclusiveWin32InfoEXTBuilder<'a> {
    type Target = SurfaceFullScreenExclusiveWin32InfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SurfaceFullScreenExclusiveWin32InfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceCapabilitiesFullScreenExclusiveEXT.html) · Structure"]
#[doc(alias = "VkSurfaceCapabilitiesFullScreenExclusiveEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SurfaceCapabilitiesFullScreenExclusiveEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub full_screen_exclusive_supported: crate::vk1_0::Bool32,
}
impl Default for SurfaceCapabilitiesFullScreenExclusiveEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT,
            p_next: std::ptr::null_mut(),
            full_screen_exclusive_supported: Default::default(),
        }
    }
}
impl std::fmt::Debug for SurfaceCapabilitiesFullScreenExclusiveEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SurfaceCapabilitiesFullScreenExclusiveEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("full_screen_exclusive_supported", &(self.full_screen_exclusive_supported != 0))
            .finish()
    }
}
impl SurfaceCapabilitiesFullScreenExclusiveEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> SurfaceCapabilitiesFullScreenExclusiveEXTBuilder<'a> {
        SurfaceCapabilitiesFullScreenExclusiveEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceCapabilitiesFullScreenExclusiveEXT.html) · Builder of [`SurfaceCapabilitiesFullScreenExclusiveEXT`]"]
#[repr(transparent)]
pub struct SurfaceCapabilitiesFullScreenExclusiveEXTBuilder<'a>(SurfaceCapabilitiesFullScreenExclusiveEXT, std::marker::PhantomData<&'a ()>);
impl<'a> SurfaceCapabilitiesFullScreenExclusiveEXTBuilder<'a> {
    #[inline]
    pub fn new() -> SurfaceCapabilitiesFullScreenExclusiveEXTBuilder<'a> {
        SurfaceCapabilitiesFullScreenExclusiveEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn full_screen_exclusive_supported(mut self, full_screen_exclusive_supported: bool) -> Self {
        self.0.full_screen_exclusive_supported = full_screen_exclusive_supported as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> SurfaceCapabilitiesFullScreenExclusiveEXT {
        self.0
    }
}
impl<'a> std::default::Default for SurfaceCapabilitiesFullScreenExclusiveEXTBuilder<'a> {
    fn default() -> SurfaceCapabilitiesFullScreenExclusiveEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for SurfaceCapabilitiesFullScreenExclusiveEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for SurfaceCapabilitiesFullScreenExclusiveEXTBuilder<'a> {
    type Target = SurfaceCapabilitiesFullScreenExclusiveEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SurfaceCapabilitiesFullScreenExclusiveEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::ext_full_screen_exclusive`]"]
impl crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfacePresentModes2EXT.html) · Function"]
    #[doc(alias = "vkGetPhysicalDeviceSurfacePresentModes2EXT")]
    pub unsafe fn get_physical_device_surface_present_modes2_ext(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        surface_info: &crate::extensions::khr_get_surface_capabilities2::PhysicalDeviceSurfaceInfo2KHR,
        present_mode_count: Option<u32>,
    ) -> crate::utils::VulkanResult<Vec<crate::extensions::khr_surface::PresentModeKHR>> {
        let _function = self.get_physical_device_surface_present_modes2_ext.expect("`get_physical_device_surface_present_modes2_ext` is not loaded");
        let mut present_mode_count = match present_mode_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                _function(physical_device as _, surface_info as _, &mut v, std::ptr::null_mut());
                v
            }
        };
        let mut present_modes = vec![Default::default(); present_mode_count as _];
        let _return = _function(physical_device as _, surface_info as _, &mut present_mode_count, present_modes.as_mut_ptr());
        crate::utils::VulkanResult::new(_return, present_modes)
    }
}
#[doc = "Provided by [`crate::extensions::ext_full_screen_exclusive`]"]
impl crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceGroupSurfacePresentModes2EXT.html) · Function"]
    #[doc(alias = "vkGetDeviceGroupSurfacePresentModes2EXT")]
    pub unsafe fn get_device_group_surface_present_modes2_ext(
        &self,
        surface_info: &crate::extensions::khr_get_surface_capabilities2::PhysicalDeviceSurfaceInfo2KHR,
        modes: &mut crate::extensions::khr_swapchain::DeviceGroupPresentModeFlagsKHR,
    ) -> crate::utils::VulkanResult<()> {
        let _function = self.get_device_group_surface_present_modes2_ext.expect("`get_device_group_surface_present_modes2_ext` is not loaded");
        let _return = _function(self.handle, surface_info as _, modes as _);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAcquireFullScreenExclusiveModeEXT.html) · Function"]
    #[doc(alias = "vkAcquireFullScreenExclusiveModeEXT")]
    pub unsafe fn acquire_full_screen_exclusive_mode_ext(&self, swapchain: crate::extensions::khr_swapchain::SwapchainKHR) -> crate::utils::VulkanResult<()> {
        let _function = self.acquire_full_screen_exclusive_mode_ext.expect("`acquire_full_screen_exclusive_mode_ext` is not loaded");
        let _return = _function(self.handle, swapchain as _);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkReleaseFullScreenExclusiveModeEXT.html) · Function"]
    #[doc(alias = "vkReleaseFullScreenExclusiveModeEXT")]
    pub unsafe fn release_full_screen_exclusive_mode_ext(&self, swapchain: crate::extensions::khr_swapchain::SwapchainKHR) -> crate::utils::VulkanResult<()> {
        let _function = self.release_full_screen_exclusive_mode_ext.expect("`release_full_screen_exclusive_mode_ext` is not loaded");
        let _return = _function(self.handle, swapchain as _);
        crate::utils::VulkanResult::new(_return, ())
    }
}
