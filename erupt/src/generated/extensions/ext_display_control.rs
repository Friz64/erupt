#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_DISPLAY_CONTROL_SPEC_VERSION")]
pub const EXT_DISPLAY_CONTROL_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_DISPLAY_CONTROL_EXTENSION_NAME")]
pub const EXT_DISPLAY_CONTROL_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_display_control");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_DISPLAY_POWER_CONTROL_EXT: *const std::os::raw::c_char = crate::cstr!("vkDisplayPowerControlEXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_REGISTER_DEVICE_EVENT_EXT: *const std::os::raw::c_char = crate::cstr!("vkRegisterDeviceEventEXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_REGISTER_DISPLAY_EVENT_EXT: *const std::os::raw::c_char = crate::cstr!("vkRegisterDisplayEventEXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_SWAPCHAIN_COUNTER_EXT: *const std::os::raw::c_char = crate::cstr!("vkGetSwapchainCounterEXT");
#[doc = "Provided by [`crate::extensions::ext_display_control`]"]
impl crate::vk1_0::StructureType {
    pub const DISPLAY_POWER_INFO_EXT: Self = Self(1000091000);
    pub const DEVICE_EVENT_INFO_EXT: Self = Self(1000091001);
    pub const DISPLAY_EVENT_INFO_EXT: Self = Self(1000091002);
    pub const SWAPCHAIN_COUNTER_CREATE_INFO_EXT: Self = Self(1000091003);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayPowerStateEXT.html) · Enum"]
#[doc(alias = "VkDisplayPowerStateEXT")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct DisplayPowerStateEXT(pub i32);
impl std::fmt::Debug for DisplayPowerStateEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::OFF_EXT => "OFF_EXT",
            &Self::SUSPEND_EXT => "SUSPEND_EXT",
            &Self::ON_EXT => "ON_EXT",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::ext_display_control`]"]
impl crate::extensions::ext_display_control::DisplayPowerStateEXT {
    pub const OFF_EXT: Self = Self(0);
    pub const SUSPEND_EXT: Self = Self(1);
    pub const ON_EXT: Self = Self(2);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceEventTypeEXT.html) · Enum"]
#[doc(alias = "VkDeviceEventTypeEXT")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct DeviceEventTypeEXT(pub i32);
impl std::fmt::Debug for DeviceEventTypeEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::DISPLAY_HOTPLUG_EXT => "DISPLAY_HOTPLUG_EXT",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::ext_display_control`]"]
impl crate::extensions::ext_display_control::DeviceEventTypeEXT {
    pub const DISPLAY_HOTPLUG_EXT: Self = Self(0);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayEventTypeEXT.html) · Enum"]
