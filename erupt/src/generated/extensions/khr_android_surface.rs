# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_KHR_android_surface.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_ANDROID_SURFACE_SPEC_VERSION: u32 = 6;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_ANDROID_SURFACE_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_android_surface");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateAndroidSurfaceKHR.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateAndroidSurfaceKHR = unsafe extern "system" fn(
    instance: crate::vk1_0::Instance,
    p_create_info: *const crate::extensions::khr_android_surface::AndroidSurfaceCreateInfoKHR,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
    p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
) -> crate::vk1_0::Result;
#[doc = "Provides Instance Commands for [`KhrAndroidSurfaceInstanceLoaderExt`](trait.KhrAndroidSurfaceInstanceLoaderExt.html)"]
pub struct KhrAndroidSurfaceInstanceCommands {
    pub create_android_surface_khr: PFN_vkCreateAndroidSurfaceKHR,
}
impl KhrAndroidSurfaceInstanceCommands {
    #[inline]
    pub fn load(loader: &crate::InstanceLoader) -> Option<KhrAndroidSurfaceInstanceCommands> {
        unsafe {
            Some(KhrAndroidSurfaceInstanceCommands {
                create_android_surface_khr: std::mem::transmute(
                    loader.symbol("vkCreateAndroidSurfaceKHR")?,
                ),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`KhrAndroidSurfaceInstanceCommands`](struct.KhrAndroidSurfaceInstanceCommands.html)"]
pub trait KhrAndroidSurfaceInstanceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateAndroidSurfaceKHR.html) · Instance Command"]
    unsafe fn create_android_surface_khr(
        &self,
        create_info: &crate::extensions::khr_android_surface::AndroidSurfaceCreateInfoKHR,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        surface: Option<crate::extensions::khr_surface::SurfaceKHR>,
    ) -> crate::utils::VulkanResult<crate::extensions::khr_surface::SurfaceKHR>;
}
impl KhrAndroidSurfaceInstanceLoaderExt for crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateAndroidSurfaceKHR.html) · Instance Command"]
    unsafe fn create_android_surface_khr(
        &self,
        create_info: &crate::extensions::khr_android_surface::AndroidSurfaceCreateInfoKHR,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        surface: Option<crate::extensions::khr_surface::SurfaceKHR>,
    ) -> crate::utils::VulkanResult<crate::extensions::khr_surface::SurfaceKHR> {
        let function = self
            .khr_android_surface
            .as_ref()
            .expect("`khr_android_surface` not loaded")
            .create_android_surface_khr;
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAndroidSurfaceCreateInfoKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AndroidSurfaceCreateInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::khr_android_surface::AndroidSurfaceCreateFlagsKHR,
    pub window: *mut std::ffi::c_void,
}
impl AndroidSurfaceCreateInfoKHR {
    #[inline]
    pub fn builder<'a>(self) -> AndroidSurfaceCreateInfoKHRBuilder<'a> {
        AndroidSurfaceCreateInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for AndroidSurfaceCreateInfoKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("AndroidSurfaceCreateInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("window", &self.window)
            .finish()
    }
}
impl Default for AndroidSurfaceCreateInfoKHR {
    fn default() -> AndroidSurfaceCreateInfoKHR {
        AndroidSurfaceCreateInfoKHR {
            s_type: crate::vk1_0::StructureType::ANDROID_SURFACE_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            flags: Default::default(),
            window: std::ptr::null_mut(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`AndroidSurfaceCreateInfoKHR`](struct.AndroidSurfaceCreateInfoKHR.html)"]
#[repr(transparent)]
pub struct AndroidSurfaceCreateInfoKHRBuilder<'a>(
    AndroidSurfaceCreateInfoKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> AndroidSurfaceCreateInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> AndroidSurfaceCreateInfoKHRBuilder<'a> {
        AndroidSurfaceCreateInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn flags(
        mut self,
        flags: crate::extensions::khr_android_surface::AndroidSurfaceCreateFlagsKHR,
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
    pub unsafe fn discard(self) -> AndroidSurfaceCreateInfoKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for AndroidSurfaceCreateInfoKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "<s>Vulkan Manual Page</s> · Flag Bits of [`AndroidSurfaceCreateFlagsKHR`](struct.AndroidSurfaceCreateFlagsKHR.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
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
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            _ => "Unknown enum variant",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAndroidSurfaceCreateFlagsKHR.html) · Flags of [`AndroidSurfaceCreateFlagBitsKHR`](struct.AndroidSurfaceCreateFlagBitsKHR.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct AndroidSurfaceCreateFlagsKHR : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
