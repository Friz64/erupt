#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_INDEX_TYPE_UINT8_SPEC_VERSION")]
pub const EXT_INDEX_TYPE_UINT8_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_INDEX_TYPE_UINT8_EXTENSION_NAME")]
pub const EXT_INDEX_TYPE_UINT8_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_index_type_uint8");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceIndexTypeUint8FeaturesEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceIndexTypeUint8FeaturesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceIndexTypeUint8FeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub index_type_uint8: crate::vk1_0::Bool32,
}
impl Default for PhysicalDeviceIndexTypeUint8FeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            index_type_uint8: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceIndexTypeUint8FeaturesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceIndexTypeUint8FeaturesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("index_type_uint8", &(self.index_type_uint8 != 0))
            .finish()
    }
}
impl PhysicalDeviceIndexTypeUint8FeaturesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceIndexTypeUint8FeaturesEXTBuilder<'a> {
        PhysicalDeviceIndexTypeUint8FeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceIndexTypeUint8FeaturesEXT.html) · Builder of [`PhysicalDeviceIndexTypeUint8FeaturesEXT`]"]
#[repr(transparent)]
pub struct PhysicalDeviceIndexTypeUint8FeaturesEXTBuilder<'a>(PhysicalDeviceIndexTypeUint8FeaturesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceIndexTypeUint8FeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceIndexTypeUint8FeaturesEXTBuilder<'a> {
        PhysicalDeviceIndexTypeUint8FeaturesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn index_type_uint8(mut self, index_type_uint8: bool) -> Self {
        self.0.index_type_uint8 = index_type_uint8 as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceIndexTypeUint8FeaturesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceIndexTypeUint8FeaturesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceIndexTypeUint8FeaturesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceIndexTypeUint8FeaturesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
