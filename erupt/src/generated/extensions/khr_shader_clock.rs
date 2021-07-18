#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_SHADER_CLOCK_SPEC_VERSION")]
pub const KHR_SHADER_CLOCK_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_SHADER_CLOCK_EXTENSION_NAME")]
pub const KHR_SHADER_CLOCK_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_shader_clock");
#[doc = "Provided by [`crate::extensions::khr_shader_clock`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_SHADER_CLOCK_FEATURES_KHR: Self = Self(1000181000);
}
impl<'a> crate::ExtendableFromConst<'a, PhysicalDeviceShaderClockFeaturesKHR> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, PhysicalDeviceShaderClockFeaturesKHRBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceShaderClockFeaturesKHR> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceShaderClockFeaturesKHRBuilder<'_>> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderClockFeaturesKHR.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceShaderClockFeaturesKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceShaderClockFeaturesKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub shader_subgroup_clock: crate::vk1_0::Bool32,
    pub shader_device_clock: crate::vk1_0::Bool32,
}
impl PhysicalDeviceShaderClockFeaturesKHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_SHADER_CLOCK_FEATURES_KHR;
}
impl Default for PhysicalDeviceShaderClockFeaturesKHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), shader_subgroup_clock: Default::default(), shader_device_clock: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceShaderClockFeaturesKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceShaderClockFeaturesKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("shader_subgroup_clock", &(self.shader_subgroup_clock != 0)).field("shader_device_clock", &(self.shader_device_clock != 0)).finish()
    }
}
impl PhysicalDeviceShaderClockFeaturesKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceShaderClockFeaturesKHRBuilder<'a> {
        PhysicalDeviceShaderClockFeaturesKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderClockFeaturesKHR.html) · Builder of [`PhysicalDeviceShaderClockFeaturesKHR`]"]
#[repr(transparent)]
pub struct PhysicalDeviceShaderClockFeaturesKHRBuilder<'a>(PhysicalDeviceShaderClockFeaturesKHR, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceShaderClockFeaturesKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceShaderClockFeaturesKHRBuilder<'a> {
        PhysicalDeviceShaderClockFeaturesKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn shader_subgroup_clock(mut self, shader_subgroup_clock: bool) -> Self {
        self.0.shader_subgroup_clock = shader_subgroup_clock as _;
        self
    }
    #[inline]
    pub fn shader_device_clock(mut self, shader_device_clock: bool) -> Self {
        self.0.shader_device_clock = shader_device_clock as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceShaderClockFeaturesKHR {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceShaderClockFeaturesKHRBuilder<'a> {
    fn default() -> PhysicalDeviceShaderClockFeaturesKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceShaderClockFeaturesKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceShaderClockFeaturesKHRBuilder<'a> {
    type Target = PhysicalDeviceShaderClockFeaturesKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceShaderClockFeaturesKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
