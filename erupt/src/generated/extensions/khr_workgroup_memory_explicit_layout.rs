#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_SPEC_VERSION")]
pub const KHR_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_EXTENSION_NAME")]
pub const KHR_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_workgroup_memory_explicit_layout");
#[doc = "Provided by [`crate::extensions::khr_workgroup_memory_explicit_layout`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_FEATURES_KHR: Self = Self(1000336000);
}
impl<'a> crate::ExtendableFromConst<'a, PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHRBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHRBuilder<'_>> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub workgroup_memory_explicit_layout: crate::vk1_0::Bool32,
    pub workgroup_memory_explicit_layout_scalar_block_layout: crate::vk1_0::Bool32,
    pub workgroup_memory_explicit_layout8_bit_access: crate::vk1_0::Bool32,
    pub workgroup_memory_explicit_layout16_bit_access: crate::vk1_0::Bool32,
}
impl PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_FEATURES_KHR;
}
impl Default for PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), workgroup_memory_explicit_layout: Default::default(), workgroup_memory_explicit_layout_scalar_block_layout: Default::default(), workgroup_memory_explicit_layout8_bit_access: Default::default(), workgroup_memory_explicit_layout16_bit_access: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("workgroup_memory_explicit_layout", &(self.workgroup_memory_explicit_layout != 0)).field("workgroup_memory_explicit_layout_scalar_block_layout", &(self.workgroup_memory_explicit_layout_scalar_block_layout != 0)).field("workgroup_memory_explicit_layout8_bit_access", &(self.workgroup_memory_explicit_layout8_bit_access != 0)).field("workgroup_memory_explicit_layout16_bit_access", &(self.workgroup_memory_explicit_layout16_bit_access != 0)).finish()
    }
}
impl PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHRBuilder<'a> {
        PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR.html) · Builder of [`PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR`]"]
#[repr(transparent)]
pub struct PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHRBuilder<'a>(PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHRBuilder<'a> {
        PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn workgroup_memory_explicit_layout(mut self, workgroup_memory_explicit_layout: bool) -> Self {
        self.0.workgroup_memory_explicit_layout = workgroup_memory_explicit_layout as _;
        self
    }
    #[inline]
    pub fn workgroup_memory_explicit_layout_scalar_block_layout(mut self, workgroup_memory_explicit_layout_scalar_block_layout: bool) -> Self {
        self.0.workgroup_memory_explicit_layout_scalar_block_layout = workgroup_memory_explicit_layout_scalar_block_layout as _;
        self
    }
    #[inline]
    pub fn workgroup_memory_explicit_layout8_bit_access(mut self, workgroup_memory_explicit_layout8_bit_access: bool) -> Self {
        self.0.workgroup_memory_explicit_layout8_bit_access = workgroup_memory_explicit_layout8_bit_access as _;
        self
    }
    #[inline]
    pub fn workgroup_memory_explicit_layout16_bit_access(mut self, workgroup_memory_explicit_layout16_bit_access: bool) -> Self {
        self.0.workgroup_memory_explicit_layout16_bit_access = workgroup_memory_explicit_layout16_bit_access as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHRBuilder<'a> {
    fn default() -> PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHRBuilder<'a> {
    type Target = PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
