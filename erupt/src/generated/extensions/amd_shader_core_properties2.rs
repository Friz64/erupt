#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_AMD_SHADER_CORE_PROPERTIES_2_SPEC_VERSION")]
pub const AMD_SHADER_CORE_PROPERTIES_2_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_AMD_SHADER_CORE_PROPERTIES_2_EXTENSION_NAME")]
pub const AMD_SHADER_CORE_PROPERTIES_2_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_AMD_shader_core_properties2");
#[doc = "Provided by [`crate::extensions::amd_shader_core_properties2`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_2_AMD: Self = Self(1000227000);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkShaderCorePropertiesFlagsAMD.html) · Bitmask of [`ShaderCorePropertiesFlagBitsAMD`]"] # [doc (alias = "VkShaderCorePropertiesFlagsAMD")] # [derive (Default)] # [repr (transparent)] pub struct ShaderCorePropertiesFlagsAMD : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`ShaderCorePropertiesFlagsAMD`]"]
#[doc(alias = "VkShaderCorePropertiesFlagBitsAMD")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct ShaderCorePropertiesFlagBitsAMD(pub u32);
impl ShaderCorePropertiesFlagBitsAMD {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> ShaderCorePropertiesFlagsAMD {
        ShaderCorePropertiesFlagsAMD::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for ShaderCorePropertiesFlagBitsAMD {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceShaderCoreProperties2AMD> for crate::vk1_1::PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceShaderCoreProperties2AMDBuilder<'_>> for crate::vk1_1::PhysicalDeviceProperties2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderCoreProperties2AMD.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceShaderCoreProperties2AMD")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceShaderCoreProperties2AMD {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub shader_core_features: crate::extensions::amd_shader_core_properties2::ShaderCorePropertiesFlagsAMD,
    pub active_compute_unit_count: u32,
}
impl PhysicalDeviceShaderCoreProperties2AMD {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_2_AMD;
}
impl Default for PhysicalDeviceShaderCoreProperties2AMD {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), shader_core_features: Default::default(), active_compute_unit_count: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceShaderCoreProperties2AMD {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceShaderCoreProperties2AMD").field("s_type", &self.s_type).field("p_next", &self.p_next).field("shader_core_features", &self.shader_core_features).field("active_compute_unit_count", &self.active_compute_unit_count).finish()
    }
}
impl PhysicalDeviceShaderCoreProperties2AMD {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceShaderCoreProperties2AMDBuilder<'a> {
        PhysicalDeviceShaderCoreProperties2AMDBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderCoreProperties2AMD.html) · Builder of [`PhysicalDeviceShaderCoreProperties2AMD`]"]
#[repr(transparent)]
pub struct PhysicalDeviceShaderCoreProperties2AMDBuilder<'a>(PhysicalDeviceShaderCoreProperties2AMD, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceShaderCoreProperties2AMDBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceShaderCoreProperties2AMDBuilder<'a> {
        PhysicalDeviceShaderCoreProperties2AMDBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn shader_core_features(mut self, shader_core_features: crate::extensions::amd_shader_core_properties2::ShaderCorePropertiesFlagsAMD) -> Self {
        self.0.shader_core_features = shader_core_features as _;
        self
    }
    #[inline]
    pub fn active_compute_unit_count(mut self, active_compute_unit_count: u32) -> Self {
        self.0.active_compute_unit_count = active_compute_unit_count as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceShaderCoreProperties2AMD {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceShaderCoreProperties2AMDBuilder<'a> {
    fn default() -> PhysicalDeviceShaderCoreProperties2AMDBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceShaderCoreProperties2AMDBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceShaderCoreProperties2AMDBuilder<'a> {
    type Target = PhysicalDeviceShaderCoreProperties2AMD;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceShaderCoreProperties2AMDBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
