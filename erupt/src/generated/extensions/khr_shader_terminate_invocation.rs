#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_SHADER_TERMINATE_INVOCATION_SPEC_VERSION")]
pub const KHR_SHADER_TERMINATE_INVOCATION_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_SHADER_TERMINATE_INVOCATION_EXTENSION_NAME")]
pub const KHR_SHADER_TERMINATE_INVOCATION_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_shader_terminate_invocation");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderTerminateInvocationFeaturesKHR.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceShaderTerminateInvocationFeaturesKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceShaderTerminateInvocationFeaturesKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub shader_terminate_invocation: crate::vk1_0::Bool32,
}
impl Default for PhysicalDeviceShaderTerminateInvocationFeaturesKHR {
    fn default() -> Self {
        Self { s_type : crate :: vk1_0 :: StructureType :: PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES_KHR , p_next : std :: ptr :: null_mut () , shader_terminate_invocation : Default :: default () }
    }
}
impl std::fmt::Debug for PhysicalDeviceShaderTerminateInvocationFeaturesKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceShaderTerminateInvocationFeaturesKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "shader_terminate_invocation",
                &(self.shader_terminate_invocation != 0),
            )
            .finish()
    }
}
impl PhysicalDeviceShaderTerminateInvocationFeaturesKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceShaderTerminateInvocationFeaturesKHRBuilder<'a> {
        PhysicalDeviceShaderTerminateInvocationFeaturesKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderTerminateInvocationFeaturesKHR.html) · Builder of [`PhysicalDeviceShaderTerminateInvocationFeaturesKHR`](struct.PhysicalDeviceShaderTerminateInvocationFeaturesKHR.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceShaderTerminateInvocationFeaturesKHRBuilder<'a>(
    PhysicalDeviceShaderTerminateInvocationFeaturesKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceShaderTerminateInvocationFeaturesKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceShaderTerminateInvocationFeaturesKHRBuilder<'a> {
        PhysicalDeviceShaderTerminateInvocationFeaturesKHRBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[inline]
    pub fn shader_terminate_invocation(mut self, shader_terminate_invocation: bool) -> Self {
        self.0.shader_terminate_invocation = shader_terminate_invocation as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceShaderTerminateInvocationFeaturesKHR {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceShaderTerminateInvocationFeaturesKHRBuilder<'a> {
    fn default() -> PhysicalDeviceShaderTerminateInvocationFeaturesKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceShaderTerminateInvocationFeaturesKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceShaderTerminateInvocationFeaturesKHRBuilder<'a> {
    type Target = PhysicalDeviceShaderTerminateInvocationFeaturesKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceShaderTerminateInvocationFeaturesKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
