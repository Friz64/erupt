# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_KHR_get_surface_capabilities2.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_GET_SURFACE_CAPABILITIES_2_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_GET_SURFACE_CAPABILITIES_2_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_get_surface_capabilities2");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilities2KHR.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR = unsafe extern "system" fn ( physical_device : crate :: vk1_0 :: PhysicalDevice , p_surface_info : * const crate :: extensions :: khr_get_surface_capabilities2 :: PhysicalDeviceSurfaceInfo2KHR , p_surface_capabilities : * mut crate :: extensions :: khr_get_surface_capabilities2 :: SurfaceCapabilities2KHR , ) -> crate :: vk1_0 :: Result ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfaceFormats2KHR.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSurfaceFormats2KHR = unsafe extern "system" fn ( physical_device : crate :: vk1_0 :: PhysicalDevice , p_surface_info : * const crate :: extensions :: khr_get_surface_capabilities2 :: PhysicalDeviceSurfaceInfo2KHR , p_surface_format_count : * mut u32 , p_surface_formats : * mut crate :: extensions :: khr_get_surface_capabilities2 :: SurfaceFormat2KHR , ) -> crate :: vk1_0 :: Result ;
#[doc = "Provides Instance Commands for [`KhrGetSurfaceCapabilities2InstanceLoaderExt`](trait.KhrGetSurfaceCapabilities2InstanceLoaderExt.html)"]
pub struct KhrGetSurfaceCapabilities2InstanceCommands {
    pub get_physical_device_surface_capabilities2_khr:
        Option<PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR>,
    pub get_physical_device_surface_formats2_khr: Option<PFN_vkGetPhysicalDeviceSurfaceFormats2KHR>,
}
impl KhrGetSurfaceCapabilities2InstanceCommands {
    #[inline]
    pub fn load(
        loader: &crate::InstanceLoader,
    ) -> Option<KhrGetSurfaceCapabilities2InstanceCommands> {
        unsafe {
            let mut success = false;
            let table = KhrGetSurfaceCapabilities2InstanceCommands {
                get_physical_device_surface_capabilities2_khr: std::mem::transmute({
                    let symbol = loader.symbol("vkGetPhysicalDeviceSurfaceCapabilities2KHR");
                    success |= symbol.is_some();
                    symbol
                }),
                get_physical_device_surface_formats2_khr: std::mem::transmute({
                    let symbol = loader.symbol("vkGetPhysicalDeviceSurfaceFormats2KHR");
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
fn instance_commands(
    loader: &crate::InstanceLoader,
) -> &KhrGetSurfaceCapabilities2InstanceCommands {
    loader
        .khr_get_surface_capabilities2
        .as_ref()
        .expect("`khr_get_surface_capabilities2` not loaded")
}
#[doc = "Provides high level command wrappers for [`KhrGetSurfaceCapabilities2InstanceCommands`](struct.KhrGetSurfaceCapabilities2InstanceCommands.html)"]
pub trait KhrGetSurfaceCapabilities2InstanceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilities2KHR.html) · Instance Command"]
    unsafe fn get_physical_device_surface_capabilities2_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        surface_info : & crate :: extensions :: khr_get_surface_capabilities2 :: PhysicalDeviceSurfaceInfo2KHR,
        surface_capabilities: Option<
            crate::extensions::khr_get_surface_capabilities2::SurfaceCapabilities2KHR,
        >,
    ) -> crate::utils::VulkanResult<
        crate::extensions::khr_get_surface_capabilities2::SurfaceCapabilities2KHR,
    >;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfaceFormats2KHR.html) · Instance Command"]
    unsafe fn get_physical_device_surface_formats2_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        surface_info : & crate :: extensions :: khr_get_surface_capabilities2 :: PhysicalDeviceSurfaceInfo2KHR,
        surface_format_count: Option<u32>,
    ) -> crate::utils::VulkanResult<
        Vec<crate::extensions::khr_get_surface_capabilities2::SurfaceFormat2KHR>,
    >;
}
impl KhrGetSurfaceCapabilities2InstanceLoaderExt for crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilities2KHR.html) · Instance Command"]
    unsafe fn get_physical_device_surface_capabilities2_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        surface_info : & crate :: extensions :: khr_get_surface_capabilities2 :: PhysicalDeviceSurfaceInfo2KHR,
        surface_capabilities: Option<
            crate::extensions::khr_get_surface_capabilities2::SurfaceCapabilities2KHR,
        >,
    ) -> crate::utils::VulkanResult<
        crate::extensions::khr_get_surface_capabilities2::SurfaceCapabilities2KHR,
    > {
        let function = instance_commands(self)
            .get_physical_device_surface_capabilities2_khr
            .as_ref()
            .expect("`get_physical_device_surface_capabilities2_khr` not available");
        let mut surface_capabilities = surface_capabilities.unwrap_or_else(|| Default::default());
        let _val = function(physical_device, surface_info, &mut surface_capabilities);
        crate::utils::VulkanResult::new(_val, surface_capabilities)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfaceFormats2KHR.html) · Instance Command"]
    unsafe fn get_physical_device_surface_formats2_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        surface_info : & crate :: extensions :: khr_get_surface_capabilities2 :: PhysicalDeviceSurfaceInfo2KHR,
        surface_format_count: Option<u32>,
    ) -> crate::utils::VulkanResult<
        Vec<crate::extensions::khr_get_surface_capabilities2::SurfaceFormat2KHR>,
    > {
        let function = instance_commands(self)
            .get_physical_device_surface_formats2_khr
            .as_ref()
            .expect("`get_physical_device_surface_formats2_khr` not available");
        let mut surface_format_count = surface_format_count.unwrap_or_else(|| {
            let mut val = Default::default();
            function(
                physical_device,
                surface_info,
                &mut val,
                std::ptr::null_mut(),
            );
            val
        });
        let mut surface_formats = vec![Default::default(); surface_format_count as _];
        let _val = function(
            physical_device,
            surface_info,
            &mut surface_format_count,
            surface_formats.as_mut_ptr(),
        );
        crate::utils::VulkanResult::new(_val, surface_formats)
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSurfaceInfo2KHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceSurfaceInfo2KHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub surface: crate::extensions::khr_surface::SurfaceKHR,
}
impl PhysicalDeviceSurfaceInfo2KHR {
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceSurfaceInfo2KHRBuilder<'a> {
        PhysicalDeviceSurfaceInfo2KHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceSurfaceInfo2KHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceSurfaceInfo2KHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("surface", &self.surface)
            .finish()
    }
}
impl Default for PhysicalDeviceSurfaceInfo2KHR {
    fn default() -> PhysicalDeviceSurfaceInfo2KHR {
        PhysicalDeviceSurfaceInfo2KHR {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_SURFACE_INFO_2_KHR,
            p_next: std::ptr::null(),
            surface: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSurfaceInfo2KHR.html) · Builder of [`PhysicalDeviceSurfaceInfo2KHR`](struct.PhysicalDeviceSurfaceInfo2KHR.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceSurfaceInfo2KHRBuilder<'a>(
    PhysicalDeviceSurfaceInfo2KHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceSurfaceInfo2KHRBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceSurfaceInfo2KHRBuilder<'a> {
        PhysicalDeviceSurfaceInfo2KHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn surface(mut self, surface: crate::extensions::khr_surface::SurfaceKHR) -> Self {
        self.0.surface = surface;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceSurfaceInfo2KHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceSurfaceInfo2KHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceSurfaceInfo2KHRBuilder<'a> {
    type Target = PhysicalDeviceSurfaceInfo2KHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceSurfaceInfo2KHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceCapabilities2KHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SurfaceCapabilities2KHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub surface_capabilities: crate::extensions::khr_surface::SurfaceCapabilitiesKHR,
}
impl SurfaceCapabilities2KHR {
    #[inline]
    pub fn builder<'a>(self) -> SurfaceCapabilities2KHRBuilder<'a> {
        SurfaceCapabilities2KHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for SurfaceCapabilities2KHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("SurfaceCapabilities2KHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("surface_capabilities", &self.surface_capabilities)
            .finish()
    }
}
impl Default for SurfaceCapabilities2KHR {
    fn default() -> SurfaceCapabilities2KHR {
        SurfaceCapabilities2KHR {
            s_type: crate::vk1_0::StructureType::SURFACE_CAPABILITIES_2_KHR,
            p_next: std::ptr::null_mut(),
            surface_capabilities: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceCapabilities2KHR.html) · Builder of [`SurfaceCapabilities2KHR`](struct.SurfaceCapabilities2KHR.html)"]
#[repr(transparent)]
pub struct SurfaceCapabilities2KHRBuilder<'a>(
    SurfaceCapabilities2KHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> SurfaceCapabilities2KHRBuilder<'a> {
    #[inline]
    pub fn new() -> SurfaceCapabilities2KHRBuilder<'a> {
        SurfaceCapabilities2KHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn surface_capabilities(
        mut self,
        surface_capabilities: crate::extensions::khr_surface::SurfaceCapabilitiesKHR,
    ) -> Self {
        self.0.surface_capabilities = surface_capabilities;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> SurfaceCapabilities2KHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for SurfaceCapabilities2KHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for SurfaceCapabilities2KHRBuilder<'a> {
    type Target = SurfaceCapabilities2KHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SurfaceCapabilities2KHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceFormat2KHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SurfaceFormat2KHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub surface_format: crate::extensions::khr_surface::SurfaceFormatKHR,
}
impl SurfaceFormat2KHR {
    #[inline]
    pub fn builder<'a>(self) -> SurfaceFormat2KHRBuilder<'a> {
        SurfaceFormat2KHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for SurfaceFormat2KHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("SurfaceFormat2KHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("surface_format", &self.surface_format)
            .finish()
    }
}
impl Default for SurfaceFormat2KHR {
    fn default() -> SurfaceFormat2KHR {
        SurfaceFormat2KHR {
            s_type: crate::vk1_0::StructureType::SURFACE_FORMAT_2_KHR,
            p_next: std::ptr::null_mut(),
            surface_format: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceFormat2KHR.html) · Builder of [`SurfaceFormat2KHR`](struct.SurfaceFormat2KHR.html)"]
#[repr(transparent)]
pub struct SurfaceFormat2KHRBuilder<'a>(SurfaceFormat2KHR, std::marker::PhantomData<&'a ()>);
impl<'a> SurfaceFormat2KHRBuilder<'a> {
    #[inline]
    pub fn new() -> SurfaceFormat2KHRBuilder<'a> {
        SurfaceFormat2KHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn surface_format(
        mut self,
        surface_format: crate::extensions::khr_surface::SurfaceFormatKHR,
    ) -> Self {
        self.0.surface_format = surface_format;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> SurfaceFormat2KHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for SurfaceFormat2KHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for SurfaceFormat2KHRBuilder<'a> {
    type Target = SurfaceFormat2KHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SurfaceFormat2KHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
