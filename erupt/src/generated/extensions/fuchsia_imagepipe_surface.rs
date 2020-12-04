#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_FUCHSIA_IMAGEPIPE_SURFACE_SPEC_VERSION")]
pub const FUCHSIA_IMAGEPIPE_SURFACE_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_FUCHSIA_IMAGEPIPE_SURFACE_EXTENSION_NAME")]
pub const FUCHSIA_IMAGEPIPE_SURFACE_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_FUCHSIA_imagepipe_surface");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "FN_CREATE_IMAGE_PIPE_SURFACE_FUCHSIA")]
pub const FN_CREATE_IMAGE_PIPE_SURFACE_FUCHSIA: *const std::os::raw::c_char =
    crate::cstr!("vkCreateImagePipeSurfaceFUCHSIA");
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImagePipeSurfaceCreateFlagsFUCHSIA.html) · Bitmask of [`ImagePipeSurfaceCreateFlagBitsFUCHSIA`]"] # [doc (alias = "VkImagePipeSurfaceCreateFlagsFUCHSIA")] # [derive (Default)] # [repr (transparent)] pub struct ImagePipeSurfaceCreateFlagsFUCHSIA : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`ImagePipeSurfaceCreateFlagsFUCHSIA`]"]
#[doc(alias = "VkImagePipeSurfaceCreateFlagBitsFUCHSIA")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
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
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateImagePipeSurfaceFUCHSIA.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateImagePipeSurfaceFUCHSIA = unsafe extern "system" fn (instance : crate :: vk1_0 :: Instance , p_create_info : * const crate :: extensions :: fuchsia_imagepipe_surface :: ImagePipeSurfaceCreateInfoFUCHSIA , p_allocator : * const crate :: vk1_0 :: AllocationCallbacks , p_surface : * mut crate :: extensions :: khr_surface :: SurfaceKHR) -> crate :: vk1_0 :: Result ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImagePipeSurfaceCreateInfoFUCHSIA.html) · Structure"]
#[doc(alias = "VkImagePipeSurfaceCreateInfoFUCHSIA")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImagePipeSurfaceCreateInfoFUCHSIA {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::fuchsia_imagepipe_surface::ImagePipeSurfaceCreateFlagsFUCHSIA,
    pub image_pipe_handle: *mut std::ffi::c_void,
}
impl Default for ImagePipeSurfaceCreateInfoFUCHSIA {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::IMAGEPIPE_SURFACE_CREATE_INFO_FUCHSIA,
            p_next: std::ptr::null(),
            flags: Default::default(),
            image_pipe_handle: std::ptr::null_mut(),
        }
    }
}
impl std::fmt::Debug for ImagePipeSurfaceCreateInfoFUCHSIA {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ImagePipeSurfaceCreateInfoFUCHSIA")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("image_pipe_handle", &self.image_pipe_handle)
            .finish()
    }
}
impl ImagePipeSurfaceCreateInfoFUCHSIA {
    #[inline]
    pub fn into_builder<'a>(self) -> ImagePipeSurfaceCreateInfoFUCHSIABuilder<'a> {
        ImagePipeSurfaceCreateInfoFUCHSIABuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImagePipeSurfaceCreateInfoFUCHSIA.html) · Builder of [`ImagePipeSurfaceCreateInfoFUCHSIA`]"]
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
    #[inline]
    pub fn flags(
        mut self,
        flags: crate::extensions::fuchsia_imagepipe_surface::ImagePipeSurfaceCreateFlagsFUCHSIA,
    ) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn image_pipe_handle(mut self, image_pipe_handle: *mut std::ffi::c_void) -> Self {
        self.0.image_pipe_handle = image_pipe_handle;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ImagePipeSurfaceCreateInfoFUCHSIA {
        self.0
    }
}
impl<'a> std::default::Default for ImagePipeSurfaceCreateInfoFUCHSIABuilder<'a> {
    fn default() -> ImagePipeSurfaceCreateInfoFUCHSIABuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ImagePipeSurfaceCreateInfoFUCHSIABuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
#[doc = "Provided by [`crate::extensions::fuchsia_imagepipe_surface`]"]
impl crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateImagePipeSurfaceFUCHSIA.html) · Function"]
    #[doc(alias = "vkCreateImagePipeSurfaceFUCHSIA")]
    pub unsafe fn create_image_pipe_surface_fuchsia(
        &self,
        create_info : & crate :: extensions :: fuchsia_imagepipe_surface :: ImagePipeSurfaceCreateInfoFUCHSIA,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        surface: Option<crate::extensions::khr_surface::SurfaceKHR>,
    ) -> crate::utils::VulkanResult<crate::extensions::khr_surface::SurfaceKHR> {
        let _function = self
            .create_image_pipe_surface_fuchsia
            .expect("`create_image_pipe_surface_fuchsia` is not loaded");
        let mut surface = match surface {
            Some(v) => v,
            None => Default::default(),
        };
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
}
