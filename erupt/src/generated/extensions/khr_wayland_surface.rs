#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_WAYLAND_SURFACE_SPEC_VERSION")]
pub const KHR_WAYLAND_SURFACE_SPEC_VERSION: u32 = 6;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_WAYLAND_SURFACE_EXTENSION_NAME")]
pub const KHR_WAYLAND_SURFACE_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_wayland_surface");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_WAYLAND_SURFACE_KHR: *const std::os::raw::c_char = crate::cstr!("vkCreateWaylandSurfaceKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_PHYSICAL_DEVICE_WAYLAND_PRESENTATION_SUPPORT_KHR: *const std::os::raw::c_char = crate::cstr!("vkGetPhysicalDeviceWaylandPresentationSupportKHR");
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkWaylandSurfaceCreateFlagsKHR.html) · Bitmask of [`WaylandSurfaceCreateFlagBitsKHR`]"] # [doc (alias = "VkWaylandSurfaceCreateFlagsKHR")] # [derive (Default)] # [repr (transparent)] pub struct WaylandSurfaceCreateFlagsKHR : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`WaylandSurfaceCreateFlagsKHR`]"]
#[doc(alias = "VkWaylandSurfaceCreateFlagBitsKHR")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct WaylandSurfaceCreateFlagBitsKHR(pub u32);
impl WaylandSurfaceCreateFlagBitsKHR {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> WaylandSurfaceCreateFlagsKHR {
        WaylandSurfaceCreateFlagsKHR::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for WaylandSurfaceCreateFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::khr_wayland_surface`]"]
impl crate::vk1_0::StructureType {
    pub const WAYLAND_SURFACE_CREATE_INFO_KHR: Self = Self(1000006000);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateWaylandSurfaceKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateWaylandSurfaceKHR = unsafe extern "system" fn(instance: crate::vk1_0::Instance, p_create_info: *const crate::extensions::khr_wayland_surface::WaylandSurfaceCreateInfoKHR, p_allocator: *const crate::vk1_0::AllocationCallbacks, p_surface: *mut crate::extensions::khr_surface::SurfaceKHR) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceWaylandPresentationSupportKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR = unsafe extern "system" fn(physical_device: crate::vk1_0::PhysicalDevice, queue_family_index: u32, display: *mut std::ffi::c_void) -> crate::vk1_0::Bool32;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkWaylandSurfaceCreateInfoKHR.html) · Structure"]
#[doc(alias = "VkWaylandSurfaceCreateInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WaylandSurfaceCreateInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::khr_wayland_surface::WaylandSurfaceCreateFlagsKHR,
    pub display: *mut std::ffi::c_void,
    pub surface: *mut std::ffi::c_void,
}
impl Default for WaylandSurfaceCreateInfoKHR {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::WAYLAND_SURFACE_CREATE_INFO_KHR, p_next: std::ptr::null(), flags: Default::default(), display: std::ptr::null_mut(), surface: std::ptr::null_mut() }
    }
}
impl std::fmt::Debug for WaylandSurfaceCreateInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("WaylandSurfaceCreateInfoKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("display", &self.display).field("surface", &self.surface).finish()
    }
}
impl WaylandSurfaceCreateInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> WaylandSurfaceCreateInfoKHRBuilder<'a> {
        WaylandSurfaceCreateInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkWaylandSurfaceCreateInfoKHR.html) · Builder of [`WaylandSurfaceCreateInfoKHR`]"]
#[repr(transparent)]
pub struct WaylandSurfaceCreateInfoKHRBuilder<'a>(WaylandSurfaceCreateInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> WaylandSurfaceCreateInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> WaylandSurfaceCreateInfoKHRBuilder<'a> {
        WaylandSurfaceCreateInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::khr_wayland_surface::WaylandSurfaceCreateFlagsKHR) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn display(mut self, display: *mut std::ffi::c_void) -> Self {
        self.0.display = display;
        self
    }
    #[inline]
    pub fn surface(mut self, surface: *mut std::ffi::c_void) -> Self {
        self.0.surface = surface;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> WaylandSurfaceCreateInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for WaylandSurfaceCreateInfoKHRBuilder<'a> {
    fn default() -> WaylandSurfaceCreateInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for WaylandSurfaceCreateInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for WaylandSurfaceCreateInfoKHRBuilder<'a> {
    type Target = WaylandSurfaceCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for WaylandSurfaceCreateInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::khr_wayland_surface`]"]
impl crate::InstanceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateWaylandSurfaceKHR.html) · Function"]
    #[doc(alias = "vkCreateWaylandSurfaceKHR")]
    pub unsafe fn create_wayland_surface_khr(&self, create_info: &crate::extensions::khr_wayland_surface::WaylandSurfaceCreateInfoKHR, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> crate::utils::VulkanResult<crate::extensions::khr_surface::SurfaceKHR> {
        let _function = self.create_wayland_surface_khr.expect("tried to call a function that isn't loaded");
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
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceWaylandPresentationSupportKHR.html) · Function"]
    #[doc(alias = "vkGetPhysicalDeviceWaylandPresentationSupportKHR")]
    pub unsafe fn get_physical_device_wayland_presentation_support_khr(&self, physical_device: crate::vk1_0::PhysicalDevice, queue_family_index: u32, display: *mut std::ffi::c_void) -> bool {
        let _function = self.get_physical_device_wayland_presentation_support_khr.expect("tried to call a function that isn't loaded");
        let _return = _function(physical_device as _, queue_family_index as _, display);
        _return != 0
    }
}
