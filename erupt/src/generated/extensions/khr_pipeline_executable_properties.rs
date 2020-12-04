#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_PIPELINE_EXECUTABLE_PROPERTIES_SPEC_VERSION")]
pub const KHR_PIPELINE_EXECUTABLE_PROPERTIES_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_PIPELINE_EXECUTABLE_PROPERTIES_EXTENSION_NAME")]
pub const KHR_PIPELINE_EXECUTABLE_PROPERTIES_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_pipeline_executable_properties");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "FN_GET_PIPELINE_EXECUTABLE_PROPERTIES_KHR")]
pub const FN_GET_PIPELINE_EXECUTABLE_PROPERTIES_KHR: *const std::os::raw::c_char =
    crate::cstr!("vkGetPipelineExecutablePropertiesKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "FN_GET_PIPELINE_EXECUTABLE_STATISTICS_KHR")]
pub const FN_GET_PIPELINE_EXECUTABLE_STATISTICS_KHR: *const std::os::raw::c_char =
    crate::cstr!("vkGetPipelineExecutableStatisticsKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "FN_GET_PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATIONS_KHR")]
pub const FN_GET_PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATIONS_KHR: *const std::os::raw::c_char =
    crate::cstr!("vkGetPipelineExecutableInternalRepresentationsKHR");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineExecutableStatisticFormatKHR.html) · Enum"]
#[doc(alias = "VkPipelineExecutableStatisticFormatKHR")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct PipelineExecutableStatisticFormatKHR(pub i32);
impl std::fmt::Debug for PipelineExecutableStatisticFormatKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::BOOL32_KHR => "BOOL32_KHR",
            &Self::INT64_KHR => "INT64_KHR",
            &Self::UINT64_KHR => "UINT64_KHR",
            &Self::FLOAT64_KHR => "FLOAT64_KHR",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::khr_pipeline_executable_properties`]"]
impl PipelineExecutableStatisticFormatKHR {
    pub const BOOL32_KHR: Self = Self(0);
    pub const INT64_KHR: Self = Self(1);
    pub const UINT64_KHR: Self = Self(2);
    pub const FLOAT64_KHR: Self = Self(3);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPipelineExecutablePropertiesKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPipelineExecutablePropertiesKHR = unsafe extern "system" fn (device : crate :: vk1_0 :: Device , p_pipeline_info : * const crate :: extensions :: khr_pipeline_executable_properties :: PipelineInfoKHR , p_executable_count : * mut u32 , p_properties : * mut crate :: extensions :: khr_pipeline_executable_properties :: PipelineExecutablePropertiesKHR) -> crate :: vk1_0 :: Result ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPipelineExecutableStatisticsKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPipelineExecutableStatisticsKHR = unsafe extern "system" fn (device : crate :: vk1_0 :: Device , p_executable_info : * const crate :: extensions :: khr_pipeline_executable_properties :: PipelineExecutableInfoKHR , p_statistic_count : * mut u32 , p_statistics : * mut crate :: extensions :: khr_pipeline_executable_properties :: PipelineExecutableStatisticKHR) -> crate :: vk1_0 :: Result ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPipelineExecutableInternalRepresentationsKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPipelineExecutableInternalRepresentationsKHR = unsafe extern "system" fn (device : crate :: vk1_0 :: Device , p_executable_info : * const crate :: extensions :: khr_pipeline_executable_properties :: PipelineExecutableInfoKHR , p_internal_representation_count : * mut u32 , p_internal_representations : * mut crate :: extensions :: khr_pipeline_executable_properties :: PipelineExecutableInternalRepresentationKHR) -> crate :: vk1_0 :: Result ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR.html) · Structure"]
#[doc(alias = "VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDevicePipelineExecutablePropertiesFeaturesKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub pipeline_executable_info: crate::vk1_0::Bool32,
}
impl Default for PhysicalDevicePipelineExecutablePropertiesFeaturesKHR {
    fn default() -> Self {
        Self { s_type : crate :: vk1_0 :: StructureType :: PHYSICAL_DEVICE_PIPELINE_EXECUTABLE_PROPERTIES_FEATURES_KHR , p_next : std :: ptr :: null_mut () , pipeline_executable_info : Default :: default () }
    }
}
impl std::fmt::Debug for PhysicalDevicePipelineExecutablePropertiesFeaturesKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDevicePipelineExecutablePropertiesFeaturesKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "pipeline_executable_info",
                &(self.pipeline_executable_info != 0),
            )
            .finish()
    }
}
impl PhysicalDevicePipelineExecutablePropertiesFeaturesKHR {
    #[inline]
    pub fn into_builder<'a>(
        self,
    ) -> PhysicalDevicePipelineExecutablePropertiesFeaturesKHRBuilder<'a> {
        PhysicalDevicePipelineExecutablePropertiesFeaturesKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR.html) · Builder of [`PhysicalDevicePipelineExecutablePropertiesFeaturesKHR`]"]
#[repr(transparent)]
pub struct PhysicalDevicePipelineExecutablePropertiesFeaturesKHRBuilder<'a>(
    PhysicalDevicePipelineExecutablePropertiesFeaturesKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDevicePipelineExecutablePropertiesFeaturesKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDevicePipelineExecutablePropertiesFeaturesKHRBuilder<'a> {
        PhysicalDevicePipelineExecutablePropertiesFeaturesKHRBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[inline]
    pub fn pipeline_executable_info(mut self, pipeline_executable_info: bool) -> Self {
        self.0.pipeline_executable_info = pipeline_executable_info as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDevicePipelineExecutablePropertiesFeaturesKHR {
        self.0
    }
}
impl<'a> std::default::Default
    for PhysicalDevicePipelineExecutablePropertiesFeaturesKHRBuilder<'a>
{
    fn default() -> PhysicalDevicePipelineExecutablePropertiesFeaturesKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDevicePipelineExecutablePropertiesFeaturesKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDevicePipelineExecutablePropertiesFeaturesKHRBuilder<'a> {
    type Target = PhysicalDevicePipelineExecutablePropertiesFeaturesKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDevicePipelineExecutablePropertiesFeaturesKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineInfoKHR.html) · Structure"]
#[doc(alias = "VkPipelineInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub pipeline: crate::vk1_0::Pipeline,
}
impl Default for PipelineInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PIPELINE_INFO_KHR,
            p_next: std::ptr::null(),
            pipeline: Default::default(),
        }
    }
}
impl std::fmt::Debug for PipelineInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PipelineInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("pipeline", &self.pipeline)
            .finish()
    }
}
impl PipelineInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> PipelineInfoKHRBuilder<'a> {
        PipelineInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineInfoKHR.html) · Builder of [`PipelineInfoKHR`]"]
#[repr(transparent)]
pub struct PipelineInfoKHRBuilder<'a>(PipelineInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> PipelineInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineInfoKHRBuilder<'a> {
        PipelineInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn pipeline(mut self, pipeline: crate::vk1_0::Pipeline) -> Self {
        self.0.pipeline = pipeline as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PipelineInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for PipelineInfoKHRBuilder<'a> {
    fn default() -> PipelineInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PipelineInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PipelineInfoKHRBuilder<'a> {
    type Target = PipelineInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineExecutablePropertiesKHR.html) · Structure"]
#[doc(alias = "VkPipelineExecutablePropertiesKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineExecutablePropertiesKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub stages: crate::vk1_0::ShaderStageFlags,
    pub name: [std::os::raw::c_char; 256],
    pub description: [std::os::raw::c_char; 256],
    pub subgroup_size: u32,
}
impl Default for PipelineExecutablePropertiesKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PIPELINE_EXECUTABLE_PROPERTIES_KHR,
            p_next: std::ptr::null_mut(),
            stages: Default::default(),
            name: unsafe { std::mem::zeroed() },
            description: unsafe { std::mem::zeroed() },
            subgroup_size: Default::default(),
        }
    }
}
impl std::fmt::Debug for PipelineExecutablePropertiesKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PipelineExecutablePropertiesKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("stages", &self.stages)
            .field("name", unsafe {
                &std::ffi::CStr::from_ptr(self.name.as_ptr())
            })
            .field("description", unsafe {
                &std::ffi::CStr::from_ptr(self.description.as_ptr())
            })
            .field("subgroup_size", &self.subgroup_size)
            .finish()
    }
}
impl PipelineExecutablePropertiesKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> PipelineExecutablePropertiesKHRBuilder<'a> {
        PipelineExecutablePropertiesKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineExecutablePropertiesKHR.html) · Builder of [`PipelineExecutablePropertiesKHR`]"]
#[repr(transparent)]
pub struct PipelineExecutablePropertiesKHRBuilder<'a>(
    PipelineExecutablePropertiesKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PipelineExecutablePropertiesKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineExecutablePropertiesKHRBuilder<'a> {
        PipelineExecutablePropertiesKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn stages(mut self, stages: crate::vk1_0::ShaderStageFlags) -> Self {
        self.0.stages = stages as _;
        self
    }
    #[inline]
    pub fn name(mut self, name: [std::os::raw::c_char; 256]) -> Self {
        self.0.name = name as _;
        self
    }
    #[inline]
    pub fn description(mut self, description: [std::os::raw::c_char; 256]) -> Self {
        self.0.description = description as _;
        self
    }
    #[inline]
    pub fn subgroup_size(mut self, subgroup_size: u32) -> Self {
        self.0.subgroup_size = subgroup_size as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PipelineExecutablePropertiesKHR {
        self.0
    }
}
impl<'a> std::default::Default for PipelineExecutablePropertiesKHRBuilder<'a> {
    fn default() -> PipelineExecutablePropertiesKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PipelineExecutablePropertiesKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PipelineExecutablePropertiesKHRBuilder<'a> {
    type Target = PipelineExecutablePropertiesKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineExecutablePropertiesKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineExecutableInfoKHR.html) · Structure"]
#[doc(alias = "VkPipelineExecutableInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineExecutableInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub pipeline: crate::vk1_0::Pipeline,
    pub executable_index: u32,
}
impl Default for PipelineExecutableInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PIPELINE_EXECUTABLE_INFO_KHR,
            p_next: std::ptr::null(),
            pipeline: Default::default(),
            executable_index: Default::default(),
        }
    }
}
impl std::fmt::Debug for PipelineExecutableInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PipelineExecutableInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("pipeline", &self.pipeline)
            .field("executable_index", &self.executable_index)
            .finish()
    }
}
impl PipelineExecutableInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> PipelineExecutableInfoKHRBuilder<'a> {
        PipelineExecutableInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineExecutableInfoKHR.html) · Builder of [`PipelineExecutableInfoKHR`]"]
#[repr(transparent)]
pub struct PipelineExecutableInfoKHRBuilder<'a>(
    PipelineExecutableInfoKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PipelineExecutableInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineExecutableInfoKHRBuilder<'a> {
        PipelineExecutableInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn pipeline(mut self, pipeline: crate::vk1_0::Pipeline) -> Self {
        self.0.pipeline = pipeline as _;
        self
    }
    #[inline]
    pub fn executable_index(mut self, executable_index: u32) -> Self {
        self.0.executable_index = executable_index as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PipelineExecutableInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for PipelineExecutableInfoKHRBuilder<'a> {
    fn default() -> PipelineExecutableInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PipelineExecutableInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PipelineExecutableInfoKHRBuilder<'a> {
    type Target = PipelineExecutableInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineExecutableInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineExecutableStatisticValueKHR.html) · Structure"]
#[doc(alias = "VkPipelineExecutableStatisticValueKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub union PipelineExecutableStatisticValueKHR {
    pub b32: crate::vk1_0::Bool32,
    pub i64: i64,
    pub u64: u64,
    pub f64: std::os::raw::c_double,
}
impl Default for PipelineExecutableStatisticValueKHR {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for PipelineExecutableStatisticValueKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PipelineExecutableStatisticValueKHR")
            .finish()
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineExecutableStatisticKHR.html) · Structure"]
#[doc(alias = "VkPipelineExecutableStatisticKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineExecutableStatisticKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub name: [std::os::raw::c_char; 256],
    pub description: [std::os::raw::c_char; 256],
    pub format:
        crate::extensions::khr_pipeline_executable_properties::PipelineExecutableStatisticFormatKHR,
    pub value:
        crate::extensions::khr_pipeline_executable_properties::PipelineExecutableStatisticValueKHR,
}
impl Default for PipelineExecutableStatisticKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PIPELINE_EXECUTABLE_STATISTIC_KHR,
            p_next: std::ptr::null_mut(),
            name: unsafe { std::mem::zeroed() },
            description: unsafe { std::mem::zeroed() },
            format: Default::default(),
            value: Default::default(),
        }
    }
}
impl std::fmt::Debug for PipelineExecutableStatisticKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PipelineExecutableStatisticKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("name", unsafe {
                &std::ffi::CStr::from_ptr(self.name.as_ptr())
            })
            .field("description", unsafe {
                &std::ffi::CStr::from_ptr(self.description.as_ptr())
            })
            .field("format", &self.format)
            .field("value", &self.value)
            .finish()
    }
}
impl PipelineExecutableStatisticKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> PipelineExecutableStatisticKHRBuilder<'a> {
        PipelineExecutableStatisticKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineExecutableStatisticKHR.html) · Builder of [`PipelineExecutableStatisticKHR`]"]
#[repr(transparent)]
pub struct PipelineExecutableStatisticKHRBuilder<'a>(
    PipelineExecutableStatisticKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PipelineExecutableStatisticKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineExecutableStatisticKHRBuilder<'a> {
        PipelineExecutableStatisticKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn name(mut self, name: [std::os::raw::c_char; 256]) -> Self {
        self.0.name = name as _;
        self
    }
    #[inline]
    pub fn description(mut self, description: [std::os::raw::c_char; 256]) -> Self {
        self.0.description = description as _;
        self
    }
    #[inline]
    pub fn format(
        mut self,
        format : crate :: extensions :: khr_pipeline_executable_properties :: PipelineExecutableStatisticFormatKHR,
    ) -> Self {
        self.0.format = format as _;
        self
    }
    #[inline]
    pub fn value(
        mut self,
        value : crate :: extensions :: khr_pipeline_executable_properties :: PipelineExecutableStatisticValueKHR,
    ) -> Self {
        self.0.value = value as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PipelineExecutableStatisticKHR {
        self.0
    }
}
impl<'a> std::default::Default for PipelineExecutableStatisticKHRBuilder<'a> {
    fn default() -> PipelineExecutableStatisticKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PipelineExecutableStatisticKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PipelineExecutableStatisticKHRBuilder<'a> {
    type Target = PipelineExecutableStatisticKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineExecutableStatisticKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineExecutableInternalRepresentationKHR.html) · Structure"]
#[doc(alias = "VkPipelineExecutableInternalRepresentationKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineExecutableInternalRepresentationKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub name: [std::os::raw::c_char; 256],
    pub description: [std::os::raw::c_char; 256],
    pub is_text: crate::vk1_0::Bool32,
    pub data_size: usize,
    pub p_data: *mut std::ffi::c_void,
}
impl Default for PipelineExecutableInternalRepresentationKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATION_KHR,
            p_next: std::ptr::null_mut(),
            name: unsafe { std::mem::zeroed() },
            description: unsafe { std::mem::zeroed() },
            is_text: Default::default(),
            data_size: Default::default(),
            p_data: std::ptr::null_mut(),
        }
    }
}
impl std::fmt::Debug for PipelineExecutableInternalRepresentationKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PipelineExecutableInternalRepresentationKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("name", unsafe {
                &std::ffi::CStr::from_ptr(self.name.as_ptr())
            })
            .field("description", unsafe {
                &std::ffi::CStr::from_ptr(self.description.as_ptr())
            })
            .field("is_text", &(self.is_text != 0))
            .field("data_size", &self.data_size)
            .field("p_data", &self.p_data)
            .finish()
    }
}
impl PipelineExecutableInternalRepresentationKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> PipelineExecutableInternalRepresentationKHRBuilder<'a> {
        PipelineExecutableInternalRepresentationKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineExecutableInternalRepresentationKHR.html) · Builder of [`PipelineExecutableInternalRepresentationKHR`]"]
#[repr(transparent)]
pub struct PipelineExecutableInternalRepresentationKHRBuilder<'a>(
    PipelineExecutableInternalRepresentationKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PipelineExecutableInternalRepresentationKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineExecutableInternalRepresentationKHRBuilder<'a> {
        PipelineExecutableInternalRepresentationKHRBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[inline]
    pub fn name(mut self, name: [std::os::raw::c_char; 256]) -> Self {
        self.0.name = name as _;
        self
    }
    #[inline]
    pub fn description(mut self, description: [std::os::raw::c_char; 256]) -> Self {
        self.0.description = description as _;
        self
    }
    #[inline]
    pub fn is_text(mut self, is_text: bool) -> Self {
        self.0.is_text = is_text as _;
        self
    }
    #[inline]
    pub fn data_size(mut self, data_size: usize) -> Self {
        self.0.data_size = data_size;
        self
    }
    #[inline]
    pub fn data(mut self, data: *mut std::ffi::c_void) -> Self {
        self.0.p_data = data;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PipelineExecutableInternalRepresentationKHR {
        self.0
    }
}
impl<'a> std::default::Default for PipelineExecutableInternalRepresentationKHRBuilder<'a> {
    fn default() -> PipelineExecutableInternalRepresentationKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PipelineExecutableInternalRepresentationKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PipelineExecutableInternalRepresentationKHRBuilder<'a> {
    type Target = PipelineExecutableInternalRepresentationKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineExecutableInternalRepresentationKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::khr_pipeline_executable_properties`]"]
