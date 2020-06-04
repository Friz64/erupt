# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_KHR_xcb_surface.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_XCB_SURFACE_SPEC_VERSION: u32 = 6;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_XCB_SURFACE_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_xcb_surface");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateXcbSurfaceKHR.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateXcbSurfaceKHR = unsafe extern "system" fn(
    instance: crate::vk1_0::Instance,
    p_create_info: *const crate::extensions::khr_xcb_surface::XcbSurfaceCreateInfoKHR,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
    p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceXcbPresentationSupportKHR.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR =
    unsafe extern "system" fn(
        physical_device: crate::vk1_0::PhysicalDevice,
        queue_family_index: u32,
        connection: *mut *const std::ffi::c_void,
        visual_id: *const std::ffi::c_void,
    ) -> crate::vk1_0::Bool32;
#[doc = "Provides Instance Commands for [`KhrXcbSurfaceInstanceLoaderExt`](trait.KhrXcbSurfaceInstanceLoaderExt.html)"]
pub struct KhrXcbSurfaceInstanceCommands {
    pub create_xcb_surface_khr: Option<PFN_vkCreateXcbSurfaceKHR>,
    pub get_physical_device_xcb_presentation_support_khr:
        Option<PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR>,
}
impl KhrXcbSurfaceInstanceCommands {
    #[inline]
    pub fn load(loader: &crate::InstanceLoader) -> Option<KhrXcbSurfaceInstanceCommands> {
        unsafe {
            let mut success = false;
            let table = KhrXcbSurfaceInstanceCommands {
                create_xcb_surface_khr: std::mem::transmute({
                    let symbol = loader.symbol("vkCreateXcbSurfaceKHR");
                    success |= symbol.is_some();
                    symbol
                }),
                get_physical_device_xcb_presentation_support_khr: std::mem::transmute({
                    let symbol = loader.symbol("vkGetPhysicalDeviceXcbPresentationSupportKHR");
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
fn instance_commands(loader: &crate::InstanceLoader) -> &KhrXcbSurfaceInstanceCommands {
    loader
        .khr_xcb_surface
        .as_ref()
        .expect("`khr_xcb_surface` not loaded")
}
#[doc = "Provides high level command wrappers for [`KhrXcbSurfaceInstanceCommands`](struct.KhrXcbSurfaceInstanceCommands.html)"]
pub trait KhrXcbSurfaceInstanceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateXcbSurfaceKHR.html) · Instance Command"]
    unsafe fn create_xcb_surface_khr(
        &self,
        create_info: &crate::extensions::khr_xcb_surface::XcbSurfaceCreateInfoKHR,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        surface: Option<crate::extensions::khr_surface::SurfaceKHR>,
    ) -> crate::utils::VulkanResult<crate::extensions::khr_surface::SurfaceKHR>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceXcbPresentationSupportKHR.html) · Instance Command"]
    unsafe fn get_physical_device_xcb_presentation_support_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        queue_family_index: u32,
        connection: *mut *const std::ffi::c_void,
        visual_id: *const std::ffi::c_void,
    ) -> crate::vk1_0::Bool32;
}
impl KhrXcbSurfaceInstanceLoaderExt for crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateXcbSurfaceKHR.html) · Instance Command"]
    unsafe fn create_xcb_surface_khr(
        &self,
        create_info: &crate::extensions::khr_xcb_surface::XcbSurfaceCreateInfoKHR,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        surface: Option<crate::extensions::khr_surface::SurfaceKHR>,
    ) -> crate::utils::VulkanResult<crate::extensions::khr_surface::SurfaceKHR> {
        let function = instance_commands(self)
            .create_xcb_surface_khr
            .as_ref()
            .expect("`create_xcb_surface_khr` not available");
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
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceXcbPresentationSupportKHR.html) · Instance Command"]
    unsafe fn get_physical_device_xcb_presentation_support_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        queue_family_index: u32,
        connection: *mut *const std::ffi::c_void,
        visual_id: *const std::ffi::c_void,
    ) -> crate::vk1_0::Bool32 {
        let function = instance_commands(self)
            .get_physical_device_xcb_presentation_support_khr
            .as_ref()
            .expect("`get_physical_device_xcb_presentation_support_khr` not available");
        let _val = function(physical_device, queue_family_index, connection, visual_id);
        _val
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkXcbSurfaceCreateInfoKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XcbSurfaceCreateInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::khr_xcb_surface::XcbSurfaceCreateFlagsKHR,
    pub connection: *mut *const std::ffi::c_void,
    pub window: u32,
}
impl XcbSurfaceCreateInfoKHR {
    #[inline]
    pub fn builder<'a>(self) -> XcbSurfaceCreateInfoKHRBuilder<'a> {
        XcbSurfaceCreateInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for XcbSurfaceCreateInfoKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("XcbSurfaceCreateInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("connection", &self.connection)
            .field("window", &self.window)
            .finish()
    }
}
impl Default for XcbSurfaceCreateInfoKHR {
    fn default() -> XcbSurfaceCreateInfoKHR {
        XcbSurfaceCreateInfoKHR {
            s_type: crate::vk1_0::StructureType::XCB_SURFACE_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            flags: Default::default(),
            connection: std::ptr::null_mut(),
            window: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkXcbSurfaceCreateInfoKHR.html) · Builder of [`XcbSurfaceCreateInfoKHR`](struct.XcbSurfaceCreateInfoKHR.html)"]
#[repr(transparent)]
pub struct XcbSurfaceCreateInfoKHRBuilder<'a>(
    XcbSurfaceCreateInfoKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> XcbSurfaceCreateInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> XcbSurfaceCreateInfoKHRBuilder<'a> {
        XcbSurfaceCreateInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn flags(
        mut self,
        flags: crate::extensions::khr_xcb_surface::XcbSurfaceCreateFlagsKHR,
    ) -> Self {
        self.0.flags = flags;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn connection(mut self, connection: &'a mut *const std::ffi::c_void) -> Self {
        self.0.connection = connection;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn window(mut self, window: u32) -> Self {
        self.0.window = window;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> XcbSurfaceCreateInfoKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for XcbSurfaceCreateInfoKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "<s>Vulkan Manual Page</s> · Flag Bits of [`XcbSurfaceCreateFlagsKHR`](struct.XcbSurfaceCreateFlagsKHR.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
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
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            _ => "(unknown)",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkXcbSurfaceCreateFlagsKHR.html) · Flags of [`XcbSurfaceCreateFlagBitsKHR`](struct.XcbSurfaceCreateFlagBitsKHR.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct XcbSurfaceCreateFlagsKHR : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
