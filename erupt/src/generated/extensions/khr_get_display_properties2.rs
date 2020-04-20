# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_KHR_get_display_properties2.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_GET_DISPLAY_PROPERTIES_2_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_GET_DISPLAY_PROPERTIES_2_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_get_display_properties2");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceDisplayProperties2KHR.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceDisplayProperties2KHR =
    unsafe extern "system" fn(
        physical_device: crate::vk1_0::PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut crate::extensions::khr_get_display_properties2::DisplayProperties2KHR,
    ) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceDisplayPlaneProperties2KHR.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceDisplayPlaneProperties2KHR = unsafe extern "system" fn ( physical_device : crate :: vk1_0 :: PhysicalDevice , p_property_count : * mut u32 , p_properties : * mut crate :: extensions :: khr_get_display_properties2 :: DisplayPlaneProperties2KHR , ) -> crate :: vk1_0 :: Result ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDisplayModeProperties2KHR.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDisplayModeProperties2KHR = unsafe extern "system" fn(
    physical_device: crate::vk1_0::PhysicalDevice,
    display: crate::extensions::khr_display::DisplayKHR,
    p_property_count: *mut u32,
    p_properties: *mut crate::extensions::khr_get_display_properties2::DisplayModeProperties2KHR,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDisplayPlaneCapabilities2KHR.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDisplayPlaneCapabilities2KHR = unsafe extern "system" fn ( physical_device : crate :: vk1_0 :: PhysicalDevice , p_display_plane_info : * const crate :: extensions :: khr_get_display_properties2 :: DisplayPlaneInfo2KHR , p_capabilities : * mut crate :: extensions :: khr_get_display_properties2 :: DisplayPlaneCapabilities2KHR , ) -> crate :: vk1_0 :: Result ;
#[doc = "Provides Instance Commands for [`KhrGetDisplayProperties2InstanceLoaderExt`](trait.KhrGetDisplayProperties2InstanceLoaderExt.html)"]
pub struct KhrGetDisplayProperties2InstanceCommands {
    pub get_physical_device_display_properties2_khr: PFN_vkGetPhysicalDeviceDisplayProperties2KHR,
    pub get_physical_device_display_plane_properties2_khr:
        PFN_vkGetPhysicalDeviceDisplayPlaneProperties2KHR,
    pub get_display_mode_properties2_khr: PFN_vkGetDisplayModeProperties2KHR,
    pub get_display_plane_capabilities2_khr: PFN_vkGetDisplayPlaneCapabilities2KHR,
}
impl KhrGetDisplayProperties2InstanceCommands {
    #[inline]
    pub fn load(
        loader: &crate::InstanceLoader,
    ) -> Option<KhrGetDisplayProperties2InstanceCommands> {
        unsafe {
            Some(KhrGetDisplayProperties2InstanceCommands {
                get_physical_device_display_properties2_khr: std::mem::transmute(
                    loader.symbol("vkGetPhysicalDeviceDisplayProperties2KHR")?,
                ),
                get_physical_device_display_plane_properties2_khr: std::mem::transmute(
                    loader.symbol("vkGetPhysicalDeviceDisplayPlaneProperties2KHR")?,
                ),
                get_display_mode_properties2_khr: std::mem::transmute(
                    loader.symbol("vkGetDisplayModeProperties2KHR")?,
                ),
                get_display_plane_capabilities2_khr: std::mem::transmute(
                    loader.symbol("vkGetDisplayPlaneCapabilities2KHR")?,
                ),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`KhrGetDisplayProperties2InstanceCommands`](struct.KhrGetDisplayProperties2InstanceCommands.html)"]
pub trait KhrGetDisplayProperties2InstanceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceDisplayProperties2KHR.html) · Instance Command"]
    unsafe fn get_physical_device_display_properties2_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        property_count: Option<u32>,
    ) -> crate::utils::VulkanResult<
        Vec<crate::extensions::khr_get_display_properties2::DisplayProperties2KHR>,
    >;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceDisplayPlaneProperties2KHR.html) · Instance Command"]
    unsafe fn get_physical_device_display_plane_properties2_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        property_count: Option<u32>,
    ) -> crate::utils::VulkanResult<
        Vec<crate::extensions::khr_get_display_properties2::DisplayPlaneProperties2KHR>,
    >;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDisplayModeProperties2KHR.html) · Instance Command"]
    unsafe fn get_display_mode_properties2_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        display: crate::extensions::khr_display::DisplayKHR,
        property_count: Option<u32>,
    ) -> crate::utils::VulkanResult<
        Vec<crate::extensions::khr_get_display_properties2::DisplayModeProperties2KHR>,
    >;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDisplayPlaneCapabilities2KHR.html) · Instance Command"]
    unsafe fn get_display_plane_capabilities2_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        display_plane_info: &crate::extensions::khr_get_display_properties2::DisplayPlaneInfo2KHR,
        capabilities: Option<
            crate::extensions::khr_get_display_properties2::DisplayPlaneCapabilities2KHR,
        >,
    ) -> crate::utils::VulkanResult<
        crate::extensions::khr_get_display_properties2::DisplayPlaneCapabilities2KHR,
    >;
}
impl KhrGetDisplayProperties2InstanceLoaderExt for crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceDisplayProperties2KHR.html) · Instance Command"]
    unsafe fn get_physical_device_display_properties2_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        property_count: Option<u32>,
    ) -> crate::utils::VulkanResult<
        Vec<crate::extensions::khr_get_display_properties2::DisplayProperties2KHR>,
    > {
        let function = self
            .khr_get_display_properties2
            .as_ref()
            .expect("`khr_get_display_properties2` not loaded")
            .get_physical_device_display_properties2_khr;
        let mut property_count = property_count.unwrap_or_else(|| {
            let mut val = Default::default();
            function(physical_device, &mut val, std::ptr::null_mut());
            val
        });
        let mut properties = vec![Default::default(); property_count as _];
        let _val = function(
            physical_device,
            &mut property_count,
            properties.as_mut_ptr(),
        );
        crate::utils::VulkanResult::new(_val, properties)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceDisplayPlaneProperties2KHR.html) · Instance Command"]
    unsafe fn get_physical_device_display_plane_properties2_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        property_count: Option<u32>,
    ) -> crate::utils::VulkanResult<
        Vec<crate::extensions::khr_get_display_properties2::DisplayPlaneProperties2KHR>,
    > {
        let function = self
            .khr_get_display_properties2
            .as_ref()
            .expect("`khr_get_display_properties2` not loaded")
            .get_physical_device_display_plane_properties2_khr;
        let mut property_count = property_count.unwrap_or_else(|| {
            let mut val = Default::default();
            function(physical_device, &mut val, std::ptr::null_mut());
            val
        });
        let mut properties = vec![Default::default(); property_count as _];
        let _val = function(
            physical_device,
            &mut property_count,
            properties.as_mut_ptr(),
        );
        crate::utils::VulkanResult::new(_val, properties)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDisplayModeProperties2KHR.html) · Instance Command"]
    unsafe fn get_display_mode_properties2_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        display: crate::extensions::khr_display::DisplayKHR,
        property_count: Option<u32>,
    ) -> crate::utils::VulkanResult<
        Vec<crate::extensions::khr_get_display_properties2::DisplayModeProperties2KHR>,
    > {
        let function = self
            .khr_get_display_properties2
            .as_ref()
            .expect("`khr_get_display_properties2` not loaded")
            .get_display_mode_properties2_khr;
        let mut property_count = property_count.unwrap_or_else(|| {
            let mut val = Default::default();
            function(physical_device, display, &mut val, std::ptr::null_mut());
            val
        });
        let mut properties = vec![Default::default(); property_count as _];
        let _val = function(
            physical_device,
            display,
            &mut property_count,
            properties.as_mut_ptr(),
        );
        crate::utils::VulkanResult::new(_val, properties)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDisplayPlaneCapabilities2KHR.html) · Instance Command"]
    unsafe fn get_display_plane_capabilities2_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        display_plane_info: &crate::extensions::khr_get_display_properties2::DisplayPlaneInfo2KHR,
        capabilities: Option<
            crate::extensions::khr_get_display_properties2::DisplayPlaneCapabilities2KHR,
        >,
    ) -> crate::utils::VulkanResult<
        crate::extensions::khr_get_display_properties2::DisplayPlaneCapabilities2KHR,
    > {
        let function = self
            .khr_get_display_properties2
            .as_ref()
            .expect("`khr_get_display_properties2` not loaded")
            .get_display_plane_capabilities2_khr;
        let mut capabilities = capabilities.unwrap_or_else(|| Default::default());
        let _val = function(physical_device, display_plane_info, &mut capabilities);
        crate::utils::VulkanResult::new(_val, capabilities)
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayProperties2KHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DisplayProperties2KHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub display_properties: crate::extensions::khr_display::DisplayPropertiesKHR,
}
impl DisplayProperties2KHR {
    #[inline]
    pub fn builder<'a>(self) -> DisplayProperties2KHRBuilder<'a> {
        DisplayProperties2KHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for DisplayProperties2KHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("DisplayProperties2KHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("display_properties", &self.display_properties)
            .finish()
    }
}
impl Default for DisplayProperties2KHR {
    fn default() -> DisplayProperties2KHR {
        DisplayProperties2KHR {
            s_type: crate::vk1_0::StructureType::DISPLAY_PROPERTIES_2_KHR,
            p_next: std::ptr::null_mut(),
            display_properties: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayProperties2KHR.html) · Builder of [`DisplayProperties2KHR`](struct.DisplayProperties2KHR.html)"]
#[repr(transparent)]
pub struct DisplayProperties2KHRBuilder<'a>(
    DisplayProperties2KHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> DisplayProperties2KHRBuilder<'a> {
    #[inline]
    pub fn new() -> DisplayProperties2KHRBuilder<'a> {
        DisplayProperties2KHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn display_properties(
        mut self,
        display_properties: crate::extensions::khr_display::DisplayPropertiesKHR,
    ) -> Self {
        self.0.display_properties = display_properties;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> DisplayProperties2KHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for DisplayProperties2KHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DisplayPlaneProperties2KHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub display_plane_properties: crate::extensions::khr_display::DisplayPlanePropertiesKHR,
}
impl DisplayPlaneProperties2KHR {
    #[inline]
    pub fn builder<'a>(self) -> DisplayPlaneProperties2KHRBuilder<'a> {
        DisplayPlaneProperties2KHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for DisplayPlaneProperties2KHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("DisplayPlaneProperties2KHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("display_plane_properties", &self.display_plane_properties)
            .finish()
    }
}
impl Default for DisplayPlaneProperties2KHR {
    fn default() -> DisplayPlaneProperties2KHR {
        DisplayPlaneProperties2KHR {
            s_type: crate::vk1_0::StructureType::DISPLAY_PLANE_PROPERTIES_2_KHR,
            p_next: std::ptr::null_mut(),
            display_plane_properties: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayPlaneProperties2KHR.html) · Builder of [`DisplayPlaneProperties2KHR`](struct.DisplayPlaneProperties2KHR.html)"]
#[repr(transparent)]
pub struct DisplayPlaneProperties2KHRBuilder<'a>(
    DisplayPlaneProperties2KHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> DisplayPlaneProperties2KHRBuilder<'a> {
    #[inline]
    pub fn new() -> DisplayPlaneProperties2KHRBuilder<'a> {
        DisplayPlaneProperties2KHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn display_plane_properties(
        mut self,
        display_plane_properties: crate::extensions::khr_display::DisplayPlanePropertiesKHR,
    ) -> Self {
        self.0.display_plane_properties = display_plane_properties;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> DisplayPlaneProperties2KHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for DisplayPlaneProperties2KHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DisplayModeProperties2KHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub display_mode_properties: crate::extensions::khr_display::DisplayModePropertiesKHR,
}
impl DisplayModeProperties2KHR {
    #[inline]
    pub fn builder<'a>(self) -> DisplayModeProperties2KHRBuilder<'a> {
        DisplayModeProperties2KHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for DisplayModeProperties2KHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("DisplayModeProperties2KHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("display_mode_properties", &self.display_mode_properties)
            .finish()
    }
}
impl Default for DisplayModeProperties2KHR {
    fn default() -> DisplayModeProperties2KHR {
        DisplayModeProperties2KHR {
            s_type: crate::vk1_0::StructureType::DISPLAY_MODE_PROPERTIES_2_KHR,
            p_next: std::ptr::null_mut(),
            display_mode_properties: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayModeProperties2KHR.html) · Builder of [`DisplayModeProperties2KHR`](struct.DisplayModeProperties2KHR.html)"]
#[repr(transparent)]
pub struct DisplayModeProperties2KHRBuilder<'a>(
    DisplayModeProperties2KHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> DisplayModeProperties2KHRBuilder<'a> {
    #[inline]
    pub fn new() -> DisplayModeProperties2KHRBuilder<'a> {
        DisplayModeProperties2KHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn display_mode_properties(
        mut self,
        display_mode_properties: crate::extensions::khr_display::DisplayModePropertiesKHR,
    ) -> Self {
        self.0.display_mode_properties = display_mode_properties;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> DisplayModeProperties2KHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for DisplayModeProperties2KHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DisplayPlaneInfo2KHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub mode: crate::extensions::khr_display::DisplayModeKHR,
    pub plane_index: u32,
}
impl DisplayPlaneInfo2KHR {
    #[inline]
    pub fn builder<'a>(self) -> DisplayPlaneInfo2KHRBuilder<'a> {
        DisplayPlaneInfo2KHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for DisplayPlaneInfo2KHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("DisplayPlaneInfo2KHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("mode", &self.mode)
            .field("plane_index", &self.plane_index)
            .finish()
    }
}
impl Default for DisplayPlaneInfo2KHR {
    fn default() -> DisplayPlaneInfo2KHR {
        DisplayPlaneInfo2KHR {
            s_type: crate::vk1_0::StructureType::DISPLAY_PLANE_INFO_2_KHR,
            p_next: std::ptr::null(),
            mode: Default::default(),
            plane_index: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayPlaneInfo2KHR.html) · Builder of [`DisplayPlaneInfo2KHR`](struct.DisplayPlaneInfo2KHR.html)"]
#[repr(transparent)]
pub struct DisplayPlaneInfo2KHRBuilder<'a>(DisplayPlaneInfo2KHR, std::marker::PhantomData<&'a ()>);
impl<'a> DisplayPlaneInfo2KHRBuilder<'a> {
    #[inline]
    pub fn new() -> DisplayPlaneInfo2KHRBuilder<'a> {
        DisplayPlaneInfo2KHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn mode(mut self, mode: crate::extensions::khr_display::DisplayModeKHR) -> Self {
        self.0.mode = mode;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn plane_index(mut self, plane_index: u32) -> Self {
        self.0.plane_index = plane_index;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> DisplayPlaneInfo2KHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for DisplayPlaneInfo2KHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DisplayPlaneCapabilities2KHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub capabilities: crate::extensions::khr_display::DisplayPlaneCapabilitiesKHR,
}
impl DisplayPlaneCapabilities2KHR {
    #[inline]
    pub fn builder<'a>(self) -> DisplayPlaneCapabilities2KHRBuilder<'a> {
        DisplayPlaneCapabilities2KHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for DisplayPlaneCapabilities2KHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("DisplayPlaneCapabilities2KHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("capabilities", &self.capabilities)
            .finish()
    }
}
impl Default for DisplayPlaneCapabilities2KHR {
    fn default() -> DisplayPlaneCapabilities2KHR {
        DisplayPlaneCapabilities2KHR {
            s_type: crate::vk1_0::StructureType::DISPLAY_PLANE_CAPABILITIES_2_KHR,
            p_next: std::ptr::null_mut(),
            capabilities: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayPlaneCapabilities2KHR.html) · Builder of [`DisplayPlaneCapabilities2KHR`](struct.DisplayPlaneCapabilities2KHR.html)"]
#[repr(transparent)]
pub struct DisplayPlaneCapabilities2KHRBuilder<'a>(
    DisplayPlaneCapabilities2KHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> DisplayPlaneCapabilities2KHRBuilder<'a> {
    #[inline]
    pub fn new() -> DisplayPlaneCapabilities2KHRBuilder<'a> {
        DisplayPlaneCapabilities2KHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn capabilities(
        mut self,
        capabilities: crate::extensions::khr_display::DisplayPlaneCapabilitiesKHR,
    ) -> Self {
        self.0.capabilities = capabilities;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> DisplayPlaneCapabilities2KHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for DisplayPlaneCapabilities2KHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
