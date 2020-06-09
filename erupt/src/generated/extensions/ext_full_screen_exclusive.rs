# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_EXT_full_screen_exclusive.html)\n\n## Extends\n- [`Result`](../../vk1_0/struct.Result.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_FULL_SCREEN_EXCLUSIVE_SPEC_VERSION: u32 = 4;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_FULL_SCREEN_EXCLUSIVE_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_full_screen_exclusive");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfacePresentModes2EXT.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT = unsafe extern "system" fn ( physical_device : crate :: vk1_0 :: PhysicalDevice , p_surface_info : * const crate :: extensions :: khr_get_surface_capabilities2 :: PhysicalDeviceSurfaceInfo2KHR , p_present_mode_count : * mut u32 , p_present_modes : * mut crate :: extensions :: khr_surface :: PresentModeKHR , ) -> crate :: vk1_0 :: Result ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAcquireFullScreenExclusiveModeEXT.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkAcquireFullScreenExclusiveModeEXT =
    unsafe extern "system" fn(
        device: crate::vk1_0::Device,
        swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
    ) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkReleaseFullScreenExclusiveModeEXT.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkReleaseFullScreenExclusiveModeEXT =
    unsafe extern "system" fn(
        device: crate::vk1_0::Device,
        swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
    ) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceGroupSurfacePresentModes2EXT.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceGroupSurfacePresentModes2EXT = unsafe extern "system" fn ( device : crate :: vk1_0 :: Device , p_surface_info : * const crate :: extensions :: khr_get_surface_capabilities2 :: PhysicalDeviceSurfaceInfo2KHR , p_modes : * mut crate :: extensions :: khr_swapchain :: DeviceGroupPresentModeFlagsKHR , ) -> crate :: vk1_0 :: Result ;
