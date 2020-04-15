# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_NN_vi_surface.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NN_VI_SURFACE_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NN_VI_SURFACE_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_NN_vi_surface");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateViSurfaceNN.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateViSurfaceNN = unsafe extern "system" fn(
    instance: crate::vk1_0::Instance,
    p_create_info: *const crate::extensions::nn_vi_surface::ViSurfaceCreateInfoNN,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
    p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
) -> crate::vk1_0::Result;
#[doc = "Provides Instance Commands for [`NnViSurfaceInstanceLoaderExt`](trait.NnViSurfaceInstanceLoaderExt.html)"]
pub struct NnViSurfaceInstanceCommands {
    pub create_vi_surface_nn: PFN_vkCreateViSurfaceNN,
}
impl NnViSurfaceInstanceCommands {
    #[inline]
    pub fn load(loader: &crate::InstanceLoader) -> Option<NnViSurfaceInstanceCommands> {
        unsafe {
            Some(NnViSurfaceInstanceCommands {
                create_vi_surface_nn: std::mem::transmute(loader.symbol("vkCreateViSurfaceNN")?),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`NnViSurfaceInstanceCommands`](struct.NnViSurfaceInstanceCommands.html)"]
pub trait NnViSurfaceInstanceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateViSurfaceNN.html) · Instance Command"]
    unsafe fn create_vi_surface_nn(
        &self,
        create_info: &crate::extensions::nn_vi_surface::ViSurfaceCreateInfoNN,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        surface: Option<crate::extensions::khr_surface::SurfaceKHR>,
    ) -> crate::utils::VulkanResult<crate::extensions::khr_surface::SurfaceKHR>;
}
impl NnViSurfaceInstanceLoaderExt for crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateViSurfaceNN.html) · Instance Command"]
    unsafe fn create_vi_surface_nn(
        &self,
        create_info: &crate::extensions::nn_vi_surface::ViSurfaceCreateInfoNN,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        surface: Option<crate::extensions::khr_surface::SurfaceKHR>,
    ) -> crate::utils::VulkanResult<crate::extensions::khr_surface::SurfaceKHR> {
        let function = self
            .nn_vi_surface
            .as_ref()
            .expect("`nn_vi_surface` not loaded")
            .create_vi_surface_nn;
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkViSurfaceCreateInfoNN.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ViSurfaceCreateInfoNN {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::nn_vi_surface::ViSurfaceCreateFlagsNN,
    pub window: *mut std::ffi::c_void,
}
impl ViSurfaceCreateInfoNN {
    #[inline]
    pub fn builder<'a>(self) -> ViSurfaceCreateInfoNNBuilder<'a> {
        ViSurfaceCreateInfoNNBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ViSurfaceCreateInfoNN {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ViSurfaceCreateInfoNN")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("window", &self.window)
            .finish()
    }
}
impl Default for ViSurfaceCreateInfoNN {
    fn default() -> ViSurfaceCreateInfoNN {
        ViSurfaceCreateInfoNN {
            s_type: crate::vk1_0::StructureType::VI_SURFACE_CREATE_INFO_NN,
            p_next: std::ptr::null(),
            flags: Default::default(),
            window: std::ptr::null_mut(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`ViSurfaceCreateInfoNN`](struct.ViSurfaceCreateInfoNN.html)"]
#[repr(transparent)]
pub struct ViSurfaceCreateInfoNNBuilder<'a>(
    ViSurfaceCreateInfoNN,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> ViSurfaceCreateInfoNNBuilder<'a> {
    #[inline]
    pub fn new() -> ViSurfaceCreateInfoNNBuilder<'a> {
        ViSurfaceCreateInfoNNBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn flags(
        mut self,
        flags: crate::extensions::nn_vi_surface::ViSurfaceCreateFlagsNN,
    ) -> Self {
        self.0.flags = flags;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn window(mut self, window: &'a mut std::ffi::c_void) -> Self {
        self.0.window = window;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> ViSurfaceCreateInfoNN {
        self.0
    }
}
impl<'a> std::fmt::Debug for ViSurfaceCreateInfoNNBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for ViSurfaceCreateInfoNNBuilder<'a> {
    type Target = ViSurfaceCreateInfoNN;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ViSurfaceCreateInfoNNBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Flag Bits of [`ViSurfaceCreateFlagsNN`](struct.ViSurfaceCreateFlagsNN.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ViSurfaceCreateFlagBitsNN(pub u32);
impl ViSurfaceCreateFlagBitsNN {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> ViSurfaceCreateFlagsNN {
        ViSurfaceCreateFlagsNN::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for ViSurfaceCreateFlagBitsNN {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            _ => "Unknown enum variant",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkViSurfaceCreateFlagsNN.html) · Flags of [`ViSurfaceCreateFlagBitsNN`](struct.ViSurfaceCreateFlagBitsNN.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct ViSurfaceCreateFlagsNN : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
