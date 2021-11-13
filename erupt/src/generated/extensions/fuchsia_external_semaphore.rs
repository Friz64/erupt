#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_FUCHSIA_EXTERNAL_SEMAPHORE_SPEC_VERSION")]
pub const FUCHSIA_EXTERNAL_SEMAPHORE_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_FUCHSIA_EXTERNAL_SEMAPHORE_EXTENSION_NAME")]
pub const FUCHSIA_EXTERNAL_SEMAPHORE_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_FUCHSIA_external_semaphore");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_SEMAPHORE_ZIRCON_HANDLE_FUCHSIA: *const std::os::raw::c_char = crate::cstr!("vkGetSemaphoreZirconHandleFUCHSIA");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_IMPORT_SEMAPHORE_ZIRCON_HANDLE_FUCHSIA: *const std::os::raw::c_char = crate::cstr!("vkImportSemaphoreZirconHandleFUCHSIA");
#[doc = "Provided by [`crate::extensions::fuchsia_external_semaphore`]"]
impl crate::vk1_0::StructureType {
    pub const IMPORT_SEMAPHORE_ZIRCON_HANDLE_INFO_FUCHSIA: Self = Self(1000365000);
    pub const SEMAPHORE_GET_ZIRCON_HANDLE_INFO_FUCHSIA: Self = Self(1000365001);
}
#[doc = "Provided by [`crate::extensions::fuchsia_external_semaphore`]"]
impl crate::vk1_1::ExternalSemaphoreHandleTypeFlagBits {
    pub const ZIRCON_EVENT_FUCHSIA: Self = Self(128);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSemaphoreZirconHandleFUCHSIA.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetSemaphoreZirconHandleFUCHSIA = unsafe extern "system" fn(device: crate::vk1_0::Device, p_get_zircon_handle_info: *const crate::extensions::fuchsia_external_semaphore::SemaphoreGetZirconHandleInfoFUCHSIA, p_zircon_handle: *mut *mut std::ffi::c_void) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkImportSemaphoreZirconHandleFUCHSIA.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkImportSemaphoreZirconHandleFUCHSIA = unsafe extern "system" fn(device: crate::vk1_0::Device, p_import_semaphore_zircon_handle_info: *const crate::extensions::fuchsia_external_semaphore::ImportSemaphoreZirconHandleInfoFUCHSIA) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImportSemaphoreZirconHandleInfoFUCHSIA.html) · Structure"]
