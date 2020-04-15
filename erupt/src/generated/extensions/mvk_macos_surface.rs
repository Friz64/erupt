# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_MVK_macos_surface.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const MVK_MACOS_SURFACE_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const MVK_MACOS_SURFACE_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_MVK_macos_surface");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateMacOSSurfaceMVK.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateMacOSSurfaceMVK = unsafe extern "system" fn(
    instance: crate::vk1_0::Instance,
    p_create_info: *const crate::extensions::mvk_macos_surface::MacOSSurfaceCreateInfoMVK,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
    p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
) -> crate::vk1_0::Result;
#[doc = "Provides Instance Commands for [`MvkMacosSurfaceInstanceLoaderExt`](trait.MvkMacosSurfaceInstanceLoaderExt.html)"]
pub struct MvkMacosSurfaceInstanceCommands {
    pub create_mac_os_surface_mvk: PFN_vkCreateMacOSSurfaceMVK,
}
impl MvkMacosSurfaceInstanceCommands {
    #[inline]
    pub fn load(loader: &crate::InstanceLoader) -> Option<MvkMacosSurfaceInstanceCommands> {
        unsafe {
            Some(MvkMacosSurfaceInstanceCommands {
                create_mac_os_surface_mvk: std::mem::transmute(
                    loader.symbol("vkCreateMacOSSurfaceMVK")?,
                ),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`MvkMacosSurfaceInstanceCommands`](struct.MvkMacosSurfaceInstanceCommands.html)"]
pub trait MvkMacosSurfaceInstanceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateMacOSSurfaceMVK.html) · Instance Command"]
    unsafe fn create_mac_os_surface_mvk(
        &self,
        create_info: &crate::extensions::mvk_macos_surface::MacOSSurfaceCreateInfoMVK,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        surface: Option<crate::extensions::khr_surface::SurfaceKHR>,
    ) -> crate::utils::VulkanResult<crate::extensions::khr_surface::SurfaceKHR>;
}
impl MvkMacosSurfaceInstanceLoaderExt for crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateMacOSSurfaceMVK.html) · Instance Command"]
    unsafe fn create_mac_os_surface_mvk(
        &self,
        create_info: &crate::extensions::mvk_macos_surface::MacOSSurfaceCreateInfoMVK,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        surface: Option<crate::extensions::khr_surface::SurfaceKHR>,
    ) -> crate::utils::VulkanResult<crate::extensions::khr_surface::SurfaceKHR> {
        let function = self
            .mvk_macos_surface
            .as_ref()
            .expect("`mvk_macos_surface` not loaded")
            .create_mac_os_surface_mvk;
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
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMacOSSurfaceCreateInfoMVK.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MacOSSurfaceCreateInfoMVK {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::mvk_macos_surface::MacOSSurfaceCreateFlagsMVK,
    pub p_view: *const std::ffi::c_void,
}
impl MacOSSurfaceCreateInfoMVK {
    #[inline]
    pub fn builder<'a>(self) -> MacOSSurfaceCreateInfoMVKBuilder<'a> {
        MacOSSurfaceCreateInfoMVKBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for MacOSSurfaceCreateInfoMVK {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("MacOSSurfaceCreateInfoMVK")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("p_view", &self.p_view)
            .finish()
    }
}
impl Default for MacOSSurfaceCreateInfoMVK {
    fn default() -> MacOSSurfaceCreateInfoMVK {
        MacOSSurfaceCreateInfoMVK {
            s_type: crate::vk1_0::StructureType::MACOS_SURFACE_CREATE_INFO_MVK,
            p_next: std::ptr::null(),
            flags: Default::default(),
            p_view: std::ptr::null(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`MacOSSurfaceCreateInfoMVK`](struct.MacOSSurfaceCreateInfoMVK.html)"]
#[repr(transparent)]
pub struct MacOSSurfaceCreateInfoMVKBuilder<'a>(
    MacOSSurfaceCreateInfoMVK,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> MacOSSurfaceCreateInfoMVKBuilder<'a> {
    #[inline]
    pub fn new() -> MacOSSurfaceCreateInfoMVKBuilder<'a> {
        MacOSSurfaceCreateInfoMVKBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn flags(
        mut self,
        flags: crate::extensions::mvk_macos_surface::MacOSSurfaceCreateFlagsMVK,
    ) -> Self {
        self.0.flags = flags;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn view(mut self, view: &'a std::ffi::c_void) -> Self {
        self.0.p_view = view;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> MacOSSurfaceCreateInfoMVK {
        self.0
    }
}
impl<'a> std::fmt::Debug for MacOSSurfaceCreateInfoMVKBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for MacOSSurfaceCreateInfoMVKBuilder<'a> {
    type Target = MacOSSurfaceCreateInfoMVK;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for MacOSSurfaceCreateInfoMVKBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Flag Bits of [`MacOSSurfaceCreateFlagsMVK`](struct.MacOSSurfaceCreateFlagsMVK.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct MacOSSurfaceCreateFlagBitsMVK(pub u32);
impl MacOSSurfaceCreateFlagBitsMVK {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> MacOSSurfaceCreateFlagsMVK {
        MacOSSurfaceCreateFlagsMVK::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for MacOSSurfaceCreateFlagBitsMVK {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            _ => "Unknown enum variant",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMacOSSurfaceCreateFlagsMVK.html) · Flags of [`MacOSSurfaceCreateFlagBitsMVK`](struct.MacOSSurfaceCreateFlagBitsMVK.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct MacOSSurfaceCreateFlagsMVK : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
