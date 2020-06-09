# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_EXT_headless_surface.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_HEADLESS_SURFACE_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_HEADLESS_SURFACE_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_headless_surface");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateHeadlessSurfaceEXT.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateHeadlessSurfaceEXT = unsafe extern "system" fn(
    instance: crate::vk1_0::Instance,
    p_create_info: *const crate::extensions::ext_headless_surface::HeadlessSurfaceCreateInfoEXT,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
    p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
) -> crate::vk1_0::Result;
#[doc = "Provides Instance Commands for [`ExtHeadlessSurfaceInstanceLoaderExt`](trait.ExtHeadlessSurfaceInstanceLoaderExt.html)"]
pub struct ExtHeadlessSurfaceInstanceCommands {
    pub create_headless_surface_ext: Option<PFN_vkCreateHeadlessSurfaceEXT>,
}
impl ExtHeadlessSurfaceInstanceCommands {
    #[inline]
    pub fn load(loader: &crate::InstanceLoader) -> Option<ExtHeadlessSurfaceInstanceCommands> {
        unsafe {
            let mut success = false;
            let table = ExtHeadlessSurfaceInstanceCommands {
                create_headless_surface_ext: std::mem::transmute({
                    let symbol = loader.symbol("vkCreateHeadlessSurfaceEXT");
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
fn instance_commands(loader: &crate::InstanceLoader) -> &ExtHeadlessSurfaceInstanceCommands {
    loader
        .ext_headless_surface
        .as_ref()
        .expect("`ext_headless_surface` not loaded")
}
#[doc = "Provides high level command wrappers for [`ExtHeadlessSurfaceInstanceCommands`](struct.ExtHeadlessSurfaceInstanceCommands.html)"]
pub trait ExtHeadlessSurfaceInstanceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateHeadlessSurfaceEXT.html) · Instance Command"]
    unsafe fn create_headless_surface_ext(
        &self,
        create_info: &crate::extensions::ext_headless_surface::HeadlessSurfaceCreateInfoEXT,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        surface: Option<crate::extensions::khr_surface::SurfaceKHR>,
    ) -> crate::utils::VulkanResult<crate::extensions::khr_surface::SurfaceKHR>;
}
impl ExtHeadlessSurfaceInstanceLoaderExt for crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateHeadlessSurfaceEXT.html) · Instance Command"]
    unsafe fn create_headless_surface_ext(
        &self,
        create_info: &crate::extensions::ext_headless_surface::HeadlessSurfaceCreateInfoEXT,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        surface: Option<crate::extensions::khr_surface::SurfaceKHR>,
    ) -> crate::utils::VulkanResult<crate::extensions::khr_surface::SurfaceKHR> {
        let function = instance_commands(self)
            .create_headless_surface_ext
            .as_ref()
            .expect("`create_headless_surface_ext` not available");
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkHeadlessSurfaceCreateInfoEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HeadlessSurfaceCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::ext_headless_surface::HeadlessSurfaceCreateFlagsEXT,
}
impl HeadlessSurfaceCreateInfoEXT {
    #[inline]
    pub fn builder<'a>(self) -> HeadlessSurfaceCreateInfoEXTBuilder<'a> {
        HeadlessSurfaceCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for HeadlessSurfaceCreateInfoEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("HeadlessSurfaceCreateInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .finish()
    }
}
impl Default for HeadlessSurfaceCreateInfoEXT {
    fn default() -> HeadlessSurfaceCreateInfoEXT {
        HeadlessSurfaceCreateInfoEXT {
            s_type: crate::vk1_0::StructureType::HEADLESS_SURFACE_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            flags: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkHeadlessSurfaceCreateInfoEXT.html) · Builder of [`HeadlessSurfaceCreateInfoEXT`](struct.HeadlessSurfaceCreateInfoEXT.html)"]
#[repr(transparent)]
pub struct HeadlessSurfaceCreateInfoEXTBuilder<'a>(
    HeadlessSurfaceCreateInfoEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> HeadlessSurfaceCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> HeadlessSurfaceCreateInfoEXTBuilder<'a> {
        HeadlessSurfaceCreateInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn flags(
        mut self,
        flags: crate::extensions::ext_headless_surface::HeadlessSurfaceCreateFlagsEXT,
    ) -> Self {
        self.0.flags = flags;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> HeadlessSurfaceCreateInfoEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for HeadlessSurfaceCreateInfoEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for HeadlessSurfaceCreateInfoEXTBuilder<'a> {
    type Target = HeadlessSurfaceCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for HeadlessSurfaceCreateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Flag Bits of [`HeadlessSurfaceCreateFlagsEXT`](struct.HeadlessSurfaceCreateFlagsEXT.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct HeadlessSurfaceCreateFlagBitsEXT(pub u32);
impl HeadlessSurfaceCreateFlagBitsEXT {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> HeadlessSurfaceCreateFlagsEXT {
        HeadlessSurfaceCreateFlagsEXT::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for HeadlessSurfaceCreateFlagBitsEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            _ => "(unknown)",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkHeadlessSurfaceCreateFlagsEXT.html) · Flags of [`HeadlessSurfaceCreateFlagBitsEXT`](struct.HeadlessSurfaceCreateFlagBitsEXT.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct HeadlessSurfaceCreateFlagsEXT : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
