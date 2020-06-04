# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_NVX_image_view_handle.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NVX_IMAGE_VIEW_HANDLE_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NVX_IMAGE_VIEW_HANDLE_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_NVX_image_view_handle");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageViewHandleNVX.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageViewHandleNVX = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_info: *const crate::extensions::nvx_image_view_handle::ImageViewHandleInfoNVX,
) -> u32;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageViewAddressNVX.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageViewAddressNVX = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    image_view: crate::vk1_0::ImageView,
    p_properties: *mut crate::extensions::nvx_image_view_handle::ImageViewAddressPropertiesNVX,
) -> crate::vk1_0::Result;
#[doc = "Provides Device Commands for [`NvxImageViewHandleDeviceLoaderExt`](trait.NvxImageViewHandleDeviceLoaderExt.html)"]
pub struct NvxImageViewHandleDeviceCommands {
    pub get_image_view_handle_nvx: Option<PFN_vkGetImageViewHandleNVX>,
    pub get_image_view_address_nvx: Option<PFN_vkGetImageViewAddressNVX>,
}
impl NvxImageViewHandleDeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<NvxImageViewHandleDeviceCommands> {
        unsafe {
            let mut success = false;
            let table = NvxImageViewHandleDeviceCommands {
                get_image_view_handle_nvx: std::mem::transmute({
                    let symbol = loader.symbol("vkGetImageViewHandleNVX");
                    success |= symbol.is_some();
                    symbol
                }),
                get_image_view_address_nvx: std::mem::transmute({
                    let symbol = loader.symbol("vkGetImageViewAddressNVX");
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
fn device_commands(loader: &crate::DeviceLoader) -> &NvxImageViewHandleDeviceCommands {
    loader
        .nvx_image_view_handle
        .as_ref()
        .expect("`nvx_image_view_handle` not loaded")
}
#[doc = "Provides high level command wrappers for [`NvxImageViewHandleDeviceCommands`](struct.NvxImageViewHandleDeviceCommands.html)"]
pub trait NvxImageViewHandleDeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageViewHandleNVX.html) · Device Command"]
    unsafe fn get_image_view_handle_nvx(
        &self,
        info: &crate::extensions::nvx_image_view_handle::ImageViewHandleInfoNVX,
    ) -> u32;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageViewAddressNVX.html) · Device Command"]
    unsafe fn get_image_view_address_nvx(
        &self,
        image_view: crate::vk1_0::ImageView,
        properties: Option<crate::extensions::nvx_image_view_handle::ImageViewAddressPropertiesNVX>,
    ) -> crate::utils::VulkanResult<
        crate::extensions::nvx_image_view_handle::ImageViewAddressPropertiesNVX,
    >;
}
impl NvxImageViewHandleDeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageViewHandleNVX.html) · Device Command"]
    unsafe fn get_image_view_handle_nvx(
        &self,
        info: &crate::extensions::nvx_image_view_handle::ImageViewHandleInfoNVX,
    ) -> u32 {
        let function = device_commands(self)
            .get_image_view_handle_nvx
            .as_ref()
            .expect("`get_image_view_handle_nvx` not available");
        let _val = function(self.handle, info);
        _val
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageViewAddressNVX.html) · Device Command"]
    unsafe fn get_image_view_address_nvx(
        &self,
        image_view: crate::vk1_0::ImageView,
        properties: Option<crate::extensions::nvx_image_view_handle::ImageViewAddressPropertiesNVX>,
    ) -> crate::utils::VulkanResult<
        crate::extensions::nvx_image_view_handle::ImageViewAddressPropertiesNVX,
    > {
        let function = device_commands(self)
            .get_image_view_address_nvx
            .as_ref()
            .expect("`get_image_view_address_nvx` not available");
        let mut properties = properties.unwrap_or_else(|| Default::default());
        let _val = function(self.handle, image_view, &mut properties);
        crate::utils::VulkanResult::new(_val, properties)
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageViewHandleInfoNVX.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImageViewHandleInfoNVX {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub image_view: crate::vk1_0::ImageView,
    pub descriptor_type: crate::vk1_0::DescriptorType,
    pub sampler: crate::vk1_0::Sampler,
}
impl ImageViewHandleInfoNVX {
    #[inline]
    pub fn builder<'a>(self) -> ImageViewHandleInfoNVXBuilder<'a> {
        ImageViewHandleInfoNVXBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ImageViewHandleInfoNVX {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ImageViewHandleInfoNVX")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("image_view", &self.image_view)
            .field("descriptor_type", &self.descriptor_type)
            .field("sampler", &self.sampler)
            .finish()
    }
}
impl Default for ImageViewHandleInfoNVX {
    fn default() -> ImageViewHandleInfoNVX {
        ImageViewHandleInfoNVX {
            s_type: crate::vk1_0::StructureType::IMAGE_VIEW_HANDLE_INFO_NVX,
            p_next: std::ptr::null(),
            image_view: Default::default(),
            descriptor_type: Default::default(),
            sampler: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageViewHandleInfoNVX.html) · Builder of [`ImageViewHandleInfoNVX`](struct.ImageViewHandleInfoNVX.html)"]
#[repr(transparent)]
pub struct ImageViewHandleInfoNVXBuilder<'a>(
    ImageViewHandleInfoNVX,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> ImageViewHandleInfoNVXBuilder<'a> {
    #[inline]
    pub fn new() -> ImageViewHandleInfoNVXBuilder<'a> {
        ImageViewHandleInfoNVXBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn image_view(mut self, image_view: crate::vk1_0::ImageView) -> Self {
        self.0.image_view = image_view;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn descriptor_type(mut self, descriptor_type: crate::vk1_0::DescriptorType) -> Self {
        self.0.descriptor_type = descriptor_type;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn sampler(mut self, sampler: crate::vk1_0::Sampler) -> Self {
        self.0.sampler = sampler;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> ImageViewHandleInfoNVX {
        self.0
    }
}
impl<'a> std::fmt::Debug for ImageViewHandleInfoNVXBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for ImageViewHandleInfoNVXBuilder<'a> {
    type Target = ImageViewHandleInfoNVX;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ImageViewHandleInfoNVXBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageViewAddressPropertiesNVX.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImageViewAddressPropertiesNVX {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub device_address: crate::vk1_0::DeviceAddress,
    pub size: crate::vk1_0::DeviceSize,
}
impl ImageViewAddressPropertiesNVX {
    #[inline]
    pub fn builder<'a>(self) -> ImageViewAddressPropertiesNVXBuilder<'a> {
        ImageViewAddressPropertiesNVXBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ImageViewAddressPropertiesNVX {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ImageViewAddressPropertiesNVX")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("device_address", &self.device_address)
            .field("size", &self.size)
            .finish()
    }
}
impl Default for ImageViewAddressPropertiesNVX {
    fn default() -> ImageViewAddressPropertiesNVX {
        ImageViewAddressPropertiesNVX {
            s_type: crate::vk1_0::StructureType::IMAGE_VIEW_ADDRESS_PROPERTIES_NVX,
            p_next: std::ptr::null_mut(),
            device_address: Default::default(),
            size: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageViewAddressPropertiesNVX.html) · Builder of [`ImageViewAddressPropertiesNVX`](struct.ImageViewAddressPropertiesNVX.html)"]
#[repr(transparent)]
pub struct ImageViewAddressPropertiesNVXBuilder<'a>(
    ImageViewAddressPropertiesNVX,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> ImageViewAddressPropertiesNVXBuilder<'a> {
    #[inline]
    pub fn new() -> ImageViewAddressPropertiesNVXBuilder<'a> {
        ImageViewAddressPropertiesNVXBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn device_address(mut self, device_address: crate::vk1_0::DeviceAddress) -> Self {
        self.0.device_address = device_address;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn size(mut self, size: crate::vk1_0::DeviceSize) -> Self {
        self.0.size = size;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> ImageViewAddressPropertiesNVX {
        self.0
    }
}
impl<'a> std::fmt::Debug for ImageViewAddressPropertiesNVXBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for ImageViewAddressPropertiesNVXBuilder<'a> {
    type Target = ImageViewAddressPropertiesNVX;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ImageViewAddressPropertiesNVXBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
