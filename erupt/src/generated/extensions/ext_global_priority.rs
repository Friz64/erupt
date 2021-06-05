#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_GLOBAL_PRIORITY_SPEC_VERSION")]
pub const EXT_GLOBAL_PRIORITY_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_GLOBAL_PRIORITY_EXTENSION_NAME")]
pub const EXT_GLOBAL_PRIORITY_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_global_priority");
#[doc = "Provided by [`crate::extensions::ext_global_priority`]"]
impl crate::vk1_0::Result {
    pub const ERROR_NOT_PERMITTED_EXT: Self = Self(-1000174001);
}
#[doc = "Provided by [`crate::extensions::ext_global_priority`]"]
impl crate::vk1_0::StructureType {
    pub const DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_EXT: Self = Self(1000174000);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueueGlobalPriorityEXT.html) · Enum"]
#[doc(alias = "VkQueueGlobalPriorityEXT")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct QueueGlobalPriorityEXT(pub i32);
impl std::fmt::Debug for QueueGlobalPriorityEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            #[cfg(feature = "ext_global_priority")]
            &Self::LOW_EXT => "LOW_EXT",
            #[cfg(feature = "ext_global_priority")]
            &Self::MEDIUM_EXT => "MEDIUM_EXT",
            #[cfg(feature = "ext_global_priority")]
            &Self::HIGH_EXT => "HIGH_EXT",
            #[cfg(feature = "ext_global_priority")]
            &Self::REALTIME_EXT => "REALTIME_EXT",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::ext_global_priority`]"]
impl crate::extensions::ext_global_priority::QueueGlobalPriorityEXT {
    pub const LOW_EXT: Self = Self(128);
    pub const MEDIUM_EXT: Self = Self(256);
    pub const HIGH_EXT: Self = Self(512);
    pub const REALTIME_EXT: Self = Self(1024);
}
impl<'a> crate::ExtendableFromConst<'a, DeviceQueueGlobalPriorityCreateInfoEXT> for crate::vk1_0::DeviceQueueCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, DeviceQueueGlobalPriorityCreateInfoEXTBuilder<'_>> for crate::vk1_0::DeviceQueueCreateInfoBuilder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceQueueGlobalPriorityCreateInfoEXT.html) · Structure"]
#[doc(alias = "VkDeviceQueueGlobalPriorityCreateInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DeviceQueueGlobalPriorityCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub global_priority: crate::extensions::ext_global_priority::QueueGlobalPriorityEXT,
}
impl Default for DeviceQueueGlobalPriorityCreateInfoEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_EXT, p_next: std::ptr::null(), global_priority: Default::default() }
    }
}
impl std::fmt::Debug for DeviceQueueGlobalPriorityCreateInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DeviceQueueGlobalPriorityCreateInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("global_priority", &self.global_priority).finish()
    }
}
impl DeviceQueueGlobalPriorityCreateInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> DeviceQueueGlobalPriorityCreateInfoEXTBuilder<'a> {
        DeviceQueueGlobalPriorityCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceQueueGlobalPriorityCreateInfoEXT.html) · Builder of [`DeviceQueueGlobalPriorityCreateInfoEXT`]"]
#[repr(transparent)]
pub struct DeviceQueueGlobalPriorityCreateInfoEXTBuilder<'a>(DeviceQueueGlobalPriorityCreateInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> DeviceQueueGlobalPriorityCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> DeviceQueueGlobalPriorityCreateInfoEXTBuilder<'a> {
        DeviceQueueGlobalPriorityCreateInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn global_priority(mut self, global_priority: crate::extensions::ext_global_priority::QueueGlobalPriorityEXT) -> Self {
        self.0.global_priority = global_priority as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DeviceQueueGlobalPriorityCreateInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for DeviceQueueGlobalPriorityCreateInfoEXTBuilder<'a> {
    fn default() -> DeviceQueueGlobalPriorityCreateInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DeviceQueueGlobalPriorityCreateInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DeviceQueueGlobalPriorityCreateInfoEXTBuilder<'a> {
    type Target = DeviceQueueGlobalPriorityCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DeviceQueueGlobalPriorityCreateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
