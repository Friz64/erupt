#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_EXTERNAL_FENCE_WIN32_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_EXTERNAL_FENCE_WIN32_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_external_fence_win32");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_FENCE_WIN32_HANDLE_KHR: *const std::os::raw::c_char =
    crate::cstr!("vkGetFenceWin32HandleKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_IMPORT_FENCE_WIN32_HANDLE_KHR: *const std::os::raw::c_char =
    crate::cstr!("vkImportFenceWin32HandleKHR");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetFenceWin32HandleKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetFenceWin32HandleKHR = unsafe extern "system" fn ( device : crate :: vk1_0 :: Device , p_get_win32_handle_info : * const crate :: extensions :: khr_external_fence_win32 :: FenceGetWin32HandleInfoKHR , p_handle : * mut * mut std :: ffi :: c_void ) -> crate :: vk1_0 :: Result ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkImportFenceWin32HandleKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkImportFenceWin32HandleKHR = unsafe extern "system" fn ( device : crate :: vk1_0 :: Device , p_import_fence_win32_handle_info : * const crate :: extensions :: khr_external_fence_win32 :: ImportFenceWin32HandleInfoKHR ) -> crate :: vk1_0 :: Result ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImportFenceWin32HandleInfoKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImportFenceWin32HandleInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub fence: crate::vk1_0::Fence,
    pub flags: crate::vk1_1::FenceImportFlags,
    pub handle_type: crate::vk1_1::ExternalFenceHandleTypeFlagBits,
    pub handle: *mut std::ffi::c_void,
    pub name: *const u16,
}
impl Default for ImportFenceWin32HandleInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::IMPORT_FENCE_WIN32_HANDLE_INFO_KHR,
            p_next: std::ptr::null(),
            fence: Default::default(),
            flags: Default::default(),
            handle_type: Default::default(),
            handle: std::ptr::null_mut(),
            name: std::ptr::null(),
        }
    }
}
impl std::fmt::Debug for ImportFenceWin32HandleInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ImportFenceWin32HandleInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("fence", &self.fence)
            .field("flags", &self.flags)
            .field("handle_type", &self.handle_type)
            .field("handle", &self.handle)
            .field("name", &self.name)
            .finish()
    }
}
impl ImportFenceWin32HandleInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> ImportFenceWin32HandleInfoKHRBuilder<'a> {
        ImportFenceWin32HandleInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImportFenceWin32HandleInfoKHR.html) · Builder of [`ImportFenceWin32HandleInfoKHR`](struct.ImportFenceWin32HandleInfoKHR.html)"]
#[repr(transparent)]
pub struct ImportFenceWin32HandleInfoKHRBuilder<'a>(
    ImportFenceWin32HandleInfoKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> ImportFenceWin32HandleInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> ImportFenceWin32HandleInfoKHRBuilder<'a> {
        ImportFenceWin32HandleInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn fence(mut self, fence: crate::vk1_0::Fence) -> Self {
        self.0.fence = fence as _;
        self
    }
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_1::FenceImportFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn handle_type(
        mut self,
        handle_type: crate::vk1_1::ExternalFenceHandleTypeFlagBits,
    ) -> Self {
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
    pub fn build(self) -> ImportFenceWin32HandleInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for ImportFenceWin32HandleInfoKHRBuilder<'a> {
    fn default() -> ImportFenceWin32HandleInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ImportFenceWin32HandleInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ImportFenceWin32HandleInfoKHRBuilder<'a> {
    type Target = ImportFenceWin32HandleInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ImportFenceWin32HandleInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExportFenceWin32HandleInfoKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExportFenceWin32HandleInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub p_attributes: *const std::ffi::c_void,
    pub dw_access: u32,
    pub name: *const u16,
}
impl Default for ExportFenceWin32HandleInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::EXPORT_FENCE_WIN32_HANDLE_INFO_KHR,
            p_next: std::ptr::null(),
            p_attributes: std::ptr::null(),
            dw_access: Default::default(),
            name: std::ptr::null(),
        }
    }
}
impl std::fmt::Debug for ExportFenceWin32HandleInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ExportFenceWin32HandleInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("p_attributes", &self.p_attributes)
            .field("dw_access", &self.dw_access)
            .field("name", &self.name)
            .finish()
    }
}
impl ExportFenceWin32HandleInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> ExportFenceWin32HandleInfoKHRBuilder<'a> {
        ExportFenceWin32HandleInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExportFenceWin32HandleInfoKHR.html) · Builder of [`ExportFenceWin32HandleInfoKHR`](struct.ExportFenceWin32HandleInfoKHR.html)"]
