#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER_SPEC_VERSION")]
pub const ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER_SPEC_VERSION: u32 = 3;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER_EXTENSION_NAME")]
pub const ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_ANDROID_external_memory_android_hardware_buffer");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID: *const std::os::raw::c_char = crate::cstr!("vkGetAndroidHardwareBufferPropertiesANDROID");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_MEMORY_ANDROID_HARDWARE_BUFFER_ANDROID: *const std::os::raw::c_char = crate::cstr!("vkGetMemoryAndroidHardwareBufferANDROID");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/AHardwareBuffer.html) · Basetype"]
pub type AHardwareBuffer = std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetAndroidHardwareBufferPropertiesANDROID.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetAndroidHardwareBufferPropertiesANDROID = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    buffer: *const crate::extensions::android_external_memory_android_hardware_buffer::AHardwareBuffer,
    p_properties: *mut crate::extensions::android_external_memory_android_hardware_buffer::AndroidHardwareBufferPropertiesANDROID,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryAndroidHardwareBufferANDROID.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetMemoryAndroidHardwareBufferANDROID = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_info: *const crate::extensions::android_external_memory_android_hardware_buffer::MemoryGetAndroidHardwareBufferInfoANDROID,
    p_buffer: *mut *mut crate::extensions::android_external_memory_android_hardware_buffer::AHardwareBuffer,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImportAndroidHardwareBufferInfoANDROID.html) · Structure"]
