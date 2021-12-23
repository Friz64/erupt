#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_VALVE_MUTABLE_DESCRIPTOR_TYPE_SPEC_VERSION")]
pub const VALVE_MUTABLE_DESCRIPTOR_TYPE_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_VALVE_MUTABLE_DESCRIPTOR_TYPE_EXTENSION_NAME")]
pub const VALVE_MUTABLE_DESCRIPTOR_TYPE_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_VALVE_mutable_descriptor_type");
#[doc = "Provided by [`crate::extensions::valve_mutable_descriptor_type`]"]
impl crate::vk1_0::DescriptorSetLayoutCreateFlagBits {
    pub const HOST_ONLY_POOL_VALVE: Self = Self(4);
}
#[doc = "Provided by [`crate::extensions::valve_mutable_descriptor_type`]"]
impl crate::vk1_0::DescriptorType {
    pub const MUTABLE_VALVE: Self = Self(1000351000);
}
#[doc = "Provided by [`crate::extensions::valve_mutable_descriptor_type`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_VALVE: Self = Self(1000351000);
    pub const MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_VALVE: Self = Self(1000351002);
}
#[doc = "Provided by [`crate::extensions::valve_mutable_descriptor_type`]"]
impl crate::vk1_0::DescriptorPoolCreateFlagBits {
    pub const HOST_ONLY_VALVE: Self = Self(4);
}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceMutableDescriptorTypeFeaturesVALVE> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceMutableDescriptorTypeFeaturesVALVEBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, MutableDescriptorTypeCreateInfoVALVE> for crate::vk1_0::DescriptorSetLayoutCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, MutableDescriptorTypeCreateInfoVALVEBuilder<'_>> for crate::vk1_0::DescriptorSetLayoutCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, MutableDescriptorTypeCreateInfoVALVE> for crate::vk1_0::DescriptorPoolCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, MutableDescriptorTypeCreateInfoVALVEBuilder<'_>> for crate::vk1_0::DescriptorPoolCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceMutableDescriptorTypeFeaturesVALVE> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceMutableDescriptorTypeFeaturesVALVEBuilder<'_>> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMutableDescriptorTypeFeaturesVALVE.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceMutableDescriptorTypeFeaturesVALVE")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceMutableDescriptorTypeFeaturesVALVE {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub mutable_descriptor_type: crate::vk1_0::Bool32,
}
impl PhysicalDeviceMutableDescriptorTypeFeaturesVALVE {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_VALVE;
}
impl Default for PhysicalDeviceMutableDescriptorTypeFeaturesVALVE {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), mutable_descriptor_type: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceMutableDescriptorTypeFeaturesVALVE {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceMutableDescriptorTypeFeaturesVALVE").field("s_type", &self.s_type).field("p_next", &self.p_next).field("mutable_descriptor_type", &(self.mutable_descriptor_type != 0)).finish()
    }
}
impl PhysicalDeviceMutableDescriptorTypeFeaturesVALVE {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceMutableDescriptorTypeFeaturesVALVEBuilder<'a> {
        PhysicalDeviceMutableDescriptorTypeFeaturesVALVEBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMutableDescriptorTypeFeaturesVALVE.html) · Builder of [`PhysicalDeviceMutableDescriptorTypeFeaturesVALVE`]"]
