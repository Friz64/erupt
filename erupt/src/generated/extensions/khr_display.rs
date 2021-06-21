#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_DISPLAY_SPEC_VERSION")]
pub const KHR_DISPLAY_SPEC_VERSION: u32 = 23;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_DISPLAY_EXTENSION_NAME")]
pub const KHR_DISPLAY_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_display");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_PHYSICAL_DEVICE_DISPLAY_PROPERTIES_KHR: *const std::os::raw::c_char = crate::cstr!("vkGetPhysicalDeviceDisplayPropertiesKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_PHYSICAL_DEVICE_DISPLAY_PLANE_PROPERTIES_KHR: *const std::os::raw::c_char = crate::cstr!("vkGetPhysicalDeviceDisplayPlanePropertiesKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_DISPLAY_PLANE_SUPPORTED_DISPLAYS_KHR: *const std::os::raw::c_char = crate::cstr!("vkGetDisplayPlaneSupportedDisplaysKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_DISPLAY_MODE_PROPERTIES_KHR: *const std::os::raw::c_char = crate::cstr!("vkGetDisplayModePropertiesKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_DISPLAY_MODE_KHR: *const std::os::raw::c_char = crate::cstr!("vkCreateDisplayModeKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_DISPLAY_PLANE_CAPABILITIES_KHR: *const std::os::raw::c_char = crate::cstr!("vkGetDisplayPlaneCapabilitiesKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_DISPLAY_PLANE_SURFACE_KHR: *const std::os::raw::c_char = crate::cstr!("vkCreateDisplayPlaneSurfaceKHR");
crate::non_dispatchable_handle!(DisplayKHR, DISPLAY_KHR, "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayKHR.html) · Non-dispatchable Handle", "VkDisplayKHR");
crate::non_dispatchable_handle!(DisplayModeKHR, DISPLAY_MODE_KHR, "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayModeKHR.html) · Non-dispatchable Handle", "VkDisplayModeKHR");
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayModeCreateFlagsKHR.html) · Bitmask of [`DisplayModeCreateFlagBitsKHR`]"] # [doc (alias = "VkDisplayModeCreateFlagsKHR")] # [derive (Default)] # [repr (transparent)] pub struct DisplayModeCreateFlagsKHR : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`DisplayModeCreateFlagsKHR`]"]
#[doc(alias = "VkDisplayModeCreateFlagBitsKHR")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct DisplayModeCreateFlagBitsKHR(pub u32);
impl DisplayModeCreateFlagBitsKHR {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> DisplayModeCreateFlagsKHR {
        DisplayModeCreateFlagsKHR::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for DisplayModeCreateFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplaySurfaceCreateFlagsKHR.html) · Bitmask of [`DisplaySurfaceCreateFlagBitsKHR`]"] # [doc (alias = "VkDisplaySurfaceCreateFlagsKHR")] # [derive (Default)] # [repr (transparent)] pub struct DisplaySurfaceCreateFlagsKHR : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`DisplaySurfaceCreateFlagsKHR`]"]
#[doc(alias = "VkDisplaySurfaceCreateFlagBitsKHR")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct DisplaySurfaceCreateFlagBitsKHR(pub u32);
impl DisplaySurfaceCreateFlagBitsKHR {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> DisplaySurfaceCreateFlagsKHR {
        DisplaySurfaceCreateFlagsKHR::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for DisplaySurfaceCreateFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::khr_display`]"]
impl crate::vk1_0::StructureType {
    pub const DISPLAY_MODE_CREATE_INFO_KHR: Self = Self(1000002000);
    pub const DISPLAY_SURFACE_CREATE_INFO_KHR: Self = Self(1000002001);
}
#[doc = "Provided by [`crate::extensions::khr_display`]"]
impl crate::vk1_0::ObjectType {
    pub const DISPLAY_KHR: Self = Self(1000002000);
    pub const DISPLAY_MODE_KHR: Self = Self(1000002001);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayPlaneAlphaFlagsKHR.html) · Bitmask of [`DisplayPlaneAlphaFlagBitsKHR`]"] # [doc (alias = "VkDisplayPlaneAlphaFlagsKHR")] # [derive (Default)] # [repr (transparent)] pub struct DisplayPlaneAlphaFlagsKHR : u32 { const OPAQUE_KHR = DisplayPlaneAlphaFlagBitsKHR :: OPAQUE_KHR . 0 ; const GLOBAL_KHR = DisplayPlaneAlphaFlagBitsKHR :: GLOBAL_KHR . 0 ; const PER_PIXEL_KHR = DisplayPlaneAlphaFlagBitsKHR :: PER_PIXEL_KHR . 0 ; const PER_PIXEL_PREMULTIPLIED_KHR = DisplayPlaneAlphaFlagBitsKHR :: PER_PIXEL_PREMULTIPLIED_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayPlaneAlphaFlagBitsKHR.html) · Bits enum of [`DisplayPlaneAlphaFlagsKHR`]"]
#[doc(alias = "VkDisplayPlaneAlphaFlagBitsKHR")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct DisplayPlaneAlphaFlagBitsKHR(pub u32);
impl DisplayPlaneAlphaFlagBitsKHR {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> DisplayPlaneAlphaFlagsKHR {
        DisplayPlaneAlphaFlagsKHR::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for DisplayPlaneAlphaFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::OPAQUE_KHR => "OPAQUE_KHR",
            &Self::GLOBAL_KHR => "GLOBAL_KHR",
            &Self::PER_PIXEL_KHR => "PER_PIXEL_KHR",
            &Self::PER_PIXEL_PREMULTIPLIED_KHR => "PER_PIXEL_PREMULTIPLIED_KHR",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::khr_display`]"]
impl crate::extensions::khr_display::DisplayPlaneAlphaFlagBitsKHR {
    pub const OPAQUE_KHR: Self = Self(1);
    pub const GLOBAL_KHR: Self = Self(2);
    pub const PER_PIXEL_KHR: Self = Self(4);
    pub const PER_PIXEL_PREMULTIPLIED_KHR: Self = Self(8);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceDisplayPropertiesKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceDisplayPropertiesKHR = unsafe extern "system" fn(physical_device: crate::vk1_0::PhysicalDevice, p_property_count: *mut u32, p_properties: *mut crate::extensions::khr_display::DisplayPropertiesKHR) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceDisplayPlanePropertiesKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR = unsafe extern "system" fn(physical_device: crate::vk1_0::PhysicalDevice, p_property_count: *mut u32, p_properties: *mut crate::extensions::khr_display::DisplayPlanePropertiesKHR) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDisplayPlaneSupportedDisplaysKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDisplayPlaneSupportedDisplaysKHR = unsafe extern "system" fn(physical_device: crate::vk1_0::PhysicalDevice, plane_index: u32, p_display_count: *mut u32, p_displays: *mut crate::extensions::khr_display::DisplayKHR) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDisplayModePropertiesKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDisplayModePropertiesKHR = unsafe extern "system" fn(physical_device: crate::vk1_0::PhysicalDevice, display: crate::extensions::khr_display::DisplayKHR, p_property_count: *mut u32, p_properties: *mut crate::extensions::khr_display::DisplayModePropertiesKHR) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDisplayModeKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateDisplayModeKHR = unsafe extern "system" fn(physical_device: crate::vk1_0::PhysicalDevice, display: crate::extensions::khr_display::DisplayKHR, p_create_info: *const crate::extensions::khr_display::DisplayModeCreateInfoKHR, p_allocator: *const crate::vk1_0::AllocationCallbacks, p_mode: *mut crate::extensions::khr_display::DisplayModeKHR) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDisplayPlaneCapabilitiesKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDisplayPlaneCapabilitiesKHR = unsafe extern "system" fn(physical_device: crate::vk1_0::PhysicalDevice, mode: crate::extensions::khr_display::DisplayModeKHR, plane_index: u32, p_capabilities: *mut crate::extensions::khr_display::DisplayPlaneCapabilitiesKHR) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDisplayPlaneSurfaceKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateDisplayPlaneSurfaceKHR = unsafe extern "system" fn(instance: crate::vk1_0::Instance, p_create_info: *const crate::extensions::khr_display::DisplaySurfaceCreateInfoKHR, p_allocator: *const crate::vk1_0::AllocationCallbacks, p_surface: *mut crate::extensions::khr_surface::SurfaceKHR) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayPropertiesKHR.html) · Structure"]
#[doc(alias = "VkDisplayPropertiesKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DisplayPropertiesKHR {
    pub display: crate::extensions::khr_display::DisplayKHR,
    pub display_name: *const std::os::raw::c_char,
    pub physical_dimensions: crate::vk1_0::Extent2D,
    pub physical_resolution: crate::vk1_0::Extent2D,
    pub supported_transforms: crate::extensions::khr_surface::SurfaceTransformFlagsKHR,
    pub plane_reorder_possible: crate::vk1_0::Bool32,
    pub persistent_content: crate::vk1_0::Bool32,
}
impl Default for DisplayPropertiesKHR {
    fn default() -> Self {
        Self { display: Default::default(), display_name: std::ptr::null(), physical_dimensions: Default::default(), physical_resolution: Default::default(), supported_transforms: Default::default(), plane_reorder_possible: Default::default(), persistent_content: Default::default() }
    }
}
impl std::fmt::Debug for DisplayPropertiesKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DisplayPropertiesKHR").field("display", &self.display).field("display_name", &self.display_name).field("physical_dimensions", &self.physical_dimensions).field("physical_resolution", &self.physical_resolution).field("supported_transforms", &self.supported_transforms).field("plane_reorder_possible", &(self.plane_reorder_possible != 0)).field("persistent_content", &(self.persistent_content != 0)).finish()
    }
}
impl DisplayPropertiesKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> DisplayPropertiesKHRBuilder<'a> {
        DisplayPropertiesKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayPropertiesKHR.html) · Builder of [`DisplayPropertiesKHR`]"]
