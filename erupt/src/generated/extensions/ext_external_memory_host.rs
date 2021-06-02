#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_EXTERNAL_MEMORY_HOST_SPEC_VERSION")]
pub const EXT_EXTERNAL_MEMORY_HOST_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_EXTERNAL_MEMORY_HOST_EXTENSION_NAME")]
pub const EXT_EXTERNAL_MEMORY_HOST_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_external_memory_host");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_MEMORY_HOST_POINTER_PROPERTIES_EXT: *const std::os::raw::c_char = crate::cstr!("vkGetMemoryHostPointerPropertiesEXT");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryHostPointerPropertiesEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetMemoryHostPointerPropertiesEXT = unsafe extern "system" fn(device: crate::vk1_0::Device, handle_type: crate::vk1_1::ExternalMemoryHandleTypeFlagBits, p_host_pointer: *const std::ffi::c_void, p_memory_host_pointer_properties: *mut crate::extensions::ext_external_memory_host::MemoryHostPointerPropertiesEXT) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImportMemoryHostPointerInfoEXT.html) · Structure"]
#[doc(alias = "VkImportMemoryHostPointerInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImportMemoryHostPointerInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub handle_type: crate::vk1_1::ExternalMemoryHandleTypeFlagBits,
    pub p_host_pointer: *mut std::ffi::c_void,
}
impl Default for ImportMemoryHostPointerInfoEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::IMPORT_MEMORY_HOST_POINTER_INFO_EXT, p_next: std::ptr::null(), handle_type: Default::default(), p_host_pointer: std::ptr::null_mut() }
    }
}
impl std::fmt::Debug for ImportMemoryHostPointerInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ImportMemoryHostPointerInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("handle_type", &self.handle_type).field("p_host_pointer", &self.p_host_pointer).finish()
    }
}
impl ImportMemoryHostPointerInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> ImportMemoryHostPointerInfoEXTBuilder<'a> {
        ImportMemoryHostPointerInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImportMemoryHostPointerInfoEXT.html) · Builder of [`ImportMemoryHostPointerInfoEXT`]"]
#[repr(transparent)]
pub struct ImportMemoryHostPointerInfoEXTBuilder<'a>(ImportMemoryHostPointerInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> ImportMemoryHostPointerInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> ImportMemoryHostPointerInfoEXTBuilder<'a> {
        ImportMemoryHostPointerInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn handle_type(mut self, handle_type: crate::vk1_1::ExternalMemoryHandleTypeFlagBits) -> Self {
        self.0.handle_type = handle_type as _;
        self
    }
    #[inline]
    pub fn host_pointer(mut self, host_pointer: *mut std::ffi::c_void) -> Self {
        self.0.p_host_pointer = host_pointer;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ImportMemoryHostPointerInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for ImportMemoryHostPointerInfoEXTBuilder<'a> {
    fn default() -> ImportMemoryHostPointerInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ImportMemoryHostPointerInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ImportMemoryHostPointerInfoEXTBuilder<'a> {
    type Target = ImportMemoryHostPointerInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ImportMemoryHostPointerInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryHostPointerPropertiesEXT.html) · Structure"]
#[doc(alias = "VkMemoryHostPointerPropertiesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryHostPointerPropertiesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub memory_type_bits: u32,
}
impl Default for MemoryHostPointerPropertiesEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::MEMORY_HOST_POINTER_PROPERTIES_EXT, p_next: std::ptr::null_mut(), memory_type_bits: Default::default() }
    }
}
impl std::fmt::Debug for MemoryHostPointerPropertiesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MemoryHostPointerPropertiesEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("memory_type_bits", &self.memory_type_bits).finish()
    }
}
impl MemoryHostPointerPropertiesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> MemoryHostPointerPropertiesEXTBuilder<'a> {
        MemoryHostPointerPropertiesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryHostPointerPropertiesEXT.html) · Builder of [`MemoryHostPointerPropertiesEXT`]"]
