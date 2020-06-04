# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_EXT_buffer_device_address.html)\n\n## Extends\n- [`BufferCreateFlagBits`](../../vk1_0/struct.BufferCreateFlagBits.html)\n- [`BufferUsageFlagBits`](../../vk1_0/struct.BufferUsageFlagBits.html)\n- [`Result`](../../vk1_0/struct.Result.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_BUFFER_DEVICE_ADDRESS_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_BUFFER_DEVICE_ADDRESS_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_buffer_device_address");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetBufferDeviceAddressEXT.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetBufferDeviceAddressEXT = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_info: *const crate::vk1_2::BufferDeviceAddressInfo,
)
    -> crate::vk1_0::DeviceAddress;
#[doc = "Provides Device Commands for [`ExtBufferDeviceAddressDeviceLoaderExt`](trait.ExtBufferDeviceAddressDeviceLoaderExt.html)"]
pub struct ExtBufferDeviceAddressDeviceCommands {
    pub get_buffer_device_address_ext: Option<PFN_vkGetBufferDeviceAddressEXT>,
}
impl ExtBufferDeviceAddressDeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<ExtBufferDeviceAddressDeviceCommands> {
        unsafe {
            let mut success = false;
            let table = ExtBufferDeviceAddressDeviceCommands {
                get_buffer_device_address_ext: std::mem::transmute({
                    let symbol = loader.symbol("vkGetBufferDeviceAddressEXT");
                    success |= symbol.is_some();
                    symbol
                }),
            };
            if success {
                Some(table)
            } else {
                None
            }
        }
    }
}
fn device_commands(loader: &crate::DeviceLoader) -> &ExtBufferDeviceAddressDeviceCommands {
    loader
        .ext_buffer_device_address
        .as_ref()
        .expect("`ext_buffer_device_address` not loaded")
}
#[doc = "Provides high level command wrappers for [`ExtBufferDeviceAddressDeviceCommands`](struct.ExtBufferDeviceAddressDeviceCommands.html)"]
pub trait ExtBufferDeviceAddressDeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetBufferDeviceAddressEXT.html) · Device Command"]
    unsafe fn get_buffer_device_address_ext(
        &self,
        info: &crate::vk1_2::BufferDeviceAddressInfo,
    ) -> crate::vk1_0::DeviceAddress;
}
impl ExtBufferDeviceAddressDeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetBufferDeviceAddressEXT.html) · Device Command"]
    unsafe fn get_buffer_device_address_ext(
        &self,
        info: &crate::vk1_2::BufferDeviceAddressInfo,
    ) -> crate::vk1_0::DeviceAddress {
        let function = device_commands(self)
            .get_buffer_device_address_ext
            .as_ref()
            .expect("`get_buffer_device_address_ext` not available");
        let _val = function(self.handle, info);
        _val
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceBufferAddressFeaturesEXT.html) · Alias"]
pub type PhysicalDeviceBufferAddressFeaturesEXT =
    crate::extensions::ext_buffer_device_address::PhysicalDeviceBufferDeviceAddressFeaturesEXT;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceBufferDeviceAddressFeaturesEXT.html) · Structure"]
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
    pub fn builder<'a>(self) -> PhysicalDeviceBufferDeviceAddressFeaturesEXTBuilder<'a> {
        PhysicalDeviceBufferDeviceAddressFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceBufferDeviceAddressFeaturesEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceBufferDeviceAddressFeaturesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("buffer_device_address", &(self.buffer_device_address != 0))
            .field(
                "buffer_device_address_capture_replay",
                &(self.buffer_device_address_capture_replay != 0),
            )
            .field(
                "buffer_device_address_multi_device",
                &(self.buffer_device_address_multi_device != 0),
            )
            .finish()
    }
}
impl Default for PhysicalDeviceBufferDeviceAddressFeaturesEXT {
    fn default() -> PhysicalDeviceBufferDeviceAddressFeaturesEXT {
        PhysicalDeviceBufferDeviceAddressFeaturesEXT {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            buffer_device_address: Default::default(),
            buffer_device_address_capture_replay: Default::default(),
            buffer_device_address_multi_device: Default::default(),
        }
    }
}
impl crate::ExtendableBy<PhysicalDeviceBufferDeviceAddressFeaturesEXT>
    for crate::vk1_1::PhysicalDeviceFeatures2
{
}
impl crate::ExtendableBy<PhysicalDeviceBufferDeviceAddressFeaturesEXT>
    for crate::vk1_0::DeviceCreateInfo
{
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceBufferDeviceAddressFeaturesEXT.html) · Builder of [`PhysicalDeviceBufferDeviceAddressFeaturesEXT`](struct.PhysicalDeviceBufferDeviceAddressFeaturesEXT.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceBufferDeviceAddressFeaturesEXTBuilder<'a>(
    PhysicalDeviceBufferDeviceAddressFeaturesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceBufferDeviceAddressFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceBufferDeviceAddressFeaturesEXTBuilder<'a> {
        PhysicalDeviceBufferDeviceAddressFeaturesEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn buffer_device_address(mut self, buffer_device_address: bool) -> Self {
        self.0.buffer_device_address = buffer_device_address as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn buffer_device_address_capture_replay(
        mut self,
        buffer_device_address_capture_replay: bool,
    ) -> Self {
        self.0.buffer_device_address_capture_replay = buffer_device_address_capture_replay as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn buffer_device_address_multi_device(
        mut self,
        buffer_device_address_multi_device: bool,
    ) -> Self {
        self.0.buffer_device_address_multi_device = buffer_device_address_multi_device as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceBufferDeviceAddressFeaturesEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceBufferDeviceAddressFeaturesEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferDeviceAddressInfoEXT.html) · Alias"]
pub type BufferDeviceAddressInfoEXT = crate::vk1_2::BufferDeviceAddressInfo;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferDeviceAddressCreateInfoEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BufferDeviceAddressCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub device_address: crate::vk1_0::DeviceAddress,
}
impl BufferDeviceAddressCreateInfoEXT {
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
    pub fn builder<'a>(self) -> BufferDeviceAddressCreateInfoEXTBuilder<'a> {
        BufferDeviceAddressCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for BufferDeviceAddressCreateInfoEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("BufferDeviceAddressCreateInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("device_address", &self.device_address)
            .finish()
    }
}
impl Default for BufferDeviceAddressCreateInfoEXT {
    fn default() -> BufferDeviceAddressCreateInfoEXT {
        BufferDeviceAddressCreateInfoEXT {
            s_type: crate::vk1_0::StructureType::BUFFER_DEVICE_ADDRESS_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            device_address: Default::default(),
        }
    }
}
impl crate::ExtendableBy<BufferDeviceAddressCreateInfoEXT> for crate::vk1_0::BufferCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferDeviceAddressCreateInfoEXT.html) · Builder of [`BufferDeviceAddressCreateInfoEXT`](struct.BufferDeviceAddressCreateInfoEXT.html)"]
#[repr(transparent)]
pub struct BufferDeviceAddressCreateInfoEXTBuilder<'a>(
    BufferDeviceAddressCreateInfoEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> BufferDeviceAddressCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> BufferDeviceAddressCreateInfoEXTBuilder<'a> {
        BufferDeviceAddressCreateInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn device_address(mut self, device_address: crate::vk1_0::DeviceAddress) -> Self {
        self.0.device_address = device_address;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> BufferDeviceAddressCreateInfoEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for BufferDeviceAddressCreateInfoEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
