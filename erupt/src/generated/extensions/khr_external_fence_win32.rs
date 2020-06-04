# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_KHR_external_fence_win32.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_EXTERNAL_FENCE_WIN32_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_EXTERNAL_FENCE_WIN32_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_external_fence_win32");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkImportFenceWin32HandleKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkImportFenceWin32HandleKHR = unsafe extern "system" fn ( device : crate :: vk1_0 :: Device , p_import_fence_win32_handle_info : * const crate :: extensions :: khr_external_fence_win32 :: ImportFenceWin32HandleInfoKHR , ) -> crate :: vk1_0 :: Result ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetFenceWin32HandleKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetFenceWin32HandleKHR = unsafe extern "system" fn ( device : crate :: vk1_0 :: Device , p_get_win32_handle_info : * const crate :: extensions :: khr_external_fence_win32 :: FenceGetWin32HandleInfoKHR , p_handle : * mut * mut std :: ffi :: c_void , ) -> crate :: vk1_0 :: Result ;
#[doc = "Provides Device Commands for [`KhrExternalFenceWin32DeviceLoaderExt`](trait.KhrExternalFenceWin32DeviceLoaderExt.html)"]
pub struct KhrExternalFenceWin32DeviceCommands {
    pub import_fence_win32_handle_khr: Option<PFN_vkImportFenceWin32HandleKHR>,
    pub get_fence_win32_handle_khr: Option<PFN_vkGetFenceWin32HandleKHR>,
}
impl KhrExternalFenceWin32DeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<KhrExternalFenceWin32DeviceCommands> {
        unsafe {
            let mut success = false;
            let table = KhrExternalFenceWin32DeviceCommands {
                import_fence_win32_handle_khr: std::mem::transmute({
                    let symbol = loader.symbol("vkImportFenceWin32HandleKHR");
                    success |= symbol.is_some();
                    symbol
                }),
                get_fence_win32_handle_khr: std::mem::transmute({
                    let symbol = loader.symbol("vkGetFenceWin32HandleKHR");
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
fn device_commands(loader: &crate::DeviceLoader) -> &KhrExternalFenceWin32DeviceCommands {
    loader
        .khr_external_fence_win32
        .as_ref()
        .expect("`khr_external_fence_win32` not loaded")
}
#[doc = "Provides high level command wrappers for [`KhrExternalFenceWin32DeviceCommands`](struct.KhrExternalFenceWin32DeviceCommands.html)"]
pub trait KhrExternalFenceWin32DeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkImportFenceWin32HandleKHR.html) · Device Command"]
    unsafe fn import_fence_win32_handle_khr(
        &self,
        import_fence_win32_handle_info : & crate :: extensions :: khr_external_fence_win32 :: ImportFenceWin32HandleInfoKHR,
    ) -> crate::utils::VulkanResult<()>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetFenceWin32HandleKHR.html) · Device Command"]
    unsafe fn get_fence_win32_handle_khr(
        &self,
        get_win32_handle_info : & crate :: extensions :: khr_external_fence_win32 :: FenceGetWin32HandleInfoKHR,
        handle: *mut *mut std::ffi::c_void,
    ) -> crate::utils::VulkanResult<()>;
}
impl KhrExternalFenceWin32DeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkImportFenceWin32HandleKHR.html) · Device Command"]
    unsafe fn import_fence_win32_handle_khr(
        &self,
        import_fence_win32_handle_info : & crate :: extensions :: khr_external_fence_win32 :: ImportFenceWin32HandleInfoKHR,
    ) -> crate::utils::VulkanResult<()> {
        let function = device_commands(self)
            .import_fence_win32_handle_khr
            .as_ref()
            .expect("`import_fence_win32_handle_khr` not available");
        let _val = function(self.handle, import_fence_win32_handle_info);
        crate::utils::VulkanResult::new(_val, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetFenceWin32HandleKHR.html) · Device Command"]
    unsafe fn get_fence_win32_handle_khr(
        &self,
        get_win32_handle_info : & crate :: extensions :: khr_external_fence_win32 :: FenceGetWin32HandleInfoKHR,
        handle: *mut *mut std::ffi::c_void,
    ) -> crate::utils::VulkanResult<()> {
        let function = device_commands(self)
            .get_fence_win32_handle_khr
            .as_ref()
            .expect("`get_fence_win32_handle_khr` not available");
        let _val = function(self.handle, get_win32_handle_info, handle);
        crate::utils::VulkanResult::new(_val, ())
    }
}
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
impl ImportFenceWin32HandleInfoKHR {
    #[inline]
    pub fn builder<'a>(self) -> ImportFenceWin32HandleInfoKHRBuilder<'a> {
        ImportFenceWin32HandleInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ImportFenceWin32HandleInfoKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ImportFenceWin32HandleInfoKHR")
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
impl Default for ImportFenceWin32HandleInfoKHR {
    fn default() -> ImportFenceWin32HandleInfoKHR {
        ImportFenceWin32HandleInfoKHR {
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
    #[allow(unused_mut)]
    #[inline]
    pub fn fence(mut self, fence: crate::vk1_0::Fence) -> Self {
        self.0.fence = fence;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_1::FenceImportFlags) -> Self {
        self.0.flags = flags;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn handle_type(
        mut self,
        handle_type: crate::vk1_1::ExternalFenceHandleTypeFlagBits,
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
    pub unsafe fn discard(self) -> ImportFenceWin32HandleInfoKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for ImportFenceWin32HandleInfoKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFenceGetWin32HandleInfoKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FenceGetWin32HandleInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub fence: crate::vk1_0::Fence,
    pub handle_type: crate::vk1_1::ExternalFenceHandleTypeFlagBits,
}
impl FenceGetWin32HandleInfoKHR {
    #[inline]
    pub fn builder<'a>(self) -> FenceGetWin32HandleInfoKHRBuilder<'a> {
        FenceGetWin32HandleInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for FenceGetWin32HandleInfoKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("FenceGetWin32HandleInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("fence", &self.fence)
            .field("handle_type", &self.handle_type)
            .finish()
    }
}
impl Default for FenceGetWin32HandleInfoKHR {
    fn default() -> FenceGetWin32HandleInfoKHR {
        FenceGetWin32HandleInfoKHR {
            s_type: crate::vk1_0::StructureType::FENCE_GET_WIN32_HANDLE_INFO_KHR,
            p_next: std::ptr::null(),
            fence: Default::default(),
            handle_type: Default::default(),
        }
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
    #[allow(unused_mut)]
    #[inline]
    pub fn fence(mut self, fence: crate::vk1_0::Fence) -> Self {
        self.0.fence = fence;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn handle_type(
        mut self,
        handle_type: crate::vk1_1::ExternalFenceHandleTypeFlagBits,
    ) -> Self {
        self.0.handle_type = handle_type;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> FenceGetWin32HandleInfoKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for FenceGetWin32HandleInfoKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExportFenceWin32HandleInfoKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExportFenceWin32HandleInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub p_attributes: *const crate::SECURITY_ATTRIBUTES,
    pub dw_access: u32,
    pub name: *const u16,
}
impl ExportFenceWin32HandleInfoKHR {
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
    pub fn builder<'a>(self) -> ExportFenceWin32HandleInfoKHRBuilder<'a> {
        ExportFenceWin32HandleInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ExportFenceWin32HandleInfoKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ExportFenceWin32HandleInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("p_attributes", &self.p_attributes)
            .field("dw_access", &self.dw_access)
            .field("name", &self.name)
            .finish()
    }
}
impl Default for ExportFenceWin32HandleInfoKHR {
    fn default() -> ExportFenceWin32HandleInfoKHR {
        ExportFenceWin32HandleInfoKHR {
            s_type: crate::vk1_0::StructureType::EXPORT_FENCE_WIN32_HANDLE_INFO_KHR,
            p_next: std::ptr::null(),
            p_attributes: std::ptr::null(),
            dw_access: Default::default(),
            name: std::ptr::null(),
        }
    }
}
impl crate::ExtendableBy<ExportFenceWin32HandleInfoKHR> for crate::vk1_0::FenceCreateInfo {}
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
    pub unsafe fn discard(self) -> ExportFenceWin32HandleInfoKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for ExportFenceWin32HandleInfoKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
