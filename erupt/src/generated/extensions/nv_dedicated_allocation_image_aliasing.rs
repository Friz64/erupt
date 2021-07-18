#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_DEDICATED_ALLOCATION_IMAGE_ALIASING_SPEC_VERSION")]
pub const NV_DEDICATED_ALLOCATION_IMAGE_ALIASING_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_DEDICATED_ALLOCATION_IMAGE_ALIASING_EXTENSION_NAME")]
pub const NV_DEDICATED_ALLOCATION_IMAGE_ALIASING_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_NV_dedicated_allocation_image_aliasing");
#[doc = "Provided by [`crate::extensions::nv_dedicated_allocation_image_aliasing`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_DEDICATED_ALLOCATION_IMAGE_ALIASING_FEATURES_NV: Self = Self(1000240000);
}
impl<'a> crate::ExtendableFromConst<'a, PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNVBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNVBuilder<'_>> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub dedicated_allocation_image_aliasing: crate::vk1_0::Bool32,
}
impl PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_DEDICATED_ALLOCATION_IMAGE_ALIASING_FEATURES_NV;
}
impl Default for PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), dedicated_allocation_image_aliasing: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV").field("s_type", &self.s_type).field("p_next", &self.p_next).field("dedicated_allocation_image_aliasing", &(self.dedicated_allocation_image_aliasing != 0)).finish()
    }
}
impl PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNVBuilder<'a> {
        PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV.html) · Builder of [`PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV`]"]
#[repr(transparent)]
pub struct PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNVBuilder<'a>(PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNVBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNVBuilder<'a> {
        PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn dedicated_allocation_image_aliasing(mut self, dedicated_allocation_image_aliasing: bool) -> Self {
        self.0.dedicated_allocation_image_aliasing = dedicated_allocation_image_aliasing as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNVBuilder<'a> {
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
impl<'a> std::ops::DerefMut for PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
