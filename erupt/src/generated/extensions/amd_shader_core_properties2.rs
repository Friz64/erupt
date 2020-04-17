# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_AMD_shader_core_properties2.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const AMD_SHADER_CORE_PROPERTIES_2_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const AMD_SHADER_CORE_PROPERTIES_2_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_AMD_shader_core_properties2");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderCoreProperties2AMD.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceShaderCoreProperties2AMD {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub shader_core_features:
        crate::extensions::amd_shader_core_properties2::ShaderCorePropertiesFlagsAMD,
    pub active_compute_unit_count: u32,
}
impl PhysicalDeviceShaderCoreProperties2AMD {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceShaderCoreProperties2AMD,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceShaderCoreProperties2AMDBuilder<'a> {
        PhysicalDeviceShaderCoreProperties2AMDBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceShaderCoreProperties2AMD {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceShaderCoreProperties2AMD")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("shader_core_features", &self.shader_core_features)
            .field("active_compute_unit_count", &self.active_compute_unit_count)
            .finish()
    }
}
impl Default for PhysicalDeviceShaderCoreProperties2AMD {
    fn default() -> PhysicalDeviceShaderCoreProperties2AMD {
        PhysicalDeviceShaderCoreProperties2AMD {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_2_AMD,
            p_next: std::ptr::null_mut(),
            shader_core_features: Default::default(),
            active_compute_unit_count: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceShaderCoreProperties2AMD::extend`](struct.PhysicalDeviceShaderCoreProperties2AMD.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceShaderCoreProperties2AMD {}
impl ExtendableByPhysicalDeviceShaderCoreProperties2AMD
    for crate::vk1_1::PhysicalDeviceProperties2
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceShaderCoreProperties2AMD`](struct.PhysicalDeviceShaderCoreProperties2AMD.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceShaderCoreProperties2AMDBuilder<'a>(
    PhysicalDeviceShaderCoreProperties2AMD,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceShaderCoreProperties2AMDBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceShaderCoreProperties2AMDBuilder<'a> {
        PhysicalDeviceShaderCoreProperties2AMDBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_core_features(
        mut self,
        shader_core_features : crate :: extensions :: amd_shader_core_properties2 :: ShaderCorePropertiesFlagsAMD,
    ) -> Self {
        self.0.shader_core_features = shader_core_features;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn active_compute_unit_count(mut self, active_compute_unit_count: u32) -> Self {
        self.0.active_compute_unit_count = active_compute_unit_count;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceShaderCoreProperties2AMD {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceShaderCoreProperties2AMDBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "<s>Vulkan Manual Page</s> · Flag Bits of [`ShaderCorePropertiesFlagsAMD`](struct.ShaderCorePropertiesFlagsAMD.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ShaderCorePropertiesFlagBitsAMD(pub u32);
impl ShaderCorePropertiesFlagBitsAMD {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> ShaderCorePropertiesFlagsAMD {
        ShaderCorePropertiesFlagsAMD::from_bits_truncate(self.0)
    }
}
#[doc = "[Part of `extensions::amd_shader_core_properties2`](../../extensions/amd_shader_core_properties2/index.html)"]
impl ShaderCorePropertiesFlagBitsAMD {}
impl std::fmt::Debug for ShaderCorePropertiesFlagBitsAMD {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            _ => "(Unknown)",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkShaderCorePropertiesFlagsAMD.html) · Flags of [`ShaderCorePropertiesFlagBitsAMD`](struct.ShaderCorePropertiesFlagBitsAMD.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct ShaderCorePropertiesFlagsAMD : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
