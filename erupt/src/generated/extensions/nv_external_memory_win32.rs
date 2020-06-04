# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_NV_external_memory_win32.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NV_EXTERNAL_MEMORY_WIN32_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NV_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_NV_external_memory_win32");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryWin32HandleNV.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetMemoryWin32HandleNV = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    memory: crate::vk1_0::DeviceMemory,
    handle_type: crate::extensions::nv_external_memory_capabilities::ExternalMemoryHandleTypeFlagsNV,
    p_handle: *mut *mut std::ffi::c_void,
) -> crate::vk1_0::Result;
#[doc = "Provides Device Commands for [`NvExternalMemoryWin32DeviceLoaderExt`](trait.NvExternalMemoryWin32DeviceLoaderExt.html)"]
pub struct NvExternalMemoryWin32DeviceCommands {
    pub get_memory_win32_handle_nv: Option<PFN_vkGetMemoryWin32HandleNV>,
}
impl NvExternalMemoryWin32DeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<NvExternalMemoryWin32DeviceCommands> {
        unsafe {
            let mut success = false;
            let table = NvExternalMemoryWin32DeviceCommands {
                get_memory_win32_handle_nv: std::mem::transmute({
                    let symbol = loader.symbol("vkGetMemoryWin32HandleNV");
                    success |= symbol.is_some();
                    symbol
                }),
            };
            if success {
                Some(table)
            } else {
                None
            }
        }
    }
}
fn device_commands(loader: &crate::DeviceLoader) -> &NvExternalMemoryWin32DeviceCommands {
    loader
        .nv_external_memory_win32
        .as_ref()
        .expect("`nv_external_memory_win32` not loaded")
}
#[doc = "Provides high level command wrappers for [`NvExternalMemoryWin32DeviceCommands`](struct.NvExternalMemoryWin32DeviceCommands.html)"]
pub trait NvExternalMemoryWin32DeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryWin32HandleNV.html) · Device Command"]
    unsafe fn get_memory_win32_handle_nv(
        &self,
        memory: crate::vk1_0::DeviceMemory,
        handle_type : crate :: extensions :: nv_external_memory_capabilities :: ExternalMemoryHandleTypeFlagsNV,
        handle: *mut *mut std::ffi::c_void,
    ) -> crate::utils::VulkanResult<()>;
}
impl NvExternalMemoryWin32DeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryWin32HandleNV.html) · Device Command"]
    unsafe fn get_memory_win32_handle_nv(
        &self,
        memory: crate::vk1_0::DeviceMemory,
        handle_type : crate :: extensions :: nv_external_memory_capabilities :: ExternalMemoryHandleTypeFlagsNV,
        handle: *mut *mut std::ffi::c_void,
    ) -> crate::utils::VulkanResult<()> {
        let function = device_commands(self)
            .get_memory_win32_handle_nv
            .as_ref()
            .expect("`get_memory_win32_handle_nv` not available");
        let _val = function(self.handle, memory, handle_type, handle);
        crate::utils::VulkanResult::new(_val, ())
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImportMemoryWin32HandleInfoNV.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImportMemoryWin32HandleInfoNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub handle_type:
        crate::extensions::nv_external_memory_capabilities::ExternalMemoryHandleTypeFlagsNV,
    pub handle: *mut std::ffi::c_void,
}
impl ImportMemoryWin32HandleInfoNV {
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
    pub fn builder<'a>(self) -> ImportMemoryWin32HandleInfoNVBuilder<'a> {
        ImportMemoryWin32HandleInfoNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ImportMemoryWin32HandleInfoNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ImportMemoryWin32HandleInfoNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("handle_type", &self.handle_type)
            .field("handle", &self.handle)
            .finish()
    }
}
impl Default for ImportMemoryWin32HandleInfoNV {
    fn default() -> ImportMemoryWin32HandleInfoNV {
        ImportMemoryWin32HandleInfoNV {
            s_type: crate::vk1_0::StructureType::IMPORT_MEMORY_WIN32_HANDLE_INFO_NV,
            p_next: std::ptr::null(),
            handle_type: Default::default(),
            handle: std::ptr::null_mut(),
        }
    }
}
impl crate::ExtendableBy<ImportMemoryWin32HandleInfoNV> for crate::vk1_0::MemoryAllocateInfo {}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImportMemoryWin32HandleInfoNV.html) · Builder of [`ImportMemoryWin32HandleInfoNV`](struct.ImportMemoryWin32HandleInfoNV.html)"]
#[repr(transparent)]
pub struct ImportMemoryWin32HandleInfoNVBuilder<'a>(
    ImportMemoryWin32HandleInfoNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> ImportMemoryWin32HandleInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> ImportMemoryWin32HandleInfoNVBuilder<'a> {
        ImportMemoryWin32HandleInfoNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn handle_type(
        mut self,
        handle_type : crate :: extensions :: nv_external_memory_capabilities :: ExternalMemoryHandleTypeFlagsNV,
    ) -> Self {
        self.0.handle_type = handle_type;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn handle(mut self, handle: &'a mut std::ffi::c_void) -> Self {
        self.0.handle = handle;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> ImportMemoryWin32HandleInfoNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for ImportMemoryWin32HandleInfoNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExportMemoryWin32HandleInfoNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub p_attributes: *const crate::SECURITY_ATTRIBUTES,
    pub dw_access: u32,
}
impl ExportMemoryWin32HandleInfoNV {
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
    pub fn builder<'a>(self) -> ExportMemoryWin32HandleInfoNVBuilder<'a> {
        ExportMemoryWin32HandleInfoNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ExportMemoryWin32HandleInfoNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ExportMemoryWin32HandleInfoNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("p_attributes", &self.p_attributes)
            .field("dw_access", &self.dw_access)
            .finish()
    }
}
impl Default for ExportMemoryWin32HandleInfoNV {
    fn default() -> ExportMemoryWin32HandleInfoNV {
        ExportMemoryWin32HandleInfoNV {
            s_type: crate::vk1_0::StructureType::EXPORT_MEMORY_WIN32_HANDLE_INFO_NV,
            p_next: std::ptr::null(),
            p_attributes: std::ptr::null(),
            dw_access: Default::default(),
        }
    }
}
impl crate::ExtendableBy<ExportMemoryWin32HandleInfoNV> for crate::vk1_0::MemoryAllocateInfo {}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExportMemoryWin32HandleInfoNV.html) · Builder of [`ExportMemoryWin32HandleInfoNV`](struct.ExportMemoryWin32HandleInfoNV.html)"]
#[repr(transparent)]
pub struct ExportMemoryWin32HandleInfoNVBuilder<'a>(
    ExportMemoryWin32HandleInfoNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> ExportMemoryWin32HandleInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> ExportMemoryWin32HandleInfoNVBuilder<'a> {
        ExportMemoryWin32HandleInfoNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn attributes(mut self, attributes: &'a crate::SECURITY_ATTRIBUTES) -> Self {
        self.0.p_attributes = attributes;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn dw_access(mut self, dw_access: u32) -> Self {
        self.0.dw_access = dw_access;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> ExportMemoryWin32HandleInfoNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for ExportMemoryWin32HandleInfoNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
