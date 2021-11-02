#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_INLINE_UNIFORM_BLOCK_SPEC_VERSION")]
pub const EXT_INLINE_UNIFORM_BLOCK_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_INLINE_UNIFORM_BLOCK_EXTENSION_NAME")]
pub const EXT_INLINE_UNIFORM_BLOCK_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_inline_uniform_block");
#[doc = "Provided by [`crate::extensions::ext_inline_uniform_block`]"]
impl crate::vk1_0::DescriptorType {
    pub const INLINE_UNIFORM_BLOCK_EXT: Self = Self(1000138000);
}
#[doc = "Provided by [`crate::extensions::ext_inline_uniform_block`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES_EXT: Self = Self(1000138000);
    pub const PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES_EXT: Self = Self(1000138001);
    pub const WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK_EXT: Self = Self(1000138002);
    pub const DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO_EXT: Self = Self(1000138003);
}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceInlineUniformBlockFeaturesEXT> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceInlineUniformBlockFeaturesEXTBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, WriteDescriptorSetInlineUniformBlockEXT> for crate::vk1_0::WriteDescriptorSetBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, WriteDescriptorSetInlineUniformBlockEXTBuilder<'_>> for crate::vk1_0::WriteDescriptorSetBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, DescriptorPoolInlineUniformBlockCreateInfoEXT> for crate::vk1_0::DescriptorPoolCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, DescriptorPoolInlineUniformBlockCreateInfoEXTBuilder<'_>> for crate::vk1_0::DescriptorPoolCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceInlineUniformBlockFeaturesEXT> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceInlineUniformBlockFeaturesEXTBuilder<'_>> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceInlineUniformBlockPropertiesEXT> for crate::vk1_1::PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceInlineUniformBlockPropertiesEXTBuilder<'_>> for crate::vk1_1::PhysicalDeviceProperties2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceInlineUniformBlockFeaturesEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceInlineUniformBlockFeaturesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceInlineUniformBlockFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub inline_uniform_block: crate::vk1_0::Bool32,
    pub descriptor_binding_inline_uniform_block_update_after_bind: crate::vk1_0::Bool32,
}
impl PhysicalDeviceInlineUniformBlockFeaturesEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES_EXT;
}
impl Default for PhysicalDeviceInlineUniformBlockFeaturesEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), inline_uniform_block: Default::default(), descriptor_binding_inline_uniform_block_update_after_bind: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceInlineUniformBlockFeaturesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceInlineUniformBlockFeaturesEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("inline_uniform_block", &(self.inline_uniform_block != 0)).field("descriptor_binding_inline_uniform_block_update_after_bind", &(self.descriptor_binding_inline_uniform_block_update_after_bind != 0)).finish()
    }
}
impl PhysicalDeviceInlineUniformBlockFeaturesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceInlineUniformBlockFeaturesEXTBuilder<'a> {
        PhysicalDeviceInlineUniformBlockFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceInlineUniformBlockFeaturesEXT.html) · Builder of [`PhysicalDeviceInlineUniformBlockFeaturesEXT`]"]
