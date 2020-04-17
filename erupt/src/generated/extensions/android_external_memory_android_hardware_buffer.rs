# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_ANDROID_external_memory_android_hardware_buffer.html)\n\n## Extends\n- [`ExternalMemoryHandleTypeFlagBits`](../../vk1_1/struct.ExternalMemoryHandleTypeFlagBits.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER_SPEC_VERSION: u32 = 3;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER_EXTENSION_NAME:
    *const std::os::raw::c_char =
    crate::cstr!("VK_ANDROID_external_memory_android_hardware_buffer");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetAndroidHardwareBufferPropertiesANDROID.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetAndroidHardwareBufferPropertiesANDROID = unsafe extern "system" fn ( device : crate :: vk1_0 :: Device , buffer : * const std :: ffi :: c_void , p_properties : * mut crate :: extensions :: android_external_memory_android_hardware_buffer :: AndroidHardwareBufferPropertiesANDROID , ) -> crate :: vk1_0 :: Result ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryAndroidHardwareBufferANDROID.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetMemoryAndroidHardwareBufferANDROID = unsafe extern "system" fn ( device : crate :: vk1_0 :: Device , p_info : * const crate :: extensions :: android_external_memory_android_hardware_buffer :: MemoryGetAndroidHardwareBufferInfoANDROID , p_buffer : * mut * mut std :: ffi :: c_void , ) -> crate :: vk1_0 :: Result ;
