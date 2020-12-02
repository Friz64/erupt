#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_AMD_DEVICE_COHERENT_MEMORY_SPEC_VERSION")]
pub const AMD_DEVICE_COHERENT_MEMORY_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_AMD_DEVICE_COHERENT_MEMORY_EXTENSION_NAME")]
pub const AMD_DEVICE_COHERENT_MEMORY_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_AMD_device_coherent_memory");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceCoherentMemoryFeaturesAMD.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceCoherentMemoryFeaturesAMD")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceCoherentMemoryFeaturesAMD {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub device_coherent_memory: crate::vk1_0::Bool32,
}
impl Default for PhysicalDeviceCoherentMemoryFeaturesAMD {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_COHERENT_MEMORY_FEATURES_AMD,
            p_next: std::ptr::null_mut(),
            device_coherent_memory: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceCoherentMemoryFeaturesAMD {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceCoherentMemoryFeaturesAMD")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "device_coherent_memory",
                &(self.device_coherent_memory != 0),
            )
            .finish()
    }
}
impl PhysicalDeviceCoherentMemoryFeaturesAMD {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceCoherentMemoryFeaturesAMDBuilder<'a> {
        PhysicalDeviceCoherentMemoryFeaturesAMDBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceCoherentMemoryFeaturesAMD.html) · Builder of [`PhysicalDeviceCoherentMemoryFeaturesAMD`](struct.PhysicalDeviceCoherentMemoryFeaturesAMD.html)"]
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
    #[inline]
    pub fn device_coherent_memory(mut self, device_coherent_memory: bool) -> Self {
        self.0.device_coherent_memory = device_coherent_memory as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceCoherentMemoryFeaturesAMD {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceCoherentMemoryFeaturesAMDBuilder<'a> {
    fn default() -> PhysicalDeviceCoherentMemoryFeaturesAMDBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceCoherentMemoryFeaturesAMDBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
