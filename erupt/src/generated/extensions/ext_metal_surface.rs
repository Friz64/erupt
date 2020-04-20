# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_EXT_metal_surface.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_METAL_SURFACE_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_METAL_SURFACE_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_metal_surface");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateMetalSurfaceEXT.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateMetalSurfaceEXT = unsafe extern "system" fn(
    instance: crate::vk1_0::Instance,
    p_create_info: *const crate::extensions::ext_metal_surface::MetalSurfaceCreateInfoEXT,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
    p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
) -> crate::vk1_0::Result;
#[doc = "Provides Instance Commands for [`ExtMetalSurfaceInstanceLoaderExt`](trait.ExtMetalSurfaceInstanceLoaderExt.html)"]
pub struct ExtMetalSurfaceInstanceCommands {
    pub create_metal_surface_ext: PFN_vkCreateMetalSurfaceEXT,
}
impl ExtMetalSurfaceInstanceCommands {
    #[inline]
    pub fn load(loader: &crate::InstanceLoader) -> Option<ExtMetalSurfaceInstanceCommands> {
        unsafe {
            Some(ExtMetalSurfaceInstanceCommands {
                create_metal_surface_ext: std::mem::transmute(
                    loader.symbol("vkCreateMetalSurfaceEXT")?,
                ),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`ExtMetalSurfaceInstanceCommands`](struct.ExtMetalSurfaceInstanceCommands.html)"]
pub trait ExtMetalSurfaceInstanceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateMetalSurfaceEXT.html) · Instance Command"]
    unsafe fn create_metal_surface_ext(
        &self,
        create_info: &crate::extensions::ext_metal_surface::MetalSurfaceCreateInfoEXT,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        surface: Option<crate::extensions::khr_surface::SurfaceKHR>,
    ) -> crate::utils::VulkanResult<crate::extensions::khr_surface::SurfaceKHR>;
}
impl ExtMetalSurfaceInstanceLoaderExt for crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateMetalSurfaceEXT.html) · Instance Command"]
    unsafe fn create_metal_surface_ext(
        &self,
        create_info: &crate::extensions::ext_metal_surface::MetalSurfaceCreateInfoEXT,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        surface: Option<crate::extensions::khr_surface::SurfaceKHR>,
    ) -> crate::utils::VulkanResult<crate::extensions::khr_surface::SurfaceKHR> {
        let function = self
            .ext_metal_surface
            .as_ref()
            .expect("`ext_metal_surface` not loaded")
            .create_metal_surface_ext;
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMetalSurfaceCreateInfoEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MetalSurfaceCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::ext_metal_surface::MetalSurfaceCreateFlagsEXT,
    pub p_layer: *const std::ffi::c_void,
}
impl MetalSurfaceCreateInfoEXT {
    #[inline]
    pub fn builder<'a>(self) -> MetalSurfaceCreateInfoEXTBuilder<'a> {
        MetalSurfaceCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for MetalSurfaceCreateInfoEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("MetalSurfaceCreateInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("p_layer", &self.p_layer)
            .finish()
    }
}
impl Default for MetalSurfaceCreateInfoEXT {
    fn default() -> MetalSurfaceCreateInfoEXT {
        MetalSurfaceCreateInfoEXT {
            s_type: crate::vk1_0::StructureType::METAL_SURFACE_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            flags: Default::default(),
            p_layer: std::ptr::null(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMetalSurfaceCreateInfoEXT.html) · Builder of [`MetalSurfaceCreateInfoEXT`](struct.MetalSurfaceCreateInfoEXT.html)"]
#[repr(transparent)]
pub struct MetalSurfaceCreateInfoEXTBuilder<'a>(
    MetalSurfaceCreateInfoEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> MetalSurfaceCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> MetalSurfaceCreateInfoEXTBuilder<'a> {
        MetalSurfaceCreateInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn flags(
        mut self,
        flags: crate::extensions::ext_metal_surface::MetalSurfaceCreateFlagsEXT,
    ) -> Self {
        self.0.flags = flags;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn layer(mut self, layer: &'a std::ffi::c_void) -> Self {
        self.0.p_layer = layer;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> MetalSurfaceCreateInfoEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for MetalSurfaceCreateInfoEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for MetalSurfaceCreateInfoEXTBuilder<'a> {
    type Target = MetalSurfaceCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for MetalSurfaceCreateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Flag Bits of [`MetalSurfaceCreateFlagsEXT`](struct.MetalSurfaceCreateFlagsEXT.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct MetalSurfaceCreateFlagBitsEXT(pub u32);
impl MetalSurfaceCreateFlagBitsEXT {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> MetalSurfaceCreateFlagsEXT {
        MetalSurfaceCreateFlagsEXT::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for MetalSurfaceCreateFlagBitsEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            _ => "(unknown)",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMetalSurfaceCreateFlagsEXT.html) · Flags of [`MetalSurfaceCreateFlagBitsEXT`](struct.MetalSurfaceCreateFlagBitsEXT.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct MetalSurfaceCreateFlagsEXT : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
