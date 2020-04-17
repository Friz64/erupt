# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_EXT_display_control.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_DISPLAY_CONTROL_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_DISPLAY_CONTROL_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_display_control");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDisplayPowerControlEXT.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkDisplayPowerControlEXT = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    display: crate::extensions::khr_display::DisplayKHR,
    p_display_power_info: *const crate::extensions::ext_display_control::DisplayPowerInfoEXT,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkRegisterDeviceEventEXT.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkRegisterDeviceEventEXT = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_device_event_info: *const crate::extensions::ext_display_control::DeviceEventInfoEXT,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
    p_fence: *mut crate::vk1_0::Fence,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkRegisterDisplayEventEXT.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkRegisterDisplayEventEXT = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    display: crate::extensions::khr_display::DisplayKHR,
    p_display_event_info: *const crate::extensions::ext_display_control::DisplayEventInfoEXT,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
    p_fence: *mut crate::vk1_0::Fence,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSwapchainCounterEXT.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetSwapchainCounterEXT = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
    counter: crate::extensions::ext_display_surface_counter::SurfaceCounterFlagBitsEXT,
    p_counter_value: *mut u64,
) -> crate::vk1_0::Result;
#[doc = "Provides Device Commands for [`ExtDisplayControlDeviceLoaderExt`](trait.ExtDisplayControlDeviceLoaderExt.html)"]
pub struct ExtDisplayControlDeviceCommands {
    pub display_power_control_ext: PFN_vkDisplayPowerControlEXT,
    pub register_device_event_ext: PFN_vkRegisterDeviceEventEXT,
    pub register_display_event_ext: PFN_vkRegisterDisplayEventEXT,
    pub get_swapchain_counter_ext: PFN_vkGetSwapchainCounterEXT,
}
impl ExtDisplayControlDeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<ExtDisplayControlDeviceCommands> {
        unsafe {
            Some(ExtDisplayControlDeviceCommands {
                display_power_control_ext: std::mem::transmute(
                    loader.symbol("vkDisplayPowerControlEXT")?,
                ),
                register_device_event_ext: std::mem::transmute(
                    loader.symbol("vkRegisterDeviceEventEXT")?,
                ),
                register_display_event_ext: std::mem::transmute(
                    loader.symbol("vkRegisterDisplayEventEXT")?,
                ),
                get_swapchain_counter_ext: std::mem::transmute(
                    loader.symbol("vkGetSwapchainCounterEXT")?,
                ),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`ExtDisplayControlDeviceCommands`](struct.ExtDisplayControlDeviceCommands.html)"]
pub trait ExtDisplayControlDeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDisplayPowerControlEXT.html) · Device Command"]
    unsafe fn display_power_control_ext(
        &self,
        display: crate::extensions::khr_display::DisplayKHR,
        display_power_info: &crate::extensions::ext_display_control::DisplayPowerInfoEXT,
    ) -> crate::utils::VulkanResult<()>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkRegisterDeviceEventEXT.html) · Device Command"]
    unsafe fn register_device_event_ext(
        &self,
        device_event_info: &crate::extensions::ext_display_control::DeviceEventInfoEXT,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        fence: Option<crate::vk1_0::Fence>,
    ) -> crate::utils::VulkanResult<crate::vk1_0::Fence>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkRegisterDisplayEventEXT.html) · Device Command"]
    unsafe fn register_display_event_ext(
        &self,
        display: crate::extensions::khr_display::DisplayKHR,
        display_event_info: &crate::extensions::ext_display_control::DisplayEventInfoEXT,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        fence: Option<crate::vk1_0::Fence>,
    ) -> crate::utils::VulkanResult<crate::vk1_0::Fence>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSwapchainCounterEXT.html) · Device Command"]
    unsafe fn get_swapchain_counter_ext(
        &self,
        swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
        counter: crate::extensions::ext_display_surface_counter::SurfaceCounterFlagBitsEXT,
        counter_value: Option<u64>,
    ) -> crate::utils::VulkanResult<u64>;
}
impl ExtDisplayControlDeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDisplayPowerControlEXT.html) · Device Command"]
    unsafe fn display_power_control_ext(
        &self,
        display: crate::extensions::khr_display::DisplayKHR,
        display_power_info: &crate::extensions::ext_display_control::DisplayPowerInfoEXT,
    ) -> crate::utils::VulkanResult<()> {
        let function = self
            .ext_display_control
            .as_ref()
            .expect("`ext_display_control` not loaded")
            .display_power_control_ext;
        let _val = function(self.handle, display, display_power_info);
        crate::utils::VulkanResult::new(_val, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkRegisterDeviceEventEXT.html) · Device Command"]
    unsafe fn register_device_event_ext(
        &self,
        device_event_info: &crate::extensions::ext_display_control::DeviceEventInfoEXT,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        fence: Option<crate::vk1_0::Fence>,
    ) -> crate::utils::VulkanResult<crate::vk1_0::Fence> {
        let function = self
            .ext_display_control
            .as_ref()
            .expect("`ext_display_control` not loaded")
            .register_device_event_ext;
        let mut fence = fence.unwrap_or_else(|| Default::default());
        let _val = function(
            self.handle,
            device_event_info,
            if let Some(allocator) = allocator {
                allocator
            } else {
                std::ptr::null()
            },
            &mut fence,
        );
        crate::utils::VulkanResult::new(_val, fence)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkRegisterDisplayEventEXT.html) · Device Command"]
    unsafe fn register_display_event_ext(
        &self,
        display: crate::extensions::khr_display::DisplayKHR,
        display_event_info: &crate::extensions::ext_display_control::DisplayEventInfoEXT,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        fence: Option<crate::vk1_0::Fence>,
    ) -> crate::utils::VulkanResult<crate::vk1_0::Fence> {
        let function = self
            .ext_display_control
            .as_ref()
            .expect("`ext_display_control` not loaded")
            .register_display_event_ext;
        let mut fence = fence.unwrap_or_else(|| Default::default());
        let _val = function(
            self.handle,
            display,
            display_event_info,
            if let Some(allocator) = allocator {
                allocator
            } else {
                std::ptr::null()
            },
            &mut fence,
        );
        crate::utils::VulkanResult::new(_val, fence)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSwapchainCounterEXT.html) · Device Command"]
    unsafe fn get_swapchain_counter_ext(
        &self,
        swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
        counter: crate::extensions::ext_display_surface_counter::SurfaceCounterFlagBitsEXT,
        counter_value: Option<u64>,
    ) -> crate::utils::VulkanResult<u64> {
        let function = self
            .ext_display_control
            .as_ref()
            .expect("`ext_display_control` not loaded")
            .get_swapchain_counter_ext;
        let mut counter_value = counter_value.unwrap_or_else(|| Default::default());
        let _val = function(self.handle, swapchain, counter, &mut counter_value);
        crate::utils::VulkanResult::new(_val, counter_value)
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayPowerInfoEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DisplayPowerInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub power_state: crate::extensions::ext_display_control::DisplayPowerStateEXT,
}
impl DisplayPowerInfoEXT {
    #[inline]
    pub fn builder<'a>(self) -> DisplayPowerInfoEXTBuilder<'a> {
        DisplayPowerInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for DisplayPowerInfoEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("DisplayPowerInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("power_state", &self.power_state)
            .finish()
    }
}
impl Default for DisplayPowerInfoEXT {
    fn default() -> DisplayPowerInfoEXT {
        DisplayPowerInfoEXT {
            s_type: crate::vk1_0::StructureType::DISPLAY_POWER_INFO_EXT,
            p_next: std::ptr::null(),
            power_state: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`DisplayPowerInfoEXT`](struct.DisplayPowerInfoEXT.html)"]
#[repr(transparent)]
pub struct DisplayPowerInfoEXTBuilder<'a>(DisplayPowerInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> DisplayPowerInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> DisplayPowerInfoEXTBuilder<'a> {
        DisplayPowerInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn power_state(
        mut self,
        power_state: crate::extensions::ext_display_control::DisplayPowerStateEXT,
    ) -> Self {
        self.0.power_state = power_state;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> DisplayPowerInfoEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for DisplayPowerInfoEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayPowerStateEXT.html) · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct DisplayPowerStateEXT(pub i32);
#[doc = "[Part of `extensions::ext_display_control`](../../extensions/ext_display_control/index.html)"]
impl DisplayPowerStateEXT {
    pub const OFF_EXT: Self = Self(0);
    pub const SUSPEND_EXT: Self = Self(1);
    pub const ON_EXT: Self = Self(2);
}
impl std::fmt::Debug for DisplayPowerStateEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::OFF_EXT => "OFF_EXT",
            &Self::SUSPEND_EXT => "SUSPEND_EXT",
            &Self::ON_EXT => "ON_EXT",
            _ => "Unknown enum variant",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceEventInfoEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DeviceEventInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub device_event: crate::extensions::ext_display_control::DeviceEventTypeEXT,
}
impl DeviceEventInfoEXT {
    #[inline]
    pub fn builder<'a>(self) -> DeviceEventInfoEXTBuilder<'a> {
        DeviceEventInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for DeviceEventInfoEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("DeviceEventInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("device_event", &self.device_event)
            .finish()
    }
}
impl Default for DeviceEventInfoEXT {
    fn default() -> DeviceEventInfoEXT {
        DeviceEventInfoEXT {
            s_type: crate::vk1_0::StructureType::DEVICE_EVENT_INFO_EXT,
            p_next: std::ptr::null(),
            device_event: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`DeviceEventInfoEXT`](struct.DeviceEventInfoEXT.html)"]
#[repr(transparent)]
pub struct DeviceEventInfoEXTBuilder<'a>(DeviceEventInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> DeviceEventInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> DeviceEventInfoEXTBuilder<'a> {
        DeviceEventInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn device_event(
        mut self,
        device_event: crate::extensions::ext_display_control::DeviceEventTypeEXT,
    ) -> Self {
        self.0.device_event = device_event;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> DeviceEventInfoEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for DeviceEventInfoEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceEventTypeEXT.html) · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct DeviceEventTypeEXT(pub i32);
#[doc = "[Part of `extensions::ext_display_control`](../../extensions/ext_display_control/index.html)"]
impl DeviceEventTypeEXT {
    pub const DISPLAY_HOTPLUG_EXT: Self = Self(0);
}
impl std::fmt::Debug for DeviceEventTypeEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::DISPLAY_HOTPLUG_EXT => "DISPLAY_HOTPLUG_EXT",
            _ => "Unknown enum variant",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayEventInfoEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DisplayEventInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub display_event: crate::extensions::ext_display_control::DisplayEventTypeEXT,
}
impl DisplayEventInfoEXT {
    #[inline]
    pub fn builder<'a>(self) -> DisplayEventInfoEXTBuilder<'a> {
        DisplayEventInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for DisplayEventInfoEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("DisplayEventInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("display_event", &self.display_event)
            .finish()
    }
}
impl Default for DisplayEventInfoEXT {
    fn default() -> DisplayEventInfoEXT {
        DisplayEventInfoEXT {
            s_type: crate::vk1_0::StructureType::DISPLAY_EVENT_INFO_EXT,
            p_next: std::ptr::null(),
            display_event: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`DisplayEventInfoEXT`](struct.DisplayEventInfoEXT.html)"]
#[repr(transparent)]
pub struct DisplayEventInfoEXTBuilder<'a>(DisplayEventInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> DisplayEventInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> DisplayEventInfoEXTBuilder<'a> {
        DisplayEventInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn display_event(
        mut self,
        display_event: crate::extensions::ext_display_control::DisplayEventTypeEXT,
    ) -> Self {
        self.0.display_event = display_event;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> DisplayEventInfoEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for DisplayEventInfoEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayEventTypeEXT.html) · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct DisplayEventTypeEXT(pub i32);
#[doc = "[Part of `extensions::ext_display_control`](../../extensions/ext_display_control/index.html)"]
impl DisplayEventTypeEXT {
    pub const FIRST_PIXEL_OUT_EXT: Self = Self(0);
}
impl std::fmt::Debug for DisplayEventTypeEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::FIRST_PIXEL_OUT_EXT => "FIRST_PIXEL_OUT_EXT",
            _ => "Unknown enum variant",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSwapchainCounterCreateInfoEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SwapchainCounterCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub surface_counters: crate::extensions::ext_display_surface_counter::SurfaceCounterFlagsEXT,
}
impl SwapchainCounterCreateInfoEXT {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableBySwapchainCounterCreateInfoEXT,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> SwapchainCounterCreateInfoEXTBuilder<'a> {
        SwapchainCounterCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for SwapchainCounterCreateInfoEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("SwapchainCounterCreateInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("surface_counters", &self.surface_counters)
            .finish()
    }
}
impl Default for SwapchainCounterCreateInfoEXT {
    fn default() -> SwapchainCounterCreateInfoEXT {
        SwapchainCounterCreateInfoEXT {
            s_type: crate::vk1_0::StructureType::SWAPCHAIN_COUNTER_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            surface_counters: Default::default(),
        }
    }
}
#[doc = "Used by [`SwapchainCounterCreateInfoEXT::extend`](struct.SwapchainCounterCreateInfoEXT.html#method.extend)"]
pub trait ExtendableBySwapchainCounterCreateInfoEXT {}
impl ExtendableBySwapchainCounterCreateInfoEXT
    for crate::extensions::khr_swapchain::SwapchainCreateInfoKHR
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`SwapchainCounterCreateInfoEXT`](struct.SwapchainCounterCreateInfoEXT.html)"]
#[repr(transparent)]
pub struct SwapchainCounterCreateInfoEXTBuilder<'a>(
    SwapchainCounterCreateInfoEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> SwapchainCounterCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> SwapchainCounterCreateInfoEXTBuilder<'a> {
        SwapchainCounterCreateInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn surface_counters(
        mut self,
        surface_counters: crate::extensions::ext_display_surface_counter::SurfaceCounterFlagsEXT,
    ) -> Self {
        self.0.surface_counters = surface_counters;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> SwapchainCounterCreateInfoEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for SwapchainCounterCreateInfoEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
