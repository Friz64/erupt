#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_GET_SURFACE_CAPABILITIES_2_SPEC_VERSION")]
pub const KHR_GET_SURFACE_CAPABILITIES_2_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_GET_SURFACE_CAPABILITIES_2_EXTENSION_NAME")]
pub const KHR_GET_SURFACE_CAPABILITIES_2_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_get_surface_capabilities2");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_PHYSICAL_DEVICE_SURFACE_CAPABILITIES2_KHR: *const std::os::raw::c_char = crate::cstr!("vkGetPhysicalDeviceSurfaceCapabilities2KHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_PHYSICAL_DEVICE_SURFACE_FORMATS2_KHR: *const std::os::raw::c_char = crate::cstr!("vkGetPhysicalDeviceSurfaceFormats2KHR");
#[doc = "Provided by [`crate::extensions::khr_get_surface_capabilities2`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_SURFACE_INFO_2_KHR: Self = Self(1000119000);
    pub const SURFACE_CAPABILITIES_2_KHR: Self = Self(1000119001);
    pub const SURFACE_FORMAT_2_KHR: Self = Self(1000119002);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilities2KHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR = unsafe extern "system" fn(physical_device: crate::vk1_0::PhysicalDevice, p_surface_info: *const crate::extensions::khr_get_surface_capabilities2::PhysicalDeviceSurfaceInfo2KHR, p_surface_capabilities: *mut crate::extensions::khr_get_surface_capabilities2::SurfaceCapabilities2KHR) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfaceFormats2KHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSurfaceFormats2KHR = unsafe extern "system" fn(physical_device: crate::vk1_0::PhysicalDevice, p_surface_info: *const crate::extensions::khr_get_surface_capabilities2::PhysicalDeviceSurfaceInfo2KHR, p_surface_format_count: *mut u32, p_surface_formats: *mut crate::extensions::khr_get_surface_capabilities2::SurfaceFormat2KHR) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSurfaceInfo2KHR.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceSurfaceInfo2KHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceSurfaceInfo2KHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub surface: crate::extensions::khr_surface::SurfaceKHR,
}
impl PhysicalDeviceSurfaceInfo2KHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_SURFACE_INFO_2_KHR;
}
impl Default for PhysicalDeviceSurfaceInfo2KHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), surface: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceSurfaceInfo2KHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceSurfaceInfo2KHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("surface", &self.surface).finish()
    }
}
impl PhysicalDeviceSurfaceInfo2KHR {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceSurfaceInfo2KHRBuilder<'a> {
        PhysicalDeviceSurfaceInfo2KHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSurfaceInfo2KHR.html) · Builder of [`PhysicalDeviceSurfaceInfo2KHR`]"]
