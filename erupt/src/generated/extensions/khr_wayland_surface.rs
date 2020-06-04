# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_KHR_wayland_surface.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_WAYLAND_SURFACE_SPEC_VERSION: u32 = 6;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_WAYLAND_SURFACE_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_wayland_surface");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateWaylandSurfaceKHR.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateWaylandSurfaceKHR = unsafe extern "system" fn(
    instance: crate::vk1_0::Instance,
    p_create_info: *const crate::extensions::khr_wayland_surface::WaylandSurfaceCreateInfoKHR,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
    p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceWaylandPresentationSupportKHR.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR =
    unsafe extern "system" fn(
        physical_device: crate::vk1_0::PhysicalDevice,
        queue_family_index: u32,
        display: *mut std::ffi::c_void,
    ) -> crate::vk1_0::Bool32;
#[doc = "Provides Instance Commands for [`KhrWaylandSurfaceInstanceLoaderExt`](trait.KhrWaylandSurfaceInstanceLoaderExt.html)"]
pub struct KhrWaylandSurfaceInstanceCommands {
    pub create_wayland_surface_khr: Option<PFN_vkCreateWaylandSurfaceKHR>,
    pub get_physical_device_wayland_presentation_support_khr:
        Option<PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR>,
}
impl KhrWaylandSurfaceInstanceCommands {
    #[inline]
    pub fn load(loader: &crate::InstanceLoader) -> Option<KhrWaylandSurfaceInstanceCommands> {
        unsafe {
            let mut success = false;
            let table = KhrWaylandSurfaceInstanceCommands {
                create_wayland_surface_khr: std::mem::transmute({
                    let symbol = loader.symbol("vkCreateWaylandSurfaceKHR");
                    success |= symbol.is_some();
                    symbol
                }),
                get_physical_device_wayland_presentation_support_khr: std::mem::transmute({
                    let symbol = loader.symbol("vkGetPhysicalDeviceWaylandPresentationSupportKHR");
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
fn instance_commands(loader: &crate::InstanceLoader) -> &KhrWaylandSurfaceInstanceCommands {
    loader
        .khr_wayland_surface
        .as_ref()
        .expect("`khr_wayland_surface` not loaded")
}
#[doc = "Provides high level command wrappers for [`KhrWaylandSurfaceInstanceCommands`](struct.KhrWaylandSurfaceInstanceCommands.html)"]
pub trait KhrWaylandSurfaceInstanceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateWaylandSurfaceKHR.html) · Instance Command"]
    unsafe fn create_wayland_surface_khr(
        &self,
        create_info: &crate::extensions::khr_wayland_surface::WaylandSurfaceCreateInfoKHR,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        surface: Option<crate::extensions::khr_surface::SurfaceKHR>,
    ) -> crate::utils::VulkanResult<crate::extensions::khr_surface::SurfaceKHR>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceWaylandPresentationSupportKHR.html) · Instance Command"]
    unsafe fn get_physical_device_wayland_presentation_support_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        queue_family_index: u32,
        display: *mut std::ffi::c_void,
    ) -> crate::vk1_0::Bool32;
}
impl KhrWaylandSurfaceInstanceLoaderExt for crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateWaylandSurfaceKHR.html) · Instance Command"]
    unsafe fn create_wayland_surface_khr(
        &self,
        create_info: &crate::extensions::khr_wayland_surface::WaylandSurfaceCreateInfoKHR,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        surface: Option<crate::extensions::khr_surface::SurfaceKHR>,
    ) -> crate::utils::VulkanResult<crate::extensions::khr_surface::SurfaceKHR> {
        let function = instance_commands(self)
            .create_wayland_surface_khr
            .as_ref()
            .expect("`create_wayland_surface_khr` not available");
        let mut surface = surface.unwrap_or_else(|| Default::default());
        let _val = function(
            self.handle,
            create_info,
            if let Some(allocator) = allocator {
                allocator
            } else {
                std::ptr::null()
            },
            &mut surface,
        );
        crate::utils::VulkanResult::new(_val, surface)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceWaylandPresentationSupportKHR.html) · Instance Command"]
    unsafe fn get_physical_device_wayland_presentation_support_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        queue_family_index: u32,
        display: *mut std::ffi::c_void,
    ) -> crate::vk1_0::Bool32 {
        let function = instance_commands(self)
            .get_physical_device_wayland_presentation_support_khr
            .as_ref()
            .expect("`get_physical_device_wayland_presentation_support_khr` not available");
        let _val = function(physical_device, queue_family_index, display);
        _val
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkWaylandSurfaceCreateInfoKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WaylandSurfaceCreateInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::khr_wayland_surface::WaylandSurfaceCreateFlagsKHR,
    pub display: *mut std::ffi::c_void,
    pub surface: *mut std::ffi::c_void,
}
impl WaylandSurfaceCreateInfoKHR {
    #[inline]
    pub fn builder<'a>(self) -> WaylandSurfaceCreateInfoKHRBuilder<'a> {
        WaylandSurfaceCreateInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for WaylandSurfaceCreateInfoKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("WaylandSurfaceCreateInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("display", &self.display)
            .field("surface", &self.surface)
            .finish()
    }
}
impl Default for WaylandSurfaceCreateInfoKHR {
    fn default() -> WaylandSurfaceCreateInfoKHR {
        WaylandSurfaceCreateInfoKHR {
            s_type: crate::vk1_0::StructureType::WAYLAND_SURFACE_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            flags: Default::default(),
            display: std::ptr::null_mut(),
            surface: std::ptr::null_mut(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkWaylandSurfaceCreateInfoKHR.html) · Builder of [`WaylandSurfaceCreateInfoKHR`](struct.WaylandSurfaceCreateInfoKHR.html)"]
#[repr(transparent)]
pub struct WaylandSurfaceCreateInfoKHRBuilder<'a>(
    WaylandSurfaceCreateInfoKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> WaylandSurfaceCreateInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> WaylandSurfaceCreateInfoKHRBuilder<'a> {
        WaylandSurfaceCreateInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn flags(
        mut self,
        flags: crate::extensions::khr_wayland_surface::WaylandSurfaceCreateFlagsKHR,
    ) -> Self {
        self.0.flags = flags;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn display(mut self, display: &'a mut std::ffi::c_void) -> Self {
        self.0.display = display;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn surface(mut self, surface: &'a mut std::ffi::c_void) -> Self {
        self.0.surface = surface;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> WaylandSurfaceCreateInfoKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for WaylandSurfaceCreateInfoKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "<s>Vulkan Manual Page</s> · Flag Bits of [`WaylandSurfaceCreateFlagsKHR`](struct.WaylandSurfaceCreateFlagsKHR.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
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
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            _ => "(unknown)",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkWaylandSurfaceCreateFlagsKHR.html) · Flags of [`WaylandSurfaceCreateFlagBitsKHR`](struct.WaylandSurfaceCreateFlagBitsKHR.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct WaylandSurfaceCreateFlagsKHR : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