#[repr(transparent)]
pub struct DisplayPropertiesKHRBuilder<'a>(DisplayPropertiesKHR, std::marker::PhantomData<&'a ()>);
impl<'a> DisplayPropertiesKHRBuilder<'a> {
    #[inline]
    pub fn new() -> DisplayPropertiesKHRBuilder<'a> {
        DisplayPropertiesKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn display(mut self, display: crate::extensions::khr_display::DisplayKHR) -> Self {
        self.0.display = display as _;
        self
    }
    #[inline]
    pub fn display_name(mut self, display_name: &'a std::ffi::CStr) -> Self {
        self.0.display_name = display_name.as_ptr();
        self
    }
    #[inline]
    pub fn physical_dimensions(mut self, physical_dimensions: crate::vk1_0::Extent2D) -> Self {
        self.0.physical_dimensions = physical_dimensions as _;
        self
    }
    #[inline]
    pub fn physical_resolution(mut self, physical_resolution: crate::vk1_0::Extent2D) -> Self {
        self.0.physical_resolution = physical_resolution as _;
        self
    }
    #[inline]
    pub fn supported_transforms(mut self, supported_transforms: crate::extensions::khr_surface::SurfaceTransformFlagsKHR) -> Self {
        self.0.supported_transforms = supported_transforms as _;
        self
    }
    #[inline]
    pub fn plane_reorder_possible(mut self, plane_reorder_possible: bool) -> Self {
        self.0.plane_reorder_possible = plane_reorder_possible as _;
        self
    }
    #[inline]
    pub fn persistent_content(mut self, persistent_content: bool) -> Self {
        self.0.persistent_content = persistent_content as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DisplayPropertiesKHR {
        self.0
    }
}
impl<'a> std::default::Default for DisplayPropertiesKHRBuilder<'a> {
    fn default() -> DisplayPropertiesKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DisplayPropertiesKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DisplayPropertiesKHRBuilder<'a> {
    type Target = DisplayPropertiesKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DisplayPropertiesKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayPlanePropertiesKHR.html) · Structure"]
#[doc(alias = "VkDisplayPlanePropertiesKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DisplayPlanePropertiesKHR {
    pub current_display: crate::extensions::khr_display::DisplayKHR,
    pub current_stack_index: u32,
}
impl Default for DisplayPlanePropertiesKHR {
    fn default() -> Self {
        Self { current_display: Default::default(), current_stack_index: Default::default() }
    }
}
impl std::fmt::Debug for DisplayPlanePropertiesKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DisplayPlanePropertiesKHR").field("current_display", &self.current_display).field("current_stack_index", &self.current_stack_index).finish()
    }
}
impl DisplayPlanePropertiesKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> DisplayPlanePropertiesKHRBuilder<'a> {
        DisplayPlanePropertiesKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayPlanePropertiesKHR.html) · Builder of [`DisplayPlanePropertiesKHR`]"]
