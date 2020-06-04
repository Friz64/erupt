# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_AMD_display_native_hdr.html)\n\n## Extends\n- [`ColorSpaceKHR`](../../extensions/khr_surface/struct.ColorSpaceKHR.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const AMD_DISPLAY_NATIVE_HDR_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const AMD_DISPLAY_NATIVE_HDR_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_AMD_display_native_hdr");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSetLocalDimmingAMD.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkSetLocalDimmingAMD = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    swap_chain: crate::extensions::khr_swapchain::SwapchainKHR,
    local_dimming_enable: crate::vk1_0::Bool32,
) -> std::ffi::c_void;
#[doc = "Provides Device Commands for [`AmdDisplayNativeHdrDeviceLoaderExt`](trait.AmdDisplayNativeHdrDeviceLoaderExt.html)"]
pub struct AmdDisplayNativeHdrDeviceCommands {
    pub set_local_dimming_amd: Option<PFN_vkSetLocalDimmingAMD>,
}
impl AmdDisplayNativeHdrDeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<AmdDisplayNativeHdrDeviceCommands> {
        unsafe {
            let mut success = false;
            let table = AmdDisplayNativeHdrDeviceCommands {
                set_local_dimming_amd: std::mem::transmute({
                    let symbol = loader.symbol("vkSetLocalDimmingAMD");
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
fn device_commands(loader: &crate::DeviceLoader) -> &AmdDisplayNativeHdrDeviceCommands {
    loader
        .amd_display_native_hdr
        .as_ref()
        .expect("`amd_display_native_hdr` not loaded")
}
#[doc = "Provides high level command wrappers for [`AmdDisplayNativeHdrDeviceCommands`](struct.AmdDisplayNativeHdrDeviceCommands.html)"]
pub trait AmdDisplayNativeHdrDeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSetLocalDimmingAMD.html) · Device Command"]
    unsafe fn set_local_dimming_amd(
        &self,
        swap_chain: crate::extensions::khr_swapchain::SwapchainKHR,
        local_dimming_enable: bool,
    ) -> ();
}
impl AmdDisplayNativeHdrDeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSetLocalDimmingAMD.html) · Device Command"]
    unsafe fn set_local_dimming_amd(
        &self,
        swap_chain: crate::extensions::khr_swapchain::SwapchainKHR,
        local_dimming_enable: bool,
    ) -> () {
        let function = device_commands(self)
            .set_local_dimming_amd
            .as_ref()
            .expect("`set_local_dimming_amd` not available");
        let _val = function(self.handle, swap_chain, local_dimming_enable as _);
        ()
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayNativeHdrSurfaceCapabilitiesAMD.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DisplayNativeHdrSurfaceCapabilitiesAMD {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub local_dimming_support: crate::vk1_0::Bool32,
}
impl DisplayNativeHdrSurfaceCapabilitiesAMD {
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
    pub fn builder<'a>(self) -> DisplayNativeHdrSurfaceCapabilitiesAMDBuilder<'a> {
        DisplayNativeHdrSurfaceCapabilitiesAMDBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for DisplayNativeHdrSurfaceCapabilitiesAMD {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("DisplayNativeHdrSurfaceCapabilitiesAMD")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("local_dimming_support", &(self.local_dimming_support != 0))
            .finish()
    }
}
impl Default for DisplayNativeHdrSurfaceCapabilitiesAMD {
    fn default() -> DisplayNativeHdrSurfaceCapabilitiesAMD {
        DisplayNativeHdrSurfaceCapabilitiesAMD {
            s_type: crate::vk1_0::StructureType::DISPLAY_NATIVE_HDR_SURFACE_CAPABILITIES_AMD,
            p_next: std::ptr::null_mut(),
            local_dimming_support: Default::default(),
        }
    }
}
impl crate::ExtendableBy<DisplayNativeHdrSurfaceCapabilitiesAMD>
    for crate::extensions::khr_get_surface_capabilities2::SurfaceCapabilities2KHR
{
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayNativeHdrSurfaceCapabilitiesAMD.html) · Builder of [`DisplayNativeHdrSurfaceCapabilitiesAMD`](struct.DisplayNativeHdrSurfaceCapabilitiesAMD.html)"]
#[repr(transparent)]
pub struct DisplayNativeHdrSurfaceCapabilitiesAMDBuilder<'a>(
    DisplayNativeHdrSurfaceCapabilitiesAMD,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> DisplayNativeHdrSurfaceCapabilitiesAMDBuilder<'a> {
    #[inline]
    pub fn new() -> DisplayNativeHdrSurfaceCapabilitiesAMDBuilder<'a> {
        DisplayNativeHdrSurfaceCapabilitiesAMDBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn local_dimming_support(mut self, local_dimming_support: bool) -> Self {
        self.0.local_dimming_support = local_dimming_support as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> DisplayNativeHdrSurfaceCapabilitiesAMD {
        self.0
    }
}
impl<'a> std::fmt::Debug for DisplayNativeHdrSurfaceCapabilitiesAMDBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for DisplayNativeHdrSurfaceCapabilitiesAMDBuilder<'a> {
    type Target = DisplayNativeHdrSurfaceCapabilitiesAMD;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DisplayNativeHdrSurfaceCapabilitiesAMDBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSwapchainDisplayNativeHdrCreateInfoAMD.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SwapchainDisplayNativeHdrCreateInfoAMD {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub local_dimming_enable: crate::vk1_0::Bool32,
}
impl SwapchainDisplayNativeHdrCreateInfoAMD {
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
    pub fn builder<'a>(self) -> SwapchainDisplayNativeHdrCreateInfoAMDBuilder<'a> {
        SwapchainDisplayNativeHdrCreateInfoAMDBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for SwapchainDisplayNativeHdrCreateInfoAMD {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("SwapchainDisplayNativeHdrCreateInfoAMD")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("local_dimming_enable", &(self.local_dimming_enable != 0))
            .finish()
    }
}
impl Default for SwapchainDisplayNativeHdrCreateInfoAMD {
    fn default() -> SwapchainDisplayNativeHdrCreateInfoAMD {
        SwapchainDisplayNativeHdrCreateInfoAMD {
            s_type: crate::vk1_0::StructureType::SWAPCHAIN_DISPLAY_NATIVE_HDR_CREATE_INFO_AMD,
            p_next: std::ptr::null(),
            local_dimming_enable: Default::default(),
        }
    }
}
impl crate::ExtendableBy<SwapchainDisplayNativeHdrCreateInfoAMD>
    for crate::extensions::khr_swapchain::SwapchainCreateInfoKHR
{
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSwapchainDisplayNativeHdrCreateInfoAMD.html) · Builder of [`SwapchainDisplayNativeHdrCreateInfoAMD`](struct.SwapchainDisplayNativeHdrCreateInfoAMD.html)"]
#[repr(transparent)]
pub struct SwapchainDisplayNativeHdrCreateInfoAMDBuilder<'a>(
    SwapchainDisplayNativeHdrCreateInfoAMD,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> SwapchainDisplayNativeHdrCreateInfoAMDBuilder<'a> {
    #[inline]
    pub fn new() -> SwapchainDisplayNativeHdrCreateInfoAMDBuilder<'a> {
        SwapchainDisplayNativeHdrCreateInfoAMDBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn local_dimming_enable(mut self, local_dimming_enable: bool) -> Self {
        self.0.local_dimming_enable = local_dimming_enable as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> SwapchainDisplayNativeHdrCreateInfoAMD {
        self.0
    }
}
impl<'a> std::fmt::Debug for SwapchainDisplayNativeHdrCreateInfoAMDBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for SwapchainDisplayNativeHdrCreateInfoAMDBuilder<'a> {
    type Target = SwapchainDisplayNativeHdrCreateInfoAMD;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SwapchainDisplayNativeHdrCreateInfoAMDBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
