# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_GGP_stream_descriptor_surface.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const GGP_STREAM_DESCRIPTOR_SURFACE_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const GGP_STREAM_DESCRIPTOR_SURFACE_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_GGP_stream_descriptor_surface");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateStreamDescriptorSurfaceGGP.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateStreamDescriptorSurfaceGGP = unsafe extern "system" fn ( instance : crate :: vk1_0 :: Instance , p_create_info : * const crate :: extensions :: ggp_stream_descriptor_surface :: StreamDescriptorSurfaceCreateInfoGGP , p_allocator : * const crate :: vk1_0 :: AllocationCallbacks , p_surface : * mut crate :: extensions :: khr_surface :: SurfaceKHR , ) -> crate :: vk1_0 :: Result ;
#[doc = "Provides Instance Commands for [`GgpStreamDescriptorSurfaceInstanceLoaderExt`](trait.GgpStreamDescriptorSurfaceInstanceLoaderExt.html)"]
pub struct GgpStreamDescriptorSurfaceInstanceCommands {
    pub create_stream_descriptor_surface_ggp: Option<PFN_vkCreateStreamDescriptorSurfaceGGP>,
}
impl GgpStreamDescriptorSurfaceInstanceCommands {
    #[inline]
    pub fn load(
        loader: &crate::InstanceLoader,
    ) -> Option<GgpStreamDescriptorSurfaceInstanceCommands> {
        unsafe {
            let mut success = false;
            let table = GgpStreamDescriptorSurfaceInstanceCommands {
                create_stream_descriptor_surface_ggp: std::mem::transmute({
                    let symbol = loader.symbol("vkCreateStreamDescriptorSurfaceGGP");
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
fn instance_commands(
    loader: &crate::InstanceLoader,
) -> &GgpStreamDescriptorSurfaceInstanceCommands {
    loader
        .ggp_stream_descriptor_surface
        .as_ref()
        .expect("`ggp_stream_descriptor_surface` not loaded")
}
#[doc = "Provides high level command wrappers for [`GgpStreamDescriptorSurfaceInstanceCommands`](struct.GgpStreamDescriptorSurfaceInstanceCommands.html)"]
pub trait GgpStreamDescriptorSurfaceInstanceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateStreamDescriptorSurfaceGGP.html) · Instance Command"]
    unsafe fn create_stream_descriptor_surface_ggp(
        &self,
        create_info : & crate :: extensions :: ggp_stream_descriptor_surface :: StreamDescriptorSurfaceCreateInfoGGP,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        surface: Option<crate::extensions::khr_surface::SurfaceKHR>,
    ) -> crate::utils::VulkanResult<crate::extensions::khr_surface::SurfaceKHR>;
}
impl GgpStreamDescriptorSurfaceInstanceLoaderExt for crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateStreamDescriptorSurfaceGGP.html) · Instance Command"]
    unsafe fn create_stream_descriptor_surface_ggp(
        &self,
        create_info : & crate :: extensions :: ggp_stream_descriptor_surface :: StreamDescriptorSurfaceCreateInfoGGP,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        surface: Option<crate::extensions::khr_surface::SurfaceKHR>,
    ) -> crate::utils::VulkanResult<crate::extensions::khr_surface::SurfaceKHR> {
        let function = instance_commands(self)
            .create_stream_descriptor_surface_ggp
            .as_ref()
            .expect("`create_stream_descriptor_surface_ggp` not available");
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkStreamDescriptorSurfaceCreateInfoGGP.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StreamDescriptorSurfaceCreateInfoGGP {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags:
        crate::extensions::ggp_stream_descriptor_surface::StreamDescriptorSurfaceCreateFlagsGGP,
    pub stream_descriptor: u32,
}
impl StreamDescriptorSurfaceCreateInfoGGP {
    #[inline]
    pub fn builder<'a>(self) -> StreamDescriptorSurfaceCreateInfoGGPBuilder<'a> {
        StreamDescriptorSurfaceCreateInfoGGPBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for StreamDescriptorSurfaceCreateInfoGGP {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("StreamDescriptorSurfaceCreateInfoGGP")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("stream_descriptor", &self.stream_descriptor)
            .finish()
    }
}
impl Default for StreamDescriptorSurfaceCreateInfoGGP {
    fn default() -> StreamDescriptorSurfaceCreateInfoGGP {
        StreamDescriptorSurfaceCreateInfoGGP {
            s_type: crate::vk1_0::StructureType::STREAM_DESCRIPTOR_SURFACE_CREATE_INFO_GGP,
            p_next: std::ptr::null(),
            flags: Default::default(),
            stream_descriptor: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkStreamDescriptorSurfaceCreateInfoGGP.html) · Builder of [`StreamDescriptorSurfaceCreateInfoGGP`](struct.StreamDescriptorSurfaceCreateInfoGGP.html)"]
#[repr(transparent)]
pub struct StreamDescriptorSurfaceCreateInfoGGPBuilder<'a>(
    StreamDescriptorSurfaceCreateInfoGGP,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> StreamDescriptorSurfaceCreateInfoGGPBuilder<'a> {
    #[inline]
    pub fn new() -> StreamDescriptorSurfaceCreateInfoGGPBuilder<'a> {
        StreamDescriptorSurfaceCreateInfoGGPBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn flags(
        mut self,
        flags : crate :: extensions :: ggp_stream_descriptor_surface :: StreamDescriptorSurfaceCreateFlagsGGP,
    ) -> Self {
        self.0.flags = flags;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn stream_descriptor(mut self, stream_descriptor: u32) -> Self {
        self.0.stream_descriptor = stream_descriptor;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> StreamDescriptorSurfaceCreateInfoGGP {
        self.0
    }
}
impl<'a> std::fmt::Debug for StreamDescriptorSurfaceCreateInfoGGPBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for StreamDescriptorSurfaceCreateInfoGGPBuilder<'a> {
    type Target = StreamDescriptorSurfaceCreateInfoGGP;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StreamDescriptorSurfaceCreateInfoGGPBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Flag Bits of [`StreamDescriptorSurfaceCreateFlagsGGP`](struct.StreamDescriptorSurfaceCreateFlagsGGP.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct StreamDescriptorSurfaceCreateFlagBitsGGP(pub u32);
impl StreamDescriptorSurfaceCreateFlagBitsGGP {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> StreamDescriptorSurfaceCreateFlagsGGP {
        StreamDescriptorSurfaceCreateFlagsGGP::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for StreamDescriptorSurfaceCreateFlagBitsGGP {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            _ => "(unknown)",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkStreamDescriptorSurfaceCreateFlagsGGP.html) · Flags of [`StreamDescriptorSurfaceCreateFlagBitsGGP`](struct.StreamDescriptorSurfaceCreateFlagBitsGGP.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct StreamDescriptorSurfaceCreateFlagsGGP : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