#[repr(transparent)]
pub struct DisplayPlanePropertiesKHRBuilder<'a>(DisplayPlanePropertiesKHR, std::marker::PhantomData<&'a ()>);
impl<'a> DisplayPlanePropertiesKHRBuilder<'a> {
    #[inline]
    pub fn new() -> DisplayPlanePropertiesKHRBuilder<'a> {
        DisplayPlanePropertiesKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn current_display(mut self, current_display: crate::extensions::khr_display::DisplayKHR) -> Self {
        self.0.current_display = current_display as _;
        self
    }
    #[inline]
    pub fn current_stack_index(mut self, current_stack_index: u32) -> Self {
        self.0.current_stack_index = current_stack_index as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DisplayPlanePropertiesKHR {
        self.0
    }
}
impl<'a> std::default::Default for DisplayPlanePropertiesKHRBuilder<'a> {
    fn default() -> DisplayPlanePropertiesKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DisplayPlanePropertiesKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DisplayPlanePropertiesKHRBuilder<'a> {
    type Target = DisplayPlanePropertiesKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DisplayPlanePropertiesKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayModeParametersKHR.html) · Structure"]
#[doc(alias = "VkDisplayModeParametersKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DisplayModeParametersKHR {
    pub visible_region: crate::vk1_0::Extent2D,
    pub refresh_rate: u32,
}
impl Default for DisplayModeParametersKHR {
    fn default() -> Self {
        Self { visible_region: Default::default(), refresh_rate: Default::default() }
    }
}
impl std::fmt::Debug for DisplayModeParametersKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DisplayModeParametersKHR").field("visible_region", &self.visible_region).field("refresh_rate", &self.refresh_rate).finish()
    }
}
impl DisplayModeParametersKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> DisplayModeParametersKHRBuilder<'a> {
        DisplayModeParametersKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayModeParametersKHR.html) · Builder of [`DisplayModeParametersKHR`]"]
