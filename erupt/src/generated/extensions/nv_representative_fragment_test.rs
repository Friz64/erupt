#[doc = "<s>Vulkan Manual Page</s> · Constant"]
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
impl Default for PhysicalDeviceRepresentativeFragmentTestFeaturesNV {
    fn default() -> Self {
        Self { s_type : crate :: vk1_0 :: StructureType :: PHYSICAL_DEVICE_REPRESENTATIVE_FRAGMENT_TEST_FEATURES_NV , p_next : std :: ptr :: null_mut ( ) , representative_fragment_test : Default :: default ( ) }
    }
}
impl std::fmt::Debug for PhysicalDeviceRepresentativeFragmentTestFeaturesNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceRepresentativeFragmentTestFeaturesNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "representative_fragment_test",
                &(self.representative_fragment_test != 0),
            )
            .finish()
    }
}
impl PhysicalDeviceRepresentativeFragmentTestFeaturesNV {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceRepresentativeFragmentTestFeaturesNVBuilder<'a> {
        PhysicalDeviceRepresentativeFragmentTestFeaturesNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV.html) · Builder of [`PhysicalDeviceRepresentativeFragmentTestFeaturesNV`](struct.PhysicalDeviceRepresentativeFragmentTestFeaturesNV.html)"]
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
    #[inline]
    pub fn representative_fragment_test(mut self, representative_fragment_test: bool) -> Self {
        self.0.representative_fragment_test = representative_fragment_test as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceRepresentativeFragmentTestFeaturesNV {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceRepresentativeFragmentTestFeaturesNVBuilder<'a> {
    fn default() -> PhysicalDeviceRepresentativeFragmentTestFeaturesNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceRepresentativeFragmentTestFeaturesNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
impl Default for PipelineRepresentativeFragmentTestStateCreateInfoNV {
    fn default() -> Self {
        Self { s_type : crate :: vk1_0 :: StructureType :: PIPELINE_REPRESENTATIVE_FRAGMENT_TEST_STATE_CREATE_INFO_NV , p_next : std :: ptr :: null ( ) , representative_fragment_test_enable : Default :: default ( ) }
    }
}
impl std::fmt::Debug for PipelineRepresentativeFragmentTestStateCreateInfoNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PipelineRepresentativeFragmentTestStateCreateInfoNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "representative_fragment_test_enable",
                &(self.representative_fragment_test_enable != 0),
            )
            .finish()
    }
}
impl PipelineRepresentativeFragmentTestStateCreateInfoNV {
    #[inline]
    pub fn into_builder<'a>(
        self,
    ) -> PipelineRepresentativeFragmentTestStateCreateInfoNVBuilder<'a> {
        PipelineRepresentativeFragmentTestStateCreateInfoNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineRepresentativeFragmentTestStateCreateInfoNV.html) · Builder of [`PipelineRepresentativeFragmentTestStateCreateInfoNV`](struct.PipelineRepresentativeFragmentTestStateCreateInfoNV.html)"]
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
    #[inline]
    pub fn representative_fragment_test_enable(
        mut self,
        representative_fragment_test_enable: bool,
    ) -> Self {
        self.0.representative_fragment_test_enable = representative_fragment_test_enable as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PipelineRepresentativeFragmentTestStateCreateInfoNV {
        self.0
    }
}
impl<'a> std::default::Default for PipelineRepresentativeFragmentTestStateCreateInfoNVBuilder<'a> {
    fn default() -> PipelineRepresentativeFragmentTestStateCreateInfoNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PipelineRepresentativeFragmentTestStateCreateInfoNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