#[doc(alias = "VkDisplayEventTypeEXT")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct DisplayEventTypeEXT(pub i32);
impl std::fmt::Debug for DisplayEventTypeEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::FIRST_PIXEL_OUT_EXT => "FIRST_PIXEL_OUT_EXT",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::ext_display_control`]"]
impl crate::extensions::ext_display_control::DisplayEventTypeEXT {
    pub const FIRST_PIXEL_OUT_EXT: Self = Self(0);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDisplayPowerControlEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkDisplayPowerControlEXT = unsafe extern "system" fn(device: crate::vk1_0::Device, display: crate::extensions::khr_display::DisplayKHR, p_display_power_info: *const crate::extensions::ext_display_control::DisplayPowerInfoEXT) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkRegisterDeviceEventEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkRegisterDeviceEventEXT = unsafe extern "system" fn(device: crate::vk1_0::Device, p_device_event_info: *const crate::extensions::ext_display_control::DeviceEventInfoEXT, p_allocator: *const crate::vk1_0::AllocationCallbacks, p_fence: *mut crate::vk1_0::Fence) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkRegisterDisplayEventEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkRegisterDisplayEventEXT = unsafe extern "system" fn(device: crate::vk1_0::Device, display: crate::extensions::khr_display::DisplayKHR, p_display_event_info: *const crate::extensions::ext_display_control::DisplayEventInfoEXT, p_allocator: *const crate::vk1_0::AllocationCallbacks, p_fence: *mut crate::vk1_0::Fence) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSwapchainCounterEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetSwapchainCounterEXT = unsafe extern "system" fn(device: crate::vk1_0::Device, swapchain: crate::extensions::khr_swapchain::SwapchainKHR, counter: crate::extensions::ext_display_surface_counter::SurfaceCounterFlagBitsEXT, p_counter_value: *mut u64) -> crate::vk1_0::Result;
impl<'a> crate::ExtendableFrom<'a, SwapchainCounterCreateInfoEXT> for crate::extensions::khr_swapchain::SwapchainCreateInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, SwapchainCounterCreateInfoEXTBuilder<'_>> for crate::extensions::khr_swapchain::SwapchainCreateInfoKHRBuilder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayPowerInfoEXT.html) · Structure"]
#[doc(alias = "VkDisplayPowerInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DisplayPowerInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub power_state: crate::extensions::ext_display_control::DisplayPowerStateEXT,
}
impl DisplayPowerInfoEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::DISPLAY_POWER_INFO_EXT;
}
impl Default for DisplayPowerInfoEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), power_state: Default::default() }
    }
}
impl std::fmt::Debug for DisplayPowerInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DisplayPowerInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("power_state", &self.power_state).finish()
    }
}
impl DisplayPowerInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> DisplayPowerInfoEXTBuilder<'a> {
        DisplayPowerInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayPowerInfoEXT.html) · Builder of [`DisplayPowerInfoEXT`]"]
#[repr(transparent)]
pub struct DisplayPowerInfoEXTBuilder<'a>(DisplayPowerInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> DisplayPowerInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> DisplayPowerInfoEXTBuilder<'a> {
        DisplayPowerInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn power_state(mut self, power_state: crate::extensions::ext_display_control::DisplayPowerStateEXT) -> Self {
        self.0.power_state = power_state as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> DisplayPowerInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for DisplayPowerInfoEXTBuilder<'a> {
    fn default() -> DisplayPowerInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DisplayPowerInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DisplayPowerInfoEXTBuilder<'a> {
    type Target = DisplayPowerInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DisplayPowerInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceEventInfoEXT.html) · Structure"]
#[doc(alias = "VkDeviceEventInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DeviceEventInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub device_event: crate::extensions::ext_display_control::DeviceEventTypeEXT,
}
impl DeviceEventInfoEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::DEVICE_EVENT_INFO_EXT;
}
impl Default for DeviceEventInfoEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), device_event: Default::default() }
    }
}
impl std::fmt::Debug for DeviceEventInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DeviceEventInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("device_event", &self.device_event).finish()
    }
}
impl DeviceEventInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> DeviceEventInfoEXTBuilder<'a> {
        DeviceEventInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceEventInfoEXT.html) · Builder of [`DeviceEventInfoEXT`]"]
#[repr(transparent)]
pub struct DeviceEventInfoEXTBuilder<'a>(DeviceEventInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> DeviceEventInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> DeviceEventInfoEXTBuilder<'a> {
        DeviceEventInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn device_event(mut self, device_event: crate::extensions::ext_display_control::DeviceEventTypeEXT) -> Self {
        self.0.device_event = device_event as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> DeviceEventInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for DeviceEventInfoEXTBuilder<'a> {
    fn default() -> DeviceEventInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DeviceEventInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DeviceEventInfoEXTBuilder<'a> {
    type Target = DeviceEventInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DeviceEventInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayEventInfoEXT.html) · Structure"]
#[doc(alias = "VkDisplayEventInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DisplayEventInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub display_event: crate::extensions::ext_display_control::DisplayEventTypeEXT,
}
impl DisplayEventInfoEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::DISPLAY_EVENT_INFO_EXT;
}
impl Default for DisplayEventInfoEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), display_event: Default::default() }
    }
}
impl std::fmt::Debug for DisplayEventInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DisplayEventInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("display_event", &self.display_event).finish()
    }
}
impl DisplayEventInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> DisplayEventInfoEXTBuilder<'a> {
        DisplayEventInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayEventInfoEXT.html) · Builder of [`DisplayEventInfoEXT`]"]
#[repr(transparent)]
pub struct DisplayEventInfoEXTBuilder<'a>(DisplayEventInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> DisplayEventInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> DisplayEventInfoEXTBuilder<'a> {
        DisplayEventInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn display_event(mut self, display_event: crate::extensions::ext_display_control::DisplayEventTypeEXT) -> Self {
        self.0.display_event = display_event as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> DisplayEventInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for DisplayEventInfoEXTBuilder<'a> {
    fn default() -> DisplayEventInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DisplayEventInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DisplayEventInfoEXTBuilder<'a> {
    type Target = DisplayEventInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DisplayEventInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSwapchainCounterCreateInfoEXT.html) · Structure"]
#[doc(alias = "VkSwapchainCounterCreateInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SwapchainCounterCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub surface_counters: crate::extensions::ext_display_surface_counter::SurfaceCounterFlagsEXT,
}
impl SwapchainCounterCreateInfoEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::SWAPCHAIN_COUNTER_CREATE_INFO_EXT;
}
impl Default for SwapchainCounterCreateInfoEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), surface_counters: Default::default() }
    }
}
impl std::fmt::Debug for SwapchainCounterCreateInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SwapchainCounterCreateInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("surface_counters", &self.surface_counters).finish()
    }
}
impl SwapchainCounterCreateInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> SwapchainCounterCreateInfoEXTBuilder<'a> {
        SwapchainCounterCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSwapchainCounterCreateInfoEXT.html) · Builder of [`SwapchainCounterCreateInfoEXT`]"]
#[repr(transparent)]
pub struct SwapchainCounterCreateInfoEXTBuilder<'a>(SwapchainCounterCreateInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> SwapchainCounterCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> SwapchainCounterCreateInfoEXTBuilder<'a> {
        SwapchainCounterCreateInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn surface_counters(mut self, surface_counters: crate::extensions::ext_display_surface_counter::SurfaceCounterFlagsEXT) -> Self {
        self.0.surface_counters = surface_counters as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> SwapchainCounterCreateInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for SwapchainCounterCreateInfoEXTBuilder<'a> {
    fn default() -> SwapchainCounterCreateInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for SwapchainCounterCreateInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for SwapchainCounterCreateInfoEXTBuilder<'a> {
    type Target = SwapchainCounterCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SwapchainCounterCreateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::ext_display_control`]"]
