# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_NV_cooperative_matrix.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NV_COOPERATIVE_MATRIX_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NV_COOPERATIVE_MATRIX_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_NV_cooperative_matrix");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceCooperativeMatrixPropertiesNV.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesNV =
    unsafe extern "system" fn(
        physical_device: crate::vk1_0::PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut crate::extensions::nv_cooperative_matrix::CooperativeMatrixPropertiesNV,
    ) -> crate::vk1_0::Result;
#[doc = "Provides Instance Commands for [`NvCooperativeMatrixInstanceLoaderExt`](trait.NvCooperativeMatrixInstanceLoaderExt.html)"]
pub struct NvCooperativeMatrixInstanceCommands {
    pub get_physical_device_cooperative_matrix_properties_nv:
        PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesNV,
}
impl NvCooperativeMatrixInstanceCommands {
    #[inline]
    pub fn load(loader: &crate::InstanceLoader) -> Option<NvCooperativeMatrixInstanceCommands> {
        unsafe {
            Some(NvCooperativeMatrixInstanceCommands {
                get_physical_device_cooperative_matrix_properties_nv: std::mem::transmute(
                    loader.symbol("vkGetPhysicalDeviceCooperativeMatrixPropertiesNV")?,
                ),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`NvCooperativeMatrixInstanceCommands`](struct.NvCooperativeMatrixInstanceCommands.html)"]
pub trait NvCooperativeMatrixInstanceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceCooperativeMatrixPropertiesNV.html) · Instance Command"]
    unsafe fn get_physical_device_cooperative_matrix_properties_nv(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        property_count: Option<u32>,
    ) -> crate::utils::VulkanResult<
        Vec<crate::extensions::nv_cooperative_matrix::CooperativeMatrixPropertiesNV>,
    >;
}
impl NvCooperativeMatrixInstanceLoaderExt for crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceCooperativeMatrixPropertiesNV.html) · Instance Command"]
    unsafe fn get_physical_device_cooperative_matrix_properties_nv(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        property_count: Option<u32>,
    ) -> crate::utils::VulkanResult<
        Vec<crate::extensions::nv_cooperative_matrix::CooperativeMatrixPropertiesNV>,
    > {
        let function = self
            .nv_cooperative_matrix
            .as_ref()
            .expect("`nv_cooperative_matrix` not loaded")
            .get_physical_device_cooperative_matrix_properties_nv;
        let mut property_count = property_count.unwrap_or_else(|| {
            let mut val = Default::default();
            function(physical_device, &mut val, std::ptr::null_mut());
            val
        });
        let mut properties = vec![Default::default(); property_count as _];
        let _val = function(
            physical_device,
            &mut property_count,
            properties.as_mut_ptr(),
        );
        crate::utils::VulkanResult::new(_val, properties)
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCooperativeMatrixPropertiesNV.html) · Structure"]
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
impl CooperativeMatrixPropertiesNV {
    #[inline]
    pub fn builder<'a>(self) -> CooperativeMatrixPropertiesNVBuilder<'a> {
        CooperativeMatrixPropertiesNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for CooperativeMatrixPropertiesNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("CooperativeMatrixPropertiesNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("m_size", &self.m_size)
            .field("n_size", &self.n_size)
            .field("k_size", &self.k_size)
            .field("a_type", &self.a_type)
            .field("b_type", &self.b_type)
            .field("c_type", &self.c_type)
            .field("d_type", &self.d_type)
            .field("scope", &self.scope)
            .finish()
    }
}
impl Default for CooperativeMatrixPropertiesNV {
    fn default() -> CooperativeMatrixPropertiesNV {
        CooperativeMatrixPropertiesNV {
            s_type: crate::vk1_0::StructureType::COOPERATIVE_MATRIX_PROPERTIES_NV,
            p_next: std::ptr::null_mut(),
            m_size: Default::default(),
            n_size: Default::default(),
            k_size: Default::default(),
            a_type: Default::default(),
            b_type: Default::default(),
            c_type: Default::default(),
            d_type: Default::default(),
            scope: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`CooperativeMatrixPropertiesNV`](struct.CooperativeMatrixPropertiesNV.html)"]
#[repr(transparent)]
pub struct CooperativeMatrixPropertiesNVBuilder<'a>(
    CooperativeMatrixPropertiesNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> CooperativeMatrixPropertiesNVBuilder<'a> {
    #[inline]
    pub fn new() -> CooperativeMatrixPropertiesNVBuilder<'a> {
        CooperativeMatrixPropertiesNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn m_size(mut self, m_size: u32) -> Self {
        self.0.m_size = m_size;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn n_size(mut self, n_size: u32) -> Self {
        self.0.n_size = n_size;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn k_size(mut self, k_size: u32) -> Self {
        self.0.k_size = k_size;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn a_type(
        mut self,
        a_type: crate::extensions::nv_cooperative_matrix::ComponentTypeNV,
    ) -> Self {
        self.0.a_type = a_type;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn b_type(
        mut self,
        b_type: crate::extensions::nv_cooperative_matrix::ComponentTypeNV,
    ) -> Self {
        self.0.b_type = b_type;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn c_type(
        mut self,
        c_type: crate::extensions::nv_cooperative_matrix::ComponentTypeNV,
    ) -> Self {
        self.0.c_type = c_type;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn d_type(
        mut self,
        d_type: crate::extensions::nv_cooperative_matrix::ComponentTypeNV,
    ) -> Self {
        self.0.d_type = d_type;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn scope(mut self, scope: crate::extensions::nv_cooperative_matrix::ScopeNV) -> Self {
        self.0.scope = scope;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> CooperativeMatrixPropertiesNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for CooperativeMatrixPropertiesNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkComponentTypeNV.html) · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ComponentTypeNV(pub i32);
#[doc = "[Part of `extensions::nv_cooperative_matrix`](../../extensions/nv_cooperative_matrix/index.html)"]
impl ComponentTypeNV {
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
impl std::fmt::Debug for ComponentTypeNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
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
            _ => "(unknown)",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkScopeNV.html) · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ScopeNV(pub i32);
#[doc = "[Part of `extensions::nv_cooperative_matrix`](../../extensions/nv_cooperative_matrix/index.html)"]
impl ScopeNV {
    pub const DEVICE_NV: Self = Self(1);
    pub const WORKGROUP_NV: Self = Self(2);
    pub const SUBGROUP_NV: Self = Self(3);
    pub const QUEUE_FAMILY_NV: Self = Self(5);
}
impl std::fmt::Debug for ScopeNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::DEVICE_NV => "DEVICE_NV",
            &Self::WORKGROUP_NV => "WORKGROUP_NV",
            &Self::SUBGROUP_NV => "SUBGROUP_NV",
            &Self::QUEUE_FAMILY_NV => "QUEUE_FAMILY_NV",
            _ => "(unknown)",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceCooperativeMatrixFeaturesNV.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceCooperativeMatrixFeaturesNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub cooperative_matrix: crate::vk1_0::Bool32,
    pub cooperative_matrix_robust_buffer_access: crate::vk1_0::Bool32,
}
impl PhysicalDeviceCooperativeMatrixFeaturesNV {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceCooperativeMatrixFeaturesNV,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceCooperativeMatrixFeaturesNVBuilder<'a> {
        PhysicalDeviceCooperativeMatrixFeaturesNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceCooperativeMatrixFeaturesNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceCooperativeMatrixFeaturesNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("cooperative_matrix", &(self.cooperative_matrix != 0))
            .field(
                "cooperative_matrix_robust_buffer_access",
                &(self.cooperative_matrix_robust_buffer_access != 0),
            )
            .finish()
    }
}
impl Default for PhysicalDeviceCooperativeMatrixFeaturesNV {
    fn default() -> PhysicalDeviceCooperativeMatrixFeaturesNV {
        PhysicalDeviceCooperativeMatrixFeaturesNV {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_NV,
            p_next: std::ptr::null_mut(),
            cooperative_matrix: Default::default(),
            cooperative_matrix_robust_buffer_access: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceCooperativeMatrixFeaturesNV::extend`](struct.PhysicalDeviceCooperativeMatrixFeaturesNV.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceCooperativeMatrixFeaturesNV {}
impl ExtendableByPhysicalDeviceCooperativeMatrixFeaturesNV
    for crate::vk1_1::PhysicalDeviceFeatures2
{
}
impl ExtendableByPhysicalDeviceCooperativeMatrixFeaturesNV for crate::vk1_0::DeviceCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceCooperativeMatrixFeaturesNV`](struct.PhysicalDeviceCooperativeMatrixFeaturesNV.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceCooperativeMatrixFeaturesNVBuilder<'a>(
    PhysicalDeviceCooperativeMatrixFeaturesNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceCooperativeMatrixFeaturesNVBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceCooperativeMatrixFeaturesNVBuilder<'a> {
        PhysicalDeviceCooperativeMatrixFeaturesNVBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn cooperative_matrix(mut self, cooperative_matrix: bool) -> Self {
        self.0.cooperative_matrix = cooperative_matrix as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn cooperative_matrix_robust_buffer_access(
        mut self,
        cooperative_matrix_robust_buffer_access: bool,
    ) -> Self {
        self.0.cooperative_matrix_robust_buffer_access =
            cooperative_matrix_robust_buffer_access as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceCooperativeMatrixFeaturesNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceCooperativeMatrixFeaturesNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceCooperativeMatrixPropertiesNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub cooperative_matrix_supported_stages: crate::vk1_0::ShaderStageFlags,
}
impl PhysicalDeviceCooperativeMatrixPropertiesNV {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceCooperativeMatrixPropertiesNV,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceCooperativeMatrixPropertiesNVBuilder<'a> {
        PhysicalDeviceCooperativeMatrixPropertiesNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceCooperativeMatrixPropertiesNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceCooperativeMatrixPropertiesNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "cooperative_matrix_supported_stages",
                &self.cooperative_matrix_supported_stages,
            )
            .finish()
    }
}
impl Default for PhysicalDeviceCooperativeMatrixPropertiesNV {
    fn default() -> PhysicalDeviceCooperativeMatrixPropertiesNV {
        PhysicalDeviceCooperativeMatrixPropertiesNV {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_NV,
            p_next: std::ptr::null_mut(),
            cooperative_matrix_supported_stages: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceCooperativeMatrixPropertiesNV::extend`](struct.PhysicalDeviceCooperativeMatrixPropertiesNV.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceCooperativeMatrixPropertiesNV {}
impl ExtendableByPhysicalDeviceCooperativeMatrixPropertiesNV
    for crate::vk1_1::PhysicalDeviceProperties2
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceCooperativeMatrixPropertiesNV`](struct.PhysicalDeviceCooperativeMatrixPropertiesNV.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceCooperativeMatrixPropertiesNVBuilder<'a>(
    PhysicalDeviceCooperativeMatrixPropertiesNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceCooperativeMatrixPropertiesNVBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceCooperativeMatrixPropertiesNVBuilder<'a> {
        PhysicalDeviceCooperativeMatrixPropertiesNVBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn cooperative_matrix_supported_stages(
        mut self,
        cooperative_matrix_supported_stages: crate::vk1_0::ShaderStageFlags,
    ) -> Self {
        self.0.cooperative_matrix_supported_stages = cooperative_matrix_supported_stages;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceCooperativeMatrixPropertiesNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceCooperativeMatrixPropertiesNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
