# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_EXT_external_memory_host.html)\n\n## Extends\n- [`ExternalMemoryHandleTypeFlagBits`](../../vk1_1/struct.ExternalMemoryHandleTypeFlagBits.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_EXTERNAL_MEMORY_HOST_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_EXTERNAL_MEMORY_HOST_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_external_memory_host");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryHostPointerPropertiesEXT.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetMemoryHostPointerPropertiesEXT = unsafe extern "system" fn ( device : crate :: vk1_0 :: Device , handle_type : crate :: vk1_1 :: ExternalMemoryHandleTypeFlagBits , p_host_pointer : * const std :: ffi :: c_void , p_memory_host_pointer_properties : * mut crate :: extensions :: ext_external_memory_host :: MemoryHostPointerPropertiesEXT , ) -> crate :: vk1_0 :: Result ;
#[doc = "Provides Device Commands for [`ExtExternalMemoryHostDeviceLoaderExt`](trait.ExtExternalMemoryHostDeviceLoaderExt.html)"]
pub struct ExtExternalMemoryHostDeviceCommands {
    pub get_memory_host_pointer_properties_ext: Option<PFN_vkGetMemoryHostPointerPropertiesEXT>,
}
impl ExtExternalMemoryHostDeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<ExtExternalMemoryHostDeviceCommands> {
        unsafe {
            let mut success = false;
            let table = ExtExternalMemoryHostDeviceCommands {
                get_memory_host_pointer_properties_ext: std::mem::transmute({
                    let symbol = loader.symbol("vkGetMemoryHostPointerPropertiesEXT");
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
fn device_commands(loader: &crate::DeviceLoader) -> &ExtExternalMemoryHostDeviceCommands {
    loader
        .ext_external_memory_host
        .as_ref()
        .expect("`ext_external_memory_host` not loaded")
}
#[doc = "Provides high level command wrappers for [`ExtExternalMemoryHostDeviceCommands`](struct.ExtExternalMemoryHostDeviceCommands.html)"]
pub trait ExtExternalMemoryHostDeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryHostPointerPropertiesEXT.html) · Device Command"]
    unsafe fn get_memory_host_pointer_properties_ext(
        &self,
        handle_type: crate::vk1_1::ExternalMemoryHandleTypeFlagBits,
        host_pointer: *const std::ffi::c_void,
        memory_host_pointer_properties: Option<
            crate::extensions::ext_external_memory_host::MemoryHostPointerPropertiesEXT,
        >,
    ) -> crate::utils::VulkanResult<
        crate::extensions::ext_external_memory_host::MemoryHostPointerPropertiesEXT,
    >;
}
impl ExtExternalMemoryHostDeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryHostPointerPropertiesEXT.html) · Device Command"]
    unsafe fn get_memory_host_pointer_properties_ext(
        &self,
        handle_type: crate::vk1_1::ExternalMemoryHandleTypeFlagBits,
        host_pointer: *const std::ffi::c_void,
        memory_host_pointer_properties: Option<
            crate::extensions::ext_external_memory_host::MemoryHostPointerPropertiesEXT,
        >,
    ) -> crate::utils::VulkanResult<
        crate::extensions::ext_external_memory_host::MemoryHostPointerPropertiesEXT,
    > {
        let function = device_commands(self)
            .get_memory_host_pointer_properties_ext
            .as_ref()
            .expect("`get_memory_host_pointer_properties_ext` not available");
        let mut memory_host_pointer_properties =
            memory_host_pointer_properties.unwrap_or_else(|| Default::default());
        let _val = function(
            self.handle,
            handle_type,
            host_pointer,
            &mut memory_host_pointer_properties,
        );
        crate::utils::VulkanResult::new(_val, memory_host_pointer_properties)
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryHostPointerPropertiesEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryHostPointerPropertiesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub memory_type_bits: u32,
}
impl MemoryHostPointerPropertiesEXT {
    #[inline]
    pub fn builder<'a>(self) -> MemoryHostPointerPropertiesEXTBuilder<'a> {
        MemoryHostPointerPropertiesEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for MemoryHostPointerPropertiesEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("MemoryHostPointerPropertiesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("memory_type_bits", &self.memory_type_bits)
            .finish()
    }
}
impl Default for MemoryHostPointerPropertiesEXT {
    fn default() -> MemoryHostPointerPropertiesEXT {
        MemoryHostPointerPropertiesEXT {
            s_type: crate::vk1_0::StructureType::MEMORY_HOST_POINTER_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            memory_type_bits: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryHostPointerPropertiesEXT.html) · Builder of [`MemoryHostPointerPropertiesEXT`](struct.MemoryHostPointerPropertiesEXT.html)"]
#[repr(transparent)]
pub struct MemoryHostPointerPropertiesEXTBuilder<'a>(
    MemoryHostPointerPropertiesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> MemoryHostPointerPropertiesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> MemoryHostPointerPropertiesEXTBuilder<'a> {
        MemoryHostPointerPropertiesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn memory_type_bits(mut self, memory_type_bits: u32) -> Self {
        self.0.memory_type_bits = memory_type_bits;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> MemoryHostPointerPropertiesEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for MemoryHostPointerPropertiesEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImportMemoryHostPointerInfoEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImportMemoryHostPointerInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub handle_type: crate::vk1_1::ExternalMemoryHandleTypeFlagBits,
    pub p_host_pointer: *mut std::ffi::c_void,
}
impl ImportMemoryHostPointerInfoEXT {
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
    pub fn builder<'a>(self) -> ImportMemoryHostPointerInfoEXTBuilder<'a> {
        ImportMemoryHostPointerInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ImportMemoryHostPointerInfoEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ImportMemoryHostPointerInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("handle_type", &self.handle_type)
            .field("p_host_pointer", &self.p_host_pointer)
            .finish()
    }
}
impl Default for ImportMemoryHostPointerInfoEXT {
    fn default() -> ImportMemoryHostPointerInfoEXT {
        ImportMemoryHostPointerInfoEXT {
            s_type: crate::vk1_0::StructureType::IMPORT_MEMORY_HOST_POINTER_INFO_EXT,
            p_next: std::ptr::null(),
            handle_type: Default::default(),
            p_host_pointer: std::ptr::null_mut(),
        }
    }
}
impl crate::ExtendableBy<ImportMemoryHostPointerInfoEXT> for crate::vk1_0::MemoryAllocateInfo {}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImportMemoryHostPointerInfoEXT.html) · Builder of [`ImportMemoryHostPointerInfoEXT`](struct.ImportMemoryHostPointerInfoEXT.html)"]
#[repr(transparent)]
pub struct ImportMemoryHostPointerInfoEXTBuilder<'a>(
    ImportMemoryHostPointerInfoEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> ImportMemoryHostPointerInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> ImportMemoryHostPointerInfoEXTBuilder<'a> {
        ImportMemoryHostPointerInfoEXTBuilder(Default::default(), std::marker::PhantomData)
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
    pub fn host_pointer(mut self, host_pointer: &'a mut std::ffi::c_void) -> Self {
        self.0.p_host_pointer = host_pointer;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> ImportMemoryHostPointerInfoEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for ImportMemoryHostPointerInfoEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceExternalMemoryHostPropertiesEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceExternalMemoryHostPropertiesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub min_imported_host_pointer_alignment: crate::vk1_0::DeviceSize,
}
impl PhysicalDeviceExternalMemoryHostPropertiesEXT {
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
    pub fn builder<'a>(self) -> PhysicalDeviceExternalMemoryHostPropertiesEXTBuilder<'a> {
        PhysicalDeviceExternalMemoryHostPropertiesEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceExternalMemoryHostPropertiesEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceExternalMemoryHostPropertiesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "min_imported_host_pointer_alignment",
                &self.min_imported_host_pointer_alignment,
            )
            .finish()
    }
}
impl Default for PhysicalDeviceExternalMemoryHostPropertiesEXT {
    fn default() -> PhysicalDeviceExternalMemoryHostPropertiesEXT {
        PhysicalDeviceExternalMemoryHostPropertiesEXT {
            s_type:
                crate::vk1_0::StructureType::PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            min_imported_host_pointer_alignment: Default::default(),
        }
    }
}
impl crate::ExtendableBy<PhysicalDeviceExternalMemoryHostPropertiesEXT>
    for crate::vk1_1::PhysicalDeviceProperties2
{
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceExternalMemoryHostPropertiesEXT.html) · Builder of [`PhysicalDeviceExternalMemoryHostPropertiesEXT`](struct.PhysicalDeviceExternalMemoryHostPropertiesEXT.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceExternalMemoryHostPropertiesEXTBuilder<'a>(
    PhysicalDeviceExternalMemoryHostPropertiesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceExternalMemoryHostPropertiesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceExternalMemoryHostPropertiesEXTBuilder<'a> {
        PhysicalDeviceExternalMemoryHostPropertiesEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn min_imported_host_pointer_alignment(
        mut self,
        min_imported_host_pointer_alignment: crate::vk1_0::DeviceSize,
    ) -> Self {
        self.0.min_imported_host_pointer_alignment = min_imported_host_pointer_alignment;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceExternalMemoryHostPropertiesEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceExternalMemoryHostPropertiesEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
