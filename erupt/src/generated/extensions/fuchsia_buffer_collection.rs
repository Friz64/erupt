#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_FUCHSIA_BUFFER_COLLECTION_SPEC_VERSION")]
pub const FUCHSIA_BUFFER_COLLECTION_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_FUCHSIA_BUFFER_COLLECTION_EXTENSION_NAME")]
pub const FUCHSIA_BUFFER_COLLECTION_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_FUCHSIA_buffer_collection");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_BUFFER_COLLECTION_FUCHSIA: *const std::os::raw::c_char = crate::cstr!("vkCreateBufferCollectionFUCHSIA");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_SET_BUFFER_COLLECTION_BUFFER_CONSTRAINTS_FUCHSIA: *const std::os::raw::c_char = crate::cstr!("vkSetBufferCollectionBufferConstraintsFUCHSIA");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_SET_BUFFER_COLLECTION_IMAGE_CONSTRAINTS_FUCHSIA: *const std::os::raw::c_char = crate::cstr!("vkSetBufferCollectionImageConstraintsFUCHSIA");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_DESTROY_BUFFER_COLLECTION_FUCHSIA: *const std::os::raw::c_char = crate::cstr!("vkDestroyBufferCollectionFUCHSIA");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_BUFFER_COLLECTION_PROPERTIES_FUCHSIA: *const std::os::raw::c_char = crate::cstr!("vkGetBufferCollectionPropertiesFUCHSIA");
crate::non_dispatchable_handle!(BufferCollectionFUCHSIA, BUFFER_COLLECTION_FUCHSIA, "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferCollectionFUCHSIA.html) · Non-dispatchable Handle", "VkBufferCollectionFUCHSIA");
#[doc = "Provided by [`crate::extensions::fuchsia_buffer_collection`]"]
impl crate::vk1_0::StructureType {
    pub const BUFFER_COLLECTION_CREATE_INFO_FUCHSIA: Self = Self(1000366000);
    pub const IMPORT_MEMORY_BUFFER_COLLECTION_FUCHSIA: Self = Self(1000366001);
    pub const BUFFER_COLLECTION_IMAGE_CREATE_INFO_FUCHSIA: Self = Self(1000366002);
    pub const BUFFER_COLLECTION_PROPERTIES_FUCHSIA: Self = Self(1000366003);
    pub const BUFFER_CONSTRAINTS_INFO_FUCHSIA: Self = Self(1000366004);
    pub const BUFFER_COLLECTION_BUFFER_CREATE_INFO_FUCHSIA: Self = Self(1000366005);
    pub const IMAGE_CONSTRAINTS_INFO_FUCHSIA: Self = Self(1000366006);
    pub const IMAGE_FORMAT_CONSTRAINTS_INFO_FUCHSIA: Self = Self(1000366007);
    pub const SYSMEM_COLOR_SPACE_FUCHSIA: Self = Self(1000366008);
    pub const BUFFER_COLLECTION_CONSTRAINTS_INFO_FUCHSIA: Self = Self(1000366009);
}
#[doc = "Provided by [`crate::extensions::fuchsia_buffer_collection`]"]
impl crate::vk1_0::ObjectType {
    pub const BUFFER_COLLECTION_FUCHSIA: Self = Self(1000366000);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageFormatConstraintsFlagsFUCHSIA.html) · Bitmask of [`ImageFormatConstraintsFlagBitsFUCHSIA`]"] # [doc (alias = "VkImageFormatConstraintsFlagsFUCHSIA")] # [derive (Default)] # [repr (transparent)] pub struct ImageFormatConstraintsFlagsFUCHSIA : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`ImageFormatConstraintsFlagsFUCHSIA`]"]
