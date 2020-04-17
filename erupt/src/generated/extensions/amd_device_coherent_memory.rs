# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_AMD_device_coherent_memory.html)\n\n## Extends\n- [`MemoryPropertyFlagBits`](../../vk1_0/struct.MemoryPropertyFlagBits.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const AMD_DEVICE_COHERENT_MEMORY_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const AMD_DEVICE_COHERENT_MEMORY_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_AMD_device_coherent_memory");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceCoherentMemoryFeaturesAMD.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceCoherentMemoryFeaturesAMD {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub device_coherent_memory: crate::vk1_0::Bool32,
}
impl PhysicalDeviceCoherentMemoryFeaturesAMD {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceCoherentMemoryFeaturesAMD,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceCoherentMemoryFeaturesAMDBuilder<'a> {
        PhysicalDeviceCoherentMemoryFeaturesAMDBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceCoherentMemoryFeaturesAMD {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceCoherentMemoryFeaturesAMD")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "device_coherent_memory",
                &(self.device_coherent_memory != 0),
            )
            .finish()
    }
}
impl Default for PhysicalDeviceCoherentMemoryFeaturesAMD {
    fn default() -> PhysicalDeviceCoherentMemoryFeaturesAMD {
        PhysicalDeviceCoherentMemoryFeaturesAMD {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_COHERENT_MEMORY_FEATURES_AMD,
            p_next: std::ptr::null_mut(),
            device_coherent_memory: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceCoherentMemoryFeaturesAMD::extend`](struct.PhysicalDeviceCoherentMemoryFeaturesAMD.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceCoherentMemoryFeaturesAMD {}
impl ExtendableByPhysicalDeviceCoherentMemoryFeaturesAMD for crate::vk1_1::PhysicalDeviceFeatures2 {}
impl ExtendableByPhysicalDeviceCoherentMemoryFeaturesAMD for crate::vk1_0::DeviceCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceCoherentMemoryFeaturesAMD`](struct.PhysicalDeviceCoherentMemoryFeaturesAMD.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceCoherentMemoryFeaturesAMDBuilder<'a>(
    PhysicalDeviceCoherentMemoryFeaturesAMD,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceCoherentMemoryFeaturesAMDBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceCoherentMemoryFeaturesAMDBuilder<'a> {
        PhysicalDeviceCoherentMemoryFeaturesAMDBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn device_coherent_memory(mut self, device_coherent_memory: bool) -> Self {
        self.0.device_coherent_memory = device_coherent_memory as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceCoherentMemoryFeaturesAMD {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceCoherentMemoryFeaturesAMDBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceCoherentMemoryFeaturesAMDBuilder<'a> {
    type Target = PhysicalDeviceCoherentMemoryFeaturesAMD;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceCoherentMemoryFeaturesAMDBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
