#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_GGP_STREAM_DESCRIPTOR_SURFACE_SPEC_VERSION")]
pub const GGP_STREAM_DESCRIPTOR_SURFACE_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_GGP_STREAM_DESCRIPTOR_SURFACE_EXTENSION_NAME")]
pub const GGP_STREAM_DESCRIPTOR_SURFACE_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_GGP_stream_descriptor_surface");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_STREAM_DESCRIPTOR_SURFACE_GGP: *const std::os::raw::c_char = crate::cstr!("vkCreateStreamDescriptorSurfaceGGP");
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkStreamDescriptorSurfaceCreateFlagsGGP.html) · Bitmask of [`StreamDescriptorSurfaceCreateFlagBitsGGP`]"] # [doc (alias = "VkStreamDescriptorSurfaceCreateFlagsGGP")] # [derive (Default)] # [repr (transparent)] pub struct StreamDescriptorSurfaceCreateFlagsGGP : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`StreamDescriptorSurfaceCreateFlagsGGP`]"]
#[doc(alias = "VkStreamDescriptorSurfaceCreateFlagBitsGGP")]
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
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::ggp_stream_descriptor_surface`]"]
impl crate::vk1_0::StructureType {
    pub const STREAM_DESCRIPTOR_SURFACE_CREATE_INFO_GGP: Self = Self(1000049000);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateStreamDescriptorSurfaceGGP.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateStreamDescriptorSurfaceGGP = unsafe extern "system" fn(instance: crate::vk1_0::Instance, p_create_info: *const crate::extensions::ggp_stream_descriptor_surface::StreamDescriptorSurfaceCreateInfoGGP, p_allocator: *const crate::vk1_0::AllocationCallbacks, p_surface: *mut crate::extensions::khr_surface::SurfaceKHR) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkStreamDescriptorSurfaceCreateInfoGGP.html) · Structure"]
#[doc(alias = "VkStreamDescriptorSurfaceCreateInfoGGP")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StreamDescriptorSurfaceCreateInfoGGP {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::ggp_stream_descriptor_surface::StreamDescriptorSurfaceCreateFlagsGGP,
    pub stream_descriptor: u32,
}
impl StreamDescriptorSurfaceCreateInfoGGP {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::STREAM_DESCRIPTOR_SURFACE_CREATE_INFO_GGP;
}
impl Default for StreamDescriptorSurfaceCreateInfoGGP {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), flags: Default::default(), stream_descriptor: Default::default() }
    }
}
impl std::fmt::Debug for StreamDescriptorSurfaceCreateInfoGGP {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StreamDescriptorSurfaceCreateInfoGGP").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("stream_descriptor", &self.stream_descriptor).finish()
    }
}
impl StreamDescriptorSurfaceCreateInfoGGP {
    #[inline]
    pub fn into_builder<'a>(self) -> StreamDescriptorSurfaceCreateInfoGGPBuilder<'a> {
        StreamDescriptorSurfaceCreateInfoGGPBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkStreamDescriptorSurfaceCreateInfoGGP.html) · Builder of [`StreamDescriptorSurfaceCreateInfoGGP`]"]
#[repr(transparent)]
pub struct StreamDescriptorSurfaceCreateInfoGGPBuilder<'a>(StreamDescriptorSurfaceCreateInfoGGP, std::marker::PhantomData<&'a ()>);
impl<'a> StreamDescriptorSurfaceCreateInfoGGPBuilder<'a> {
    #[inline]
    pub fn new() -> StreamDescriptorSurfaceCreateInfoGGPBuilder<'a> {
        StreamDescriptorSurfaceCreateInfoGGPBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn flags(mut self, flags: crate::extensions::ggp_stream_descriptor_surface::StreamDescriptorSurfaceCreateFlagsGGP) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn stream_descriptor(mut self, stream_descriptor: u32) -> Self {
        self.0.stream_descriptor = stream_descriptor as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> StreamDescriptorSurfaceCreateInfoGGP {
        self.0
    }
}
impl<'a> std::default::Default for StreamDescriptorSurfaceCreateInfoGGPBuilder<'a> {
    fn default() -> StreamDescriptorSurfaceCreateInfoGGPBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for StreamDescriptorSurfaceCreateInfoGGPBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
#[doc = "Provided by [`crate::extensions::ggp_stream_descriptor_surface`]"]
impl crate::InstanceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateStreamDescriptorSurfaceGGP.html) · Function"]
    #[doc(alias = "vkCreateStreamDescriptorSurfaceGGP")]
    pub unsafe fn create_stream_descriptor_surface_ggp(&self, create_info: &crate::extensions::ggp_stream_descriptor_surface::StreamDescriptorSurfaceCreateInfoGGP, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> crate::utils::VulkanResult<crate::extensions::khr_surface::SurfaceKHR> {
        let _function = self.create_stream_descriptor_surface_ggp.expect(crate::NOT_LOADED_MESSAGE);
        let mut surface = Default::default();
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
