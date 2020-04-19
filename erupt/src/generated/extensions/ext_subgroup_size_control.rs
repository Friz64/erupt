# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_EXT_subgroup_size_control.html)\n\n## Extends\n- [`PipelineShaderStageCreateFlagBits`](../../vk1_0/struct.PipelineShaderStageCreateFlagBits.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_SUBGROUP_SIZE_CONTROL_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_SUBGROUP_SIZE_CONTROL_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_subgroup_size_control");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSubgroupSizeControlFeaturesEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceSubgroupSizeControlFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub subgroup_size_control: crate::vk1_0::Bool32,
    pub compute_full_subgroups: crate::vk1_0::Bool32,
}
impl PhysicalDeviceSubgroupSizeControlFeaturesEXT {
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
    pub fn builder<'a>(self) -> PhysicalDeviceSubgroupSizeControlFeaturesEXTBuilder<'a> {
        PhysicalDeviceSubgroupSizeControlFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceSubgroupSizeControlFeaturesEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceSubgroupSizeControlFeaturesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("subgroup_size_control", &(self.subgroup_size_control != 0))
            .field(
                "compute_full_subgroups",
                &(self.compute_full_subgroups != 0),
            )
            .finish()
    }
}
impl Default for PhysicalDeviceSubgroupSizeControlFeaturesEXT {
    fn default() -> PhysicalDeviceSubgroupSizeControlFeaturesEXT {
        PhysicalDeviceSubgroupSizeControlFeaturesEXT {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            subgroup_size_control: Default::default(),
            compute_full_subgroups: Default::default(),
        }
    }
}
impl crate::ExtendableBy<PhysicalDeviceSubgroupSizeControlFeaturesEXT>
    for crate::vk1_1::PhysicalDeviceFeatures2
{
}
impl crate::ExtendableBy<PhysicalDeviceSubgroupSizeControlFeaturesEXT>
    for crate::vk1_0::DeviceCreateInfo
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceSubgroupSizeControlFeaturesEXT`](struct.PhysicalDeviceSubgroupSizeControlFeaturesEXT.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceSubgroupSizeControlFeaturesEXTBuilder<'a>(
    PhysicalDeviceSubgroupSizeControlFeaturesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceSubgroupSizeControlFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceSubgroupSizeControlFeaturesEXTBuilder<'a> {
        PhysicalDeviceSubgroupSizeControlFeaturesEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn subgroup_size_control(mut self, subgroup_size_control: bool) -> Self {
        self.0.subgroup_size_control = subgroup_size_control as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn compute_full_subgroups(mut self, compute_full_subgroups: bool) -> Self {
        self.0.compute_full_subgroups = compute_full_subgroups as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceSubgroupSizeControlFeaturesEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceSubgroupSizeControlFeaturesEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceSubgroupSizeControlFeaturesEXTBuilder<'a> {
    type Target = PhysicalDeviceSubgroupSizeControlFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceSubgroupSizeControlFeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSubgroupSizeControlPropertiesEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceSubgroupSizeControlPropertiesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub min_subgroup_size: u32,
    pub max_subgroup_size: u32,
    pub max_compute_workgroup_subgroups: u32,
    pub required_subgroup_size_stages: crate::vk1_0::ShaderStageFlags,
}
impl PhysicalDeviceSubgroupSizeControlPropertiesEXT {
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
    pub fn builder<'a>(self) -> PhysicalDeviceSubgroupSizeControlPropertiesEXTBuilder<'a> {
        PhysicalDeviceSubgroupSizeControlPropertiesEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceSubgroupSizeControlPropertiesEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceSubgroupSizeControlPropertiesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("min_subgroup_size", &self.min_subgroup_size)
            .field("max_subgroup_size", &self.max_subgroup_size)
            .field(
                "max_compute_workgroup_subgroups",
                &self.max_compute_workgroup_subgroups,
            )
            .field(
                "required_subgroup_size_stages",
                &self.required_subgroup_size_stages,
            )
            .finish()
    }
}
impl Default for PhysicalDeviceSubgroupSizeControlPropertiesEXT {
    fn default() -> PhysicalDeviceSubgroupSizeControlPropertiesEXT {
        PhysicalDeviceSubgroupSizeControlPropertiesEXT {
            s_type:
                crate::vk1_0::StructureType::PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            min_subgroup_size: Default::default(),
            max_subgroup_size: Default::default(),
            max_compute_workgroup_subgroups: Default::default(),
            required_subgroup_size_stages: Default::default(),
        }
    }
}
impl crate::ExtendableBy<PhysicalDeviceSubgroupSizeControlPropertiesEXT>
    for crate::vk1_1::PhysicalDeviceProperties2
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceSubgroupSizeControlPropertiesEXT`](struct.PhysicalDeviceSubgroupSizeControlPropertiesEXT.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceSubgroupSizeControlPropertiesEXTBuilder<'a>(
    PhysicalDeviceSubgroupSizeControlPropertiesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceSubgroupSizeControlPropertiesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceSubgroupSizeControlPropertiesEXTBuilder<'a> {
        PhysicalDeviceSubgroupSizeControlPropertiesEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn min_subgroup_size(mut self, min_subgroup_size: u32) -> Self {
        self.0.min_subgroup_size = min_subgroup_size;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_subgroup_size(mut self, max_subgroup_size: u32) -> Self {
        self.0.max_subgroup_size = max_subgroup_size;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_compute_workgroup_subgroups(mut self, max_compute_workgroup_subgroups: u32) -> Self {
        self.0.max_compute_workgroup_subgroups = max_compute_workgroup_subgroups;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn required_subgroup_size_stages(
        mut self,
        required_subgroup_size_stages: crate::vk1_0::ShaderStageFlags,
    ) -> Self {
        self.0.required_subgroup_size_stages = required_subgroup_size_stages;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceSubgroupSizeControlPropertiesEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceSubgroupSizeControlPropertiesEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceSubgroupSizeControlPropertiesEXTBuilder<'a> {
    type Target = PhysicalDeviceSubgroupSizeControlPropertiesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceSubgroupSizeControlPropertiesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineShaderStageRequiredSubgroupSizeCreateInfoEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineShaderStageRequiredSubgroupSizeCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub required_subgroup_size: u32,
}
impl PipelineShaderStageRequiredSubgroupSizeCreateInfoEXT {
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
    pub fn builder<'a>(self) -> PipelineShaderStageRequiredSubgroupSizeCreateInfoEXTBuilder<'a> {
        PipelineShaderStageRequiredSubgroupSizeCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PipelineShaderStageRequiredSubgroupSizeCreateInfoEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PipelineShaderStageRequiredSubgroupSizeCreateInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("required_subgroup_size", &self.required_subgroup_size)
            .finish()
    }
}
impl Default for PipelineShaderStageRequiredSubgroupSizeCreateInfoEXT {
    fn default() -> PipelineShaderStageRequiredSubgroupSizeCreateInfoEXT {
        PipelineShaderStageRequiredSubgroupSizeCreateInfoEXT { s_type : crate :: vk1_0 :: StructureType :: PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO_EXT , p_next : std :: ptr :: null_mut ( ) , required_subgroup_size : Default :: default ( ) , }
    }
}
impl crate::ExtendableBy<PipelineShaderStageRequiredSubgroupSizeCreateInfoEXT>
    for crate::vk1_0::PipelineShaderStageCreateInfo
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PipelineShaderStageRequiredSubgroupSizeCreateInfoEXT`](struct.PipelineShaderStageRequiredSubgroupSizeCreateInfoEXT.html)"]
#[repr(transparent)]
pub struct PipelineShaderStageRequiredSubgroupSizeCreateInfoEXTBuilder<'a>(
    PipelineShaderStageRequiredSubgroupSizeCreateInfoEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PipelineShaderStageRequiredSubgroupSizeCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineShaderStageRequiredSubgroupSizeCreateInfoEXTBuilder<'a> {
        PipelineShaderStageRequiredSubgroupSizeCreateInfoEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn required_subgroup_size(mut self, required_subgroup_size: u32) -> Self {
        self.0.required_subgroup_size = required_subgroup_size;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PipelineShaderStageRequiredSubgroupSizeCreateInfoEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for PipelineShaderStageRequiredSubgroupSizeCreateInfoEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PipelineShaderStageRequiredSubgroupSizeCreateInfoEXTBuilder<'a> {
    type Target = PipelineShaderStageRequiredSubgroupSizeCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineShaderStageRequiredSubgroupSizeCreateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
