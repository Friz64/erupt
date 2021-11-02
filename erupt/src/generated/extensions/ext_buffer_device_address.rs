#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_BUFFER_DEVICE_ADDRESS_SPEC_VERSION")]
pub const EXT_BUFFER_DEVICE_ADDRESS_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_BUFFER_DEVICE_ADDRESS_EXTENSION_NAME")]
pub const EXT_BUFFER_DEVICE_ADDRESS_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_buffer_device_address");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_BUFFER_DEVICE_ADDRESS_EXT: *const std::os::raw::c_char = crate::cstr!("vkGetBufferDeviceAddressEXT");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceBufferAddressFeaturesEXT.html) · Alias"]
#[doc(alias = "VkPhysicalDeviceBufferAddressFeaturesEXT")]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceBufferAddressFeaturesEXT = crate::extensions::ext_buffer_device_address::PhysicalDeviceBufferDeviceAddressFeaturesEXT;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceBufferAddressFeaturesEXT.html) · Alias"]
#[doc(alias = "VkPhysicalDeviceBufferAddressFeaturesEXT")]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceBufferAddressFeaturesEXTBuilder<'a> = crate::extensions::ext_buffer_device_address::PhysicalDeviceBufferDeviceAddressFeaturesEXTBuilder<'a>;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferDeviceAddressInfoEXT.html) · Alias"]
#[doc(alias = "VkBufferDeviceAddressInfoEXT")]
#[allow(non_camel_case_types)]
pub type BufferDeviceAddressInfoEXT = crate::vk1_2::BufferDeviceAddressInfo;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferDeviceAddressInfoEXT.html) · Alias"]
#[doc(alias = "VkBufferDeviceAddressInfoEXT")]
#[allow(non_camel_case_types)]
pub type BufferDeviceAddressInfoEXTBuilder<'a> = crate::vk1_2::BufferDeviceAddressInfoBuilder<'a>;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetBufferDeviceAddressEXT.html) · Alias"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetBufferDeviceAddressEXT = crate::vk1_2::PFN_vkGetBufferDeviceAddress;
#[doc = "Provided by [`crate::extensions::ext_buffer_device_address`]"]
impl crate::vk1_0::BufferCreateFlagBits {
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY_EXT: Self = Self::DEVICE_ADDRESS_CAPTURE_REPLAY;
}
#[doc = "Provided by [`crate::extensions::ext_buffer_device_address`]"]
impl crate::vk1_0::BufferUsageFlagBits {
    pub const SHADER_DEVICE_ADDRESS_EXT: Self = Self::SHADER_DEVICE_ADDRESS;
}
#[doc = "Provided by [`crate::extensions::ext_buffer_device_address`]"]
impl crate::vk1_0::Result {
    pub const ERROR_INVALID_DEVICE_ADDRESS_EXT: Self = Self::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS;
}
#[doc = "Provided by [`crate::extensions::ext_buffer_device_address`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT: Self = Self(1000244000);
    pub const BUFFER_DEVICE_ADDRESS_CREATE_INFO_EXT: Self = Self(1000244002);
    pub const PHYSICAL_DEVICE_BUFFER_ADDRESS_FEATURES_EXT: Self = Self::PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT;
    pub const BUFFER_DEVICE_ADDRESS_INFO_EXT: Self = Self::BUFFER_DEVICE_ADDRESS_INFO;
}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceBufferDeviceAddressFeaturesEXT> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceBufferDeviceAddressFeaturesEXTBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, BufferDeviceAddressCreateInfoEXT> for crate::vk1_0::BufferCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, BufferDeviceAddressCreateInfoEXTBuilder<'_>> for crate::vk1_0::BufferCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceBufferDeviceAddressFeaturesEXT> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceBufferDeviceAddressFeaturesEXTBuilder<'_>> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceBufferDeviceAddressFeaturesEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceBufferDeviceAddressFeaturesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceBufferDeviceAddressFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub buffer_device_address: crate::vk1_0::Bool32,
    pub buffer_device_address_capture_replay: crate::vk1_0::Bool32,
    pub buffer_device_address_multi_device: crate::vk1_0::Bool32,
}
impl PhysicalDeviceBufferDeviceAddressFeaturesEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT;
}
impl Default for PhysicalDeviceBufferDeviceAddressFeaturesEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), buffer_device_address: Default::default(), buffer_device_address_capture_replay: Default::default(), buffer_device_address_multi_device: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceBufferDeviceAddressFeaturesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceBufferDeviceAddressFeaturesEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("buffer_device_address", &(self.buffer_device_address != 0)).field("buffer_device_address_capture_replay", &(self.buffer_device_address_capture_replay != 0)).field("buffer_device_address_multi_device", &(self.buffer_device_address_multi_device != 0)).finish()
    }
}
impl PhysicalDeviceBufferDeviceAddressFeaturesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceBufferDeviceAddressFeaturesEXTBuilder<'a> {
        PhysicalDeviceBufferDeviceAddressFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceBufferDeviceAddressFeaturesEXT.html) · Builder of [`PhysicalDeviceBufferDeviceAddressFeaturesEXT`]"]
