#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NV_DEDICATED_ALLOCATION_IMAGE_ALIASING_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NV_DEDICATED_ALLOCATION_IMAGE_ALIASING_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_NV_dedicated_allocation_image_aliasing");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub dedicated_allocation_image_aliasing: crate::vk1_0::Bool32,
}
impl Default for PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {
    fn default() -> Self {
        Self { s_type : crate :: vk1_0 :: StructureType :: PHYSICAL_DEVICE_DEDICATED_ALLOCATION_IMAGE_ALIASING_FEATURES_NV , p_next : std :: ptr :: null_mut () , dedicated_allocation_image_aliasing : Default :: default () }
    }
}
impl std::fmt::Debug for PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "dedicated_allocation_image_aliasing",
                &(self.dedicated_allocation_image_aliasing != 0),
            )
            .finish()
    }
}
impl PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {
    #[inline]
    pub fn into_builder<'a>(
        self,
    ) -> PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNVBuilder<'a> {
        PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNVBuilder(
            self,
            std::marker::PhantomData,
        )
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV.html) · Builder of [`PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV`](struct.PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNVBuilder<'a>(
    PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNVBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNVBuilder<'a> {
        PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNVBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[inline]
    pub fn dedicated_allocation_image_aliasing(
        mut self,
        dedicated_allocation_image_aliasing: bool,
    ) -> Self {
        self.0.dedicated_allocation_image_aliasing = dedicated_allocation_image_aliasing as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {
        self.0
    }
}
impl<'a> std::default::Default
    for PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNVBuilder<'a>
{
    fn default() -> PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNVBuilder<'a> {
    type Target = PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut
    for PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNVBuilder<'a>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
