#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_EXTERNAL_MEMORY_WIN32_SPEC_VERSION")]
pub const NV_EXTERNAL_MEMORY_WIN32_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME")]
pub const NV_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_NV_external_memory_win32");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_MEMORY_WIN32_HANDLE_NV: *const std::os::raw::c_char = crate::cstr!("vkGetMemoryWin32HandleNV");
#[doc = "Provided by [`crate::extensions::nv_external_memory_win32`]"]
impl crate::vk1_0::StructureType {
    pub const IMPORT_MEMORY_WIN32_HANDLE_INFO_NV: Self = Self(1000057000);
    pub const EXPORT_MEMORY_WIN32_HANDLE_INFO_NV: Self = Self(1000057001);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryWin32HandleNV.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetMemoryWin32HandleNV = unsafe extern "system" fn(device: crate::vk1_0::Device, memory: crate::vk1_0::DeviceMemory, handle_type: crate::extensions::nv_external_memory_capabilities::ExternalMemoryHandleTypeFlagsNV, p_handle: *mut *mut std::ffi::c_void) -> crate::vk1_0::Result;
impl<'a> crate::ExtendableFromConst<'a, ImportMemoryWin32HandleInfoNV> for crate::vk1_0::MemoryAllocateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, ImportMemoryWin32HandleInfoNVBuilder<'_>> for crate::vk1_0::MemoryAllocateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, ExportMemoryWin32HandleInfoNV> for crate::vk1_0::MemoryAllocateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, ExportMemoryWin32HandleInfoNVBuilder<'_>> for crate::vk1_0::MemoryAllocateInfoBuilder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImportMemoryWin32HandleInfoNV.html) · Structure"]
#[doc(alias = "VkImportMemoryWin32HandleInfoNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImportMemoryWin32HandleInfoNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub handle_type: crate::extensions::nv_external_memory_capabilities::ExternalMemoryHandleTypeFlagsNV,
    pub handle: *mut std::ffi::c_void,
}
impl ImportMemoryWin32HandleInfoNV {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::IMPORT_MEMORY_WIN32_HANDLE_INFO_NV;
}
impl Default for ImportMemoryWin32HandleInfoNV {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), handle_type: Default::default(), handle: std::ptr::null_mut() }
    }
}
impl std::fmt::Debug for ImportMemoryWin32HandleInfoNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ImportMemoryWin32HandleInfoNV").field("s_type", &self.s_type).field("p_next", &self.p_next).field("handle_type", &self.handle_type).field("handle", &self.handle).finish()
    }
}
impl ImportMemoryWin32HandleInfoNV {
    #[inline]
    pub fn into_builder<'a>(self) -> ImportMemoryWin32HandleInfoNVBuilder<'a> {
        ImportMemoryWin32HandleInfoNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImportMemoryWin32HandleInfoNV.html) · Builder of [`ImportMemoryWin32HandleInfoNV`]"]
#[repr(transparent)]
pub struct ImportMemoryWin32HandleInfoNVBuilder<'a>(ImportMemoryWin32HandleInfoNV, std::marker::PhantomData<&'a ()>);
impl<'a> ImportMemoryWin32HandleInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> ImportMemoryWin32HandleInfoNVBuilder<'a> {
        ImportMemoryWin32HandleInfoNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn handle_type(mut self, handle_type: crate::extensions::nv_external_memory_capabilities::ExternalMemoryHandleTypeFlagsNV) -> Self {
        self.0.handle_type = handle_type as _;
        self
    }
    #[inline]
    pub fn handle(mut self, handle: *mut std::ffi::c_void) -> Self {
        self.0.handle = handle;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ImportMemoryWin32HandleInfoNV {
        self.0
    }
}
impl<'a> std::default::Default for ImportMemoryWin32HandleInfoNVBuilder<'a> {
    fn default() -> ImportMemoryWin32HandleInfoNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ImportMemoryWin32HandleInfoNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ImportMemoryWin32HandleInfoNVBuilder<'a> {
    type Target = ImportMemoryWin32HandleInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ImportMemoryWin32HandleInfoNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExportMemoryWin32HandleInfoNV.html) · Structure"]
#[doc(alias = "VkExportMemoryWin32HandleInfoNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExportMemoryWin32HandleInfoNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub p_attributes: *const std::ffi::c_void,
    pub dw_access: u32,
}
impl ExportMemoryWin32HandleInfoNV {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::EXPORT_MEMORY_WIN32_HANDLE_INFO_NV;
}
impl Default for ExportMemoryWin32HandleInfoNV {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), p_attributes: std::ptr::null(), dw_access: Default::default() }
    }
}
impl std::fmt::Debug for ExportMemoryWin32HandleInfoNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ExportMemoryWin32HandleInfoNV").field("s_type", &self.s_type).field("p_next", &self.p_next).field("p_attributes", &self.p_attributes).field("dw_access", &self.dw_access).finish()
    }
}
impl ExportMemoryWin32HandleInfoNV {
    #[inline]
    pub fn into_builder<'a>(self) -> ExportMemoryWin32HandleInfoNVBuilder<'a> {
        ExportMemoryWin32HandleInfoNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExportMemoryWin32HandleInfoNV.html) · Builder of [`ExportMemoryWin32HandleInfoNV`]"]
#[repr(transparent)]
pub struct ExportMemoryWin32HandleInfoNVBuilder<'a>(ExportMemoryWin32HandleInfoNV, std::marker::PhantomData<&'a ()>);
impl<'a> ExportMemoryWin32HandleInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> ExportMemoryWin32HandleInfoNVBuilder<'a> {
        ExportMemoryWin32HandleInfoNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn attributes(mut self, attributes: *const std::ffi::c_void) -> Self {
        self.0.p_attributes = attributes;
        self
    }
    #[inline]
    pub fn dw_access(mut self, dw_access: u32) -> Self {
        self.0.dw_access = dw_access as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ExportMemoryWin32HandleInfoNV {
        self.0
    }
}
impl<'a> std::default::Default for ExportMemoryWin32HandleInfoNVBuilder<'a> {
    fn default() -> ExportMemoryWin32HandleInfoNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ExportMemoryWin32HandleInfoNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ExportMemoryWin32HandleInfoNVBuilder<'a> {
    type Target = ExportMemoryWin32HandleInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ExportMemoryWin32HandleInfoNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::nv_external_memory_win32`]"]
impl crate::DeviceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryWin32HandleNV.html) · Function"]
    #[doc(alias = "vkGetMemoryWin32HandleNV")]
    pub unsafe fn get_memory_win32_handle_nv(&self, memory: crate::vk1_0::DeviceMemory, handle_type: crate::extensions::nv_external_memory_capabilities::ExternalMemoryHandleTypeFlagsNV, handle: *mut *mut std::ffi::c_void) -> crate::utils::VulkanResult<()> {
        let _function = self.get_memory_win32_handle_nv.expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(self.handle, memory as _, handle_type as _, handle);
        crate::utils::VulkanResult::new(_return, ())
    }
}
