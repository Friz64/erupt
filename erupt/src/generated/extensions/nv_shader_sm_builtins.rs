# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_NV_shader_sm_builtins.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NV_SHADER_SM_BUILTINS_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NV_SHADER_SM_BUILTINS_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_NV_shader_sm_builtins");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderSMBuiltinsPropertiesNV.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceShaderSMBuiltinsPropertiesNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub shader_sm_count: u32,
    pub shader_warps_per_sm: u32,
}
impl PhysicalDeviceShaderSMBuiltinsPropertiesNV {
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
    pub fn builder<'a>(self) -> PhysicalDeviceShaderSMBuiltinsPropertiesNVBuilder<'a> {
        PhysicalDeviceShaderSMBuiltinsPropertiesNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceShaderSMBuiltinsPropertiesNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceShaderSMBuiltinsPropertiesNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("shader_sm_count", &self.shader_sm_count)
            .field("shader_warps_per_sm", &self.shader_warps_per_sm)
            .finish()
    }
}
impl Default for PhysicalDeviceShaderSMBuiltinsPropertiesNV {
    fn default() -> PhysicalDeviceShaderSMBuiltinsPropertiesNV {
        PhysicalDeviceShaderSMBuiltinsPropertiesNV {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_SHADER_SM_BUILTINS_PROPERTIES_NV,
            p_next: std::ptr::null_mut(),
            shader_sm_count: Default::default(),
            shader_warps_per_sm: Default::default(),
        }
    }
}
impl crate::ExtendableBy<PhysicalDeviceShaderSMBuiltinsPropertiesNV>
    for crate::vk1_1::PhysicalDeviceProperties2
{
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderSMBuiltinsPropertiesNV.html) · Builder of [`PhysicalDeviceShaderSMBuiltinsPropertiesNV`](struct.PhysicalDeviceShaderSMBuiltinsPropertiesNV.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceShaderSMBuiltinsPropertiesNVBuilder<'a>(
    PhysicalDeviceShaderSMBuiltinsPropertiesNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceShaderSMBuiltinsPropertiesNVBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceShaderSMBuiltinsPropertiesNVBuilder<'a> {
        PhysicalDeviceShaderSMBuiltinsPropertiesNVBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_sm_count(mut self, shader_sm_count: u32) -> Self {
        self.0.shader_sm_count = shader_sm_count;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_warps_per_sm(mut self, shader_warps_per_sm: u32) -> Self {
        self.0.shader_warps_per_sm = shader_warps_per_sm;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceShaderSMBuiltinsPropertiesNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceShaderSMBuiltinsPropertiesNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceShaderSMBuiltinsFeaturesNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub shader_sm_builtins: crate::vk1_0::Bool32,
}
impl PhysicalDeviceShaderSMBuiltinsFeaturesNV {
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
    pub fn builder<'a>(self) -> PhysicalDeviceShaderSMBuiltinsFeaturesNVBuilder<'a> {
        PhysicalDeviceShaderSMBuiltinsFeaturesNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceShaderSMBuiltinsFeaturesNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceShaderSMBuiltinsFeaturesNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("shader_sm_builtins", &(self.shader_sm_builtins != 0))
            .finish()
    }
}
impl Default for PhysicalDeviceShaderSMBuiltinsFeaturesNV {
    fn default() -> PhysicalDeviceShaderSMBuiltinsFeaturesNV {
        PhysicalDeviceShaderSMBuiltinsFeaturesNV {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_SHADER_SM_BUILTINS_FEATURES_NV,
            p_next: std::ptr::null_mut(),
            shader_sm_builtins: Default::default(),
        }
    }
}
impl crate::ExtendableBy<PhysicalDeviceShaderSMBuiltinsFeaturesNV>
    for crate::vk1_1::PhysicalDeviceFeatures2
{
}
impl crate::ExtendableBy<PhysicalDeviceShaderSMBuiltinsFeaturesNV>
    for crate::vk1_0::DeviceCreateInfo
{
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderSMBuiltinsFeaturesNV.html) · Builder of [`PhysicalDeviceShaderSMBuiltinsFeaturesNV`](struct.PhysicalDeviceShaderSMBuiltinsFeaturesNV.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceShaderSMBuiltinsFeaturesNVBuilder<'a>(
    PhysicalDeviceShaderSMBuiltinsFeaturesNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceShaderSMBuiltinsFeaturesNVBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceShaderSMBuiltinsFeaturesNVBuilder<'a> {
        PhysicalDeviceShaderSMBuiltinsFeaturesNVBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_sm_builtins(mut self, shader_sm_builtins: bool) -> Self {
        self.0.shader_sm_builtins = shader_sm_builtins as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceShaderSMBuiltinsFeaturesNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceShaderSMBuiltinsFeaturesNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
