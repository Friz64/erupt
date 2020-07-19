#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_EXTERNAL_MEMORY_WIN32_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_external_memory_win32");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_MEMORY_WIN32_HANDLE_KHR: *const std::os::raw::c_char =
    crate::cstr!("vkGetMemoryWin32HandleKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_MEMORY_WIN32_HANDLE_PROPERTIES_KHR: *const std::os::raw::c_char =
    crate::cstr!("vkGetMemoryWin32HandlePropertiesKHR");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryWin32HandleKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetMemoryWin32HandleKHR = unsafe extern "system" fn ( device : crate :: vk1_0 :: Device , p_get_win32_handle_info : * const crate :: extensions :: khr_external_memory_win32 :: MemoryGetWin32HandleInfoKHR , p_handle : * mut * mut std :: ffi :: c_void ) -> crate :: vk1_0 :: Result ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryWin32HandlePropertiesKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetMemoryWin32HandlePropertiesKHR = unsafe extern "system" fn ( device : crate :: vk1_0 :: Device , handle_type : crate :: vk1_1 :: ExternalMemoryHandleTypeFlagBits , handle : * mut std :: ffi :: c_void , p_memory_win32_handle_properties : * mut crate :: extensions :: khr_external_memory_win32 :: MemoryWin32HandlePropertiesKHR ) -> crate :: vk1_0 :: Result ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImportMemoryWin32HandleInfoKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImportMemoryWin32HandleInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub handle_type: crate::vk1_1::ExternalMemoryHandleTypeFlagBits,
    pub handle: *mut std::ffi::c_void,
    pub name: *const u16,
}
impl Default for ImportMemoryWin32HandleInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR,
            p_next: std::ptr::null(),
            handle_type: Default::default(),
            handle: std::ptr::null_mut(),
            name: std::ptr::null(),
        }
    }
}
impl std::fmt::Debug for ImportMemoryWin32HandleInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ImportMemoryWin32HandleInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("handle_type", &self.handle_type)
            .field("handle", &self.handle)
            .field("name", &self.name)
            .finish()
    }
}
impl ImportMemoryWin32HandleInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> ImportMemoryWin32HandleInfoKHRBuilder<'a> {
        ImportMemoryWin32HandleInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImportMemoryWin32HandleInfoKHR.html) · Builder of [`ImportMemoryWin32HandleInfoKHR`](struct.ImportMemoryWin32HandleInfoKHR.html)"]
#[repr(transparent)]
pub struct ImportMemoryWin32HandleInfoKHRBuilder<'a>(
    ImportMemoryWin32HandleInfoKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> ImportMemoryWin32HandleInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> ImportMemoryWin32HandleInfoKHRBuilder<'a> {
        ImportMemoryWin32HandleInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn handle_type(
        mut self,
        handle_type: crate::vk1_1::ExternalMemoryHandleTypeFlagBits,
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
    pub fn build(self) -> ImportMemoryWin32HandleInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for ImportMemoryWin32HandleInfoKHRBuilder<'a> {
    fn default() -> ImportMemoryWin32HandleInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ImportMemoryWin32HandleInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ImportMemoryWin32HandleInfoKHRBuilder<'a> {
    type Target = ImportMemoryWin32HandleInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ImportMemoryWin32HandleInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExportMemoryWin32HandleInfoKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExportMemoryWin32HandleInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub p_attributes: *const std::ffi::c_void,
    pub dw_access: u32,
    pub name: *const u16,
}
impl Default for ExportMemoryWin32HandleInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR,
            p_next: std::ptr::null(),
            p_attributes: std::ptr::null(),
            dw_access: Default::default(),
            name: std::ptr::null(),
        }
    }
}
impl std::fmt::Debug for ExportMemoryWin32HandleInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ExportMemoryWin32HandleInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("p_attributes", &self.p_attributes)
            .field("dw_access", &self.dw_access)
            .field("name", &self.name)
            .finish()
    }
}
impl ExportMemoryWin32HandleInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> ExportMemoryWin32HandleInfoKHRBuilder<'a> {
        ExportMemoryWin32HandleInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExportMemoryWin32HandleInfoKHR.html) · Builder of [`ExportMemoryWin32HandleInfoKHR`](struct.ExportMemoryWin32HandleInfoKHR.html)"]
#[repr(transparent)]
pub struct ExportMemoryWin32HandleInfoKHRBuilder<'a>(
    ExportMemoryWin32HandleInfoKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> ExportMemoryWin32HandleInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> ExportMemoryWin32HandleInfoKHRBuilder<'a> {
        ExportMemoryWin32HandleInfoKHRBuilder(Default::default(), std::marker::PhantomData)
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
    pub fn build(self) -> ExportMemoryWin32HandleInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for ExportMemoryWin32HandleInfoKHRBuilder<'a> {
    fn default() -> ExportMemoryWin32HandleInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ExportMemoryWin32HandleInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ExportMemoryWin32HandleInfoKHRBuilder<'a> {
    type Target = ExportMemoryWin32HandleInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ExportMemoryWin32HandleInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryWin32HandlePropertiesKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryWin32HandlePropertiesKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub memory_type_bits: u32,
}
impl Default for MemoryWin32HandlePropertiesKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::MEMORY_WIN32_HANDLE_PROPERTIES_KHR,
            p_next: std::ptr::null_mut(),
            memory_type_bits: Default::default(),
        }
    }
}
impl std::fmt::Debug for MemoryWin32HandlePropertiesKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MemoryWin32HandlePropertiesKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("memory_type_bits", &self.memory_type_bits)
            .finish()
    }
}
impl MemoryWin32HandlePropertiesKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> MemoryWin32HandlePropertiesKHRBuilder<'a> {
        MemoryWin32HandlePropertiesKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryWin32HandlePropertiesKHR.html) · Builder of [`MemoryWin32HandlePropertiesKHR`](struct.MemoryWin32HandlePropertiesKHR.html)"]