impl crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPipelineExecutablePropertiesKHR.html) · Function"]
    #[doc(alias = "vkGetPipelineExecutablePropertiesKHR")]
    pub unsafe fn get_pipeline_executable_properties_khr(
        &self,
        pipeline_info: &crate::extensions::khr_pipeline_executable_properties::PipelineInfoKHR,
        executable_count: Option<u32>,
    ) -> crate::utils::VulkanResult<
        Vec<crate::extensions::khr_pipeline_executable_properties::PipelineExecutablePropertiesKHR>,
    > {
        let _function = self
            .get_pipeline_executable_properties_khr
            .expect("`get_pipeline_executable_properties_khr` is not loaded");
        let mut executable_count = match executable_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                _function(
                    self.handle,
                    pipeline_info as _,
                    &mut v,
                    std::ptr::null_mut(),
                );
                v
            }
        };
        let mut properties = vec![Default::default(); executable_count as _];
        let _return = _function(
            self.handle,
            pipeline_info as _,
            &mut executable_count,
            properties.as_mut_ptr(),
        );
        crate::utils::VulkanResult::new(_return, properties)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPipelineExecutableStatisticsKHR.html) · Function"]
    #[doc(alias = "vkGetPipelineExecutableStatisticsKHR")]
    pub unsafe fn get_pipeline_executable_statistics_khr(
        &self,
        executable_info : & crate :: extensions :: khr_pipeline_executable_properties :: PipelineExecutableInfoKHR,
        statistic_count: Option<u32>,
    ) -> crate::utils::VulkanResult<
        Vec<crate::extensions::khr_pipeline_executable_properties::PipelineExecutableStatisticKHR>,
    > {
        let _function = self
            .get_pipeline_executable_statistics_khr
            .expect("`get_pipeline_executable_statistics_khr` is not loaded");
        let mut statistic_count = match statistic_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                _function(
                    self.handle,
                    executable_info as _,
                    &mut v,
                    std::ptr::null_mut(),
                );
                v
            }
        };
        let mut statistics = vec![Default::default(); statistic_count as _];
        let _return = _function(
            self.handle,
            executable_info as _,
            &mut statistic_count,
            statistics.as_mut_ptr(),
        );
        crate::utils::VulkanResult::new(_return, statistics)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPipelineExecutableInternalRepresentationsKHR.html) · Function"]
    #[doc(alias = "vkGetPipelineExecutableInternalRepresentationsKHR")]    pub unsafe fn get_pipeline_executable_internal_representations_khr (& self , executable_info : & crate :: extensions :: khr_pipeline_executable_properties :: PipelineExecutableInfoKHR , internal_representation_count : Option < u32 >) -> crate :: utils :: VulkanResult < Vec < crate :: extensions :: khr_pipeline_executable_properties :: PipelineExecutableInternalRepresentationKHR > >{
        let _function = self
            .get_pipeline_executable_internal_representations_khr
            .expect("`get_pipeline_executable_internal_representations_khr` is not loaded");
        let mut internal_representation_count = match internal_representation_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                _function(
                    self.handle,
                    executable_info as _,
                    &mut v,
                    std::ptr::null_mut(),
                );
                v
            }
        };
        let mut internal_representations =
            vec![Default::default(); internal_representation_count as _];
        let _return = _function(
            self.handle,
            executable_info as _,
            &mut internal_representation_count,
            internal_representations.as_mut_ptr(),
        );
        crate::utils::VulkanResult::new(_return, internal_representations)
    }
}
