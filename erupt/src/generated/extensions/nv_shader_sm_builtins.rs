#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_SHADER_SM_BUILTINS_SPEC_VERSION")]
pub const NV_SHADER_SM_BUILTINS_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_SHADER_SM_BUILTINS_EXTENSION_NAME")]
pub const NV_SHADER_SM_BUILTINS_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_NV_shader_sm_builtins");
#[doc = "Provided by [`crate::extensions::nv_shader_sm_builtins`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_SHADER_SM_BUILTINS_FEATURES_NV: Self = Self(1000154000);
    pub const PHYSICAL_DEVICE_SHADER_SM_BUILTINS_PROPERTIES_NV: Self = Self(1000154001);
}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceShaderSMBuiltinsFeaturesNV> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceShaderSMBuiltinsFeaturesNVBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceShaderSMBuiltinsFeaturesNV> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceShaderSMBuiltinsFeaturesNVBuilder<'_>> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceShaderSMBuiltinsPropertiesNV> for crate::vk1_1::PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceShaderSMBuiltinsPropertiesNVBuilder<'_>> for crate::vk1_1::PhysicalDeviceProperties2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderSMBuiltinsPropertiesNV.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceShaderSMBuiltinsPropertiesNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceShaderSMBuiltinsPropertiesNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub shader_sm_count: u32,
    pub shader_warps_per_sm: u32,
}
impl PhysicalDeviceShaderSMBuiltinsPropertiesNV {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_SHADER_SM_BUILTINS_PROPERTIES_NV;
}
impl Default for PhysicalDeviceShaderSMBuiltinsPropertiesNV {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), shader_sm_count: Default::default(), shader_warps_per_sm: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceShaderSMBuiltinsPropertiesNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceShaderSMBuiltinsPropertiesNV").field("s_type", &self.s_type).field("p_next", &self.p_next).field("shader_sm_count", &self.shader_sm_count).field("shader_warps_per_sm", &self.shader_warps_per_sm).finish()
    }
}
impl PhysicalDeviceShaderSMBuiltinsPropertiesNV {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceShaderSMBuiltinsPropertiesNVBuilder<'a> {
        PhysicalDeviceShaderSMBuiltinsPropertiesNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderSMBuiltinsPropertiesNV.html) · Builder of [`PhysicalDeviceShaderSMBuiltinsPropertiesNV`]"]
#[repr(transparent)]
pub struct PhysicalDeviceShaderSMBuiltinsPropertiesNVBuilder<'a>(PhysicalDeviceShaderSMBuiltinsPropertiesNV, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceShaderSMBuiltinsPropertiesNVBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceShaderSMBuiltinsPropertiesNVBuilder<'a> {
        PhysicalDeviceShaderSMBuiltinsPropertiesNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn shader_sm_count(mut self, shader_sm_count: u32) -> Self {
        self.0.shader_sm_count = shader_sm_count as _;
        self
    }
    #[inline]
    #[must_use]
    pub fn shader_warps_per_sm(mut self, shader_warps_per_sm: u32) -> Self {
        self.0.shader_warps_per_sm = shader_warps_per_sm as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> PhysicalDeviceShaderSMBuiltinsPropertiesNV {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceShaderSMBuiltinsPropertiesNVBuilder<'a> {
    fn default() -> PhysicalDeviceShaderSMBuiltinsPropertiesNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceShaderSMBuiltinsPropertiesNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceShaderSMBuiltinsPropertiesNVBuilder<'a> {
    type Target = PhysicalDeviceShaderSMBuiltinsPropertiesNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceShaderSMBuiltinsPropertiesNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderSMBuiltinsFeaturesNV.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceShaderSMBuiltinsFeaturesNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceShaderSMBuiltinsFeaturesNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub shader_sm_builtins: crate::vk1_0::Bool32,
}
impl PhysicalDeviceShaderSMBuiltinsFeaturesNV {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_SHADER_SM_BUILTINS_FEATURES_NV;
}
impl Default for PhysicalDeviceShaderSMBuiltinsFeaturesNV {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), shader_sm_builtins: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceShaderSMBuiltinsFeaturesNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceShaderSMBuiltinsFeaturesNV").field("s_type", &self.s_type).field("p_next", &self.p_next).field("shader_sm_builtins", &(self.shader_sm_builtins != 0)).finish()
    }
}
impl PhysicalDeviceShaderSMBuiltinsFeaturesNV {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceShaderSMBuiltinsFeaturesNVBuilder<'a> {
        PhysicalDeviceShaderSMBuiltinsFeaturesNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderSMBuiltinsFeaturesNV.html) · Builder of [`PhysicalDeviceShaderSMBuiltinsFeaturesNV`]"]
#[repr(transparent)]
pub struct PhysicalDeviceShaderSMBuiltinsFeaturesNVBuilder<'a>(PhysicalDeviceShaderSMBuiltinsFeaturesNV, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceShaderSMBuiltinsFeaturesNVBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceShaderSMBuiltinsFeaturesNVBuilder<'a> {
        PhysicalDeviceShaderSMBuiltinsFeaturesNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn shader_sm_builtins(mut self, shader_sm_builtins: bool) -> Self {
        self.0.shader_sm_builtins = shader_sm_builtins as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> PhysicalDeviceShaderSMBuiltinsFeaturesNV {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceShaderSMBuiltinsFeaturesNVBuilder<'a> {
    fn default() -> PhysicalDeviceShaderSMBuiltinsFeaturesNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceShaderSMBuiltinsFeaturesNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceShaderSMBuiltinsFeaturesNVBuilder<'a> {
    type Target = PhysicalDeviceShaderSMBuiltinsFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceShaderSMBuiltinsFeaturesNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
