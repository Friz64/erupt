# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_EXT_vertex_attribute_divisor.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_VERTEX_ATTRIBUTE_DIVISOR_SPEC_VERSION: u32 = 3;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_VERTEX_ATTRIBUTE_DIVISOR_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_vertex_attribute_divisor");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceVertexAttributeDivisorPropertiesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub max_vertex_attrib_divisor: u32,
}
impl PhysicalDeviceVertexAttributeDivisorPropertiesEXT {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceVertexAttributeDivisorPropertiesEXT,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceVertexAttributeDivisorPropertiesEXTBuilder<'a> {
        PhysicalDeviceVertexAttributeDivisorPropertiesEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceVertexAttributeDivisorPropertiesEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceVertexAttributeDivisorPropertiesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("max_vertex_attrib_divisor", &self.max_vertex_attrib_divisor)
            .finish()
    }
}
impl Default for PhysicalDeviceVertexAttributeDivisorPropertiesEXT {
    fn default() -> PhysicalDeviceVertexAttributeDivisorPropertiesEXT {
        PhysicalDeviceVertexAttributeDivisorPropertiesEXT {
            s_type:
                crate::vk1_0::StructureType::PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            max_vertex_attrib_divisor: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceVertexAttributeDivisorPropertiesEXT::extend`](struct.PhysicalDeviceVertexAttributeDivisorPropertiesEXT.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceVertexAttributeDivisorPropertiesEXT {}
impl ExtendableByPhysicalDeviceVertexAttributeDivisorPropertiesEXT
    for crate::vk1_1::PhysicalDeviceProperties2
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceVertexAttributeDivisorPropertiesEXT`](struct.PhysicalDeviceVertexAttributeDivisorPropertiesEXT.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceVertexAttributeDivisorPropertiesEXTBuilder<'a>(
    PhysicalDeviceVertexAttributeDivisorPropertiesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceVertexAttributeDivisorPropertiesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceVertexAttributeDivisorPropertiesEXTBuilder<'a> {
        PhysicalDeviceVertexAttributeDivisorPropertiesEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_vertex_attrib_divisor(mut self, max_vertex_attrib_divisor: u32) -> Self {
        self.0.max_vertex_attrib_divisor = max_vertex_attrib_divisor;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceVertexAttributeDivisorPropertiesEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceVertexAttributeDivisorPropertiesEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceVertexAttributeDivisorPropertiesEXTBuilder<'a> {
    type Target = PhysicalDeviceVertexAttributeDivisorPropertiesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceVertexAttributeDivisorPropertiesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVertexInputBindingDivisorDescriptionEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VertexInputBindingDivisorDescriptionEXT {
    pub binding: u32,
    pub divisor: u32,
}
impl VertexInputBindingDivisorDescriptionEXT {
    #[inline]
    pub fn builder<'a>(self) -> VertexInputBindingDivisorDescriptionEXTBuilder<'a> {
        VertexInputBindingDivisorDescriptionEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for VertexInputBindingDivisorDescriptionEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("VertexInputBindingDivisorDescriptionEXT")
            .field("binding", &self.binding)
            .field("divisor", &self.divisor)
            .finish()
    }
}
impl Default for VertexInputBindingDivisorDescriptionEXT {
    fn default() -> VertexInputBindingDivisorDescriptionEXT {
        VertexInputBindingDivisorDescriptionEXT {
            binding: Default::default(),
            divisor: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`VertexInputBindingDivisorDescriptionEXT`](struct.VertexInputBindingDivisorDescriptionEXT.html)"]
#[repr(transparent)]
pub struct VertexInputBindingDivisorDescriptionEXTBuilder<'a>(
    VertexInputBindingDivisorDescriptionEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> VertexInputBindingDivisorDescriptionEXTBuilder<'a> {
    #[inline]
    pub fn new() -> VertexInputBindingDivisorDescriptionEXTBuilder<'a> {
        VertexInputBindingDivisorDescriptionEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn binding(mut self, binding: u32) -> Self {
        self.0.binding = binding;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn divisor(mut self, divisor: u32) -> Self {
        self.0.divisor = divisor;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> VertexInputBindingDivisorDescriptionEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for VertexInputBindingDivisorDescriptionEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for VertexInputBindingDivisorDescriptionEXTBuilder<'a> {
    type Target = VertexInputBindingDivisorDescriptionEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VertexInputBindingDivisorDescriptionEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineVertexInputDivisorStateCreateInfoEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineVertexInputDivisorStateCreateInfoEXT { pub s_type : crate :: vk1_0 :: StructureType , pub p_next : * const std :: ffi :: c_void , pub vertex_binding_divisor_count : u32 , pub p_vertex_binding_divisors : * const crate :: extensions :: ext_vertex_attribute_divisor :: VertexInputBindingDivisorDescriptionEXT , }
impl PipelineVertexInputDivisorStateCreateInfoEXT {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPipelineVertexInputDivisorStateCreateInfoEXT,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PipelineVertexInputDivisorStateCreateInfoEXTBuilder<'a> {
        PipelineVertexInputDivisorStateCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PipelineVertexInputDivisorStateCreateInfoEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PipelineVertexInputDivisorStateCreateInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "vertex_binding_divisor_count",
                &self.vertex_binding_divisor_count,
            )
            .field("p_vertex_binding_divisors", &self.p_vertex_binding_divisors)
            .finish()
    }
}
impl Default for PipelineVertexInputDivisorStateCreateInfoEXT {
    fn default() -> PipelineVertexInputDivisorStateCreateInfoEXT {
        PipelineVertexInputDivisorStateCreateInfoEXT {
            s_type:
                crate::vk1_0::StructureType::PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            vertex_binding_divisor_count: Default::default(),
            p_vertex_binding_divisors: std::ptr::null(),
        }
    }
}
#[doc = "Used by [`PipelineVertexInputDivisorStateCreateInfoEXT::extend`](struct.PipelineVertexInputDivisorStateCreateInfoEXT.html#method.extend)"]
pub trait ExtendableByPipelineVertexInputDivisorStateCreateInfoEXT {}
impl ExtendableByPipelineVertexInputDivisorStateCreateInfoEXT
    for crate::vk1_0::PipelineVertexInputStateCreateInfo
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PipelineVertexInputDivisorStateCreateInfoEXT`](struct.PipelineVertexInputDivisorStateCreateInfoEXT.html)"]
#[repr(transparent)]
pub struct PipelineVertexInputDivisorStateCreateInfoEXTBuilder<'a>(
    PipelineVertexInputDivisorStateCreateInfoEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PipelineVertexInputDivisorStateCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineVertexInputDivisorStateCreateInfoEXTBuilder<'a> {
        PipelineVertexInputDivisorStateCreateInfoEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn vertex_binding_divisors(
        mut self,
        vertex_binding_divisors : &'a [ crate :: extensions :: ext_vertex_attribute_divisor :: VertexInputBindingDivisorDescriptionEXTBuilder ],
    ) -> Self {
        self.0.vertex_binding_divisor_count = vertex_binding_divisors.len() as _;
        self.0.p_vertex_binding_divisors = vertex_binding_divisors.as_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PipelineVertexInputDivisorStateCreateInfoEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for PipelineVertexInputDivisorStateCreateInfoEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PipelineVertexInputDivisorStateCreateInfoEXTBuilder<'a> {
    type Target = PipelineVertexInputDivisorStateCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineVertexInputDivisorStateCreateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceVertexAttributeDivisorFeaturesEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceVertexAttributeDivisorFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub vertex_attribute_instance_rate_divisor: crate::vk1_0::Bool32,
    pub vertex_attribute_instance_rate_zero_divisor: crate::vk1_0::Bool32,
}
impl PhysicalDeviceVertexAttributeDivisorFeaturesEXT {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceVertexAttributeDivisorFeaturesEXT,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceVertexAttributeDivisorFeaturesEXTBuilder<'a> {
        PhysicalDeviceVertexAttributeDivisorFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceVertexAttributeDivisorFeaturesEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceVertexAttributeDivisorFeaturesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "vertex_attribute_instance_rate_divisor",
                &(self.vertex_attribute_instance_rate_divisor != 0),
            )
            .field(
                "vertex_attribute_instance_rate_zero_divisor",
                &(self.vertex_attribute_instance_rate_zero_divisor != 0),
            )
            .finish()
    }
}
impl Default for PhysicalDeviceVertexAttributeDivisorFeaturesEXT {
    fn default() -> PhysicalDeviceVertexAttributeDivisorFeaturesEXT {
        PhysicalDeviceVertexAttributeDivisorFeaturesEXT {
            s_type:
                crate::vk1_0::StructureType::PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            vertex_attribute_instance_rate_divisor: Default::default(),
            vertex_attribute_instance_rate_zero_divisor: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceVertexAttributeDivisorFeaturesEXT::extend`](struct.PhysicalDeviceVertexAttributeDivisorFeaturesEXT.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceVertexAttributeDivisorFeaturesEXT {}
impl ExtendableByPhysicalDeviceVertexAttributeDivisorFeaturesEXT
    for crate::vk1_1::PhysicalDeviceFeatures2
{
}
impl ExtendableByPhysicalDeviceVertexAttributeDivisorFeaturesEXT
    for crate::vk1_0::DeviceCreateInfo
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceVertexAttributeDivisorFeaturesEXT`](struct.PhysicalDeviceVertexAttributeDivisorFeaturesEXT.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceVertexAttributeDivisorFeaturesEXTBuilder<'a>(
    PhysicalDeviceVertexAttributeDivisorFeaturesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceVertexAttributeDivisorFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceVertexAttributeDivisorFeaturesEXTBuilder<'a> {
        PhysicalDeviceVertexAttributeDivisorFeaturesEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn vertex_attribute_instance_rate_divisor(
        mut self,
        vertex_attribute_instance_rate_divisor: bool,
    ) -> Self {
        self.0.vertex_attribute_instance_rate_divisor =
            vertex_attribute_instance_rate_divisor as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn vertex_attribute_instance_rate_zero_divisor(
        mut self,
        vertex_attribute_instance_rate_zero_divisor: bool,
    ) -> Self {
        self.0.vertex_attribute_instance_rate_zero_divisor =
            vertex_attribute_instance_rate_zero_divisor as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceVertexAttributeDivisorFeaturesEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceVertexAttributeDivisorFeaturesEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceVertexAttributeDivisorFeaturesEXTBuilder<'a> {
    type Target = PhysicalDeviceVertexAttributeDivisorFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceVertexAttributeDivisorFeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
