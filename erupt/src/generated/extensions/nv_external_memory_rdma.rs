#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_EXTERNAL_MEMORY_RDMA_SPEC_VERSION")]
pub const NV_EXTERNAL_MEMORY_RDMA_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_EXTERNAL_MEMORY_RDMA_EXTENSION_NAME")]
pub const NV_EXTERNAL_MEMORY_RDMA_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_NV_external_memory_rdma");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_MEMORY_REMOTE_ADDRESS_NV: *const std::os::raw::c_char = crate::cstr!("vkGetMemoryRemoteAddressNV");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRemoteAddressNV.html) · Basetype"]
#[doc(alias = "VkRemoteAddressNV")]
pub type RemoteAddressNV = *mut std::ffi::c_void;
#[doc = "Provided by [`crate::extensions::nv_external_memory_rdma`]"]
impl crate::vk1_0::MemoryPropertyFlagBits {
    pub const RDMA_CAPABLE_NV: Self = Self(256);
}
#[doc = "Provided by [`crate::extensions::nv_external_memory_rdma`]"]
impl crate::vk1_0::StructureType {
    pub const MEMORY_GET_REMOTE_ADDRESS_INFO_NV: Self = Self(1000371000);
    pub const PHYSICAL_DEVICE_EXTERNAL_MEMORY_RDMA_FEATURES_NV: Self = Self(1000371001);
}
#[doc = "Provided by [`crate::extensions::nv_external_memory_rdma`]"]
impl crate::vk1_1::ExternalMemoryHandleTypeFlagBits {
    pub const RDMA_ADDRESS_NV: Self = Self(4096);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryRemoteAddressNV.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetMemoryRemoteAddressNV = unsafe extern "system" fn(device: crate::vk1_0::Device, get_memory_remote_address_info: *const crate::extensions::nv_external_memory_rdma::MemoryGetRemoteAddressInfoNV, p_address: *mut crate::extensions::nv_external_memory_rdma::RemoteAddressNV) -> crate::vk1_0::Result;
impl<'a> crate::ExtendableFromConst<'a, PhysicalDeviceExternalMemoryRDMAFeaturesNV> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, PhysicalDeviceExternalMemoryRDMAFeaturesNVBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceExternalMemoryRDMAFeaturesNV> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceExternalMemoryRDMAFeaturesNVBuilder<'_>> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceExternalMemoryRDMAFeaturesNV.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceExternalMemoryRDMAFeaturesNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceExternalMemoryRDMAFeaturesNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub external_memory_rdma: crate::vk1_0::Bool32,
}
impl PhysicalDeviceExternalMemoryRDMAFeaturesNV {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_EXTERNAL_MEMORY_RDMA_FEATURES_NV;
}
impl Default for PhysicalDeviceExternalMemoryRDMAFeaturesNV {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_EXTERNAL_MEMORY_RDMA_FEATURES_NV, p_next: std::ptr::null_mut(), external_memory_rdma: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceExternalMemoryRDMAFeaturesNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceExternalMemoryRDMAFeaturesNV").field("s_type", &self.s_type).field("p_next", &self.p_next).field("external_memory_rdma", &(self.external_memory_rdma != 0)).finish()
    }
}
impl PhysicalDeviceExternalMemoryRDMAFeaturesNV {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceExternalMemoryRDMAFeaturesNVBuilder<'a> {
        PhysicalDeviceExternalMemoryRDMAFeaturesNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceExternalMemoryRDMAFeaturesNV.html) · Builder of [`PhysicalDeviceExternalMemoryRDMAFeaturesNV`]"]
