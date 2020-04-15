# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_KHR_device_group.html)\n\n## Extends\n- [`DependencyFlagBits`](../../vk1_0/struct.DependencyFlagBits.html)\n- [`ImageCreateFlagBits`](../../vk1_0/struct.ImageCreateFlagBits.html)\n- [`MemoryAllocateFlagBits`](../../vk1_1/struct.MemoryAllocateFlagBits.html)\n- [`PeerMemoryFeatureFlagBits`](../../vk1_1/struct.PeerMemoryFeatureFlagBits.html)\n- [`PipelineCreateFlagBits`](../../vk1_0/struct.PipelineCreateFlagBits.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)\n- [`SwapchainCreateFlagBitsKHR`](../../extensions/khr_swapchain/struct.SwapchainCreateFlagBitsKHR.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_DEVICE_GROUP_SPEC_VERSION: u32 = 4;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_DEVICE_GROUP_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_device_group");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceGroupPeerMemoryFeaturesKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceGroupPeerMemoryFeaturesKHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    heap_index: u32,
    local_device_index: u32,
    remote_device_index: u32,
    p_peer_memory_features: *mut crate::vk1_1::PeerMemoryFeatureFlags,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDeviceMaskKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDeviceMaskKHR = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    device_mask: u32,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDispatchBaseKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDispatchBaseKHR = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    base_group_x: u32,
    base_group_y: u32,
    base_group_z: u32,
    group_count_x: u32,
    group_count_y: u32,
    group_count_z: u32,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceGroupPresentCapabilitiesKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceGroupPresentCapabilitiesKHR = unsafe extern "system" fn ( device : crate :: vk1_0 :: Device , p_device_group_present_capabilities : * mut crate :: extensions :: khr_swapchain :: DeviceGroupPresentCapabilitiesKHR , ) -> crate :: vk1_0 :: Result ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceGroupSurfacePresentModesKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceGroupSurfacePresentModesKHR =
    unsafe extern "system" fn(
        device: crate::vk1_0::Device,
        surface: crate::extensions::khr_surface::SurfaceKHR,
        p_modes: *mut crate::extensions::khr_swapchain::DeviceGroupPresentModeFlagsKHR,
    ) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDevicePresentRectanglesKHR.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDevicePresentRectanglesKHR =
    unsafe extern "system" fn(
        physical_device: crate::vk1_0::PhysicalDevice,
        surface: crate::extensions::khr_surface::SurfaceKHR,
        p_rect_count: *mut u32,
        p_rects: *mut crate::vk1_0::Rect2D,
    ) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAcquireNextImage2KHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkAcquireNextImage2KHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_acquire_info: *const crate::extensions::khr_swapchain::AcquireNextImageInfoKHR,
    p_image_index: *mut u32,
) -> crate::vk1_0::Result;
#[doc = "Provides Instance Commands for [`KhrDeviceGroupInstanceLoaderExt`](trait.KhrDeviceGroupInstanceLoaderExt.html)"]
pub struct KhrDeviceGroupInstanceCommands {
    pub get_physical_device_present_rectangles_khr: PFN_vkGetPhysicalDevicePresentRectanglesKHR,
}
impl KhrDeviceGroupInstanceCommands {
    #[inline]
    pub fn load(loader: &crate::InstanceLoader) -> Option<KhrDeviceGroupInstanceCommands> {
        unsafe {
            Some(KhrDeviceGroupInstanceCommands {
                get_physical_device_present_rectangles_khr: std::mem::transmute(
                    loader.symbol("vkGetPhysicalDevicePresentRectanglesKHR")?,
                ),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`KhrDeviceGroupInstanceCommands`](struct.KhrDeviceGroupInstanceCommands.html)"]
pub trait KhrDeviceGroupInstanceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDevicePresentRectanglesKHR.html) · Instance Command"]
    unsafe fn get_physical_device_present_rectangles_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        surface: crate::extensions::khr_surface::SurfaceKHR,
        rect_count: Option<u32>,
    ) -> crate::utils::VulkanResult<Vec<crate::vk1_0::Rect2D>>;
}
impl KhrDeviceGroupInstanceLoaderExt for crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDevicePresentRectanglesKHR.html) · Instance Command"]
    unsafe fn get_physical_device_present_rectangles_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        surface: crate::extensions::khr_surface::SurfaceKHR,
        rect_count: Option<u32>,
    ) -> crate::utils::VulkanResult<Vec<crate::vk1_0::Rect2D>> {
        let function = self
            .khr_device_group
            .as_ref()
            .expect("`khr_device_group` not loaded")
            .get_physical_device_present_rectangles_khr;
        let mut rect_count = rect_count.unwrap_or_else(|| {
            let mut val = Default::default();
            function(physical_device, surface, &mut val, std::ptr::null_mut());
            val
        });
        let mut rects = vec![Default::default(); rect_count as _];
        let _val = function(
            physical_device,
            surface,
            &mut rect_count,
            rects.as_mut_ptr(),
        );
        crate::utils::VulkanResult::new(_val, rects)
    }
}
#[doc = "Provides Device Commands for [`KhrDeviceGroupDeviceLoaderExt`](trait.KhrDeviceGroupDeviceLoaderExt.html)"]
pub struct KhrDeviceGroupDeviceCommands {
    pub get_device_group_peer_memory_features_khr: PFN_vkGetDeviceGroupPeerMemoryFeaturesKHR,
    pub cmd_set_device_mask_khr: PFN_vkCmdSetDeviceMaskKHR,
    pub cmd_dispatch_base_khr: PFN_vkCmdDispatchBaseKHR,
    pub get_device_group_present_capabilities_khr: PFN_vkGetDeviceGroupPresentCapabilitiesKHR,
    pub get_device_group_surface_present_modes_khr: PFN_vkGetDeviceGroupSurfacePresentModesKHR,
    pub acquire_next_image2_khr: PFN_vkAcquireNextImage2KHR,
}
impl KhrDeviceGroupDeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<KhrDeviceGroupDeviceCommands> {
        unsafe {
            Some(KhrDeviceGroupDeviceCommands {
                get_device_group_peer_memory_features_khr: std::mem::transmute(
                    loader.symbol("vkGetDeviceGroupPeerMemoryFeaturesKHR")?,
                ),
                cmd_set_device_mask_khr: std::mem::transmute(
                    loader.symbol("vkCmdSetDeviceMaskKHR")?,
                ),
                cmd_dispatch_base_khr: std::mem::transmute(loader.symbol("vkCmdDispatchBaseKHR")?),
                get_device_group_present_capabilities_khr: std::mem::transmute(
                    loader.symbol("vkGetDeviceGroupPresentCapabilitiesKHR")?,
                ),
                get_device_group_surface_present_modes_khr: std::mem::transmute(
                    loader.symbol("vkGetDeviceGroupSurfacePresentModesKHR")?,
                ),
                acquire_next_image2_khr: std::mem::transmute(
                    loader.symbol("vkAcquireNextImage2KHR")?,
                ),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`KhrDeviceGroupDeviceCommands`](struct.KhrDeviceGroupDeviceCommands.html)"]
pub trait KhrDeviceGroupDeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceGroupPeerMemoryFeaturesKHR.html) · Device Command"]
    unsafe fn get_device_group_peer_memory_features_khr(
        &self,
        heap_index: u32,
        local_device_index: u32,
        remote_device_index: u32,
        peer_memory_features: Option<crate::vk1_1::PeerMemoryFeatureFlags>,
    ) -> crate::vk1_1::PeerMemoryFeatureFlags;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDeviceMaskKHR.html) · Device Command"]
    unsafe fn cmd_set_device_mask_khr(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        device_mask: u32,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDispatchBaseKHR.html) · Device Command"]
    unsafe fn cmd_dispatch_base_khr(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        base_group_x: u32,
        base_group_y: u32,
        base_group_z: u32,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceGroupPresentCapabilitiesKHR.html) · Device Command"]
    unsafe fn get_device_group_present_capabilities_khr(
        &self,
        device_group_present_capabilities: Option<
            crate::extensions::khr_swapchain::DeviceGroupPresentCapabilitiesKHR,
        >,
    ) -> crate::utils::VulkanResult<
        crate::extensions::khr_swapchain::DeviceGroupPresentCapabilitiesKHR,
    >;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceGroupSurfacePresentModesKHR.html) · Device Command"]
    unsafe fn get_device_group_surface_present_modes_khr(
        &self,
        surface: crate::extensions::khr_surface::SurfaceKHR,
        modes: Option<crate::extensions::khr_swapchain::DeviceGroupPresentModeFlagsKHR>,
    ) -> crate::utils::VulkanResult<crate::extensions::khr_swapchain::DeviceGroupPresentModeFlagsKHR>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAcquireNextImage2KHR.html) · Device Command"]
    unsafe fn acquire_next_image2_khr(
        &self,
        acquire_info: &crate::extensions::khr_swapchain::AcquireNextImageInfoKHR,
        image_index: Option<u32>,
    ) -> crate::utils::VulkanResult<u32>;
}
impl KhrDeviceGroupDeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceGroupPeerMemoryFeaturesKHR.html) · Device Command"]
    unsafe fn get_device_group_peer_memory_features_khr(
        &self,
        heap_index: u32,
        local_device_index: u32,
        remote_device_index: u32,
        peer_memory_features: Option<crate::vk1_1::PeerMemoryFeatureFlags>,
    ) -> crate::vk1_1::PeerMemoryFeatureFlags {
        let function = self
            .khr_device_group
            .as_ref()
            .expect("`khr_device_group` not loaded")
            .get_device_group_peer_memory_features_khr;
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
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDeviceMaskKHR.html) · Device Command"]
    unsafe fn cmd_set_device_mask_khr(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        device_mask: u32,
    ) -> () {
        let function = self
            .khr_device_group
            .as_ref()
            .expect("`khr_device_group` not loaded")
            .cmd_set_device_mask_khr;
        let _val = function(command_buffer, device_mask);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDispatchBaseKHR.html) · Device Command"]
    unsafe fn cmd_dispatch_base_khr(
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
            .khr_device_group
            .as_ref()
            .expect("`khr_device_group` not loaded")
            .cmd_dispatch_base_khr;
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
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceGroupPresentCapabilitiesKHR.html) · Device Command"]
    unsafe fn get_device_group_present_capabilities_khr(
        &self,
        device_group_present_capabilities: Option<
            crate::extensions::khr_swapchain::DeviceGroupPresentCapabilitiesKHR,
        >,
    ) -> crate::utils::VulkanResult<
        crate::extensions::khr_swapchain::DeviceGroupPresentCapabilitiesKHR,
    > {
        let function = self
            .khr_device_group
            .as_ref()
            .expect("`khr_device_group` not loaded")
            .get_device_group_present_capabilities_khr;
        let mut device_group_present_capabilities =
            device_group_present_capabilities.unwrap_or_else(|| Default::default());
        let _val = function(self.handle, &mut device_group_present_capabilities);
        crate::utils::VulkanResult::new(_val, device_group_present_capabilities)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceGroupSurfacePresentModesKHR.html) · Device Command"]
    unsafe fn get_device_group_surface_present_modes_khr(
        &self,
        surface: crate::extensions::khr_surface::SurfaceKHR,
        modes: Option<crate::extensions::khr_swapchain::DeviceGroupPresentModeFlagsKHR>,
    ) -> crate::utils::VulkanResult<crate::extensions::khr_swapchain::DeviceGroupPresentModeFlagsKHR>
    {
        let function = self
            .khr_device_group
            .as_ref()
            .expect("`khr_device_group` not loaded")
            .get_device_group_surface_present_modes_khr;
        let mut modes = modes.unwrap_or_else(|| Default::default());
        let _val = function(self.handle, surface, &mut modes);
        crate::utils::VulkanResult::new(_val, modes)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAcquireNextImage2KHR.html) · Device Command"]
    unsafe fn acquire_next_image2_khr(
        &self,
        acquire_info: &crate::extensions::khr_swapchain::AcquireNextImageInfoKHR,
        image_index: Option<u32>,
    ) -> crate::utils::VulkanResult<u32> {
        let function = self
            .khr_device_group
            .as_ref()
            .expect("`khr_device_group` not loaded")
            .acquire_next_image2_khr;
        let mut image_index = image_index.unwrap_or_else(|| Default::default());
        let _val = function(self.handle, acquire_info, &mut image_index);
        crate::utils::VulkanResult::new(_val, image_index)
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPeerMemoryFeatureFlagsKHR.html) · Alias"]
pub type PeerMemoryFeatureFlagsKHR = crate::vk1_1::PeerMemoryFeatureFlags;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryAllocateFlagsKHR.html) · Alias"]
pub type MemoryAllocateFlagsKHR = crate::vk1_1::MemoryAllocateFlags;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryAllocateFlagsInfoKHR.html) · Alias"]
pub type MemoryAllocateFlagsInfoKHR = crate::vk1_1::MemoryAllocateFlagsInfo;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceGroupRenderPassBeginInfoKHR.html) · Alias"]
pub type DeviceGroupRenderPassBeginInfoKHR = crate::vk1_1::DeviceGroupRenderPassBeginInfo;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceGroupCommandBufferBeginInfoKHR.html) · Alias"]
pub type DeviceGroupCommandBufferBeginInfoKHR = crate::vk1_1::DeviceGroupCommandBufferBeginInfo;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceGroupSubmitInfoKHR.html) · Alias"]
pub type DeviceGroupSubmitInfoKHR = crate::vk1_1::DeviceGroupSubmitInfo;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceGroupBindSparseInfoKHR.html) · Alias"]
pub type DeviceGroupBindSparseInfoKHR = crate::vk1_1::DeviceGroupBindSparseInfo;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindBufferMemoryDeviceGroupInfoKHR.html) · Alias"]
pub type BindBufferMemoryDeviceGroupInfoKHR = crate::vk1_1::BindBufferMemoryDeviceGroupInfo;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindImageMemoryDeviceGroupInfoKHR.html) · Alias"]
pub type BindImageMemoryDeviceGroupInfoKHR = crate::vk1_1::BindImageMemoryDeviceGroupInfo;
