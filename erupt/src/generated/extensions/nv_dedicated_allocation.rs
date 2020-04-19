# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_NV_dedicated_allocation.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NV_DEDICATED_ALLOCATION_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NV_DEDICATED_ALLOCATION_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_NV_dedicated_allocation");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDedicatedAllocationImageCreateInfoNV.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DedicatedAllocationImageCreateInfoNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub dedicated_allocation: crate::vk1_0::Bool32,
}
impl DedicatedAllocationImageCreateInfoNV {
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
    pub fn builder<'a>(self) -> DedicatedAllocationImageCreateInfoNVBuilder<'a> {
        DedicatedAllocationImageCreateInfoNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for DedicatedAllocationImageCreateInfoNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("DedicatedAllocationImageCreateInfoNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("dedicated_allocation", &(self.dedicated_allocation != 0))
            .finish()
    }
}
impl Default for DedicatedAllocationImageCreateInfoNV {
    fn default() -> DedicatedAllocationImageCreateInfoNV {
        DedicatedAllocationImageCreateInfoNV {
            s_type: crate::vk1_0::StructureType::DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV,
            p_next: std::ptr::null(),
            dedicated_allocation: Default::default(),
        }
    }
}
impl crate::ExtendableBy<DedicatedAllocationImageCreateInfoNV> for crate::vk1_0::ImageCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`DedicatedAllocationImageCreateInfoNV`](struct.DedicatedAllocationImageCreateInfoNV.html)"]
#[repr(transparent)]
pub struct DedicatedAllocationImageCreateInfoNVBuilder<'a>(
    DedicatedAllocationImageCreateInfoNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> DedicatedAllocationImageCreateInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> DedicatedAllocationImageCreateInfoNVBuilder<'a> {
        DedicatedAllocationImageCreateInfoNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn dedicated_allocation(mut self, dedicated_allocation: bool) -> Self {
        self.0.dedicated_allocation = dedicated_allocation as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> DedicatedAllocationImageCreateInfoNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for DedicatedAllocationImageCreateInfoNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DedicatedAllocationBufferCreateInfoNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub dedicated_allocation: crate::vk1_0::Bool32,
}
impl DedicatedAllocationBufferCreateInfoNV {
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
    pub fn builder<'a>(self) -> DedicatedAllocationBufferCreateInfoNVBuilder<'a> {
        DedicatedAllocationBufferCreateInfoNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for DedicatedAllocationBufferCreateInfoNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("DedicatedAllocationBufferCreateInfoNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("dedicated_allocation", &(self.dedicated_allocation != 0))
            .finish()
    }
}
impl Default for DedicatedAllocationBufferCreateInfoNV {
    fn default() -> DedicatedAllocationBufferCreateInfoNV {
        DedicatedAllocationBufferCreateInfoNV {
            s_type: crate::vk1_0::StructureType::DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV,
            p_next: std::ptr::null(),
            dedicated_allocation: Default::default(),
        }
    }
}
impl crate::ExtendableBy<DedicatedAllocationBufferCreateInfoNV> for crate::vk1_0::BufferCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`DedicatedAllocationBufferCreateInfoNV`](struct.DedicatedAllocationBufferCreateInfoNV.html)"]
#[repr(transparent)]
pub struct DedicatedAllocationBufferCreateInfoNVBuilder<'a>(
    DedicatedAllocationBufferCreateInfoNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> DedicatedAllocationBufferCreateInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> DedicatedAllocationBufferCreateInfoNVBuilder<'a> {
        DedicatedAllocationBufferCreateInfoNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn dedicated_allocation(mut self, dedicated_allocation: bool) -> Self {
        self.0.dedicated_allocation = dedicated_allocation as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> DedicatedAllocationBufferCreateInfoNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for DedicatedAllocationBufferCreateInfoNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DedicatedAllocationMemoryAllocateInfoNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub image: crate::vk1_0::Image,
    pub buffer: crate::vk1_0::Buffer,
}
impl DedicatedAllocationMemoryAllocateInfoNV {
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
    pub fn builder<'a>(self) -> DedicatedAllocationMemoryAllocateInfoNVBuilder<'a> {
        DedicatedAllocationMemoryAllocateInfoNVBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for DedicatedAllocationMemoryAllocateInfoNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("DedicatedAllocationMemoryAllocateInfoNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("image", &self.image)
            .field("buffer", &self.buffer)
            .finish()
    }
}
impl Default for DedicatedAllocationMemoryAllocateInfoNV {
    fn default() -> DedicatedAllocationMemoryAllocateInfoNV {
        DedicatedAllocationMemoryAllocateInfoNV {
            s_type: crate::vk1_0::StructureType::DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV,
            p_next: std::ptr::null(),
            image: Default::default(),
            buffer: Default::default(),
        }
    }
}
impl crate::ExtendableBy<DedicatedAllocationMemoryAllocateInfoNV>
    for crate::vk1_0::MemoryAllocateInfo
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`DedicatedAllocationMemoryAllocateInfoNV`](struct.DedicatedAllocationMemoryAllocateInfoNV.html)"]
#[repr(transparent)]
pub struct DedicatedAllocationMemoryAllocateInfoNVBuilder<'a>(
    DedicatedAllocationMemoryAllocateInfoNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> DedicatedAllocationMemoryAllocateInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> DedicatedAllocationMemoryAllocateInfoNVBuilder<'a> {
        DedicatedAllocationMemoryAllocateInfoNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn image(mut self, image: crate::vk1_0::Image) -> Self {
        self.0.image = image;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn buffer(mut self, buffer: crate::vk1_0::Buffer) -> Self {
        self.0.buffer = buffer;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> DedicatedAllocationMemoryAllocateInfoNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for DedicatedAllocationMemoryAllocateInfoNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
