# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_NV_external_memory.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NV_EXTERNAL_MEMORY_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NV_EXTERNAL_MEMORY_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_NV_external_memory");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalMemoryImageCreateInfoNV.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExternalMemoryImageCreateInfoNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub handle_types:
        crate::extensions::nv_external_memory_capabilities::ExternalMemoryHandleTypeFlagsNV,
}
impl ExternalMemoryImageCreateInfoNV {
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
    pub fn builder<'a>(self) -> ExternalMemoryImageCreateInfoNVBuilder<'a> {
        ExternalMemoryImageCreateInfoNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ExternalMemoryImageCreateInfoNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ExternalMemoryImageCreateInfoNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("handle_types", &self.handle_types)
            .finish()
    }
}
impl Default for ExternalMemoryImageCreateInfoNV {
    fn default() -> ExternalMemoryImageCreateInfoNV {
        ExternalMemoryImageCreateInfoNV {
            s_type: crate::vk1_0::StructureType::EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV,
            p_next: std::ptr::null(),
            handle_types: Default::default(),
        }
    }
}
impl crate::ExtendableBy<ExternalMemoryImageCreateInfoNV> for crate::vk1_0::ImageCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`ExternalMemoryImageCreateInfoNV`](struct.ExternalMemoryImageCreateInfoNV.html)"]
#[repr(transparent)]
pub struct ExternalMemoryImageCreateInfoNVBuilder<'a>(
    ExternalMemoryImageCreateInfoNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> ExternalMemoryImageCreateInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> ExternalMemoryImageCreateInfoNVBuilder<'a> {
        ExternalMemoryImageCreateInfoNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn handle_types(
        mut self,
        handle_types : crate :: extensions :: nv_external_memory_capabilities :: ExternalMemoryHandleTypeFlagsNV,
    ) -> Self {
        self.0.handle_types = handle_types;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> ExternalMemoryImageCreateInfoNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for ExternalMemoryImageCreateInfoNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExportMemoryAllocateInfoNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub handle_types:
        crate::extensions::nv_external_memory_capabilities::ExternalMemoryHandleTypeFlagsNV,
}
impl ExportMemoryAllocateInfoNV {
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
    pub fn builder<'a>(self) -> ExportMemoryAllocateInfoNVBuilder<'a> {
        ExportMemoryAllocateInfoNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ExportMemoryAllocateInfoNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ExportMemoryAllocateInfoNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("handle_types", &self.handle_types)
            .finish()
    }
}
impl Default for ExportMemoryAllocateInfoNV {
    fn default() -> ExportMemoryAllocateInfoNV {
        ExportMemoryAllocateInfoNV {
            s_type: crate::vk1_0::StructureType::EXPORT_MEMORY_ALLOCATE_INFO_NV,
            p_next: std::ptr::null(),
            handle_types: Default::default(),
        }
    }
}
impl crate::ExtendableBy<ExportMemoryAllocateInfoNV> for crate::vk1_0::MemoryAllocateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`ExportMemoryAllocateInfoNV`](struct.ExportMemoryAllocateInfoNV.html)"]
#[repr(transparent)]
pub struct ExportMemoryAllocateInfoNVBuilder<'a>(
    ExportMemoryAllocateInfoNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> ExportMemoryAllocateInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> ExportMemoryAllocateInfoNVBuilder<'a> {
        ExportMemoryAllocateInfoNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn handle_types(
        mut self,
        handle_types : crate :: extensions :: nv_external_memory_capabilities :: ExternalMemoryHandleTypeFlagsNV,
    ) -> Self {
        self.0.handle_types = handle_types;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> ExportMemoryAllocateInfoNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for ExportMemoryAllocateInfoNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
