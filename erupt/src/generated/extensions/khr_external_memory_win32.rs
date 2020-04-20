# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_KHR_external_memory_win32.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_EXTERNAL_MEMORY_WIN32_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_external_memory_win32");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryWin32HandleKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetMemoryWin32HandleKHR = unsafe extern "system" fn ( device : crate :: vk1_0 :: Device , p_get_win32_handle_info : * const crate :: extensions :: khr_external_memory_win32 :: MemoryGetWin32HandleInfoKHR , p_handle : * mut * mut std :: ffi :: c_void , ) -> crate :: vk1_0 :: Result ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryWin32HandlePropertiesKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetMemoryWin32HandlePropertiesKHR = unsafe extern "system" fn ( device : crate :: vk1_0 :: Device , handle_type : crate :: vk1_1 :: ExternalMemoryHandleTypeFlagBits , handle : * mut std :: ffi :: c_void , p_memory_win32_handle_properties : * mut crate :: extensions :: khr_external_memory_win32 :: MemoryWin32HandlePropertiesKHR , ) -> crate :: vk1_0 :: Result ;
#[doc = "Provides Device Commands for [`KhrExternalMemoryWin32DeviceLoaderExt`](trait.KhrExternalMemoryWin32DeviceLoaderExt.html)"]
pub struct KhrExternalMemoryWin32DeviceCommands {
    pub get_memory_win32_handle_khr: PFN_vkGetMemoryWin32HandleKHR,
    pub get_memory_win32_handle_properties_khr: PFN_vkGetMemoryWin32HandlePropertiesKHR,
}
impl KhrExternalMemoryWin32DeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<KhrExternalMemoryWin32DeviceCommands> {
        unsafe {
            Some(KhrExternalMemoryWin32DeviceCommands {
                get_memory_win32_handle_khr: std::mem::transmute(
                    loader.symbol("vkGetMemoryWin32HandleKHR")?,
                ),
                get_memory_win32_handle_properties_khr: std::mem::transmute(
                    loader.symbol("vkGetMemoryWin32HandlePropertiesKHR")?,
                ),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`KhrExternalMemoryWin32DeviceCommands`](struct.KhrExternalMemoryWin32DeviceCommands.html)"]
pub trait KhrExternalMemoryWin32DeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryWin32HandleKHR.html) · Device Command"]
    unsafe fn get_memory_win32_handle_khr(
        &self,
        get_win32_handle_info : & crate :: extensions :: khr_external_memory_win32 :: MemoryGetWin32HandleInfoKHR,
        handle: *mut *mut std::ffi::c_void,
    ) -> crate::utils::VulkanResult<()>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryWin32HandlePropertiesKHR.html) · Device Command"]
    unsafe fn get_memory_win32_handle_properties_khr(
        &self,
        handle_type: crate::vk1_1::ExternalMemoryHandleTypeFlagBits,
        handle: *mut std::ffi::c_void,
        memory_win32_handle_properties: Option<
            crate::extensions::khr_external_memory_win32::MemoryWin32HandlePropertiesKHR,
        >,
    ) -> crate::utils::VulkanResult<
        crate::extensions::khr_external_memory_win32::MemoryWin32HandlePropertiesKHR,
    >;
}
impl KhrExternalMemoryWin32DeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryWin32HandleKHR.html) · Device Command"]
    unsafe fn get_memory_win32_handle_khr(
        &self,
        get_win32_handle_info : & crate :: extensions :: khr_external_memory_win32 :: MemoryGetWin32HandleInfoKHR,
        handle: *mut *mut std::ffi::c_void,
    ) -> crate::utils::VulkanResult<()> {
        let function = self
            .khr_external_memory_win32
            .as_ref()
            .expect("`khr_external_memory_win32` not loaded")
            .get_memory_win32_handle_khr;
        let _val = function(self.handle, get_win32_handle_info, handle);
        crate::utils::VulkanResult::new(_val, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryWin32HandlePropertiesKHR.html) · Device Command"]
    unsafe fn get_memory_win32_handle_properties_khr(
        &self,
        handle_type: crate::vk1_1::ExternalMemoryHandleTypeFlagBits,
        handle: *mut std::ffi::c_void,
        memory_win32_handle_properties: Option<
            crate::extensions::khr_external_memory_win32::MemoryWin32HandlePropertiesKHR,
        >,
    ) -> crate::utils::VulkanResult<
        crate::extensions::khr_external_memory_win32::MemoryWin32HandlePropertiesKHR,
    > {
        let function = self
            .khr_external_memory_win32
            .as_ref()
            .expect("`khr_external_memory_win32` not loaded")
            .get_memory_win32_handle_properties_khr;
        let mut memory_win32_handle_properties =
            memory_win32_handle_properties.unwrap_or_else(|| Default::default());
        let _val = function(
            self.handle,
            handle_type,
            handle,
            &mut memory_win32_handle_properties,
        );
        crate::utils::VulkanResult::new(_val, memory_win32_handle_properties)
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
impl MemoryGetWin32HandleInfoKHR {
    #[inline]
    pub fn builder<'a>(self) -> MemoryGetWin32HandleInfoKHRBuilder<'a> {
        MemoryGetWin32HandleInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for MemoryGetWin32HandleInfoKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("MemoryGetWin32HandleInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("memory", &self.memory)
            .field("handle_type", &self.handle_type)
            .finish()
    }
}
impl Default for MemoryGetWin32HandleInfoKHR {
    fn default() -> MemoryGetWin32HandleInfoKHR {
        MemoryGetWin32HandleInfoKHR {
            s_type: crate::vk1_0::StructureType::MEMORY_GET_WIN32_HANDLE_INFO_KHR,
            p_next: std::ptr::null(),
            memory: Default::default(),
            handle_type: Default::default(),
        }
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
    #[allow(unused_mut)]
    #[inline]
    pub fn memory(mut self, memory: crate::vk1_0::DeviceMemory) -> Self {
        self.0.memory = memory;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn handle_type(
        mut self,
        handle_type: crate::vk1_1::ExternalMemoryHandleTypeFlagBits,
    ) -> Self {
        self.0.handle_type = handle_type;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> MemoryGetWin32HandleInfoKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for MemoryGetWin32HandleInfoKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryWin32HandlePropertiesKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryWin32HandlePropertiesKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub memory_type_bits: u32,
}
impl MemoryWin32HandlePropertiesKHR {
    #[inline]
    pub fn builder<'a>(self) -> MemoryWin32HandlePropertiesKHRBuilder<'a> {
        MemoryWin32HandlePropertiesKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for MemoryWin32HandlePropertiesKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("MemoryWin32HandlePropertiesKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("memory_type_bits", &self.memory_type_bits)
            .finish()
    }
}
impl Default for MemoryWin32HandlePropertiesKHR {
    fn default() -> MemoryWin32HandlePropertiesKHR {
        MemoryWin32HandlePropertiesKHR {
            s_type: crate::vk1_0::StructureType::MEMORY_WIN32_HANDLE_PROPERTIES_KHR,
            p_next: std::ptr::null_mut(),
            memory_type_bits: Default::default(),
        }
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
    #[allow(unused_mut)]
    #[inline]
    pub fn memory_type_bits(mut self, memory_type_bits: u32) -> Self {
        self.0.memory_type_bits = memory_type_bits;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> MemoryWin32HandlePropertiesKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for MemoryWin32HandlePropertiesKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
impl ImportMemoryWin32HandleInfoKHR {
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
    pub fn builder<'a>(self) -> ImportMemoryWin32HandleInfoKHRBuilder<'a> {
        ImportMemoryWin32HandleInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ImportMemoryWin32HandleInfoKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ImportMemoryWin32HandleInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("handle_type", &self.handle_type)
            .field("handle", &self.handle)
            .field("name", &self.name)
            .finish()
    }
}
impl Default for ImportMemoryWin32HandleInfoKHR {
    fn default() -> ImportMemoryWin32HandleInfoKHR {
        ImportMemoryWin32HandleInfoKHR {
            s_type: crate::vk1_0::StructureType::IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR,
            p_next: std::ptr::null(),
            handle_type: Default::default(),
            handle: std::ptr::null_mut(),
            name: std::ptr::null(),
        }
    }
}
impl crate::ExtendableBy<ImportMemoryWin32HandleInfoKHR> for crate::vk1_0::MemoryAllocateInfo {}
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
    #[allow(unused_mut)]
    #[inline]
    pub fn handle_type(
        mut self,
        handle_type: crate::vk1_1::ExternalMemoryHandleTypeFlagBits,
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
    pub unsafe fn discard(self) -> ImportMemoryWin32HandleInfoKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for ImportMemoryWin32HandleInfoKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
    pub p_attributes: *const crate::SECURITY_ATTRIBUTES,
    pub dw_access: u32,
    pub name: *const u16,
}
impl ExportMemoryWin32HandleInfoKHR {
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
    pub fn builder<'a>(self) -> ExportMemoryWin32HandleInfoKHRBuilder<'a> {
        ExportMemoryWin32HandleInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ExportMemoryWin32HandleInfoKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ExportMemoryWin32HandleInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("p_attributes", &self.p_attributes)
            .field("dw_access", &self.dw_access)
            .field("name", &self.name)
            .finish()
    }
}
impl Default for ExportMemoryWin32HandleInfoKHR {
    fn default() -> ExportMemoryWin32HandleInfoKHR {
        ExportMemoryWin32HandleInfoKHR {
            s_type: crate::vk1_0::StructureType::EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR,
            p_next: std::ptr::null(),
            p_attributes: std::ptr::null(),
            dw_access: Default::default(),
            name: std::ptr::null(),
        }
    }
}
impl crate::ExtendableBy<ExportMemoryWin32HandleInfoKHR> for crate::vk1_0::MemoryAllocateInfo {}
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
    pub unsafe fn discard(self) -> ExportMemoryWin32HandleInfoKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for ExportMemoryWin32HandleInfoKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
