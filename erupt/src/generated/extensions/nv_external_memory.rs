#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_EXTERNAL_MEMORY_SPEC_VERSION")]
pub const NV_EXTERNAL_MEMORY_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_EXTERNAL_MEMORY_EXTENSION_NAME")]
pub const NV_EXTERNAL_MEMORY_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_NV_external_memory");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalMemoryImageCreateInfoNV.html) · Structure"]
#[doc(alias = "VkExternalMemoryImageCreateInfoNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExternalMemoryImageCreateInfoNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub handle_types: crate::extensions::nv_external_memory_capabilities::ExternalMemoryHandleTypeFlagsNV,
}
impl Default for ExternalMemoryImageCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV,
            p_next: std::ptr::null(),
            handle_types: Default::default(),
        }
    }
}
impl std::fmt::Debug for ExternalMemoryImageCreateInfoNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ExternalMemoryImageCreateInfoNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("handle_types", &self.handle_types)
            .finish()
    }
}
impl ExternalMemoryImageCreateInfoNV {
    #[inline]
    pub fn into_builder<'a>(self) -> ExternalMemoryImageCreateInfoNVBuilder<'a> {
        ExternalMemoryImageCreateInfoNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalMemoryImageCreateInfoNV.html) · Builder of [`ExternalMemoryImageCreateInfoNV`]"]
#[repr(transparent)]
pub struct ExternalMemoryImageCreateInfoNVBuilder<'a>(ExternalMemoryImageCreateInfoNV, std::marker::PhantomData<&'a ()>);
impl<'a> ExternalMemoryImageCreateInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> ExternalMemoryImageCreateInfoNVBuilder<'a> {
        ExternalMemoryImageCreateInfoNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn handle_types(mut self, handle_types: crate::extensions::nv_external_memory_capabilities::ExternalMemoryHandleTypeFlagsNV) -> Self {
        self.0.handle_types = handle_types as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ExternalMemoryImageCreateInfoNV {
        self.0
    }
}
impl<'a> std::default::Default for ExternalMemoryImageCreateInfoNVBuilder<'a> {
    fn default() -> ExternalMemoryImageCreateInfoNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ExternalMemoryImageCreateInfoNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ExternalMemoryImageCreateInfoNVBuilder<'a> {
    type Target = ExternalMemoryImageCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ExternalMemoryImageCreateInfoNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExportMemoryAllocateInfoNV.html) · Structure"]
#[doc(alias = "VkExportMemoryAllocateInfoNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExportMemoryAllocateInfoNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub handle_types: crate::extensions::nv_external_memory_capabilities::ExternalMemoryHandleTypeFlagsNV,
}
impl Default for ExportMemoryAllocateInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::EXPORT_MEMORY_ALLOCATE_INFO_NV,
            p_next: std::ptr::null(),
            handle_types: Default::default(),
        }
    }
}
impl std::fmt::Debug for ExportMemoryAllocateInfoNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ExportMemoryAllocateInfoNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("handle_types", &self.handle_types)
            .finish()
    }
}
impl ExportMemoryAllocateInfoNV {
    #[inline]
    pub fn into_builder<'a>(self) -> ExportMemoryAllocateInfoNVBuilder<'a> {
        ExportMemoryAllocateInfoNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExportMemoryAllocateInfoNV.html) · Builder of [`ExportMemoryAllocateInfoNV`]"]
#[repr(transparent)]
pub struct ExportMemoryAllocateInfoNVBuilder<'a>(ExportMemoryAllocateInfoNV, std::marker::PhantomData<&'a ()>);
impl<'a> ExportMemoryAllocateInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> ExportMemoryAllocateInfoNVBuilder<'a> {
        ExportMemoryAllocateInfoNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn handle_types(mut self, handle_types: crate::extensions::nv_external_memory_capabilities::ExternalMemoryHandleTypeFlagsNV) -> Self {
        self.0.handle_types = handle_types as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ExportMemoryAllocateInfoNV {
        self.0
    }
}
impl<'a> std::default::Default for ExportMemoryAllocateInfoNVBuilder<'a> {
    fn default() -> ExportMemoryAllocateInfoNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ExportMemoryAllocateInfoNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ExportMemoryAllocateInfoNVBuilder<'a> {
    type Target = ExportMemoryAllocateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ExportMemoryAllocateInfoNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