#[repr(transparent)]
pub struct MemoryWin32HandlePropertiesKHRBuilder<'a>(
    MemoryWin32HandlePropertiesKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> MemoryWin32HandlePropertiesKHRBuilder<'a> {
    #[inline]
    pub fn new() -> MemoryWin32HandlePropertiesKHRBuilder<'a> {
        MemoryWin32HandlePropertiesKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn memory_type_bits(mut self, memory_type_bits: u32) -> Self {
        self.0.memory_type_bits = memory_type_bits as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> MemoryWin32HandlePropertiesKHR {
        self.0
    }
}
impl<'a> std::default::Default for MemoryWin32HandlePropertiesKHRBuilder<'a> {
    fn default() -> MemoryWin32HandlePropertiesKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for MemoryWin32HandlePropertiesKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for MemoryWin32HandlePropertiesKHRBuilder<'a> {
    type Target = MemoryWin32HandlePropertiesKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for MemoryWin32HandlePropertiesKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryGetWin32HandleInfoKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryGetWin32HandleInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub memory: crate::vk1_0::DeviceMemory,
    pub handle_type: crate::vk1_1::ExternalMemoryHandleTypeFlagBits,
}
impl Default for MemoryGetWin32HandleInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::MEMORY_GET_WIN32_HANDLE_INFO_KHR,
            p_next: std::ptr::null(),
            memory: Default::default(),
            handle_type: Default::default(),
        }
    }
}
impl std::fmt::Debug for MemoryGetWin32HandleInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MemoryGetWin32HandleInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("memory", &self.memory)
            .field("handle_type", &self.handle_type)
            .finish()
    }
}
impl MemoryGetWin32HandleInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> MemoryGetWin32HandleInfoKHRBuilder<'a> {
        MemoryGetWin32HandleInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryGetWin32HandleInfoKHR.html) · Builder of [`MemoryGetWin32HandleInfoKHR`](struct.MemoryGetWin32HandleInfoKHR.html)"]
#[repr(transparent)]
pub struct MemoryGetWin32HandleInfoKHRBuilder<'a>(
    MemoryGetWin32HandleInfoKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> MemoryGetWin32HandleInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> MemoryGetWin32HandleInfoKHRBuilder<'a> {
        MemoryGetWin32HandleInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn memory(mut self, memory: crate::vk1_0::DeviceMemory) -> Self {
        self.0.memory = memory as _;
        self
    }
    #[inline]
    pub fn handle_type(
        mut self,
        handle_type: crate::vk1_1::ExternalMemoryHandleTypeFlagBits,
    ) -> Self {
        self.0.handle_type = handle_type as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> MemoryGetWin32HandleInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for MemoryGetWin32HandleInfoKHRBuilder<'a> {
    fn default() -> MemoryGetWin32HandleInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for MemoryGetWin32HandleInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for MemoryGetWin32HandleInfoKHRBuilder<'a> {
    type Target = MemoryGetWin32HandleInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for MemoryGetWin32HandleInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`extensions::khr_external_memory_win32`](extensions/khr_external_memory_win32/index.html)"]
impl crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryWin32HandleKHR.html) · Function"]
    pub unsafe fn get_memory_win32_handle_khr(
        &self,
        get_win32_handle_info : & crate :: extensions :: khr_external_memory_win32 :: MemoryGetWin32HandleInfoKHR,
        handle: *mut *mut std::ffi::c_void,
    ) -> crate::utils::VulkanResult<()> {
        let _function = self
            .get_memory_win32_handle_khr
            .expect("`get_memory_win32_handle_khr` is not loaded");
        let _return = _function(self.handle, get_win32_handle_info as _, handle);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryWin32HandlePropertiesKHR.html) · Function"]
    pub unsafe fn get_memory_win32_handle_properties_khr(
        &self,
        handle_type: crate::vk1_1::ExternalMemoryHandleTypeFlagBits,
        handle: *mut std::ffi::c_void,
        memory_win32_handle_properties: Option<
            crate::extensions::khr_external_memory_win32::MemoryWin32HandlePropertiesKHR,
        >,
    ) -> crate::utils::VulkanResult<
        crate::extensions::khr_external_memory_win32::MemoryWin32HandlePropertiesKHR,
    > {
        let _function = self
            .get_memory_win32_handle_properties_khr
            .expect("`get_memory_win32_handle_properties_khr` is not loaded");
        let mut memory_win32_handle_properties = match memory_win32_handle_properties {
            Some(v) => v,
            None => Default::default(),
        };
        let _return = _function(
            self.handle,
            handle_type as _,
            handle,
            &mut memory_win32_handle_properties,
        );
        crate::utils::VulkanResult::new(_return, memory_win32_handle_properties)
    }
}
