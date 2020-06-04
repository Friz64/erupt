# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_MVK_ios_surface.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const MVK_IOS_SURFACE_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const MVK_IOS_SURFACE_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_MVK_ios_surface");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateIOSSurfaceMVK.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateIOSSurfaceMVK = unsafe extern "system" fn(
    instance: crate::vk1_0::Instance,
    p_create_info: *const crate::extensions::mvk_ios_surface::IOSSurfaceCreateInfoMVK,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
    p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
) -> crate::vk1_0::Result;
#[doc = "Provides Instance Commands for [`MvkIosSurfaceInstanceLoaderExt`](trait.MvkIosSurfaceInstanceLoaderExt.html)"]
pub struct MvkIosSurfaceInstanceCommands {
    pub create_ios_surface_mvk: Option<PFN_vkCreateIOSSurfaceMVK>,
}
impl MvkIosSurfaceInstanceCommands {
    #[inline]
    pub fn load(loader: &crate::InstanceLoader) -> Option<MvkIosSurfaceInstanceCommands> {
        unsafe {
            let mut success = false;
            let table = MvkIosSurfaceInstanceCommands {
                create_ios_surface_mvk: std::mem::transmute({
                    let symbol = loader.symbol("vkCreateIOSSurfaceMVK");
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
fn instance_commands(loader: &crate::InstanceLoader) -> &MvkIosSurfaceInstanceCommands {
    loader
        .mvk_ios_surface
        .as_ref()
        .expect("`mvk_ios_surface` not loaded")
}
#[doc = "Provides high level command wrappers for [`MvkIosSurfaceInstanceCommands`](struct.MvkIosSurfaceInstanceCommands.html)"]
pub trait MvkIosSurfaceInstanceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateIOSSurfaceMVK.html) · Instance Command"]
    unsafe fn create_ios_surface_mvk(
        &self,
        create_info: &crate::extensions::mvk_ios_surface::IOSSurfaceCreateInfoMVK,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        surface: Option<crate::extensions::khr_surface::SurfaceKHR>,
    ) -> crate::utils::VulkanResult<crate::extensions::khr_surface::SurfaceKHR>;
}
impl MvkIosSurfaceInstanceLoaderExt for crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateIOSSurfaceMVK.html) · Instance Command"]
    unsafe fn create_ios_surface_mvk(
        &self,
        create_info: &crate::extensions::mvk_ios_surface::IOSSurfaceCreateInfoMVK,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        surface: Option<crate::extensions::khr_surface::SurfaceKHR>,
    ) -> crate::utils::VulkanResult<crate::extensions::khr_surface::SurfaceKHR> {
        let function = instance_commands(self)
            .create_ios_surface_mvk
            .as_ref()
            .expect("`create_ios_surface_mvk` not available");
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkIOSSurfaceCreateInfoMVK.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IOSSurfaceCreateInfoMVK {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::mvk_ios_surface::IOSSurfaceCreateFlagsMVK,
    pub p_view: *const std::ffi::c_void,
}
impl IOSSurfaceCreateInfoMVK {
    #[inline]
    pub fn builder<'a>(self) -> IOSSurfaceCreateInfoMVKBuilder<'a> {
        IOSSurfaceCreateInfoMVKBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for IOSSurfaceCreateInfoMVK {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("IOSSurfaceCreateInfoMVK")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("p_view", &self.p_view)
            .finish()
    }
}
impl Default for IOSSurfaceCreateInfoMVK {
    fn default() -> IOSSurfaceCreateInfoMVK {
        IOSSurfaceCreateInfoMVK {
            s_type: crate::vk1_0::StructureType::IOS_SURFACE_CREATE_INFO_MVK,
            p_next: std::ptr::null(),
            flags: Default::default(),
            p_view: std::ptr::null(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkIOSSurfaceCreateInfoMVK.html) · Builder of [`IOSSurfaceCreateInfoMVK`](struct.IOSSurfaceCreateInfoMVK.html)"]
#[repr(transparent)]
pub struct IOSSurfaceCreateInfoMVKBuilder<'a>(
    IOSSurfaceCreateInfoMVK,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> IOSSurfaceCreateInfoMVKBuilder<'a> {
    #[inline]
    pub fn new() -> IOSSurfaceCreateInfoMVKBuilder<'a> {
        IOSSurfaceCreateInfoMVKBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn flags(
        mut self,
        flags: crate::extensions::mvk_ios_surface::IOSSurfaceCreateFlagsMVK,
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
    pub unsafe fn discard(self) -> IOSSurfaceCreateInfoMVK {
        self.0
    }
}
impl<'a> std::fmt::Debug for IOSSurfaceCreateInfoMVKBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for IOSSurfaceCreateInfoMVKBuilder<'a> {
    type Target = IOSSurfaceCreateInfoMVK;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for IOSSurfaceCreateInfoMVKBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Flag Bits of [`IOSSurfaceCreateFlagsMVK`](struct.IOSSurfaceCreateFlagsMVK.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct IOSSurfaceCreateFlagBitsMVK(pub u32);
impl IOSSurfaceCreateFlagBitsMVK {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> IOSSurfaceCreateFlagsMVK {
        IOSSurfaceCreateFlagsMVK::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for IOSSurfaceCreateFlagBitsMVK {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            _ => "(unknown)",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkIOSSurfaceCreateFlagsMVK.html) · Flags of [`IOSSurfaceCreateFlagBitsMVK`](struct.IOSSurfaceCreateFlagBitsMVK.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct IOSSurfaceCreateFlagsMVK : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
