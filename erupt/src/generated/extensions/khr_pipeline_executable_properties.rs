# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_KHR_pipeline_executable_properties.html)\n\n## Extends\n- [`PipelineCreateFlagBits`](../../vk1_0/struct.PipelineCreateFlagBits.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_PIPELINE_EXECUTABLE_PROPERTIES_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_PIPELINE_EXECUTABLE_PROPERTIES_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_pipeline_executable_properties");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPipelineExecutablePropertiesKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPipelineExecutablePropertiesKHR = unsafe extern "system" fn ( device : crate :: vk1_0 :: Device , p_pipeline_info : * const crate :: extensions :: khr_pipeline_executable_properties :: PipelineInfoKHR , p_executable_count : * mut u32 , p_properties : * mut crate :: extensions :: khr_pipeline_executable_properties :: PipelineExecutablePropertiesKHR , ) -> crate :: vk1_0 :: Result ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPipelineExecutableStatisticsKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPipelineExecutableStatisticsKHR = unsafe extern "system" fn ( device : crate :: vk1_0 :: Device , p_executable_info : * const crate :: extensions :: khr_pipeline_executable_properties :: PipelineExecutableInfoKHR , p_statistic_count : * mut u32 , p_statistics : * mut crate :: extensions :: khr_pipeline_executable_properties :: PipelineExecutableStatisticKHR , ) -> crate :: vk1_0 :: Result ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPipelineExecutableInternalRepresentationsKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPipelineExecutableInternalRepresentationsKHR = unsafe extern "system" fn ( device : crate :: vk1_0 :: Device , p_executable_info : * const crate :: extensions :: khr_pipeline_executable_properties :: PipelineExecutableInfoKHR , p_internal_representation_count : * mut u32 , p_internal_representations : * mut crate :: extensions :: khr_pipeline_executable_properties :: PipelineExecutableInternalRepresentationKHR , ) -> crate :: vk1_0 :: Result ;
#[doc = "Provides Device Commands for [`KhrPipelineExecutablePropertiesDeviceLoaderExt`](trait.KhrPipelineExecutablePropertiesDeviceLoaderExt.html)"]
pub struct KhrPipelineExecutablePropertiesDeviceCommands {
    pub get_pipeline_executable_properties_khr: PFN_vkGetPipelineExecutablePropertiesKHR,
    pub get_pipeline_executable_statistics_khr: PFN_vkGetPipelineExecutableStatisticsKHR,
    pub get_pipeline_executable_internal_representations_khr:
        PFN_vkGetPipelineExecutableInternalRepresentationsKHR,
}
impl KhrPipelineExecutablePropertiesDeviceCommands {
    #[inline]
    pub fn load(
        loader: &crate::DeviceLoader,
    ) -> Option<KhrPipelineExecutablePropertiesDeviceCommands> {
        unsafe {
            Some(KhrPipelineExecutablePropertiesDeviceCommands {
                get_pipeline_executable_properties_khr: std::mem::transmute(
                    loader.symbol("vkGetPipelineExecutablePropertiesKHR")?,
                ),
                get_pipeline_executable_statistics_khr: std::mem::transmute(
                    loader.symbol("vkGetPipelineExecutableStatisticsKHR")?,
                ),
                get_pipeline_executable_internal_representations_khr: std::mem::transmute(
                    loader.symbol("vkGetPipelineExecutableInternalRepresentationsKHR")?,
                ),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`KhrPipelineExecutablePropertiesDeviceCommands`](struct.KhrPipelineExecutablePropertiesDeviceCommands.html)"]
pub trait KhrPipelineExecutablePropertiesDeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPipelineExecutablePropertiesKHR.html) · Device Command"]
    unsafe fn get_pipeline_executable_properties_khr(
        &self,
        pipeline_info: &crate::extensions::khr_pipeline_executable_properties::PipelineInfoKHR,
        executable_count: Option<u32>,
    ) -> crate::utils::VulkanResult<
        Vec<crate::extensions::khr_pipeline_executable_properties::PipelineExecutablePropertiesKHR>,
    >;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPipelineExecutableStatisticsKHR.html) · Device Command"]
    unsafe fn get_pipeline_executable_statistics_khr(
        &self,
        executable_info : & crate :: extensions :: khr_pipeline_executable_properties :: PipelineExecutableInfoKHR,
        statistic_count: Option<u32>,
    ) -> crate::utils::VulkanResult<
        Vec<crate::extensions::khr_pipeline_executable_properties::PipelineExecutableStatisticKHR>,
    >;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPipelineExecutableInternalRepresentationsKHR.html) · Device Command"]
    unsafe fn get_pipeline_executable_internal_representations_khr ( & self , executable_info : & crate :: extensions :: khr_pipeline_executable_properties :: PipelineExecutableInfoKHR , internal_representation_count : Option < u32 > , ) -> crate :: utils :: VulkanResult < Vec < crate :: extensions :: khr_pipeline_executable_properties :: PipelineExecutableInternalRepresentationKHR > > ;
}
impl KhrPipelineExecutablePropertiesDeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPipelineExecutablePropertiesKHR.html) · Device Command"]
    unsafe fn get_pipeline_executable_properties_khr(
        &self,
        pipeline_info: &crate::extensions::khr_pipeline_executable_properties::PipelineInfoKHR,
        executable_count: Option<u32>,
    ) -> crate::utils::VulkanResult<
        Vec<crate::extensions::khr_pipeline_executable_properties::PipelineExecutablePropertiesKHR>,
    > {
        let function = self
            .khr_pipeline_executable_properties
            .as_ref()
            .expect("`khr_pipeline_executable_properties` not loaded")
            .get_pipeline_executable_properties_khr;
        let mut executable_count = executable_count.unwrap_or_else(|| {
            let mut val = Default::default();
            function(self.handle, pipeline_info, &mut val, std::ptr::null_mut());
            val
        });
        let mut properties = vec![Default::default(); executable_count as _];
        let _val = function(
            self.handle,
            pipeline_info,
            &mut executable_count,
            properties.as_mut_ptr(),
        );
        crate::utils::VulkanResult::new(_val, properties)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPipelineExecutableStatisticsKHR.html) · Device Command"]
    unsafe fn get_pipeline_executable_statistics_khr(
        &self,
        executable_info : & crate :: extensions :: khr_pipeline_executable_properties :: PipelineExecutableInfoKHR,
        statistic_count: Option<u32>,
    ) -> crate::utils::VulkanResult<
        Vec<crate::extensions::khr_pipeline_executable_properties::PipelineExecutableStatisticKHR>,
    > {
        let function = self
            .khr_pipeline_executable_properties
            .as_ref()
            .expect("`khr_pipeline_executable_properties` not loaded")
            .get_pipeline_executable_statistics_khr;
        let mut statistic_count = statistic_count.unwrap_or_else(|| {
            let mut val = Default::default();
            function(self.handle, executable_info, &mut val, std::ptr::null_mut());
            val
        });
        let mut statistics = vec![Default::default(); statistic_count as _];
        let _val = function(
            self.handle,
            executable_info,
            &mut statistic_count,
            statistics.as_mut_ptr(),
        );
        crate::utils::VulkanResult::new(_val, statistics)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPipelineExecutableInternalRepresentationsKHR.html) · Device Command"]unsafe fn get_pipeline_executable_internal_representations_khr ( & self , executable_info : & crate :: extensions :: khr_pipeline_executable_properties :: PipelineExecutableInfoKHR , internal_representation_count : Option < u32 > , ) -> crate :: utils :: VulkanResult < Vec < crate :: extensions :: khr_pipeline_executable_properties :: PipelineExecutableInternalRepresentationKHR > >{
        let function = self
            .khr_pipeline_executable_properties
            .as_ref()
            .expect("`khr_pipeline_executable_properties` not loaded")
            .get_pipeline_executable_internal_representations_khr;
        let mut internal_representation_count =
            internal_representation_count.unwrap_or_else(|| {
                let mut val = Default::default();
                function(self.handle, executable_info, &mut val, std::ptr::null_mut());
                val
            });
        let mut internal_representations =
            vec![Default::default(); internal_representation_count as _];
        let _val = function(
            self.handle,
            executable_info,
            &mut internal_representation_count,
            internal_representations.as_mut_ptr(),
        );
        crate::utils::VulkanResult::new(_val, internal_representations)
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineInfoKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub pipeline: crate::vk1_0::Pipeline,
}
impl PipelineInfoKHR {
    #[inline]
    pub fn builder<'a>(self) -> PipelineInfoKHRBuilder<'a> {
        PipelineInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PipelineInfoKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PipelineInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("pipeline", &self.pipeline)
            .finish()
    }
}
impl Default for PipelineInfoKHR {
    fn default() -> PipelineInfoKHR {
        PipelineInfoKHR {
            s_type: crate::vk1_0::StructureType::PIPELINE_INFO_KHR,
            p_next: std::ptr::null(),
            pipeline: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PipelineInfoKHR`](struct.PipelineInfoKHR.html)"]
#[repr(transparent)]
pub struct PipelineInfoKHRBuilder<'a>(PipelineInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> PipelineInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineInfoKHRBuilder<'a> {
        PipelineInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn pipeline(mut self, pipeline: crate::vk1_0::Pipeline) -> Self {
        self.0.pipeline = pipeline;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PipelineInfoKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for PipelineInfoKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineExecutablePropertiesKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub stages: crate::vk1_0::ShaderStageFlags,
    pub name: [std::os::raw::c_char; crate::vk1_0::MAX_DESCRIPTION_SIZE as usize],
    pub description: [std::os::raw::c_char; crate::vk1_0::MAX_DESCRIPTION_SIZE as usize],
    pub subgroup_size: u32,
}
impl PipelineExecutablePropertiesKHR {
    #[inline]
    pub fn builder<'a>(self) -> PipelineExecutablePropertiesKHRBuilder<'a> {
        PipelineExecutablePropertiesKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PipelineExecutablePropertiesKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PipelineExecutablePropertiesKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("stages", &self.stages)
            .field("name", &unsafe {
                std::ffi::CStr::from_ptr(self.name.as_ptr() as _)
            })
            .field("description", &unsafe {
                std::ffi::CStr::from_ptr(self.description.as_ptr() as _)
            })
            .field("subgroup_size", &self.subgroup_size)
            .finish()
    }
}
impl Default for PipelineExecutablePropertiesKHR {
    fn default() -> PipelineExecutablePropertiesKHR {
        PipelineExecutablePropertiesKHR {
            s_type: crate::vk1_0::StructureType::PIPELINE_EXECUTABLE_PROPERTIES_KHR,
            p_next: std::ptr::null_mut(),
            stages: Default::default(),
            name: unsafe { std::mem::zeroed() },
            description: unsafe { std::mem::zeroed() },
            subgroup_size: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PipelineExecutablePropertiesKHR`](struct.PipelineExecutablePropertiesKHR.html)"]
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
    #[allow(unused_mut)]
    #[inline]
    pub fn stages(mut self, stages: crate::vk1_0::ShaderStageFlags) -> Self {
        self.0.stages = stages;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn name(
        mut self,
        name: [std::os::raw::c_char; crate::vk1_0::MAX_DESCRIPTION_SIZE as usize],
    ) -> Self {
        self.0.name = name;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn description(
        mut self,
        description: [std::os::raw::c_char; crate::vk1_0::MAX_DESCRIPTION_SIZE as usize],
    ) -> Self {
        self.0.description = description;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn subgroup_size(mut self, subgroup_size: u32) -> Self {
        self.0.subgroup_size = subgroup_size;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PipelineExecutablePropertiesKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for PipelineExecutablePropertiesKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineExecutableInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub pipeline: crate::vk1_0::Pipeline,
    pub executable_index: u32,
}
impl PipelineExecutableInfoKHR {
    #[inline]
    pub fn builder<'a>(self) -> PipelineExecutableInfoKHRBuilder<'a> {
        PipelineExecutableInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PipelineExecutableInfoKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PipelineExecutableInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("pipeline", &self.pipeline)
            .field("executable_index", &self.executable_index)
            .finish()
    }
}
impl Default for PipelineExecutableInfoKHR {
    fn default() -> PipelineExecutableInfoKHR {
        PipelineExecutableInfoKHR {
            s_type: crate::vk1_0::StructureType::PIPELINE_EXECUTABLE_INFO_KHR,
            p_next: std::ptr::null(),
            pipeline: Default::default(),
            executable_index: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PipelineExecutableInfoKHR`](struct.PipelineExecutableInfoKHR.html)"]
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
    #[allow(unused_mut)]
    #[inline]
    pub fn pipeline(mut self, pipeline: crate::vk1_0::Pipeline) -> Self {
        self.0.pipeline = pipeline;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn executable_index(mut self, executable_index: u32) -> Self {
        self.0.executable_index = executable_index;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PipelineExecutableInfoKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for PipelineExecutableInfoKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineExecutableStatisticKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineExecutableStatisticKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub name: [std::os::raw::c_char; crate::vk1_0::MAX_DESCRIPTION_SIZE as usize],
    pub description: [std::os::raw::c_char; crate::vk1_0::MAX_DESCRIPTION_SIZE as usize],
    pub format:
        crate::extensions::khr_pipeline_executable_properties::PipelineExecutableStatisticFormatKHR,
    pub value:
        crate::extensions::khr_pipeline_executable_properties::PipelineExecutableStatisticValueKHR,
}
impl PipelineExecutableStatisticKHR {
    #[inline]
    pub fn builder<'a>(self) -> PipelineExecutableStatisticKHRBuilder<'a> {
        PipelineExecutableStatisticKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PipelineExecutableStatisticKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PipelineExecutableStatisticKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("name", &unsafe {
                std::ffi::CStr::from_ptr(self.name.as_ptr() as _)
            })
            .field("description", &unsafe {
                std::ffi::CStr::from_ptr(self.description.as_ptr() as _)
            })
            .field("format", &self.format)
            .field("value", &self.value)
            .finish()
    }
}
impl Default for PipelineExecutableStatisticKHR {
    fn default() -> PipelineExecutableStatisticKHR {
        PipelineExecutableStatisticKHR {
            s_type: crate::vk1_0::StructureType::PIPELINE_EXECUTABLE_STATISTIC_KHR,
            p_next: std::ptr::null_mut(),
            name: unsafe { std::mem::zeroed() },
            description: unsafe { std::mem::zeroed() },
            format: Default::default(),
            value: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PipelineExecutableStatisticKHR`](struct.PipelineExecutableStatisticKHR.html)"]
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
    #[allow(unused_mut)]
    #[inline]
    pub fn name(
        mut self,
        name: [std::os::raw::c_char; crate::vk1_0::MAX_DESCRIPTION_SIZE as usize],
    ) -> Self {
        self.0.name = name;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn description(
        mut self,
        description: [std::os::raw::c_char; crate::vk1_0::MAX_DESCRIPTION_SIZE as usize],
    ) -> Self {
        self.0.description = description;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn format(
        mut self,
        format : crate :: extensions :: khr_pipeline_executable_properties :: PipelineExecutableStatisticFormatKHR,
    ) -> Self {
        self.0.format = format;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn value(
        mut self,
        value : crate :: extensions :: khr_pipeline_executable_properties :: PipelineExecutableStatisticValueKHR,
    ) -> Self {
        self.0.value = value;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PipelineExecutableStatisticKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for PipelineExecutableStatisticKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineExecutableStatisticFormatKHR.html) · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PipelineExecutableStatisticFormatKHR(pub i32);
#[doc = "[Part of `extensions::khr_pipeline_executable_properties`](../../extensions/khr_pipeline_executable_properties/index.html)"]
impl PipelineExecutableStatisticFormatKHR {
    pub const BOOL32_KHR: Self = Self(0);
    pub const INT64_KHR: Self = Self(1);
    pub const UINT64_KHR: Self = Self(2);
    pub const FLOAT64_KHR: Self = Self(3);
}
impl std::fmt::Debug for PipelineExecutableStatisticFormatKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::BOOL32_KHR => "BOOL32_KHR",
            &Self::INT64_KHR => "INT64_KHR",
            &Self::UINT64_KHR => "UINT64_KHR",
            &Self::FLOAT64_KHR => "FLOAT64_KHR",
            _ => "Unknown enum variant",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineExecutableStatisticValueKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub union PipelineExecutableStatisticValueKHR {
    pub b32: crate::vk1_0::Bool32,
    pub i64: i64,
    pub u64: u64,
    pub f64: f64,
}
impl std::fmt::Debug for PipelineExecutableStatisticValueKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PipelineExecutableStatisticValueKHR")
            .finish()
    }
}
impl Default for PipelineExecutableStatisticValueKHR {
    fn default() -> PipelineExecutableStatisticValueKHR {
        unsafe { std::mem::zeroed() }
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineExecutableInternalRepresentationKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineExecutableInternalRepresentationKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub name: [std::os::raw::c_char; crate::vk1_0::MAX_DESCRIPTION_SIZE as usize],
    pub description: [std::os::raw::c_char; crate::vk1_0::MAX_DESCRIPTION_SIZE as usize],
    pub is_text: crate::vk1_0::Bool32,
    pub data_size: usize,
    pub p_data: *mut std::ffi::c_void,
}
impl PipelineExecutableInternalRepresentationKHR {
    #[inline]
    pub fn builder<'a>(self) -> PipelineExecutableInternalRepresentationKHRBuilder<'a> {
        PipelineExecutableInternalRepresentationKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PipelineExecutableInternalRepresentationKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PipelineExecutableInternalRepresentationKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("name", &unsafe {
                std::ffi::CStr::from_ptr(self.name.as_ptr() as _)
            })
            .field("description", &unsafe {
                std::ffi::CStr::from_ptr(self.description.as_ptr() as _)
            })
            .field("is_text", &(self.is_text != 0))
            .field("data_size", &self.data_size)
            .field("p_data", &self.p_data)
            .finish()
    }
}
impl Default for PipelineExecutableInternalRepresentationKHR {
    fn default() -> PipelineExecutableInternalRepresentationKHR {
        PipelineExecutableInternalRepresentationKHR {
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
#[derive(Copy, Clone)]
#[doc = "Builder of [`PipelineExecutableInternalRepresentationKHR`](struct.PipelineExecutableInternalRepresentationKHR.html)"]
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
    #[allow(unused_mut)]
    #[inline]
    pub fn name(
        mut self,
        name: [std::os::raw::c_char; crate::vk1_0::MAX_DESCRIPTION_SIZE as usize],
    ) -> Self {
        self.0.name = name;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn description(
        mut self,
        description: [std::os::raw::c_char; crate::vk1_0::MAX_DESCRIPTION_SIZE as usize],
    ) -> Self {
        self.0.description = description;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn is_text(mut self, is_text: bool) -> Self {
        self.0.is_text = is_text as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn data(mut self, data: &'a mut [std::ffi::c_void]) -> Self {
        self.0.data_size = data.len() as _;
        self.0.p_data = data.as_mut_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PipelineExecutableInternalRepresentationKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for PipelineExecutableInternalRepresentationKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDevicePipelineExecutablePropertiesFeaturesKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub pipeline_executable_info: crate::vk1_0::Bool32,
}
impl PhysicalDevicePipelineExecutablePropertiesFeaturesKHR {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDevicePipelineExecutablePropertiesFeaturesKHR,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDevicePipelineExecutablePropertiesFeaturesKHRBuilder<'a> {
        PhysicalDevicePipelineExecutablePropertiesFeaturesKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDevicePipelineExecutablePropertiesFeaturesKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDevicePipelineExecutablePropertiesFeaturesKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "pipeline_executable_info",
                &(self.pipeline_executable_info != 0),
            )
            .finish()
    }
}
impl Default for PhysicalDevicePipelineExecutablePropertiesFeaturesKHR {
    fn default() -> PhysicalDevicePipelineExecutablePropertiesFeaturesKHR {
        PhysicalDevicePipelineExecutablePropertiesFeaturesKHR { s_type : crate :: vk1_0 :: StructureType :: PHYSICAL_DEVICE_PIPELINE_EXECUTABLE_PROPERTIES_FEATURES_KHR , p_next : std :: ptr :: null_mut ( ) , pipeline_executable_info : Default :: default ( ) , }
    }
}
#[doc = "Used by [`PhysicalDevicePipelineExecutablePropertiesFeaturesKHR::extend`](struct.PhysicalDevicePipelineExecutablePropertiesFeaturesKHR.html#method.extend)"]
pub trait ExtendableByPhysicalDevicePipelineExecutablePropertiesFeaturesKHR {}
impl ExtendableByPhysicalDevicePipelineExecutablePropertiesFeaturesKHR
    for crate::vk1_1::PhysicalDeviceFeatures2
{
}
impl ExtendableByPhysicalDevicePipelineExecutablePropertiesFeaturesKHR
    for crate::vk1_0::DeviceCreateInfo
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDevicePipelineExecutablePropertiesFeaturesKHR`](struct.PhysicalDevicePipelineExecutablePropertiesFeaturesKHR.html)"]
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
    #[allow(unused_mut)]
    #[inline]
    pub fn pipeline_executable_info(mut self, pipeline_executable_info: bool) -> Self {
        self.0.pipeline_executable_info = pipeline_executable_info as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDevicePipelineExecutablePropertiesFeaturesKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDevicePipelineExecutablePropertiesFeaturesKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