#[repr(transparent)]
pub struct PhysicalDeviceSurfaceInfo2KHRBuilder<'a>(PhysicalDeviceSurfaceInfo2KHR, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceSurfaceInfo2KHRBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceSurfaceInfo2KHRBuilder<'a> {
        PhysicalDeviceSurfaceInfo2KHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn surface(mut self, surface: crate::extensions::khr_surface::SurfaceKHR) -> Self {
        self.0.surface = surface as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> PhysicalDeviceSurfaceInfo2KHR {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceSurfaceInfo2KHRBuilder<'a> {
    fn default() -> PhysicalDeviceSurfaceInfo2KHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceSurfaceInfo2KHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
#[doc(alias = "VkSurfaceCapabilities2KHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SurfaceCapabilities2KHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub surface_capabilities: crate::extensions::khr_surface::SurfaceCapabilitiesKHR,
}
impl SurfaceCapabilities2KHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::SURFACE_CAPABILITIES_2_KHR;
}
impl Default for SurfaceCapabilities2KHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), surface_capabilities: Default::default() }
    }
}
impl std::fmt::Debug for SurfaceCapabilities2KHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SurfaceCapabilities2KHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("surface_capabilities", &self.surface_capabilities).finish()
    }
}
impl SurfaceCapabilities2KHR {
    #[inline]
    pub fn into_builder<'a>(self) -> SurfaceCapabilities2KHRBuilder<'a> {
        SurfaceCapabilities2KHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceCapabilities2KHR.html) · Builder of [`SurfaceCapabilities2KHR`]"]
#[repr(transparent)]
pub struct SurfaceCapabilities2KHRBuilder<'a>(SurfaceCapabilities2KHR, std::marker::PhantomData<&'a ()>);
impl<'a> SurfaceCapabilities2KHRBuilder<'a> {
    #[inline]
    pub fn new() -> SurfaceCapabilities2KHRBuilder<'a> {
        SurfaceCapabilities2KHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn surface_capabilities(mut self, surface_capabilities: crate::extensions::khr_surface::SurfaceCapabilitiesKHR) -> Self {
        self.0.surface_capabilities = surface_capabilities as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> SurfaceCapabilities2KHR {
        self.0
    }
}
impl<'a> std::default::Default for SurfaceCapabilities2KHRBuilder<'a> {
    fn default() -> SurfaceCapabilities2KHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for SurfaceCapabilities2KHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
#[doc(alias = "VkSurfaceFormat2KHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SurfaceFormat2KHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub surface_format: crate::extensions::khr_surface::SurfaceFormatKHR,
}
impl SurfaceFormat2KHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::SURFACE_FORMAT_2_KHR;
}
impl Default for SurfaceFormat2KHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), surface_format: Default::default() }
    }
}
impl std::fmt::Debug for SurfaceFormat2KHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SurfaceFormat2KHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("surface_format", &self.surface_format).finish()
    }
}
impl SurfaceFormat2KHR {
    #[inline]
    pub fn into_builder<'a>(self) -> SurfaceFormat2KHRBuilder<'a> {
        SurfaceFormat2KHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceFormat2KHR.html) · Builder of [`SurfaceFormat2KHR`]"]
#[repr(transparent)]
pub struct SurfaceFormat2KHRBuilder<'a>(SurfaceFormat2KHR, std::marker::PhantomData<&'a ()>);
impl<'a> SurfaceFormat2KHRBuilder<'a> {
    #[inline]
    pub fn new() -> SurfaceFormat2KHRBuilder<'a> {
        SurfaceFormat2KHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn surface_format(mut self, surface_format: crate::extensions::khr_surface::SurfaceFormatKHR) -> Self {
        self.0.surface_format = surface_format as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> SurfaceFormat2KHR {
        self.0
    }
}
impl<'a> std::default::Default for SurfaceFormat2KHRBuilder<'a> {
    fn default() -> SurfaceFormat2KHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for SurfaceFormat2KHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
#[doc = "Provided by [`crate::extensions::khr_get_surface_capabilities2`]"]
impl crate::InstanceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilities2KHR.html) · Function"]
    #[doc(alias = "vkGetPhysicalDeviceSurfaceCapabilities2KHR")]
    pub unsafe fn get_physical_device_surface_capabilities2_khr(&self, physical_device: crate::vk1_0::PhysicalDevice, surface_info: &crate::extensions::khr_get_surface_capabilities2::PhysicalDeviceSurfaceInfo2KHR, surface_capabilities: Option<crate::extensions::khr_get_surface_capabilities2::SurfaceCapabilities2KHR>) -> crate::utils::VulkanResult<crate::extensions::khr_get_surface_capabilities2::SurfaceCapabilities2KHR> {
        let _function = self.get_physical_device_surface_capabilities2_khr.expect(crate::NOT_LOADED_MESSAGE);
        let mut surface_capabilities = match surface_capabilities {
            Some(v) => v,
            None => Default::default(),
        };
        let _return = _function(physical_device as _, surface_info as _, &mut surface_capabilities);
        crate::utils::VulkanResult::new(_return, surface_capabilities)
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfaceFormats2KHR.html) · Function"]
    #[doc(alias = "vkGetPhysicalDeviceSurfaceFormats2KHR")]
    pub unsafe fn get_physical_device_surface_formats2_khr(&self, physical_device: crate::vk1_0::PhysicalDevice, surface_info: &crate::extensions::khr_get_surface_capabilities2::PhysicalDeviceSurfaceInfo2KHR, surface_format_count: Option<u32>) -> crate::utils::VulkanResult<crate::SmallVec<crate::extensions::khr_get_surface_capabilities2::SurfaceFormat2KHR>> {
        let _function = self.get_physical_device_surface_formats2_khr.expect(crate::NOT_LOADED_MESSAGE);
        let mut surface_format_count = match surface_format_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                _function(physical_device as _, surface_info as _, &mut v, std::ptr::null_mut());
                v
            }
        };
        let mut surface_formats = crate::SmallVec::from_elem(Default::default(), surface_format_count as _);
        let _return = _function(physical_device as _, surface_info as _, &mut surface_format_count, surface_formats.as_mut_ptr());
        crate::utils::VulkanResult::new(_return, surface_formats)
    }
}
