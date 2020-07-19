#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NVX_IMAGE_VIEW_HANDLE_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NVX_IMAGE_VIEW_HANDLE_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_NVX_image_view_handle");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_IMAGE_VIEW_HANDLE_NVX: *const std::os::raw::c_char =
    crate::cstr!("vkGetImageViewHandleNVX");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_IMAGE_VIEW_ADDRESS_NVX: *const std::os::raw::c_char =
    crate::cstr!("vkGetImageViewAddressNVX");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageViewHandleNVX.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageViewHandleNVX = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_info: *const crate::extensions::nvx_image_view_handle::ImageViewHandleInfoNVX,
) -> u32;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageViewAddressNVX.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageViewAddressNVX = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    image_view: crate::vk1_0::ImageView,
    p_properties: *mut crate::extensions::nvx_image_view_handle::ImageViewAddressPropertiesNVX,
) -> crate::vk1_0::Result;
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
impl Default for ImageViewHandleInfoNVX {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::IMAGE_VIEW_HANDLE_INFO_NVX,
            p_next: std::ptr::null(),
            image_view: Default::default(),
            descriptor_type: Default::default(),
            sampler: Default::default(),
        }
    }
}
impl std::fmt::Debug for ImageViewHandleInfoNVX {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ImageViewHandleInfoNVX")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("image_view", &self.image_view)
            .field("descriptor_type", &self.descriptor_type)
            .field("sampler", &self.sampler)
            .finish()
    }
}
impl ImageViewHandleInfoNVX {
    #[inline]
    pub fn into_builder<'a>(self) -> ImageViewHandleInfoNVXBuilder<'a> {
        ImageViewHandleInfoNVXBuilder(self, std::marker::PhantomData)
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
    #[inline]
    pub fn image_view(mut self, image_view: crate::vk1_0::ImageView) -> Self {
        self.0.image_view = image_view as _;
        self
    }
    #[inline]
    pub fn descriptor_type(mut self, descriptor_type: crate::vk1_0::DescriptorType) -> Self {
        self.0.descriptor_type = descriptor_type as _;
        self
    }
    #[inline]
    pub fn sampler(mut self, sampler: crate::vk1_0::Sampler) -> Self {
        self.0.sampler = sampler as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ImageViewHandleInfoNVX {
        self.0
    }
}
impl<'a> std::default::Default for ImageViewHandleInfoNVXBuilder<'a> {
    fn default() -> ImageViewHandleInfoNVXBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ImageViewHandleInfoNVXBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
impl Default for ImageViewAddressPropertiesNVX {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::IMAGE_VIEW_ADDRESS_PROPERTIES_NVX,
            p_next: std::ptr::null_mut(),
            device_address: Default::default(),
            size: Default::default(),
        }
    }
}
impl std::fmt::Debug for ImageViewAddressPropertiesNVX {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ImageViewAddressPropertiesNVX")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("device_address", &self.device_address)
            .field("size", &self.size)
            .finish()
    }
}
impl ImageViewAddressPropertiesNVX {
    #[inline]
    pub fn into_builder<'a>(self) -> ImageViewAddressPropertiesNVXBuilder<'a> {
        ImageViewAddressPropertiesNVXBuilder(self, std::marker::PhantomData)
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
    #[inline]
    pub fn device_address(mut self, device_address: crate::vk1_0::DeviceAddress) -> Self {
        self.0.device_address = device_address as _;
        self
    }
    #[inline]
    pub fn size(mut self, size: crate::vk1_0::DeviceSize) -> Self {
        self.0.size = size as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ImageViewAddressPropertiesNVX {
        self.0
    }
}
impl<'a> std::default::Default for ImageViewAddressPropertiesNVXBuilder<'a> {
    fn default() -> ImageViewAddressPropertiesNVXBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ImageViewAddressPropertiesNVXBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
#[doc = "Provided by [`extensions::nvx_image_view_handle`](extensions/nvx_image_view_handle/index.html)"]
impl crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageViewHandleNVX.html) · Function"]
    pub unsafe fn get_image_view_handle_nvx(
        &self,
        info: &crate::extensions::nvx_image_view_handle::ImageViewHandleInfoNVX,
    ) -> u32 {
        let _function = self
            .get_image_view_handle_nvx
            .expect("`get_image_view_handle_nvx` is not loaded");
        let _return = _function(self.handle, info as _);
        _return
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageViewAddressNVX.html) · Function"]
    pub unsafe fn get_image_view_address_nvx(
        &self,
        image_view: crate::vk1_0::ImageView,
        properties: Option<crate::extensions::nvx_image_view_handle::ImageViewAddressPropertiesNVX>,
    ) -> crate::utils::VulkanResult<
        crate::extensions::nvx_image_view_handle::ImageViewAddressPropertiesNVX,
    > {
        let _function = self
            .get_image_view_address_nvx
            .expect("`get_image_view_address_nvx` is not loaded");
        let mut properties = match properties {
            Some(v) => v,
            None => Default::default(),
        };
        let _return = _function(self.handle, image_view as _, &mut properties);
        crate::utils::VulkanResult::new(_return, properties)
    }
}
