#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_ANDROID_SURFACE_SPEC_VERSION")]
pub const KHR_ANDROID_SURFACE_SPEC_VERSION: u32 = 6;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_ANDROID_SURFACE_EXTENSION_NAME")]
pub const KHR_ANDROID_SURFACE_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_android_surface");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_ANDROID_SURFACE_KHR: *const std::os::raw::c_char = crate::cstr!("vkCreateAndroidSurfaceKHR");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/ANativeWindow.html) · Basetype"]
pub type ANativeWindow = std::ffi::c_void;
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAndroidSurfaceCreateFlagsKHR.html) · Bitmask of [`AndroidSurfaceCreateFlagBitsKHR`]"] # [doc (alias = "VkAndroidSurfaceCreateFlagsKHR")] # [derive (Default)] # [repr (transparent)] pub struct AndroidSurfaceCreateFlagsKHR : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`AndroidSurfaceCreateFlagsKHR`]"]
#[doc(alias = "VkAndroidSurfaceCreateFlagBitsKHR")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct AndroidSurfaceCreateFlagBitsKHR(pub u32);
impl AndroidSurfaceCreateFlagBitsKHR {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> AndroidSurfaceCreateFlagsKHR {
        AndroidSurfaceCreateFlagsKHR::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for AndroidSurfaceCreateFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateAndroidSurfaceKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateAndroidSurfaceKHR = unsafe extern "system" fn(
    instance: crate::vk1_0::Instance,
    p_create_info: *const crate::extensions::khr_android_surface::AndroidSurfaceCreateInfoKHR,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
    p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAndroidSurfaceCreateInfoKHR.html) · Structure"]
#[doc(alias = "VkAndroidSurfaceCreateInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AndroidSurfaceCreateInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::khr_android_surface::AndroidSurfaceCreateFlagsKHR,
    pub window: *mut crate::extensions::khr_android_surface::ANativeWindow,
}
impl Default for AndroidSurfaceCreateInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::ANDROID_SURFACE_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            flags: Default::default(),
            window: std::ptr::null_mut(),
        }
    }
}
impl std::fmt::Debug for AndroidSurfaceCreateInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AndroidSurfaceCreateInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("window", &self.window)
            .finish()
    }
}
impl AndroidSurfaceCreateInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> AndroidSurfaceCreateInfoKHRBuilder<'a> {
        AndroidSurfaceCreateInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAndroidSurfaceCreateInfoKHR.html) · Builder of [`AndroidSurfaceCreateInfoKHR`]"]
#[repr(transparent)]
pub struct AndroidSurfaceCreateInfoKHRBuilder<'a>(AndroidSurfaceCreateInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> AndroidSurfaceCreateInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> AndroidSurfaceCreateInfoKHRBuilder<'a> {
        AndroidSurfaceCreateInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::khr_android_surface::AndroidSurfaceCreateFlagsKHR) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn window(mut self, window: &'a mut crate::extensions::khr_android_surface::ANativeWindow) -> Self {
        self.0.window = window as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> AndroidSurfaceCreateInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for AndroidSurfaceCreateInfoKHRBuilder<'a> {
    fn default() -> AndroidSurfaceCreateInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for AndroidSurfaceCreateInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for AndroidSurfaceCreateInfoKHRBuilder<'a> {
    type Target = AndroidSurfaceCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for AndroidSurfaceCreateInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<AndroidSurfaceCreateInfoKHR> for AndroidSurfaceCreateInfoKHRBuilder<'_> {}
#[doc = "Provided by [`crate::extensions::khr_android_surface`]"]
impl crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateAndroidSurfaceKHR.html) · Function"]
    #[doc(alias = "vkCreateAndroidSurfaceKHR")]
    pub unsafe fn create_android_surface_khr(
        &self,
        create_info: &crate::extensions::khr_android_surface::AndroidSurfaceCreateInfoKHR,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
    ) -> crate::utils::VulkanResult<crate::extensions::khr_surface::SurfaceKHR> {
        let _function = self.create_android_surface_khr.expect("`create_android_surface_khr` is not loaded");
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
