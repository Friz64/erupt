# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_NV_dedicated_allocation_image_aliasing.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
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
impl PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV,
    {
        unsafe {
            crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
        }
    }
    #[inline]
    pub fn builder<'a>(
        self,
    ) -> PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNVBuilder<'a> {
        PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNVBuilder(
            self,
            std::marker::PhantomData,
        )
    }
}
impl std::fmt::Debug for PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "dedicated_allocation_image_aliasing",
                &(self.dedicated_allocation_image_aliasing != 0),
            )
            .finish()
    }
}
impl Default for PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {
    fn default() -> PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {
        PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV { s_type : crate :: vk1_0 :: StructureType :: PHYSICAL_DEVICE_DEDICATED_ALLOCATION_IMAGE_ALIASING_FEATURES_NV , p_next : std :: ptr :: null_mut ( ) , dedicated_allocation_image_aliasing : Default :: default ( ) , }
    }
}
#[doc = "Used by [`PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV::extend`](struct.PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {}
impl ExtendableByPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV
    for crate::vk1_1::PhysicalDeviceFeatures2
{
}
impl ExtendableByPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV
    for crate::vk1_0::DeviceCreateInfo
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV`](struct.PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV.html)"]
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
    #[allow(unused_mut)]
    #[inline]
    pub fn dedicated_allocation_image_aliasing(
        mut self,
        dedicated_allocation_image_aliasing: bool,
    ) -> Self {
        self.0.dedicated_allocation_image_aliasing = dedicated_allocation_image_aliasing as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNVBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
