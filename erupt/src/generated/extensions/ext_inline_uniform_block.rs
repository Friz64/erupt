# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_EXT_inline_uniform_block.html)\n\n## Extends\n- [`DescriptorType`](../../vk1_0/struct.DescriptorType.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_INLINE_UNIFORM_BLOCK_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_INLINE_UNIFORM_BLOCK_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_inline_uniform_block");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceInlineUniformBlockFeaturesEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceInlineUniformBlockFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub inline_uniform_block: crate::vk1_0::Bool32,
    pub descriptor_binding_inline_uniform_block_update_after_bind: crate::vk1_0::Bool32,
}
impl PhysicalDeviceInlineUniformBlockFeaturesEXT {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceInlineUniformBlockFeaturesEXT,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceInlineUniformBlockFeaturesEXTBuilder<'a> {
        PhysicalDeviceInlineUniformBlockFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceInlineUniformBlockFeaturesEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceInlineUniformBlockFeaturesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("inline_uniform_block", &(self.inline_uniform_block != 0))
            .field(
                "descriptor_binding_inline_uniform_block_update_after_bind",
                &(self.descriptor_binding_inline_uniform_block_update_after_bind != 0),
            )
            .finish()
    }
}
impl Default for PhysicalDeviceInlineUniformBlockFeaturesEXT {
    fn default() -> PhysicalDeviceInlineUniformBlockFeaturesEXT {
        PhysicalDeviceInlineUniformBlockFeaturesEXT {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            inline_uniform_block: Default::default(),
            descriptor_binding_inline_uniform_block_update_after_bind: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceInlineUniformBlockFeaturesEXT::extend`](struct.PhysicalDeviceInlineUniformBlockFeaturesEXT.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceInlineUniformBlockFeaturesEXT {}
impl ExtendableByPhysicalDeviceInlineUniformBlockFeaturesEXT
    for crate::vk1_1::PhysicalDeviceFeatures2
{
}
impl ExtendableByPhysicalDeviceInlineUniformBlockFeaturesEXT for crate::vk1_0::DeviceCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceInlineUniformBlockFeaturesEXT`](struct.PhysicalDeviceInlineUniformBlockFeaturesEXT.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceInlineUniformBlockFeaturesEXTBuilder<'a>(
    PhysicalDeviceInlineUniformBlockFeaturesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceInlineUniformBlockFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceInlineUniformBlockFeaturesEXTBuilder<'a> {
        PhysicalDeviceInlineUniformBlockFeaturesEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn inline_uniform_block(mut self, inline_uniform_block: bool) -> Self {
        self.0.inline_uniform_block = inline_uniform_block as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn descriptor_binding_inline_uniform_block_update_after_bind(
        mut self,
        descriptor_binding_inline_uniform_block_update_after_bind: bool,
    ) -> Self {
        self.0
            .descriptor_binding_inline_uniform_block_update_after_bind =
            descriptor_binding_inline_uniform_block_update_after_bind as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceInlineUniformBlockFeaturesEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceInlineUniformBlockFeaturesEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceInlineUniformBlockPropertiesEXT,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceInlineUniformBlockPropertiesEXTBuilder<'a> {
        PhysicalDeviceInlineUniformBlockPropertiesEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceInlineUniformBlockPropertiesEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceInlineUniformBlockPropertiesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "max_inline_uniform_block_size",
                &self.max_inline_uniform_block_size,
            )
            .field(
                "max_per_stage_descriptor_inline_uniform_blocks",
                &self.max_per_stage_descriptor_inline_uniform_blocks,
            )
            .field(
                "max_per_stage_descriptor_update_after_bind_inline_uniform_blocks",
                &self.max_per_stage_descriptor_update_after_bind_inline_uniform_blocks,
            )
            .field(
                "max_descriptor_set_inline_uniform_blocks",
                &self.max_descriptor_set_inline_uniform_blocks,
            )
            .field(
                "max_descriptor_set_update_after_bind_inline_uniform_blocks",
                &self.max_descriptor_set_update_after_bind_inline_uniform_blocks,
            )
            .finish()
    }
}
impl Default for PhysicalDeviceInlineUniformBlockPropertiesEXT {
    fn default() -> PhysicalDeviceInlineUniformBlockPropertiesEXT {
        PhysicalDeviceInlineUniformBlockPropertiesEXT {
            s_type:
                crate::vk1_0::StructureType::PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            max_inline_uniform_block_size: Default::default(),
            max_per_stage_descriptor_inline_uniform_blocks: Default::default(),
            max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: Default::default(),
            max_descriptor_set_inline_uniform_blocks: Default::default(),
            max_descriptor_set_update_after_bind_inline_uniform_blocks: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceInlineUniformBlockPropertiesEXT::extend`](struct.PhysicalDeviceInlineUniformBlockPropertiesEXT.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceInlineUniformBlockPropertiesEXT {}
impl ExtendableByPhysicalDeviceInlineUniformBlockPropertiesEXT
    for crate::vk1_1::PhysicalDeviceProperties2
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceInlineUniformBlockPropertiesEXT`](struct.PhysicalDeviceInlineUniformBlockPropertiesEXT.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceInlineUniformBlockPropertiesEXTBuilder<'a>(
    PhysicalDeviceInlineUniformBlockPropertiesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceInlineUniformBlockPropertiesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceInlineUniformBlockPropertiesEXTBuilder<'a> {
        PhysicalDeviceInlineUniformBlockPropertiesEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_inline_uniform_block_size(mut self, max_inline_uniform_block_size: u32) -> Self {
        self.0.max_inline_uniform_block_size = max_inline_uniform_block_size;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_per_stage_descriptor_inline_uniform_blocks(
        mut self,
        max_per_stage_descriptor_inline_uniform_blocks: u32,
    ) -> Self {
        self.0.max_per_stage_descriptor_inline_uniform_blocks =
            max_per_stage_descriptor_inline_uniform_blocks;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_per_stage_descriptor_update_after_bind_inline_uniform_blocks(
        mut self,
        max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: u32,
    ) -> Self {
        self.0
            .max_per_stage_descriptor_update_after_bind_inline_uniform_blocks =
            max_per_stage_descriptor_update_after_bind_inline_uniform_blocks;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_descriptor_set_inline_uniform_blocks(
        mut self,
        max_descriptor_set_inline_uniform_blocks: u32,
    ) -> Self {
        self.0.max_descriptor_set_inline_uniform_blocks = max_descriptor_set_inline_uniform_blocks;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_descriptor_set_update_after_bind_inline_uniform_blocks(
        mut self,
        max_descriptor_set_update_after_bind_inline_uniform_blocks: u32,
    ) -> Self {
        self.0
            .max_descriptor_set_update_after_bind_inline_uniform_blocks =
            max_descriptor_set_update_after_bind_inline_uniform_blocks;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceInlineUniformBlockPropertiesEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceInlineUniformBlockPropertiesEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WriteDescriptorSetInlineUniformBlockEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub data_size: u32,
    pub p_data: *const std::ffi::c_void,
}
impl WriteDescriptorSetInlineUniformBlockEXT {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByWriteDescriptorSetInlineUniformBlockEXT,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> WriteDescriptorSetInlineUniformBlockEXTBuilder<'a> {
        WriteDescriptorSetInlineUniformBlockEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for WriteDescriptorSetInlineUniformBlockEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("WriteDescriptorSetInlineUniformBlockEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("data_size", &self.data_size)
            .field("p_data", &self.p_data)
            .finish()
    }
}
impl Default for WriteDescriptorSetInlineUniformBlockEXT {
    fn default() -> WriteDescriptorSetInlineUniformBlockEXT {
        WriteDescriptorSetInlineUniformBlockEXT {
            s_type: crate::vk1_0::StructureType::WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK_EXT,
            p_next: std::ptr::null(),
            data_size: Default::default(),
            p_data: std::ptr::null(),
        }
    }
}
#[doc = "Used by [`WriteDescriptorSetInlineUniformBlockEXT::extend`](struct.WriteDescriptorSetInlineUniformBlockEXT.html#method.extend)"]
pub trait ExtendableByWriteDescriptorSetInlineUniformBlockEXT {}
impl ExtendableByWriteDescriptorSetInlineUniformBlockEXT for crate::vk1_0::WriteDescriptorSet {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`WriteDescriptorSetInlineUniformBlockEXT`](struct.WriteDescriptorSetInlineUniformBlockEXT.html)"]
#[repr(transparent)]
pub struct WriteDescriptorSetInlineUniformBlockEXTBuilder<'a>(
    WriteDescriptorSetInlineUniformBlockEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> WriteDescriptorSetInlineUniformBlockEXTBuilder<'a> {
    #[inline]
    pub fn new() -> WriteDescriptorSetInlineUniformBlockEXTBuilder<'a> {
        WriteDescriptorSetInlineUniformBlockEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn data(mut self, data: &'a [std::ffi::c_void]) -> Self {
        self.0.data_size = data.len() as _;
        self.0.p_data = data.as_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> WriteDescriptorSetInlineUniformBlockEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for WriteDescriptorSetInlineUniformBlockEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DescriptorPoolInlineUniformBlockCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub max_inline_uniform_block_bindings: u32,
}
impl DescriptorPoolInlineUniformBlockCreateInfoEXT {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByDescriptorPoolInlineUniformBlockCreateInfoEXT,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> DescriptorPoolInlineUniformBlockCreateInfoEXTBuilder<'a> {
        DescriptorPoolInlineUniformBlockCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for DescriptorPoolInlineUniformBlockCreateInfoEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("DescriptorPoolInlineUniformBlockCreateInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "max_inline_uniform_block_bindings",
                &self.max_inline_uniform_block_bindings,
            )
            .finish()
    }
}
impl Default for DescriptorPoolInlineUniformBlockCreateInfoEXT {
    fn default() -> DescriptorPoolInlineUniformBlockCreateInfoEXT {
        DescriptorPoolInlineUniformBlockCreateInfoEXT {
            s_type:
                crate::vk1_0::StructureType::DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            max_inline_uniform_block_bindings: Default::default(),
        }
    }
}
#[doc = "Used by [`DescriptorPoolInlineUniformBlockCreateInfoEXT::extend`](struct.DescriptorPoolInlineUniformBlockCreateInfoEXT.html#method.extend)"]
pub trait ExtendableByDescriptorPoolInlineUniformBlockCreateInfoEXT {}
impl ExtendableByDescriptorPoolInlineUniformBlockCreateInfoEXT
    for crate::vk1_0::DescriptorPoolCreateInfo
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`DescriptorPoolInlineUniformBlockCreateInfoEXT`](struct.DescriptorPoolInlineUniformBlockCreateInfoEXT.html)"]
#[repr(transparent)]
pub struct DescriptorPoolInlineUniformBlockCreateInfoEXTBuilder<'a>(
    DescriptorPoolInlineUniformBlockCreateInfoEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> DescriptorPoolInlineUniformBlockCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> DescriptorPoolInlineUniformBlockCreateInfoEXTBuilder<'a> {
        DescriptorPoolInlineUniformBlockCreateInfoEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_inline_uniform_block_bindings(
        mut self,
        max_inline_uniform_block_bindings: u32,
    ) -> Self {
        self.0.max_inline_uniform_block_bindings = max_inline_uniform_block_bindings;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> DescriptorPoolInlineUniformBlockCreateInfoEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for DescriptorPoolInlineUniformBlockCreateInfoEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
