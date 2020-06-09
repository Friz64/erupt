# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_EXT_blend_operation_advanced.html)\n\n## Extends\n- [`AccessFlagBits`](../../vk1_0/struct.AccessFlagBits.html)\n- [`BlendOp`](../../vk1_0/struct.BlendOp.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_BLEND_OPERATION_ADVANCED_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_BLEND_OPERATION_ADVANCED_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_blend_operation_advanced");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceBlendOperationAdvancedFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub advanced_blend_coherent_operations: crate::vk1_0::Bool32,
}
impl PhysicalDeviceBlendOperationAdvancedFeaturesEXT {
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
    pub fn builder<'a>(self) -> PhysicalDeviceBlendOperationAdvancedFeaturesEXTBuilder<'a> {
        PhysicalDeviceBlendOperationAdvancedFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceBlendOperationAdvancedFeaturesEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceBlendOperationAdvancedFeaturesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "advanced_blend_coherent_operations",
                &(self.advanced_blend_coherent_operations != 0),
            )
            .finish()
    }
}
impl Default for PhysicalDeviceBlendOperationAdvancedFeaturesEXT {
    fn default() -> PhysicalDeviceBlendOperationAdvancedFeaturesEXT {
        PhysicalDeviceBlendOperationAdvancedFeaturesEXT {
            s_type:
                crate::vk1_0::StructureType::PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            advanced_blend_coherent_operations: Default::default(),
        }
    }
}
impl crate::ExtendableBy<PhysicalDeviceBlendOperationAdvancedFeaturesEXT>
    for crate::vk1_1::PhysicalDeviceFeatures2
{
}
impl crate::ExtendableBy<PhysicalDeviceBlendOperationAdvancedFeaturesEXT>
    for crate::vk1_0::DeviceCreateInfo
{
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT.html) · Builder of [`PhysicalDeviceBlendOperationAdvancedFeaturesEXT`](struct.PhysicalDeviceBlendOperationAdvancedFeaturesEXT.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceBlendOperationAdvancedFeaturesEXTBuilder<'a>(
    PhysicalDeviceBlendOperationAdvancedFeaturesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceBlendOperationAdvancedFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceBlendOperationAdvancedFeaturesEXTBuilder<'a> {
        PhysicalDeviceBlendOperationAdvancedFeaturesEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn advanced_blend_coherent_operations(
        mut self,
        advanced_blend_coherent_operations: bool,
    ) -> Self {
        self.0.advanced_blend_coherent_operations = advanced_blend_coherent_operations as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceBlendOperationAdvancedFeaturesEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceBlendOperationAdvancedFeaturesEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceBlendOperationAdvancedFeaturesEXTBuilder<'a> {
    type Target = PhysicalDeviceBlendOperationAdvancedFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceBlendOperationAdvancedFeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceBlendOperationAdvancedPropertiesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub advanced_blend_max_color_attachments: u32,
    pub advanced_blend_independent_blend: crate::vk1_0::Bool32,
    pub advanced_blend_non_premultiplied_src_color: crate::vk1_0::Bool32,
    pub advanced_blend_non_premultiplied_dst_color: crate::vk1_0::Bool32,
    pub advanced_blend_correlated_overlap: crate::vk1_0::Bool32,
    pub advanced_blend_all_operations: crate::vk1_0::Bool32,
}
impl PhysicalDeviceBlendOperationAdvancedPropertiesEXT {
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
    pub fn builder<'a>(self) -> PhysicalDeviceBlendOperationAdvancedPropertiesEXTBuilder<'a> {
        PhysicalDeviceBlendOperationAdvancedPropertiesEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceBlendOperationAdvancedPropertiesEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceBlendOperationAdvancedPropertiesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "advanced_blend_max_color_attachments",
                &self.advanced_blend_max_color_attachments,
            )
            .field(
                "advanced_blend_independent_blend",
                &(self.advanced_blend_independent_blend != 0),
            )
            .field(
                "advanced_blend_non_premultiplied_src_color",
                &(self.advanced_blend_non_premultiplied_src_color != 0),
            )
            .field(
                "advanced_blend_non_premultiplied_dst_color",
                &(self.advanced_blend_non_premultiplied_dst_color != 0),
            )
            .field(
                "advanced_blend_correlated_overlap",
                &(self.advanced_blend_correlated_overlap != 0),
            )
            .field(
                "advanced_blend_all_operations",
                &(self.advanced_blend_all_operations != 0),
            )
            .finish()
    }
}
impl Default for PhysicalDeviceBlendOperationAdvancedPropertiesEXT {
    fn default() -> PhysicalDeviceBlendOperationAdvancedPropertiesEXT {
        PhysicalDeviceBlendOperationAdvancedPropertiesEXT {
            s_type:
                crate::vk1_0::StructureType::PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            advanced_blend_max_color_attachments: Default::default(),
            advanced_blend_independent_blend: Default::default(),
            advanced_blend_non_premultiplied_src_color: Default::default(),
            advanced_blend_non_premultiplied_dst_color: Default::default(),
            advanced_blend_correlated_overlap: Default::default(),
            advanced_blend_all_operations: Default::default(),
        }
    }
}
impl crate::ExtendableBy<PhysicalDeviceBlendOperationAdvancedPropertiesEXT>
    for crate::vk1_1::PhysicalDeviceProperties2
{
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT.html) · Builder of [`PhysicalDeviceBlendOperationAdvancedPropertiesEXT`](struct.PhysicalDeviceBlendOperationAdvancedPropertiesEXT.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceBlendOperationAdvancedPropertiesEXTBuilder<'a>(
    PhysicalDeviceBlendOperationAdvancedPropertiesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceBlendOperationAdvancedPropertiesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceBlendOperationAdvancedPropertiesEXTBuilder<'a> {
        PhysicalDeviceBlendOperationAdvancedPropertiesEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn advanced_blend_max_color_attachments(
        mut self,
        advanced_blend_max_color_attachments: u32,
    ) -> Self {
        self.0.advanced_blend_max_color_attachments = advanced_blend_max_color_attachments;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn advanced_blend_independent_blend(
        mut self,
        advanced_blend_independent_blend: bool,
    ) -> Self {
        self.0.advanced_blend_independent_blend = advanced_blend_independent_blend as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn advanced_blend_non_premultiplied_src_color(
        mut self,
        advanced_blend_non_premultiplied_src_color: bool,
    ) -> Self {
        self.0.advanced_blend_non_premultiplied_src_color =
            advanced_blend_non_premultiplied_src_color as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn advanced_blend_non_premultiplied_dst_color(
        mut self,
        advanced_blend_non_premultiplied_dst_color: bool,
    ) -> Self {
        self.0.advanced_blend_non_premultiplied_dst_color =
            advanced_blend_non_premultiplied_dst_color as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn advanced_blend_correlated_overlap(
        mut self,
        advanced_blend_correlated_overlap: bool,
    ) -> Self {
        self.0.advanced_blend_correlated_overlap = advanced_blend_correlated_overlap as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn advanced_blend_all_operations(mut self, advanced_blend_all_operations: bool) -> Self {
        self.0.advanced_blend_all_operations = advanced_blend_all_operations as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceBlendOperationAdvancedPropertiesEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceBlendOperationAdvancedPropertiesEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceBlendOperationAdvancedPropertiesEXTBuilder<'a> {
    type Target = PhysicalDeviceBlendOperationAdvancedPropertiesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceBlendOperationAdvancedPropertiesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineColorBlendAdvancedStateCreateInfoEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineColorBlendAdvancedStateCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub src_premultiplied: crate::vk1_0::Bool32,
    pub dst_premultiplied: crate::vk1_0::Bool32,
    pub blend_overlap: crate::extensions::ext_blend_operation_advanced::BlendOverlapEXT,
}
impl PipelineColorBlendAdvancedStateCreateInfoEXT {
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
    pub fn builder<'a>(self) -> PipelineColorBlendAdvancedStateCreateInfoEXTBuilder<'a> {
        PipelineColorBlendAdvancedStateCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PipelineColorBlendAdvancedStateCreateInfoEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PipelineColorBlendAdvancedStateCreateInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("src_premultiplied", &(self.src_premultiplied != 0))
            .field("dst_premultiplied", &(self.dst_premultiplied != 0))
            .field("blend_overlap", &self.blend_overlap)
            .finish()
    }
}
impl Default for PipelineColorBlendAdvancedStateCreateInfoEXT {
    fn default() -> PipelineColorBlendAdvancedStateCreateInfoEXT {
        PipelineColorBlendAdvancedStateCreateInfoEXT {
            s_type:
                crate::vk1_0::StructureType::PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            src_premultiplied: Default::default(),
            dst_premultiplied: Default::default(),
            blend_overlap: Default::default(),
        }
    }
}
impl crate::ExtendableBy<PipelineColorBlendAdvancedStateCreateInfoEXT>
    for crate::vk1_0::PipelineColorBlendStateCreateInfo
{
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineColorBlendAdvancedStateCreateInfoEXT.html) · Builder of [`PipelineColorBlendAdvancedStateCreateInfoEXT`](struct.PipelineColorBlendAdvancedStateCreateInfoEXT.html)"]
#[repr(transparent)]
pub struct PipelineColorBlendAdvancedStateCreateInfoEXTBuilder<'a>(
    PipelineColorBlendAdvancedStateCreateInfoEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PipelineColorBlendAdvancedStateCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineColorBlendAdvancedStateCreateInfoEXTBuilder<'a> {
        PipelineColorBlendAdvancedStateCreateInfoEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn src_premultiplied(mut self, src_premultiplied: bool) -> Self {
        self.0.src_premultiplied = src_premultiplied as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn dst_premultiplied(mut self, dst_premultiplied: bool) -> Self {
        self.0.dst_premultiplied = dst_premultiplied as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn blend_overlap(
        mut self,
        blend_overlap: crate::extensions::ext_blend_operation_advanced::BlendOverlapEXT,
    ) -> Self {
        self.0.blend_overlap = blend_overlap;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PipelineColorBlendAdvancedStateCreateInfoEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for PipelineColorBlendAdvancedStateCreateInfoEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PipelineColorBlendAdvancedStateCreateInfoEXTBuilder<'a> {
    type Target = PipelineColorBlendAdvancedStateCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineColorBlendAdvancedStateCreateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBlendOverlapEXT.html) · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct BlendOverlapEXT(pub i32);
#[doc = "[Part of `extensions::ext_blend_operation_advanced`](../../extensions/ext_blend_operation_advanced/index.html)"]
impl BlendOverlapEXT {
    pub const UNCORRELATED_EXT: Self = Self(0);
    pub const DISJOINT_EXT: Self = Self(1);
    pub const CONJOINT_EXT: Self = Self(2);
}
impl std::fmt::Debug for BlendOverlapEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::UNCORRELATED_EXT => "UNCORRELATED_EXT",
            &Self::DISJOINT_EXT => "DISJOINT_EXT",
            &Self::CONJOINT_EXT => "CONJOINT_EXT",
            _ => "(unknown)",
        })
    }
}
