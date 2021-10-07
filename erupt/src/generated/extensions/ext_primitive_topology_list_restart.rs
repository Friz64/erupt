#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_PRIMITIVE_TOPOLOGY_LIST_RESTART_SPEC_VERSION")]
pub const EXT_PRIMITIVE_TOPOLOGY_LIST_RESTART_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_PRIMITIVE_TOPOLOGY_LIST_RESTART_EXTENSION_NAME")]
pub const EXT_PRIMITIVE_TOPOLOGY_LIST_RESTART_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_primitive_topology_list_restart");
#[doc = "Provided by [`crate::extensions::ext_primitive_topology_list_restart`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_PRIMITIVE_TOPOLOGY_LIST_RESTART_FEATURES_EXT: Self = Self(1000356000);
}
impl<'a> crate::ExtendableFrom<'a, PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDevicePrimitiveTopologyListRestartFeaturesEXTBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDevicePrimitiveTopologyListRestartFeaturesEXTBuilder<'_>> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevicePrimitiveTopologyListRestartFeaturesEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDevicePrimitiveTopologyListRestartFeaturesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub primitive_topology_list_restart: crate::vk1_0::Bool32,
    pub primitive_topology_patch_list_restart: crate::vk1_0::Bool32,
}
impl PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_PRIMITIVE_TOPOLOGY_LIST_RESTART_FEATURES_EXT;
}
impl Default for PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), primitive_topology_list_restart: Default::default(), primitive_topology_patch_list_restart: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("primitive_topology_list_restart", &(self.primitive_topology_list_restart != 0)).field("primitive_topology_patch_list_restart", &(self.primitive_topology_patch_list_restart != 0)).finish()
    }
}
impl PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDevicePrimitiveTopologyListRestartFeaturesEXTBuilder<'a> {
        PhysicalDevicePrimitiveTopologyListRestartFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevicePrimitiveTopologyListRestartFeaturesEXT.html) · Builder of [`PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT`]"]
#[repr(transparent)]
pub struct PhysicalDevicePrimitiveTopologyListRestartFeaturesEXTBuilder<'a>(PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDevicePrimitiveTopologyListRestartFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDevicePrimitiveTopologyListRestartFeaturesEXTBuilder<'a> {
        PhysicalDevicePrimitiveTopologyListRestartFeaturesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn primitive_topology_list_restart(mut self, primitive_topology_list_restart: bool) -> Self {
        self.0.primitive_topology_list_restart = primitive_topology_list_restart as _;
        self
    }
    #[inline]
    pub fn primitive_topology_patch_list_restart(mut self, primitive_topology_patch_list_restart: bool) -> Self {
        self.0.primitive_topology_patch_list_restart = primitive_topology_patch_list_restart as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDevicePrimitiveTopologyListRestartFeaturesEXTBuilder<'a> {
    fn default() -> PhysicalDevicePrimitiveTopologyListRestartFeaturesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDevicePrimitiveTopologyListRestartFeaturesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDevicePrimitiveTopologyListRestartFeaturesEXTBuilder<'a> {
    type Target = PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDevicePrimitiveTopologyListRestartFeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
