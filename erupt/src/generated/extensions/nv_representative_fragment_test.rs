# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_NV_representative_fragment_test.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NV_REPRESENTATIVE_FRAGMENT_TEST_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NV_REPRESENTATIVE_FRAGMENT_TEST_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_NV_representative_fragment_test");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceRepresentativeFragmentTestFeaturesNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub representative_fragment_test: crate::vk1_0::Bool32,
}
impl PhysicalDeviceRepresentativeFragmentTestFeaturesNV {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceRepresentativeFragmentTestFeaturesNV,
    {
        unsafe {
            crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
        }
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceRepresentativeFragmentTestFeaturesNVBuilder<'a> {
        PhysicalDeviceRepresentativeFragmentTestFeaturesNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceRepresentativeFragmentTestFeaturesNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceRepresentativeFragmentTestFeaturesNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "representative_fragment_test",
                &(self.representative_fragment_test != 0),
            )
            .finish()
    }
}
impl Default for PhysicalDeviceRepresentativeFragmentTestFeaturesNV {
    fn default() -> PhysicalDeviceRepresentativeFragmentTestFeaturesNV {
        PhysicalDeviceRepresentativeFragmentTestFeaturesNV { s_type : crate :: vk1_0 :: StructureType :: PHYSICAL_DEVICE_REPRESENTATIVE_FRAGMENT_TEST_FEATURES_NV , p_next : std :: ptr :: null_mut ( ) , representative_fragment_test : Default :: default ( ) , }
    }
}
#[doc = "Used by [`PhysicalDeviceRepresentativeFragmentTestFeaturesNV::extend`](struct.PhysicalDeviceRepresentativeFragmentTestFeaturesNV.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceRepresentativeFragmentTestFeaturesNV {}
impl ExtendableByPhysicalDeviceRepresentativeFragmentTestFeaturesNV
    for crate::vk1_1::PhysicalDeviceFeatures2
{
}
impl ExtendableByPhysicalDeviceRepresentativeFragmentTestFeaturesNV
    for crate::vk1_0::DeviceCreateInfo
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceRepresentativeFragmentTestFeaturesNV`](struct.PhysicalDeviceRepresentativeFragmentTestFeaturesNV.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceRepresentativeFragmentTestFeaturesNVBuilder<'a>(
    PhysicalDeviceRepresentativeFragmentTestFeaturesNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceRepresentativeFragmentTestFeaturesNVBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceRepresentativeFragmentTestFeaturesNVBuilder<'a> {
        PhysicalDeviceRepresentativeFragmentTestFeaturesNVBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn representative_fragment_test(mut self, representative_fragment_test: bool) -> Self {
        self.0.representative_fragment_test = representative_fragment_test as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceRepresentativeFragmentTestFeaturesNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceRepresentativeFragmentTestFeaturesNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceRepresentativeFragmentTestFeaturesNVBuilder<'a> {
    type Target = PhysicalDeviceRepresentativeFragmentTestFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceRepresentativeFragmentTestFeaturesNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineRepresentativeFragmentTestStateCreateInfoNV.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineRepresentativeFragmentTestStateCreateInfoNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub representative_fragment_test_enable: crate::vk1_0::Bool32,
}
impl PipelineRepresentativeFragmentTestStateCreateInfoNV {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPipelineRepresentativeFragmentTestStateCreateInfoNV,
    {
        unsafe {
            crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
        }
    }
    #[inline]
    pub fn builder<'a>(self) -> PipelineRepresentativeFragmentTestStateCreateInfoNVBuilder<'a> {
        PipelineRepresentativeFragmentTestStateCreateInfoNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PipelineRepresentativeFragmentTestStateCreateInfoNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PipelineRepresentativeFragmentTestStateCreateInfoNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "representative_fragment_test_enable",
                &(self.representative_fragment_test_enable != 0),
            )
            .finish()
    }
}
impl Default for PipelineRepresentativeFragmentTestStateCreateInfoNV {
    fn default() -> PipelineRepresentativeFragmentTestStateCreateInfoNV {
        PipelineRepresentativeFragmentTestStateCreateInfoNV { s_type : crate :: vk1_0 :: StructureType :: PIPELINE_REPRESENTATIVE_FRAGMENT_TEST_STATE_CREATE_INFO_NV , p_next : std :: ptr :: null ( ) , representative_fragment_test_enable : Default :: default ( ) , }
    }
}
#[doc = "Used by [`PipelineRepresentativeFragmentTestStateCreateInfoNV::extend`](struct.PipelineRepresentativeFragmentTestStateCreateInfoNV.html#method.extend)"]
pub trait ExtendableByPipelineRepresentativeFragmentTestStateCreateInfoNV {}
impl ExtendableByPipelineRepresentativeFragmentTestStateCreateInfoNV
    for crate::vk1_0::GraphicsPipelineCreateInfo
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PipelineRepresentativeFragmentTestStateCreateInfoNV`](struct.PipelineRepresentativeFragmentTestStateCreateInfoNV.html)"]
#[repr(transparent)]
pub struct PipelineRepresentativeFragmentTestStateCreateInfoNVBuilder<'a>(
    PipelineRepresentativeFragmentTestStateCreateInfoNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PipelineRepresentativeFragmentTestStateCreateInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineRepresentativeFragmentTestStateCreateInfoNVBuilder<'a> {
        PipelineRepresentativeFragmentTestStateCreateInfoNVBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn representative_fragment_test_enable(
        mut self,
        representative_fragment_test_enable: bool,
    ) -> Self {
        self.0.representative_fragment_test_enable = representative_fragment_test_enable as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PipelineRepresentativeFragmentTestStateCreateInfoNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for PipelineRepresentativeFragmentTestStateCreateInfoNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PipelineRepresentativeFragmentTestStateCreateInfoNVBuilder<'a> {
    type Target = PipelineRepresentativeFragmentTestStateCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineRepresentativeFragmentTestStateCreateInfoNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
