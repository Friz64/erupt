# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_KHR_sampler_ycbcr_conversion.html)\n\n## Extends\n- [`ChromaLocation`](../../vk1_1/struct.ChromaLocation.html)\n- [`DebugReportObjectTypeEXT`](../../extensions/ext_debug_report/struct.DebugReportObjectTypeEXT.html)\n- [`Format`](../../vk1_0/struct.Format.html)\n- [`FormatFeatureFlagBits`](../../vk1_0/struct.FormatFeatureFlagBits.html)\n- [`ImageAspectFlagBits`](../../vk1_0/struct.ImageAspectFlagBits.html)\n- [`ImageCreateFlagBits`](../../vk1_0/struct.ImageCreateFlagBits.html)\n- [`ObjectType`](../../vk1_0/struct.ObjectType.html)\n- [`SamplerYcbcrModelConversion`](../../vk1_1/struct.SamplerYcbcrModelConversion.html)\n- [`SamplerYcbcrRange`](../../vk1_1/struct.SamplerYcbcrRange.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_SAMPLER_YCBCR_CONVERSION_SPEC_VERSION: u32 = 14;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_SAMPLER_YCBCR_CONVERSION_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_sampler_ycbcr_conversion");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateSamplerYcbcrConversionKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateSamplerYcbcrConversionKHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_create_info: *const crate::vk1_1::SamplerYcbcrConversionCreateInfo,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
    p_ycbcr_conversion: *mut crate::vk1_1::SamplerYcbcrConversion,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroySamplerYcbcrConversionKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroySamplerYcbcrConversionKHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    ycbcr_conversion: crate::vk1_1::SamplerYcbcrConversion,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
) -> std::ffi::c_void;
#[doc = "Provides Device Commands for [`KhrSamplerYcbcrConversionDeviceLoaderExt`](trait.KhrSamplerYcbcrConversionDeviceLoaderExt.html)"]
pub struct KhrSamplerYcbcrConversionDeviceCommands {
    pub create_sampler_ycbcr_conversion_khr: Option<PFN_vkCreateSamplerYcbcrConversionKHR>,
    pub destroy_sampler_ycbcr_conversion_khr: Option<PFN_vkDestroySamplerYcbcrConversionKHR>,
}
impl KhrSamplerYcbcrConversionDeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<KhrSamplerYcbcrConversionDeviceCommands> {
        unsafe {
            let mut success = false;
            let table = KhrSamplerYcbcrConversionDeviceCommands {
                create_sampler_ycbcr_conversion_khr: std::mem::transmute({
                    let symbol = loader.symbol("vkCreateSamplerYcbcrConversionKHR");
                    success |= symbol.is_some();
                    symbol
                }),
                destroy_sampler_ycbcr_conversion_khr: std::mem::transmute({
                    let symbol = loader.symbol("vkDestroySamplerYcbcrConversionKHR");
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
fn device_commands(loader: &crate::DeviceLoader) -> &KhrSamplerYcbcrConversionDeviceCommands {
    loader
        .khr_sampler_ycbcr_conversion
        .as_ref()
        .expect("`khr_sampler_ycbcr_conversion` not loaded")
}
#[doc = "Provides high level command wrappers for [`KhrSamplerYcbcrConversionDeviceCommands`](struct.KhrSamplerYcbcrConversionDeviceCommands.html)"]
pub trait KhrSamplerYcbcrConversionDeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateSamplerYcbcrConversionKHR.html) · Device Command"]
    unsafe fn create_sampler_ycbcr_conversion_khr(
        &self,
        create_info: &crate::vk1_1::SamplerYcbcrConversionCreateInfo,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        ycbcr_conversion: Option<crate::vk1_1::SamplerYcbcrConversion>,
    ) -> crate::utils::VulkanResult<crate::vk1_1::SamplerYcbcrConversion>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroySamplerYcbcrConversionKHR.html) · Device Command"]
    unsafe fn destroy_sampler_ycbcr_conversion_khr(
        &self,
        ycbcr_conversion: crate::vk1_1::SamplerYcbcrConversion,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
    ) -> ();
}
impl KhrSamplerYcbcrConversionDeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateSamplerYcbcrConversionKHR.html) · Device Command"]
    unsafe fn create_sampler_ycbcr_conversion_khr(
        &self,
        create_info: &crate::vk1_1::SamplerYcbcrConversionCreateInfo,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        ycbcr_conversion: Option<crate::vk1_1::SamplerYcbcrConversion>,
    ) -> crate::utils::VulkanResult<crate::vk1_1::SamplerYcbcrConversion> {
        let function = device_commands(self)
            .create_sampler_ycbcr_conversion_khr
            .as_ref()
            .expect("`create_sampler_ycbcr_conversion_khr` not available");
        let mut ycbcr_conversion = ycbcr_conversion.unwrap_or_else(|| Default::default());
        let _val = function(
            self.handle,
            create_info,
            if let Some(allocator) = allocator {
                allocator
            } else {
                std::ptr::null()
            },
            &mut ycbcr_conversion,
        );
        crate::utils::VulkanResult::new(_val, ycbcr_conversion)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroySamplerYcbcrConversionKHR.html) · Device Command"]
    unsafe fn destroy_sampler_ycbcr_conversion_khr(
        &self,
        ycbcr_conversion: crate::vk1_1::SamplerYcbcrConversion,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
    ) -> () {
        let function = device_commands(self)
            .destroy_sampler_ycbcr_conversion_khr
            .as_ref()
            .expect("`destroy_sampler_ycbcr_conversion_khr` not available");
        let _val = function(
            self.handle,
            ycbcr_conversion,
            if let Some(allocator) = allocator {
                allocator
            } else {
                std::ptr::null()
            },
        );
        ()
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerYcbcrConversionCreateInfoKHR.html) · Alias"]
pub type SamplerYcbcrConversionCreateInfoKHR = crate::vk1_1::SamplerYcbcrConversionCreateInfo;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerYcbcrConversionInfoKHR.html) · Alias"]
pub type SamplerYcbcrConversionInfoKHR = crate::vk1_1::SamplerYcbcrConversionInfo;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindImagePlaneMemoryInfoKHR.html) · Alias"]
pub type BindImagePlaneMemoryInfoKHR = crate::vk1_1::BindImagePlaneMemoryInfo;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImagePlaneMemoryRequirementsInfoKHR.html) · Alias"]
pub type ImagePlaneMemoryRequirementsInfoKHR = crate::vk1_1::ImagePlaneMemoryRequirementsInfo;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR.html) · Alias"]
pub type PhysicalDeviceSamplerYcbcrConversionFeaturesKHR =
    crate::vk1_1::PhysicalDeviceSamplerYcbcrConversionFeatures;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerYcbcrConversionImageFormatPropertiesKHR.html) · Alias"]
pub type SamplerYcbcrConversionImageFormatPropertiesKHR =
    crate::vk1_1::SamplerYcbcrConversionImageFormatProperties;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerYcbcrConversionKHR.html) · Alias"]
pub type SamplerYcbcrConversionKHR = crate::vk1_1::SamplerYcbcrConversion;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerYcbcrModelConversionKHR.html) · Alias"]
pub type SamplerYcbcrModelConversionKHR = crate::vk1_1::SamplerYcbcrModelConversion;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerYcbcrRangeKHR.html) · Alias"]
pub type SamplerYcbcrRangeKHR = crate::vk1_1::SamplerYcbcrRange;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkChromaLocationKHR.html) · Alias"]
pub type ChromaLocationKHR = crate::vk1_1::ChromaLocation;
