# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_KHR_buffer_device_address.html)\n\n## Extends\n- [`BufferCreateFlagBits`](../../vk1_0/struct.BufferCreateFlagBits.html)\n- [`BufferUsageFlagBits`](../../vk1_0/struct.BufferUsageFlagBits.html)\n- [`MemoryAllocateFlagBits`](../../vk1_1/struct.MemoryAllocateFlagBits.html)\n- [`Result`](../../vk1_0/struct.Result.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_BUFFER_DEVICE_ADDRESS_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_BUFFER_DEVICE_ADDRESS_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_buffer_device_address");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetBufferDeviceAddressKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetBufferDeviceAddressKHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_info: *const crate::vk1_2::BufferDeviceAddressInfo,
)
    -> crate::vk1_0::DeviceAddress;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetBufferOpaqueCaptureAddressKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetBufferOpaqueCaptureAddressKHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_info: *const crate::vk1_2::BufferDeviceAddressInfo,
) -> u64;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceMemoryOpaqueCaptureAddressKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceMemoryOpaqueCaptureAddressKHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_info: *const crate::vk1_2::DeviceMemoryOpaqueCaptureAddressInfo,
) -> u64;
#[doc = "Provides Device Commands for [`KhrBufferDeviceAddressDeviceLoaderExt`](trait.KhrBufferDeviceAddressDeviceLoaderExt.html)"]
pub struct KhrBufferDeviceAddressDeviceCommands {
    pub get_buffer_device_address_khr: Option<PFN_vkGetBufferDeviceAddressKHR>,
    pub get_buffer_opaque_capture_address_khr: Option<PFN_vkGetBufferOpaqueCaptureAddressKHR>,
    pub get_device_memory_opaque_capture_address_khr:
        Option<PFN_vkGetDeviceMemoryOpaqueCaptureAddressKHR>,
}
impl KhrBufferDeviceAddressDeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<KhrBufferDeviceAddressDeviceCommands> {
        unsafe {
            let mut success = false;
            let table = KhrBufferDeviceAddressDeviceCommands {
                get_buffer_device_address_khr: std::mem::transmute({
                    let symbol = loader.symbol("vkGetBufferDeviceAddressKHR");
                    success |= symbol.is_some();
                    symbol
                }),
                get_buffer_opaque_capture_address_khr: std::mem::transmute({
                    let symbol = loader.symbol("vkGetBufferOpaqueCaptureAddressKHR");
                    success |= symbol.is_some();
                    symbol
                }),
                get_device_memory_opaque_capture_address_khr: std::mem::transmute({
                    let symbol = loader.symbol("vkGetDeviceMemoryOpaqueCaptureAddressKHR");
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
#[inline]
fn device_commands(loader: &crate::DeviceLoader) -> &KhrBufferDeviceAddressDeviceCommands {
    loader
        .khr_buffer_device_address
        .as_ref()
        .expect("`khr_buffer_device_address` not loaded")
}
#[doc = "Provides high level command wrappers for [`KhrBufferDeviceAddressDeviceCommands`](struct.KhrBufferDeviceAddressDeviceCommands.html)"]
pub trait KhrBufferDeviceAddressDeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetBufferDeviceAddressKHR.html) · Device Command"]
    unsafe fn get_buffer_device_address_khr(
        &self,
        info: &crate::vk1_2::BufferDeviceAddressInfo,
    ) -> crate::vk1_0::DeviceAddress;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetBufferOpaqueCaptureAddressKHR.html) · Device Command"]
    unsafe fn get_buffer_opaque_capture_address_khr(
        &self,
        info: &crate::vk1_2::BufferDeviceAddressInfo,
    ) -> u64;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceMemoryOpaqueCaptureAddressKHR.html) · Device Command"]
    unsafe fn get_device_memory_opaque_capture_address_khr(
        &self,
        info: &crate::vk1_2::DeviceMemoryOpaqueCaptureAddressInfo,
    ) -> u64;
}
impl KhrBufferDeviceAddressDeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetBufferDeviceAddressKHR.html) · Device Command"]
    unsafe fn get_buffer_device_address_khr(
        &self,
        info: &crate::vk1_2::BufferDeviceAddressInfo,
    ) -> crate::vk1_0::DeviceAddress {
        let function = device_commands(self)
            .get_buffer_device_address_khr
            .as_ref()
            .expect("`get_buffer_device_address_khr` not available");
        let _val = function(self.handle, info);
        _val
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetBufferOpaqueCaptureAddressKHR.html) · Device Command"]
    unsafe fn get_buffer_opaque_capture_address_khr(
        &self,
        info: &crate::vk1_2::BufferDeviceAddressInfo,
    ) -> u64 {
        let function = device_commands(self)
            .get_buffer_opaque_capture_address_khr
            .as_ref()
            .expect("`get_buffer_opaque_capture_address_khr` not available");
        let _val = function(self.handle, info);
        _val
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceMemoryOpaqueCaptureAddressKHR.html) · Device Command"]
    unsafe fn get_device_memory_opaque_capture_address_khr(
        &self,
        info: &crate::vk1_2::DeviceMemoryOpaqueCaptureAddressInfo,
    ) -> u64 {
        let function = device_commands(self)
            .get_device_memory_opaque_capture_address_khr
            .as_ref()
            .expect("`get_device_memory_opaque_capture_address_khr` not available");
        let _val = function(self.handle, info);
        _val
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceBufferDeviceAddressFeaturesKHR.html) · Alias"]
pub type PhysicalDeviceBufferDeviceAddressFeaturesKHR =
    crate::vk1_2::PhysicalDeviceBufferDeviceAddressFeatures;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferDeviceAddressInfoKHR.html) · Alias"]
pub type BufferDeviceAddressInfoKHR = crate::vk1_2::BufferDeviceAddressInfo;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferOpaqueCaptureAddressCreateInfoKHR.html) · Alias"]
pub type BufferOpaqueCaptureAddressCreateInfoKHR =
    crate::vk1_2::BufferOpaqueCaptureAddressCreateInfo;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryOpaqueCaptureAddressAllocateInfoKHR.html) · Alias"]
pub type MemoryOpaqueCaptureAddressAllocateInfoKHR =
    crate::vk1_2::MemoryOpaqueCaptureAddressAllocateInfo;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceMemoryOpaqueCaptureAddressInfoKHR.html) · Alias"]
pub type DeviceMemoryOpaqueCaptureAddressInfoKHR =
    crate::vk1_2::DeviceMemoryOpaqueCaptureAddressInfo;
