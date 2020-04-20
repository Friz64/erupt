# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_EXT_index_type_uint8.html)\n\n## Extends\n- [`IndexType`](../../vk1_0/struct.IndexType.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_INDEX_TYPE_UINT8_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_INDEX_TYPE_UINT8_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_index_type_uint8");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceIndexTypeUint8FeaturesEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceIndexTypeUint8FeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub index_type_uint8: crate::vk1_0::Bool32,
}
impl PhysicalDeviceIndexTypeUint8FeaturesEXT {
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
    pub fn builder<'a>(self) -> PhysicalDeviceIndexTypeUint8FeaturesEXTBuilder<'a> {
        PhysicalDeviceIndexTypeUint8FeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceIndexTypeUint8FeaturesEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceIndexTypeUint8FeaturesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("index_type_uint8", &(self.index_type_uint8 != 0))
            .finish()
    }
}
impl Default for PhysicalDeviceIndexTypeUint8FeaturesEXT {
    fn default() -> PhysicalDeviceIndexTypeUint8FeaturesEXT {
        PhysicalDeviceIndexTypeUint8FeaturesEXT {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            index_type_uint8: Default::default(),
        }
    }
}
impl crate::ExtendableBy<PhysicalDeviceIndexTypeUint8FeaturesEXT>
    for crate::vk1_1::PhysicalDeviceFeatures2
{
}
impl crate::ExtendableBy<PhysicalDeviceIndexTypeUint8FeaturesEXT>
    for crate::vk1_0::DeviceCreateInfo
{
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceIndexTypeUint8FeaturesEXT.html) · Builder of [`PhysicalDeviceIndexTypeUint8FeaturesEXT`](struct.PhysicalDeviceIndexTypeUint8FeaturesEXT.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceIndexTypeUint8FeaturesEXTBuilder<'a>(
    PhysicalDeviceIndexTypeUint8FeaturesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceIndexTypeUint8FeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceIndexTypeUint8FeaturesEXTBuilder<'a> {
        PhysicalDeviceIndexTypeUint8FeaturesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn index_type_uint8(mut self, index_type_uint8: bool) -> Self {
        self.0.index_type_uint8 = index_type_uint8 as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceIndexTypeUint8FeaturesEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceIndexTypeUint8FeaturesEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceIndexTypeUint8FeaturesEXTBuilder<'a> {
    type Target = PhysicalDeviceIndexTypeUint8FeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceIndexTypeUint8FeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
