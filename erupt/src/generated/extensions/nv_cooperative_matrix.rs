#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_COOPERATIVE_MATRIX_SPEC_VERSION")]
pub const NV_COOPERATIVE_MATRIX_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_COOPERATIVE_MATRIX_EXTENSION_NAME")]
pub const NV_COOPERATIVE_MATRIX_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_NV_cooperative_matrix");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_NV: *const std::os::raw::c_char = crate::cstr!("vkGetPhysicalDeviceCooperativeMatrixPropertiesNV");
#[doc = "Provided by [`crate::extensions::nv_cooperative_matrix`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_NV: Self = Self(1000249000);
    pub const COOPERATIVE_MATRIX_PROPERTIES_NV: Self = Self(1000249001);
    pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_NV: Self = Self(1000249002);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkScopeNV.html) · Enum"]
#[doc(alias = "VkScopeNV")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct ScopeNV(pub i32);
impl std::fmt::Debug for ScopeNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::DEVICE_NV => "DEVICE_NV",
            &Self::WORKGROUP_NV => "WORKGROUP_NV",
            &Self::SUBGROUP_NV => "SUBGROUP_NV",
            &Self::QUEUE_FAMILY_NV => "QUEUE_FAMILY_NV",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::nv_cooperative_matrix`]"]
impl crate::extensions::nv_cooperative_matrix::ScopeNV {
    pub const DEVICE_NV: Self = Self(1);
    pub const WORKGROUP_NV: Self = Self(2);
    pub const SUBGROUP_NV: Self = Self(3);
    pub const QUEUE_FAMILY_NV: Self = Self(5);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkComponentTypeNV.html) · Enum"]
#[doc(alias = "VkComponentTypeNV")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct ComponentTypeNV(pub i32);
impl std::fmt::Debug for ComponentTypeNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::FLOAT16_NV => "FLOAT16_NV",
            &Self::FLOAT32_NV => "FLOAT32_NV",
            &Self::FLOAT64_NV => "FLOAT64_NV",
            &Self::SINT8_NV => "SINT8_NV",
            &Self::SINT16_NV => "SINT16_NV",
            &Self::SINT32_NV => "SINT32_NV",
            &Self::SINT64_NV => "SINT64_NV",
            &Self::UINT8_NV => "UINT8_NV",
            &Self::UINT16_NV => "UINT16_NV",
            &Self::UINT32_NV => "UINT32_NV",
            &Self::UINT64_NV => "UINT64_NV",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::nv_cooperative_matrix`]"]
impl crate::extensions::nv_cooperative_matrix::ComponentTypeNV {
    pub const FLOAT16_NV: Self = Self(0);
    pub const FLOAT32_NV: Self = Self(1);
    pub const FLOAT64_NV: Self = Self(2);
    pub const SINT8_NV: Self = Self(3);
    pub const SINT16_NV: Self = Self(4);
    pub const SINT32_NV: Self = Self(5);
    pub const SINT64_NV: Self = Self(6);
    pub const UINT8_NV: Self = Self(7);
    pub const UINT16_NV: Self = Self(8);
    pub const UINT32_NV: Self = Self(9);
    pub const UINT64_NV: Self = Self(10);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceCooperativeMatrixPropertiesNV.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesNV = unsafe extern "system" fn(physical_device: crate::vk1_0::PhysicalDevice, p_property_count: *mut u32, p_properties: *mut crate::extensions::nv_cooperative_matrix::CooperativeMatrixPropertiesNV) -> crate::vk1_0::Result;
impl<'a> crate::ExtendableFromConst<'a, PhysicalDeviceCooperativeMatrixFeaturesNV> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, PhysicalDeviceCooperativeMatrixFeaturesNVBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceCooperativeMatrixFeaturesNV> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceCooperativeMatrixFeaturesNVBuilder<'_>> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceCooperativeMatrixPropertiesNV> for crate::vk1_1::PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceCooperativeMatrixPropertiesNVBuilder<'_>> for crate::vk1_1::PhysicalDeviceProperties2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceCooperativeMatrixFeaturesNV.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceCooperativeMatrixFeaturesNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceCooperativeMatrixFeaturesNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub cooperative_matrix: crate::vk1_0::Bool32,
    pub cooperative_matrix_robust_buffer_access: crate::vk1_0::Bool32,
}
impl Default for PhysicalDeviceCooperativeMatrixFeaturesNV {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_NV, p_next: std::ptr::null_mut(), cooperative_matrix: Default::default(), cooperative_matrix_robust_buffer_access: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceCooperativeMatrixFeaturesNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceCooperativeMatrixFeaturesNV").field("s_type", &self.s_type).field("p_next", &self.p_next).field("cooperative_matrix", &(self.cooperative_matrix != 0)).field("cooperative_matrix_robust_buffer_access", &(self.cooperative_matrix_robust_buffer_access != 0)).finish()
    }
}
impl PhysicalDeviceCooperativeMatrixFeaturesNV {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceCooperativeMatrixFeaturesNVBuilder<'a> {
        PhysicalDeviceCooperativeMatrixFeaturesNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceCooperativeMatrixFeaturesNV.html) · Builder of [`PhysicalDeviceCooperativeMatrixFeaturesNV`]"]
