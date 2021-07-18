#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_AMD_MEMORY_OVERALLOCATION_BEHAVIOR_SPEC_VERSION")]
pub const AMD_MEMORY_OVERALLOCATION_BEHAVIOR_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_AMD_MEMORY_OVERALLOCATION_BEHAVIOR_EXTENSION_NAME")]
pub const AMD_MEMORY_OVERALLOCATION_BEHAVIOR_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_AMD_memory_overallocation_behavior");
#[doc = "Provided by [`crate::extensions::amd_memory_overallocation_behavior`]"]
impl crate::vk1_0::StructureType {
    pub const DEVICE_MEMORY_OVERALLOCATION_CREATE_INFO_AMD: Self = Self(1000189000);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryOverallocationBehaviorAMD.html) · Enum"]
#[doc(alias = "VkMemoryOverallocationBehaviorAMD")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct MemoryOverallocationBehaviorAMD(pub i32);
impl std::fmt::Debug for MemoryOverallocationBehaviorAMD {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::DEFAULT_AMD => "DEFAULT_AMD",
            &Self::ALLOWED_AMD => "ALLOWED_AMD",
            &Self::DISALLOWED_AMD => "DISALLOWED_AMD",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::amd_memory_overallocation_behavior`]"]
impl crate::extensions::amd_memory_overallocation_behavior::MemoryOverallocationBehaviorAMD {
    pub const DEFAULT_AMD: Self = Self(0);
    pub const ALLOWED_AMD: Self = Self(1);
    pub const DISALLOWED_AMD: Self = Self(2);
}
impl<'a> crate::ExtendableFromConst<'a, DeviceMemoryOverallocationCreateInfoAMD> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, DeviceMemoryOverallocationCreateInfoAMDBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceMemoryOverallocationCreateInfoAMD.html) · Structure"]
#[doc(alias = "VkDeviceMemoryOverallocationCreateInfoAMD")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DeviceMemoryOverallocationCreateInfoAMD {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub overallocation_behavior: crate::extensions::amd_memory_overallocation_behavior::MemoryOverallocationBehaviorAMD,
}
impl DeviceMemoryOverallocationCreateInfoAMD {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::DEVICE_MEMORY_OVERALLOCATION_CREATE_INFO_AMD;
}
impl Default for DeviceMemoryOverallocationCreateInfoAMD {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::DEVICE_MEMORY_OVERALLOCATION_CREATE_INFO_AMD, p_next: std::ptr::null(), overallocation_behavior: Default::default() }
    }
}
impl std::fmt::Debug for DeviceMemoryOverallocationCreateInfoAMD {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DeviceMemoryOverallocationCreateInfoAMD").field("s_type", &self.s_type).field("p_next", &self.p_next).field("overallocation_behavior", &self.overallocation_behavior).finish()
    }
}
impl DeviceMemoryOverallocationCreateInfoAMD {
    #[inline]
    pub fn into_builder<'a>(self) -> DeviceMemoryOverallocationCreateInfoAMDBuilder<'a> {
        DeviceMemoryOverallocationCreateInfoAMDBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceMemoryOverallocationCreateInfoAMD.html) · Builder of [`DeviceMemoryOverallocationCreateInfoAMD`]"]
#[repr(transparent)]
pub struct DeviceMemoryOverallocationCreateInfoAMDBuilder<'a>(DeviceMemoryOverallocationCreateInfoAMD, std::marker::PhantomData<&'a ()>);
impl<'a> DeviceMemoryOverallocationCreateInfoAMDBuilder<'a> {
    #[inline]
    pub fn new() -> DeviceMemoryOverallocationCreateInfoAMDBuilder<'a> {
        DeviceMemoryOverallocationCreateInfoAMDBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn overallocation_behavior(mut self, overallocation_behavior: crate::extensions::amd_memory_overallocation_behavior::MemoryOverallocationBehaviorAMD) -> Self {
        self.0.overallocation_behavior = overallocation_behavior as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DeviceMemoryOverallocationCreateInfoAMD {
        self.0
    }
}
impl<'a> std::default::Default for DeviceMemoryOverallocationCreateInfoAMDBuilder<'a> {
    fn default() -> DeviceMemoryOverallocationCreateInfoAMDBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DeviceMemoryOverallocationCreateInfoAMDBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DeviceMemoryOverallocationCreateInfoAMDBuilder<'a> {
    type Target = DeviceMemoryOverallocationCreateInfoAMD;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DeviceMemoryOverallocationCreateInfoAMDBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
