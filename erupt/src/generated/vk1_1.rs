#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const MAX_DEVICE_GROUP_SIZE: u32 = 32;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const LUID_SIZE: u32 = 8;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const QUEUE_FAMILY_EXTERNAL: u32 = !0u32 - 1;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumerateInstanceVersion.html) · Core Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkEnumerateInstanceVersion =
    unsafe extern "system" fn(p_api_version: *mut u32) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBindBufferMemory2.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkBindBufferMemory2 = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    bind_info_count: u32,
    p_bind_infos: *const crate::vk1_1::BindBufferMemoryInfo,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBindImageMemory2.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkBindImageMemory2 = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    bind_info_count: u32,
    p_bind_infos: *const crate::vk1_1::BindImageMemoryInfo,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceGroupPeerMemoryFeatures.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceGroupPeerMemoryFeatures = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    heap_index: u32,
    local_device_index: u32,
    remote_device_index: u32,
    p_peer_memory_features: *mut crate::vk1_1::PeerMemoryFeatureFlags,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDeviceMask.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDeviceMask = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    device_mask: u32,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDispatchBase.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDispatchBase = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    base_group_x: u32,
    base_group_y: u32,
    base_group_z: u32,
    group_count_x: u32,
    group_count_y: u32,
    group_count_z: u32,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumeratePhysicalDeviceGroups.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkEnumeratePhysicalDeviceGroups = unsafe extern "system" fn(
    instance: crate::vk1_0::Instance,
    p_physical_device_group_count: *mut u32,
    p_physical_device_group_properties: *mut crate::vk1_1::PhysicalDeviceGroupProperties,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageMemoryRequirements2.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageMemoryRequirements2 = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_info: *const crate::vk1_1::ImageMemoryRequirementsInfo2,
    p_memory_requirements: *mut crate::vk1_1::MemoryRequirements2,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetBufferMemoryRequirements2.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetBufferMemoryRequirements2 = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_info: *const crate::vk1_1::BufferMemoryRequirementsInfo2,
    p_memory_requirements: *mut crate::vk1_1::MemoryRequirements2,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageSparseMemoryRequirements2.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageSparseMemoryRequirements2 = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_info: *const crate::vk1_1::ImageSparseMemoryRequirementsInfo2,
    p_sparse_memory_requirement_count: *mut u32,
    p_sparse_memory_requirements: *mut crate::vk1_1::SparseImageMemoryRequirements2,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceFeatures2.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceFeatures2 = unsafe extern "system" fn(
    physical_device: crate::vk1_0::PhysicalDevice,
    p_features: *mut crate::vk1_1::PhysicalDeviceFeatures2,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceProperties2.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceProperties2 = unsafe extern "system" fn(
    physical_device: crate::vk1_0::PhysicalDevice,
    p_properties: *mut crate::vk1_1::PhysicalDeviceProperties2,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceFormatProperties2.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceFormatProperties2 = unsafe extern "system" fn(
    physical_device: crate::vk1_0::PhysicalDevice,
    format: crate::vk1_0::Format,
    p_format_properties: *mut crate::vk1_1::FormatProperties2,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceImageFormatProperties2.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceImageFormatProperties2 =
    unsafe extern "system" fn(
        physical_device: crate::vk1_0::PhysicalDevice,
        p_image_format_info: *const crate::vk1_1::PhysicalDeviceImageFormatInfo2,
        p_image_format_properties: *mut crate::vk1_1::ImageFormatProperties2,
    ) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties2.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties2 =
    unsafe extern "system" fn(
        physical_device: crate::vk1_0::PhysicalDevice,
        p_queue_family_property_count: *mut u32,
        p_queue_family_properties: *mut crate::vk1_1::QueueFamilyProperties2,
    ) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceMemoryProperties2.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceMemoryProperties2 = unsafe extern "system" fn(
    physical_device: crate::vk1_0::PhysicalDevice,
    p_memory_properties: *mut crate::vk1_1::PhysicalDeviceMemoryProperties2,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSparseImageFormatProperties2.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties2 =
    unsafe extern "system" fn(
        physical_device: crate::vk1_0::PhysicalDevice,
        p_format_info: *const crate::vk1_1::PhysicalDeviceSparseImageFormatInfo2,
        p_property_count: *mut u32,
        p_properties: *mut crate::vk1_1::SparseImageFormatProperties2,
    ) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkTrimCommandPool.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkTrimCommandPool = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    command_pool: crate::vk1_0::CommandPool,
    flags: crate::vk1_1::CommandPoolTrimFlags,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceQueue2.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceQueue2 = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_queue_info: *const crate::vk1_1::DeviceQueueInfo2,
    p_queue: *mut crate::vk1_0::Queue,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateSamplerYcbcrConversion.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateSamplerYcbcrConversion = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_create_info: *const crate::vk1_1::SamplerYcbcrConversionCreateInfo,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
    p_ycbcr_conversion: *mut crate::vk1_1::SamplerYcbcrConversion,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroySamplerYcbcrConversion.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroySamplerYcbcrConversion = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    ycbcr_conversion: crate::vk1_1::SamplerYcbcrConversion,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDescriptorUpdateTemplate.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateDescriptorUpdateTemplate = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_create_info: *const crate::vk1_1::DescriptorUpdateTemplateCreateInfo,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
    p_descriptor_update_template: *mut crate::vk1_1::DescriptorUpdateTemplate,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyDescriptorUpdateTemplate.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyDescriptorUpdateTemplate = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    descriptor_update_template: crate::vk1_1::DescriptorUpdateTemplate,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkUpdateDescriptorSetWithTemplate.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkUpdateDescriptorSetWithTemplate = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    descriptor_set: crate::vk1_0::DescriptorSet,
    descriptor_update_template: crate::vk1_1::DescriptorUpdateTemplate,
    p_data: *const std::ffi::c_void,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceExternalBufferProperties.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceExternalBufferProperties =
    unsafe extern "system" fn(
        physical_device: crate::vk1_0::PhysicalDevice,
        p_external_buffer_info: *const crate::vk1_1::PhysicalDeviceExternalBufferInfo,
        p_external_buffer_properties: *mut crate::vk1_1::ExternalBufferProperties,
    ) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceExternalFenceProperties.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceExternalFenceProperties =
    unsafe extern "system" fn(
        physical_device: crate::vk1_0::PhysicalDevice,
        p_external_fence_info: *const crate::vk1_1::PhysicalDeviceExternalFenceInfo,
        p_external_fence_properties: *mut crate::vk1_1::ExternalFenceProperties,
    ) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceExternalSemaphoreProperties.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceExternalSemaphoreProperties =
    unsafe extern "system" fn(
        physical_device: crate::vk1_0::PhysicalDevice,
        p_external_semaphore_info: *const crate::vk1_1::PhysicalDeviceExternalSemaphoreInfo,
        p_external_semaphore_properties: *mut crate::vk1_1::ExternalSemaphoreProperties,
    ) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDescriptorSetLayoutSupport.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDescriptorSetLayoutSupport = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_create_info: *const crate::vk1_0::DescriptorSetLayoutCreateInfo,
    p_support: *mut crate::vk1_1::DescriptorSetLayoutSupport,
) -> std::ffi::c_void;
#[doc = "Provides Core Commands for [`Vk11CoreLoaderExt`](trait.Vk11CoreLoaderExt.html)"]
pub struct Vk11CoreCommands {
    pub enumerate_instance_version: PFN_vkEnumerateInstanceVersion,
}
impl Vk11CoreCommands {
    #[inline]
    pub fn load<T>(loader: &crate::CoreLoader<T>) -> Option<Vk11CoreCommands> {
        unsafe {
            Some(Vk11CoreCommands {
                enumerate_instance_version: std::mem::transmute(
                    loader.symbol("vkEnumerateInstanceVersion")?,
                ),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`Vk11CoreCommands`](struct.Vk11CoreCommands.html)"]
pub trait Vk11CoreLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumerateInstanceVersion.html) · Core Command"]
    unsafe fn enumerate_instance_version(
        &self,
        api_version: Option<u32>,
    ) -> crate::utils::VulkanResult<u32>;
}
impl<T> Vk11CoreLoaderExt for crate::CoreLoader<T> {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumerateInstanceVersion.html) · Core Command"]
    unsafe fn enumerate_instance_version(
        &self,
        api_version: Option<u32>,
    ) -> crate::utils::VulkanResult<u32> {
        let function = self
            .vk1_1
            .as_ref()
            .expect("`vk1_1` not loaded")
            .enumerate_instance_version;
        let mut api_version = api_version.unwrap_or_else(|| Default::default());
        let _val = function(&mut api_version);
        crate::utils::VulkanResult::new(_val, api_version)
    }
}
#[doc = "Provides Instance Commands for [`Vk11InstanceLoaderExt`](trait.Vk11InstanceLoaderExt.html)"]
pub struct Vk11InstanceCommands {
    pub enumerate_physical_device_groups: PFN_vkEnumeratePhysicalDeviceGroups,
    pub get_physical_device_features2: PFN_vkGetPhysicalDeviceFeatures2,
    pub get_physical_device_properties2: PFN_vkGetPhysicalDeviceProperties2,
    pub get_physical_device_format_properties2: PFN_vkGetPhysicalDeviceFormatProperties2,
    pub get_physical_device_image_format_properties2: PFN_vkGetPhysicalDeviceImageFormatProperties2,
    pub get_physical_device_queue_family_properties2: PFN_vkGetPhysicalDeviceQueueFamilyProperties2,
    pub get_physical_device_memory_properties2: PFN_vkGetPhysicalDeviceMemoryProperties2,
    pub get_physical_device_sparse_image_format_properties2:
        PFN_vkGetPhysicalDeviceSparseImageFormatProperties2,
    pub get_physical_device_external_buffer_properties:
        PFN_vkGetPhysicalDeviceExternalBufferProperties,
    pub get_physical_device_external_fence_properties:
        PFN_vkGetPhysicalDeviceExternalFenceProperties,
    pub get_physical_device_external_semaphore_properties:
        PFN_vkGetPhysicalDeviceExternalSemaphoreProperties,
}
impl Vk11InstanceCommands {
    #[inline]
    pub fn load(loader: &crate::InstanceLoader) -> Option<Vk11InstanceCommands> {
        unsafe {
            Some(Vk11InstanceCommands {
                enumerate_physical_device_groups: std::mem::transmute(
                    loader.symbol("vkEnumeratePhysicalDeviceGroups")?,
                ),
                get_physical_device_features2: std::mem::transmute(
                    loader.symbol("vkGetPhysicalDeviceFeatures2")?,
                ),
                get_physical_device_properties2: std::mem::transmute(
                    loader.symbol("vkGetPhysicalDeviceProperties2")?,
                ),
                get_physical_device_format_properties2: std::mem::transmute(
                    loader.symbol("vkGetPhysicalDeviceFormatProperties2")?,
                ),
                get_physical_device_image_format_properties2: std::mem::transmute(
                    loader.symbol("vkGetPhysicalDeviceImageFormatProperties2")?,
                ),
                get_physical_device_queue_family_properties2: std::mem::transmute(
                    loader.symbol("vkGetPhysicalDeviceQueueFamilyProperties2")?,
                ),
                get_physical_device_memory_properties2: std::mem::transmute(
                    loader.symbol("vkGetPhysicalDeviceMemoryProperties2")?,
                ),
                get_physical_device_sparse_image_format_properties2: std::mem::transmute(
                    loader.symbol("vkGetPhysicalDeviceSparseImageFormatProperties2")?,
                ),
                get_physical_device_external_buffer_properties: std::mem::transmute(
                    loader.symbol("vkGetPhysicalDeviceExternalBufferProperties")?,
                ),
                get_physical_device_external_fence_properties: std::mem::transmute(
                    loader.symbol("vkGetPhysicalDeviceExternalFenceProperties")?,
                ),
                get_physical_device_external_semaphore_properties: std::mem::transmute(
                    loader.symbol("vkGetPhysicalDeviceExternalSemaphoreProperties")?,
                ),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`Vk11InstanceCommands`](struct.Vk11InstanceCommands.html)"]
pub trait Vk11InstanceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumeratePhysicalDeviceGroups.html) · Instance Command"]
    unsafe fn enumerate_physical_device_groups(
        &self,
        physical_device_group_count: Option<u32>,
    ) -> crate::utils::VulkanResult<Vec<crate::vk1_1::PhysicalDeviceGroupProperties>>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceFeatures2.html) · Instance Command"]
    unsafe fn get_physical_device_features2(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        features: Option<crate::vk1_1::PhysicalDeviceFeatures2>,
    ) -> crate::vk1_1::PhysicalDeviceFeatures2;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceProperties2.html) · Instance Command"]
    unsafe fn get_physical_device_properties2(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        properties: Option<crate::vk1_1::PhysicalDeviceProperties2>,
    ) -> crate::vk1_1::PhysicalDeviceProperties2;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceFormatProperties2.html) · Instance Command"]
    unsafe fn get_physical_device_format_properties2(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        format: crate::vk1_0::Format,
        format_properties: Option<crate::vk1_1::FormatProperties2>,
    ) -> crate::vk1_1::FormatProperties2;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceImageFormatProperties2.html) · Instance Command"]
    unsafe fn get_physical_device_image_format_properties2(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        image_format_info: &crate::vk1_1::PhysicalDeviceImageFormatInfo2,
        image_format_properties: Option<crate::vk1_1::ImageFormatProperties2>,
    ) -> crate::utils::VulkanResult<crate::vk1_1::ImageFormatProperties2>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties2.html) · Instance Command"]
    unsafe fn get_physical_device_queue_family_properties2(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        queue_family_property_count: Option<u32>,
    ) -> Vec<crate::vk1_1::QueueFamilyProperties2>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceMemoryProperties2.html) · Instance Command"]
    unsafe fn get_physical_device_memory_properties2(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        memory_properties: Option<crate::vk1_1::PhysicalDeviceMemoryProperties2>,
    ) -> crate::vk1_1::PhysicalDeviceMemoryProperties2;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSparseImageFormatProperties2.html) · Instance Command"]
    unsafe fn get_physical_device_sparse_image_format_properties2(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        format_info: &crate::vk1_1::PhysicalDeviceSparseImageFormatInfo2,
        property_count: Option<u32>,
    ) -> Vec<crate::vk1_1::SparseImageFormatProperties2>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceExternalBufferProperties.html) · Instance Command"]
    unsafe fn get_physical_device_external_buffer_properties(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        external_buffer_info: &crate::vk1_1::PhysicalDeviceExternalBufferInfo,
        external_buffer_properties: Option<crate::vk1_1::ExternalBufferProperties>,
    ) -> crate::vk1_1::ExternalBufferProperties;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceExternalFenceProperties.html) · Instance Command"]
    unsafe fn get_physical_device_external_fence_properties(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        external_fence_info: &crate::vk1_1::PhysicalDeviceExternalFenceInfo,
        external_fence_properties: Option<crate::vk1_1::ExternalFenceProperties>,
    ) -> crate::vk1_1::ExternalFenceProperties;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceExternalSemaphoreProperties.html) · Instance Command"]
    unsafe fn get_physical_device_external_semaphore_properties(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        external_semaphore_info: &crate::vk1_1::PhysicalDeviceExternalSemaphoreInfo,
        external_semaphore_properties: Option<crate::vk1_1::ExternalSemaphoreProperties>,
    ) -> crate::vk1_1::ExternalSemaphoreProperties;
}
impl Vk11InstanceLoaderExt for crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumeratePhysicalDeviceGroups.html) · Instance Command"]
    unsafe fn enumerate_physical_device_groups(
        &self,
        physical_device_group_count: Option<u32>,
    ) -> crate::utils::VulkanResult<Vec<crate::vk1_1::PhysicalDeviceGroupProperties>> {
        let function = self
            .vk1_1
            .as_ref()
            .expect("`vk1_1` not loaded")
            .enumerate_physical_device_groups;
        let mut physical_device_group_count = physical_device_group_count.unwrap_or_else(|| {
            let mut val = Default::default();
            function(self.handle, &mut val, std::ptr::null_mut());
            val
        });
        let mut physical_device_group_properties =
            vec![Default::default(); physical_device_group_count as _];
        let _val = function(
            self.handle,
            &mut physical_device_group_count,
            physical_device_group_properties.as_mut_ptr(),
        );
        crate::utils::VulkanResult::new(_val, physical_device_group_properties)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceFeatures2.html) · Instance Command"]
    unsafe fn get_physical_device_features2(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        features: Option<crate::vk1_1::PhysicalDeviceFeatures2>,
    ) -> crate::vk1_1::PhysicalDeviceFeatures2 {
        let function = self
            .vk1_1
            .as_ref()
            .expect("`vk1_1` not loaded")
            .get_physical_device_features2;
        let mut features = features.unwrap_or_else(|| Default::default());
        let _val = function(physical_device, &mut features);
        features
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceProperties2.html) · Instance Command"]
    unsafe fn get_physical_device_properties2(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        properties: Option<crate::vk1_1::PhysicalDeviceProperties2>,
    ) -> crate::vk1_1::PhysicalDeviceProperties2 {
        let function = self
            .vk1_1
            .as_ref()
            .expect("`vk1_1` not loaded")
            .get_physical_device_properties2;
        let mut properties = properties.unwrap_or_else(|| Default::default());
        let _val = function(physical_device, &mut properties);
        properties
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceFormatProperties2.html) · Instance Command"]
    unsafe fn get_physical_device_format_properties2(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        format: crate::vk1_0::Format,
        format_properties: Option<crate::vk1_1::FormatProperties2>,
    ) -> crate::vk1_1::FormatProperties2 {
        let function = self
            .vk1_1
            .as_ref()
            .expect("`vk1_1` not loaded")
            .get_physical_device_format_properties2;
        let mut format_properties = format_properties.unwrap_or_else(|| Default::default());
        let _val = function(physical_device, format, &mut format_properties);
        format_properties
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceImageFormatProperties2.html) · Instance Command"]
    unsafe fn get_physical_device_image_format_properties2(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        image_format_info: &crate::vk1_1::PhysicalDeviceImageFormatInfo2,
        image_format_properties: Option<crate::vk1_1::ImageFormatProperties2>,
    ) -> crate::utils::VulkanResult<crate::vk1_1::ImageFormatProperties2> {
        let function = self
            .vk1_1
            .as_ref()
            .expect("`vk1_1` not loaded")
            .get_physical_device_image_format_properties2;
        let mut image_format_properties =
            image_format_properties.unwrap_or_else(|| Default::default());
        let _val = function(
            physical_device,
            image_format_info,
            &mut image_format_properties,
        );
        crate::utils::VulkanResult::new(_val, image_format_properties)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties2.html) · Instance Command"]
    unsafe fn get_physical_device_queue_family_properties2(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        queue_family_property_count: Option<u32>,
    ) -> Vec<crate::vk1_1::QueueFamilyProperties2> {
        let function = self
            .vk1_1
            .as_ref()
            .expect("`vk1_1` not loaded")
            .get_physical_device_queue_family_properties2;
        let mut queue_family_property_count = queue_family_property_count.unwrap_or_else(|| {
            let mut val = Default::default();
            function(physical_device, &mut val, std::ptr::null_mut());
            val
        });
        let mut queue_family_properties =
            vec![Default::default(); queue_family_property_count as _];
        let _val = function(
            physical_device,
            &mut queue_family_property_count,
            queue_family_properties.as_mut_ptr(),
        );
        queue_family_properties
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceMemoryProperties2.html) · Instance Command"]
    unsafe fn get_physical_device_memory_properties2(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        memory_properties: Option<crate::vk1_1::PhysicalDeviceMemoryProperties2>,
    ) -> crate::vk1_1::PhysicalDeviceMemoryProperties2 {
        let function = self
            .vk1_1
            .as_ref()
            .expect("`vk1_1` not loaded")
            .get_physical_device_memory_properties2;
        let mut memory_properties = memory_properties.unwrap_or_else(|| Default::default());
        let _val = function(physical_device, &mut memory_properties);
        memory_properties
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSparseImageFormatProperties2.html) · Instance Command"]
    unsafe fn get_physical_device_sparse_image_format_properties2(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        format_info: &crate::vk1_1::PhysicalDeviceSparseImageFormatInfo2,
        property_count: Option<u32>,
    ) -> Vec<crate::vk1_1::SparseImageFormatProperties2> {
        let function = self
            .vk1_1
            .as_ref()
            .expect("`vk1_1` not loaded")
            .get_physical_device_sparse_image_format_properties2;
        let mut property_count = property_count.unwrap_or_else(|| {
            let mut val = Default::default();
            function(physical_device, format_info, &mut val, std::ptr::null_mut());
            val
        });
        let mut properties = vec![Default::default(); property_count as _];
        let _val = function(
            physical_device,
            format_info,
            &mut property_count,
            properties.as_mut_ptr(),
        );
        properties
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceExternalBufferProperties.html) · Instance Command"]
    unsafe fn get_physical_device_external_buffer_properties(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        external_buffer_info: &crate::vk1_1::PhysicalDeviceExternalBufferInfo,
        external_buffer_properties: Option<crate::vk1_1::ExternalBufferProperties>,
    ) -> crate::vk1_1::ExternalBufferProperties {
        let function = self
            .vk1_1
            .as_ref()
            .expect("`vk1_1` not loaded")
            .get_physical_device_external_buffer_properties;
        let mut external_buffer_properties =
            external_buffer_properties.unwrap_or_else(|| Default::default());
        let _val = function(
            physical_device,
            external_buffer_info,
            &mut external_buffer_properties,
        );
        external_buffer_properties
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceExternalFenceProperties.html) · Instance Command"]
    unsafe fn get_physical_device_external_fence_properties(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        external_fence_info: &crate::vk1_1::PhysicalDeviceExternalFenceInfo,
        external_fence_properties: Option<crate::vk1_1::ExternalFenceProperties>,
    ) -> crate::vk1_1::ExternalFenceProperties {
        let function = self
            .vk1_1
            .as_ref()
            .expect("`vk1_1` not loaded")
            .get_physical_device_external_fence_properties;
        let mut external_fence_properties =
            external_fence_properties.unwrap_or_else(|| Default::default());
        let _val = function(
            physical_device,
            external_fence_info,
            &mut external_fence_properties,
        );
        external_fence_properties
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceExternalSemaphoreProperties.html) · Instance Command"]
    unsafe fn get_physical_device_external_semaphore_properties(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        external_semaphore_info: &crate::vk1_1::PhysicalDeviceExternalSemaphoreInfo,
        external_semaphore_properties: Option<crate::vk1_1::ExternalSemaphoreProperties>,
    ) -> crate::vk1_1::ExternalSemaphoreProperties {
        let function = self
            .vk1_1
            .as_ref()
            .expect("`vk1_1` not loaded")
            .get_physical_device_external_semaphore_properties;
        let mut external_semaphore_properties =
            external_semaphore_properties.unwrap_or_else(|| Default::default());
        let _val = function(
            physical_device,
            external_semaphore_info,
            &mut external_semaphore_properties,
        );
        external_semaphore_properties
    }
}
#[doc = "Provides Device Commands for [`Vk11DeviceLoaderExt`](trait.Vk11DeviceLoaderExt.html)"]
pub struct Vk11DeviceCommands {
    pub bind_buffer_memory2: PFN_vkBindBufferMemory2,
    pub bind_image_memory2: PFN_vkBindImageMemory2,
    pub get_device_group_peer_memory_features: PFN_vkGetDeviceGroupPeerMemoryFeatures,
    pub cmd_set_device_mask: PFN_vkCmdSetDeviceMask,
    pub cmd_dispatch_base: PFN_vkCmdDispatchBase,
    pub get_image_memory_requirements2: PFN_vkGetImageMemoryRequirements2,
    pub get_buffer_memory_requirements2: PFN_vkGetBufferMemoryRequirements2,
    pub get_image_sparse_memory_requirements2: PFN_vkGetImageSparseMemoryRequirements2,
    pub trim_command_pool: PFN_vkTrimCommandPool,
    pub get_device_queue2: PFN_vkGetDeviceQueue2,
    pub create_sampler_ycbcr_conversion: PFN_vkCreateSamplerYcbcrConversion,
    pub destroy_sampler_ycbcr_conversion: PFN_vkDestroySamplerYcbcrConversion,
    pub create_descriptor_update_template: PFN_vkCreateDescriptorUpdateTemplate,
    pub destroy_descriptor_update_template: PFN_vkDestroyDescriptorUpdateTemplate,
    pub update_descriptor_set_with_template: PFN_vkUpdateDescriptorSetWithTemplate,
    pub get_descriptor_set_layout_support: PFN_vkGetDescriptorSetLayoutSupport,
}
impl Vk11DeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<Vk11DeviceCommands> {
        unsafe {
            Some(Vk11DeviceCommands {
                bind_buffer_memory2: std::mem::transmute(loader.symbol("vkBindBufferMemory2")?),
                bind_image_memory2: std::mem::transmute(loader.symbol("vkBindImageMemory2")?),
                get_device_group_peer_memory_features: std::mem::transmute(
                    loader.symbol("vkGetDeviceGroupPeerMemoryFeatures")?,
                ),
                cmd_set_device_mask: std::mem::transmute(loader.symbol("vkCmdSetDeviceMask")?),
                cmd_dispatch_base: std::mem::transmute(loader.symbol("vkCmdDispatchBase")?),
                get_image_memory_requirements2: std::mem::transmute(
                    loader.symbol("vkGetImageMemoryRequirements2")?,
                ),
                get_buffer_memory_requirements2: std::mem::transmute(
                    loader.symbol("vkGetBufferMemoryRequirements2")?,
                ),
                get_image_sparse_memory_requirements2: std::mem::transmute(
                    loader.symbol("vkGetImageSparseMemoryRequirements2")?,
                ),
                trim_command_pool: std::mem::transmute(loader.symbol("vkTrimCommandPool")?),
                get_device_queue2: std::mem::transmute(loader.symbol("vkGetDeviceQueue2")?),
                create_sampler_ycbcr_conversion: std::mem::transmute(
                    loader.symbol("vkCreateSamplerYcbcrConversion")?,
                ),
                destroy_sampler_ycbcr_conversion: std::mem::transmute(
                    loader.symbol("vkDestroySamplerYcbcrConversion")?,
                ),
                create_descriptor_update_template: std::mem::transmute(
                    loader.symbol("vkCreateDescriptorUpdateTemplate")?,
                ),
                destroy_descriptor_update_template: std::mem::transmute(
                    loader.symbol("vkDestroyDescriptorUpdateTemplate")?,
                ),
                update_descriptor_set_with_template: std::mem::transmute(
                    loader.symbol("vkUpdateDescriptorSetWithTemplate")?,
                ),
                get_descriptor_set_layout_support: std::mem::transmute(
                    loader.symbol("vkGetDescriptorSetLayoutSupport")?,
                ),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`Vk11DeviceCommands`](struct.Vk11DeviceCommands.html)"]
pub trait Vk11DeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBindBufferMemory2.html) · Device Command"]
    unsafe fn bind_buffer_memory2(
        &self,
        bind_infos: &[crate::vk1_1::BindBufferMemoryInfoBuilder],
    ) -> crate::utils::VulkanResult<()>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBindImageMemory2.html) · Device Command"]
    unsafe fn bind_image_memory2(
        &self,
        bind_infos: &[crate::vk1_1::BindImageMemoryInfoBuilder],
    ) -> crate::utils::VulkanResult<()>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceGroupPeerMemoryFeatures.html) · Device Command"]
    unsafe fn get_device_group_peer_memory_features(
        &self,
        heap_index: u32,
        local_device_index: u32,
        remote_device_index: u32,
        peer_memory_features: Option<crate::vk1_1::PeerMemoryFeatureFlags>,
    ) -> crate::vk1_1::PeerMemoryFeatureFlags;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDeviceMask.html) · Device Command"]
    unsafe fn cmd_set_device_mask(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        device_mask: u32,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDispatchBase.html) · Device Command"]
    unsafe fn cmd_dispatch_base(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        base_group_x: u32,
        base_group_y: u32,
        base_group_z: u32,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageMemoryRequirements2.html) · Device Command"]
    unsafe fn get_image_memory_requirements2(
        &self,
        info: &crate::vk1_1::ImageMemoryRequirementsInfo2,
        memory_requirements: Option<crate::vk1_1::MemoryRequirements2>,
    ) -> crate::vk1_1::MemoryRequirements2;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetBufferMemoryRequirements2.html) · Device Command"]
    unsafe fn get_buffer_memory_requirements2(
        &self,
        info: &crate::vk1_1::BufferMemoryRequirementsInfo2,
        memory_requirements: Option<crate::vk1_1::MemoryRequirements2>,
    ) -> crate::vk1_1::MemoryRequirements2;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageSparseMemoryRequirements2.html) · Device Command"]
    unsafe fn get_image_sparse_memory_requirements2(
        &self,
        info: &crate::vk1_1::ImageSparseMemoryRequirementsInfo2,
        sparse_memory_requirement_count: Option<u32>,
    ) -> Vec<crate::vk1_1::SparseImageMemoryRequirements2>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkTrimCommandPool.html) · Device Command"]
    unsafe fn trim_command_pool(
        &self,
        command_pool: crate::vk1_0::CommandPool,
        flags: crate::vk1_1::CommandPoolTrimFlags,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceQueue2.html) · Device Command"]
    unsafe fn get_device_queue2(
        &self,
        queue_info: &crate::vk1_1::DeviceQueueInfo2,
        queue: Option<crate::vk1_0::Queue>,
    ) -> crate::vk1_0::Queue;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateSamplerYcbcrConversion.html) · Device Command"]
    unsafe fn create_sampler_ycbcr_conversion(
        &self,
        create_info: &crate::vk1_1::SamplerYcbcrConversionCreateInfo,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        ycbcr_conversion: Option<crate::vk1_1::SamplerYcbcrConversion>,
    ) -> crate::utils::VulkanResult<crate::vk1_1::SamplerYcbcrConversion>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroySamplerYcbcrConversion.html) · Device Command"]
    unsafe fn destroy_sampler_ycbcr_conversion(
        &self,
        ycbcr_conversion: crate::vk1_1::SamplerYcbcrConversion,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDescriptorUpdateTemplate.html) · Device Command"]
    unsafe fn create_descriptor_update_template(
        &self,
        create_info: &crate::vk1_1::DescriptorUpdateTemplateCreateInfo,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        descriptor_update_template: Option<crate::vk1_1::DescriptorUpdateTemplate>,
    ) -> crate::utils::VulkanResult<crate::vk1_1::DescriptorUpdateTemplate>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyDescriptorUpdateTemplate.html) · Device Command"]
    unsafe fn destroy_descriptor_update_template(
        &self,
        descriptor_update_template: crate::vk1_1::DescriptorUpdateTemplate,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkUpdateDescriptorSetWithTemplate.html) · Device Command"]
    unsafe fn update_descriptor_set_with_template(
        &self,
        descriptor_set: crate::vk1_0::DescriptorSet,
        descriptor_update_template: crate::vk1_1::DescriptorUpdateTemplate,
        data: *const std::ffi::c_void,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDescriptorSetLayoutSupport.html) · Device Command"]
    unsafe fn get_descriptor_set_layout_support(
        &self,
        create_info: &crate::vk1_0::DescriptorSetLayoutCreateInfo,
        support: Option<crate::vk1_1::DescriptorSetLayoutSupport>,
    ) -> crate::vk1_1::DescriptorSetLayoutSupport;
}
impl Vk11DeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBindBufferMemory2.html) · Device Command"]
    unsafe fn bind_buffer_memory2(
        &self,
        bind_infos: &[crate::vk1_1::BindBufferMemoryInfoBuilder],
    ) -> crate::utils::VulkanResult<()> {
        let function = self
            .vk1_1
            .as_ref()
            .expect("`vk1_1` not loaded")
            .bind_buffer_memory2;
        let bind_info_count = bind_infos.len() as _;
        let _val = function(self.handle, bind_info_count, bind_infos.as_ptr() as _);
        crate::utils::VulkanResult::new(_val, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBindImageMemory2.html) · Device Command"]
    unsafe fn bind_image_memory2(
        &self,
        bind_infos: &[crate::vk1_1::BindImageMemoryInfoBuilder],
    ) -> crate::utils::VulkanResult<()> {
        let function = self
            .vk1_1
            .as_ref()
            .expect("`vk1_1` not loaded")
            .bind_image_memory2;
        let bind_info_count = bind_infos.len() as _;
        let _val = function(self.handle, bind_info_count, bind_infos.as_ptr() as _);
        crate::utils::VulkanResult::new(_val, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceGroupPeerMemoryFeatures.html) · Device Command"]
    unsafe fn get_device_group_peer_memory_features(
        &self,
        heap_index: u32,
        local_device_index: u32,
        remote_device_index: u32,
        peer_memory_features: Option<crate::vk1_1::PeerMemoryFeatureFlags>,
    ) -> crate::vk1_1::PeerMemoryFeatureFlags {
        let function = self
            .vk1_1
            .as_ref()
            .expect("`vk1_1` not loaded")
            .get_device_group_peer_memory_features;
        let mut peer_memory_features = peer_memory_features.unwrap_or_else(|| Default::default());
        let _val = function(
            self.handle,
            heap_index,
            local_device_index,
            remote_device_index,
            &mut peer_memory_features,
        );
        peer_memory_features
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDeviceMask.html) · Device Command"]
    unsafe fn cmd_set_device_mask(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        device_mask: u32,
    ) -> () {
        let function = self
            .vk1_1
            .as_ref()
            .expect("`vk1_1` not loaded")
            .cmd_set_device_mask;
        let _val = function(command_buffer, device_mask);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDispatchBase.html) · Device Command"]
    unsafe fn cmd_dispatch_base(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        base_group_x: u32,
        base_group_y: u32,
        base_group_z: u32,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) -> () {
        let function = self
            .vk1_1
            .as_ref()
            .expect("`vk1_1` not loaded")
            .cmd_dispatch_base;
        let _val = function(
            command_buffer,
            base_group_x,
            base_group_y,
            base_group_z,
            group_count_x,
            group_count_y,
            group_count_z,
        );
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageMemoryRequirements2.html) · Device Command"]
    unsafe fn get_image_memory_requirements2(
        &self,
        info: &crate::vk1_1::ImageMemoryRequirementsInfo2,
        memory_requirements: Option<crate::vk1_1::MemoryRequirements2>,
    ) -> crate::vk1_1::MemoryRequirements2 {
        let function = self
            .vk1_1
            .as_ref()
            .expect("`vk1_1` not loaded")
            .get_image_memory_requirements2;
        let mut memory_requirements = memory_requirements.unwrap_or_else(|| Default::default());
        let _val = function(self.handle, info, &mut memory_requirements);
        memory_requirements
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetBufferMemoryRequirements2.html) · Device Command"]
    unsafe fn get_buffer_memory_requirements2(
        &self,
        info: &crate::vk1_1::BufferMemoryRequirementsInfo2,
        memory_requirements: Option<crate::vk1_1::MemoryRequirements2>,
    ) -> crate::vk1_1::MemoryRequirements2 {
        let function = self
            .vk1_1
            .as_ref()
            .expect("`vk1_1` not loaded")
            .get_buffer_memory_requirements2;
        let mut memory_requirements = memory_requirements.unwrap_or_else(|| Default::default());
        let _val = function(self.handle, info, &mut memory_requirements);
        memory_requirements
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageSparseMemoryRequirements2.html) · Device Command"]
    unsafe fn get_image_sparse_memory_requirements2(
        &self,
        info: &crate::vk1_1::ImageSparseMemoryRequirementsInfo2,
        sparse_memory_requirement_count: Option<u32>,
    ) -> Vec<crate::vk1_1::SparseImageMemoryRequirements2> {
        let function = self
            .vk1_1
            .as_ref()
            .expect("`vk1_1` not loaded")
            .get_image_sparse_memory_requirements2;
        let mut sparse_memory_requirement_count =
            sparse_memory_requirement_count.unwrap_or_else(|| {
                let mut val = Default::default();
                function(self.handle, info, &mut val, std::ptr::null_mut());
                val
            });
        let mut sparse_memory_requirements =
            vec![Default::default(); sparse_memory_requirement_count as _];
        let _val = function(
            self.handle,
            info,
            &mut sparse_memory_requirement_count,
            sparse_memory_requirements.as_mut_ptr(),
        );
        sparse_memory_requirements
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkTrimCommandPool.html) · Device Command"]
    unsafe fn trim_command_pool(
        &self,
        command_pool: crate::vk1_0::CommandPool,
        flags: crate::vk1_1::CommandPoolTrimFlags,
    ) -> () {
        let function = self
            .vk1_1
            .as_ref()
            .expect("`vk1_1` not loaded")
            .trim_command_pool;
        let _val = function(self.handle, command_pool, flags);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceQueue2.html) · Device Command"]
    unsafe fn get_device_queue2(
        &self,
        queue_info: &crate::vk1_1::DeviceQueueInfo2,
        queue: Option<crate::vk1_0::Queue>,
    ) -> crate::vk1_0::Queue {
        let function = self
            .vk1_1
            .as_ref()
            .expect("`vk1_1` not loaded")
            .get_device_queue2;
        let mut queue = queue.unwrap_or_else(|| Default::default());
        let _val = function(self.handle, queue_info, &mut queue);
        queue
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateSamplerYcbcrConversion.html) · Device Command"]
    unsafe fn create_sampler_ycbcr_conversion(
        &self,
        create_info: &crate::vk1_1::SamplerYcbcrConversionCreateInfo,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        ycbcr_conversion: Option<crate::vk1_1::SamplerYcbcrConversion>,
    ) -> crate::utils::VulkanResult<crate::vk1_1::SamplerYcbcrConversion> {
        let function = self
            .vk1_1
            .as_ref()
            .expect("`vk1_1` not loaded")
            .create_sampler_ycbcr_conversion;
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
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroySamplerYcbcrConversion.html) · Device Command"]
    unsafe fn destroy_sampler_ycbcr_conversion(
        &self,
        ycbcr_conversion: crate::vk1_1::SamplerYcbcrConversion,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
    ) -> () {
        let function = self
            .vk1_1
            .as_ref()
            .expect("`vk1_1` not loaded")
            .destroy_sampler_ycbcr_conversion;
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
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDescriptorUpdateTemplate.html) · Device Command"]
    unsafe fn create_descriptor_update_template(
        &self,
        create_info: &crate::vk1_1::DescriptorUpdateTemplateCreateInfo,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        descriptor_update_template: Option<crate::vk1_1::DescriptorUpdateTemplate>,
    ) -> crate::utils::VulkanResult<crate::vk1_1::DescriptorUpdateTemplate> {
        let function = self
            .vk1_1
            .as_ref()
            .expect("`vk1_1` not loaded")
            .create_descriptor_update_template;
        let mut descriptor_update_template =
            descriptor_update_template.unwrap_or_else(|| Default::default());
        let _val = function(
            self.handle,
            create_info,
            if let Some(allocator) = allocator {
                allocator
            } else {
                std::ptr::null()
            },
            &mut descriptor_update_template,
        );
        crate::utils::VulkanResult::new(_val, descriptor_update_template)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyDescriptorUpdateTemplate.html) · Device Command"]
    unsafe fn destroy_descriptor_update_template(
        &self,
        descriptor_update_template: crate::vk1_1::DescriptorUpdateTemplate,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
    ) -> () {
        let function = self
            .vk1_1
            .as_ref()
            .expect("`vk1_1` not loaded")
            .destroy_descriptor_update_template;
        let _val = function(
            self.handle,
            descriptor_update_template,
            if let Some(allocator) = allocator {
                allocator
            } else {
                std::ptr::null()
            },
        );
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkUpdateDescriptorSetWithTemplate.html) · Device Command"]
    unsafe fn update_descriptor_set_with_template(
        &self,
        descriptor_set: crate::vk1_0::DescriptorSet,
        descriptor_update_template: crate::vk1_1::DescriptorUpdateTemplate,
        data: *const std::ffi::c_void,
    ) -> () {
        let function = self
            .vk1_1
            .as_ref()
            .expect("`vk1_1` not loaded")
            .update_descriptor_set_with_template;
        let _val = function(
            self.handle,
            descriptor_set,
            descriptor_update_template,
            data,
        );
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDescriptorSetLayoutSupport.html) · Device Command"]
    unsafe fn get_descriptor_set_layout_support(
        &self,
        create_info: &crate::vk1_0::DescriptorSetLayoutCreateInfo,
        support: Option<crate::vk1_1::DescriptorSetLayoutSupport>,
    ) -> crate::vk1_1::DescriptorSetLayoutSupport {
        let function = self
            .vk1_1
            .as_ref()
            .expect("`vk1_1` not loaded")
            .get_descriptor_set_layout_support;
        let mut support = support.unwrap_or_else(|| Default::default());
        let _val = function(self.handle, create_info, &mut support);
        support
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindBufferMemoryInfo.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BindBufferMemoryInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub buffer: crate::vk1_0::Buffer,
    pub memory: crate::vk1_0::DeviceMemory,
    pub memory_offset: crate::vk1_0::DeviceSize,
}
impl BindBufferMemoryInfo {
    #[inline]
    pub fn builder<'a>(self) -> BindBufferMemoryInfoBuilder<'a> {
        BindBufferMemoryInfoBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for BindBufferMemoryInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("BindBufferMemoryInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("buffer", &self.buffer)
            .field("memory", &self.memory)
            .field("memory_offset", &self.memory_offset)
            .finish()
    }
}
impl Default for BindBufferMemoryInfo {
    fn default() -> BindBufferMemoryInfo {
        BindBufferMemoryInfo {
            s_type: crate::vk1_0::StructureType::BIND_BUFFER_MEMORY_INFO,
            p_next: std::ptr::null(),
            buffer: Default::default(),
            memory: Default::default(),
            memory_offset: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`BindBufferMemoryInfo`](struct.BindBufferMemoryInfo.html)"]
#[repr(transparent)]
pub struct BindBufferMemoryInfoBuilder<'a>(BindBufferMemoryInfo, std::marker::PhantomData<&'a ()>);
impl<'a> BindBufferMemoryInfoBuilder<'a> {
    #[inline]
    pub fn new() -> BindBufferMemoryInfoBuilder<'a> {
        BindBufferMemoryInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn buffer(mut self, buffer: crate::vk1_0::Buffer) -> Self {
        self.0.buffer = buffer;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn memory(mut self, memory: crate::vk1_0::DeviceMemory) -> Self {
        self.0.memory = memory;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn memory_offset(mut self, memory_offset: crate::vk1_0::DeviceSize) -> Self {
        self.0.memory_offset = memory_offset;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> BindBufferMemoryInfo {
        self.0
    }
}
impl<'a> std::fmt::Debug for BindBufferMemoryInfoBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for BindBufferMemoryInfoBuilder<'a> {
    type Target = BindBufferMemoryInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for BindBufferMemoryInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindImageMemoryInfo.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BindImageMemoryInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub image: crate::vk1_0::Image,
    pub memory: crate::vk1_0::DeviceMemory,
    pub memory_offset: crate::vk1_0::DeviceSize,
}
impl BindImageMemoryInfo {
    #[inline]
    pub fn builder<'a>(self) -> BindImageMemoryInfoBuilder<'a> {
        BindImageMemoryInfoBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for BindImageMemoryInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("BindImageMemoryInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("image", &self.image)
            .field("memory", &self.memory)
            .field("memory_offset", &self.memory_offset)
            .finish()
    }
}
impl Default for BindImageMemoryInfo {
    fn default() -> BindImageMemoryInfo {
        BindImageMemoryInfo {
            s_type: crate::vk1_0::StructureType::BIND_IMAGE_MEMORY_INFO,
            p_next: std::ptr::null(),
            image: Default::default(),
            memory: Default::default(),
            memory_offset: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`BindImageMemoryInfo`](struct.BindImageMemoryInfo.html)"]
#[repr(transparent)]
pub struct BindImageMemoryInfoBuilder<'a>(BindImageMemoryInfo, std::marker::PhantomData<&'a ()>);
impl<'a> BindImageMemoryInfoBuilder<'a> {
    #[inline]
    pub fn new() -> BindImageMemoryInfoBuilder<'a> {
        BindImageMemoryInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn image(mut self, image: crate::vk1_0::Image) -> Self {
        self.0.image = image;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn memory(mut self, memory: crate::vk1_0::DeviceMemory) -> Self {
        self.0.memory = memory;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn memory_offset(mut self, memory_offset: crate::vk1_0::DeviceSize) -> Self {
        self.0.memory_offset = memory_offset;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> BindImageMemoryInfo {
        self.0
    }
}
impl<'a> std::fmt::Debug for BindImageMemoryInfoBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for BindImageMemoryInfoBuilder<'a> {
    type Target = BindImageMemoryInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for BindImageMemoryInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPeerMemoryFeatureFlagBits.html) · Flag Bits of [`PeerMemoryFeatureFlags`](struct.PeerMemoryFeatureFlags.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PeerMemoryFeatureFlagBits(pub u32);
impl PeerMemoryFeatureFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> PeerMemoryFeatureFlags {
        PeerMemoryFeatureFlags::from_bits_truncate(self.0)
    }
}
#[doc = "[Part of `vk1_1`](../vk1_1/index.html)"]
impl PeerMemoryFeatureFlagBits {
    pub const COPY_SRC: Self = Self(0x00000001);
    pub const COPY_DST: Self = Self(0x00000002);
    pub const GENERIC_SRC: Self = Self(0x00000004);
    pub const GENERIC_DST: Self = Self(0x00000008);
}
#[doc = "[Part of `extensions::khr_device_group`](../extensions/khr_device_group/index.html)"]
impl PeerMemoryFeatureFlagBits {
    pub const COPY_SRC_KHR: Self = Self::COPY_SRC;
    pub const COPY_DST_KHR: Self = Self::COPY_DST;
    pub const GENERIC_SRC_KHR: Self = Self::GENERIC_SRC;
    pub const GENERIC_DST_KHR: Self = Self::GENERIC_DST;
}
impl std::fmt::Debug for PeerMemoryFeatureFlagBits {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::COPY_SRC => "COPY_SRC",
            &Self::COPY_DST => "COPY_DST",
            &Self::GENERIC_SRC => "GENERIC_SRC",
            &Self::GENERIC_DST => "GENERIC_DST",
            _ => "(Unknown)",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPeerMemoryFeatureFlags.html) · Flags of [`PeerMemoryFeatureFlagBits`](struct.PeerMemoryFeatureFlagBits.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct PeerMemoryFeatureFlags : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; const COPY_SRC = PeerMemoryFeatureFlagBits :: COPY_SRC . 0 ; const COPY_DST = PeerMemoryFeatureFlagBits :: COPY_DST . 0 ; const GENERIC_SRC = PeerMemoryFeatureFlagBits :: GENERIC_SRC . 0 ; const GENERIC_DST = PeerMemoryFeatureFlagBits :: GENERIC_DST . 0 ; const COPY_SRC_KHR = PeerMemoryFeatureFlagBits :: COPY_SRC_KHR . 0 ; const COPY_DST_KHR = PeerMemoryFeatureFlagBits :: COPY_DST_KHR . 0 ; const GENERIC_SRC_KHR = PeerMemoryFeatureFlagBits :: GENERIC_SRC_KHR . 0 ; const GENERIC_DST_KHR = PeerMemoryFeatureFlagBits :: GENERIC_DST_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceGroupProperties.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceGroupProperties {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub physical_device_count: u32,
    pub physical_devices:
        [crate::vk1_0::PhysicalDevice; crate::vk1_1::MAX_DEVICE_GROUP_SIZE as usize],
    pub subset_allocation: crate::vk1_0::Bool32,
}
impl PhysicalDeviceGroupProperties {
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceGroupPropertiesBuilder<'a> {
        PhysicalDeviceGroupPropertiesBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceGroupProperties {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceGroupProperties")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("physical_device_count", &self.physical_device_count)
            .field("physical_devices", &self.physical_devices)
            .field("subset_allocation", &(self.subset_allocation != 0))
            .finish()
    }
}
impl Default for PhysicalDeviceGroupProperties {
    fn default() -> PhysicalDeviceGroupProperties {
        PhysicalDeviceGroupProperties {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_GROUP_PROPERTIES,
            p_next: std::ptr::null_mut(),
            physical_device_count: Default::default(),
            physical_devices: Default::default(),
            subset_allocation: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceGroupProperties`](struct.PhysicalDeviceGroupProperties.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceGroupPropertiesBuilder<'a>(
    PhysicalDeviceGroupProperties,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceGroupPropertiesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceGroupPropertiesBuilder<'a> {
        PhysicalDeviceGroupPropertiesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn physical_device_count(mut self, physical_device_count: u32) -> Self {
        self.0.physical_device_count = physical_device_count;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn physical_devices(
        mut self,
        physical_devices: [crate::vk1_0::PhysicalDevice;
            crate::vk1_1::MAX_DEVICE_GROUP_SIZE as usize],
    ) -> Self {
        self.0.physical_devices = physical_devices;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn subset_allocation(mut self, subset_allocation: bool) -> Self {
        self.0.subset_allocation = subset_allocation as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceGroupProperties {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceGroupPropertiesBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceGroupPropertiesBuilder<'a> {
    type Target = PhysicalDeviceGroupProperties;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceGroupPropertiesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageMemoryRequirementsInfo2.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImageMemoryRequirementsInfo2 {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub image: crate::vk1_0::Image,
}
impl ImageMemoryRequirementsInfo2 {
    #[inline]
    pub fn builder<'a>(self) -> ImageMemoryRequirementsInfo2Builder<'a> {
        ImageMemoryRequirementsInfo2Builder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ImageMemoryRequirementsInfo2 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ImageMemoryRequirementsInfo2")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("image", &self.image)
            .finish()
    }
}
impl Default for ImageMemoryRequirementsInfo2 {
    fn default() -> ImageMemoryRequirementsInfo2 {
        ImageMemoryRequirementsInfo2 {
            s_type: crate::vk1_0::StructureType::IMAGE_MEMORY_REQUIREMENTS_INFO_2,
            p_next: std::ptr::null(),
            image: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`ImageMemoryRequirementsInfo2`](struct.ImageMemoryRequirementsInfo2.html)"]
#[repr(transparent)]
pub struct ImageMemoryRequirementsInfo2Builder<'a>(
    ImageMemoryRequirementsInfo2,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> ImageMemoryRequirementsInfo2Builder<'a> {
    #[inline]
    pub fn new() -> ImageMemoryRequirementsInfo2Builder<'a> {
        ImageMemoryRequirementsInfo2Builder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn image(mut self, image: crate::vk1_0::Image) -> Self {
        self.0.image = image;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> ImageMemoryRequirementsInfo2 {
        self.0
    }
}
impl<'a> std::fmt::Debug for ImageMemoryRequirementsInfo2Builder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for ImageMemoryRequirementsInfo2Builder<'a> {
    type Target = ImageMemoryRequirementsInfo2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ImageMemoryRequirementsInfo2Builder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryRequirements2.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryRequirements2 {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub memory_requirements: crate::vk1_0::MemoryRequirements,
}
impl MemoryRequirements2 {
    #[inline]
    pub fn builder<'a>(self) -> MemoryRequirements2Builder<'a> {
        MemoryRequirements2Builder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for MemoryRequirements2 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("MemoryRequirements2")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("memory_requirements", &self.memory_requirements)
            .finish()
    }
}
impl Default for MemoryRequirements2 {
    fn default() -> MemoryRequirements2 {
        MemoryRequirements2 {
            s_type: crate::vk1_0::StructureType::MEMORY_REQUIREMENTS_2,
            p_next: std::ptr::null_mut(),
            memory_requirements: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`MemoryRequirements2`](struct.MemoryRequirements2.html)"]
#[repr(transparent)]
pub struct MemoryRequirements2Builder<'a>(MemoryRequirements2, std::marker::PhantomData<&'a ()>);
impl<'a> MemoryRequirements2Builder<'a> {
    #[inline]
    pub fn new() -> MemoryRequirements2Builder<'a> {
        MemoryRequirements2Builder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn memory_requirements(
        mut self,
        memory_requirements: crate::vk1_0::MemoryRequirements,
    ) -> Self {
        self.0.memory_requirements = memory_requirements;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> MemoryRequirements2 {
        self.0
    }
}
impl<'a> std::fmt::Debug for MemoryRequirements2Builder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for MemoryRequirements2Builder<'a> {
    type Target = MemoryRequirements2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for MemoryRequirements2Builder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferMemoryRequirementsInfo2.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BufferMemoryRequirementsInfo2 {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub buffer: crate::vk1_0::Buffer,
}
impl BufferMemoryRequirementsInfo2 {
    #[inline]
    pub fn builder<'a>(self) -> BufferMemoryRequirementsInfo2Builder<'a> {
        BufferMemoryRequirementsInfo2Builder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for BufferMemoryRequirementsInfo2 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("BufferMemoryRequirementsInfo2")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("buffer", &self.buffer)
            .finish()
    }
}
impl Default for BufferMemoryRequirementsInfo2 {
    fn default() -> BufferMemoryRequirementsInfo2 {
        BufferMemoryRequirementsInfo2 {
            s_type: crate::vk1_0::StructureType::BUFFER_MEMORY_REQUIREMENTS_INFO_2,
            p_next: std::ptr::null(),
            buffer: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`BufferMemoryRequirementsInfo2`](struct.BufferMemoryRequirementsInfo2.html)"]
#[repr(transparent)]
pub struct BufferMemoryRequirementsInfo2Builder<'a>(
    BufferMemoryRequirementsInfo2,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> BufferMemoryRequirementsInfo2Builder<'a> {
    #[inline]
    pub fn new() -> BufferMemoryRequirementsInfo2Builder<'a> {
        BufferMemoryRequirementsInfo2Builder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn buffer(mut self, buffer: crate::vk1_0::Buffer) -> Self {
        self.0.buffer = buffer;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> BufferMemoryRequirementsInfo2 {
        self.0
    }
}
impl<'a> std::fmt::Debug for BufferMemoryRequirementsInfo2Builder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for BufferMemoryRequirementsInfo2Builder<'a> {
    type Target = BufferMemoryRequirementsInfo2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for BufferMemoryRequirementsInfo2Builder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageSparseMemoryRequirementsInfo2.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImageSparseMemoryRequirementsInfo2 {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub image: crate::vk1_0::Image,
}
impl ImageSparseMemoryRequirementsInfo2 {
    #[inline]
    pub fn builder<'a>(self) -> ImageSparseMemoryRequirementsInfo2Builder<'a> {
        ImageSparseMemoryRequirementsInfo2Builder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ImageSparseMemoryRequirementsInfo2 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ImageSparseMemoryRequirementsInfo2")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("image", &self.image)
            .finish()
    }
}
impl Default for ImageSparseMemoryRequirementsInfo2 {
    fn default() -> ImageSparseMemoryRequirementsInfo2 {
        ImageSparseMemoryRequirementsInfo2 {
            s_type: crate::vk1_0::StructureType::IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2,
            p_next: std::ptr::null(),
            image: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`ImageSparseMemoryRequirementsInfo2`](struct.ImageSparseMemoryRequirementsInfo2.html)"]
#[repr(transparent)]
pub struct ImageSparseMemoryRequirementsInfo2Builder<'a>(
    ImageSparseMemoryRequirementsInfo2,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> ImageSparseMemoryRequirementsInfo2Builder<'a> {
    #[inline]
    pub fn new() -> ImageSparseMemoryRequirementsInfo2Builder<'a> {
        ImageSparseMemoryRequirementsInfo2Builder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn image(mut self, image: crate::vk1_0::Image) -> Self {
        self.0.image = image;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> ImageSparseMemoryRequirementsInfo2 {
        self.0
    }
}
impl<'a> std::fmt::Debug for ImageSparseMemoryRequirementsInfo2Builder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for ImageSparseMemoryRequirementsInfo2Builder<'a> {
    type Target = ImageSparseMemoryRequirementsInfo2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ImageSparseMemoryRequirementsInfo2Builder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSparseImageMemoryRequirements2.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SparseImageMemoryRequirements2 {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub memory_requirements: crate::vk1_0::SparseImageMemoryRequirements,
}
impl SparseImageMemoryRequirements2 {
    #[inline]
    pub fn builder<'a>(self) -> SparseImageMemoryRequirements2Builder<'a> {
        SparseImageMemoryRequirements2Builder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for SparseImageMemoryRequirements2 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("SparseImageMemoryRequirements2")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("memory_requirements", &self.memory_requirements)
            .finish()
    }
}
impl Default for SparseImageMemoryRequirements2 {
    fn default() -> SparseImageMemoryRequirements2 {
        SparseImageMemoryRequirements2 {
            s_type: crate::vk1_0::StructureType::SPARSE_IMAGE_MEMORY_REQUIREMENTS_2,
            p_next: std::ptr::null_mut(),
            memory_requirements: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`SparseImageMemoryRequirements2`](struct.SparseImageMemoryRequirements2.html)"]
#[repr(transparent)]
pub struct SparseImageMemoryRequirements2Builder<'a>(
    SparseImageMemoryRequirements2,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> SparseImageMemoryRequirements2Builder<'a> {
    #[inline]
    pub fn new() -> SparseImageMemoryRequirements2Builder<'a> {
        SparseImageMemoryRequirements2Builder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn memory_requirements(
        mut self,
        memory_requirements: crate::vk1_0::SparseImageMemoryRequirements,
    ) -> Self {
        self.0.memory_requirements = memory_requirements;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> SparseImageMemoryRequirements2 {
        self.0
    }
}
impl<'a> std::fmt::Debug for SparseImageMemoryRequirements2Builder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for SparseImageMemoryRequirements2Builder<'a> {
    type Target = SparseImageMemoryRequirements2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SparseImageMemoryRequirements2Builder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceFeatures2.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceFeatures2 {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub features: crate::vk1_0::PhysicalDeviceFeatures,
}
impl PhysicalDeviceFeatures2 {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceFeatures2,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceFeatures2Builder<'a> {
        PhysicalDeviceFeatures2Builder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceFeatures2 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceFeatures2")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("features", &self.features)
            .finish()
    }
}
impl Default for PhysicalDeviceFeatures2 {
    fn default() -> PhysicalDeviceFeatures2 {
        PhysicalDeviceFeatures2 {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_FEATURES_2,
            p_next: std::ptr::null_mut(),
            features: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceFeatures2::extend`](struct.PhysicalDeviceFeatures2.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceFeatures2 {}
impl ExtendableByPhysicalDeviceFeatures2 for crate::vk1_0::DeviceCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceFeatures2`](struct.PhysicalDeviceFeatures2.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceFeatures2Builder<'a>(
    PhysicalDeviceFeatures2,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceFeatures2Builder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceFeatures2Builder<'a> {
        PhysicalDeviceFeatures2Builder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn features(mut self, features: crate::vk1_0::PhysicalDeviceFeatures) -> Self {
        self.0.features = features;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceFeatures2 {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceFeatures2Builder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceFeatures2Builder<'a> {
    type Target = PhysicalDeviceFeatures2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceFeatures2Builder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceProperties2.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceProperties2 {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub properties: crate::vk1_0::PhysicalDeviceProperties,
}
impl PhysicalDeviceProperties2 {
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceProperties2Builder<'a> {
        PhysicalDeviceProperties2Builder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceProperties2 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceProperties2")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("properties", &self.properties)
            .finish()
    }
}
impl Default for PhysicalDeviceProperties2 {
    fn default() -> PhysicalDeviceProperties2 {
        PhysicalDeviceProperties2 {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_PROPERTIES_2,
            p_next: std::ptr::null_mut(),
            properties: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceProperties2`](struct.PhysicalDeviceProperties2.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceProperties2Builder<'a>(
    PhysicalDeviceProperties2,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceProperties2Builder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceProperties2Builder<'a> {
        PhysicalDeviceProperties2Builder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn properties(mut self, properties: crate::vk1_0::PhysicalDeviceProperties) -> Self {
        self.0.properties = properties;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceProperties2 {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceProperties2Builder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceProperties2Builder<'a> {
    type Target = PhysicalDeviceProperties2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceProperties2Builder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFormatProperties2.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FormatProperties2 {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub format_properties: crate::vk1_0::FormatProperties,
}
impl FormatProperties2 {
    #[inline]
    pub fn builder<'a>(self) -> FormatProperties2Builder<'a> {
        FormatProperties2Builder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for FormatProperties2 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("FormatProperties2")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("format_properties", &self.format_properties)
            .finish()
    }
}
impl Default for FormatProperties2 {
    fn default() -> FormatProperties2 {
        FormatProperties2 {
            s_type: crate::vk1_0::StructureType::FORMAT_PROPERTIES_2,
            p_next: std::ptr::null_mut(),
            format_properties: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`FormatProperties2`](struct.FormatProperties2.html)"]
#[repr(transparent)]
pub struct FormatProperties2Builder<'a>(FormatProperties2, std::marker::PhantomData<&'a ()>);
impl<'a> FormatProperties2Builder<'a> {
    #[inline]
    pub fn new() -> FormatProperties2Builder<'a> {
        FormatProperties2Builder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn format_properties(mut self, format_properties: crate::vk1_0::FormatProperties) -> Self {
        self.0.format_properties = format_properties;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> FormatProperties2 {
        self.0
    }
}
impl<'a> std::fmt::Debug for FormatProperties2Builder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for FormatProperties2Builder<'a> {
    type Target = FormatProperties2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for FormatProperties2Builder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceImageFormatInfo2.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceImageFormatInfo2 {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub format: crate::vk1_0::Format,
    pub _type: crate::vk1_0::ImageType,
    pub tiling: crate::vk1_0::ImageTiling,
    pub usage: crate::vk1_0::ImageUsageFlags,
    pub flags: crate::vk1_0::ImageCreateFlags,
}
impl PhysicalDeviceImageFormatInfo2 {
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceImageFormatInfo2Builder<'a> {
        PhysicalDeviceImageFormatInfo2Builder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceImageFormatInfo2 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceImageFormatInfo2")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("format", &self.format)
            .field("_type", &self._type)
            .field("tiling", &self.tiling)
            .field("usage", &self.usage)
            .field("flags", &self.flags)
            .finish()
    }
}
impl Default for PhysicalDeviceImageFormatInfo2 {
    fn default() -> PhysicalDeviceImageFormatInfo2 {
        PhysicalDeviceImageFormatInfo2 {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2,
            p_next: std::ptr::null(),
            format: Default::default(),
            _type: Default::default(),
            tiling: Default::default(),
            usage: Default::default(),
            flags: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceImageFormatInfo2`](struct.PhysicalDeviceImageFormatInfo2.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceImageFormatInfo2Builder<'a>(
    PhysicalDeviceImageFormatInfo2,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceImageFormatInfo2Builder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceImageFormatInfo2Builder<'a> {
        PhysicalDeviceImageFormatInfo2Builder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn format(mut self, format: crate::vk1_0::Format) -> Self {
        self.0.format = format;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn _type(mut self, _type: crate::vk1_0::ImageType) -> Self {
        self.0._type = _type;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn tiling(mut self, tiling: crate::vk1_0::ImageTiling) -> Self {
        self.0.tiling = tiling;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn usage(mut self, usage: crate::vk1_0::ImageUsageFlags) -> Self {
        self.0.usage = usage;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_0::ImageCreateFlags) -> Self {
        self.0.flags = flags;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceImageFormatInfo2 {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceImageFormatInfo2Builder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceImageFormatInfo2Builder<'a> {
    type Target = PhysicalDeviceImageFormatInfo2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceImageFormatInfo2Builder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageFormatProperties2.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImageFormatProperties2 {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub image_format_properties: crate::vk1_0::ImageFormatProperties,
}
impl ImageFormatProperties2 {
    #[inline]
    pub fn builder<'a>(self) -> ImageFormatProperties2Builder<'a> {
        ImageFormatProperties2Builder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ImageFormatProperties2 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ImageFormatProperties2")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("image_format_properties", &self.image_format_properties)
            .finish()
    }
}
impl Default for ImageFormatProperties2 {
    fn default() -> ImageFormatProperties2 {
        ImageFormatProperties2 {
            s_type: crate::vk1_0::StructureType::IMAGE_FORMAT_PROPERTIES_2,
            p_next: std::ptr::null_mut(),
            image_format_properties: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`ImageFormatProperties2`](struct.ImageFormatProperties2.html)"]
#[repr(transparent)]
pub struct ImageFormatProperties2Builder<'a>(
    ImageFormatProperties2,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> ImageFormatProperties2Builder<'a> {
    #[inline]
    pub fn new() -> ImageFormatProperties2Builder<'a> {
        ImageFormatProperties2Builder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn image_format_properties(
        mut self,
        image_format_properties: crate::vk1_0::ImageFormatProperties,
    ) -> Self {
        self.0.image_format_properties = image_format_properties;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> ImageFormatProperties2 {
        self.0
    }
}
impl<'a> std::fmt::Debug for ImageFormatProperties2Builder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for ImageFormatProperties2Builder<'a> {
    type Target = ImageFormatProperties2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ImageFormatProperties2Builder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueueFamilyProperties2.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct QueueFamilyProperties2 {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub queue_family_properties: crate::vk1_0::QueueFamilyProperties,
}
impl QueueFamilyProperties2 {
    #[inline]
    pub fn builder<'a>(self) -> QueueFamilyProperties2Builder<'a> {
        QueueFamilyProperties2Builder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for QueueFamilyProperties2 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("QueueFamilyProperties2")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("queue_family_properties", &self.queue_family_properties)
            .finish()
    }
}
impl Default for QueueFamilyProperties2 {
    fn default() -> QueueFamilyProperties2 {
        QueueFamilyProperties2 {
            s_type: crate::vk1_0::StructureType::QUEUE_FAMILY_PROPERTIES_2,
            p_next: std::ptr::null_mut(),
            queue_family_properties: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`QueueFamilyProperties2`](struct.QueueFamilyProperties2.html)"]
#[repr(transparent)]
pub struct QueueFamilyProperties2Builder<'a>(
    QueueFamilyProperties2,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> QueueFamilyProperties2Builder<'a> {
    #[inline]
    pub fn new() -> QueueFamilyProperties2Builder<'a> {
        QueueFamilyProperties2Builder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn queue_family_properties(
        mut self,
        queue_family_properties: crate::vk1_0::QueueFamilyProperties,
    ) -> Self {
        self.0.queue_family_properties = queue_family_properties;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> QueueFamilyProperties2 {
        self.0
    }
}
impl<'a> std::fmt::Debug for QueueFamilyProperties2Builder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for QueueFamilyProperties2Builder<'a> {
    type Target = QueueFamilyProperties2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for QueueFamilyProperties2Builder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMemoryProperties2.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceMemoryProperties2 {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub memory_properties: crate::vk1_0::PhysicalDeviceMemoryProperties,
}
impl PhysicalDeviceMemoryProperties2 {
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceMemoryProperties2Builder<'a> {
        PhysicalDeviceMemoryProperties2Builder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceMemoryProperties2 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceMemoryProperties2")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("memory_properties", &self.memory_properties)
            .finish()
    }
}
impl Default for PhysicalDeviceMemoryProperties2 {
    fn default() -> PhysicalDeviceMemoryProperties2 {
        PhysicalDeviceMemoryProperties2 {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_MEMORY_PROPERTIES_2,
            p_next: std::ptr::null_mut(),
            memory_properties: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceMemoryProperties2`](struct.PhysicalDeviceMemoryProperties2.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceMemoryProperties2Builder<'a>(
    PhysicalDeviceMemoryProperties2,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceMemoryProperties2Builder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceMemoryProperties2Builder<'a> {
        PhysicalDeviceMemoryProperties2Builder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn memory_properties(
        mut self,
        memory_properties: crate::vk1_0::PhysicalDeviceMemoryProperties,
    ) -> Self {
        self.0.memory_properties = memory_properties;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceMemoryProperties2 {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceMemoryProperties2Builder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceMemoryProperties2Builder<'a> {
    type Target = PhysicalDeviceMemoryProperties2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceMemoryProperties2Builder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSparseImageFormatInfo2.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceSparseImageFormatInfo2 {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub format: crate::vk1_0::Format,
    pub _type: crate::vk1_0::ImageType,
    pub samples: crate::vk1_0::SampleCountFlagBits,
    pub usage: crate::vk1_0::ImageUsageFlags,
    pub tiling: crate::vk1_0::ImageTiling,
}
impl PhysicalDeviceSparseImageFormatInfo2 {
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceSparseImageFormatInfo2Builder<'a> {
        PhysicalDeviceSparseImageFormatInfo2Builder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceSparseImageFormatInfo2 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceSparseImageFormatInfo2")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("format", &self.format)
            .field("_type", &self._type)
            .field("samples", &self.samples)
            .field("usage", &self.usage)
            .field("tiling", &self.tiling)
            .finish()
    }
}
impl Default for PhysicalDeviceSparseImageFormatInfo2 {
    fn default() -> PhysicalDeviceSparseImageFormatInfo2 {
        PhysicalDeviceSparseImageFormatInfo2 {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2,
            p_next: std::ptr::null(),
            format: Default::default(),
            _type: Default::default(),
            samples: Default::default(),
            usage: Default::default(),
            tiling: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceSparseImageFormatInfo2`](struct.PhysicalDeviceSparseImageFormatInfo2.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceSparseImageFormatInfo2Builder<'a>(
    PhysicalDeviceSparseImageFormatInfo2,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceSparseImageFormatInfo2Builder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceSparseImageFormatInfo2Builder<'a> {
        PhysicalDeviceSparseImageFormatInfo2Builder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn format(mut self, format: crate::vk1_0::Format) -> Self {
        self.0.format = format;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn _type(mut self, _type: crate::vk1_0::ImageType) -> Self {
        self.0._type = _type;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn samples(mut self, samples: crate::vk1_0::SampleCountFlagBits) -> Self {
        self.0.samples = samples;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn usage(mut self, usage: crate::vk1_0::ImageUsageFlags) -> Self {
        self.0.usage = usage;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn tiling(mut self, tiling: crate::vk1_0::ImageTiling) -> Self {
        self.0.tiling = tiling;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceSparseImageFormatInfo2 {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceSparseImageFormatInfo2Builder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceSparseImageFormatInfo2Builder<'a> {
    type Target = PhysicalDeviceSparseImageFormatInfo2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceSparseImageFormatInfo2Builder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSparseImageFormatProperties2.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SparseImageFormatProperties2 {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub properties: crate::vk1_0::SparseImageFormatProperties,
}
impl SparseImageFormatProperties2 {
    #[inline]
    pub fn builder<'a>(self) -> SparseImageFormatProperties2Builder<'a> {
        SparseImageFormatProperties2Builder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for SparseImageFormatProperties2 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("SparseImageFormatProperties2")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("properties", &self.properties)
            .finish()
    }
}
impl Default for SparseImageFormatProperties2 {
    fn default() -> SparseImageFormatProperties2 {
        SparseImageFormatProperties2 {
            s_type: crate::vk1_0::StructureType::SPARSE_IMAGE_FORMAT_PROPERTIES_2,
            p_next: std::ptr::null_mut(),
            properties: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`SparseImageFormatProperties2`](struct.SparseImageFormatProperties2.html)"]
#[repr(transparent)]
pub struct SparseImageFormatProperties2Builder<'a>(
    SparseImageFormatProperties2,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> SparseImageFormatProperties2Builder<'a> {
    #[inline]
    pub fn new() -> SparseImageFormatProperties2Builder<'a> {
        SparseImageFormatProperties2Builder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn properties(mut self, properties: crate::vk1_0::SparseImageFormatProperties) -> Self {
        self.0.properties = properties;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> SparseImageFormatProperties2 {
        self.0
    }
}
impl<'a> std::fmt::Debug for SparseImageFormatProperties2Builder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for SparseImageFormatProperties2Builder<'a> {
    type Target = SparseImageFormatProperties2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SparseImageFormatProperties2Builder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Flag Bits of [`CommandPoolTrimFlags`](struct.CommandPoolTrimFlags.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct CommandPoolTrimFlagBits(pub u32);
impl CommandPoolTrimFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> CommandPoolTrimFlags {
        CommandPoolTrimFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for CommandPoolTrimFlagBits {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            _ => "(Unknown)",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandPoolTrimFlags.html) · Flags of [`CommandPoolTrimFlagBits`](struct.CommandPoolTrimFlagBits.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct CommandPoolTrimFlags : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceQueueInfo2.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DeviceQueueInfo2 {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::vk1_0::DeviceQueueCreateFlags,
    pub queue_family_index: u32,
    pub queue_index: u32,
}
impl DeviceQueueInfo2 {
    #[inline]
    pub fn builder<'a>(self) -> DeviceQueueInfo2Builder<'a> {
        DeviceQueueInfo2Builder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for DeviceQueueInfo2 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("DeviceQueueInfo2")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("queue_family_index", &self.queue_family_index)
            .field("queue_index", &self.queue_index)
            .finish()
    }
}
impl Default for DeviceQueueInfo2 {
    fn default() -> DeviceQueueInfo2 {
        DeviceQueueInfo2 {
            s_type: crate::vk1_0::StructureType::DEVICE_QUEUE_INFO_2,
            p_next: std::ptr::null(),
            flags: Default::default(),
            queue_family_index: Default::default(),
            queue_index: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`DeviceQueueInfo2`](struct.DeviceQueueInfo2.html)"]
#[repr(transparent)]
pub struct DeviceQueueInfo2Builder<'a>(DeviceQueueInfo2, std::marker::PhantomData<&'a ()>);
impl<'a> DeviceQueueInfo2Builder<'a> {
    #[inline]
    pub fn new() -> DeviceQueueInfo2Builder<'a> {
        DeviceQueueInfo2Builder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_0::DeviceQueueCreateFlags) -> Self {
        self.0.flags = flags;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn queue_family_index(mut self, queue_family_index: u32) -> Self {
        self.0.queue_family_index = queue_family_index;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn queue_index(mut self, queue_index: u32) -> Self {
        self.0.queue_index = queue_index;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> DeviceQueueInfo2 {
        self.0
    }
}
impl<'a> std::fmt::Debug for DeviceQueueInfo2Builder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for DeviceQueueInfo2Builder<'a> {
    type Target = DeviceQueueInfo2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DeviceQueueInfo2Builder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerYcbcrConversionCreateInfo.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SamplerYcbcrConversionCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub format: crate::vk1_0::Format,
    pub ycbcr_model: crate::vk1_1::SamplerYcbcrModelConversion,
    pub ycbcr_range: crate::vk1_1::SamplerYcbcrRange,
    pub components: crate::vk1_0::ComponentMapping,
    pub x_chroma_offset: crate::vk1_1::ChromaLocation,
    pub y_chroma_offset: crate::vk1_1::ChromaLocation,
    pub chroma_filter: crate::vk1_0::Filter,
    pub force_explicit_reconstruction: crate::vk1_0::Bool32,
}
impl SamplerYcbcrConversionCreateInfo {
    #[inline]
    pub fn builder<'a>(self) -> SamplerYcbcrConversionCreateInfoBuilder<'a> {
        SamplerYcbcrConversionCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for SamplerYcbcrConversionCreateInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("SamplerYcbcrConversionCreateInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("format", &self.format)
            .field("ycbcr_model", &self.ycbcr_model)
            .field("ycbcr_range", &self.ycbcr_range)
            .field("components", &self.components)
            .field("x_chroma_offset", &self.x_chroma_offset)
            .field("y_chroma_offset", &self.y_chroma_offset)
            .field("chroma_filter", &self.chroma_filter)
            .field(
                "force_explicit_reconstruction",
                &(self.force_explicit_reconstruction != 0),
            )
            .finish()
    }
}
impl Default for SamplerYcbcrConversionCreateInfo {
    fn default() -> SamplerYcbcrConversionCreateInfo {
        SamplerYcbcrConversionCreateInfo {
            s_type: crate::vk1_0::StructureType::SAMPLER_YCBCR_CONVERSION_CREATE_INFO,
            p_next: std::ptr::null(),
            format: Default::default(),
            ycbcr_model: Default::default(),
            ycbcr_range: Default::default(),
            components: Default::default(),
            x_chroma_offset: Default::default(),
            y_chroma_offset: Default::default(),
            chroma_filter: Default::default(),
            force_explicit_reconstruction: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`SamplerYcbcrConversionCreateInfo`](struct.SamplerYcbcrConversionCreateInfo.html)"]
#[repr(transparent)]
pub struct SamplerYcbcrConversionCreateInfoBuilder<'a>(
    SamplerYcbcrConversionCreateInfo,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> SamplerYcbcrConversionCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> SamplerYcbcrConversionCreateInfoBuilder<'a> {
        SamplerYcbcrConversionCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn format(mut self, format: crate::vk1_0::Format) -> Self {
        self.0.format = format;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn ycbcr_model(mut self, ycbcr_model: crate::vk1_1::SamplerYcbcrModelConversion) -> Self {
        self.0.ycbcr_model = ycbcr_model;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn ycbcr_range(mut self, ycbcr_range: crate::vk1_1::SamplerYcbcrRange) -> Self {
        self.0.ycbcr_range = ycbcr_range;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn components(mut self, components: crate::vk1_0::ComponentMapping) -> Self {
        self.0.components = components;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn x_chroma_offset(mut self, x_chroma_offset: crate::vk1_1::ChromaLocation) -> Self {
        self.0.x_chroma_offset = x_chroma_offset;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn y_chroma_offset(mut self, y_chroma_offset: crate::vk1_1::ChromaLocation) -> Self {
        self.0.y_chroma_offset = y_chroma_offset;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn chroma_filter(mut self, chroma_filter: crate::vk1_0::Filter) -> Self {
        self.0.chroma_filter = chroma_filter;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn force_explicit_reconstruction(mut self, force_explicit_reconstruction: bool) -> Self {
        self.0.force_explicit_reconstruction = force_explicit_reconstruction as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> SamplerYcbcrConversionCreateInfo {
        self.0
    }
}
impl<'a> std::fmt::Debug for SamplerYcbcrConversionCreateInfoBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for SamplerYcbcrConversionCreateInfoBuilder<'a> {
    type Target = SamplerYcbcrConversionCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SamplerYcbcrConversionCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerYcbcrModelConversion.html) · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct SamplerYcbcrModelConversion(pub i32);
#[doc = "[Part of `vk1_1`](../vk1_1/index.html)"]
impl SamplerYcbcrModelConversion {
    pub const RGB_IDENTITY: Self = Self(0);
    pub const YCBCR_IDENTITY: Self = Self(1);
    pub const YCBCR_709: Self = Self(2);
    pub const YCBCR_601: Self = Self(3);
    pub const YCBCR_2020: Self = Self(4);
}
#[doc = "[Part of `extensions::khr_sampler_ycbcr_conversion`](../extensions/khr_sampler_ycbcr_conversion/index.html)"]
impl SamplerYcbcrModelConversion {
    pub const RGB_IDENTITY_KHR: Self = Self::RGB_IDENTITY;
    pub const YCBCR_IDENTITY_KHR: Self = Self::YCBCR_IDENTITY;
    pub const YCBCR_709_KHR: Self = Self::YCBCR_709;
    pub const YCBCR_601_KHR: Self = Self::YCBCR_601;
    pub const YCBCR_2020_KHR: Self = Self::YCBCR_2020;
}
impl std::fmt::Debug for SamplerYcbcrModelConversion {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::RGB_IDENTITY => "RGB_IDENTITY",
            &Self::YCBCR_IDENTITY => "YCBCR_IDENTITY",
            &Self::YCBCR_709 => "YCBCR_709",
            &Self::YCBCR_601 => "YCBCR_601",
            &Self::YCBCR_2020 => "YCBCR_2020",
            _ => "(Unknown)",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerYcbcrRange.html) · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct SamplerYcbcrRange(pub i32);
#[doc = "[Part of `vk1_1`](../vk1_1/index.html)"]
impl SamplerYcbcrRange {
    pub const ITU_FULL: Self = Self(0);
    pub const ITU_NARROW: Self = Self(1);
}
#[doc = "[Part of `extensions::khr_sampler_ycbcr_conversion`](../extensions/khr_sampler_ycbcr_conversion/index.html)"]
impl SamplerYcbcrRange {
    pub const ITU_FULL_KHR: Self = Self::ITU_FULL;
    pub const ITU_NARROW_KHR: Self = Self::ITU_NARROW;
}
impl std::fmt::Debug for SamplerYcbcrRange {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::ITU_FULL => "ITU_FULL",
            &Self::ITU_NARROW => "ITU_NARROW",
            _ => "(Unknown)",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkChromaLocation.html) · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ChromaLocation(pub i32);
#[doc = "[Part of `vk1_1`](../vk1_1/index.html)"]
impl ChromaLocation {
    pub const COSITED_EVEN: Self = Self(0);
    pub const MIDPOINT: Self = Self(1);
}
#[doc = "[Part of `extensions::khr_sampler_ycbcr_conversion`](../extensions/khr_sampler_ycbcr_conversion/index.html)"]
impl ChromaLocation {
    pub const COSITED_EVEN_KHR: Self = Self::COSITED_EVEN;
    pub const MIDPOINT_KHR: Self = Self::MIDPOINT;
}
impl std::fmt::Debug for ChromaLocation {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::COSITED_EVEN => "COSITED_EVEN",
            &Self::MIDPOINT => "MIDPOINT",
            _ => "(Unknown)",
        })
    }
}
crate :: non_dispatchable_handle ! ( SamplerYcbcrConversion , SAMPLER_YCBCR_CONVERSION , doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerYcbcrConversion.html) · Non-dispatchable Handle" ) ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorUpdateTemplateCreateInfo.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DescriptorUpdateTemplateCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::vk1_1::DescriptorUpdateTemplateCreateFlags,
    pub descriptor_update_entry_count: u32,
    pub p_descriptor_update_entries: *const crate::vk1_1::DescriptorUpdateTemplateEntry,
    pub template_type: crate::vk1_1::DescriptorUpdateTemplateType,
    pub descriptor_set_layout: crate::vk1_0::DescriptorSetLayout,
    pub pipeline_bind_point: crate::vk1_0::PipelineBindPoint,
    pub pipeline_layout: crate::vk1_0::PipelineLayout,
    pub set: u32,
}
impl DescriptorUpdateTemplateCreateInfo {
    #[inline]
    pub fn builder<'a>(self) -> DescriptorUpdateTemplateCreateInfoBuilder<'a> {
        DescriptorUpdateTemplateCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for DescriptorUpdateTemplateCreateInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("DescriptorUpdateTemplateCreateInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field(
                "descriptor_update_entry_count",
                &self.descriptor_update_entry_count,
            )
            .field(
                "p_descriptor_update_entries",
                &self.p_descriptor_update_entries,
            )
            .field("template_type", &self.template_type)
            .field("descriptor_set_layout", &self.descriptor_set_layout)
            .field("pipeline_bind_point", &self.pipeline_bind_point)
            .field("pipeline_layout", &self.pipeline_layout)
            .field("set", &self.set)
            .finish()
    }
}
impl Default for DescriptorUpdateTemplateCreateInfo {
    fn default() -> DescriptorUpdateTemplateCreateInfo {
        DescriptorUpdateTemplateCreateInfo {
            s_type: crate::vk1_0::StructureType::DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            descriptor_update_entry_count: Default::default(),
            p_descriptor_update_entries: std::ptr::null(),
            template_type: Default::default(),
            descriptor_set_layout: Default::default(),
            pipeline_bind_point: Default::default(),
            pipeline_layout: Default::default(),
            set: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`DescriptorUpdateTemplateCreateInfo`](struct.DescriptorUpdateTemplateCreateInfo.html)"]
#[repr(transparent)]
pub struct DescriptorUpdateTemplateCreateInfoBuilder<'a>(
    DescriptorUpdateTemplateCreateInfo,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> DescriptorUpdateTemplateCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> DescriptorUpdateTemplateCreateInfoBuilder<'a> {
        DescriptorUpdateTemplateCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_1::DescriptorUpdateTemplateCreateFlags) -> Self {
        self.0.flags = flags;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn descriptor_update_entries(
        mut self,
        descriptor_update_entries: &'a [crate::vk1_1::DescriptorUpdateTemplateEntryBuilder],
    ) -> Self {
        self.0.descriptor_update_entry_count = descriptor_update_entries.len() as _;
        self.0.p_descriptor_update_entries = descriptor_update_entries.as_ptr() as _;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn template_type(
        mut self,
        template_type: crate::vk1_1::DescriptorUpdateTemplateType,
    ) -> Self {
        self.0.template_type = template_type;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn descriptor_set_layout(
        mut self,
        descriptor_set_layout: crate::vk1_0::DescriptorSetLayout,
    ) -> Self {
        self.0.descriptor_set_layout = descriptor_set_layout;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn pipeline_bind_point(
        mut self,
        pipeline_bind_point: crate::vk1_0::PipelineBindPoint,
    ) -> Self {
        self.0.pipeline_bind_point = pipeline_bind_point;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn pipeline_layout(mut self, pipeline_layout: crate::vk1_0::PipelineLayout) -> Self {
        self.0.pipeline_layout = pipeline_layout;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn set(mut self, set: u32) -> Self {
        self.0.set = set;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> DescriptorUpdateTemplateCreateInfo {
        self.0
    }
}
impl<'a> std::fmt::Debug for DescriptorUpdateTemplateCreateInfoBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for DescriptorUpdateTemplateCreateInfoBuilder<'a> {
    type Target = DescriptorUpdateTemplateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DescriptorUpdateTemplateCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Flag Bits of [`DescriptorUpdateTemplateCreateFlags`](struct.DescriptorUpdateTemplateCreateFlags.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct DescriptorUpdateTemplateCreateFlagBits(pub u32);
impl DescriptorUpdateTemplateCreateFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> DescriptorUpdateTemplateCreateFlags {
        DescriptorUpdateTemplateCreateFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for DescriptorUpdateTemplateCreateFlagBits {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            _ => "(Unknown)",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorUpdateTemplateCreateFlags.html) · Flags of [`DescriptorUpdateTemplateCreateFlagBits`](struct.DescriptorUpdateTemplateCreateFlagBits.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct DescriptorUpdateTemplateCreateFlags : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorUpdateTemplateEntry.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DescriptorUpdateTemplateEntry {
    pub dst_binding: u32,
    pub dst_array_element: u32,
    pub descriptor_count: u32,
    pub descriptor_type: crate::vk1_0::DescriptorType,
    pub offset: usize,
    pub stride: usize,
}
impl DescriptorUpdateTemplateEntry {
    #[inline]
    pub fn builder<'a>(self) -> DescriptorUpdateTemplateEntryBuilder<'a> {
        DescriptorUpdateTemplateEntryBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for DescriptorUpdateTemplateEntry {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("DescriptorUpdateTemplateEntry")
            .field("dst_binding", &self.dst_binding)
            .field("dst_array_element", &self.dst_array_element)
            .field("descriptor_count", &self.descriptor_count)
            .field("descriptor_type", &self.descriptor_type)
            .field("offset", &self.offset)
            .field("stride", &self.stride)
            .finish()
    }
}
impl Default for DescriptorUpdateTemplateEntry {
    fn default() -> DescriptorUpdateTemplateEntry {
        DescriptorUpdateTemplateEntry {
            dst_binding: Default::default(),
            dst_array_element: Default::default(),
            descriptor_count: Default::default(),
            descriptor_type: Default::default(),
            offset: Default::default(),
            stride: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`DescriptorUpdateTemplateEntry`](struct.DescriptorUpdateTemplateEntry.html)"]
#[repr(transparent)]
pub struct DescriptorUpdateTemplateEntryBuilder<'a>(
    DescriptorUpdateTemplateEntry,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> DescriptorUpdateTemplateEntryBuilder<'a> {
    #[inline]
    pub fn new() -> DescriptorUpdateTemplateEntryBuilder<'a> {
        DescriptorUpdateTemplateEntryBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn dst_binding(mut self, dst_binding: u32) -> Self {
        self.0.dst_binding = dst_binding;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn dst_array_element(mut self, dst_array_element: u32) -> Self {
        self.0.dst_array_element = dst_array_element;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn descriptor_count(mut self, descriptor_count: u32) -> Self {
        self.0.descriptor_count = descriptor_count;
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
    pub fn offset(mut self, offset: usize) -> Self {
        self.0.offset = offset;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn stride(mut self, stride: usize) -> Self {
        self.0.stride = stride;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> DescriptorUpdateTemplateEntry {
        self.0
    }
}
impl<'a> std::fmt::Debug for DescriptorUpdateTemplateEntryBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for DescriptorUpdateTemplateEntryBuilder<'a> {
    type Target = DescriptorUpdateTemplateEntry;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DescriptorUpdateTemplateEntryBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorUpdateTemplateType.html) · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct DescriptorUpdateTemplateType(pub i32);
#[doc = "[Part of `vk1_1`](../vk1_1/index.html)"]
impl DescriptorUpdateTemplateType {
    pub const DESCRIPTOR_SET: Self = Self(0);
}
#[doc = "[Part of `extensions::khr_descriptor_update_template`](../extensions/khr_descriptor_update_template/index.html)"]
impl DescriptorUpdateTemplateType {
    pub const DESCRIPTOR_SET_KHR: Self = Self::DESCRIPTOR_SET;
    pub const PUSH_DESCRIPTORS_KHR: Self = Self(1);
}
#[doc = "[Part of `extensions::khr_push_descriptor`](../extensions/khr_push_descriptor/index.html)"]
impl DescriptorUpdateTemplateType {}
impl std::fmt::Debug for DescriptorUpdateTemplateType {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::DESCRIPTOR_SET => "DESCRIPTOR_SET",
            &Self::PUSH_DESCRIPTORS_KHR => "PUSH_DESCRIPTORS_KHR",
            _ => "(Unknown)",
        })
    }
}
crate :: non_dispatchable_handle ! ( DescriptorUpdateTemplate , DESCRIPTOR_UPDATE_TEMPLATE , doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorUpdateTemplate.html) · Non-dispatchable Handle" ) ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceExternalBufferInfo.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceExternalBufferInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::vk1_0::BufferCreateFlags,
    pub usage: crate::vk1_0::BufferUsageFlags,
    pub handle_type: crate::vk1_1::ExternalMemoryHandleTypeFlagBits,
}
impl PhysicalDeviceExternalBufferInfo {
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceExternalBufferInfoBuilder<'a> {
        PhysicalDeviceExternalBufferInfoBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceExternalBufferInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceExternalBufferInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("usage", &self.usage)
            .field("handle_type", &self.handle_type)
            .finish()
    }
}
impl Default for PhysicalDeviceExternalBufferInfo {
    fn default() -> PhysicalDeviceExternalBufferInfo {
        PhysicalDeviceExternalBufferInfo {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            usage: Default::default(),
            handle_type: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceExternalBufferInfo`](struct.PhysicalDeviceExternalBufferInfo.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceExternalBufferInfoBuilder<'a>(
    PhysicalDeviceExternalBufferInfo,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceExternalBufferInfoBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceExternalBufferInfoBuilder<'a> {
        PhysicalDeviceExternalBufferInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_0::BufferCreateFlags) -> Self {
        self.0.flags = flags;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn usage(mut self, usage: crate::vk1_0::BufferUsageFlags) -> Self {
        self.0.usage = usage;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn handle_type(
        mut self,
        handle_type: crate::vk1_1::ExternalMemoryHandleTypeFlagBits,
    ) -> Self {
        self.0.handle_type = handle_type;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceExternalBufferInfo {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceExternalBufferInfoBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceExternalBufferInfoBuilder<'a> {
    type Target = PhysicalDeviceExternalBufferInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceExternalBufferInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalMemoryHandleTypeFlagBits.html) · Flag Bits of [`ExternalMemoryHandleTypeFlags`](struct.ExternalMemoryHandleTypeFlags.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ExternalMemoryHandleTypeFlagBits(pub u32);
impl ExternalMemoryHandleTypeFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> ExternalMemoryHandleTypeFlags {
        ExternalMemoryHandleTypeFlags::from_bits_truncate(self.0)
    }
}
#[doc = "[Part of `vk1_1`](../vk1_1/index.html)"]
impl ExternalMemoryHandleTypeFlagBits {
    pub const OPAQUE_FD: Self = Self(0x00000001);
    pub const OPAQUE_WIN32: Self = Self(0x00000002);
    pub const OPAQUE_WIN32_KMT: Self = Self(0x00000004);
    pub const D3D11_TEXTURE: Self = Self(0x00000008);
    pub const D3D11_TEXTURE_KMT: Self = Self(0x00000010);
    pub const D3D12_HEAP: Self = Self(0x00000020);
    pub const D3D12_RESOURCE: Self = Self(0x00000040);
}
#[doc = "[Part of `extensions::android_external_memory_android_hardware_buffer`](../extensions/android_external_memory_android_hardware_buffer/index.html)"]
impl ExternalMemoryHandleTypeFlagBits {
    pub const ANDROID_HARDWARE_BUFFER_ANDROID: Self = Self(0x00000400);
}
#[doc = "[Part of `extensions::ext_external_memory_dma_buf`](../extensions/ext_external_memory_dma_buf/index.html)"]
impl ExternalMemoryHandleTypeFlagBits {
    pub const DMA_BUF_EXT: Self = Self(0x00000200);
}
#[doc = "[Part of `extensions::ext_external_memory_host`](../extensions/ext_external_memory_host/index.html)"]
impl ExternalMemoryHandleTypeFlagBits {
    pub const HOST_ALLOCATION_EXT: Self = Self(0x00000080);
    pub const HOST_MAPPED_FOREIGN_MEMORY_EXT: Self = Self(0x00000100);
}
#[doc = "[Part of `extensions::khr_external_memory_capabilities`](../extensions/khr_external_memory_capabilities/index.html)"]
impl ExternalMemoryHandleTypeFlagBits {
    pub const OPAQUE_FD_KHR: Self = Self::OPAQUE_FD;
    pub const OPAQUE_WIN32_KHR: Self = Self::OPAQUE_WIN32;
    pub const OPAQUE_WIN32_KMT_KHR: Self = Self::OPAQUE_WIN32_KMT;
    pub const D3D11_TEXTURE_KHR: Self = Self::D3D11_TEXTURE;
    pub const D3D11_TEXTURE_KMT_KHR: Self = Self::D3D11_TEXTURE_KMT;
    pub const D3D12_HEAP_KHR: Self = Self::D3D12_HEAP;
    pub const D3D12_RESOURCE_KHR: Self = Self::D3D12_RESOURCE;
}
impl std::fmt::Debug for ExternalMemoryHandleTypeFlagBits {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::OPAQUE_FD => "OPAQUE_FD",
            &Self::OPAQUE_WIN32 => "OPAQUE_WIN32",
            &Self::OPAQUE_WIN32_KMT => "OPAQUE_WIN32_KMT",
            &Self::D3D11_TEXTURE => "D3D11_TEXTURE",
            &Self::D3D11_TEXTURE_KMT => "D3D11_TEXTURE_KMT",
            &Self::D3D12_HEAP => "D3D12_HEAP",
            &Self::D3D12_RESOURCE => "D3D12_RESOURCE",
            &Self::ANDROID_HARDWARE_BUFFER_ANDROID => "ANDROID_HARDWARE_BUFFER_ANDROID",
            &Self::DMA_BUF_EXT => "DMA_BUF_EXT",
            &Self::HOST_ALLOCATION_EXT => "HOST_ALLOCATION_EXT",
            &Self::HOST_MAPPED_FOREIGN_MEMORY_EXT => "HOST_MAPPED_FOREIGN_MEMORY_EXT",
            _ => "(Unknown)",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalMemoryHandleTypeFlags.html) · Flags of [`ExternalMemoryHandleTypeFlagBits`](struct.ExternalMemoryHandleTypeFlagBits.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct ExternalMemoryHandleTypeFlags : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; const OPAQUE_FD = ExternalMemoryHandleTypeFlagBits :: OPAQUE_FD . 0 ; const OPAQUE_WIN32 = ExternalMemoryHandleTypeFlagBits :: OPAQUE_WIN32 . 0 ; const OPAQUE_WIN32_KMT = ExternalMemoryHandleTypeFlagBits :: OPAQUE_WIN32_KMT . 0 ; const D3D11_TEXTURE = ExternalMemoryHandleTypeFlagBits :: D3D11_TEXTURE . 0 ; const D3D11_TEXTURE_KMT = ExternalMemoryHandleTypeFlagBits :: D3D11_TEXTURE_KMT . 0 ; const D3D12_HEAP = ExternalMemoryHandleTypeFlagBits :: D3D12_HEAP . 0 ; const D3D12_RESOURCE = ExternalMemoryHandleTypeFlagBits :: D3D12_RESOURCE . 0 ; const ANDROID_HARDWARE_BUFFER_ANDROID = ExternalMemoryHandleTypeFlagBits :: ANDROID_HARDWARE_BUFFER_ANDROID . 0 ; const DMA_BUF_EXT = ExternalMemoryHandleTypeFlagBits :: DMA_BUF_EXT . 0 ; const HOST_ALLOCATION_EXT = ExternalMemoryHandleTypeFlagBits :: HOST_ALLOCATION_EXT . 0 ; const HOST_MAPPED_FOREIGN_MEMORY_EXT = ExternalMemoryHandleTypeFlagBits :: HOST_MAPPED_FOREIGN_MEMORY_EXT . 0 ; const OPAQUE_FD_KHR = ExternalMemoryHandleTypeFlagBits :: OPAQUE_FD_KHR . 0 ; const OPAQUE_WIN32_KHR = ExternalMemoryHandleTypeFlagBits :: OPAQUE_WIN32_KHR . 0 ; const OPAQUE_WIN32_KMT_KHR = ExternalMemoryHandleTypeFlagBits :: OPAQUE_WIN32_KMT_KHR . 0 ; const D3D11_TEXTURE_KHR = ExternalMemoryHandleTypeFlagBits :: D3D11_TEXTURE_KHR . 0 ; const D3D11_TEXTURE_KMT_KHR = ExternalMemoryHandleTypeFlagBits :: D3D11_TEXTURE_KMT_KHR . 0 ; const D3D12_HEAP_KHR = ExternalMemoryHandleTypeFlagBits :: D3D12_HEAP_KHR . 0 ; const D3D12_RESOURCE_KHR = ExternalMemoryHandleTypeFlagBits :: D3D12_RESOURCE_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalBufferProperties.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExternalBufferProperties {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub external_memory_properties: crate::vk1_1::ExternalMemoryProperties,
}
impl ExternalBufferProperties {
    #[inline]
    pub fn builder<'a>(self) -> ExternalBufferPropertiesBuilder<'a> {
        ExternalBufferPropertiesBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ExternalBufferProperties {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ExternalBufferProperties")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "external_memory_properties",
                &self.external_memory_properties,
            )
            .finish()
    }
}
impl Default for ExternalBufferProperties {
    fn default() -> ExternalBufferProperties {
        ExternalBufferProperties {
            s_type: crate::vk1_0::StructureType::EXTERNAL_BUFFER_PROPERTIES,
            p_next: std::ptr::null_mut(),
            external_memory_properties: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`ExternalBufferProperties`](struct.ExternalBufferProperties.html)"]
#[repr(transparent)]
pub struct ExternalBufferPropertiesBuilder<'a>(
    ExternalBufferProperties,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> ExternalBufferPropertiesBuilder<'a> {
    #[inline]
    pub fn new() -> ExternalBufferPropertiesBuilder<'a> {
        ExternalBufferPropertiesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn external_memory_properties(
        mut self,
        external_memory_properties: crate::vk1_1::ExternalMemoryProperties,
    ) -> Self {
        self.0.external_memory_properties = external_memory_properties;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> ExternalBufferProperties {
        self.0
    }
}
impl<'a> std::fmt::Debug for ExternalBufferPropertiesBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for ExternalBufferPropertiesBuilder<'a> {
    type Target = ExternalBufferProperties;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ExternalBufferPropertiesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalMemoryProperties.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExternalMemoryProperties {
    pub external_memory_features: crate::vk1_1::ExternalMemoryFeatureFlags,
    pub export_from_imported_handle_types: crate::vk1_1::ExternalMemoryHandleTypeFlags,
    pub compatible_handle_types: crate::vk1_1::ExternalMemoryHandleTypeFlags,
}
impl ExternalMemoryProperties {
    #[inline]
    pub fn builder<'a>(self) -> ExternalMemoryPropertiesBuilder<'a> {
        ExternalMemoryPropertiesBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ExternalMemoryProperties {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ExternalMemoryProperties")
            .field("external_memory_features", &self.external_memory_features)
            .field(
                "export_from_imported_handle_types",
                &self.export_from_imported_handle_types,
            )
            .field("compatible_handle_types", &self.compatible_handle_types)
            .finish()
    }
}
impl Default for ExternalMemoryProperties {
    fn default() -> ExternalMemoryProperties {
        ExternalMemoryProperties {
            external_memory_features: Default::default(),
            export_from_imported_handle_types: Default::default(),
            compatible_handle_types: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`ExternalMemoryProperties`](struct.ExternalMemoryProperties.html)"]
#[repr(transparent)]
pub struct ExternalMemoryPropertiesBuilder<'a>(
    ExternalMemoryProperties,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> ExternalMemoryPropertiesBuilder<'a> {
    #[inline]
    pub fn new() -> ExternalMemoryPropertiesBuilder<'a> {
        ExternalMemoryPropertiesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn external_memory_features(
        mut self,
        external_memory_features: crate::vk1_1::ExternalMemoryFeatureFlags,
    ) -> Self {
        self.0.external_memory_features = external_memory_features;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn export_from_imported_handle_types(
        mut self,
        export_from_imported_handle_types: crate::vk1_1::ExternalMemoryHandleTypeFlags,
    ) -> Self {
        self.0.export_from_imported_handle_types = export_from_imported_handle_types;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn compatible_handle_types(
        mut self,
        compatible_handle_types: crate::vk1_1::ExternalMemoryHandleTypeFlags,
    ) -> Self {
        self.0.compatible_handle_types = compatible_handle_types;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> ExternalMemoryProperties {
        self.0
    }
}
impl<'a> std::fmt::Debug for ExternalMemoryPropertiesBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for ExternalMemoryPropertiesBuilder<'a> {
    type Target = ExternalMemoryProperties;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ExternalMemoryPropertiesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalMemoryFeatureFlagBits.html) · Flag Bits of [`ExternalMemoryFeatureFlags`](struct.ExternalMemoryFeatureFlags.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ExternalMemoryFeatureFlagBits(pub u32);
impl ExternalMemoryFeatureFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> ExternalMemoryFeatureFlags {
        ExternalMemoryFeatureFlags::from_bits_truncate(self.0)
    }
}
#[doc = "[Part of `vk1_1`](../vk1_1/index.html)"]
impl ExternalMemoryFeatureFlagBits {
    pub const DEDICATED_ONLY: Self = Self(0x00000001);
    pub const EXPORTABLE: Self = Self(0x00000002);
    pub const IMPORTABLE: Self = Self(0x00000004);
}
#[doc = "[Part of `extensions::khr_external_memory_capabilities`](../extensions/khr_external_memory_capabilities/index.html)"]
impl ExternalMemoryFeatureFlagBits {
    pub const DEDICATED_ONLY_KHR: Self = Self::DEDICATED_ONLY;
    pub const EXPORTABLE_KHR: Self = Self::EXPORTABLE;
    pub const IMPORTABLE_KHR: Self = Self::IMPORTABLE;
}
impl std::fmt::Debug for ExternalMemoryFeatureFlagBits {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::DEDICATED_ONLY => "DEDICATED_ONLY",
            &Self::EXPORTABLE => "EXPORTABLE",
            &Self::IMPORTABLE => "IMPORTABLE",
            _ => "(Unknown)",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalMemoryFeatureFlags.html) · Flags of [`ExternalMemoryFeatureFlagBits`](struct.ExternalMemoryFeatureFlagBits.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct ExternalMemoryFeatureFlags : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; const DEDICATED_ONLY = ExternalMemoryFeatureFlagBits :: DEDICATED_ONLY . 0 ; const EXPORTABLE = ExternalMemoryFeatureFlagBits :: EXPORTABLE . 0 ; const IMPORTABLE = ExternalMemoryFeatureFlagBits :: IMPORTABLE . 0 ; const DEDICATED_ONLY_KHR = ExternalMemoryFeatureFlagBits :: DEDICATED_ONLY_KHR . 0 ; const EXPORTABLE_KHR = ExternalMemoryFeatureFlagBits :: EXPORTABLE_KHR . 0 ; const IMPORTABLE_KHR = ExternalMemoryFeatureFlagBits :: IMPORTABLE_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceExternalFenceInfo.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceExternalFenceInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub handle_type: crate::vk1_1::ExternalFenceHandleTypeFlagBits,
}
impl PhysicalDeviceExternalFenceInfo {
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceExternalFenceInfoBuilder<'a> {
        PhysicalDeviceExternalFenceInfoBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceExternalFenceInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceExternalFenceInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("handle_type", &self.handle_type)
            .finish()
    }
}
impl Default for PhysicalDeviceExternalFenceInfo {
    fn default() -> PhysicalDeviceExternalFenceInfo {
        PhysicalDeviceExternalFenceInfo {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO,
            p_next: std::ptr::null(),
            handle_type: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceExternalFenceInfo`](struct.PhysicalDeviceExternalFenceInfo.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceExternalFenceInfoBuilder<'a>(
    PhysicalDeviceExternalFenceInfo,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceExternalFenceInfoBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceExternalFenceInfoBuilder<'a> {
        PhysicalDeviceExternalFenceInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn handle_type(
        mut self,
        handle_type: crate::vk1_1::ExternalFenceHandleTypeFlagBits,
    ) -> Self {
        self.0.handle_type = handle_type;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceExternalFenceInfo {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceExternalFenceInfoBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceExternalFenceInfoBuilder<'a> {
    type Target = PhysicalDeviceExternalFenceInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceExternalFenceInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalFenceHandleTypeFlagBits.html) · Flag Bits of [`ExternalFenceHandleTypeFlags`](struct.ExternalFenceHandleTypeFlags.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ExternalFenceHandleTypeFlagBits(pub u32);
impl ExternalFenceHandleTypeFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> ExternalFenceHandleTypeFlags {
        ExternalFenceHandleTypeFlags::from_bits_truncate(self.0)
    }
}
#[doc = "[Part of `vk1_1`](../vk1_1/index.html)"]
impl ExternalFenceHandleTypeFlagBits {
    pub const OPAQUE_FD: Self = Self(0x00000001);
    pub const OPAQUE_WIN32: Self = Self(0x00000002);
    pub const OPAQUE_WIN32_KMT: Self = Self(0x00000004);
    pub const SYNC_FD: Self = Self(0x00000008);
}
#[doc = "[Part of `extensions::khr_external_fence_capabilities`](../extensions/khr_external_fence_capabilities/index.html)"]
impl ExternalFenceHandleTypeFlagBits {
    pub const OPAQUE_FD_KHR: Self = Self::OPAQUE_FD;
    pub const OPAQUE_WIN32_KHR: Self = Self::OPAQUE_WIN32;
    pub const OPAQUE_WIN32_KMT_KHR: Self = Self::OPAQUE_WIN32_KMT;
    pub const SYNC_FD_KHR: Self = Self::SYNC_FD;
}
impl std::fmt::Debug for ExternalFenceHandleTypeFlagBits {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::OPAQUE_FD => "OPAQUE_FD",
            &Self::OPAQUE_WIN32 => "OPAQUE_WIN32",
            &Self::OPAQUE_WIN32_KMT => "OPAQUE_WIN32_KMT",
            &Self::SYNC_FD => "SYNC_FD",
            _ => "(Unknown)",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalFenceHandleTypeFlags.html) · Flags of [`ExternalFenceHandleTypeFlagBits`](struct.ExternalFenceHandleTypeFlagBits.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct ExternalFenceHandleTypeFlags : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; const OPAQUE_FD = ExternalFenceHandleTypeFlagBits :: OPAQUE_FD . 0 ; const OPAQUE_WIN32 = ExternalFenceHandleTypeFlagBits :: OPAQUE_WIN32 . 0 ; const OPAQUE_WIN32_KMT = ExternalFenceHandleTypeFlagBits :: OPAQUE_WIN32_KMT . 0 ; const SYNC_FD = ExternalFenceHandleTypeFlagBits :: SYNC_FD . 0 ; const OPAQUE_FD_KHR = ExternalFenceHandleTypeFlagBits :: OPAQUE_FD_KHR . 0 ; const OPAQUE_WIN32_KHR = ExternalFenceHandleTypeFlagBits :: OPAQUE_WIN32_KHR . 0 ; const OPAQUE_WIN32_KMT_KHR = ExternalFenceHandleTypeFlagBits :: OPAQUE_WIN32_KMT_KHR . 0 ; const SYNC_FD_KHR = ExternalFenceHandleTypeFlagBits :: SYNC_FD_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalFenceProperties.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExternalFenceProperties {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub export_from_imported_handle_types: crate::vk1_1::ExternalFenceHandleTypeFlags,
    pub compatible_handle_types: crate::vk1_1::ExternalFenceHandleTypeFlags,
    pub external_fence_features: crate::vk1_1::ExternalFenceFeatureFlags,
}
impl ExternalFenceProperties {
    #[inline]
    pub fn builder<'a>(self) -> ExternalFencePropertiesBuilder<'a> {
        ExternalFencePropertiesBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ExternalFenceProperties {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ExternalFenceProperties")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "export_from_imported_handle_types",
                &self.export_from_imported_handle_types,
            )
            .field("compatible_handle_types", &self.compatible_handle_types)
            .field("external_fence_features", &self.external_fence_features)
            .finish()
    }
}
impl Default for ExternalFenceProperties {
    fn default() -> ExternalFenceProperties {
        ExternalFenceProperties {
            s_type: crate::vk1_0::StructureType::EXTERNAL_FENCE_PROPERTIES,
            p_next: std::ptr::null_mut(),
            export_from_imported_handle_types: Default::default(),
            compatible_handle_types: Default::default(),
            external_fence_features: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`ExternalFenceProperties`](struct.ExternalFenceProperties.html)"]
#[repr(transparent)]
pub struct ExternalFencePropertiesBuilder<'a>(
    ExternalFenceProperties,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> ExternalFencePropertiesBuilder<'a> {
    #[inline]
    pub fn new() -> ExternalFencePropertiesBuilder<'a> {
        ExternalFencePropertiesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn export_from_imported_handle_types(
        mut self,
        export_from_imported_handle_types: crate::vk1_1::ExternalFenceHandleTypeFlags,
    ) -> Self {
        self.0.export_from_imported_handle_types = export_from_imported_handle_types;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn compatible_handle_types(
        mut self,
        compatible_handle_types: crate::vk1_1::ExternalFenceHandleTypeFlags,
    ) -> Self {
        self.0.compatible_handle_types = compatible_handle_types;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn external_fence_features(
        mut self,
        external_fence_features: crate::vk1_1::ExternalFenceFeatureFlags,
    ) -> Self {
        self.0.external_fence_features = external_fence_features;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> ExternalFenceProperties {
        self.0
    }
}
impl<'a> std::fmt::Debug for ExternalFencePropertiesBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for ExternalFencePropertiesBuilder<'a> {
    type Target = ExternalFenceProperties;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ExternalFencePropertiesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalFenceFeatureFlagBits.html) · Flag Bits of [`ExternalFenceFeatureFlags`](struct.ExternalFenceFeatureFlags.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ExternalFenceFeatureFlagBits(pub u32);
impl ExternalFenceFeatureFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> ExternalFenceFeatureFlags {
        ExternalFenceFeatureFlags::from_bits_truncate(self.0)
    }
}
#[doc = "[Part of `vk1_1`](../vk1_1/index.html)"]
impl ExternalFenceFeatureFlagBits {
    pub const EXPORTABLE: Self = Self(0x00000001);
    pub const IMPORTABLE: Self = Self(0x00000002);
}
#[doc = "[Part of `extensions::khr_external_fence_capabilities`](../extensions/khr_external_fence_capabilities/index.html)"]
impl ExternalFenceFeatureFlagBits {
    pub const EXPORTABLE_KHR: Self = Self::EXPORTABLE;
    pub const IMPORTABLE_KHR: Self = Self::IMPORTABLE;
}
impl std::fmt::Debug for ExternalFenceFeatureFlagBits {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::EXPORTABLE => "EXPORTABLE",
            &Self::IMPORTABLE => "IMPORTABLE",
            _ => "(Unknown)",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalFenceFeatureFlags.html) · Flags of [`ExternalFenceFeatureFlagBits`](struct.ExternalFenceFeatureFlagBits.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct ExternalFenceFeatureFlags : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; const EXPORTABLE = ExternalFenceFeatureFlagBits :: EXPORTABLE . 0 ; const IMPORTABLE = ExternalFenceFeatureFlagBits :: IMPORTABLE . 0 ; const EXPORTABLE_KHR = ExternalFenceFeatureFlagBits :: EXPORTABLE_KHR . 0 ; const IMPORTABLE_KHR = ExternalFenceFeatureFlagBits :: IMPORTABLE_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceExternalSemaphoreInfo.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceExternalSemaphoreInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub handle_type: crate::vk1_1::ExternalSemaphoreHandleTypeFlagBits,
}
impl PhysicalDeviceExternalSemaphoreInfo {
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceExternalSemaphoreInfoBuilder<'a> {
        PhysicalDeviceExternalSemaphoreInfoBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceExternalSemaphoreInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceExternalSemaphoreInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("handle_type", &self.handle_type)
            .finish()
    }
}
impl Default for PhysicalDeviceExternalSemaphoreInfo {
    fn default() -> PhysicalDeviceExternalSemaphoreInfo {
        PhysicalDeviceExternalSemaphoreInfo {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO,
            p_next: std::ptr::null(),
            handle_type: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceExternalSemaphoreInfo`](struct.PhysicalDeviceExternalSemaphoreInfo.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceExternalSemaphoreInfoBuilder<'a>(
    PhysicalDeviceExternalSemaphoreInfo,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceExternalSemaphoreInfoBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceExternalSemaphoreInfoBuilder<'a> {
        PhysicalDeviceExternalSemaphoreInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn handle_type(
        mut self,
        handle_type: crate::vk1_1::ExternalSemaphoreHandleTypeFlagBits,
    ) -> Self {
        self.0.handle_type = handle_type;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceExternalSemaphoreInfo {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceExternalSemaphoreInfoBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceExternalSemaphoreInfoBuilder<'a> {
    type Target = PhysicalDeviceExternalSemaphoreInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceExternalSemaphoreInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalSemaphoreHandleTypeFlagBits.html) · Flag Bits of [`ExternalSemaphoreHandleTypeFlags`](struct.ExternalSemaphoreHandleTypeFlags.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ExternalSemaphoreHandleTypeFlagBits(pub u32);
impl ExternalSemaphoreHandleTypeFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> ExternalSemaphoreHandleTypeFlags {
        ExternalSemaphoreHandleTypeFlags::from_bits_truncate(self.0)
    }
}
#[doc = "[Part of `vk1_1`](../vk1_1/index.html)"]
impl ExternalSemaphoreHandleTypeFlagBits {
    pub const OPAQUE_FD: Self = Self(0x00000001);
    pub const OPAQUE_WIN32: Self = Self(0x00000002);
    pub const OPAQUE_WIN32_KMT: Self = Self(0x00000004);
    pub const D3D12_FENCE: Self = Self(0x00000008);
    pub const SYNC_FD: Self = Self(0x00000010);
}
#[doc = "[Part of `extensions::khr_external_semaphore_capabilities`](../extensions/khr_external_semaphore_capabilities/index.html)"]
impl ExternalSemaphoreHandleTypeFlagBits {
    pub const OPAQUE_FD_KHR: Self = Self::OPAQUE_FD;
    pub const OPAQUE_WIN32_KHR: Self = Self::OPAQUE_WIN32;
    pub const OPAQUE_WIN32_KMT_KHR: Self = Self::OPAQUE_WIN32_KMT;
    pub const D3D12_FENCE_KHR: Self = Self::D3D12_FENCE;
    pub const SYNC_FD_KHR: Self = Self::SYNC_FD;
}
impl std::fmt::Debug for ExternalSemaphoreHandleTypeFlagBits {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::OPAQUE_FD => "OPAQUE_FD",
            &Self::OPAQUE_WIN32 => "OPAQUE_WIN32",
            &Self::OPAQUE_WIN32_KMT => "OPAQUE_WIN32_KMT",
            &Self::D3D12_FENCE => "D3D12_FENCE",
            &Self::SYNC_FD => "SYNC_FD",
            _ => "(Unknown)",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalSemaphoreHandleTypeFlags.html) · Flags of [`ExternalSemaphoreHandleTypeFlagBits`](struct.ExternalSemaphoreHandleTypeFlagBits.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct ExternalSemaphoreHandleTypeFlags : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; const OPAQUE_FD = ExternalSemaphoreHandleTypeFlagBits :: OPAQUE_FD . 0 ; const OPAQUE_WIN32 = ExternalSemaphoreHandleTypeFlagBits :: OPAQUE_WIN32 . 0 ; const OPAQUE_WIN32_KMT = ExternalSemaphoreHandleTypeFlagBits :: OPAQUE_WIN32_KMT . 0 ; const D3D12_FENCE = ExternalSemaphoreHandleTypeFlagBits :: D3D12_FENCE . 0 ; const SYNC_FD = ExternalSemaphoreHandleTypeFlagBits :: SYNC_FD . 0 ; const OPAQUE_FD_KHR = ExternalSemaphoreHandleTypeFlagBits :: OPAQUE_FD_KHR . 0 ; const OPAQUE_WIN32_KHR = ExternalSemaphoreHandleTypeFlagBits :: OPAQUE_WIN32_KHR . 0 ; const OPAQUE_WIN32_KMT_KHR = ExternalSemaphoreHandleTypeFlagBits :: OPAQUE_WIN32_KMT_KHR . 0 ; const D3D12_FENCE_KHR = ExternalSemaphoreHandleTypeFlagBits :: D3D12_FENCE_KHR . 0 ; const SYNC_FD_KHR = ExternalSemaphoreHandleTypeFlagBits :: SYNC_FD_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalSemaphoreProperties.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExternalSemaphoreProperties {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub export_from_imported_handle_types: crate::vk1_1::ExternalSemaphoreHandleTypeFlags,
    pub compatible_handle_types: crate::vk1_1::ExternalSemaphoreHandleTypeFlags,
    pub external_semaphore_features: crate::vk1_1::ExternalSemaphoreFeatureFlags,
}
impl ExternalSemaphoreProperties {
    #[inline]
    pub fn builder<'a>(self) -> ExternalSemaphorePropertiesBuilder<'a> {
        ExternalSemaphorePropertiesBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ExternalSemaphoreProperties {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ExternalSemaphoreProperties")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "export_from_imported_handle_types",
                &self.export_from_imported_handle_types,
            )
            .field("compatible_handle_types", &self.compatible_handle_types)
            .field(
                "external_semaphore_features",
                &self.external_semaphore_features,
            )
            .finish()
    }
}
impl Default for ExternalSemaphoreProperties {
    fn default() -> ExternalSemaphoreProperties {
        ExternalSemaphoreProperties {
            s_type: crate::vk1_0::StructureType::EXTERNAL_SEMAPHORE_PROPERTIES,
            p_next: std::ptr::null_mut(),
            export_from_imported_handle_types: Default::default(),
            compatible_handle_types: Default::default(),
            external_semaphore_features: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`ExternalSemaphoreProperties`](struct.ExternalSemaphoreProperties.html)"]
#[repr(transparent)]
pub struct ExternalSemaphorePropertiesBuilder<'a>(
    ExternalSemaphoreProperties,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> ExternalSemaphorePropertiesBuilder<'a> {
    #[inline]
    pub fn new() -> ExternalSemaphorePropertiesBuilder<'a> {
        ExternalSemaphorePropertiesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn export_from_imported_handle_types(
        mut self,
        export_from_imported_handle_types: crate::vk1_1::ExternalSemaphoreHandleTypeFlags,
    ) -> Self {
        self.0.export_from_imported_handle_types = export_from_imported_handle_types;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn compatible_handle_types(
        mut self,
        compatible_handle_types: crate::vk1_1::ExternalSemaphoreHandleTypeFlags,
    ) -> Self {
        self.0.compatible_handle_types = compatible_handle_types;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn external_semaphore_features(
        mut self,
        external_semaphore_features: crate::vk1_1::ExternalSemaphoreFeatureFlags,
    ) -> Self {
        self.0.external_semaphore_features = external_semaphore_features;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> ExternalSemaphoreProperties {
        self.0
    }
}
impl<'a> std::fmt::Debug for ExternalSemaphorePropertiesBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for ExternalSemaphorePropertiesBuilder<'a> {
    type Target = ExternalSemaphoreProperties;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ExternalSemaphorePropertiesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalSemaphoreFeatureFlagBits.html) · Flag Bits of [`ExternalSemaphoreFeatureFlags`](struct.ExternalSemaphoreFeatureFlags.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ExternalSemaphoreFeatureFlagBits(pub u32);
impl ExternalSemaphoreFeatureFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> ExternalSemaphoreFeatureFlags {
        ExternalSemaphoreFeatureFlags::from_bits_truncate(self.0)
    }
}
#[doc = "[Part of `vk1_1`](../vk1_1/index.html)"]
impl ExternalSemaphoreFeatureFlagBits {
    pub const EXPORTABLE: Self = Self(0x00000001);
    pub const IMPORTABLE: Self = Self(0x00000002);
}
#[doc = "[Part of `extensions::khr_external_semaphore_capabilities`](../extensions/khr_external_semaphore_capabilities/index.html)"]
impl ExternalSemaphoreFeatureFlagBits {
    pub const EXPORTABLE_KHR: Self = Self::EXPORTABLE;
    pub const IMPORTABLE_KHR: Self = Self::IMPORTABLE;
}
impl std::fmt::Debug for ExternalSemaphoreFeatureFlagBits {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::EXPORTABLE => "EXPORTABLE",
            &Self::IMPORTABLE => "IMPORTABLE",
            _ => "(Unknown)",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalSemaphoreFeatureFlags.html) · Flags of [`ExternalSemaphoreFeatureFlagBits`](struct.ExternalSemaphoreFeatureFlagBits.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct ExternalSemaphoreFeatureFlags : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; const EXPORTABLE = ExternalSemaphoreFeatureFlagBits :: EXPORTABLE . 0 ; const IMPORTABLE = ExternalSemaphoreFeatureFlagBits :: IMPORTABLE . 0 ; const EXPORTABLE_KHR = ExternalSemaphoreFeatureFlagBits :: EXPORTABLE_KHR . 0 ; const IMPORTABLE_KHR = ExternalSemaphoreFeatureFlagBits :: IMPORTABLE_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorSetLayoutSupport.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DescriptorSetLayoutSupport {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub supported: crate::vk1_0::Bool32,
}
impl DescriptorSetLayoutSupport {
    #[inline]
    pub fn builder<'a>(self) -> DescriptorSetLayoutSupportBuilder<'a> {
        DescriptorSetLayoutSupportBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for DescriptorSetLayoutSupport {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("DescriptorSetLayoutSupport")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("supported", &(self.supported != 0))
            .finish()
    }
}
impl Default for DescriptorSetLayoutSupport {
    fn default() -> DescriptorSetLayoutSupport {
        DescriptorSetLayoutSupport {
            s_type: crate::vk1_0::StructureType::DESCRIPTOR_SET_LAYOUT_SUPPORT,
            p_next: std::ptr::null_mut(),
            supported: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`DescriptorSetLayoutSupport`](struct.DescriptorSetLayoutSupport.html)"]
#[repr(transparent)]
pub struct DescriptorSetLayoutSupportBuilder<'a>(
    DescriptorSetLayoutSupport,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> DescriptorSetLayoutSupportBuilder<'a> {
    #[inline]
    pub fn new() -> DescriptorSetLayoutSupportBuilder<'a> {
        DescriptorSetLayoutSupportBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn supported(mut self, supported: bool) -> Self {
        self.0.supported = supported as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> DescriptorSetLayoutSupport {
        self.0
    }
}
impl<'a> std::fmt::Debug for DescriptorSetLayoutSupportBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for DescriptorSetLayoutSupportBuilder<'a> {
    type Target = DescriptorSetLayoutSupport;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DescriptorSetLayoutSupportBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSubgroupProperties.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceSubgroupProperties {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub subgroup_size: u32,
    pub supported_stages: crate::vk1_0::ShaderStageFlags,
    pub supported_operations: crate::vk1_1::SubgroupFeatureFlags,
    pub quad_operations_in_all_stages: crate::vk1_0::Bool32,
}
impl PhysicalDeviceSubgroupProperties {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceSubgroupProperties,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceSubgroupPropertiesBuilder<'a> {
        PhysicalDeviceSubgroupPropertiesBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceSubgroupProperties {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceSubgroupProperties")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("subgroup_size", &self.subgroup_size)
            .field("supported_stages", &self.supported_stages)
            .field("supported_operations", &self.supported_operations)
            .field(
                "quad_operations_in_all_stages",
                &(self.quad_operations_in_all_stages != 0),
            )
            .finish()
    }
}
impl Default for PhysicalDeviceSubgroupProperties {
    fn default() -> PhysicalDeviceSubgroupProperties {
        PhysicalDeviceSubgroupProperties {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_SUBGROUP_PROPERTIES,
            p_next: std::ptr::null_mut(),
            subgroup_size: Default::default(),
            supported_stages: Default::default(),
            supported_operations: Default::default(),
            quad_operations_in_all_stages: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceSubgroupProperties::extend`](struct.PhysicalDeviceSubgroupProperties.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceSubgroupProperties {}
impl ExtendableByPhysicalDeviceSubgroupProperties for crate::vk1_1::PhysicalDeviceProperties2 {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceSubgroupProperties`](struct.PhysicalDeviceSubgroupProperties.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceSubgroupPropertiesBuilder<'a>(
    PhysicalDeviceSubgroupProperties,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceSubgroupPropertiesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceSubgroupPropertiesBuilder<'a> {
        PhysicalDeviceSubgroupPropertiesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn subgroup_size(mut self, subgroup_size: u32) -> Self {
        self.0.subgroup_size = subgroup_size;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn supported_stages(mut self, supported_stages: crate::vk1_0::ShaderStageFlags) -> Self {
        self.0.supported_stages = supported_stages;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn supported_operations(
        mut self,
        supported_operations: crate::vk1_1::SubgroupFeatureFlags,
    ) -> Self {
        self.0.supported_operations = supported_operations;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn quad_operations_in_all_stages(mut self, quad_operations_in_all_stages: bool) -> Self {
        self.0.quad_operations_in_all_stages = quad_operations_in_all_stages as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceSubgroupProperties {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceSubgroupPropertiesBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceSubgroupPropertiesBuilder<'a> {
    type Target = PhysicalDeviceSubgroupProperties;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceSubgroupPropertiesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubgroupFeatureFlagBits.html) · Flag Bits of [`SubgroupFeatureFlags`](struct.SubgroupFeatureFlags.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct SubgroupFeatureFlagBits(pub u32);
impl SubgroupFeatureFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> SubgroupFeatureFlags {
        SubgroupFeatureFlags::from_bits_truncate(self.0)
    }
}
#[doc = "[Part of `vk1_1`](../vk1_1/index.html)"]
impl SubgroupFeatureFlagBits {
    pub const BASIC: Self = Self(0x00000001);
    pub const VOTE: Self = Self(0x00000002);
    pub const ARITHMETIC: Self = Self(0x00000004);
    pub const BALLOT: Self = Self(0x00000008);
    pub const SHUFFLE: Self = Self(0x00000010);
    pub const SHUFFLE_RELATIVE: Self = Self(0x00000020);
    pub const CLUSTERED: Self = Self(0x00000040);
    pub const QUAD: Self = Self(0x00000080);
}
#[doc = "[Part of `extensions::nv_shader_subgroup_partitioned`](../extensions/nv_shader_subgroup_partitioned/index.html)"]
impl SubgroupFeatureFlagBits {
    pub const PARTITIONED_NV: Self = Self(0x00000100);
}
impl std::fmt::Debug for SubgroupFeatureFlagBits {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::BASIC => "BASIC",
            &Self::VOTE => "VOTE",
            &Self::ARITHMETIC => "ARITHMETIC",
            &Self::BALLOT => "BALLOT",
            &Self::SHUFFLE => "SHUFFLE",
            &Self::SHUFFLE_RELATIVE => "SHUFFLE_RELATIVE",
            &Self::CLUSTERED => "CLUSTERED",
            &Self::QUAD => "QUAD",
            &Self::PARTITIONED_NV => "PARTITIONED_NV",
            _ => "(Unknown)",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubgroupFeatureFlags.html) · Flags of [`SubgroupFeatureFlagBits`](struct.SubgroupFeatureFlagBits.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct SubgroupFeatureFlags : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; const BASIC = SubgroupFeatureFlagBits :: BASIC . 0 ; const VOTE = SubgroupFeatureFlagBits :: VOTE . 0 ; const ARITHMETIC = SubgroupFeatureFlagBits :: ARITHMETIC . 0 ; const BALLOT = SubgroupFeatureFlagBits :: BALLOT . 0 ; const SHUFFLE = SubgroupFeatureFlagBits :: SHUFFLE . 0 ; const SHUFFLE_RELATIVE = SubgroupFeatureFlagBits :: SHUFFLE_RELATIVE . 0 ; const CLUSTERED = SubgroupFeatureFlagBits :: CLUSTERED . 0 ; const QUAD = SubgroupFeatureFlagBits :: QUAD . 0 ; const PARTITIONED_NV = SubgroupFeatureFlagBits :: PARTITIONED_NV . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevice16BitStorageFeatures.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDevice16BitStorageFeatures {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub storage_buffer16_bit_access: crate::vk1_0::Bool32,
    pub uniform_and_storage_buffer16_bit_access: crate::vk1_0::Bool32,
    pub storage_push_constant16: crate::vk1_0::Bool32,
    pub storage_input_output16: crate::vk1_0::Bool32,
}
impl PhysicalDevice16BitStorageFeatures {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDevice16BitStorageFeatures,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDevice16BitStorageFeaturesBuilder<'a> {
        PhysicalDevice16BitStorageFeaturesBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDevice16BitStorageFeatures {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDevice16BitStorageFeatures")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "storage_buffer16_bit_access",
                &(self.storage_buffer16_bit_access != 0),
            )
            .field(
                "uniform_and_storage_buffer16_bit_access",
                &(self.uniform_and_storage_buffer16_bit_access != 0),
            )
            .field(
                "storage_push_constant16",
                &(self.storage_push_constant16 != 0),
            )
            .field(
                "storage_input_output16",
                &(self.storage_input_output16 != 0),
            )
            .finish()
    }
}
impl Default for PhysicalDevice16BitStorageFeatures {
    fn default() -> PhysicalDevice16BitStorageFeatures {
        PhysicalDevice16BitStorageFeatures {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES,
            p_next: std::ptr::null_mut(),
            storage_buffer16_bit_access: Default::default(),
            uniform_and_storage_buffer16_bit_access: Default::default(),
            storage_push_constant16: Default::default(),
            storage_input_output16: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDevice16BitStorageFeatures::extend`](struct.PhysicalDevice16BitStorageFeatures.html#method.extend)"]
pub trait ExtendableByPhysicalDevice16BitStorageFeatures {}
impl ExtendableByPhysicalDevice16BitStorageFeatures for crate::vk1_1::PhysicalDeviceFeatures2 {}
impl ExtendableByPhysicalDevice16BitStorageFeatures for crate::vk1_0::DeviceCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDevice16BitStorageFeatures`](struct.PhysicalDevice16BitStorageFeatures.html)"]
#[repr(transparent)]
pub struct PhysicalDevice16BitStorageFeaturesBuilder<'a>(
    PhysicalDevice16BitStorageFeatures,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDevice16BitStorageFeaturesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDevice16BitStorageFeaturesBuilder<'a> {
        PhysicalDevice16BitStorageFeaturesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn storage_buffer16_bit_access(mut self, storage_buffer16_bit_access: bool) -> Self {
        self.0.storage_buffer16_bit_access = storage_buffer16_bit_access as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn uniform_and_storage_buffer16_bit_access(
        mut self,
        uniform_and_storage_buffer16_bit_access: bool,
    ) -> Self {
        self.0.uniform_and_storage_buffer16_bit_access =
            uniform_and_storage_buffer16_bit_access as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn storage_push_constant16(mut self, storage_push_constant16: bool) -> Self {
        self.0.storage_push_constant16 = storage_push_constant16 as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn storage_input_output16(mut self, storage_input_output16: bool) -> Self {
        self.0.storage_input_output16 = storage_input_output16 as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDevice16BitStorageFeatures {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDevice16BitStorageFeaturesBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDevice16BitStorageFeaturesBuilder<'a> {
    type Target = PhysicalDevice16BitStorageFeatures;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDevice16BitStorageFeaturesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryDedicatedRequirements.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryDedicatedRequirements {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub prefers_dedicated_allocation: crate::vk1_0::Bool32,
    pub requires_dedicated_allocation: crate::vk1_0::Bool32,
}
impl MemoryDedicatedRequirements {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByMemoryDedicatedRequirements,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> MemoryDedicatedRequirementsBuilder<'a> {
        MemoryDedicatedRequirementsBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for MemoryDedicatedRequirements {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("MemoryDedicatedRequirements")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "prefers_dedicated_allocation",
                &(self.prefers_dedicated_allocation != 0),
            )
            .field(
                "requires_dedicated_allocation",
                &(self.requires_dedicated_allocation != 0),
            )
            .finish()
    }
}
impl Default for MemoryDedicatedRequirements {
    fn default() -> MemoryDedicatedRequirements {
        MemoryDedicatedRequirements {
            s_type: crate::vk1_0::StructureType::MEMORY_DEDICATED_REQUIREMENTS,
            p_next: std::ptr::null_mut(),
            prefers_dedicated_allocation: Default::default(),
            requires_dedicated_allocation: Default::default(),
        }
    }
}
#[doc = "Used by [`MemoryDedicatedRequirements::extend`](struct.MemoryDedicatedRequirements.html#method.extend)"]
pub trait ExtendableByMemoryDedicatedRequirements {}
impl ExtendableByMemoryDedicatedRequirements for crate::vk1_1::MemoryRequirements2 {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`MemoryDedicatedRequirements`](struct.MemoryDedicatedRequirements.html)"]
#[repr(transparent)]
pub struct MemoryDedicatedRequirementsBuilder<'a>(
    MemoryDedicatedRequirements,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> MemoryDedicatedRequirementsBuilder<'a> {
    #[inline]
    pub fn new() -> MemoryDedicatedRequirementsBuilder<'a> {
        MemoryDedicatedRequirementsBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn prefers_dedicated_allocation(mut self, prefers_dedicated_allocation: bool) -> Self {
        self.0.prefers_dedicated_allocation = prefers_dedicated_allocation as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn requires_dedicated_allocation(mut self, requires_dedicated_allocation: bool) -> Self {
        self.0.requires_dedicated_allocation = requires_dedicated_allocation as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> MemoryDedicatedRequirements {
        self.0
    }
}
impl<'a> std::fmt::Debug for MemoryDedicatedRequirementsBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for MemoryDedicatedRequirementsBuilder<'a> {
    type Target = MemoryDedicatedRequirements;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for MemoryDedicatedRequirementsBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryDedicatedAllocateInfo.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryDedicatedAllocateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub image: crate::vk1_0::Image,
    pub buffer: crate::vk1_0::Buffer,
}
impl MemoryDedicatedAllocateInfo {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByMemoryDedicatedAllocateInfo,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> MemoryDedicatedAllocateInfoBuilder<'a> {
        MemoryDedicatedAllocateInfoBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for MemoryDedicatedAllocateInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("MemoryDedicatedAllocateInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("image", &self.image)
            .field("buffer", &self.buffer)
            .finish()
    }
}
impl Default for MemoryDedicatedAllocateInfo {
    fn default() -> MemoryDedicatedAllocateInfo {
        MemoryDedicatedAllocateInfo {
            s_type: crate::vk1_0::StructureType::MEMORY_DEDICATED_ALLOCATE_INFO,
            p_next: std::ptr::null(),
            image: Default::default(),
            buffer: Default::default(),
        }
    }
}
#[doc = "Used by [`MemoryDedicatedAllocateInfo::extend`](struct.MemoryDedicatedAllocateInfo.html#method.extend)"]
pub trait ExtendableByMemoryDedicatedAllocateInfo {}
impl ExtendableByMemoryDedicatedAllocateInfo for crate::vk1_0::MemoryAllocateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`MemoryDedicatedAllocateInfo`](struct.MemoryDedicatedAllocateInfo.html)"]
#[repr(transparent)]
pub struct MemoryDedicatedAllocateInfoBuilder<'a>(
    MemoryDedicatedAllocateInfo,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> MemoryDedicatedAllocateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> MemoryDedicatedAllocateInfoBuilder<'a> {
        MemoryDedicatedAllocateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn image(mut self, image: crate::vk1_0::Image) -> Self {
        self.0.image = image;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn buffer(mut self, buffer: crate::vk1_0::Buffer) -> Self {
        self.0.buffer = buffer;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> MemoryDedicatedAllocateInfo {
        self.0
    }
}
impl<'a> std::fmt::Debug for MemoryDedicatedAllocateInfoBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for MemoryDedicatedAllocateInfoBuilder<'a> {
    type Target = MemoryDedicatedAllocateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for MemoryDedicatedAllocateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryAllocateFlagBits.html) · Flag Bits of [`MemoryAllocateFlags`](struct.MemoryAllocateFlags.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct MemoryAllocateFlagBits(pub u32);
impl MemoryAllocateFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> MemoryAllocateFlags {
        MemoryAllocateFlags::from_bits_truncate(self.0)
    }
}
#[doc = "[Part of `vk1_1`](../vk1_1/index.html)"]
impl MemoryAllocateFlagBits {
    pub const DEVICE_MASK: Self = Self(0x00000001);
}
#[doc = "[Part of `vk1_2`](../vk1_2/index.html)"]
impl MemoryAllocateFlagBits {
    pub const DEVICE_ADDRESS: Self = Self(0x00000002);
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY: Self = Self(0x00000004);
}
#[doc = "[Part of `extensions::khr_buffer_device_address`](../extensions/khr_buffer_device_address/index.html)"]
impl MemoryAllocateFlagBits {
    pub const DEVICE_ADDRESS_KHR: Self = Self::DEVICE_ADDRESS;
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY_KHR: Self = Self::DEVICE_ADDRESS_CAPTURE_REPLAY;
}
#[doc = "[Part of `extensions::khr_device_group`](../extensions/khr_device_group/index.html)"]
impl MemoryAllocateFlagBits {
    pub const DEVICE_MASK_KHR: Self = Self::DEVICE_MASK;
}
impl std::fmt::Debug for MemoryAllocateFlagBits {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::DEVICE_MASK => "DEVICE_MASK",
            &Self::DEVICE_ADDRESS => "DEVICE_ADDRESS",
            &Self::DEVICE_ADDRESS_CAPTURE_REPLAY => "DEVICE_ADDRESS_CAPTURE_REPLAY",
            _ => "(Unknown)",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryAllocateFlags.html) · Flags of [`MemoryAllocateFlagBits`](struct.MemoryAllocateFlagBits.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct MemoryAllocateFlags : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; const DEVICE_MASK = MemoryAllocateFlagBits :: DEVICE_MASK . 0 ; const DEVICE_ADDRESS = MemoryAllocateFlagBits :: DEVICE_ADDRESS . 0 ; const DEVICE_ADDRESS_CAPTURE_REPLAY = MemoryAllocateFlagBits :: DEVICE_ADDRESS_CAPTURE_REPLAY . 0 ; const DEVICE_ADDRESS_KHR = MemoryAllocateFlagBits :: DEVICE_ADDRESS_KHR . 0 ; const DEVICE_ADDRESS_CAPTURE_REPLAY_KHR = MemoryAllocateFlagBits :: DEVICE_ADDRESS_CAPTURE_REPLAY_KHR . 0 ; const DEVICE_MASK_KHR = MemoryAllocateFlagBits :: DEVICE_MASK_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryAllocateFlagsInfo.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryAllocateFlagsInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::vk1_1::MemoryAllocateFlags,
    pub device_mask: u32,
}
impl MemoryAllocateFlagsInfo {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByMemoryAllocateFlagsInfo,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> MemoryAllocateFlagsInfoBuilder<'a> {
        MemoryAllocateFlagsInfoBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for MemoryAllocateFlagsInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("MemoryAllocateFlagsInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("device_mask", &self.device_mask)
            .finish()
    }
}
impl Default for MemoryAllocateFlagsInfo {
    fn default() -> MemoryAllocateFlagsInfo {
        MemoryAllocateFlagsInfo {
            s_type: crate::vk1_0::StructureType::MEMORY_ALLOCATE_FLAGS_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            device_mask: Default::default(),
        }
    }
}
#[doc = "Used by [`MemoryAllocateFlagsInfo::extend`](struct.MemoryAllocateFlagsInfo.html#method.extend)"]
pub trait ExtendableByMemoryAllocateFlagsInfo {}
impl ExtendableByMemoryAllocateFlagsInfo for crate::vk1_0::MemoryAllocateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`MemoryAllocateFlagsInfo`](struct.MemoryAllocateFlagsInfo.html)"]
#[repr(transparent)]
pub struct MemoryAllocateFlagsInfoBuilder<'a>(
    MemoryAllocateFlagsInfo,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> MemoryAllocateFlagsInfoBuilder<'a> {
    #[inline]
    pub fn new() -> MemoryAllocateFlagsInfoBuilder<'a> {
        MemoryAllocateFlagsInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_1::MemoryAllocateFlags) -> Self {
        self.0.flags = flags;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn device_mask(mut self, device_mask: u32) -> Self {
        self.0.device_mask = device_mask;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> MemoryAllocateFlagsInfo {
        self.0
    }
}
impl<'a> std::fmt::Debug for MemoryAllocateFlagsInfoBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for MemoryAllocateFlagsInfoBuilder<'a> {
    type Target = MemoryAllocateFlagsInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for MemoryAllocateFlagsInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceGroupRenderPassBeginInfo.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DeviceGroupRenderPassBeginInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub device_mask: u32,
    pub device_render_area_count: u32,
    pub p_device_render_areas: *const crate::vk1_0::Rect2D,
}
impl DeviceGroupRenderPassBeginInfo {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByDeviceGroupRenderPassBeginInfo,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> DeviceGroupRenderPassBeginInfoBuilder<'a> {
        DeviceGroupRenderPassBeginInfoBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for DeviceGroupRenderPassBeginInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("DeviceGroupRenderPassBeginInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("device_mask", &self.device_mask)
            .field("device_render_area_count", &self.device_render_area_count)
            .field("p_device_render_areas", &self.p_device_render_areas)
            .finish()
    }
}
impl Default for DeviceGroupRenderPassBeginInfo {
    fn default() -> DeviceGroupRenderPassBeginInfo {
        DeviceGroupRenderPassBeginInfo {
            s_type: crate::vk1_0::StructureType::DEVICE_GROUP_RENDER_PASS_BEGIN_INFO,
            p_next: std::ptr::null(),
            device_mask: Default::default(),
            device_render_area_count: Default::default(),
            p_device_render_areas: std::ptr::null(),
        }
    }
}
#[doc = "Used by [`DeviceGroupRenderPassBeginInfo::extend`](struct.DeviceGroupRenderPassBeginInfo.html#method.extend)"]
pub trait ExtendableByDeviceGroupRenderPassBeginInfo {}
impl ExtendableByDeviceGroupRenderPassBeginInfo for crate::vk1_0::RenderPassBeginInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`DeviceGroupRenderPassBeginInfo`](struct.DeviceGroupRenderPassBeginInfo.html)"]
#[repr(transparent)]
pub struct DeviceGroupRenderPassBeginInfoBuilder<'a>(
    DeviceGroupRenderPassBeginInfo,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> DeviceGroupRenderPassBeginInfoBuilder<'a> {
    #[inline]
    pub fn new() -> DeviceGroupRenderPassBeginInfoBuilder<'a> {
        DeviceGroupRenderPassBeginInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn device_mask(mut self, device_mask: u32) -> Self {
        self.0.device_mask = device_mask;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn device_render_areas(
        mut self,
        device_render_areas: &'a [crate::vk1_0::Rect2DBuilder],
    ) -> Self {
        self.0.device_render_area_count = device_render_areas.len() as _;
        self.0.p_device_render_areas = device_render_areas.as_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> DeviceGroupRenderPassBeginInfo {
        self.0
    }
}
impl<'a> std::fmt::Debug for DeviceGroupRenderPassBeginInfoBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for DeviceGroupRenderPassBeginInfoBuilder<'a> {
    type Target = DeviceGroupRenderPassBeginInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DeviceGroupRenderPassBeginInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceGroupCommandBufferBeginInfo.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DeviceGroupCommandBufferBeginInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub device_mask: u32,
}
impl DeviceGroupCommandBufferBeginInfo {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByDeviceGroupCommandBufferBeginInfo,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> DeviceGroupCommandBufferBeginInfoBuilder<'a> {
        DeviceGroupCommandBufferBeginInfoBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for DeviceGroupCommandBufferBeginInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("DeviceGroupCommandBufferBeginInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("device_mask", &self.device_mask)
            .finish()
    }
}
impl Default for DeviceGroupCommandBufferBeginInfo {
    fn default() -> DeviceGroupCommandBufferBeginInfo {
        DeviceGroupCommandBufferBeginInfo {
            s_type: crate::vk1_0::StructureType::DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO,
            p_next: std::ptr::null(),
            device_mask: Default::default(),
        }
    }
}
#[doc = "Used by [`DeviceGroupCommandBufferBeginInfo::extend`](struct.DeviceGroupCommandBufferBeginInfo.html#method.extend)"]
pub trait ExtendableByDeviceGroupCommandBufferBeginInfo {}
impl ExtendableByDeviceGroupCommandBufferBeginInfo for crate::vk1_0::CommandBufferBeginInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`DeviceGroupCommandBufferBeginInfo`](struct.DeviceGroupCommandBufferBeginInfo.html)"]
#[repr(transparent)]
pub struct DeviceGroupCommandBufferBeginInfoBuilder<'a>(
    DeviceGroupCommandBufferBeginInfo,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> DeviceGroupCommandBufferBeginInfoBuilder<'a> {
    #[inline]
    pub fn new() -> DeviceGroupCommandBufferBeginInfoBuilder<'a> {
        DeviceGroupCommandBufferBeginInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn device_mask(mut self, device_mask: u32) -> Self {
        self.0.device_mask = device_mask;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> DeviceGroupCommandBufferBeginInfo {
        self.0
    }
}
impl<'a> std::fmt::Debug for DeviceGroupCommandBufferBeginInfoBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for DeviceGroupCommandBufferBeginInfoBuilder<'a> {
    type Target = DeviceGroupCommandBufferBeginInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DeviceGroupCommandBufferBeginInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceGroupSubmitInfo.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DeviceGroupSubmitInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub wait_semaphore_count: u32,
    pub p_wait_semaphore_device_indices: *const u32,
    pub command_buffer_count: u32,
    pub p_command_buffer_device_masks: *const u32,
    pub signal_semaphore_count: u32,
    pub p_signal_semaphore_device_indices: *const u32,
}
impl DeviceGroupSubmitInfo {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByDeviceGroupSubmitInfo,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> DeviceGroupSubmitInfoBuilder<'a> {
        DeviceGroupSubmitInfoBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for DeviceGroupSubmitInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("DeviceGroupSubmitInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("wait_semaphore_count", &self.wait_semaphore_count)
            .field(
                "p_wait_semaphore_device_indices",
                &self.p_wait_semaphore_device_indices,
            )
            .field("command_buffer_count", &self.command_buffer_count)
            .field(
                "p_command_buffer_device_masks",
                &self.p_command_buffer_device_masks,
            )
            .field("signal_semaphore_count", &self.signal_semaphore_count)
            .field(
                "p_signal_semaphore_device_indices",
                &self.p_signal_semaphore_device_indices,
            )
            .finish()
    }
}
impl Default for DeviceGroupSubmitInfo {
    fn default() -> DeviceGroupSubmitInfo {
        DeviceGroupSubmitInfo {
            s_type: crate::vk1_0::StructureType::DEVICE_GROUP_SUBMIT_INFO,
            p_next: std::ptr::null(),
            wait_semaphore_count: Default::default(),
            p_wait_semaphore_device_indices: std::ptr::null(),
            command_buffer_count: Default::default(),
            p_command_buffer_device_masks: std::ptr::null(),
            signal_semaphore_count: Default::default(),
            p_signal_semaphore_device_indices: std::ptr::null(),
        }
    }
}
#[doc = "Used by [`DeviceGroupSubmitInfo::extend`](struct.DeviceGroupSubmitInfo.html#method.extend)"]
pub trait ExtendableByDeviceGroupSubmitInfo {}
impl ExtendableByDeviceGroupSubmitInfo for crate::vk1_0::SubmitInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`DeviceGroupSubmitInfo`](struct.DeviceGroupSubmitInfo.html)"]
#[repr(transparent)]
pub struct DeviceGroupSubmitInfoBuilder<'a>(
    DeviceGroupSubmitInfo,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> DeviceGroupSubmitInfoBuilder<'a> {
    #[inline]
    pub fn new() -> DeviceGroupSubmitInfoBuilder<'a> {
        DeviceGroupSubmitInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn wait_semaphore_device_indices(
        mut self,
        wait_semaphore_device_indices: &'a [u32],
    ) -> Self {
        self.0.wait_semaphore_count = wait_semaphore_device_indices.len() as _;
        self.0.p_wait_semaphore_device_indices = wait_semaphore_device_indices.as_ptr() as _;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn command_buffer_device_masks(mut self, command_buffer_device_masks: &'a [u32]) -> Self {
        self.0.command_buffer_count = command_buffer_device_masks.len() as _;
        self.0.p_command_buffer_device_masks = command_buffer_device_masks.as_ptr() as _;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn signal_semaphore_device_indices(
        mut self,
        signal_semaphore_device_indices: &'a [u32],
    ) -> Self {
        self.0.signal_semaphore_count = signal_semaphore_device_indices.len() as _;
        self.0.p_signal_semaphore_device_indices = signal_semaphore_device_indices.as_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> DeviceGroupSubmitInfo {
        self.0
    }
}
impl<'a> std::fmt::Debug for DeviceGroupSubmitInfoBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for DeviceGroupSubmitInfoBuilder<'a> {
    type Target = DeviceGroupSubmitInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DeviceGroupSubmitInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceGroupBindSparseInfo.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DeviceGroupBindSparseInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub resource_device_index: u32,
    pub memory_device_index: u32,
}
impl DeviceGroupBindSparseInfo {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByDeviceGroupBindSparseInfo,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> DeviceGroupBindSparseInfoBuilder<'a> {
        DeviceGroupBindSparseInfoBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for DeviceGroupBindSparseInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("DeviceGroupBindSparseInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("resource_device_index", &self.resource_device_index)
            .field("memory_device_index", &self.memory_device_index)
            .finish()
    }
}
impl Default for DeviceGroupBindSparseInfo {
    fn default() -> DeviceGroupBindSparseInfo {
        DeviceGroupBindSparseInfo {
            s_type: crate::vk1_0::StructureType::DEVICE_GROUP_BIND_SPARSE_INFO,
            p_next: std::ptr::null(),
            resource_device_index: Default::default(),
            memory_device_index: Default::default(),
        }
    }
}
#[doc = "Used by [`DeviceGroupBindSparseInfo::extend`](struct.DeviceGroupBindSparseInfo.html#method.extend)"]
pub trait ExtendableByDeviceGroupBindSparseInfo {}
impl ExtendableByDeviceGroupBindSparseInfo for crate::vk1_0::BindSparseInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`DeviceGroupBindSparseInfo`](struct.DeviceGroupBindSparseInfo.html)"]
#[repr(transparent)]
pub struct DeviceGroupBindSparseInfoBuilder<'a>(
    DeviceGroupBindSparseInfo,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> DeviceGroupBindSparseInfoBuilder<'a> {
    #[inline]
    pub fn new() -> DeviceGroupBindSparseInfoBuilder<'a> {
        DeviceGroupBindSparseInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn resource_device_index(mut self, resource_device_index: u32) -> Self {
        self.0.resource_device_index = resource_device_index;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn memory_device_index(mut self, memory_device_index: u32) -> Self {
        self.0.memory_device_index = memory_device_index;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> DeviceGroupBindSparseInfo {
        self.0
    }
}
impl<'a> std::fmt::Debug for DeviceGroupBindSparseInfoBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for DeviceGroupBindSparseInfoBuilder<'a> {
    type Target = DeviceGroupBindSparseInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DeviceGroupBindSparseInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindBufferMemoryDeviceGroupInfo.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BindBufferMemoryDeviceGroupInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub device_index_count: u32,
    pub p_device_indices: *const u32,
}
impl BindBufferMemoryDeviceGroupInfo {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByBindBufferMemoryDeviceGroupInfo,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> BindBufferMemoryDeviceGroupInfoBuilder<'a> {
        BindBufferMemoryDeviceGroupInfoBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for BindBufferMemoryDeviceGroupInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("BindBufferMemoryDeviceGroupInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("device_index_count", &self.device_index_count)
            .field("p_device_indices", &self.p_device_indices)
            .finish()
    }
}
impl Default for BindBufferMemoryDeviceGroupInfo {
    fn default() -> BindBufferMemoryDeviceGroupInfo {
        BindBufferMemoryDeviceGroupInfo {
            s_type: crate::vk1_0::StructureType::BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO,
            p_next: std::ptr::null(),
            device_index_count: Default::default(),
            p_device_indices: std::ptr::null(),
        }
    }
}
#[doc = "Used by [`BindBufferMemoryDeviceGroupInfo::extend`](struct.BindBufferMemoryDeviceGroupInfo.html#method.extend)"]
pub trait ExtendableByBindBufferMemoryDeviceGroupInfo {}
impl ExtendableByBindBufferMemoryDeviceGroupInfo for crate::vk1_1::BindBufferMemoryInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`BindBufferMemoryDeviceGroupInfo`](struct.BindBufferMemoryDeviceGroupInfo.html)"]
#[repr(transparent)]
pub struct BindBufferMemoryDeviceGroupInfoBuilder<'a>(
    BindBufferMemoryDeviceGroupInfo,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> BindBufferMemoryDeviceGroupInfoBuilder<'a> {
    #[inline]
    pub fn new() -> BindBufferMemoryDeviceGroupInfoBuilder<'a> {
        BindBufferMemoryDeviceGroupInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn device_indices(mut self, device_indices: &'a [u32]) -> Self {
        self.0.device_index_count = device_indices.len() as _;
        self.0.p_device_indices = device_indices.as_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> BindBufferMemoryDeviceGroupInfo {
        self.0
    }
}
impl<'a> std::fmt::Debug for BindBufferMemoryDeviceGroupInfoBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for BindBufferMemoryDeviceGroupInfoBuilder<'a> {
    type Target = BindBufferMemoryDeviceGroupInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for BindBufferMemoryDeviceGroupInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindImageMemoryDeviceGroupInfo.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BindImageMemoryDeviceGroupInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub device_index_count: u32,
    pub p_device_indices: *const u32,
    pub split_instance_bind_region_count: u32,
    pub p_split_instance_bind_regions: *const crate::vk1_0::Rect2D,
}
impl BindImageMemoryDeviceGroupInfo {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByBindImageMemoryDeviceGroupInfo,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> BindImageMemoryDeviceGroupInfoBuilder<'a> {
        BindImageMemoryDeviceGroupInfoBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for BindImageMemoryDeviceGroupInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("BindImageMemoryDeviceGroupInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("device_index_count", &self.device_index_count)
            .field("p_device_indices", &self.p_device_indices)
            .field(
                "split_instance_bind_region_count",
                &self.split_instance_bind_region_count,
            )
            .field(
                "p_split_instance_bind_regions",
                &self.p_split_instance_bind_regions,
            )
            .finish()
    }
}
impl Default for BindImageMemoryDeviceGroupInfo {
    fn default() -> BindImageMemoryDeviceGroupInfo {
        BindImageMemoryDeviceGroupInfo {
            s_type: crate::vk1_0::StructureType::BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO,
            p_next: std::ptr::null(),
            device_index_count: Default::default(),
            p_device_indices: std::ptr::null(),
            split_instance_bind_region_count: Default::default(),
            p_split_instance_bind_regions: std::ptr::null(),
        }
    }
}
#[doc = "Used by [`BindImageMemoryDeviceGroupInfo::extend`](struct.BindImageMemoryDeviceGroupInfo.html#method.extend)"]
pub trait ExtendableByBindImageMemoryDeviceGroupInfo {}
impl ExtendableByBindImageMemoryDeviceGroupInfo for crate::vk1_1::BindImageMemoryInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`BindImageMemoryDeviceGroupInfo`](struct.BindImageMemoryDeviceGroupInfo.html)"]
#[repr(transparent)]
pub struct BindImageMemoryDeviceGroupInfoBuilder<'a>(
    BindImageMemoryDeviceGroupInfo,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> BindImageMemoryDeviceGroupInfoBuilder<'a> {
    #[inline]
    pub fn new() -> BindImageMemoryDeviceGroupInfoBuilder<'a> {
        BindImageMemoryDeviceGroupInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn device_indices(mut self, device_indices: &'a [u32]) -> Self {
        self.0.device_index_count = device_indices.len() as _;
        self.0.p_device_indices = device_indices.as_ptr() as _;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn split_instance_bind_regions(
        mut self,
        split_instance_bind_regions: &'a [crate::vk1_0::Rect2DBuilder],
    ) -> Self {
        self.0.split_instance_bind_region_count = split_instance_bind_regions.len() as _;
        self.0.p_split_instance_bind_regions = split_instance_bind_regions.as_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> BindImageMemoryDeviceGroupInfo {
        self.0
    }
}
impl<'a> std::fmt::Debug for BindImageMemoryDeviceGroupInfoBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for BindImageMemoryDeviceGroupInfoBuilder<'a> {
    type Target = BindImageMemoryDeviceGroupInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for BindImageMemoryDeviceGroupInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceGroupDeviceCreateInfo.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DeviceGroupDeviceCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub physical_device_count: u32,
    pub p_physical_devices: *const crate::vk1_0::PhysicalDevice,
}
impl DeviceGroupDeviceCreateInfo {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByDeviceGroupDeviceCreateInfo,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> DeviceGroupDeviceCreateInfoBuilder<'a> {
        DeviceGroupDeviceCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for DeviceGroupDeviceCreateInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("DeviceGroupDeviceCreateInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("physical_device_count", &self.physical_device_count)
            .field("p_physical_devices", &self.p_physical_devices)
            .finish()
    }
}
impl Default for DeviceGroupDeviceCreateInfo {
    fn default() -> DeviceGroupDeviceCreateInfo {
        DeviceGroupDeviceCreateInfo {
            s_type: crate::vk1_0::StructureType::DEVICE_GROUP_DEVICE_CREATE_INFO,
            p_next: std::ptr::null(),
            physical_device_count: Default::default(),
            p_physical_devices: std::ptr::null(),
        }
    }
}
#[doc = "Used by [`DeviceGroupDeviceCreateInfo::extend`](struct.DeviceGroupDeviceCreateInfo.html#method.extend)"]
pub trait ExtendableByDeviceGroupDeviceCreateInfo {}
impl ExtendableByDeviceGroupDeviceCreateInfo for crate::vk1_0::DeviceCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`DeviceGroupDeviceCreateInfo`](struct.DeviceGroupDeviceCreateInfo.html)"]
#[repr(transparent)]
pub struct DeviceGroupDeviceCreateInfoBuilder<'a>(
    DeviceGroupDeviceCreateInfo,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> DeviceGroupDeviceCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> DeviceGroupDeviceCreateInfoBuilder<'a> {
        DeviceGroupDeviceCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn physical_devices(
        mut self,
        physical_devices: &'a [crate::vk1_0::PhysicalDevice],
    ) -> Self {
        self.0.physical_device_count = physical_devices.len() as _;
        self.0.p_physical_devices = physical_devices.as_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> DeviceGroupDeviceCreateInfo {
        self.0
    }
}
impl<'a> std::fmt::Debug for DeviceGroupDeviceCreateInfoBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for DeviceGroupDeviceCreateInfoBuilder<'a> {
    type Target = DeviceGroupDeviceCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DeviceGroupDeviceCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryRequirements2KHR.html) · Alias"]
pub type MemoryRequirements2KHR = crate::vk1_1::MemoryRequirements2;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevicePointClippingProperties.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDevicePointClippingProperties {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub point_clipping_behavior: crate::vk1_1::PointClippingBehavior,
}
impl PhysicalDevicePointClippingProperties {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDevicePointClippingProperties,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDevicePointClippingPropertiesBuilder<'a> {
        PhysicalDevicePointClippingPropertiesBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDevicePointClippingProperties {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDevicePointClippingProperties")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("point_clipping_behavior", &self.point_clipping_behavior)
            .finish()
    }
}
impl Default for PhysicalDevicePointClippingProperties {
    fn default() -> PhysicalDevicePointClippingProperties {
        PhysicalDevicePointClippingProperties {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES,
            p_next: std::ptr::null_mut(),
            point_clipping_behavior: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDevicePointClippingProperties::extend`](struct.PhysicalDevicePointClippingProperties.html#method.extend)"]
pub trait ExtendableByPhysicalDevicePointClippingProperties {}
impl ExtendableByPhysicalDevicePointClippingProperties for crate::vk1_1::PhysicalDeviceProperties2 {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDevicePointClippingProperties`](struct.PhysicalDevicePointClippingProperties.html)"]
#[repr(transparent)]
pub struct PhysicalDevicePointClippingPropertiesBuilder<'a>(
    PhysicalDevicePointClippingProperties,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDevicePointClippingPropertiesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDevicePointClippingPropertiesBuilder<'a> {
        PhysicalDevicePointClippingPropertiesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn point_clipping_behavior(
        mut self,
        point_clipping_behavior: crate::vk1_1::PointClippingBehavior,
    ) -> Self {
        self.0.point_clipping_behavior = point_clipping_behavior;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDevicePointClippingProperties {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDevicePointClippingPropertiesBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDevicePointClippingPropertiesBuilder<'a> {
    type Target = PhysicalDevicePointClippingProperties;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDevicePointClippingPropertiesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPointClippingBehavior.html) · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PointClippingBehavior(pub i32);
#[doc = "[Part of `vk1_1`](../vk1_1/index.html)"]
impl PointClippingBehavior {
    pub const ALL_CLIP_PLANES: Self = Self(0);
    pub const USER_CLIP_PLANES_ONLY: Self = Self(1);
}
#[doc = "[Part of `extensions::khr_maintenance2`](../extensions/khr_maintenance2/index.html)"]
impl PointClippingBehavior {
    pub const ALL_CLIP_PLANES_KHR: Self = Self::ALL_CLIP_PLANES;
    pub const USER_CLIP_PLANES_ONLY_KHR: Self = Self::USER_CLIP_PLANES_ONLY;
}
impl std::fmt::Debug for PointClippingBehavior {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::ALL_CLIP_PLANES => "ALL_CLIP_PLANES",
            &Self::USER_CLIP_PLANES_ONLY => "USER_CLIP_PLANES_ONLY",
            _ => "(Unknown)",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderPassInputAttachmentAspectCreateInfo.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RenderPassInputAttachmentAspectCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub aspect_reference_count: u32,
    pub p_aspect_references: *const crate::vk1_1::InputAttachmentAspectReference,
}
impl RenderPassInputAttachmentAspectCreateInfo {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByRenderPassInputAttachmentAspectCreateInfo,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> RenderPassInputAttachmentAspectCreateInfoBuilder<'a> {
        RenderPassInputAttachmentAspectCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for RenderPassInputAttachmentAspectCreateInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("RenderPassInputAttachmentAspectCreateInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("aspect_reference_count", &self.aspect_reference_count)
            .field("p_aspect_references", &self.p_aspect_references)
            .finish()
    }
}
impl Default for RenderPassInputAttachmentAspectCreateInfo {
    fn default() -> RenderPassInputAttachmentAspectCreateInfo {
        RenderPassInputAttachmentAspectCreateInfo {
            s_type: crate::vk1_0::StructureType::RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO,
            p_next: std::ptr::null(),
            aspect_reference_count: Default::default(),
            p_aspect_references: std::ptr::null(),
        }
    }
}
#[doc = "Used by [`RenderPassInputAttachmentAspectCreateInfo::extend`](struct.RenderPassInputAttachmentAspectCreateInfo.html#method.extend)"]
pub trait ExtendableByRenderPassInputAttachmentAspectCreateInfo {}
impl ExtendableByRenderPassInputAttachmentAspectCreateInfo for crate::vk1_0::RenderPassCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`RenderPassInputAttachmentAspectCreateInfo`](struct.RenderPassInputAttachmentAspectCreateInfo.html)"]
#[repr(transparent)]
pub struct RenderPassInputAttachmentAspectCreateInfoBuilder<'a>(
    RenderPassInputAttachmentAspectCreateInfo,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> RenderPassInputAttachmentAspectCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> RenderPassInputAttachmentAspectCreateInfoBuilder<'a> {
        RenderPassInputAttachmentAspectCreateInfoBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn aspect_references(
        mut self,
        aspect_references: &'a [crate::vk1_1::InputAttachmentAspectReferenceBuilder],
    ) -> Self {
        self.0.aspect_reference_count = aspect_references.len() as _;
        self.0.p_aspect_references = aspect_references.as_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> RenderPassInputAttachmentAspectCreateInfo {
        self.0
    }
}
impl<'a> std::fmt::Debug for RenderPassInputAttachmentAspectCreateInfoBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for RenderPassInputAttachmentAspectCreateInfoBuilder<'a> {
    type Target = RenderPassInputAttachmentAspectCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for RenderPassInputAttachmentAspectCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkInputAttachmentAspectReference.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InputAttachmentAspectReference {
    pub subpass: u32,
    pub input_attachment_index: u32,
    pub aspect_mask: crate::vk1_0::ImageAspectFlags,
}
impl InputAttachmentAspectReference {
    #[inline]
    pub fn builder<'a>(self) -> InputAttachmentAspectReferenceBuilder<'a> {
        InputAttachmentAspectReferenceBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for InputAttachmentAspectReference {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("InputAttachmentAspectReference")
            .field("subpass", &self.subpass)
            .field("input_attachment_index", &self.input_attachment_index)
            .field("aspect_mask", &self.aspect_mask)
            .finish()
    }
}
impl Default for InputAttachmentAspectReference {
    fn default() -> InputAttachmentAspectReference {
        InputAttachmentAspectReference {
            subpass: Default::default(),
            input_attachment_index: Default::default(),
            aspect_mask: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`InputAttachmentAspectReference`](struct.InputAttachmentAspectReference.html)"]
#[repr(transparent)]
pub struct InputAttachmentAspectReferenceBuilder<'a>(
    InputAttachmentAspectReference,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> InputAttachmentAspectReferenceBuilder<'a> {
    #[inline]
    pub fn new() -> InputAttachmentAspectReferenceBuilder<'a> {
        InputAttachmentAspectReferenceBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn subpass(mut self, subpass: u32) -> Self {
        self.0.subpass = subpass;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn input_attachment_index(mut self, input_attachment_index: u32) -> Self {
        self.0.input_attachment_index = input_attachment_index;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn aspect_mask(mut self, aspect_mask: crate::vk1_0::ImageAspectFlags) -> Self {
        self.0.aspect_mask = aspect_mask;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> InputAttachmentAspectReference {
        self.0
    }
}
impl<'a> std::fmt::Debug for InputAttachmentAspectReferenceBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for InputAttachmentAspectReferenceBuilder<'a> {
    type Target = InputAttachmentAspectReference;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for InputAttachmentAspectReferenceBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageViewUsageCreateInfo.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImageViewUsageCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub usage: crate::vk1_0::ImageUsageFlags,
}
impl ImageViewUsageCreateInfo {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByImageViewUsageCreateInfo,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> ImageViewUsageCreateInfoBuilder<'a> {
        ImageViewUsageCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ImageViewUsageCreateInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ImageViewUsageCreateInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("usage", &self.usage)
            .finish()
    }
}
impl Default for ImageViewUsageCreateInfo {
    fn default() -> ImageViewUsageCreateInfo {
        ImageViewUsageCreateInfo {
            s_type: crate::vk1_0::StructureType::IMAGE_VIEW_USAGE_CREATE_INFO,
            p_next: std::ptr::null(),
            usage: Default::default(),
        }
    }
}
#[doc = "Used by [`ImageViewUsageCreateInfo::extend`](struct.ImageViewUsageCreateInfo.html#method.extend)"]
pub trait ExtendableByImageViewUsageCreateInfo {}
impl ExtendableByImageViewUsageCreateInfo for crate::vk1_0::ImageViewCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`ImageViewUsageCreateInfo`](struct.ImageViewUsageCreateInfo.html)"]
#[repr(transparent)]
pub struct ImageViewUsageCreateInfoBuilder<'a>(
    ImageViewUsageCreateInfo,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> ImageViewUsageCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> ImageViewUsageCreateInfoBuilder<'a> {
        ImageViewUsageCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn usage(mut self, usage: crate::vk1_0::ImageUsageFlags) -> Self {
        self.0.usage = usage;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> ImageViewUsageCreateInfo {
        self.0
    }
}
impl<'a> std::fmt::Debug for ImageViewUsageCreateInfoBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for ImageViewUsageCreateInfoBuilder<'a> {
    type Target = ImageViewUsageCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ImageViewUsageCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkTessellationDomainOrigin.html) · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct TessellationDomainOrigin(pub i32);
#[doc = "[Part of `vk1_1`](../vk1_1/index.html)"]
impl TessellationDomainOrigin {
    pub const UPPER_LEFT: Self = Self(0);
    pub const LOWER_LEFT: Self = Self(1);
}
#[doc = "[Part of `extensions::khr_maintenance2`](../extensions/khr_maintenance2/index.html)"]
impl TessellationDomainOrigin {
    pub const UPPER_LEFT_KHR: Self = Self::UPPER_LEFT;
    pub const LOWER_LEFT_KHR: Self = Self::LOWER_LEFT;
}
impl std::fmt::Debug for TessellationDomainOrigin {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::UPPER_LEFT => "UPPER_LEFT",
            &Self::LOWER_LEFT => "LOWER_LEFT",
            _ => "(Unknown)",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineTessellationDomainOriginStateCreateInfo.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineTessellationDomainOriginStateCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub domain_origin: crate::vk1_1::TessellationDomainOrigin,
}
impl PipelineTessellationDomainOriginStateCreateInfo {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPipelineTessellationDomainOriginStateCreateInfo,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PipelineTessellationDomainOriginStateCreateInfoBuilder<'a> {
        PipelineTessellationDomainOriginStateCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PipelineTessellationDomainOriginStateCreateInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PipelineTessellationDomainOriginStateCreateInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("domain_origin", &self.domain_origin)
            .finish()
    }
}
impl Default for PipelineTessellationDomainOriginStateCreateInfo {
    fn default() -> PipelineTessellationDomainOriginStateCreateInfo {
        PipelineTessellationDomainOriginStateCreateInfo {
            s_type:
                crate::vk1_0::StructureType::PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO,
            p_next: std::ptr::null(),
            domain_origin: Default::default(),
        }
    }
}
#[doc = "Used by [`PipelineTessellationDomainOriginStateCreateInfo::extend`](struct.PipelineTessellationDomainOriginStateCreateInfo.html#method.extend)"]
pub trait ExtendableByPipelineTessellationDomainOriginStateCreateInfo {}
impl ExtendableByPipelineTessellationDomainOriginStateCreateInfo
    for crate::vk1_0::PipelineTessellationStateCreateInfo
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PipelineTessellationDomainOriginStateCreateInfo`](struct.PipelineTessellationDomainOriginStateCreateInfo.html)"]
#[repr(transparent)]
pub struct PipelineTessellationDomainOriginStateCreateInfoBuilder<'a>(
    PipelineTessellationDomainOriginStateCreateInfo,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PipelineTessellationDomainOriginStateCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineTessellationDomainOriginStateCreateInfoBuilder<'a> {
        PipelineTessellationDomainOriginStateCreateInfoBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn domain_origin(mut self, domain_origin: crate::vk1_1::TessellationDomainOrigin) -> Self {
        self.0.domain_origin = domain_origin;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PipelineTessellationDomainOriginStateCreateInfo {
        self.0
    }
}
impl<'a> std::fmt::Debug for PipelineTessellationDomainOriginStateCreateInfoBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PipelineTessellationDomainOriginStateCreateInfoBuilder<'a> {
    type Target = PipelineTessellationDomainOriginStateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineTessellationDomainOriginStateCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderPassMultiviewCreateInfo.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RenderPassMultiviewCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub subpass_count: u32,
    pub p_view_masks: *const u32,
    pub dependency_count: u32,
    pub p_view_offsets: *const i32,
    pub correlation_mask_count: u32,
    pub p_correlation_masks: *const u32,
}
impl RenderPassMultiviewCreateInfo {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByRenderPassMultiviewCreateInfo,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> RenderPassMultiviewCreateInfoBuilder<'a> {
        RenderPassMultiviewCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for RenderPassMultiviewCreateInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("RenderPassMultiviewCreateInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("subpass_count", &self.subpass_count)
            .field("p_view_masks", &self.p_view_masks)
            .field("dependency_count", &self.dependency_count)
            .field("p_view_offsets", &self.p_view_offsets)
            .field("correlation_mask_count", &self.correlation_mask_count)
            .field("p_correlation_masks", &self.p_correlation_masks)
            .finish()
    }
}
impl Default for RenderPassMultiviewCreateInfo {
    fn default() -> RenderPassMultiviewCreateInfo {
        RenderPassMultiviewCreateInfo {
            s_type: crate::vk1_0::StructureType::RENDER_PASS_MULTIVIEW_CREATE_INFO,
            p_next: std::ptr::null(),
            subpass_count: Default::default(),
            p_view_masks: std::ptr::null(),
            dependency_count: Default::default(),
            p_view_offsets: std::ptr::null(),
            correlation_mask_count: Default::default(),
            p_correlation_masks: std::ptr::null(),
        }
    }
}
#[doc = "Used by [`RenderPassMultiviewCreateInfo::extend`](struct.RenderPassMultiviewCreateInfo.html#method.extend)"]
pub trait ExtendableByRenderPassMultiviewCreateInfo {}
impl ExtendableByRenderPassMultiviewCreateInfo for crate::vk1_0::RenderPassCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`RenderPassMultiviewCreateInfo`](struct.RenderPassMultiviewCreateInfo.html)"]
#[repr(transparent)]
pub struct RenderPassMultiviewCreateInfoBuilder<'a>(
    RenderPassMultiviewCreateInfo,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> RenderPassMultiviewCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> RenderPassMultiviewCreateInfoBuilder<'a> {
        RenderPassMultiviewCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn view_masks(mut self, view_masks: &'a [u32]) -> Self {
        self.0.subpass_count = view_masks.len() as _;
        self.0.p_view_masks = view_masks.as_ptr() as _;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn view_offsets(mut self, view_offsets: &'a [i32]) -> Self {
        self.0.dependency_count = view_offsets.len() as _;
        self.0.p_view_offsets = view_offsets.as_ptr() as _;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn correlation_masks(mut self, correlation_masks: &'a [u32]) -> Self {
        self.0.correlation_mask_count = correlation_masks.len() as _;
        self.0.p_correlation_masks = correlation_masks.as_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> RenderPassMultiviewCreateInfo {
        self.0
    }
}
impl<'a> std::fmt::Debug for RenderPassMultiviewCreateInfoBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for RenderPassMultiviewCreateInfoBuilder<'a> {
    type Target = RenderPassMultiviewCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for RenderPassMultiviewCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMultiviewFeatures.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceMultiviewFeatures {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub multiview: crate::vk1_0::Bool32,
    pub multiview_geometry_shader: crate::vk1_0::Bool32,
    pub multiview_tessellation_shader: crate::vk1_0::Bool32,
}
impl PhysicalDeviceMultiviewFeatures {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceMultiviewFeatures,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceMultiviewFeaturesBuilder<'a> {
        PhysicalDeviceMultiviewFeaturesBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceMultiviewFeatures {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceMultiviewFeatures")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("multiview", &(self.multiview != 0))
            .field(
                "multiview_geometry_shader",
                &(self.multiview_geometry_shader != 0),
            )
            .field(
                "multiview_tessellation_shader",
                &(self.multiview_tessellation_shader != 0),
            )
            .finish()
    }
}
impl Default for PhysicalDeviceMultiviewFeatures {
    fn default() -> PhysicalDeviceMultiviewFeatures {
        PhysicalDeviceMultiviewFeatures {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_MULTIVIEW_FEATURES,
            p_next: std::ptr::null_mut(),
            multiview: Default::default(),
            multiview_geometry_shader: Default::default(),
            multiview_tessellation_shader: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceMultiviewFeatures::extend`](struct.PhysicalDeviceMultiviewFeatures.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceMultiviewFeatures {}
impl ExtendableByPhysicalDeviceMultiviewFeatures for crate::vk1_1::PhysicalDeviceFeatures2 {}
impl ExtendableByPhysicalDeviceMultiviewFeatures for crate::vk1_0::DeviceCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceMultiviewFeatures`](struct.PhysicalDeviceMultiviewFeatures.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceMultiviewFeaturesBuilder<'a>(
    PhysicalDeviceMultiviewFeatures,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceMultiviewFeaturesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceMultiviewFeaturesBuilder<'a> {
        PhysicalDeviceMultiviewFeaturesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn multiview(mut self, multiview: bool) -> Self {
        self.0.multiview = multiview as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn multiview_geometry_shader(mut self, multiview_geometry_shader: bool) -> Self {
        self.0.multiview_geometry_shader = multiview_geometry_shader as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn multiview_tessellation_shader(mut self, multiview_tessellation_shader: bool) -> Self {
        self.0.multiview_tessellation_shader = multiview_tessellation_shader as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceMultiviewFeatures {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceMultiviewFeaturesBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceMultiviewFeaturesBuilder<'a> {
    type Target = PhysicalDeviceMultiviewFeatures;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceMultiviewFeaturesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMultiviewProperties.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceMultiviewProperties {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub max_multiview_view_count: u32,
    pub max_multiview_instance_index: u32,
}
impl PhysicalDeviceMultiviewProperties {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceMultiviewProperties,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceMultiviewPropertiesBuilder<'a> {
        PhysicalDeviceMultiviewPropertiesBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceMultiviewProperties {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceMultiviewProperties")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("max_multiview_view_count", &self.max_multiview_view_count)
            .field(
                "max_multiview_instance_index",
                &self.max_multiview_instance_index,
            )
            .finish()
    }
}
impl Default for PhysicalDeviceMultiviewProperties {
    fn default() -> PhysicalDeviceMultiviewProperties {
        PhysicalDeviceMultiviewProperties {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES,
            p_next: std::ptr::null_mut(),
            max_multiview_view_count: Default::default(),
            max_multiview_instance_index: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceMultiviewProperties::extend`](struct.PhysicalDeviceMultiviewProperties.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceMultiviewProperties {}
impl ExtendableByPhysicalDeviceMultiviewProperties for crate::vk1_1::PhysicalDeviceProperties2 {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceMultiviewProperties`](struct.PhysicalDeviceMultiviewProperties.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceMultiviewPropertiesBuilder<'a>(
    PhysicalDeviceMultiviewProperties,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceMultiviewPropertiesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceMultiviewPropertiesBuilder<'a> {
        PhysicalDeviceMultiviewPropertiesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_multiview_view_count(mut self, max_multiview_view_count: u32) -> Self {
        self.0.max_multiview_view_count = max_multiview_view_count;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_multiview_instance_index(mut self, max_multiview_instance_index: u32) -> Self {
        self.0.max_multiview_instance_index = max_multiview_instance_index;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceMultiviewProperties {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceMultiviewPropertiesBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceMultiviewPropertiesBuilder<'a> {
    type Target = PhysicalDeviceMultiviewProperties;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceMultiviewPropertiesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceVariablePointerFeatures.html) · Alias"]
pub type PhysicalDeviceVariablePointerFeatures =
    crate::vk1_1::PhysicalDeviceVariablePointersFeatures;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceVariablePointersFeatures.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceVariablePointersFeatures {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub variable_pointers_storage_buffer: crate::vk1_0::Bool32,
    pub variable_pointers: crate::vk1_0::Bool32,
}
impl PhysicalDeviceVariablePointersFeatures {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceVariablePointersFeatures,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceVariablePointersFeaturesBuilder<'a> {
        PhysicalDeviceVariablePointersFeaturesBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceVariablePointersFeatures {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceVariablePointersFeatures")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "variable_pointers_storage_buffer",
                &(self.variable_pointers_storage_buffer != 0),
            )
            .field("variable_pointers", &(self.variable_pointers != 0))
            .finish()
    }
}
impl Default for PhysicalDeviceVariablePointersFeatures {
    fn default() -> PhysicalDeviceVariablePointersFeatures {
        PhysicalDeviceVariablePointersFeatures {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES,
            p_next: std::ptr::null_mut(),
            variable_pointers_storage_buffer: Default::default(),
            variable_pointers: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceVariablePointersFeatures::extend`](struct.PhysicalDeviceVariablePointersFeatures.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceVariablePointersFeatures {}
impl ExtendableByPhysicalDeviceVariablePointersFeatures for crate::vk1_1::PhysicalDeviceFeatures2 {}
impl ExtendableByPhysicalDeviceVariablePointersFeatures for crate::vk1_0::DeviceCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceVariablePointersFeatures`](struct.PhysicalDeviceVariablePointersFeatures.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceVariablePointersFeaturesBuilder<'a>(
    PhysicalDeviceVariablePointersFeatures,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceVariablePointersFeaturesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceVariablePointersFeaturesBuilder<'a> {
        PhysicalDeviceVariablePointersFeaturesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn variable_pointers_storage_buffer(
        mut self,
        variable_pointers_storage_buffer: bool,
    ) -> Self {
        self.0.variable_pointers_storage_buffer = variable_pointers_storage_buffer as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn variable_pointers(mut self, variable_pointers: bool) -> Self {
        self.0.variable_pointers = variable_pointers as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceVariablePointersFeatures {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceVariablePointersFeaturesBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceVariablePointersFeaturesBuilder<'a> {
    type Target = PhysicalDeviceVariablePointersFeatures;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceVariablePointersFeaturesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceProtectedMemoryFeatures.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceProtectedMemoryFeatures {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub protected_memory: crate::vk1_0::Bool32,
}
impl PhysicalDeviceProtectedMemoryFeatures {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceProtectedMemoryFeatures,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceProtectedMemoryFeaturesBuilder<'a> {
        PhysicalDeviceProtectedMemoryFeaturesBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceProtectedMemoryFeatures {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceProtectedMemoryFeatures")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("protected_memory", &(self.protected_memory != 0))
            .finish()
    }
}
impl Default for PhysicalDeviceProtectedMemoryFeatures {
    fn default() -> PhysicalDeviceProtectedMemoryFeatures {
        PhysicalDeviceProtectedMemoryFeatures {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES,
            p_next: std::ptr::null_mut(),
            protected_memory: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceProtectedMemoryFeatures::extend`](struct.PhysicalDeviceProtectedMemoryFeatures.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceProtectedMemoryFeatures {}
impl ExtendableByPhysicalDeviceProtectedMemoryFeatures for crate::vk1_1::PhysicalDeviceFeatures2 {}
impl ExtendableByPhysicalDeviceProtectedMemoryFeatures for crate::vk1_0::DeviceCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceProtectedMemoryFeatures`](struct.PhysicalDeviceProtectedMemoryFeatures.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceProtectedMemoryFeaturesBuilder<'a>(
    PhysicalDeviceProtectedMemoryFeatures,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceProtectedMemoryFeaturesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceProtectedMemoryFeaturesBuilder<'a> {
        PhysicalDeviceProtectedMemoryFeaturesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn protected_memory(mut self, protected_memory: bool) -> Self {
        self.0.protected_memory = protected_memory as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceProtectedMemoryFeatures {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceProtectedMemoryFeaturesBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceProtectedMemoryFeaturesBuilder<'a> {
    type Target = PhysicalDeviceProtectedMemoryFeatures;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceProtectedMemoryFeaturesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceProtectedMemoryProperties.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceProtectedMemoryProperties {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub protected_no_fault: crate::vk1_0::Bool32,
}
impl PhysicalDeviceProtectedMemoryProperties {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceProtectedMemoryProperties,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceProtectedMemoryPropertiesBuilder<'a> {
        PhysicalDeviceProtectedMemoryPropertiesBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceProtectedMemoryProperties {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceProtectedMemoryProperties")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("protected_no_fault", &(self.protected_no_fault != 0))
            .finish()
    }
}
impl Default for PhysicalDeviceProtectedMemoryProperties {
    fn default() -> PhysicalDeviceProtectedMemoryProperties {
        PhysicalDeviceProtectedMemoryProperties {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES,
            p_next: std::ptr::null_mut(),
            protected_no_fault: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceProtectedMemoryProperties::extend`](struct.PhysicalDeviceProtectedMemoryProperties.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceProtectedMemoryProperties {}
impl ExtendableByPhysicalDeviceProtectedMemoryProperties
    for crate::vk1_1::PhysicalDeviceProperties2
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceProtectedMemoryProperties`](struct.PhysicalDeviceProtectedMemoryProperties.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceProtectedMemoryPropertiesBuilder<'a>(
    PhysicalDeviceProtectedMemoryProperties,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceProtectedMemoryPropertiesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceProtectedMemoryPropertiesBuilder<'a> {
        PhysicalDeviceProtectedMemoryPropertiesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn protected_no_fault(mut self, protected_no_fault: bool) -> Self {
        self.0.protected_no_fault = protected_no_fault as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceProtectedMemoryProperties {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceProtectedMemoryPropertiesBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceProtectedMemoryPropertiesBuilder<'a> {
    type Target = PhysicalDeviceProtectedMemoryProperties;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceProtectedMemoryPropertiesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkProtectedSubmitInfo.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ProtectedSubmitInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub protected_submit: crate::vk1_0::Bool32,
}
impl ProtectedSubmitInfo {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByProtectedSubmitInfo,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> ProtectedSubmitInfoBuilder<'a> {
        ProtectedSubmitInfoBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ProtectedSubmitInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ProtectedSubmitInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("protected_submit", &(self.protected_submit != 0))
            .finish()
    }
}
impl Default for ProtectedSubmitInfo {
    fn default() -> ProtectedSubmitInfo {
        ProtectedSubmitInfo {
            s_type: crate::vk1_0::StructureType::PROTECTED_SUBMIT_INFO,
            p_next: std::ptr::null(),
            protected_submit: Default::default(),
        }
    }
}
#[doc = "Used by [`ProtectedSubmitInfo::extend`](struct.ProtectedSubmitInfo.html#method.extend)"]
pub trait ExtendableByProtectedSubmitInfo {}
impl ExtendableByProtectedSubmitInfo for crate::vk1_0::SubmitInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`ProtectedSubmitInfo`](struct.ProtectedSubmitInfo.html)"]
#[repr(transparent)]
pub struct ProtectedSubmitInfoBuilder<'a>(ProtectedSubmitInfo, std::marker::PhantomData<&'a ()>);
impl<'a> ProtectedSubmitInfoBuilder<'a> {
    #[inline]
    pub fn new() -> ProtectedSubmitInfoBuilder<'a> {
        ProtectedSubmitInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn protected_submit(mut self, protected_submit: bool) -> Self {
        self.0.protected_submit = protected_submit as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> ProtectedSubmitInfo {
        self.0
    }
}
impl<'a> std::fmt::Debug for ProtectedSubmitInfoBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for ProtectedSubmitInfoBuilder<'a> {
    type Target = ProtectedSubmitInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ProtectedSubmitInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerYcbcrConversionInfo.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SamplerYcbcrConversionInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub conversion: crate::vk1_1::SamplerYcbcrConversion,
}
impl SamplerYcbcrConversionInfo {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableBySamplerYcbcrConversionInfo,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> SamplerYcbcrConversionInfoBuilder<'a> {
        SamplerYcbcrConversionInfoBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for SamplerYcbcrConversionInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("SamplerYcbcrConversionInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("conversion", &self.conversion)
            .finish()
    }
}
impl Default for SamplerYcbcrConversionInfo {
    fn default() -> SamplerYcbcrConversionInfo {
        SamplerYcbcrConversionInfo {
            s_type: crate::vk1_0::StructureType::SAMPLER_YCBCR_CONVERSION_INFO,
            p_next: std::ptr::null(),
            conversion: Default::default(),
        }
    }
}
#[doc = "Used by [`SamplerYcbcrConversionInfo::extend`](struct.SamplerYcbcrConversionInfo.html#method.extend)"]
pub trait ExtendableBySamplerYcbcrConversionInfo {}
impl ExtendableBySamplerYcbcrConversionInfo for crate::vk1_0::SamplerCreateInfo {}
impl ExtendableBySamplerYcbcrConversionInfo for crate::vk1_0::ImageViewCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`SamplerYcbcrConversionInfo`](struct.SamplerYcbcrConversionInfo.html)"]
#[repr(transparent)]
pub struct SamplerYcbcrConversionInfoBuilder<'a>(
    SamplerYcbcrConversionInfo,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> SamplerYcbcrConversionInfoBuilder<'a> {
    #[inline]
    pub fn new() -> SamplerYcbcrConversionInfoBuilder<'a> {
        SamplerYcbcrConversionInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn conversion(mut self, conversion: crate::vk1_1::SamplerYcbcrConversion) -> Self {
        self.0.conversion = conversion;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> SamplerYcbcrConversionInfo {
        self.0
    }
}
impl<'a> std::fmt::Debug for SamplerYcbcrConversionInfoBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for SamplerYcbcrConversionInfoBuilder<'a> {
    type Target = SamplerYcbcrConversionInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SamplerYcbcrConversionInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindImagePlaneMemoryInfo.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BindImagePlaneMemoryInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub plane_aspect: crate::vk1_0::ImageAspectFlagBits,
}
impl BindImagePlaneMemoryInfo {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByBindImagePlaneMemoryInfo,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> BindImagePlaneMemoryInfoBuilder<'a> {
        BindImagePlaneMemoryInfoBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for BindImagePlaneMemoryInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("BindImagePlaneMemoryInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("plane_aspect", &self.plane_aspect)
            .finish()
    }
}
impl Default for BindImagePlaneMemoryInfo {
    fn default() -> BindImagePlaneMemoryInfo {
        BindImagePlaneMemoryInfo {
            s_type: crate::vk1_0::StructureType::BIND_IMAGE_PLANE_MEMORY_INFO,
            p_next: std::ptr::null(),
            plane_aspect: Default::default(),
        }
    }
}
#[doc = "Used by [`BindImagePlaneMemoryInfo::extend`](struct.BindImagePlaneMemoryInfo.html#method.extend)"]
pub trait ExtendableByBindImagePlaneMemoryInfo {}
impl ExtendableByBindImagePlaneMemoryInfo for crate::vk1_1::BindImageMemoryInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`BindImagePlaneMemoryInfo`](struct.BindImagePlaneMemoryInfo.html)"]
#[repr(transparent)]
pub struct BindImagePlaneMemoryInfoBuilder<'a>(
    BindImagePlaneMemoryInfo,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> BindImagePlaneMemoryInfoBuilder<'a> {
    #[inline]
    pub fn new() -> BindImagePlaneMemoryInfoBuilder<'a> {
        BindImagePlaneMemoryInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn plane_aspect(mut self, plane_aspect: crate::vk1_0::ImageAspectFlagBits) -> Self {
        self.0.plane_aspect = plane_aspect;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> BindImagePlaneMemoryInfo {
        self.0
    }
}
impl<'a> std::fmt::Debug for BindImagePlaneMemoryInfoBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for BindImagePlaneMemoryInfoBuilder<'a> {
    type Target = BindImagePlaneMemoryInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for BindImagePlaneMemoryInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImagePlaneMemoryRequirementsInfo.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImagePlaneMemoryRequirementsInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub plane_aspect: crate::vk1_0::ImageAspectFlagBits,
}
impl ImagePlaneMemoryRequirementsInfo {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByImagePlaneMemoryRequirementsInfo,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> ImagePlaneMemoryRequirementsInfoBuilder<'a> {
        ImagePlaneMemoryRequirementsInfoBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ImagePlaneMemoryRequirementsInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ImagePlaneMemoryRequirementsInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("plane_aspect", &self.plane_aspect)
            .finish()
    }
}
impl Default for ImagePlaneMemoryRequirementsInfo {
    fn default() -> ImagePlaneMemoryRequirementsInfo {
        ImagePlaneMemoryRequirementsInfo {
            s_type: crate::vk1_0::StructureType::IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO,
            p_next: std::ptr::null(),
            plane_aspect: Default::default(),
        }
    }
}
#[doc = "Used by [`ImagePlaneMemoryRequirementsInfo::extend`](struct.ImagePlaneMemoryRequirementsInfo.html#method.extend)"]
pub trait ExtendableByImagePlaneMemoryRequirementsInfo {}
impl ExtendableByImagePlaneMemoryRequirementsInfo for crate::vk1_1::ImageMemoryRequirementsInfo2 {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`ImagePlaneMemoryRequirementsInfo`](struct.ImagePlaneMemoryRequirementsInfo.html)"]
#[repr(transparent)]
pub struct ImagePlaneMemoryRequirementsInfoBuilder<'a>(
    ImagePlaneMemoryRequirementsInfo,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> ImagePlaneMemoryRequirementsInfoBuilder<'a> {
    #[inline]
    pub fn new() -> ImagePlaneMemoryRequirementsInfoBuilder<'a> {
        ImagePlaneMemoryRequirementsInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn plane_aspect(mut self, plane_aspect: crate::vk1_0::ImageAspectFlagBits) -> Self {
        self.0.plane_aspect = plane_aspect;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> ImagePlaneMemoryRequirementsInfo {
        self.0
    }
}
impl<'a> std::fmt::Debug for ImagePlaneMemoryRequirementsInfoBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for ImagePlaneMemoryRequirementsInfoBuilder<'a> {
    type Target = ImagePlaneMemoryRequirementsInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ImagePlaneMemoryRequirementsInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSamplerYcbcrConversionFeatures.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceSamplerYcbcrConversionFeatures {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub sampler_ycbcr_conversion: crate::vk1_0::Bool32,
}
impl PhysicalDeviceSamplerYcbcrConversionFeatures {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceSamplerYcbcrConversionFeatures,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceSamplerYcbcrConversionFeaturesBuilder<'a> {
        PhysicalDeviceSamplerYcbcrConversionFeaturesBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceSamplerYcbcrConversionFeatures {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceSamplerYcbcrConversionFeatures")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "sampler_ycbcr_conversion",
                &(self.sampler_ycbcr_conversion != 0),
            )
            .finish()
    }
}
impl Default for PhysicalDeviceSamplerYcbcrConversionFeatures {
    fn default() -> PhysicalDeviceSamplerYcbcrConversionFeatures {
        PhysicalDeviceSamplerYcbcrConversionFeatures {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES,
            p_next: std::ptr::null_mut(),
            sampler_ycbcr_conversion: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceSamplerYcbcrConversionFeatures::extend`](struct.PhysicalDeviceSamplerYcbcrConversionFeatures.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceSamplerYcbcrConversionFeatures {}
impl ExtendableByPhysicalDeviceSamplerYcbcrConversionFeatures
    for crate::vk1_1::PhysicalDeviceFeatures2
{
}
impl ExtendableByPhysicalDeviceSamplerYcbcrConversionFeatures for crate::vk1_0::DeviceCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceSamplerYcbcrConversionFeatures`](struct.PhysicalDeviceSamplerYcbcrConversionFeatures.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceSamplerYcbcrConversionFeaturesBuilder<'a>(
    PhysicalDeviceSamplerYcbcrConversionFeatures,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceSamplerYcbcrConversionFeaturesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceSamplerYcbcrConversionFeaturesBuilder<'a> {
        PhysicalDeviceSamplerYcbcrConversionFeaturesBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn sampler_ycbcr_conversion(mut self, sampler_ycbcr_conversion: bool) -> Self {
        self.0.sampler_ycbcr_conversion = sampler_ycbcr_conversion as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceSamplerYcbcrConversionFeatures {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceSamplerYcbcrConversionFeaturesBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceSamplerYcbcrConversionFeaturesBuilder<'a> {
    type Target = PhysicalDeviceSamplerYcbcrConversionFeatures;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceSamplerYcbcrConversionFeaturesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerYcbcrConversionImageFormatProperties.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SamplerYcbcrConversionImageFormatProperties {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub combined_image_sampler_descriptor_count: u32,
}
impl SamplerYcbcrConversionImageFormatProperties {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableBySamplerYcbcrConversionImageFormatProperties,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> SamplerYcbcrConversionImageFormatPropertiesBuilder<'a> {
        SamplerYcbcrConversionImageFormatPropertiesBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for SamplerYcbcrConversionImageFormatProperties {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("SamplerYcbcrConversionImageFormatProperties")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "combined_image_sampler_descriptor_count",
                &self.combined_image_sampler_descriptor_count,
            )
            .finish()
    }
}
impl Default for SamplerYcbcrConversionImageFormatProperties {
    fn default() -> SamplerYcbcrConversionImageFormatProperties {
        SamplerYcbcrConversionImageFormatProperties {
            s_type: crate::vk1_0::StructureType::SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES,
            p_next: std::ptr::null_mut(),
            combined_image_sampler_descriptor_count: Default::default(),
        }
    }
}
#[doc = "Used by [`SamplerYcbcrConversionImageFormatProperties::extend`](struct.SamplerYcbcrConversionImageFormatProperties.html#method.extend)"]
pub trait ExtendableBySamplerYcbcrConversionImageFormatProperties {}
impl ExtendableBySamplerYcbcrConversionImageFormatProperties
    for crate::vk1_1::ImageFormatProperties2
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`SamplerYcbcrConversionImageFormatProperties`](struct.SamplerYcbcrConversionImageFormatProperties.html)"]
#[repr(transparent)]
pub struct SamplerYcbcrConversionImageFormatPropertiesBuilder<'a>(
    SamplerYcbcrConversionImageFormatProperties,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> SamplerYcbcrConversionImageFormatPropertiesBuilder<'a> {
    #[inline]
    pub fn new() -> SamplerYcbcrConversionImageFormatPropertiesBuilder<'a> {
        SamplerYcbcrConversionImageFormatPropertiesBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn combined_image_sampler_descriptor_count(
        mut self,
        combined_image_sampler_descriptor_count: u32,
    ) -> Self {
        self.0.combined_image_sampler_descriptor_count = combined_image_sampler_descriptor_count;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> SamplerYcbcrConversionImageFormatProperties {
        self.0
    }
}
impl<'a> std::fmt::Debug for SamplerYcbcrConversionImageFormatPropertiesBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for SamplerYcbcrConversionImageFormatPropertiesBuilder<'a> {
    type Target = SamplerYcbcrConversionImageFormatProperties;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SamplerYcbcrConversionImageFormatPropertiesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceExternalImageFormatInfo.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceExternalImageFormatInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub handle_type: crate::vk1_1::ExternalMemoryHandleTypeFlagBits,
}
impl PhysicalDeviceExternalImageFormatInfo {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceExternalImageFormatInfo,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceExternalImageFormatInfoBuilder<'a> {
        PhysicalDeviceExternalImageFormatInfoBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceExternalImageFormatInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceExternalImageFormatInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("handle_type", &self.handle_type)
            .finish()
    }
}
impl Default for PhysicalDeviceExternalImageFormatInfo {
    fn default() -> PhysicalDeviceExternalImageFormatInfo {
        PhysicalDeviceExternalImageFormatInfo {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO,
            p_next: std::ptr::null(),
            handle_type: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceExternalImageFormatInfo::extend`](struct.PhysicalDeviceExternalImageFormatInfo.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceExternalImageFormatInfo {}
impl ExtendableByPhysicalDeviceExternalImageFormatInfo
    for crate::vk1_1::PhysicalDeviceImageFormatInfo2
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceExternalImageFormatInfo`](struct.PhysicalDeviceExternalImageFormatInfo.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceExternalImageFormatInfoBuilder<'a>(
    PhysicalDeviceExternalImageFormatInfo,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceExternalImageFormatInfoBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceExternalImageFormatInfoBuilder<'a> {
        PhysicalDeviceExternalImageFormatInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn handle_type(
        mut self,
        handle_type: crate::vk1_1::ExternalMemoryHandleTypeFlagBits,
    ) -> Self {
        self.0.handle_type = handle_type;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceExternalImageFormatInfo {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceExternalImageFormatInfoBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceExternalImageFormatInfoBuilder<'a> {
    type Target = PhysicalDeviceExternalImageFormatInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceExternalImageFormatInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalImageFormatProperties.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExternalImageFormatProperties {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub external_memory_properties: crate::vk1_1::ExternalMemoryProperties,
}
impl ExternalImageFormatProperties {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByExternalImageFormatProperties,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> ExternalImageFormatPropertiesBuilder<'a> {
        ExternalImageFormatPropertiesBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ExternalImageFormatProperties {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ExternalImageFormatProperties")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "external_memory_properties",
                &self.external_memory_properties,
            )
            .finish()
    }
}
impl Default for ExternalImageFormatProperties {
    fn default() -> ExternalImageFormatProperties {
        ExternalImageFormatProperties {
            s_type: crate::vk1_0::StructureType::EXTERNAL_IMAGE_FORMAT_PROPERTIES,
            p_next: std::ptr::null_mut(),
            external_memory_properties: Default::default(),
        }
    }
}
#[doc = "Used by [`ExternalImageFormatProperties::extend`](struct.ExternalImageFormatProperties.html#method.extend)"]
pub trait ExtendableByExternalImageFormatProperties {}
impl ExtendableByExternalImageFormatProperties for crate::vk1_1::ImageFormatProperties2 {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`ExternalImageFormatProperties`](struct.ExternalImageFormatProperties.html)"]
#[repr(transparent)]
pub struct ExternalImageFormatPropertiesBuilder<'a>(
    ExternalImageFormatProperties,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> ExternalImageFormatPropertiesBuilder<'a> {
    #[inline]
    pub fn new() -> ExternalImageFormatPropertiesBuilder<'a> {
        ExternalImageFormatPropertiesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn external_memory_properties(
        mut self,
        external_memory_properties: crate::vk1_1::ExternalMemoryProperties,
    ) -> Self {
        self.0.external_memory_properties = external_memory_properties;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> ExternalImageFormatProperties {
        self.0
    }
}
impl<'a> std::fmt::Debug for ExternalImageFormatPropertiesBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for ExternalImageFormatPropertiesBuilder<'a> {
    type Target = ExternalImageFormatProperties;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ExternalImageFormatPropertiesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceIDProperties.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceIDProperties {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub device_uuid: [u8; crate::vk1_0::UUID_SIZE as usize],
    pub driver_uuid: [u8; crate::vk1_0::UUID_SIZE as usize],
    pub device_luid: [u8; crate::vk1_1::LUID_SIZE as usize],
    pub device_node_mask: u32,
    pub device_luid_valid: crate::vk1_0::Bool32,
}
impl PhysicalDeviceIDProperties {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceIDProperties,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceIDPropertiesBuilder<'a> {
        PhysicalDeviceIDPropertiesBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceIDProperties {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceIDProperties")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("device_uuid", &self.device_uuid)
            .field("driver_uuid", &self.driver_uuid)
            .field("device_luid", &self.device_luid)
            .field("device_node_mask", &self.device_node_mask)
            .field("device_luid_valid", &(self.device_luid_valid != 0))
            .finish()
    }
}
impl Default for PhysicalDeviceIDProperties {
    fn default() -> PhysicalDeviceIDProperties {
        PhysicalDeviceIDProperties {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_ID_PROPERTIES,
            p_next: std::ptr::null_mut(),
            device_uuid: Default::default(),
            driver_uuid: Default::default(),
            device_luid: Default::default(),
            device_node_mask: Default::default(),
            device_luid_valid: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceIDProperties::extend`](struct.PhysicalDeviceIDProperties.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceIDProperties {}
impl ExtendableByPhysicalDeviceIDProperties for crate::vk1_1::PhysicalDeviceProperties2 {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceIDProperties`](struct.PhysicalDeviceIDProperties.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceIDPropertiesBuilder<'a>(
    PhysicalDeviceIDProperties,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceIDPropertiesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceIDPropertiesBuilder<'a> {
        PhysicalDeviceIDPropertiesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn device_uuid(mut self, device_uuid: [u8; crate::vk1_0::UUID_SIZE as usize]) -> Self {
        self.0.device_uuid = device_uuid;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn driver_uuid(mut self, driver_uuid: [u8; crate::vk1_0::UUID_SIZE as usize]) -> Self {
        self.0.driver_uuid = driver_uuid;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn device_luid(mut self, device_luid: [u8; crate::vk1_1::LUID_SIZE as usize]) -> Self {
        self.0.device_luid = device_luid;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn device_node_mask(mut self, device_node_mask: u32) -> Self {
        self.0.device_node_mask = device_node_mask;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn device_luid_valid(mut self, device_luid_valid: bool) -> Self {
        self.0.device_luid_valid = device_luid_valid as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceIDProperties {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceIDPropertiesBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceIDPropertiesBuilder<'a> {
    type Target = PhysicalDeviceIDProperties;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceIDPropertiesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalMemoryImageCreateInfo.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExternalMemoryImageCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub handle_types: crate::vk1_1::ExternalMemoryHandleTypeFlags,
}
impl ExternalMemoryImageCreateInfo {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByExternalMemoryImageCreateInfo,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> ExternalMemoryImageCreateInfoBuilder<'a> {
        ExternalMemoryImageCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ExternalMemoryImageCreateInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ExternalMemoryImageCreateInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("handle_types", &self.handle_types)
            .finish()
    }
}
impl Default for ExternalMemoryImageCreateInfo {
    fn default() -> ExternalMemoryImageCreateInfo {
        ExternalMemoryImageCreateInfo {
            s_type: crate::vk1_0::StructureType::EXTERNAL_MEMORY_IMAGE_CREATE_INFO,
            p_next: std::ptr::null(),
            handle_types: Default::default(),
        }
    }
}
#[doc = "Used by [`ExternalMemoryImageCreateInfo::extend`](struct.ExternalMemoryImageCreateInfo.html#method.extend)"]
pub trait ExtendableByExternalMemoryImageCreateInfo {}
impl ExtendableByExternalMemoryImageCreateInfo for crate::vk1_0::ImageCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`ExternalMemoryImageCreateInfo`](struct.ExternalMemoryImageCreateInfo.html)"]
#[repr(transparent)]
pub struct ExternalMemoryImageCreateInfoBuilder<'a>(
    ExternalMemoryImageCreateInfo,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> ExternalMemoryImageCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> ExternalMemoryImageCreateInfoBuilder<'a> {
        ExternalMemoryImageCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn handle_types(
        mut self,
        handle_types: crate::vk1_1::ExternalMemoryHandleTypeFlags,
    ) -> Self {
        self.0.handle_types = handle_types;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> ExternalMemoryImageCreateInfo {
        self.0
    }
}
impl<'a> std::fmt::Debug for ExternalMemoryImageCreateInfoBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for ExternalMemoryImageCreateInfoBuilder<'a> {
    type Target = ExternalMemoryImageCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ExternalMemoryImageCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalMemoryBufferCreateInfo.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExternalMemoryBufferCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub handle_types: crate::vk1_1::ExternalMemoryHandleTypeFlags,
}
impl ExternalMemoryBufferCreateInfo {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByExternalMemoryBufferCreateInfo,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> ExternalMemoryBufferCreateInfoBuilder<'a> {
        ExternalMemoryBufferCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ExternalMemoryBufferCreateInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ExternalMemoryBufferCreateInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("handle_types", &self.handle_types)
            .finish()
    }
}
impl Default for ExternalMemoryBufferCreateInfo {
    fn default() -> ExternalMemoryBufferCreateInfo {
        ExternalMemoryBufferCreateInfo {
            s_type: crate::vk1_0::StructureType::EXTERNAL_MEMORY_BUFFER_CREATE_INFO,
            p_next: std::ptr::null(),
            handle_types: Default::default(),
        }
    }
}
#[doc = "Used by [`ExternalMemoryBufferCreateInfo::extend`](struct.ExternalMemoryBufferCreateInfo.html#method.extend)"]
pub trait ExtendableByExternalMemoryBufferCreateInfo {}
impl ExtendableByExternalMemoryBufferCreateInfo for crate::vk1_0::BufferCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`ExternalMemoryBufferCreateInfo`](struct.ExternalMemoryBufferCreateInfo.html)"]
#[repr(transparent)]
pub struct ExternalMemoryBufferCreateInfoBuilder<'a>(
    ExternalMemoryBufferCreateInfo,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> ExternalMemoryBufferCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> ExternalMemoryBufferCreateInfoBuilder<'a> {
        ExternalMemoryBufferCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn handle_types(
        mut self,
        handle_types: crate::vk1_1::ExternalMemoryHandleTypeFlags,
    ) -> Self {
        self.0.handle_types = handle_types;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> ExternalMemoryBufferCreateInfo {
        self.0
    }
}
impl<'a> std::fmt::Debug for ExternalMemoryBufferCreateInfoBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for ExternalMemoryBufferCreateInfoBuilder<'a> {
    type Target = ExternalMemoryBufferCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ExternalMemoryBufferCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExportMemoryAllocateInfo.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExportMemoryAllocateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub handle_types: crate::vk1_1::ExternalMemoryHandleTypeFlags,
}
impl ExportMemoryAllocateInfo {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByExportMemoryAllocateInfo,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> ExportMemoryAllocateInfoBuilder<'a> {
        ExportMemoryAllocateInfoBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ExportMemoryAllocateInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ExportMemoryAllocateInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("handle_types", &self.handle_types)
            .finish()
    }
}
impl Default for ExportMemoryAllocateInfo {
    fn default() -> ExportMemoryAllocateInfo {
        ExportMemoryAllocateInfo {
            s_type: crate::vk1_0::StructureType::EXPORT_MEMORY_ALLOCATE_INFO,
            p_next: std::ptr::null(),
            handle_types: Default::default(),
        }
    }
}
#[doc = "Used by [`ExportMemoryAllocateInfo::extend`](struct.ExportMemoryAllocateInfo.html#method.extend)"]
pub trait ExtendableByExportMemoryAllocateInfo {}
impl ExtendableByExportMemoryAllocateInfo for crate::vk1_0::MemoryAllocateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`ExportMemoryAllocateInfo`](struct.ExportMemoryAllocateInfo.html)"]
#[repr(transparent)]
pub struct ExportMemoryAllocateInfoBuilder<'a>(
    ExportMemoryAllocateInfo,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> ExportMemoryAllocateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> ExportMemoryAllocateInfoBuilder<'a> {
        ExportMemoryAllocateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn handle_types(
        mut self,
        handle_types: crate::vk1_1::ExternalMemoryHandleTypeFlags,
    ) -> Self {
        self.0.handle_types = handle_types;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> ExportMemoryAllocateInfo {
        self.0
    }
}
impl<'a> std::fmt::Debug for ExportMemoryAllocateInfoBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for ExportMemoryAllocateInfoBuilder<'a> {
    type Target = ExportMemoryAllocateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ExportMemoryAllocateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFenceImportFlagBits.html) · Flag Bits of [`FenceImportFlags`](struct.FenceImportFlags.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct FenceImportFlagBits(pub u32);
impl FenceImportFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> FenceImportFlags {
        FenceImportFlags::from_bits_truncate(self.0)
    }
}
#[doc = "[Part of `vk1_1`](../vk1_1/index.html)"]
impl FenceImportFlagBits {
    pub const TEMPORARY: Self = Self(0x00000001);
}
#[doc = "[Part of `extensions::khr_external_fence`](../extensions/khr_external_fence/index.html)"]
impl FenceImportFlagBits {
    pub const TEMPORARY_KHR: Self = Self::TEMPORARY;
}
impl std::fmt::Debug for FenceImportFlagBits {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::TEMPORARY => "TEMPORARY",
            _ => "(Unknown)",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFenceImportFlags.html) · Flags of [`FenceImportFlagBits`](struct.FenceImportFlagBits.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct FenceImportFlags : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; const TEMPORARY = FenceImportFlagBits :: TEMPORARY . 0 ; const TEMPORARY_KHR = FenceImportFlagBits :: TEMPORARY_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExportFenceCreateInfo.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExportFenceCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub handle_types: crate::vk1_1::ExternalFenceHandleTypeFlags,
}
impl ExportFenceCreateInfo {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByExportFenceCreateInfo,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> ExportFenceCreateInfoBuilder<'a> {
        ExportFenceCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ExportFenceCreateInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ExportFenceCreateInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("handle_types", &self.handle_types)
            .finish()
    }
}
impl Default for ExportFenceCreateInfo {
    fn default() -> ExportFenceCreateInfo {
        ExportFenceCreateInfo {
            s_type: crate::vk1_0::StructureType::EXPORT_FENCE_CREATE_INFO,
            p_next: std::ptr::null(),
            handle_types: Default::default(),
        }
    }
}
#[doc = "Used by [`ExportFenceCreateInfo::extend`](struct.ExportFenceCreateInfo.html#method.extend)"]
pub trait ExtendableByExportFenceCreateInfo {}
impl ExtendableByExportFenceCreateInfo for crate::vk1_0::FenceCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`ExportFenceCreateInfo`](struct.ExportFenceCreateInfo.html)"]
#[repr(transparent)]
pub struct ExportFenceCreateInfoBuilder<'a>(
    ExportFenceCreateInfo,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> ExportFenceCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> ExportFenceCreateInfoBuilder<'a> {
        ExportFenceCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn handle_types(
        mut self,
        handle_types: crate::vk1_1::ExternalFenceHandleTypeFlags,
    ) -> Self {
        self.0.handle_types = handle_types;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> ExportFenceCreateInfo {
        self.0
    }
}
impl<'a> std::fmt::Debug for ExportFenceCreateInfoBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for ExportFenceCreateInfoBuilder<'a> {
    type Target = ExportFenceCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ExportFenceCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreImportFlagBits.html) · Flag Bits of [`SemaphoreImportFlags`](struct.SemaphoreImportFlags.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct SemaphoreImportFlagBits(pub u32);
impl SemaphoreImportFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> SemaphoreImportFlags {
        SemaphoreImportFlags::from_bits_truncate(self.0)
    }
}
#[doc = "[Part of `vk1_1`](../vk1_1/index.html)"]
impl SemaphoreImportFlagBits {
    pub const TEMPORARY: Self = Self(0x00000001);
}
#[doc = "[Part of `extensions::khr_external_semaphore`](../extensions/khr_external_semaphore/index.html)"]
impl SemaphoreImportFlagBits {
    pub const TEMPORARY_KHR: Self = Self::TEMPORARY;
}
impl std::fmt::Debug for SemaphoreImportFlagBits {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::TEMPORARY => "TEMPORARY",
            _ => "(Unknown)",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreImportFlags.html) · Flags of [`SemaphoreImportFlagBits`](struct.SemaphoreImportFlagBits.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct SemaphoreImportFlags : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; const TEMPORARY = SemaphoreImportFlagBits :: TEMPORARY . 0 ; const TEMPORARY_KHR = SemaphoreImportFlagBits :: TEMPORARY_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExportSemaphoreCreateInfo.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExportSemaphoreCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub handle_types: crate::vk1_1::ExternalSemaphoreHandleTypeFlags,
}
impl ExportSemaphoreCreateInfo {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByExportSemaphoreCreateInfo,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> ExportSemaphoreCreateInfoBuilder<'a> {
        ExportSemaphoreCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ExportSemaphoreCreateInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ExportSemaphoreCreateInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("handle_types", &self.handle_types)
            .finish()
    }
}
impl Default for ExportSemaphoreCreateInfo {
    fn default() -> ExportSemaphoreCreateInfo {
        ExportSemaphoreCreateInfo {
            s_type: crate::vk1_0::StructureType::EXPORT_SEMAPHORE_CREATE_INFO,
            p_next: std::ptr::null(),
            handle_types: Default::default(),
        }
    }
}
#[doc = "Used by [`ExportSemaphoreCreateInfo::extend`](struct.ExportSemaphoreCreateInfo.html#method.extend)"]
pub trait ExtendableByExportSemaphoreCreateInfo {}
impl ExtendableByExportSemaphoreCreateInfo for crate::vk1_0::SemaphoreCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`ExportSemaphoreCreateInfo`](struct.ExportSemaphoreCreateInfo.html)"]
#[repr(transparent)]
pub struct ExportSemaphoreCreateInfoBuilder<'a>(
    ExportSemaphoreCreateInfo,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> ExportSemaphoreCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> ExportSemaphoreCreateInfoBuilder<'a> {
        ExportSemaphoreCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn handle_types(
        mut self,
        handle_types: crate::vk1_1::ExternalSemaphoreHandleTypeFlags,
    ) -> Self {
        self.0.handle_types = handle_types;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> ExportSemaphoreCreateInfo {
        self.0
    }
}
impl<'a> std::fmt::Debug for ExportSemaphoreCreateInfoBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for ExportSemaphoreCreateInfoBuilder<'a> {
    type Target = ExportSemaphoreCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ExportSemaphoreCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMaintenance3Properties.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceMaintenance3Properties {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub max_per_set_descriptors: u32,
    pub max_memory_allocation_size: crate::vk1_0::DeviceSize,
}
impl PhysicalDeviceMaintenance3Properties {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceMaintenance3Properties,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceMaintenance3PropertiesBuilder<'a> {
        PhysicalDeviceMaintenance3PropertiesBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceMaintenance3Properties {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceMaintenance3Properties")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("max_per_set_descriptors", &self.max_per_set_descriptors)
            .field(
                "max_memory_allocation_size",
                &self.max_memory_allocation_size,
            )
            .finish()
    }
}
impl Default for PhysicalDeviceMaintenance3Properties {
    fn default() -> PhysicalDeviceMaintenance3Properties {
        PhysicalDeviceMaintenance3Properties {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES,
            p_next: std::ptr::null_mut(),
            max_per_set_descriptors: Default::default(),
            max_memory_allocation_size: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceMaintenance3Properties::extend`](struct.PhysicalDeviceMaintenance3Properties.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceMaintenance3Properties {}
impl ExtendableByPhysicalDeviceMaintenance3Properties for crate::vk1_1::PhysicalDeviceProperties2 {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceMaintenance3Properties`](struct.PhysicalDeviceMaintenance3Properties.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceMaintenance3PropertiesBuilder<'a>(
    PhysicalDeviceMaintenance3Properties,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceMaintenance3PropertiesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceMaintenance3PropertiesBuilder<'a> {
        PhysicalDeviceMaintenance3PropertiesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_per_set_descriptors(mut self, max_per_set_descriptors: u32) -> Self {
        self.0.max_per_set_descriptors = max_per_set_descriptors;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_memory_allocation_size(
        mut self,
        max_memory_allocation_size: crate::vk1_0::DeviceSize,
    ) -> Self {
        self.0.max_memory_allocation_size = max_memory_allocation_size;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceMaintenance3Properties {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceMaintenance3PropertiesBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceMaintenance3PropertiesBuilder<'a> {
    type Target = PhysicalDeviceMaintenance3Properties;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceMaintenance3PropertiesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderDrawParameterFeatures.html) · Alias"]
pub type PhysicalDeviceShaderDrawParameterFeatures =
    crate::vk1_1::PhysicalDeviceShaderDrawParametersFeatures;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderDrawParametersFeatures.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceShaderDrawParametersFeatures {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub shader_draw_parameters: crate::vk1_0::Bool32,
}
impl PhysicalDeviceShaderDrawParametersFeatures {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceShaderDrawParametersFeatures,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceShaderDrawParametersFeaturesBuilder<'a> {
        PhysicalDeviceShaderDrawParametersFeaturesBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceShaderDrawParametersFeatures {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceShaderDrawParametersFeatures")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "shader_draw_parameters",
                &(self.shader_draw_parameters != 0),
            )
            .finish()
    }
}
impl Default for PhysicalDeviceShaderDrawParametersFeatures {
    fn default() -> PhysicalDeviceShaderDrawParametersFeatures {
        PhysicalDeviceShaderDrawParametersFeatures {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES,
            p_next: std::ptr::null_mut(),
            shader_draw_parameters: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceShaderDrawParametersFeatures::extend`](struct.PhysicalDeviceShaderDrawParametersFeatures.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceShaderDrawParametersFeatures {}
impl ExtendableByPhysicalDeviceShaderDrawParametersFeatures
    for crate::vk1_1::PhysicalDeviceFeatures2
{
}
impl ExtendableByPhysicalDeviceShaderDrawParametersFeatures for crate::vk1_0::DeviceCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceShaderDrawParametersFeatures`](struct.PhysicalDeviceShaderDrawParametersFeatures.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceShaderDrawParametersFeaturesBuilder<'a>(
    PhysicalDeviceShaderDrawParametersFeatures,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceShaderDrawParametersFeaturesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceShaderDrawParametersFeaturesBuilder<'a> {
        PhysicalDeviceShaderDrawParametersFeaturesBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_draw_parameters(mut self, shader_draw_parameters: bool) -> Self {
        self.0.shader_draw_parameters = shader_draw_parameters as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceShaderDrawParametersFeatures {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceShaderDrawParametersFeaturesBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceShaderDrawParametersFeaturesBuilder<'a> {
    type Target = PhysicalDeviceShaderDrawParametersFeatures;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceShaderDrawParametersFeaturesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