#[repr(transparent)]
pub struct MemoryHostPointerPropertiesEXTBuilder<'a>(MemoryHostPointerPropertiesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> MemoryHostPointerPropertiesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> MemoryHostPointerPropertiesEXTBuilder<'a> {
        MemoryHostPointerPropertiesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn memory_type_bits(mut self, memory_type_bits: u32) -> Self {
        self.0.memory_type_bits = memory_type_bits as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> MemoryHostPointerPropertiesEXT {
        self.0
    }
}
impl<'a> std::default::Default for MemoryHostPointerPropertiesEXTBuilder<'a> {
    fn default() -> MemoryHostPointerPropertiesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for MemoryHostPointerPropertiesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for MemoryHostPointerPropertiesEXTBuilder<'a> {
    type Target = MemoryHostPointerPropertiesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for MemoryHostPointerPropertiesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceExternalMemoryHostPropertiesEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceExternalMemoryHostPropertiesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceExternalMemoryHostPropertiesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub min_imported_host_pointer_alignment: crate::vk1_0::DeviceSize,
}
impl Default for PhysicalDeviceExternalMemoryHostPropertiesEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT, p_next: std::ptr::null_mut(), min_imported_host_pointer_alignment: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceExternalMemoryHostPropertiesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceExternalMemoryHostPropertiesEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("min_imported_host_pointer_alignment", &self.min_imported_host_pointer_alignment).finish()
    }
}
impl PhysicalDeviceExternalMemoryHostPropertiesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceExternalMemoryHostPropertiesEXTBuilder<'a> {
        PhysicalDeviceExternalMemoryHostPropertiesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceExternalMemoryHostPropertiesEXT.html) · Builder of [`PhysicalDeviceExternalMemoryHostPropertiesEXT`]"]
#[repr(transparent)]
pub struct PhysicalDeviceExternalMemoryHostPropertiesEXTBuilder<'a>(PhysicalDeviceExternalMemoryHostPropertiesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceExternalMemoryHostPropertiesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceExternalMemoryHostPropertiesEXTBuilder<'a> {
        PhysicalDeviceExternalMemoryHostPropertiesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn min_imported_host_pointer_alignment(mut self, min_imported_host_pointer_alignment: crate::vk1_0::DeviceSize) -> Self {
        self.0.min_imported_host_pointer_alignment = min_imported_host_pointer_alignment as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceExternalMemoryHostPropertiesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceExternalMemoryHostPropertiesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceExternalMemoryHostPropertiesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceExternalMemoryHostPropertiesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceExternalMemoryHostPropertiesEXTBuilder<'a> {
    type Target = PhysicalDeviceExternalMemoryHostPropertiesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceExternalMemoryHostPropertiesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::ext_external_memory_host`]"]
impl crate::DeviceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryHostPointerPropertiesEXT.html) · Function"]
    #[doc(alias = "vkGetMemoryHostPointerPropertiesEXT")]
    pub unsafe fn get_memory_host_pointer_properties_ext(&self, handle_type: crate::vk1_1::ExternalMemoryHandleTypeFlagBits, host_pointer: *const std::ffi::c_void, memory_host_pointer_properties: Option<crate::extensions::ext_external_memory_host::MemoryHostPointerPropertiesEXT>) -> crate::utils::VulkanResult<crate::extensions::ext_external_memory_host::MemoryHostPointerPropertiesEXT> {
        let _function = self.get_memory_host_pointer_properties_ext.expect("tried to call a function that isn't loaded");
        let mut memory_host_pointer_properties = match memory_host_pointer_properties {
            Some(v) => v,
            None => Default::default(),
        };
        let _return = _function(self.handle, handle_type as _, host_pointer, &mut memory_host_pointer_properties);
        crate::utils::VulkanResult::new(_return, memory_host_pointer_properties)
    }
}
