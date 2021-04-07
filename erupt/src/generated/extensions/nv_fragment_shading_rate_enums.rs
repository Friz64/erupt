#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_FRAGMENT_SHADING_RATE_ENUMS_SPEC_VERSION")]
pub const NV_FRAGMENT_SHADING_RATE_ENUMS_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_FRAGMENT_SHADING_RATE_ENUMS_EXTENSION_NAME")]
pub const NV_FRAGMENT_SHADING_RATE_ENUMS_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_NV_fragment_shading_rate_enums");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_SET_FRAGMENT_SHADING_RATE_ENUM_NV: *const std::os::raw::c_char = crate::cstr!("vkCmdSetFragmentShadingRateEnumNV");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFragmentShadingRateNV.html) · Enum"]
#[doc(alias = "VkFragmentShadingRateNV")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct FragmentShadingRateNV(pub i32);
impl std::fmt::Debug for FragmentShadingRateNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::_1_INVOCATION_PER_PIXEL_NV => "_1_INVOCATION_PER_PIXEL_NV",
            &Self::_1_INVOCATION_PER_1X2_PIXELS_NV => "_1_INVOCATION_PER_1X2_PIXELS_NV",
            &Self::_1_INVOCATION_PER_2X1_PIXELS_NV => "_1_INVOCATION_PER_2X1_PIXELS_NV",
            &Self::_1_INVOCATION_PER_2X2_PIXELS_NV => "_1_INVOCATION_PER_2X2_PIXELS_NV",
            &Self::_1_INVOCATION_PER_2X4_PIXELS_NV => "_1_INVOCATION_PER_2X4_PIXELS_NV",
            &Self::_1_INVOCATION_PER_4X2_PIXELS_NV => "_1_INVOCATION_PER_4X2_PIXELS_NV",
            &Self::_1_INVOCATION_PER_4X4_PIXELS_NV => "_1_INVOCATION_PER_4X4_PIXELS_NV",
            &Self::_2_INVOCATIONS_PER_PIXEL_NV => "_2_INVOCATIONS_PER_PIXEL_NV",
            &Self::_4_INVOCATIONS_PER_PIXEL_NV => "_4_INVOCATIONS_PER_PIXEL_NV",
            &Self::_8_INVOCATIONS_PER_PIXEL_NV => "_8_INVOCATIONS_PER_PIXEL_NV",
            &Self::_16_INVOCATIONS_PER_PIXEL_NV => "_16_INVOCATIONS_PER_PIXEL_NV",
            &Self::NO_INVOCATIONS_NV => "NO_INVOCATIONS_NV",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::nv_fragment_shading_rate_enums`]"]
impl FragmentShadingRateNV {
    pub const _1_INVOCATION_PER_PIXEL_NV: Self = Self(0);
    pub const _1_INVOCATION_PER_1X2_PIXELS_NV: Self = Self(1);
    pub const _1_INVOCATION_PER_2X1_PIXELS_NV: Self = Self(4);
    pub const _1_INVOCATION_PER_2X2_PIXELS_NV: Self = Self(5);
    pub const _1_INVOCATION_PER_2X4_PIXELS_NV: Self = Self(6);
    pub const _1_INVOCATION_PER_4X2_PIXELS_NV: Self = Self(9);
    pub const _1_INVOCATION_PER_4X4_PIXELS_NV: Self = Self(10);
    pub const _2_INVOCATIONS_PER_PIXEL_NV: Self = Self(11);
    pub const _4_INVOCATIONS_PER_PIXEL_NV: Self = Self(12);
    pub const _8_INVOCATIONS_PER_PIXEL_NV: Self = Self(13);
    pub const _16_INVOCATIONS_PER_PIXEL_NV: Self = Self(14);
    pub const NO_INVOCATIONS_NV: Self = Self(15);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFragmentShadingRateTypeNV.html) · Enum"]
#[doc(alias = "VkFragmentShadingRateTypeNV")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct FragmentShadingRateTypeNV(pub i32);
impl std::fmt::Debug for FragmentShadingRateTypeNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::FRAGMENT_SIZE_NV => "FRAGMENT_SIZE_NV",
            &Self::ENUMS_NV => "ENUMS_NV",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::nv_fragment_shading_rate_enums`]"]
