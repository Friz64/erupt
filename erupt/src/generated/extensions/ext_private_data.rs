#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_PRIVATE_DATA_SPEC_VERSION")]
pub const EXT_PRIVATE_DATA_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_PRIVATE_DATA_EXTENSION_NAME")]
pub const EXT_PRIVATE_DATA_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_private_data");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_PRIVATE_DATA_SLOT_EXT: *const std::os::raw::c_char = crate::cstr!("vkCreatePrivateDataSlotEXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_DESTROY_PRIVATE_DATA_SLOT_EXT: *const std::os::raw::c_char = crate::cstr!("vkDestroyPrivateDataSlotEXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_SET_PRIVATE_DATA_EXT: *const std::os::raw::c_char = crate::cstr!("vkSetPrivateDataEXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_PRIVATE_DATA_EXT: *const std::os::raw::c_char = crate::cstr!("vkGetPrivateDataEXT");
crate::non_dispatchable_handle!(PrivateDataSlotEXT, PRIVATE_DATA_SLOT_EXT, "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPrivateDataSlotEXT.html) · Non-dispatchable Handle", "VkPrivateDataSlotEXT");
#[doc = "Provided by [`crate::extensions::ext_private_data`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES_EXT: Self = Self(1000295000);
    pub const DEVICE_PRIVATE_DATA_CREATE_INFO_EXT: Self = Self(1000295001);
    pub const PRIVATE_DATA_SLOT_CREATE_INFO_EXT: Self = Self(1000295002);
}
#[doc = "Provided by [`crate::extensions::ext_private_data`]"]
impl crate::vk1_0::ObjectType {
    pub const PRIVATE_DATA_SLOT_EXT: Self = Self(1000295000);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPrivateDataSlotCreateFlagsEXT.html) · Bitmask of [`PrivateDataSlotCreateFlagBitsEXT`]"] # [doc (alias = "VkPrivateDataSlotCreateFlagsEXT")] # [derive (Default)] # [repr (transparent)] pub struct PrivateDataSlotCreateFlagsEXT : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`PrivateDataSlotCreateFlagsEXT`]"]
#[doc(alias = "VkPrivateDataSlotCreateFlagBitsEXT")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct PrivateDataSlotCreateFlagBitsEXT(pub u32);
impl PrivateDataSlotCreateFlagBitsEXT {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> PrivateDataSlotCreateFlagsEXT {
        PrivateDataSlotCreateFlagsEXT::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for PrivateDataSlotCreateFlagBitsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreatePrivateDataSlotEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreatePrivateDataSlotEXT = unsafe extern "system" fn(device: crate::vk1_0::Device, p_create_info: *const crate::extensions::ext_private_data::PrivateDataSlotCreateInfoEXT, p_allocator: *const crate::vk1_0::AllocationCallbacks, p_private_data_slot: *mut crate::extensions::ext_private_data::PrivateDataSlotEXT) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyPrivateDataSlotEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyPrivateDataSlotEXT = unsafe extern "system" fn(device: crate::vk1_0::Device, private_data_slot: crate::extensions::ext_private_data::PrivateDataSlotEXT, p_allocator: *const crate::vk1_0::AllocationCallbacks) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSetPrivateDataEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkSetPrivateDataEXT = unsafe extern "system" fn(device: crate::vk1_0::Device, object_type: crate::vk1_0::ObjectType, object_handle: u64, private_data_slot: crate::extensions::ext_private_data::PrivateDataSlotEXT, data: u64) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPrivateDataEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPrivateDataEXT = unsafe extern "system" fn(device: crate::vk1_0::Device, object_type: crate::vk1_0::ObjectType, object_handle: u64, private_data_slot: crate::extensions::ext_private_data::PrivateDataSlotEXT, p_data: *mut u64) -> ();
impl<'a> crate::ExtendableFrom<'a, DevicePrivateDataCreateInfoEXT> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, DevicePrivateDataCreateInfoEXTBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDevicePrivateDataFeaturesEXT> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDevicePrivateDataFeaturesEXTBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDevicePrivateDataCreateInfoEXT.html) · Structure"]
#[doc(alias = "VkDevicePrivateDataCreateInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DevicePrivateDataCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub private_data_slot_request_count: u32,
}
impl DevicePrivateDataCreateInfoEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::DEVICE_PRIVATE_DATA_CREATE_INFO_EXT;
}
impl Default for DevicePrivateDataCreateInfoEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), private_data_slot_request_count: Default::default() }
    }
}
impl std::fmt::Debug for DevicePrivateDataCreateInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DevicePrivateDataCreateInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("private_data_slot_request_count", &self.private_data_slot_request_count).finish()
    }
}
impl DevicePrivateDataCreateInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> DevicePrivateDataCreateInfoEXTBuilder<'a> {
        DevicePrivateDataCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDevicePrivateDataCreateInfoEXT.html) · Builder of [`DevicePrivateDataCreateInfoEXT`]"]
