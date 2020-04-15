# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_FUCHSIA_imagepipe_surface.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FUCHSIA_IMAGEPIPE_SURFACE_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FUCHSIA_IMAGEPIPE_SURFACE_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_FUCHSIA_imagepipe_surface");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateImagePipeSurfaceFUCHSIA.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateImagePipeSurfaceFUCHSIA = unsafe extern "system" fn ( instance : crate :: vk1_0 :: Instance , p_create_info : * const crate :: extensions :: fuchsia_imagepipe_surface :: ImagePipeSurfaceCreateInfoFUCHSIA , p_allocator : * const crate :: vk1_0 :: AllocationCallbacks , p_surface : * mut crate :: extensions :: khr_surface :: SurfaceKHR , ) -> crate :: vk1_0 :: Result ;
#[doc = "Provides Instance Commands for [`FuchsiaImagepipeSurfaceInstanceLoaderExt`](trait.FuchsiaImagepipeSurfaceInstanceLoaderExt.html)"]
pub struct FuchsiaImagepipeSurfaceInstanceCommands {
    pub create_image_pipe_surface_fuchsia: PFN_vkCreateImagePipeSurfaceFUCHSIA,
}
impl FuchsiaImagepipeSurfaceInstanceCommands {
    #[inline]
    pub fn load(loader: &crate::InstanceLoader) -> Option<FuchsiaImagepipeSurfaceInstanceCommands> {
        unsafe {
            Some(FuchsiaImagepipeSurfaceInstanceCommands {
                create_image_pipe_surface_fuchsia: std::mem::transmute(
                    loader.symbol("vkCreateImagePipeSurfaceFUCHSIA")?,
                ),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`FuchsiaImagepipeSurfaceInstanceCommands`](struct.FuchsiaImagepipeSurfaceInstanceCommands.html)"]
pub trait FuchsiaImagepipeSurfaceInstanceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateImagePipeSurfaceFUCHSIA.html) · Instance Command"]
    unsafe fn create_image_pipe_surface_fuchsia(
        &self,
        create_info : & crate :: extensions :: fuchsia_imagepipe_surface :: ImagePipeSurfaceCreateInfoFUCHSIA,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        surface: Option<crate::extensions::khr_surface::SurfaceKHR>,
    ) -> crate::utils::VulkanResult<crate::extensions::khr_surface::SurfaceKHR>;
}
impl FuchsiaImagepipeSurfaceInstanceLoaderExt for crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateImagePipeSurfaceFUCHSIA.html) · Instance Command"]
    unsafe fn create_image_pipe_surface_fuchsia(
        &self,
        create_info : & crate :: extensions :: fuchsia_imagepipe_surface :: ImagePipeSurfaceCreateInfoFUCHSIA,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        surface: Option<crate::extensions::khr_surface::SurfaceKHR>,
    ) -> crate::utils::VulkanResult<crate::extensions::khr_surface::SurfaceKHR> {
        let function = self
            .fuchsia_imagepipe_surface
            .as_ref()
            .expect("`fuchsia_imagepipe_surface` not loaded")
            .create_image_pipe_surface_fuchsia;
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImagePipeSurfaceCreateInfoFUCHSIA.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImagePipeSurfaceCreateInfoFUCHSIA {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::fuchsia_imagepipe_surface::ImagePipeSurfaceCreateFlagsFUCHSIA,
    pub image_pipe_handle: u32,
}
impl ImagePipeSurfaceCreateInfoFUCHSIA {
    #[inline]
    pub fn builder<'a>(self) -> ImagePipeSurfaceCreateInfoFUCHSIABuilder<'a> {
        ImagePipeSurfaceCreateInfoFUCHSIABuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ImagePipeSurfaceCreateInfoFUCHSIA {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ImagePipeSurfaceCreateInfoFUCHSIA")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("image_pipe_handle", &self.image_pipe_handle)
            .finish()
    }
}
impl Default for ImagePipeSurfaceCreateInfoFUCHSIA {
    fn default() -> ImagePipeSurfaceCreateInfoFUCHSIA {
        ImagePipeSurfaceCreateInfoFUCHSIA {
            s_type: crate::vk1_0::StructureType::IMAGEPIPE_SURFACE_CREATE_INFO_FUCHSIA,
            p_next: std::ptr::null(),
            flags: Default::default(),
            image_pipe_handle: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`ImagePipeSurfaceCreateInfoFUCHSIA`](struct.ImagePipeSurfaceCreateInfoFUCHSIA.html)"]
#[repr(transparent)]
pub struct ImagePipeSurfaceCreateInfoFUCHSIABuilder<'a>(
    ImagePipeSurfaceCreateInfoFUCHSIA,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> ImagePipeSurfaceCreateInfoFUCHSIABuilder<'a> {
    #[inline]
    pub fn new() -> ImagePipeSurfaceCreateInfoFUCHSIABuilder<'a> {
        ImagePipeSurfaceCreateInfoFUCHSIABuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn flags(
        mut self,
        flags: crate::extensions::fuchsia_imagepipe_surface::ImagePipeSurfaceCreateFlagsFUCHSIA,
    ) -> Self {
        self.0.flags = flags;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn image_pipe_handle(mut self, image_pipe_handle: u32) -> Self {
        self.0.image_pipe_handle = image_pipe_handle;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> ImagePipeSurfaceCreateInfoFUCHSIA {
        self.0
    }
}
impl<'a> std::fmt::Debug for ImagePipeSurfaceCreateInfoFUCHSIABuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for ImagePipeSurfaceCreateInfoFUCHSIABuilder<'a> {
    type Target = ImagePipeSurfaceCreateInfoFUCHSIA;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ImagePipeSurfaceCreateInfoFUCHSIABuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Flag Bits of [`ImagePipeSurfaceCreateFlagsFUCHSIA`](struct.ImagePipeSurfaceCreateFlagsFUCHSIA.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ImagePipeSurfaceCreateFlagBitsFUCHSIA(pub u32);
impl ImagePipeSurfaceCreateFlagBitsFUCHSIA {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> ImagePipeSurfaceCreateFlagsFUCHSIA {
        ImagePipeSurfaceCreateFlagsFUCHSIA::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for ImagePipeSurfaceCreateFlagBitsFUCHSIA {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            _ => "Unknown enum variant",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImagePipeSurfaceCreateFlagsFUCHSIA.html) · Flags of [`ImagePipeSurfaceCreateFlagBitsFUCHSIA`](struct.ImagePipeSurfaceCreateFlagBitsFUCHSIA.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct ImagePipeSurfaceCreateFlagsFUCHSIA : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
