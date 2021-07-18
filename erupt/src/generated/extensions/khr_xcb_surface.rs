#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_XCB_SURFACE_SPEC_VERSION")]
pub const KHR_XCB_SURFACE_SPEC_VERSION: u32 = 6;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_XCB_SURFACE_EXTENSION_NAME")]
pub const KHR_XCB_SURFACE_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_xcb_surface");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_XCB_SURFACE_KHR: *const std::os::raw::c_char = crate::cstr!("vkCreateXcbSurfaceKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_PHYSICAL_DEVICE_XCB_PRESENTATION_SUPPORT_KHR: *const std::os::raw::c_char = crate::cstr!("vkGetPhysicalDeviceXcbPresentationSupportKHR");
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkXcbSurfaceCreateFlagsKHR.html) · Bitmask of [`XcbSurfaceCreateFlagBitsKHR`]"] # [doc (alias = "VkXcbSurfaceCreateFlagsKHR")] # [derive (Default)] # [repr (transparent)] pub struct XcbSurfaceCreateFlagsKHR : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`XcbSurfaceCreateFlagsKHR`]"]
#[doc(alias = "VkXcbSurfaceCreateFlagBitsKHR")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct XcbSurfaceCreateFlagBitsKHR(pub u32);
impl XcbSurfaceCreateFlagBitsKHR {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> XcbSurfaceCreateFlagsKHR {
        XcbSurfaceCreateFlagsKHR::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for XcbSurfaceCreateFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::khr_xcb_surface`]"]
impl crate::vk1_0::StructureType {
    pub const XCB_SURFACE_CREATE_INFO_KHR: Self = Self(1000005000);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateXcbSurfaceKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateXcbSurfaceKHR = unsafe extern "system" fn(instance: crate::vk1_0::Instance, p_create_info: *const crate::extensions::khr_xcb_surface::XcbSurfaceCreateInfoKHR, p_allocator: *const crate::vk1_0::AllocationCallbacks, p_surface: *mut crate::extensions::khr_surface::SurfaceKHR) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceXcbPresentationSupportKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR = unsafe extern "system" fn(physical_device: crate::vk1_0::PhysicalDevice, queue_family_index: u32, connection: *mut std::ffi::c_void, visual_id: u32) -> crate::vk1_0::Bool32;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkXcbSurfaceCreateInfoKHR.html) · Structure"]
#[doc(alias = "VkXcbSurfaceCreateInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XcbSurfaceCreateInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::khr_xcb_surface::XcbSurfaceCreateFlagsKHR,
    pub connection: *mut std::ffi::c_void,
    pub window: u32,
}
impl XcbSurfaceCreateInfoKHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::XCB_SURFACE_CREATE_INFO_KHR;
}
impl Default for XcbSurfaceCreateInfoKHR {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::XCB_SURFACE_CREATE_INFO_KHR, p_next: std::ptr::null(), flags: Default::default(), connection: std::ptr::null_mut(), window: Default::default() }
    }
}
impl std::fmt::Debug for XcbSurfaceCreateInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("XcbSurfaceCreateInfoKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("connection", &self.connection).field("window", &self.window).finish()
    }
}
impl XcbSurfaceCreateInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> XcbSurfaceCreateInfoKHRBuilder<'a> {
        XcbSurfaceCreateInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkXcbSurfaceCreateInfoKHR.html) · Builder of [`XcbSurfaceCreateInfoKHR`]"]
#[repr(transparent)]
pub struct XcbSurfaceCreateInfoKHRBuilder<'a>(XcbSurfaceCreateInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> XcbSurfaceCreateInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> XcbSurfaceCreateInfoKHRBuilder<'a> {
        XcbSurfaceCreateInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::khr_xcb_surface::XcbSurfaceCreateFlagsKHR) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn connection(mut self, connection: *mut std::ffi::c_void) -> Self {
        self.0.connection = connection;
        self
    }
    #[inline]
    pub fn window(mut self, window: u32) -> Self {
        self.0.window = window as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> XcbSurfaceCreateInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for XcbSurfaceCreateInfoKHRBuilder<'a> {
    fn default() -> XcbSurfaceCreateInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for XcbSurfaceCreateInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for XcbSurfaceCreateInfoKHRBuilder<'a> {
    type Target = XcbSurfaceCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for XcbSurfaceCreateInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::khr_xcb_surface`]"]
impl crate::InstanceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateXcbSurfaceKHR.html) · Function"]
    #[doc(alias = "vkCreateXcbSurfaceKHR")]
    pub unsafe fn create_xcb_surface_khr(&self, create_info: &crate::extensions::khr_xcb_surface::XcbSurfaceCreateInfoKHR, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> crate::utils::VulkanResult<crate::extensions::khr_surface::SurfaceKHR> {
        let _function = self.create_xcb_surface_khr.expect(crate::NOT_LOADED_MESSAGE);
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
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceXcbPresentationSupportKHR.html) · Function"]
    #[doc(alias = "vkGetPhysicalDeviceXcbPresentationSupportKHR")]
    pub unsafe fn get_physical_device_xcb_presentation_support_khr(&self, physical_device: crate::vk1_0::PhysicalDevice, queue_family_index: u32, connection: *mut std::ffi::c_void, visual_id: u32) -> bool {
        let _function = self.get_physical_device_xcb_presentation_support_khr.expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(physical_device as _, queue_family_index as _, connection, visual_id as _);
        _return != 0
    }
}