#[repr(transparent)]
pub struct DevicePrivateDataCreateInfoEXTBuilder<'a>(DevicePrivateDataCreateInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> DevicePrivateDataCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> DevicePrivateDataCreateInfoEXTBuilder<'a> {
        DevicePrivateDataCreateInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn private_data_slot_request_count(mut self, private_data_slot_request_count: u32) -> Self {
        self.0.private_data_slot_request_count = private_data_slot_request_count as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DevicePrivateDataCreateInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for DevicePrivateDataCreateInfoEXTBuilder<'a> {
    fn default() -> DevicePrivateDataCreateInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DevicePrivateDataCreateInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DevicePrivateDataCreateInfoEXTBuilder<'a> {
    type Target = DevicePrivateDataCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DevicePrivateDataCreateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPrivateDataSlotCreateInfoEXT.html) · Structure"]
#[doc(alias = "VkPrivateDataSlotCreateInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PrivateDataSlotCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::ext_private_data::PrivateDataSlotCreateFlagsEXT,
}
impl PrivateDataSlotCreateInfoEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PRIVATE_DATA_SLOT_CREATE_INFO_EXT;
}
impl Default for PrivateDataSlotCreateInfoEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), flags: Default::default() }
    }
}
impl std::fmt::Debug for PrivateDataSlotCreateInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PrivateDataSlotCreateInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).finish()
    }
}
impl PrivateDataSlotCreateInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PrivateDataSlotCreateInfoEXTBuilder<'a> {
        PrivateDataSlotCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPrivateDataSlotCreateInfoEXT.html) · Builder of [`PrivateDataSlotCreateInfoEXT`]"]
#[repr(transparent)]
pub struct PrivateDataSlotCreateInfoEXTBuilder<'a>(PrivateDataSlotCreateInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PrivateDataSlotCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PrivateDataSlotCreateInfoEXTBuilder<'a> {
        PrivateDataSlotCreateInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::ext_private_data::PrivateDataSlotCreateFlagsEXT) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PrivateDataSlotCreateInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for PrivateDataSlotCreateInfoEXTBuilder<'a> {
    fn default() -> PrivateDataSlotCreateInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PrivateDataSlotCreateInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PrivateDataSlotCreateInfoEXTBuilder<'a> {
    type Target = PrivateDataSlotCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PrivateDataSlotCreateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevicePrivateDataFeaturesEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDevicePrivateDataFeaturesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDevicePrivateDataFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub private_data: crate::vk1_0::Bool32,
}
impl PhysicalDevicePrivateDataFeaturesEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES_EXT;
}
impl Default for PhysicalDevicePrivateDataFeaturesEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), private_data: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDevicePrivateDataFeaturesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDevicePrivateDataFeaturesEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("private_data", &(self.private_data != 0)).finish()
    }
}
impl PhysicalDevicePrivateDataFeaturesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDevicePrivateDataFeaturesEXTBuilder<'a> {
        PhysicalDevicePrivateDataFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevicePrivateDataFeaturesEXT.html) · Builder of [`PhysicalDevicePrivateDataFeaturesEXT`]"]
#[repr(transparent)]
pub struct PhysicalDevicePrivateDataFeaturesEXTBuilder<'a>(PhysicalDevicePrivateDataFeaturesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDevicePrivateDataFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDevicePrivateDataFeaturesEXTBuilder<'a> {
        PhysicalDevicePrivateDataFeaturesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn private_data(mut self, private_data: bool) -> Self {
        self.0.private_data = private_data as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDevicePrivateDataFeaturesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDevicePrivateDataFeaturesEXTBuilder<'a> {
    fn default() -> PhysicalDevicePrivateDataFeaturesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDevicePrivateDataFeaturesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDevicePrivateDataFeaturesEXTBuilder<'a> {
    type Target = PhysicalDevicePrivateDataFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDevicePrivateDataFeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<'a> crate::ExtendableFrom<'a, PhysicalDevicePrivateDataFeaturesEXT> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDevicePrivateDataFeaturesEXTBuilder<'_>> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
#[doc = "Provided by [`crate::extensions::ext_private_data`]"]
impl crate::DeviceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreatePrivateDataSlotEXT.html) · Function"]
    #[doc(alias = "vkCreatePrivateDataSlotEXT")]
    pub unsafe fn create_private_data_slot_ext(&self, create_info: &crate::extensions::ext_private_data::PrivateDataSlotCreateInfoEXT, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> crate::utils::VulkanResult<crate::extensions::ext_private_data::PrivateDataSlotEXT> {
        let _function = self.create_private_data_slot_ext.expect(crate::NOT_LOADED_MESSAGE);
        let mut private_data_slot = Default::default();
        let _return = _function(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut private_data_slot,
        );
        crate::utils::VulkanResult::new(_return, private_data_slot)
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyPrivateDataSlotEXT.html) · Function"]
    #[doc(alias = "vkDestroyPrivateDataSlotEXT")]
    pub unsafe fn destroy_private_data_slot_ext(&self, private_data_slot: Option<crate::extensions::ext_private_data::PrivateDataSlotEXT>, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> () {
        let _function = self.destroy_private_data_slot_ext.expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(
            self.handle,
            match private_data_slot {
                Some(v) => v,
                None => Default::default(),
            },
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSetPrivateDataEXT.html) · Function"]
    #[doc(alias = "vkSetPrivateDataEXT")]
    pub unsafe fn set_private_data_ext(&self, object_type: crate::vk1_0::ObjectType, object_handle: u64, private_data_slot: crate::extensions::ext_private_data::PrivateDataSlotEXT, data: u64) -> crate::utils::VulkanResult<()> {
        let _function = self.set_private_data_ext.expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(self.handle, object_type as _, object_handle as _, private_data_slot as _, data as _);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPrivateDataEXT.html) · Function"]
    #[doc(alias = "vkGetPrivateDataEXT")]
    pub unsafe fn get_private_data_ext(&self, object_type: crate::vk1_0::ObjectType, object_handle: u64, private_data_slot: crate::extensions::ext_private_data::PrivateDataSlotEXT) -> u64 {
        let _function = self.get_private_data_ext.expect(crate::NOT_LOADED_MESSAGE);
        let mut data = Default::default();
        let _return = _function(self.handle, object_type as _, object_handle as _, private_data_slot as _, &mut data);
        data
    }
}
