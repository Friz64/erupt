#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_FUCHSIA_EXTERNAL_MEMORY_SPEC_VERSION")]
pub const FUCHSIA_EXTERNAL_MEMORY_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_FUCHSIA_EXTERNAL_MEMORY_EXTENSION_NAME")]
pub const FUCHSIA_EXTERNAL_MEMORY_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_FUCHSIA_external_memory");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_MEMORY_ZIRCON_HANDLE_FUCHSIA: *const std::os::raw::c_char = crate::cstr!("vkGetMemoryZirconHandleFUCHSIA");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_MEMORY_ZIRCON_HANDLE_PROPERTIES_FUCHSIA: *const std::os::raw::c_char = crate::cstr!("vkGetMemoryZirconHandlePropertiesFUCHSIA");
#[doc = "Provided by [`crate::extensions::fuchsia_external_memory`]"]
impl crate::vk1_0::StructureType {
    pub const IMPORT_MEMORY_ZIRCON_HANDLE_INFO_FUCHSIA: Self = Self(1000364000);
    pub const MEMORY_ZIRCON_HANDLE_PROPERTIES_FUCHSIA: Self = Self(1000364001);
    pub const MEMORY_GET_ZIRCON_HANDLE_INFO_FUCHSIA: Self = Self(1000364002);
}
#[doc = "Provided by [`crate::extensions::fuchsia_external_memory`]"]
impl crate::vk1_1::ExternalMemoryHandleTypeFlagBits {
    pub const ZIRCON_VMO_FUCHSIA: Self = Self(2048);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryZirconHandleFUCHSIA.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetMemoryZirconHandleFUCHSIA = unsafe extern "system" fn(device: crate::vk1_0::Device, p_get_zircon_handle_info: *const crate::extensions::fuchsia_external_memory::MemoryGetZirconHandleInfoFUCHSIA, p_zircon_handle: *mut *mut std::ffi::c_void) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryZirconHandlePropertiesFUCHSIA.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetMemoryZirconHandlePropertiesFUCHSIA = unsafe extern "system" fn(device: crate::vk1_0::Device, handle_type: crate::vk1_1::ExternalMemoryHandleTypeFlagBits, zircon_handle: *mut std::ffi::c_void, p_memory_zircon_handle_properties: *mut crate::extensions::fuchsia_external_memory::MemoryZirconHandlePropertiesFUCHSIA) -> crate::vk1_0::Result;
impl<'a> crate::ExtendableFrom<'a, ImportMemoryZirconHandleInfoFUCHSIA> for crate::vk1_0::MemoryAllocateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, ImportMemoryZirconHandleInfoFUCHSIABuilder<'_>> for crate::vk1_0::MemoryAllocateInfoBuilder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImportMemoryZirconHandleInfoFUCHSIA.html) · Structure"]
#[doc(alias = "VkImportMemoryZirconHandleInfoFUCHSIA")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImportMemoryZirconHandleInfoFUCHSIA {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub handle_type: crate::vk1_1::ExternalMemoryHandleTypeFlagBits,
    pub handle: *mut std::ffi::c_void,
}
impl ImportMemoryZirconHandleInfoFUCHSIA {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::IMPORT_MEMORY_ZIRCON_HANDLE_INFO_FUCHSIA;
}
impl Default for ImportMemoryZirconHandleInfoFUCHSIA {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), handle_type: Default::default(), handle: std::ptr::null_mut() }
    }
}
impl std::fmt::Debug for ImportMemoryZirconHandleInfoFUCHSIA {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ImportMemoryZirconHandleInfoFUCHSIA").field("s_type", &self.s_type).field("p_next", &self.p_next).field("handle_type", &self.handle_type).field("handle", &self.handle).finish()
    }
}
impl ImportMemoryZirconHandleInfoFUCHSIA {
    #[inline]
    pub fn into_builder<'a>(self) -> ImportMemoryZirconHandleInfoFUCHSIABuilder<'a> {
        ImportMemoryZirconHandleInfoFUCHSIABuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImportMemoryZirconHandleInfoFUCHSIA.html) · Builder of [`ImportMemoryZirconHandleInfoFUCHSIA`]"]
#[repr(transparent)]
pub struct ImportMemoryZirconHandleInfoFUCHSIABuilder<'a>(ImportMemoryZirconHandleInfoFUCHSIA, std::marker::PhantomData<&'a ()>);
impl<'a> ImportMemoryZirconHandleInfoFUCHSIABuilder<'a> {
    #[inline]
    pub fn new() -> ImportMemoryZirconHandleInfoFUCHSIABuilder<'a> {
        ImportMemoryZirconHandleInfoFUCHSIABuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn handle_type(mut self, handle_type: crate::vk1_1::ExternalMemoryHandleTypeFlagBits) -> Self {
        self.0.handle_type = handle_type as _;
        self
    }
    #[inline]
    pub fn handle(mut self, handle: *mut std::ffi::c_void) -> Self {
        self.0.handle = handle;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> ImportMemoryZirconHandleInfoFUCHSIA {
        self.0
    }
}
impl<'a> std::default::Default for ImportMemoryZirconHandleInfoFUCHSIABuilder<'a> {
    fn default() -> ImportMemoryZirconHandleInfoFUCHSIABuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ImportMemoryZirconHandleInfoFUCHSIABuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ImportMemoryZirconHandleInfoFUCHSIABuilder<'a> {
    type Target = ImportMemoryZirconHandleInfoFUCHSIA;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ImportMemoryZirconHandleInfoFUCHSIABuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryZirconHandlePropertiesFUCHSIA.html) · Structure"]
#[doc(alias = "VkMemoryZirconHandlePropertiesFUCHSIA")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryZirconHandlePropertiesFUCHSIA {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub memory_type_bits: u32,
}
impl MemoryZirconHandlePropertiesFUCHSIA {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::MEMORY_ZIRCON_HANDLE_PROPERTIES_FUCHSIA;
}
impl Default for MemoryZirconHandlePropertiesFUCHSIA {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), memory_type_bits: Default::default() }
    }
}
impl std::fmt::Debug for MemoryZirconHandlePropertiesFUCHSIA {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MemoryZirconHandlePropertiesFUCHSIA").field("s_type", &self.s_type).field("p_next", &self.p_next).field("memory_type_bits", &self.memory_type_bits).finish()
    }
}
impl MemoryZirconHandlePropertiesFUCHSIA {
    #[inline]
    pub fn into_builder<'a>(self) -> MemoryZirconHandlePropertiesFUCHSIABuilder<'a> {
        MemoryZirconHandlePropertiesFUCHSIABuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryZirconHandlePropertiesFUCHSIA.html) · Builder of [`MemoryZirconHandlePropertiesFUCHSIA`]"]
#[repr(transparent)]
pub struct MemoryZirconHandlePropertiesFUCHSIABuilder<'a>(MemoryZirconHandlePropertiesFUCHSIA, std::marker::PhantomData<&'a ()>);
impl<'a> MemoryZirconHandlePropertiesFUCHSIABuilder<'a> {
    #[inline]
    pub fn new() -> MemoryZirconHandlePropertiesFUCHSIABuilder<'a> {
        MemoryZirconHandlePropertiesFUCHSIABuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn memory_type_bits(mut self, memory_type_bits: u32) -> Self {
        self.0.memory_type_bits = memory_type_bits as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> MemoryZirconHandlePropertiesFUCHSIA {
        self.0
    }
}
impl<'a> std::default::Default for MemoryZirconHandlePropertiesFUCHSIABuilder<'a> {
    fn default() -> MemoryZirconHandlePropertiesFUCHSIABuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for MemoryZirconHandlePropertiesFUCHSIABuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for MemoryZirconHandlePropertiesFUCHSIABuilder<'a> {
    type Target = MemoryZirconHandlePropertiesFUCHSIA;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for MemoryZirconHandlePropertiesFUCHSIABuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryGetZirconHandleInfoFUCHSIA.html) · Structure"]
#[doc(alias = "VkMemoryGetZirconHandleInfoFUCHSIA")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryGetZirconHandleInfoFUCHSIA {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub memory: crate::vk1_0::DeviceMemory,
    pub handle_type: crate::vk1_1::ExternalMemoryHandleTypeFlagBits,
}
impl MemoryGetZirconHandleInfoFUCHSIA {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::MEMORY_GET_ZIRCON_HANDLE_INFO_FUCHSIA;
}
impl Default for MemoryGetZirconHandleInfoFUCHSIA {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), memory: Default::default(), handle_type: Default::default() }
    }
}
impl std::fmt::Debug for MemoryGetZirconHandleInfoFUCHSIA {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MemoryGetZirconHandleInfoFUCHSIA").field("s_type", &self.s_type).field("p_next", &self.p_next).field("memory", &self.memory).field("handle_type", &self.handle_type).finish()
    }
}
impl MemoryGetZirconHandleInfoFUCHSIA {
    #[inline]
    pub fn into_builder<'a>(self) -> MemoryGetZirconHandleInfoFUCHSIABuilder<'a> {
        MemoryGetZirconHandleInfoFUCHSIABuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryGetZirconHandleInfoFUCHSIA.html) · Builder of [`MemoryGetZirconHandleInfoFUCHSIA`]"]
#[repr(transparent)]
pub struct MemoryGetZirconHandleInfoFUCHSIABuilder<'a>(MemoryGetZirconHandleInfoFUCHSIA, std::marker::PhantomData<&'a ()>);
impl<'a> MemoryGetZirconHandleInfoFUCHSIABuilder<'a> {
    #[inline]
    pub fn new() -> MemoryGetZirconHandleInfoFUCHSIABuilder<'a> {
        MemoryGetZirconHandleInfoFUCHSIABuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn memory(mut self, memory: crate::vk1_0::DeviceMemory) -> Self {
        self.0.memory = memory as _;
        self
    }
    #[inline]
    pub fn handle_type(mut self, handle_type: crate::vk1_1::ExternalMemoryHandleTypeFlagBits) -> Self {
        self.0.handle_type = handle_type as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> MemoryGetZirconHandleInfoFUCHSIA {
        self.0
    }
}
impl<'a> std::default::Default for MemoryGetZirconHandleInfoFUCHSIABuilder<'a> {
    fn default() -> MemoryGetZirconHandleInfoFUCHSIABuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for MemoryGetZirconHandleInfoFUCHSIABuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for MemoryGetZirconHandleInfoFUCHSIABuilder<'a> {
    type Target = MemoryGetZirconHandleInfoFUCHSIA;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for MemoryGetZirconHandleInfoFUCHSIABuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::fuchsia_external_memory`]"]
impl crate::DeviceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryZirconHandleFUCHSIA.html) · Function"]
    #[doc(alias = "vkGetMemoryZirconHandleFUCHSIA")]
    pub unsafe fn get_memory_zircon_handle_fuchsia(&self, get_zircon_handle_info: &crate::extensions::fuchsia_external_memory::MemoryGetZirconHandleInfoFUCHSIA, zircon_handle: *mut *mut std::ffi::c_void) -> crate::utils::VulkanResult<()> {
        let _function = self.get_memory_zircon_handle_fuchsia.expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(self.handle, get_zircon_handle_info as _, zircon_handle);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryZirconHandlePropertiesFUCHSIA.html) · Function"]
    #[doc(alias = "vkGetMemoryZirconHandlePropertiesFUCHSIA")]
    pub unsafe fn get_memory_zircon_handle_properties_fuchsia(&self, handle_type: crate::vk1_1::ExternalMemoryHandleTypeFlagBits, zircon_handle: *mut std::ffi::c_void, memory_zircon_handle_properties: Option<crate::extensions::fuchsia_external_memory::MemoryZirconHandlePropertiesFUCHSIA>) -> crate::utils::VulkanResult<crate::extensions::fuchsia_external_memory::MemoryZirconHandlePropertiesFUCHSIA> {
        let _function = self.get_memory_zircon_handle_properties_fuchsia.expect(crate::NOT_LOADED_MESSAGE);
        let mut memory_zircon_handle_properties = match memory_zircon_handle_properties {
            Some(v) => v,
            None => Default::default(),
        };
        let _return = _function(self.handle, handle_type as _, zircon_handle, &mut memory_zircon_handle_properties);
        crate::utils::VulkanResult::new(_return, memory_zircon_handle_properties)
    }
}
