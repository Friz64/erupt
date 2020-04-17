# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_AMD_memory_overallocation_behavior.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const AMD_MEMORY_OVERALLOCATION_BEHAVIOR_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const AMD_MEMORY_OVERALLOCATION_BEHAVIOR_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_AMD_memory_overallocation_behavior");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryOverallocationBehaviorAMD.html) · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct MemoryOverallocationBehaviorAMD(pub i32);
#[doc = "[Part of `extensions::amd_memory_overallocation_behavior`](../../extensions/amd_memory_overallocation_behavior/index.html)"]
impl MemoryOverallocationBehaviorAMD {
    pub const DEFAULT_AMD: Self = Self(0);
    pub const ALLOWED_AMD: Self = Self(1);
    pub const DISALLOWED_AMD: Self = Self(2);
}
impl std::fmt::Debug for MemoryOverallocationBehaviorAMD {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::DEFAULT_AMD => "DEFAULT_AMD",
            &Self::ALLOWED_AMD => "ALLOWED_AMD",
            &Self::DISALLOWED_AMD => "DISALLOWED_AMD",
            _ => "Unknown enum variant",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceMemoryOverallocationCreateInfoAMD.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DeviceMemoryOverallocationCreateInfoAMD {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub overallocation_behavior:
        crate::extensions::amd_memory_overallocation_behavior::MemoryOverallocationBehaviorAMD,
}
impl DeviceMemoryOverallocationCreateInfoAMD {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByDeviceMemoryOverallocationCreateInfoAMD,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> DeviceMemoryOverallocationCreateInfoAMDBuilder<'a> {
        DeviceMemoryOverallocationCreateInfoAMDBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for DeviceMemoryOverallocationCreateInfoAMD {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("DeviceMemoryOverallocationCreateInfoAMD")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("overallocation_behavior", &self.overallocation_behavior)
            .finish()
    }
}
impl Default for DeviceMemoryOverallocationCreateInfoAMD {
    fn default() -> DeviceMemoryOverallocationCreateInfoAMD {
        DeviceMemoryOverallocationCreateInfoAMD {
            s_type: crate::vk1_0::StructureType::DEVICE_MEMORY_OVERALLOCATION_CREATE_INFO_AMD,
            p_next: std::ptr::null(),
            overallocation_behavior: Default::default(),
        }
    }
}
#[doc = "Used by [`DeviceMemoryOverallocationCreateInfoAMD::extend`](struct.DeviceMemoryOverallocationCreateInfoAMD.html#method.extend)"]
pub trait ExtendableByDeviceMemoryOverallocationCreateInfoAMD {}
impl ExtendableByDeviceMemoryOverallocationCreateInfoAMD for crate::vk1_0::DeviceCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`DeviceMemoryOverallocationCreateInfoAMD`](struct.DeviceMemoryOverallocationCreateInfoAMD.html)"]
#[repr(transparent)]
pub struct DeviceMemoryOverallocationCreateInfoAMDBuilder<'a>(
    DeviceMemoryOverallocationCreateInfoAMD,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> DeviceMemoryOverallocationCreateInfoAMDBuilder<'a> {
    #[inline]
    pub fn new() -> DeviceMemoryOverallocationCreateInfoAMDBuilder<'a> {
        DeviceMemoryOverallocationCreateInfoAMDBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn overallocation_behavior(
        mut self,
        overallocation_behavior : crate :: extensions :: amd_memory_overallocation_behavior :: MemoryOverallocationBehaviorAMD,
    ) -> Self {
        self.0.overallocation_behavior = overallocation_behavior;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> DeviceMemoryOverallocationCreateInfoAMD {
        self.0
    }
}
impl<'a> std::fmt::Debug for DeviceMemoryOverallocationCreateInfoAMDBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
