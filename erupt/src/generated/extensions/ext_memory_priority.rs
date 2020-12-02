#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_MEMORY_PRIORITY_SPEC_VERSION")]
pub const EXT_MEMORY_PRIORITY_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_MEMORY_PRIORITY_EXTENSION_NAME")]
pub const EXT_MEMORY_PRIORITY_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_memory_priority");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMemoryPriorityFeaturesEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceMemoryPriorityFeaturesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceMemoryPriorityFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub memory_priority: crate::vk1_0::Bool32,
}
impl Default for PhysicalDeviceMemoryPriorityFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_MEMORY_PRIORITY_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            memory_priority: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceMemoryPriorityFeaturesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceMemoryPriorityFeaturesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("memory_priority", &(self.memory_priority != 0))
            .finish()
    }
}
impl PhysicalDeviceMemoryPriorityFeaturesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceMemoryPriorityFeaturesEXTBuilder<'a> {
        PhysicalDeviceMemoryPriorityFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMemoryPriorityFeaturesEXT.html) · Builder of [`PhysicalDeviceMemoryPriorityFeaturesEXT`](struct.PhysicalDeviceMemoryPriorityFeaturesEXT.html)"]
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
    #[inline]
    pub fn memory_priority(mut self, memory_priority: bool) -> Self {
        self.0.memory_priority = memory_priority as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceMemoryPriorityFeaturesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceMemoryPriorityFeaturesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceMemoryPriorityFeaturesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceMemoryPriorityFeaturesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
#[doc(alias = "VkMemoryPriorityAllocateInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryPriorityAllocateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub priority: std::os::raw::c_float,
}
impl Default for MemoryPriorityAllocateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::MEMORY_PRIORITY_ALLOCATE_INFO_EXT,
            p_next: std::ptr::null(),
            priority: Default::default(),
        }
    }
}
impl std::fmt::Debug for MemoryPriorityAllocateInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MemoryPriorityAllocateInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("priority", &self.priority)
            .finish()
    }
}
impl MemoryPriorityAllocateInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> MemoryPriorityAllocateInfoEXTBuilder<'a> {
        MemoryPriorityAllocateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryPriorityAllocateInfoEXT.html) · Builder of [`MemoryPriorityAllocateInfoEXT`](struct.MemoryPriorityAllocateInfoEXT.html)"]
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
    #[inline]
    pub fn priority(mut self, priority: std::os::raw::c_float) -> Self {
        self.0.priority = priority as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> MemoryPriorityAllocateInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for MemoryPriorityAllocateInfoEXTBuilder<'a> {
    fn default() -> MemoryPriorityAllocateInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for MemoryPriorityAllocateInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