#[doc(alias = "VkImportAndroidHardwareBufferInfoANDROID")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImportAndroidHardwareBufferInfoANDROID {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub buffer: *mut crate::extensions::android_external_memory_android_hardware_buffer::AHardwareBuffer,
}
impl Default for ImportAndroidHardwareBufferInfoANDROID {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::IMPORT_ANDROID_HARDWARE_BUFFER_INFO_ANDROID,
            p_next: std::ptr::null(),
            buffer: std::ptr::null_mut(),
        }
    }
}
impl std::fmt::Debug for ImportAndroidHardwareBufferInfoANDROID {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ImportAndroidHardwareBufferInfoANDROID")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("buffer", &self.buffer)
            .finish()
    }
}
impl ImportAndroidHardwareBufferInfoANDROID {
    #[inline]
    pub fn into_builder<'a>(self) -> ImportAndroidHardwareBufferInfoANDROIDBuilder<'a> {
        ImportAndroidHardwareBufferInfoANDROIDBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImportAndroidHardwareBufferInfoANDROID.html) · Builder of [`ImportAndroidHardwareBufferInfoANDROID`]"]
#[repr(transparent)]
pub struct ImportAndroidHardwareBufferInfoANDROIDBuilder<'a>(ImportAndroidHardwareBufferInfoANDROID, std::marker::PhantomData<&'a ()>);
impl<'a> ImportAndroidHardwareBufferInfoANDROIDBuilder<'a> {
    #[inline]
    pub fn new() -> ImportAndroidHardwareBufferInfoANDROIDBuilder<'a> {
        ImportAndroidHardwareBufferInfoANDROIDBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn buffer(mut self, buffer: &'a mut crate::extensions::android_external_memory_android_hardware_buffer::AHardwareBuffer) -> Self {
        self.0.buffer = buffer as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ImportAndroidHardwareBufferInfoANDROID {
        self.0
    }
}
impl<'a> std::default::Default for ImportAndroidHardwareBufferInfoANDROIDBuilder<'a> {
    fn default() -> ImportAndroidHardwareBufferInfoANDROIDBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ImportAndroidHardwareBufferInfoANDROIDBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
unsafe impl crate::Repr<ImportAndroidHardwareBufferInfoANDROID> for ImportAndroidHardwareBufferInfoANDROIDBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAndroidHardwareBufferUsageANDROID.html) · Structure"]
#[doc(alias = "VkAndroidHardwareBufferUsageANDROID")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AndroidHardwareBufferUsageANDROID {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub android_hardware_buffer_usage: u64,
}
impl Default for AndroidHardwareBufferUsageANDROID {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::ANDROID_HARDWARE_BUFFER_USAGE_ANDROID,
            p_next: std::ptr::null_mut(),
            android_hardware_buffer_usage: Default::default(),
        }
    }
}
impl std::fmt::Debug for AndroidHardwareBufferUsageANDROID {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AndroidHardwareBufferUsageANDROID")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("android_hardware_buffer_usage", &self.android_hardware_buffer_usage)
            .finish()
    }
}
impl AndroidHardwareBufferUsageANDROID {
    #[inline]
    pub fn into_builder<'a>(self) -> AndroidHardwareBufferUsageANDROIDBuilder<'a> {
        AndroidHardwareBufferUsageANDROIDBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAndroidHardwareBufferUsageANDROID.html) · Builder of [`AndroidHardwareBufferUsageANDROID`]"]
#[repr(transparent)]
pub struct AndroidHardwareBufferUsageANDROIDBuilder<'a>(AndroidHardwareBufferUsageANDROID, std::marker::PhantomData<&'a ()>);
impl<'a> AndroidHardwareBufferUsageANDROIDBuilder<'a> {
    #[inline]
    pub fn new() -> AndroidHardwareBufferUsageANDROIDBuilder<'a> {
        AndroidHardwareBufferUsageANDROIDBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn android_hardware_buffer_usage(mut self, android_hardware_buffer_usage: u64) -> Self {
        self.0.android_hardware_buffer_usage = android_hardware_buffer_usage as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> AndroidHardwareBufferUsageANDROID {
        self.0
    }
}
impl<'a> std::default::Default for AndroidHardwareBufferUsageANDROIDBuilder<'a> {
    fn default() -> AndroidHardwareBufferUsageANDROIDBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for AndroidHardwareBufferUsageANDROIDBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
unsafe impl crate::Repr<AndroidHardwareBufferUsageANDROID> for AndroidHardwareBufferUsageANDROIDBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAndroidHardwareBufferPropertiesANDROID.html) · Structure"]
#[doc(alias = "VkAndroidHardwareBufferPropertiesANDROID")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AndroidHardwareBufferPropertiesANDROID {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub allocation_size: crate::vk1_0::DeviceSize,
    pub memory_type_bits: u32,
}
impl Default for AndroidHardwareBufferPropertiesANDROID {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID,
            p_next: std::ptr::null_mut(),
            allocation_size: Default::default(),
            memory_type_bits: Default::default(),
        }
    }
}
impl std::fmt::Debug for AndroidHardwareBufferPropertiesANDROID {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AndroidHardwareBufferPropertiesANDROID")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("allocation_size", &self.allocation_size)
            .field("memory_type_bits", &self.memory_type_bits)
            .finish()
    }
}
impl AndroidHardwareBufferPropertiesANDROID {
    #[inline]
    pub fn into_builder<'a>(self) -> AndroidHardwareBufferPropertiesANDROIDBuilder<'a> {
        AndroidHardwareBufferPropertiesANDROIDBuilder(self, std::marker::PhantomData)
    }
}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::android_external_memory_android_hardware_buffer::AndroidHardwareBufferFormatPropertiesANDROID>
    for AndroidHardwareBufferPropertiesANDROIDBuilder<'a>
{
}
impl<'a> crate::ExtendableFrom<'a, crate::extensions::android_external_memory_android_hardware_buffer::AndroidHardwareBufferFormatPropertiesANDROIDBuilder<'_>>
    for AndroidHardwareBufferPropertiesANDROIDBuilder<'a>
{
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAndroidHardwareBufferPropertiesANDROID.html) · Builder of [`AndroidHardwareBufferPropertiesANDROID`]"]
#[repr(transparent)]
pub struct AndroidHardwareBufferPropertiesANDROIDBuilder<'a>(AndroidHardwareBufferPropertiesANDROID, std::marker::PhantomData<&'a ()>);
impl<'a> AndroidHardwareBufferPropertiesANDROIDBuilder<'a> {
    #[inline]
    pub fn new() -> AndroidHardwareBufferPropertiesANDROIDBuilder<'a> {
        AndroidHardwareBufferPropertiesANDROIDBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn allocation_size(mut self, allocation_size: crate::vk1_0::DeviceSize) -> Self {
        self.0.allocation_size = allocation_size as _;
        self
    }
    #[inline]
    pub fn memory_type_bits(mut self, memory_type_bits: u32) -> Self {
        self.0.memory_type_bits = memory_type_bits as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> AndroidHardwareBufferPropertiesANDROID {
        self.0
    }
}
impl<'a> std::default::Default for AndroidHardwareBufferPropertiesANDROIDBuilder<'a> {
    fn default() -> AndroidHardwareBufferPropertiesANDROIDBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for AndroidHardwareBufferPropertiesANDROIDBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
unsafe impl crate::Repr<AndroidHardwareBufferPropertiesANDROID> for AndroidHardwareBufferPropertiesANDROIDBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryGetAndroidHardwareBufferInfoANDROID.html) · Structure"]
#[doc(alias = "VkMemoryGetAndroidHardwareBufferInfoANDROID")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryGetAndroidHardwareBufferInfoANDROID {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub memory: crate::vk1_0::DeviceMemory,
}
impl Default for MemoryGetAndroidHardwareBufferInfoANDROID {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::MEMORY_GET_ANDROID_HARDWARE_BUFFER_INFO_ANDROID,
            p_next: std::ptr::null(),
            memory: Default::default(),
        }
    }
}
impl std::fmt::Debug for MemoryGetAndroidHardwareBufferInfoANDROID {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MemoryGetAndroidHardwareBufferInfoANDROID")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("memory", &self.memory)
            .finish()
    }
}
impl MemoryGetAndroidHardwareBufferInfoANDROID {
    #[inline]
    pub fn into_builder<'a>(self) -> MemoryGetAndroidHardwareBufferInfoANDROIDBuilder<'a> {
        MemoryGetAndroidHardwareBufferInfoANDROIDBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryGetAndroidHardwareBufferInfoANDROID.html) · Builder of [`MemoryGetAndroidHardwareBufferInfoANDROID`]"]
#[repr(transparent)]
pub struct MemoryGetAndroidHardwareBufferInfoANDROIDBuilder<'a>(MemoryGetAndroidHardwareBufferInfoANDROID, std::marker::PhantomData<&'a ()>);
impl<'a> MemoryGetAndroidHardwareBufferInfoANDROIDBuilder<'a> {
    #[inline]
    pub fn new() -> MemoryGetAndroidHardwareBufferInfoANDROIDBuilder<'a> {
        MemoryGetAndroidHardwareBufferInfoANDROIDBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn memory(mut self, memory: crate::vk1_0::DeviceMemory) -> Self {
        self.0.memory = memory as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> MemoryGetAndroidHardwareBufferInfoANDROID {
        self.0
    }
}
impl<'a> std::default::Default for MemoryGetAndroidHardwareBufferInfoANDROIDBuilder<'a> {
    fn default() -> MemoryGetAndroidHardwareBufferInfoANDROIDBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for MemoryGetAndroidHardwareBufferInfoANDROIDBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
unsafe impl crate::Repr<MemoryGetAndroidHardwareBufferInfoANDROID> for MemoryGetAndroidHardwareBufferInfoANDROIDBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAndroidHardwareBufferFormatPropertiesANDROID.html) · Structure"]
#[doc(alias = "VkAndroidHardwareBufferFormatPropertiesANDROID")]
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
impl Default for AndroidHardwareBufferFormatPropertiesANDROID {
    fn default() -> Self {
        Self {
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
impl std::fmt::Debug for AndroidHardwareBufferFormatPropertiesANDROID {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AndroidHardwareBufferFormatPropertiesANDROID")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("format", &self.format)
            .field("external_format", &self.external_format)
            .field("format_features", &self.format_features)
            .field("sampler_ycbcr_conversion_components", &self.sampler_ycbcr_conversion_components)
            .field("suggested_ycbcr_model", &self.suggested_ycbcr_model)
            .field("suggested_ycbcr_range", &self.suggested_ycbcr_range)
            .field("suggested_x_chroma_offset", &self.suggested_x_chroma_offset)
            .field("suggested_y_chroma_offset", &self.suggested_y_chroma_offset)
            .finish()
    }
}
impl AndroidHardwareBufferFormatPropertiesANDROID {
    #[inline]
    pub fn into_builder<'a>(self) -> AndroidHardwareBufferFormatPropertiesANDROIDBuilder<'a> {
        AndroidHardwareBufferFormatPropertiesANDROIDBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAndroidHardwareBufferFormatPropertiesANDROID.html) · Builder of [`AndroidHardwareBufferFormatPropertiesANDROID`]"]
#[repr(transparent)]
pub struct AndroidHardwareBufferFormatPropertiesANDROIDBuilder<'a>(AndroidHardwareBufferFormatPropertiesANDROID, std::marker::PhantomData<&'a ()>);
impl<'a> AndroidHardwareBufferFormatPropertiesANDROIDBuilder<'a> {
    #[inline]
    pub fn new() -> AndroidHardwareBufferFormatPropertiesANDROIDBuilder<'a> {
        AndroidHardwareBufferFormatPropertiesANDROIDBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn format(mut self, format: crate::vk1_0::Format) -> Self {
        self.0.format = format as _;
        self
    }
    #[inline]
    pub fn external_format(mut self, external_format: u64) -> Self {
        self.0.external_format = external_format as _;
        self
    }
    #[inline]
    pub fn format_features(mut self, format_features: crate::vk1_0::FormatFeatureFlags) -> Self {
        self.0.format_features = format_features as _;
        self
    }
    #[inline]
    pub fn sampler_ycbcr_conversion_components(mut self, sampler_ycbcr_conversion_components: crate::vk1_0::ComponentMapping) -> Self {
        self.0.sampler_ycbcr_conversion_components = sampler_ycbcr_conversion_components as _;
        self
    }
    #[inline]
    pub fn suggested_ycbcr_model(mut self, suggested_ycbcr_model: crate::vk1_1::SamplerYcbcrModelConversion) -> Self {
        self.0.suggested_ycbcr_model = suggested_ycbcr_model as _;
        self
    }
    #[inline]
    pub fn suggested_ycbcr_range(mut self, suggested_ycbcr_range: crate::vk1_1::SamplerYcbcrRange) -> Self {
        self.0.suggested_ycbcr_range = suggested_ycbcr_range as _;
        self
    }
    #[inline]
    pub fn suggested_x_chroma_offset(mut self, suggested_x_chroma_offset: crate::vk1_1::ChromaLocation) -> Self {
        self.0.suggested_x_chroma_offset = suggested_x_chroma_offset as _;
        self
    }
    #[inline]
    pub fn suggested_y_chroma_offset(mut self, suggested_y_chroma_offset: crate::vk1_1::ChromaLocation) -> Self {
        self.0.suggested_y_chroma_offset = suggested_y_chroma_offset as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> AndroidHardwareBufferFormatPropertiesANDROID {
        self.0
    }
}
impl<'a> std::default::Default for AndroidHardwareBufferFormatPropertiesANDROIDBuilder<'a> {
    fn default() -> AndroidHardwareBufferFormatPropertiesANDROIDBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for AndroidHardwareBufferFormatPropertiesANDROIDBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
unsafe impl crate::Repr<AndroidHardwareBufferFormatPropertiesANDROID> for AndroidHardwareBufferFormatPropertiesANDROIDBuilder<'_> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalFormatANDROID.html) · Structure"]
#[doc(alias = "VkExternalFormatANDROID")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExternalFormatANDROID {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub external_format: u64,
}
impl Default for ExternalFormatANDROID {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::EXTERNAL_FORMAT_ANDROID,
            p_next: std::ptr::null_mut(),
            external_format: Default::default(),
        }
    }
}
impl std::fmt::Debug for ExternalFormatANDROID {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ExternalFormatANDROID")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("external_format", &self.external_format)
            .finish()
    }
}
impl ExternalFormatANDROID {
    #[inline]
    pub fn into_builder<'a>(self) -> ExternalFormatANDROIDBuilder<'a> {
        ExternalFormatANDROIDBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalFormatANDROID.html) · Builder of [`ExternalFormatANDROID`]"]
#[repr(transparent)]
pub struct ExternalFormatANDROIDBuilder<'a>(ExternalFormatANDROID, std::marker::PhantomData<&'a ()>);
impl<'a> ExternalFormatANDROIDBuilder<'a> {
    #[inline]
    pub fn new() -> ExternalFormatANDROIDBuilder<'a> {
        ExternalFormatANDROIDBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn external_format(mut self, external_format: u64) -> Self {
        self.0.external_format = external_format as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ExternalFormatANDROID {
        self.0
    }
}
impl<'a> std::default::Default for ExternalFormatANDROIDBuilder<'a> {
    fn default() -> ExternalFormatANDROIDBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ExternalFormatANDROIDBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
unsafe impl crate::Repr<ExternalFormatANDROID> for ExternalFormatANDROIDBuilder<'_> {}
#[doc = "Provided by [`crate::extensions::android_external_memory_android_hardware_buffer`]"]
impl crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetAndroidHardwareBufferPropertiesANDROID.html) · Function"]
    #[doc(alias = "vkGetAndroidHardwareBufferPropertiesANDROID")]
    pub unsafe fn get_android_hardware_buffer_properties_android(
        &self,
        buffer: &crate::extensions::android_external_memory_android_hardware_buffer::AHardwareBuffer,
        properties: Option<crate::extensions::android_external_memory_android_hardware_buffer::AndroidHardwareBufferPropertiesANDROID>,
    ) -> crate::utils::VulkanResult<crate::extensions::android_external_memory_android_hardware_buffer::AndroidHardwareBufferPropertiesANDROID> {
        let _function = self
            .get_android_hardware_buffer_properties_android
            .expect("`get_android_hardware_buffer_properties_android` is not loaded");
        let mut properties = match properties {
            Some(v) => v,
            None => Default::default(),
        };
        let _return = _function(self.handle, buffer as _, &mut properties);
        crate::utils::VulkanResult::new(_return, properties)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryAndroidHardwareBufferANDROID.html) · Function"]
    #[doc(alias = "vkGetMemoryAndroidHardwareBufferANDROID")]
    pub unsafe fn get_memory_android_hardware_buffer_android(
        &self,
        info: &crate::extensions::android_external_memory_android_hardware_buffer::MemoryGetAndroidHardwareBufferInfoANDROID,
        buffer: Option<*mut crate::extensions::android_external_memory_android_hardware_buffer::AHardwareBuffer>,
    ) -> crate::utils::VulkanResult<*mut crate::extensions::android_external_memory_android_hardware_buffer::AHardwareBuffer> {
        let _function = self.get_memory_android_hardware_buffer_android.expect("`get_memory_android_hardware_buffer_android` is not loaded");
        let mut buffer = match buffer {
            Some(v) => v,
            None => std::ptr::null_mut(),
        };
        let _return = _function(self.handle, info as _, &mut buffer);
        crate::utils::VulkanResult::new(_return, buffer)
    }
}