impl FragmentShadingRateTypeNV {
    pub const FRAGMENT_SIZE_NV: Self = Self(0);
    pub const ENUMS_NV: Self = Self(1);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetFragmentShadingRateEnumNV.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetFragmentShadingRateEnumNV = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    shading_rate: crate::extensions::nv_fragment_shading_rate_enums::FragmentShadingRateNV,
    combiner_ops: [crate::extensions::khr_fragment_shading_rate::FragmentShadingRateCombinerOpKHR; 2],
) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceFragmentShadingRateEnumsFeaturesNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub fragment_shading_rate_enums: crate::vk1_0::Bool32,
    pub supersample_fragment_shading_rates: crate::vk1_0::Bool32,
    pub no_invocation_fragment_shading_rates: crate::vk1_0::Bool32,
}
impl Default for PhysicalDeviceFragmentShadingRateEnumsFeaturesNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_FEATURES_NV,
            p_next: std::ptr::null_mut(),
            fragment_shading_rate_enums: Default::default(),
            supersample_fragment_shading_rates: Default::default(),
            no_invocation_fragment_shading_rates: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceFragmentShadingRateEnumsFeaturesNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceFragmentShadingRateEnumsFeaturesNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("fragment_shading_rate_enums", &(self.fragment_shading_rate_enums != 0))
            .field("supersample_fragment_shading_rates", &(self.supersample_fragment_shading_rates != 0))
            .field("no_invocation_fragment_shading_rates", &(self.no_invocation_fragment_shading_rates != 0))
            .finish()
    }
}
impl PhysicalDeviceFragmentShadingRateEnumsFeaturesNV {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceFragmentShadingRateEnumsFeaturesNVBuilder<'a> {
        PhysicalDeviceFragmentShadingRateEnumsFeaturesNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV.html) · Builder of [`PhysicalDeviceFragmentShadingRateEnumsFeaturesNV`]"]
#[repr(transparent)]
pub struct PhysicalDeviceFragmentShadingRateEnumsFeaturesNVBuilder<'a>(PhysicalDeviceFragmentShadingRateEnumsFeaturesNV, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceFragmentShadingRateEnumsFeaturesNVBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceFragmentShadingRateEnumsFeaturesNVBuilder<'a> {
        PhysicalDeviceFragmentShadingRateEnumsFeaturesNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn fragment_shading_rate_enums(mut self, fragment_shading_rate_enums: bool) -> Self {
        self.0.fragment_shading_rate_enums = fragment_shading_rate_enums as _;
        self
    }
    #[inline]
    pub fn supersample_fragment_shading_rates(mut self, supersample_fragment_shading_rates: bool) -> Self {
        self.0.supersample_fragment_shading_rates = supersample_fragment_shading_rates as _;
        self
    }
    #[inline]
    pub fn no_invocation_fragment_shading_rates(mut self, no_invocation_fragment_shading_rates: bool) -> Self {
        self.0.no_invocation_fragment_shading_rates = no_invocation_fragment_shading_rates as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceFragmentShadingRateEnumsFeaturesNV {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceFragmentShadingRateEnumsFeaturesNVBuilder<'a> {
    fn default() -> PhysicalDeviceFragmentShadingRateEnumsFeaturesNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceFragmentShadingRateEnumsFeaturesNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceFragmentShadingRateEnumsFeaturesNVBuilder<'a> {
    type Target = PhysicalDeviceFragmentShadingRateEnumsFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceFragmentShadingRateEnumsFeaturesNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceFragmentShadingRateEnumsPropertiesNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub max_fragment_shading_rate_invocation_count: crate::vk1_0::SampleCountFlagBits,
}
impl Default for PhysicalDeviceFragmentShadingRateEnumsPropertiesNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_PROPERTIES_NV,
            p_next: std::ptr::null_mut(),
            max_fragment_shading_rate_invocation_count: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceFragmentShadingRateEnumsPropertiesNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceFragmentShadingRateEnumsPropertiesNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("max_fragment_shading_rate_invocation_count", &self.max_fragment_shading_rate_invocation_count)
            .finish()
    }
}
impl PhysicalDeviceFragmentShadingRateEnumsPropertiesNV {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceFragmentShadingRateEnumsPropertiesNVBuilder<'a> {
        PhysicalDeviceFragmentShadingRateEnumsPropertiesNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV.html) · Builder of [`PhysicalDeviceFragmentShadingRateEnumsPropertiesNV`]"]
