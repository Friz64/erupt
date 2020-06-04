# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_KHR_external_semaphore_fd.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_EXTERNAL_SEMAPHORE_FD_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_EXTERNAL_SEMAPHORE_FD_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_external_semaphore_fd");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkImportSemaphoreFdKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkImportSemaphoreFdKHR = unsafe extern "system" fn ( device : crate :: vk1_0 :: Device , p_import_semaphore_fd_info : * const crate :: extensions :: khr_external_semaphore_fd :: ImportSemaphoreFdInfoKHR , ) -> crate :: vk1_0 :: Result ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSemaphoreFdKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetSemaphoreFdKHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_get_fd_info: *const crate::extensions::khr_external_semaphore_fd::SemaphoreGetFdInfoKHR,
    p_fd: *mut std::os::raw::c_int,
) -> crate::vk1_0::Result;
#[doc = "Provides Device Commands for [`KhrExternalSemaphoreFdDeviceLoaderExt`](trait.KhrExternalSemaphoreFdDeviceLoaderExt.html)"]
pub struct KhrExternalSemaphoreFdDeviceCommands {
    pub import_semaphore_fd_khr: Option<PFN_vkImportSemaphoreFdKHR>,
    pub get_semaphore_fd_khr: Option<PFN_vkGetSemaphoreFdKHR>,
}
impl KhrExternalSemaphoreFdDeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<KhrExternalSemaphoreFdDeviceCommands> {
        unsafe {
            let mut success = false;
            let table = KhrExternalSemaphoreFdDeviceCommands {
                import_semaphore_fd_khr: std::mem::transmute({
                    let symbol = loader.symbol("vkImportSemaphoreFdKHR");
                    success |= symbol.is_some();
                    symbol
                }),
                get_semaphore_fd_khr: std::mem::transmute({
                    let symbol = loader.symbol("vkGetSemaphoreFdKHR");
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
#[inline]
fn device_commands(loader: &crate::DeviceLoader) -> &KhrExternalSemaphoreFdDeviceCommands {
    loader
        .khr_external_semaphore_fd
        .as_ref()
        .expect("`khr_external_semaphore_fd` not loaded")
}
#[doc = "Provides high level command wrappers for [`KhrExternalSemaphoreFdDeviceCommands`](struct.KhrExternalSemaphoreFdDeviceCommands.html)"]
pub trait KhrExternalSemaphoreFdDeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkImportSemaphoreFdKHR.html) · Device Command"]
    unsafe fn import_semaphore_fd_khr(
        &self,
        import_semaphore_fd_info : & crate :: extensions :: khr_external_semaphore_fd :: ImportSemaphoreFdInfoKHR,
    ) -> crate::utils::VulkanResult<()>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSemaphoreFdKHR.html) · Device Command"]
    unsafe fn get_semaphore_fd_khr(
        &self,
        get_fd_info: &crate::extensions::khr_external_semaphore_fd::SemaphoreGetFdInfoKHR,
        fd: Option<std::os::raw::c_int>,
    ) -> crate::utils::VulkanResult<std::os::raw::c_int>;
}
impl KhrExternalSemaphoreFdDeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkImportSemaphoreFdKHR.html) · Device Command"]
    unsafe fn import_semaphore_fd_khr(
        &self,
        import_semaphore_fd_info : & crate :: extensions :: khr_external_semaphore_fd :: ImportSemaphoreFdInfoKHR,
    ) -> crate::utils::VulkanResult<()> {
        let function = device_commands(self)
            .import_semaphore_fd_khr
            .as_ref()
            .expect("`import_semaphore_fd_khr` not available");
        let _val = function(self.handle, import_semaphore_fd_info);
        crate::utils::VulkanResult::new(_val, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSemaphoreFdKHR.html) · Device Command"]
    unsafe fn get_semaphore_fd_khr(
        &self,
        get_fd_info: &crate::extensions::khr_external_semaphore_fd::SemaphoreGetFdInfoKHR,
        fd: Option<std::os::raw::c_int>,
    ) -> crate::utils::VulkanResult<std::os::raw::c_int> {
        let function = device_commands(self)
            .get_semaphore_fd_khr
            .as_ref()
            .expect("`get_semaphore_fd_khr` not available");
        let mut fd = fd.unwrap_or_else(|| Default::default());
        let _val = function(self.handle, get_fd_info, &mut fd);
        crate::utils::VulkanResult::new(_val, fd)
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImportSemaphoreFdInfoKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImportSemaphoreFdInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub semaphore: crate::vk1_0::Semaphore,
    pub flags: crate::vk1_1::SemaphoreImportFlags,
    pub handle_type: crate::vk1_1::ExternalSemaphoreHandleTypeFlagBits,
    pub fd: std::os::raw::c_int,
}
impl ImportSemaphoreFdInfoKHR {
    #[inline]
    pub fn builder<'a>(self) -> ImportSemaphoreFdInfoKHRBuilder<'a> {
        ImportSemaphoreFdInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ImportSemaphoreFdInfoKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ImportSemaphoreFdInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("semaphore", &self.semaphore)
            .field("flags", &self.flags)
            .field("handle_type", &self.handle_type)
            .field("fd", &self.fd)
            .finish()
    }
}
impl Default for ImportSemaphoreFdInfoKHR {
    fn default() -> ImportSemaphoreFdInfoKHR {
        ImportSemaphoreFdInfoKHR {
            s_type: crate::vk1_0::StructureType::IMPORT_SEMAPHORE_FD_INFO_KHR,
            p_next: std::ptr::null(),
            semaphore: Default::default(),
            flags: Default::default(),
            handle_type: Default::default(),
            fd: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImportSemaphoreFdInfoKHR.html) · Builder of [`ImportSemaphoreFdInfoKHR`](struct.ImportSemaphoreFdInfoKHR.html)"]
#[repr(transparent)]
pub struct ImportSemaphoreFdInfoKHRBuilder<'a>(
    ImportSemaphoreFdInfoKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> ImportSemaphoreFdInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> ImportSemaphoreFdInfoKHRBuilder<'a> {
        ImportSemaphoreFdInfoKHRBuilder(Default::default(), std::marker::PhantomData)
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
    pub fn fd(mut self, fd: std::os::raw::c_int) -> Self {
        self.0.fd = fd;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> ImportSemaphoreFdInfoKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for ImportSemaphoreFdInfoKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for ImportSemaphoreFdInfoKHRBuilder<'a> {
    type Target = ImportSemaphoreFdInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ImportSemaphoreFdInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreGetFdInfoKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SemaphoreGetFdInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub semaphore: crate::vk1_0::Semaphore,
    pub handle_type: crate::vk1_1::ExternalSemaphoreHandleTypeFlagBits,
}
impl SemaphoreGetFdInfoKHR {
    #[inline]
    pub fn builder<'a>(self) -> SemaphoreGetFdInfoKHRBuilder<'a> {
        SemaphoreGetFdInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for SemaphoreGetFdInfoKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("SemaphoreGetFdInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("semaphore", &self.semaphore)
            .field("handle_type", &self.handle_type)
            .finish()
    }
}
impl Default for SemaphoreGetFdInfoKHR {
    fn default() -> SemaphoreGetFdInfoKHR {
        SemaphoreGetFdInfoKHR {
            s_type: crate::vk1_0::StructureType::SEMAPHORE_GET_FD_INFO_KHR,
            p_next: std::ptr::null(),
            semaphore: Default::default(),
            handle_type: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreGetFdInfoKHR.html) · Builder of [`SemaphoreGetFdInfoKHR`](struct.SemaphoreGetFdInfoKHR.html)"]
#[repr(transparent)]
pub struct SemaphoreGetFdInfoKHRBuilder<'a>(
    SemaphoreGetFdInfoKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> SemaphoreGetFdInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> SemaphoreGetFdInfoKHRBuilder<'a> {
        SemaphoreGetFdInfoKHRBuilder(Default::default(), std::marker::PhantomData)
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
    pub unsafe fn discard(self) -> SemaphoreGetFdInfoKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for SemaphoreGetFdInfoKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for SemaphoreGetFdInfoKHRBuilder<'a> {
    type Target = SemaphoreGetFdInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SemaphoreGetFdInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
