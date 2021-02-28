#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_VERTEX_ATTRIBUTE_DIVISOR_SPEC_VERSION")]
pub const EXT_VERTEX_ATTRIBUTE_DIVISOR_SPEC_VERSION: u32 = 3;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_VERTEX_ATTRIBUTE_DIVISOR_EXTENSION_NAME")]
pub const EXT_VERTEX_ATTRIBUTE_DIVISOR_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_vertex_attribute_divisor");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVertexInputBindingDivisorDescriptionEXT.html) · Structure"]
#[doc(alias = "VkVertexInputBindingDivisorDescriptionEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VertexInputBindingDivisorDescriptionEXT {
    pub binding: u32,
    pub divisor: u32,
}
impl Default for VertexInputBindingDivisorDescriptionEXT {
    fn default() -> Self {
        Self {
            binding: Default::default(),
            divisor: Default::default(),
        }
    }
}
impl std::fmt::Debug for VertexInputBindingDivisorDescriptionEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VertexInputBindingDivisorDescriptionEXT")
            .field("binding", &self.binding)
            .field("divisor", &self.divisor)
            .finish()
    }
}
impl VertexInputBindingDivisorDescriptionEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> VertexInputBindingDivisorDescriptionEXTBuilder<'a> {
        VertexInputBindingDivisorDescriptionEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVertexInputBindingDivisorDescriptionEXT.html) · Builder of [`VertexInputBindingDivisorDescriptionEXT`]"]
#[repr(transparent)]
pub struct VertexInputBindingDivisorDescriptionEXTBuilder<'a>(VertexInputBindingDivisorDescriptionEXT, std::marker::PhantomData<&'a ()>);
impl<'a> VertexInputBindingDivisorDescriptionEXTBuilder<'a> {
    #[inline]
    pub fn new() -> VertexInputBindingDivisorDescriptionEXTBuilder<'a> {
        VertexInputBindingDivisorDescriptionEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn binding(mut self, binding: u32) -> Self {
        self.0.binding = binding as _;
        self
    }
    #[inline]
    pub fn divisor(mut self, divisor: u32) -> Self {
        self.0.divisor = divisor as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> VertexInputBindingDivisorDescriptionEXT {
        self.0
    }
}
impl<'a> std::default::Default for VertexInputBindingDivisorDescriptionEXTBuilder<'a> {
    fn default() -> VertexInputBindingDivisorDescriptionEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VertexInputBindingDivisorDescriptionEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
#[doc(alias = "VkPipelineVertexInputDivisorStateCreateInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineVertexInputDivisorStateCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub vertex_binding_divisor_count: u32,
    pub p_vertex_binding_divisors: *const crate::extensions::ext_vertex_attribute_divisor::VertexInputBindingDivisorDescriptionEXT,
}
impl Default for PipelineVertexInputDivisorStateCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            vertex_binding_divisor_count: Default::default(),
            p_vertex_binding_divisors: std::ptr::null(),
        }
    }
}
impl std::fmt::Debug for PipelineVertexInputDivisorStateCreateInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PipelineVertexInputDivisorStateCreateInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("vertex_binding_divisor_count", &self.vertex_binding_divisor_count)
            .field("p_vertex_binding_divisors", &self.p_vertex_binding_divisors)
            .finish()
    }
}
impl PipelineVertexInputDivisorStateCreateInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PipelineVertexInputDivisorStateCreateInfoEXTBuilder<'a> {
        PipelineVertexInputDivisorStateCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineVertexInputDivisorStateCreateInfoEXT.html) · Builder of [`PipelineVertexInputDivisorStateCreateInfoEXT`]"]
