# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_KHR_shader_clock.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_SHADER_CLOCK_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_SHADER_CLOCK_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_shader_clock");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderClockFeaturesKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceShaderClockFeaturesKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub shader_subgroup_clock: crate::vk1_0::Bool32,
    pub shader_device_clock: crate::vk1_0::Bool32,
}
impl PhysicalDeviceShaderClockFeaturesKHR {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: crate::ExtendableBy<Self>,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceShaderClockFeaturesKHRBuilder<'a> {
        PhysicalDeviceShaderClockFeaturesKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceShaderClockFeaturesKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceShaderClockFeaturesKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("shader_subgroup_clock", &(self.shader_subgroup_clock != 0))
            .field("shader_device_clock", &(self.shader_device_clock != 0))
            .finish()
    }
}
impl Default for PhysicalDeviceShaderClockFeaturesKHR {
    fn default() -> PhysicalDeviceShaderClockFeaturesKHR {
        PhysicalDeviceShaderClockFeaturesKHR {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_SHADER_CLOCK_FEATURES_KHR,
            p_next: std::ptr::null_mut(),
            shader_subgroup_clock: Default::default(),
            shader_device_clock: Default::default(),
        }
    }
}
impl crate::ExtendableBy<PhysicalDeviceShaderClockFeaturesKHR>
    for crate::vk1_1::PhysicalDeviceFeatures2
{
}
impl crate::ExtendableBy<PhysicalDeviceShaderClockFeaturesKHR> for crate::vk1_0::DeviceCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceShaderClockFeaturesKHR`](struct.PhysicalDeviceShaderClockFeaturesKHR.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceShaderClockFeaturesKHRBuilder<'a>(
    PhysicalDeviceShaderClockFeaturesKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceShaderClockFeaturesKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceShaderClockFeaturesKHRBuilder<'a> {
        PhysicalDeviceShaderClockFeaturesKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_subgroup_clock(mut self, shader_subgroup_clock: bool) -> Self {
        self.0.shader_subgroup_clock = shader_subgroup_clock as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_device_clock(mut self, shader_device_clock: bool) -> Self {
        self.0.shader_device_clock = shader_device_clock as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceShaderClockFeaturesKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceShaderClockFeaturesKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