impl crate::DeviceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDisplayPowerControlEXT.html) · Function"]
    #[doc(alias = "vkDisplayPowerControlEXT")]
    pub unsafe fn display_power_control_ext(&self, display: crate::extensions::khr_display::DisplayKHR, display_power_info: &crate::extensions::ext_display_control::DisplayPowerInfoEXT) -> crate::utils::VulkanResult<()> {
        let _function = self.display_power_control_ext.expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(self.handle, display as _, display_power_info as _);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkRegisterDeviceEventEXT.html) · Function"]
    #[doc(alias = "vkRegisterDeviceEventEXT")]
    pub unsafe fn register_device_event_ext(&self, device_event_info: &crate::extensions::ext_display_control::DeviceEventInfoEXT, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> crate::utils::VulkanResult<crate::vk1_0::Fence> {
        let _function = self.register_device_event_ext.expect(crate::NOT_LOADED_MESSAGE);
        let mut fence = Default::default();
        let _return = _function(
            self.handle,
            device_event_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut fence,
        );
        crate::utils::VulkanResult::new(_return, fence)
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkRegisterDisplayEventEXT.html) · Function"]
    #[doc(alias = "vkRegisterDisplayEventEXT")]
    pub unsafe fn register_display_event_ext(&self, display: crate::extensions::khr_display::DisplayKHR, display_event_info: &crate::extensions::ext_display_control::DisplayEventInfoEXT, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> crate::utils::VulkanResult<crate::vk1_0::Fence> {
        let _function = self.register_display_event_ext.expect(crate::NOT_LOADED_MESSAGE);
        let mut fence = Default::default();
        let _return = _function(
            self.handle,
            display as _,
            display_event_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut fence,
        );
        crate::utils::VulkanResult::new(_return, fence)
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSwapchainCounterEXT.html) · Function"]
    #[doc(alias = "vkGetSwapchainCounterEXT")]
    pub unsafe fn get_swapchain_counter_ext(&self, swapchain: crate::extensions::khr_swapchain::SwapchainKHR, counter: crate::extensions::ext_display_surface_counter::SurfaceCounterFlagBitsEXT) -> crate::utils::VulkanResult<u64> {
        let _function = self.get_swapchain_counter_ext.expect(crate::NOT_LOADED_MESSAGE);
        let mut counter_value = Default::default();
        let _return = _function(self.handle, swapchain as _, counter as _, &mut counter_value);
        crate::utils::VulkanResult::new(_return, counter_value)
    }
}
