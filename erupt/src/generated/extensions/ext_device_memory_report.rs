#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_DEVICE_MEMORY_REPORT_SPEC_VERSION")]
pub const EXT_DEVICE_MEMORY_REPORT_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_DEVICE_MEMORY_REPORT_EXTENSION_NAME")]
pub const EXT_DEVICE_MEMORY_REPORT_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_device_memory_report");
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceMemoryReportFlagsEXT.html) · Bitmask of [`DeviceMemoryReportFlagBitsEXT`]"] # [doc (alias = "VkDeviceMemoryReportFlagsEXT")] # [derive (Default)] # [repr (transparent)] pub struct DeviceMemoryReportFlagsEXT : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`DeviceMemoryReportFlagsEXT`]"]
#[doc(alias = "VkDeviceMemoryReportFlagBitsEXT")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct DeviceMemoryReportFlagBitsEXT(pub u32);
impl DeviceMemoryReportFlagBitsEXT {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> DeviceMemoryReportFlagsEXT {
        DeviceMemoryReportFlagsEXT::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for DeviceMemoryReportFlagBitsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::ext_device_memory_report`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_DEVICE_MEMORY_REPORT_FEATURES_EXT: Self = Self(1000284000);
    pub const DEVICE_DEVICE_MEMORY_REPORT_CREATE_INFO_EXT: Self = Self(1000284001);
    pub const DEVICE_MEMORY_REPORT_CALLBACK_DATA_EXT: Self = Self(1000284002);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceMemoryReportEventTypeEXT.html) · Enum"]
#[doc(alias = "VkDeviceMemoryReportEventTypeEXT")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct DeviceMemoryReportEventTypeEXT(pub i32);
impl std::fmt::Debug for DeviceMemoryReportEventTypeEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::ALLOCATE_EXT => "ALLOCATE_EXT",
            &Self::FREE_EXT => "FREE_EXT",
            &Self::IMPORT_EXT => "IMPORT_EXT",
            &Self::UNIMPORT_EXT => "UNIMPORT_EXT",
            &Self::ALLOCATION_FAILED_EXT => "ALLOCATION_FAILED_EXT",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::ext_device_memory_report`]"]
impl crate::extensions::ext_device_memory_report::DeviceMemoryReportEventTypeEXT {
    pub const ALLOCATE_EXT: Self = Self(0);
    pub const FREE_EXT: Self = Self(1);
    pub const IMPORT_EXT: Self = Self(2);
    pub const UNIMPORT_EXT: Self = Self(3);
    pub const ALLOCATION_FAILED_EXT: Self = Self(4);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/PFN_vkDeviceMemoryReportCallbackEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkDeviceMemoryReportCallbackEXT = unsafe extern "system" fn(p_callback_data: *const crate::extensions::ext_device_memory_report::DeviceMemoryReportCallbackDataEXT, p_user_data: *mut std::ffi::c_void) -> ();
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceDeviceMemoryReportFeaturesEXT> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceDeviceMemoryReportFeaturesEXTBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, DeviceDeviceMemoryReportCreateInfoEXT> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, DeviceDeviceMemoryReportCreateInfoEXTBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceDeviceMemoryReportFeaturesEXT> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceDeviceMemoryReportFeaturesEXTBuilder<'_>> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDeviceMemoryReportFeaturesEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceDeviceMemoryReportFeaturesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceDeviceMemoryReportFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub device_memory_report: crate::vk1_0::Bool32,
}
impl PhysicalDeviceDeviceMemoryReportFeaturesEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_DEVICE_MEMORY_REPORT_FEATURES_EXT;
}
impl Default for PhysicalDeviceDeviceMemoryReportFeaturesEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), device_memory_report: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceDeviceMemoryReportFeaturesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceDeviceMemoryReportFeaturesEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("device_memory_report", &(self.device_memory_report != 0)).finish()
    }
}
impl PhysicalDeviceDeviceMemoryReportFeaturesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceDeviceMemoryReportFeaturesEXTBuilder<'a> {
        PhysicalDeviceDeviceMemoryReportFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDeviceMemoryReportFeaturesEXT.html) · Builder of [`PhysicalDeviceDeviceMemoryReportFeaturesEXT`]"]
#[repr(transparent)]
pub struct PhysicalDeviceDeviceMemoryReportFeaturesEXTBuilder<'a>(PhysicalDeviceDeviceMemoryReportFeaturesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceDeviceMemoryReportFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceDeviceMemoryReportFeaturesEXTBuilder<'a> {
        PhysicalDeviceDeviceMemoryReportFeaturesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn device_memory_report(mut self, device_memory_report: bool) -> Self {
        self.0.device_memory_report = device_memory_report as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceDeviceMemoryReportFeaturesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceDeviceMemoryReportFeaturesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceDeviceMemoryReportFeaturesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceDeviceMemoryReportFeaturesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceDeviceMemoryReportFeaturesEXTBuilder<'a> {
    type Target = PhysicalDeviceDeviceMemoryReportFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceDeviceMemoryReportFeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceDeviceMemoryReportCreateInfoEXT.html) · Structure"]
#[doc(alias = "VkDeviceDeviceMemoryReportCreateInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DeviceDeviceMemoryReportCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::ext_device_memory_report::DeviceMemoryReportFlagsEXT,
    pub pfn_user_callback: Option<crate::extensions::ext_device_memory_report::PFN_vkDeviceMemoryReportCallbackEXT>,
    pub p_user_data: *mut std::ffi::c_void,
}
impl DeviceDeviceMemoryReportCreateInfoEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::DEVICE_DEVICE_MEMORY_REPORT_CREATE_INFO_EXT;
}
impl Default for DeviceDeviceMemoryReportCreateInfoEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), flags: Default::default(), pfn_user_callback: Default::default(), p_user_data: std::ptr::null_mut() }
    }
}
impl std::fmt::Debug for DeviceDeviceMemoryReportCreateInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DeviceDeviceMemoryReportCreateInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("pfn_user_callback", unsafe { &std::mem::transmute::<_, *const ()>(self.pfn_user_callback) }).field("p_user_data", &self.p_user_data).finish()
    }
}
impl DeviceDeviceMemoryReportCreateInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> DeviceDeviceMemoryReportCreateInfoEXTBuilder<'a> {
        DeviceDeviceMemoryReportCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceDeviceMemoryReportCreateInfoEXT.html) · Builder of [`DeviceDeviceMemoryReportCreateInfoEXT`]"]
