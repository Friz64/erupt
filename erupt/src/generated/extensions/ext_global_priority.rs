# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_EXT_global_priority.html)\n\n## Extends\n- [`Result`](../../vk1_0/struct.Result.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_GLOBAL_PRIORITY_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_GLOBAL_PRIORITY_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_global_priority");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceQueueGlobalPriorityCreateInfoEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DeviceQueueGlobalPriorityCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub global_priority: crate::extensions::ext_global_priority::QueueGlobalPriorityEXT,
}
impl DeviceQueueGlobalPriorityCreateInfoEXT {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByDeviceQueueGlobalPriorityCreateInfoEXT,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> DeviceQueueGlobalPriorityCreateInfoEXTBuilder<'a> {
        DeviceQueueGlobalPriorityCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for DeviceQueueGlobalPriorityCreateInfoEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("DeviceQueueGlobalPriorityCreateInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("global_priority", &self.global_priority)
            .finish()
    }
}
impl Default for DeviceQueueGlobalPriorityCreateInfoEXT {
    fn default() -> DeviceQueueGlobalPriorityCreateInfoEXT {
        DeviceQueueGlobalPriorityCreateInfoEXT {
            s_type: crate::vk1_0::StructureType::DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            global_priority: Default::default(),
        }
    }
}
#[doc = "Used by [`DeviceQueueGlobalPriorityCreateInfoEXT::extend`](struct.DeviceQueueGlobalPriorityCreateInfoEXT.html#method.extend)"]
pub trait ExtendableByDeviceQueueGlobalPriorityCreateInfoEXT {}
impl ExtendableByDeviceQueueGlobalPriorityCreateInfoEXT for crate::vk1_0::DeviceQueueCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`DeviceQueueGlobalPriorityCreateInfoEXT`](struct.DeviceQueueGlobalPriorityCreateInfoEXT.html)"]
#[repr(transparent)]
pub struct DeviceQueueGlobalPriorityCreateInfoEXTBuilder<'a>(
    DeviceQueueGlobalPriorityCreateInfoEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> DeviceQueueGlobalPriorityCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> DeviceQueueGlobalPriorityCreateInfoEXTBuilder<'a> {
        DeviceQueueGlobalPriorityCreateInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn global_priority(
        mut self,
        global_priority: crate::extensions::ext_global_priority::QueueGlobalPriorityEXT,
    ) -> Self {
        self.0.global_priority = global_priority;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> DeviceQueueGlobalPriorityCreateInfoEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for DeviceQueueGlobalPriorityCreateInfoEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueueGlobalPriorityEXT.html) · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct QueueGlobalPriorityEXT(pub i32);
#[doc = "[Part of `extensions::ext_global_priority`](../../extensions/ext_global_priority/index.html)"]
impl QueueGlobalPriorityEXT {
    pub const LOW_EXT: Self = Self(128);
    pub const MEDIUM_EXT: Self = Self(256);
    pub const HIGH_EXT: Self = Self(512);
    pub const REALTIME_EXT: Self = Self(1024);
}
impl std::fmt::Debug for QueueGlobalPriorityEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::LOW_EXT => "LOW_EXT",
            &Self::MEDIUM_EXT => "MEDIUM_EXT",
            &Self::HIGH_EXT => "HIGH_EXT",
            &Self::REALTIME_EXT => "REALTIME_EXT",
            _ => "(Unknown)",
        })
    }
}