#[repr(transparent)]
pub struct PhysicalDeviceBufferDeviceAddressFeaturesEXTBuilder<'a>(PhysicalDeviceBufferDeviceAddressFeaturesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceBufferDeviceAddressFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceBufferDeviceAddressFeaturesEXTBuilder<'a> {
        PhysicalDeviceBufferDeviceAddressFeaturesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn buffer_device_address(mut self, buffer_device_address: bool) -> Self {
        self.0.buffer_device_address = buffer_device_address as _;
        self
    }
    #[inline]
    pub fn buffer_device_address_capture_replay(mut self, buffer_device_address_capture_replay: bool) -> Self {
        self.0.buffer_device_address_capture_replay = buffer_device_address_capture_replay as _;
        self
    }
    #[inline]
    pub fn buffer_device_address_multi_device(mut self, buffer_device_address_multi_device: bool) -> Self {
        self.0.buffer_device_address_multi_device = buffer_device_address_multi_device as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> PhysicalDeviceBufferDeviceAddressFeaturesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceBufferDeviceAddressFeaturesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceBufferDeviceAddressFeaturesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceBufferDeviceAddressFeaturesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceBufferDeviceAddressFeaturesEXTBuilder<'a> {
    type Target = PhysicalDeviceBufferDeviceAddressFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceBufferDeviceAddressFeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferDeviceAddressCreateInfoEXT.html) · Structure"]
#[doc(alias = "VkBufferDeviceAddressCreateInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BufferDeviceAddressCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub device_address: crate::vk1_0::DeviceAddress,
}
impl BufferDeviceAddressCreateInfoEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::BUFFER_DEVICE_ADDRESS_CREATE_INFO_EXT;
}
impl Default for BufferDeviceAddressCreateInfoEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), device_address: Default::default() }
    }
}
impl std::fmt::Debug for BufferDeviceAddressCreateInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BufferDeviceAddressCreateInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("device_address", &self.device_address).finish()
    }
}
impl BufferDeviceAddressCreateInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> BufferDeviceAddressCreateInfoEXTBuilder<'a> {
        BufferDeviceAddressCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferDeviceAddressCreateInfoEXT.html) · Builder of [`BufferDeviceAddressCreateInfoEXT`]"]
#[repr(transparent)]
pub struct BufferDeviceAddressCreateInfoEXTBuilder<'a>(BufferDeviceAddressCreateInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> BufferDeviceAddressCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> BufferDeviceAddressCreateInfoEXTBuilder<'a> {
        BufferDeviceAddressCreateInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn device_address(mut self, device_address: crate::vk1_0::DeviceAddress) -> Self {
        self.0.device_address = device_address as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> BufferDeviceAddressCreateInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for BufferDeviceAddressCreateInfoEXTBuilder<'a> {
    fn default() -> BufferDeviceAddressCreateInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for BufferDeviceAddressCreateInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for BufferDeviceAddressCreateInfoEXTBuilder<'a> {
    type Target = BufferDeviceAddressCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for BufferDeviceAddressCreateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::ext_buffer_device_address`]"]
impl crate::DeviceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetBufferDeviceAddressEXT.html) · Function"]
    #[doc(alias = "vkGetBufferDeviceAddressEXT")]
    pub unsafe fn get_buffer_device_address_ext(&self, info: &crate::vk1_2::BufferDeviceAddressInfo) -> crate::vk1_0::DeviceAddress {
        let _function = self.get_buffer_device_address_ext.expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(self.handle, info as _);
        _return
    }
}