#[repr(transparent)]
pub struct PhysicalDeviceCooperativeMatrixFeaturesNVBuilder<'a>(PhysicalDeviceCooperativeMatrixFeaturesNV, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceCooperativeMatrixFeaturesNVBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceCooperativeMatrixFeaturesNVBuilder<'a> {
        PhysicalDeviceCooperativeMatrixFeaturesNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn cooperative_matrix(mut self, cooperative_matrix: bool) -> Self {
        self.0.cooperative_matrix = cooperative_matrix as _;
        self
    }
    #[inline]
    pub fn cooperative_matrix_robust_buffer_access(mut self, cooperative_matrix_robust_buffer_access: bool) -> Self {
        self.0.cooperative_matrix_robust_buffer_access = cooperative_matrix_robust_buffer_access as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceCooperativeMatrixFeaturesNV {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceCooperativeMatrixFeaturesNVBuilder<'a> {
    fn default() -> PhysicalDeviceCooperativeMatrixFeaturesNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceCooperativeMatrixFeaturesNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceCooperativeMatrixFeaturesNVBuilder<'a> {
    type Target = PhysicalDeviceCooperativeMatrixFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceCooperativeMatrixFeaturesNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceCooperativeMatrixPropertiesNV.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceCooperativeMatrixPropertiesNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceCooperativeMatrixPropertiesNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub cooperative_matrix_supported_stages: crate::vk1_0::ShaderStageFlags,
}
impl Default for PhysicalDeviceCooperativeMatrixPropertiesNV {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_NV, p_next: std::ptr::null_mut(), cooperative_matrix_supported_stages: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceCooperativeMatrixPropertiesNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceCooperativeMatrixPropertiesNV").field("s_type", &self.s_type).field("p_next", &self.p_next).field("cooperative_matrix_supported_stages", &self.cooperative_matrix_supported_stages).finish()
    }
}
impl PhysicalDeviceCooperativeMatrixPropertiesNV {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceCooperativeMatrixPropertiesNVBuilder<'a> {
        PhysicalDeviceCooperativeMatrixPropertiesNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceCooperativeMatrixPropertiesNV.html) · Builder of [`PhysicalDeviceCooperativeMatrixPropertiesNV`]"]
#[repr(transparent)]
pub struct PhysicalDeviceCooperativeMatrixPropertiesNVBuilder<'a>(PhysicalDeviceCooperativeMatrixPropertiesNV, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceCooperativeMatrixPropertiesNVBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceCooperativeMatrixPropertiesNVBuilder<'a> {
        PhysicalDeviceCooperativeMatrixPropertiesNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn cooperative_matrix_supported_stages(mut self, cooperative_matrix_supported_stages: crate::vk1_0::ShaderStageFlags) -> Self {
        self.0.cooperative_matrix_supported_stages = cooperative_matrix_supported_stages as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceCooperativeMatrixPropertiesNV {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceCooperativeMatrixPropertiesNVBuilder<'a> {
    fn default() -> PhysicalDeviceCooperativeMatrixPropertiesNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceCooperativeMatrixPropertiesNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceCooperativeMatrixPropertiesNVBuilder<'a> {
    type Target = PhysicalDeviceCooperativeMatrixPropertiesNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceCooperativeMatrixPropertiesNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCooperativeMatrixPropertiesNV.html) · Structure"]
#[doc(alias = "VkCooperativeMatrixPropertiesNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CooperativeMatrixPropertiesNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub m_size: u32,
    pub n_size: u32,
    pub k_size: u32,
    pub a_type: crate::extensions::nv_cooperative_matrix::ComponentTypeNV,
    pub b_type: crate::extensions::nv_cooperative_matrix::ComponentTypeNV,
    pub c_type: crate::extensions::nv_cooperative_matrix::ComponentTypeNV,
    pub d_type: crate::extensions::nv_cooperative_matrix::ComponentTypeNV,
    pub scope: crate::extensions::nv_cooperative_matrix::ScopeNV,
}
impl Default for CooperativeMatrixPropertiesNV {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::COOPERATIVE_MATRIX_PROPERTIES_NV, p_next: std::ptr::null_mut(), m_size: Default::default(), n_size: Default::default(), k_size: Default::default(), a_type: Default::default(), b_type: Default::default(), c_type: Default::default(), d_type: Default::default(), scope: Default::default() }
    }
}
impl std::fmt::Debug for CooperativeMatrixPropertiesNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CooperativeMatrixPropertiesNV").field("s_type", &self.s_type).field("p_next", &self.p_next).field("m_size", &self.m_size).field("n_size", &self.n_size).field("k_size", &self.k_size).field("a_type", &self.a_type).field("b_type", &self.b_type).field("c_type", &self.c_type).field("d_type", &self.d_type).field("scope", &self.scope).finish()
    }
}
impl CooperativeMatrixPropertiesNV {
    #[inline]
    pub fn into_builder<'a>(self) -> CooperativeMatrixPropertiesNVBuilder<'a> {
        CooperativeMatrixPropertiesNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCooperativeMatrixPropertiesNV.html) · Builder of [`CooperativeMatrixPropertiesNV`]"]
#[repr(transparent)]
pub struct CooperativeMatrixPropertiesNVBuilder<'a>(CooperativeMatrixPropertiesNV, std::marker::PhantomData<&'a ()>);
impl<'a> CooperativeMatrixPropertiesNVBuilder<'a> {
    #[inline]
    pub fn new() -> CooperativeMatrixPropertiesNVBuilder<'a> {
        CooperativeMatrixPropertiesNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn m_size(mut self, m_size: u32) -> Self {
        self.0.m_size = m_size as _;
        self
    }
    #[inline]
    pub fn n_size(mut self, n_size: u32) -> Self {
        self.0.n_size = n_size as _;
        self
    }
    #[inline]
    pub fn k_size(mut self, k_size: u32) -> Self {
        self.0.k_size = k_size as _;
        self
    }
    #[inline]
    pub fn a_type(mut self, a_type: crate::extensions::nv_cooperative_matrix::ComponentTypeNV) -> Self {
        self.0.a_type = a_type as _;
        self
    }
    #[inline]
    pub fn b_type(mut self, b_type: crate::extensions::nv_cooperative_matrix::ComponentTypeNV) -> Self {
        self.0.b_type = b_type as _;
        self
    }
    #[inline]
    pub fn c_type(mut self, c_type: crate::extensions::nv_cooperative_matrix::ComponentTypeNV) -> Self {
        self.0.c_type = c_type as _;
        self
    }
    #[inline]
    pub fn d_type(mut self, d_type: crate::extensions::nv_cooperative_matrix::ComponentTypeNV) -> Self {
        self.0.d_type = d_type as _;
        self
    }
    #[inline]
    pub fn scope(mut self, scope: crate::extensions::nv_cooperative_matrix::ScopeNV) -> Self {
        self.0.scope = scope as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> CooperativeMatrixPropertiesNV {
        self.0
    }
}
impl<'a> std::default::Default for CooperativeMatrixPropertiesNVBuilder<'a> {
    fn default() -> CooperativeMatrixPropertiesNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for CooperativeMatrixPropertiesNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for CooperativeMatrixPropertiesNVBuilder<'a> {
    type Target = CooperativeMatrixPropertiesNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for CooperativeMatrixPropertiesNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::nv_cooperative_matrix`]"]
impl crate::InstanceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceCooperativeMatrixPropertiesNV.html) · Function"]
    #[doc(alias = "vkGetPhysicalDeviceCooperativeMatrixPropertiesNV")]
    pub unsafe fn get_physical_device_cooperative_matrix_properties_nv(&self, physical_device: crate::vk1_0::PhysicalDevice, property_count: Option<u32>) -> crate::utils::VulkanResult<Vec<crate::extensions::nv_cooperative_matrix::CooperativeMatrixPropertiesNV>> {
        let _function = self.get_physical_device_cooperative_matrix_properties_nv.expect(crate::NOT_LOADED_MESSAGE);
        let mut property_count = match property_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                _function(physical_device as _, &mut v, std::ptr::null_mut());
                v
            }
        };
        let mut properties = vec![Default::default(); property_count as _];
        let _return = _function(physical_device as _, &mut property_count, properties.as_mut_ptr());
        crate::utils::VulkanResult::new(_return, properties)
    }
}