#[repr(transparent)]
pub struct PhysicalDeviceInlineUniformBlockFeaturesEXTBuilder<'a>(PhysicalDeviceInlineUniformBlockFeaturesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceInlineUniformBlockFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceInlineUniformBlockFeaturesEXTBuilder<'a> {
        PhysicalDeviceInlineUniformBlockFeaturesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn inline_uniform_block(mut self, inline_uniform_block: bool) -> Self {
        self.0.inline_uniform_block = inline_uniform_block as _;
        self
    }
    #[inline]
    pub fn descriptor_binding_inline_uniform_block_update_after_bind(mut self, descriptor_binding_inline_uniform_block_update_after_bind: bool) -> Self {
        self.0.descriptor_binding_inline_uniform_block_update_after_bind = descriptor_binding_inline_uniform_block_update_after_bind as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> PhysicalDeviceInlineUniformBlockFeaturesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceInlineUniformBlockFeaturesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceInlineUniformBlockFeaturesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceInlineUniformBlockFeaturesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceInlineUniformBlockFeaturesEXTBuilder<'a> {
    type Target = PhysicalDeviceInlineUniformBlockFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceInlineUniformBlockFeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceInlineUniformBlockPropertiesEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceInlineUniformBlockPropertiesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceInlineUniformBlockPropertiesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub max_inline_uniform_block_size: u32,
    pub max_per_stage_descriptor_inline_uniform_blocks: u32,
    pub max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: u32,
    pub max_descriptor_set_inline_uniform_blocks: u32,
    pub max_descriptor_set_update_after_bind_inline_uniform_blocks: u32,
}
impl PhysicalDeviceInlineUniformBlockPropertiesEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES_EXT;
}
impl Default for PhysicalDeviceInlineUniformBlockPropertiesEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), max_inline_uniform_block_size: Default::default(), max_per_stage_descriptor_inline_uniform_blocks: Default::default(), max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: Default::default(), max_descriptor_set_inline_uniform_blocks: Default::default(), max_descriptor_set_update_after_bind_inline_uniform_blocks: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceInlineUniformBlockPropertiesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceInlineUniformBlockPropertiesEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("max_inline_uniform_block_size", &self.max_inline_uniform_block_size).field("max_per_stage_descriptor_inline_uniform_blocks", &self.max_per_stage_descriptor_inline_uniform_blocks).field("max_per_stage_descriptor_update_after_bind_inline_uniform_blocks", &self.max_per_stage_descriptor_update_after_bind_inline_uniform_blocks).field("max_descriptor_set_inline_uniform_blocks", &self.max_descriptor_set_inline_uniform_blocks).field("max_descriptor_set_update_after_bind_inline_uniform_blocks", &self.max_descriptor_set_update_after_bind_inline_uniform_blocks).finish()
    }
}
impl PhysicalDeviceInlineUniformBlockPropertiesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceInlineUniformBlockPropertiesEXTBuilder<'a> {
        PhysicalDeviceInlineUniformBlockPropertiesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceInlineUniformBlockPropertiesEXT.html) · Builder of [`PhysicalDeviceInlineUniformBlockPropertiesEXT`]"]
#[repr(transparent)]
pub struct PhysicalDeviceInlineUniformBlockPropertiesEXTBuilder<'a>(PhysicalDeviceInlineUniformBlockPropertiesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceInlineUniformBlockPropertiesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceInlineUniformBlockPropertiesEXTBuilder<'a> {
        PhysicalDeviceInlineUniformBlockPropertiesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn max_inline_uniform_block_size(mut self, max_inline_uniform_block_size: u32) -> Self {
        self.0.max_inline_uniform_block_size = max_inline_uniform_block_size as _;
        self
    }
    #[inline]
    pub fn max_per_stage_descriptor_inline_uniform_blocks(mut self, max_per_stage_descriptor_inline_uniform_blocks: u32) -> Self {
        self.0.max_per_stage_descriptor_inline_uniform_blocks = max_per_stage_descriptor_inline_uniform_blocks as _;
        self
    }
    #[inline]
    pub fn max_per_stage_descriptor_update_after_bind_inline_uniform_blocks(mut self, max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: u32) -> Self {
        self.0.max_per_stage_descriptor_update_after_bind_inline_uniform_blocks = max_per_stage_descriptor_update_after_bind_inline_uniform_blocks as _;
        self
    }
    #[inline]
    pub fn max_descriptor_set_inline_uniform_blocks(mut self, max_descriptor_set_inline_uniform_blocks: u32) -> Self {
        self.0.max_descriptor_set_inline_uniform_blocks = max_descriptor_set_inline_uniform_blocks as _;
        self
    }
    #[inline]
    pub fn max_descriptor_set_update_after_bind_inline_uniform_blocks(mut self, max_descriptor_set_update_after_bind_inline_uniform_blocks: u32) -> Self {
        self.0.max_descriptor_set_update_after_bind_inline_uniform_blocks = max_descriptor_set_update_after_bind_inline_uniform_blocks as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> PhysicalDeviceInlineUniformBlockPropertiesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceInlineUniformBlockPropertiesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceInlineUniformBlockPropertiesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceInlineUniformBlockPropertiesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceInlineUniformBlockPropertiesEXTBuilder<'a> {
    type Target = PhysicalDeviceInlineUniformBlockPropertiesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceInlineUniformBlockPropertiesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkWriteDescriptorSetInlineUniformBlockEXT.html) · Structure"]
#[doc(alias = "VkWriteDescriptorSetInlineUniformBlockEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WriteDescriptorSetInlineUniformBlockEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub data_size: u32,
    pub p_data: *const std::ffi::c_void,
}
impl WriteDescriptorSetInlineUniformBlockEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK_EXT;
}
impl Default for WriteDescriptorSetInlineUniformBlockEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), data_size: Default::default(), p_data: std::ptr::null() }
    }
}
impl std::fmt::Debug for WriteDescriptorSetInlineUniformBlockEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("WriteDescriptorSetInlineUniformBlockEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("data_size", &self.data_size).field("p_data", &self.p_data).finish()
    }
}
impl WriteDescriptorSetInlineUniformBlockEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> WriteDescriptorSetInlineUniformBlockEXTBuilder<'a> {
        WriteDescriptorSetInlineUniformBlockEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkWriteDescriptorSetInlineUniformBlockEXT.html) · Builder of [`WriteDescriptorSetInlineUniformBlockEXT`]"]
