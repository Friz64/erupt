# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_KHR_external_semaphore_win32.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_EXTERNAL_SEMAPHORE_WIN32_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_EXTERNAL_SEMAPHORE_WIN32_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_external_semaphore_win32");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkImportSemaphoreWin32HandleKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkImportSemaphoreWin32HandleKHR = unsafe extern "system" fn ( device : crate :: vk1_0 :: Device , p_import_semaphore_win32_handle_info : * const crate :: extensions :: khr_external_semaphore_win32 :: ImportSemaphoreWin32HandleInfoKHR , ) -> crate :: vk1_0 :: Result ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSemaphoreWin32HandleKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetSemaphoreWin32HandleKHR = unsafe extern "system" fn ( device : crate :: vk1_0 :: Device , p_get_win32_handle_info : * const crate :: extensions :: khr_external_semaphore_win32 :: SemaphoreGetWin32HandleInfoKHR , p_handle : * mut * mut std :: ffi :: c_void , ) -> crate :: vk1_0 :: Result ;
#[doc = "Provides Device Commands for [`KhrExternalSemaphoreWin32DeviceLoaderExt`](trait.KhrExternalSemaphoreWin32DeviceLoaderExt.html)"]
pub struct KhrExternalSemaphoreWin32DeviceCommands {
    pub import_semaphore_win32_handle_khr: PFN_vkImportSemaphoreWin32HandleKHR,
    pub get_semaphore_win32_handle_khr: PFN_vkGetSemaphoreWin32HandleKHR,
}
impl KhrExternalSemaphoreWin32DeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<KhrExternalSemaphoreWin32DeviceCommands> {
        unsafe {
            Some(KhrExternalSemaphoreWin32DeviceCommands {
                import_semaphore_win32_handle_khr: std::mem::transmute(
                    loader.symbol("vkImportSemaphoreWin32HandleKHR")?,
                ),
                get_semaphore_win32_handle_khr: std::mem::transmute(
                    loader.symbol("vkGetSemaphoreWin32HandleKHR")?,
                ),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`KhrExternalSemaphoreWin32DeviceCommands`](struct.KhrExternalSemaphoreWin32DeviceCommands.html)"]
pub trait KhrExternalSemaphoreWin32DeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkImportSemaphoreWin32HandleKHR.html) · Device Command"]
    unsafe fn import_semaphore_win32_handle_khr(
        &self,
        import_semaphore_win32_handle_info : & crate :: extensions :: khr_external_semaphore_win32 :: ImportSemaphoreWin32HandleInfoKHR,
    ) -> crate::utils::VulkanResult<()>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSemaphoreWin32HandleKHR.html) · Device Command"]
    unsafe fn get_semaphore_win32_handle_khr(
        &self,
        get_win32_handle_info : & crate :: extensions :: khr_external_semaphore_win32 :: SemaphoreGetWin32HandleInfoKHR,
        handle: *mut *mut std::ffi::c_void,
    ) -> crate::utils::VulkanResult<()>;
}
impl KhrExternalSemaphoreWin32DeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkImportSemaphoreWin32HandleKHR.html) · Device Command"]
    unsafe fn import_semaphore_win32_handle_khr(
        &self,
        import_semaphore_win32_handle_info : & crate :: extensions :: khr_external_semaphore_win32 :: ImportSemaphoreWin32HandleInfoKHR,
    ) -> crate::utils::VulkanResult<()> {
        let function = self
            .khr_external_semaphore_win32
            .as_ref()
            .expect("`khr_external_semaphore_win32` not loaded")
            .import_semaphore_win32_handle_khr;
        let _val = function(self.handle, import_semaphore_win32_handle_info);
        crate::utils::VulkanResult::new(_val, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSemaphoreWin32HandleKHR.html) · Device Command"]
    unsafe fn get_semaphore_win32_handle_khr(
        &self,
        get_win32_handle_info : & crate :: extensions :: khr_external_semaphore_win32 :: SemaphoreGetWin32HandleInfoKHR,
        handle: *mut *mut std::ffi::c_void,
    ) -> crate::utils::VulkanResult<()> {
        let function = self
            .khr_external_semaphore_win32
            .as_ref()
            .expect("`khr_external_semaphore_win32` not loaded")
            .get_semaphore_win32_handle_khr;
        let _val = function(self.handle, get_win32_handle_info, handle);
        crate::utils::VulkanResult::new(_val, ())
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImportSemaphoreWin32HandleInfoKHR.html) · Structure"]
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
impl ImportSemaphoreWin32HandleInfoKHR {
    #[inline]
    pub fn builder<'a>(self) -> ImportSemaphoreWin32HandleInfoKHRBuilder<'a> {
        ImportSemaphoreWin32HandleInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ImportSemaphoreWin32HandleInfoKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ImportSemaphoreWin32HandleInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("semaphore", &self.semaphore)
            .field("flags", &self.flags)
            .field("handle_type", &self.handle_type)
            .field("handle", &self.handle)
            .field("name", &self.name)
            .finish()
    }
}
impl Default for ImportSemaphoreWin32HandleInfoKHR {
    fn default() -> ImportSemaphoreWin32HandleInfoKHR {
        ImportSemaphoreWin32HandleInfoKHR {
            s_type: crate::vk1_0::StructureType::IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR,
            p_next: std::ptr::null(),
            semaphore: Default::default(),
            flags: Default::default(),
            handle_type: Default::default(),
            handle: std::ptr::null_mut(),
            name: std::ptr::null(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`ImportSemaphoreWin32HandleInfoKHR`](struct.ImportSemaphoreWin32HandleInfoKHR.html)"]
#[repr(transparent)]
pub struct ImportSemaphoreWin32HandleInfoKHRBuilder<'a>(
    ImportSemaphoreWin32HandleInfoKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> ImportSemaphoreWin32HandleInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> ImportSemaphoreWin32HandleInfoKHRBuilder<'a> {
        ImportSemaphoreWin32HandleInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn semaphore(mut self, semaphore: crate::vk1_0::Semaphore) -> Self {
        self.0.semaphore = semaphore;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_1::SemaphoreImportFlags) -> Self {
        self.0.flags = flags;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn handle_type(
        mut self,
        handle_type: crate::vk1_1::ExternalSemaphoreHandleTypeFlagBits,
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
    #[allow(unused_mut)]
    #[inline]
    pub fn name(mut self, name: &'a u16) -> Self {
        self.0.name = name;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> ImportSemaphoreWin32HandleInfoKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for ImportSemaphoreWin32HandleInfoKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreGetWin32HandleInfoKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SemaphoreGetWin32HandleInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub semaphore: crate::vk1_0::Semaphore,
    pub handle_type: crate::vk1_1::ExternalSemaphoreHandleTypeFlagBits,
}
impl SemaphoreGetWin32HandleInfoKHR {
    #[inline]
    pub fn builder<'a>(self) -> SemaphoreGetWin32HandleInfoKHRBuilder<'a> {
        SemaphoreGetWin32HandleInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for SemaphoreGetWin32HandleInfoKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("SemaphoreGetWin32HandleInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("semaphore", &self.semaphore)
            .field("handle_type", &self.handle_type)
            .finish()
    }
}
impl Default for SemaphoreGetWin32HandleInfoKHR {
    fn default() -> SemaphoreGetWin32HandleInfoKHR {
        SemaphoreGetWin32HandleInfoKHR {
            s_type: crate::vk1_0::StructureType::SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR,
            p_next: std::ptr::null(),
            semaphore: Default::default(),
            handle_type: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`SemaphoreGetWin32HandleInfoKHR`](struct.SemaphoreGetWin32HandleInfoKHR.html)"]
#[repr(transparent)]
pub struct SemaphoreGetWin32HandleInfoKHRBuilder<'a>(
    SemaphoreGetWin32HandleInfoKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> SemaphoreGetWin32HandleInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> SemaphoreGetWin32HandleInfoKHRBuilder<'a> {
        SemaphoreGetWin32HandleInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn semaphore(mut self, semaphore: crate::vk1_0::Semaphore) -> Self {
        self.0.semaphore = semaphore;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn handle_type(
        mut self,
        handle_type: crate::vk1_1::ExternalSemaphoreHandleTypeFlagBits,
    ) -> Self {
        self.0.handle_type = handle_type;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> SemaphoreGetWin32HandleInfoKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for SemaphoreGetWin32HandleInfoKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExportSemaphoreWin32HandleInfoKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExportSemaphoreWin32HandleInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub p_attributes: *const crate::SECURITY_ATTRIBUTES,
    pub dw_access: u32,
    pub name: *const u16,
}
impl ExportSemaphoreWin32HandleInfoKHR {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByExportSemaphoreWin32HandleInfoKHR,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> ExportSemaphoreWin32HandleInfoKHRBuilder<'a> {
        ExportSemaphoreWin32HandleInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ExportSemaphoreWin32HandleInfoKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ExportSemaphoreWin32HandleInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("p_attributes", &self.p_attributes)
            .field("dw_access", &self.dw_access)
            .field("name", &self.name)
            .finish()
    }
}
impl Default for ExportSemaphoreWin32HandleInfoKHR {
    fn default() -> ExportSemaphoreWin32HandleInfoKHR {
        ExportSemaphoreWin32HandleInfoKHR {
            s_type: crate::vk1_0::StructureType::EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR,
            p_next: std::ptr::null(),
            p_attributes: std::ptr::null(),
            dw_access: Default::default(),
            name: std::ptr::null(),
        }
    }
}
#[doc = "Used by [`ExportSemaphoreWin32HandleInfoKHR::extend`](struct.ExportSemaphoreWin32HandleInfoKHR.html#method.extend)"]
pub trait ExtendableByExportSemaphoreWin32HandleInfoKHR {}
impl ExtendableByExportSemaphoreWin32HandleInfoKHR for crate::vk1_0::SemaphoreCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`ExportSemaphoreWin32HandleInfoKHR`](struct.ExportSemaphoreWin32HandleInfoKHR.html)"]
#[repr(transparent)]
pub struct ExportSemaphoreWin32HandleInfoKHRBuilder<'a>(
    ExportSemaphoreWin32HandleInfoKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> ExportSemaphoreWin32HandleInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> ExportSemaphoreWin32HandleInfoKHRBuilder<'a> {
        ExportSemaphoreWin32HandleInfoKHRBuilder(Default::default(), std::marker::PhantomData)
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
    #[allow(unused_mut)]
    #[inline]
    pub fn name(mut self, name: &'a u16) -> Self {
        self.0.name = name;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> ExportSemaphoreWin32HandleInfoKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for ExportSemaphoreWin32HandleInfoKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
impl D3D12FenceSubmitInfoKHR {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByD3D12FenceSubmitInfoKHR,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> D3D12FenceSubmitInfoKHRBuilder<'a> {
        D3D12FenceSubmitInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for D3D12FenceSubmitInfoKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("D3D12FenceSubmitInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "wait_semaphore_values_count",
                &self.wait_semaphore_values_count,
            )
            .field("p_wait_semaphore_values", &self.p_wait_semaphore_values)
            .field(
                "signal_semaphore_values_count",
                &self.signal_semaphore_values_count,
            )
            .field("p_signal_semaphore_values", &self.p_signal_semaphore_values)
            .finish()
    }
}
impl Default for D3D12FenceSubmitInfoKHR {
    fn default() -> D3D12FenceSubmitInfoKHR {
        D3D12FenceSubmitInfoKHR {
            s_type: crate::vk1_0::StructureType::D3D12_FENCE_SUBMIT_INFO_KHR,
            p_next: std::ptr::null(),
            wait_semaphore_values_count: Default::default(),
            p_wait_semaphore_values: std::ptr::null(),
            signal_semaphore_values_count: Default::default(),
            p_signal_semaphore_values: std::ptr::null(),
        }
    }
}
#[doc = "Used by [`D3D12FenceSubmitInfoKHR::extend`](struct.D3D12FenceSubmitInfoKHR.html#method.extend)"]
pub trait ExtendableByD3D12FenceSubmitInfoKHR {}
impl ExtendableByD3D12FenceSubmitInfoKHR for crate::vk1_0::SubmitInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`D3D12FenceSubmitInfoKHR`](struct.D3D12FenceSubmitInfoKHR.html)"]
#[repr(transparent)]
pub struct D3D12FenceSubmitInfoKHRBuilder<'a>(
    D3D12FenceSubmitInfoKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> D3D12FenceSubmitInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> D3D12FenceSubmitInfoKHRBuilder<'a> {
        D3D12FenceSubmitInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn wait_semaphore_values(mut self, wait_semaphore_values: &'a [u64]) -> Self {
        self.0.wait_semaphore_values_count = wait_semaphore_values.len() as _;
        self.0.p_wait_semaphore_values = wait_semaphore_values.as_ptr() as _;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn signal_semaphore_values(mut self, signal_semaphore_values: &'a [u64]) -> Self {
        self.0.signal_semaphore_values_count = signal_semaphore_values.len() as _;
        self.0.p_signal_semaphore_values = signal_semaphore_values.as_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> D3D12FenceSubmitInfoKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for D3D12FenceSubmitInfoKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
