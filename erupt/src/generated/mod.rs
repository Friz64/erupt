#[doc = r" Provides Vulkan extension items"]
pub mod extensions;
#[doc = r" Loader for entry commands"]
#[doc = r""]
#[doc = r" To create a new loader, call [`new`](#method.new)."]
pub struct EntryLoader<T> {
    arc: std::sync::Arc<()>,
    pub loader: T,
    pub get_instance_proc_addr: crate::vk1_0::PFN_vkGetInstanceProcAddr,
    pub instance_version: u32,
    pub create_instance: Option<vk1_0::PFN_vkCreateInstance>,
    pub enumerate_instance_version: Option<vk1_1::PFN_vkEnumerateInstanceVersion>,
    pub enumerate_instance_layer_properties: Option<vk1_0::PFN_vkEnumerateInstanceLayerProperties>,
    pub enumerate_instance_extension_properties:
        Option<vk1_0::PFN_vkEnumerateInstanceExtensionProperties>,
}
impl<T> EntryLoader<T> {
    #[inline]
    pub fn custom(
        mut loader: T,
        mut symbol: impl FnMut(
            &mut T,
            *const std::os::raw::c_char,
        ) -> Option<crate::vk1_0::PFN_vkVoidFunction>,
    ) -> Result<EntryLoader<T>, crate::LoaderError> {
        let mut symbol = |name| symbol(&mut loader, name);
        let mut version = crate::vk1_0::make_version(1, 0, 0);
        if let Some(function) = symbol(crate::vk1_1::FN_ENUMERATE_INSTANCE_VERSION) {
            let function: crate::vk1_1::PFN_vkEnumerateInstanceVersion =
                unsafe { std::mem::transmute(function) };
            let result = unsafe { function(&mut version) };
            if result.0 < 0 {
                return Err(crate::LoaderError::VulkanError(result));
            }
        }
        let vk1_1 = version >= crate::vk1_0::make_version(1, 1, 0);
        let get_instance_proc_addr = symbol(crate::vk1_0::FN_GET_INSTANCE_PROC_ADDR)
            .ok_or(crate::LoaderError::SymbolNotAvailable)?;
        Ok(EntryLoader {
            arc: std::sync::Arc::new(()),
            get_instance_proc_addr: unsafe { std::mem::transmute(get_instance_proc_addr) },
            instance_version: version,
            create_instance: unsafe {
                match symbol(crate::vk1_0::FN_CREATE_INSTANCE) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            enumerate_instance_version: if vk1_1 {
                unsafe {
                    match symbol(crate::vk1_1::FN_ENUMERATE_INSTANCE_VERSION) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            enumerate_instance_layer_properties: unsafe {
                match symbol(crate::vk1_0::FN_ENUMERATE_INSTANCE_LAYER_PROPERTIES) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            enumerate_instance_extension_properties: unsafe {
                match symbol(crate::vk1_0::FN_ENUMERATE_INSTANCE_EXTENSION_PROPERTIES) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            loader,
        })
    }
}
impl<T> Drop for EntryLoader<T> {
    fn drop(&mut self) {
        if std::sync::Arc::weak_count(&self.arc) != 0 {
            panic!("Attempting to drop a entry loader with active references to it");
        }
    }
}
impl<T> std::fmt::Debug for EntryLoader<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Entry")
    }
}
#[doc = r" Loader for instance commands"]
#[doc = r""]
#[doc = r" To create a new loader, call [`new`](#method.new)."]
pub struct InstanceLoader { # [ allow ( dead_code ) ] parent : std :: sync :: Weak < ( ) > , arc : std :: sync :: Arc < ( ) > , selected_instance_version : u32 , pub get_device_proc_addr : crate :: vk1_0 :: PFN_vkGetDeviceProcAddr , pub handle : crate :: vk1_0 :: Instance , pub destroy_instance : Option < vk1_0 :: PFN_vkDestroyInstance > , pub enumerate_physical_devices : Option < vk1_0 :: PFN_vkEnumeratePhysicalDevices > , pub get_instance_proc_addr : Option < vk1_0 :: PFN_vkGetInstanceProcAddr > , pub get_physical_device_properties : Option < vk1_0 :: PFN_vkGetPhysicalDeviceProperties > , pub get_physical_device_queue_family_properties : Option < vk1_0 :: PFN_vkGetPhysicalDeviceQueueFamilyProperties > , pub get_physical_device_memory_properties : Option < vk1_0 :: PFN_vkGetPhysicalDeviceMemoryProperties > , pub get_physical_device_features : Option < vk1_0 :: PFN_vkGetPhysicalDeviceFeatures > , pub get_physical_device_format_properties : Option < vk1_0 :: PFN_vkGetPhysicalDeviceFormatProperties > , pub get_physical_device_image_format_properties : Option < vk1_0 :: PFN_vkGetPhysicalDeviceImageFormatProperties > , pub create_device : Option < vk1_0 :: PFN_vkCreateDevice > , pub enumerate_device_layer_properties : Option < vk1_0 :: PFN_vkEnumerateDeviceLayerProperties > , pub enumerate_device_extension_properties : Option < vk1_0 :: PFN_vkEnumerateDeviceExtensionProperties > , pub get_physical_device_sparse_image_format_properties : Option < vk1_0 :: PFN_vkGetPhysicalDeviceSparseImageFormatProperties > , pub create_android_surface_khr : Option < extensions :: khr_android_surface :: PFN_vkCreateAndroidSurfaceKHR > , pub get_physical_device_display_properties_khr : Option < extensions :: khr_display :: PFN_vkGetPhysicalDeviceDisplayPropertiesKHR > , pub get_physical_device_display_plane_properties_khr : Option < extensions :: khr_display :: PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR > , pub get_display_plane_supported_displays_khr : Option < extensions :: khr_display :: PFN_vkGetDisplayPlaneSupportedDisplaysKHR > , pub get_display_mode_properties_khr : Option < extensions :: khr_display :: PFN_vkGetDisplayModePropertiesKHR > , pub create_display_mode_khr : Option < extensions :: khr_display :: PFN_vkCreateDisplayModeKHR > , pub get_display_plane_capabilities_khr : Option < extensions :: khr_display :: PFN_vkGetDisplayPlaneCapabilitiesKHR > , pub create_display_plane_surface_khr : Option < extensions :: khr_display :: PFN_vkCreateDisplayPlaneSurfaceKHR > , pub destroy_surface_khr : Option < extensions :: khr_surface :: PFN_vkDestroySurfaceKHR > , pub get_physical_device_surface_support_khr : Option < extensions :: khr_surface :: PFN_vkGetPhysicalDeviceSurfaceSupportKHR > , pub get_physical_device_surface_capabilities_khr : Option < extensions :: khr_surface :: PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR > , pub get_physical_device_surface_formats_khr : Option < extensions :: khr_surface :: PFN_vkGetPhysicalDeviceSurfaceFormatsKHR > , pub get_physical_device_surface_present_modes_khr : Option < extensions :: khr_surface :: PFN_vkGetPhysicalDeviceSurfacePresentModesKHR > , pub create_vi_surface_nn : Option < extensions :: nn_vi_surface :: PFN_vkCreateViSurfaceNN > , pub create_wayland_surface_khr : Option < extensions :: khr_wayland_surface :: PFN_vkCreateWaylandSurfaceKHR > , pub get_physical_device_wayland_presentation_support_khr : Option < extensions :: khr_wayland_surface :: PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR > , pub create_win32_surface_khr : Option < extensions :: khr_win32_surface :: PFN_vkCreateWin32SurfaceKHR > , pub get_physical_device_win32_presentation_support_khr : Option < extensions :: khr_win32_surface :: PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR > , pub create_xlib_surface_khr : Option < extensions :: khr_xlib_surface :: PFN_vkCreateXlibSurfaceKHR > , pub get_physical_device_xlib_presentation_support_khr : Option < extensions :: khr_xlib_surface :: PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR > , pub create_xcb_surface_khr : Option < extensions :: khr_xcb_surface :: PFN_vkCreateXcbSurfaceKHR > , pub get_physical_device_xcb_presentation_support_khr : Option < extensions :: khr_xcb_surface :: PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR > , pub create_direct_fb_surface_ext : Option < extensions :: ext_directfb_surface :: PFN_vkCreateDirectFBSurfaceEXT > , pub get_physical_device_direct_fb_presentation_support_ext : Option < extensions :: ext_directfb_surface :: PFN_vkGetPhysicalDeviceDirectFBPresentationSupportEXT > , pub create_image_pipe_surface_fuchsia : Option < extensions :: fuchsia_imagepipe_surface :: PFN_vkCreateImagePipeSurfaceFUCHSIA > , pub create_stream_descriptor_surface_ggp : Option < extensions :: ggp_stream_descriptor_surface :: PFN_vkCreateStreamDescriptorSurfaceGGP > , pub create_debug_report_callback_ext : Option < extensions :: ext_debug_report :: PFN_vkCreateDebugReportCallbackEXT > , pub destroy_debug_report_callback_ext : Option < extensions :: ext_debug_report :: PFN_vkDestroyDebugReportCallbackEXT > , pub debug_report_message_ext : Option < extensions :: ext_debug_report :: PFN_vkDebugReportMessageEXT > , pub get_physical_device_external_image_format_properties_nv : Option < extensions :: nv_external_memory_capabilities :: PFN_vkGetPhysicalDeviceExternalImageFormatPropertiesNV > , pub get_physical_device_features2 : Option < vk1_1 :: PFN_vkGetPhysicalDeviceFeatures2 > , pub get_physical_device_properties2 : Option < vk1_1 :: PFN_vkGetPhysicalDeviceProperties2 > , pub get_physical_device_format_properties2 : Option < vk1_1 :: PFN_vkGetPhysicalDeviceFormatProperties2 > , pub get_physical_device_image_format_properties2 : Option < vk1_1 :: PFN_vkGetPhysicalDeviceImageFormatProperties2 > , pub get_physical_device_queue_family_properties2 : Option < vk1_1 :: PFN_vkGetPhysicalDeviceQueueFamilyProperties2 > , pub get_physical_device_memory_properties2 : Option < vk1_1 :: PFN_vkGetPhysicalDeviceMemoryProperties2 > , pub get_physical_device_sparse_image_format_properties2 : Option < vk1_1 :: PFN_vkGetPhysicalDeviceSparseImageFormatProperties2 > , pub get_physical_device_external_buffer_properties : Option < vk1_1 :: PFN_vkGetPhysicalDeviceExternalBufferProperties > , pub get_physical_device_external_semaphore_properties : Option < vk1_1 :: PFN_vkGetPhysicalDeviceExternalSemaphoreProperties > , pub get_physical_device_external_fence_properties : Option < vk1_1 :: PFN_vkGetPhysicalDeviceExternalFenceProperties > , pub release_display_ext : Option < extensions :: ext_direct_mode_display :: PFN_vkReleaseDisplayEXT > , pub acquire_xlib_display_ext : Option < extensions :: ext_acquire_xlib_display :: PFN_vkAcquireXlibDisplayEXT > , pub get_rand_r_output_display_ext : Option < extensions :: ext_acquire_xlib_display :: PFN_vkGetRandROutputDisplayEXT > , pub get_physical_device_surface_capabilities2_ext : Option < extensions :: ext_display_surface_counter :: PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT > , pub enumerate_physical_device_groups : Option < vk1_1 :: PFN_vkEnumeratePhysicalDeviceGroups > , pub get_physical_device_present_rectangles_khr : Option < extensions :: khr_swapchain :: PFN_vkGetPhysicalDevicePresentRectanglesKHR > , pub create_ios_surface_mvk : Option < extensions :: mvk_ios_surface :: PFN_vkCreateIOSSurfaceMVK > , pub create_mac_os_surface_mvk : Option < extensions :: mvk_macos_surface :: PFN_vkCreateMacOSSurfaceMVK > , pub create_metal_surface_ext : Option < extensions :: ext_metal_surface :: PFN_vkCreateMetalSurfaceEXT > , pub get_physical_device_multisample_properties_ext : Option < extensions :: ext_sample_locations :: PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT > , pub get_physical_device_surface_capabilities2_khr : Option < extensions :: khr_get_surface_capabilities2 :: PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR > , pub get_physical_device_surface_formats2_khr : Option < extensions :: khr_get_surface_capabilities2 :: PFN_vkGetPhysicalDeviceSurfaceFormats2KHR > , pub get_physical_device_display_properties2_khr : Option < extensions :: khr_get_display_properties2 :: PFN_vkGetPhysicalDeviceDisplayProperties2KHR > , pub get_physical_device_display_plane_properties2_khr : Option < extensions :: khr_get_display_properties2 :: PFN_vkGetPhysicalDeviceDisplayPlaneProperties2KHR > , pub get_display_mode_properties2_khr : Option < extensions :: khr_get_display_properties2 :: PFN_vkGetDisplayModeProperties2KHR > , pub get_display_plane_capabilities2_khr : Option < extensions :: khr_get_display_properties2 :: PFN_vkGetDisplayPlaneCapabilities2KHR > , pub get_physical_device_calibrateable_time_domains_ext : Option < extensions :: ext_calibrated_timestamps :: PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsEXT > , pub create_debug_utils_messenger_ext : Option < extensions :: ext_debug_utils :: PFN_vkCreateDebugUtilsMessengerEXT > , pub destroy_debug_utils_messenger_ext : Option < extensions :: ext_debug_utils :: PFN_vkDestroyDebugUtilsMessengerEXT > , pub submit_debug_utils_message_ext : Option < extensions :: ext_debug_utils :: PFN_vkSubmitDebugUtilsMessageEXT > , pub get_physical_device_cooperative_matrix_properties_nv : Option < extensions :: nv_cooperative_matrix :: PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesNV > , pub get_physical_device_surface_present_modes2_ext : Option < extensions :: ext_full_screen_exclusive :: PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT > , pub enumerate_physical_device_queue_family_performance_query_counters_khr : Option < extensions :: khr_performance_query :: PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR > , pub get_physical_device_queue_family_performance_query_passes_khr : Option < extensions :: khr_performance_query :: PFN_vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR > , pub create_headless_surface_ext : Option < extensions :: ext_headless_surface :: PFN_vkCreateHeadlessSurfaceEXT > , pub get_physical_device_supported_framebuffer_mixed_samples_combinations_nv : Option < extensions :: nv_coverage_reduction_mode :: PFN_vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV > , pub get_physical_device_tool_properties_ext : Option < extensions :: ext_tooling_info :: PFN_vkGetPhysicalDeviceToolPropertiesEXT > , pub get_physical_device_features2_khr : Option < extensions :: khr_get_physical_device_properties2 :: PFN_vkGetPhysicalDeviceFeatures2KHR > , pub get_physical_device_properties2_khr : Option < extensions :: khr_get_physical_device_properties2 :: PFN_vkGetPhysicalDeviceProperties2KHR > , pub get_physical_device_format_properties2_khr : Option < extensions :: khr_get_physical_device_properties2 :: PFN_vkGetPhysicalDeviceFormatProperties2KHR > , pub get_physical_device_image_format_properties2_khr : Option < extensions :: khr_get_physical_device_properties2 :: PFN_vkGetPhysicalDeviceImageFormatProperties2KHR > , pub get_physical_device_queue_family_properties2_khr : Option < extensions :: khr_get_physical_device_properties2 :: PFN_vkGetPhysicalDeviceQueueFamilyProperties2KHR > , pub get_physical_device_memory_properties2_khr : Option < extensions :: khr_get_physical_device_properties2 :: PFN_vkGetPhysicalDeviceMemoryProperties2KHR > , pub get_physical_device_sparse_image_format_properties2_khr : Option < extensions :: khr_get_physical_device_properties2 :: PFN_vkGetPhysicalDeviceSparseImageFormatProperties2KHR > , pub get_physical_device_external_buffer_properties_khr : Option < extensions :: khr_external_memory_capabilities :: PFN_vkGetPhysicalDeviceExternalBufferPropertiesKHR > , pub get_physical_device_external_semaphore_properties_khr : Option < extensions :: khr_external_semaphore_capabilities :: PFN_vkGetPhysicalDeviceExternalSemaphorePropertiesKHR > , pub get_physical_device_external_fence_properties_khr : Option < extensions :: khr_external_fence_capabilities :: PFN_vkGetPhysicalDeviceExternalFencePropertiesKHR > , pub enumerate_physical_device_groups_khr : Option < extensions :: khr_device_group_creation :: PFN_vkEnumeratePhysicalDeviceGroupsKHR > , }
impl InstanceLoader {
    #[inline]
    pub fn custom<T>(
        entry_loader: &EntryLoader<T>,
        instance: crate::vk1_0::Instance,
        version: u32,
        extensions_len: usize,
        extensions: *const *const std::os::raw::c_char,
        mut symbol: impl FnMut(*const std::os::raw::c_char) -> Option<crate::vk1_0::PFN_vkVoidFunction>,
    ) -> Result<InstanceLoader, crate::LoaderError> {
        let khr_android_surface = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::khr_android_surface::KHR_ANDROID_SURFACE_EXTENSION_NAME,
            )
        };
        let khr_display = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::khr_display::KHR_DISPLAY_EXTENSION_NAME,
            )
        };
        let khr_surface = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::khr_surface::KHR_SURFACE_EXTENSION_NAME,
            )
        };
        let nn_vi_surface = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::nn_vi_surface::NN_VI_SURFACE_EXTENSION_NAME,
            )
        };
        let khr_wayland_surface = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::khr_wayland_surface::KHR_WAYLAND_SURFACE_EXTENSION_NAME,
            )
        };
        let khr_win32_surface = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::khr_win32_surface::KHR_WIN32_SURFACE_EXTENSION_NAME,
            )
        };
        let khr_xlib_surface = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::khr_xlib_surface::KHR_XLIB_SURFACE_EXTENSION_NAME,
            )
        };
        let khr_xcb_surface = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::khr_xcb_surface::KHR_XCB_SURFACE_EXTENSION_NAME,
            )
        };
        let ext_directfb_surface = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::ext_directfb_surface::EXT_DIRECTFB_SURFACE_EXTENSION_NAME,
            )
        };
        let fuchsia_imagepipe_surface = unsafe {
            crate :: c_str_array_contains ( extensions , extensions_len , crate :: extensions :: fuchsia_imagepipe_surface :: FUCHSIA_IMAGEPIPE_SURFACE_EXTENSION_NAME )
        };
        let ggp_stream_descriptor_surface = unsafe {
            crate :: c_str_array_contains ( extensions , extensions_len , crate :: extensions :: ggp_stream_descriptor_surface :: GGP_STREAM_DESCRIPTOR_SURFACE_EXTENSION_NAME )
        };
        let ext_debug_report = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::ext_debug_report::EXT_DEBUG_REPORT_EXTENSION_NAME,
            )
        };
        let nv_external_memory_capabilities = unsafe {
            crate :: c_str_array_contains ( extensions , extensions_len , crate :: extensions :: nv_external_memory_capabilities :: NV_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME )
        };
        let vk1_1 = version >= crate::vk1_0::make_version(1, 1, 0);
        let ext_direct_mode_display = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::ext_direct_mode_display::EXT_DIRECT_MODE_DISPLAY_EXTENSION_NAME,
            )
        };
        let ext_acquire_xlib_display = unsafe {
            crate :: c_str_array_contains ( extensions , extensions_len , crate :: extensions :: ext_acquire_xlib_display :: EXT_ACQUIRE_XLIB_DISPLAY_EXTENSION_NAME )
        };
        let ext_display_surface_counter = unsafe {
            crate :: c_str_array_contains ( extensions , extensions_len , crate :: extensions :: ext_display_surface_counter :: EXT_DISPLAY_SURFACE_COUNTER_EXTENSION_NAME )
        };
        let khr_swapchain = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::khr_swapchain::KHR_SWAPCHAIN_EXTENSION_NAME,
            )
        };
        let khr_device_group = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::khr_device_group::KHR_DEVICE_GROUP_EXTENSION_NAME,
            )
        };
        let mvk_ios_surface = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::mvk_ios_surface::MVK_IOS_SURFACE_EXTENSION_NAME,
            )
        };
        let mvk_macos_surface = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::mvk_macos_surface::MVK_MACOS_SURFACE_EXTENSION_NAME,
            )
        };
        let ext_metal_surface = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::ext_metal_surface::EXT_METAL_SURFACE_EXTENSION_NAME,
            )
        };
        let ext_sample_locations = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::ext_sample_locations::EXT_SAMPLE_LOCATIONS_EXTENSION_NAME,
            )
        };
        let khr_get_surface_capabilities2 = unsafe {
            crate :: c_str_array_contains ( extensions , extensions_len , crate :: extensions :: khr_get_surface_capabilities2 :: KHR_GET_SURFACE_CAPABILITIES_2_EXTENSION_NAME )
        };
        let khr_get_display_properties2 = unsafe {
            crate :: c_str_array_contains ( extensions , extensions_len , crate :: extensions :: khr_get_display_properties2 :: KHR_GET_DISPLAY_PROPERTIES_2_EXTENSION_NAME )
        };
        let ext_calibrated_timestamps = unsafe {
            crate :: c_str_array_contains ( extensions , extensions_len , crate :: extensions :: ext_calibrated_timestamps :: EXT_CALIBRATED_TIMESTAMPS_EXTENSION_NAME )
        };
        let ext_debug_utils = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::ext_debug_utils::EXT_DEBUG_UTILS_EXTENSION_NAME,
            )
        };
        let nv_cooperative_matrix = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::nv_cooperative_matrix::NV_COOPERATIVE_MATRIX_EXTENSION_NAME,
            )
        };
        let ext_full_screen_exclusive = unsafe {
            crate :: c_str_array_contains ( extensions , extensions_len , crate :: extensions :: ext_full_screen_exclusive :: EXT_FULL_SCREEN_EXCLUSIVE_EXTENSION_NAME )
        };
        let khr_performance_query = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::khr_performance_query::KHR_PERFORMANCE_QUERY_EXTENSION_NAME,
            )
        };
        let ext_headless_surface = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::ext_headless_surface::EXT_HEADLESS_SURFACE_EXTENSION_NAME,
            )
        };
        let nv_coverage_reduction_mode = unsafe {
            crate :: c_str_array_contains ( extensions , extensions_len , crate :: extensions :: nv_coverage_reduction_mode :: NV_COVERAGE_REDUCTION_MODE_EXTENSION_NAME )
        };
        let ext_tooling_info = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::ext_tooling_info::EXT_TOOLING_INFO_EXTENSION_NAME,
            )
        };
        let khr_get_physical_device_properties2 = unsafe {
            crate :: c_str_array_contains ( extensions , extensions_len , crate :: extensions :: khr_get_physical_device_properties2 :: KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_EXTENSION_NAME )
        };
        let khr_external_memory_capabilities = unsafe {
            crate :: c_str_array_contains ( extensions , extensions_len , crate :: extensions :: khr_external_memory_capabilities :: KHR_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME )
        };
        let khr_external_semaphore_capabilities = unsafe {
            crate :: c_str_array_contains ( extensions , extensions_len , crate :: extensions :: khr_external_semaphore_capabilities :: KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_EXTENSION_NAME )
        };
        let khr_external_fence_capabilities = unsafe {
            crate :: c_str_array_contains ( extensions , extensions_len , crate :: extensions :: khr_external_fence_capabilities :: KHR_EXTERNAL_FENCE_CAPABILITIES_EXTENSION_NAME )
        };
        let khr_device_group_creation = unsafe {
            crate :: c_str_array_contains ( extensions , extensions_len , crate :: extensions :: khr_device_group_creation :: KHR_DEVICE_GROUP_CREATION_EXTENSION_NAME )
        };
        let get_device_proc_addr = symbol(crate::vk1_0::FN_GET_DEVICE_PROC_ADDR)
            .ok_or(crate::LoaderError::SymbolNotAvailable)?;
        Ok(InstanceLoader {
            parent: std::sync::Arc::downgrade(&entry_loader.arc),
            arc: std::sync::Arc::new(()),
            selected_instance_version: version,
            get_device_proc_addr: unsafe { std::mem::transmute(get_device_proc_addr) },
            handle: instance,
            destroy_instance: unsafe {
                match symbol(crate::vk1_0::FN_DESTROY_INSTANCE) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            enumerate_physical_devices: unsafe {
                match symbol(crate::vk1_0::FN_ENUMERATE_PHYSICAL_DEVICES) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            get_instance_proc_addr: unsafe {
                match symbol(crate::vk1_0::FN_GET_INSTANCE_PROC_ADDR) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            get_physical_device_properties: unsafe {
                match symbol(crate::vk1_0::FN_GET_PHYSICAL_DEVICE_PROPERTIES) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            get_physical_device_queue_family_properties: unsafe {
                match symbol(crate::vk1_0::FN_GET_PHYSICAL_DEVICE_QUEUE_FAMILY_PROPERTIES) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            get_physical_device_memory_properties: unsafe {
                match symbol(crate::vk1_0::FN_GET_PHYSICAL_DEVICE_MEMORY_PROPERTIES) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            get_physical_device_features: unsafe {
                match symbol(crate::vk1_0::FN_GET_PHYSICAL_DEVICE_FEATURES) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            get_physical_device_format_properties: unsafe {
                match symbol(crate::vk1_0::FN_GET_PHYSICAL_DEVICE_FORMAT_PROPERTIES) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            get_physical_device_image_format_properties: unsafe {
                match symbol(crate::vk1_0::FN_GET_PHYSICAL_DEVICE_IMAGE_FORMAT_PROPERTIES) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            create_device: unsafe {
                match symbol(crate::vk1_0::FN_CREATE_DEVICE) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            enumerate_device_layer_properties: unsafe {
                match symbol(crate::vk1_0::FN_ENUMERATE_DEVICE_LAYER_PROPERTIES) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            enumerate_device_extension_properties: unsafe {
                match symbol(crate::vk1_0::FN_ENUMERATE_DEVICE_EXTENSION_PROPERTIES) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            get_physical_device_sparse_image_format_properties: unsafe {
                match symbol(crate::vk1_0::FN_GET_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_PROPERTIES) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            create_android_surface_khr: if khr_android_surface {
                unsafe {
                    match symbol(
                        crate::extensions::khr_android_surface::FN_CREATE_ANDROID_SURFACE_KHR,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_physical_device_display_properties_khr: if khr_display {
                unsafe {
                    match symbol ( crate :: extensions :: khr_display :: FN_GET_PHYSICAL_DEVICE_DISPLAY_PROPERTIES_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            get_physical_device_display_plane_properties_khr: if khr_display {
                unsafe {
                    match symbol ( crate :: extensions :: khr_display :: FN_GET_PHYSICAL_DEVICE_DISPLAY_PLANE_PROPERTIES_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            get_display_plane_supported_displays_khr: if khr_display {
                unsafe {
                    match symbol(
                        crate::extensions::khr_display::FN_GET_DISPLAY_PLANE_SUPPORTED_DISPLAYS_KHR,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_display_mode_properties_khr: if khr_display {
                unsafe {
                    match symbol(crate::extensions::khr_display::FN_GET_DISPLAY_MODE_PROPERTIES_KHR)
                    {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            create_display_mode_khr: if khr_display {
                unsafe {
                    match symbol(crate::extensions::khr_display::FN_CREATE_DISPLAY_MODE_KHR) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_display_plane_capabilities_khr: if khr_display {
                unsafe {
                    match symbol(
                        crate::extensions::khr_display::FN_GET_DISPLAY_PLANE_CAPABILITIES_KHR,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            create_display_plane_surface_khr: if khr_display {
                unsafe {
                    match symbol(
                        crate::extensions::khr_display::FN_CREATE_DISPLAY_PLANE_SURFACE_KHR,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            destroy_surface_khr: if khr_surface {
                unsafe {
                    match symbol(crate::extensions::khr_surface::FN_DESTROY_SURFACE_KHR) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_physical_device_surface_support_khr: if khr_surface {
                unsafe {
                    match symbol(
                        crate::extensions::khr_surface::FN_GET_PHYSICAL_DEVICE_SURFACE_SUPPORT_KHR,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_physical_device_surface_capabilities_khr: if khr_surface {
                unsafe {
                    match symbol ( crate :: extensions :: khr_surface :: FN_GET_PHYSICAL_DEVICE_SURFACE_CAPABILITIES_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            get_physical_device_surface_formats_khr: if khr_surface {
                unsafe {
                    match symbol(
                        crate::extensions::khr_surface::FN_GET_PHYSICAL_DEVICE_SURFACE_FORMATS_KHR,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_physical_device_surface_present_modes_khr: if khr_surface {
                unsafe {
                    match symbol ( crate :: extensions :: khr_surface :: FN_GET_PHYSICAL_DEVICE_SURFACE_PRESENT_MODES_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            create_vi_surface_nn: if nn_vi_surface {
                unsafe {
                    match symbol(crate::extensions::nn_vi_surface::FN_CREATE_VI_SURFACE_NN) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            create_wayland_surface_khr: if khr_wayland_surface {
                unsafe {
                    match symbol(
                        crate::extensions::khr_wayland_surface::FN_CREATE_WAYLAND_SURFACE_KHR,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_physical_device_wayland_presentation_support_khr: if khr_wayland_surface {
                unsafe {
                    match symbol ( crate :: extensions :: khr_wayland_surface :: FN_GET_PHYSICAL_DEVICE_WAYLAND_PRESENTATION_SUPPORT_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            create_win32_surface_khr: if khr_win32_surface {
                unsafe {
                    match symbol(crate::extensions::khr_win32_surface::FN_CREATE_WIN32_SURFACE_KHR)
                    {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_physical_device_win32_presentation_support_khr: if khr_win32_surface {
                unsafe {
                    match symbol ( crate :: extensions :: khr_win32_surface :: FN_GET_PHYSICAL_DEVICE_WIN32_PRESENTATION_SUPPORT_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            create_xlib_surface_khr: if khr_xlib_surface {
                unsafe {
                    match symbol(crate::extensions::khr_xlib_surface::FN_CREATE_XLIB_SURFACE_KHR) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_physical_device_xlib_presentation_support_khr: if khr_xlib_surface {
                unsafe {
                    match symbol ( crate :: extensions :: khr_xlib_surface :: FN_GET_PHYSICAL_DEVICE_XLIB_PRESENTATION_SUPPORT_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            create_xcb_surface_khr: if khr_xcb_surface {
                unsafe {
                    match symbol(crate::extensions::khr_xcb_surface::FN_CREATE_XCB_SURFACE_KHR) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_physical_device_xcb_presentation_support_khr: if khr_xcb_surface {
                unsafe {
                    match symbol ( crate :: extensions :: khr_xcb_surface :: FN_GET_PHYSICAL_DEVICE_XCB_PRESENTATION_SUPPORT_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            create_direct_fb_surface_ext: if ext_directfb_surface {
                unsafe {
                    match symbol(
                        crate::extensions::ext_directfb_surface::FN_CREATE_DIRECT_FB_SURFACE_EXT,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_physical_device_direct_fb_presentation_support_ext: if ext_directfb_surface {
                unsafe {
                    match symbol ( crate :: extensions :: ext_directfb_surface :: FN_GET_PHYSICAL_DEVICE_DIRECT_FB_PRESENTATION_SUPPORT_EXT ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            create_image_pipe_surface_fuchsia: if fuchsia_imagepipe_surface {
                unsafe {
                    match symbol ( crate :: extensions :: fuchsia_imagepipe_surface :: FN_CREATE_IMAGE_PIPE_SURFACE_FUCHSIA ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            create_stream_descriptor_surface_ggp: if ggp_stream_descriptor_surface {
                unsafe {
                    match symbol ( crate :: extensions :: ggp_stream_descriptor_surface :: FN_CREATE_STREAM_DESCRIPTOR_SURFACE_GGP ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            create_debug_report_callback_ext: if ext_debug_report {
                unsafe {
                    match symbol(
                        crate::extensions::ext_debug_report::FN_CREATE_DEBUG_REPORT_CALLBACK_EXT,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            destroy_debug_report_callback_ext: if ext_debug_report {
                unsafe {
                    match symbol(
                        crate::extensions::ext_debug_report::FN_DESTROY_DEBUG_REPORT_CALLBACK_EXT,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            debug_report_message_ext: if ext_debug_report {
                unsafe {
                    match symbol(crate::extensions::ext_debug_report::FN_DEBUG_REPORT_MESSAGE_EXT) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_physical_device_external_image_format_properties_nv:
                if nv_external_memory_capabilities {
                    unsafe {
                        match symbol ( crate :: extensions :: nv_external_memory_capabilities :: FN_GET_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_PROPERTIES_NV ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                    }
                } else {
                    None
                },
            get_physical_device_features2: if vk1_1 {
                unsafe {
                    match symbol(crate::vk1_1::FN_GET_PHYSICAL_DEVICE_FEATURES2) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_physical_device_properties2: if vk1_1 {
                unsafe {
                    match symbol(crate::vk1_1::FN_GET_PHYSICAL_DEVICE_PROPERTIES2) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_physical_device_format_properties2: if vk1_1 {
                unsafe {
                    match symbol(crate::vk1_1::FN_GET_PHYSICAL_DEVICE_FORMAT_PROPERTIES2) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_physical_device_image_format_properties2: if vk1_1 {
                unsafe {
                    match symbol(crate::vk1_1::FN_GET_PHYSICAL_DEVICE_IMAGE_FORMAT_PROPERTIES2) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_physical_device_queue_family_properties2: if vk1_1 {
                unsafe {
                    match symbol(crate::vk1_1::FN_GET_PHYSICAL_DEVICE_QUEUE_FAMILY_PROPERTIES2) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_physical_device_memory_properties2: if vk1_1 {
                unsafe {
                    match symbol(crate::vk1_1::FN_GET_PHYSICAL_DEVICE_MEMORY_PROPERTIES2) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_physical_device_sparse_image_format_properties2: if vk1_1 {
                unsafe {
                    match symbol(
                        crate::vk1_1::FN_GET_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_PROPERTIES2,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_physical_device_external_buffer_properties: if vk1_1 {
                unsafe {
                    match symbol(crate::vk1_1::FN_GET_PHYSICAL_DEVICE_EXTERNAL_BUFFER_PROPERTIES) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_physical_device_external_semaphore_properties: if vk1_1 {
                unsafe {
                    match symbol(crate::vk1_1::FN_GET_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_PROPERTIES)
                    {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_physical_device_external_fence_properties: if vk1_1 {
                unsafe {
                    match symbol(crate::vk1_1::FN_GET_PHYSICAL_DEVICE_EXTERNAL_FENCE_PROPERTIES) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            release_display_ext: if ext_direct_mode_display {
                unsafe {
                    match symbol(crate::extensions::ext_direct_mode_display::FN_RELEASE_DISPLAY_EXT)
                    {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            acquire_xlib_display_ext: if ext_acquire_xlib_display {
                unsafe {
                    match symbol(
                        crate::extensions::ext_acquire_xlib_display::FN_ACQUIRE_XLIB_DISPLAY_EXT,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_rand_r_output_display_ext: if ext_acquire_xlib_display {
                unsafe {
                    match symbol ( crate :: extensions :: ext_acquire_xlib_display :: FN_GET_RAND_R_OUTPUT_DISPLAY_EXT ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            get_physical_device_surface_capabilities2_ext: if ext_display_surface_counter {
                unsafe {
                    match symbol ( crate :: extensions :: ext_display_surface_counter :: FN_GET_PHYSICAL_DEVICE_SURFACE_CAPABILITIES2_EXT ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            enumerate_physical_device_groups: if vk1_1 {
                unsafe {
                    match symbol(crate::vk1_1::FN_ENUMERATE_PHYSICAL_DEVICE_GROUPS) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_physical_device_present_rectangles_khr: if (khr_swapchain && vk1_1)
                || (khr_device_group && khr_surface)
            {
                unsafe {
                    match symbol ( crate :: extensions :: khr_swapchain :: FN_GET_PHYSICAL_DEVICE_PRESENT_RECTANGLES_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            create_ios_surface_mvk: if mvk_ios_surface {
                unsafe {
                    match symbol(crate::extensions::mvk_ios_surface::FN_CREATE_IOS_SURFACE_MVK) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            create_mac_os_surface_mvk: if mvk_macos_surface {
                unsafe {
                    match symbol(crate::extensions::mvk_macos_surface::FN_CREATE_MAC_OS_SURFACE_MVK)
                    {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            create_metal_surface_ext: if ext_metal_surface {
                unsafe {
                    match symbol(crate::extensions::ext_metal_surface::FN_CREATE_METAL_SURFACE_EXT)
                    {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_physical_device_multisample_properties_ext: if ext_sample_locations {
                unsafe {
                    match symbol ( crate :: extensions :: ext_sample_locations :: FN_GET_PHYSICAL_DEVICE_MULTISAMPLE_PROPERTIES_EXT ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            get_physical_device_surface_capabilities2_khr: if khr_get_surface_capabilities2 {
                unsafe {
                    match symbol ( crate :: extensions :: khr_get_surface_capabilities2 :: FN_GET_PHYSICAL_DEVICE_SURFACE_CAPABILITIES2_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            get_physical_device_surface_formats2_khr: if khr_get_surface_capabilities2 {
                unsafe {
                    match symbol ( crate :: extensions :: khr_get_surface_capabilities2 :: FN_GET_PHYSICAL_DEVICE_SURFACE_FORMATS2_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            get_physical_device_display_properties2_khr: if khr_get_display_properties2 {
                unsafe {
                    match symbol ( crate :: extensions :: khr_get_display_properties2 :: FN_GET_PHYSICAL_DEVICE_DISPLAY_PROPERTIES2_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            get_physical_device_display_plane_properties2_khr: if khr_get_display_properties2 {
                unsafe {
                    match symbol ( crate :: extensions :: khr_get_display_properties2 :: FN_GET_PHYSICAL_DEVICE_DISPLAY_PLANE_PROPERTIES2_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            get_display_mode_properties2_khr: if khr_get_display_properties2 {
                unsafe {
                    match symbol ( crate :: extensions :: khr_get_display_properties2 :: FN_GET_DISPLAY_MODE_PROPERTIES2_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            get_display_plane_capabilities2_khr: if khr_get_display_properties2 {
                unsafe {
                    match symbol ( crate :: extensions :: khr_get_display_properties2 :: FN_GET_DISPLAY_PLANE_CAPABILITIES2_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            get_physical_device_calibrateable_time_domains_ext: if ext_calibrated_timestamps {
                unsafe {
                    match symbol ( crate :: extensions :: ext_calibrated_timestamps :: FN_GET_PHYSICAL_DEVICE_CALIBRATEABLE_TIME_DOMAINS_EXT ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            create_debug_utils_messenger_ext: if ext_debug_utils {
                unsafe {
                    match symbol(
                        crate::extensions::ext_debug_utils::FN_CREATE_DEBUG_UTILS_MESSENGER_EXT,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            destroy_debug_utils_messenger_ext: if ext_debug_utils {
                unsafe {
                    match symbol(
                        crate::extensions::ext_debug_utils::FN_DESTROY_DEBUG_UTILS_MESSENGER_EXT,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            submit_debug_utils_message_ext: if ext_debug_utils {
                unsafe {
                    match symbol(
                        crate::extensions::ext_debug_utils::FN_SUBMIT_DEBUG_UTILS_MESSAGE_EXT,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_physical_device_cooperative_matrix_properties_nv: if nv_cooperative_matrix {
                unsafe {
                    match symbol ( crate :: extensions :: nv_cooperative_matrix :: FN_GET_PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_NV ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            get_physical_device_surface_present_modes2_ext: if ext_full_screen_exclusive {
                unsafe {
                    match symbol ( crate :: extensions :: ext_full_screen_exclusive :: FN_GET_PHYSICAL_DEVICE_SURFACE_PRESENT_MODES2_EXT ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            enumerate_physical_device_queue_family_performance_query_counters_khr:
                if khr_performance_query {
                    unsafe {
                        match symbol ( crate :: extensions :: khr_performance_query :: FN_ENUMERATE_PHYSICAL_DEVICE_QUEUE_FAMILY_PERFORMANCE_QUERY_COUNTERS_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                    }
                } else {
                    None
                },
            get_physical_device_queue_family_performance_query_passes_khr: if khr_performance_query
            {
                unsafe {
                    match symbol ( crate :: extensions :: khr_performance_query :: FN_GET_PHYSICAL_DEVICE_QUEUE_FAMILY_PERFORMANCE_QUERY_PASSES_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            create_headless_surface_ext: if ext_headless_surface {
                unsafe {
                    match symbol(
                        crate::extensions::ext_headless_surface::FN_CREATE_HEADLESS_SURFACE_EXT,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_physical_device_supported_framebuffer_mixed_samples_combinations_nv:
                if nv_coverage_reduction_mode {
                    unsafe {
                        match symbol ( crate :: extensions :: nv_coverage_reduction_mode :: FN_GET_PHYSICAL_DEVICE_SUPPORTED_FRAMEBUFFER_MIXED_SAMPLES_COMBINATIONS_NV ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                    }
                } else {
                    None
                },
            get_physical_device_tool_properties_ext: if ext_tooling_info {
                unsafe {
                    match symbol ( crate :: extensions :: ext_tooling_info :: FN_GET_PHYSICAL_DEVICE_TOOL_PROPERTIES_EXT ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            get_physical_device_features2_khr: if khr_get_physical_device_properties2 {
                unsafe {
                    match symbol ( crate :: extensions :: khr_get_physical_device_properties2 :: FN_GET_PHYSICAL_DEVICE_FEATURES2_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            get_physical_device_properties2_khr: if khr_get_physical_device_properties2 {
                unsafe {
                    match symbol ( crate :: extensions :: khr_get_physical_device_properties2 :: FN_GET_PHYSICAL_DEVICE_PROPERTIES2_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            get_physical_device_format_properties2_khr: if khr_get_physical_device_properties2 {
                unsafe {
                    match symbol ( crate :: extensions :: khr_get_physical_device_properties2 :: FN_GET_PHYSICAL_DEVICE_FORMAT_PROPERTIES2_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            get_physical_device_image_format_properties2_khr: if khr_get_physical_device_properties2
            {
                unsafe {
                    match symbol ( crate :: extensions :: khr_get_physical_device_properties2 :: FN_GET_PHYSICAL_DEVICE_IMAGE_FORMAT_PROPERTIES2_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            get_physical_device_queue_family_properties2_khr: if khr_get_physical_device_properties2
            {
                unsafe {
                    match symbol ( crate :: extensions :: khr_get_physical_device_properties2 :: FN_GET_PHYSICAL_DEVICE_QUEUE_FAMILY_PROPERTIES2_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            get_physical_device_memory_properties2_khr: if khr_get_physical_device_properties2 {
                unsafe {
                    match symbol ( crate :: extensions :: khr_get_physical_device_properties2 :: FN_GET_PHYSICAL_DEVICE_MEMORY_PROPERTIES2_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            get_physical_device_sparse_image_format_properties2_khr:
                if khr_get_physical_device_properties2 {
                    unsafe {
                        match symbol ( crate :: extensions :: khr_get_physical_device_properties2 :: FN_GET_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_PROPERTIES2_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                    }
                } else {
                    None
                },
            get_physical_device_external_buffer_properties_khr: if khr_external_memory_capabilities
            {
                unsafe {
                    match symbol ( crate :: extensions :: khr_external_memory_capabilities :: FN_GET_PHYSICAL_DEVICE_EXTERNAL_BUFFER_PROPERTIES_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            get_physical_device_external_semaphore_properties_khr:
                if khr_external_semaphore_capabilities {
                    unsafe {
                        match symbol ( crate :: extensions :: khr_external_semaphore_capabilities :: FN_GET_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_PROPERTIES_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                    }
                } else {
                    None
                },
            get_physical_device_external_fence_properties_khr: if khr_external_fence_capabilities {
                unsafe {
                    match symbol ( crate :: extensions :: khr_external_fence_capabilities :: FN_GET_PHYSICAL_DEVICE_EXTERNAL_FENCE_PROPERTIES_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            enumerate_physical_device_groups_khr: if khr_device_group_creation {
                unsafe {
                    match symbol ( crate :: extensions :: khr_device_group_creation :: FN_ENUMERATE_PHYSICAL_DEVICE_GROUPS_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
        })
    }
}
impl Drop for InstanceLoader {
    fn drop(&mut self) {
        if std::sync::Arc::weak_count(&self.arc) != 0 {
            panic!("Attempting to drop a instance loader with active references to it");
        }
    }
}
impl std::fmt::Debug for InstanceLoader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.handle, f)
    }
}
#[doc = r" Loader for device commands"]
#[doc = r""]
#[doc = r" To create a new loader, call [`new`](#method.new)."]
pub struct DeviceLoader { # [ allow ( dead_code ) ] parent : std :: sync :: Weak < ( ) > , pub handle : crate :: vk1_0 :: Device , pub get_device_proc_addr : Option < vk1_0 :: PFN_vkGetDeviceProcAddr > , pub destroy_device : Option < vk1_0 :: PFN_vkDestroyDevice > , pub get_device_queue : Option < vk1_0 :: PFN_vkGetDeviceQueue > , pub queue_submit : Option < vk1_0 :: PFN_vkQueueSubmit > , pub queue_wait_idle : Option < vk1_0 :: PFN_vkQueueWaitIdle > , pub device_wait_idle : Option < vk1_0 :: PFN_vkDeviceWaitIdle > , pub allocate_memory : Option < vk1_0 :: PFN_vkAllocateMemory > , pub free_memory : Option < vk1_0 :: PFN_vkFreeMemory > , pub map_memory : Option < vk1_0 :: PFN_vkMapMemory > , pub unmap_memory : Option < vk1_0 :: PFN_vkUnmapMemory > , pub flush_mapped_memory_ranges : Option < vk1_0 :: PFN_vkFlushMappedMemoryRanges > , pub invalidate_mapped_memory_ranges : Option < vk1_0 :: PFN_vkInvalidateMappedMemoryRanges > , pub get_device_memory_commitment : Option < vk1_0 :: PFN_vkGetDeviceMemoryCommitment > , pub get_buffer_memory_requirements : Option < vk1_0 :: PFN_vkGetBufferMemoryRequirements > , pub bind_buffer_memory : Option < vk1_0 :: PFN_vkBindBufferMemory > , pub get_image_memory_requirements : Option < vk1_0 :: PFN_vkGetImageMemoryRequirements > , pub bind_image_memory : Option < vk1_0 :: PFN_vkBindImageMemory > , pub get_image_sparse_memory_requirements : Option < vk1_0 :: PFN_vkGetImageSparseMemoryRequirements > , pub queue_bind_sparse : Option < vk1_0 :: PFN_vkQueueBindSparse > , pub create_fence : Option < vk1_0 :: PFN_vkCreateFence > , pub destroy_fence : Option < vk1_0 :: PFN_vkDestroyFence > , pub reset_fences : Option < vk1_0 :: PFN_vkResetFences > , pub get_fence_status : Option < vk1_0 :: PFN_vkGetFenceStatus > , pub wait_for_fences : Option < vk1_0 :: PFN_vkWaitForFences > , pub create_semaphore : Option < vk1_0 :: PFN_vkCreateSemaphore > , pub destroy_semaphore : Option < vk1_0 :: PFN_vkDestroySemaphore > , pub create_event : Option < vk1_0 :: PFN_vkCreateEvent > , pub destroy_event : Option < vk1_0 :: PFN_vkDestroyEvent > , pub get_event_status : Option < vk1_0 :: PFN_vkGetEventStatus > , pub set_event : Option < vk1_0 :: PFN_vkSetEvent > , pub reset_event : Option < vk1_0 :: PFN_vkResetEvent > , pub create_query_pool : Option < vk1_0 :: PFN_vkCreateQueryPool > , pub destroy_query_pool : Option < vk1_0 :: PFN_vkDestroyQueryPool > , pub get_query_pool_results : Option < vk1_0 :: PFN_vkGetQueryPoolResults > , pub reset_query_pool : Option < vk1_2 :: PFN_vkResetQueryPool > , pub create_buffer : Option < vk1_0 :: PFN_vkCreateBuffer > , pub destroy_buffer : Option < vk1_0 :: PFN_vkDestroyBuffer > , pub create_buffer_view : Option < vk1_0 :: PFN_vkCreateBufferView > , pub destroy_buffer_view : Option < vk1_0 :: PFN_vkDestroyBufferView > , pub create_image : Option < vk1_0 :: PFN_vkCreateImage > , pub destroy_image : Option < vk1_0 :: PFN_vkDestroyImage > , pub get_image_subresource_layout : Option < vk1_0 :: PFN_vkGetImageSubresourceLayout > , pub create_image_view : Option < vk1_0 :: PFN_vkCreateImageView > , pub destroy_image_view : Option < vk1_0 :: PFN_vkDestroyImageView > , pub create_shader_module : Option < vk1_0 :: PFN_vkCreateShaderModule > , pub destroy_shader_module : Option < vk1_0 :: PFN_vkDestroyShaderModule > , pub create_pipeline_cache : Option < vk1_0 :: PFN_vkCreatePipelineCache > , pub destroy_pipeline_cache : Option < vk1_0 :: PFN_vkDestroyPipelineCache > , pub get_pipeline_cache_data : Option < vk1_0 :: PFN_vkGetPipelineCacheData > , pub merge_pipeline_caches : Option < vk1_0 :: PFN_vkMergePipelineCaches > , pub create_graphics_pipelines : Option < vk1_0 :: PFN_vkCreateGraphicsPipelines > , pub create_compute_pipelines : Option < vk1_0 :: PFN_vkCreateComputePipelines > , pub destroy_pipeline : Option < vk1_0 :: PFN_vkDestroyPipeline > , pub create_pipeline_layout : Option < vk1_0 :: PFN_vkCreatePipelineLayout > , pub destroy_pipeline_layout : Option < vk1_0 :: PFN_vkDestroyPipelineLayout > , pub create_sampler : Option < vk1_0 :: PFN_vkCreateSampler > , pub destroy_sampler : Option < vk1_0 :: PFN_vkDestroySampler > , pub create_descriptor_set_layout : Option < vk1_0 :: PFN_vkCreateDescriptorSetLayout > , pub destroy_descriptor_set_layout : Option < vk1_0 :: PFN_vkDestroyDescriptorSetLayout > , pub create_descriptor_pool : Option < vk1_0 :: PFN_vkCreateDescriptorPool > , pub destroy_descriptor_pool : Option < vk1_0 :: PFN_vkDestroyDescriptorPool > , pub reset_descriptor_pool : Option < vk1_0 :: PFN_vkResetDescriptorPool > , pub allocate_descriptor_sets : Option < vk1_0 :: PFN_vkAllocateDescriptorSets > , pub free_descriptor_sets : Option < vk1_0 :: PFN_vkFreeDescriptorSets > , pub update_descriptor_sets : Option < vk1_0 :: PFN_vkUpdateDescriptorSets > , pub create_framebuffer : Option < vk1_0 :: PFN_vkCreateFramebuffer > , pub destroy_framebuffer : Option < vk1_0 :: PFN_vkDestroyFramebuffer > , pub create_render_pass : Option < vk1_0 :: PFN_vkCreateRenderPass > , pub destroy_render_pass : Option < vk1_0 :: PFN_vkDestroyRenderPass > , pub get_render_area_granularity : Option < vk1_0 :: PFN_vkGetRenderAreaGranularity > , pub create_command_pool : Option < vk1_0 :: PFN_vkCreateCommandPool > , pub destroy_command_pool : Option < vk1_0 :: PFN_vkDestroyCommandPool > , pub reset_command_pool : Option < vk1_0 :: PFN_vkResetCommandPool > , pub allocate_command_buffers : Option < vk1_0 :: PFN_vkAllocateCommandBuffers > , pub free_command_buffers : Option < vk1_0 :: PFN_vkFreeCommandBuffers > , pub begin_command_buffer : Option < vk1_0 :: PFN_vkBeginCommandBuffer > , pub end_command_buffer : Option < vk1_0 :: PFN_vkEndCommandBuffer > , pub reset_command_buffer : Option < vk1_0 :: PFN_vkResetCommandBuffer > , pub cmd_bind_pipeline : Option < vk1_0 :: PFN_vkCmdBindPipeline > , pub cmd_set_viewport : Option < vk1_0 :: PFN_vkCmdSetViewport > , pub cmd_set_scissor : Option < vk1_0 :: PFN_vkCmdSetScissor > , pub cmd_set_line_width : Option < vk1_0 :: PFN_vkCmdSetLineWidth > , pub cmd_set_depth_bias : Option < vk1_0 :: PFN_vkCmdSetDepthBias > , pub cmd_set_blend_constants : Option < vk1_0 :: PFN_vkCmdSetBlendConstants > , pub cmd_set_depth_bounds : Option < vk1_0 :: PFN_vkCmdSetDepthBounds > , pub cmd_set_stencil_compare_mask : Option < vk1_0 :: PFN_vkCmdSetStencilCompareMask > , pub cmd_set_stencil_write_mask : Option < vk1_0 :: PFN_vkCmdSetStencilWriteMask > , pub cmd_set_stencil_reference : Option < vk1_0 :: PFN_vkCmdSetStencilReference > , pub cmd_bind_descriptor_sets : Option < vk1_0 :: PFN_vkCmdBindDescriptorSets > , pub cmd_bind_index_buffer : Option < vk1_0 :: PFN_vkCmdBindIndexBuffer > , pub cmd_bind_vertex_buffers : Option < vk1_0 :: PFN_vkCmdBindVertexBuffers > , pub cmd_draw : Option < vk1_0 :: PFN_vkCmdDraw > , pub cmd_draw_indexed : Option < vk1_0 :: PFN_vkCmdDrawIndexed > , pub cmd_draw_indirect : Option < vk1_0 :: PFN_vkCmdDrawIndirect > , pub cmd_draw_indexed_indirect : Option < vk1_0 :: PFN_vkCmdDrawIndexedIndirect > , pub cmd_dispatch : Option < vk1_0 :: PFN_vkCmdDispatch > , pub cmd_dispatch_indirect : Option < vk1_0 :: PFN_vkCmdDispatchIndirect > , pub cmd_copy_buffer : Option < vk1_0 :: PFN_vkCmdCopyBuffer > , pub cmd_copy_image : Option < vk1_0 :: PFN_vkCmdCopyImage > , pub cmd_blit_image : Option < vk1_0 :: PFN_vkCmdBlitImage > , pub cmd_copy_buffer_to_image : Option < vk1_0 :: PFN_vkCmdCopyBufferToImage > , pub cmd_copy_image_to_buffer : Option < vk1_0 :: PFN_vkCmdCopyImageToBuffer > , pub cmd_update_buffer : Option < vk1_0 :: PFN_vkCmdUpdateBuffer > , pub cmd_fill_buffer : Option < vk1_0 :: PFN_vkCmdFillBuffer > , pub cmd_clear_color_image : Option < vk1_0 :: PFN_vkCmdClearColorImage > , pub cmd_clear_depth_stencil_image : Option < vk1_0 :: PFN_vkCmdClearDepthStencilImage > , pub cmd_clear_attachments : Option < vk1_0 :: PFN_vkCmdClearAttachments > , pub cmd_resolve_image : Option < vk1_0 :: PFN_vkCmdResolveImage > , pub cmd_set_event : Option < vk1_0 :: PFN_vkCmdSetEvent > , pub cmd_reset_event : Option < vk1_0 :: PFN_vkCmdResetEvent > , pub cmd_wait_events : Option < vk1_0 :: PFN_vkCmdWaitEvents > , pub cmd_pipeline_barrier : Option < vk1_0 :: PFN_vkCmdPipelineBarrier > , pub cmd_begin_query : Option < vk1_0 :: PFN_vkCmdBeginQuery > , pub cmd_end_query : Option < vk1_0 :: PFN_vkCmdEndQuery > , pub cmd_begin_conditional_rendering_ext : Option < extensions :: ext_conditional_rendering :: PFN_vkCmdBeginConditionalRenderingEXT > , pub cmd_end_conditional_rendering_ext : Option < extensions :: ext_conditional_rendering :: PFN_vkCmdEndConditionalRenderingEXT > , pub cmd_reset_query_pool : Option < vk1_0 :: PFN_vkCmdResetQueryPool > , pub cmd_write_timestamp : Option < vk1_0 :: PFN_vkCmdWriteTimestamp > , pub cmd_copy_query_pool_results : Option < vk1_0 :: PFN_vkCmdCopyQueryPoolResults > , pub cmd_push_constants : Option < vk1_0 :: PFN_vkCmdPushConstants > , pub cmd_begin_render_pass : Option < vk1_0 :: PFN_vkCmdBeginRenderPass > , pub cmd_next_subpass : Option < vk1_0 :: PFN_vkCmdNextSubpass > , pub cmd_end_render_pass : Option < vk1_0 :: PFN_vkCmdEndRenderPass > , pub cmd_execute_commands : Option < vk1_0 :: PFN_vkCmdExecuteCommands > , pub create_shared_swapchains_khr : Option < extensions :: khr_display_swapchain :: PFN_vkCreateSharedSwapchainsKHR > , pub create_swapchain_khr : Option < extensions :: khr_swapchain :: PFN_vkCreateSwapchainKHR > , pub destroy_swapchain_khr : Option < extensions :: khr_swapchain :: PFN_vkDestroySwapchainKHR > , pub get_swapchain_images_khr : Option < extensions :: khr_swapchain :: PFN_vkGetSwapchainImagesKHR > , pub acquire_next_image_khr : Option < extensions :: khr_swapchain :: PFN_vkAcquireNextImageKHR > , pub queue_present_khr : Option < extensions :: khr_swapchain :: PFN_vkQueuePresentKHR > , pub debug_marker_set_object_name_ext : Option < extensions :: ext_debug_marker :: PFN_vkDebugMarkerSetObjectNameEXT > , pub debug_marker_set_object_tag_ext : Option < extensions :: ext_debug_marker :: PFN_vkDebugMarkerSetObjectTagEXT > , pub cmd_debug_marker_begin_ext : Option < extensions :: ext_debug_marker :: PFN_vkCmdDebugMarkerBeginEXT > , pub cmd_debug_marker_end_ext : Option < extensions :: ext_debug_marker :: PFN_vkCmdDebugMarkerEndEXT > , pub cmd_debug_marker_insert_ext : Option < extensions :: ext_debug_marker :: PFN_vkCmdDebugMarkerInsertEXT > , pub get_memory_win32_handle_nv : Option < extensions :: nv_external_memory_win32 :: PFN_vkGetMemoryWin32HandleNV > , pub cmd_execute_generated_commands_nv : Option < extensions :: nv_device_generated_commands :: PFN_vkCmdExecuteGeneratedCommandsNV > , pub cmd_preprocess_generated_commands_nv : Option < extensions :: nv_device_generated_commands :: PFN_vkCmdPreprocessGeneratedCommandsNV > , pub cmd_bind_pipeline_shader_group_nv : Option < extensions :: nv_device_generated_commands :: PFN_vkCmdBindPipelineShaderGroupNV > , pub get_generated_commands_memory_requirements_nv : Option < extensions :: nv_device_generated_commands :: PFN_vkGetGeneratedCommandsMemoryRequirementsNV > , pub create_indirect_commands_layout_nv : Option < extensions :: nv_device_generated_commands :: PFN_vkCreateIndirectCommandsLayoutNV > , pub destroy_indirect_commands_layout_nv : Option < extensions :: nv_device_generated_commands :: PFN_vkDestroyIndirectCommandsLayoutNV > , pub cmd_push_descriptor_set_khr : Option < extensions :: khr_push_descriptor :: PFN_vkCmdPushDescriptorSetKHR > , pub trim_command_pool : Option < vk1_1 :: PFN_vkTrimCommandPool > , pub get_memory_win32_handle_khr : Option < extensions :: khr_external_memory_win32 :: PFN_vkGetMemoryWin32HandleKHR > , pub get_memory_win32_handle_properties_khr : Option < extensions :: khr_external_memory_win32 :: PFN_vkGetMemoryWin32HandlePropertiesKHR > , pub get_memory_fd_khr : Option < extensions :: khr_external_memory_fd :: PFN_vkGetMemoryFdKHR > , pub get_memory_fd_properties_khr : Option < extensions :: khr_external_memory_fd :: PFN_vkGetMemoryFdPropertiesKHR > , pub get_semaphore_win32_handle_khr : Option < extensions :: khr_external_semaphore_win32 :: PFN_vkGetSemaphoreWin32HandleKHR > , pub import_semaphore_win32_handle_khr : Option < extensions :: khr_external_semaphore_win32 :: PFN_vkImportSemaphoreWin32HandleKHR > , pub get_semaphore_fd_khr : Option < extensions :: khr_external_semaphore_fd :: PFN_vkGetSemaphoreFdKHR > , pub import_semaphore_fd_khr : Option < extensions :: khr_external_semaphore_fd :: PFN_vkImportSemaphoreFdKHR > , pub get_fence_win32_handle_khr : Option < extensions :: khr_external_fence_win32 :: PFN_vkGetFenceWin32HandleKHR > , pub import_fence_win32_handle_khr : Option < extensions :: khr_external_fence_win32 :: PFN_vkImportFenceWin32HandleKHR > , pub get_fence_fd_khr : Option < extensions :: khr_external_fence_fd :: PFN_vkGetFenceFdKHR > , pub import_fence_fd_khr : Option < extensions :: khr_external_fence_fd :: PFN_vkImportFenceFdKHR > , pub display_power_control_ext : Option < extensions :: ext_display_control :: PFN_vkDisplayPowerControlEXT > , pub register_device_event_ext : Option < extensions :: ext_display_control :: PFN_vkRegisterDeviceEventEXT > , pub register_display_event_ext : Option < extensions :: ext_display_control :: PFN_vkRegisterDisplayEventEXT > , pub get_swapchain_counter_ext : Option < extensions :: ext_display_control :: PFN_vkGetSwapchainCounterEXT > , pub get_device_group_peer_memory_features : Option < vk1_1 :: PFN_vkGetDeviceGroupPeerMemoryFeatures > , pub bind_buffer_memory2 : Option < vk1_1 :: PFN_vkBindBufferMemory2 > , pub bind_image_memory2 : Option < vk1_1 :: PFN_vkBindImageMemory2 > , pub cmd_set_device_mask : Option < vk1_1 :: PFN_vkCmdSetDeviceMask > , pub get_device_group_present_capabilities_khr : Option < extensions :: khr_swapchain :: PFN_vkGetDeviceGroupPresentCapabilitiesKHR > , pub get_device_group_surface_present_modes_khr : Option < extensions :: khr_swapchain :: PFN_vkGetDeviceGroupSurfacePresentModesKHR > , pub acquire_next_image2_khr : Option < extensions :: khr_swapchain :: PFN_vkAcquireNextImage2KHR > , pub cmd_dispatch_base : Option < vk1_1 :: PFN_vkCmdDispatchBase > , pub create_descriptor_update_template : Option < vk1_1 :: PFN_vkCreateDescriptorUpdateTemplate > , pub destroy_descriptor_update_template : Option < vk1_1 :: PFN_vkDestroyDescriptorUpdateTemplate > , pub update_descriptor_set_with_template : Option < vk1_1 :: PFN_vkUpdateDescriptorSetWithTemplate > , pub cmd_push_descriptor_set_with_template_khr : Option < extensions :: khr_push_descriptor :: PFN_vkCmdPushDescriptorSetWithTemplateKHR > , pub set_hdr_metadata_ext : Option < extensions :: ext_hdr_metadata :: PFN_vkSetHdrMetadataEXT > , pub get_swapchain_status_khr : Option < extensions :: khr_shared_presentable_image :: PFN_vkGetSwapchainStatusKHR > , pub get_refresh_cycle_duration_google : Option < extensions :: google_display_timing :: PFN_vkGetRefreshCycleDurationGOOGLE > , pub get_past_presentation_timing_google : Option < extensions :: google_display_timing :: PFN_vkGetPastPresentationTimingGOOGLE > , pub cmd_set_viewport_w_scaling_nv : Option < extensions :: nv_clip_space_w_scaling :: PFN_vkCmdSetViewportWScalingNV > , pub cmd_set_discard_rectangle_ext : Option < extensions :: ext_discard_rectangles :: PFN_vkCmdSetDiscardRectangleEXT > , pub cmd_set_sample_locations_ext : Option < extensions :: ext_sample_locations :: PFN_vkCmdSetSampleLocationsEXT > , pub get_buffer_memory_requirements2 : Option < vk1_1 :: PFN_vkGetBufferMemoryRequirements2 > , pub get_image_memory_requirements2 : Option < vk1_1 :: PFN_vkGetImageMemoryRequirements2 > , pub get_image_sparse_memory_requirements2 : Option < vk1_1 :: PFN_vkGetImageSparseMemoryRequirements2 > , pub create_sampler_ycbcr_conversion : Option < vk1_1 :: PFN_vkCreateSamplerYcbcrConversion > , pub destroy_sampler_ycbcr_conversion : Option < vk1_1 :: PFN_vkDestroySamplerYcbcrConversion > , pub get_device_queue2 : Option < vk1_1 :: PFN_vkGetDeviceQueue2 > , pub create_validation_cache_ext : Option < extensions :: ext_validation_cache :: PFN_vkCreateValidationCacheEXT > , pub destroy_validation_cache_ext : Option < extensions :: ext_validation_cache :: PFN_vkDestroyValidationCacheEXT > , pub get_validation_cache_data_ext : Option < extensions :: ext_validation_cache :: PFN_vkGetValidationCacheDataEXT > , pub merge_validation_caches_ext : Option < extensions :: ext_validation_cache :: PFN_vkMergeValidationCachesEXT > , pub get_descriptor_set_layout_support : Option < vk1_1 :: PFN_vkGetDescriptorSetLayoutSupport > , pub get_shader_info_amd : Option < extensions :: amd_shader_info :: PFN_vkGetShaderInfoAMD > , pub set_local_dimming_amd : Option < extensions :: amd_display_native_hdr :: PFN_vkSetLocalDimmingAMD > , pub get_calibrated_timestamps_ext : Option < extensions :: ext_calibrated_timestamps :: PFN_vkGetCalibratedTimestampsEXT > , pub set_debug_utils_object_name_ext : Option < extensions :: ext_debug_utils :: PFN_vkSetDebugUtilsObjectNameEXT > , pub set_debug_utils_object_tag_ext : Option < extensions :: ext_debug_utils :: PFN_vkSetDebugUtilsObjectTagEXT > , pub queue_begin_debug_utils_label_ext : Option < extensions :: ext_debug_utils :: PFN_vkQueueBeginDebugUtilsLabelEXT > , pub queue_end_debug_utils_label_ext : Option < extensions :: ext_debug_utils :: PFN_vkQueueEndDebugUtilsLabelEXT > , pub queue_insert_debug_utils_label_ext : Option < extensions :: ext_debug_utils :: PFN_vkQueueInsertDebugUtilsLabelEXT > , pub cmd_begin_debug_utils_label_ext : Option < extensions :: ext_debug_utils :: PFN_vkCmdBeginDebugUtilsLabelEXT > , pub cmd_end_debug_utils_label_ext : Option < extensions :: ext_debug_utils :: PFN_vkCmdEndDebugUtilsLabelEXT > , pub cmd_insert_debug_utils_label_ext : Option < extensions :: ext_debug_utils :: PFN_vkCmdInsertDebugUtilsLabelEXT > , pub get_memory_host_pointer_properties_ext : Option < extensions :: ext_external_memory_host :: PFN_vkGetMemoryHostPointerPropertiesEXT > , pub cmd_write_buffer_marker_amd : Option < extensions :: amd_buffer_marker :: PFN_vkCmdWriteBufferMarkerAMD > , pub create_render_pass2 : Option < vk1_2 :: PFN_vkCreateRenderPass2 > , pub cmd_begin_render_pass2 : Option < vk1_2 :: PFN_vkCmdBeginRenderPass2 > , pub cmd_next_subpass2 : Option < vk1_2 :: PFN_vkCmdNextSubpass2 > , pub cmd_end_render_pass2 : Option < vk1_2 :: PFN_vkCmdEndRenderPass2 > , pub get_semaphore_counter_value : Option < vk1_2 :: PFN_vkGetSemaphoreCounterValue > , pub wait_semaphores : Option < vk1_2 :: PFN_vkWaitSemaphores > , pub signal_semaphore : Option < vk1_2 :: PFN_vkSignalSemaphore > , pub get_android_hardware_buffer_properties_android : Option < extensions :: android_external_memory_android_hardware_buffer :: PFN_vkGetAndroidHardwareBufferPropertiesANDROID > , pub get_memory_android_hardware_buffer_android : Option < extensions :: android_external_memory_android_hardware_buffer :: PFN_vkGetMemoryAndroidHardwareBufferANDROID > , pub cmd_draw_indirect_count : Option < vk1_2 :: PFN_vkCmdDrawIndirectCount > , pub cmd_draw_indexed_indirect_count : Option < vk1_2 :: PFN_vkCmdDrawIndexedIndirectCount > , pub cmd_set_checkpoint_nv : Option < extensions :: nv_device_diagnostic_checkpoints :: PFN_vkCmdSetCheckpointNV > , pub get_queue_checkpoint_data_nv : Option < extensions :: nv_device_diagnostic_checkpoints :: PFN_vkGetQueueCheckpointDataNV > , pub cmd_bind_transform_feedback_buffers_ext : Option < extensions :: ext_transform_feedback :: PFN_vkCmdBindTransformFeedbackBuffersEXT > , pub cmd_begin_transform_feedback_ext : Option < extensions :: ext_transform_feedback :: PFN_vkCmdBeginTransformFeedbackEXT > , pub cmd_end_transform_feedback_ext : Option < extensions :: ext_transform_feedback :: PFN_vkCmdEndTransformFeedbackEXT > , pub cmd_begin_query_indexed_ext : Option < extensions :: ext_transform_feedback :: PFN_vkCmdBeginQueryIndexedEXT > , pub cmd_end_query_indexed_ext : Option < extensions :: ext_transform_feedback :: PFN_vkCmdEndQueryIndexedEXT > , pub cmd_draw_indirect_byte_count_ext : Option < extensions :: ext_transform_feedback :: PFN_vkCmdDrawIndirectByteCountEXT > , pub cmd_set_exclusive_scissor_nv : Option < extensions :: nv_scissor_exclusive :: PFN_vkCmdSetExclusiveScissorNV > , pub cmd_bind_shading_rate_image_nv : Option < extensions :: nv_shading_rate_image :: PFN_vkCmdBindShadingRateImageNV > , pub cmd_set_viewport_shading_rate_palette_nv : Option < extensions :: nv_shading_rate_image :: PFN_vkCmdSetViewportShadingRatePaletteNV > , pub cmd_set_coarse_sample_order_nv : Option < extensions :: nv_shading_rate_image :: PFN_vkCmdSetCoarseSampleOrderNV > , pub cmd_draw_mesh_tasks_nv : Option < extensions :: nv_mesh_shader :: PFN_vkCmdDrawMeshTasksNV > , pub cmd_draw_mesh_tasks_indirect_nv : Option < extensions :: nv_mesh_shader :: PFN_vkCmdDrawMeshTasksIndirectNV > , pub cmd_draw_mesh_tasks_indirect_count_nv : Option < extensions :: nv_mesh_shader :: PFN_vkCmdDrawMeshTasksIndirectCountNV > , pub compile_deferred_nv : Option < extensions :: nv_ray_tracing :: PFN_vkCompileDeferredNV > , pub create_acceleration_structure_nv : Option < extensions :: nv_ray_tracing :: PFN_vkCreateAccelerationStructureNV > , pub destroy_acceleration_structure_khr : Option < extensions :: khr_ray_tracing :: PFN_vkDestroyAccelerationStructureKHR > , pub get_acceleration_structure_memory_requirements_khr : Option < extensions :: khr_ray_tracing :: PFN_vkGetAccelerationStructureMemoryRequirementsKHR > , pub get_acceleration_structure_memory_requirements_nv : Option < extensions :: nv_ray_tracing :: PFN_vkGetAccelerationStructureMemoryRequirementsNV > , pub bind_acceleration_structure_memory_khr : Option < extensions :: khr_ray_tracing :: PFN_vkBindAccelerationStructureMemoryKHR > , pub cmd_copy_acceleration_structure_nv : Option < extensions :: nv_ray_tracing :: PFN_vkCmdCopyAccelerationStructureNV > , pub cmd_copy_acceleration_structure_khr : Option < extensions :: khr_ray_tracing :: PFN_vkCmdCopyAccelerationStructureKHR > , pub copy_acceleration_structure_khr : Option < extensions :: khr_ray_tracing :: PFN_vkCopyAccelerationStructureKHR > , pub cmd_copy_acceleration_structure_to_memory_khr : Option < extensions :: khr_ray_tracing :: PFN_vkCmdCopyAccelerationStructureToMemoryKHR > , pub copy_acceleration_structure_to_memory_khr : Option < extensions :: khr_ray_tracing :: PFN_vkCopyAccelerationStructureToMemoryKHR > , pub cmd_copy_memory_to_acceleration_structure_khr : Option < extensions :: khr_ray_tracing :: PFN_vkCmdCopyMemoryToAccelerationStructureKHR > , pub copy_memory_to_acceleration_structure_khr : Option < extensions :: khr_ray_tracing :: PFN_vkCopyMemoryToAccelerationStructureKHR > , pub cmd_write_acceleration_structures_properties_khr : Option < extensions :: khr_ray_tracing :: PFN_vkCmdWriteAccelerationStructuresPropertiesKHR > , pub cmd_build_acceleration_structure_nv : Option < extensions :: nv_ray_tracing :: PFN_vkCmdBuildAccelerationStructureNV > , pub write_acceleration_structures_properties_khr : Option < extensions :: khr_ray_tracing :: PFN_vkWriteAccelerationStructuresPropertiesKHR > , pub cmd_trace_rays_khr : Option < extensions :: khr_ray_tracing :: PFN_vkCmdTraceRaysKHR > , pub cmd_trace_rays_nv : Option < extensions :: nv_ray_tracing :: PFN_vkCmdTraceRaysNV > , pub get_ray_tracing_shader_group_handles_khr : Option < extensions :: khr_ray_tracing :: PFN_vkGetRayTracingShaderGroupHandlesKHR > , pub get_ray_tracing_capture_replay_shader_group_handles_khr : Option < extensions :: khr_ray_tracing :: PFN_vkGetRayTracingCaptureReplayShaderGroupHandlesKHR > , pub get_acceleration_structure_handle_nv : Option < extensions :: nv_ray_tracing :: PFN_vkGetAccelerationStructureHandleNV > , pub create_ray_tracing_pipelines_nv : Option < extensions :: nv_ray_tracing :: PFN_vkCreateRayTracingPipelinesNV > , pub create_ray_tracing_pipelines_khr : Option < extensions :: khr_ray_tracing :: PFN_vkCreateRayTracingPipelinesKHR > , pub cmd_trace_rays_indirect_khr : Option < extensions :: khr_ray_tracing :: PFN_vkCmdTraceRaysIndirectKHR > , pub get_device_acceleration_structure_compatibility_khr : Option < extensions :: khr_ray_tracing :: PFN_vkGetDeviceAccelerationStructureCompatibilityKHR > , pub get_image_view_handle_nvx : Option < extensions :: nvx_image_view_handle :: PFN_vkGetImageViewHandleNVX > , pub get_image_view_address_nvx : Option < extensions :: nvx_image_view_handle :: PFN_vkGetImageViewAddressNVX > , pub get_device_group_surface_present_modes2_ext : Option < extensions :: ext_full_screen_exclusive :: PFN_vkGetDeviceGroupSurfacePresentModes2EXT > , pub acquire_full_screen_exclusive_mode_ext : Option < extensions :: ext_full_screen_exclusive :: PFN_vkAcquireFullScreenExclusiveModeEXT > , pub release_full_screen_exclusive_mode_ext : Option < extensions :: ext_full_screen_exclusive :: PFN_vkReleaseFullScreenExclusiveModeEXT > , pub acquire_profiling_lock_khr : Option < extensions :: khr_performance_query :: PFN_vkAcquireProfilingLockKHR > , pub release_profiling_lock_khr : Option < extensions :: khr_performance_query :: PFN_vkReleaseProfilingLockKHR > , pub get_image_drm_format_modifier_properties_ext : Option < extensions :: ext_image_drm_format_modifier :: PFN_vkGetImageDrmFormatModifierPropertiesEXT > , pub get_buffer_opaque_capture_address : Option < vk1_2 :: PFN_vkGetBufferOpaqueCaptureAddress > , pub get_buffer_device_address : Option < vk1_2 :: PFN_vkGetBufferDeviceAddress > , pub initialize_performance_api_intel : Option < extensions :: intel_performance_query :: PFN_vkInitializePerformanceApiINTEL > , pub uninitialize_performance_api_intel : Option < extensions :: intel_performance_query :: PFN_vkUninitializePerformanceApiINTEL > , pub cmd_set_performance_marker_intel : Option < extensions :: intel_performance_query :: PFN_vkCmdSetPerformanceMarkerINTEL > , pub cmd_set_performance_stream_marker_intel : Option < extensions :: intel_performance_query :: PFN_vkCmdSetPerformanceStreamMarkerINTEL > , pub cmd_set_performance_override_intel : Option < extensions :: intel_performance_query :: PFN_vkCmdSetPerformanceOverrideINTEL > , pub acquire_performance_configuration_intel : Option < extensions :: intel_performance_query :: PFN_vkAcquirePerformanceConfigurationINTEL > , pub release_performance_configuration_intel : Option < extensions :: intel_performance_query :: PFN_vkReleasePerformanceConfigurationINTEL > , pub queue_set_performance_configuration_intel : Option < extensions :: intel_performance_query :: PFN_vkQueueSetPerformanceConfigurationINTEL > , pub get_performance_parameter_intel : Option < extensions :: intel_performance_query :: PFN_vkGetPerformanceParameterINTEL > , pub get_device_memory_opaque_capture_address : Option < vk1_2 :: PFN_vkGetDeviceMemoryOpaqueCaptureAddress > , pub get_pipeline_executable_properties_khr : Option < extensions :: khr_pipeline_executable_properties :: PFN_vkGetPipelineExecutablePropertiesKHR > , pub get_pipeline_executable_statistics_khr : Option < extensions :: khr_pipeline_executable_properties :: PFN_vkGetPipelineExecutableStatisticsKHR > , pub get_pipeline_executable_internal_representations_khr : Option < extensions :: khr_pipeline_executable_properties :: PFN_vkGetPipelineExecutableInternalRepresentationsKHR > , pub cmd_set_line_stipple_ext : Option < extensions :: ext_line_rasterization :: PFN_vkCmdSetLineStippleEXT > , pub create_acceleration_structure_khr : Option < extensions :: khr_ray_tracing :: PFN_vkCreateAccelerationStructureKHR > , pub cmd_build_acceleration_structure_khr : Option < extensions :: khr_ray_tracing :: PFN_vkCmdBuildAccelerationStructureKHR > , pub cmd_build_acceleration_structure_indirect_khr : Option < extensions :: khr_ray_tracing :: PFN_vkCmdBuildAccelerationStructureIndirectKHR > , pub build_acceleration_structure_khr : Option < extensions :: khr_ray_tracing :: PFN_vkBuildAccelerationStructureKHR > , pub get_acceleration_structure_device_address_khr : Option < extensions :: khr_ray_tracing :: PFN_vkGetAccelerationStructureDeviceAddressKHR > , pub create_deferred_operation_khr : Option < extensions :: khr_deferred_host_operations :: PFN_vkCreateDeferredOperationKHR > , pub destroy_deferred_operation_khr : Option < extensions :: khr_deferred_host_operations :: PFN_vkDestroyDeferredOperationKHR > , pub get_deferred_operation_max_concurrency_khr : Option < extensions :: khr_deferred_host_operations :: PFN_vkGetDeferredOperationMaxConcurrencyKHR > , pub get_deferred_operation_result_khr : Option < extensions :: khr_deferred_host_operations :: PFN_vkGetDeferredOperationResultKHR > , pub deferred_operation_join_khr : Option < extensions :: khr_deferred_host_operations :: PFN_vkDeferredOperationJoinKHR > , pub cmd_set_cull_mode_ext : Option < extensions :: ext_extended_dynamic_state :: PFN_vkCmdSetCullModeEXT > , pub cmd_set_front_face_ext : Option < extensions :: ext_extended_dynamic_state :: PFN_vkCmdSetFrontFaceEXT > , pub cmd_set_primitive_topology_ext : Option < extensions :: ext_extended_dynamic_state :: PFN_vkCmdSetPrimitiveTopologyEXT > , pub cmd_set_viewport_with_count_ext : Option < extensions :: ext_extended_dynamic_state :: PFN_vkCmdSetViewportWithCountEXT > , pub cmd_set_scissor_with_count_ext : Option < extensions :: ext_extended_dynamic_state :: PFN_vkCmdSetScissorWithCountEXT > , pub cmd_bind_vertex_buffers2_ext : Option < extensions :: ext_extended_dynamic_state :: PFN_vkCmdBindVertexBuffers2EXT > , pub cmd_set_depth_test_enable_ext : Option < extensions :: ext_extended_dynamic_state :: PFN_vkCmdSetDepthTestEnableEXT > , pub cmd_set_depth_write_enable_ext : Option < extensions :: ext_extended_dynamic_state :: PFN_vkCmdSetDepthWriteEnableEXT > , pub cmd_set_depth_compare_op_ext : Option < extensions :: ext_extended_dynamic_state :: PFN_vkCmdSetDepthCompareOpEXT > , pub cmd_set_depth_bounds_test_enable_ext : Option < extensions :: ext_extended_dynamic_state :: PFN_vkCmdSetDepthBoundsTestEnableEXT > , pub cmd_set_stencil_test_enable_ext : Option < extensions :: ext_extended_dynamic_state :: PFN_vkCmdSetStencilTestEnableEXT > , pub cmd_set_stencil_op_ext : Option < extensions :: ext_extended_dynamic_state :: PFN_vkCmdSetStencilOpEXT > , pub create_private_data_slot_ext : Option < extensions :: ext_private_data :: PFN_vkCreatePrivateDataSlotEXT > , pub destroy_private_data_slot_ext : Option < extensions :: ext_private_data :: PFN_vkDestroyPrivateDataSlotEXT > , pub set_private_data_ext : Option < extensions :: ext_private_data :: PFN_vkSetPrivateDataEXT > , pub get_private_data_ext : Option < extensions :: ext_private_data :: PFN_vkGetPrivateDataEXT > , pub reset_query_pool_ext : Option < extensions :: ext_host_query_reset :: PFN_vkResetQueryPoolEXT > , pub trim_command_pool_khr : Option < extensions :: khr_maintenance1 :: PFN_vkTrimCommandPoolKHR > , pub get_device_group_peer_memory_features_khr : Option < extensions :: khr_device_group :: PFN_vkGetDeviceGroupPeerMemoryFeaturesKHR > , pub bind_buffer_memory2_khr : Option < extensions :: khr_bind_memory2 :: PFN_vkBindBufferMemory2KHR > , pub bind_image_memory2_khr : Option < extensions :: khr_bind_memory2 :: PFN_vkBindImageMemory2KHR > , pub cmd_set_device_mask_khr : Option < extensions :: khr_device_group :: PFN_vkCmdSetDeviceMaskKHR > , pub cmd_dispatch_base_khr : Option < extensions :: khr_device_group :: PFN_vkCmdDispatchBaseKHR > , pub create_descriptor_update_template_khr : Option < extensions :: khr_descriptor_update_template :: PFN_vkCreateDescriptorUpdateTemplateKHR > , pub destroy_descriptor_update_template_khr : Option < extensions :: khr_descriptor_update_template :: PFN_vkDestroyDescriptorUpdateTemplateKHR > , pub update_descriptor_set_with_template_khr : Option < extensions :: khr_descriptor_update_template :: PFN_vkUpdateDescriptorSetWithTemplateKHR > , pub get_buffer_memory_requirements2_khr : Option < extensions :: khr_get_memory_requirements2 :: PFN_vkGetBufferMemoryRequirements2KHR > , pub get_image_memory_requirements2_khr : Option < extensions :: khr_get_memory_requirements2 :: PFN_vkGetImageMemoryRequirements2KHR > , pub get_image_sparse_memory_requirements2_khr : Option < extensions :: khr_get_memory_requirements2 :: PFN_vkGetImageSparseMemoryRequirements2KHR > , pub create_sampler_ycbcr_conversion_khr : Option < extensions :: khr_sampler_ycbcr_conversion :: PFN_vkCreateSamplerYcbcrConversionKHR > , pub destroy_sampler_ycbcr_conversion_khr : Option < extensions :: khr_sampler_ycbcr_conversion :: PFN_vkDestroySamplerYcbcrConversionKHR > , pub get_descriptor_set_layout_support_khr : Option < extensions :: khr_maintenance3 :: PFN_vkGetDescriptorSetLayoutSupportKHR > , pub create_render_pass2_khr : Option < extensions :: khr_create_renderpass2 :: PFN_vkCreateRenderPass2KHR > , pub cmd_begin_render_pass2_khr : Option < extensions :: khr_create_renderpass2 :: PFN_vkCmdBeginRenderPass2KHR > , pub cmd_next_subpass2_khr : Option < extensions :: khr_create_renderpass2 :: PFN_vkCmdNextSubpass2KHR > , pub cmd_end_render_pass2_khr : Option < extensions :: khr_create_renderpass2 :: PFN_vkCmdEndRenderPass2KHR > , pub get_semaphore_counter_value_khr : Option < extensions :: khr_timeline_semaphore :: PFN_vkGetSemaphoreCounterValueKHR > , pub wait_semaphores_khr : Option < extensions :: khr_timeline_semaphore :: PFN_vkWaitSemaphoresKHR > , pub signal_semaphore_khr : Option < extensions :: khr_timeline_semaphore :: PFN_vkSignalSemaphoreKHR > , pub cmd_draw_indirect_count_khr : Option < extensions :: khr_draw_indirect_count :: PFN_vkCmdDrawIndirectCountKHR > , pub cmd_draw_indirect_count_amd : Option < extensions :: amd_draw_indirect_count :: PFN_vkCmdDrawIndirectCountAMD > , pub cmd_draw_indexed_indirect_count_khr : Option < extensions :: khr_draw_indirect_count :: PFN_vkCmdDrawIndexedIndirectCountKHR > , pub cmd_draw_indexed_indirect_count_amd : Option < extensions :: amd_draw_indirect_count :: PFN_vkCmdDrawIndexedIndirectCountAMD > , pub destroy_acceleration_structure_nv : Option < extensions :: nv_ray_tracing :: PFN_vkDestroyAccelerationStructureNV > , pub bind_acceleration_structure_memory_nv : Option < extensions :: nv_ray_tracing :: PFN_vkBindAccelerationStructureMemoryNV > , pub cmd_write_acceleration_structures_properties_nv : Option < extensions :: nv_ray_tracing :: PFN_vkCmdWriteAccelerationStructuresPropertiesNV > , pub get_ray_tracing_shader_group_handles_nv : Option < extensions :: nv_ray_tracing :: PFN_vkGetRayTracingShaderGroupHandlesNV > , pub get_buffer_opaque_capture_address_khr : Option < extensions :: khr_buffer_device_address :: PFN_vkGetBufferOpaqueCaptureAddressKHR > , pub get_buffer_device_address_khr : Option < extensions :: khr_buffer_device_address :: PFN_vkGetBufferDeviceAddressKHR > , pub get_buffer_device_address_ext : Option < extensions :: ext_buffer_device_address :: PFN_vkGetBufferDeviceAddressEXT > , pub get_device_memory_opaque_capture_address_khr : Option < extensions :: khr_buffer_device_address :: PFN_vkGetDeviceMemoryOpaqueCaptureAddressKHR > , }
impl DeviceLoader {
    #[inline]
    pub fn custom(
        instance_loader: &InstanceLoader,
        device: crate::vk1_0::Device,
        extensions_len: usize,
        extensions: *const *const std::os::raw::c_char,
        mut symbol: impl FnMut(*const std::os::raw::c_char) -> Option<crate::vk1_0::PFN_vkVoidFunction>,
    ) -> Result<DeviceLoader, crate::LoaderError> {
        let version = instance_loader.selected_instance_version;
        let vk1_2 = version >= crate::vk1_0::make_version(1, 2, 0);
        let ext_conditional_rendering = unsafe {
            crate :: c_str_array_contains ( extensions , extensions_len , crate :: extensions :: ext_conditional_rendering :: EXT_CONDITIONAL_RENDERING_EXTENSION_NAME )
        };
        let khr_display_swapchain = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::khr_display_swapchain::KHR_DISPLAY_SWAPCHAIN_EXTENSION_NAME,
            )
        };
        let khr_swapchain = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::khr_swapchain::KHR_SWAPCHAIN_EXTENSION_NAME,
            )
        };
        let ext_debug_marker = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::ext_debug_marker::EXT_DEBUG_MARKER_EXTENSION_NAME,
            )
        };
        let nv_external_memory_win32 = unsafe {
            crate :: c_str_array_contains ( extensions , extensions_len , crate :: extensions :: nv_external_memory_win32 :: NV_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME )
        };
        let nv_device_generated_commands = unsafe {
            crate :: c_str_array_contains ( extensions , extensions_len , crate :: extensions :: nv_device_generated_commands :: NV_DEVICE_GENERATED_COMMANDS_EXTENSION_NAME )
        };
        let khr_push_descriptor = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::khr_push_descriptor::KHR_PUSH_DESCRIPTOR_EXTENSION_NAME,
            )
        };
        let vk1_1 = version >= crate::vk1_0::make_version(1, 1, 0);
        let khr_external_memory_win32 = unsafe {
            crate :: c_str_array_contains ( extensions , extensions_len , crate :: extensions :: khr_external_memory_win32 :: KHR_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME )
        };
        let khr_external_memory_fd = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::khr_external_memory_fd::KHR_EXTERNAL_MEMORY_FD_EXTENSION_NAME,
            )
        };
        let khr_external_semaphore_win32 = unsafe {
            crate :: c_str_array_contains ( extensions , extensions_len , crate :: extensions :: khr_external_semaphore_win32 :: KHR_EXTERNAL_SEMAPHORE_WIN32_EXTENSION_NAME )
        };
        let khr_external_semaphore_fd = unsafe {
            crate :: c_str_array_contains ( extensions , extensions_len , crate :: extensions :: khr_external_semaphore_fd :: KHR_EXTERNAL_SEMAPHORE_FD_EXTENSION_NAME )
        };
        let khr_external_fence_win32 = unsafe {
            crate :: c_str_array_contains ( extensions , extensions_len , crate :: extensions :: khr_external_fence_win32 :: KHR_EXTERNAL_FENCE_WIN32_EXTENSION_NAME )
        };
        let khr_external_fence_fd = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::khr_external_fence_fd::KHR_EXTERNAL_FENCE_FD_EXTENSION_NAME,
            )
        };
        let ext_display_control = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::ext_display_control::EXT_DISPLAY_CONTROL_EXTENSION_NAME,
            )
        };
        let khr_device_group = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::khr_device_group::KHR_DEVICE_GROUP_EXTENSION_NAME,
            )
        };
        let khr_surface = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::khr_surface::KHR_SURFACE_EXTENSION_NAME,
            )
        };
        let khr_descriptor_update_template = unsafe {
            crate :: c_str_array_contains ( extensions , extensions_len , crate :: extensions :: khr_descriptor_update_template :: KHR_DESCRIPTOR_UPDATE_TEMPLATE_EXTENSION_NAME )
        };
        let ext_hdr_metadata = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::ext_hdr_metadata::EXT_HDR_METADATA_EXTENSION_NAME,
            )
        };
        let khr_shared_presentable_image = unsafe {
            crate :: c_str_array_contains ( extensions , extensions_len , crate :: extensions :: khr_shared_presentable_image :: KHR_SHARED_PRESENTABLE_IMAGE_EXTENSION_NAME )
        };
        let google_display_timing = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::google_display_timing::GOOGLE_DISPLAY_TIMING_EXTENSION_NAME,
            )
        };
        let nv_clip_space_w_scaling = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::nv_clip_space_w_scaling::NV_CLIP_SPACE_W_SCALING_EXTENSION_NAME,
            )
        };
        let ext_discard_rectangles = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::ext_discard_rectangles::EXT_DISCARD_RECTANGLES_EXTENSION_NAME,
            )
        };
        let ext_sample_locations = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::ext_sample_locations::EXT_SAMPLE_LOCATIONS_EXTENSION_NAME,
            )
        };
        let ext_validation_cache = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::ext_validation_cache::EXT_VALIDATION_CACHE_EXTENSION_NAME,
            )
        };
        let amd_shader_info = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::amd_shader_info::AMD_SHADER_INFO_EXTENSION_NAME,
            )
        };
        let amd_display_native_hdr = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::amd_display_native_hdr::AMD_DISPLAY_NATIVE_HDR_EXTENSION_NAME,
            )
        };
        let ext_calibrated_timestamps = unsafe {
            crate :: c_str_array_contains ( extensions , extensions_len , crate :: extensions :: ext_calibrated_timestamps :: EXT_CALIBRATED_TIMESTAMPS_EXTENSION_NAME )
        };
        let ext_debug_utils = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::ext_debug_utils::EXT_DEBUG_UTILS_EXTENSION_NAME,
            )
        };
        let ext_external_memory_host = unsafe {
            crate :: c_str_array_contains ( extensions , extensions_len , crate :: extensions :: ext_external_memory_host :: EXT_EXTERNAL_MEMORY_HOST_EXTENSION_NAME )
        };
        let amd_buffer_marker = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::amd_buffer_marker::AMD_BUFFER_MARKER_EXTENSION_NAME,
            )
        };
        let android_external_memory_android_hardware_buffer = unsafe {
            crate :: c_str_array_contains ( extensions , extensions_len , crate :: extensions :: android_external_memory_android_hardware_buffer :: ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER_EXTENSION_NAME )
        };
        let nv_device_diagnostic_checkpoints = unsafe {
            crate :: c_str_array_contains ( extensions , extensions_len , crate :: extensions :: nv_device_diagnostic_checkpoints :: NV_DEVICE_DIAGNOSTIC_CHECKPOINTS_EXTENSION_NAME )
        };
        let ext_transform_feedback = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::ext_transform_feedback::EXT_TRANSFORM_FEEDBACK_EXTENSION_NAME,
            )
        };
        let nv_scissor_exclusive = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::nv_scissor_exclusive::NV_SCISSOR_EXCLUSIVE_EXTENSION_NAME,
            )
        };
        let nv_shading_rate_image = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::nv_shading_rate_image::NV_SHADING_RATE_IMAGE_EXTENSION_NAME,
            )
        };
        let nv_mesh_shader = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::nv_mesh_shader::NV_MESH_SHADER_EXTENSION_NAME,
            )
        };
        let nv_ray_tracing = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::nv_ray_tracing::NV_RAY_TRACING_EXTENSION_NAME,
            )
        };
        let khr_ray_tracing = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::khr_ray_tracing::KHR_RAY_TRACING_EXTENSION_NAME,
            )
        };
        let nvx_image_view_handle = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::nvx_image_view_handle::NVX_IMAGE_VIEW_HANDLE_EXTENSION_NAME,
            )
        };
        let ext_full_screen_exclusive = unsafe {
            crate :: c_str_array_contains ( extensions , extensions_len , crate :: extensions :: ext_full_screen_exclusive :: EXT_FULL_SCREEN_EXCLUSIVE_EXTENSION_NAME )
        };
        let khr_performance_query = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::khr_performance_query::KHR_PERFORMANCE_QUERY_EXTENSION_NAME,
            )
        };
        let ext_image_drm_format_modifier = unsafe {
            crate :: c_str_array_contains ( extensions , extensions_len , crate :: extensions :: ext_image_drm_format_modifier :: EXT_IMAGE_DRM_FORMAT_MODIFIER_EXTENSION_NAME )
        };
        let intel_performance_query = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::intel_performance_query::INTEL_PERFORMANCE_QUERY_EXTENSION_NAME,
            )
        };
        let khr_pipeline_executable_properties = unsafe {
            crate :: c_str_array_contains ( extensions , extensions_len , crate :: extensions :: khr_pipeline_executable_properties :: KHR_PIPELINE_EXECUTABLE_PROPERTIES_EXTENSION_NAME )
        };
        let ext_line_rasterization = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::ext_line_rasterization::EXT_LINE_RASTERIZATION_EXTENSION_NAME,
            )
        };
        let khr_deferred_host_operations = unsafe {
            crate :: c_str_array_contains ( extensions , extensions_len , crate :: extensions :: khr_deferred_host_operations :: KHR_DEFERRED_HOST_OPERATIONS_EXTENSION_NAME )
        };
        let ext_extended_dynamic_state = unsafe {
            crate :: c_str_array_contains ( extensions , extensions_len , crate :: extensions :: ext_extended_dynamic_state :: EXT_EXTENDED_DYNAMIC_STATE_EXTENSION_NAME )
        };
        let ext_private_data = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::ext_private_data::EXT_PRIVATE_DATA_EXTENSION_NAME,
            )
        };
        let ext_host_query_reset = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::ext_host_query_reset::EXT_HOST_QUERY_RESET_EXTENSION_NAME,
            )
        };
        let khr_maintenance1 = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::khr_maintenance1::KHR_MAINTENANCE1_EXTENSION_NAME,
            )
        };
        let khr_bind_memory2 = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::khr_bind_memory2::KHR_BIND_MEMORY_2_EXTENSION_NAME,
            )
        };
        let khr_get_memory_requirements2 = unsafe {
            crate :: c_str_array_contains ( extensions , extensions_len , crate :: extensions :: khr_get_memory_requirements2 :: KHR_GET_MEMORY_REQUIREMENTS_2_EXTENSION_NAME )
        };
        let khr_sampler_ycbcr_conversion = unsafe {
            crate :: c_str_array_contains ( extensions , extensions_len , crate :: extensions :: khr_sampler_ycbcr_conversion :: KHR_SAMPLER_YCBCR_CONVERSION_EXTENSION_NAME )
        };
        let khr_maintenance3 = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::khr_maintenance3::KHR_MAINTENANCE3_EXTENSION_NAME,
            )
        };
        let khr_create_renderpass2 = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::khr_create_renderpass2::KHR_CREATE_RENDERPASS_2_EXTENSION_NAME,
            )
        };
        let khr_timeline_semaphore = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::khr_timeline_semaphore::KHR_TIMELINE_SEMAPHORE_EXTENSION_NAME,
            )
        };
        let khr_draw_indirect_count = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::khr_draw_indirect_count::KHR_DRAW_INDIRECT_COUNT_EXTENSION_NAME,
            )
        };
        let amd_draw_indirect_count = unsafe {
            crate::c_str_array_contains(
                extensions,
                extensions_len,
                crate::extensions::amd_draw_indirect_count::AMD_DRAW_INDIRECT_COUNT_EXTENSION_NAME,
            )
        };
        let khr_buffer_device_address = unsafe {
            crate :: c_str_array_contains ( extensions , extensions_len , crate :: extensions :: khr_buffer_device_address :: KHR_BUFFER_DEVICE_ADDRESS_EXTENSION_NAME )
        };
        let ext_buffer_device_address = unsafe {
            crate :: c_str_array_contains ( extensions , extensions_len , crate :: extensions :: ext_buffer_device_address :: EXT_BUFFER_DEVICE_ADDRESS_EXTENSION_NAME )
        };
        Ok(DeviceLoader {
            parent: std::sync::Arc::downgrade(&instance_loader.arc),
            handle: device,
            get_device_proc_addr: unsafe {
                match symbol(crate::vk1_0::FN_GET_DEVICE_PROC_ADDR) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            destroy_device: unsafe {
                match symbol(crate::vk1_0::FN_DESTROY_DEVICE) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            get_device_queue: unsafe {
                match symbol(crate::vk1_0::FN_GET_DEVICE_QUEUE) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            queue_submit: unsafe {
                match symbol(crate::vk1_0::FN_QUEUE_SUBMIT) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            queue_wait_idle: unsafe {
                match symbol(crate::vk1_0::FN_QUEUE_WAIT_IDLE) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            device_wait_idle: unsafe {
                match symbol(crate::vk1_0::FN_DEVICE_WAIT_IDLE) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            allocate_memory: unsafe {
                match symbol(crate::vk1_0::FN_ALLOCATE_MEMORY) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            free_memory: unsafe {
                match symbol(crate::vk1_0::FN_FREE_MEMORY) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            map_memory: unsafe {
                match symbol(crate::vk1_0::FN_MAP_MEMORY) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            unmap_memory: unsafe {
                match symbol(crate::vk1_0::FN_UNMAP_MEMORY) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            flush_mapped_memory_ranges: unsafe {
                match symbol(crate::vk1_0::FN_FLUSH_MAPPED_MEMORY_RANGES) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            invalidate_mapped_memory_ranges: unsafe {
                match symbol(crate::vk1_0::FN_INVALIDATE_MAPPED_MEMORY_RANGES) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            get_device_memory_commitment: unsafe {
                match symbol(crate::vk1_0::FN_GET_DEVICE_MEMORY_COMMITMENT) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            get_buffer_memory_requirements: unsafe {
                match symbol(crate::vk1_0::FN_GET_BUFFER_MEMORY_REQUIREMENTS) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            bind_buffer_memory: unsafe {
                match symbol(crate::vk1_0::FN_BIND_BUFFER_MEMORY) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            get_image_memory_requirements: unsafe {
                match symbol(crate::vk1_0::FN_GET_IMAGE_MEMORY_REQUIREMENTS) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            bind_image_memory: unsafe {
                match symbol(crate::vk1_0::FN_BIND_IMAGE_MEMORY) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            get_image_sparse_memory_requirements: unsafe {
                match symbol(crate::vk1_0::FN_GET_IMAGE_SPARSE_MEMORY_REQUIREMENTS) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            queue_bind_sparse: unsafe {
                match symbol(crate::vk1_0::FN_QUEUE_BIND_SPARSE) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            create_fence: unsafe {
                match symbol(crate::vk1_0::FN_CREATE_FENCE) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            destroy_fence: unsafe {
                match symbol(crate::vk1_0::FN_DESTROY_FENCE) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            reset_fences: unsafe {
                match symbol(crate::vk1_0::FN_RESET_FENCES) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            get_fence_status: unsafe {
                match symbol(crate::vk1_0::FN_GET_FENCE_STATUS) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            wait_for_fences: unsafe {
                match symbol(crate::vk1_0::FN_WAIT_FOR_FENCES) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            create_semaphore: unsafe {
                match symbol(crate::vk1_0::FN_CREATE_SEMAPHORE) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            destroy_semaphore: unsafe {
                match symbol(crate::vk1_0::FN_DESTROY_SEMAPHORE) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            create_event: unsafe {
                match symbol(crate::vk1_0::FN_CREATE_EVENT) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            destroy_event: unsafe {
                match symbol(crate::vk1_0::FN_DESTROY_EVENT) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            get_event_status: unsafe {
                match symbol(crate::vk1_0::FN_GET_EVENT_STATUS) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            set_event: unsafe {
                match symbol(crate::vk1_0::FN_SET_EVENT) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            reset_event: unsafe {
                match symbol(crate::vk1_0::FN_RESET_EVENT) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            create_query_pool: unsafe {
                match symbol(crate::vk1_0::FN_CREATE_QUERY_POOL) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            destroy_query_pool: unsafe {
                match symbol(crate::vk1_0::FN_DESTROY_QUERY_POOL) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            get_query_pool_results: unsafe {
                match symbol(crate::vk1_0::FN_GET_QUERY_POOL_RESULTS) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            reset_query_pool: if vk1_2 {
                unsafe {
                    match symbol(crate::vk1_2::FN_RESET_QUERY_POOL) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            create_buffer: unsafe {
                match symbol(crate::vk1_0::FN_CREATE_BUFFER) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            destroy_buffer: unsafe {
                match symbol(crate::vk1_0::FN_DESTROY_BUFFER) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            create_buffer_view: unsafe {
                match symbol(crate::vk1_0::FN_CREATE_BUFFER_VIEW) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            destroy_buffer_view: unsafe {
                match symbol(crate::vk1_0::FN_DESTROY_BUFFER_VIEW) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            create_image: unsafe {
                match symbol(crate::vk1_0::FN_CREATE_IMAGE) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            destroy_image: unsafe {
                match symbol(crate::vk1_0::FN_DESTROY_IMAGE) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            get_image_subresource_layout: unsafe {
                match symbol(crate::vk1_0::FN_GET_IMAGE_SUBRESOURCE_LAYOUT) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            create_image_view: unsafe {
                match symbol(crate::vk1_0::FN_CREATE_IMAGE_VIEW) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            destroy_image_view: unsafe {
                match symbol(crate::vk1_0::FN_DESTROY_IMAGE_VIEW) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            create_shader_module: unsafe {
                match symbol(crate::vk1_0::FN_CREATE_SHADER_MODULE) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            destroy_shader_module: unsafe {
                match symbol(crate::vk1_0::FN_DESTROY_SHADER_MODULE) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            create_pipeline_cache: unsafe {
                match symbol(crate::vk1_0::FN_CREATE_PIPELINE_CACHE) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            destroy_pipeline_cache: unsafe {
                match symbol(crate::vk1_0::FN_DESTROY_PIPELINE_CACHE) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            get_pipeline_cache_data: unsafe {
                match symbol(crate::vk1_0::FN_GET_PIPELINE_CACHE_DATA) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            merge_pipeline_caches: unsafe {
                match symbol(crate::vk1_0::FN_MERGE_PIPELINE_CACHES) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            create_graphics_pipelines: unsafe {
                match symbol(crate::vk1_0::FN_CREATE_GRAPHICS_PIPELINES) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            create_compute_pipelines: unsafe {
                match symbol(crate::vk1_0::FN_CREATE_COMPUTE_PIPELINES) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            destroy_pipeline: unsafe {
                match symbol(crate::vk1_0::FN_DESTROY_PIPELINE) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            create_pipeline_layout: unsafe {
                match symbol(crate::vk1_0::FN_CREATE_PIPELINE_LAYOUT) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            destroy_pipeline_layout: unsafe {
                match symbol(crate::vk1_0::FN_DESTROY_PIPELINE_LAYOUT) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            create_sampler: unsafe {
                match symbol(crate::vk1_0::FN_CREATE_SAMPLER) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            destroy_sampler: unsafe {
                match symbol(crate::vk1_0::FN_DESTROY_SAMPLER) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            create_descriptor_set_layout: unsafe {
                match symbol(crate::vk1_0::FN_CREATE_DESCRIPTOR_SET_LAYOUT) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            destroy_descriptor_set_layout: unsafe {
                match symbol(crate::vk1_0::FN_DESTROY_DESCRIPTOR_SET_LAYOUT) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            create_descriptor_pool: unsafe {
                match symbol(crate::vk1_0::FN_CREATE_DESCRIPTOR_POOL) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            destroy_descriptor_pool: unsafe {
                match symbol(crate::vk1_0::FN_DESTROY_DESCRIPTOR_POOL) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            reset_descriptor_pool: unsafe {
                match symbol(crate::vk1_0::FN_RESET_DESCRIPTOR_POOL) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            allocate_descriptor_sets: unsafe {
                match symbol(crate::vk1_0::FN_ALLOCATE_DESCRIPTOR_SETS) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            free_descriptor_sets: unsafe {
                match symbol(crate::vk1_0::FN_FREE_DESCRIPTOR_SETS) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            update_descriptor_sets: unsafe {
                match symbol(crate::vk1_0::FN_UPDATE_DESCRIPTOR_SETS) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            create_framebuffer: unsafe {
                match symbol(crate::vk1_0::FN_CREATE_FRAMEBUFFER) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            destroy_framebuffer: unsafe {
                match symbol(crate::vk1_0::FN_DESTROY_FRAMEBUFFER) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            create_render_pass: unsafe {
                match symbol(crate::vk1_0::FN_CREATE_RENDER_PASS) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            destroy_render_pass: unsafe {
                match symbol(crate::vk1_0::FN_DESTROY_RENDER_PASS) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            get_render_area_granularity: unsafe {
                match symbol(crate::vk1_0::FN_GET_RENDER_AREA_GRANULARITY) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            create_command_pool: unsafe {
                match symbol(crate::vk1_0::FN_CREATE_COMMAND_POOL) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            destroy_command_pool: unsafe {
                match symbol(crate::vk1_0::FN_DESTROY_COMMAND_POOL) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            reset_command_pool: unsafe {
                match symbol(crate::vk1_0::FN_RESET_COMMAND_POOL) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            allocate_command_buffers: unsafe {
                match symbol(crate::vk1_0::FN_ALLOCATE_COMMAND_BUFFERS) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            free_command_buffers: unsafe {
                match symbol(crate::vk1_0::FN_FREE_COMMAND_BUFFERS) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            begin_command_buffer: unsafe {
                match symbol(crate::vk1_0::FN_BEGIN_COMMAND_BUFFER) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            end_command_buffer: unsafe {
                match symbol(crate::vk1_0::FN_END_COMMAND_BUFFER) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            reset_command_buffer: unsafe {
                match symbol(crate::vk1_0::FN_RESET_COMMAND_BUFFER) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            cmd_bind_pipeline: unsafe {
                match symbol(crate::vk1_0::FN_CMD_BIND_PIPELINE) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            cmd_set_viewport: unsafe {
                match symbol(crate::vk1_0::FN_CMD_SET_VIEWPORT) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            cmd_set_scissor: unsafe {
                match symbol(crate::vk1_0::FN_CMD_SET_SCISSOR) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            cmd_set_line_width: unsafe {
                match symbol(crate::vk1_0::FN_CMD_SET_LINE_WIDTH) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            cmd_set_depth_bias: unsafe {
                match symbol(crate::vk1_0::FN_CMD_SET_DEPTH_BIAS) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            cmd_set_blend_constants: unsafe {
                match symbol(crate::vk1_0::FN_CMD_SET_BLEND_CONSTANTS) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            cmd_set_depth_bounds: unsafe {
                match symbol(crate::vk1_0::FN_CMD_SET_DEPTH_BOUNDS) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            cmd_set_stencil_compare_mask: unsafe {
                match symbol(crate::vk1_0::FN_CMD_SET_STENCIL_COMPARE_MASK) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            cmd_set_stencil_write_mask: unsafe {
                match symbol(crate::vk1_0::FN_CMD_SET_STENCIL_WRITE_MASK) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            cmd_set_stencil_reference: unsafe {
                match symbol(crate::vk1_0::FN_CMD_SET_STENCIL_REFERENCE) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            cmd_bind_descriptor_sets: unsafe {
                match symbol(crate::vk1_0::FN_CMD_BIND_DESCRIPTOR_SETS) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            cmd_bind_index_buffer: unsafe {
                match symbol(crate::vk1_0::FN_CMD_BIND_INDEX_BUFFER) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            cmd_bind_vertex_buffers: unsafe {
                match symbol(crate::vk1_0::FN_CMD_BIND_VERTEX_BUFFERS) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            cmd_draw: unsafe {
                match symbol(crate::vk1_0::FN_CMD_DRAW) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            cmd_draw_indexed: unsafe {
                match symbol(crate::vk1_0::FN_CMD_DRAW_INDEXED) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            cmd_draw_indirect: unsafe {
                match symbol(crate::vk1_0::FN_CMD_DRAW_INDIRECT) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            cmd_draw_indexed_indirect: unsafe {
                match symbol(crate::vk1_0::FN_CMD_DRAW_INDEXED_INDIRECT) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            cmd_dispatch: unsafe {
                match symbol(crate::vk1_0::FN_CMD_DISPATCH) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            cmd_dispatch_indirect: unsafe {
                match symbol(crate::vk1_0::FN_CMD_DISPATCH_INDIRECT) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            cmd_copy_buffer: unsafe {
                match symbol(crate::vk1_0::FN_CMD_COPY_BUFFER) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            cmd_copy_image: unsafe {
                match symbol(crate::vk1_0::FN_CMD_COPY_IMAGE) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            cmd_blit_image: unsafe {
                match symbol(crate::vk1_0::FN_CMD_BLIT_IMAGE) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            cmd_copy_buffer_to_image: unsafe {
                match symbol(crate::vk1_0::FN_CMD_COPY_BUFFER_TO_IMAGE) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            cmd_copy_image_to_buffer: unsafe {
                match symbol(crate::vk1_0::FN_CMD_COPY_IMAGE_TO_BUFFER) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            cmd_update_buffer: unsafe {
                match symbol(crate::vk1_0::FN_CMD_UPDATE_BUFFER) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            cmd_fill_buffer: unsafe {
                match symbol(crate::vk1_0::FN_CMD_FILL_BUFFER) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            cmd_clear_color_image: unsafe {
                match symbol(crate::vk1_0::FN_CMD_CLEAR_COLOR_IMAGE) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            cmd_clear_depth_stencil_image: unsafe {
                match symbol(crate::vk1_0::FN_CMD_CLEAR_DEPTH_STENCIL_IMAGE) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            cmd_clear_attachments: unsafe {
                match symbol(crate::vk1_0::FN_CMD_CLEAR_ATTACHMENTS) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            cmd_resolve_image: unsafe {
                match symbol(crate::vk1_0::FN_CMD_RESOLVE_IMAGE) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            cmd_set_event: unsafe {
                match symbol(crate::vk1_0::FN_CMD_SET_EVENT) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            cmd_reset_event: unsafe {
                match symbol(crate::vk1_0::FN_CMD_RESET_EVENT) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            cmd_wait_events: unsafe {
                match symbol(crate::vk1_0::FN_CMD_WAIT_EVENTS) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            cmd_pipeline_barrier: unsafe {
                match symbol(crate::vk1_0::FN_CMD_PIPELINE_BARRIER) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            cmd_begin_query: unsafe {
                match symbol(crate::vk1_0::FN_CMD_BEGIN_QUERY) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            cmd_end_query: unsafe {
                match symbol(crate::vk1_0::FN_CMD_END_QUERY) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            cmd_begin_conditional_rendering_ext: if ext_conditional_rendering {
                unsafe {
                    match symbol ( crate :: extensions :: ext_conditional_rendering :: FN_CMD_BEGIN_CONDITIONAL_RENDERING_EXT ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            cmd_end_conditional_rendering_ext: if ext_conditional_rendering {
                unsafe {
                    match symbol ( crate :: extensions :: ext_conditional_rendering :: FN_CMD_END_CONDITIONAL_RENDERING_EXT ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            cmd_reset_query_pool: unsafe {
                match symbol(crate::vk1_0::FN_CMD_RESET_QUERY_POOL) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            cmd_write_timestamp: unsafe {
                match symbol(crate::vk1_0::FN_CMD_WRITE_TIMESTAMP) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            cmd_copy_query_pool_results: unsafe {
                match symbol(crate::vk1_0::FN_CMD_COPY_QUERY_POOL_RESULTS) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            cmd_push_constants: unsafe {
                match symbol(crate::vk1_0::FN_CMD_PUSH_CONSTANTS) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            cmd_begin_render_pass: unsafe {
                match symbol(crate::vk1_0::FN_CMD_BEGIN_RENDER_PASS) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            cmd_next_subpass: unsafe {
                match symbol(crate::vk1_0::FN_CMD_NEXT_SUBPASS) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            cmd_end_render_pass: unsafe {
                match symbol(crate::vk1_0::FN_CMD_END_RENDER_PASS) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            cmd_execute_commands: unsafe {
                match symbol(crate::vk1_0::FN_CMD_EXECUTE_COMMANDS) {
                    Some(ptr) => Some(std::mem::transmute(ptr)),
                    None => return Err(crate::LoaderError::SymbolNotAvailable),
                }
            },
            create_shared_swapchains_khr: if khr_display_swapchain {
                unsafe {
                    match symbol(
                        crate::extensions::khr_display_swapchain::FN_CREATE_SHARED_SWAPCHAINS_KHR,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            create_swapchain_khr: if khr_swapchain {
                unsafe {
                    match symbol(crate::extensions::khr_swapchain::FN_CREATE_SWAPCHAIN_KHR) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            destroy_swapchain_khr: if khr_swapchain {
                unsafe {
                    match symbol(crate::extensions::khr_swapchain::FN_DESTROY_SWAPCHAIN_KHR) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_swapchain_images_khr: if khr_swapchain {
                unsafe {
                    match symbol(crate::extensions::khr_swapchain::FN_GET_SWAPCHAIN_IMAGES_KHR) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            acquire_next_image_khr: if khr_swapchain {
                unsafe {
                    match symbol(crate::extensions::khr_swapchain::FN_ACQUIRE_NEXT_IMAGE_KHR) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            queue_present_khr: if khr_swapchain {
                unsafe {
                    match symbol(crate::extensions::khr_swapchain::FN_QUEUE_PRESENT_KHR) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            debug_marker_set_object_name_ext: if ext_debug_marker {
                unsafe {
                    match symbol(
                        crate::extensions::ext_debug_marker::FN_DEBUG_MARKER_SET_OBJECT_NAME_EXT,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            debug_marker_set_object_tag_ext: if ext_debug_marker {
                unsafe {
                    match symbol(
                        crate::extensions::ext_debug_marker::FN_DEBUG_MARKER_SET_OBJECT_TAG_EXT,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            cmd_debug_marker_begin_ext: if ext_debug_marker {
                unsafe {
                    match symbol(crate::extensions::ext_debug_marker::FN_CMD_DEBUG_MARKER_BEGIN_EXT)
                    {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            cmd_debug_marker_end_ext: if ext_debug_marker {
                unsafe {
                    match symbol(crate::extensions::ext_debug_marker::FN_CMD_DEBUG_MARKER_END_EXT) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            cmd_debug_marker_insert_ext: if ext_debug_marker {
                unsafe {
                    match symbol(
                        crate::extensions::ext_debug_marker::FN_CMD_DEBUG_MARKER_INSERT_EXT,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_memory_win32_handle_nv: if nv_external_memory_win32 {
                unsafe {
                    match symbol(
                        crate::extensions::nv_external_memory_win32::FN_GET_MEMORY_WIN32_HANDLE_NV,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            cmd_execute_generated_commands_nv: if nv_device_generated_commands {
                unsafe {
                    match symbol ( crate :: extensions :: nv_device_generated_commands :: FN_CMD_EXECUTE_GENERATED_COMMANDS_NV ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            cmd_preprocess_generated_commands_nv: if nv_device_generated_commands {
                unsafe {
                    match symbol ( crate :: extensions :: nv_device_generated_commands :: FN_CMD_PREPROCESS_GENERATED_COMMANDS_NV ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            cmd_bind_pipeline_shader_group_nv: if nv_device_generated_commands {
                unsafe {
                    match symbol ( crate :: extensions :: nv_device_generated_commands :: FN_CMD_BIND_PIPELINE_SHADER_GROUP_NV ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            get_generated_commands_memory_requirements_nv: if nv_device_generated_commands {
                unsafe {
                    match symbol ( crate :: extensions :: nv_device_generated_commands :: FN_GET_GENERATED_COMMANDS_MEMORY_REQUIREMENTS_NV ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            create_indirect_commands_layout_nv: if nv_device_generated_commands {
                unsafe {
                    match symbol ( crate :: extensions :: nv_device_generated_commands :: FN_CREATE_INDIRECT_COMMANDS_LAYOUT_NV ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            destroy_indirect_commands_layout_nv: if nv_device_generated_commands {
                unsafe {
                    match symbol ( crate :: extensions :: nv_device_generated_commands :: FN_DESTROY_INDIRECT_COMMANDS_LAYOUT_NV ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            cmd_push_descriptor_set_khr: if khr_push_descriptor {
                unsafe {
                    match symbol(
                        crate::extensions::khr_push_descriptor::FN_CMD_PUSH_DESCRIPTOR_SET_KHR,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            trim_command_pool: if vk1_1 {
                unsafe {
                    match symbol(crate::vk1_1::FN_TRIM_COMMAND_POOL) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_memory_win32_handle_khr: if khr_external_memory_win32 {
                unsafe {
                    match symbol ( crate :: extensions :: khr_external_memory_win32 :: FN_GET_MEMORY_WIN32_HANDLE_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            get_memory_win32_handle_properties_khr: if khr_external_memory_win32 {
                unsafe {
                    match symbol ( crate :: extensions :: khr_external_memory_win32 :: FN_GET_MEMORY_WIN32_HANDLE_PROPERTIES_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            get_memory_fd_khr: if khr_external_memory_fd {
                unsafe {
                    match symbol(crate::extensions::khr_external_memory_fd::FN_GET_MEMORY_FD_KHR) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_memory_fd_properties_khr: if khr_external_memory_fd {
                unsafe {
                    match symbol(
                        crate::extensions::khr_external_memory_fd::FN_GET_MEMORY_FD_PROPERTIES_KHR,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_semaphore_win32_handle_khr: if khr_external_semaphore_win32 {
                unsafe {
                    match symbol ( crate :: extensions :: khr_external_semaphore_win32 :: FN_GET_SEMAPHORE_WIN32_HANDLE_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            import_semaphore_win32_handle_khr: if khr_external_semaphore_win32 {
                unsafe {
                    match symbol ( crate :: extensions :: khr_external_semaphore_win32 :: FN_IMPORT_SEMAPHORE_WIN32_HANDLE_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            get_semaphore_fd_khr: if khr_external_semaphore_fd {
                unsafe {
                    match symbol(
                        crate::extensions::khr_external_semaphore_fd::FN_GET_SEMAPHORE_FD_KHR,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            import_semaphore_fd_khr: if khr_external_semaphore_fd {
                unsafe {
                    match symbol(
                        crate::extensions::khr_external_semaphore_fd::FN_IMPORT_SEMAPHORE_FD_KHR,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_fence_win32_handle_khr: if khr_external_fence_win32 {
                unsafe {
                    match symbol(
                        crate::extensions::khr_external_fence_win32::FN_GET_FENCE_WIN32_HANDLE_KHR,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            import_fence_win32_handle_khr: if khr_external_fence_win32 {
                unsafe {
                    match symbol ( crate :: extensions :: khr_external_fence_win32 :: FN_IMPORT_FENCE_WIN32_HANDLE_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            get_fence_fd_khr: if khr_external_fence_fd {
                unsafe {
                    match symbol(crate::extensions::khr_external_fence_fd::FN_GET_FENCE_FD_KHR) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            import_fence_fd_khr: if khr_external_fence_fd {
                unsafe {
                    match symbol(crate::extensions::khr_external_fence_fd::FN_IMPORT_FENCE_FD_KHR) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            display_power_control_ext: if ext_display_control {
                unsafe {
                    match symbol(
                        crate::extensions::ext_display_control::FN_DISPLAY_POWER_CONTROL_EXT,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            register_device_event_ext: if ext_display_control {
                unsafe {
                    match symbol(
                        crate::extensions::ext_display_control::FN_REGISTER_DEVICE_EVENT_EXT,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            register_display_event_ext: if ext_display_control {
                unsafe {
                    match symbol(
                        crate::extensions::ext_display_control::FN_REGISTER_DISPLAY_EVENT_EXT,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_swapchain_counter_ext: if ext_display_control {
                unsafe {
                    match symbol(
                        crate::extensions::ext_display_control::FN_GET_SWAPCHAIN_COUNTER_EXT,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_device_group_peer_memory_features: if vk1_1 {
                unsafe {
                    match symbol(crate::vk1_1::FN_GET_DEVICE_GROUP_PEER_MEMORY_FEATURES) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            bind_buffer_memory2: if vk1_1 {
                unsafe {
                    match symbol(crate::vk1_1::FN_BIND_BUFFER_MEMORY2) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            bind_image_memory2: if vk1_1 {
                unsafe {
                    match symbol(crate::vk1_1::FN_BIND_IMAGE_MEMORY2) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            cmd_set_device_mask: if vk1_1 {
                unsafe {
                    match symbol(crate::vk1_1::FN_CMD_SET_DEVICE_MASK) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_device_group_present_capabilities_khr: if (khr_swapchain && vk1_1)
                || (khr_device_group && khr_surface)
            {
                unsafe {
                    match symbol ( crate :: extensions :: khr_swapchain :: FN_GET_DEVICE_GROUP_PRESENT_CAPABILITIES_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            get_device_group_surface_present_modes_khr: if (khr_swapchain && vk1_1)
                || (khr_device_group && khr_surface)
            {
                unsafe {
                    match symbol ( crate :: extensions :: khr_swapchain :: FN_GET_DEVICE_GROUP_SURFACE_PRESENT_MODES_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            acquire_next_image2_khr: if (khr_swapchain && vk1_1)
                || (khr_device_group && khr_swapchain)
            {
                unsafe {
                    match symbol(crate::extensions::khr_swapchain::FN_ACQUIRE_NEXT_IMAGE2_KHR) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            cmd_dispatch_base: if vk1_1 {
                unsafe {
                    match symbol(crate::vk1_1::FN_CMD_DISPATCH_BASE) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            create_descriptor_update_template: if vk1_1 {
                unsafe {
                    match symbol(crate::vk1_1::FN_CREATE_DESCRIPTOR_UPDATE_TEMPLATE) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            destroy_descriptor_update_template: if vk1_1 {
                unsafe {
                    match symbol(crate::vk1_1::FN_DESTROY_DESCRIPTOR_UPDATE_TEMPLATE) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            update_descriptor_set_with_template: if vk1_1 {
                unsafe {
                    match symbol(crate::vk1_1::FN_UPDATE_DESCRIPTOR_SET_WITH_TEMPLATE) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            cmd_push_descriptor_set_with_template_khr: if (khr_push_descriptor && vk1_1)
                || (khr_push_descriptor && khr_descriptor_update_template)
                || (khr_descriptor_update_template && khr_push_descriptor)
            {
                unsafe {
                    match symbol ( crate :: extensions :: khr_push_descriptor :: FN_CMD_PUSH_DESCRIPTOR_SET_WITH_TEMPLATE_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            set_hdr_metadata_ext: if ext_hdr_metadata {
                unsafe {
                    match symbol(crate::extensions::ext_hdr_metadata::FN_SET_HDR_METADATA_EXT) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_swapchain_status_khr: if khr_shared_presentable_image {
                unsafe {
                    match symbol ( crate :: extensions :: khr_shared_presentable_image :: FN_GET_SWAPCHAIN_STATUS_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            get_refresh_cycle_duration_google: if google_display_timing {
                unsafe {
                    match symbol ( crate :: extensions :: google_display_timing :: FN_GET_REFRESH_CYCLE_DURATION_GOOGLE ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            get_past_presentation_timing_google: if google_display_timing {
                unsafe {
                    match symbol ( crate :: extensions :: google_display_timing :: FN_GET_PAST_PRESENTATION_TIMING_GOOGLE ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            cmd_set_viewport_w_scaling_nv: if nv_clip_space_w_scaling {
                unsafe {
                    match symbol ( crate :: extensions :: nv_clip_space_w_scaling :: FN_CMD_SET_VIEWPORT_W_SCALING_NV ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            cmd_set_discard_rectangle_ext: if ext_discard_rectangles {
                unsafe {
                    match symbol(
                        crate::extensions::ext_discard_rectangles::FN_CMD_SET_DISCARD_RECTANGLE_EXT,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            cmd_set_sample_locations_ext: if ext_sample_locations {
                unsafe {
                    match symbol(
                        crate::extensions::ext_sample_locations::FN_CMD_SET_SAMPLE_LOCATIONS_EXT,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_buffer_memory_requirements2: if vk1_1 {
                unsafe {
                    match symbol(crate::vk1_1::FN_GET_BUFFER_MEMORY_REQUIREMENTS2) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_image_memory_requirements2: if vk1_1 {
                unsafe {
                    match symbol(crate::vk1_1::FN_GET_IMAGE_MEMORY_REQUIREMENTS2) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_image_sparse_memory_requirements2: if vk1_1 {
                unsafe {
                    match symbol(crate::vk1_1::FN_GET_IMAGE_SPARSE_MEMORY_REQUIREMENTS2) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            create_sampler_ycbcr_conversion: if vk1_1 {
                unsafe {
                    match symbol(crate::vk1_1::FN_CREATE_SAMPLER_YCBCR_CONVERSION) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            destroy_sampler_ycbcr_conversion: if vk1_1 {
                unsafe {
                    match symbol(crate::vk1_1::FN_DESTROY_SAMPLER_YCBCR_CONVERSION) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_device_queue2: if vk1_1 {
                unsafe {
                    match symbol(crate::vk1_1::FN_GET_DEVICE_QUEUE2) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            create_validation_cache_ext: if ext_validation_cache {
                unsafe {
                    match symbol(
                        crate::extensions::ext_validation_cache::FN_CREATE_VALIDATION_CACHE_EXT,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            destroy_validation_cache_ext: if ext_validation_cache {
                unsafe {
                    match symbol(
                        crate::extensions::ext_validation_cache::FN_DESTROY_VALIDATION_CACHE_EXT,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_validation_cache_data_ext: if ext_validation_cache {
                unsafe {
                    match symbol(
                        crate::extensions::ext_validation_cache::FN_GET_VALIDATION_CACHE_DATA_EXT,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            merge_validation_caches_ext: if ext_validation_cache {
                unsafe {
                    match symbol(
                        crate::extensions::ext_validation_cache::FN_MERGE_VALIDATION_CACHES_EXT,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_descriptor_set_layout_support: if vk1_1 {
                unsafe {
                    match symbol(crate::vk1_1::FN_GET_DESCRIPTOR_SET_LAYOUT_SUPPORT) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_shader_info_amd: if amd_shader_info {
                unsafe {
                    match symbol(crate::extensions::amd_shader_info::FN_GET_SHADER_INFO_AMD) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            set_local_dimming_amd: if amd_display_native_hdr {
                unsafe {
                    match symbol(
                        crate::extensions::amd_display_native_hdr::FN_SET_LOCAL_DIMMING_AMD,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_calibrated_timestamps_ext: if ext_calibrated_timestamps {
                unsafe {
                    match symbol ( crate :: extensions :: ext_calibrated_timestamps :: FN_GET_CALIBRATED_TIMESTAMPS_EXT ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            set_debug_utils_object_name_ext: if ext_debug_utils {
                unsafe {
                    match symbol(
                        crate::extensions::ext_debug_utils::FN_SET_DEBUG_UTILS_OBJECT_NAME_EXT,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            set_debug_utils_object_tag_ext: if ext_debug_utils {
                unsafe {
                    match symbol(
                        crate::extensions::ext_debug_utils::FN_SET_DEBUG_UTILS_OBJECT_TAG_EXT,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            queue_begin_debug_utils_label_ext: if ext_debug_utils {
                unsafe {
                    match symbol(
                        crate::extensions::ext_debug_utils::FN_QUEUE_BEGIN_DEBUG_UTILS_LABEL_EXT,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            queue_end_debug_utils_label_ext: if ext_debug_utils {
                unsafe {
                    match symbol(
                        crate::extensions::ext_debug_utils::FN_QUEUE_END_DEBUG_UTILS_LABEL_EXT,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            queue_insert_debug_utils_label_ext: if ext_debug_utils {
                unsafe {
                    match symbol(
                        crate::extensions::ext_debug_utils::FN_QUEUE_INSERT_DEBUG_UTILS_LABEL_EXT,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            cmd_begin_debug_utils_label_ext: if ext_debug_utils {
                unsafe {
                    match symbol(
                        crate::extensions::ext_debug_utils::FN_CMD_BEGIN_DEBUG_UTILS_LABEL_EXT,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            cmd_end_debug_utils_label_ext: if ext_debug_utils {
                unsafe {
                    match symbol(
                        crate::extensions::ext_debug_utils::FN_CMD_END_DEBUG_UTILS_LABEL_EXT,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            cmd_insert_debug_utils_label_ext: if ext_debug_utils {
                unsafe {
                    match symbol(
                        crate::extensions::ext_debug_utils::FN_CMD_INSERT_DEBUG_UTILS_LABEL_EXT,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_memory_host_pointer_properties_ext: if ext_external_memory_host {
                unsafe {
                    match symbol ( crate :: extensions :: ext_external_memory_host :: FN_GET_MEMORY_HOST_POINTER_PROPERTIES_EXT ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            cmd_write_buffer_marker_amd: if amd_buffer_marker {
                unsafe {
                    match symbol(
                        crate::extensions::amd_buffer_marker::FN_CMD_WRITE_BUFFER_MARKER_AMD,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            create_render_pass2: if vk1_2 {
                unsafe {
                    match symbol(crate::vk1_2::FN_CREATE_RENDER_PASS2) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            cmd_begin_render_pass2: if vk1_2 {
                unsafe {
                    match symbol(crate::vk1_2::FN_CMD_BEGIN_RENDER_PASS2) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            cmd_next_subpass2: if vk1_2 {
                unsafe {
                    match symbol(crate::vk1_2::FN_CMD_NEXT_SUBPASS2) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            cmd_end_render_pass2: if vk1_2 {
                unsafe {
                    match symbol(crate::vk1_2::FN_CMD_END_RENDER_PASS2) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_semaphore_counter_value: if vk1_2 {
                unsafe {
                    match symbol(crate::vk1_2::FN_GET_SEMAPHORE_COUNTER_VALUE) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            wait_semaphores: if vk1_2 {
                unsafe {
                    match symbol(crate::vk1_2::FN_WAIT_SEMAPHORES) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            signal_semaphore: if vk1_2 {
                unsafe {
                    match symbol(crate::vk1_2::FN_SIGNAL_SEMAPHORE) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_android_hardware_buffer_properties_android:
                if android_external_memory_android_hardware_buffer {
                    unsafe {
                        match symbol ( crate :: extensions :: android_external_memory_android_hardware_buffer :: FN_GET_ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                    }
                } else {
                    None
                },
            get_memory_android_hardware_buffer_android:
                if android_external_memory_android_hardware_buffer {
                    unsafe {
                        match symbol ( crate :: extensions :: android_external_memory_android_hardware_buffer :: FN_GET_MEMORY_ANDROID_HARDWARE_BUFFER_ANDROID ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                    }
                } else {
                    None
                },
            cmd_draw_indirect_count: if vk1_2 {
                unsafe {
                    match symbol(crate::vk1_2::FN_CMD_DRAW_INDIRECT_COUNT) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            cmd_draw_indexed_indirect_count: if vk1_2 {
                unsafe {
                    match symbol(crate::vk1_2::FN_CMD_DRAW_INDEXED_INDIRECT_COUNT) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            cmd_set_checkpoint_nv: if nv_device_diagnostic_checkpoints {
                unsafe {
                    match symbol ( crate :: extensions :: nv_device_diagnostic_checkpoints :: FN_CMD_SET_CHECKPOINT_NV ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            get_queue_checkpoint_data_nv: if nv_device_diagnostic_checkpoints {
                unsafe {
                    match symbol ( crate :: extensions :: nv_device_diagnostic_checkpoints :: FN_GET_QUEUE_CHECKPOINT_DATA_NV ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            cmd_bind_transform_feedback_buffers_ext: if ext_transform_feedback {
                unsafe {
                    match symbol ( crate :: extensions :: ext_transform_feedback :: FN_CMD_BIND_TRANSFORM_FEEDBACK_BUFFERS_EXT ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            cmd_begin_transform_feedback_ext: if ext_transform_feedback {
                unsafe {
                    match symbol ( crate :: extensions :: ext_transform_feedback :: FN_CMD_BEGIN_TRANSFORM_FEEDBACK_EXT ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            cmd_end_transform_feedback_ext: if ext_transform_feedback {
                unsafe {
                    match symbol ( crate :: extensions :: ext_transform_feedback :: FN_CMD_END_TRANSFORM_FEEDBACK_EXT ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            cmd_begin_query_indexed_ext: if ext_transform_feedback {
                unsafe {
                    match symbol(
                        crate::extensions::ext_transform_feedback::FN_CMD_BEGIN_QUERY_INDEXED_EXT,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            cmd_end_query_indexed_ext: if ext_transform_feedback {
                unsafe {
                    match symbol(
                        crate::extensions::ext_transform_feedback::FN_CMD_END_QUERY_INDEXED_EXT,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            cmd_draw_indirect_byte_count_ext: if ext_transform_feedback {
                unsafe {
                    match symbol ( crate :: extensions :: ext_transform_feedback :: FN_CMD_DRAW_INDIRECT_BYTE_COUNT_EXT ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            cmd_set_exclusive_scissor_nv: if nv_scissor_exclusive {
                unsafe {
                    match symbol(
                        crate::extensions::nv_scissor_exclusive::FN_CMD_SET_EXCLUSIVE_SCISSOR_NV,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            cmd_bind_shading_rate_image_nv: if nv_shading_rate_image {
                unsafe {
                    match symbol(
                        crate::extensions::nv_shading_rate_image::FN_CMD_BIND_SHADING_RATE_IMAGE_NV,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            cmd_set_viewport_shading_rate_palette_nv: if nv_shading_rate_image {
                unsafe {
                    match symbol ( crate :: extensions :: nv_shading_rate_image :: FN_CMD_SET_VIEWPORT_SHADING_RATE_PALETTE_NV ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            cmd_set_coarse_sample_order_nv: if nv_shading_rate_image {
                unsafe {
                    match symbol(
                        crate::extensions::nv_shading_rate_image::FN_CMD_SET_COARSE_SAMPLE_ORDER_NV,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            cmd_draw_mesh_tasks_nv: if nv_mesh_shader {
                unsafe {
                    match symbol(crate::extensions::nv_mesh_shader::FN_CMD_DRAW_MESH_TASKS_NV) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            cmd_draw_mesh_tasks_indirect_nv: if nv_mesh_shader {
                unsafe {
                    match symbol(
                        crate::extensions::nv_mesh_shader::FN_CMD_DRAW_MESH_TASKS_INDIRECT_NV,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            cmd_draw_mesh_tasks_indirect_count_nv: if nv_mesh_shader {
                unsafe {
                    match symbol(
                        crate::extensions::nv_mesh_shader::FN_CMD_DRAW_MESH_TASKS_INDIRECT_COUNT_NV,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            compile_deferred_nv: if nv_ray_tracing {
                unsafe {
                    match symbol(crate::extensions::nv_ray_tracing::FN_COMPILE_DEFERRED_NV) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            create_acceleration_structure_nv: if nv_ray_tracing {
                unsafe {
                    match symbol(
                        crate::extensions::nv_ray_tracing::FN_CREATE_ACCELERATION_STRUCTURE_NV,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            destroy_acceleration_structure_khr: if khr_ray_tracing {
                unsafe {
                    match symbol(
                        crate::extensions::khr_ray_tracing::FN_DESTROY_ACCELERATION_STRUCTURE_KHR,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_acceleration_structure_memory_requirements_khr: if khr_ray_tracing {
                unsafe {
                    match symbol ( crate :: extensions :: khr_ray_tracing :: FN_GET_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            get_acceleration_structure_memory_requirements_nv: if nv_ray_tracing {
                unsafe {
                    match symbol ( crate :: extensions :: nv_ray_tracing :: FN_GET_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_NV ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            bind_acceleration_structure_memory_khr: if khr_ray_tracing {
                unsafe {
                    match symbol ( crate :: extensions :: khr_ray_tracing :: FN_BIND_ACCELERATION_STRUCTURE_MEMORY_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            cmd_copy_acceleration_structure_nv: if nv_ray_tracing {
                unsafe {
                    match symbol(
                        crate::extensions::nv_ray_tracing::FN_CMD_COPY_ACCELERATION_STRUCTURE_NV,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            cmd_copy_acceleration_structure_khr: if khr_ray_tracing {
                unsafe {
                    match symbol(
                        crate::extensions::khr_ray_tracing::FN_CMD_COPY_ACCELERATION_STRUCTURE_KHR,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            copy_acceleration_structure_khr: if khr_ray_tracing {
                unsafe {
                    match symbol(
                        crate::extensions::khr_ray_tracing::FN_COPY_ACCELERATION_STRUCTURE_KHR,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            cmd_copy_acceleration_structure_to_memory_khr: if khr_ray_tracing {
                unsafe {
                    match symbol ( crate :: extensions :: khr_ray_tracing :: FN_CMD_COPY_ACCELERATION_STRUCTURE_TO_MEMORY_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            copy_acceleration_structure_to_memory_khr: if khr_ray_tracing {
                unsafe {
                    match symbol ( crate :: extensions :: khr_ray_tracing :: FN_COPY_ACCELERATION_STRUCTURE_TO_MEMORY_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            cmd_copy_memory_to_acceleration_structure_khr: if khr_ray_tracing {
                unsafe {
                    match symbol ( crate :: extensions :: khr_ray_tracing :: FN_CMD_COPY_MEMORY_TO_ACCELERATION_STRUCTURE_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            copy_memory_to_acceleration_structure_khr: if khr_ray_tracing {
                unsafe {
                    match symbol ( crate :: extensions :: khr_ray_tracing :: FN_COPY_MEMORY_TO_ACCELERATION_STRUCTURE_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            cmd_write_acceleration_structures_properties_khr: if khr_ray_tracing {
                unsafe {
                    match symbol ( crate :: extensions :: khr_ray_tracing :: FN_CMD_WRITE_ACCELERATION_STRUCTURES_PROPERTIES_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            cmd_build_acceleration_structure_nv: if nv_ray_tracing {
                unsafe {
                    match symbol(
                        crate::extensions::nv_ray_tracing::FN_CMD_BUILD_ACCELERATION_STRUCTURE_NV,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            write_acceleration_structures_properties_khr: if khr_ray_tracing {
                unsafe {
                    match symbol ( crate :: extensions :: khr_ray_tracing :: FN_WRITE_ACCELERATION_STRUCTURES_PROPERTIES_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            cmd_trace_rays_khr: if khr_ray_tracing {
                unsafe {
                    match symbol(crate::extensions::khr_ray_tracing::FN_CMD_TRACE_RAYS_KHR) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            cmd_trace_rays_nv: if nv_ray_tracing {
                unsafe {
                    match symbol(crate::extensions::nv_ray_tracing::FN_CMD_TRACE_RAYS_NV) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_ray_tracing_shader_group_handles_khr: if khr_ray_tracing {
                unsafe {
                    match symbol ( crate :: extensions :: khr_ray_tracing :: FN_GET_RAY_TRACING_SHADER_GROUP_HANDLES_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            get_ray_tracing_capture_replay_shader_group_handles_khr: if khr_ray_tracing {
                unsafe {
                    match symbol ( crate :: extensions :: khr_ray_tracing :: FN_GET_RAY_TRACING_CAPTURE_REPLAY_SHADER_GROUP_HANDLES_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            get_acceleration_structure_handle_nv: if nv_ray_tracing {
                unsafe {
                    match symbol(
                        crate::extensions::nv_ray_tracing::FN_GET_ACCELERATION_STRUCTURE_HANDLE_NV,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            create_ray_tracing_pipelines_nv: if nv_ray_tracing {
                unsafe {
                    match symbol(
                        crate::extensions::nv_ray_tracing::FN_CREATE_RAY_TRACING_PIPELINES_NV,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            create_ray_tracing_pipelines_khr: if khr_ray_tracing {
                unsafe {
                    match symbol(
                        crate::extensions::khr_ray_tracing::FN_CREATE_RAY_TRACING_PIPELINES_KHR,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            cmd_trace_rays_indirect_khr: if khr_ray_tracing {
                unsafe {
                    match symbol(crate::extensions::khr_ray_tracing::FN_CMD_TRACE_RAYS_INDIRECT_KHR)
                    {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_device_acceleration_structure_compatibility_khr: if khr_ray_tracing {
                unsafe {
                    match symbol ( crate :: extensions :: khr_ray_tracing :: FN_GET_DEVICE_ACCELERATION_STRUCTURE_COMPATIBILITY_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            get_image_view_handle_nvx: if nvx_image_view_handle {
                unsafe {
                    match symbol(
                        crate::extensions::nvx_image_view_handle::FN_GET_IMAGE_VIEW_HANDLE_NVX,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_image_view_address_nvx: if nvx_image_view_handle {
                unsafe {
                    match symbol(
                        crate::extensions::nvx_image_view_handle::FN_GET_IMAGE_VIEW_ADDRESS_NVX,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_device_group_surface_present_modes2_ext: if (ext_full_screen_exclusive
                && khr_device_group)
                || (ext_full_screen_exclusive && vk1_1)
            {
                unsafe {
                    match symbol ( crate :: extensions :: ext_full_screen_exclusive :: FN_GET_DEVICE_GROUP_SURFACE_PRESENT_MODES2_EXT ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            acquire_full_screen_exclusive_mode_ext: if ext_full_screen_exclusive {
                unsafe {
                    match symbol ( crate :: extensions :: ext_full_screen_exclusive :: FN_ACQUIRE_FULL_SCREEN_EXCLUSIVE_MODE_EXT ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            release_full_screen_exclusive_mode_ext: if ext_full_screen_exclusive {
                unsafe {
                    match symbol ( crate :: extensions :: ext_full_screen_exclusive :: FN_RELEASE_FULL_SCREEN_EXCLUSIVE_MODE_EXT ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            acquire_profiling_lock_khr: if khr_performance_query {
                unsafe {
                    match symbol(
                        crate::extensions::khr_performance_query::FN_ACQUIRE_PROFILING_LOCK_KHR,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            release_profiling_lock_khr: if khr_performance_query {
                unsafe {
                    match symbol(
                        crate::extensions::khr_performance_query::FN_RELEASE_PROFILING_LOCK_KHR,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_image_drm_format_modifier_properties_ext: if ext_image_drm_format_modifier {
                unsafe {
                    match symbol ( crate :: extensions :: ext_image_drm_format_modifier :: FN_GET_IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            get_buffer_opaque_capture_address: if vk1_2 {
                unsafe {
                    match symbol(crate::vk1_2::FN_GET_BUFFER_OPAQUE_CAPTURE_ADDRESS) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_buffer_device_address: if vk1_2 {
                unsafe {
                    match symbol(crate::vk1_2::FN_GET_BUFFER_DEVICE_ADDRESS) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            initialize_performance_api_intel: if intel_performance_query {
                unsafe {
                    match symbol ( crate :: extensions :: intel_performance_query :: FN_INITIALIZE_PERFORMANCE_API_INTEL ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            uninitialize_performance_api_intel: if intel_performance_query {
                unsafe {
                    match symbol ( crate :: extensions :: intel_performance_query :: FN_UNINITIALIZE_PERFORMANCE_API_INTEL ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            cmd_set_performance_marker_intel: if intel_performance_query {
                unsafe {
                    match symbol ( crate :: extensions :: intel_performance_query :: FN_CMD_SET_PERFORMANCE_MARKER_INTEL ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            cmd_set_performance_stream_marker_intel: if intel_performance_query {
                unsafe {
                    match symbol ( crate :: extensions :: intel_performance_query :: FN_CMD_SET_PERFORMANCE_STREAM_MARKER_INTEL ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            cmd_set_performance_override_intel: if intel_performance_query {
                unsafe {
                    match symbol ( crate :: extensions :: intel_performance_query :: FN_CMD_SET_PERFORMANCE_OVERRIDE_INTEL ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            acquire_performance_configuration_intel: if intel_performance_query {
                unsafe {
                    match symbol ( crate :: extensions :: intel_performance_query :: FN_ACQUIRE_PERFORMANCE_CONFIGURATION_INTEL ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            release_performance_configuration_intel: if intel_performance_query {
                unsafe {
                    match symbol ( crate :: extensions :: intel_performance_query :: FN_RELEASE_PERFORMANCE_CONFIGURATION_INTEL ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            queue_set_performance_configuration_intel: if intel_performance_query {
                unsafe {
                    match symbol ( crate :: extensions :: intel_performance_query :: FN_QUEUE_SET_PERFORMANCE_CONFIGURATION_INTEL ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            get_performance_parameter_intel: if intel_performance_query {
                unsafe {
                    match symbol ( crate :: extensions :: intel_performance_query :: FN_GET_PERFORMANCE_PARAMETER_INTEL ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            get_device_memory_opaque_capture_address: if vk1_2 {
                unsafe {
                    match symbol(crate::vk1_2::FN_GET_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_pipeline_executable_properties_khr: if khr_pipeline_executable_properties {
                unsafe {
                    match symbol ( crate :: extensions :: khr_pipeline_executable_properties :: FN_GET_PIPELINE_EXECUTABLE_PROPERTIES_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            get_pipeline_executable_statistics_khr: if khr_pipeline_executable_properties {
                unsafe {
                    match symbol ( crate :: extensions :: khr_pipeline_executable_properties :: FN_GET_PIPELINE_EXECUTABLE_STATISTICS_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            get_pipeline_executable_internal_representations_khr:
                if khr_pipeline_executable_properties {
                    unsafe {
                        match symbol ( crate :: extensions :: khr_pipeline_executable_properties :: FN_GET_PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATIONS_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                    }
                } else {
                    None
                },
            cmd_set_line_stipple_ext: if ext_line_rasterization {
                unsafe {
                    match symbol(
                        crate::extensions::ext_line_rasterization::FN_CMD_SET_LINE_STIPPLE_EXT,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            create_acceleration_structure_khr: if khr_ray_tracing {
                unsafe {
                    match symbol(
                        crate::extensions::khr_ray_tracing::FN_CREATE_ACCELERATION_STRUCTURE_KHR,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            cmd_build_acceleration_structure_khr: if khr_ray_tracing {
                unsafe {
                    match symbol(
                        crate::extensions::khr_ray_tracing::FN_CMD_BUILD_ACCELERATION_STRUCTURE_KHR,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            cmd_build_acceleration_structure_indirect_khr: if khr_ray_tracing {
                unsafe {
                    match symbol ( crate :: extensions :: khr_ray_tracing :: FN_CMD_BUILD_ACCELERATION_STRUCTURE_INDIRECT_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            build_acceleration_structure_khr: if khr_ray_tracing {
                unsafe {
                    match symbol(
                        crate::extensions::khr_ray_tracing::FN_BUILD_ACCELERATION_STRUCTURE_KHR,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_acceleration_structure_device_address_khr: if khr_ray_tracing {
                unsafe {
                    match symbol ( crate :: extensions :: khr_ray_tracing :: FN_GET_ACCELERATION_STRUCTURE_DEVICE_ADDRESS_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            create_deferred_operation_khr: if khr_deferred_host_operations {
                unsafe {
                    match symbol ( crate :: extensions :: khr_deferred_host_operations :: FN_CREATE_DEFERRED_OPERATION_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            destroy_deferred_operation_khr: if khr_deferred_host_operations {
                unsafe {
                    match symbol ( crate :: extensions :: khr_deferred_host_operations :: FN_DESTROY_DEFERRED_OPERATION_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            get_deferred_operation_max_concurrency_khr: if khr_deferred_host_operations {
                unsafe {
                    match symbol ( crate :: extensions :: khr_deferred_host_operations :: FN_GET_DEFERRED_OPERATION_MAX_CONCURRENCY_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            get_deferred_operation_result_khr: if khr_deferred_host_operations {
                unsafe {
                    match symbol ( crate :: extensions :: khr_deferred_host_operations :: FN_GET_DEFERRED_OPERATION_RESULT_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            deferred_operation_join_khr: if khr_deferred_host_operations {
                unsafe {
                    match symbol ( crate :: extensions :: khr_deferred_host_operations :: FN_DEFERRED_OPERATION_JOIN_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            cmd_set_cull_mode_ext: if ext_extended_dynamic_state {
                unsafe {
                    match symbol(
                        crate::extensions::ext_extended_dynamic_state::FN_CMD_SET_CULL_MODE_EXT,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            cmd_set_front_face_ext: if ext_extended_dynamic_state {
                unsafe {
                    match symbol(
                        crate::extensions::ext_extended_dynamic_state::FN_CMD_SET_FRONT_FACE_EXT,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            cmd_set_primitive_topology_ext: if ext_extended_dynamic_state {
                unsafe {
                    match symbol ( crate :: extensions :: ext_extended_dynamic_state :: FN_CMD_SET_PRIMITIVE_TOPOLOGY_EXT ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            cmd_set_viewport_with_count_ext: if ext_extended_dynamic_state {
                unsafe {
                    match symbol ( crate :: extensions :: ext_extended_dynamic_state :: FN_CMD_SET_VIEWPORT_WITH_COUNT_EXT ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            cmd_set_scissor_with_count_ext: if ext_extended_dynamic_state {
                unsafe {
                    match symbol ( crate :: extensions :: ext_extended_dynamic_state :: FN_CMD_SET_SCISSOR_WITH_COUNT_EXT ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            cmd_bind_vertex_buffers2_ext: if ext_extended_dynamic_state {
                unsafe {
                    match symbol ( crate :: extensions :: ext_extended_dynamic_state :: FN_CMD_BIND_VERTEX_BUFFERS2_EXT ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            cmd_set_depth_test_enable_ext: if ext_extended_dynamic_state {
                unsafe {
                    match symbol ( crate :: extensions :: ext_extended_dynamic_state :: FN_CMD_SET_DEPTH_TEST_ENABLE_EXT ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            cmd_set_depth_write_enable_ext: if ext_extended_dynamic_state {
                unsafe {
                    match symbol ( crate :: extensions :: ext_extended_dynamic_state :: FN_CMD_SET_DEPTH_WRITE_ENABLE_EXT ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            cmd_set_depth_compare_op_ext: if ext_extended_dynamic_state {
                unsafe {
                    match symbol ( crate :: extensions :: ext_extended_dynamic_state :: FN_CMD_SET_DEPTH_COMPARE_OP_EXT ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            cmd_set_depth_bounds_test_enable_ext: if ext_extended_dynamic_state {
                unsafe {
                    match symbol ( crate :: extensions :: ext_extended_dynamic_state :: FN_CMD_SET_DEPTH_BOUNDS_TEST_ENABLE_EXT ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            cmd_set_stencil_test_enable_ext: if ext_extended_dynamic_state {
                unsafe {
                    match symbol ( crate :: extensions :: ext_extended_dynamic_state :: FN_CMD_SET_STENCIL_TEST_ENABLE_EXT ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            cmd_set_stencil_op_ext: if ext_extended_dynamic_state {
                unsafe {
                    match symbol(
                        crate::extensions::ext_extended_dynamic_state::FN_CMD_SET_STENCIL_OP_EXT,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            create_private_data_slot_ext: if ext_private_data {
                unsafe {
                    match symbol(
                        crate::extensions::ext_private_data::FN_CREATE_PRIVATE_DATA_SLOT_EXT,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            destroy_private_data_slot_ext: if ext_private_data {
                unsafe {
                    match symbol(
                        crate::extensions::ext_private_data::FN_DESTROY_PRIVATE_DATA_SLOT_EXT,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            set_private_data_ext: if ext_private_data {
                unsafe {
                    match symbol(crate::extensions::ext_private_data::FN_SET_PRIVATE_DATA_EXT) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_private_data_ext: if ext_private_data {
                unsafe {
                    match symbol(crate::extensions::ext_private_data::FN_GET_PRIVATE_DATA_EXT) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            reset_query_pool_ext: if ext_host_query_reset {
                unsafe {
                    match symbol(crate::extensions::ext_host_query_reset::FN_RESET_QUERY_POOL_EXT) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            trim_command_pool_khr: if khr_maintenance1 {
                unsafe {
                    match symbol(crate::extensions::khr_maintenance1::FN_TRIM_COMMAND_POOL_KHR) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_device_group_peer_memory_features_khr: if khr_device_group {
                unsafe {
                    match symbol ( crate :: extensions :: khr_device_group :: FN_GET_DEVICE_GROUP_PEER_MEMORY_FEATURES_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            bind_buffer_memory2_khr: if khr_bind_memory2 {
                unsafe {
                    match symbol(crate::extensions::khr_bind_memory2::FN_BIND_BUFFER_MEMORY2_KHR) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            bind_image_memory2_khr: if khr_bind_memory2 {
                unsafe {
                    match symbol(crate::extensions::khr_bind_memory2::FN_BIND_IMAGE_MEMORY2_KHR) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            cmd_set_device_mask_khr: if khr_device_group {
                unsafe {
                    match symbol(crate::extensions::khr_device_group::FN_CMD_SET_DEVICE_MASK_KHR) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            cmd_dispatch_base_khr: if khr_device_group {
                unsafe {
                    match symbol(crate::extensions::khr_device_group::FN_CMD_DISPATCH_BASE_KHR) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            create_descriptor_update_template_khr: if khr_descriptor_update_template {
                unsafe {
                    match symbol ( crate :: extensions :: khr_descriptor_update_template :: FN_CREATE_DESCRIPTOR_UPDATE_TEMPLATE_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            destroy_descriptor_update_template_khr: if khr_descriptor_update_template {
                unsafe {
                    match symbol ( crate :: extensions :: khr_descriptor_update_template :: FN_DESTROY_DESCRIPTOR_UPDATE_TEMPLATE_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            update_descriptor_set_with_template_khr: if khr_descriptor_update_template {
                unsafe {
                    match symbol ( crate :: extensions :: khr_descriptor_update_template :: FN_UPDATE_DESCRIPTOR_SET_WITH_TEMPLATE_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            get_buffer_memory_requirements2_khr: if khr_get_memory_requirements2 {
                unsafe {
                    match symbol ( crate :: extensions :: khr_get_memory_requirements2 :: FN_GET_BUFFER_MEMORY_REQUIREMENTS2_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            get_image_memory_requirements2_khr: if khr_get_memory_requirements2 {
                unsafe {
                    match symbol ( crate :: extensions :: khr_get_memory_requirements2 :: FN_GET_IMAGE_MEMORY_REQUIREMENTS2_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            get_image_sparse_memory_requirements2_khr: if khr_get_memory_requirements2 {
                unsafe {
                    match symbol ( crate :: extensions :: khr_get_memory_requirements2 :: FN_GET_IMAGE_SPARSE_MEMORY_REQUIREMENTS2_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            create_sampler_ycbcr_conversion_khr: if khr_sampler_ycbcr_conversion {
                unsafe {
                    match symbol ( crate :: extensions :: khr_sampler_ycbcr_conversion :: FN_CREATE_SAMPLER_YCBCR_CONVERSION_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            destroy_sampler_ycbcr_conversion_khr: if khr_sampler_ycbcr_conversion {
                unsafe {
                    match symbol ( crate :: extensions :: khr_sampler_ycbcr_conversion :: FN_DESTROY_SAMPLER_YCBCR_CONVERSION_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            get_descriptor_set_layout_support_khr: if khr_maintenance3 {
                unsafe {
                    match symbol ( crate :: extensions :: khr_maintenance3 :: FN_GET_DESCRIPTOR_SET_LAYOUT_SUPPORT_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            create_render_pass2_khr: if khr_create_renderpass2 {
                unsafe {
                    match symbol(
                        crate::extensions::khr_create_renderpass2::FN_CREATE_RENDER_PASS2_KHR,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            cmd_begin_render_pass2_khr: if khr_create_renderpass2 {
                unsafe {
                    match symbol(
                        crate::extensions::khr_create_renderpass2::FN_CMD_BEGIN_RENDER_PASS2_KHR,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            cmd_next_subpass2_khr: if khr_create_renderpass2 {
                unsafe {
                    match symbol(
                        crate::extensions::khr_create_renderpass2::FN_CMD_NEXT_SUBPASS2_KHR,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            cmd_end_render_pass2_khr: if khr_create_renderpass2 {
                unsafe {
                    match symbol(
                        crate::extensions::khr_create_renderpass2::FN_CMD_END_RENDER_PASS2_KHR,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            get_semaphore_counter_value_khr: if khr_timeline_semaphore {
                unsafe {
                    match symbol ( crate :: extensions :: khr_timeline_semaphore :: FN_GET_SEMAPHORE_COUNTER_VALUE_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            wait_semaphores_khr: if khr_timeline_semaphore {
                unsafe {
                    match symbol(crate::extensions::khr_timeline_semaphore::FN_WAIT_SEMAPHORES_KHR)
                    {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            signal_semaphore_khr: if khr_timeline_semaphore {
                unsafe {
                    match symbol(crate::extensions::khr_timeline_semaphore::FN_SIGNAL_SEMAPHORE_KHR)
                    {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            cmd_draw_indirect_count_khr: if khr_draw_indirect_count {
                unsafe {
                    match symbol(
                        crate::extensions::khr_draw_indirect_count::FN_CMD_DRAW_INDIRECT_COUNT_KHR,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            cmd_draw_indirect_count_amd: if amd_draw_indirect_count {
                unsafe {
                    match symbol(
                        crate::extensions::amd_draw_indirect_count::FN_CMD_DRAW_INDIRECT_COUNT_AMD,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            cmd_draw_indexed_indirect_count_khr: if khr_draw_indirect_count {
                unsafe {
                    match symbol ( crate :: extensions :: khr_draw_indirect_count :: FN_CMD_DRAW_INDEXED_INDIRECT_COUNT_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            cmd_draw_indexed_indirect_count_amd: if amd_draw_indirect_count {
                unsafe {
                    match symbol ( crate :: extensions :: amd_draw_indirect_count :: FN_CMD_DRAW_INDEXED_INDIRECT_COUNT_AMD ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            destroy_acceleration_structure_nv: if nv_ray_tracing {
                unsafe {
                    match symbol(
                        crate::extensions::nv_ray_tracing::FN_DESTROY_ACCELERATION_STRUCTURE_NV,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            bind_acceleration_structure_memory_nv: if nv_ray_tracing {
                unsafe {
                    match symbol(
                        crate::extensions::nv_ray_tracing::FN_BIND_ACCELERATION_STRUCTURE_MEMORY_NV,
                    ) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            } else {
                None
            },
            cmd_write_acceleration_structures_properties_nv: if nv_ray_tracing {
                unsafe {
                    match symbol ( crate :: extensions :: nv_ray_tracing :: FN_CMD_WRITE_ACCELERATION_STRUCTURES_PROPERTIES_NV ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            get_ray_tracing_shader_group_handles_nv: if nv_ray_tracing {
                unsafe {
                    match symbol ( crate :: extensions :: nv_ray_tracing :: FN_GET_RAY_TRACING_SHADER_GROUP_HANDLES_NV ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            get_buffer_opaque_capture_address_khr: if khr_buffer_device_address {
                unsafe {
                    match symbol ( crate :: extensions :: khr_buffer_device_address :: FN_GET_BUFFER_OPAQUE_CAPTURE_ADDRESS_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            get_buffer_device_address_khr: if khr_buffer_device_address {
                unsafe {
                    match symbol ( crate :: extensions :: khr_buffer_device_address :: FN_GET_BUFFER_DEVICE_ADDRESS_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            get_buffer_device_address_ext: if ext_buffer_device_address {
                unsafe {
                    match symbol ( crate :: extensions :: ext_buffer_device_address :: FN_GET_BUFFER_DEVICE_ADDRESS_EXT ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
            get_device_memory_opaque_capture_address_khr: if khr_buffer_device_address {
                unsafe {
                    match symbol ( crate :: extensions :: khr_buffer_device_address :: FN_GET_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_KHR ) { Some ( ptr ) => Some ( std :: mem :: transmute ( ptr ) ) , None => return Err ( crate :: LoaderError :: SymbolNotAvailable ) , }
                }
            } else {
                None
            },
        })
    }
}
impl std::fmt::Debug for DeviceLoader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.handle, f)
    }
}
#[doc = r" Provides Vulkan feature items"]
pub mod vk1_0;
#[doc = r" Provides Vulkan feature items"]
pub mod vk1_1;
#[doc = r" Provides Vulkan feature items"]
pub mod vk1_2;
