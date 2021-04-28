#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_EXTERNAL_SEMAPHORE_WIN32_SPEC_VERSION")]
pub const KHR_EXTERNAL_SEMAPHORE_WIN32_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_EXTERNAL_SEMAPHORE_WIN32_EXTENSION_NAME")]
pub const KHR_EXTERNAL_SEMAPHORE_WIN32_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_external_semaphore_win32");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_SEMAPHORE_WIN32_HANDLE_KHR: *const std::os::raw::c_char = crate::cstr!("vkGetSemaphoreWin32HandleKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_IMPORT_SEMAPHORE_WIN32_HANDLE_KHR: *const std::os::raw::c_char = crate::cstr!("vkImportSemaphoreWin32HandleKHR");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSemaphoreWin32HandleKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetSemaphoreWin32HandleKHR = unsafe extern "system" fn(device: crate::vk1_0::Device, p_get_win32_handle_info: *const crate::extensions::khr_external_semaphore_win32::SemaphoreGetWin32HandleInfoKHR, p_handle: *mut *mut std::ffi::c_void) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkImportSemaphoreWin32HandleKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkImportSemaphoreWin32HandleKHR = unsafe extern "system" fn(device: crate::vk1_0::Device, p_import_semaphore_win32_handle_info: *const crate::extensions::khr_external_semaphore_win32::ImportSemaphoreWin32HandleInfoKHR) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImportSemaphoreWin32HandleInfoKHR.html) · Structure"]
#[doc(alias = "VkImportSemaphoreWin32HandleInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImportSemaphoreWin32HandleInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub semaphore: crate::vk1_0::Semaphore,
    pub flags: crate::vk1_1::SemaphoreImportFlags,
    pub handle_type: crate::vk1_1::ExternalSemaphoreHandleTypeFlagBits,
    pub handle: *mut std::ffi::c_void,
    pub name: *const u16,
}
impl Default for ImportSemaphoreWin32HandleInfoKHR {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR, p_next: std::ptr::null(), semaphore: Default::default(), flags: Default::default(), handle_type: Default::default(), handle: std::ptr::null_mut(), name: std::ptr::null() }
    }
}
impl std::fmt::Debug for ImportSemaphoreWin32HandleInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ImportSemaphoreWin32HandleInfoKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("semaphore", &self.semaphore).field("flags", &self.flags).field("handle_type", &self.handle_type).field("handle", &self.handle).field("name", &self.name).finish()
    }
}
impl ImportSemaphoreWin32HandleInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> ImportSemaphoreWin32HandleInfoKHRBuilder<'a> {
        ImportSemaphoreWin32HandleInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImportSemaphoreWin32HandleInfoKHR.html) · Builder of [`ImportSemaphoreWin32HandleInfoKHR`]"]
#[repr(transparent)]
pub struct ImportSemaphoreWin32HandleInfoKHRBuilder<'a>(ImportSemaphoreWin32HandleInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> ImportSemaphoreWin32HandleInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> ImportSemaphoreWin32HandleInfoKHRBuilder<'a> {
        ImportSemaphoreWin32HandleInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn semaphore(mut self, semaphore: crate::vk1_0::Semaphore) -> Self {
        self.0.semaphore = semaphore as _;
        self
    }
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_1::SemaphoreImportFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn handle_type(mut self, handle_type: crate::vk1_1::ExternalSemaphoreHandleTypeFlagBits) -> Self {
        self.0.handle_type = handle_type as _;
        self
    }
    #[inline]
    pub fn handle(mut self, handle: *mut std::ffi::c_void) -> Self {
        self.0.handle = handle;
        self
    }
    #[inline]
    pub fn name(mut self, name: &'a u16) -> Self {
        self.0.name = name as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ImportSemaphoreWin32HandleInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for ImportSemaphoreWin32HandleInfoKHRBuilder<'a> {
    fn default() -> ImportSemaphoreWin32HandleInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ImportSemaphoreWin32HandleInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ImportSemaphoreWin32HandleInfoKHRBuilder<'a> {
    type Target = ImportSemaphoreWin32HandleInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ImportSemaphoreWin32HandleInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExportSemaphoreWin32HandleInfoKHR.html) · Structure"]
#[doc(alias = "VkExportSemaphoreWin32HandleInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExportSemaphoreWin32HandleInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub p_attributes: *const std::ffi::c_void,
    pub dw_access: u32,
    pub name: *const u16,
}
impl Default for ExportSemaphoreWin32HandleInfoKHR {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR, p_next: std::ptr::null(), p_attributes: std::ptr::null(), dw_access: Default::default(), name: std::ptr::null() }
    }
}
impl std::fmt::Debug for ExportSemaphoreWin32HandleInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ExportSemaphoreWin32HandleInfoKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("p_attributes", &self.p_attributes).field("dw_access", &self.dw_access).field("name", &self.name).finish()
    }
}
impl ExportSemaphoreWin32HandleInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> ExportSemaphoreWin32HandleInfoKHRBuilder<'a> {
        ExportSemaphoreWin32HandleInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExportSemaphoreWin32HandleInfoKHR.html) · Builder of [`ExportSemaphoreWin32HandleInfoKHR`]"]