#[repr(transparent)]
pub struct PhysicalDeviceExternalMemoryRDMAFeaturesNVBuilder<'a>(PhysicalDeviceExternalMemoryRDMAFeaturesNV, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceExternalMemoryRDMAFeaturesNVBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceExternalMemoryRDMAFeaturesNVBuilder<'a> {
        PhysicalDeviceExternalMemoryRDMAFeaturesNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn external_memory_rdma(mut self, external_memory_rdma: bool) -> Self {
        self.0.external_memory_rdma = external_memory_rdma as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceExternalMemoryRDMAFeaturesNV {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceExternalMemoryRDMAFeaturesNVBuilder<'a> {
    fn default() -> PhysicalDeviceExternalMemoryRDMAFeaturesNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceExternalMemoryRDMAFeaturesNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceExternalMemoryRDMAFeaturesNVBuilder<'a> {
    type Target = PhysicalDeviceExternalMemoryRDMAFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceExternalMemoryRDMAFeaturesNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryGetRemoteAddressInfoNV.html) · Structure"]
#[doc(alias = "VkMemoryGetRemoteAddressInfoNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryGetRemoteAddressInfoNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub memory: crate::vk1_0::DeviceMemory,
    pub handle_type: crate::vk1_1::ExternalMemoryHandleTypeFlagBits,
}
impl MemoryGetRemoteAddressInfoNV {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::MEMORY_GET_REMOTE_ADDRESS_INFO_NV;
}
impl Default for MemoryGetRemoteAddressInfoNV {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::MEMORY_GET_REMOTE_ADDRESS_INFO_NV, p_next: std::ptr::null(), memory: Default::default(), handle_type: Default::default() }
    }
}
impl std::fmt::Debug for MemoryGetRemoteAddressInfoNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MemoryGetRemoteAddressInfoNV").field("s_type", &self.s_type).field("p_next", &self.p_next).field("memory", &self.memory).field("handle_type", &self.handle_type).finish()
    }
}
impl MemoryGetRemoteAddressInfoNV {
    #[inline]
    pub fn into_builder<'a>(self) -> MemoryGetRemoteAddressInfoNVBuilder<'a> {
        MemoryGetRemoteAddressInfoNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryGetRemoteAddressInfoNV.html) · Builder of [`MemoryGetRemoteAddressInfoNV`]"]
#[repr(transparent)]
pub struct MemoryGetRemoteAddressInfoNVBuilder<'a>(MemoryGetRemoteAddressInfoNV, std::marker::PhantomData<&'a ()>);
impl<'a> MemoryGetRemoteAddressInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> MemoryGetRemoteAddressInfoNVBuilder<'a> {
        MemoryGetRemoteAddressInfoNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn memory(mut self, memory: crate::vk1_0::DeviceMemory) -> Self {
        self.0.memory = memory as _;
        self
    }
    #[inline]
    pub fn handle_type(mut self, handle_type: crate::vk1_1::ExternalMemoryHandleTypeFlagBits) -> Self {
        self.0.handle_type = handle_type as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> MemoryGetRemoteAddressInfoNV {
        self.0
    }
}
impl<'a> std::default::Default for MemoryGetRemoteAddressInfoNVBuilder<'a> {
    fn default() -> MemoryGetRemoteAddressInfoNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for MemoryGetRemoteAddressInfoNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for MemoryGetRemoteAddressInfoNVBuilder<'a> {
    type Target = MemoryGetRemoteAddressInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for MemoryGetRemoteAddressInfoNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::nv_external_memory_rdma`]"]
impl crate::DeviceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryRemoteAddressNV.html) · Function"]
    #[doc(alias = "vkGetMemoryRemoteAddressNV")]
    pub unsafe fn get_memory_remote_address_nv(&self, get_memory_remote_address_info: &crate::extensions::nv_external_memory_rdma::MemoryGetRemoteAddressInfoNV) -> crate::utils::VulkanResult<crate::extensions::nv_external_memory_rdma::RemoteAddressNV> {
        let _function = self.get_memory_remote_address_nv.expect(crate::NOT_LOADED_MESSAGE);
        let mut address = std::ptr::null_mut();
        let _return = _function(self.handle, get_memory_remote_address_info as _, &mut address);
        crate::utils::VulkanResult::new(_return, address)
    }
}