#[repr(transparent)]
pub struct PipelineVertexInputDivisorStateCreateInfoEXTBuilder<'a>(PipelineVertexInputDivisorStateCreateInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PipelineVertexInputDivisorStateCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineVertexInputDivisorStateCreateInfoEXTBuilder<'a> {
        PipelineVertexInputDivisorStateCreateInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn vertex_binding_divisors(mut self, vertex_binding_divisors: &'a [crate::extensions::ext_vertex_attribute_divisor::VertexInputBindingDivisorDescriptionEXTBuilder]) -> Self {
        self.0.p_vertex_binding_divisors = vertex_binding_divisors.as_ptr() as _;
        self.0.vertex_binding_divisor_count = vertex_binding_divisors.len() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PipelineVertexInputDivisorStateCreateInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for PipelineVertexInputDivisorStateCreateInfoEXTBuilder<'a> {
    fn default() -> PipelineVertexInputDivisorStateCreateInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PipelineVertexInputDivisorStateCreateInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceVertexAttributeDivisorPropertiesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub max_vertex_attrib_divisor: u32,
}
impl Default for PhysicalDeviceVertexAttributeDivisorPropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            max_vertex_attrib_divisor: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceVertexAttributeDivisorPropertiesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceVertexAttributeDivisorPropertiesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("max_vertex_attrib_divisor", &self.max_vertex_attrib_divisor)
            .finish()
    }
}
impl PhysicalDeviceVertexAttributeDivisorPropertiesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceVertexAttributeDivisorPropertiesEXTBuilder<'a> {
        PhysicalDeviceVertexAttributeDivisorPropertiesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT.html) · Builder of [`PhysicalDeviceVertexAttributeDivisorPropertiesEXT`]"]
#[repr(transparent)]
pub struct PhysicalDeviceVertexAttributeDivisorPropertiesEXTBuilder<'a>(PhysicalDeviceVertexAttributeDivisorPropertiesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceVertexAttributeDivisorPropertiesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceVertexAttributeDivisorPropertiesEXTBuilder<'a> {
        PhysicalDeviceVertexAttributeDivisorPropertiesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn max_vertex_attrib_divisor(mut self, max_vertex_attrib_divisor: u32) -> Self {
        self.0.max_vertex_attrib_divisor = max_vertex_attrib_divisor as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceVertexAttributeDivisorPropertiesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceVertexAttributeDivisorPropertiesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceVertexAttributeDivisorPropertiesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceVertexAttributeDivisorPropertiesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceVertexAttributeDivisorFeaturesEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceVertexAttributeDivisorFeaturesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceVertexAttributeDivisorFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub vertex_attribute_instance_rate_divisor: crate::vk1_0::Bool32,
    pub vertex_attribute_instance_rate_zero_divisor: crate::vk1_0::Bool32,
}
impl Default for PhysicalDeviceVertexAttributeDivisorFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            vertex_attribute_instance_rate_divisor: Default::default(),
            vertex_attribute_instance_rate_zero_divisor: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceVertexAttributeDivisorFeaturesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceVertexAttributeDivisorFeaturesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("vertex_attribute_instance_rate_divisor", &(self.vertex_attribute_instance_rate_divisor != 0))
            .field("vertex_attribute_instance_rate_zero_divisor", &(self.vertex_attribute_instance_rate_zero_divisor != 0))
            .finish()
    }
}
impl PhysicalDeviceVertexAttributeDivisorFeaturesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceVertexAttributeDivisorFeaturesEXTBuilder<'a> {
        PhysicalDeviceVertexAttributeDivisorFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceVertexAttributeDivisorFeaturesEXT.html) · Builder of [`PhysicalDeviceVertexAttributeDivisorFeaturesEXT`]"]
#[repr(transparent)]
pub struct PhysicalDeviceVertexAttributeDivisorFeaturesEXTBuilder<'a>(PhysicalDeviceVertexAttributeDivisorFeaturesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceVertexAttributeDivisorFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceVertexAttributeDivisorFeaturesEXTBuilder<'a> {
        PhysicalDeviceVertexAttributeDivisorFeaturesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn vertex_attribute_instance_rate_divisor(mut self, vertex_attribute_instance_rate_divisor: bool) -> Self {
        self.0.vertex_attribute_instance_rate_divisor = vertex_attribute_instance_rate_divisor as _;
        self
    }
    #[inline]
    pub fn vertex_attribute_instance_rate_zero_divisor(mut self, vertex_attribute_instance_rate_zero_divisor: bool) -> Self {
        self.0.vertex_attribute_instance_rate_zero_divisor = vertex_attribute_instance_rate_zero_divisor as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceVertexAttributeDivisorFeaturesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceVertexAttributeDivisorFeaturesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceVertexAttributeDivisorFeaturesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceVertexAttributeDivisorFeaturesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