#[repr(transparent)]
pub struct ExportFenceWin32HandleInfoKHRBuilder<'a>(
    ExportFenceWin32HandleInfoKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> ExportFenceWin32HandleInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> ExportFenceWin32HandleInfoKHRBuilder<'a> {
        ExportFenceWin32HandleInfoKHRBuilder(Default::default(), std::marker::PhantomData)
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
    pub fn build(self) -> ExportFenceWin32HandleInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for ExportFenceWin32HandleInfoKHRBuilder<'a> {
    fn default() -> ExportFenceWin32HandleInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ExportFenceWin32HandleInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ExportFenceWin32HandleInfoKHRBuilder<'a> {
    type Target = ExportFenceWin32HandleInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ExportFenceWin32HandleInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFenceGetWin32HandleInfoKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FenceGetWin32HandleInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub fence: crate::vk1_0::Fence,
    pub handle_type: crate::vk1_1::ExternalFenceHandleTypeFlagBits,
}
impl Default for FenceGetWin32HandleInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::FENCE_GET_WIN32_HANDLE_INFO_KHR,
            p_next: std::ptr::null(),
            fence: Default::default(),
            handle_type: Default::default(),
        }
    }
}
impl std::fmt::Debug for FenceGetWin32HandleInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FenceGetWin32HandleInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("fence", &self.fence)
            .field("handle_type", &self.handle_type)
            .finish()
    }
}
impl FenceGetWin32HandleInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> FenceGetWin32HandleInfoKHRBuilder<'a> {
        FenceGetWin32HandleInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFenceGetWin32HandleInfoKHR.html) · Builder of [`FenceGetWin32HandleInfoKHR`](struct.FenceGetWin32HandleInfoKHR.html)"]
#[repr(transparent)]
pub struct FenceGetWin32HandleInfoKHRBuilder<'a>(
    FenceGetWin32HandleInfoKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> FenceGetWin32HandleInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> FenceGetWin32HandleInfoKHRBuilder<'a> {
        FenceGetWin32HandleInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn fence(mut self, fence: crate::vk1_0::Fence) -> Self {
        self.0.fence = fence as _;
        self
    }
    #[inline]
    pub fn handle_type(
        mut self,
        handle_type: crate::vk1_1::ExternalFenceHandleTypeFlagBits,
    ) -> Self {
        self.0.handle_type = handle_type as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> FenceGetWin32HandleInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for FenceGetWin32HandleInfoKHRBuilder<'a> {
    fn default() -> FenceGetWin32HandleInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for FenceGetWin32HandleInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for FenceGetWin32HandleInfoKHRBuilder<'a> {
    type Target = FenceGetWin32HandleInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for FenceGetWin32HandleInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`extensions::khr_external_fence_win32`](extensions/khr_external_fence_win32/index.html)"]
impl crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetFenceWin32HandleKHR.html) · Function"]
    pub unsafe fn get_fence_win32_handle_khr(
        &self,
        get_win32_handle_info : & crate :: extensions :: khr_external_fence_win32 :: FenceGetWin32HandleInfoKHR,
        handle: *mut *mut std::ffi::c_void,
    ) -> crate::utils::VulkanResult<()> {
        let _function = self
            .get_fence_win32_handle_khr
            .expect("`get_fence_win32_handle_khr` is not loaded");
        let _return = _function(self.handle, get_win32_handle_info as _, handle);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkImportFenceWin32HandleKHR.html) · Function"]
    pub unsafe fn import_fence_win32_handle_khr(
        &self,
        import_fence_win32_handle_info : & crate :: extensions :: khr_external_fence_win32 :: ImportFenceWin32HandleInfoKHR,
    ) -> crate::utils::VulkanResult<()> {
        let _function = self
            .import_fence_win32_handle_khr
            .expect("`import_fence_win32_handle_khr` is not loaded");
        let _return = _function(self.handle, import_fence_win32_handle_info as _);
        crate::utils::VulkanResult::new(_return, ())
    }
}
