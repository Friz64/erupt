#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NN_VI_SURFACE_SPEC_VERSION")]
pub const NN_VI_SURFACE_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NN_VI_SURFACE_EXTENSION_NAME")]
pub const NN_VI_SURFACE_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_NN_vi_surface");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_VI_SURFACE_NN: *const std::os::raw::c_char = crate::cstr!("vkCreateViSurfaceNN");
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkViSurfaceCreateFlagsNN.html) · Bitmask of [`ViSurfaceCreateFlagBitsNN`]"] # [doc (alias = "VkViSurfaceCreateFlagsNN")] # [derive (Default)] # [repr (transparent)] pub struct ViSurfaceCreateFlagsNN : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`ViSurfaceCreateFlagsNN`]"]
#[doc(alias = "VkViSurfaceCreateFlagBitsNN")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
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
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateViSurfaceNN.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateViSurfaceNN = unsafe extern "system" fn(
    instance: crate::vk1_0::Instance,
    p_create_info: *const crate::extensions::nn_vi_surface::ViSurfaceCreateInfoNN,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
    p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkViSurfaceCreateInfoNN.html) · Structure"]
#[doc(alias = "VkViSurfaceCreateInfoNN")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ViSurfaceCreateInfoNN {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::nn_vi_surface::ViSurfaceCreateFlagsNN,
    pub window: *mut std::ffi::c_void,
}
impl Default for ViSurfaceCreateInfoNN {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::VI_SURFACE_CREATE_INFO_NN,
            p_next: std::ptr::null(),
            flags: Default::default(),
            window: std::ptr::null_mut(),
        }
    }
}
impl std::fmt::Debug for ViSurfaceCreateInfoNN {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ViSurfaceCreateInfoNN")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("window", &self.window)
            .finish()
    }
}
impl ViSurfaceCreateInfoNN {
    #[inline]
    pub fn into_builder<'a>(self) -> ViSurfaceCreateInfoNNBuilder<'a> {
        ViSurfaceCreateInfoNNBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkViSurfaceCreateInfoNN.html) · Builder of [`ViSurfaceCreateInfoNN`]"]
#[repr(transparent)]
pub struct ViSurfaceCreateInfoNNBuilder<'a>(ViSurfaceCreateInfoNN, std::marker::PhantomData<&'a ()>);
impl<'a> ViSurfaceCreateInfoNNBuilder<'a> {
    #[inline]
    pub fn new() -> ViSurfaceCreateInfoNNBuilder<'a> {
        ViSurfaceCreateInfoNNBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::nn_vi_surface::ViSurfaceCreateFlagsNN) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn window(mut self, window: *mut std::ffi::c_void) -> Self {
        self.0.window = window;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ViSurfaceCreateInfoNN {
        self.0
    }
}
impl<'a> std::default::Default for ViSurfaceCreateInfoNNBuilder<'a> {
    fn default() -> ViSurfaceCreateInfoNNBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ViSurfaceCreateInfoNNBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
#[doc = "Provided by [`crate::extensions::nn_vi_surface`]"]
impl crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateViSurfaceNN.html) · Function"]
    #[doc(alias = "vkCreateViSurfaceNN")]
    pub unsafe fn create_vi_surface_nn(&self, create_info: &crate::extensions::nn_vi_surface::ViSurfaceCreateInfoNN, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> crate::utils::VulkanResult<crate::extensions::khr_surface::SurfaceKHR> {
        let _function = self.create_vi_surface_nn.expect("`create_vi_surface_nn` is not loaded");
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
