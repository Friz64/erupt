#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_MAX_GLOBAL_PRIORITY_SIZE_EXT")]
pub const MAX_GLOBAL_PRIORITY_SIZE_EXT: u32 = 16;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_GLOBAL_PRIORITY_QUERY_SPEC_VERSION")]
pub const EXT_GLOBAL_PRIORITY_QUERY_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_GLOBAL_PRIORITY_QUERY_EXTENSION_NAME")]
pub const EXT_GLOBAL_PRIORITY_QUERY_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_global_priority_query");
#[doc = "Provided by [`crate::extensions::ext_global_priority_query`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_EXT: Self = Self(1000388000);
    pub const QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_EXT: Self = Self(1000388001);
}
impl<'a> crate::ExtendableFromConst<'a, PhysicalDeviceGlobalPriorityQueryFeaturesEXT> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, PhysicalDeviceGlobalPriorityQueryFeaturesEXTBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceGlobalPriorityQueryFeaturesEXT> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceGlobalPriorityQueryFeaturesEXTBuilder<'_>> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, QueueFamilyGlobalPriorityPropertiesEXT> for crate::vk1_1::QueueFamilyProperties2Builder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, QueueFamilyGlobalPriorityPropertiesEXTBuilder<'_>> for crate::vk1_1::QueueFamilyProperties2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceGlobalPriorityQueryFeaturesEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceGlobalPriorityQueryFeaturesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceGlobalPriorityQueryFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub global_priority_query: crate::vk1_0::Bool32,
}
impl Default for PhysicalDeviceGlobalPriorityQueryFeaturesEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_EXT, p_next: std::ptr::null_mut(), global_priority_query: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceGlobalPriorityQueryFeaturesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceGlobalPriorityQueryFeaturesEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("global_priority_query", &(self.global_priority_query != 0)).finish()
    }
}
impl PhysicalDeviceGlobalPriorityQueryFeaturesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceGlobalPriorityQueryFeaturesEXTBuilder<'a> {
        PhysicalDeviceGlobalPriorityQueryFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceGlobalPriorityQueryFeaturesEXT.html) · Builder of [`PhysicalDeviceGlobalPriorityQueryFeaturesEXT`]"]
#[repr(transparent)]
pub struct PhysicalDeviceGlobalPriorityQueryFeaturesEXTBuilder<'a>(PhysicalDeviceGlobalPriorityQueryFeaturesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceGlobalPriorityQueryFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceGlobalPriorityQueryFeaturesEXTBuilder<'a> {
        PhysicalDeviceGlobalPriorityQueryFeaturesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn global_priority_query(mut self, global_priority_query: bool) -> Self {
        self.0.global_priority_query = global_priority_query as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceGlobalPriorityQueryFeaturesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceGlobalPriorityQueryFeaturesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceGlobalPriorityQueryFeaturesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceGlobalPriorityQueryFeaturesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceGlobalPriorityQueryFeaturesEXTBuilder<'a> {
    type Target = PhysicalDeviceGlobalPriorityQueryFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceGlobalPriorityQueryFeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueueFamilyGlobalPriorityPropertiesEXT.html) · Structure"]
#[doc(alias = "VkQueueFamilyGlobalPriorityPropertiesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct QueueFamilyGlobalPriorityPropertiesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub priority_count: u32,
    pub priorities: [crate::extensions::ext_global_priority::QueueGlobalPriorityEXT; 16],
}
impl Default for QueueFamilyGlobalPriorityPropertiesEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_EXT, p_next: std::ptr::null_mut(), priority_count: Default::default(), priorities: unsafe { std::mem::zeroed() } }
    }
}
impl std::fmt::Debug for QueueFamilyGlobalPriorityPropertiesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("QueueFamilyGlobalPriorityPropertiesEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("priority_count", &self.priority_count).field("priorities", &self.priorities).finish()
    }
}
impl QueueFamilyGlobalPriorityPropertiesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> QueueFamilyGlobalPriorityPropertiesEXTBuilder<'a> {
        QueueFamilyGlobalPriorityPropertiesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueueFamilyGlobalPriorityPropertiesEXT.html) · Builder of [`QueueFamilyGlobalPriorityPropertiesEXT`]"]
#[repr(transparent)]
pub struct QueueFamilyGlobalPriorityPropertiesEXTBuilder<'a>(QueueFamilyGlobalPriorityPropertiesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> QueueFamilyGlobalPriorityPropertiesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> QueueFamilyGlobalPriorityPropertiesEXTBuilder<'a> {
        QueueFamilyGlobalPriorityPropertiesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn priorities(mut self, priorities: &'a [crate::extensions::ext_global_priority::QueueGlobalPriorityEXT]) -> Self {
        let mut priorities_array = [Default::default(); 16];
        let truncated_len = priorities.len().min(priorities_array.len());
        priorities_array[..truncated_len].copy_from_slice(&priorities[..truncated_len]);
        self.0.priority_count = truncated_len as _;
        self.0.priorities = priorities_array;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> QueueFamilyGlobalPriorityPropertiesEXT {
        self.0
    }
}
impl<'a> std::default::Default for QueueFamilyGlobalPriorityPropertiesEXTBuilder<'a> {
    fn default() -> QueueFamilyGlobalPriorityPropertiesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for QueueFamilyGlobalPriorityPropertiesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for QueueFamilyGlobalPriorityPropertiesEXTBuilder<'a> {
    type Target = QueueFamilyGlobalPriorityPropertiesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for QueueFamilyGlobalPriorityPropertiesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
