#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_ROBUSTNESS_2_SPEC_VERSION")]
pub const EXT_ROBUSTNESS_2_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_ROBUSTNESS_2_EXTENSION_NAME")]
pub const EXT_ROBUSTNESS_2_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_robustness2");
#[doc = "Provided by [`crate::extensions::ext_robustness2`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_EXT: Self = Self(1000286000);
    pub const PHYSICAL_DEVICE_ROBUSTNESS_2_PROPERTIES_EXT: Self = Self(1000286001);
}
impl<'a> crate::ExtendableFromConst<'a, PhysicalDeviceRobustness2FeaturesEXT> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, PhysicalDeviceRobustness2FeaturesEXTBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceRobustness2FeaturesEXT> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceRobustness2FeaturesEXTBuilder<'_>> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceRobustness2PropertiesEXT> for crate::vk1_1::PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceRobustness2PropertiesEXTBuilder<'_>> for crate::vk1_1::PhysicalDeviceProperties2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceRobustness2FeaturesEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceRobustness2FeaturesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceRobustness2FeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub robust_buffer_access2: crate::vk1_0::Bool32,
    pub robust_image_access2: crate::vk1_0::Bool32,
    pub null_descriptor: crate::vk1_0::Bool32,
}
impl Default for PhysicalDeviceRobustness2FeaturesEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_EXT, p_next: std::ptr::null_mut(), robust_buffer_access2: Default::default(), robust_image_access2: Default::default(), null_descriptor: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceRobustness2FeaturesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceRobustness2FeaturesEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("robust_buffer_access2", &(self.robust_buffer_access2 != 0)).field("robust_image_access2", &(self.robust_image_access2 != 0)).field("null_descriptor", &(self.null_descriptor != 0)).finish()
    }
}
impl PhysicalDeviceRobustness2FeaturesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceRobustness2FeaturesEXTBuilder<'a> {
        PhysicalDeviceRobustness2FeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceRobustness2FeaturesEXT.html) · Builder of [`PhysicalDeviceRobustness2FeaturesEXT`]"]
#[repr(transparent)]
pub struct PhysicalDeviceRobustness2FeaturesEXTBuilder<'a>(PhysicalDeviceRobustness2FeaturesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceRobustness2FeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceRobustness2FeaturesEXTBuilder<'a> {
        PhysicalDeviceRobustness2FeaturesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn robust_buffer_access2(mut self, robust_buffer_access2: bool) -> Self {
        self.0.robust_buffer_access2 = robust_buffer_access2 as _;
        self
    }
    #[inline]
    pub fn robust_image_access2(mut self, robust_image_access2: bool) -> Self {
        self.0.robust_image_access2 = robust_image_access2 as _;
        self
    }
    #[inline]
    pub fn null_descriptor(mut self, null_descriptor: bool) -> Self {
        self.0.null_descriptor = null_descriptor as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceRobustness2FeaturesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceRobustness2FeaturesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceRobustness2FeaturesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceRobustness2FeaturesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceRobustness2FeaturesEXTBuilder<'a> {
    type Target = PhysicalDeviceRobustness2FeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceRobustness2FeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceRobustness2PropertiesEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceRobustness2PropertiesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceRobustness2PropertiesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub robust_storage_buffer_access_size_alignment: crate::vk1_0::DeviceSize,
    pub robust_uniform_buffer_access_size_alignment: crate::vk1_0::DeviceSize,
}
impl Default for PhysicalDeviceRobustness2PropertiesEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_ROBUSTNESS_2_PROPERTIES_EXT, p_next: std::ptr::null_mut(), robust_storage_buffer_access_size_alignment: Default::default(), robust_uniform_buffer_access_size_alignment: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceRobustness2PropertiesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceRobustness2PropertiesEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("robust_storage_buffer_access_size_alignment", &self.robust_storage_buffer_access_size_alignment).field("robust_uniform_buffer_access_size_alignment", &self.robust_uniform_buffer_access_size_alignment).finish()
    }
}
impl PhysicalDeviceRobustness2PropertiesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceRobustness2PropertiesEXTBuilder<'a> {
        PhysicalDeviceRobustness2PropertiesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceRobustness2PropertiesEXT.html) · Builder of [`PhysicalDeviceRobustness2PropertiesEXT`]"]
#[repr(transparent)]
pub struct PhysicalDeviceRobustness2PropertiesEXTBuilder<'a>(PhysicalDeviceRobustness2PropertiesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceRobustness2PropertiesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceRobustness2PropertiesEXTBuilder<'a> {
        PhysicalDeviceRobustness2PropertiesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn robust_storage_buffer_access_size_alignment(mut self, robust_storage_buffer_access_size_alignment: crate::vk1_0::DeviceSize) -> Self {
        self.0.robust_storage_buffer_access_size_alignment = robust_storage_buffer_access_size_alignment as _;
        self
    }
    #[inline]
    pub fn robust_uniform_buffer_access_size_alignment(mut self, robust_uniform_buffer_access_size_alignment: crate::vk1_0::DeviceSize) -> Self {
        self.0.robust_uniform_buffer_access_size_alignment = robust_uniform_buffer_access_size_alignment as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceRobustness2PropertiesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceRobustness2PropertiesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceRobustness2PropertiesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceRobustness2PropertiesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceRobustness2PropertiesEXTBuilder<'a> {
    type Target = PhysicalDeviceRobustness2PropertiesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceRobustness2PropertiesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
