# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_KHR_get_physical_device_properties2.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_get_physical_device_properties2");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceFeatures2KHR.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceFeatures2KHR = unsafe extern "system" fn(
    physical_device: crate::vk1_0::PhysicalDevice,
    p_features: *mut crate::vk1_1::PhysicalDeviceFeatures2,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceProperties2KHR.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceProperties2KHR = unsafe extern "system" fn(
    physical_device: crate::vk1_0::PhysicalDevice,
    p_properties: *mut crate::vk1_1::PhysicalDeviceProperties2,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceFormatProperties2KHR.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceFormatProperties2KHR =
    unsafe extern "system" fn(
        physical_device: crate::vk1_0::PhysicalDevice,
        format: crate::vk1_0::Format,
        p_format_properties: *mut crate::vk1_1::FormatProperties2,
    ) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceImageFormatProperties2KHR.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceImageFormatProperties2KHR =
    unsafe extern "system" fn(
        physical_device: crate::vk1_0::PhysicalDevice,
        p_image_format_info: *const crate::vk1_1::PhysicalDeviceImageFormatInfo2,
        p_image_format_properties: *mut crate::vk1_1::ImageFormatProperties2,
    ) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties2KHR.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties2KHR =
    unsafe extern "system" fn(
        physical_device: crate::vk1_0::PhysicalDevice,
        p_queue_family_property_count: *mut u32,
        p_queue_family_properties: *mut crate::vk1_1::QueueFamilyProperties2,
    ) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceMemoryProperties2KHR.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceMemoryProperties2KHR =
    unsafe extern "system" fn(
        physical_device: crate::vk1_0::PhysicalDevice,
        p_memory_properties: *mut crate::vk1_1::PhysicalDeviceMemoryProperties2,
    ) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSparseImageFormatProperties2KHR.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties2KHR =
    unsafe extern "system" fn(
        physical_device: crate::vk1_0::PhysicalDevice,
        p_format_info: *const crate::vk1_1::PhysicalDeviceSparseImageFormatInfo2,
        p_property_count: *mut u32,
        p_properties: *mut crate::vk1_1::SparseImageFormatProperties2,
    ) -> std::ffi::c_void;
#[doc = "Provides Instance Commands for [`KhrGetPhysicalDeviceProperties2InstanceLoaderExt`](trait.KhrGetPhysicalDeviceProperties2InstanceLoaderExt.html)"]
pub struct KhrGetPhysicalDeviceProperties2InstanceCommands {
    pub get_physical_device_features2_khr: Option<PFN_vkGetPhysicalDeviceFeatures2KHR>,
    pub get_physical_device_properties2_khr: Option<PFN_vkGetPhysicalDeviceProperties2KHR>,
    pub get_physical_device_format_properties2_khr:
        Option<PFN_vkGetPhysicalDeviceFormatProperties2KHR>,
    pub get_physical_device_image_format_properties2_khr:
        Option<PFN_vkGetPhysicalDeviceImageFormatProperties2KHR>,
    pub get_physical_device_queue_family_properties2_khr:
        Option<PFN_vkGetPhysicalDeviceQueueFamilyProperties2KHR>,
    pub get_physical_device_memory_properties2_khr:
        Option<PFN_vkGetPhysicalDeviceMemoryProperties2KHR>,
    pub get_physical_device_sparse_image_format_properties2_khr:
        Option<PFN_vkGetPhysicalDeviceSparseImageFormatProperties2KHR>,
}
impl KhrGetPhysicalDeviceProperties2InstanceCommands {
    #[inline]
    pub fn load(
        loader: &crate::InstanceLoader,
    ) -> Option<KhrGetPhysicalDeviceProperties2InstanceCommands> {
        unsafe {
            let mut success = false;
            let table = KhrGetPhysicalDeviceProperties2InstanceCommands {
                get_physical_device_features2_khr: std::mem::transmute({
                    let symbol = loader.symbol("vkGetPhysicalDeviceFeatures2KHR");
                    success |= symbol.is_some();
                    symbol
                }),
                get_physical_device_properties2_khr: std::mem::transmute({
                    let symbol = loader.symbol("vkGetPhysicalDeviceProperties2KHR");
                    success |= symbol.is_some();
                    symbol
                }),
                get_physical_device_format_properties2_khr: std::mem::transmute({
                    let symbol = loader.symbol("vkGetPhysicalDeviceFormatProperties2KHR");
                    success |= symbol.is_some();
                    symbol
                }),
                get_physical_device_image_format_properties2_khr: std::mem::transmute({
                    let symbol = loader.symbol("vkGetPhysicalDeviceImageFormatProperties2KHR");
                    success |= symbol.is_some();
                    symbol
                }),
                get_physical_device_queue_family_properties2_khr: std::mem::transmute({
                    let symbol = loader.symbol("vkGetPhysicalDeviceQueueFamilyProperties2KHR");
                    success |= symbol.is_some();
                    symbol
                }),
                get_physical_device_memory_properties2_khr: std::mem::transmute({
                    let symbol = loader.symbol("vkGetPhysicalDeviceMemoryProperties2KHR");
                    success |= symbol.is_some();
                    symbol
                }),
                get_physical_device_sparse_image_format_properties2_khr: std::mem::transmute({
                    let symbol =
                        loader.symbol("vkGetPhysicalDeviceSparseImageFormatProperties2KHR");
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
fn instance_commands(
    loader: &crate::InstanceLoader,
) -> &KhrGetPhysicalDeviceProperties2InstanceCommands {
    loader
        .khr_get_physical_device_properties2
        .as_ref()
        .expect("`khr_get_physical_device_properties2` not loaded")
}
#[doc = "Provides high level command wrappers for [`KhrGetPhysicalDeviceProperties2InstanceCommands`](struct.KhrGetPhysicalDeviceProperties2InstanceCommands.html)"]
pub trait KhrGetPhysicalDeviceProperties2InstanceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceFeatures2KHR.html) · Instance Command"]
    unsafe fn get_physical_device_features2_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        features: Option<crate::vk1_1::PhysicalDeviceFeatures2>,
    ) -> crate::vk1_1::PhysicalDeviceFeatures2;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceProperties2KHR.html) · Instance Command"]
    unsafe fn get_physical_device_properties2_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        properties: Option<crate::vk1_1::PhysicalDeviceProperties2>,
    ) -> crate::vk1_1::PhysicalDeviceProperties2;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceFormatProperties2KHR.html) · Instance Command"]
    unsafe fn get_physical_device_format_properties2_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        format: crate::vk1_0::Format,
        format_properties: Option<crate::vk1_1::FormatProperties2>,
    ) -> crate::vk1_1::FormatProperties2;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceImageFormatProperties2KHR.html) · Instance Command"]
    unsafe fn get_physical_device_image_format_properties2_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        image_format_info: &crate::vk1_1::PhysicalDeviceImageFormatInfo2,
        image_format_properties: Option<crate::vk1_1::ImageFormatProperties2>,
    ) -> crate::utils::VulkanResult<crate::vk1_1::ImageFormatProperties2>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties2KHR.html) · Instance Command"]
    unsafe fn get_physical_device_queue_family_properties2_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        queue_family_property_count: Option<u32>,
    ) -> Vec<crate::vk1_1::QueueFamilyProperties2>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceMemoryProperties2KHR.html) · Instance Command"]
    unsafe fn get_physical_device_memory_properties2_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        memory_properties: Option<crate::vk1_1::PhysicalDeviceMemoryProperties2>,
    ) -> crate::vk1_1::PhysicalDeviceMemoryProperties2;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSparseImageFormatProperties2KHR.html) · Instance Command"]
    unsafe fn get_physical_device_sparse_image_format_properties2_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        format_info: &crate::vk1_1::PhysicalDeviceSparseImageFormatInfo2,
        property_count: Option<u32>,
    ) -> Vec<crate::vk1_1::SparseImageFormatProperties2>;
}
impl KhrGetPhysicalDeviceProperties2InstanceLoaderExt for crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceFeatures2KHR.html) · Instance Command"]
    unsafe fn get_physical_device_features2_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        features: Option<crate::vk1_1::PhysicalDeviceFeatures2>,
    ) -> crate::vk1_1::PhysicalDeviceFeatures2 {
        let function = instance_commands(self)
            .get_physical_device_features2_khr
            .as_ref()
            .expect("`get_physical_device_features2_khr` not available");
        let mut features = features.unwrap_or_else(|| Default::default());
        let _val = function(physical_device, &mut features);
        features
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceProperties2KHR.html) · Instance Command"]
    unsafe fn get_physical_device_properties2_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        properties: Option<crate::vk1_1::PhysicalDeviceProperties2>,
    ) -> crate::vk1_1::PhysicalDeviceProperties2 {
        let function = instance_commands(self)
            .get_physical_device_properties2_khr
            .as_ref()
            .expect("`get_physical_device_properties2_khr` not available");
        let mut properties = properties.unwrap_or_else(|| Default::default());
        let _val = function(physical_device, &mut properties);
        properties
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceFormatProperties2KHR.html) · Instance Command"]
    unsafe fn get_physical_device_format_properties2_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        format: crate::vk1_0::Format,
        format_properties: Option<crate::vk1_1::FormatProperties2>,
    ) -> crate::vk1_1::FormatProperties2 {
        let function = instance_commands(self)
            .get_physical_device_format_properties2_khr
            .as_ref()
            .expect("`get_physical_device_format_properties2_khr` not available");
        let mut format_properties = format_properties.unwrap_or_else(|| Default::default());
        let _val = function(physical_device, format, &mut format_properties);
        format_properties
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceImageFormatProperties2KHR.html) · Instance Command"]
    unsafe fn get_physical_device_image_format_properties2_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        image_format_info: &crate::vk1_1::PhysicalDeviceImageFormatInfo2,
        image_format_properties: Option<crate::vk1_1::ImageFormatProperties2>,
    ) -> crate::utils::VulkanResult<crate::vk1_1::ImageFormatProperties2> {
        let function = instance_commands(self)
            .get_physical_device_image_format_properties2_khr
            .as_ref()
            .expect("`get_physical_device_image_format_properties2_khr` not available");
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
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties2KHR.html) · Instance Command"]
    unsafe fn get_physical_device_queue_family_properties2_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        queue_family_property_count: Option<u32>,
    ) -> Vec<crate::vk1_1::QueueFamilyProperties2> {
        let function = instance_commands(self)
            .get_physical_device_queue_family_properties2_khr
            .as_ref()
            .expect("`get_physical_device_queue_family_properties2_khr` not available");
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
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceMemoryProperties2KHR.html) · Instance Command"]
    unsafe fn get_physical_device_memory_properties2_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        memory_properties: Option<crate::vk1_1::PhysicalDeviceMemoryProperties2>,
    ) -> crate::vk1_1::PhysicalDeviceMemoryProperties2 {
        let function = instance_commands(self)
            .get_physical_device_memory_properties2_khr
            .as_ref()
            .expect("`get_physical_device_memory_properties2_khr` not available");
        let mut memory_properties = memory_properties.unwrap_or_else(|| Default::default());
        let _val = function(physical_device, &mut memory_properties);
        memory_properties
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSparseImageFormatProperties2KHR.html) · Instance Command"]
    unsafe fn get_physical_device_sparse_image_format_properties2_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        format_info: &crate::vk1_1::PhysicalDeviceSparseImageFormatInfo2,
        property_count: Option<u32>,
    ) -> Vec<crate::vk1_1::SparseImageFormatProperties2> {
        let function = instance_commands(self)
            .get_physical_device_sparse_image_format_properties2_khr
            .as_ref()
            .expect("`get_physical_device_sparse_image_format_properties2_khr` not available");
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
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceFeatures2KHR.html) · Alias"]
pub type PhysicalDeviceFeatures2KHR = crate::vk1_1::PhysicalDeviceFeatures2;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceProperties2KHR.html) · Alias"]
pub type PhysicalDeviceProperties2KHR = crate::vk1_1::PhysicalDeviceProperties2;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFormatProperties2KHR.html) · Alias"]
pub type FormatProperties2KHR = crate::vk1_1::FormatProperties2;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageFormatProperties2KHR.html) · Alias"]
pub type ImageFormatProperties2KHR = crate::vk1_1::ImageFormatProperties2;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceImageFormatInfo2KHR.html) · Alias"]
pub type PhysicalDeviceImageFormatInfo2KHR = crate::vk1_1::PhysicalDeviceImageFormatInfo2;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueueFamilyProperties2KHR.html) · Alias"]
pub type QueueFamilyProperties2KHR = crate::vk1_1::QueueFamilyProperties2;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMemoryProperties2KHR.html) · Alias"]
pub type PhysicalDeviceMemoryProperties2KHR = crate::vk1_1::PhysicalDeviceMemoryProperties2;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSparseImageFormatProperties2KHR.html) · Alias"]
pub type SparseImageFormatProperties2KHR = crate::vk1_1::SparseImageFormatProperties2;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSparseImageFormatInfo2KHR.html) · Alias"]
pub type PhysicalDeviceSparseImageFormatInfo2KHR =
    crate::vk1_1::PhysicalDeviceSparseImageFormatInfo2;