#[doc(alias = "VkImageFormatConstraintsFlagBitsFUCHSIA")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct ImageFormatConstraintsFlagBitsFUCHSIA(pub u32);
impl ImageFormatConstraintsFlagBitsFUCHSIA {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> ImageFormatConstraintsFlagsFUCHSIA {
        ImageFormatConstraintsFlagsFUCHSIA::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for ImageFormatConstraintsFlagBitsFUCHSIA {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageConstraintsInfoFlagsFUCHSIA.html) · Bitmask of [`ImageConstraintsInfoFlagBitsFUCHSIA`]"] # [doc (alias = "VkImageConstraintsInfoFlagsFUCHSIA")] # [derive (Default)] # [repr (transparent)] pub struct ImageConstraintsInfoFlagsFUCHSIA : u32 { const CPU_READ_RARELY_FUCHSIA = ImageConstraintsInfoFlagBitsFUCHSIA :: CPU_READ_RARELY_FUCHSIA . 0 ; const CPU_READ_OFTEN_FUCHSIA = ImageConstraintsInfoFlagBitsFUCHSIA :: CPU_READ_OFTEN_FUCHSIA . 0 ; const CPU_WRITE_RARELY_FUCHSIA = ImageConstraintsInfoFlagBitsFUCHSIA :: CPU_WRITE_RARELY_FUCHSIA . 0 ; const CPU_WRITE_OFTEN_FUCHSIA = ImageConstraintsInfoFlagBitsFUCHSIA :: CPU_WRITE_OFTEN_FUCHSIA . 0 ; const PROTECTED_OPTIONAL_FUCHSIA = ImageConstraintsInfoFlagBitsFUCHSIA :: PROTECTED_OPTIONAL_FUCHSIA . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageConstraintsInfoFlagBitsFUCHSIA.html) · Bits enum of [`ImageConstraintsInfoFlagsFUCHSIA`]"]
#[doc(alias = "VkImageConstraintsInfoFlagBitsFUCHSIA")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct ImageConstraintsInfoFlagBitsFUCHSIA(pub u32);
impl ImageConstraintsInfoFlagBitsFUCHSIA {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> ImageConstraintsInfoFlagsFUCHSIA {
        ImageConstraintsInfoFlagsFUCHSIA::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for ImageConstraintsInfoFlagBitsFUCHSIA {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::CPU_READ_RARELY_FUCHSIA => "CPU_READ_RARELY_FUCHSIA",
            &Self::CPU_READ_OFTEN_FUCHSIA => "CPU_READ_OFTEN_FUCHSIA",
            &Self::CPU_WRITE_RARELY_FUCHSIA => "CPU_WRITE_RARELY_FUCHSIA",
            &Self::CPU_WRITE_OFTEN_FUCHSIA => "CPU_WRITE_OFTEN_FUCHSIA",
            &Self::PROTECTED_OPTIONAL_FUCHSIA => "PROTECTED_OPTIONAL_FUCHSIA",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::fuchsia_buffer_collection`]"]
impl crate::extensions::fuchsia_buffer_collection::ImageConstraintsInfoFlagBitsFUCHSIA {
    pub const CPU_READ_RARELY_FUCHSIA: Self = Self(1);
    pub const CPU_READ_OFTEN_FUCHSIA: Self = Self(2);
    pub const CPU_WRITE_RARELY_FUCHSIA: Self = Self(4);
    pub const CPU_WRITE_OFTEN_FUCHSIA: Self = Self(8);
    pub const PROTECTED_OPTIONAL_FUCHSIA: Self = Self(16);
}
#[doc = "Provided by [`crate::extensions::fuchsia_buffer_collection`]"]
impl crate::extensions::ext_debug_report::DebugReportObjectTypeEXT {
    pub const BUFFER_COLLECTION_FUCHSIA_EXT: Self = Self(1000366000);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateBufferCollectionFUCHSIA.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateBufferCollectionFUCHSIA = unsafe extern "system" fn(device: crate::vk1_0::Device, p_create_info: *const crate::extensions::fuchsia_buffer_collection::BufferCollectionCreateInfoFUCHSIA, p_allocator: *const crate::vk1_0::AllocationCallbacks, p_collection: *mut crate::extensions::fuchsia_buffer_collection::BufferCollectionFUCHSIA) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSetBufferCollectionBufferConstraintsFUCHSIA.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkSetBufferCollectionBufferConstraintsFUCHSIA = unsafe extern "system" fn(device: crate::vk1_0::Device, collection: crate::extensions::fuchsia_buffer_collection::BufferCollectionFUCHSIA, p_buffer_constraints_info: *const crate::extensions::fuchsia_buffer_collection::BufferConstraintsInfoFUCHSIA) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSetBufferCollectionImageConstraintsFUCHSIA.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkSetBufferCollectionImageConstraintsFUCHSIA = unsafe extern "system" fn(device: crate::vk1_0::Device, collection: crate::extensions::fuchsia_buffer_collection::BufferCollectionFUCHSIA, p_image_constraints_info: *const crate::extensions::fuchsia_buffer_collection::ImageConstraintsInfoFUCHSIA) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyBufferCollectionFUCHSIA.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyBufferCollectionFUCHSIA = unsafe extern "system" fn(device: crate::vk1_0::Device, collection: crate::extensions::fuchsia_buffer_collection::BufferCollectionFUCHSIA, p_allocator: *const crate::vk1_0::AllocationCallbacks) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetBufferCollectionPropertiesFUCHSIA.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetBufferCollectionPropertiesFUCHSIA = unsafe extern "system" fn(device: crate::vk1_0::Device, collection: crate::extensions::fuchsia_buffer_collection::BufferCollectionFUCHSIA, p_properties: *mut crate::extensions::fuchsia_buffer_collection::BufferCollectionPropertiesFUCHSIA) -> crate::vk1_0::Result;
impl<'a> crate::ExtendableFrom<'a, ImportMemoryBufferCollectionFUCHSIA> for crate::vk1_0::MemoryAllocateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, ImportMemoryBufferCollectionFUCHSIABuilder<'_>> for crate::vk1_0::MemoryAllocateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, BufferCollectionBufferCreateInfoFUCHSIA> for crate::vk1_0::BufferCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, BufferCollectionBufferCreateInfoFUCHSIABuilder<'_>> for crate::vk1_0::BufferCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, BufferCollectionImageCreateInfoFUCHSIA> for crate::vk1_0::ImageCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, BufferCollectionImageCreateInfoFUCHSIABuilder<'_>> for crate::vk1_0::ImageCreateInfoBuilder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImportMemoryBufferCollectionFUCHSIA.html) · Structure"]
#[doc(alias = "VkImportMemoryBufferCollectionFUCHSIA")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImportMemoryBufferCollectionFUCHSIA {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub collection: crate::extensions::fuchsia_buffer_collection::BufferCollectionFUCHSIA,
    pub index: u32,
}
impl ImportMemoryBufferCollectionFUCHSIA {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::IMPORT_MEMORY_BUFFER_COLLECTION_FUCHSIA;
}
impl Default for ImportMemoryBufferCollectionFUCHSIA {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), collection: Default::default(), index: Default::default() }
    }
}
impl std::fmt::Debug for ImportMemoryBufferCollectionFUCHSIA {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ImportMemoryBufferCollectionFUCHSIA").field("s_type", &self.s_type).field("p_next", &self.p_next).field("collection", &self.collection).field("index", &self.index).finish()
    }
}
impl ImportMemoryBufferCollectionFUCHSIA {
    #[inline]
    pub fn into_builder<'a>(self) -> ImportMemoryBufferCollectionFUCHSIABuilder<'a> {
        ImportMemoryBufferCollectionFUCHSIABuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImportMemoryBufferCollectionFUCHSIA.html) · Builder of [`ImportMemoryBufferCollectionFUCHSIA`]"]
#[repr(transparent)]
pub struct ImportMemoryBufferCollectionFUCHSIABuilder<'a>(ImportMemoryBufferCollectionFUCHSIA, std::marker::PhantomData<&'a ()>);
impl<'a> ImportMemoryBufferCollectionFUCHSIABuilder<'a> {
    #[inline]
    pub fn new() -> ImportMemoryBufferCollectionFUCHSIABuilder<'a> {
        ImportMemoryBufferCollectionFUCHSIABuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn collection(mut self, collection: crate::extensions::fuchsia_buffer_collection::BufferCollectionFUCHSIA) -> Self {
        self.0.collection = collection as _;
        self
    }
    #[inline]
    pub fn index(mut self, index: u32) -> Self {
        self.0.index = index as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ImportMemoryBufferCollectionFUCHSIA {
        self.0
    }
}
impl<'a> std::default::Default for ImportMemoryBufferCollectionFUCHSIABuilder<'a> {
    fn default() -> ImportMemoryBufferCollectionFUCHSIABuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ImportMemoryBufferCollectionFUCHSIABuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ImportMemoryBufferCollectionFUCHSIABuilder<'a> {
    type Target = ImportMemoryBufferCollectionFUCHSIA;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ImportMemoryBufferCollectionFUCHSIABuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferCollectionImageCreateInfoFUCHSIA.html) · Structure"]
#[doc(alias = "VkBufferCollectionImageCreateInfoFUCHSIA")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BufferCollectionImageCreateInfoFUCHSIA {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub collection: crate::extensions::fuchsia_buffer_collection::BufferCollectionFUCHSIA,
    pub index: u32,
}
impl BufferCollectionImageCreateInfoFUCHSIA {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::BUFFER_COLLECTION_IMAGE_CREATE_INFO_FUCHSIA;
}
impl Default for BufferCollectionImageCreateInfoFUCHSIA {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), collection: Default::default(), index: Default::default() }
    }
}
impl std::fmt::Debug for BufferCollectionImageCreateInfoFUCHSIA {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BufferCollectionImageCreateInfoFUCHSIA").field("s_type", &self.s_type).field("p_next", &self.p_next).field("collection", &self.collection).field("index", &self.index).finish()
    }
}
impl BufferCollectionImageCreateInfoFUCHSIA {
    #[inline]
    pub fn into_builder<'a>(self) -> BufferCollectionImageCreateInfoFUCHSIABuilder<'a> {
        BufferCollectionImageCreateInfoFUCHSIABuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferCollectionImageCreateInfoFUCHSIA.html) · Builder of [`BufferCollectionImageCreateInfoFUCHSIA`]"]
#[repr(transparent)]
pub struct BufferCollectionImageCreateInfoFUCHSIABuilder<'a>(BufferCollectionImageCreateInfoFUCHSIA, std::marker::PhantomData<&'a ()>);
impl<'a> BufferCollectionImageCreateInfoFUCHSIABuilder<'a> {
    #[inline]
    pub fn new() -> BufferCollectionImageCreateInfoFUCHSIABuilder<'a> {
        BufferCollectionImageCreateInfoFUCHSIABuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn collection(mut self, collection: crate::extensions::fuchsia_buffer_collection::BufferCollectionFUCHSIA) -> Self {
        self.0.collection = collection as _;
        self
    }
    #[inline]
    pub fn index(mut self, index: u32) -> Self {
        self.0.index = index as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> BufferCollectionImageCreateInfoFUCHSIA {
        self.0
    }
}
impl<'a> std::default::Default for BufferCollectionImageCreateInfoFUCHSIABuilder<'a> {
    fn default() -> BufferCollectionImageCreateInfoFUCHSIABuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for BufferCollectionImageCreateInfoFUCHSIABuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for BufferCollectionImageCreateInfoFUCHSIABuilder<'a> {
    type Target = BufferCollectionImageCreateInfoFUCHSIA;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for BufferCollectionImageCreateInfoFUCHSIABuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferCollectionBufferCreateInfoFUCHSIA.html) · Structure"]
#[doc(alias = "VkBufferCollectionBufferCreateInfoFUCHSIA")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BufferCollectionBufferCreateInfoFUCHSIA {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub collection: crate::extensions::fuchsia_buffer_collection::BufferCollectionFUCHSIA,
    pub index: u32,
}
impl BufferCollectionBufferCreateInfoFUCHSIA {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::BUFFER_COLLECTION_BUFFER_CREATE_INFO_FUCHSIA;
}
impl Default for BufferCollectionBufferCreateInfoFUCHSIA {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), collection: Default::default(), index: Default::default() }
    }
}
impl std::fmt::Debug for BufferCollectionBufferCreateInfoFUCHSIA {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BufferCollectionBufferCreateInfoFUCHSIA").field("s_type", &self.s_type).field("p_next", &self.p_next).field("collection", &self.collection).field("index", &self.index).finish()
    }
}
impl BufferCollectionBufferCreateInfoFUCHSIA {
    #[inline]
    pub fn into_builder<'a>(self) -> BufferCollectionBufferCreateInfoFUCHSIABuilder<'a> {
        BufferCollectionBufferCreateInfoFUCHSIABuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferCollectionBufferCreateInfoFUCHSIA.html) · Builder of [`BufferCollectionBufferCreateInfoFUCHSIA`]"]
#[repr(transparent)]
pub struct BufferCollectionBufferCreateInfoFUCHSIABuilder<'a>(BufferCollectionBufferCreateInfoFUCHSIA, std::marker::PhantomData<&'a ()>);
impl<'a> BufferCollectionBufferCreateInfoFUCHSIABuilder<'a> {
    #[inline]
    pub fn new() -> BufferCollectionBufferCreateInfoFUCHSIABuilder<'a> {
        BufferCollectionBufferCreateInfoFUCHSIABuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn collection(mut self, collection: crate::extensions::fuchsia_buffer_collection::BufferCollectionFUCHSIA) -> Self {
        self.0.collection = collection as _;
        self
    }
    #[inline]
    pub fn index(mut self, index: u32) -> Self {
        self.0.index = index as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> BufferCollectionBufferCreateInfoFUCHSIA {
        self.0
    }
}
impl<'a> std::default::Default for BufferCollectionBufferCreateInfoFUCHSIABuilder<'a> {
    fn default() -> BufferCollectionBufferCreateInfoFUCHSIABuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for BufferCollectionBufferCreateInfoFUCHSIABuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for BufferCollectionBufferCreateInfoFUCHSIABuilder<'a> {
    type Target = BufferCollectionBufferCreateInfoFUCHSIA;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for BufferCollectionBufferCreateInfoFUCHSIABuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferCollectionCreateInfoFUCHSIA.html) · Structure"]
#[doc(alias = "VkBufferCollectionCreateInfoFUCHSIA")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BufferCollectionCreateInfoFUCHSIA {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub collection_token: *mut std::ffi::c_void,
}
impl BufferCollectionCreateInfoFUCHSIA {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::BUFFER_COLLECTION_CREATE_INFO_FUCHSIA;
}
impl Default for BufferCollectionCreateInfoFUCHSIA {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), collection_token: std::ptr::null_mut() }
    }
}
impl std::fmt::Debug for BufferCollectionCreateInfoFUCHSIA {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BufferCollectionCreateInfoFUCHSIA").field("s_type", &self.s_type).field("p_next", &self.p_next).field("collection_token", &self.collection_token).finish()
    }
}
impl BufferCollectionCreateInfoFUCHSIA {
    #[inline]
    pub fn into_builder<'a>(self) -> BufferCollectionCreateInfoFUCHSIABuilder<'a> {
        BufferCollectionCreateInfoFUCHSIABuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferCollectionCreateInfoFUCHSIA.html) · Builder of [`BufferCollectionCreateInfoFUCHSIA`]"]
#[repr(transparent)]
pub struct BufferCollectionCreateInfoFUCHSIABuilder<'a>(BufferCollectionCreateInfoFUCHSIA, std::marker::PhantomData<&'a ()>);
impl<'a> BufferCollectionCreateInfoFUCHSIABuilder<'a> {
    #[inline]
    pub fn new() -> BufferCollectionCreateInfoFUCHSIABuilder<'a> {
        BufferCollectionCreateInfoFUCHSIABuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn collection_token(mut self, collection_token: *mut std::ffi::c_void) -> Self {
        self.0.collection_token = collection_token;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> BufferCollectionCreateInfoFUCHSIA {
        self.0
    }
}
impl<'a> std::default::Default for BufferCollectionCreateInfoFUCHSIABuilder<'a> {
    fn default() -> BufferCollectionCreateInfoFUCHSIABuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for BufferCollectionCreateInfoFUCHSIABuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for BufferCollectionCreateInfoFUCHSIABuilder<'a> {
    type Target = BufferCollectionCreateInfoFUCHSIA;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for BufferCollectionCreateInfoFUCHSIABuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferCollectionPropertiesFUCHSIA.html) · Structure"]
#[doc(alias = "VkBufferCollectionPropertiesFUCHSIA")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BufferCollectionPropertiesFUCHSIA {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub memory_type_bits: u32,
    pub buffer_count: u32,
    pub create_info_index: u32,
    pub sysmem_pixel_format: u64,
    pub format_features: crate::vk1_0::FormatFeatureFlags,
    pub sysmem_color_space_index: crate::extensions::fuchsia_buffer_collection::SysmemColorSpaceFUCHSIA,
    pub sampler_ycbcr_conversion_components: crate::vk1_0::ComponentMapping,
    pub suggested_ycbcr_model: crate::vk1_1::SamplerYcbcrModelConversion,
    pub suggested_ycbcr_range: crate::vk1_1::SamplerYcbcrRange,
    pub suggested_x_chroma_offset: crate::vk1_1::ChromaLocation,
    pub suggested_y_chroma_offset: crate::vk1_1::ChromaLocation,
}
impl BufferCollectionPropertiesFUCHSIA {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::BUFFER_COLLECTION_PROPERTIES_FUCHSIA;
}
impl Default for BufferCollectionPropertiesFUCHSIA {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), memory_type_bits: Default::default(), buffer_count: Default::default(), create_info_index: Default::default(), sysmem_pixel_format: Default::default(), format_features: Default::default(), sysmem_color_space_index: Default::default(), sampler_ycbcr_conversion_components: Default::default(), suggested_ycbcr_model: Default::default(), suggested_ycbcr_range: Default::default(), suggested_x_chroma_offset: Default::default(), suggested_y_chroma_offset: Default::default() }
    }
}
impl std::fmt::Debug for BufferCollectionPropertiesFUCHSIA {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BufferCollectionPropertiesFUCHSIA").field("s_type", &self.s_type).field("p_next", &self.p_next).field("memory_type_bits", &self.memory_type_bits).field("buffer_count", &self.buffer_count).field("create_info_index", &self.create_info_index).field("sysmem_pixel_format", &self.sysmem_pixel_format).field("format_features", &self.format_features).field("sysmem_color_space_index", &self.sysmem_color_space_index).field("sampler_ycbcr_conversion_components", &self.sampler_ycbcr_conversion_components).field("suggested_ycbcr_model", &self.suggested_ycbcr_model).field("suggested_ycbcr_range", &self.suggested_ycbcr_range).field("suggested_x_chroma_offset", &self.suggested_x_chroma_offset).field("suggested_y_chroma_offset", &self.suggested_y_chroma_offset).finish()
    }
}
impl BufferCollectionPropertiesFUCHSIA {
    #[inline]
    pub fn into_builder<'a>(self) -> BufferCollectionPropertiesFUCHSIABuilder<'a> {
        BufferCollectionPropertiesFUCHSIABuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferCollectionPropertiesFUCHSIA.html) · Builder of [`BufferCollectionPropertiesFUCHSIA`]"]
#[repr(transparent)]
pub struct BufferCollectionPropertiesFUCHSIABuilder<'a>(BufferCollectionPropertiesFUCHSIA, std::marker::PhantomData<&'a ()>);
impl<'a> BufferCollectionPropertiesFUCHSIABuilder<'a> {
    #[inline]
    pub fn new() -> BufferCollectionPropertiesFUCHSIABuilder<'a> {
        BufferCollectionPropertiesFUCHSIABuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn memory_type_bits(mut self, memory_type_bits: u32) -> Self {
        self.0.memory_type_bits = memory_type_bits as _;
        self
    }
    #[inline]
    pub fn buffer_count(mut self, buffer_count: u32) -> Self {
        self.0.buffer_count = buffer_count as _;
        self
    }
    #[inline]
    pub fn create_info_index(mut self, create_info_index: u32) -> Self {
        self.0.create_info_index = create_info_index as _;
        self
    }
    #[inline]
    pub fn sysmem_pixel_format(mut self, sysmem_pixel_format: u64) -> Self {
        self.0.sysmem_pixel_format = sysmem_pixel_format as _;
        self
    }
    #[inline]
    pub fn format_features(mut self, format_features: crate::vk1_0::FormatFeatureFlags) -> Self {
        self.0.format_features = format_features as _;
        self
    }
    #[inline]
    pub fn sysmem_color_space_index(mut self, sysmem_color_space_index: crate::extensions::fuchsia_buffer_collection::SysmemColorSpaceFUCHSIA) -> Self {
        self.0.sysmem_color_space_index = sysmem_color_space_index as _;
        self
    }
    #[inline]
    pub fn sampler_ycbcr_conversion_components(mut self, sampler_ycbcr_conversion_components: crate::vk1_0::ComponentMapping) -> Self {
        self.0.sampler_ycbcr_conversion_components = sampler_ycbcr_conversion_components as _;
        self
    }
    #[inline]
    pub fn suggested_ycbcr_model(mut self, suggested_ycbcr_model: crate::vk1_1::SamplerYcbcrModelConversion) -> Self {
        self.0.suggested_ycbcr_model = suggested_ycbcr_model as _;
        self
    }
    #[inline]
    pub fn suggested_ycbcr_range(mut self, suggested_ycbcr_range: crate::vk1_1::SamplerYcbcrRange) -> Self {
        self.0.suggested_ycbcr_range = suggested_ycbcr_range as _;
        self
    }
    #[inline]
    pub fn suggested_x_chroma_offset(mut self, suggested_x_chroma_offset: crate::vk1_1::ChromaLocation) -> Self {
        self.0.suggested_x_chroma_offset = suggested_x_chroma_offset as _;
        self
    }
    #[inline]
    pub fn suggested_y_chroma_offset(mut self, suggested_y_chroma_offset: crate::vk1_1::ChromaLocation) -> Self {
        self.0.suggested_y_chroma_offset = suggested_y_chroma_offset as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> BufferCollectionPropertiesFUCHSIA {
        self.0
    }
}
impl<'a> std::default::Default for BufferCollectionPropertiesFUCHSIABuilder<'a> {
    fn default() -> BufferCollectionPropertiesFUCHSIABuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for BufferCollectionPropertiesFUCHSIABuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for BufferCollectionPropertiesFUCHSIABuilder<'a> {
    type Target = BufferCollectionPropertiesFUCHSIA;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for BufferCollectionPropertiesFUCHSIABuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferConstraintsInfoFUCHSIA.html) · Structure"]
#[doc(alias = "VkBufferConstraintsInfoFUCHSIA")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BufferConstraintsInfoFUCHSIA {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub create_info: crate::vk1_0::BufferCreateInfo,
    pub required_format_features: crate::vk1_0::FormatFeatureFlags,
    pub buffer_collection_constraints: crate::extensions::fuchsia_buffer_collection::BufferCollectionConstraintsInfoFUCHSIA,
}
impl BufferConstraintsInfoFUCHSIA {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::BUFFER_CONSTRAINTS_INFO_FUCHSIA;
}
impl Default for BufferConstraintsInfoFUCHSIA {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), create_info: Default::default(), required_format_features: Default::default(), buffer_collection_constraints: Default::default() }
    }
}
impl std::fmt::Debug for BufferConstraintsInfoFUCHSIA {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BufferConstraintsInfoFUCHSIA").field("s_type", &self.s_type).field("p_next", &self.p_next).field("create_info", &self.create_info).field("required_format_features", &self.required_format_features).field("buffer_collection_constraints", &self.buffer_collection_constraints).finish()
    }
}
impl BufferConstraintsInfoFUCHSIA {
    #[inline]
    pub fn into_builder<'a>(self) -> BufferConstraintsInfoFUCHSIABuilder<'a> {
        BufferConstraintsInfoFUCHSIABuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferConstraintsInfoFUCHSIA.html) · Builder of [`BufferConstraintsInfoFUCHSIA`]"]
#[repr(transparent)]
pub struct BufferConstraintsInfoFUCHSIABuilder<'a>(BufferConstraintsInfoFUCHSIA, std::marker::PhantomData<&'a ()>);
impl<'a> BufferConstraintsInfoFUCHSIABuilder<'a> {
    #[inline]
    pub fn new() -> BufferConstraintsInfoFUCHSIABuilder<'a> {
        BufferConstraintsInfoFUCHSIABuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn create_info(mut self, create_info: crate::vk1_0::BufferCreateInfo) -> Self {
        self.0.create_info = create_info as _;
        self
    }
    #[inline]
    pub fn required_format_features(mut self, required_format_features: crate::vk1_0::FormatFeatureFlags) -> Self {
        self.0.required_format_features = required_format_features as _;
        self
    }
    #[inline]
    pub fn buffer_collection_constraints(mut self, buffer_collection_constraints: crate::extensions::fuchsia_buffer_collection::BufferCollectionConstraintsInfoFUCHSIA) -> Self {
        self.0.buffer_collection_constraints = buffer_collection_constraints as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> BufferConstraintsInfoFUCHSIA {
        self.0
    }
}
impl<'a> std::default::Default for BufferConstraintsInfoFUCHSIABuilder<'a> {
    fn default() -> BufferConstraintsInfoFUCHSIABuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for BufferConstraintsInfoFUCHSIABuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for BufferConstraintsInfoFUCHSIABuilder<'a> {
    type Target = BufferConstraintsInfoFUCHSIA;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for BufferConstraintsInfoFUCHSIABuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSysmemColorSpaceFUCHSIA.html) · Structure"]
#[doc(alias = "VkSysmemColorSpaceFUCHSIA")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SysmemColorSpaceFUCHSIA {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub color_space: u32,
}
impl SysmemColorSpaceFUCHSIA {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::SYSMEM_COLOR_SPACE_FUCHSIA;
}
impl Default for SysmemColorSpaceFUCHSIA {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), color_space: Default::default() }
    }
}
impl std::fmt::Debug for SysmemColorSpaceFUCHSIA {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SysmemColorSpaceFUCHSIA").field("s_type", &self.s_type).field("p_next", &self.p_next).field("color_space", &self.color_space).finish()
    }
}
impl SysmemColorSpaceFUCHSIA {
    #[inline]
    pub fn into_builder<'a>(self) -> SysmemColorSpaceFUCHSIABuilder<'a> {
        SysmemColorSpaceFUCHSIABuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSysmemColorSpaceFUCHSIA.html) · Builder of [`SysmemColorSpaceFUCHSIA`]"]
#[repr(transparent)]
pub struct SysmemColorSpaceFUCHSIABuilder<'a>(SysmemColorSpaceFUCHSIA, std::marker::PhantomData<&'a ()>);
impl<'a> SysmemColorSpaceFUCHSIABuilder<'a> {
    #[inline]
    pub fn new() -> SysmemColorSpaceFUCHSIABuilder<'a> {
        SysmemColorSpaceFUCHSIABuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn color_space(mut self, color_space: u32) -> Self {
        self.0.color_space = color_space as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> SysmemColorSpaceFUCHSIA {
        self.0
    }
}
impl<'a> std::default::Default for SysmemColorSpaceFUCHSIABuilder<'a> {
    fn default() -> SysmemColorSpaceFUCHSIABuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for SysmemColorSpaceFUCHSIABuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for SysmemColorSpaceFUCHSIABuilder<'a> {
    type Target = SysmemColorSpaceFUCHSIA;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SysmemColorSpaceFUCHSIABuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageFormatConstraintsInfoFUCHSIA.html) · Structure"]
#[doc(alias = "VkImageFormatConstraintsInfoFUCHSIA")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImageFormatConstraintsInfoFUCHSIA {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub image_create_info: crate::vk1_0::ImageCreateInfo,
    pub required_format_features: crate::vk1_0::FormatFeatureFlags,
    pub flags: crate::extensions::fuchsia_buffer_collection::ImageFormatConstraintsFlagsFUCHSIA,
    pub sysmem_pixel_format: u64,
    pub color_space_count: u32,
    pub p_color_spaces: *const crate::extensions::fuchsia_buffer_collection::SysmemColorSpaceFUCHSIA,
}
impl ImageFormatConstraintsInfoFUCHSIA {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::IMAGE_FORMAT_CONSTRAINTS_INFO_FUCHSIA;
}
impl Default for ImageFormatConstraintsInfoFUCHSIA {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), image_create_info: Default::default(), required_format_features: Default::default(), flags: Default::default(), sysmem_pixel_format: Default::default(), color_space_count: Default::default(), p_color_spaces: std::ptr::null() }
    }
}
impl std::fmt::Debug for ImageFormatConstraintsInfoFUCHSIA {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ImageFormatConstraintsInfoFUCHSIA").field("s_type", &self.s_type).field("p_next", &self.p_next).field("image_create_info", &self.image_create_info).field("required_format_features", &self.required_format_features).field("flags", &self.flags).field("sysmem_pixel_format", &self.sysmem_pixel_format).field("color_space_count", &self.color_space_count).field("p_color_spaces", &self.p_color_spaces).finish()
    }
}
impl ImageFormatConstraintsInfoFUCHSIA {
    #[inline]
    pub fn into_builder<'a>(self) -> ImageFormatConstraintsInfoFUCHSIABuilder<'a> {
        ImageFormatConstraintsInfoFUCHSIABuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageFormatConstraintsInfoFUCHSIA.html) · Builder of [`ImageFormatConstraintsInfoFUCHSIA`]"]
#[repr(transparent)]
pub struct ImageFormatConstraintsInfoFUCHSIABuilder<'a>(ImageFormatConstraintsInfoFUCHSIA, std::marker::PhantomData<&'a ()>);
impl<'a> ImageFormatConstraintsInfoFUCHSIABuilder<'a> {
    #[inline]
    pub fn new() -> ImageFormatConstraintsInfoFUCHSIABuilder<'a> {
        ImageFormatConstraintsInfoFUCHSIABuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn image_create_info(mut self, image_create_info: crate::vk1_0::ImageCreateInfo) -> Self {
        self.0.image_create_info = image_create_info as _;
        self
    }
    #[inline]
    pub fn required_format_features(mut self, required_format_features: crate::vk1_0::FormatFeatureFlags) -> Self {
        self.0.required_format_features = required_format_features as _;
        self
    }
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::fuchsia_buffer_collection::ImageFormatConstraintsFlagsFUCHSIA) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn sysmem_pixel_format(mut self, sysmem_pixel_format: u64) -> Self {
        self.0.sysmem_pixel_format = sysmem_pixel_format as _;
        self
    }
    #[inline]
    pub fn color_space_count(mut self, color_space_count: u32) -> Self {
        self.0.color_space_count = color_space_count as _;
        self
    }
    #[inline]
    pub fn color_spaces(mut self, color_spaces: &'a crate::extensions::fuchsia_buffer_collection::SysmemColorSpaceFUCHSIA) -> Self {
        self.0.p_color_spaces = color_spaces as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ImageFormatConstraintsInfoFUCHSIA {
        self.0
    }
}
impl<'a> std::default::Default for ImageFormatConstraintsInfoFUCHSIABuilder<'a> {
    fn default() -> ImageFormatConstraintsInfoFUCHSIABuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ImageFormatConstraintsInfoFUCHSIABuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ImageFormatConstraintsInfoFUCHSIABuilder<'a> {
    type Target = ImageFormatConstraintsInfoFUCHSIA;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ImageFormatConstraintsInfoFUCHSIABuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageConstraintsInfoFUCHSIA.html) · Structure"]
#[doc(alias = "VkImageConstraintsInfoFUCHSIA")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImageConstraintsInfoFUCHSIA {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub format_constraints_count: u32,
    pub p_format_constraints: *const crate::extensions::fuchsia_buffer_collection::ImageFormatConstraintsInfoFUCHSIA,
    pub buffer_collection_constraints: crate::extensions::fuchsia_buffer_collection::BufferCollectionConstraintsInfoFUCHSIA,
    pub flags: crate::extensions::fuchsia_buffer_collection::ImageConstraintsInfoFlagsFUCHSIA,
}
impl ImageConstraintsInfoFUCHSIA {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::IMAGE_CONSTRAINTS_INFO_FUCHSIA;
}
impl Default for ImageConstraintsInfoFUCHSIA {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), format_constraints_count: Default::default(), p_format_constraints: std::ptr::null(), buffer_collection_constraints: Default::default(), flags: Default::default() }
    }
}
impl std::fmt::Debug for ImageConstraintsInfoFUCHSIA {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ImageConstraintsInfoFUCHSIA").field("s_type", &self.s_type).field("p_next", &self.p_next).field("format_constraints_count", &self.format_constraints_count).field("p_format_constraints", &self.p_format_constraints).field("buffer_collection_constraints", &self.buffer_collection_constraints).field("flags", &self.flags).finish()
    }
}
impl ImageConstraintsInfoFUCHSIA {
    #[inline]
    pub fn into_builder<'a>(self) -> ImageConstraintsInfoFUCHSIABuilder<'a> {
        ImageConstraintsInfoFUCHSIABuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageConstraintsInfoFUCHSIA.html) · Builder of [`ImageConstraintsInfoFUCHSIA`]"]
#[repr(transparent)]
pub struct ImageConstraintsInfoFUCHSIABuilder<'a>(ImageConstraintsInfoFUCHSIA, std::marker::PhantomData<&'a ()>);
impl<'a> ImageConstraintsInfoFUCHSIABuilder<'a> {
    #[inline]
    pub fn new() -> ImageConstraintsInfoFUCHSIABuilder<'a> {
        ImageConstraintsInfoFUCHSIABuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn format_constraints(mut self, format_constraints: &'a [crate::extensions::fuchsia_buffer_collection::ImageFormatConstraintsInfoFUCHSIABuilder]) -> Self {
        self.0.p_format_constraints = format_constraints.as_ptr() as _;
        self.0.format_constraints_count = format_constraints.len() as _;
        self
    }
    #[inline]
    pub fn buffer_collection_constraints(mut self, buffer_collection_constraints: crate::extensions::fuchsia_buffer_collection::BufferCollectionConstraintsInfoFUCHSIA) -> Self {
        self.0.buffer_collection_constraints = buffer_collection_constraints as _;
        self
    }
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::fuchsia_buffer_collection::ImageConstraintsInfoFlagsFUCHSIA) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ImageConstraintsInfoFUCHSIA {
        self.0
    }
}
impl<'a> std::default::Default for ImageConstraintsInfoFUCHSIABuilder<'a> {
    fn default() -> ImageConstraintsInfoFUCHSIABuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ImageConstraintsInfoFUCHSIABuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ImageConstraintsInfoFUCHSIABuilder<'a> {
    type Target = ImageConstraintsInfoFUCHSIA;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ImageConstraintsInfoFUCHSIABuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferCollectionConstraintsInfoFUCHSIA.html) · Structure"]
#[doc(alias = "VkBufferCollectionConstraintsInfoFUCHSIA")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BufferCollectionConstraintsInfoFUCHSIA {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub min_buffer_count: u32,
    pub max_buffer_count: u32,
    pub min_buffer_count_for_camping: u32,
    pub min_buffer_count_for_dedicated_slack: u32,
    pub min_buffer_count_for_shared_slack: u32,
}
impl BufferCollectionConstraintsInfoFUCHSIA {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::BUFFER_COLLECTION_CONSTRAINTS_INFO_FUCHSIA;
}
impl Default for BufferCollectionConstraintsInfoFUCHSIA {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), min_buffer_count: Default::default(), max_buffer_count: Default::default(), min_buffer_count_for_camping: Default::default(), min_buffer_count_for_dedicated_slack: Default::default(), min_buffer_count_for_shared_slack: Default::default() }
    }
}
impl std::fmt::Debug for BufferCollectionConstraintsInfoFUCHSIA {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BufferCollectionConstraintsInfoFUCHSIA").field("s_type", &self.s_type).field("p_next", &self.p_next).field("min_buffer_count", &self.min_buffer_count).field("max_buffer_count", &self.max_buffer_count).field("min_buffer_count_for_camping", &self.min_buffer_count_for_camping).field("min_buffer_count_for_dedicated_slack", &self.min_buffer_count_for_dedicated_slack).field("min_buffer_count_for_shared_slack", &self.min_buffer_count_for_shared_slack).finish()
    }
}
impl BufferCollectionConstraintsInfoFUCHSIA {
    #[inline]
    pub fn into_builder<'a>(self) -> BufferCollectionConstraintsInfoFUCHSIABuilder<'a> {
        BufferCollectionConstraintsInfoFUCHSIABuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferCollectionConstraintsInfoFUCHSIA.html) · Builder of [`BufferCollectionConstraintsInfoFUCHSIA`]"]
#[repr(transparent)]
pub struct BufferCollectionConstraintsInfoFUCHSIABuilder<'a>(BufferCollectionConstraintsInfoFUCHSIA, std::marker::PhantomData<&'a ()>);
impl<'a> BufferCollectionConstraintsInfoFUCHSIABuilder<'a> {
    #[inline]
    pub fn new() -> BufferCollectionConstraintsInfoFUCHSIABuilder<'a> {
        BufferCollectionConstraintsInfoFUCHSIABuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn min_buffer_count(mut self, min_buffer_count: u32) -> Self {
        self.0.min_buffer_count = min_buffer_count as _;
        self
    }
    #[inline]
    pub fn max_buffer_count(mut self, max_buffer_count: u32) -> Self {
        self.0.max_buffer_count = max_buffer_count as _;
        self
    }
    #[inline]
    pub fn min_buffer_count_for_camping(mut self, min_buffer_count_for_camping: u32) -> Self {
        self.0.min_buffer_count_for_camping = min_buffer_count_for_camping as _;
        self
    }
    #[inline]
    pub fn min_buffer_count_for_dedicated_slack(mut self, min_buffer_count_for_dedicated_slack: u32) -> Self {
        self.0.min_buffer_count_for_dedicated_slack = min_buffer_count_for_dedicated_slack as _;
        self
    }
    #[inline]
    pub fn min_buffer_count_for_shared_slack(mut self, min_buffer_count_for_shared_slack: u32) -> Self {
        self.0.min_buffer_count_for_shared_slack = min_buffer_count_for_shared_slack as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> BufferCollectionConstraintsInfoFUCHSIA {
        self.0
    }
}
impl<'a> std::default::Default for BufferCollectionConstraintsInfoFUCHSIABuilder<'a> {
    fn default() -> BufferCollectionConstraintsInfoFUCHSIABuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for BufferCollectionConstraintsInfoFUCHSIABuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for BufferCollectionConstraintsInfoFUCHSIABuilder<'a> {
    type Target = BufferCollectionConstraintsInfoFUCHSIA;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for BufferCollectionConstraintsInfoFUCHSIABuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::fuchsia_buffer_collection`]"]
impl crate::DeviceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateBufferCollectionFUCHSIA.html) · Function"]
    #[doc(alias = "vkCreateBufferCollectionFUCHSIA")]
    pub unsafe fn create_buffer_collection_fuchsia(&self, create_info: &crate::extensions::fuchsia_buffer_collection::BufferCollectionCreateInfoFUCHSIA, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> crate::utils::VulkanResult<crate::extensions::fuchsia_buffer_collection::BufferCollectionFUCHSIA> {
        let _function = self.create_buffer_collection_fuchsia.expect(crate::NOT_LOADED_MESSAGE);
        let mut collection = Default::default();
        let _return = _function(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut collection,
        );
        crate::utils::VulkanResult::new(_return, collection)
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSetBufferCollectionBufferConstraintsFUCHSIA.html) · Function"]
    #[doc(alias = "vkSetBufferCollectionBufferConstraintsFUCHSIA")]
    pub unsafe fn set_buffer_collection_buffer_constraints_fuchsia(&self, collection: crate::extensions::fuchsia_buffer_collection::BufferCollectionFUCHSIA, buffer_constraints_info: &crate::extensions::fuchsia_buffer_collection::BufferConstraintsInfoFUCHSIA) -> crate::utils::VulkanResult<()> {
        let _function = self.set_buffer_collection_buffer_constraints_fuchsia.expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(self.handle, collection as _, buffer_constraints_info as _);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSetBufferCollectionImageConstraintsFUCHSIA.html) · Function"]
    #[doc(alias = "vkSetBufferCollectionImageConstraintsFUCHSIA")]
    pub unsafe fn set_buffer_collection_image_constraints_fuchsia(&self, collection: crate::extensions::fuchsia_buffer_collection::BufferCollectionFUCHSIA, image_constraints_info: &crate::extensions::fuchsia_buffer_collection::ImageConstraintsInfoFUCHSIA) -> crate::utils::VulkanResult<()> {
        let _function = self.set_buffer_collection_image_constraints_fuchsia.expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(self.handle, collection as _, image_constraints_info as _);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyBufferCollectionFUCHSIA.html) · Function"]
    #[doc(alias = "vkDestroyBufferCollectionFUCHSIA")]
    pub unsafe fn destroy_buffer_collection_fuchsia(&self, collection: crate::extensions::fuchsia_buffer_collection::BufferCollectionFUCHSIA, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> () {
        let _function = self.destroy_buffer_collection_fuchsia.expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(
            self.handle,
            collection as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetBufferCollectionPropertiesFUCHSIA.html) · Function"]
    #[doc(alias = "vkGetBufferCollectionPropertiesFUCHSIA")]
    pub unsafe fn get_buffer_collection_properties_fuchsia(&self, collection: crate::extensions::fuchsia_buffer_collection::BufferCollectionFUCHSIA, properties: Option<crate::extensions::fuchsia_buffer_collection::BufferCollectionPropertiesFUCHSIA>) -> crate::utils::VulkanResult<crate::extensions::fuchsia_buffer_collection::BufferCollectionPropertiesFUCHSIA> {
        let _function = self.get_buffer_collection_properties_fuchsia.expect(crate::NOT_LOADED_MESSAGE);
        let mut properties = match properties {
            Some(v) => v,
            None => Default::default(),
        };
        let _return = _function(self.handle, collection as _, &mut properties);
        crate::utils::VulkanResult::new(_return, properties)
    }
}
