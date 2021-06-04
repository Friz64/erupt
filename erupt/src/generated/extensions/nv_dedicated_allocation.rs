#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_DEDICATED_ALLOCATION_SPEC_VERSION")]
pub const NV_DEDICATED_ALLOCATION_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_DEDICATED_ALLOCATION_EXTENSION_NAME")]
pub const NV_DEDICATED_ALLOCATION_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_NV_dedicated_allocation");
#[doc = "Provided by [`crate::extensions::nv_dedicated_allocation`]"]
impl crate::vk1_0::StructureType {
    pub const DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV: Self = Self(1000026000);
    pub const DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV: Self = Self(1000026001);
    pub const DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV: Self = Self(1000026002);
}
impl<'a> crate::ExtendableFromConst<'a, DedicatedAllocationMemoryAllocateInfoNV> for crate::vk1_0::MemoryAllocateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, DedicatedAllocationMemoryAllocateInfoNVBuilder<'_>> for crate::vk1_0::MemoryAllocateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, DedicatedAllocationBufferCreateInfoNV> for crate::vk1_0::BufferCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, DedicatedAllocationBufferCreateInfoNVBuilder<'_>> for crate::vk1_0::BufferCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, DedicatedAllocationImageCreateInfoNV> for crate::vk1_0::ImageCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, DedicatedAllocationImageCreateInfoNVBuilder<'_>> for crate::vk1_0::ImageCreateInfoBuilder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDedicatedAllocationImageCreateInfoNV.html) · Structure"]
#[doc(alias = "VkDedicatedAllocationImageCreateInfoNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DedicatedAllocationImageCreateInfoNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub dedicated_allocation: crate::vk1_0::Bool32,
}
impl Default for DedicatedAllocationImageCreateInfoNV {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV, p_next: std::ptr::null(), dedicated_allocation: Default::default() }
    }
}
impl std::fmt::Debug for DedicatedAllocationImageCreateInfoNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DedicatedAllocationImageCreateInfoNV").field("s_type", &self.s_type).field("p_next", &self.p_next).field("dedicated_allocation", &(self.dedicated_allocation != 0)).finish()
    }
}
impl DedicatedAllocationImageCreateInfoNV {
    #[inline]
    pub fn into_builder<'a>(self) -> DedicatedAllocationImageCreateInfoNVBuilder<'a> {
        DedicatedAllocationImageCreateInfoNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDedicatedAllocationImageCreateInfoNV.html) · Builder of [`DedicatedAllocationImageCreateInfoNV`]"]
#[repr(transparent)]
pub struct DedicatedAllocationImageCreateInfoNVBuilder<'a>(DedicatedAllocationImageCreateInfoNV, std::marker::PhantomData<&'a ()>);
impl<'a> DedicatedAllocationImageCreateInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> DedicatedAllocationImageCreateInfoNVBuilder<'a> {
        DedicatedAllocationImageCreateInfoNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn dedicated_allocation(mut self, dedicated_allocation: bool) -> Self {
        self.0.dedicated_allocation = dedicated_allocation as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DedicatedAllocationImageCreateInfoNV {
        self.0
    }
}
impl<'a> std::default::Default for DedicatedAllocationImageCreateInfoNVBuilder<'a> {
    fn default() -> DedicatedAllocationImageCreateInfoNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DedicatedAllocationImageCreateInfoNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DedicatedAllocationImageCreateInfoNVBuilder<'a> {
    type Target = DedicatedAllocationImageCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DedicatedAllocationImageCreateInfoNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDedicatedAllocationBufferCreateInfoNV.html) · Structure"]
#[doc(alias = "VkDedicatedAllocationBufferCreateInfoNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DedicatedAllocationBufferCreateInfoNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub dedicated_allocation: crate::vk1_0::Bool32,
}
impl Default for DedicatedAllocationBufferCreateInfoNV {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV, p_next: std::ptr::null(), dedicated_allocation: Default::default() }
    }
}
impl std::fmt::Debug for DedicatedAllocationBufferCreateInfoNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DedicatedAllocationBufferCreateInfoNV").field("s_type", &self.s_type).field("p_next", &self.p_next).field("dedicated_allocation", &(self.dedicated_allocation != 0)).finish()
    }
}
impl DedicatedAllocationBufferCreateInfoNV {
    #[inline]
    pub fn into_builder<'a>(self) -> DedicatedAllocationBufferCreateInfoNVBuilder<'a> {
        DedicatedAllocationBufferCreateInfoNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDedicatedAllocationBufferCreateInfoNV.html) · Builder of [`DedicatedAllocationBufferCreateInfoNV`]"]
#[repr(transparent)]
pub struct DedicatedAllocationBufferCreateInfoNVBuilder<'a>(DedicatedAllocationBufferCreateInfoNV, std::marker::PhantomData<&'a ()>);
impl<'a> DedicatedAllocationBufferCreateInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> DedicatedAllocationBufferCreateInfoNVBuilder<'a> {
        DedicatedAllocationBufferCreateInfoNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn dedicated_allocation(mut self, dedicated_allocation: bool) -> Self {
        self.0.dedicated_allocation = dedicated_allocation as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DedicatedAllocationBufferCreateInfoNV {
        self.0
    }
}
impl<'a> std::default::Default for DedicatedAllocationBufferCreateInfoNVBuilder<'a> {
    fn default() -> DedicatedAllocationBufferCreateInfoNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DedicatedAllocationBufferCreateInfoNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DedicatedAllocationBufferCreateInfoNVBuilder<'a> {
    type Target = DedicatedAllocationBufferCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DedicatedAllocationBufferCreateInfoNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDedicatedAllocationMemoryAllocateInfoNV.html) · Structure"]
#[doc(alias = "VkDedicatedAllocationMemoryAllocateInfoNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DedicatedAllocationMemoryAllocateInfoNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub image: crate::vk1_0::Image,
    pub buffer: crate::vk1_0::Buffer,
}
impl Default for DedicatedAllocationMemoryAllocateInfoNV {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV, p_next: std::ptr::null(), image: Default::default(), buffer: Default::default() }
    }
}
impl std::fmt::Debug for DedicatedAllocationMemoryAllocateInfoNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DedicatedAllocationMemoryAllocateInfoNV").field("s_type", &self.s_type).field("p_next", &self.p_next).field("image", &self.image).field("buffer", &self.buffer).finish()
    }
}
impl DedicatedAllocationMemoryAllocateInfoNV {
    #[inline]
    pub fn into_builder<'a>(self) -> DedicatedAllocationMemoryAllocateInfoNVBuilder<'a> {
        DedicatedAllocationMemoryAllocateInfoNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDedicatedAllocationMemoryAllocateInfoNV.html) · Builder of [`DedicatedAllocationMemoryAllocateInfoNV`]"]
#[repr(transparent)]
pub struct DedicatedAllocationMemoryAllocateInfoNVBuilder<'a>(DedicatedAllocationMemoryAllocateInfoNV, std::marker::PhantomData<&'a ()>);
impl<'a> DedicatedAllocationMemoryAllocateInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> DedicatedAllocationMemoryAllocateInfoNVBuilder<'a> {
        DedicatedAllocationMemoryAllocateInfoNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn image(mut self, image: crate::vk1_0::Image) -> Self {
        self.0.image = image as _;
        self
    }
    #[inline]
    pub fn buffer(mut self, buffer: crate::vk1_0::Buffer) -> Self {
        self.0.buffer = buffer as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DedicatedAllocationMemoryAllocateInfoNV {
        self.0
    }
}
impl<'a> std::default::Default for DedicatedAllocationMemoryAllocateInfoNVBuilder<'a> {
    fn default() -> DedicatedAllocationMemoryAllocateInfoNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DedicatedAllocationMemoryAllocateInfoNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DedicatedAllocationMemoryAllocateInfoNVBuilder<'a> {
    type Target = DedicatedAllocationMemoryAllocateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DedicatedAllocationMemoryAllocateInfoNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