#[doc = "Provides Instance Commands for [`ExtFullScreenExclusiveInstanceLoaderExt`](trait.ExtFullScreenExclusiveInstanceLoaderExt.html)"]
pub struct ExtFullScreenExclusiveInstanceCommands {
    pub get_physical_device_surface_present_modes2_ext:
        Option<PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT>,
}
impl ExtFullScreenExclusiveInstanceCommands {
    #[inline]
    pub fn load(loader: &crate::InstanceLoader) -> Option<ExtFullScreenExclusiveInstanceCommands> {
        unsafe {
            let mut success = false;
            let table = ExtFullScreenExclusiveInstanceCommands {
                get_physical_device_surface_present_modes2_ext: std::mem::transmute({
                    let symbol = loader.symbol("vkGetPhysicalDeviceSurfacePresentModes2EXT");
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
#[inline]
fn instance_commands(loader: &crate::InstanceLoader) -> &ExtFullScreenExclusiveInstanceCommands {
    loader
        .ext_full_screen_exclusive
        .as_ref()
        .expect("`ext_full_screen_exclusive` not loaded")
}
#[doc = "Provides high level command wrappers for [`ExtFullScreenExclusiveInstanceCommands`](struct.ExtFullScreenExclusiveInstanceCommands.html)"]
pub trait ExtFullScreenExclusiveInstanceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfacePresentModes2EXT.html) · Instance Command"]
    unsafe fn get_physical_device_surface_present_modes2_ext(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        surface_info : & crate :: extensions :: khr_get_surface_capabilities2 :: PhysicalDeviceSurfaceInfo2KHR,
        present_mode_count: Option<u32>,
    ) -> crate::utils::VulkanResult<Vec<crate::extensions::khr_surface::PresentModeKHR>>;
}
impl ExtFullScreenExclusiveInstanceLoaderExt for crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfacePresentModes2EXT.html) · Instance Command"]
    unsafe fn get_physical_device_surface_present_modes2_ext(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        surface_info : & crate :: extensions :: khr_get_surface_capabilities2 :: PhysicalDeviceSurfaceInfo2KHR,
        present_mode_count: Option<u32>,
    ) -> crate::utils::VulkanResult<Vec<crate::extensions::khr_surface::PresentModeKHR>> {
        let function = instance_commands(self)
            .get_physical_device_surface_present_modes2_ext
            .as_ref()
            .expect("`get_physical_device_surface_present_modes2_ext` not available");
        let mut present_mode_count = present_mode_count.unwrap_or_else(|| {
            let mut val = Default::default();
            function(
                physical_device,
                surface_info,
                &mut val,
                std::ptr::null_mut(),
            );
            val
        });
        let mut present_modes = vec![Default::default(); present_mode_count as _];
        let _val = function(
            physical_device,
            surface_info,
            &mut present_mode_count,
            present_modes.as_mut_ptr(),
        );
        crate::utils::VulkanResult::new(_val, present_modes)
    }
}
#[doc = "Provides Device Commands for [`ExtFullScreenExclusiveDeviceLoaderExt`](trait.ExtFullScreenExclusiveDeviceLoaderExt.html)"]
pub struct ExtFullScreenExclusiveDeviceCommands {
    pub acquire_full_screen_exclusive_mode_ext: Option<PFN_vkAcquireFullScreenExclusiveModeEXT>,
    pub release_full_screen_exclusive_mode_ext: Option<PFN_vkReleaseFullScreenExclusiveModeEXT>,
    pub get_device_group_surface_present_modes2_ext:
        Option<PFN_vkGetDeviceGroupSurfacePresentModes2EXT>,
}
impl ExtFullScreenExclusiveDeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<ExtFullScreenExclusiveDeviceCommands> {
        unsafe {
            let mut success = false;
            let table = ExtFullScreenExclusiveDeviceCommands {
                acquire_full_screen_exclusive_mode_ext: std::mem::transmute({
                    let symbol = loader.symbol("vkAcquireFullScreenExclusiveModeEXT");
                    success |= symbol.is_some();
                    symbol
                }),
                release_full_screen_exclusive_mode_ext: std::mem::transmute({
                    let symbol = loader.symbol("vkReleaseFullScreenExclusiveModeEXT");
                    success |= symbol.is_some();
                    symbol
                }),
                get_device_group_surface_present_modes2_ext: std::mem::transmute({
                    let symbol = loader.symbol("vkGetDeviceGroupSurfacePresentModes2EXT");
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
#[inline]
fn device_commands(loader: &crate::DeviceLoader) -> &ExtFullScreenExclusiveDeviceCommands {
    loader
        .ext_full_screen_exclusive
        .as_ref()
        .expect("`ext_full_screen_exclusive` not loaded")
}
#[doc = "Provides high level command wrappers for [`ExtFullScreenExclusiveDeviceCommands`](struct.ExtFullScreenExclusiveDeviceCommands.html)"]
pub trait ExtFullScreenExclusiveDeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAcquireFullScreenExclusiveModeEXT.html) · Device Command"]
    unsafe fn acquire_full_screen_exclusive_mode_ext(
        &self,
        swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
    ) -> crate::utils::VulkanResult<()>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkReleaseFullScreenExclusiveModeEXT.html) · Device Command"]
    unsafe fn release_full_screen_exclusive_mode_ext(
        &self,
        swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
    ) -> crate::utils::VulkanResult<()>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceGroupSurfacePresentModes2EXT.html) · Device Command"]
    unsafe fn get_device_group_surface_present_modes2_ext(
        &self,
        surface_info : & crate :: extensions :: khr_get_surface_capabilities2 :: PhysicalDeviceSurfaceInfo2KHR,
        modes: Option<crate::extensions::khr_swapchain::DeviceGroupPresentModeFlagsKHR>,
    ) -> crate::utils::VulkanResult<crate::extensions::khr_swapchain::DeviceGroupPresentModeFlagsKHR>;
}
impl ExtFullScreenExclusiveDeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAcquireFullScreenExclusiveModeEXT.html) · Device Command"]
    unsafe fn acquire_full_screen_exclusive_mode_ext(
        &self,
        swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
    ) -> crate::utils::VulkanResult<()> {
        let function = device_commands(self)
            .acquire_full_screen_exclusive_mode_ext
            .as_ref()
            .expect("`acquire_full_screen_exclusive_mode_ext` not available");
        let _val = function(self.handle, swapchain);
        crate::utils::VulkanResult::new(_val, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkReleaseFullScreenExclusiveModeEXT.html) · Device Command"]
    unsafe fn release_full_screen_exclusive_mode_ext(
        &self,
        swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
    ) -> crate::utils::VulkanResult<()> {
        let function = device_commands(self)
            .release_full_screen_exclusive_mode_ext
            .as_ref()
            .expect("`release_full_screen_exclusive_mode_ext` not available");
        let _val = function(self.handle, swapchain);
        crate::utils::VulkanResult::new(_val, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceGroupSurfacePresentModes2EXT.html) · Device Command"]
    unsafe fn get_device_group_surface_present_modes2_ext(
        &self,
        surface_info : & crate :: extensions :: khr_get_surface_capabilities2 :: PhysicalDeviceSurfaceInfo2KHR,
        modes: Option<crate::extensions::khr_swapchain::DeviceGroupPresentModeFlagsKHR>,
    ) -> crate::utils::VulkanResult<crate::extensions::khr_swapchain::DeviceGroupPresentModeFlagsKHR>
    {
        let function = device_commands(self)
            .get_device_group_surface_present_modes2_ext
            .as_ref()
            .expect("`get_device_group_surface_present_modes2_ext` not available");
        let mut modes = modes.unwrap_or_else(|| Default::default());
        let _val = function(self.handle, surface_info, &mut modes);
        crate::utils::VulkanResult::new(_val, modes)
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFullScreenExclusiveEXT.html) · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct FullScreenExclusiveEXT(pub i32);
#[doc = "[Part of `extensions::ext_full_screen_exclusive`](../../extensions/ext_full_screen_exclusive/index.html)"]
impl FullScreenExclusiveEXT {
    pub const DEFAULT_EXT: Self = Self(0);
    pub const ALLOWED_EXT: Self = Self(1);
    pub const DISALLOWED_EXT: Self = Self(2);
    pub const APPLICATION_CONTROLLED_EXT: Self = Self(3);
}
impl std::fmt::Debug for FullScreenExclusiveEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::DEFAULT_EXT => "DEFAULT_EXT",
            &Self::ALLOWED_EXT => "ALLOWED_EXT",
            &Self::DISALLOWED_EXT => "DISALLOWED_EXT",
            &Self::APPLICATION_CONTROLLED_EXT => "APPLICATION_CONTROLLED_EXT",
            _ => "(unknown)",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceFullScreenExclusiveInfoEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SurfaceFullScreenExclusiveInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub full_screen_exclusive: crate::extensions::ext_full_screen_exclusive::FullScreenExclusiveEXT,
}
impl SurfaceFullScreenExclusiveInfoEXT {
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
    pub fn builder<'a>(self) -> SurfaceFullScreenExclusiveInfoEXTBuilder<'a> {
        SurfaceFullScreenExclusiveInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for SurfaceFullScreenExclusiveInfoEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("SurfaceFullScreenExclusiveInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("full_screen_exclusive", &self.full_screen_exclusive)
            .finish()
    }
}
impl Default for SurfaceFullScreenExclusiveInfoEXT {
    fn default() -> SurfaceFullScreenExclusiveInfoEXT {
        SurfaceFullScreenExclusiveInfoEXT {
            s_type: crate::vk1_0::StructureType::SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT,
            p_next: std::ptr::null_mut(),
            full_screen_exclusive: Default::default(),
        }
    }
}
impl crate::ExtendableBy<SurfaceFullScreenExclusiveInfoEXT>
    for crate::extensions::khr_get_surface_capabilities2::PhysicalDeviceSurfaceInfo2KHR
{
}
impl crate::ExtendableBy<SurfaceFullScreenExclusiveInfoEXT>
    for crate::extensions::khr_swapchain::SwapchainCreateInfoKHR
{
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceFullScreenExclusiveInfoEXT.html) · Builder of [`SurfaceFullScreenExclusiveInfoEXT`](struct.SurfaceFullScreenExclusiveInfoEXT.html)"]
#[repr(transparent)]
pub struct SurfaceFullScreenExclusiveInfoEXTBuilder<'a>(
    SurfaceFullScreenExclusiveInfoEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> SurfaceFullScreenExclusiveInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> SurfaceFullScreenExclusiveInfoEXTBuilder<'a> {
        SurfaceFullScreenExclusiveInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn full_screen_exclusive(
        mut self,
        full_screen_exclusive: crate::extensions::ext_full_screen_exclusive::FullScreenExclusiveEXT,
    ) -> Self {
        self.0.full_screen_exclusive = full_screen_exclusive;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> SurfaceFullScreenExclusiveInfoEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for SurfaceFullScreenExclusiveInfoEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceCapabilitiesFullScreenExclusiveEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SurfaceCapabilitiesFullScreenExclusiveEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub full_screen_exclusive_supported: crate::vk1_0::Bool32,
}
impl SurfaceCapabilitiesFullScreenExclusiveEXT {
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
    pub fn builder<'a>(self) -> SurfaceCapabilitiesFullScreenExclusiveEXTBuilder<'a> {
        SurfaceCapabilitiesFullScreenExclusiveEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for SurfaceCapabilitiesFullScreenExclusiveEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("SurfaceCapabilitiesFullScreenExclusiveEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "full_screen_exclusive_supported",
                &(self.full_screen_exclusive_supported != 0),
            )
            .finish()
    }
}
impl Default for SurfaceCapabilitiesFullScreenExclusiveEXT {
    fn default() -> SurfaceCapabilitiesFullScreenExclusiveEXT {
        SurfaceCapabilitiesFullScreenExclusiveEXT {
            s_type: crate::vk1_0::StructureType::SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT,
            p_next: std::ptr::null_mut(),
            full_screen_exclusive_supported: Default::default(),
        }
    }
}
impl crate::ExtendableBy<SurfaceCapabilitiesFullScreenExclusiveEXT>
    for crate::extensions::khr_get_surface_capabilities2::SurfaceCapabilities2KHR
{
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceCapabilitiesFullScreenExclusiveEXT.html) · Builder of [`SurfaceCapabilitiesFullScreenExclusiveEXT`](struct.SurfaceCapabilitiesFullScreenExclusiveEXT.html)"]
#[repr(transparent)]
pub struct SurfaceCapabilitiesFullScreenExclusiveEXTBuilder<'a>(
    SurfaceCapabilitiesFullScreenExclusiveEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> SurfaceCapabilitiesFullScreenExclusiveEXTBuilder<'a> {
    #[inline]
    pub fn new() -> SurfaceCapabilitiesFullScreenExclusiveEXTBuilder<'a> {
        SurfaceCapabilitiesFullScreenExclusiveEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn full_screen_exclusive_supported(
        mut self,
        full_screen_exclusive_supported: bool,
    ) -> Self {
        self.0.full_screen_exclusive_supported = full_screen_exclusive_supported as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> SurfaceCapabilitiesFullScreenExclusiveEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for SurfaceCapabilitiesFullScreenExclusiveEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceFullScreenExclusiveWin32InfoEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SurfaceFullScreenExclusiveWin32InfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub hmonitor: *mut std::ffi::c_void,
}
impl SurfaceFullScreenExclusiveWin32InfoEXT {
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
    pub fn builder<'a>(self) -> SurfaceFullScreenExclusiveWin32InfoEXTBuilder<'a> {
        SurfaceFullScreenExclusiveWin32InfoEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for SurfaceFullScreenExclusiveWin32InfoEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("SurfaceFullScreenExclusiveWin32InfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("hmonitor", &self.hmonitor)
            .finish()
    }
}
impl Default for SurfaceFullScreenExclusiveWin32InfoEXT {
    fn default() -> SurfaceFullScreenExclusiveWin32InfoEXT {
        SurfaceFullScreenExclusiveWin32InfoEXT {
            s_type: crate::vk1_0::StructureType::SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT,
            p_next: std::ptr::null(),
            hmonitor: std::ptr::null_mut(),
        }
    }
}
impl crate::ExtendableBy<SurfaceFullScreenExclusiveWin32InfoEXT>
    for crate::extensions::khr_get_surface_capabilities2::PhysicalDeviceSurfaceInfo2KHR
{
}
impl crate::ExtendableBy<SurfaceFullScreenExclusiveWin32InfoEXT>
    for crate::extensions::khr_swapchain::SwapchainCreateInfoKHR
{
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceFullScreenExclusiveWin32InfoEXT.html) · Builder of [`SurfaceFullScreenExclusiveWin32InfoEXT`](struct.SurfaceFullScreenExclusiveWin32InfoEXT.html)"]
#[repr(transparent)]
pub struct SurfaceFullScreenExclusiveWin32InfoEXTBuilder<'a>(
    SurfaceFullScreenExclusiveWin32InfoEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> SurfaceFullScreenExclusiveWin32InfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> SurfaceFullScreenExclusiveWin32InfoEXTBuilder<'a> {
        SurfaceFullScreenExclusiveWin32InfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn hmonitor(mut self, hmonitor: &'a mut std::ffi::c_void) -> Self {
        self.0.hmonitor = hmonitor;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> SurfaceFullScreenExclusiveWin32InfoEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for SurfaceFullScreenExclusiveWin32InfoEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