#[repr(transparent)]
pub struct DisplayModeParametersKHRBuilder<'a>(DisplayModeParametersKHR, std::marker::PhantomData<&'a ()>);
impl<'a> DisplayModeParametersKHRBuilder<'a> {
    #[inline]
    pub fn new() -> DisplayModeParametersKHRBuilder<'a> {
        DisplayModeParametersKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn visible_region(mut self, visible_region: crate::vk1_0::Extent2D) -> Self {
        self.0.visible_region = visible_region as _;
        self
    }
    #[inline]
    pub fn refresh_rate(mut self, refresh_rate: u32) -> Self {
        self.0.refresh_rate = refresh_rate as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DisplayModeParametersKHR {
        self.0
    }
}
impl<'a> std::default::Default for DisplayModeParametersKHRBuilder<'a> {
    fn default() -> DisplayModeParametersKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DisplayModeParametersKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DisplayModeParametersKHRBuilder<'a> {
    type Target = DisplayModeParametersKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DisplayModeParametersKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayModePropertiesKHR.html) · Structure"]
#[doc(alias = "VkDisplayModePropertiesKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DisplayModePropertiesKHR {
    pub display_mode: crate::extensions::khr_display::DisplayModeKHR,
    pub parameters: crate::extensions::khr_display::DisplayModeParametersKHR,
}
impl Default for DisplayModePropertiesKHR {
    fn default() -> Self {
        Self { display_mode: Default::default(), parameters: Default::default() }
    }
}
impl std::fmt::Debug for DisplayModePropertiesKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DisplayModePropertiesKHR").field("display_mode", &self.display_mode).field("parameters", &self.parameters).finish()
    }
}
impl DisplayModePropertiesKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> DisplayModePropertiesKHRBuilder<'a> {
        DisplayModePropertiesKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayModePropertiesKHR.html) · Builder of [`DisplayModePropertiesKHR`]"]
