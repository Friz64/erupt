# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_KHR_swapchain.html)\n\n## Extends\n- [`ImageLayout`](../../vk1_0/struct.ImageLayout.html)\n- [`ObjectType`](../../vk1_0/struct.ObjectType.html)\n- [`Result`](../../vk1_0/struct.Result.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_SWAPCHAIN_SPEC_VERSION: u32 = 70;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_SWAPCHAIN_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_swapchain");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateSwapchainKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateSwapchainKHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_create_info: *const crate::extensions::khr_swapchain::SwapchainCreateInfoKHR,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
    p_swapchain: *mut crate::extensions::khr_swapchain::SwapchainKHR,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroySwapchainKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroySwapchainKHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSwapchainImagesKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetSwapchainImagesKHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
    p_swapchain_image_count: *mut u32,
    p_swapchain_images: *mut crate::vk1_0::Image,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAcquireNextImageKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkAcquireNextImageKHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
    timeout: u64,
    semaphore: crate::vk1_0::Semaphore,
    fence: crate::vk1_0::Fence,
    p_image_index: *mut u32,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueuePresentKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkQueuePresentKHR = unsafe extern "system" fn(
    queue: crate::vk1_0::Queue,
    p_present_info: *const crate::extensions::khr_swapchain::PresentInfoKHR,
) -> crate::vk1_0::Result;
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
#[doc = "Provides Instance Commands for [`KhrSwapchainInstanceLoaderExt`](trait.KhrSwapchainInstanceLoaderExt.html)"]
pub struct KhrSwapchainInstanceCommands {
    pub get_physical_device_present_rectangles_khr: PFN_vkGetPhysicalDevicePresentRectanglesKHR,
}
impl KhrSwapchainInstanceCommands {
    #[inline]
    pub fn load(loader: &crate::InstanceLoader) -> Option<KhrSwapchainInstanceCommands> {
        unsafe {
            Some(KhrSwapchainInstanceCommands {
                get_physical_device_present_rectangles_khr: std::mem::transmute(
                    loader.symbol("vkGetPhysicalDevicePresentRectanglesKHR")?,
                ),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`KhrSwapchainInstanceCommands`](struct.KhrSwapchainInstanceCommands.html)"]
pub trait KhrSwapchainInstanceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDevicePresentRectanglesKHR.html) · Instance Command"]
    unsafe fn get_physical_device_present_rectangles_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        surface: crate::extensions::khr_surface::SurfaceKHR,
        rect_count: Option<u32>,
    ) -> crate::utils::VulkanResult<Vec<crate::vk1_0::Rect2D>>;
}
impl KhrSwapchainInstanceLoaderExt for crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDevicePresentRectanglesKHR.html) · Instance Command"]
    unsafe fn get_physical_device_present_rectangles_khr(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        surface: crate::extensions::khr_surface::SurfaceKHR,
        rect_count: Option<u32>,
    ) -> crate::utils::VulkanResult<Vec<crate::vk1_0::Rect2D>> {
        let function = self
            .khr_swapchain
            .as_ref()
            .expect("`khr_swapchain` not loaded")
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
#[doc = "Provides Device Commands for [`KhrSwapchainDeviceLoaderExt`](trait.KhrSwapchainDeviceLoaderExt.html)"]
pub struct KhrSwapchainDeviceCommands {
    pub create_swapchain_khr: PFN_vkCreateSwapchainKHR,
    pub destroy_swapchain_khr: PFN_vkDestroySwapchainKHR,
    pub get_swapchain_images_khr: PFN_vkGetSwapchainImagesKHR,
    pub acquire_next_image_khr: PFN_vkAcquireNextImageKHR,
    pub queue_present_khr: PFN_vkQueuePresentKHR,
    pub get_device_group_present_capabilities_khr: PFN_vkGetDeviceGroupPresentCapabilitiesKHR,
    pub get_device_group_surface_present_modes_khr: PFN_vkGetDeviceGroupSurfacePresentModesKHR,
    pub acquire_next_image2_khr: PFN_vkAcquireNextImage2KHR,
}
impl KhrSwapchainDeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<KhrSwapchainDeviceCommands> {
        unsafe {
            Some(KhrSwapchainDeviceCommands {
                create_swapchain_khr: std::mem::transmute(loader.symbol("vkCreateSwapchainKHR")?),
                destroy_swapchain_khr: std::mem::transmute(loader.symbol("vkDestroySwapchainKHR")?),
                get_swapchain_images_khr: std::mem::transmute(
                    loader.symbol("vkGetSwapchainImagesKHR")?,
                ),
                acquire_next_image_khr: std::mem::transmute(
                    loader.symbol("vkAcquireNextImageKHR")?,
                ),
                queue_present_khr: std::mem::transmute(loader.symbol("vkQueuePresentKHR")?),
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
#[doc = "Provides high level command wrappers for [`KhrSwapchainDeviceCommands`](struct.KhrSwapchainDeviceCommands.html)"]
pub trait KhrSwapchainDeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateSwapchainKHR.html) · Device Command"]
    unsafe fn create_swapchain_khr(
        &self,
        create_info: &crate::extensions::khr_swapchain::SwapchainCreateInfoKHR,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        swapchain: Option<crate::extensions::khr_swapchain::SwapchainKHR>,
    ) -> crate::utils::VulkanResult<crate::extensions::khr_swapchain::SwapchainKHR>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroySwapchainKHR.html) · Device Command"]
    unsafe fn destroy_swapchain_khr(
        &self,
        swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSwapchainImagesKHR.html) · Device Command"]
    unsafe fn get_swapchain_images_khr(
        &self,
        swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
        swapchain_image_count: Option<u32>,
    ) -> crate::utils::VulkanResult<Vec<crate::vk1_0::Image>>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAcquireNextImageKHR.html) · Device Command"]
    unsafe fn acquire_next_image_khr(
        &self,
        swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
        timeout: u64,
        semaphore: crate::vk1_0::Semaphore,
        fence: crate::vk1_0::Fence,
        image_index: Option<u32>,
    ) -> crate::utils::VulkanResult<u32>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueuePresentKHR.html) · Device Command"]
    unsafe fn queue_present_khr(
        &self,
        queue: crate::vk1_0::Queue,
        present_info: &crate::extensions::khr_swapchain::PresentInfoKHR,
    ) -> crate::utils::VulkanResult<()>;
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
impl KhrSwapchainDeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateSwapchainKHR.html) · Device Command"]
    unsafe fn create_swapchain_khr(
        &self,
        create_info: &crate::extensions::khr_swapchain::SwapchainCreateInfoKHR,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        swapchain: Option<crate::extensions::khr_swapchain::SwapchainKHR>,
    ) -> crate::utils::VulkanResult<crate::extensions::khr_swapchain::SwapchainKHR> {
        let function = self
            .khr_swapchain
            .as_ref()
            .expect("`khr_swapchain` not loaded")
            .create_swapchain_khr;
        let mut swapchain = swapchain.unwrap_or_else(|| Default::default());
        let _val = function(
            self.handle,
            create_info,
            if let Some(allocator) = allocator {
                allocator
            } else {
                std::ptr::null()
            },
            &mut swapchain,
        );
        crate::utils::VulkanResult::new(_val, swapchain)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroySwapchainKHR.html) · Device Command"]
    unsafe fn destroy_swapchain_khr(
        &self,
        swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
    ) -> () {
        let function = self
            .khr_swapchain
            .as_ref()
            .expect("`khr_swapchain` not loaded")
            .destroy_swapchain_khr;
        let _val = function(
            self.handle,
            swapchain,
            if let Some(allocator) = allocator {
                allocator
            } else {
                std::ptr::null()
            },
        );
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSwapchainImagesKHR.html) · Device Command"]
    unsafe fn get_swapchain_images_khr(
        &self,
        swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
        swapchain_image_count: Option<u32>,
    ) -> crate::utils::VulkanResult<Vec<crate::vk1_0::Image>> {
        let function = self
            .khr_swapchain
            .as_ref()
            .expect("`khr_swapchain` not loaded")
            .get_swapchain_images_khr;
        let mut swapchain_image_count = swapchain_image_count.unwrap_or_else(|| {
            let mut val = Default::default();
            function(self.handle, swapchain, &mut val, std::ptr::null_mut());
            val
        });
        let mut swapchain_images = vec![Default::default(); swapchain_image_count as _];
        let _val = function(
            self.handle,
            swapchain,
            &mut swapchain_image_count,
            swapchain_images.as_mut_ptr(),
        );
        crate::utils::VulkanResult::new(_val, swapchain_images)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAcquireNextImageKHR.html) · Device Command"]
    unsafe fn acquire_next_image_khr(
        &self,
        swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
        timeout: u64,
        semaphore: crate::vk1_0::Semaphore,
        fence: crate::vk1_0::Fence,
        image_index: Option<u32>,
    ) -> crate::utils::VulkanResult<u32> {
        let function = self
            .khr_swapchain
            .as_ref()
            .expect("`khr_swapchain` not loaded")
            .acquire_next_image_khr;
        let mut image_index = image_index.unwrap_or_else(|| Default::default());
        let _val = function(
            self.handle,
            swapchain,
            timeout,
            semaphore,
            fence,
            &mut image_index,
        );
        crate::utils::VulkanResult::new(_val, image_index)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueuePresentKHR.html) · Device Command"]
    unsafe fn queue_present_khr(
        &self,
        queue: crate::vk1_0::Queue,
        present_info: &crate::extensions::khr_swapchain::PresentInfoKHR,
    ) -> crate::utils::VulkanResult<()> {
        let function = self
            .khr_swapchain
            .as_ref()
            .expect("`khr_swapchain` not loaded")
            .queue_present_khr;
        let _val = function(queue, present_info);
        crate::utils::VulkanResult::new(_val, ())
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
            .khr_swapchain
            .as_ref()
            .expect("`khr_swapchain` not loaded")
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
            .khr_swapchain
            .as_ref()
            .expect("`khr_swapchain` not loaded")
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
            .khr_swapchain
            .as_ref()
            .expect("`khr_swapchain` not loaded")
            .acquire_next_image2_khr;
        let mut image_index = image_index.unwrap_or_else(|| Default::default());
        let _val = function(self.handle, acquire_info, &mut image_index);
        crate::utils::VulkanResult::new(_val, image_index)
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSwapchainCreateInfoKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SwapchainCreateInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::khr_swapchain::SwapchainCreateFlagsKHR,
    pub surface: crate::extensions::khr_surface::SurfaceKHR,
    pub min_image_count: u32,
    pub image_format: crate::vk1_0::Format,
    pub image_color_space: crate::extensions::khr_surface::ColorSpaceKHR,
    pub image_extent: crate::vk1_0::Extent2D,
    pub image_array_layers: u32,
    pub image_usage: crate::vk1_0::ImageUsageFlags,
    pub image_sharing_mode: crate::vk1_0::SharingMode,
    pub queue_family_index_count: u32,
    pub p_queue_family_indices: *const u32,
    pub pre_transform: crate::extensions::khr_surface::SurfaceTransformFlagBitsKHR,
    pub composite_alpha: crate::extensions::khr_surface::CompositeAlphaFlagBitsKHR,
    pub present_mode: crate::extensions::khr_surface::PresentModeKHR,
    pub clipped: crate::vk1_0::Bool32,
    pub old_swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
}
impl SwapchainCreateInfoKHR {
    #[inline]
    pub fn builder<'a>(self) -> SwapchainCreateInfoKHRBuilder<'a> {
        SwapchainCreateInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for SwapchainCreateInfoKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("SwapchainCreateInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("surface", &self.surface)
            .field("min_image_count", &self.min_image_count)
            .field("image_format", &self.image_format)
            .field("image_color_space", &self.image_color_space)
            .field("image_extent", &self.image_extent)
            .field("image_array_layers", &self.image_array_layers)
            .field("image_usage", &self.image_usage)
            .field("image_sharing_mode", &self.image_sharing_mode)
            .field("queue_family_index_count", &self.queue_family_index_count)
            .field("p_queue_family_indices", &self.p_queue_family_indices)
            .field("pre_transform", &self.pre_transform)
            .field("composite_alpha", &self.composite_alpha)
            .field("present_mode", &self.present_mode)
            .field("clipped", &(self.clipped != 0))
            .field("old_swapchain", &self.old_swapchain)
            .finish()
    }
}
impl Default for SwapchainCreateInfoKHR {
    fn default() -> SwapchainCreateInfoKHR {
        SwapchainCreateInfoKHR {
            s_type: crate::vk1_0::StructureType::SWAPCHAIN_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            flags: Default::default(),
            surface: Default::default(),
            min_image_count: Default::default(),
            image_format: Default::default(),
            image_color_space: Default::default(),
            image_extent: Default::default(),
            image_array_layers: Default::default(),
            image_usage: Default::default(),
            image_sharing_mode: Default::default(),
            queue_family_index_count: Default::default(),
            p_queue_family_indices: std::ptr::null(),
            pre_transform: Default::default(),
            composite_alpha: Default::default(),
            present_mode: Default::default(),
            clipped: Default::default(),
            old_swapchain: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`SwapchainCreateInfoKHR`](struct.SwapchainCreateInfoKHR.html)"]
#[repr(transparent)]
pub struct SwapchainCreateInfoKHRBuilder<'a>(
    SwapchainCreateInfoKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> SwapchainCreateInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> SwapchainCreateInfoKHRBuilder<'a> {
        SwapchainCreateInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn flags(
        mut self,
        flags: crate::extensions::khr_swapchain::SwapchainCreateFlagsKHR,
    ) -> Self {
        self.0.flags = flags;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn surface(mut self, surface: crate::extensions::khr_surface::SurfaceKHR) -> Self {
        self.0.surface = surface;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn min_image_count(mut self, min_image_count: u32) -> Self {
        self.0.min_image_count = min_image_count;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn image_format(mut self, image_format: crate::vk1_0::Format) -> Self {
        self.0.image_format = image_format;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn image_color_space(
        mut self,
        image_color_space: crate::extensions::khr_surface::ColorSpaceKHR,
    ) -> Self {
        self.0.image_color_space = image_color_space;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn image_extent(mut self, image_extent: crate::vk1_0::Extent2D) -> Self {
        self.0.image_extent = image_extent;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn image_array_layers(mut self, image_array_layers: u32) -> Self {
        self.0.image_array_layers = image_array_layers;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn image_usage(mut self, image_usage: crate::vk1_0::ImageUsageFlags) -> Self {
        self.0.image_usage = image_usage;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn image_sharing_mode(mut self, image_sharing_mode: crate::vk1_0::SharingMode) -> Self {
        self.0.image_sharing_mode = image_sharing_mode;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn queue_family_indices(mut self, queue_family_indices: &'a [u32]) -> Self {
        self.0.queue_family_index_count = queue_family_indices.len() as _;
        self.0.p_queue_family_indices = queue_family_indices.as_ptr() as _;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn pre_transform(
        mut self,
        pre_transform: crate::extensions::khr_surface::SurfaceTransformFlagBitsKHR,
    ) -> Self {
        self.0.pre_transform = pre_transform;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn composite_alpha(
        mut self,
        composite_alpha: crate::extensions::khr_surface::CompositeAlphaFlagBitsKHR,
    ) -> Self {
        self.0.composite_alpha = composite_alpha;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn present_mode(
        mut self,
        present_mode: crate::extensions::khr_surface::PresentModeKHR,
    ) -> Self {
        self.0.present_mode = present_mode;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn clipped(mut self, clipped: bool) -> Self {
        self.0.clipped = clipped as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn old_swapchain(
        mut self,
        old_swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
    ) -> Self {
        self.0.old_swapchain = old_swapchain;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> SwapchainCreateInfoKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for SwapchainCreateInfoKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for SwapchainCreateInfoKHRBuilder<'a> {
    type Target = SwapchainCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SwapchainCreateInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSwapchainCreateFlagBitsKHR.html) · Flag Bits of [`SwapchainCreateFlagsKHR`](struct.SwapchainCreateFlagsKHR.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct SwapchainCreateFlagBitsKHR(pub u32);
impl SwapchainCreateFlagBitsKHR {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> SwapchainCreateFlagsKHR {
        SwapchainCreateFlagsKHR::from_bits_truncate(self.0)
    }
}
#[doc = "[Part of `extensions::khr_swapchain`](../../extensions/khr_swapchain/index.html)"]
impl SwapchainCreateFlagBitsKHR {}
#[doc = "[Part of `extensions::khr_device_group`](../../extensions/khr_device_group/index.html)"]
impl SwapchainCreateFlagBitsKHR {
    pub const SPLIT_INSTANCE_BIND_REGIONS_KHR: Self = Self(0x00000001);
}
#[doc = "[Part of `extensions::khr_swapchain`](../../extensions/khr_swapchain/index.html)"]
impl SwapchainCreateFlagBitsKHR {
    pub const PROTECTED_KHR: Self = Self(0x00000002);
}
#[doc = "[Part of `extensions::khr_swapchain_mutable_format`](../../extensions/khr_swapchain_mutable_format/index.html)"]
impl SwapchainCreateFlagBitsKHR {
    pub const MUTABLE_FORMAT_KHR: Self = Self(0x00000004);
}
impl std::fmt::Debug for SwapchainCreateFlagBitsKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::SPLIT_INSTANCE_BIND_REGIONS_KHR => "SPLIT_INSTANCE_BIND_REGIONS_KHR",
            &Self::PROTECTED_KHR => "PROTECTED_KHR",
            &Self::MUTABLE_FORMAT_KHR => "MUTABLE_FORMAT_KHR",
            _ => "(Unknown)",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSwapchainCreateFlagsKHR.html) · Flags of [`SwapchainCreateFlagBitsKHR`](struct.SwapchainCreateFlagBitsKHR.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct SwapchainCreateFlagsKHR : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; const SPLIT_INSTANCE_BIND_REGIONS_KHR = SwapchainCreateFlagBitsKHR :: SPLIT_INSTANCE_BIND_REGIONS_KHR . 0 ; const PROTECTED_KHR = SwapchainCreateFlagBitsKHR :: PROTECTED_KHR . 0 ; const MUTABLE_FORMAT_KHR = SwapchainCreateFlagBitsKHR :: MUTABLE_FORMAT_KHR . 0 ; } }
crate :: non_dispatchable_handle ! ( SwapchainKHR , SWAPCHAIN_KHR , doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSwapchainKHR.html) · Non-dispatchable Handle" ) ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPresentInfoKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PresentInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub wait_semaphore_count: u32,
    pub p_wait_semaphores: *const crate::vk1_0::Semaphore,
    pub swapchain_count: u32,
    pub p_swapchains: *const crate::extensions::khr_swapchain::SwapchainKHR,
    pub p_image_indices: *const u32,
    pub p_results: *mut crate::vk1_0::Result,
}
impl PresentInfoKHR {
    #[inline]
    pub fn builder<'a>(self) -> PresentInfoKHRBuilder<'a> {
        PresentInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PresentInfoKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PresentInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("wait_semaphore_count", &self.wait_semaphore_count)
            .field("p_wait_semaphores", &self.p_wait_semaphores)
            .field("swapchain_count", &self.swapchain_count)
            .field("p_swapchains", &self.p_swapchains)
            .field("p_image_indices", &self.p_image_indices)
            .field("p_results", &self.p_results)
            .finish()
    }
}
impl Default for PresentInfoKHR {
    fn default() -> PresentInfoKHR {
        PresentInfoKHR {
            s_type: crate::vk1_0::StructureType::PRESENT_INFO_KHR,
            p_next: std::ptr::null(),
            wait_semaphore_count: Default::default(),
            p_wait_semaphores: std::ptr::null(),
            swapchain_count: Default::default(),
            p_swapchains: std::ptr::null(),
            p_image_indices: std::ptr::null(),
            p_results: std::ptr::null_mut(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PresentInfoKHR`](struct.PresentInfoKHR.html)"]
#[repr(transparent)]
pub struct PresentInfoKHRBuilder<'a>(PresentInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> PresentInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PresentInfoKHRBuilder<'a> {
        PresentInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn wait_semaphores(mut self, wait_semaphores: &'a [crate::vk1_0::Semaphore]) -> Self {
        self.0.wait_semaphore_count = wait_semaphores.len() as _;
        self.0.p_wait_semaphores = wait_semaphores.as_ptr() as _;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn swapchains(
        mut self,
        swapchains: &'a [crate::extensions::khr_swapchain::SwapchainKHR],
    ) -> Self {
        self.0.swapchain_count = swapchains.len() as _;
        self.0.p_swapchains = swapchains.as_ptr() as _;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn image_indices(mut self, image_indices: &'a [u32]) -> Self {
        self.0.swapchain_count = image_indices.len() as _;
        self.0.p_image_indices = image_indices.as_ptr() as _;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn results(mut self, results: &'a mut [crate::vk1_0::Result]) -> Self {
        self.0.swapchain_count = results.len() as _;
        self.0.p_results = results.as_mut_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PresentInfoKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for PresentInfoKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PresentInfoKHRBuilder<'a> {
    type Target = PresentInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PresentInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceGroupPresentCapabilitiesKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DeviceGroupPresentCapabilitiesKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub present_mask: [u32; crate::vk1_1::MAX_DEVICE_GROUP_SIZE as usize],
    pub modes: crate::extensions::khr_swapchain::DeviceGroupPresentModeFlagsKHR,
}
impl DeviceGroupPresentCapabilitiesKHR {
    #[inline]
    pub fn builder<'a>(self) -> DeviceGroupPresentCapabilitiesKHRBuilder<'a> {
        DeviceGroupPresentCapabilitiesKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for DeviceGroupPresentCapabilitiesKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("DeviceGroupPresentCapabilitiesKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("present_mask", &self.present_mask)
            .field("modes", &self.modes)
            .finish()
    }
}
impl Default for DeviceGroupPresentCapabilitiesKHR {
    fn default() -> DeviceGroupPresentCapabilitiesKHR {
        DeviceGroupPresentCapabilitiesKHR {
            s_type: crate::vk1_0::StructureType::DEVICE_GROUP_PRESENT_CAPABILITIES_KHR,
            p_next: std::ptr::null(),
            present_mask: Default::default(),
            modes: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`DeviceGroupPresentCapabilitiesKHR`](struct.DeviceGroupPresentCapabilitiesKHR.html)"]
#[repr(transparent)]
pub struct DeviceGroupPresentCapabilitiesKHRBuilder<'a>(
    DeviceGroupPresentCapabilitiesKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> DeviceGroupPresentCapabilitiesKHRBuilder<'a> {
    #[inline]
    pub fn new() -> DeviceGroupPresentCapabilitiesKHRBuilder<'a> {
        DeviceGroupPresentCapabilitiesKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn present_mask(
        mut self,
        present_mask: [u32; crate::vk1_1::MAX_DEVICE_GROUP_SIZE as usize],
    ) -> Self {
        self.0.present_mask = present_mask;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn modes(
        mut self,
        modes: crate::extensions::khr_swapchain::DeviceGroupPresentModeFlagsKHR,
    ) -> Self {
        self.0.modes = modes;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> DeviceGroupPresentCapabilitiesKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for DeviceGroupPresentCapabilitiesKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for DeviceGroupPresentCapabilitiesKHRBuilder<'a> {
    type Target = DeviceGroupPresentCapabilitiesKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DeviceGroupPresentCapabilitiesKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceGroupPresentModeFlagBitsKHR.html) · Flag Bits of [`DeviceGroupPresentModeFlagsKHR`](struct.DeviceGroupPresentModeFlagsKHR.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct DeviceGroupPresentModeFlagBitsKHR(pub u32);
impl DeviceGroupPresentModeFlagBitsKHR {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> DeviceGroupPresentModeFlagsKHR {
        DeviceGroupPresentModeFlagsKHR::from_bits_truncate(self.0)
    }
}
#[doc = "[Part of `extensions::khr_swapchain`](../../extensions/khr_swapchain/index.html)"]
impl DeviceGroupPresentModeFlagBitsKHR {
    pub const LOCAL_KHR: Self = Self(0x00000001);
    pub const REMOTE_KHR: Self = Self(0x00000002);
    pub const SUM_KHR: Self = Self(0x00000004);
    pub const LOCAL_MULTI_DEVICE_KHR: Self = Self(0x00000008);
}
impl std::fmt::Debug for DeviceGroupPresentModeFlagBitsKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::LOCAL_KHR => "LOCAL_KHR",
            &Self::REMOTE_KHR => "REMOTE_KHR",
            &Self::SUM_KHR => "SUM_KHR",
            &Self::LOCAL_MULTI_DEVICE_KHR => "LOCAL_MULTI_DEVICE_KHR",
            _ => "(Unknown)",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceGroupPresentModeFlagsKHR.html) · Flags of [`DeviceGroupPresentModeFlagBitsKHR`](struct.DeviceGroupPresentModeFlagBitsKHR.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct DeviceGroupPresentModeFlagsKHR : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; const LOCAL_KHR = DeviceGroupPresentModeFlagBitsKHR :: LOCAL_KHR . 0 ; const REMOTE_KHR = DeviceGroupPresentModeFlagBitsKHR :: REMOTE_KHR . 0 ; const SUM_KHR = DeviceGroupPresentModeFlagBitsKHR :: SUM_KHR . 0 ; const LOCAL_MULTI_DEVICE_KHR = DeviceGroupPresentModeFlagBitsKHR :: LOCAL_MULTI_DEVICE_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAcquireNextImageInfoKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AcquireNextImageInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
    pub timeout: u64,
    pub semaphore: crate::vk1_0::Semaphore,
    pub fence: crate::vk1_0::Fence,
    pub device_mask: u32,
}
impl AcquireNextImageInfoKHR {
    #[inline]
    pub fn builder<'a>(self) -> AcquireNextImageInfoKHRBuilder<'a> {
        AcquireNextImageInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for AcquireNextImageInfoKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("AcquireNextImageInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("swapchain", &self.swapchain)
            .field("timeout", &self.timeout)
            .field("semaphore", &self.semaphore)
            .field("fence", &self.fence)
            .field("device_mask", &self.device_mask)
            .finish()
    }
}
impl Default for AcquireNextImageInfoKHR {
    fn default() -> AcquireNextImageInfoKHR {
        AcquireNextImageInfoKHR {
            s_type: crate::vk1_0::StructureType::ACQUIRE_NEXT_IMAGE_INFO_KHR,
            p_next: std::ptr::null(),
            swapchain: Default::default(),
            timeout: Default::default(),
            semaphore: Default::default(),
            fence: Default::default(),
            device_mask: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`AcquireNextImageInfoKHR`](struct.AcquireNextImageInfoKHR.html)"]
#[repr(transparent)]
pub struct AcquireNextImageInfoKHRBuilder<'a>(
    AcquireNextImageInfoKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> AcquireNextImageInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> AcquireNextImageInfoKHRBuilder<'a> {
        AcquireNextImageInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn swapchain(mut self, swapchain: crate::extensions::khr_swapchain::SwapchainKHR) -> Self {
        self.0.swapchain = swapchain;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn timeout(mut self, timeout: u64) -> Self {
        self.0.timeout = timeout;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn semaphore(mut self, semaphore: crate::vk1_0::Semaphore) -> Self {
        self.0.semaphore = semaphore;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn fence(mut self, fence: crate::vk1_0::Fence) -> Self {
        self.0.fence = fence;
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
    pub unsafe fn discard(self) -> AcquireNextImageInfoKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for AcquireNextImageInfoKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for AcquireNextImageInfoKHRBuilder<'a> {
    type Target = AcquireNextImageInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for AcquireNextImageInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageSwapchainCreateInfoKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImageSwapchainCreateInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
}
impl ImageSwapchainCreateInfoKHR {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByImageSwapchainCreateInfoKHR,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> ImageSwapchainCreateInfoKHRBuilder<'a> {
        ImageSwapchainCreateInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ImageSwapchainCreateInfoKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ImageSwapchainCreateInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("swapchain", &self.swapchain)
            .finish()
    }
}
impl Default for ImageSwapchainCreateInfoKHR {
    fn default() -> ImageSwapchainCreateInfoKHR {
        ImageSwapchainCreateInfoKHR {
            s_type: crate::vk1_0::StructureType::IMAGE_SWAPCHAIN_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            swapchain: Default::default(),
        }
    }
}
#[doc = "Used by [`ImageSwapchainCreateInfoKHR::extend`](struct.ImageSwapchainCreateInfoKHR.html#method.extend)"]
pub trait ExtendableByImageSwapchainCreateInfoKHR {}
impl ExtendableByImageSwapchainCreateInfoKHR for crate::vk1_0::ImageCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`ImageSwapchainCreateInfoKHR`](struct.ImageSwapchainCreateInfoKHR.html)"]
#[repr(transparent)]
pub struct ImageSwapchainCreateInfoKHRBuilder<'a>(
    ImageSwapchainCreateInfoKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> ImageSwapchainCreateInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> ImageSwapchainCreateInfoKHRBuilder<'a> {
        ImageSwapchainCreateInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn swapchain(mut self, swapchain: crate::extensions::khr_swapchain::SwapchainKHR) -> Self {
        self.0.swapchain = swapchain;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> ImageSwapchainCreateInfoKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for ImageSwapchainCreateInfoKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for ImageSwapchainCreateInfoKHRBuilder<'a> {
    type Target = ImageSwapchainCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ImageSwapchainCreateInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindImageMemorySwapchainInfoKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BindImageMemorySwapchainInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
    pub image_index: u32,
}
impl BindImageMemorySwapchainInfoKHR {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByBindImageMemorySwapchainInfoKHR,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> BindImageMemorySwapchainInfoKHRBuilder<'a> {
        BindImageMemorySwapchainInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for BindImageMemorySwapchainInfoKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("BindImageMemorySwapchainInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("swapchain", &self.swapchain)
            .field("image_index", &self.image_index)
            .finish()
    }
}
impl Default for BindImageMemorySwapchainInfoKHR {
    fn default() -> BindImageMemorySwapchainInfoKHR {
        BindImageMemorySwapchainInfoKHR {
            s_type: crate::vk1_0::StructureType::BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR,
            p_next: std::ptr::null(),
            swapchain: Default::default(),
            image_index: Default::default(),
        }
    }
}
#[doc = "Used by [`BindImageMemorySwapchainInfoKHR::extend`](struct.BindImageMemorySwapchainInfoKHR.html#method.extend)"]
pub trait ExtendableByBindImageMemorySwapchainInfoKHR {}
impl ExtendableByBindImageMemorySwapchainInfoKHR for crate::vk1_1::BindImageMemoryInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`BindImageMemorySwapchainInfoKHR`](struct.BindImageMemorySwapchainInfoKHR.html)"]
#[repr(transparent)]
pub struct BindImageMemorySwapchainInfoKHRBuilder<'a>(
    BindImageMemorySwapchainInfoKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> BindImageMemorySwapchainInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> BindImageMemorySwapchainInfoKHRBuilder<'a> {
        BindImageMemorySwapchainInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn swapchain(mut self, swapchain: crate::extensions::khr_swapchain::SwapchainKHR) -> Self {
        self.0.swapchain = swapchain;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn image_index(mut self, image_index: u32) -> Self {
        self.0.image_index = image_index;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> BindImageMemorySwapchainInfoKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for BindImageMemorySwapchainInfoKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for BindImageMemorySwapchainInfoKHRBuilder<'a> {
    type Target = BindImageMemorySwapchainInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for BindImageMemorySwapchainInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceGroupPresentInfoKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DeviceGroupPresentInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub swapchain_count: u32,
    pub p_device_masks: *const u32,
    pub mode: crate::extensions::khr_swapchain::DeviceGroupPresentModeFlagBitsKHR,
}
impl DeviceGroupPresentInfoKHR {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByDeviceGroupPresentInfoKHR,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> DeviceGroupPresentInfoKHRBuilder<'a> {
        DeviceGroupPresentInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for DeviceGroupPresentInfoKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("DeviceGroupPresentInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("swapchain_count", &self.swapchain_count)
            .field("p_device_masks", &self.p_device_masks)
            .field("mode", &self.mode)
            .finish()
    }
}
impl Default for DeviceGroupPresentInfoKHR {
    fn default() -> DeviceGroupPresentInfoKHR {
        DeviceGroupPresentInfoKHR {
            s_type: crate::vk1_0::StructureType::DEVICE_GROUP_PRESENT_INFO_KHR,
            p_next: std::ptr::null(),
            swapchain_count: Default::default(),
            p_device_masks: std::ptr::null(),
            mode: Default::default(),
        }
    }
}
#[doc = "Used by [`DeviceGroupPresentInfoKHR::extend`](struct.DeviceGroupPresentInfoKHR.html#method.extend)"]
pub trait ExtendableByDeviceGroupPresentInfoKHR {}
impl ExtendableByDeviceGroupPresentInfoKHR for crate::extensions::khr_swapchain::PresentInfoKHR {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`DeviceGroupPresentInfoKHR`](struct.DeviceGroupPresentInfoKHR.html)"]
#[repr(transparent)]
pub struct DeviceGroupPresentInfoKHRBuilder<'a>(
    DeviceGroupPresentInfoKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> DeviceGroupPresentInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> DeviceGroupPresentInfoKHRBuilder<'a> {
        DeviceGroupPresentInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn device_masks(mut self, device_masks: &'a [u32]) -> Self {
        self.0.swapchain_count = device_masks.len() as _;
        self.0.p_device_masks = device_masks.as_ptr() as _;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn mode(
        mut self,
        mode: crate::extensions::khr_swapchain::DeviceGroupPresentModeFlagBitsKHR,
    ) -> Self {
        self.0.mode = mode;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> DeviceGroupPresentInfoKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for DeviceGroupPresentInfoKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for DeviceGroupPresentInfoKHRBuilder<'a> {
    type Target = DeviceGroupPresentInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DeviceGroupPresentInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceGroupSwapchainCreateInfoKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DeviceGroupSwapchainCreateInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub modes: crate::extensions::khr_swapchain::DeviceGroupPresentModeFlagsKHR,
}
impl DeviceGroupSwapchainCreateInfoKHR {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByDeviceGroupSwapchainCreateInfoKHR,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> DeviceGroupSwapchainCreateInfoKHRBuilder<'a> {
        DeviceGroupSwapchainCreateInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for DeviceGroupSwapchainCreateInfoKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("DeviceGroupSwapchainCreateInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("modes", &self.modes)
            .finish()
    }
}
impl Default for DeviceGroupSwapchainCreateInfoKHR {
    fn default() -> DeviceGroupSwapchainCreateInfoKHR {
        DeviceGroupSwapchainCreateInfoKHR {
            s_type: crate::vk1_0::StructureType::DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            modes: Default::default(),
        }
    }
}
#[doc = "Used by [`DeviceGroupSwapchainCreateInfoKHR::extend`](struct.DeviceGroupSwapchainCreateInfoKHR.html#method.extend)"]
pub trait ExtendableByDeviceGroupSwapchainCreateInfoKHR {}
impl ExtendableByDeviceGroupSwapchainCreateInfoKHR
    for crate::extensions::khr_swapchain::SwapchainCreateInfoKHR
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`DeviceGroupSwapchainCreateInfoKHR`](struct.DeviceGroupSwapchainCreateInfoKHR.html)"]
#[repr(transparent)]
pub struct DeviceGroupSwapchainCreateInfoKHRBuilder<'a>(
    DeviceGroupSwapchainCreateInfoKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> DeviceGroupSwapchainCreateInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> DeviceGroupSwapchainCreateInfoKHRBuilder<'a> {
        DeviceGroupSwapchainCreateInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn modes(
        mut self,
        modes: crate::extensions::khr_swapchain::DeviceGroupPresentModeFlagsKHR,
    ) -> Self {
        self.0.modes = modes;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> DeviceGroupSwapchainCreateInfoKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for DeviceGroupSwapchainCreateInfoKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for DeviceGroupSwapchainCreateInfoKHRBuilder<'a> {
    type Target = DeviceGroupSwapchainCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DeviceGroupSwapchainCreateInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