#[repr(transparent)]
pub struct ExportSemaphoreWin32HandleInfoKHRBuilder<'a>(ExportSemaphoreWin32HandleInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> ExportSemaphoreWin32HandleInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> ExportSemaphoreWin32HandleInfoKHRBuilder<'a> {
        ExportSemaphoreWin32HandleInfoKHRBuilder(Default::default(), std::marker::PhantomData)
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
    pub fn name(mut self, name: &'a u16) -> Self {
        self.0.name = name as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ExportSemaphoreWin32HandleInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for ExportSemaphoreWin32HandleInfoKHRBuilder<'a> {
    fn default() -> ExportSemaphoreWin32HandleInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ExportSemaphoreWin32HandleInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ExportSemaphoreWin32HandleInfoKHRBuilder<'a> {
    type Target = ExportSemaphoreWin32HandleInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ExportSemaphoreWin32HandleInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkD3D12FenceSubmitInfoKHR.html) · Structure"]
#[doc(alias = "VkD3D12FenceSubmitInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct D3D12FenceSubmitInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub wait_semaphore_values_count: u32,
    pub p_wait_semaphore_values: *const u64,
    pub signal_semaphore_values_count: u32,
    pub p_signal_semaphore_values: *const u64,
}
impl Default for D3D12FenceSubmitInfoKHR {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::D3D12_FENCE_SUBMIT_INFO_KHR, p_next: std::ptr::null(), wait_semaphore_values_count: Default::default(), p_wait_semaphore_values: std::ptr::null(), signal_semaphore_values_count: Default::default(), p_signal_semaphore_values: std::ptr::null() }
    }
}
impl std::fmt::Debug for D3D12FenceSubmitInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("D3D12FenceSubmitInfoKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("wait_semaphore_values_count", &self.wait_semaphore_values_count).field("p_wait_semaphore_values", &self.p_wait_semaphore_values).field("signal_semaphore_values_count", &self.signal_semaphore_values_count).field("p_signal_semaphore_values", &self.p_signal_semaphore_values).finish()
    }
}
impl D3D12FenceSubmitInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> D3D12FenceSubmitInfoKHRBuilder<'a> {
        D3D12FenceSubmitInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkD3D12FenceSubmitInfoKHR.html) · Builder of [`D3D12FenceSubmitInfoKHR`]"]
