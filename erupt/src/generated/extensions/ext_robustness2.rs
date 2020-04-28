# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_EXT_robustness2.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_ROBUSTNESS_2_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_ROBUSTNESS_2_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_robustness2");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceRobustness2FeaturesEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceRobustness2FeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub robust_buffer_access2: crate::vk1_0::Bool32,
    pub robust_image_access2: crate::vk1_0::Bool32,
    pub null_descriptor: crate::vk1_0::Bool32,
}
impl PhysicalDeviceRobustness2FeaturesEXT {
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
    pub fn builder<'a>(self) -> PhysicalDeviceRobustness2FeaturesEXTBuilder<'a> {
        PhysicalDeviceRobustness2FeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceRobustness2FeaturesEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceRobustness2FeaturesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("robust_buffer_access2", &(self.robust_buffer_access2 != 0))
            .field("robust_image_access2", &(self.robust_image_access2 != 0))
            .field("null_descriptor", &(self.null_descriptor != 0))
            .finish()
    }
}
impl Default for PhysicalDeviceRobustness2FeaturesEXT {
    fn default() -> PhysicalDeviceRobustness2FeaturesEXT {
        PhysicalDeviceRobustness2FeaturesEXT {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            robust_buffer_access2: Default::default(),
            robust_image_access2: Default::default(),
            null_descriptor: Default::default(),
        }
    }
}
impl crate::ExtendableBy<PhysicalDeviceRobustness2FeaturesEXT>
    for crate::vk1_1::PhysicalDeviceFeatures2
{
}
impl crate::ExtendableBy<PhysicalDeviceRobustness2FeaturesEXT> for crate::vk1_0::DeviceCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceRobustness2FeaturesEXT.html) · Builder of [`PhysicalDeviceRobustness2FeaturesEXT`](struct.PhysicalDeviceRobustness2FeaturesEXT.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceRobustness2FeaturesEXTBuilder<'a>(
    PhysicalDeviceRobustness2FeaturesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceRobustness2FeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceRobustness2FeaturesEXTBuilder<'a> {
        PhysicalDeviceRobustness2FeaturesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn robust_buffer_access2(mut self, robust_buffer_access2: bool) -> Self {
        self.0.robust_buffer_access2 = robust_buffer_access2 as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn robust_image_access2(mut self, robust_image_access2: bool) -> Self {
        self.0.robust_image_access2 = robust_image_access2 as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn null_descriptor(mut self, null_descriptor: bool) -> Self {
        self.0.null_descriptor = null_descriptor as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceRobustness2FeaturesEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceRobustness2FeaturesEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceRobustness2PropertiesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub robust_storage_buffer_access_size_alignment: crate::vk1_0::DeviceSize,
    pub robust_uniform_buffer_access_size_alignment: crate::vk1_0::DeviceSize,
}
impl PhysicalDeviceRobustness2PropertiesEXT {
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
    pub fn builder<'a>(self) -> PhysicalDeviceRobustness2PropertiesEXTBuilder<'a> {
        PhysicalDeviceRobustness2PropertiesEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceRobustness2PropertiesEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceRobustness2PropertiesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "robust_storage_buffer_access_size_alignment",
                &self.robust_storage_buffer_access_size_alignment,
            )
            .field(
                "robust_uniform_buffer_access_size_alignment",
                &self.robust_uniform_buffer_access_size_alignment,
            )
            .finish()
    }
}
impl Default for PhysicalDeviceRobustness2PropertiesEXT {
    fn default() -> PhysicalDeviceRobustness2PropertiesEXT {
        PhysicalDeviceRobustness2PropertiesEXT {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_ROBUSTNESS_2_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            robust_storage_buffer_access_size_alignment: Default::default(),
            robust_uniform_buffer_access_size_alignment: Default::default(),
        }
    }
}
impl crate::ExtendableBy<PhysicalDeviceRobustness2PropertiesEXT>
    for crate::vk1_1::PhysicalDeviceProperties2
{
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceRobustness2PropertiesEXT.html) · Builder of [`PhysicalDeviceRobustness2PropertiesEXT`](struct.PhysicalDeviceRobustness2PropertiesEXT.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceRobustness2PropertiesEXTBuilder<'a>(
    PhysicalDeviceRobustness2PropertiesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceRobustness2PropertiesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceRobustness2PropertiesEXTBuilder<'a> {
        PhysicalDeviceRobustness2PropertiesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn robust_storage_buffer_access_size_alignment(
        mut self,
        robust_storage_buffer_access_size_alignment: crate::vk1_0::DeviceSize,
    ) -> Self {
        self.0.robust_storage_buffer_access_size_alignment =
            robust_storage_buffer_access_size_alignment;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn robust_uniform_buffer_access_size_alignment(
        mut self,
        robust_uniform_buffer_access_size_alignment: crate::vk1_0::DeviceSize,
    ) -> Self {
        self.0.robust_uniform_buffer_access_size_alignment =
            robust_uniform_buffer_access_size_alignment;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceRobustness2PropertiesEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceRobustness2PropertiesEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
