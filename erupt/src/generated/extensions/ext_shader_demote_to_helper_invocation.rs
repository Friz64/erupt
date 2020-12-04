#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_SHADER_DEMOTE_TO_HELPER_INVOCATION_SPEC_VERSION")]
pub const EXT_SHADER_DEMOTE_TO_HELPER_INVOCATION_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_SHADER_DEMOTE_TO_HELPER_INVOCATION_EXTENSION_NAME")]
pub const EXT_SHADER_DEMOTE_TO_HELPER_INVOCATION_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_shader_demote_to_helper_invocation");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub shader_demote_to_helper_invocation: crate::vk1_0::Bool32,
}
impl Default for PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT {
    fn default() -> Self {
        Self { s_type : crate :: vk1_0 :: StructureType :: PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES_EXT , p_next : std :: ptr :: null_mut () , shader_demote_to_helper_invocation : Default :: default () }
    }
}
impl std::fmt::Debug for PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "shader_demote_to_helper_invocation",
                &(self.shader_demote_to_helper_invocation != 0),
            )
            .finish()
    }
}
impl PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT {
    #[inline]
    pub fn into_builder<'a>(
        self,
    ) -> PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXTBuilder<'a> {
        PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXTBuilder(
            self,
            std::marker::PhantomData,
        )
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT.html) · Builder of [`PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT`]"]
#[repr(transparent)]
pub struct PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXTBuilder<'a>(
    PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXTBuilder<'a> {
        PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[inline]
    pub fn shader_demote_to_helper_invocation(
        mut self,
        shader_demote_to_helper_invocation: bool,
    ) -> Self {
        self.0.shader_demote_to_helper_invocation = shader_demote_to_helper_invocation as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT {
        self.0
    }
}
impl<'a> std::default::Default
    for PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXTBuilder<'a>
{
    fn default() -> PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXTBuilder<'a> {
    type Target = PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