#[repr(transparent)]
pub struct PhysicalDeviceFragmentShadingRateEnumsPropertiesNVBuilder<'a>(PhysicalDeviceFragmentShadingRateEnumsPropertiesNV, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceFragmentShadingRateEnumsPropertiesNVBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceFragmentShadingRateEnumsPropertiesNVBuilder<'a> {
        PhysicalDeviceFragmentShadingRateEnumsPropertiesNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn max_fragment_shading_rate_invocation_count(mut self, max_fragment_shading_rate_invocation_count: crate::vk1_0::SampleCountFlagBits) -> Self {
        self.0.max_fragment_shading_rate_invocation_count = max_fragment_shading_rate_invocation_count as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceFragmentShadingRateEnumsPropertiesNV {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceFragmentShadingRateEnumsPropertiesNVBuilder<'a> {
    fn default() -> PhysicalDeviceFragmentShadingRateEnumsPropertiesNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceFragmentShadingRateEnumsPropertiesNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceFragmentShadingRateEnumsPropertiesNVBuilder<'a> {
    type Target = PhysicalDeviceFragmentShadingRateEnumsPropertiesNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceFragmentShadingRateEnumsPropertiesNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineFragmentShadingRateEnumStateCreateInfoNV.html) · Structure"]
#[doc(alias = "VkPipelineFragmentShadingRateEnumStateCreateInfoNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineFragmentShadingRateEnumStateCreateInfoNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub shading_rate_type: crate::extensions::nv_fragment_shading_rate_enums::FragmentShadingRateTypeNV,
    pub shading_rate: crate::extensions::nv_fragment_shading_rate_enums::FragmentShadingRateNV,
    pub combiner_ops: [crate::extensions::khr_fragment_shading_rate::FragmentShadingRateCombinerOpKHR; 2],
}
impl Default for PipelineFragmentShadingRateEnumStateCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PIPELINE_FRAGMENT_SHADING_RATE_ENUM_STATE_CREATE_INFO_NV,
            p_next: std::ptr::null(),
            shading_rate_type: Default::default(),
            shading_rate: Default::default(),
            combiner_ops: unsafe { std::mem::zeroed() },
        }
    }
}
impl std::fmt::Debug for PipelineFragmentShadingRateEnumStateCreateInfoNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PipelineFragmentShadingRateEnumStateCreateInfoNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("shading_rate_type", &self.shading_rate_type)
            .field("shading_rate", &self.shading_rate)
            .field("combiner_ops", &self.combiner_ops)
            .finish()
    }
}
impl PipelineFragmentShadingRateEnumStateCreateInfoNV {
    #[inline]
    pub fn into_builder<'a>(self) -> PipelineFragmentShadingRateEnumStateCreateInfoNVBuilder<'a> {
        PipelineFragmentShadingRateEnumStateCreateInfoNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineFragmentShadingRateEnumStateCreateInfoNV.html) · Builder of [`PipelineFragmentShadingRateEnumStateCreateInfoNV`]"]
#[repr(transparent)]
pub struct PipelineFragmentShadingRateEnumStateCreateInfoNVBuilder<'a>(PipelineFragmentShadingRateEnumStateCreateInfoNV, std::marker::PhantomData<&'a ()>);
impl<'a> PipelineFragmentShadingRateEnumStateCreateInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineFragmentShadingRateEnumStateCreateInfoNVBuilder<'a> {
        PipelineFragmentShadingRateEnumStateCreateInfoNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn shading_rate_type(mut self, shading_rate_type: crate::extensions::nv_fragment_shading_rate_enums::FragmentShadingRateTypeNV) -> Self {
        self.0.shading_rate_type = shading_rate_type as _;
        self
    }
    #[inline]
    pub fn shading_rate(mut self, shading_rate: crate::extensions::nv_fragment_shading_rate_enums::FragmentShadingRateNV) -> Self {
        self.0.shading_rate = shading_rate as _;
        self
    }
    #[inline]
    pub fn combiner_ops(mut self, combiner_ops: [crate::extensions::khr_fragment_shading_rate::FragmentShadingRateCombinerOpKHR; 2]) -> Self {
        self.0.combiner_ops = combiner_ops as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PipelineFragmentShadingRateEnumStateCreateInfoNV {
        self.0
    }
}
impl<'a> std::default::Default for PipelineFragmentShadingRateEnumStateCreateInfoNVBuilder<'a> {
    fn default() -> PipelineFragmentShadingRateEnumStateCreateInfoNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PipelineFragmentShadingRateEnumStateCreateInfoNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PipelineFragmentShadingRateEnumStateCreateInfoNVBuilder<'a> {
    type Target = PipelineFragmentShadingRateEnumStateCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineFragmentShadingRateEnumStateCreateInfoNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::nv_fragment_shading_rate_enums`]"]
impl crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetFragmentShadingRateEnumNV.html) · Function"]
    #[doc(alias = "vkCmdSetFragmentShadingRateEnumNV")]
    pub unsafe fn cmd_set_fragment_shading_rate_enum_nv(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        shading_rate: crate::extensions::nv_fragment_shading_rate_enums::FragmentShadingRateNV,
        combiner_ops: [crate::extensions::khr_fragment_shading_rate::FragmentShadingRateCombinerOpKHR; 2],
    ) -> () {
        let _function = self.cmd_set_fragment_shading_rate_enum_nv.expect("`cmd_set_fragment_shading_rate_enum_nv` is not loaded");
        let _return = _function(command_buffer as _, shading_rate as _, combiner_ops as _);
        ()
    }
}
