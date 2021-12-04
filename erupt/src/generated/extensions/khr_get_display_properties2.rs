#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_GET_DISPLAY_PROPERTIES_2_SPEC_VERSION")]
pub const KHR_GET_DISPLAY_PROPERTIES_2_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_GET_DISPLAY_PROPERTIES_2_EXTENSION_NAME")]
pub const KHR_GET_DISPLAY_PROPERTIES_2_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_get_display_properties2");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_PHYSICAL_DEVICE_DISPLAY_PROPERTIES2_KHR: *const std::os::raw::c_char = crate::cstr!("vkGetPhysicalDeviceDisplayProperties2KHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_PHYSICAL_DEVICE_DISPLAY_PLANE_PROPERTIES2_KHR: *const std::os::raw::c_char = crate::cstr!("vkGetPhysicalDeviceDisplayPlaneProperties2KHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_DISPLAY_MODE_PROPERTIES2_KHR: *const std::os::raw::c_char = crate::cstr!("vkGetDisplayModeProperties2KHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_DISPLAY_PLANE_CAPABILITIES2_KHR: *const std::os::raw::c_char = crate::cstr!("vkGetDisplayPlaneCapabilities2KHR");
#[doc = "Provided by [`crate::extensions::khr_get_display_properties2`]"]
impl crate::vk1_0::StructureType {
    pub const DISPLAY_PROPERTIES_2_KHR: Self = Self(1000121000);
    pub const DISPLAY_PLANE_PROPERTIES_2_KHR: Self = Self(1000121001);
    pub const DISPLAY_MODE_PROPERTIES_2_KHR: Self = Self(1000121002);
    pub const DISPLAY_PLANE_INFO_2_KHR: Self = Self(1000121003);
    pub const DISPLAY_PLANE_CAPABILITIES_2_KHR: Self = Self(1000121004);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceDisplayProperties2KHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceDisplayProperties2KHR = unsafe extern "system" fn(physical_device: crate::vk1_0::PhysicalDevice, p_property_count: *mut u32, p_properties: *mut crate::extensions::khr_get_display_properties2::DisplayProperties2KHR) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceDisplayPlaneProperties2KHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceDisplayPlaneProperties2KHR = unsafe extern "system" fn(physical_device: crate::vk1_0::PhysicalDevice, p_property_count: *mut u32, p_properties: *mut crate::extensions::khr_get_display_properties2::DisplayPlaneProperties2KHR) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDisplayModeProperties2KHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDisplayModeProperties2KHR = unsafe extern "system" fn(physical_device: crate::vk1_0::PhysicalDevice, display: crate::extensions::khr_display::DisplayKHR, p_property_count: *mut u32, p_properties: *mut crate::extensions::khr_get_display_properties2::DisplayModeProperties2KHR) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDisplayPlaneCapabilities2KHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDisplayPlaneCapabilities2KHR = unsafe extern "system" fn(physical_device: crate::vk1_0::PhysicalDevice, p_display_plane_info: *const crate::extensions::khr_get_display_properties2::DisplayPlaneInfo2KHR, p_capabilities: *mut crate::extensions::khr_get_display_properties2::DisplayPlaneCapabilities2KHR) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayProperties2KHR.html) · Structure"]
#[doc(alias = "VkDisplayProperties2KHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DisplayProperties2KHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub display_properties: crate::extensions::khr_display::DisplayPropertiesKHR,
}
impl DisplayProperties2KHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::DISPLAY_PROPERTIES_2_KHR;
}
impl Default for DisplayProperties2KHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), display_properties: Default::default() }
    }
}
impl std::fmt::Debug for DisplayProperties2KHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DisplayProperties2KHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("display_properties", &self.display_properties).finish()
    }
}
impl DisplayProperties2KHR {
    #[inline]
    pub fn into_builder<'a>(self) -> DisplayProperties2KHRBuilder<'a> {
        DisplayProperties2KHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayProperties2KHR.html) · Builder of [`DisplayProperties2KHR`]"]
#[repr(transparent)]
pub struct DisplayProperties2KHRBuilder<'a>(DisplayProperties2KHR, std::marker::PhantomData<&'a ()>);
impl<'a> DisplayProperties2KHRBuilder<'a> {
    #[inline]
    pub fn new() -> DisplayProperties2KHRBuilder<'a> {
        DisplayProperties2KHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn display_properties(mut self, display_properties: crate::extensions::khr_display::DisplayPropertiesKHR) -> Self {
        self.0.display_properties = display_properties as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> DisplayProperties2KHR {
        self.0
    }
}
impl<'a> std::default::Default for DisplayProperties2KHRBuilder<'a> {
    fn default() -> DisplayProperties2KHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DisplayProperties2KHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DisplayProperties2KHRBuilder<'a> {
    type Target = DisplayProperties2KHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DisplayProperties2KHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayPlaneProperties2KHR.html) · Structure"]
#[doc(alias = "VkDisplayPlaneProperties2KHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DisplayPlaneProperties2KHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub display_plane_properties: crate::extensions::khr_display::DisplayPlanePropertiesKHR,
}
impl DisplayPlaneProperties2KHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::DISPLAY_PLANE_PROPERTIES_2_KHR;
}
impl Default for DisplayPlaneProperties2KHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), display_plane_properties: Default::default() }
    }
}
impl std::fmt::Debug for DisplayPlaneProperties2KHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DisplayPlaneProperties2KHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("display_plane_properties", &self.display_plane_properties).finish()
    }
}
impl DisplayPlaneProperties2KHR {
    #[inline]
    pub fn into_builder<'a>(self) -> DisplayPlaneProperties2KHRBuilder<'a> {
        DisplayPlaneProperties2KHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayPlaneProperties2KHR.html) · Builder of [`DisplayPlaneProperties2KHR`]"]
#[repr(transparent)]
pub struct DisplayPlaneProperties2KHRBuilder<'a>(DisplayPlaneProperties2KHR, std::marker::PhantomData<&'a ()>);
impl<'a> DisplayPlaneProperties2KHRBuilder<'a> {
    #[inline]
    pub fn new() -> DisplayPlaneProperties2KHRBuilder<'a> {
        DisplayPlaneProperties2KHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn display_plane_properties(mut self, display_plane_properties: crate::extensions::khr_display::DisplayPlanePropertiesKHR) -> Self {
        self.0.display_plane_properties = display_plane_properties as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> DisplayPlaneProperties2KHR {
        self.0
    }
}
impl<'a> std::default::Default for DisplayPlaneProperties2KHRBuilder<'a> {
    fn default() -> DisplayPlaneProperties2KHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DisplayPlaneProperties2KHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DisplayPlaneProperties2KHRBuilder<'a> {
    type Target = DisplayPlaneProperties2KHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DisplayPlaneProperties2KHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayModeProperties2KHR.html) · Structure"]
#[doc(alias = "VkDisplayModeProperties2KHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DisplayModeProperties2KHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub display_mode_properties: crate::extensions::khr_display::DisplayModePropertiesKHR,
}
impl DisplayModeProperties2KHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::DISPLAY_MODE_PROPERTIES_2_KHR;
}
impl Default for DisplayModeProperties2KHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), display_mode_properties: Default::default() }
    }
}
impl std::fmt::Debug for DisplayModeProperties2KHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DisplayModeProperties2KHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("display_mode_properties", &self.display_mode_properties).finish()
    }
}
impl DisplayModeProperties2KHR {
    #[inline]
    pub fn into_builder<'a>(self) -> DisplayModeProperties2KHRBuilder<'a> {
        DisplayModeProperties2KHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayModeProperties2KHR.html) · Builder of [`DisplayModeProperties2KHR`]"]
#[repr(transparent)]
pub struct DisplayModeProperties2KHRBuilder<'a>(DisplayModeProperties2KHR, std::marker::PhantomData<&'a ()>);
impl<'a> DisplayModeProperties2KHRBuilder<'a> {
    #[inline]
    pub fn new() -> DisplayModeProperties2KHRBuilder<'a> {
        DisplayModeProperties2KHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn display_mode_properties(mut self, display_mode_properties: crate::extensions::khr_display::DisplayModePropertiesKHR) -> Self {
        self.0.display_mode_properties = display_mode_properties as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> DisplayModeProperties2KHR {
        self.0
    }
}
impl<'a> std::default::Default for DisplayModeProperties2KHRBuilder<'a> {
    fn default() -> DisplayModeProperties2KHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DisplayModeProperties2KHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DisplayModeProperties2KHRBuilder<'a> {
    type Target = DisplayModeProperties2KHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DisplayModeProperties2KHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayPlaneInfo2KHR.html) · Structure"]
#[doc(alias = "VkDisplayPlaneInfo2KHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DisplayPlaneInfo2KHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub mode: crate::extensions::khr_display::DisplayModeKHR,
    pub plane_index: u32,
}
impl DisplayPlaneInfo2KHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::DISPLAY_PLANE_INFO_2_KHR;
}
impl Default for DisplayPlaneInfo2KHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), mode: Default::default(), plane_index: Default::default() }
    }
}
impl std::fmt::Debug for DisplayPlaneInfo2KHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DisplayPlaneInfo2KHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("mode", &self.mode).field("plane_index", &self.plane_index).finish()
    }
}
impl DisplayPlaneInfo2KHR {
    #[inline]
    pub fn into_builder<'a>(self) -> DisplayPlaneInfo2KHRBuilder<'a> {
        DisplayPlaneInfo2KHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayPlaneInfo2KHR.html) · Builder of [`DisplayPlaneInfo2KHR`]"]
#[repr(transparent)]
pub struct DisplayPlaneInfo2KHRBuilder<'a>(DisplayPlaneInfo2KHR, std::marker::PhantomData<&'a ()>);
impl<'a> DisplayPlaneInfo2KHRBuilder<'a> {
    #[inline]
    pub fn new() -> DisplayPlaneInfo2KHRBuilder<'a> {
        DisplayPlaneInfo2KHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn mode(mut self, mode: crate::extensions::khr_display::DisplayModeKHR) -> Self {
        self.0.mode = mode as _;
        self
    }
    #[inline]
    pub fn plane_index(mut self, plane_index: u32) -> Self {
        self.0.plane_index = plane_index as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> DisplayPlaneInfo2KHR {
        self.0
    }
}
impl<'a> std::default::Default for DisplayPlaneInfo2KHRBuilder<'a> {
    fn default() -> DisplayPlaneInfo2KHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DisplayPlaneInfo2KHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DisplayPlaneInfo2KHRBuilder<'a> {
    type Target = DisplayPlaneInfo2KHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DisplayPlaneInfo2KHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayPlaneCapabilities2KHR.html) · Structure"]
#[doc(alias = "VkDisplayPlaneCapabilities2KHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DisplayPlaneCapabilities2KHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub capabilities: crate::extensions::khr_display::DisplayPlaneCapabilitiesKHR,
}
impl DisplayPlaneCapabilities2KHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::DISPLAY_PLANE_CAPABILITIES_2_KHR;
}
impl Default for DisplayPlaneCapabilities2KHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), capabilities: Default::default() }
    }
}
impl std::fmt::Debug for DisplayPlaneCapabilities2KHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DisplayPlaneCapabilities2KHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("capabilities", &self.capabilities).finish()
    }
}
impl DisplayPlaneCapabilities2KHR {
    #[inline]
    pub fn into_builder<'a>(self) -> DisplayPlaneCapabilities2KHRBuilder<'a> {
        DisplayPlaneCapabilities2KHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayPlaneCapabilities2KHR.html) · Builder of [`DisplayPlaneCapabilities2KHR`]"]
#[repr(transparent)]
pub struct DisplayPlaneCapabilities2KHRBuilder<'a>(DisplayPlaneCapabilities2KHR, std::marker::PhantomData<&'a ()>);
impl<'a> DisplayPlaneCapabilities2KHRBuilder<'a> {
    #[inline]
    pub fn new() -> DisplayPlaneCapabilities2KHRBuilder<'a> {
        DisplayPlaneCapabilities2KHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn capabilities(mut self, capabilities: crate::extensions::khr_display::DisplayPlaneCapabilitiesKHR) -> Self {
        self.0.capabilities = capabilities as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> DisplayPlaneCapabilities2KHR {
        self.0
    }
}
impl<'a> std::default::Default for DisplayPlaneCapabilities2KHRBuilder<'a> {
    fn default() -> DisplayPlaneCapabilities2KHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DisplayPlaneCapabilities2KHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DisplayPlaneCapabilities2KHRBuilder<'a> {
    type Target = DisplayPlaneCapabilities2KHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DisplayPlaneCapabilities2KHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::khr_get_display_properties2`]"]
impl crate::InstanceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceDisplayProperties2KHR.html) · Function"]
    #[doc(alias = "vkGetPhysicalDeviceDisplayProperties2KHR")]
    pub unsafe fn get_physical_device_display_properties2_khr(&self, physical_device: crate::vk1_0::PhysicalDevice, property_count: Option<u32>, properties_callback: Option<impl FnMut(&mut crate::SmallVec<crate::extensions::khr_get_display_properties2::DisplayProperties2KHR>) -> ()>) -> crate::utils::VulkanResult<crate::SmallVec<crate::extensions::khr_get_display_properties2::DisplayProperties2KHR>> {
        let _function = self.get_physical_device_display_properties2_khr.expect(crate::NOT_LOADED_MESSAGE);
        let mut property_count = match property_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                _function(physical_device as _, &mut v, std::ptr::null_mut());
                v
            }
        };
        let mut properties = crate::SmallVec::from_elem(Default::default(), property_count as _);
        if let Some(mut _callback) = properties_callback {
            _callback(&mut properties);
        }
        let _return = _function(physical_device as _, &mut property_count, properties.as_mut_ptr());
        crate::utils::VulkanResult::new(_return, properties)
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceDisplayPlaneProperties2KHR.html) · Function"]
    #[doc(alias = "vkGetPhysicalDeviceDisplayPlaneProperties2KHR")]
    pub unsafe fn get_physical_device_display_plane_properties2_khr(&self, physical_device: crate::vk1_0::PhysicalDevice, property_count: Option<u32>, properties_callback: Option<impl FnMut(&mut crate::SmallVec<crate::extensions::khr_get_display_properties2::DisplayPlaneProperties2KHR>) -> ()>) -> crate::utils::VulkanResult<crate::SmallVec<crate::extensions::khr_get_display_properties2::DisplayPlaneProperties2KHR>> {
        let _function = self.get_physical_device_display_plane_properties2_khr.expect(crate::NOT_LOADED_MESSAGE);
        let mut property_count = match property_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                _function(physical_device as _, &mut v, std::ptr::null_mut());
                v
            }
        };
        let mut properties = crate::SmallVec::from_elem(Default::default(), property_count as _);
        if let Some(mut _callback) = properties_callback {
            _callback(&mut properties);
        }
        let _return = _function(physical_device as _, &mut property_count, properties.as_mut_ptr());
        crate::utils::VulkanResult::new(_return, properties)
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDisplayModeProperties2KHR.html) · Function"]
    #[doc(alias = "vkGetDisplayModeProperties2KHR")]
    pub unsafe fn get_display_mode_properties2_khr(&self, physical_device: crate::vk1_0::PhysicalDevice, display: crate::extensions::khr_display::DisplayKHR, property_count: Option<u32>, properties_callback: Option<impl FnMut(&mut crate::SmallVec<crate::extensions::khr_get_display_properties2::DisplayModeProperties2KHR>) -> ()>) -> crate::utils::VulkanResult<crate::SmallVec<crate::extensions::khr_get_display_properties2::DisplayModeProperties2KHR>> {
        let _function = self.get_display_mode_properties2_khr.expect(crate::NOT_LOADED_MESSAGE);
        let mut property_count = match property_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                _function(physical_device as _, display as _, &mut v, std::ptr::null_mut());
                v
            }
        };
        let mut properties = crate::SmallVec::from_elem(Default::default(), property_count as _);
        if let Some(mut _callback) = properties_callback {
            _callback(&mut properties);
        }
        let _return = _function(physical_device as _, display as _, &mut property_count, properties.as_mut_ptr());
        crate::utils::VulkanResult::new(_return, properties)
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDisplayPlaneCapabilities2KHR.html) · Function"]
    #[doc(alias = "vkGetDisplayPlaneCapabilities2KHR")]
    pub unsafe fn get_display_plane_capabilities2_khr(&self, physical_device: crate::vk1_0::PhysicalDevice, display_plane_info: &crate::extensions::khr_get_display_properties2::DisplayPlaneInfo2KHR, capabilities: Option<crate::extensions::khr_get_display_properties2::DisplayPlaneCapabilities2KHR>) -> crate::utils::VulkanResult<crate::extensions::khr_get_display_properties2::DisplayPlaneCapabilities2KHR> {
        let _function = self.get_display_plane_capabilities2_khr.expect(crate::NOT_LOADED_MESSAGE);
        let mut capabilities = match capabilities {
            Some(v) => v,
            None => Default::default(),
        };
        let _return = _function(physical_device as _, display_plane_info as _, &mut capabilities);
        crate::utils::VulkanResult::new(_return, capabilities)
    }
}