#[repr(transparent)]
pub struct D3D12FenceSubmitInfoKHRBuilder<'a>(D3D12FenceSubmitInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> D3D12FenceSubmitInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> D3D12FenceSubmitInfoKHRBuilder<'a> {
        D3D12FenceSubmitInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn wait_semaphore_values(mut self, wait_semaphore_values: &'a [u64]) -> Self {
        self.0.p_wait_semaphore_values = wait_semaphore_values.as_ptr() as _;
        self.0.wait_semaphore_values_count = wait_semaphore_values.len() as _;
        self
    }
    #[inline]
    pub fn signal_semaphore_values(mut self, signal_semaphore_values: &'a [u64]) -> Self {
        self.0.p_signal_semaphore_values = signal_semaphore_values.as_ptr() as _;
        self.0.signal_semaphore_values_count = signal_semaphore_values.len() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> D3D12FenceSubmitInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for D3D12FenceSubmitInfoKHRBuilder<'a> {
    fn default() -> D3D12FenceSubmitInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for D3D12FenceSubmitInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for D3D12FenceSubmitInfoKHRBuilder<'a> {
    type Target = D3D12FenceSubmitInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for D3D12FenceSubmitInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreGetWin32HandleInfoKHR.html) · Structure"]
#[doc(alias = "VkSemaphoreGetWin32HandleInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SemaphoreGetWin32HandleInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub semaphore: crate::vk1_0::Semaphore,
    pub handle_type: crate::vk1_1::ExternalSemaphoreHandleTypeFlagBits,
}
impl Default for SemaphoreGetWin32HandleInfoKHR {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR, p_next: std::ptr::null(), semaphore: Default::default(), handle_type: Default::default() }
    }
}
impl std::fmt::Debug for SemaphoreGetWin32HandleInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SemaphoreGetWin32HandleInfoKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("semaphore", &self.semaphore).field("handle_type", &self.handle_type).finish()
    }
}
impl SemaphoreGetWin32HandleInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> SemaphoreGetWin32HandleInfoKHRBuilder<'a> {
        SemaphoreGetWin32HandleInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreGetWin32HandleInfoKHR.html) · Builder of [`SemaphoreGetWin32HandleInfoKHR`]"]
#[repr(transparent)]
pub struct SemaphoreGetWin32HandleInfoKHRBuilder<'a>(SemaphoreGetWin32HandleInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> SemaphoreGetWin32HandleInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> SemaphoreGetWin32HandleInfoKHRBuilder<'a> {
        SemaphoreGetWin32HandleInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn semaphore(mut self, semaphore: crate::vk1_0::Semaphore) -> Self {
        self.0.semaphore = semaphore as _;
        self
    }
    #[inline]
    pub fn handle_type(mut self, handle_type: crate::vk1_1::ExternalSemaphoreHandleTypeFlagBits) -> Self {
        self.0.handle_type = handle_type as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> SemaphoreGetWin32HandleInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for SemaphoreGetWin32HandleInfoKHRBuilder<'a> {
    fn default() -> SemaphoreGetWin32HandleInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for SemaphoreGetWin32HandleInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for SemaphoreGetWin32HandleInfoKHRBuilder<'a> {
    type Target = SemaphoreGetWin32HandleInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SemaphoreGetWin32HandleInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::khr_external_semaphore_win32`]"]
impl crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSemaphoreWin32HandleKHR.html) · Function"]
    #[doc(alias = "vkGetSemaphoreWin32HandleKHR")]
    pub unsafe fn get_semaphore_win32_handle_khr(&self, get_win32_handle_info: &crate::extensions::khr_external_semaphore_win32::SemaphoreGetWin32HandleInfoKHR, handle: *mut *mut std::ffi::c_void) -> crate::utils::VulkanResult<()> {
        let _function = self.get_semaphore_win32_handle_khr.expect("`get_semaphore_win32_handle_khr` is not loaded");
        let _return = _function(self.handle, get_win32_handle_info as _, handle);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkImportSemaphoreWin32HandleKHR.html) · Function"]
    #[doc(alias = "vkImportSemaphoreWin32HandleKHR")]
    pub unsafe fn import_semaphore_win32_handle_khr(&self, import_semaphore_win32_handle_info: &crate::extensions::khr_external_semaphore_win32::ImportSemaphoreWin32HandleInfoKHR) -> crate::utils::VulkanResult<()> {
        let _function = self.import_semaphore_win32_handle_khr.expect("`import_semaphore_win32_handle_khr` is not loaded");
        let _return = _function(self.handle, import_semaphore_win32_handle_info as _);
        crate::utils::VulkanResult::new(_return, ())
    }
}
