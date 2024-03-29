// DO NOT EDIT: @generated by erupt's generator
///<s>Vulkan Manual Page</s> · Constant
#[doc(alias = "VK_EXT_METAL_SURFACE_SPEC_VERSION")]
pub const EXT_METAL_SURFACE_SPEC_VERSION: u32 = 1;
///<s>Vulkan Manual Page</s> · Constant
#[doc(alias = "VK_EXT_METAL_SURFACE_EXTENSION_NAME")]
pub const EXT_METAL_SURFACE_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!(
    "VK_EXT_metal_surface"
);
///<s>Vulkan Manual Page</s> · Constant
pub const FN_CREATE_METAL_SURFACE_EXT: *const std::os::raw::c_char = crate::cstr!(
    "vkCreateMetalSurfaceEXT"
);
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/CAMetalLayer.html) · Basetype
pub type CAMetalLayer = std::ffi::c_void;
bitflags::bitflags! {
    #[doc =
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMetalSurfaceCreateFlagsEXT.html) · Bitmask of [`MetalSurfaceCreateFlagBitsEXT`]"]
    #[doc(alias = "VkMetalSurfaceCreateFlagsEXT")] #[derive(Default)]
    #[repr(transparent)] pub struct MetalSurfaceCreateFlagsEXT : u32 {
    #[cfg(empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0; }
}
///<s>Vulkan Manual Page</s> · Bits enum of [`MetalSurfaceCreateFlagsEXT`]
#[doc(alias = "VkMetalSurfaceCreateFlagBitsEXT")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct MetalSurfaceCreateFlagBitsEXT(pub u32);
impl MetalSurfaceCreateFlagBitsEXT {
    #[inline]
    ///Converts this enum variant to the corresponding bitmask
    pub const fn bitmask(&self) -> MetalSurfaceCreateFlagsEXT {
        MetalSurfaceCreateFlagsEXT::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for MetalSurfaceCreateFlagBitsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(
            match self {
                _ => "(unknown variant)",
            },
        )
    }
}
///Provided by [`crate::extensions::ext_metal_surface`]
impl crate::vk1_0::StructureType {
    pub const METAL_SURFACE_CREATE_INFO_EXT: Self = Self(1000217000);
}
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateMetalSurfaceEXT.html) · Function
#[allow(non_camel_case_types)]
pub type PFN_vkCreateMetalSurfaceEXT = unsafe extern "system" fn(
    instance: crate::vk1_0::Instance,
    p_create_info: *const crate::extensions::ext_metal_surface::MetalSurfaceCreateInfoEXT,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
    p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
) -> crate::vk1_0::Result;
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMetalSurfaceCreateInfoEXT.html) · Structure
#[doc(alias = "VkMetalSurfaceCreateInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MetalSurfaceCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::ext_metal_surface::MetalSurfaceCreateFlagsEXT,
    pub p_layer: *const crate::extensions::ext_metal_surface::CAMetalLayer,
}
impl MetalSurfaceCreateInfoEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::METAL_SURFACE_CREATE_INFO_EXT;
}
impl Default for MetalSurfaceCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: Self::STRUCTURE_TYPE,
            p_next: std::ptr::null(),
            flags: Default::default(),
            p_layer: std::ptr::null(),
        }
    }
}
impl std::fmt::Debug for MetalSurfaceCreateInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MetalSurfaceCreateInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("p_layer", &self.p_layer)
            .finish()
    }
}
impl MetalSurfaceCreateInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> MetalSurfaceCreateInfoEXTBuilder<'a> {
        MetalSurfaceCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMetalSurfaceCreateInfoEXT.html) · Builder of [`MetalSurfaceCreateInfoEXT`]
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
    #[inline]
    #[must_use]
    pub fn flags(
        mut self,
        flags: crate::extensions::ext_metal_surface::MetalSurfaceCreateFlagsEXT,
    ) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn layer(
        mut self,
        layer: &'a crate::extensions::ext_metal_surface::CAMetalLayer,
    ) -> Self {
        self.0.p_layer = layer as _;
        self
    }
    #[inline]
    /// Discards all lifetime information.
    /// Use the `Deref` and `DerefMut` implementations if possible.
    pub fn build_dangling(self) -> MetalSurfaceCreateInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for MetalSurfaceCreateInfoEXTBuilder<'a> {
    fn default() -> MetalSurfaceCreateInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for MetalSurfaceCreateInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
///Provided by [`crate::extensions::ext_metal_surface`]
impl crate::InstanceLoader {
    #[inline]
    #[track_caller]
    ///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateMetalSurfaceEXT.html) · Function
    #[doc(alias = "vkCreateMetalSurfaceEXT")]
    pub unsafe fn create_metal_surface_ext(
        &self,
        create_info: &crate::extensions::ext_metal_surface::MetalSurfaceCreateInfoEXT,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
    ) -> crate::utils::VulkanResult<crate::extensions::khr_surface::SurfaceKHR> {
        let _function = self.create_metal_surface_ext.expect(crate::NOT_LOADED_MESSAGE);
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
