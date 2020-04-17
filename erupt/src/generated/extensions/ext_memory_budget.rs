# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_EXT_memory_budget.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_MEMORY_BUDGET_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_MEMORY_BUDGET_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_memory_budget");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMemoryBudgetPropertiesEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceMemoryBudgetPropertiesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub heap_budget: [crate::vk1_0::DeviceSize; crate::vk1_0::MAX_MEMORY_HEAPS as usize],
    pub heap_usage: [crate::vk1_0::DeviceSize; crate::vk1_0::MAX_MEMORY_HEAPS as usize],
}
impl PhysicalDeviceMemoryBudgetPropertiesEXT {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceMemoryBudgetPropertiesEXT,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceMemoryBudgetPropertiesEXTBuilder<'a> {
        PhysicalDeviceMemoryBudgetPropertiesEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceMemoryBudgetPropertiesEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceMemoryBudgetPropertiesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("heap_budget", &self.heap_budget)
            .field("heap_usage", &self.heap_usage)
            .finish()
    }
}
impl Default for PhysicalDeviceMemoryBudgetPropertiesEXT {
    fn default() -> PhysicalDeviceMemoryBudgetPropertiesEXT {
        PhysicalDeviceMemoryBudgetPropertiesEXT {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_MEMORY_BUDGET_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            heap_budget: Default::default(),
            heap_usage: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceMemoryBudgetPropertiesEXT::extend`](struct.PhysicalDeviceMemoryBudgetPropertiesEXT.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceMemoryBudgetPropertiesEXT {}
impl ExtendableByPhysicalDeviceMemoryBudgetPropertiesEXT
    for crate::vk1_1::PhysicalDeviceMemoryProperties2
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceMemoryBudgetPropertiesEXT`](struct.PhysicalDeviceMemoryBudgetPropertiesEXT.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceMemoryBudgetPropertiesEXTBuilder<'a>(
    PhysicalDeviceMemoryBudgetPropertiesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceMemoryBudgetPropertiesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceMemoryBudgetPropertiesEXTBuilder<'a> {
        PhysicalDeviceMemoryBudgetPropertiesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn heap_budget(
        mut self,
        heap_budget: [crate::vk1_0::DeviceSize; crate::vk1_0::MAX_MEMORY_HEAPS as usize],
    ) -> Self {
        self.0.heap_budget = heap_budget;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn heap_usage(
        mut self,
        heap_usage: [crate::vk1_0::DeviceSize; crate::vk1_0::MAX_MEMORY_HEAPS as usize],
    ) -> Self {
        self.0.heap_usage = heap_usage;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceMemoryBudgetPropertiesEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceMemoryBudgetPropertiesEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceMemoryBudgetPropertiesEXTBuilder<'a> {
    type Target = PhysicalDeviceMemoryBudgetPropertiesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceMemoryBudgetPropertiesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