#[repr(transparent)]
pub struct DisplayModePropertiesKHRBuilder<'a>(DisplayModePropertiesKHR, std::marker::PhantomData<&'a ()>);
impl<'a> DisplayModePropertiesKHRBuilder<'a> {
    #[inline]
    pub fn new() -> DisplayModePropertiesKHRBuilder<'a> {
        DisplayModePropertiesKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn display_mode(mut self, display_mode: crate::extensions::khr_display::DisplayModeKHR) -> Self {
        self.0.display_mode = display_mode as _;
        self
    }
    #[inline]
    pub fn parameters(mut self, parameters: crate::extensions::khr_display::DisplayModeParametersKHR) -> Self {
        self.0.parameters = parameters as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DisplayModePropertiesKHR {
        self.0
    }
}
impl<'a> std::default::Default for DisplayModePropertiesKHRBuilder<'a> {
    fn default() -> DisplayModePropertiesKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DisplayModePropertiesKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DisplayModePropertiesKHRBuilder<'a> {
    type Target = DisplayModePropertiesKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DisplayModePropertiesKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayModeCreateInfoKHR.html) · Structure"]
#[doc(alias = "VkDisplayModeCreateInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DisplayModeCreateInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::khr_display::DisplayModeCreateFlagsKHR,
    pub parameters: crate::extensions::khr_display::DisplayModeParametersKHR,
}
impl Default for DisplayModeCreateInfoKHR {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::DISPLAY_MODE_CREATE_INFO_KHR, p_next: std::ptr::null(), flags: Default::default(), parameters: Default::default() }
    }
}
impl std::fmt::Debug for DisplayModeCreateInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DisplayModeCreateInfoKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("parameters", &self.parameters).finish()
    }
}
impl DisplayModeCreateInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> DisplayModeCreateInfoKHRBuilder<'a> {
        DisplayModeCreateInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayModeCreateInfoKHR.html) · Builder of [`DisplayModeCreateInfoKHR`]"]
#[repr(transparent)]
pub struct DisplayModeCreateInfoKHRBuilder<'a>(DisplayModeCreateInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> DisplayModeCreateInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> DisplayModeCreateInfoKHRBuilder<'a> {
        DisplayModeCreateInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::khr_display::DisplayModeCreateFlagsKHR) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn parameters(mut self, parameters: crate::extensions::khr_display::DisplayModeParametersKHR) -> Self {
        self.0.parameters = parameters as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DisplayModeCreateInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for DisplayModeCreateInfoKHRBuilder<'a> {
    fn default() -> DisplayModeCreateInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DisplayModeCreateInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DisplayModeCreateInfoKHRBuilder<'a> {
    type Target = DisplayModeCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DisplayModeCreateInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayPlaneCapabilitiesKHR.html) · Structure"]
#[doc(alias = "VkDisplayPlaneCapabilitiesKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DisplayPlaneCapabilitiesKHR {
    pub supported_alpha: crate::extensions::khr_display::DisplayPlaneAlphaFlagsKHR,
    pub min_src_position: crate::vk1_0::Offset2D,
    pub max_src_position: crate::vk1_0::Offset2D,
    pub min_src_extent: crate::vk1_0::Extent2D,
    pub max_src_extent: crate::vk1_0::Extent2D,
    pub min_dst_position: crate::vk1_0::Offset2D,
    pub max_dst_position: crate::vk1_0::Offset2D,
    pub min_dst_extent: crate::vk1_0::Extent2D,
    pub max_dst_extent: crate::vk1_0::Extent2D,
}
impl Default for DisplayPlaneCapabilitiesKHR {
    fn default() -> Self {
        Self { supported_alpha: Default::default(), min_src_position: Default::default(), max_src_position: Default::default(), min_src_extent: Default::default(), max_src_extent: Default::default(), min_dst_position: Default::default(), max_dst_position: Default::default(), min_dst_extent: Default::default(), max_dst_extent: Default::default() }
    }
}
impl std::fmt::Debug for DisplayPlaneCapabilitiesKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DisplayPlaneCapabilitiesKHR").field("supported_alpha", &self.supported_alpha).field("min_src_position", &self.min_src_position).field("max_src_position", &self.max_src_position).field("min_src_extent", &self.min_src_extent).field("max_src_extent", &self.max_src_extent).field("min_dst_position", &self.min_dst_position).field("max_dst_position", &self.max_dst_position).field("min_dst_extent", &self.min_dst_extent).field("max_dst_extent", &self.max_dst_extent).finish()
    }
}
impl DisplayPlaneCapabilitiesKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> DisplayPlaneCapabilitiesKHRBuilder<'a> {
        DisplayPlaneCapabilitiesKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayPlaneCapabilitiesKHR.html) · Builder of [`DisplayPlaneCapabilitiesKHR`]"]
