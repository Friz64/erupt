# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_EXT_shader_demote_to_helper_invocation.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_SHADER_DEMOTE_TO_HELPER_INVOCATION_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_SHADER_DEMOTE_TO_HELPER_INVOCATION_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_shader_demote_to_helper_invocation");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub shader_demote_to_helper_invocation: crate::vk1_0::Bool32,
}
impl PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXTBuilder<'a> {
        PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXTBuilder(
            self,
            std::marker::PhantomData,
        )
    }
}
impl std::fmt::Debug for PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "shader_demote_to_helper_invocation",
                &(self.shader_demote_to_helper_invocation != 0),
            )
            .finish()
    }
}
impl Default for PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT {
    fn default() -> PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT {
        PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT { s_type : crate :: vk1_0 :: StructureType :: PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES_EXT , p_next : std :: ptr :: null_mut ( ) , shader_demote_to_helper_invocation : Default :: default ( ) , }
    }
}
#[doc = "Used by [`PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT::extend`](struct.PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT {}
impl ExtendableByPhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT
    for crate::vk1_1::PhysicalDeviceFeatures2
{
}
impl ExtendableByPhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT
    for crate::vk1_0::DeviceCreateInfo
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT`](struct.PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT.html)"]
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
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_demote_to_helper_invocation(
        mut self,
        shader_demote_to_helper_invocation: bool,
    ) -> Self {
        self.0.shader_demote_to_helper_invocation = shader_demote_to_helper_invocation as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
