#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_QNX_SCREEN_SURFACE_SPEC_VERSION")]
pub const QNX_SCREEN_SURFACE_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_QNX_SCREEN_SURFACE_EXTENSION_NAME")]
pub const QNX_SCREEN_SURFACE_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_QNX_screen_surface");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_SCREEN_SURFACE_QNX: *const std::os::raw::c_char = crate::cstr!("vkCreateScreenSurfaceQNX");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_PHYSICAL_DEVICE_SCREEN_PRESENTATION_SUPPORT_QNX: *const std::os::raw::c_char = crate::cstr!("vkGetPhysicalDeviceScreenPresentationSupportQNX");
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkScreenSurfaceCreateFlagsQNX.html) · Bitmask of [`ScreenSurfaceCreateFlagBitsQNX`]"] # [doc (alias = "VkScreenSurfaceCreateFlagsQNX")] # [derive (Default)] # [repr (transparent)] pub struct ScreenSurfaceCreateFlagsQNX : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`ScreenSurfaceCreateFlagsQNX`]"]
#[doc(alias = "VkScreenSurfaceCreateFlagBitsQNX")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct ScreenSurfaceCreateFlagBitsQNX(pub u32);
impl ScreenSurfaceCreateFlagBitsQNX {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> ScreenSurfaceCreateFlagsQNX {
        ScreenSurfaceCreateFlagsQNX::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for ScreenSurfaceCreateFlagBitsQNX {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateScreenSurfaceQNX.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateScreenSurfaceQNX = unsafe extern "system" fn(
    instance: crate::vk1_0::Instance,
    p_create_info: *const crate::extensions::qnx_screen_surface::ScreenSurfaceCreateInfoQNX,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
    p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceScreenPresentationSupportQNX.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceScreenPresentationSupportQNX =
    unsafe extern "system" fn(physical_device: crate::vk1_0::PhysicalDevice, queue_family_index: u32, window: *mut std::ffi::c_void) -> crate::vk1_0::Bool32;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkScreenSurfaceCreateInfoQNX.html) · Structure"]
#[doc(alias = "VkScreenSurfaceCreateInfoQNX")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ScreenSurfaceCreateInfoQNX {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::qnx_screen_surface::ScreenSurfaceCreateFlagsQNX,
    pub context: *mut std::ffi::c_void,
    pub window: *mut std::ffi::c_void,
}
impl Default for ScreenSurfaceCreateInfoQNX {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::SCREEN_SURFACE_CREATE_INFO_QNX,
            p_next: std::ptr::null(),
            flags: Default::default(),
            context: std::ptr::null_mut(),
            window: std::ptr::null_mut(),
        }
    }
}
impl std::fmt::Debug for ScreenSurfaceCreateInfoQNX {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ScreenSurfaceCreateInfoQNX")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("context", &self.context)
            .field("window", &self.window)
            .finish()
    }
}
impl ScreenSurfaceCreateInfoQNX {
    #[inline]
    pub fn into_builder<'a>(self) -> ScreenSurfaceCreateInfoQNXBuilder<'a> {
        ScreenSurfaceCreateInfoQNXBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkScreenSurfaceCreateInfoQNX.html) · Builder of [`ScreenSurfaceCreateInfoQNX`]"]
#[repr(transparent)]
pub struct ScreenSurfaceCreateInfoQNXBuilder<'a>(ScreenSurfaceCreateInfoQNX, std::marker::PhantomData<&'a ()>);
impl<'a> ScreenSurfaceCreateInfoQNXBuilder<'a> {
    #[inline]
    pub fn new() -> ScreenSurfaceCreateInfoQNXBuilder<'a> {
        ScreenSurfaceCreateInfoQNXBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::qnx_screen_surface::ScreenSurfaceCreateFlagsQNX) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn context(mut self, context: *mut std::ffi::c_void) -> Self {
        self.0.context = context;
        self
    }
    #[inline]
    pub fn window(mut self, window: *mut std::ffi::c_void) -> Self {
        self.0.window = window;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ScreenSurfaceCreateInfoQNX {
        self.0
    }
}
impl<'a> std::default::Default for ScreenSurfaceCreateInfoQNXBuilder<'a> {
    fn default() -> ScreenSurfaceCreateInfoQNXBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ScreenSurfaceCreateInfoQNXBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ScreenSurfaceCreateInfoQNXBuilder<'a> {
    type Target = ScreenSurfaceCreateInfoQNX;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ScreenSurfaceCreateInfoQNXBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<ScreenSurfaceCreateInfoQNX> for ScreenSurfaceCreateInfoQNXBuilder<'_> {}
#[doc = "Provided by [`crate::extensions::qnx_screen_surface`]"]
impl crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateScreenSurfaceQNX.html) · Function"]
    #[doc(alias = "vkCreateScreenSurfaceQNX")]
    pub unsafe fn create_screen_surface_qnx(
        &self,
        create_info: &crate::extensions::qnx_screen_surface::ScreenSurfaceCreateInfoQNX,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
    ) -> crate::utils::VulkanResult<crate::extensions::khr_surface::SurfaceKHR> {
        let _function = self.create_screen_surface_qnx.expect("`create_screen_surface_qnx` is not loaded");
        let mut surface = Default::default();
        let _return = _function(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut surface,
        );
        crate::utils::VulkanResult::new(_return, surface)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceScreenPresentationSupportQNX.html) · Function"]
    #[doc(alias = "vkGetPhysicalDeviceScreenPresentationSupportQNX")]
    pub unsafe fn get_physical_device_screen_presentation_support_qnx(&self, physical_device: crate::vk1_0::PhysicalDevice, queue_family_index: u32, window: *mut std::ffi::c_void) -> bool {
        let _function = self
            .get_physical_device_screen_presentation_support_qnx
            .expect("`get_physical_device_screen_presentation_support_qnx` is not loaded");
        let _return = _function(physical_device as _, queue_family_index as _, window);
        _return != 0
    }
}