#[repr(transparent)]
pub struct DisplayPlaneCapabilitiesKHRBuilder<'a>(DisplayPlaneCapabilitiesKHR, std::marker::PhantomData<&'a ()>);
impl<'a> DisplayPlaneCapabilitiesKHRBuilder<'a> {
    #[inline]
    pub fn new() -> DisplayPlaneCapabilitiesKHRBuilder<'a> {
        DisplayPlaneCapabilitiesKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn supported_alpha(mut self, supported_alpha: crate::extensions::khr_display::DisplayPlaneAlphaFlagsKHR) -> Self {
        self.0.supported_alpha = supported_alpha as _;
        self
    }
    #[inline]
    pub fn min_src_position(mut self, min_src_position: crate::vk1_0::Offset2D) -> Self {
        self.0.min_src_position = min_src_position as _;
        self
    }
    #[inline]
    pub fn max_src_position(mut self, max_src_position: crate::vk1_0::Offset2D) -> Self {
        self.0.max_src_position = max_src_position as _;
        self
    }
    #[inline]
    pub fn min_src_extent(mut self, min_src_extent: crate::vk1_0::Extent2D) -> Self {
        self.0.min_src_extent = min_src_extent as _;
        self
    }
    #[inline]
    pub fn max_src_extent(mut self, max_src_extent: crate::vk1_0::Extent2D) -> Self {
        self.0.max_src_extent = max_src_extent as _;
        self
    }
    #[inline]
    pub fn min_dst_position(mut self, min_dst_position: crate::vk1_0::Offset2D) -> Self {
        self.0.min_dst_position = min_dst_position as _;
        self
    }
    #[inline]
    pub fn max_dst_position(mut self, max_dst_position: crate::vk1_0::Offset2D) -> Self {
        self.0.max_dst_position = max_dst_position as _;
        self
    }
    #[inline]
    pub fn min_dst_extent(mut self, min_dst_extent: crate::vk1_0::Extent2D) -> Self {
        self.0.min_dst_extent = min_dst_extent as _;
        self
    }
    #[inline]
    pub fn max_dst_extent(mut self, max_dst_extent: crate::vk1_0::Extent2D) -> Self {
        self.0.max_dst_extent = max_dst_extent as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DisplayPlaneCapabilitiesKHR {
        self.0
    }
}
impl<'a> std::default::Default for DisplayPlaneCapabilitiesKHRBuilder<'a> {
    fn default() -> DisplayPlaneCapabilitiesKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DisplayPlaneCapabilitiesKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DisplayPlaneCapabilitiesKHRBuilder<'a> {
    type Target = DisplayPlaneCapabilitiesKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DisplayPlaneCapabilitiesKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplaySurfaceCreateInfoKHR.html) · Structure"]
#[doc(alias = "VkDisplaySurfaceCreateInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DisplaySurfaceCreateInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::khr_display::DisplaySurfaceCreateFlagsKHR,
    pub display_mode: crate::extensions::khr_display::DisplayModeKHR,
    pub plane_index: u32,
    pub plane_stack_index: u32,
    pub transform: crate::extensions::khr_surface::SurfaceTransformFlagBitsKHR,
    pub global_alpha: std::os::raw::c_float,
    pub alpha_mode: crate::extensions::khr_display::DisplayPlaneAlphaFlagBitsKHR,
    pub image_extent: crate::vk1_0::Extent2D,
}
impl Default for DisplaySurfaceCreateInfoKHR {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::DISPLAY_SURFACE_CREATE_INFO_KHR, p_next: std::ptr::null(), flags: Default::default(), display_mode: Default::default(), plane_index: Default::default(), plane_stack_index: Default::default(), transform: Default::default(), global_alpha: Default::default(), alpha_mode: Default::default(), image_extent: Default::default() }
    }
}
impl std::fmt::Debug for DisplaySurfaceCreateInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DisplaySurfaceCreateInfoKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("display_mode", &self.display_mode).field("plane_index", &self.plane_index).field("plane_stack_index", &self.plane_stack_index).field("transform", &self.transform).field("global_alpha", &self.global_alpha).field("alpha_mode", &self.alpha_mode).field("image_extent", &self.image_extent).finish()
    }
}
impl DisplaySurfaceCreateInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> DisplaySurfaceCreateInfoKHRBuilder<'a> {
        DisplaySurfaceCreateInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplaySurfaceCreateInfoKHR.html) · Builder of [`DisplaySurfaceCreateInfoKHR`]"]
