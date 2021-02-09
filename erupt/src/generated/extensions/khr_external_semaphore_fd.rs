#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_EXTERNAL_SEMAPHORE_FD_SPEC_VERSION")]
pub const KHR_EXTERNAL_SEMAPHORE_FD_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_EXTERNAL_SEMAPHORE_FD_EXTENSION_NAME")]
pub const KHR_EXTERNAL_SEMAPHORE_FD_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_external_semaphore_fd");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_SEMAPHORE_FD_KHR: *const std::os::raw::c_char =
    crate::cstr!("vkGetSemaphoreFdKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_IMPORT_SEMAPHORE_FD_KHR: *const std::os::raw::c_char =
    crate::cstr!("vkImportSemaphoreFdKHR");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSemaphoreFdKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetSemaphoreFdKHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_get_fd_info: *const crate::extensions::khr_external_semaphore_fd::SemaphoreGetFdInfoKHR,
    p_fd: *mut std::os::raw::c_int,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkImportSemaphoreFdKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkImportSemaphoreFdKHR = unsafe extern "system" fn (device : crate :: vk1_0 :: Device , p_import_semaphore_fd_info : * const crate :: extensions :: khr_external_semaphore_fd :: ImportSemaphoreFdInfoKHR) -> crate :: vk1_0 :: Result ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImportSemaphoreFdInfoKHR.html) · Structure"]
#[doc(alias = "VkImportSemaphoreFdInfoKHR")]
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
impl Default for ImportSemaphoreFdInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::IMPORT_SEMAPHORE_FD_INFO_KHR,
            p_next: std::ptr::null(),
            semaphore: Default::default(),
            flags: Default::default(),
            handle_type: Default::default(),
            fd: Default::default(),
        }
    }
}
impl std::fmt::Debug for ImportSemaphoreFdInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ImportSemaphoreFdInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("semaphore", &self.semaphore)
            .field("flags", &self.flags)
            .field("handle_type", &self.handle_type)
            .field("fd", &self.fd)
            .finish()
    }
}
impl ImportSemaphoreFdInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> ImportSemaphoreFdInfoKHRBuilder<'a> {
        ImportSemaphoreFdInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImportSemaphoreFdInfoKHR.html) · Builder of [`ImportSemaphoreFdInfoKHR`]"]
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
    pub fn handle_type(
        mut self,
        handle_type: crate::vk1_1::ExternalSemaphoreHandleTypeFlagBits,
    ) -> Self {
        self.0.handle_type = handle_type as _;
        self
    }
    #[inline]
    pub fn fd(mut self, fd: std::os::raw::c_int) -> Self {
        self.0.fd = fd as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ImportSemaphoreFdInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for ImportSemaphoreFdInfoKHRBuilder<'a> {
    fn default() -> ImportSemaphoreFdInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ImportSemaphoreFdInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
#[doc(alias = "VkSemaphoreGetFdInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SemaphoreGetFdInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub semaphore: crate::vk1_0::Semaphore,
    pub handle_type: crate::vk1_1::ExternalSemaphoreHandleTypeFlagBits,
}
impl Default for SemaphoreGetFdInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::SEMAPHORE_GET_FD_INFO_KHR,
            p_next: std::ptr::null(),
            semaphore: Default::default(),
            handle_type: Default::default(),
        }
    }
}
impl std::fmt::Debug for SemaphoreGetFdInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SemaphoreGetFdInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("semaphore", &self.semaphore)
            .field("handle_type", &self.handle_type)
            .finish()
    }
}
impl SemaphoreGetFdInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> SemaphoreGetFdInfoKHRBuilder<'a> {
        SemaphoreGetFdInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreGetFdInfoKHR.html) · Builder of [`SemaphoreGetFdInfoKHR`]"]
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
    #[inline]
    pub fn semaphore(mut self, semaphore: crate::vk1_0::Semaphore) -> Self {
        self.0.semaphore = semaphore as _;
        self
    }
    #[inline]
    pub fn handle_type(
        mut self,
        handle_type: crate::vk1_1::ExternalSemaphoreHandleTypeFlagBits,
    ) -> Self {
        self.0.handle_type = handle_type as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> SemaphoreGetFdInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for SemaphoreGetFdInfoKHRBuilder<'a> {
    fn default() -> SemaphoreGetFdInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for SemaphoreGetFdInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
#[doc = "Provided by [`crate::extensions::khr_external_semaphore_fd`]"]
impl crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSemaphoreFdKHR.html) · Function"]
    #[doc(alias = "vkGetSemaphoreFdKHR")]
    pub unsafe fn get_semaphore_fd_khr(
        &self,
        get_fd_info: &crate::extensions::khr_external_semaphore_fd::SemaphoreGetFdInfoKHR,
        fd: Option<std::os::raw::c_int>,
    ) -> crate::utils::VulkanResult<std::os::raw::c_int> {
        let _function = self
            .get_semaphore_fd_khr
            .expect("`get_semaphore_fd_khr` is not loaded");
        let mut fd = match fd {
            Some(v) => v,
            None => Default::default(),
        };
        let _return = _function(self.handle, get_fd_info as _, &mut fd);
        crate::utils::VulkanResult::new(_return, fd)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkImportSemaphoreFdKHR.html) · Function"]
    #[doc(alias = "vkImportSemaphoreFdKHR")]
    pub unsafe fn import_semaphore_fd_khr(
        &self,
        import_semaphore_fd_info : & crate :: extensions :: khr_external_semaphore_fd :: ImportSemaphoreFdInfoKHR,
    ) -> crate::utils::VulkanResult<()> {
        let _function = self
            .import_semaphore_fd_khr
            .expect("`import_semaphore_fd_khr` is not loaded");
        let _return = _function(self.handle, import_semaphore_fd_info as _);
        crate::utils::VulkanResult::new(_return, ())
    }
}