#[repr(transparent)]
pub struct PhysicalDeviceMutableDescriptorTypeFeaturesVALVEBuilder<'a>(PhysicalDeviceMutableDescriptorTypeFeaturesVALVE, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceMutableDescriptorTypeFeaturesVALVEBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceMutableDescriptorTypeFeaturesVALVEBuilder<'a> {
        PhysicalDeviceMutableDescriptorTypeFeaturesVALVEBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn mutable_descriptor_type(mut self, mutable_descriptor_type: bool) -> Self {
        self.0.mutable_descriptor_type = mutable_descriptor_type as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> PhysicalDeviceMutableDescriptorTypeFeaturesVALVE {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceMutableDescriptorTypeFeaturesVALVEBuilder<'a> {
    fn default() -> PhysicalDeviceMutableDescriptorTypeFeaturesVALVEBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceMutableDescriptorTypeFeaturesVALVEBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceMutableDescriptorTypeFeaturesVALVEBuilder<'a> {
    type Target = PhysicalDeviceMutableDescriptorTypeFeaturesVALVE;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceMutableDescriptorTypeFeaturesVALVEBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMutableDescriptorTypeListVALVE.html) · Structure"]
#[doc(alias = "VkMutableDescriptorTypeListVALVE")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MutableDescriptorTypeListVALVE {
    pub descriptor_type_count: u32,
    pub p_descriptor_types: *const crate::vk1_0::DescriptorType,
}
impl Default for MutableDescriptorTypeListVALVE {
    fn default() -> Self {
        Self { descriptor_type_count: Default::default(), p_descriptor_types: std::ptr::null() }
    }
}
impl std::fmt::Debug for MutableDescriptorTypeListVALVE {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MutableDescriptorTypeListVALVE").field("descriptor_type_count", &self.descriptor_type_count).field("p_descriptor_types", &self.p_descriptor_types).finish()
    }
}
impl MutableDescriptorTypeListVALVE {
    #[inline]
    pub fn into_builder<'a>(self) -> MutableDescriptorTypeListVALVEBuilder<'a> {
        MutableDescriptorTypeListVALVEBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMutableDescriptorTypeListVALVE.html) · Builder of [`MutableDescriptorTypeListVALVE`]"]
#[repr(transparent)]
pub struct MutableDescriptorTypeListVALVEBuilder<'a>(MutableDescriptorTypeListVALVE, std::marker::PhantomData<&'a ()>);
impl<'a> MutableDescriptorTypeListVALVEBuilder<'a> {
    #[inline]
    pub fn new() -> MutableDescriptorTypeListVALVEBuilder<'a> {
        MutableDescriptorTypeListVALVEBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn descriptor_types(mut self, descriptor_types: &'a [crate::vk1_0::DescriptorType]) -> Self {
        self.0.p_descriptor_types = descriptor_types.as_ptr() as _;
        self.0.descriptor_type_count = descriptor_types.len() as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> MutableDescriptorTypeListVALVE {
        self.0
    }
}
impl<'a> std::default::Default for MutableDescriptorTypeListVALVEBuilder<'a> {
    fn default() -> MutableDescriptorTypeListVALVEBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for MutableDescriptorTypeListVALVEBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for MutableDescriptorTypeListVALVEBuilder<'a> {
    type Target = MutableDescriptorTypeListVALVE;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for MutableDescriptorTypeListVALVEBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMutableDescriptorTypeCreateInfoVALVE.html) · Structure"]
#[doc(alias = "VkMutableDescriptorTypeCreateInfoVALVE")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MutableDescriptorTypeCreateInfoVALVE {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub mutable_descriptor_type_list_count: u32,
    pub p_mutable_descriptor_type_lists: *const crate::extensions::valve_mutable_descriptor_type::MutableDescriptorTypeListVALVE,
}
impl MutableDescriptorTypeCreateInfoVALVE {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_VALVE;
}
impl Default for MutableDescriptorTypeCreateInfoVALVE {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), mutable_descriptor_type_list_count: Default::default(), p_mutable_descriptor_type_lists: std::ptr::null() }
    }
}
impl std::fmt::Debug for MutableDescriptorTypeCreateInfoVALVE {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MutableDescriptorTypeCreateInfoVALVE").field("s_type", &self.s_type).field("p_next", &self.p_next).field("mutable_descriptor_type_list_count", &self.mutable_descriptor_type_list_count).field("p_mutable_descriptor_type_lists", &self.p_mutable_descriptor_type_lists).finish()
    }
}
impl MutableDescriptorTypeCreateInfoVALVE {
    #[inline]
    pub fn into_builder<'a>(self) -> MutableDescriptorTypeCreateInfoVALVEBuilder<'a> {
        MutableDescriptorTypeCreateInfoVALVEBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMutableDescriptorTypeCreateInfoVALVE.html) · Builder of [`MutableDescriptorTypeCreateInfoVALVE`]"]
#[repr(transparent)]
pub struct MutableDescriptorTypeCreateInfoVALVEBuilder<'a>(MutableDescriptorTypeCreateInfoVALVE, std::marker::PhantomData<&'a ()>);
impl<'a> MutableDescriptorTypeCreateInfoVALVEBuilder<'a> {
    #[inline]
    pub fn new() -> MutableDescriptorTypeCreateInfoVALVEBuilder<'a> {
        MutableDescriptorTypeCreateInfoVALVEBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    #[must_use]
    pub fn mutable_descriptor_type_lists(mut self, mutable_descriptor_type_lists: &'a [crate::extensions::valve_mutable_descriptor_type::MutableDescriptorTypeListVALVEBuilder]) -> Self {
        self.0.p_mutable_descriptor_type_lists = mutable_descriptor_type_lists.as_ptr() as _;
        self.0.mutable_descriptor_type_list_count = mutable_descriptor_type_lists.len() as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> MutableDescriptorTypeCreateInfoVALVE {
        self.0
    }
}
impl<'a> std::default::Default for MutableDescriptorTypeCreateInfoVALVEBuilder<'a> {
    fn default() -> MutableDescriptorTypeCreateInfoVALVEBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for MutableDescriptorTypeCreateInfoVALVEBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for MutableDescriptorTypeCreateInfoVALVEBuilder<'a> {
    type Target = MutableDescriptorTypeCreateInfoVALVE;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for MutableDescriptorTypeCreateInfoVALVEBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
