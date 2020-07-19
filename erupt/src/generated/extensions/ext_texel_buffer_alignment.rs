#[doc = "<s>Vulkan Manual Page</s> · Constant"]
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
impl Default for PhysicalDeviceTexelBufferAlignmentFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type:
                crate::vk1_0::StructureType::PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            texel_buffer_alignment: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceTexelBufferAlignmentFeaturesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceTexelBufferAlignmentFeaturesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "texel_buffer_alignment",
                &(self.texel_buffer_alignment != 0),
            )
            .finish()
    }
}
impl PhysicalDeviceTexelBufferAlignmentFeaturesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceTexelBufferAlignmentFeaturesEXTBuilder<'a> {
        PhysicalDeviceTexelBufferAlignmentFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT.html) · Builder of [`PhysicalDeviceTexelBufferAlignmentFeaturesEXT`](struct.PhysicalDeviceTexelBufferAlignmentFeaturesEXT.html)"]
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
    #[inline]
    pub fn texel_buffer_alignment(mut self, texel_buffer_alignment: bool) -> Self {
        self.0.texel_buffer_alignment = texel_buffer_alignment as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceTexelBufferAlignmentFeaturesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceTexelBufferAlignmentFeaturesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceTexelBufferAlignmentFeaturesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceTexelBufferAlignmentFeaturesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
impl Default for PhysicalDeviceTexelBufferAlignmentPropertiesEXT {
    fn default() -> Self {
        Self {
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
impl std::fmt::Debug for PhysicalDeviceTexelBufferAlignmentPropertiesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceTexelBufferAlignmentPropertiesEXT")
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
impl PhysicalDeviceTexelBufferAlignmentPropertiesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceTexelBufferAlignmentPropertiesEXTBuilder<'a> {
        PhysicalDeviceTexelBufferAlignmentPropertiesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceTexelBufferAlignmentPropertiesEXT.html) · Builder of [`PhysicalDeviceTexelBufferAlignmentPropertiesEXT`](struct.PhysicalDeviceTexelBufferAlignmentPropertiesEXT.html)"]
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
    #[inline]
    pub fn storage_texel_buffer_offset_alignment_bytes(
        mut self,
        storage_texel_buffer_offset_alignment_bytes: crate::vk1_0::DeviceSize,
    ) -> Self {
        self.0.storage_texel_buffer_offset_alignment_bytes =
            storage_texel_buffer_offset_alignment_bytes as _;
        self
    }
    #[inline]
    pub fn storage_texel_buffer_offset_single_texel_alignment(
        mut self,
        storage_texel_buffer_offset_single_texel_alignment: bool,
    ) -> Self {
        self.0.storage_texel_buffer_offset_single_texel_alignment =
            storage_texel_buffer_offset_single_texel_alignment as _;
        self
    }
    #[inline]
    pub fn uniform_texel_buffer_offset_alignment_bytes(
        mut self,
        uniform_texel_buffer_offset_alignment_bytes: crate::vk1_0::DeviceSize,
    ) -> Self {
        self.0.uniform_texel_buffer_offset_alignment_bytes =
            uniform_texel_buffer_offset_alignment_bytes as _;
        self
    }
    #[inline]
    pub fn uniform_texel_buffer_offset_single_texel_alignment(
        mut self,
        uniform_texel_buffer_offset_single_texel_alignment: bool,
    ) -> Self {
        self.0.uniform_texel_buffer_offset_single_texel_alignment =
            uniform_texel_buffer_offset_single_texel_alignment as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceTexelBufferAlignmentPropertiesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceTexelBufferAlignmentPropertiesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceTexelBufferAlignmentPropertiesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceTexelBufferAlignmentPropertiesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
