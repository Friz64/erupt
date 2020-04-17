# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_EXT_memory_priority.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_MEMORY_PRIORITY_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_MEMORY_PRIORITY_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_memory_priority");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMemoryPriorityFeaturesEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceMemoryPriorityFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub memory_priority: crate::vk1_0::Bool32,
}
impl PhysicalDeviceMemoryPriorityFeaturesEXT {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceMemoryPriorityFeaturesEXT,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceMemoryPriorityFeaturesEXTBuilder<'a> {
        PhysicalDeviceMemoryPriorityFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceMemoryPriorityFeaturesEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceMemoryPriorityFeaturesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("memory_priority", &(self.memory_priority != 0))
            .finish()
    }
}
impl Default for PhysicalDeviceMemoryPriorityFeaturesEXT {
    fn default() -> PhysicalDeviceMemoryPriorityFeaturesEXT {
        PhysicalDeviceMemoryPriorityFeaturesEXT {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_MEMORY_PRIORITY_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            memory_priority: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceMemoryPriorityFeaturesEXT::extend`](struct.PhysicalDeviceMemoryPriorityFeaturesEXT.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceMemoryPriorityFeaturesEXT {}
impl ExtendableByPhysicalDeviceMemoryPriorityFeaturesEXT for crate::vk1_1::PhysicalDeviceFeatures2 {}
impl ExtendableByPhysicalDeviceMemoryPriorityFeaturesEXT for crate::vk1_0::DeviceCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceMemoryPriorityFeaturesEXT`](struct.PhysicalDeviceMemoryPriorityFeaturesEXT.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceMemoryPriorityFeaturesEXTBuilder<'a>(
    PhysicalDeviceMemoryPriorityFeaturesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceMemoryPriorityFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceMemoryPriorityFeaturesEXTBuilder<'a> {
        PhysicalDeviceMemoryPriorityFeaturesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn memory_priority(mut self, memory_priority: bool) -> Self {
        self.0.memory_priority = memory_priority as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceMemoryPriorityFeaturesEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceMemoryPriorityFeaturesEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceMemoryPriorityFeaturesEXTBuilder<'a> {
    type Target = PhysicalDeviceMemoryPriorityFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceMemoryPriorityFeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryPriorityAllocateInfoEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryPriorityAllocateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub priority: f32,
}
impl MemoryPriorityAllocateInfoEXT {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByMemoryPriorityAllocateInfoEXT,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> MemoryPriorityAllocateInfoEXTBuilder<'a> {
        MemoryPriorityAllocateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for MemoryPriorityAllocateInfoEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("MemoryPriorityAllocateInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("priority", &self.priority)
            .finish()
    }
}
impl Default for MemoryPriorityAllocateInfoEXT {
    fn default() -> MemoryPriorityAllocateInfoEXT {
        MemoryPriorityAllocateInfoEXT {
            s_type: crate::vk1_0::StructureType::MEMORY_PRIORITY_ALLOCATE_INFO_EXT,
            p_next: std::ptr::null(),
            priority: Default::default(),
        }
    }
}
#[doc = "Used by [`MemoryPriorityAllocateInfoEXT::extend`](struct.MemoryPriorityAllocateInfoEXT.html#method.extend)"]
pub trait ExtendableByMemoryPriorityAllocateInfoEXT {}
impl ExtendableByMemoryPriorityAllocateInfoEXT for crate::vk1_0::MemoryAllocateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`MemoryPriorityAllocateInfoEXT`](struct.MemoryPriorityAllocateInfoEXT.html)"]
#[repr(transparent)]
pub struct MemoryPriorityAllocateInfoEXTBuilder<'a>(
    MemoryPriorityAllocateInfoEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> MemoryPriorityAllocateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> MemoryPriorityAllocateInfoEXTBuilder<'a> {
        MemoryPriorityAllocateInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn priority(mut self, priority: f32) -> Self {
        self.0.priority = priority;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> MemoryPriorityAllocateInfoEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for MemoryPriorityAllocateInfoEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for MemoryPriorityAllocateInfoEXTBuilder<'a> {
    type Target = MemoryPriorityAllocateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for MemoryPriorityAllocateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