#[doc = "Provides Device Commands for [`AndroidExternalMemoryAndroidHardwareBufferDeviceLoaderExt`](trait.AndroidExternalMemoryAndroidHardwareBufferDeviceLoaderExt.html)"]
pub struct AndroidExternalMemoryAndroidHardwareBufferDeviceCommands {
    pub get_android_hardware_buffer_properties_android:
        PFN_vkGetAndroidHardwareBufferPropertiesANDROID,
    pub get_memory_android_hardware_buffer_android: PFN_vkGetMemoryAndroidHardwareBufferANDROID,
}
impl AndroidExternalMemoryAndroidHardwareBufferDeviceCommands {
    #[inline]
    pub fn load(
        loader: &crate::DeviceLoader,
    ) -> Option<AndroidExternalMemoryAndroidHardwareBufferDeviceCommands> {
        unsafe {
            Some(AndroidExternalMemoryAndroidHardwareBufferDeviceCommands {
                get_android_hardware_buffer_properties_android: std::mem::transmute(
                    loader.symbol("vkGetAndroidHardwareBufferPropertiesANDROID")?,
                ),
                get_memory_android_hardware_buffer_android: std::mem::transmute(
                    loader.symbol("vkGetMemoryAndroidHardwareBufferANDROID")?,
                ),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`AndroidExternalMemoryAndroidHardwareBufferDeviceCommands`](struct.AndroidExternalMemoryAndroidHardwareBufferDeviceCommands.html)"]
pub trait AndroidExternalMemoryAndroidHardwareBufferDeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetAndroidHardwareBufferPropertiesANDROID.html) · Device Command"]
    unsafe fn get_android_hardware_buffer_properties_android ( & self , buffer : * const std :: ffi :: c_void , properties : Option < crate :: extensions :: android_external_memory_android_hardware_buffer :: AndroidHardwareBufferPropertiesANDROID > , ) -> crate :: utils :: VulkanResult < crate :: extensions :: android_external_memory_android_hardware_buffer :: AndroidHardwareBufferPropertiesANDROID > ;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryAndroidHardwareBufferANDROID.html) · Device Command"]
    unsafe fn get_memory_android_hardware_buffer_android(
        &self,
        info : & crate :: extensions :: android_external_memory_android_hardware_buffer :: MemoryGetAndroidHardwareBufferInfoANDROID,
        buffer: *mut *mut std::ffi::c_void,
    ) -> crate::utils::VulkanResult<()>;
}
impl AndroidExternalMemoryAndroidHardwareBufferDeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetAndroidHardwareBufferPropertiesANDROID.html) · Device Command"]unsafe fn get_android_hardware_buffer_properties_android ( & self , buffer : * const std :: ffi :: c_void , properties : Option < crate :: extensions :: android_external_memory_android_hardware_buffer :: AndroidHardwareBufferPropertiesANDROID > , ) -> crate :: utils :: VulkanResult < crate :: extensions :: android_external_memory_android_hardware_buffer :: AndroidHardwareBufferPropertiesANDROID >{
        let function = self
            .android_external_memory_android_hardware_buffer
            .as_ref()
            .expect("`android_external_memory_android_hardware_buffer` not loaded")
            .get_android_hardware_buffer_properties_android;
        let mut properties = properties.unwrap_or_else(|| Default::default());
        let _val = function(self.handle, buffer, &mut properties);
        crate::utils::VulkanResult::new(_val, properties)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryAndroidHardwareBufferANDROID.html) · Device Command"]
    unsafe fn get_memory_android_hardware_buffer_android(
        &self,
        info : & crate :: extensions :: android_external_memory_android_hardware_buffer :: MemoryGetAndroidHardwareBufferInfoANDROID,
        buffer: *mut *mut std::ffi::c_void,
    ) -> crate::utils::VulkanResult<()> {
        let function = self
            .android_external_memory_android_hardware_buffer
            .as_ref()
            .expect("`android_external_memory_android_hardware_buffer` not loaded")
            .get_memory_android_hardware_buffer_android;
        let _val = function(self.handle, info, buffer);
        crate::utils::VulkanResult::new(_val, ())
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAndroidHardwareBufferPropertiesANDROID.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AndroidHardwareBufferPropertiesANDROID {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub allocation_size: crate::vk1_0::DeviceSize,
    pub memory_type_bits: u32,
}
impl AndroidHardwareBufferPropertiesANDROID {
    #[inline]
    pub fn builder<'a>(self) -> AndroidHardwareBufferPropertiesANDROIDBuilder<'a> {
        AndroidHardwareBufferPropertiesANDROIDBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for AndroidHardwareBufferPropertiesANDROID {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("AndroidHardwareBufferPropertiesANDROID")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("allocation_size", &self.allocation_size)
            .field("memory_type_bits", &self.memory_type_bits)
            .finish()
    }
}
impl Default for AndroidHardwareBufferPropertiesANDROID {
    fn default() -> AndroidHardwareBufferPropertiesANDROID {
        AndroidHardwareBufferPropertiesANDROID {
            s_type: crate::vk1_0::StructureType::ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID,
            p_next: std::ptr::null_mut(),
            allocation_size: Default::default(),
            memory_type_bits: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`AndroidHardwareBufferPropertiesANDROID`](struct.AndroidHardwareBufferPropertiesANDROID.html)"]
#[repr(transparent)]
pub struct AndroidHardwareBufferPropertiesANDROIDBuilder<'a>(
    AndroidHardwareBufferPropertiesANDROID,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> AndroidHardwareBufferPropertiesANDROIDBuilder<'a> {
    #[inline]
    pub fn new() -> AndroidHardwareBufferPropertiesANDROIDBuilder<'a> {
        AndroidHardwareBufferPropertiesANDROIDBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn allocation_size(mut self, allocation_size: crate::vk1_0::DeviceSize) -> Self {
        self.0.allocation_size = allocation_size;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn memory_type_bits(mut self, memory_type_bits: u32) -> Self {
        self.0.memory_type_bits = memory_type_bits;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> AndroidHardwareBufferPropertiesANDROID {
        self.0
    }
}
impl<'a> std::fmt::Debug for AndroidHardwareBufferPropertiesANDROIDBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for AndroidHardwareBufferPropertiesANDROIDBuilder<'a> {
    type Target = AndroidHardwareBufferPropertiesANDROID;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for AndroidHardwareBufferPropertiesANDROIDBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryGetAndroidHardwareBufferInfoANDROID.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryGetAndroidHardwareBufferInfoANDROID {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub memory: crate::vk1_0::DeviceMemory,
}
impl MemoryGetAndroidHardwareBufferInfoANDROID {
    #[inline]
    pub fn builder<'a>(self) -> MemoryGetAndroidHardwareBufferInfoANDROIDBuilder<'a> {
        MemoryGetAndroidHardwareBufferInfoANDROIDBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for MemoryGetAndroidHardwareBufferInfoANDROID {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("MemoryGetAndroidHardwareBufferInfoANDROID")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("memory", &self.memory)
            .finish()
    }
}
impl Default for MemoryGetAndroidHardwareBufferInfoANDROID {
    fn default() -> MemoryGetAndroidHardwareBufferInfoANDROID {
        MemoryGetAndroidHardwareBufferInfoANDROID {
            s_type: crate::vk1_0::StructureType::MEMORY_GET_ANDROID_HARDWARE_BUFFER_INFO_ANDROID,
            p_next: std::ptr::null(),
            memory: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`MemoryGetAndroidHardwareBufferInfoANDROID`](struct.MemoryGetAndroidHardwareBufferInfoANDROID.html)"]
#[repr(transparent)]
pub struct MemoryGetAndroidHardwareBufferInfoANDROIDBuilder<'a>(
    MemoryGetAndroidHardwareBufferInfoANDROID,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> MemoryGetAndroidHardwareBufferInfoANDROIDBuilder<'a> {
    #[inline]
    pub fn new() -> MemoryGetAndroidHardwareBufferInfoANDROIDBuilder<'a> {
        MemoryGetAndroidHardwareBufferInfoANDROIDBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn memory(mut self, memory: crate::vk1_0::DeviceMemory) -> Self {
        self.0.memory = memory;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> MemoryGetAndroidHardwareBufferInfoANDROID {
        self.0
    }
}
impl<'a> std::fmt::Debug for MemoryGetAndroidHardwareBufferInfoANDROIDBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for MemoryGetAndroidHardwareBufferInfoANDROIDBuilder<'a> {
    type Target = MemoryGetAndroidHardwareBufferInfoANDROID;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for MemoryGetAndroidHardwareBufferInfoANDROIDBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAndroidHardwareBufferUsageANDROID.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AndroidHardwareBufferUsageANDROID {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub android_hardware_buffer_usage: u64,
}
impl AndroidHardwareBufferUsageANDROID {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByAndroidHardwareBufferUsageANDROID,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> AndroidHardwareBufferUsageANDROIDBuilder<'a> {
        AndroidHardwareBufferUsageANDROIDBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for AndroidHardwareBufferUsageANDROID {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("AndroidHardwareBufferUsageANDROID")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "android_hardware_buffer_usage",
                &self.android_hardware_buffer_usage,
            )
            .finish()
    }
}
impl Default for AndroidHardwareBufferUsageANDROID {
    fn default() -> AndroidHardwareBufferUsageANDROID {
        AndroidHardwareBufferUsageANDROID {
            s_type: crate::vk1_0::StructureType::ANDROID_HARDWARE_BUFFER_USAGE_ANDROID,
            p_next: std::ptr::null_mut(),
            android_hardware_buffer_usage: Default::default(),
        }
    }
}
#[doc = "Used by [`AndroidHardwareBufferUsageANDROID::extend`](struct.AndroidHardwareBufferUsageANDROID.html#method.extend)"]
pub trait ExtendableByAndroidHardwareBufferUsageANDROID {}
impl ExtendableByAndroidHardwareBufferUsageANDROID for crate::vk1_1::ImageFormatProperties2 {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`AndroidHardwareBufferUsageANDROID`](struct.AndroidHardwareBufferUsageANDROID.html)"]
#[repr(transparent)]
pub struct AndroidHardwareBufferUsageANDROIDBuilder<'a>(
    AndroidHardwareBufferUsageANDROID,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> AndroidHardwareBufferUsageANDROIDBuilder<'a> {
    #[inline]
    pub fn new() -> AndroidHardwareBufferUsageANDROIDBuilder<'a> {
        AndroidHardwareBufferUsageANDROIDBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn android_hardware_buffer_usage(mut self, android_hardware_buffer_usage: u64) -> Self {
        self.0.android_hardware_buffer_usage = android_hardware_buffer_usage;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> AndroidHardwareBufferUsageANDROID {
        self.0
    }
}
impl<'a> std::fmt::Debug for AndroidHardwareBufferUsageANDROIDBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for AndroidHardwareBufferUsageANDROIDBuilder<'a> {
    type Target = AndroidHardwareBufferUsageANDROID;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for AndroidHardwareBufferUsageANDROIDBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAndroidHardwareBufferFormatPropertiesANDROID.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AndroidHardwareBufferFormatPropertiesANDROID {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub format: crate::vk1_0::Format,
    pub external_format: u64,
    pub format_features: crate::vk1_0::FormatFeatureFlags,
    pub sampler_ycbcr_conversion_components: crate::vk1_0::ComponentMapping,
    pub suggested_ycbcr_model: crate::vk1_1::SamplerYcbcrModelConversion,
    pub suggested_ycbcr_range: crate::vk1_1::SamplerYcbcrRange,
    pub suggested_x_chroma_offset: crate::vk1_1::ChromaLocation,
    pub suggested_y_chroma_offset: crate::vk1_1::ChromaLocation,
}
impl AndroidHardwareBufferFormatPropertiesANDROID {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByAndroidHardwareBufferFormatPropertiesANDROID,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> AndroidHardwareBufferFormatPropertiesANDROIDBuilder<'a> {
        AndroidHardwareBufferFormatPropertiesANDROIDBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for AndroidHardwareBufferFormatPropertiesANDROID {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("AndroidHardwareBufferFormatPropertiesANDROID")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("format", &self.format)
            .field("external_format", &self.external_format)
            .field("format_features", &self.format_features)
            .field(
                "sampler_ycbcr_conversion_components",
                &self.sampler_ycbcr_conversion_components,
            )
            .field("suggested_ycbcr_model", &self.suggested_ycbcr_model)
            .field("suggested_ycbcr_range", &self.suggested_ycbcr_range)
            .field("suggested_x_chroma_offset", &self.suggested_x_chroma_offset)
            .field("suggested_y_chroma_offset", &self.suggested_y_chroma_offset)
            .finish()
    }
}
impl Default for AndroidHardwareBufferFormatPropertiesANDROID {
    fn default() -> AndroidHardwareBufferFormatPropertiesANDROID {
        AndroidHardwareBufferFormatPropertiesANDROID {
            s_type: crate::vk1_0::StructureType::ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_ANDROID,
            p_next: std::ptr::null_mut(),
            format: Default::default(),
            external_format: Default::default(),
            format_features: Default::default(),
            sampler_ycbcr_conversion_components: Default::default(),
            suggested_ycbcr_model: Default::default(),
            suggested_ycbcr_range: Default::default(),
            suggested_x_chroma_offset: Default::default(),
            suggested_y_chroma_offset: Default::default(),
        }
    }
}
#[doc = "Used by [`AndroidHardwareBufferFormatPropertiesANDROID::extend`](struct.AndroidHardwareBufferFormatPropertiesANDROID.html#method.extend)"]
pub trait ExtendableByAndroidHardwareBufferFormatPropertiesANDROID {}
impl ExtendableByAndroidHardwareBufferFormatPropertiesANDROID for crate :: extensions :: android_external_memory_android_hardware_buffer :: AndroidHardwareBufferPropertiesANDROID { }
#[derive(Copy, Clone)]
#[doc = "Builder of [`AndroidHardwareBufferFormatPropertiesANDROID`](struct.AndroidHardwareBufferFormatPropertiesANDROID.html)"]
#[repr(transparent)]
pub struct AndroidHardwareBufferFormatPropertiesANDROIDBuilder<'a>(
    AndroidHardwareBufferFormatPropertiesANDROID,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> AndroidHardwareBufferFormatPropertiesANDROIDBuilder<'a> {
    #[inline]
    pub fn new() -> AndroidHardwareBufferFormatPropertiesANDROIDBuilder<'a> {
        AndroidHardwareBufferFormatPropertiesANDROIDBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn format(mut self, format: crate::vk1_0::Format) -> Self {
        self.0.format = format;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn external_format(mut self, external_format: u64) -> Self {
        self.0.external_format = external_format;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn format_features(mut self, format_features: crate::vk1_0::FormatFeatureFlags) -> Self {
        self.0.format_features = format_features;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn sampler_ycbcr_conversion_components(
        mut self,
        sampler_ycbcr_conversion_components: crate::vk1_0::ComponentMapping,
    ) -> Self {
        self.0.sampler_ycbcr_conversion_components = sampler_ycbcr_conversion_components;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn suggested_ycbcr_model(
        mut self,
        suggested_ycbcr_model: crate::vk1_1::SamplerYcbcrModelConversion,
    ) -> Self {
        self.0.suggested_ycbcr_model = suggested_ycbcr_model;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn suggested_ycbcr_range(
        mut self,
        suggested_ycbcr_range: crate::vk1_1::SamplerYcbcrRange,
    ) -> Self {
        self.0.suggested_ycbcr_range = suggested_ycbcr_range;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn suggested_x_chroma_offset(
        mut self,
        suggested_x_chroma_offset: crate::vk1_1::ChromaLocation,
    ) -> Self {
        self.0.suggested_x_chroma_offset = suggested_x_chroma_offset;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn suggested_y_chroma_offset(
        mut self,
        suggested_y_chroma_offset: crate::vk1_1::ChromaLocation,
    ) -> Self {
        self.0.suggested_y_chroma_offset = suggested_y_chroma_offset;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> AndroidHardwareBufferFormatPropertiesANDROID {
        self.0
    }
}
impl<'a> std::fmt::Debug for AndroidHardwareBufferFormatPropertiesANDROIDBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for AndroidHardwareBufferFormatPropertiesANDROIDBuilder<'a> {
    type Target = AndroidHardwareBufferFormatPropertiesANDROID;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for AndroidHardwareBufferFormatPropertiesANDROIDBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImportAndroidHardwareBufferInfoANDROID.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImportAndroidHardwareBufferInfoANDROID {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub buffer: *mut std::ffi::c_void,
}
impl ImportAndroidHardwareBufferInfoANDROID {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByImportAndroidHardwareBufferInfoANDROID,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> ImportAndroidHardwareBufferInfoANDROIDBuilder<'a> {
        ImportAndroidHardwareBufferInfoANDROIDBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ImportAndroidHardwareBufferInfoANDROID {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ImportAndroidHardwareBufferInfoANDROID")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("buffer", &self.buffer)
            .finish()
    }
}
impl Default for ImportAndroidHardwareBufferInfoANDROID {
    fn default() -> ImportAndroidHardwareBufferInfoANDROID {
        ImportAndroidHardwareBufferInfoANDROID {
            s_type: crate::vk1_0::StructureType::IMPORT_ANDROID_HARDWARE_BUFFER_INFO_ANDROID,
            p_next: std::ptr::null(),
            buffer: std::ptr::null_mut(),
        }
    }
}
#[doc = "Used by [`ImportAndroidHardwareBufferInfoANDROID::extend`](struct.ImportAndroidHardwareBufferInfoANDROID.html#method.extend)"]
pub trait ExtendableByImportAndroidHardwareBufferInfoANDROID {}
impl ExtendableByImportAndroidHardwareBufferInfoANDROID for crate::vk1_0::MemoryAllocateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`ImportAndroidHardwareBufferInfoANDROID`](struct.ImportAndroidHardwareBufferInfoANDROID.html)"]
#[repr(transparent)]
pub struct ImportAndroidHardwareBufferInfoANDROIDBuilder<'a>(
    ImportAndroidHardwareBufferInfoANDROID,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> ImportAndroidHardwareBufferInfoANDROIDBuilder<'a> {
    #[inline]
    pub fn new() -> ImportAndroidHardwareBufferInfoANDROIDBuilder<'a> {
        ImportAndroidHardwareBufferInfoANDROIDBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn buffer(mut self, buffer: &'a mut std::ffi::c_void) -> Self {
        self.0.buffer = buffer;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> ImportAndroidHardwareBufferInfoANDROID {
        self.0
    }
}
impl<'a> std::fmt::Debug for ImportAndroidHardwareBufferInfoANDROIDBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for ImportAndroidHardwareBufferInfoANDROIDBuilder<'a> {
    type Target = ImportAndroidHardwareBufferInfoANDROID;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ImportAndroidHardwareBufferInfoANDROIDBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalFormatANDROID.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExternalFormatANDROID {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub external_format: u64,
}
impl ExternalFormatANDROID {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByExternalFormatANDROID,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> ExternalFormatANDROIDBuilder<'a> {
        ExternalFormatANDROIDBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ExternalFormatANDROID {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ExternalFormatANDROID")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("external_format", &self.external_format)
            .finish()
    }
}
impl Default for ExternalFormatANDROID {
    fn default() -> ExternalFormatANDROID {
        ExternalFormatANDROID {
            s_type: crate::vk1_0::StructureType::EXTERNAL_FORMAT_ANDROID,
            p_next: std::ptr::null_mut(),
            external_format: Default::default(),
        }
    }
}
#[doc = "Used by [`ExternalFormatANDROID::extend`](struct.ExternalFormatANDROID.html#method.extend)"]
pub trait ExtendableByExternalFormatANDROID {}
impl ExtendableByExternalFormatANDROID for crate::vk1_0::ImageCreateInfo {}
impl ExtendableByExternalFormatANDROID for crate::vk1_1::SamplerYcbcrConversionCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`ExternalFormatANDROID`](struct.ExternalFormatANDROID.html)"]
#[repr(transparent)]
pub struct ExternalFormatANDROIDBuilder<'a>(
    ExternalFormatANDROID,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> ExternalFormatANDROIDBuilder<'a> {
    #[inline]
    pub fn new() -> ExternalFormatANDROIDBuilder<'a> {
        ExternalFormatANDROIDBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn external_format(mut self, external_format: u64) -> Self {
        self.0.external_format = external_format;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> ExternalFormatANDROID {
        self.0
    }
}
impl<'a> std::fmt::Debug for ExternalFormatANDROIDBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for ExternalFormatANDROIDBuilder<'a> {
    type Target = ExternalFormatANDROID;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ExternalFormatANDROIDBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