#[repr(transparent)]
pub struct WriteDescriptorSetInlineUniformBlockEXTBuilder<'a>(WriteDescriptorSetInlineUniformBlockEXT, std::marker::PhantomData<&'a ()>);
impl<'a> WriteDescriptorSetInlineUniformBlockEXTBuilder<'a> {
    #[inline]
    pub fn new() -> WriteDescriptorSetInlineUniformBlockEXTBuilder<'a> {
        WriteDescriptorSetInlineUniformBlockEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn data_size(mut self, data_size: u32) -> Self {
        self.0.data_size = data_size;
        self
    }
    #[inline]
    pub fn data(mut self, data: *const std::ffi::c_void) -> Self {
        self.0.p_data = data;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> WriteDescriptorSetInlineUniformBlockEXT {
        self.0
    }
}
impl<'a> std::default::Default for WriteDescriptorSetInlineUniformBlockEXTBuilder<'a> {
    fn default() -> WriteDescriptorSetInlineUniformBlockEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for WriteDescriptorSetInlineUniformBlockEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for WriteDescriptorSetInlineUniformBlockEXTBuilder<'a> {
    type Target = WriteDescriptorSetInlineUniformBlockEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for WriteDescriptorSetInlineUniformBlockEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorPoolInlineUniformBlockCreateInfoEXT.html) · Structure"]
#[doc(alias = "VkDescriptorPoolInlineUniformBlockCreateInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DescriptorPoolInlineUniformBlockCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub max_inline_uniform_block_bindings: u32,
}
impl DescriptorPoolInlineUniformBlockCreateInfoEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO_EXT;
}
impl Default for DescriptorPoolInlineUniformBlockCreateInfoEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), max_inline_uniform_block_bindings: Default::default() }
    }
}
impl std::fmt::Debug for DescriptorPoolInlineUniformBlockCreateInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DescriptorPoolInlineUniformBlockCreateInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("max_inline_uniform_block_bindings", &self.max_inline_uniform_block_bindings).finish()
    }
}
impl DescriptorPoolInlineUniformBlockCreateInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> DescriptorPoolInlineUniformBlockCreateInfoEXTBuilder<'a> {
        DescriptorPoolInlineUniformBlockCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorPoolInlineUniformBlockCreateInfoEXT.html) · Builder of [`DescriptorPoolInlineUniformBlockCreateInfoEXT`]"]
#[repr(transparent)]
pub struct DescriptorPoolInlineUniformBlockCreateInfoEXTBuilder<'a>(DescriptorPoolInlineUniformBlockCreateInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> DescriptorPoolInlineUniformBlockCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> DescriptorPoolInlineUniformBlockCreateInfoEXTBuilder<'a> {
        DescriptorPoolInlineUniformBlockCreateInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn max_inline_uniform_block_bindings(mut self, max_inline_uniform_block_bindings: u32) -> Self {
        self.0.max_inline_uniform_block_bindings = max_inline_uniform_block_bindings as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> DescriptorPoolInlineUniformBlockCreateInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for DescriptorPoolInlineUniformBlockCreateInfoEXTBuilder<'a> {
    fn default() -> DescriptorPoolInlineUniformBlockCreateInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DescriptorPoolInlineUniformBlockCreateInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DescriptorPoolInlineUniformBlockCreateInfoEXTBuilder<'a> {
    type Target = DescriptorPoolInlineUniformBlockCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DescriptorPoolInlineUniformBlockCreateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
