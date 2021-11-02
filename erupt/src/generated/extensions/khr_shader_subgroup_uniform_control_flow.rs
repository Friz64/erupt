#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_SPEC_VERSION")]
pub const KHR_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_EXTENSION_NAME")]
pub const KHR_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_shader_subgroup_uniform_control_flow");
#[doc = "Provided by [`crate::extensions::khr_shader_subgroup_uniform_control_flow`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_FEATURES_KHR: Self = Self(1000323000);
}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHRBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHRBuilder<'_>> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub shader_subgroup_uniform_control_flow: crate::vk1_0::Bool32,
}
impl PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_FEATURES_KHR;
}
impl Default for PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), shader_subgroup_uniform_control_flow: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("shader_subgroup_uniform_control_flow", &(self.shader_subgroup_uniform_control_flow != 0)).finish()
    }
}
impl PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHRBuilder<'a> {
        PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR.html) · Builder of [`PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR`]"]
#[repr(transparent)]
pub struct PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHRBuilder<'a>(PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHRBuilder<'a> {
        PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn shader_subgroup_uniform_control_flow(mut self, shader_subgroup_uniform_control_flow: bool) -> Self {
        self.0.shader_subgroup_uniform_control_flow = shader_subgroup_uniform_control_flow as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHRBuilder<'a> {
    fn default() -> PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHRBuilder<'a> {
    type Target = PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
