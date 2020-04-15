# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_EXT_texel_buffer_alignment.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_TEXEL_BUFFER_ALIGNMENT_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_TEXEL_BUFFER_ALIGNMENT_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_texel_buffer_alignment");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceTexelBufferAlignmentFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub texel_buffer_alignment: crate::vk1_0::Bool32,
}
impl PhysicalDeviceTexelBufferAlignmentFeaturesEXT {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceTexelBufferAlignmentFeaturesEXT,
    {
        unsafe {
            crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
        }
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceTexelBufferAlignmentFeaturesEXTBuilder<'a> {
        PhysicalDeviceTexelBufferAlignmentFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceTexelBufferAlignmentFeaturesEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceTexelBufferAlignmentFeaturesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "texel_buffer_alignment",
                &(self.texel_buffer_alignment != 0),
            )
            .finish()
    }
}
impl Default for PhysicalDeviceTexelBufferAlignmentFeaturesEXT {
    fn default() -> PhysicalDeviceTexelBufferAlignmentFeaturesEXT {
        PhysicalDeviceTexelBufferAlignmentFeaturesEXT {
            s_type:
                crate::vk1_0::StructureType::PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            texel_buffer_alignment: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceTexelBufferAlignmentFeaturesEXT::extend`](struct.PhysicalDeviceTexelBufferAlignmentFeaturesEXT.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceTexelBufferAlignmentFeaturesEXT {}
impl ExtendableByPhysicalDeviceTexelBufferAlignmentFeaturesEXT
    for crate::vk1_1::PhysicalDeviceFeatures2
{
}
impl ExtendableByPhysicalDeviceTexelBufferAlignmentFeaturesEXT for crate::vk1_0::DeviceCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceTexelBufferAlignmentFeaturesEXT`](struct.PhysicalDeviceTexelBufferAlignmentFeaturesEXT.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceTexelBufferAlignmentFeaturesEXTBuilder<'a>(
    PhysicalDeviceTexelBufferAlignmentFeaturesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceTexelBufferAlignmentFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceTexelBufferAlignmentFeaturesEXTBuilder<'a> {
        PhysicalDeviceTexelBufferAlignmentFeaturesEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn texel_buffer_alignment(mut self, texel_buffer_alignment: bool) -> Self {
        self.0.texel_buffer_alignment = texel_buffer_alignment as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceTexelBufferAlignmentFeaturesEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceTexelBufferAlignmentFeaturesEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceTexelBufferAlignmentFeaturesEXTBuilder<'a> {
    type Target = PhysicalDeviceTexelBufferAlignmentFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceTexelBufferAlignmentFeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceTexelBufferAlignmentPropertiesEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceTexelBufferAlignmentPropertiesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub storage_texel_buffer_offset_alignment_bytes: crate::vk1_0::DeviceSize,
    pub storage_texel_buffer_offset_single_texel_alignment: crate::vk1_0::Bool32,
    pub uniform_texel_buffer_offset_alignment_bytes: crate::vk1_0::DeviceSize,
    pub uniform_texel_buffer_offset_single_texel_alignment: crate::vk1_0::Bool32,
}
impl PhysicalDeviceTexelBufferAlignmentPropertiesEXT {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceTexelBufferAlignmentPropertiesEXT,
    {
        unsafe {
            crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
        }
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceTexelBufferAlignmentPropertiesEXTBuilder<'a> {
        PhysicalDeviceTexelBufferAlignmentPropertiesEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceTexelBufferAlignmentPropertiesEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceTexelBufferAlignmentPropertiesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "storage_texel_buffer_offset_alignment_bytes",
                &self.storage_texel_buffer_offset_alignment_bytes,
            )
            .field(
                "storage_texel_buffer_offset_single_texel_alignment",
                &(self.storage_texel_buffer_offset_single_texel_alignment != 0),
            )
            .field(
                "uniform_texel_buffer_offset_alignment_bytes",
                &self.uniform_texel_buffer_offset_alignment_bytes,
            )
            .field(
                "uniform_texel_buffer_offset_single_texel_alignment",
                &(self.uniform_texel_buffer_offset_single_texel_alignment != 0),
            )
            .finish()
    }
}
impl Default for PhysicalDeviceTexelBufferAlignmentPropertiesEXT {
    fn default() -> PhysicalDeviceTexelBufferAlignmentPropertiesEXT {
        PhysicalDeviceTexelBufferAlignmentPropertiesEXT {
            s_type:
                crate::vk1_0::StructureType::PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            storage_texel_buffer_offset_alignment_bytes: Default::default(),
            storage_texel_buffer_offset_single_texel_alignment: Default::default(),
            uniform_texel_buffer_offset_alignment_bytes: Default::default(),
            uniform_texel_buffer_offset_single_texel_alignment: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceTexelBufferAlignmentPropertiesEXT::extend`](struct.PhysicalDeviceTexelBufferAlignmentPropertiesEXT.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceTexelBufferAlignmentPropertiesEXT {}
impl ExtendableByPhysicalDeviceTexelBufferAlignmentPropertiesEXT
    for crate::vk1_1::PhysicalDeviceProperties2
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceTexelBufferAlignmentPropertiesEXT`](struct.PhysicalDeviceTexelBufferAlignmentPropertiesEXT.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceTexelBufferAlignmentPropertiesEXTBuilder<'a>(
    PhysicalDeviceTexelBufferAlignmentPropertiesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceTexelBufferAlignmentPropertiesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceTexelBufferAlignmentPropertiesEXTBuilder<'a> {
        PhysicalDeviceTexelBufferAlignmentPropertiesEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn storage_texel_buffer_offset_alignment_bytes(
        mut self,
        storage_texel_buffer_offset_alignment_bytes: crate::vk1_0::DeviceSize,
    ) -> Self {
        self.0.storage_texel_buffer_offset_alignment_bytes =
            storage_texel_buffer_offset_alignment_bytes;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn storage_texel_buffer_offset_single_texel_alignment(
        mut self,
        storage_texel_buffer_offset_single_texel_alignment: bool,
    ) -> Self {
        self.0.storage_texel_buffer_offset_single_texel_alignment =
            storage_texel_buffer_offset_single_texel_alignment as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn uniform_texel_buffer_offset_alignment_bytes(
        mut self,
        uniform_texel_buffer_offset_alignment_bytes: crate::vk1_0::DeviceSize,
    ) -> Self {
        self.0.uniform_texel_buffer_offset_alignment_bytes =
            uniform_texel_buffer_offset_alignment_bytes;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn uniform_texel_buffer_offset_single_texel_alignment(
        mut self,
        uniform_texel_buffer_offset_single_texel_alignment: bool,
    ) -> Self {
        self.0.uniform_texel_buffer_offset_single_texel_alignment =
            uniform_texel_buffer_offset_single_texel_alignment as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceTexelBufferAlignmentPropertiesEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceTexelBufferAlignmentPropertiesEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceTexelBufferAlignmentPropertiesEXTBuilder<'a> {
    type Target = PhysicalDeviceTexelBufferAlignmentPropertiesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceTexelBufferAlignmentPropertiesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