#[doc(alias = "VkImportSemaphoreZirconHandleInfoFUCHSIA")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImportSemaphoreZirconHandleInfoFUCHSIA {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub semaphore: crate::vk1_0::Semaphore,
    pub flags: crate::vk1_1::SemaphoreImportFlags,
    pub handle_type: crate::vk1_1::ExternalSemaphoreHandleTypeFlagBits,
    pub zircon_handle: *mut std::ffi::c_void,
}
impl ImportSemaphoreZirconHandleInfoFUCHSIA {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::IMPORT_SEMAPHORE_ZIRCON_HANDLE_INFO_FUCHSIA;
}
impl Default for ImportSemaphoreZirconHandleInfoFUCHSIA {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), semaphore: Default::default(), flags: Default::default(), handle_type: Default::default(), zircon_handle: std::ptr::null_mut() }
    }
}
impl std::fmt::Debug for ImportSemaphoreZirconHandleInfoFUCHSIA {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ImportSemaphoreZirconHandleInfoFUCHSIA").field("s_type", &self.s_type).field("p_next", &self.p_next).field("semaphore", &self.semaphore).field("flags", &self.flags).field("handle_type", &self.handle_type).field("zircon_handle", &self.zircon_handle).finish()
    }
}
impl ImportSemaphoreZirconHandleInfoFUCHSIA {
    #[inline]
    pub fn into_builder<'a>(self) -> ImportSemaphoreZirconHandleInfoFUCHSIABuilder<'a> {
        ImportSemaphoreZirconHandleInfoFUCHSIABuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImportSemaphoreZirconHandleInfoFUCHSIA.html) · Builder of [`ImportSemaphoreZirconHandleInfoFUCHSIA`]"]
#[repr(transparent)]
pub struct ImportSemaphoreZirconHandleInfoFUCHSIABuilder<'a>(ImportSemaphoreZirconHandleInfoFUCHSIA, std::marker::PhantomData<&'a ()>);
impl<'a> ImportSemaphoreZirconHandleInfoFUCHSIABuilder<'a> {
    #[inline]
    pub fn new() -> ImportSemaphoreZirconHandleInfoFUCHSIABuilder<'a> {
        ImportSemaphoreZirconHandleInfoFUCHSIABuilder(Default::default(), std::marker::PhantomData)
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
    pub fn zircon_handle(mut self, zircon_handle: *mut std::ffi::c_void) -> Self {
        self.0.zircon_handle = zircon_handle;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> ImportSemaphoreZirconHandleInfoFUCHSIA {
        self.0
    }
}
impl<'a> std::default::Default for ImportSemaphoreZirconHandleInfoFUCHSIABuilder<'a> {
    fn default() -> ImportSemaphoreZirconHandleInfoFUCHSIABuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ImportSemaphoreZirconHandleInfoFUCHSIABuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ImportSemaphoreZirconHandleInfoFUCHSIABuilder<'a> {
    type Target = ImportSemaphoreZirconHandleInfoFUCHSIA;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ImportSemaphoreZirconHandleInfoFUCHSIABuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreGetZirconHandleInfoFUCHSIA.html) · Structure"]
#[doc(alias = "VkSemaphoreGetZirconHandleInfoFUCHSIA")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SemaphoreGetZirconHandleInfoFUCHSIA {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub semaphore: crate::vk1_0::Semaphore,
    pub handle_type: crate::vk1_1::ExternalSemaphoreHandleTypeFlagBits,
}
impl SemaphoreGetZirconHandleInfoFUCHSIA {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::SEMAPHORE_GET_ZIRCON_HANDLE_INFO_FUCHSIA;
}
impl Default for SemaphoreGetZirconHandleInfoFUCHSIA {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), semaphore: Default::default(), handle_type: Default::default() }
    }
}
impl std::fmt::Debug for SemaphoreGetZirconHandleInfoFUCHSIA {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SemaphoreGetZirconHandleInfoFUCHSIA").field("s_type", &self.s_type).field("p_next", &self.p_next).field("semaphore", &self.semaphore).field("handle_type", &self.handle_type).finish()
    }
}
impl SemaphoreGetZirconHandleInfoFUCHSIA {
    #[inline]
    pub fn into_builder<'a>(self) -> SemaphoreGetZirconHandleInfoFUCHSIABuilder<'a> {
        SemaphoreGetZirconHandleInfoFUCHSIABuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreGetZirconHandleInfoFUCHSIA.html) · Builder of [`SemaphoreGetZirconHandleInfoFUCHSIA`]"]
#[repr(transparent)]
pub struct SemaphoreGetZirconHandleInfoFUCHSIABuilder<'a>(SemaphoreGetZirconHandleInfoFUCHSIA, std::marker::PhantomData<&'a ()>);
impl<'a> SemaphoreGetZirconHandleInfoFUCHSIABuilder<'a> {
    #[inline]
    pub fn new() -> SemaphoreGetZirconHandleInfoFUCHSIABuilder<'a> {
        SemaphoreGetZirconHandleInfoFUCHSIABuilder(Default::default(), std::marker::PhantomData)
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
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> SemaphoreGetZirconHandleInfoFUCHSIA {
        self.0
    }
}
impl<'a> std::default::Default for SemaphoreGetZirconHandleInfoFUCHSIABuilder<'a> {
    fn default() -> SemaphoreGetZirconHandleInfoFUCHSIABuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for SemaphoreGetZirconHandleInfoFUCHSIABuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for SemaphoreGetZirconHandleInfoFUCHSIABuilder<'a> {
    type Target = SemaphoreGetZirconHandleInfoFUCHSIA;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SemaphoreGetZirconHandleInfoFUCHSIABuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::fuchsia_external_semaphore`]"]
impl crate::DeviceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSemaphoreZirconHandleFUCHSIA.html) · Function"]
    #[doc(alias = "vkGetSemaphoreZirconHandleFUCHSIA")]
    pub unsafe fn get_semaphore_zircon_handle_fuchsia(&self, get_zircon_handle_info: &crate::extensions::fuchsia_external_semaphore::SemaphoreGetZirconHandleInfoFUCHSIA) -> crate::utils::VulkanResult<*mut std::ffi::c_void> {
        let _function = self.get_semaphore_zircon_handle_fuchsia.expect(crate::NOT_LOADED_MESSAGE);
        let mut zircon_handle = std::ptr::null_mut();
        let _return = _function(self.handle, get_zircon_handle_info as _, &mut zircon_handle);
        crate::utils::VulkanResult::new(_return, zircon_handle)
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkImportSemaphoreZirconHandleFUCHSIA.html) · Function"]
    #[doc(alias = "vkImportSemaphoreZirconHandleFUCHSIA")]
    pub unsafe fn import_semaphore_zircon_handle_fuchsia(&self, import_semaphore_zircon_handle_info: &crate::extensions::fuchsia_external_semaphore::ImportSemaphoreZirconHandleInfoFUCHSIA) -> crate::utils::VulkanResult<()> {
        let _function = self.import_semaphore_zircon_handle_fuchsia.expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(self.handle, import_semaphore_zircon_handle_info as _);
        crate::utils::VulkanResult::new(_return, ())
    }
}