#[repr(transparent)]
pub struct DeviceDeviceMemoryReportCreateInfoEXTBuilder<'a>(DeviceDeviceMemoryReportCreateInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> DeviceDeviceMemoryReportCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> DeviceDeviceMemoryReportCreateInfoEXTBuilder<'a> {
        DeviceDeviceMemoryReportCreateInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::ext_device_memory_report::DeviceMemoryReportFlagsEXT) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn pfn_user_callback(mut self, pfn_user_callback: Option<crate::extensions::ext_device_memory_report::PFN_vkDeviceMemoryReportCallbackEXT>) -> Self {
        self.0.pfn_user_callback = pfn_user_callback as _;
        self
    }
    #[inline]
    pub fn user_data(mut self, user_data: *mut std::ffi::c_void) -> Self {
        self.0.p_user_data = user_data;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DeviceDeviceMemoryReportCreateInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for DeviceDeviceMemoryReportCreateInfoEXTBuilder<'a> {
    fn default() -> DeviceDeviceMemoryReportCreateInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DeviceDeviceMemoryReportCreateInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DeviceDeviceMemoryReportCreateInfoEXTBuilder<'a> {
    type Target = DeviceDeviceMemoryReportCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DeviceDeviceMemoryReportCreateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceMemoryReportCallbackDataEXT.html) · Structure"]
#[doc(alias = "VkDeviceMemoryReportCallbackDataEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DeviceMemoryReportCallbackDataEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub flags: crate::extensions::ext_device_memory_report::DeviceMemoryReportFlagsEXT,
    pub _type: crate::extensions::ext_device_memory_report::DeviceMemoryReportEventTypeEXT,
    pub memory_object_id: u64,
    pub size: crate::vk1_0::DeviceSize,
    pub object_type: crate::vk1_0::ObjectType,
    pub object_handle: u64,
    pub heap_index: u32,
}
impl DeviceMemoryReportCallbackDataEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::DEVICE_MEMORY_REPORT_CALLBACK_DATA_EXT;
}
impl Default for DeviceMemoryReportCallbackDataEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), flags: Default::default(), _type: Default::default(), memory_object_id: Default::default(), size: Default::default(), object_type: Default::default(), object_handle: Default::default(), heap_index: Default::default() }
    }
}
impl std::fmt::Debug for DeviceMemoryReportCallbackDataEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DeviceMemoryReportCallbackDataEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("_type", &self._type).field("memory_object_id", &self.memory_object_id).field("size", &self.size).field("object_type", &self.object_type).field("object_handle", &self.object_handle).field("heap_index", &self.heap_index).finish()
    }
}
impl DeviceMemoryReportCallbackDataEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> DeviceMemoryReportCallbackDataEXTBuilder<'a> {
        DeviceMemoryReportCallbackDataEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceMemoryReportCallbackDataEXT.html) · Builder of [`DeviceMemoryReportCallbackDataEXT`]"]
#[repr(transparent)]
pub struct DeviceMemoryReportCallbackDataEXTBuilder<'a>(DeviceMemoryReportCallbackDataEXT, std::marker::PhantomData<&'a ()>);
impl<'a> DeviceMemoryReportCallbackDataEXTBuilder<'a> {
    #[inline]
    pub fn new() -> DeviceMemoryReportCallbackDataEXTBuilder<'a> {
        DeviceMemoryReportCallbackDataEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::ext_device_memory_report::DeviceMemoryReportFlagsEXT) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn _type(mut self, _type: crate::extensions::ext_device_memory_report::DeviceMemoryReportEventTypeEXT) -> Self {
        self.0._type = _type as _;
        self
    }
    #[inline]
    pub fn memory_object_id(mut self, memory_object_id: u64) -> Self {
        self.0.memory_object_id = memory_object_id as _;
        self
    }
    #[inline]
    pub fn size(mut self, size: crate::vk1_0::DeviceSize) -> Self {
        self.0.size = size as _;
        self
    }
    #[inline]
    pub fn object_type(mut self, object_type: crate::vk1_0::ObjectType) -> Self {
        self.0.object_type = object_type as _;
        self
    }
    #[inline]
    pub fn object_handle(mut self, object_handle: u64) -> Self {
        self.0.object_handle = object_handle as _;
        self
    }
    #[inline]
    pub fn heap_index(mut self, heap_index: u32) -> Self {
        self.0.heap_index = heap_index as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DeviceMemoryReportCallbackDataEXT {
        self.0
    }
}
impl<'a> std::default::Default for DeviceMemoryReportCallbackDataEXTBuilder<'a> {
    fn default() -> DeviceMemoryReportCallbackDataEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DeviceMemoryReportCallbackDataEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DeviceMemoryReportCallbackDataEXTBuilder<'a> {
    type Target = DeviceMemoryReportCallbackDataEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DeviceMemoryReportCallbackDataEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
