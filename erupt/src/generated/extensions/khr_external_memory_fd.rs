# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_KHR_external_memory_fd.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_EXTERNAL_MEMORY_FD_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_EXTERNAL_MEMORY_FD_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_external_memory_fd");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryFdKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetMemoryFdKHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_get_fd_info: *const crate::extensions::khr_external_memory_fd::MemoryGetFdInfoKHR,
    p_fd: *mut std::os::raw::c_int,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryFdPropertiesKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetMemoryFdPropertiesKHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    handle_type: crate::vk1_1::ExternalMemoryHandleTypeFlagBits,
    fd: std::os::raw::c_int,
    p_memory_fd_properties: *mut crate::extensions::khr_external_memory_fd::MemoryFdPropertiesKHR,
) -> crate::vk1_0::Result;
#[doc = "Provides Device Commands for [`KhrExternalMemoryFdDeviceLoaderExt`](trait.KhrExternalMemoryFdDeviceLoaderExt.html)"]
pub struct KhrExternalMemoryFdDeviceCommands {
    pub get_memory_fd_khr: PFN_vkGetMemoryFdKHR,
    pub get_memory_fd_properties_khr: PFN_vkGetMemoryFdPropertiesKHR,
}
impl KhrExternalMemoryFdDeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<KhrExternalMemoryFdDeviceCommands> {
        unsafe {
            Some(KhrExternalMemoryFdDeviceCommands {
                get_memory_fd_khr: std::mem::transmute(loader.symbol("vkGetMemoryFdKHR")?),
                get_memory_fd_properties_khr: std::mem::transmute(
                    loader.symbol("vkGetMemoryFdPropertiesKHR")?,
                ),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`KhrExternalMemoryFdDeviceCommands`](struct.KhrExternalMemoryFdDeviceCommands.html)"]
pub trait KhrExternalMemoryFdDeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryFdKHR.html) · Device Command"]
    unsafe fn get_memory_fd_khr(
        &self,
        get_fd_info: &crate::extensions::khr_external_memory_fd::MemoryGetFdInfoKHR,
        fd: Option<std::os::raw::c_int>,
    ) -> crate::utils::VulkanResult<std::os::raw::c_int>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryFdPropertiesKHR.html) · Device Command"]
    unsafe fn get_memory_fd_properties_khr(
        &self,
        handle_type: crate::vk1_1::ExternalMemoryHandleTypeFlagBits,
        fd: std::os::raw::c_int,
        memory_fd_properties: Option<
            crate::extensions::khr_external_memory_fd::MemoryFdPropertiesKHR,
        >,
    ) -> crate::utils::VulkanResult<crate::extensions::khr_external_memory_fd::MemoryFdPropertiesKHR>;
}
impl KhrExternalMemoryFdDeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryFdKHR.html) · Device Command"]
    unsafe fn get_memory_fd_khr(
        &self,
        get_fd_info: &crate::extensions::khr_external_memory_fd::MemoryGetFdInfoKHR,
        fd: Option<std::os::raw::c_int>,
    ) -> crate::utils::VulkanResult<std::os::raw::c_int> {
        let function = self
            .khr_external_memory_fd
            .as_ref()
            .expect("`khr_external_memory_fd` not loaded")
            .get_memory_fd_khr;
        let mut fd = fd.unwrap_or_else(|| Default::default());
        let _val = function(self.handle, get_fd_info, &mut fd);
        crate::utils::VulkanResult::new(_val, fd)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryFdPropertiesKHR.html) · Device Command"]
    unsafe fn get_memory_fd_properties_khr(
        &self,
        handle_type: crate::vk1_1::ExternalMemoryHandleTypeFlagBits,
        fd: std::os::raw::c_int,
        memory_fd_properties: Option<
            crate::extensions::khr_external_memory_fd::MemoryFdPropertiesKHR,
        >,
    ) -> crate::utils::VulkanResult<crate::extensions::khr_external_memory_fd::MemoryFdPropertiesKHR>
    {
        let function = self
            .khr_external_memory_fd
            .as_ref()
            .expect("`khr_external_memory_fd` not loaded")
            .get_memory_fd_properties_khr;
        let mut memory_fd_properties = memory_fd_properties.unwrap_or_else(|| Default::default());
        let _val = function(self.handle, handle_type, fd, &mut memory_fd_properties);
        crate::utils::VulkanResult::new(_val, memory_fd_properties)
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryGetFdInfoKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryGetFdInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub memory: crate::vk1_0::DeviceMemory,
    pub handle_type: crate::vk1_1::ExternalMemoryHandleTypeFlagBits,
}
impl MemoryGetFdInfoKHR {
    #[inline]
    pub fn builder<'a>(self) -> MemoryGetFdInfoKHRBuilder<'a> {
        MemoryGetFdInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for MemoryGetFdInfoKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("MemoryGetFdInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("memory", &self.memory)
            .field("handle_type", &self.handle_type)
            .finish()
    }
}
impl Default for MemoryGetFdInfoKHR {
    fn default() -> MemoryGetFdInfoKHR {
        MemoryGetFdInfoKHR {
            s_type: crate::vk1_0::StructureType::MEMORY_GET_FD_INFO_KHR,
            p_next: std::ptr::null(),
            memory: Default::default(),
            handle_type: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`MemoryGetFdInfoKHR`](struct.MemoryGetFdInfoKHR.html)"]
#[repr(transparent)]
pub struct MemoryGetFdInfoKHRBuilder<'a>(MemoryGetFdInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> MemoryGetFdInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> MemoryGetFdInfoKHRBuilder<'a> {
        MemoryGetFdInfoKHRBuilder(Default::default(), std::marker::PhantomData)
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
    pub unsafe fn discard(self) -> MemoryGetFdInfoKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for MemoryGetFdInfoKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for MemoryGetFdInfoKHRBuilder<'a> {
    type Target = MemoryGetFdInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for MemoryGetFdInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryFdPropertiesKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryFdPropertiesKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub memory_type_bits: u32,
}
impl MemoryFdPropertiesKHR {
    #[inline]
    pub fn builder<'a>(self) -> MemoryFdPropertiesKHRBuilder<'a> {
        MemoryFdPropertiesKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for MemoryFdPropertiesKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("MemoryFdPropertiesKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("memory_type_bits", &self.memory_type_bits)
            .finish()
    }
}
impl Default for MemoryFdPropertiesKHR {
    fn default() -> MemoryFdPropertiesKHR {
        MemoryFdPropertiesKHR {
            s_type: crate::vk1_0::StructureType::MEMORY_FD_PROPERTIES_KHR,
            p_next: std::ptr::null_mut(),
            memory_type_bits: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`MemoryFdPropertiesKHR`](struct.MemoryFdPropertiesKHR.html)"]
#[repr(transparent)]
pub struct MemoryFdPropertiesKHRBuilder<'a>(
    MemoryFdPropertiesKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> MemoryFdPropertiesKHRBuilder<'a> {
    #[inline]
    pub fn new() -> MemoryFdPropertiesKHRBuilder<'a> {
        MemoryFdPropertiesKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn memory_type_bits(mut self, memory_type_bits: u32) -> Self {
        self.0.memory_type_bits = memory_type_bits;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> MemoryFdPropertiesKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for MemoryFdPropertiesKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for MemoryFdPropertiesKHRBuilder<'a> {
    type Target = MemoryFdPropertiesKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for MemoryFdPropertiesKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImportMemoryFdInfoKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImportMemoryFdInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub handle_type: crate::vk1_1::ExternalMemoryHandleTypeFlagBits,
    pub fd: std::os::raw::c_int,
}
impl ImportMemoryFdInfoKHR {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByImportMemoryFdInfoKHR,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> ImportMemoryFdInfoKHRBuilder<'a> {
        ImportMemoryFdInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ImportMemoryFdInfoKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ImportMemoryFdInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("handle_type", &self.handle_type)
            .field("fd", &self.fd)
            .finish()
    }
}
impl Default for ImportMemoryFdInfoKHR {
    fn default() -> ImportMemoryFdInfoKHR {
        ImportMemoryFdInfoKHR {
            s_type: crate::vk1_0::StructureType::IMPORT_MEMORY_FD_INFO_KHR,
            p_next: std::ptr::null(),
            handle_type: Default::default(),
            fd: Default::default(),
        }
    }
}
#[doc = "Used by [`ImportMemoryFdInfoKHR::extend`](struct.ImportMemoryFdInfoKHR.html#method.extend)"]
pub trait ExtendableByImportMemoryFdInfoKHR {}
impl ExtendableByImportMemoryFdInfoKHR for crate::vk1_0::MemoryAllocateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`ImportMemoryFdInfoKHR`](struct.ImportMemoryFdInfoKHR.html)"]
#[repr(transparent)]
pub struct ImportMemoryFdInfoKHRBuilder<'a>(
    ImportMemoryFdInfoKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> ImportMemoryFdInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> ImportMemoryFdInfoKHRBuilder<'a> {
        ImportMemoryFdInfoKHRBuilder(Default::default(), std::marker::PhantomData)
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
    pub fn fd(mut self, fd: std::os::raw::c_int) -> Self {
        self.0.fd = fd;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> ImportMemoryFdInfoKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for ImportMemoryFdInfoKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for ImportMemoryFdInfoKHRBuilder<'a> {
    type Target = ImportMemoryFdInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ImportMemoryFdInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
