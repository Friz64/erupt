#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_AMD_DISPLAY_NATIVE_HDR_SPEC_VERSION")]
pub const AMD_DISPLAY_NATIVE_HDR_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_AMD_DISPLAY_NATIVE_HDR_EXTENSION_NAME")]
pub const AMD_DISPLAY_NATIVE_HDR_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_AMD_display_native_hdr");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_SET_LOCAL_DIMMING_AMD: *const std::os::raw::c_char =
    crate::cstr!("vkSetLocalDimmingAMD");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSetLocalDimmingAMD.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkSetLocalDimmingAMD = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    swap_chain: crate::extensions::khr_swapchain::SwapchainKHR,
    local_dimming_enable: crate::vk1_0::Bool32,
) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayNativeHdrSurfaceCapabilitiesAMD.html) · Structure"]
#[doc(alias = "VkDisplayNativeHdrSurfaceCapabilitiesAMD")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DisplayNativeHdrSurfaceCapabilitiesAMD {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub local_dimming_support: crate::vk1_0::Bool32,
}
impl Default for DisplayNativeHdrSurfaceCapabilitiesAMD {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::DISPLAY_NATIVE_HDR_SURFACE_CAPABILITIES_AMD,
            p_next: std::ptr::null_mut(),
            local_dimming_support: Default::default(),
        }
    }
}
impl std::fmt::Debug for DisplayNativeHdrSurfaceCapabilitiesAMD {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DisplayNativeHdrSurfaceCapabilitiesAMD")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("local_dimming_support", &(self.local_dimming_support != 0))
            .finish()
    }
}
impl DisplayNativeHdrSurfaceCapabilitiesAMD {
    #[inline]
    pub fn into_builder<'a>(self) -> DisplayNativeHdrSurfaceCapabilitiesAMDBuilder<'a> {
        DisplayNativeHdrSurfaceCapabilitiesAMDBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayNativeHdrSurfaceCapabilitiesAMD.html) · Builder of [`DisplayNativeHdrSurfaceCapabilitiesAMD`]"]
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
    #[inline]
    pub fn local_dimming_support(mut self, local_dimming_support: bool) -> Self {
        self.0.local_dimming_support = local_dimming_support as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DisplayNativeHdrSurfaceCapabilitiesAMD {
        self.0
    }
}
impl<'a> std::default::Default for DisplayNativeHdrSurfaceCapabilitiesAMDBuilder<'a> {
    fn default() -> DisplayNativeHdrSurfaceCapabilitiesAMDBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DisplayNativeHdrSurfaceCapabilitiesAMDBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
#[doc(alias = "VkSwapchainDisplayNativeHdrCreateInfoAMD")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SwapchainDisplayNativeHdrCreateInfoAMD {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub local_dimming_enable: crate::vk1_0::Bool32,
}
impl Default for SwapchainDisplayNativeHdrCreateInfoAMD {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::SWAPCHAIN_DISPLAY_NATIVE_HDR_CREATE_INFO_AMD,
            p_next: std::ptr::null(),
            local_dimming_enable: Default::default(),
        }
    }
}
impl std::fmt::Debug for SwapchainDisplayNativeHdrCreateInfoAMD {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SwapchainDisplayNativeHdrCreateInfoAMD")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("local_dimming_enable", &(self.local_dimming_enable != 0))
            .finish()
    }
}
impl SwapchainDisplayNativeHdrCreateInfoAMD {
    #[inline]
    pub fn into_builder<'a>(self) -> SwapchainDisplayNativeHdrCreateInfoAMDBuilder<'a> {
        SwapchainDisplayNativeHdrCreateInfoAMDBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSwapchainDisplayNativeHdrCreateInfoAMD.html) · Builder of [`SwapchainDisplayNativeHdrCreateInfoAMD`]"]
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
    #[inline]
    pub fn local_dimming_enable(mut self, local_dimming_enable: bool) -> Self {
        self.0.local_dimming_enable = local_dimming_enable as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> SwapchainDisplayNativeHdrCreateInfoAMD {
        self.0
    }
}
impl<'a> std::default::Default for SwapchainDisplayNativeHdrCreateInfoAMDBuilder<'a> {
    fn default() -> SwapchainDisplayNativeHdrCreateInfoAMDBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for SwapchainDisplayNativeHdrCreateInfoAMDBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
#[doc = "Provided by [`crate::extensions::amd_display_native_hdr`]"]
impl crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSetLocalDimmingAMD.html) · Function"]
    #[doc(alias = "vkSetLocalDimmingAMD")]
    pub unsafe fn set_local_dimming_amd(
        &self,
        swap_chain: crate::extensions::khr_swapchain::SwapchainKHR,
        local_dimming_enable: bool,
    ) -> () {
        let _function = self
            .set_local_dimming_amd
            .expect("`set_local_dimming_amd` is not loaded");
        let _return = _function(self.handle, swap_chain as _, local_dimming_enable as _);
        ()
    }
}