#[repr(transparent)]
pub struct DisplaySurfaceCreateInfoKHRBuilder<'a>(DisplaySurfaceCreateInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> DisplaySurfaceCreateInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> DisplaySurfaceCreateInfoKHRBuilder<'a> {
        DisplaySurfaceCreateInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::khr_display::DisplaySurfaceCreateFlagsKHR) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn display_mode(mut self, display_mode: crate::extensions::khr_display::DisplayModeKHR) -> Self {
        self.0.display_mode = display_mode as _;
        self
    }
    #[inline]
    pub fn plane_index(mut self, plane_index: u32) -> Self {
        self.0.plane_index = plane_index as _;
        self
    }
    #[inline]
    pub fn plane_stack_index(mut self, plane_stack_index: u32) -> Self {
        self.0.plane_stack_index = plane_stack_index as _;
        self
    }
    #[inline]
    pub fn transform(mut self, transform: crate::extensions::khr_surface::SurfaceTransformFlagBitsKHR) -> Self {
        self.0.transform = transform as _;
        self
    }
    #[inline]
    pub fn global_alpha(mut self, global_alpha: std::os::raw::c_float) -> Self {
        self.0.global_alpha = global_alpha as _;
        self
    }
    #[inline]
    pub fn alpha_mode(mut self, alpha_mode: crate::extensions::khr_display::DisplayPlaneAlphaFlagBitsKHR) -> Self {
        self.0.alpha_mode = alpha_mode as _;
        self
    }
    #[inline]
    pub fn image_extent(mut self, image_extent: crate::vk1_0::Extent2D) -> Self {
        self.0.image_extent = image_extent as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DisplaySurfaceCreateInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for DisplaySurfaceCreateInfoKHRBuilder<'a> {
    fn default() -> DisplaySurfaceCreateInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DisplaySurfaceCreateInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DisplaySurfaceCreateInfoKHRBuilder<'a> {
    type Target = DisplaySurfaceCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DisplaySurfaceCreateInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::khr_display`]"]
impl crate::InstanceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceDisplayPropertiesKHR.html) · Function"]
    #[doc(alias = "vkGetPhysicalDeviceDisplayPropertiesKHR")]
    pub unsafe fn get_physical_device_display_properties_khr(&self, physical_device: crate::vk1_0::PhysicalDevice, property_count: Option<u32>) -> crate::utils::VulkanResult<Vec<crate::extensions::khr_display::DisplayPropertiesKHR>> {
        let _function = self.get_physical_device_display_properties_khr.expect(crate::NOT_LOADED_MESSAGE);
        let mut property_count = match property_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                _function(physical_device as _, &mut v, std::ptr::null_mut());
                v
            }
        };
        let mut properties = vec![Default::default(); property_count as _];
        let _return = _function(physical_device as _, &mut property_count, properties.as_mut_ptr());
        crate::utils::VulkanResult::new(_return, properties)
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceDisplayPlanePropertiesKHR.html) · Function"]
    #[doc(alias = "vkGetPhysicalDeviceDisplayPlanePropertiesKHR")]
    pub unsafe fn get_physical_device_display_plane_properties_khr(&self, physical_device: crate::vk1_0::PhysicalDevice, property_count: Option<u32>) -> crate::utils::VulkanResult<Vec<crate::extensions::khr_display::DisplayPlanePropertiesKHR>> {
        let _function = self.get_physical_device_display_plane_properties_khr.expect(crate::NOT_LOADED_MESSAGE);
        let mut property_count = match property_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                _function(physical_device as _, &mut v, std::ptr::null_mut());
                v
            }
        };
        let mut properties = vec![Default::default(); property_count as _];
        let _return = _function(physical_device as _, &mut property_count, properties.as_mut_ptr());
        crate::utils::VulkanResult::new(_return, properties)
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDisplayPlaneSupportedDisplaysKHR.html) · Function"]
    #[doc(alias = "vkGetDisplayPlaneSupportedDisplaysKHR")]
    pub unsafe fn get_display_plane_supported_displays_khr(&self, physical_device: crate::vk1_0::PhysicalDevice, plane_index: u32, display_count: Option<u32>) -> crate::utils::VulkanResult<Vec<crate::extensions::khr_display::DisplayKHR>> {
        let _function = self.get_display_plane_supported_displays_khr.expect(crate::NOT_LOADED_MESSAGE);
        let mut display_count = match display_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                _function(physical_device as _, plane_index as _, &mut v, std::ptr::null_mut());
                v
            }
        };
        let mut displays = vec![Default::default(); display_count as _];
        let _return = _function(physical_device as _, plane_index as _, &mut display_count, displays.as_mut_ptr());
        crate::utils::VulkanResult::new(_return, displays)
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDisplayModePropertiesKHR.html) · Function"]
    #[doc(alias = "vkGetDisplayModePropertiesKHR")]
    pub unsafe fn get_display_mode_properties_khr(&self, physical_device: crate::vk1_0::PhysicalDevice, display: crate::extensions::khr_display::DisplayKHR, property_count: Option<u32>) -> crate::utils::VulkanResult<Vec<crate::extensions::khr_display::DisplayModePropertiesKHR>> {
        let _function = self.get_display_mode_properties_khr.expect(crate::NOT_LOADED_MESSAGE);
        let mut property_count = match property_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                _function(physical_device as _, display as _, &mut v, std::ptr::null_mut());
                v
            }
        };
        let mut properties = vec![Default::default(); property_count as _];
        let _return = _function(physical_device as _, display as _, &mut property_count, properties.as_mut_ptr());
        crate::utils::VulkanResult::new(_return, properties)
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDisplayModeKHR.html) · Function"]
    #[doc(alias = "vkCreateDisplayModeKHR")]
    pub unsafe fn create_display_mode_khr(&self, physical_device: crate::vk1_0::PhysicalDevice, display: crate::extensions::khr_display::DisplayKHR, create_info: &crate::extensions::khr_display::DisplayModeCreateInfoKHR, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> crate::utils::VulkanResult<crate::extensions::khr_display::DisplayModeKHR> {
        let _function = self.create_display_mode_khr.expect(crate::NOT_LOADED_MESSAGE);
        let mut mode = Default::default();
        let _return = _function(
            physical_device as _,
            display as _,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut mode,
        );
        crate::utils::VulkanResult::new(_return, mode)
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDisplayPlaneCapabilitiesKHR.html) · Function"]
    #[doc(alias = "vkGetDisplayPlaneCapabilitiesKHR")]
    pub unsafe fn get_display_plane_capabilities_khr(&self, physical_device: crate::vk1_0::PhysicalDevice, mode: crate::extensions::khr_display::DisplayModeKHR, plane_index: u32) -> crate::utils::VulkanResult<crate::extensions::khr_display::DisplayPlaneCapabilitiesKHR> {
        let _function = self.get_display_plane_capabilities_khr.expect(crate::NOT_LOADED_MESSAGE);
        let mut capabilities = Default::default();
        let _return = _function(physical_device as _, mode as _, plane_index as _, &mut capabilities);
        crate::utils::VulkanResult::new(_return, capabilities)
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDisplayPlaneSurfaceKHR.html) · Function"]
    #[doc(alias = "vkCreateDisplayPlaneSurfaceKHR")]
    pub unsafe fn create_display_plane_surface_khr(&self, create_info: &crate::extensions::khr_display::DisplaySurfaceCreateInfoKHR, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> crate::utils::VulkanResult<crate::extensions::khr_surface::SurfaceKHR> {
        let _function = self.create_display_plane_surface_khr.expect(crate::NOT_LOADED_MESSAGE);
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
