#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_ZERO_INITIALIZE_WORKGROUP_MEMORY_SPEC_VERSION")]
pub const KHR_ZERO_INITIALIZE_WORKGROUP_MEMORY_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_ZERO_INITIALIZE_WORKGROUP_MEMORY_EXTENSION_NAME")]
pub const KHR_ZERO_INITIALIZE_WORKGROUP_MEMORY_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_zero_initialize_workgroup_memory");
#[doc = "Provided by [`crate::extensions::khr_zero_initialize_workgroup_memory`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES_KHR: Self = Self(1000325000);
}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesKHR> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesKHRBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesKHR> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesKHRBuilder<'_>> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesKHR.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub shader_zero_initialize_workgroup_memory: crate::vk1_0::Bool32,
}
impl PhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesKHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES_KHR;
}
impl Default for PhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesKHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), shader_zero_initialize_workgroup_memory: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("shader_zero_initialize_workgroup_memory", &(self.shader_zero_initialize_workgroup_memory != 0)).finish()
    }
}
impl PhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesKHRBuilder<'a> {
        PhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesKHR.html) · Builder of [`PhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesKHR`]"]
#[repr(transparent)]
pub struct PhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesKHRBuilder<'a>(PhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesKHR, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesKHRBuilder<'a> {
        PhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn shader_zero_initialize_workgroup_memory(mut self, shader_zero_initialize_workgroup_memory: bool) -> Self {
        self.0.shader_zero_initialize_workgroup_memory = shader_zero_initialize_workgroup_memory as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> PhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesKHR {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesKHRBuilder<'a> {
    fn default() -> PhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesKHRBuilder<'a> {
    type Target = PhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
