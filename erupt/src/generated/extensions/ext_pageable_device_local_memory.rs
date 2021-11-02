#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_PAGEABLE_DEVICE_LOCAL_MEMORY_SPEC_VERSION")]
pub const EXT_PAGEABLE_DEVICE_LOCAL_MEMORY_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_PAGEABLE_DEVICE_LOCAL_MEMORY_EXTENSION_NAME")]
pub const EXT_PAGEABLE_DEVICE_LOCAL_MEMORY_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_pageable_device_local_memory");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_SET_DEVICE_MEMORY_PRIORITY_EXT: *const std::os::raw::c_char = crate::cstr!("vkSetDeviceMemoryPriorityEXT");
#[doc = "Provided by [`crate::extensions::ext_pageable_device_local_memory`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_PAGEABLE_DEVICE_LOCAL_MEMORY_FEATURES_EXT: Self = Self(1000412000);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSetDeviceMemoryPriorityEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkSetDeviceMemoryPriorityEXT = unsafe extern "system" fn(device: crate::vk1_0::Device, memory: crate::vk1_0::DeviceMemory, priority: std::os::raw::c_float) -> ();
impl<'a> crate::ExtendableFrom<'a, PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDevicePageableDeviceLocalMemoryFeaturesEXTBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDevicePageableDeviceLocalMemoryFeaturesEXTBuilder<'_>> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevicePageableDeviceLocalMemoryFeaturesEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDevicePageableDeviceLocalMemoryFeaturesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub pageable_device_local_memory: crate::vk1_0::Bool32,
}
impl PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_PAGEABLE_DEVICE_LOCAL_MEMORY_FEATURES_EXT;
}
impl Default for PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), pageable_device_local_memory: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("pageable_device_local_memory", &(self.pageable_device_local_memory != 0)).finish()
    }
}
impl PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDevicePageableDeviceLocalMemoryFeaturesEXTBuilder<'a> {
        PhysicalDevicePageableDeviceLocalMemoryFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevicePageableDeviceLocalMemoryFeaturesEXT.html) · Builder of [`PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT`]"]
#[repr(transparent)]
pub struct PhysicalDevicePageableDeviceLocalMemoryFeaturesEXTBuilder<'a>(PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDevicePageableDeviceLocalMemoryFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDevicePageableDeviceLocalMemoryFeaturesEXTBuilder<'a> {
        PhysicalDevicePageableDeviceLocalMemoryFeaturesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn pageable_device_local_memory(mut self, pageable_device_local_memory: bool) -> Self {
        self.0.pageable_device_local_memory = pageable_device_local_memory as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDevicePageableDeviceLocalMemoryFeaturesEXTBuilder<'a> {
    fn default() -> PhysicalDevicePageableDeviceLocalMemoryFeaturesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDevicePageableDeviceLocalMemoryFeaturesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDevicePageableDeviceLocalMemoryFeaturesEXTBuilder<'a> {
    type Target = PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDevicePageableDeviceLocalMemoryFeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::ext_pageable_device_local_memory`]"]
impl crate::DeviceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSetDeviceMemoryPriorityEXT.html) · Function"]
    #[doc(alias = "vkSetDeviceMemoryPriorityEXT")]
    pub unsafe fn set_device_memory_priority_ext(&self, memory: crate::vk1_0::DeviceMemory, priority: std::os::raw::c_float) -> () {
        let _function = self.set_device_memory_priority_ext.expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(self.handle, memory as _, priority as _);
        ()
    }
}
