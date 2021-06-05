#[doc = r" Provides Vulkan extension items"]
pub mod extensions;
#[doc = r" Provides external library items"]
pub mod external;
#[doc = r" Re-exports **every** Vulkan item"]
pub mod vk;
#[doc = r" A list of requirements enabled in the entry loader."]
#[derive(Debug)]
pub struct EntryEnabled {
    pub instance_version: u32,
    pub vk1_1: bool,
    pub vk1_2: bool,
}
impl EntryEnabled {
    pub unsafe fn new<T>(loader: &mut T, mut symbol: impl FnMut(&mut T, *const std::os::raw::c_char) -> Option<crate::vk1_0::PFN_vkVoidFunction>) -> Result<EntryEnabled, crate::LoaderError> {
        let mut version = crate::vk1_0::make_api_version(0, 1, 2, 0);
        if let Some(function) = symbol(loader, crate::vk1_1::FN_ENUMERATE_INSTANCE_VERSION) {
            let function: crate::vk1_1::PFN_vkEnumerateInstanceVersion = std::mem::transmute(function);
            let result = function(&mut version);
            if result.0 < 0 {
                return Err(crate::LoaderError::VulkanError(result));
            }
        }
        Ok(EntryEnabled { instance_version: version, vk1_1: version >= crate::vk1_0::make_api_version(0, 1, 1, 0), vk1_2: version >= crate::vk1_0::make_api_version(0, 1, 2, 0) })
    }
}
#[doc = r" Loader for entry commands."]
#[doc = r""]
#[doc = r" To create a new loader, call [`EntryLoader::new`]."]
pub struct EntryLoader<T> {
    arc: std::sync::Arc<()>,
    pub loader: T,
    enabled: EntryEnabled,
    pub get_instance_proc_addr: crate::vk1_0::PFN_vkGetInstanceProcAddr,
    pub create_instance: Option<vk1_0::PFN_vkCreateInstance>,
    pub enumerate_instance_version: Option<vk1_1::PFN_vkEnumerateInstanceVersion>,
    pub enumerate_instance_layer_properties: Option<vk1_0::PFN_vkEnumerateInstanceLayerProperties>,
    pub enumerate_instance_extension_properties: Option<vk1_0::PFN_vkEnumerateInstanceExtensionProperties>,
}
impl<T> EntryLoader<T> {
    #[allow(unused_parens)]
    pub unsafe fn custom(mut loader: T, mut symbol: impl FnMut(&mut T, *const std::os::raw::c_char) -> Option<crate::vk1_0::PFN_vkVoidFunction>, entry_enabled: EntryEnabled) -> Result<EntryLoader<T>, crate::LoaderError> {
        let mut symbol = |name| symbol(&mut loader, name);
        let get_instance_proc_addr = symbol(crate::vk1_0::FN_GET_INSTANCE_PROC_ADDR).ok_or(crate::LoaderError::SymbolNotAvailable)?;
        Ok(EntryLoader { arc: std::sync::Arc::new(()), get_instance_proc_addr: std::mem::transmute(get_instance_proc_addr), create_instance: std::mem::transmute(symbol(crate::vk1_0::FN_CREATE_INSTANCE)), enumerate_instance_version: if entry_enabled.vk1_1 { std::mem::transmute(symbol(crate::vk1_1::FN_ENUMERATE_INSTANCE_VERSION)) } else { None }, enumerate_instance_layer_properties: std::mem::transmute(symbol(crate::vk1_0::FN_ENUMERATE_INSTANCE_LAYER_PROPERTIES)), enumerate_instance_extension_properties: std::mem::transmute(symbol(crate::vk1_0::FN_ENUMERATE_INSTANCE_EXTENSION_PROPERTIES)), loader, enabled: entry_enabled })
    }
    pub fn enabled(&self) -> &EntryEnabled {
        &self.enabled
    }
    pub fn instance_version(&self) -> u32 {
        self.enabled.instance_version
    }
}
impl<T> Drop for EntryLoader<T> {
    fn drop(&mut self) {
        if std::sync::Arc::weak_count(&self.arc) != 0 {
            panic!("Attempting to drop a entry loader with active references to it");
        }
    }
}
#[doc = r" A list of requirements enabled in the instance loader."]
#[derive(Debug)]
pub struct InstanceEnabled {
    pub vk1_1: bool,
    pub vk1_2: bool,
    pub khr_android_surface: bool,
    pub khr_display: bool,
    pub khr_surface: bool,
    pub nn_vi_surface: bool,
    pub khr_wayland_surface: bool,
    pub khr_win32_surface: bool,
    pub khr_xlib_surface: bool,
    pub khr_xcb_surface: bool,
    pub ext_directfb_surface: bool,
    pub fuchsia_imagepipe_surface: bool,
    pub ggp_stream_descriptor_surface: bool,
    pub qnx_screen_surface: bool,
    pub ext_debug_report: bool,
    pub nv_external_memory_capabilities: bool,
    pub ext_direct_mode_display: bool,
    pub ext_acquire_xlib_display: bool,
    pub nv_acquire_winrt_display: bool,
    pub ext_display_surface_counter: bool,
    pub khr_swapchain: bool,
    pub khr_device_group: bool,
    pub mvk_ios_surface: bool,
    pub mvk_macos_surface: bool,
    pub ext_metal_surface: bool,
    pub ext_sample_locations: bool,
    pub khr_get_surface_capabilities2: bool,
    pub khr_get_display_properties2: bool,
    pub ext_calibrated_timestamps: bool,
    pub ext_debug_utils: bool,
    pub nv_cooperative_matrix: bool,
    pub ext_full_screen_exclusive: bool,
    pub khr_performance_query: bool,
    pub ext_headless_surface: bool,
    pub nv_coverage_reduction_mode: bool,
    pub ext_tooling_info: bool,
    pub khr_fragment_shading_rate: bool,
    pub khr_video_queue: bool,
    pub khr_get_physical_device_properties2: bool,
    pub khr_external_memory_capabilities: bool,
    pub khr_external_semaphore_capabilities: bool,
    pub khr_external_fence_capabilities: bool,
    pub khr_device_group_creation: bool,
}
impl InstanceEnabled {
    #[allow(unused_variables)]
    pub unsafe fn new(instance_version: u32, enabled_extensions: &[&std::ffi::CStr], available_device_extensions: &[&std::ffi::CStr]) -> Result<InstanceEnabled, crate::LoaderError> {
        let version = instance_version;
        let enabled_extension = |extension| enabled_extensions.contains(&std::ffi::CStr::from_ptr(extension));
        let available_device_extension = |extension| available_device_extensions.contains(&std::ffi::CStr::from_ptr(extension));
        Ok(InstanceEnabled {
            vk1_1: {
                #[cfg(all())]
                {
                    version >= crate::vk1_0::make_api_version(0, 1, 1, 0)
                }
                #[cfg(any())]
                {
                    false
                }
            },
            vk1_2: {
                #[cfg(all())]
                {
                    version >= crate::vk1_0::make_api_version(0, 1, 2, 0)
                }
                #[cfg(any())]
                {
                    false
                }
            },
            khr_android_surface: {
                #[cfg(feature = "khr_android_surface")]
                {
                    enabled_extension(crate::extensions::khr_android_surface::KHR_ANDROID_SURFACE_EXTENSION_NAME)
                }
                #[cfg(not(feature = "khr_android_surface"))]
                {
                    false
                }
            },
            khr_display: {
                #[cfg(feature = "khr_display")]
                {
                    enabled_extension(crate::extensions::khr_display::KHR_DISPLAY_EXTENSION_NAME)
                }
                #[cfg(not(feature = "khr_display"))]
                {
                    false
                }
            },
            khr_surface: {
                #[cfg(feature = "khr_surface")]
                {
                    enabled_extension(crate::extensions::khr_surface::KHR_SURFACE_EXTENSION_NAME)
                }
                #[cfg(not(feature = "khr_surface"))]
                {
                    false
                }
            },
            nn_vi_surface: {
                #[cfg(feature = "nn_vi_surface")]
                {
                    enabled_extension(crate::extensions::nn_vi_surface::NN_VI_SURFACE_EXTENSION_NAME)
                }
                #[cfg(not(feature = "nn_vi_surface"))]
                {
                    false
                }
            },
            khr_wayland_surface: {
                #[cfg(feature = "khr_wayland_surface")]
                {
                    enabled_extension(crate::extensions::khr_wayland_surface::KHR_WAYLAND_SURFACE_EXTENSION_NAME)
                }
                #[cfg(not(feature = "khr_wayland_surface"))]
                {
                    false
                }
            },
            khr_win32_surface: {
                #[cfg(feature = "khr_win32_surface")]
                {
                    enabled_extension(crate::extensions::khr_win32_surface::KHR_WIN32_SURFACE_EXTENSION_NAME)
                }
                #[cfg(not(feature = "khr_win32_surface"))]
                {
                    false
                }
            },
            khr_xlib_surface: {
                #[cfg(feature = "khr_xlib_surface")]
                {
                    enabled_extension(crate::extensions::khr_xlib_surface::KHR_XLIB_SURFACE_EXTENSION_NAME)
                }
                #[cfg(not(feature = "khr_xlib_surface"))]
                {
                    false
                }
            },
            khr_xcb_surface: {
                #[cfg(feature = "khr_xcb_surface")]
                {
                    enabled_extension(crate::extensions::khr_xcb_surface::KHR_XCB_SURFACE_EXTENSION_NAME)
                }
                #[cfg(not(feature = "khr_xcb_surface"))]
                {
                    false
                }
            },
            ext_directfb_surface: {
                #[cfg(feature = "ext_directfb_surface")]
                {
                    enabled_extension(crate::extensions::ext_directfb_surface::EXT_DIRECTFB_SURFACE_EXTENSION_NAME)
                }
                #[cfg(not(feature = "ext_directfb_surface"))]
                {
                    false
                }
            },
            fuchsia_imagepipe_surface: {
                #[cfg(feature = "fuchsia_imagepipe_surface")]
                {
                    enabled_extension(crate::extensions::fuchsia_imagepipe_surface::FUCHSIA_IMAGEPIPE_SURFACE_EXTENSION_NAME)
                }
                #[cfg(not(feature = "fuchsia_imagepipe_surface"))]
                {
                    false
                }
            },
            ggp_stream_descriptor_surface: {
                #[cfg(feature = "ggp_stream_descriptor_surface")]
                {
                    enabled_extension(crate::extensions::ggp_stream_descriptor_surface::GGP_STREAM_DESCRIPTOR_SURFACE_EXTENSION_NAME)
                }
                #[cfg(not(feature = "ggp_stream_descriptor_surface"))]
                {
                    false
                }
            },
            qnx_screen_surface: {
                #[cfg(feature = "qnx_screen_surface")]
                {
                    enabled_extension(crate::extensions::qnx_screen_surface::QNX_SCREEN_SURFACE_EXTENSION_NAME)
                }
                #[cfg(not(feature = "qnx_screen_surface"))]
                {
                    false
                }
            },
            ext_debug_report: {
                #[cfg(feature = "ext_debug_report")]
                {
                    enabled_extension(crate::extensions::ext_debug_report::EXT_DEBUG_REPORT_EXTENSION_NAME)
                }
                #[cfg(not(feature = "ext_debug_report"))]
                {
                    false
                }
            },
            nv_external_memory_capabilities: {
                #[cfg(feature = "nv_external_memory_capabilities")]
                {
                    enabled_extension(crate::extensions::nv_external_memory_capabilities::NV_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME)
                }
                #[cfg(not(feature = "nv_external_memory_capabilities"))]
                {
                    false
                }
            },
            ext_direct_mode_display: {
                #[cfg(feature = "ext_direct_mode_display")]
                {
                    enabled_extension(crate::extensions::ext_direct_mode_display::EXT_DIRECT_MODE_DISPLAY_EXTENSION_NAME)
                }
                #[cfg(not(feature = "ext_direct_mode_display"))]
                {
                    false
                }
            },
            ext_acquire_xlib_display: {
                #[cfg(feature = "ext_acquire_xlib_display")]
                {
                    enabled_extension(crate::extensions::ext_acquire_xlib_display::EXT_ACQUIRE_XLIB_DISPLAY_EXTENSION_NAME)
                }
                #[cfg(not(feature = "ext_acquire_xlib_display"))]
                {
                    false
                }
            },
            nv_acquire_winrt_display: {
                #[cfg(feature = "nv_acquire_winrt_display")]
                {
                    available_device_extension(crate::extensions::nv_acquire_winrt_display::NV_ACQUIRE_WINRT_DISPLAY_EXTENSION_NAME)
                }
                #[cfg(not(feature = "nv_acquire_winrt_display"))]
                {
                    false
                }
            },
            ext_display_surface_counter: {
                #[cfg(feature = "ext_display_surface_counter")]
                {
                    enabled_extension(crate::extensions::ext_display_surface_counter::EXT_DISPLAY_SURFACE_COUNTER_EXTENSION_NAME)
                }
                #[cfg(not(feature = "ext_display_surface_counter"))]
                {
                    false
                }
            },
            khr_swapchain: {
                #[cfg(feature = "khr_swapchain")]
                {
                    available_device_extension(crate::extensions::khr_swapchain::KHR_SWAPCHAIN_EXTENSION_NAME)
                }
                #[cfg(not(feature = "khr_swapchain"))]
                {
                    false
                }
            },
            khr_device_group: {
                #[cfg(feature = "khr_device_group")]
                {
                    available_device_extension(crate::extensions::khr_device_group::KHR_DEVICE_GROUP_EXTENSION_NAME)
                }
                #[cfg(not(feature = "khr_device_group"))]
                {
                    false
                }
            },
            mvk_ios_surface: {
                #[cfg(feature = "mvk_ios_surface")]
                {
                    enabled_extension(crate::extensions::mvk_ios_surface::MVK_IOS_SURFACE_EXTENSION_NAME)
                }
                #[cfg(not(feature = "mvk_ios_surface"))]
                {
                    false
                }
            },
            mvk_macos_surface: {
                #[cfg(feature = "mvk_macos_surface")]
                {
                    enabled_extension(crate::extensions::mvk_macos_surface::MVK_MACOS_SURFACE_EXTENSION_NAME)
                }
                #[cfg(not(feature = "mvk_macos_surface"))]
                {
                    false
                }
            },
            ext_metal_surface: {
                #[cfg(feature = "ext_metal_surface")]
                {
                    enabled_extension(crate::extensions::ext_metal_surface::EXT_METAL_SURFACE_EXTENSION_NAME)
                }
                #[cfg(not(feature = "ext_metal_surface"))]
                {
                    false
                }
            },
            ext_sample_locations: {
                #[cfg(feature = "ext_sample_locations")]
                {
                    available_device_extension(crate::extensions::ext_sample_locations::EXT_SAMPLE_LOCATIONS_EXTENSION_NAME)
                }
                #[cfg(not(feature = "ext_sample_locations"))]
                {
                    false
                }
            },
            khr_get_surface_capabilities2: {
                #[cfg(feature = "khr_get_surface_capabilities2")]
                {
                    enabled_extension(crate::extensions::khr_get_surface_capabilities2::KHR_GET_SURFACE_CAPABILITIES_2_EXTENSION_NAME)
                }
                #[cfg(not(feature = "khr_get_surface_capabilities2"))]
                {
                    false
                }
            },
            khr_get_display_properties2: {
                #[cfg(feature = "khr_get_display_properties2")]
                {
                    enabled_extension(crate::extensions::khr_get_display_properties2::KHR_GET_DISPLAY_PROPERTIES_2_EXTENSION_NAME)
                }
                #[cfg(not(feature = "khr_get_display_properties2"))]
                {
                    false
                }
            },
            ext_calibrated_timestamps: {
                #[cfg(feature = "ext_calibrated_timestamps")]
                {
                    available_device_extension(crate::extensions::ext_calibrated_timestamps::EXT_CALIBRATED_TIMESTAMPS_EXTENSION_NAME)
                }
                #[cfg(not(feature = "ext_calibrated_timestamps"))]
                {
                    false
                }
            },
            ext_debug_utils: {
                #[cfg(feature = "ext_debug_utils")]
                {
                    enabled_extension(crate::extensions::ext_debug_utils::EXT_DEBUG_UTILS_EXTENSION_NAME)
                }
                #[cfg(not(feature = "ext_debug_utils"))]
                {
                    false
                }
            },
            nv_cooperative_matrix: {
                #[cfg(feature = "nv_cooperative_matrix")]
                {
                    available_device_extension(crate::extensions::nv_cooperative_matrix::NV_COOPERATIVE_MATRIX_EXTENSION_NAME)
                }
                #[cfg(not(feature = "nv_cooperative_matrix"))]
                {
                    false
                }
            },
            ext_full_screen_exclusive: {
                #[cfg(feature = "ext_full_screen_exclusive")]
                {
                    available_device_extension(crate::extensions::ext_full_screen_exclusive::EXT_FULL_SCREEN_EXCLUSIVE_EXTENSION_NAME)
                }
                #[cfg(not(feature = "ext_full_screen_exclusive"))]
                {
                    false
                }
            },
            khr_performance_query: {
                #[cfg(feature = "khr_performance_query")]
                {
                    available_device_extension(crate::extensions::khr_performance_query::KHR_PERFORMANCE_QUERY_EXTENSION_NAME)
                }
                #[cfg(not(feature = "khr_performance_query"))]
                {
                    false
                }
            },
            ext_headless_surface: {
                #[cfg(feature = "ext_headless_surface")]
                {
                    enabled_extension(crate::extensions::ext_headless_surface::EXT_HEADLESS_SURFACE_EXTENSION_NAME)
                }
                #[cfg(not(feature = "ext_headless_surface"))]
                {
                    false
                }
            },
            nv_coverage_reduction_mode: {
                #[cfg(feature = "nv_coverage_reduction_mode")]
                {
                    available_device_extension(crate::extensions::nv_coverage_reduction_mode::NV_COVERAGE_REDUCTION_MODE_EXTENSION_NAME)
                }
                #[cfg(not(feature = "nv_coverage_reduction_mode"))]
                {
                    false
                }
            },
            ext_tooling_info: {
                #[cfg(feature = "ext_tooling_info")]
                {
                    available_device_extension(crate::extensions::ext_tooling_info::EXT_TOOLING_INFO_EXTENSION_NAME)
                }
                #[cfg(not(feature = "ext_tooling_info"))]
                {
                    false
                }
            },
            khr_fragment_shading_rate: {
                #[cfg(feature = "khr_fragment_shading_rate")]
                {
                    available_device_extension(crate::extensions::khr_fragment_shading_rate::KHR_FRAGMENT_SHADING_RATE_EXTENSION_NAME)
                }
                #[cfg(not(feature = "khr_fragment_shading_rate"))]
                {
                    false
                }
            },
            khr_video_queue: {
                #[cfg(feature = "khr_video_queue")]
                {
                    available_device_extension(crate::extensions::khr_video_queue::KHR_VIDEO_QUEUE_EXTENSION_NAME)
                }
                #[cfg(not(feature = "khr_video_queue"))]
                {
                    false
                }
            },
            khr_get_physical_device_properties2: {
                #[cfg(feature = "khr_get_physical_device_properties2")]
                {
                    enabled_extension(crate::extensions::khr_get_physical_device_properties2::KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_EXTENSION_NAME)
                }
                #[cfg(not(feature = "khr_get_physical_device_properties2"))]
                {
                    false
                }
            },
            khr_external_memory_capabilities: {
                #[cfg(feature = "khr_external_memory_capabilities")]
                {
                    enabled_extension(crate::extensions::khr_external_memory_capabilities::KHR_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME)
                }
                #[cfg(not(feature = "khr_external_memory_capabilities"))]
                {
                    false
                }
            },
            khr_external_semaphore_capabilities: {
                #[cfg(feature = "khr_external_semaphore_capabilities")]
                {
                    enabled_extension(crate::extensions::khr_external_semaphore_capabilities::KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_EXTENSION_NAME)
                }
                #[cfg(not(feature = "khr_external_semaphore_capabilities"))]
                {
                    false
                }
            },
            khr_external_fence_capabilities: {
                #[cfg(feature = "khr_external_fence_capabilities")]
                {
                    enabled_extension(crate::extensions::khr_external_fence_capabilities::KHR_EXTERNAL_FENCE_CAPABILITIES_EXTENSION_NAME)
                }
                #[cfg(not(feature = "khr_external_fence_capabilities"))]
                {
                    false
                }
            },
            khr_device_group_creation: {
                #[cfg(feature = "khr_device_group_creation")]
                {
                    enabled_extension(crate::extensions::khr_device_group_creation::KHR_DEVICE_GROUP_CREATION_EXTENSION_NAME)
                }
                #[cfg(not(feature = "khr_device_group_creation"))]
                {
                    false
                }
            },
        })
    }
}
#[doc = r" Loader for instance commands."]
#[doc = r""]
#[doc = r" This will consume lots of stack space, so consider putting it into"]
#[doc = r" a heap pointer type such as `Box` or `Arc`."]
#[doc = r""]
#[doc = r" To create a new loader, call [`InstanceLoader::new`]."]
pub struct InstanceLoader {
    #[allow(dead_code)]
    parent: std::sync::Weak<()>,
    arc: std::sync::Arc<()>,
    pub handle: crate::vk1_0::Instance,
    enabled: InstanceEnabled,
    pub get_device_proc_addr: crate::vk1_0::PFN_vkGetDeviceProcAddr,
    pub destroy_instance: Option<vk1_0::PFN_vkDestroyInstance>,
    pub enumerate_physical_devices: Option<vk1_0::PFN_vkEnumeratePhysicalDevices>,
    pub get_instance_proc_addr: Option<vk1_0::PFN_vkGetInstanceProcAddr>,
    pub get_physical_device_properties: Option<vk1_0::PFN_vkGetPhysicalDeviceProperties>,
    pub get_physical_device_queue_family_properties: Option<vk1_0::PFN_vkGetPhysicalDeviceQueueFamilyProperties>,
    pub get_physical_device_memory_properties: Option<vk1_0::PFN_vkGetPhysicalDeviceMemoryProperties>,
    pub get_physical_device_features: Option<vk1_0::PFN_vkGetPhysicalDeviceFeatures>,
    pub get_physical_device_format_properties: Option<vk1_0::PFN_vkGetPhysicalDeviceFormatProperties>,
    pub get_physical_device_image_format_properties: Option<vk1_0::PFN_vkGetPhysicalDeviceImageFormatProperties>,
    pub create_device: Option<vk1_0::PFN_vkCreateDevice>,
    pub enumerate_device_layer_properties: Option<vk1_0::PFN_vkEnumerateDeviceLayerProperties>,
    pub enumerate_device_extension_properties: Option<vk1_0::PFN_vkEnumerateDeviceExtensionProperties>,
    pub get_physical_device_sparse_image_format_properties: Option<vk1_0::PFN_vkGetPhysicalDeviceSparseImageFormatProperties>,
    #[cfg(feature = "khr_android_surface")]
    pub create_android_surface_khr: Option<extensions::khr_android_surface::PFN_vkCreateAndroidSurfaceKHR>,
    #[cfg(feature = "khr_display")]
    pub get_physical_device_display_properties_khr: Option<extensions::khr_display::PFN_vkGetPhysicalDeviceDisplayPropertiesKHR>,
    #[cfg(feature = "khr_display")]
    pub get_physical_device_display_plane_properties_khr: Option<extensions::khr_display::PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR>,
    #[cfg(feature = "khr_display")]
    pub get_display_plane_supported_displays_khr: Option<extensions::khr_display::PFN_vkGetDisplayPlaneSupportedDisplaysKHR>,
    #[cfg(feature = "khr_display")]
    pub get_display_mode_properties_khr: Option<extensions::khr_display::PFN_vkGetDisplayModePropertiesKHR>,
    #[cfg(feature = "khr_display")]
    pub create_display_mode_khr: Option<extensions::khr_display::PFN_vkCreateDisplayModeKHR>,
    #[cfg(feature = "khr_display")]
    pub get_display_plane_capabilities_khr: Option<extensions::khr_display::PFN_vkGetDisplayPlaneCapabilitiesKHR>,
    #[cfg(feature = "khr_display")]
    pub create_display_plane_surface_khr: Option<extensions::khr_display::PFN_vkCreateDisplayPlaneSurfaceKHR>,
    #[cfg(feature = "khr_surface")]
    pub destroy_surface_khr: Option<extensions::khr_surface::PFN_vkDestroySurfaceKHR>,
    #[cfg(feature = "khr_surface")]
    pub get_physical_device_surface_support_khr: Option<extensions::khr_surface::PFN_vkGetPhysicalDeviceSurfaceSupportKHR>,
    #[cfg(feature = "khr_surface")]
    pub get_physical_device_surface_capabilities_khr: Option<extensions::khr_surface::PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR>,
    #[cfg(feature = "khr_surface")]
    pub get_physical_device_surface_formats_khr: Option<extensions::khr_surface::PFN_vkGetPhysicalDeviceSurfaceFormatsKHR>,
    #[cfg(feature = "khr_surface")]
    pub get_physical_device_surface_present_modes_khr: Option<extensions::khr_surface::PFN_vkGetPhysicalDeviceSurfacePresentModesKHR>,
    #[cfg(feature = "nn_vi_surface")]
    pub create_vi_surface_nn: Option<extensions::nn_vi_surface::PFN_vkCreateViSurfaceNN>,
    #[cfg(feature = "khr_wayland_surface")]
    pub create_wayland_surface_khr: Option<extensions::khr_wayland_surface::PFN_vkCreateWaylandSurfaceKHR>,
    #[cfg(feature = "khr_wayland_surface")]
    pub get_physical_device_wayland_presentation_support_khr: Option<extensions::khr_wayland_surface::PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR>,
    #[cfg(feature = "khr_win32_surface")]
    pub create_win32_surface_khr: Option<extensions::khr_win32_surface::PFN_vkCreateWin32SurfaceKHR>,
    #[cfg(feature = "khr_win32_surface")]
    pub get_physical_device_win32_presentation_support_khr: Option<extensions::khr_win32_surface::PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR>,
    #[cfg(feature = "khr_xlib_surface")]
    pub create_xlib_surface_khr: Option<extensions::khr_xlib_surface::PFN_vkCreateXlibSurfaceKHR>,
    #[cfg(feature = "khr_xlib_surface")]
    pub get_physical_device_xlib_presentation_support_khr: Option<extensions::khr_xlib_surface::PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR>,
    #[cfg(feature = "khr_xcb_surface")]
    pub create_xcb_surface_khr: Option<extensions::khr_xcb_surface::PFN_vkCreateXcbSurfaceKHR>,
    #[cfg(feature = "khr_xcb_surface")]
    pub get_physical_device_xcb_presentation_support_khr: Option<extensions::khr_xcb_surface::PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR>,
    #[cfg(feature = "ext_directfb_surface")]
    pub create_direct_fb_surface_ext: Option<extensions::ext_directfb_surface::PFN_vkCreateDirectFBSurfaceEXT>,
    #[cfg(feature = "ext_directfb_surface")]
    pub get_physical_device_direct_fb_presentation_support_ext: Option<extensions::ext_directfb_surface::PFN_vkGetPhysicalDeviceDirectFBPresentationSupportEXT>,
    #[cfg(feature = "fuchsia_imagepipe_surface")]
    pub create_image_pipe_surface_fuchsia: Option<extensions::fuchsia_imagepipe_surface::PFN_vkCreateImagePipeSurfaceFUCHSIA>,
    #[cfg(feature = "ggp_stream_descriptor_surface")]
    pub create_stream_descriptor_surface_ggp: Option<extensions::ggp_stream_descriptor_surface::PFN_vkCreateStreamDescriptorSurfaceGGP>,
    #[cfg(feature = "qnx_screen_surface")]
    pub create_screen_surface_qnx: Option<extensions::qnx_screen_surface::PFN_vkCreateScreenSurfaceQNX>,
    #[cfg(feature = "qnx_screen_surface")]
    pub get_physical_device_screen_presentation_support_qnx: Option<extensions::qnx_screen_surface::PFN_vkGetPhysicalDeviceScreenPresentationSupportQNX>,
    #[cfg(feature = "ext_debug_report")]
    pub create_debug_report_callback_ext: Option<extensions::ext_debug_report::PFN_vkCreateDebugReportCallbackEXT>,
    #[cfg(feature = "ext_debug_report")]
    pub destroy_debug_report_callback_ext: Option<extensions::ext_debug_report::PFN_vkDestroyDebugReportCallbackEXT>,
    #[cfg(feature = "ext_debug_report")]
    pub debug_report_message_ext: Option<extensions::ext_debug_report::PFN_vkDebugReportMessageEXT>,
    #[cfg(feature = "nv_external_memory_capabilities")]
    pub get_physical_device_external_image_format_properties_nv: Option<extensions::nv_external_memory_capabilities::PFN_vkGetPhysicalDeviceExternalImageFormatPropertiesNV>,
    pub get_physical_device_features2: Option<vk1_1::PFN_vkGetPhysicalDeviceFeatures2>,
    pub get_physical_device_properties2: Option<vk1_1::PFN_vkGetPhysicalDeviceProperties2>,
    pub get_physical_device_format_properties2: Option<vk1_1::PFN_vkGetPhysicalDeviceFormatProperties2>,
    pub get_physical_device_image_format_properties2: Option<vk1_1::PFN_vkGetPhysicalDeviceImageFormatProperties2>,
    pub get_physical_device_queue_family_properties2: Option<vk1_1::PFN_vkGetPhysicalDeviceQueueFamilyProperties2>,
    pub get_physical_device_memory_properties2: Option<vk1_1::PFN_vkGetPhysicalDeviceMemoryProperties2>,
    pub get_physical_device_sparse_image_format_properties2: Option<vk1_1::PFN_vkGetPhysicalDeviceSparseImageFormatProperties2>,
    pub get_physical_device_external_buffer_properties: Option<vk1_1::PFN_vkGetPhysicalDeviceExternalBufferProperties>,
    pub get_physical_device_external_semaphore_properties: Option<vk1_1::PFN_vkGetPhysicalDeviceExternalSemaphoreProperties>,
    pub get_physical_device_external_fence_properties: Option<vk1_1::PFN_vkGetPhysicalDeviceExternalFenceProperties>,
    #[cfg(feature = "ext_direct_mode_display")]
    pub release_display_ext: Option<extensions::ext_direct_mode_display::PFN_vkReleaseDisplayEXT>,
    #[cfg(feature = "ext_acquire_xlib_display")]
    pub acquire_xlib_display_ext: Option<extensions::ext_acquire_xlib_display::PFN_vkAcquireXlibDisplayEXT>,
    #[cfg(feature = "ext_acquire_xlib_display")]
    pub get_rand_r_output_display_ext: Option<extensions::ext_acquire_xlib_display::PFN_vkGetRandROutputDisplayEXT>,
    #[cfg(feature = "nv_acquire_winrt_display")]
    pub acquire_winrt_display_nv: Option<extensions::nv_acquire_winrt_display::PFN_vkAcquireWinrtDisplayNV>,
    #[cfg(feature = "nv_acquire_winrt_display")]
    pub get_winrt_display_nv: Option<extensions::nv_acquire_winrt_display::PFN_vkGetWinrtDisplayNV>,
    #[cfg(feature = "ext_display_surface_counter")]
    pub get_physical_device_surface_capabilities2_ext: Option<extensions::ext_display_surface_counter::PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT>,
    pub enumerate_physical_device_groups: Option<vk1_1::PFN_vkEnumeratePhysicalDeviceGroups>,
    #[cfg(feature = "khr_swapchain")]
    pub get_physical_device_present_rectangles_khr: Option<extensions::khr_swapchain::PFN_vkGetPhysicalDevicePresentRectanglesKHR>,
    #[cfg(feature = "mvk_ios_surface")]
    pub create_ios_surface_mvk: Option<extensions::mvk_ios_surface::PFN_vkCreateIOSSurfaceMVK>,
    #[cfg(feature = "mvk_macos_surface")]
    pub create_mac_os_surface_mvk: Option<extensions::mvk_macos_surface::PFN_vkCreateMacOSSurfaceMVK>,
    #[cfg(feature = "ext_metal_surface")]
    pub create_metal_surface_ext: Option<extensions::ext_metal_surface::PFN_vkCreateMetalSurfaceEXT>,
    #[cfg(feature = "ext_sample_locations")]
    pub get_physical_device_multisample_properties_ext: Option<extensions::ext_sample_locations::PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT>,
    #[cfg(feature = "khr_get_surface_capabilities2")]
    pub get_physical_device_surface_capabilities2_khr: Option<extensions::khr_get_surface_capabilities2::PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR>,
    #[cfg(feature = "khr_get_surface_capabilities2")]
    pub get_physical_device_surface_formats2_khr: Option<extensions::khr_get_surface_capabilities2::PFN_vkGetPhysicalDeviceSurfaceFormats2KHR>,
    #[cfg(feature = "khr_get_display_properties2")]
    pub get_physical_device_display_properties2_khr: Option<extensions::khr_get_display_properties2::PFN_vkGetPhysicalDeviceDisplayProperties2KHR>,
    #[cfg(feature = "khr_get_display_properties2")]
    pub get_physical_device_display_plane_properties2_khr: Option<extensions::khr_get_display_properties2::PFN_vkGetPhysicalDeviceDisplayPlaneProperties2KHR>,
    #[cfg(feature = "khr_get_display_properties2")]
    pub get_display_mode_properties2_khr: Option<extensions::khr_get_display_properties2::PFN_vkGetDisplayModeProperties2KHR>,
    #[cfg(feature = "khr_get_display_properties2")]
    pub get_display_plane_capabilities2_khr: Option<extensions::khr_get_display_properties2::PFN_vkGetDisplayPlaneCapabilities2KHR>,
    #[cfg(feature = "ext_calibrated_timestamps")]
    pub get_physical_device_calibrateable_time_domains_ext: Option<extensions::ext_calibrated_timestamps::PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsEXT>,
    #[cfg(feature = "ext_debug_utils")]
    pub create_debug_utils_messenger_ext: Option<extensions::ext_debug_utils::PFN_vkCreateDebugUtilsMessengerEXT>,
    #[cfg(feature = "ext_debug_utils")]
    pub destroy_debug_utils_messenger_ext: Option<extensions::ext_debug_utils::PFN_vkDestroyDebugUtilsMessengerEXT>,
    #[cfg(feature = "ext_debug_utils")]
    pub submit_debug_utils_message_ext: Option<extensions::ext_debug_utils::PFN_vkSubmitDebugUtilsMessageEXT>,
    #[cfg(feature = "nv_cooperative_matrix")]
    pub get_physical_device_cooperative_matrix_properties_nv: Option<extensions::nv_cooperative_matrix::PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesNV>,
    #[cfg(feature = "ext_full_screen_exclusive")]
    pub get_physical_device_surface_present_modes2_ext: Option<extensions::ext_full_screen_exclusive::PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT>,
    #[cfg(feature = "khr_performance_query")]
    pub enumerate_physical_device_queue_family_performance_query_counters_khr: Option<extensions::khr_performance_query::PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR>,
    #[cfg(feature = "khr_performance_query")]
    pub get_physical_device_queue_family_performance_query_passes_khr: Option<extensions::khr_performance_query::PFN_vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR>,
    #[cfg(feature = "ext_headless_surface")]
    pub create_headless_surface_ext: Option<extensions::ext_headless_surface::PFN_vkCreateHeadlessSurfaceEXT>,
    #[cfg(feature = "nv_coverage_reduction_mode")]
    pub get_physical_device_supported_framebuffer_mixed_samples_combinations_nv: Option<extensions::nv_coverage_reduction_mode::PFN_vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV>,
    #[cfg(feature = "ext_tooling_info")]
    pub get_physical_device_tool_properties_ext: Option<extensions::ext_tooling_info::PFN_vkGetPhysicalDeviceToolPropertiesEXT>,
    #[cfg(feature = "khr_fragment_shading_rate")]
    pub get_physical_device_fragment_shading_rates_khr: Option<extensions::khr_fragment_shading_rate::PFN_vkGetPhysicalDeviceFragmentShadingRatesKHR>,
    #[cfg(feature = "khr_video_queue")]
    pub get_physical_device_video_capabilities_khr: Option<extensions::khr_video_queue::PFN_vkGetPhysicalDeviceVideoCapabilitiesKHR>,
    #[cfg(feature = "khr_video_queue")]
    pub get_physical_device_video_format_properties_khr: Option<extensions::khr_video_queue::PFN_vkGetPhysicalDeviceVideoFormatPropertiesKHR>,
    #[cfg(feature = "khr_get_physical_device_properties2")]
    pub get_physical_device_features2_khr: Option<extensions::khr_get_physical_device_properties2::PFN_vkGetPhysicalDeviceFeatures2KHR>,
    #[cfg(feature = "khr_get_physical_device_properties2")]
    pub get_physical_device_properties2_khr: Option<extensions::khr_get_physical_device_properties2::PFN_vkGetPhysicalDeviceProperties2KHR>,
    #[cfg(feature = "khr_get_physical_device_properties2")]
    pub get_physical_device_format_properties2_khr: Option<extensions::khr_get_physical_device_properties2::PFN_vkGetPhysicalDeviceFormatProperties2KHR>,
    #[cfg(feature = "khr_get_physical_device_properties2")]
    pub get_physical_device_image_format_properties2_khr: Option<extensions::khr_get_physical_device_properties2::PFN_vkGetPhysicalDeviceImageFormatProperties2KHR>,
    #[cfg(feature = "khr_get_physical_device_properties2")]
    pub get_physical_device_queue_family_properties2_khr: Option<extensions::khr_get_physical_device_properties2::PFN_vkGetPhysicalDeviceQueueFamilyProperties2KHR>,
    #[cfg(feature = "khr_get_physical_device_properties2")]
    pub get_physical_device_memory_properties2_khr: Option<extensions::khr_get_physical_device_properties2::PFN_vkGetPhysicalDeviceMemoryProperties2KHR>,
    #[cfg(feature = "khr_get_physical_device_properties2")]
    pub get_physical_device_sparse_image_format_properties2_khr: Option<extensions::khr_get_physical_device_properties2::PFN_vkGetPhysicalDeviceSparseImageFormatProperties2KHR>,
    #[cfg(feature = "khr_external_memory_capabilities")]
    pub get_physical_device_external_buffer_properties_khr: Option<extensions::khr_external_memory_capabilities::PFN_vkGetPhysicalDeviceExternalBufferPropertiesKHR>,
    #[cfg(feature = "khr_external_semaphore_capabilities")]
    pub get_physical_device_external_semaphore_properties_khr: Option<extensions::khr_external_semaphore_capabilities::PFN_vkGetPhysicalDeviceExternalSemaphorePropertiesKHR>,
    #[cfg(feature = "khr_external_fence_capabilities")]
    pub get_physical_device_external_fence_properties_khr: Option<extensions::khr_external_fence_capabilities::PFN_vkGetPhysicalDeviceExternalFencePropertiesKHR>,
    #[cfg(feature = "khr_device_group_creation")]
    pub enumerate_physical_device_groups_khr: Option<extensions::khr_device_group_creation::PFN_vkEnumeratePhysicalDeviceGroupsKHR>,
}
impl InstanceLoader {
    #[allow(unused_parens)]
    pub unsafe fn custom<T>(entry_loader: &EntryLoader<T>, instance: crate::vk1_0::Instance, instance_enabled: InstanceEnabled, mut symbol: impl FnMut(*const std::os::raw::c_char) -> Option<crate::vk1_0::PFN_vkVoidFunction>) -> Result<InstanceLoader, crate::LoaderError> {
        let get_device_proc_addr = symbol(crate::vk1_0::FN_GET_DEVICE_PROC_ADDR).ok_or(crate::LoaderError::SymbolNotAvailable)?;
        Ok(InstanceLoader {
            parent: std::sync::Arc::downgrade(&entry_loader.arc),
            arc: std::sync::Arc::new(()),
            handle: instance,
            get_device_proc_addr: std::mem::transmute(get_device_proc_addr),
            destroy_instance: std::mem::transmute(symbol(crate::vk1_0::FN_DESTROY_INSTANCE)),
            enumerate_physical_devices: std::mem::transmute(symbol(crate::vk1_0::FN_ENUMERATE_PHYSICAL_DEVICES)),
            get_instance_proc_addr: std::mem::transmute(symbol(crate::vk1_0::FN_GET_INSTANCE_PROC_ADDR)),
            get_physical_device_properties: std::mem::transmute(symbol(crate::vk1_0::FN_GET_PHYSICAL_DEVICE_PROPERTIES)),
            get_physical_device_queue_family_properties: std::mem::transmute(symbol(crate::vk1_0::FN_GET_PHYSICAL_DEVICE_QUEUE_FAMILY_PROPERTIES)),
            get_physical_device_memory_properties: std::mem::transmute(symbol(crate::vk1_0::FN_GET_PHYSICAL_DEVICE_MEMORY_PROPERTIES)),
            get_physical_device_features: std::mem::transmute(symbol(crate::vk1_0::FN_GET_PHYSICAL_DEVICE_FEATURES)),
            get_physical_device_format_properties: std::mem::transmute(symbol(crate::vk1_0::FN_GET_PHYSICAL_DEVICE_FORMAT_PROPERTIES)),
            get_physical_device_image_format_properties: std::mem::transmute(symbol(crate::vk1_0::FN_GET_PHYSICAL_DEVICE_IMAGE_FORMAT_PROPERTIES)),
            create_device: std::mem::transmute(symbol(crate::vk1_0::FN_CREATE_DEVICE)),
            enumerate_device_layer_properties: std::mem::transmute(symbol(crate::vk1_0::FN_ENUMERATE_DEVICE_LAYER_PROPERTIES)),
            enumerate_device_extension_properties: std::mem::transmute(symbol(crate::vk1_0::FN_ENUMERATE_DEVICE_EXTENSION_PROPERTIES)),
            get_physical_device_sparse_image_format_properties: std::mem::transmute(symbol(crate::vk1_0::FN_GET_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_PROPERTIES)),
            #[cfg(feature = "khr_android_surface")]
            create_android_surface_khr: if instance_enabled.khr_android_surface { std::mem::transmute(symbol(crate::extensions::khr_android_surface::FN_CREATE_ANDROID_SURFACE_KHR)) } else { None },
            #[cfg(feature = "khr_display")]
            get_physical_device_display_properties_khr: if instance_enabled.khr_display { std::mem::transmute(symbol(crate::extensions::khr_display::FN_GET_PHYSICAL_DEVICE_DISPLAY_PROPERTIES_KHR)) } else { None },
            #[cfg(feature = "khr_display")]
            get_physical_device_display_plane_properties_khr: if instance_enabled.khr_display { std::mem::transmute(symbol(crate::extensions::khr_display::FN_GET_PHYSICAL_DEVICE_DISPLAY_PLANE_PROPERTIES_KHR)) } else { None },
            #[cfg(feature = "khr_display")]
            get_display_plane_supported_displays_khr: if instance_enabled.khr_display { std::mem::transmute(symbol(crate::extensions::khr_display::FN_GET_DISPLAY_PLANE_SUPPORTED_DISPLAYS_KHR)) } else { None },
            #[cfg(feature = "khr_display")]
            get_display_mode_properties_khr: if instance_enabled.khr_display { std::mem::transmute(symbol(crate::extensions::khr_display::FN_GET_DISPLAY_MODE_PROPERTIES_KHR)) } else { None },
            #[cfg(feature = "khr_display")]
            create_display_mode_khr: if instance_enabled.khr_display { std::mem::transmute(symbol(crate::extensions::khr_display::FN_CREATE_DISPLAY_MODE_KHR)) } else { None },
            #[cfg(feature = "khr_display")]
            get_display_plane_capabilities_khr: if instance_enabled.khr_display { std::mem::transmute(symbol(crate::extensions::khr_display::FN_GET_DISPLAY_PLANE_CAPABILITIES_KHR)) } else { None },
            #[cfg(feature = "khr_display")]
            create_display_plane_surface_khr: if instance_enabled.khr_display { std::mem::transmute(symbol(crate::extensions::khr_display::FN_CREATE_DISPLAY_PLANE_SURFACE_KHR)) } else { None },
            #[cfg(feature = "khr_surface")]
            destroy_surface_khr: if instance_enabled.khr_surface { std::mem::transmute(symbol(crate::extensions::khr_surface::FN_DESTROY_SURFACE_KHR)) } else { None },
            #[cfg(feature = "khr_surface")]
            get_physical_device_surface_support_khr: if instance_enabled.khr_surface { std::mem::transmute(symbol(crate::extensions::khr_surface::FN_GET_PHYSICAL_DEVICE_SURFACE_SUPPORT_KHR)) } else { None },
            #[cfg(feature = "khr_surface")]
            get_physical_device_surface_capabilities_khr: if instance_enabled.khr_surface { std::mem::transmute(symbol(crate::extensions::khr_surface::FN_GET_PHYSICAL_DEVICE_SURFACE_CAPABILITIES_KHR)) } else { None },
            #[cfg(feature = "khr_surface")]
            get_physical_device_surface_formats_khr: if instance_enabled.khr_surface { std::mem::transmute(symbol(crate::extensions::khr_surface::FN_GET_PHYSICAL_DEVICE_SURFACE_FORMATS_KHR)) } else { None },
            #[cfg(feature = "khr_surface")]
            get_physical_device_surface_present_modes_khr: if instance_enabled.khr_surface { std::mem::transmute(symbol(crate::extensions::khr_surface::FN_GET_PHYSICAL_DEVICE_SURFACE_PRESENT_MODES_KHR)) } else { None },
            #[cfg(feature = "nn_vi_surface")]
            create_vi_surface_nn: if instance_enabled.nn_vi_surface { std::mem::transmute(symbol(crate::extensions::nn_vi_surface::FN_CREATE_VI_SURFACE_NN)) } else { None },
            #[cfg(feature = "khr_wayland_surface")]
            create_wayland_surface_khr: if instance_enabled.khr_wayland_surface { std::mem::transmute(symbol(crate::extensions::khr_wayland_surface::FN_CREATE_WAYLAND_SURFACE_KHR)) } else { None },
            #[cfg(feature = "khr_wayland_surface")]
            get_physical_device_wayland_presentation_support_khr: if instance_enabled.khr_wayland_surface { std::mem::transmute(symbol(crate::extensions::khr_wayland_surface::FN_GET_PHYSICAL_DEVICE_WAYLAND_PRESENTATION_SUPPORT_KHR)) } else { None },
            #[cfg(feature = "khr_win32_surface")]
            create_win32_surface_khr: if instance_enabled.khr_win32_surface { std::mem::transmute(symbol(crate::extensions::khr_win32_surface::FN_CREATE_WIN32_SURFACE_KHR)) } else { None },
            #[cfg(feature = "khr_win32_surface")]
            get_physical_device_win32_presentation_support_khr: if instance_enabled.khr_win32_surface { std::mem::transmute(symbol(crate::extensions::khr_win32_surface::FN_GET_PHYSICAL_DEVICE_WIN32_PRESENTATION_SUPPORT_KHR)) } else { None },
            #[cfg(feature = "khr_xlib_surface")]
            create_xlib_surface_khr: if instance_enabled.khr_xlib_surface { std::mem::transmute(symbol(crate::extensions::khr_xlib_surface::FN_CREATE_XLIB_SURFACE_KHR)) } else { None },
            #[cfg(feature = "khr_xlib_surface")]
            get_physical_device_xlib_presentation_support_khr: if instance_enabled.khr_xlib_surface { std::mem::transmute(symbol(crate::extensions::khr_xlib_surface::FN_GET_PHYSICAL_DEVICE_XLIB_PRESENTATION_SUPPORT_KHR)) } else { None },
            #[cfg(feature = "khr_xcb_surface")]
            create_xcb_surface_khr: if instance_enabled.khr_xcb_surface { std::mem::transmute(symbol(crate::extensions::khr_xcb_surface::FN_CREATE_XCB_SURFACE_KHR)) } else { None },
            #[cfg(feature = "khr_xcb_surface")]
            get_physical_device_xcb_presentation_support_khr: if instance_enabled.khr_xcb_surface { std::mem::transmute(symbol(crate::extensions::khr_xcb_surface::FN_GET_PHYSICAL_DEVICE_XCB_PRESENTATION_SUPPORT_KHR)) } else { None },
            #[cfg(feature = "ext_directfb_surface")]
            create_direct_fb_surface_ext: if instance_enabled.ext_directfb_surface { std::mem::transmute(symbol(crate::extensions::ext_directfb_surface::FN_CREATE_DIRECT_FB_SURFACE_EXT)) } else { None },
            #[cfg(feature = "ext_directfb_surface")]
            get_physical_device_direct_fb_presentation_support_ext: if instance_enabled.ext_directfb_surface { std::mem::transmute(symbol(crate::extensions::ext_directfb_surface::FN_GET_PHYSICAL_DEVICE_DIRECT_FB_PRESENTATION_SUPPORT_EXT)) } else { None },
            #[cfg(feature = "fuchsia_imagepipe_surface")]
            create_image_pipe_surface_fuchsia: if instance_enabled.fuchsia_imagepipe_surface { std::mem::transmute(symbol(crate::extensions::fuchsia_imagepipe_surface::FN_CREATE_IMAGE_PIPE_SURFACE_FUCHSIA)) } else { None },
            #[cfg(feature = "ggp_stream_descriptor_surface")]
            create_stream_descriptor_surface_ggp: if instance_enabled.ggp_stream_descriptor_surface { std::mem::transmute(symbol(crate::extensions::ggp_stream_descriptor_surface::FN_CREATE_STREAM_DESCRIPTOR_SURFACE_GGP)) } else { None },
            #[cfg(feature = "qnx_screen_surface")]
            create_screen_surface_qnx: if instance_enabled.qnx_screen_surface { std::mem::transmute(symbol(crate::extensions::qnx_screen_surface::FN_CREATE_SCREEN_SURFACE_QNX)) } else { None },
            #[cfg(feature = "qnx_screen_surface")]
            get_physical_device_screen_presentation_support_qnx: if instance_enabled.qnx_screen_surface { std::mem::transmute(symbol(crate::extensions::qnx_screen_surface::FN_GET_PHYSICAL_DEVICE_SCREEN_PRESENTATION_SUPPORT_QNX)) } else { None },
            #[cfg(feature = "ext_debug_report")]
            create_debug_report_callback_ext: if instance_enabled.ext_debug_report { std::mem::transmute(symbol(crate::extensions::ext_debug_report::FN_CREATE_DEBUG_REPORT_CALLBACK_EXT)) } else { None },
            #[cfg(feature = "ext_debug_report")]
            destroy_debug_report_callback_ext: if instance_enabled.ext_debug_report { std::mem::transmute(symbol(crate::extensions::ext_debug_report::FN_DESTROY_DEBUG_REPORT_CALLBACK_EXT)) } else { None },
            #[cfg(feature = "ext_debug_report")]
            debug_report_message_ext: if instance_enabled.ext_debug_report { std::mem::transmute(symbol(crate::extensions::ext_debug_report::FN_DEBUG_REPORT_MESSAGE_EXT)) } else { None },
            #[cfg(feature = "nv_external_memory_capabilities")]
            get_physical_device_external_image_format_properties_nv: if instance_enabled.nv_external_memory_capabilities { std::mem::transmute(symbol(crate::extensions::nv_external_memory_capabilities::FN_GET_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_PROPERTIES_NV)) } else { None },
            get_physical_device_features2: if instance_enabled.vk1_1 { std::mem::transmute(symbol(crate::vk1_1::FN_GET_PHYSICAL_DEVICE_FEATURES2)) } else { None },
            get_physical_device_properties2: if instance_enabled.vk1_1 { std::mem::transmute(symbol(crate::vk1_1::FN_GET_PHYSICAL_DEVICE_PROPERTIES2)) } else { None },
            get_physical_device_format_properties2: if instance_enabled.vk1_1 { std::mem::transmute(symbol(crate::vk1_1::FN_GET_PHYSICAL_DEVICE_FORMAT_PROPERTIES2)) } else { None },
            get_physical_device_image_format_properties2: if instance_enabled.vk1_1 { std::mem::transmute(symbol(crate::vk1_1::FN_GET_PHYSICAL_DEVICE_IMAGE_FORMAT_PROPERTIES2)) } else { None },
            get_physical_device_queue_family_properties2: if instance_enabled.vk1_1 { std::mem::transmute(symbol(crate::vk1_1::FN_GET_PHYSICAL_DEVICE_QUEUE_FAMILY_PROPERTIES2)) } else { None },
            get_physical_device_memory_properties2: if instance_enabled.vk1_1 { std::mem::transmute(symbol(crate::vk1_1::FN_GET_PHYSICAL_DEVICE_MEMORY_PROPERTIES2)) } else { None },
            get_physical_device_sparse_image_format_properties2: if instance_enabled.vk1_1 { std::mem::transmute(symbol(crate::vk1_1::FN_GET_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_PROPERTIES2)) } else { None },
            get_physical_device_external_buffer_properties: if instance_enabled.vk1_1 { std::mem::transmute(symbol(crate::vk1_1::FN_GET_PHYSICAL_DEVICE_EXTERNAL_BUFFER_PROPERTIES)) } else { None },
            get_physical_device_external_semaphore_properties: if instance_enabled.vk1_1 { std::mem::transmute(symbol(crate::vk1_1::FN_GET_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_PROPERTIES)) } else { None },
            get_physical_device_external_fence_properties: if instance_enabled.vk1_1 { std::mem::transmute(symbol(crate::vk1_1::FN_GET_PHYSICAL_DEVICE_EXTERNAL_FENCE_PROPERTIES)) } else { None },
            #[cfg(feature = "ext_direct_mode_display")]
            release_display_ext: if instance_enabled.ext_direct_mode_display { std::mem::transmute(symbol(crate::extensions::ext_direct_mode_display::FN_RELEASE_DISPLAY_EXT)) } else { None },
            #[cfg(feature = "ext_acquire_xlib_display")]
            acquire_xlib_display_ext: if instance_enabled.ext_acquire_xlib_display { std::mem::transmute(symbol(crate::extensions::ext_acquire_xlib_display::FN_ACQUIRE_XLIB_DISPLAY_EXT)) } else { None },
            #[cfg(feature = "ext_acquire_xlib_display")]
            get_rand_r_output_display_ext: if instance_enabled.ext_acquire_xlib_display { std::mem::transmute(symbol(crate::extensions::ext_acquire_xlib_display::FN_GET_RAND_R_OUTPUT_DISPLAY_EXT)) } else { None },
            #[cfg(feature = "nv_acquire_winrt_display")]
            acquire_winrt_display_nv: if instance_enabled.nv_acquire_winrt_display { std::mem::transmute(symbol(crate::extensions::nv_acquire_winrt_display::FN_ACQUIRE_WINRT_DISPLAY_NV)) } else { None },
            #[cfg(feature = "nv_acquire_winrt_display")]
            get_winrt_display_nv: if instance_enabled.nv_acquire_winrt_display { std::mem::transmute(symbol(crate::extensions::nv_acquire_winrt_display::FN_GET_WINRT_DISPLAY_NV)) } else { None },
            #[cfg(feature = "ext_display_surface_counter")]
            get_physical_device_surface_capabilities2_ext: if instance_enabled.ext_display_surface_counter { std::mem::transmute(symbol(crate::extensions::ext_display_surface_counter::FN_GET_PHYSICAL_DEVICE_SURFACE_CAPABILITIES2_EXT)) } else { None },
            enumerate_physical_device_groups: if instance_enabled.vk1_1 { std::mem::transmute(symbol(crate::vk1_1::FN_ENUMERATE_PHYSICAL_DEVICE_GROUPS)) } else { None },
            #[cfg(feature = "khr_swapchain")]
            get_physical_device_present_rectangles_khr: if (instance_enabled.khr_swapchain && instance_enabled.vk1_1) || (instance_enabled.khr_device_group && instance_enabled.khr_surface) { std::mem::transmute(symbol(crate::extensions::khr_swapchain::FN_GET_PHYSICAL_DEVICE_PRESENT_RECTANGLES_KHR)) } else { None },
            #[cfg(feature = "mvk_ios_surface")]
            create_ios_surface_mvk: if instance_enabled.mvk_ios_surface { std::mem::transmute(symbol(crate::extensions::mvk_ios_surface::FN_CREATE_IOS_SURFACE_MVK)) } else { None },
            #[cfg(feature = "mvk_macos_surface")]
            create_mac_os_surface_mvk: if instance_enabled.mvk_macos_surface { std::mem::transmute(symbol(crate::extensions::mvk_macos_surface::FN_CREATE_MAC_OS_SURFACE_MVK)) } else { None },
            #[cfg(feature = "ext_metal_surface")]
            create_metal_surface_ext: if instance_enabled.ext_metal_surface { std::mem::transmute(symbol(crate::extensions::ext_metal_surface::FN_CREATE_METAL_SURFACE_EXT)) } else { None },
            #[cfg(feature = "ext_sample_locations")]
            get_physical_device_multisample_properties_ext: if instance_enabled.ext_sample_locations { std::mem::transmute(symbol(crate::extensions::ext_sample_locations::FN_GET_PHYSICAL_DEVICE_MULTISAMPLE_PROPERTIES_EXT)) } else { None },
            #[cfg(feature = "khr_get_surface_capabilities2")]
            get_physical_device_surface_capabilities2_khr: if instance_enabled.khr_get_surface_capabilities2 { std::mem::transmute(symbol(crate::extensions::khr_get_surface_capabilities2::FN_GET_PHYSICAL_DEVICE_SURFACE_CAPABILITIES2_KHR)) } else { None },
            #[cfg(feature = "khr_get_surface_capabilities2")]
            get_physical_device_surface_formats2_khr: if instance_enabled.khr_get_surface_capabilities2 { std::mem::transmute(symbol(crate::extensions::khr_get_surface_capabilities2::FN_GET_PHYSICAL_DEVICE_SURFACE_FORMATS2_KHR)) } else { None },
            #[cfg(feature = "khr_get_display_properties2")]
            get_physical_device_display_properties2_khr: if instance_enabled.khr_get_display_properties2 { std::mem::transmute(symbol(crate::extensions::khr_get_display_properties2::FN_GET_PHYSICAL_DEVICE_DISPLAY_PROPERTIES2_KHR)) } else { None },
            #[cfg(feature = "khr_get_display_properties2")]
            get_physical_device_display_plane_properties2_khr: if instance_enabled.khr_get_display_properties2 { std::mem::transmute(symbol(crate::extensions::khr_get_display_properties2::FN_GET_PHYSICAL_DEVICE_DISPLAY_PLANE_PROPERTIES2_KHR)) } else { None },
            #[cfg(feature = "khr_get_display_properties2")]
            get_display_mode_properties2_khr: if instance_enabled.khr_get_display_properties2 { std::mem::transmute(symbol(crate::extensions::khr_get_display_properties2::FN_GET_DISPLAY_MODE_PROPERTIES2_KHR)) } else { None },
            #[cfg(feature = "khr_get_display_properties2")]
            get_display_plane_capabilities2_khr: if instance_enabled.khr_get_display_properties2 { std::mem::transmute(symbol(crate::extensions::khr_get_display_properties2::FN_GET_DISPLAY_PLANE_CAPABILITIES2_KHR)) } else { None },
            #[cfg(feature = "ext_calibrated_timestamps")]
            get_physical_device_calibrateable_time_domains_ext: if instance_enabled.ext_calibrated_timestamps { std::mem::transmute(symbol(crate::extensions::ext_calibrated_timestamps::FN_GET_PHYSICAL_DEVICE_CALIBRATEABLE_TIME_DOMAINS_EXT)) } else { None },
            #[cfg(feature = "ext_debug_utils")]
            create_debug_utils_messenger_ext: if instance_enabled.ext_debug_utils { std::mem::transmute(symbol(crate::extensions::ext_debug_utils::FN_CREATE_DEBUG_UTILS_MESSENGER_EXT)) } else { None },
            #[cfg(feature = "ext_debug_utils")]
            destroy_debug_utils_messenger_ext: if instance_enabled.ext_debug_utils { std::mem::transmute(symbol(crate::extensions::ext_debug_utils::FN_DESTROY_DEBUG_UTILS_MESSENGER_EXT)) } else { None },
            #[cfg(feature = "ext_debug_utils")]
            submit_debug_utils_message_ext: if instance_enabled.ext_debug_utils { std::mem::transmute(symbol(crate::extensions::ext_debug_utils::FN_SUBMIT_DEBUG_UTILS_MESSAGE_EXT)) } else { None },
            #[cfg(feature = "nv_cooperative_matrix")]
            get_physical_device_cooperative_matrix_properties_nv: if instance_enabled.nv_cooperative_matrix { std::mem::transmute(symbol(crate::extensions::nv_cooperative_matrix::FN_GET_PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_NV)) } else { None },
            #[cfg(feature = "ext_full_screen_exclusive")]
            get_physical_device_surface_present_modes2_ext: if instance_enabled.ext_full_screen_exclusive { std::mem::transmute(symbol(crate::extensions::ext_full_screen_exclusive::FN_GET_PHYSICAL_DEVICE_SURFACE_PRESENT_MODES2_EXT)) } else { None },
            #[cfg(feature = "khr_performance_query")]
            enumerate_physical_device_queue_family_performance_query_counters_khr: if instance_enabled.khr_performance_query { std::mem::transmute(symbol(crate::extensions::khr_performance_query::FN_ENUMERATE_PHYSICAL_DEVICE_QUEUE_FAMILY_PERFORMANCE_QUERY_COUNTERS_KHR)) } else { None },
            #[cfg(feature = "khr_performance_query")]
            get_physical_device_queue_family_performance_query_passes_khr: if instance_enabled.khr_performance_query { std::mem::transmute(symbol(crate::extensions::khr_performance_query::FN_GET_PHYSICAL_DEVICE_QUEUE_FAMILY_PERFORMANCE_QUERY_PASSES_KHR)) } else { None },
            #[cfg(feature = "ext_headless_surface")]
            create_headless_surface_ext: if instance_enabled.ext_headless_surface { std::mem::transmute(symbol(crate::extensions::ext_headless_surface::FN_CREATE_HEADLESS_SURFACE_EXT)) } else { None },
            #[cfg(feature = "nv_coverage_reduction_mode")]
            get_physical_device_supported_framebuffer_mixed_samples_combinations_nv: if instance_enabled.nv_coverage_reduction_mode { std::mem::transmute(symbol(crate::extensions::nv_coverage_reduction_mode::FN_GET_PHYSICAL_DEVICE_SUPPORTED_FRAMEBUFFER_MIXED_SAMPLES_COMBINATIONS_NV)) } else { None },
            #[cfg(feature = "ext_tooling_info")]
            get_physical_device_tool_properties_ext: if instance_enabled.ext_tooling_info { std::mem::transmute(symbol(crate::extensions::ext_tooling_info::FN_GET_PHYSICAL_DEVICE_TOOL_PROPERTIES_EXT)) } else { None },
            #[cfg(feature = "khr_fragment_shading_rate")]
            get_physical_device_fragment_shading_rates_khr: if instance_enabled.khr_fragment_shading_rate { std::mem::transmute(symbol(crate::extensions::khr_fragment_shading_rate::FN_GET_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATES_KHR)) } else { None },
            #[cfg(feature = "khr_video_queue")]
            get_physical_device_video_capabilities_khr: if instance_enabled.khr_video_queue { std::mem::transmute(symbol(crate::extensions::khr_video_queue::FN_GET_PHYSICAL_DEVICE_VIDEO_CAPABILITIES_KHR)) } else { None },
            #[cfg(feature = "khr_video_queue")]
            get_physical_device_video_format_properties_khr: if instance_enabled.khr_video_queue { std::mem::transmute(symbol(crate::extensions::khr_video_queue::FN_GET_PHYSICAL_DEVICE_VIDEO_FORMAT_PROPERTIES_KHR)) } else { None },
            #[cfg(feature = "khr_get_physical_device_properties2")]
            get_physical_device_features2_khr: if instance_enabled.khr_get_physical_device_properties2 { std::mem::transmute(symbol(crate::extensions::khr_get_physical_device_properties2::FN_GET_PHYSICAL_DEVICE_FEATURES2_KHR)) } else { None },
            #[cfg(feature = "khr_get_physical_device_properties2")]
            get_physical_device_properties2_khr: if instance_enabled.khr_get_physical_device_properties2 { std::mem::transmute(symbol(crate::extensions::khr_get_physical_device_properties2::FN_GET_PHYSICAL_DEVICE_PROPERTIES2_KHR)) } else { None },
            #[cfg(feature = "khr_get_physical_device_properties2")]
            get_physical_device_format_properties2_khr: if instance_enabled.khr_get_physical_device_properties2 { std::mem::transmute(symbol(crate::extensions::khr_get_physical_device_properties2::FN_GET_PHYSICAL_DEVICE_FORMAT_PROPERTIES2_KHR)) } else { None },
            #[cfg(feature = "khr_get_physical_device_properties2")]
            get_physical_device_image_format_properties2_khr: if instance_enabled.khr_get_physical_device_properties2 { std::mem::transmute(symbol(crate::extensions::khr_get_physical_device_properties2::FN_GET_PHYSICAL_DEVICE_IMAGE_FORMAT_PROPERTIES2_KHR)) } else { None },
            #[cfg(feature = "khr_get_physical_device_properties2")]
            get_physical_device_queue_family_properties2_khr: if instance_enabled.khr_get_physical_device_properties2 { std::mem::transmute(symbol(crate::extensions::khr_get_physical_device_properties2::FN_GET_PHYSICAL_DEVICE_QUEUE_FAMILY_PROPERTIES2_KHR)) } else { None },
            #[cfg(feature = "khr_get_physical_device_properties2")]
            get_physical_device_memory_properties2_khr: if instance_enabled.khr_get_physical_device_properties2 { std::mem::transmute(symbol(crate::extensions::khr_get_physical_device_properties2::FN_GET_PHYSICAL_DEVICE_MEMORY_PROPERTIES2_KHR)) } else { None },
            #[cfg(feature = "khr_get_physical_device_properties2")]
            get_physical_device_sparse_image_format_properties2_khr: if instance_enabled.khr_get_physical_device_properties2 { std::mem::transmute(symbol(crate::extensions::khr_get_physical_device_properties2::FN_GET_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_PROPERTIES2_KHR)) } else { None },
            #[cfg(feature = "khr_external_memory_capabilities")]
            get_physical_device_external_buffer_properties_khr: if instance_enabled.khr_external_memory_capabilities { std::mem::transmute(symbol(crate::extensions::khr_external_memory_capabilities::FN_GET_PHYSICAL_DEVICE_EXTERNAL_BUFFER_PROPERTIES_KHR)) } else { None },
            #[cfg(feature = "khr_external_semaphore_capabilities")]
            get_physical_device_external_semaphore_properties_khr: if instance_enabled.khr_external_semaphore_capabilities { std::mem::transmute(symbol(crate::extensions::khr_external_semaphore_capabilities::FN_GET_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_PROPERTIES_KHR)) } else { None },
            #[cfg(feature = "khr_external_fence_capabilities")]
            get_physical_device_external_fence_properties_khr: if instance_enabled.khr_external_fence_capabilities { std::mem::transmute(symbol(crate::extensions::khr_external_fence_capabilities::FN_GET_PHYSICAL_DEVICE_EXTERNAL_FENCE_PROPERTIES_KHR)) } else { None },
            #[cfg(feature = "khr_device_group_creation")]
            enumerate_physical_device_groups_khr: if instance_enabled.khr_device_group_creation { std::mem::transmute(symbol(crate::extensions::khr_device_group_creation::FN_ENUMERATE_PHYSICAL_DEVICE_GROUPS_KHR)) } else { None },
            enabled: instance_enabled,
        })
    }
    pub fn enabled(&self) -> &InstanceEnabled {
        &self.enabled
    }
}
impl Drop for InstanceLoader {
    fn drop(&mut self) {
        if std::sync::Arc::weak_count(&self.arc) != 0 {
            panic!("Attempting to drop a instance loader with active references to it");
        }
    }
}
#[doc = r" A list of requirements enabled in the device loader."]
#[derive(Debug)]
pub struct DeviceEnabled {
    pub ext_conditional_rendering: bool,
    pub khr_display_swapchain: bool,
    pub khr_swapchain: bool,
    pub ext_debug_marker: bool,
    pub nv_external_memory_win32: bool,
    pub nv_device_generated_commands: bool,
    pub khr_push_descriptor: bool,
    pub khr_external_memory_win32: bool,
    pub khr_external_memory_fd: bool,
    pub fuchsia_external_memory: bool,
    pub khr_external_semaphore_win32: bool,
    pub khr_external_semaphore_fd: bool,
    pub fuchsia_external_semaphore: bool,
    pub khr_external_fence_win32: bool,
    pub khr_external_fence_fd: bool,
    pub nv_acquire_winrt_display: bool,
    pub ext_display_control: bool,
    pub ext_hdr_metadata: bool,
    pub khr_shared_presentable_image: bool,
    pub google_display_timing: bool,
    pub nv_clip_space_w_scaling: bool,
    pub ext_discard_rectangles: bool,
    pub ext_sample_locations: bool,
    pub ext_validation_cache: bool,
    pub amd_shader_info: bool,
    pub amd_display_native_hdr: bool,
    pub ext_calibrated_timestamps: bool,
    pub ext_external_memory_host: bool,
    pub amd_buffer_marker: bool,
    pub android_external_memory_android_hardware_buffer: bool,
    pub nv_device_diagnostic_checkpoints: bool,
    pub ext_transform_feedback: bool,
    pub nv_scissor_exclusive: bool,
    pub nv_shading_rate_image: bool,
    pub nv_mesh_shader: bool,
    pub nv_ray_tracing: bool,
    pub khr_acceleration_structure: bool,
    pub khr_ray_tracing_pipeline: bool,
    pub nv_cooperative_matrix: bool,
    pub nvx_image_view_handle: bool,
    pub ext_full_screen_exclusive: bool,
    pub khr_performance_query: bool,
    pub ext_image_drm_format_modifier: bool,
    pub nv_coverage_reduction_mode: bool,
    pub intel_performance_query: bool,
    pub khr_pipeline_executable_properties: bool,
    pub ext_line_rasterization: bool,
    pub ext_tooling_info: bool,
    pub khr_deferred_host_operations: bool,
    pub ext_extended_dynamic_state: bool,
    pub ext_extended_dynamic_state2: bool,
    pub ext_private_data: bool,
    pub khr_copy_commands2: bool,
    pub khr_fragment_shading_rate: bool,
    pub nv_fragment_shading_rate_enums: bool,
    pub ext_vertex_input_dynamic_state: bool,
    pub ext_color_write_enable: bool,
    pub khr_synchronization2: bool,
    pub khr_video_queue: bool,
    pub khr_video_decode_queue: bool,
    pub khr_video_encode_queue: bool,
    pub nvx_binary_import: bool,
    pub ext_host_query_reset: bool,
    pub khr_maintenance1: bool,
    pub khr_device_group: bool,
    pub khr_bind_memory2: bool,
    pub khr_descriptor_update_template: bool,
    pub khr_get_memory_requirements2: bool,
    pub khr_sampler_ycbcr_conversion: bool,
    pub khr_maintenance3: bool,
    pub khr_create_renderpass2: bool,
    pub khr_timeline_semaphore: bool,
    pub khr_draw_indirect_count: bool,
    pub amd_draw_indirect_count: bool,
    pub khr_buffer_device_address: bool,
    pub ext_buffer_device_address: bool,
}
impl DeviceEnabled {
    #[allow(unused_variables)]
    pub unsafe fn new(enabled_extensions: &[&std::ffi::CStr]) -> DeviceEnabled {
        let enabled_extension = |extension| enabled_extensions.contains(&std::ffi::CStr::from_ptr(extension));
        DeviceEnabled {
            ext_conditional_rendering: {
                #[cfg(feature = "ext_conditional_rendering")]
                {
                    enabled_extension(crate::extensions::ext_conditional_rendering::EXT_CONDITIONAL_RENDERING_EXTENSION_NAME)
                }
                #[cfg(not(feature = "ext_conditional_rendering"))]
                {
                    false
                }
            },
            khr_display_swapchain: {
                #[cfg(feature = "khr_display_swapchain")]
                {
                    enabled_extension(crate::extensions::khr_display_swapchain::KHR_DISPLAY_SWAPCHAIN_EXTENSION_NAME)
                }
                #[cfg(not(feature = "khr_display_swapchain"))]
                {
                    false
                }
            },
            khr_swapchain: {
                #[cfg(feature = "khr_swapchain")]
                {
                    enabled_extension(crate::extensions::khr_swapchain::KHR_SWAPCHAIN_EXTENSION_NAME)
                }
                #[cfg(not(feature = "khr_swapchain"))]
                {
                    false
                }
            },
            ext_debug_marker: {
                #[cfg(feature = "ext_debug_marker")]
                {
                    enabled_extension(crate::extensions::ext_debug_marker::EXT_DEBUG_MARKER_EXTENSION_NAME)
                }
                #[cfg(not(feature = "ext_debug_marker"))]
                {
                    false
                }
            },
            nv_external_memory_win32: {
                #[cfg(feature = "nv_external_memory_win32")]
                {
                    enabled_extension(crate::extensions::nv_external_memory_win32::NV_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME)
                }
                #[cfg(not(feature = "nv_external_memory_win32"))]
                {
                    false
                }
            },
            nv_device_generated_commands: {
                #[cfg(feature = "nv_device_generated_commands")]
                {
                    enabled_extension(crate::extensions::nv_device_generated_commands::NV_DEVICE_GENERATED_COMMANDS_EXTENSION_NAME)
                }
                #[cfg(not(feature = "nv_device_generated_commands"))]
                {
                    false
                }
            },
            khr_push_descriptor: {
                #[cfg(feature = "khr_push_descriptor")]
                {
                    enabled_extension(crate::extensions::khr_push_descriptor::KHR_PUSH_DESCRIPTOR_EXTENSION_NAME)
                }
                #[cfg(not(feature = "khr_push_descriptor"))]
                {
                    false
                }
            },
            khr_external_memory_win32: {
                #[cfg(feature = "khr_external_memory_win32")]
                {
                    enabled_extension(crate::extensions::khr_external_memory_win32::KHR_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME)
                }
                #[cfg(not(feature = "khr_external_memory_win32"))]
                {
                    false
                }
            },
            khr_external_memory_fd: {
                #[cfg(feature = "khr_external_memory_fd")]
                {
                    enabled_extension(crate::extensions::khr_external_memory_fd::KHR_EXTERNAL_MEMORY_FD_EXTENSION_NAME)
                }
                #[cfg(not(feature = "khr_external_memory_fd"))]
                {
                    false
                }
            },
            fuchsia_external_memory: {
                #[cfg(feature = "fuchsia_external_memory")]
                {
                    enabled_extension(crate::extensions::fuchsia_external_memory::FUCHSIA_EXTERNAL_MEMORY_EXTENSION_NAME)
                }
                #[cfg(not(feature = "fuchsia_external_memory"))]
                {
                    false
                }
            },
            khr_external_semaphore_win32: {
                #[cfg(feature = "khr_external_semaphore_win32")]
                {
                    enabled_extension(crate::extensions::khr_external_semaphore_win32::KHR_EXTERNAL_SEMAPHORE_WIN32_EXTENSION_NAME)
                }
                #[cfg(not(feature = "khr_external_semaphore_win32"))]
                {
                    false
                }
            },
            khr_external_semaphore_fd: {
                #[cfg(feature = "khr_external_semaphore_fd")]
                {
                    enabled_extension(crate::extensions::khr_external_semaphore_fd::KHR_EXTERNAL_SEMAPHORE_FD_EXTENSION_NAME)
                }
                #[cfg(not(feature = "khr_external_semaphore_fd"))]
                {
                    false
                }
            },
            fuchsia_external_semaphore: {
                #[cfg(feature = "fuchsia_external_semaphore")]
                {
                    enabled_extension(crate::extensions::fuchsia_external_semaphore::FUCHSIA_EXTERNAL_SEMAPHORE_EXTENSION_NAME)
                }
                #[cfg(not(feature = "fuchsia_external_semaphore"))]
                {
                    false
                }
            },
            khr_external_fence_win32: {
                #[cfg(feature = "khr_external_fence_win32")]
                {
                    enabled_extension(crate::extensions::khr_external_fence_win32::KHR_EXTERNAL_FENCE_WIN32_EXTENSION_NAME)
                }
                #[cfg(not(feature = "khr_external_fence_win32"))]
                {
                    false
                }
            },
            khr_external_fence_fd: {
                #[cfg(feature = "khr_external_fence_fd")]
                {
                    enabled_extension(crate::extensions::khr_external_fence_fd::KHR_EXTERNAL_FENCE_FD_EXTENSION_NAME)
                }
                #[cfg(not(feature = "khr_external_fence_fd"))]
                {
                    false
                }
            },
            nv_acquire_winrt_display: {
                #[cfg(feature = "nv_acquire_winrt_display")]
                {
                    enabled_extension(crate::extensions::nv_acquire_winrt_display::NV_ACQUIRE_WINRT_DISPLAY_EXTENSION_NAME)
                }
                #[cfg(not(feature = "nv_acquire_winrt_display"))]
                {
                    false
                }
            },
            ext_display_control: {
                #[cfg(feature = "ext_display_control")]
                {
                    enabled_extension(crate::extensions::ext_display_control::EXT_DISPLAY_CONTROL_EXTENSION_NAME)
                }
                #[cfg(not(feature = "ext_display_control"))]
                {
                    false
                }
            },
            ext_hdr_metadata: {
                #[cfg(feature = "ext_hdr_metadata")]
                {
                    enabled_extension(crate::extensions::ext_hdr_metadata::EXT_HDR_METADATA_EXTENSION_NAME)
                }
                #[cfg(not(feature = "ext_hdr_metadata"))]
                {
                    false
                }
            },
            khr_shared_presentable_image: {
                #[cfg(feature = "khr_shared_presentable_image")]
                {
                    enabled_extension(crate::extensions::khr_shared_presentable_image::KHR_SHARED_PRESENTABLE_IMAGE_EXTENSION_NAME)
                }
                #[cfg(not(feature = "khr_shared_presentable_image"))]
                {
                    false
                }
            },
            google_display_timing: {
                #[cfg(feature = "google_display_timing")]
                {
                    enabled_extension(crate::extensions::google_display_timing::GOOGLE_DISPLAY_TIMING_EXTENSION_NAME)
                }
                #[cfg(not(feature = "google_display_timing"))]
                {
                    false
                }
            },
            nv_clip_space_w_scaling: {
                #[cfg(feature = "nv_clip_space_w_scaling")]
                {
                    enabled_extension(crate::extensions::nv_clip_space_w_scaling::NV_CLIP_SPACE_W_SCALING_EXTENSION_NAME)
                }
                #[cfg(not(feature = "nv_clip_space_w_scaling"))]
                {
                    false
                }
            },
            ext_discard_rectangles: {
                #[cfg(feature = "ext_discard_rectangles")]
                {
                    enabled_extension(crate::extensions::ext_discard_rectangles::EXT_DISCARD_RECTANGLES_EXTENSION_NAME)
                }
                #[cfg(not(feature = "ext_discard_rectangles"))]
                {
                    false
                }
            },
            ext_sample_locations: {
                #[cfg(feature = "ext_sample_locations")]
                {
                    enabled_extension(crate::extensions::ext_sample_locations::EXT_SAMPLE_LOCATIONS_EXTENSION_NAME)
                }
                #[cfg(not(feature = "ext_sample_locations"))]
                {
                    false
                }
            },
            ext_validation_cache: {
                #[cfg(feature = "ext_validation_cache")]
                {
                    enabled_extension(crate::extensions::ext_validation_cache::EXT_VALIDATION_CACHE_EXTENSION_NAME)
                }
                #[cfg(not(feature = "ext_validation_cache"))]
                {
                    false
                }
            },
            amd_shader_info: {
                #[cfg(feature = "amd_shader_info")]
                {
                    enabled_extension(crate::extensions::amd_shader_info::AMD_SHADER_INFO_EXTENSION_NAME)
                }
                #[cfg(not(feature = "amd_shader_info"))]
                {
                    false
                }
            },
            amd_display_native_hdr: {
                #[cfg(feature = "amd_display_native_hdr")]
                {
                    enabled_extension(crate::extensions::amd_display_native_hdr::AMD_DISPLAY_NATIVE_HDR_EXTENSION_NAME)
                }
                #[cfg(not(feature = "amd_display_native_hdr"))]
                {
                    false
                }
            },
            ext_calibrated_timestamps: {
                #[cfg(feature = "ext_calibrated_timestamps")]
                {
                    enabled_extension(crate::extensions::ext_calibrated_timestamps::EXT_CALIBRATED_TIMESTAMPS_EXTENSION_NAME)
                }
                #[cfg(not(feature = "ext_calibrated_timestamps"))]
                {
                    false
                }
            },
            ext_external_memory_host: {
                #[cfg(feature = "ext_external_memory_host")]
                {
                    enabled_extension(crate::extensions::ext_external_memory_host::EXT_EXTERNAL_MEMORY_HOST_EXTENSION_NAME)
                }
                #[cfg(not(feature = "ext_external_memory_host"))]
                {
                    false
                }
            },
            amd_buffer_marker: {
                #[cfg(feature = "amd_buffer_marker")]
                {
                    enabled_extension(crate::extensions::amd_buffer_marker::AMD_BUFFER_MARKER_EXTENSION_NAME)
                }
                #[cfg(not(feature = "amd_buffer_marker"))]
                {
                    false
                }
            },
            android_external_memory_android_hardware_buffer: {
                #[cfg(feature = "android_external_memory_android_hardware_buffer")]
                {
                    enabled_extension(crate::extensions::android_external_memory_android_hardware_buffer::ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER_EXTENSION_NAME)
                }
                #[cfg(not(feature = "android_external_memory_android_hardware_buffer"))]
                {
                    false
                }
            },
            nv_device_diagnostic_checkpoints: {
                #[cfg(feature = "nv_device_diagnostic_checkpoints")]
                {
                    enabled_extension(crate::extensions::nv_device_diagnostic_checkpoints::NV_DEVICE_DIAGNOSTIC_CHECKPOINTS_EXTENSION_NAME)
                }
                #[cfg(not(feature = "nv_device_diagnostic_checkpoints"))]
                {
                    false
                }
            },
            ext_transform_feedback: {
                #[cfg(feature = "ext_transform_feedback")]
                {
                    enabled_extension(crate::extensions::ext_transform_feedback::EXT_TRANSFORM_FEEDBACK_EXTENSION_NAME)
                }
                #[cfg(not(feature = "ext_transform_feedback"))]
                {
                    false
                }
            },
            nv_scissor_exclusive: {
                #[cfg(feature = "nv_scissor_exclusive")]
                {
                    enabled_extension(crate::extensions::nv_scissor_exclusive::NV_SCISSOR_EXCLUSIVE_EXTENSION_NAME)
                }
                #[cfg(not(feature = "nv_scissor_exclusive"))]
                {
                    false
                }
            },
            nv_shading_rate_image: {
                #[cfg(feature = "nv_shading_rate_image")]
                {
                    enabled_extension(crate::extensions::nv_shading_rate_image::NV_SHADING_RATE_IMAGE_EXTENSION_NAME)
                }
                #[cfg(not(feature = "nv_shading_rate_image"))]
                {
                    false
                }
            },
            nv_mesh_shader: {
                #[cfg(feature = "nv_mesh_shader")]
                {
                    enabled_extension(crate::extensions::nv_mesh_shader::NV_MESH_SHADER_EXTENSION_NAME)
                }
                #[cfg(not(feature = "nv_mesh_shader"))]
                {
                    false
                }
            },
            nv_ray_tracing: {
                #[cfg(feature = "nv_ray_tracing")]
                {
                    enabled_extension(crate::extensions::nv_ray_tracing::NV_RAY_TRACING_EXTENSION_NAME)
                }
                #[cfg(not(feature = "nv_ray_tracing"))]
                {
                    false
                }
            },
            khr_acceleration_structure: {
                #[cfg(feature = "khr_acceleration_structure")]
                {
                    enabled_extension(crate::extensions::khr_acceleration_structure::KHR_ACCELERATION_STRUCTURE_EXTENSION_NAME)
                }
                #[cfg(not(feature = "khr_acceleration_structure"))]
                {
                    false
                }
            },
            khr_ray_tracing_pipeline: {
                #[cfg(feature = "khr_ray_tracing_pipeline")]
                {
                    enabled_extension(crate::extensions::khr_ray_tracing_pipeline::KHR_RAY_TRACING_PIPELINE_EXTENSION_NAME)
                }
                #[cfg(not(feature = "khr_ray_tracing_pipeline"))]
                {
                    false
                }
            },
            nv_cooperative_matrix: {
                #[cfg(feature = "nv_cooperative_matrix")]
                {
                    enabled_extension(crate::extensions::nv_cooperative_matrix::NV_COOPERATIVE_MATRIX_EXTENSION_NAME)
                }
                #[cfg(not(feature = "nv_cooperative_matrix"))]
                {
                    false
                }
            },
            nvx_image_view_handle: {
                #[cfg(feature = "nvx_image_view_handle")]
                {
                    enabled_extension(crate::extensions::nvx_image_view_handle::NVX_IMAGE_VIEW_HANDLE_EXTENSION_NAME)
                }
                #[cfg(not(feature = "nvx_image_view_handle"))]
                {
                    false
                }
            },
            ext_full_screen_exclusive: {
                #[cfg(feature = "ext_full_screen_exclusive")]
                {
                    enabled_extension(crate::extensions::ext_full_screen_exclusive::EXT_FULL_SCREEN_EXCLUSIVE_EXTENSION_NAME)
                }
                #[cfg(not(feature = "ext_full_screen_exclusive"))]
                {
                    false
                }
            },
            khr_performance_query: {
                #[cfg(feature = "khr_performance_query")]
                {
                    enabled_extension(crate::extensions::khr_performance_query::KHR_PERFORMANCE_QUERY_EXTENSION_NAME)
                }
                #[cfg(not(feature = "khr_performance_query"))]
                {
                    false
                }
            },
            ext_image_drm_format_modifier: {
                #[cfg(feature = "ext_image_drm_format_modifier")]
                {
                    enabled_extension(crate::extensions::ext_image_drm_format_modifier::EXT_IMAGE_DRM_FORMAT_MODIFIER_EXTENSION_NAME)
                }
                #[cfg(not(feature = "ext_image_drm_format_modifier"))]
                {
                    false
                }
            },
            nv_coverage_reduction_mode: {
                #[cfg(feature = "nv_coverage_reduction_mode")]
                {
                    enabled_extension(crate::extensions::nv_coverage_reduction_mode::NV_COVERAGE_REDUCTION_MODE_EXTENSION_NAME)
                }
                #[cfg(not(feature = "nv_coverage_reduction_mode"))]
                {
                    false
                }
            },
            intel_performance_query: {
                #[cfg(feature = "intel_performance_query")]
                {
                    enabled_extension(crate::extensions::intel_performance_query::INTEL_PERFORMANCE_QUERY_EXTENSION_NAME)
                }
                #[cfg(not(feature = "intel_performance_query"))]
                {
                    false
                }
            },
            khr_pipeline_executable_properties: {
                #[cfg(feature = "khr_pipeline_executable_properties")]
                {
                    enabled_extension(crate::extensions::khr_pipeline_executable_properties::KHR_PIPELINE_EXECUTABLE_PROPERTIES_EXTENSION_NAME)
                }
                #[cfg(not(feature = "khr_pipeline_executable_properties"))]
                {
                    false
                }
            },
            ext_line_rasterization: {
                #[cfg(feature = "ext_line_rasterization")]
                {
                    enabled_extension(crate::extensions::ext_line_rasterization::EXT_LINE_RASTERIZATION_EXTENSION_NAME)
                }
                #[cfg(not(feature = "ext_line_rasterization"))]
                {
                    false
                }
            },
            ext_tooling_info: {
                #[cfg(feature = "ext_tooling_info")]
                {
                    enabled_extension(crate::extensions::ext_tooling_info::EXT_TOOLING_INFO_EXTENSION_NAME)
                }
                #[cfg(not(feature = "ext_tooling_info"))]
                {
                    false
                }
            },
            khr_deferred_host_operations: {
                #[cfg(feature = "khr_deferred_host_operations")]
                {
                    enabled_extension(crate::extensions::khr_deferred_host_operations::KHR_DEFERRED_HOST_OPERATIONS_EXTENSION_NAME)
                }
                #[cfg(not(feature = "khr_deferred_host_operations"))]
                {
                    false
                }
            },
            ext_extended_dynamic_state: {
                #[cfg(feature = "ext_extended_dynamic_state")]
                {
                    enabled_extension(crate::extensions::ext_extended_dynamic_state::EXT_EXTENDED_DYNAMIC_STATE_EXTENSION_NAME)
                }
                #[cfg(not(feature = "ext_extended_dynamic_state"))]
                {
                    false
                }
            },
            ext_extended_dynamic_state2: {
                #[cfg(feature = "ext_extended_dynamic_state2")]
                {
                    enabled_extension(crate::extensions::ext_extended_dynamic_state2::EXT_EXTENDED_DYNAMIC_STATE_2_EXTENSION_NAME)
                }
                #[cfg(not(feature = "ext_extended_dynamic_state2"))]
                {
                    false
                }
            },
            ext_private_data: {
                #[cfg(feature = "ext_private_data")]
                {
                    enabled_extension(crate::extensions::ext_private_data::EXT_PRIVATE_DATA_EXTENSION_NAME)
                }
                #[cfg(not(feature = "ext_private_data"))]
                {
                    false
                }
            },
            khr_copy_commands2: {
                #[cfg(feature = "khr_copy_commands2")]
                {
                    enabled_extension(crate::extensions::khr_copy_commands2::KHR_COPY_COMMANDS_2_EXTENSION_NAME)
                }
                #[cfg(not(feature = "khr_copy_commands2"))]
                {
                    false
                }
            },
            khr_fragment_shading_rate: {
                #[cfg(feature = "khr_fragment_shading_rate")]
                {
                    enabled_extension(crate::extensions::khr_fragment_shading_rate::KHR_FRAGMENT_SHADING_RATE_EXTENSION_NAME)
                }
                #[cfg(not(feature = "khr_fragment_shading_rate"))]
                {
                    false
                }
            },
            nv_fragment_shading_rate_enums: {
                #[cfg(feature = "nv_fragment_shading_rate_enums")]
                {
                    enabled_extension(crate::extensions::nv_fragment_shading_rate_enums::NV_FRAGMENT_SHADING_RATE_ENUMS_EXTENSION_NAME)
                }
                #[cfg(not(feature = "nv_fragment_shading_rate_enums"))]
                {
                    false
                }
            },
            ext_vertex_input_dynamic_state: {
                #[cfg(feature = "ext_vertex_input_dynamic_state")]
                {
                    enabled_extension(crate::extensions::ext_vertex_input_dynamic_state::EXT_VERTEX_INPUT_DYNAMIC_STATE_EXTENSION_NAME)
                }
                #[cfg(not(feature = "ext_vertex_input_dynamic_state"))]
                {
                    false
                }
            },
            ext_color_write_enable: {
                #[cfg(feature = "ext_color_write_enable")]
                {
                    enabled_extension(crate::extensions::ext_color_write_enable::EXT_COLOR_WRITE_ENABLE_EXTENSION_NAME)
                }
                #[cfg(not(feature = "ext_color_write_enable"))]
                {
                    false
                }
            },
            khr_synchronization2: {
                #[cfg(feature = "khr_synchronization2")]
                {
                    enabled_extension(crate::extensions::khr_synchronization2::KHR_SYNCHRONIZATION_2_EXTENSION_NAME)
                }
                #[cfg(not(feature = "khr_synchronization2"))]
                {
                    false
                }
            },
            khr_video_queue: {
                #[cfg(feature = "khr_video_queue")]
                {
                    enabled_extension(crate::extensions::khr_video_queue::KHR_VIDEO_QUEUE_EXTENSION_NAME)
                }
                #[cfg(not(feature = "khr_video_queue"))]
                {
                    false
                }
            },
            khr_video_decode_queue: {
                #[cfg(feature = "khr_video_decode_queue")]
                {
                    enabled_extension(crate::extensions::khr_video_decode_queue::KHR_VIDEO_DECODE_QUEUE_EXTENSION_NAME)
                }
                #[cfg(not(feature = "khr_video_decode_queue"))]
                {
                    false
                }
            },
            khr_video_encode_queue: {
                #[cfg(feature = "khr_video_encode_queue")]
                {
                    enabled_extension(crate::extensions::khr_video_encode_queue::KHR_VIDEO_ENCODE_QUEUE_EXTENSION_NAME)
                }
                #[cfg(not(feature = "khr_video_encode_queue"))]
                {
                    false
                }
            },
            nvx_binary_import: {
                #[cfg(feature = "nvx_binary_import")]
                {
                    enabled_extension(crate::extensions::nvx_binary_import::NVX_BINARY_IMPORT_EXTENSION_NAME)
                }
                #[cfg(not(feature = "nvx_binary_import"))]
                {
                    false
                }
            },
            ext_host_query_reset: {
                #[cfg(feature = "ext_host_query_reset")]
                {
                    enabled_extension(crate::extensions::ext_host_query_reset::EXT_HOST_QUERY_RESET_EXTENSION_NAME)
                }
                #[cfg(not(feature = "ext_host_query_reset"))]
                {
                    false
                }
            },
            khr_maintenance1: {
                #[cfg(feature = "khr_maintenance1")]
                {
                    enabled_extension(crate::extensions::khr_maintenance1::KHR_MAINTENANCE1_EXTENSION_NAME)
                }
                #[cfg(not(feature = "khr_maintenance1"))]
                {
                    false
                }
            },
            khr_device_group: {
                #[cfg(feature = "khr_device_group")]
                {
                    enabled_extension(crate::extensions::khr_device_group::KHR_DEVICE_GROUP_EXTENSION_NAME)
                }
                #[cfg(not(feature = "khr_device_group"))]
                {
                    false
                }
            },
            khr_bind_memory2: {
                #[cfg(feature = "khr_bind_memory2")]
                {
                    enabled_extension(crate::extensions::khr_bind_memory2::KHR_BIND_MEMORY_2_EXTENSION_NAME)
                }
                #[cfg(not(feature = "khr_bind_memory2"))]
                {
                    false
                }
            },
            khr_descriptor_update_template: {
                #[cfg(feature = "khr_descriptor_update_template")]
                {
                    enabled_extension(crate::extensions::khr_descriptor_update_template::KHR_DESCRIPTOR_UPDATE_TEMPLATE_EXTENSION_NAME)
                }
                #[cfg(not(feature = "khr_descriptor_update_template"))]
                {
                    false
                }
            },
            khr_get_memory_requirements2: {
                #[cfg(feature = "khr_get_memory_requirements2")]
                {
                    enabled_extension(crate::extensions::khr_get_memory_requirements2::KHR_GET_MEMORY_REQUIREMENTS_2_EXTENSION_NAME)
                }
                #[cfg(not(feature = "khr_get_memory_requirements2"))]
                {
                    false
                }
            },
            khr_sampler_ycbcr_conversion: {
                #[cfg(feature = "khr_sampler_ycbcr_conversion")]
                {
                    enabled_extension(crate::extensions::khr_sampler_ycbcr_conversion::KHR_SAMPLER_YCBCR_CONVERSION_EXTENSION_NAME)
                }
                #[cfg(not(feature = "khr_sampler_ycbcr_conversion"))]
                {
                    false
                }
            },
            khr_maintenance3: {
                #[cfg(feature = "khr_maintenance3")]
                {
                    enabled_extension(crate::extensions::khr_maintenance3::KHR_MAINTENANCE3_EXTENSION_NAME)
                }
                #[cfg(not(feature = "khr_maintenance3"))]
                {
                    false
                }
            },
            khr_create_renderpass2: {
                #[cfg(feature = "khr_create_renderpass2")]
                {
                    enabled_extension(crate::extensions::khr_create_renderpass2::KHR_CREATE_RENDERPASS_2_EXTENSION_NAME)
                }
                #[cfg(not(feature = "khr_create_renderpass2"))]
                {
                    false
                }
            },
            khr_timeline_semaphore: {
                #[cfg(feature = "khr_timeline_semaphore")]
                {
                    enabled_extension(crate::extensions::khr_timeline_semaphore::KHR_TIMELINE_SEMAPHORE_EXTENSION_NAME)
                }
                #[cfg(not(feature = "khr_timeline_semaphore"))]
                {
                    false
                }
            },
            khr_draw_indirect_count: {
                #[cfg(feature = "khr_draw_indirect_count")]
                {
                    enabled_extension(crate::extensions::khr_draw_indirect_count::KHR_DRAW_INDIRECT_COUNT_EXTENSION_NAME)
                }
                #[cfg(not(feature = "khr_draw_indirect_count"))]
                {
                    false
                }
            },
            amd_draw_indirect_count: {
                #[cfg(feature = "amd_draw_indirect_count")]
                {
                    enabled_extension(crate::extensions::amd_draw_indirect_count::AMD_DRAW_INDIRECT_COUNT_EXTENSION_NAME)
                }
                #[cfg(not(feature = "amd_draw_indirect_count"))]
                {
                    false
                }
            },
            khr_buffer_device_address: {
                #[cfg(feature = "khr_buffer_device_address")]
                {
                    enabled_extension(crate::extensions::khr_buffer_device_address::KHR_BUFFER_DEVICE_ADDRESS_EXTENSION_NAME)
                }
                #[cfg(not(feature = "khr_buffer_device_address"))]
                {
                    false
                }
            },
            ext_buffer_device_address: {
                #[cfg(feature = "ext_buffer_device_address")]
                {
                    enabled_extension(crate::extensions::ext_buffer_device_address::EXT_BUFFER_DEVICE_ADDRESS_EXTENSION_NAME)
                }
                #[cfg(not(feature = "ext_buffer_device_address"))]
                {
                    false
                }
            },
        }
    }
}
#[doc = r" Loader for device commands."]
#[doc = r""]
#[doc = r" This will consume lots of stack space, so consider putting it into"]
#[doc = r" a heap pointer type such as `Box` or `Arc`."]
#[doc = r""]
#[doc = r" To create a new loader, call [`DeviceLoader::new`]."]
pub struct DeviceLoader {
    #[allow(dead_code)]
    parent: std::sync::Weak<()>,
    pub handle: crate::vk1_0::Device,
    enabled: DeviceEnabled,
    pub get_device_proc_addr: Option<vk1_0::PFN_vkGetDeviceProcAddr>,
    pub destroy_device: Option<vk1_0::PFN_vkDestroyDevice>,
    pub get_device_queue: Option<vk1_0::PFN_vkGetDeviceQueue>,
    pub queue_submit: Option<vk1_0::PFN_vkQueueSubmit>,
    pub queue_wait_idle: Option<vk1_0::PFN_vkQueueWaitIdle>,
    pub device_wait_idle: Option<vk1_0::PFN_vkDeviceWaitIdle>,
    pub allocate_memory: Option<vk1_0::PFN_vkAllocateMemory>,
    pub free_memory: Option<vk1_0::PFN_vkFreeMemory>,
    pub map_memory: Option<vk1_0::PFN_vkMapMemory>,
    pub unmap_memory: Option<vk1_0::PFN_vkUnmapMemory>,
    pub flush_mapped_memory_ranges: Option<vk1_0::PFN_vkFlushMappedMemoryRanges>,
    pub invalidate_mapped_memory_ranges: Option<vk1_0::PFN_vkInvalidateMappedMemoryRanges>,
    pub get_device_memory_commitment: Option<vk1_0::PFN_vkGetDeviceMemoryCommitment>,
    pub get_buffer_memory_requirements: Option<vk1_0::PFN_vkGetBufferMemoryRequirements>,
    pub bind_buffer_memory: Option<vk1_0::PFN_vkBindBufferMemory>,
    pub get_image_memory_requirements: Option<vk1_0::PFN_vkGetImageMemoryRequirements>,
    pub bind_image_memory: Option<vk1_0::PFN_vkBindImageMemory>,
    pub get_image_sparse_memory_requirements: Option<vk1_0::PFN_vkGetImageSparseMemoryRequirements>,
    pub queue_bind_sparse: Option<vk1_0::PFN_vkQueueBindSparse>,
    pub create_fence: Option<vk1_0::PFN_vkCreateFence>,
    pub destroy_fence: Option<vk1_0::PFN_vkDestroyFence>,
    pub reset_fences: Option<vk1_0::PFN_vkResetFences>,
    pub get_fence_status: Option<vk1_0::PFN_vkGetFenceStatus>,
    pub wait_for_fences: Option<vk1_0::PFN_vkWaitForFences>,
    pub create_semaphore: Option<vk1_0::PFN_vkCreateSemaphore>,
    pub destroy_semaphore: Option<vk1_0::PFN_vkDestroySemaphore>,
    pub create_event: Option<vk1_0::PFN_vkCreateEvent>,
    pub destroy_event: Option<vk1_0::PFN_vkDestroyEvent>,
    pub get_event_status: Option<vk1_0::PFN_vkGetEventStatus>,
    pub set_event: Option<vk1_0::PFN_vkSetEvent>,
    pub reset_event: Option<vk1_0::PFN_vkResetEvent>,
    pub create_query_pool: Option<vk1_0::PFN_vkCreateQueryPool>,
    pub destroy_query_pool: Option<vk1_0::PFN_vkDestroyQueryPool>,
    pub get_query_pool_results: Option<vk1_0::PFN_vkGetQueryPoolResults>,
    pub reset_query_pool: Option<vk1_2::PFN_vkResetQueryPool>,
    pub create_buffer: Option<vk1_0::PFN_vkCreateBuffer>,
    pub destroy_buffer: Option<vk1_0::PFN_vkDestroyBuffer>,
    pub create_buffer_view: Option<vk1_0::PFN_vkCreateBufferView>,
    pub destroy_buffer_view: Option<vk1_0::PFN_vkDestroyBufferView>,
    pub create_image: Option<vk1_0::PFN_vkCreateImage>,
    pub destroy_image: Option<vk1_0::PFN_vkDestroyImage>,
    pub get_image_subresource_layout: Option<vk1_0::PFN_vkGetImageSubresourceLayout>,
    pub create_image_view: Option<vk1_0::PFN_vkCreateImageView>,
    pub destroy_image_view: Option<vk1_0::PFN_vkDestroyImageView>,
    pub create_shader_module: Option<vk1_0::PFN_vkCreateShaderModule>,
    pub destroy_shader_module: Option<vk1_0::PFN_vkDestroyShaderModule>,
    pub create_pipeline_cache: Option<vk1_0::PFN_vkCreatePipelineCache>,
    pub destroy_pipeline_cache: Option<vk1_0::PFN_vkDestroyPipelineCache>,
    pub get_pipeline_cache_data: Option<vk1_0::PFN_vkGetPipelineCacheData>,
    pub merge_pipeline_caches: Option<vk1_0::PFN_vkMergePipelineCaches>,
    pub create_graphics_pipelines: Option<vk1_0::PFN_vkCreateGraphicsPipelines>,
    pub create_compute_pipelines: Option<vk1_0::PFN_vkCreateComputePipelines>,
    pub destroy_pipeline: Option<vk1_0::PFN_vkDestroyPipeline>,
    pub create_pipeline_layout: Option<vk1_0::PFN_vkCreatePipelineLayout>,
    pub destroy_pipeline_layout: Option<vk1_0::PFN_vkDestroyPipelineLayout>,
    pub create_sampler: Option<vk1_0::PFN_vkCreateSampler>,
    pub destroy_sampler: Option<vk1_0::PFN_vkDestroySampler>,
    pub create_descriptor_set_layout: Option<vk1_0::PFN_vkCreateDescriptorSetLayout>,
    pub destroy_descriptor_set_layout: Option<vk1_0::PFN_vkDestroyDescriptorSetLayout>,
    pub create_descriptor_pool: Option<vk1_0::PFN_vkCreateDescriptorPool>,
    pub destroy_descriptor_pool: Option<vk1_0::PFN_vkDestroyDescriptorPool>,
    pub reset_descriptor_pool: Option<vk1_0::PFN_vkResetDescriptorPool>,
    pub allocate_descriptor_sets: Option<vk1_0::PFN_vkAllocateDescriptorSets>,
    pub free_descriptor_sets: Option<vk1_0::PFN_vkFreeDescriptorSets>,
    pub update_descriptor_sets: Option<vk1_0::PFN_vkUpdateDescriptorSets>,
    pub create_framebuffer: Option<vk1_0::PFN_vkCreateFramebuffer>,
    pub destroy_framebuffer: Option<vk1_0::PFN_vkDestroyFramebuffer>,
    pub create_render_pass: Option<vk1_0::PFN_vkCreateRenderPass>,
    pub destroy_render_pass: Option<vk1_0::PFN_vkDestroyRenderPass>,
    pub get_render_area_granularity: Option<vk1_0::PFN_vkGetRenderAreaGranularity>,
    pub create_command_pool: Option<vk1_0::PFN_vkCreateCommandPool>,
    pub destroy_command_pool: Option<vk1_0::PFN_vkDestroyCommandPool>,
    pub reset_command_pool: Option<vk1_0::PFN_vkResetCommandPool>,
    pub allocate_command_buffers: Option<vk1_0::PFN_vkAllocateCommandBuffers>,
    pub free_command_buffers: Option<vk1_0::PFN_vkFreeCommandBuffers>,
    pub begin_command_buffer: Option<vk1_0::PFN_vkBeginCommandBuffer>,
    pub end_command_buffer: Option<vk1_0::PFN_vkEndCommandBuffer>,
    pub reset_command_buffer: Option<vk1_0::PFN_vkResetCommandBuffer>,
    pub cmd_bind_pipeline: Option<vk1_0::PFN_vkCmdBindPipeline>,
    pub cmd_set_viewport: Option<vk1_0::PFN_vkCmdSetViewport>,
    pub cmd_set_scissor: Option<vk1_0::PFN_vkCmdSetScissor>,
    pub cmd_set_line_width: Option<vk1_0::PFN_vkCmdSetLineWidth>,
    pub cmd_set_depth_bias: Option<vk1_0::PFN_vkCmdSetDepthBias>,
    pub cmd_set_blend_constants: Option<vk1_0::PFN_vkCmdSetBlendConstants>,
    pub cmd_set_depth_bounds: Option<vk1_0::PFN_vkCmdSetDepthBounds>,
    pub cmd_set_stencil_compare_mask: Option<vk1_0::PFN_vkCmdSetStencilCompareMask>,
    pub cmd_set_stencil_write_mask: Option<vk1_0::PFN_vkCmdSetStencilWriteMask>,
    pub cmd_set_stencil_reference: Option<vk1_0::PFN_vkCmdSetStencilReference>,
    pub cmd_bind_descriptor_sets: Option<vk1_0::PFN_vkCmdBindDescriptorSets>,
    pub cmd_bind_index_buffer: Option<vk1_0::PFN_vkCmdBindIndexBuffer>,
    pub cmd_bind_vertex_buffers: Option<vk1_0::PFN_vkCmdBindVertexBuffers>,
    pub cmd_draw: Option<vk1_0::PFN_vkCmdDraw>,
    pub cmd_draw_indexed: Option<vk1_0::PFN_vkCmdDrawIndexed>,
    pub cmd_draw_indirect: Option<vk1_0::PFN_vkCmdDrawIndirect>,
    pub cmd_draw_indexed_indirect: Option<vk1_0::PFN_vkCmdDrawIndexedIndirect>,
    pub cmd_dispatch: Option<vk1_0::PFN_vkCmdDispatch>,
    pub cmd_dispatch_indirect: Option<vk1_0::PFN_vkCmdDispatchIndirect>,
    pub cmd_copy_buffer: Option<vk1_0::PFN_vkCmdCopyBuffer>,
    pub cmd_copy_image: Option<vk1_0::PFN_vkCmdCopyImage>,
    pub cmd_blit_image: Option<vk1_0::PFN_vkCmdBlitImage>,
    pub cmd_copy_buffer_to_image: Option<vk1_0::PFN_vkCmdCopyBufferToImage>,
    pub cmd_copy_image_to_buffer: Option<vk1_0::PFN_vkCmdCopyImageToBuffer>,
    pub cmd_update_buffer: Option<vk1_0::PFN_vkCmdUpdateBuffer>,
    pub cmd_fill_buffer: Option<vk1_0::PFN_vkCmdFillBuffer>,
    pub cmd_clear_color_image: Option<vk1_0::PFN_vkCmdClearColorImage>,
    pub cmd_clear_depth_stencil_image: Option<vk1_0::PFN_vkCmdClearDepthStencilImage>,
    pub cmd_clear_attachments: Option<vk1_0::PFN_vkCmdClearAttachments>,
    pub cmd_resolve_image: Option<vk1_0::PFN_vkCmdResolveImage>,
    pub cmd_set_event: Option<vk1_0::PFN_vkCmdSetEvent>,
    pub cmd_reset_event: Option<vk1_0::PFN_vkCmdResetEvent>,
    pub cmd_wait_events: Option<vk1_0::PFN_vkCmdWaitEvents>,
    pub cmd_pipeline_barrier: Option<vk1_0::PFN_vkCmdPipelineBarrier>,
    pub cmd_begin_query: Option<vk1_0::PFN_vkCmdBeginQuery>,
    pub cmd_end_query: Option<vk1_0::PFN_vkCmdEndQuery>,
    #[cfg(feature = "ext_conditional_rendering")]
    pub cmd_begin_conditional_rendering_ext: Option<extensions::ext_conditional_rendering::PFN_vkCmdBeginConditionalRenderingEXT>,
    #[cfg(feature = "ext_conditional_rendering")]
    pub cmd_end_conditional_rendering_ext: Option<extensions::ext_conditional_rendering::PFN_vkCmdEndConditionalRenderingEXT>,
    pub cmd_reset_query_pool: Option<vk1_0::PFN_vkCmdResetQueryPool>,
    pub cmd_write_timestamp: Option<vk1_0::PFN_vkCmdWriteTimestamp>,
    pub cmd_copy_query_pool_results: Option<vk1_0::PFN_vkCmdCopyQueryPoolResults>,
    pub cmd_push_constants: Option<vk1_0::PFN_vkCmdPushConstants>,
    pub cmd_begin_render_pass: Option<vk1_0::PFN_vkCmdBeginRenderPass>,
    pub cmd_next_subpass: Option<vk1_0::PFN_vkCmdNextSubpass>,
    pub cmd_end_render_pass: Option<vk1_0::PFN_vkCmdEndRenderPass>,
    pub cmd_execute_commands: Option<vk1_0::PFN_vkCmdExecuteCommands>,
    #[cfg(feature = "khr_display_swapchain")]
    pub create_shared_swapchains_khr: Option<extensions::khr_display_swapchain::PFN_vkCreateSharedSwapchainsKHR>,
    #[cfg(feature = "khr_swapchain")]
    pub create_swapchain_khr: Option<extensions::khr_swapchain::PFN_vkCreateSwapchainKHR>,
    #[cfg(feature = "khr_swapchain")]
    pub destroy_swapchain_khr: Option<extensions::khr_swapchain::PFN_vkDestroySwapchainKHR>,
    #[cfg(feature = "khr_swapchain")]
    pub get_swapchain_images_khr: Option<extensions::khr_swapchain::PFN_vkGetSwapchainImagesKHR>,
    #[cfg(feature = "khr_swapchain")]
    pub acquire_next_image_khr: Option<extensions::khr_swapchain::PFN_vkAcquireNextImageKHR>,
    #[cfg(feature = "khr_swapchain")]
    pub queue_present_khr: Option<extensions::khr_swapchain::PFN_vkQueuePresentKHR>,
    #[cfg(feature = "ext_debug_marker")]
    pub debug_marker_set_object_name_ext: Option<extensions::ext_debug_marker::PFN_vkDebugMarkerSetObjectNameEXT>,
    #[cfg(feature = "ext_debug_marker")]
    pub debug_marker_set_object_tag_ext: Option<extensions::ext_debug_marker::PFN_vkDebugMarkerSetObjectTagEXT>,
    #[cfg(feature = "ext_debug_marker")]
    pub cmd_debug_marker_begin_ext: Option<extensions::ext_debug_marker::PFN_vkCmdDebugMarkerBeginEXT>,
    #[cfg(feature = "ext_debug_marker")]
    pub cmd_debug_marker_end_ext: Option<extensions::ext_debug_marker::PFN_vkCmdDebugMarkerEndEXT>,
    #[cfg(feature = "ext_debug_marker")]
    pub cmd_debug_marker_insert_ext: Option<extensions::ext_debug_marker::PFN_vkCmdDebugMarkerInsertEXT>,
    #[cfg(feature = "nv_external_memory_win32")]
    pub get_memory_win32_handle_nv: Option<extensions::nv_external_memory_win32::PFN_vkGetMemoryWin32HandleNV>,
    #[cfg(feature = "nv_device_generated_commands")]
    pub cmd_execute_generated_commands_nv: Option<extensions::nv_device_generated_commands::PFN_vkCmdExecuteGeneratedCommandsNV>,
    #[cfg(feature = "nv_device_generated_commands")]
    pub cmd_preprocess_generated_commands_nv: Option<extensions::nv_device_generated_commands::PFN_vkCmdPreprocessGeneratedCommandsNV>,
    #[cfg(feature = "nv_device_generated_commands")]
    pub cmd_bind_pipeline_shader_group_nv: Option<extensions::nv_device_generated_commands::PFN_vkCmdBindPipelineShaderGroupNV>,
    #[cfg(feature = "nv_device_generated_commands")]
    pub get_generated_commands_memory_requirements_nv: Option<extensions::nv_device_generated_commands::PFN_vkGetGeneratedCommandsMemoryRequirementsNV>,
    #[cfg(feature = "nv_device_generated_commands")]
    pub create_indirect_commands_layout_nv: Option<extensions::nv_device_generated_commands::PFN_vkCreateIndirectCommandsLayoutNV>,
    #[cfg(feature = "nv_device_generated_commands")]
    pub destroy_indirect_commands_layout_nv: Option<extensions::nv_device_generated_commands::PFN_vkDestroyIndirectCommandsLayoutNV>,
    #[cfg(feature = "khr_push_descriptor")]
    pub cmd_push_descriptor_set_khr: Option<extensions::khr_push_descriptor::PFN_vkCmdPushDescriptorSetKHR>,
    pub trim_command_pool: Option<vk1_1::PFN_vkTrimCommandPool>,
    #[cfg(feature = "khr_external_memory_win32")]
    pub get_memory_win32_handle_khr: Option<extensions::khr_external_memory_win32::PFN_vkGetMemoryWin32HandleKHR>,
    #[cfg(feature = "khr_external_memory_win32")]
    pub get_memory_win32_handle_properties_khr: Option<extensions::khr_external_memory_win32::PFN_vkGetMemoryWin32HandlePropertiesKHR>,
    #[cfg(feature = "khr_external_memory_fd")]
    pub get_memory_fd_khr: Option<extensions::khr_external_memory_fd::PFN_vkGetMemoryFdKHR>,
    #[cfg(feature = "khr_external_memory_fd")]
    pub get_memory_fd_properties_khr: Option<extensions::khr_external_memory_fd::PFN_vkGetMemoryFdPropertiesKHR>,
    #[cfg(feature = "fuchsia_external_memory")]
    pub get_memory_zircon_handle_fuchsia: Option<extensions::fuchsia_external_memory::PFN_vkGetMemoryZirconHandleFUCHSIA>,
    #[cfg(feature = "fuchsia_external_memory")]
    pub get_memory_zircon_handle_properties_fuchsia: Option<extensions::fuchsia_external_memory::PFN_vkGetMemoryZirconHandlePropertiesFUCHSIA>,
    #[cfg(feature = "khr_external_semaphore_win32")]
    pub get_semaphore_win32_handle_khr: Option<extensions::khr_external_semaphore_win32::PFN_vkGetSemaphoreWin32HandleKHR>,
    #[cfg(feature = "khr_external_semaphore_win32")]
    pub import_semaphore_win32_handle_khr: Option<extensions::khr_external_semaphore_win32::PFN_vkImportSemaphoreWin32HandleKHR>,
    #[cfg(feature = "khr_external_semaphore_fd")]
    pub get_semaphore_fd_khr: Option<extensions::khr_external_semaphore_fd::PFN_vkGetSemaphoreFdKHR>,
    #[cfg(feature = "khr_external_semaphore_fd")]
    pub import_semaphore_fd_khr: Option<extensions::khr_external_semaphore_fd::PFN_vkImportSemaphoreFdKHR>,
    #[cfg(feature = "fuchsia_external_semaphore")]
    pub get_semaphore_zircon_handle_fuchsia: Option<extensions::fuchsia_external_semaphore::PFN_vkGetSemaphoreZirconHandleFUCHSIA>,
    #[cfg(feature = "fuchsia_external_semaphore")]
    pub import_semaphore_zircon_handle_fuchsia: Option<extensions::fuchsia_external_semaphore::PFN_vkImportSemaphoreZirconHandleFUCHSIA>,
    #[cfg(feature = "khr_external_fence_win32")]
    pub get_fence_win32_handle_khr: Option<extensions::khr_external_fence_win32::PFN_vkGetFenceWin32HandleKHR>,
    #[cfg(feature = "khr_external_fence_win32")]
    pub import_fence_win32_handle_khr: Option<extensions::khr_external_fence_win32::PFN_vkImportFenceWin32HandleKHR>,
    #[cfg(feature = "khr_external_fence_fd")]
    pub get_fence_fd_khr: Option<extensions::khr_external_fence_fd::PFN_vkGetFenceFdKHR>,
    #[cfg(feature = "khr_external_fence_fd")]
    pub import_fence_fd_khr: Option<extensions::khr_external_fence_fd::PFN_vkImportFenceFdKHR>,
    #[cfg(feature = "ext_display_control")]
    pub display_power_control_ext: Option<extensions::ext_display_control::PFN_vkDisplayPowerControlEXT>,
    #[cfg(feature = "ext_display_control")]
    pub register_device_event_ext: Option<extensions::ext_display_control::PFN_vkRegisterDeviceEventEXT>,
    #[cfg(feature = "ext_display_control")]
    pub register_display_event_ext: Option<extensions::ext_display_control::PFN_vkRegisterDisplayEventEXT>,
    #[cfg(feature = "ext_display_control")]
    pub get_swapchain_counter_ext: Option<extensions::ext_display_control::PFN_vkGetSwapchainCounterEXT>,
    pub get_device_group_peer_memory_features: Option<vk1_1::PFN_vkGetDeviceGroupPeerMemoryFeatures>,
    pub bind_buffer_memory2: Option<vk1_1::PFN_vkBindBufferMemory2>,
    pub bind_image_memory2: Option<vk1_1::PFN_vkBindImageMemory2>,
    pub cmd_set_device_mask: Option<vk1_1::PFN_vkCmdSetDeviceMask>,
    #[cfg(feature = "khr_swapchain")]
    pub get_device_group_present_capabilities_khr: Option<extensions::khr_swapchain::PFN_vkGetDeviceGroupPresentCapabilitiesKHR>,
    #[cfg(feature = "khr_swapchain")]
    pub get_device_group_surface_present_modes_khr: Option<extensions::khr_swapchain::PFN_vkGetDeviceGroupSurfacePresentModesKHR>,
    #[cfg(feature = "khr_swapchain")]
    pub acquire_next_image2_khr: Option<extensions::khr_swapchain::PFN_vkAcquireNextImage2KHR>,
    pub cmd_dispatch_base: Option<vk1_1::PFN_vkCmdDispatchBase>,
    pub create_descriptor_update_template: Option<vk1_1::PFN_vkCreateDescriptorUpdateTemplate>,
    pub destroy_descriptor_update_template: Option<vk1_1::PFN_vkDestroyDescriptorUpdateTemplate>,
    pub update_descriptor_set_with_template: Option<vk1_1::PFN_vkUpdateDescriptorSetWithTemplate>,
    #[cfg(feature = "khr_push_descriptor")]
    pub cmd_push_descriptor_set_with_template_khr: Option<extensions::khr_push_descriptor::PFN_vkCmdPushDescriptorSetWithTemplateKHR>,
    #[cfg(feature = "ext_hdr_metadata")]
    pub set_hdr_metadata_ext: Option<extensions::ext_hdr_metadata::PFN_vkSetHdrMetadataEXT>,
    #[cfg(feature = "khr_shared_presentable_image")]
    pub get_swapchain_status_khr: Option<extensions::khr_shared_presentable_image::PFN_vkGetSwapchainStatusKHR>,
    #[cfg(feature = "google_display_timing")]
    pub get_refresh_cycle_duration_google: Option<extensions::google_display_timing::PFN_vkGetRefreshCycleDurationGOOGLE>,
    #[cfg(feature = "google_display_timing")]
    pub get_past_presentation_timing_google: Option<extensions::google_display_timing::PFN_vkGetPastPresentationTimingGOOGLE>,
    #[cfg(feature = "nv_clip_space_w_scaling")]
    pub cmd_set_viewport_w_scaling_nv: Option<extensions::nv_clip_space_w_scaling::PFN_vkCmdSetViewportWScalingNV>,
    #[cfg(feature = "ext_discard_rectangles")]
    pub cmd_set_discard_rectangle_ext: Option<extensions::ext_discard_rectangles::PFN_vkCmdSetDiscardRectangleEXT>,
    #[cfg(feature = "ext_sample_locations")]
    pub cmd_set_sample_locations_ext: Option<extensions::ext_sample_locations::PFN_vkCmdSetSampleLocationsEXT>,
    pub get_buffer_memory_requirements2: Option<vk1_1::PFN_vkGetBufferMemoryRequirements2>,
    pub get_image_memory_requirements2: Option<vk1_1::PFN_vkGetImageMemoryRequirements2>,
    pub get_image_sparse_memory_requirements2: Option<vk1_1::PFN_vkGetImageSparseMemoryRequirements2>,
    pub create_sampler_ycbcr_conversion: Option<vk1_1::PFN_vkCreateSamplerYcbcrConversion>,
    pub destroy_sampler_ycbcr_conversion: Option<vk1_1::PFN_vkDestroySamplerYcbcrConversion>,
    pub get_device_queue2: Option<vk1_1::PFN_vkGetDeviceQueue2>,
    #[cfg(feature = "ext_validation_cache")]
    pub create_validation_cache_ext: Option<extensions::ext_validation_cache::PFN_vkCreateValidationCacheEXT>,
    #[cfg(feature = "ext_validation_cache")]
    pub destroy_validation_cache_ext: Option<extensions::ext_validation_cache::PFN_vkDestroyValidationCacheEXT>,
    #[cfg(feature = "ext_validation_cache")]
    pub get_validation_cache_data_ext: Option<extensions::ext_validation_cache::PFN_vkGetValidationCacheDataEXT>,
    #[cfg(feature = "ext_validation_cache")]
    pub merge_validation_caches_ext: Option<extensions::ext_validation_cache::PFN_vkMergeValidationCachesEXT>,
    pub get_descriptor_set_layout_support: Option<vk1_1::PFN_vkGetDescriptorSetLayoutSupport>,
    #[cfg(feature = "amd_shader_info")]
    pub get_shader_info_amd: Option<extensions::amd_shader_info::PFN_vkGetShaderInfoAMD>,
    #[cfg(feature = "amd_display_native_hdr")]
    pub set_local_dimming_amd: Option<extensions::amd_display_native_hdr::PFN_vkSetLocalDimmingAMD>,
    #[cfg(feature = "ext_calibrated_timestamps")]
    pub get_calibrated_timestamps_ext: Option<extensions::ext_calibrated_timestamps::PFN_vkGetCalibratedTimestampsEXT>,
    #[cfg(feature = "ext_debug_utils")]
    pub set_debug_utils_object_name_ext: Option<extensions::ext_debug_utils::PFN_vkSetDebugUtilsObjectNameEXT>,
    #[cfg(feature = "ext_debug_utils")]
    pub set_debug_utils_object_tag_ext: Option<extensions::ext_debug_utils::PFN_vkSetDebugUtilsObjectTagEXT>,
    #[cfg(feature = "ext_debug_utils")]
    pub queue_begin_debug_utils_label_ext: Option<extensions::ext_debug_utils::PFN_vkQueueBeginDebugUtilsLabelEXT>,
    #[cfg(feature = "ext_debug_utils")]
    pub queue_end_debug_utils_label_ext: Option<extensions::ext_debug_utils::PFN_vkQueueEndDebugUtilsLabelEXT>,
    #[cfg(feature = "ext_debug_utils")]
    pub queue_insert_debug_utils_label_ext: Option<extensions::ext_debug_utils::PFN_vkQueueInsertDebugUtilsLabelEXT>,
    #[cfg(feature = "ext_debug_utils")]
    pub cmd_begin_debug_utils_label_ext: Option<extensions::ext_debug_utils::PFN_vkCmdBeginDebugUtilsLabelEXT>,
    #[cfg(feature = "ext_debug_utils")]
    pub cmd_end_debug_utils_label_ext: Option<extensions::ext_debug_utils::PFN_vkCmdEndDebugUtilsLabelEXT>,
    #[cfg(feature = "ext_debug_utils")]
    pub cmd_insert_debug_utils_label_ext: Option<extensions::ext_debug_utils::PFN_vkCmdInsertDebugUtilsLabelEXT>,
    #[cfg(feature = "ext_external_memory_host")]
    pub get_memory_host_pointer_properties_ext: Option<extensions::ext_external_memory_host::PFN_vkGetMemoryHostPointerPropertiesEXT>,
    #[cfg(feature = "amd_buffer_marker")]
    pub cmd_write_buffer_marker_amd: Option<extensions::amd_buffer_marker::PFN_vkCmdWriteBufferMarkerAMD>,
    pub create_render_pass2: Option<vk1_2::PFN_vkCreateRenderPass2>,
    pub cmd_begin_render_pass2: Option<vk1_2::PFN_vkCmdBeginRenderPass2>,
    pub cmd_next_subpass2: Option<vk1_2::PFN_vkCmdNextSubpass2>,
    pub cmd_end_render_pass2: Option<vk1_2::PFN_vkCmdEndRenderPass2>,
    pub get_semaphore_counter_value: Option<vk1_2::PFN_vkGetSemaphoreCounterValue>,
    pub wait_semaphores: Option<vk1_2::PFN_vkWaitSemaphores>,
    pub signal_semaphore: Option<vk1_2::PFN_vkSignalSemaphore>,
    #[cfg(feature = "android_external_memory_android_hardware_buffer")]
    pub get_android_hardware_buffer_properties_android: Option<extensions::android_external_memory_android_hardware_buffer::PFN_vkGetAndroidHardwareBufferPropertiesANDROID>,
    #[cfg(feature = "android_external_memory_android_hardware_buffer")]
    pub get_memory_android_hardware_buffer_android: Option<extensions::android_external_memory_android_hardware_buffer::PFN_vkGetMemoryAndroidHardwareBufferANDROID>,
    pub cmd_draw_indirect_count: Option<vk1_2::PFN_vkCmdDrawIndirectCount>,
    pub cmd_draw_indexed_indirect_count: Option<vk1_2::PFN_vkCmdDrawIndexedIndirectCount>,
    #[cfg(feature = "nv_device_diagnostic_checkpoints")]
    pub cmd_set_checkpoint_nv: Option<extensions::nv_device_diagnostic_checkpoints::PFN_vkCmdSetCheckpointNV>,
    #[cfg(feature = "nv_device_diagnostic_checkpoints")]
    pub get_queue_checkpoint_data_nv: Option<extensions::nv_device_diagnostic_checkpoints::PFN_vkGetQueueCheckpointDataNV>,
    #[cfg(feature = "ext_transform_feedback")]
    pub cmd_bind_transform_feedback_buffers_ext: Option<extensions::ext_transform_feedback::PFN_vkCmdBindTransformFeedbackBuffersEXT>,
    #[cfg(feature = "ext_transform_feedback")]
    pub cmd_begin_transform_feedback_ext: Option<extensions::ext_transform_feedback::PFN_vkCmdBeginTransformFeedbackEXT>,
    #[cfg(feature = "ext_transform_feedback")]
    pub cmd_end_transform_feedback_ext: Option<extensions::ext_transform_feedback::PFN_vkCmdEndTransformFeedbackEXT>,
    #[cfg(feature = "ext_transform_feedback")]
    pub cmd_begin_query_indexed_ext: Option<extensions::ext_transform_feedback::PFN_vkCmdBeginQueryIndexedEXT>,
    #[cfg(feature = "ext_transform_feedback")]
    pub cmd_end_query_indexed_ext: Option<extensions::ext_transform_feedback::PFN_vkCmdEndQueryIndexedEXT>,
    #[cfg(feature = "ext_transform_feedback")]
    pub cmd_draw_indirect_byte_count_ext: Option<extensions::ext_transform_feedback::PFN_vkCmdDrawIndirectByteCountEXT>,
    #[cfg(feature = "nv_scissor_exclusive")]
    pub cmd_set_exclusive_scissor_nv: Option<extensions::nv_scissor_exclusive::PFN_vkCmdSetExclusiveScissorNV>,
    #[cfg(feature = "nv_shading_rate_image")]
    pub cmd_bind_shading_rate_image_nv: Option<extensions::nv_shading_rate_image::PFN_vkCmdBindShadingRateImageNV>,
    #[cfg(feature = "nv_shading_rate_image")]
    pub cmd_set_viewport_shading_rate_palette_nv: Option<extensions::nv_shading_rate_image::PFN_vkCmdSetViewportShadingRatePaletteNV>,
    #[cfg(feature = "nv_shading_rate_image")]
    pub cmd_set_coarse_sample_order_nv: Option<extensions::nv_shading_rate_image::PFN_vkCmdSetCoarseSampleOrderNV>,
    #[cfg(feature = "nv_mesh_shader")]
    pub cmd_draw_mesh_tasks_nv: Option<extensions::nv_mesh_shader::PFN_vkCmdDrawMeshTasksNV>,
    #[cfg(feature = "nv_mesh_shader")]
    pub cmd_draw_mesh_tasks_indirect_nv: Option<extensions::nv_mesh_shader::PFN_vkCmdDrawMeshTasksIndirectNV>,
    #[cfg(feature = "nv_mesh_shader")]
    pub cmd_draw_mesh_tasks_indirect_count_nv: Option<extensions::nv_mesh_shader::PFN_vkCmdDrawMeshTasksIndirectCountNV>,
    #[cfg(feature = "nv_ray_tracing")]
    pub compile_deferred_nv: Option<extensions::nv_ray_tracing::PFN_vkCompileDeferredNV>,
    #[cfg(feature = "nv_ray_tracing")]
    pub create_acceleration_structure_nv: Option<extensions::nv_ray_tracing::PFN_vkCreateAccelerationStructureNV>,
    #[cfg(feature = "khr_acceleration_structure")]
    pub destroy_acceleration_structure_khr: Option<extensions::khr_acceleration_structure::PFN_vkDestroyAccelerationStructureKHR>,
    #[cfg(feature = "nv_ray_tracing")]
    pub destroy_acceleration_structure_nv: Option<extensions::nv_ray_tracing::PFN_vkDestroyAccelerationStructureNV>,
    #[cfg(feature = "nv_ray_tracing")]
    pub get_acceleration_structure_memory_requirements_nv: Option<extensions::nv_ray_tracing::PFN_vkGetAccelerationStructureMemoryRequirementsNV>,
    #[cfg(feature = "nv_ray_tracing")]
    pub bind_acceleration_structure_memory_nv: Option<extensions::nv_ray_tracing::PFN_vkBindAccelerationStructureMemoryNV>,
    #[cfg(feature = "nv_ray_tracing")]
    pub cmd_copy_acceleration_structure_nv: Option<extensions::nv_ray_tracing::PFN_vkCmdCopyAccelerationStructureNV>,
    #[cfg(feature = "khr_acceleration_structure")]
    pub cmd_copy_acceleration_structure_khr: Option<extensions::khr_acceleration_structure::PFN_vkCmdCopyAccelerationStructureKHR>,
    #[cfg(feature = "khr_acceleration_structure")]
    pub copy_acceleration_structure_khr: Option<extensions::khr_acceleration_structure::PFN_vkCopyAccelerationStructureKHR>,
    #[cfg(feature = "khr_acceleration_structure")]
    pub cmd_copy_acceleration_structure_to_memory_khr: Option<extensions::khr_acceleration_structure::PFN_vkCmdCopyAccelerationStructureToMemoryKHR>,
    #[cfg(feature = "khr_acceleration_structure")]
    pub copy_acceleration_structure_to_memory_khr: Option<extensions::khr_acceleration_structure::PFN_vkCopyAccelerationStructureToMemoryKHR>,
    #[cfg(feature = "khr_acceleration_structure")]
    pub cmd_copy_memory_to_acceleration_structure_khr: Option<extensions::khr_acceleration_structure::PFN_vkCmdCopyMemoryToAccelerationStructureKHR>,
    #[cfg(feature = "khr_acceleration_structure")]
    pub copy_memory_to_acceleration_structure_khr: Option<extensions::khr_acceleration_structure::PFN_vkCopyMemoryToAccelerationStructureKHR>,
    #[cfg(feature = "khr_acceleration_structure")]
    pub cmd_write_acceleration_structures_properties_khr: Option<extensions::khr_acceleration_structure::PFN_vkCmdWriteAccelerationStructuresPropertiesKHR>,
    #[cfg(feature = "nv_ray_tracing")]
    pub cmd_write_acceleration_structures_properties_nv: Option<extensions::nv_ray_tracing::PFN_vkCmdWriteAccelerationStructuresPropertiesNV>,
    #[cfg(feature = "nv_ray_tracing")]
    pub cmd_build_acceleration_structure_nv: Option<extensions::nv_ray_tracing::PFN_vkCmdBuildAccelerationStructureNV>,
    #[cfg(feature = "khr_acceleration_structure")]
    pub write_acceleration_structures_properties_khr: Option<extensions::khr_acceleration_structure::PFN_vkWriteAccelerationStructuresPropertiesKHR>,
    #[cfg(feature = "khr_ray_tracing_pipeline")]
    pub cmd_trace_rays_khr: Option<extensions::khr_ray_tracing_pipeline::PFN_vkCmdTraceRaysKHR>,
    #[cfg(feature = "nv_ray_tracing")]
    pub cmd_trace_rays_nv: Option<extensions::nv_ray_tracing::PFN_vkCmdTraceRaysNV>,
    #[cfg(feature = "khr_ray_tracing_pipeline")]
    pub get_ray_tracing_shader_group_handles_khr: Option<extensions::khr_ray_tracing_pipeline::PFN_vkGetRayTracingShaderGroupHandlesKHR>,
    #[cfg(feature = "khr_ray_tracing_pipeline")]
    pub get_ray_tracing_capture_replay_shader_group_handles_khr: Option<extensions::khr_ray_tracing_pipeline::PFN_vkGetRayTracingCaptureReplayShaderGroupHandlesKHR>,
    #[cfg(feature = "nv_ray_tracing")]
    pub get_acceleration_structure_handle_nv: Option<extensions::nv_ray_tracing::PFN_vkGetAccelerationStructureHandleNV>,
    #[cfg(feature = "nv_ray_tracing")]
    pub create_ray_tracing_pipelines_nv: Option<extensions::nv_ray_tracing::PFN_vkCreateRayTracingPipelinesNV>,
    #[cfg(feature = "khr_ray_tracing_pipeline")]
    pub create_ray_tracing_pipelines_khr: Option<extensions::khr_ray_tracing_pipeline::PFN_vkCreateRayTracingPipelinesKHR>,
    #[cfg(feature = "khr_ray_tracing_pipeline")]
    pub cmd_trace_rays_indirect_khr: Option<extensions::khr_ray_tracing_pipeline::PFN_vkCmdTraceRaysIndirectKHR>,
    #[cfg(feature = "khr_acceleration_structure")]
    pub get_device_acceleration_structure_compatibility_khr: Option<extensions::khr_acceleration_structure::PFN_vkGetDeviceAccelerationStructureCompatibilityKHR>,
    #[cfg(feature = "khr_ray_tracing_pipeline")]
    pub get_ray_tracing_shader_group_stack_size_khr: Option<extensions::khr_ray_tracing_pipeline::PFN_vkGetRayTracingShaderGroupStackSizeKHR>,
    #[cfg(feature = "khr_ray_tracing_pipeline")]
    pub cmd_set_ray_tracing_pipeline_stack_size_khr: Option<extensions::khr_ray_tracing_pipeline::PFN_vkCmdSetRayTracingPipelineStackSizeKHR>,
    #[cfg(feature = "nvx_image_view_handle")]
    pub get_image_view_handle_nvx: Option<extensions::nvx_image_view_handle::PFN_vkGetImageViewHandleNVX>,
    #[cfg(feature = "nvx_image_view_handle")]
    pub get_image_view_address_nvx: Option<extensions::nvx_image_view_handle::PFN_vkGetImageViewAddressNVX>,
    #[cfg(feature = "ext_full_screen_exclusive")]
    pub get_device_group_surface_present_modes2_ext: Option<extensions::ext_full_screen_exclusive::PFN_vkGetDeviceGroupSurfacePresentModes2EXT>,
    #[cfg(feature = "ext_full_screen_exclusive")]
    pub acquire_full_screen_exclusive_mode_ext: Option<extensions::ext_full_screen_exclusive::PFN_vkAcquireFullScreenExclusiveModeEXT>,
    #[cfg(feature = "ext_full_screen_exclusive")]
    pub release_full_screen_exclusive_mode_ext: Option<extensions::ext_full_screen_exclusive::PFN_vkReleaseFullScreenExclusiveModeEXT>,
    #[cfg(feature = "khr_performance_query")]
    pub acquire_profiling_lock_khr: Option<extensions::khr_performance_query::PFN_vkAcquireProfilingLockKHR>,
    #[cfg(feature = "khr_performance_query")]
    pub release_profiling_lock_khr: Option<extensions::khr_performance_query::PFN_vkReleaseProfilingLockKHR>,
    #[cfg(feature = "ext_image_drm_format_modifier")]
    pub get_image_drm_format_modifier_properties_ext: Option<extensions::ext_image_drm_format_modifier::PFN_vkGetImageDrmFormatModifierPropertiesEXT>,
    pub get_buffer_opaque_capture_address: Option<vk1_2::PFN_vkGetBufferOpaqueCaptureAddress>,
    pub get_buffer_device_address: Option<vk1_2::PFN_vkGetBufferDeviceAddress>,
    #[cfg(feature = "intel_performance_query")]
    pub initialize_performance_api_intel: Option<extensions::intel_performance_query::PFN_vkInitializePerformanceApiINTEL>,
    #[cfg(feature = "intel_performance_query")]
    pub uninitialize_performance_api_intel: Option<extensions::intel_performance_query::PFN_vkUninitializePerformanceApiINTEL>,
    #[cfg(feature = "intel_performance_query")]
    pub cmd_set_performance_marker_intel: Option<extensions::intel_performance_query::PFN_vkCmdSetPerformanceMarkerINTEL>,
    #[cfg(feature = "intel_performance_query")]
    pub cmd_set_performance_stream_marker_intel: Option<extensions::intel_performance_query::PFN_vkCmdSetPerformanceStreamMarkerINTEL>,
    #[cfg(feature = "intel_performance_query")]
    pub cmd_set_performance_override_intel: Option<extensions::intel_performance_query::PFN_vkCmdSetPerformanceOverrideINTEL>,
    #[cfg(feature = "intel_performance_query")]
    pub acquire_performance_configuration_intel: Option<extensions::intel_performance_query::PFN_vkAcquirePerformanceConfigurationINTEL>,
    #[cfg(feature = "intel_performance_query")]
    pub release_performance_configuration_intel: Option<extensions::intel_performance_query::PFN_vkReleasePerformanceConfigurationINTEL>,
    #[cfg(feature = "intel_performance_query")]
    pub queue_set_performance_configuration_intel: Option<extensions::intel_performance_query::PFN_vkQueueSetPerformanceConfigurationINTEL>,
    #[cfg(feature = "intel_performance_query")]
    pub get_performance_parameter_intel: Option<extensions::intel_performance_query::PFN_vkGetPerformanceParameterINTEL>,
    pub get_device_memory_opaque_capture_address: Option<vk1_2::PFN_vkGetDeviceMemoryOpaqueCaptureAddress>,
    #[cfg(feature = "khr_pipeline_executable_properties")]
    pub get_pipeline_executable_properties_khr: Option<extensions::khr_pipeline_executable_properties::PFN_vkGetPipelineExecutablePropertiesKHR>,
    #[cfg(feature = "khr_pipeline_executable_properties")]
    pub get_pipeline_executable_statistics_khr: Option<extensions::khr_pipeline_executable_properties::PFN_vkGetPipelineExecutableStatisticsKHR>,
    #[cfg(feature = "khr_pipeline_executable_properties")]
    pub get_pipeline_executable_internal_representations_khr: Option<extensions::khr_pipeline_executable_properties::PFN_vkGetPipelineExecutableInternalRepresentationsKHR>,
    #[cfg(feature = "ext_line_rasterization")]
    pub cmd_set_line_stipple_ext: Option<extensions::ext_line_rasterization::PFN_vkCmdSetLineStippleEXT>,
    #[cfg(feature = "khr_acceleration_structure")]
    pub create_acceleration_structure_khr: Option<extensions::khr_acceleration_structure::PFN_vkCreateAccelerationStructureKHR>,
    #[cfg(feature = "khr_acceleration_structure")]
    pub cmd_build_acceleration_structures_khr: Option<extensions::khr_acceleration_structure::PFN_vkCmdBuildAccelerationStructuresKHR>,
    #[cfg(feature = "khr_acceleration_structure")]
    pub cmd_build_acceleration_structures_indirect_khr: Option<extensions::khr_acceleration_structure::PFN_vkCmdBuildAccelerationStructuresIndirectKHR>,
    #[cfg(feature = "khr_acceleration_structure")]
    pub build_acceleration_structures_khr: Option<extensions::khr_acceleration_structure::PFN_vkBuildAccelerationStructuresKHR>,
    #[cfg(feature = "khr_acceleration_structure")]
    pub get_acceleration_structure_device_address_khr: Option<extensions::khr_acceleration_structure::PFN_vkGetAccelerationStructureDeviceAddressKHR>,
    #[cfg(feature = "khr_deferred_host_operations")]
    pub create_deferred_operation_khr: Option<extensions::khr_deferred_host_operations::PFN_vkCreateDeferredOperationKHR>,
    #[cfg(feature = "khr_deferred_host_operations")]
    pub destroy_deferred_operation_khr: Option<extensions::khr_deferred_host_operations::PFN_vkDestroyDeferredOperationKHR>,
    #[cfg(feature = "khr_deferred_host_operations")]
    pub get_deferred_operation_max_concurrency_khr: Option<extensions::khr_deferred_host_operations::PFN_vkGetDeferredOperationMaxConcurrencyKHR>,
    #[cfg(feature = "khr_deferred_host_operations")]
    pub get_deferred_operation_result_khr: Option<extensions::khr_deferred_host_operations::PFN_vkGetDeferredOperationResultKHR>,
    #[cfg(feature = "khr_deferred_host_operations")]
    pub deferred_operation_join_khr: Option<extensions::khr_deferred_host_operations::PFN_vkDeferredOperationJoinKHR>,
    #[cfg(feature = "ext_extended_dynamic_state")]
    pub cmd_set_cull_mode_ext: Option<extensions::ext_extended_dynamic_state::PFN_vkCmdSetCullModeEXT>,
    #[cfg(feature = "ext_extended_dynamic_state")]
    pub cmd_set_front_face_ext: Option<extensions::ext_extended_dynamic_state::PFN_vkCmdSetFrontFaceEXT>,
    #[cfg(feature = "ext_extended_dynamic_state")]
    pub cmd_set_primitive_topology_ext: Option<extensions::ext_extended_dynamic_state::PFN_vkCmdSetPrimitiveTopologyEXT>,
    #[cfg(feature = "ext_extended_dynamic_state")]
    pub cmd_set_viewport_with_count_ext: Option<extensions::ext_extended_dynamic_state::PFN_vkCmdSetViewportWithCountEXT>,
    #[cfg(feature = "ext_extended_dynamic_state")]
    pub cmd_set_scissor_with_count_ext: Option<extensions::ext_extended_dynamic_state::PFN_vkCmdSetScissorWithCountEXT>,
    #[cfg(feature = "ext_extended_dynamic_state")]
    pub cmd_bind_vertex_buffers2_ext: Option<extensions::ext_extended_dynamic_state::PFN_vkCmdBindVertexBuffers2EXT>,
    #[cfg(feature = "ext_extended_dynamic_state")]
    pub cmd_set_depth_test_enable_ext: Option<extensions::ext_extended_dynamic_state::PFN_vkCmdSetDepthTestEnableEXT>,
    #[cfg(feature = "ext_extended_dynamic_state")]
    pub cmd_set_depth_write_enable_ext: Option<extensions::ext_extended_dynamic_state::PFN_vkCmdSetDepthWriteEnableEXT>,
    #[cfg(feature = "ext_extended_dynamic_state")]
    pub cmd_set_depth_compare_op_ext: Option<extensions::ext_extended_dynamic_state::PFN_vkCmdSetDepthCompareOpEXT>,
    #[cfg(feature = "ext_extended_dynamic_state")]
    pub cmd_set_depth_bounds_test_enable_ext: Option<extensions::ext_extended_dynamic_state::PFN_vkCmdSetDepthBoundsTestEnableEXT>,
    #[cfg(feature = "ext_extended_dynamic_state")]
    pub cmd_set_stencil_test_enable_ext: Option<extensions::ext_extended_dynamic_state::PFN_vkCmdSetStencilTestEnableEXT>,
    #[cfg(feature = "ext_extended_dynamic_state")]
    pub cmd_set_stencil_op_ext: Option<extensions::ext_extended_dynamic_state::PFN_vkCmdSetStencilOpEXT>,
    #[cfg(feature = "ext_extended_dynamic_state2")]
    pub cmd_set_patch_control_points_ext: Option<extensions::ext_extended_dynamic_state2::PFN_vkCmdSetPatchControlPointsEXT>,
    #[cfg(feature = "ext_extended_dynamic_state2")]
    pub cmd_set_rasterizer_discard_enable_ext: Option<extensions::ext_extended_dynamic_state2::PFN_vkCmdSetRasterizerDiscardEnableEXT>,
    #[cfg(feature = "ext_extended_dynamic_state2")]
    pub cmd_set_depth_bias_enable_ext: Option<extensions::ext_extended_dynamic_state2::PFN_vkCmdSetDepthBiasEnableEXT>,
    #[cfg(feature = "ext_extended_dynamic_state2")]
    pub cmd_set_logic_op_ext: Option<extensions::ext_extended_dynamic_state2::PFN_vkCmdSetLogicOpEXT>,
    #[cfg(feature = "ext_extended_dynamic_state2")]
    pub cmd_set_primitive_restart_enable_ext: Option<extensions::ext_extended_dynamic_state2::PFN_vkCmdSetPrimitiveRestartEnableEXT>,
    #[cfg(feature = "ext_private_data")]
    pub create_private_data_slot_ext: Option<extensions::ext_private_data::PFN_vkCreatePrivateDataSlotEXT>,
    #[cfg(feature = "ext_private_data")]
    pub destroy_private_data_slot_ext: Option<extensions::ext_private_data::PFN_vkDestroyPrivateDataSlotEXT>,
    #[cfg(feature = "ext_private_data")]
    pub set_private_data_ext: Option<extensions::ext_private_data::PFN_vkSetPrivateDataEXT>,
    #[cfg(feature = "ext_private_data")]
    pub get_private_data_ext: Option<extensions::ext_private_data::PFN_vkGetPrivateDataEXT>,
    #[cfg(feature = "khr_copy_commands2")]
    pub cmd_copy_buffer2_khr: Option<extensions::khr_copy_commands2::PFN_vkCmdCopyBuffer2KHR>,
    #[cfg(feature = "khr_copy_commands2")]
    pub cmd_copy_image2_khr: Option<extensions::khr_copy_commands2::PFN_vkCmdCopyImage2KHR>,
    #[cfg(feature = "khr_copy_commands2")]
    pub cmd_blit_image2_khr: Option<extensions::khr_copy_commands2::PFN_vkCmdBlitImage2KHR>,
    #[cfg(feature = "khr_copy_commands2")]
    pub cmd_copy_buffer_to_image2_khr: Option<extensions::khr_copy_commands2::PFN_vkCmdCopyBufferToImage2KHR>,
    #[cfg(feature = "khr_copy_commands2")]
    pub cmd_copy_image_to_buffer2_khr: Option<extensions::khr_copy_commands2::PFN_vkCmdCopyImageToBuffer2KHR>,
    #[cfg(feature = "khr_copy_commands2")]
    pub cmd_resolve_image2_khr: Option<extensions::khr_copy_commands2::PFN_vkCmdResolveImage2KHR>,
    #[cfg(feature = "khr_fragment_shading_rate")]
    pub cmd_set_fragment_shading_rate_khr: Option<extensions::khr_fragment_shading_rate::PFN_vkCmdSetFragmentShadingRateKHR>,
    #[cfg(feature = "nv_fragment_shading_rate_enums")]
    pub cmd_set_fragment_shading_rate_enum_nv: Option<extensions::nv_fragment_shading_rate_enums::PFN_vkCmdSetFragmentShadingRateEnumNV>,
    #[cfg(feature = "khr_acceleration_structure")]
    pub get_acceleration_structure_build_sizes_khr: Option<extensions::khr_acceleration_structure::PFN_vkGetAccelerationStructureBuildSizesKHR>,
    #[cfg(feature = "ext_vertex_input_dynamic_state")]
    pub cmd_set_vertex_input_ext: Option<extensions::ext_vertex_input_dynamic_state::PFN_vkCmdSetVertexInputEXT>,
    #[cfg(feature = "ext_color_write_enable")]
    pub cmd_set_color_write_enable_ext: Option<extensions::ext_color_write_enable::PFN_vkCmdSetColorWriteEnableEXT>,
    #[cfg(feature = "khr_synchronization2")]
    pub cmd_set_event2_khr: Option<extensions::khr_synchronization2::PFN_vkCmdSetEvent2KHR>,
    #[cfg(feature = "khr_synchronization2")]
    pub cmd_reset_event2_khr: Option<extensions::khr_synchronization2::PFN_vkCmdResetEvent2KHR>,
    #[cfg(feature = "khr_synchronization2")]
    pub cmd_wait_events2_khr: Option<extensions::khr_synchronization2::PFN_vkCmdWaitEvents2KHR>,
    #[cfg(feature = "khr_synchronization2")]
    pub cmd_pipeline_barrier2_khr: Option<extensions::khr_synchronization2::PFN_vkCmdPipelineBarrier2KHR>,
    #[cfg(feature = "khr_synchronization2")]
    pub queue_submit2_khr: Option<extensions::khr_synchronization2::PFN_vkQueueSubmit2KHR>,
    #[cfg(feature = "khr_synchronization2")]
    pub cmd_write_timestamp2_khr: Option<extensions::khr_synchronization2::PFN_vkCmdWriteTimestamp2KHR>,
    #[cfg(feature = "khr_synchronization2")]
    pub cmd_write_buffer_marker2_amd: Option<extensions::khr_synchronization2::PFN_vkCmdWriteBufferMarker2AMD>,
    #[cfg(feature = "khr_synchronization2")]
    pub get_queue_checkpoint_data2_nv: Option<extensions::khr_synchronization2::PFN_vkGetQueueCheckpointData2NV>,
    #[cfg(feature = "khr_video_queue")]
    pub create_video_session_khr: Option<extensions::khr_video_queue::PFN_vkCreateVideoSessionKHR>,
    #[cfg(feature = "khr_video_queue")]
    pub destroy_video_session_khr: Option<extensions::khr_video_queue::PFN_vkDestroyVideoSessionKHR>,
    #[cfg(feature = "khr_video_queue")]
    pub create_video_session_parameters_khr: Option<extensions::khr_video_queue::PFN_vkCreateVideoSessionParametersKHR>,
    #[cfg(feature = "khr_video_queue")]
    pub update_video_session_parameters_khr: Option<extensions::khr_video_queue::PFN_vkUpdateVideoSessionParametersKHR>,
    #[cfg(feature = "khr_video_queue")]
    pub destroy_video_session_parameters_khr: Option<extensions::khr_video_queue::PFN_vkDestroyVideoSessionParametersKHR>,
    #[cfg(feature = "khr_video_queue")]
    pub get_video_session_memory_requirements_khr: Option<extensions::khr_video_queue::PFN_vkGetVideoSessionMemoryRequirementsKHR>,
    #[cfg(feature = "khr_video_queue")]
    pub bind_video_session_memory_khr: Option<extensions::khr_video_queue::PFN_vkBindVideoSessionMemoryKHR>,
    #[cfg(feature = "khr_video_decode_queue")]
    pub cmd_decode_video_khr: Option<extensions::khr_video_decode_queue::PFN_vkCmdDecodeVideoKHR>,
    #[cfg(feature = "khr_video_queue")]
    pub cmd_begin_video_coding_khr: Option<extensions::khr_video_queue::PFN_vkCmdBeginVideoCodingKHR>,
    #[cfg(feature = "khr_video_queue")]
    pub cmd_control_video_coding_khr: Option<extensions::khr_video_queue::PFN_vkCmdControlVideoCodingKHR>,
    #[cfg(feature = "khr_video_queue")]
    pub cmd_end_video_coding_khr: Option<extensions::khr_video_queue::PFN_vkCmdEndVideoCodingKHR>,
    #[cfg(feature = "khr_video_encode_queue")]
    pub cmd_encode_video_khr: Option<extensions::khr_video_encode_queue::PFN_vkCmdEncodeVideoKHR>,
    #[cfg(feature = "nvx_binary_import")]
    pub create_cu_module_nvx: Option<extensions::nvx_binary_import::PFN_vkCreateCuModuleNVX>,
    #[cfg(feature = "nvx_binary_import")]
    pub create_cu_function_nvx: Option<extensions::nvx_binary_import::PFN_vkCreateCuFunctionNVX>,
    #[cfg(feature = "nvx_binary_import")]
    pub destroy_cu_module_nvx: Option<extensions::nvx_binary_import::PFN_vkDestroyCuModuleNVX>,
    #[cfg(feature = "nvx_binary_import")]
    pub destroy_cu_function_nvx: Option<extensions::nvx_binary_import::PFN_vkDestroyCuFunctionNVX>,
    #[cfg(feature = "nvx_binary_import")]
    pub cmd_cu_launch_kernel_nvx: Option<extensions::nvx_binary_import::PFN_vkCmdCuLaunchKernelNVX>,
    #[cfg(feature = "ext_host_query_reset")]
    pub reset_query_pool_ext: Option<extensions::ext_host_query_reset::PFN_vkResetQueryPoolEXT>,
    #[cfg(feature = "khr_maintenance1")]
    pub trim_command_pool_khr: Option<extensions::khr_maintenance1::PFN_vkTrimCommandPoolKHR>,
    #[cfg(feature = "khr_device_group")]
    pub get_device_group_peer_memory_features_khr: Option<extensions::khr_device_group::PFN_vkGetDeviceGroupPeerMemoryFeaturesKHR>,
    #[cfg(feature = "khr_bind_memory2")]
    pub bind_buffer_memory2_khr: Option<extensions::khr_bind_memory2::PFN_vkBindBufferMemory2KHR>,
    #[cfg(feature = "khr_bind_memory2")]
    pub bind_image_memory2_khr: Option<extensions::khr_bind_memory2::PFN_vkBindImageMemory2KHR>,
    #[cfg(feature = "khr_device_group")]
    pub cmd_set_device_mask_khr: Option<extensions::khr_device_group::PFN_vkCmdSetDeviceMaskKHR>,
    #[cfg(feature = "khr_device_group")]
    pub cmd_dispatch_base_khr: Option<extensions::khr_device_group::PFN_vkCmdDispatchBaseKHR>,
    #[cfg(feature = "khr_descriptor_update_template")]
    pub create_descriptor_update_template_khr: Option<extensions::khr_descriptor_update_template::PFN_vkCreateDescriptorUpdateTemplateKHR>,
    #[cfg(feature = "khr_descriptor_update_template")]
    pub destroy_descriptor_update_template_khr: Option<extensions::khr_descriptor_update_template::PFN_vkDestroyDescriptorUpdateTemplateKHR>,
    #[cfg(feature = "khr_descriptor_update_template")]
    pub update_descriptor_set_with_template_khr: Option<extensions::khr_descriptor_update_template::PFN_vkUpdateDescriptorSetWithTemplateKHR>,
    #[cfg(feature = "khr_get_memory_requirements2")]
    pub get_buffer_memory_requirements2_khr: Option<extensions::khr_get_memory_requirements2::PFN_vkGetBufferMemoryRequirements2KHR>,
    #[cfg(feature = "khr_get_memory_requirements2")]
    pub get_image_memory_requirements2_khr: Option<extensions::khr_get_memory_requirements2::PFN_vkGetImageMemoryRequirements2KHR>,
    #[cfg(feature = "khr_get_memory_requirements2")]
    pub get_image_sparse_memory_requirements2_khr: Option<extensions::khr_get_memory_requirements2::PFN_vkGetImageSparseMemoryRequirements2KHR>,
    #[cfg(feature = "khr_sampler_ycbcr_conversion")]
    pub create_sampler_ycbcr_conversion_khr: Option<extensions::khr_sampler_ycbcr_conversion::PFN_vkCreateSamplerYcbcrConversionKHR>,
    #[cfg(feature = "khr_sampler_ycbcr_conversion")]
    pub destroy_sampler_ycbcr_conversion_khr: Option<extensions::khr_sampler_ycbcr_conversion::PFN_vkDestroySamplerYcbcrConversionKHR>,
    #[cfg(feature = "khr_maintenance3")]
    pub get_descriptor_set_layout_support_khr: Option<extensions::khr_maintenance3::PFN_vkGetDescriptorSetLayoutSupportKHR>,
    #[cfg(feature = "khr_create_renderpass2")]
    pub create_render_pass2_khr: Option<extensions::khr_create_renderpass2::PFN_vkCreateRenderPass2KHR>,
    #[cfg(feature = "khr_create_renderpass2")]
    pub cmd_begin_render_pass2_khr: Option<extensions::khr_create_renderpass2::PFN_vkCmdBeginRenderPass2KHR>,
    #[cfg(feature = "khr_create_renderpass2")]
    pub cmd_next_subpass2_khr: Option<extensions::khr_create_renderpass2::PFN_vkCmdNextSubpass2KHR>,
    #[cfg(feature = "khr_create_renderpass2")]
    pub cmd_end_render_pass2_khr: Option<extensions::khr_create_renderpass2::PFN_vkCmdEndRenderPass2KHR>,
    #[cfg(feature = "khr_timeline_semaphore")]
    pub get_semaphore_counter_value_khr: Option<extensions::khr_timeline_semaphore::PFN_vkGetSemaphoreCounterValueKHR>,
    #[cfg(feature = "khr_timeline_semaphore")]
    pub wait_semaphores_khr: Option<extensions::khr_timeline_semaphore::PFN_vkWaitSemaphoresKHR>,
    #[cfg(feature = "khr_timeline_semaphore")]
    pub signal_semaphore_khr: Option<extensions::khr_timeline_semaphore::PFN_vkSignalSemaphoreKHR>,
    #[cfg(feature = "khr_draw_indirect_count")]
    pub cmd_draw_indirect_count_khr: Option<extensions::khr_draw_indirect_count::PFN_vkCmdDrawIndirectCountKHR>,
    #[cfg(feature = "amd_draw_indirect_count")]
    pub cmd_draw_indirect_count_amd: Option<extensions::amd_draw_indirect_count::PFN_vkCmdDrawIndirectCountAMD>,
    #[cfg(feature = "khr_draw_indirect_count")]
    pub cmd_draw_indexed_indirect_count_khr: Option<extensions::khr_draw_indirect_count::PFN_vkCmdDrawIndexedIndirectCountKHR>,
    #[cfg(feature = "amd_draw_indirect_count")]
    pub cmd_draw_indexed_indirect_count_amd: Option<extensions::amd_draw_indirect_count::PFN_vkCmdDrawIndexedIndirectCountAMD>,
    #[cfg(feature = "nv_ray_tracing")]
    pub get_ray_tracing_shader_group_handles_nv: Option<extensions::nv_ray_tracing::PFN_vkGetRayTracingShaderGroupHandlesNV>,
    #[cfg(feature = "khr_buffer_device_address")]
    pub get_buffer_opaque_capture_address_khr: Option<extensions::khr_buffer_device_address::PFN_vkGetBufferOpaqueCaptureAddressKHR>,
    #[cfg(feature = "khr_buffer_device_address")]
    pub get_buffer_device_address_khr: Option<extensions::khr_buffer_device_address::PFN_vkGetBufferDeviceAddressKHR>,
    #[cfg(feature = "ext_buffer_device_address")]
    pub get_buffer_device_address_ext: Option<extensions::ext_buffer_device_address::PFN_vkGetBufferDeviceAddressEXT>,
    #[cfg(feature = "khr_buffer_device_address")]
    pub get_device_memory_opaque_capture_address_khr: Option<extensions::khr_buffer_device_address::PFN_vkGetDeviceMemoryOpaqueCaptureAddressKHR>,
}
impl DeviceLoader {
    #[allow(unused_parens)]
    pub unsafe fn custom(instance_loader: &InstanceLoader, device: crate::vk1_0::Device, device_enabled: DeviceEnabled, mut symbol: impl FnMut(*const std::os::raw::c_char) -> Option<crate::vk1_0::PFN_vkVoidFunction>) -> Result<DeviceLoader, crate::LoaderError> {
        let instance_enabled = &instance_loader.enabled;
        Ok(DeviceLoader {
            parent: std::sync::Arc::downgrade(&instance_loader.arc),
            handle: device,
            get_device_proc_addr: std::mem::transmute(symbol(crate::vk1_0::FN_GET_DEVICE_PROC_ADDR)),
            destroy_device: std::mem::transmute(symbol(crate::vk1_0::FN_DESTROY_DEVICE)),
            get_device_queue: std::mem::transmute(symbol(crate::vk1_0::FN_GET_DEVICE_QUEUE)),
            queue_submit: std::mem::transmute(symbol(crate::vk1_0::FN_QUEUE_SUBMIT)),
            queue_wait_idle: std::mem::transmute(symbol(crate::vk1_0::FN_QUEUE_WAIT_IDLE)),
            device_wait_idle: std::mem::transmute(symbol(crate::vk1_0::FN_DEVICE_WAIT_IDLE)),
            allocate_memory: std::mem::transmute(symbol(crate::vk1_0::FN_ALLOCATE_MEMORY)),
            free_memory: std::mem::transmute(symbol(crate::vk1_0::FN_FREE_MEMORY)),
            map_memory: std::mem::transmute(symbol(crate::vk1_0::FN_MAP_MEMORY)),
            unmap_memory: std::mem::transmute(symbol(crate::vk1_0::FN_UNMAP_MEMORY)),
            flush_mapped_memory_ranges: std::mem::transmute(symbol(crate::vk1_0::FN_FLUSH_MAPPED_MEMORY_RANGES)),
            invalidate_mapped_memory_ranges: std::mem::transmute(symbol(crate::vk1_0::FN_INVALIDATE_MAPPED_MEMORY_RANGES)),
            get_device_memory_commitment: std::mem::transmute(symbol(crate::vk1_0::FN_GET_DEVICE_MEMORY_COMMITMENT)),
            get_buffer_memory_requirements: std::mem::transmute(symbol(crate::vk1_0::FN_GET_BUFFER_MEMORY_REQUIREMENTS)),
            bind_buffer_memory: std::mem::transmute(symbol(crate::vk1_0::FN_BIND_BUFFER_MEMORY)),
            get_image_memory_requirements: std::mem::transmute(symbol(crate::vk1_0::FN_GET_IMAGE_MEMORY_REQUIREMENTS)),
            bind_image_memory: std::mem::transmute(symbol(crate::vk1_0::FN_BIND_IMAGE_MEMORY)),
            get_image_sparse_memory_requirements: std::mem::transmute(symbol(crate::vk1_0::FN_GET_IMAGE_SPARSE_MEMORY_REQUIREMENTS)),
            queue_bind_sparse: std::mem::transmute(symbol(crate::vk1_0::FN_QUEUE_BIND_SPARSE)),
            create_fence: std::mem::transmute(symbol(crate::vk1_0::FN_CREATE_FENCE)),
            destroy_fence: std::mem::transmute(symbol(crate::vk1_0::FN_DESTROY_FENCE)),
            reset_fences: std::mem::transmute(symbol(crate::vk1_0::FN_RESET_FENCES)),
            get_fence_status: std::mem::transmute(symbol(crate::vk1_0::FN_GET_FENCE_STATUS)),
            wait_for_fences: std::mem::transmute(symbol(crate::vk1_0::FN_WAIT_FOR_FENCES)),
            create_semaphore: std::mem::transmute(symbol(crate::vk1_0::FN_CREATE_SEMAPHORE)),
            destroy_semaphore: std::mem::transmute(symbol(crate::vk1_0::FN_DESTROY_SEMAPHORE)),
            create_event: std::mem::transmute(symbol(crate::vk1_0::FN_CREATE_EVENT)),
            destroy_event: std::mem::transmute(symbol(crate::vk1_0::FN_DESTROY_EVENT)),
            get_event_status: std::mem::transmute(symbol(crate::vk1_0::FN_GET_EVENT_STATUS)),
            set_event: std::mem::transmute(symbol(crate::vk1_0::FN_SET_EVENT)),
            reset_event: std::mem::transmute(symbol(crate::vk1_0::FN_RESET_EVENT)),
            create_query_pool: std::mem::transmute(symbol(crate::vk1_0::FN_CREATE_QUERY_POOL)),
            destroy_query_pool: std::mem::transmute(symbol(crate::vk1_0::FN_DESTROY_QUERY_POOL)),
            get_query_pool_results: std::mem::transmute(symbol(crate::vk1_0::FN_GET_QUERY_POOL_RESULTS)),
            reset_query_pool: if instance_enabled.vk1_2 { std::mem::transmute(symbol(crate::vk1_2::FN_RESET_QUERY_POOL)) } else { None },
            create_buffer: std::mem::transmute(symbol(crate::vk1_0::FN_CREATE_BUFFER)),
            destroy_buffer: std::mem::transmute(symbol(crate::vk1_0::FN_DESTROY_BUFFER)),
            create_buffer_view: std::mem::transmute(symbol(crate::vk1_0::FN_CREATE_BUFFER_VIEW)),
            destroy_buffer_view: std::mem::transmute(symbol(crate::vk1_0::FN_DESTROY_BUFFER_VIEW)),
            create_image: std::mem::transmute(symbol(crate::vk1_0::FN_CREATE_IMAGE)),
            destroy_image: std::mem::transmute(symbol(crate::vk1_0::FN_DESTROY_IMAGE)),
            get_image_subresource_layout: std::mem::transmute(symbol(crate::vk1_0::FN_GET_IMAGE_SUBRESOURCE_LAYOUT)),
            create_image_view: std::mem::transmute(symbol(crate::vk1_0::FN_CREATE_IMAGE_VIEW)),
            destroy_image_view: std::mem::transmute(symbol(crate::vk1_0::FN_DESTROY_IMAGE_VIEW)),
            create_shader_module: std::mem::transmute(symbol(crate::vk1_0::FN_CREATE_SHADER_MODULE)),
            destroy_shader_module: std::mem::transmute(symbol(crate::vk1_0::FN_DESTROY_SHADER_MODULE)),
            create_pipeline_cache: std::mem::transmute(symbol(crate::vk1_0::FN_CREATE_PIPELINE_CACHE)),
            destroy_pipeline_cache: std::mem::transmute(symbol(crate::vk1_0::FN_DESTROY_PIPELINE_CACHE)),
            get_pipeline_cache_data: std::mem::transmute(symbol(crate::vk1_0::FN_GET_PIPELINE_CACHE_DATA)),
            merge_pipeline_caches: std::mem::transmute(symbol(crate::vk1_0::FN_MERGE_PIPELINE_CACHES)),
            create_graphics_pipelines: std::mem::transmute(symbol(crate::vk1_0::FN_CREATE_GRAPHICS_PIPELINES)),
            create_compute_pipelines: std::mem::transmute(symbol(crate::vk1_0::FN_CREATE_COMPUTE_PIPELINES)),
            destroy_pipeline: std::mem::transmute(symbol(crate::vk1_0::FN_DESTROY_PIPELINE)),
            create_pipeline_layout: std::mem::transmute(symbol(crate::vk1_0::FN_CREATE_PIPELINE_LAYOUT)),
            destroy_pipeline_layout: std::mem::transmute(symbol(crate::vk1_0::FN_DESTROY_PIPELINE_LAYOUT)),
            create_sampler: std::mem::transmute(symbol(crate::vk1_0::FN_CREATE_SAMPLER)),
            destroy_sampler: std::mem::transmute(symbol(crate::vk1_0::FN_DESTROY_SAMPLER)),
            create_descriptor_set_layout: std::mem::transmute(symbol(crate::vk1_0::FN_CREATE_DESCRIPTOR_SET_LAYOUT)),
            destroy_descriptor_set_layout: std::mem::transmute(symbol(crate::vk1_0::FN_DESTROY_DESCRIPTOR_SET_LAYOUT)),
            create_descriptor_pool: std::mem::transmute(symbol(crate::vk1_0::FN_CREATE_DESCRIPTOR_POOL)),
            destroy_descriptor_pool: std::mem::transmute(symbol(crate::vk1_0::FN_DESTROY_DESCRIPTOR_POOL)),
            reset_descriptor_pool: std::mem::transmute(symbol(crate::vk1_0::FN_RESET_DESCRIPTOR_POOL)),
            allocate_descriptor_sets: std::mem::transmute(symbol(crate::vk1_0::FN_ALLOCATE_DESCRIPTOR_SETS)),
            free_descriptor_sets: std::mem::transmute(symbol(crate::vk1_0::FN_FREE_DESCRIPTOR_SETS)),
            update_descriptor_sets: std::mem::transmute(symbol(crate::vk1_0::FN_UPDATE_DESCRIPTOR_SETS)),
            create_framebuffer: std::mem::transmute(symbol(crate::vk1_0::FN_CREATE_FRAMEBUFFER)),
            destroy_framebuffer: std::mem::transmute(symbol(crate::vk1_0::FN_DESTROY_FRAMEBUFFER)),
            create_render_pass: std::mem::transmute(symbol(crate::vk1_0::FN_CREATE_RENDER_PASS)),
            destroy_render_pass: std::mem::transmute(symbol(crate::vk1_0::FN_DESTROY_RENDER_PASS)),
            get_render_area_granularity: std::mem::transmute(symbol(crate::vk1_0::FN_GET_RENDER_AREA_GRANULARITY)),
            create_command_pool: std::mem::transmute(symbol(crate::vk1_0::FN_CREATE_COMMAND_POOL)),
            destroy_command_pool: std::mem::transmute(symbol(crate::vk1_0::FN_DESTROY_COMMAND_POOL)),
            reset_command_pool: std::mem::transmute(symbol(crate::vk1_0::FN_RESET_COMMAND_POOL)),
            allocate_command_buffers: std::mem::transmute(symbol(crate::vk1_0::FN_ALLOCATE_COMMAND_BUFFERS)),
            free_command_buffers: std::mem::transmute(symbol(crate::vk1_0::FN_FREE_COMMAND_BUFFERS)),
            begin_command_buffer: std::mem::transmute(symbol(crate::vk1_0::FN_BEGIN_COMMAND_BUFFER)),
            end_command_buffer: std::mem::transmute(symbol(crate::vk1_0::FN_END_COMMAND_BUFFER)),
            reset_command_buffer: std::mem::transmute(symbol(crate::vk1_0::FN_RESET_COMMAND_BUFFER)),
            cmd_bind_pipeline: std::mem::transmute(symbol(crate::vk1_0::FN_CMD_BIND_PIPELINE)),
            cmd_set_viewport: std::mem::transmute(symbol(crate::vk1_0::FN_CMD_SET_VIEWPORT)),
            cmd_set_scissor: std::mem::transmute(symbol(crate::vk1_0::FN_CMD_SET_SCISSOR)),
            cmd_set_line_width: std::mem::transmute(symbol(crate::vk1_0::FN_CMD_SET_LINE_WIDTH)),
            cmd_set_depth_bias: std::mem::transmute(symbol(crate::vk1_0::FN_CMD_SET_DEPTH_BIAS)),
            cmd_set_blend_constants: std::mem::transmute(symbol(crate::vk1_0::FN_CMD_SET_BLEND_CONSTANTS)),
            cmd_set_depth_bounds: std::mem::transmute(symbol(crate::vk1_0::FN_CMD_SET_DEPTH_BOUNDS)),
            cmd_set_stencil_compare_mask: std::mem::transmute(symbol(crate::vk1_0::FN_CMD_SET_STENCIL_COMPARE_MASK)),
            cmd_set_stencil_write_mask: std::mem::transmute(symbol(crate::vk1_0::FN_CMD_SET_STENCIL_WRITE_MASK)),
            cmd_set_stencil_reference: std::mem::transmute(symbol(crate::vk1_0::FN_CMD_SET_STENCIL_REFERENCE)),
            cmd_bind_descriptor_sets: std::mem::transmute(symbol(crate::vk1_0::FN_CMD_BIND_DESCRIPTOR_SETS)),
            cmd_bind_index_buffer: std::mem::transmute(symbol(crate::vk1_0::FN_CMD_BIND_INDEX_BUFFER)),
            cmd_bind_vertex_buffers: std::mem::transmute(symbol(crate::vk1_0::FN_CMD_BIND_VERTEX_BUFFERS)),
            cmd_draw: std::mem::transmute(symbol(crate::vk1_0::FN_CMD_DRAW)),
            cmd_draw_indexed: std::mem::transmute(symbol(crate::vk1_0::FN_CMD_DRAW_INDEXED)),
            cmd_draw_indirect: std::mem::transmute(symbol(crate::vk1_0::FN_CMD_DRAW_INDIRECT)),
            cmd_draw_indexed_indirect: std::mem::transmute(symbol(crate::vk1_0::FN_CMD_DRAW_INDEXED_INDIRECT)),
            cmd_dispatch: std::mem::transmute(symbol(crate::vk1_0::FN_CMD_DISPATCH)),
            cmd_dispatch_indirect: std::mem::transmute(symbol(crate::vk1_0::FN_CMD_DISPATCH_INDIRECT)),
            cmd_copy_buffer: std::mem::transmute(symbol(crate::vk1_0::FN_CMD_COPY_BUFFER)),
            cmd_copy_image: std::mem::transmute(symbol(crate::vk1_0::FN_CMD_COPY_IMAGE)),
            cmd_blit_image: std::mem::transmute(symbol(crate::vk1_0::FN_CMD_BLIT_IMAGE)),
            cmd_copy_buffer_to_image: std::mem::transmute(symbol(crate::vk1_0::FN_CMD_COPY_BUFFER_TO_IMAGE)),
            cmd_copy_image_to_buffer: std::mem::transmute(symbol(crate::vk1_0::FN_CMD_COPY_IMAGE_TO_BUFFER)),
            cmd_update_buffer: std::mem::transmute(symbol(crate::vk1_0::FN_CMD_UPDATE_BUFFER)),
            cmd_fill_buffer: std::mem::transmute(symbol(crate::vk1_0::FN_CMD_FILL_BUFFER)),
            cmd_clear_color_image: std::mem::transmute(symbol(crate::vk1_0::FN_CMD_CLEAR_COLOR_IMAGE)),
            cmd_clear_depth_stencil_image: std::mem::transmute(symbol(crate::vk1_0::FN_CMD_CLEAR_DEPTH_STENCIL_IMAGE)),
            cmd_clear_attachments: std::mem::transmute(symbol(crate::vk1_0::FN_CMD_CLEAR_ATTACHMENTS)),
            cmd_resolve_image: std::mem::transmute(symbol(crate::vk1_0::FN_CMD_RESOLVE_IMAGE)),
            cmd_set_event: std::mem::transmute(symbol(crate::vk1_0::FN_CMD_SET_EVENT)),
            cmd_reset_event: std::mem::transmute(symbol(crate::vk1_0::FN_CMD_RESET_EVENT)),
            cmd_wait_events: std::mem::transmute(symbol(crate::vk1_0::FN_CMD_WAIT_EVENTS)),
            cmd_pipeline_barrier: std::mem::transmute(symbol(crate::vk1_0::FN_CMD_PIPELINE_BARRIER)),
            cmd_begin_query: std::mem::transmute(symbol(crate::vk1_0::FN_CMD_BEGIN_QUERY)),
            cmd_end_query: std::mem::transmute(symbol(crate::vk1_0::FN_CMD_END_QUERY)),
            #[cfg(feature = "ext_conditional_rendering")]
            cmd_begin_conditional_rendering_ext: if device_enabled.ext_conditional_rendering { std::mem::transmute(symbol(crate::extensions::ext_conditional_rendering::FN_CMD_BEGIN_CONDITIONAL_RENDERING_EXT)) } else { None },
            #[cfg(feature = "ext_conditional_rendering")]
            cmd_end_conditional_rendering_ext: if device_enabled.ext_conditional_rendering { std::mem::transmute(symbol(crate::extensions::ext_conditional_rendering::FN_CMD_END_CONDITIONAL_RENDERING_EXT)) } else { None },
            cmd_reset_query_pool: std::mem::transmute(symbol(crate::vk1_0::FN_CMD_RESET_QUERY_POOL)),
            cmd_write_timestamp: std::mem::transmute(symbol(crate::vk1_0::FN_CMD_WRITE_TIMESTAMP)),
            cmd_copy_query_pool_results: std::mem::transmute(symbol(crate::vk1_0::FN_CMD_COPY_QUERY_POOL_RESULTS)),
            cmd_push_constants: std::mem::transmute(symbol(crate::vk1_0::FN_CMD_PUSH_CONSTANTS)),
            cmd_begin_render_pass: std::mem::transmute(symbol(crate::vk1_0::FN_CMD_BEGIN_RENDER_PASS)),
            cmd_next_subpass: std::mem::transmute(symbol(crate::vk1_0::FN_CMD_NEXT_SUBPASS)),
            cmd_end_render_pass: std::mem::transmute(symbol(crate::vk1_0::FN_CMD_END_RENDER_PASS)),
            cmd_execute_commands: std::mem::transmute(symbol(crate::vk1_0::FN_CMD_EXECUTE_COMMANDS)),
            #[cfg(feature = "khr_display_swapchain")]
            create_shared_swapchains_khr: if device_enabled.khr_display_swapchain { std::mem::transmute(symbol(crate::extensions::khr_display_swapchain::FN_CREATE_SHARED_SWAPCHAINS_KHR)) } else { None },
            #[cfg(feature = "khr_swapchain")]
            create_swapchain_khr: if device_enabled.khr_swapchain { std::mem::transmute(symbol(crate::extensions::khr_swapchain::FN_CREATE_SWAPCHAIN_KHR)) } else { None },
            #[cfg(feature = "khr_swapchain")]
            destroy_swapchain_khr: if device_enabled.khr_swapchain { std::mem::transmute(symbol(crate::extensions::khr_swapchain::FN_DESTROY_SWAPCHAIN_KHR)) } else { None },
            #[cfg(feature = "khr_swapchain")]
            get_swapchain_images_khr: if device_enabled.khr_swapchain { std::mem::transmute(symbol(crate::extensions::khr_swapchain::FN_GET_SWAPCHAIN_IMAGES_KHR)) } else { None },
            #[cfg(feature = "khr_swapchain")]
            acquire_next_image_khr: if device_enabled.khr_swapchain { std::mem::transmute(symbol(crate::extensions::khr_swapchain::FN_ACQUIRE_NEXT_IMAGE_KHR)) } else { None },
            #[cfg(feature = "khr_swapchain")]
            queue_present_khr: if device_enabled.khr_swapchain { std::mem::transmute(symbol(crate::extensions::khr_swapchain::FN_QUEUE_PRESENT_KHR)) } else { None },
            #[cfg(feature = "ext_debug_marker")]
            debug_marker_set_object_name_ext: if device_enabled.ext_debug_marker { std::mem::transmute(symbol(crate::extensions::ext_debug_marker::FN_DEBUG_MARKER_SET_OBJECT_NAME_EXT)) } else { None },
            #[cfg(feature = "ext_debug_marker")]
            debug_marker_set_object_tag_ext: if device_enabled.ext_debug_marker { std::mem::transmute(symbol(crate::extensions::ext_debug_marker::FN_DEBUG_MARKER_SET_OBJECT_TAG_EXT)) } else { None },
            #[cfg(feature = "ext_debug_marker")]
            cmd_debug_marker_begin_ext: if device_enabled.ext_debug_marker { std::mem::transmute(symbol(crate::extensions::ext_debug_marker::FN_CMD_DEBUG_MARKER_BEGIN_EXT)) } else { None },
            #[cfg(feature = "ext_debug_marker")]
            cmd_debug_marker_end_ext: if device_enabled.ext_debug_marker { std::mem::transmute(symbol(crate::extensions::ext_debug_marker::FN_CMD_DEBUG_MARKER_END_EXT)) } else { None },
            #[cfg(feature = "ext_debug_marker")]
            cmd_debug_marker_insert_ext: if device_enabled.ext_debug_marker { std::mem::transmute(symbol(crate::extensions::ext_debug_marker::FN_CMD_DEBUG_MARKER_INSERT_EXT)) } else { None },
            #[cfg(feature = "nv_external_memory_win32")]
            get_memory_win32_handle_nv: if device_enabled.nv_external_memory_win32 { std::mem::transmute(symbol(crate::extensions::nv_external_memory_win32::FN_GET_MEMORY_WIN32_HANDLE_NV)) } else { None },
            #[cfg(feature = "nv_device_generated_commands")]
            cmd_execute_generated_commands_nv: if device_enabled.nv_device_generated_commands { std::mem::transmute(symbol(crate::extensions::nv_device_generated_commands::FN_CMD_EXECUTE_GENERATED_COMMANDS_NV)) } else { None },
            #[cfg(feature = "nv_device_generated_commands")]
            cmd_preprocess_generated_commands_nv: if device_enabled.nv_device_generated_commands { std::mem::transmute(symbol(crate::extensions::nv_device_generated_commands::FN_CMD_PREPROCESS_GENERATED_COMMANDS_NV)) } else { None },
            #[cfg(feature = "nv_device_generated_commands")]
            cmd_bind_pipeline_shader_group_nv: if device_enabled.nv_device_generated_commands { std::mem::transmute(symbol(crate::extensions::nv_device_generated_commands::FN_CMD_BIND_PIPELINE_SHADER_GROUP_NV)) } else { None },
            #[cfg(feature = "nv_device_generated_commands")]
            get_generated_commands_memory_requirements_nv: if device_enabled.nv_device_generated_commands { std::mem::transmute(symbol(crate::extensions::nv_device_generated_commands::FN_GET_GENERATED_COMMANDS_MEMORY_REQUIREMENTS_NV)) } else { None },
            #[cfg(feature = "nv_device_generated_commands")]
            create_indirect_commands_layout_nv: if device_enabled.nv_device_generated_commands { std::mem::transmute(symbol(crate::extensions::nv_device_generated_commands::FN_CREATE_INDIRECT_COMMANDS_LAYOUT_NV)) } else { None },
            #[cfg(feature = "nv_device_generated_commands")]
            destroy_indirect_commands_layout_nv: if device_enabled.nv_device_generated_commands { std::mem::transmute(symbol(crate::extensions::nv_device_generated_commands::FN_DESTROY_INDIRECT_COMMANDS_LAYOUT_NV)) } else { None },
            #[cfg(feature = "khr_push_descriptor")]
            cmd_push_descriptor_set_khr: if device_enabled.khr_push_descriptor { std::mem::transmute(symbol(crate::extensions::khr_push_descriptor::FN_CMD_PUSH_DESCRIPTOR_SET_KHR)) } else { None },
            trim_command_pool: if instance_enabled.vk1_1 { std::mem::transmute(symbol(crate::vk1_1::FN_TRIM_COMMAND_POOL)) } else { None },
            #[cfg(feature = "khr_external_memory_win32")]
            get_memory_win32_handle_khr: if device_enabled.khr_external_memory_win32 { std::mem::transmute(symbol(crate::extensions::khr_external_memory_win32::FN_GET_MEMORY_WIN32_HANDLE_KHR)) } else { None },
            #[cfg(feature = "khr_external_memory_win32")]
            get_memory_win32_handle_properties_khr: if device_enabled.khr_external_memory_win32 { std::mem::transmute(symbol(crate::extensions::khr_external_memory_win32::FN_GET_MEMORY_WIN32_HANDLE_PROPERTIES_KHR)) } else { None },
            #[cfg(feature = "khr_external_memory_fd")]
            get_memory_fd_khr: if device_enabled.khr_external_memory_fd { std::mem::transmute(symbol(crate::extensions::khr_external_memory_fd::FN_GET_MEMORY_FD_KHR)) } else { None },
            #[cfg(feature = "khr_external_memory_fd")]
            get_memory_fd_properties_khr: if device_enabled.khr_external_memory_fd { std::mem::transmute(symbol(crate::extensions::khr_external_memory_fd::FN_GET_MEMORY_FD_PROPERTIES_KHR)) } else { None },
            #[cfg(feature = "fuchsia_external_memory")]
            get_memory_zircon_handle_fuchsia: if device_enabled.fuchsia_external_memory { std::mem::transmute(symbol(crate::extensions::fuchsia_external_memory::FN_GET_MEMORY_ZIRCON_HANDLE_FUCHSIA)) } else { None },
            #[cfg(feature = "fuchsia_external_memory")]
            get_memory_zircon_handle_properties_fuchsia: if device_enabled.fuchsia_external_memory { std::mem::transmute(symbol(crate::extensions::fuchsia_external_memory::FN_GET_MEMORY_ZIRCON_HANDLE_PROPERTIES_FUCHSIA)) } else { None },
            #[cfg(feature = "khr_external_semaphore_win32")]
            get_semaphore_win32_handle_khr: if device_enabled.khr_external_semaphore_win32 { std::mem::transmute(symbol(crate::extensions::khr_external_semaphore_win32::FN_GET_SEMAPHORE_WIN32_HANDLE_KHR)) } else { None },
            #[cfg(feature = "khr_external_semaphore_win32")]
            import_semaphore_win32_handle_khr: if device_enabled.khr_external_semaphore_win32 { std::mem::transmute(symbol(crate::extensions::khr_external_semaphore_win32::FN_IMPORT_SEMAPHORE_WIN32_HANDLE_KHR)) } else { None },
            #[cfg(feature = "khr_external_semaphore_fd")]
            get_semaphore_fd_khr: if device_enabled.khr_external_semaphore_fd { std::mem::transmute(symbol(crate::extensions::khr_external_semaphore_fd::FN_GET_SEMAPHORE_FD_KHR)) } else { None },
            #[cfg(feature = "khr_external_semaphore_fd")]
            import_semaphore_fd_khr: if device_enabled.khr_external_semaphore_fd { std::mem::transmute(symbol(crate::extensions::khr_external_semaphore_fd::FN_IMPORT_SEMAPHORE_FD_KHR)) } else { None },
            #[cfg(feature = "fuchsia_external_semaphore")]
            get_semaphore_zircon_handle_fuchsia: if device_enabled.fuchsia_external_semaphore { std::mem::transmute(symbol(crate::extensions::fuchsia_external_semaphore::FN_GET_SEMAPHORE_ZIRCON_HANDLE_FUCHSIA)) } else { None },
            #[cfg(feature = "fuchsia_external_semaphore")]
            import_semaphore_zircon_handle_fuchsia: if device_enabled.fuchsia_external_semaphore { std::mem::transmute(symbol(crate::extensions::fuchsia_external_semaphore::FN_IMPORT_SEMAPHORE_ZIRCON_HANDLE_FUCHSIA)) } else { None },
            #[cfg(feature = "khr_external_fence_win32")]
            get_fence_win32_handle_khr: if device_enabled.khr_external_fence_win32 { std::mem::transmute(symbol(crate::extensions::khr_external_fence_win32::FN_GET_FENCE_WIN32_HANDLE_KHR)) } else { None },
            #[cfg(feature = "khr_external_fence_win32")]
            import_fence_win32_handle_khr: if device_enabled.khr_external_fence_win32 { std::mem::transmute(symbol(crate::extensions::khr_external_fence_win32::FN_IMPORT_FENCE_WIN32_HANDLE_KHR)) } else { None },
            #[cfg(feature = "khr_external_fence_fd")]
            get_fence_fd_khr: if device_enabled.khr_external_fence_fd { std::mem::transmute(symbol(crate::extensions::khr_external_fence_fd::FN_GET_FENCE_FD_KHR)) } else { None },
            #[cfg(feature = "khr_external_fence_fd")]
            import_fence_fd_khr: if device_enabled.khr_external_fence_fd { std::mem::transmute(symbol(crate::extensions::khr_external_fence_fd::FN_IMPORT_FENCE_FD_KHR)) } else { None },
            #[cfg(feature = "ext_display_control")]
            display_power_control_ext: if device_enabled.ext_display_control { std::mem::transmute(symbol(crate::extensions::ext_display_control::FN_DISPLAY_POWER_CONTROL_EXT)) } else { None },
            #[cfg(feature = "ext_display_control")]
            register_device_event_ext: if device_enabled.ext_display_control { std::mem::transmute(symbol(crate::extensions::ext_display_control::FN_REGISTER_DEVICE_EVENT_EXT)) } else { None },
            #[cfg(feature = "ext_display_control")]
            register_display_event_ext: if device_enabled.ext_display_control { std::mem::transmute(symbol(crate::extensions::ext_display_control::FN_REGISTER_DISPLAY_EVENT_EXT)) } else { None },
            #[cfg(feature = "ext_display_control")]
            get_swapchain_counter_ext: if device_enabled.ext_display_control { std::mem::transmute(symbol(crate::extensions::ext_display_control::FN_GET_SWAPCHAIN_COUNTER_EXT)) } else { None },
            get_device_group_peer_memory_features: if instance_enabled.vk1_1 { std::mem::transmute(symbol(crate::vk1_1::FN_GET_DEVICE_GROUP_PEER_MEMORY_FEATURES)) } else { None },
            bind_buffer_memory2: if instance_enabled.vk1_1 { std::mem::transmute(symbol(crate::vk1_1::FN_BIND_BUFFER_MEMORY2)) } else { None },
            bind_image_memory2: if instance_enabled.vk1_1 { std::mem::transmute(symbol(crate::vk1_1::FN_BIND_IMAGE_MEMORY2)) } else { None },
            cmd_set_device_mask: if instance_enabled.vk1_1 { std::mem::transmute(symbol(crate::vk1_1::FN_CMD_SET_DEVICE_MASK)) } else { None },
            #[cfg(feature = "khr_swapchain")]
            get_device_group_present_capabilities_khr: if (device_enabled.khr_swapchain && instance_enabled.vk1_1) || (device_enabled.khr_device_group && instance_enabled.khr_surface) { std::mem::transmute(symbol(crate::extensions::khr_swapchain::FN_GET_DEVICE_GROUP_PRESENT_CAPABILITIES_KHR)) } else { None },
            #[cfg(feature = "khr_swapchain")]
            get_device_group_surface_present_modes_khr: if (device_enabled.khr_swapchain && instance_enabled.vk1_1) || (device_enabled.khr_device_group && instance_enabled.khr_surface) { std::mem::transmute(symbol(crate::extensions::khr_swapchain::FN_GET_DEVICE_GROUP_SURFACE_PRESENT_MODES_KHR)) } else { None },
            #[cfg(feature = "khr_swapchain")]
            acquire_next_image2_khr: if (device_enabled.khr_swapchain && instance_enabled.vk1_1) || (device_enabled.khr_device_group && device_enabled.khr_swapchain) { std::mem::transmute(symbol(crate::extensions::khr_swapchain::FN_ACQUIRE_NEXT_IMAGE2_KHR)) } else { None },
            cmd_dispatch_base: if instance_enabled.vk1_1 { std::mem::transmute(symbol(crate::vk1_1::FN_CMD_DISPATCH_BASE)) } else { None },
            create_descriptor_update_template: if instance_enabled.vk1_1 { std::mem::transmute(symbol(crate::vk1_1::FN_CREATE_DESCRIPTOR_UPDATE_TEMPLATE)) } else { None },
            destroy_descriptor_update_template: if instance_enabled.vk1_1 { std::mem::transmute(symbol(crate::vk1_1::FN_DESTROY_DESCRIPTOR_UPDATE_TEMPLATE)) } else { None },
            update_descriptor_set_with_template: if instance_enabled.vk1_1 { std::mem::transmute(symbol(crate::vk1_1::FN_UPDATE_DESCRIPTOR_SET_WITH_TEMPLATE)) } else { None },
            #[cfg(feature = "khr_push_descriptor")]
            cmd_push_descriptor_set_with_template_khr: if (device_enabled.khr_push_descriptor && instance_enabled.vk1_1) || (device_enabled.khr_push_descriptor && device_enabled.khr_descriptor_update_template) || (device_enabled.khr_descriptor_update_template && device_enabled.khr_push_descriptor) { std::mem::transmute(symbol(crate::extensions::khr_push_descriptor::FN_CMD_PUSH_DESCRIPTOR_SET_WITH_TEMPLATE_KHR)) } else { None },
            #[cfg(feature = "ext_hdr_metadata")]
            set_hdr_metadata_ext: if device_enabled.ext_hdr_metadata { std::mem::transmute(symbol(crate::extensions::ext_hdr_metadata::FN_SET_HDR_METADATA_EXT)) } else { None },
            #[cfg(feature = "khr_shared_presentable_image")]
            get_swapchain_status_khr: if device_enabled.khr_shared_presentable_image { std::mem::transmute(symbol(crate::extensions::khr_shared_presentable_image::FN_GET_SWAPCHAIN_STATUS_KHR)) } else { None },
            #[cfg(feature = "google_display_timing")]
            get_refresh_cycle_duration_google: if device_enabled.google_display_timing { std::mem::transmute(symbol(crate::extensions::google_display_timing::FN_GET_REFRESH_CYCLE_DURATION_GOOGLE)) } else { None },
            #[cfg(feature = "google_display_timing")]
            get_past_presentation_timing_google: if device_enabled.google_display_timing { std::mem::transmute(symbol(crate::extensions::google_display_timing::FN_GET_PAST_PRESENTATION_TIMING_GOOGLE)) } else { None },
            #[cfg(feature = "nv_clip_space_w_scaling")]
            cmd_set_viewport_w_scaling_nv: if device_enabled.nv_clip_space_w_scaling { std::mem::transmute(symbol(crate::extensions::nv_clip_space_w_scaling::FN_CMD_SET_VIEWPORT_W_SCALING_NV)) } else { None },
            #[cfg(feature = "ext_discard_rectangles")]
            cmd_set_discard_rectangle_ext: if device_enabled.ext_discard_rectangles { std::mem::transmute(symbol(crate::extensions::ext_discard_rectangles::FN_CMD_SET_DISCARD_RECTANGLE_EXT)) } else { None },
            #[cfg(feature = "ext_sample_locations")]
            cmd_set_sample_locations_ext: if device_enabled.ext_sample_locations { std::mem::transmute(symbol(crate::extensions::ext_sample_locations::FN_CMD_SET_SAMPLE_LOCATIONS_EXT)) } else { None },
            get_buffer_memory_requirements2: if instance_enabled.vk1_1 { std::mem::transmute(symbol(crate::vk1_1::FN_GET_BUFFER_MEMORY_REQUIREMENTS2)) } else { None },
            get_image_memory_requirements2: if instance_enabled.vk1_1 { std::mem::transmute(symbol(crate::vk1_1::FN_GET_IMAGE_MEMORY_REQUIREMENTS2)) } else { None },
            get_image_sparse_memory_requirements2: if instance_enabled.vk1_1 { std::mem::transmute(symbol(crate::vk1_1::FN_GET_IMAGE_SPARSE_MEMORY_REQUIREMENTS2)) } else { None },
            create_sampler_ycbcr_conversion: if instance_enabled.vk1_1 { std::mem::transmute(symbol(crate::vk1_1::FN_CREATE_SAMPLER_YCBCR_CONVERSION)) } else { None },
            destroy_sampler_ycbcr_conversion: if instance_enabled.vk1_1 { std::mem::transmute(symbol(crate::vk1_1::FN_DESTROY_SAMPLER_YCBCR_CONVERSION)) } else { None },
            get_device_queue2: if instance_enabled.vk1_1 { std::mem::transmute(symbol(crate::vk1_1::FN_GET_DEVICE_QUEUE2)) } else { None },
            #[cfg(feature = "ext_validation_cache")]
            create_validation_cache_ext: if device_enabled.ext_validation_cache { std::mem::transmute(symbol(crate::extensions::ext_validation_cache::FN_CREATE_VALIDATION_CACHE_EXT)) } else { None },
            #[cfg(feature = "ext_validation_cache")]
            destroy_validation_cache_ext: if device_enabled.ext_validation_cache { std::mem::transmute(symbol(crate::extensions::ext_validation_cache::FN_DESTROY_VALIDATION_CACHE_EXT)) } else { None },
            #[cfg(feature = "ext_validation_cache")]
            get_validation_cache_data_ext: if device_enabled.ext_validation_cache { std::mem::transmute(symbol(crate::extensions::ext_validation_cache::FN_GET_VALIDATION_CACHE_DATA_EXT)) } else { None },
            #[cfg(feature = "ext_validation_cache")]
            merge_validation_caches_ext: if device_enabled.ext_validation_cache { std::mem::transmute(symbol(crate::extensions::ext_validation_cache::FN_MERGE_VALIDATION_CACHES_EXT)) } else { None },
            get_descriptor_set_layout_support: if instance_enabled.vk1_1 { std::mem::transmute(symbol(crate::vk1_1::FN_GET_DESCRIPTOR_SET_LAYOUT_SUPPORT)) } else { None },
            #[cfg(feature = "amd_shader_info")]
            get_shader_info_amd: if device_enabled.amd_shader_info { std::mem::transmute(symbol(crate::extensions::amd_shader_info::FN_GET_SHADER_INFO_AMD)) } else { None },
            #[cfg(feature = "amd_display_native_hdr")]
            set_local_dimming_amd: if device_enabled.amd_display_native_hdr { std::mem::transmute(symbol(crate::extensions::amd_display_native_hdr::FN_SET_LOCAL_DIMMING_AMD)) } else { None },
            #[cfg(feature = "ext_calibrated_timestamps")]
            get_calibrated_timestamps_ext: if device_enabled.ext_calibrated_timestamps { std::mem::transmute(symbol(crate::extensions::ext_calibrated_timestamps::FN_GET_CALIBRATED_TIMESTAMPS_EXT)) } else { None },
            #[cfg(feature = "ext_debug_utils")]
            set_debug_utils_object_name_ext: if instance_enabled.ext_debug_utils { std::mem::transmute(symbol(crate::extensions::ext_debug_utils::FN_SET_DEBUG_UTILS_OBJECT_NAME_EXT)) } else { None },
            #[cfg(feature = "ext_debug_utils")]
            set_debug_utils_object_tag_ext: if instance_enabled.ext_debug_utils { std::mem::transmute(symbol(crate::extensions::ext_debug_utils::FN_SET_DEBUG_UTILS_OBJECT_TAG_EXT)) } else { None },
            #[cfg(feature = "ext_debug_utils")]
            queue_begin_debug_utils_label_ext: if instance_enabled.ext_debug_utils { std::mem::transmute(symbol(crate::extensions::ext_debug_utils::FN_QUEUE_BEGIN_DEBUG_UTILS_LABEL_EXT)) } else { None },
            #[cfg(feature = "ext_debug_utils")]
            queue_end_debug_utils_label_ext: if instance_enabled.ext_debug_utils { std::mem::transmute(symbol(crate::extensions::ext_debug_utils::FN_QUEUE_END_DEBUG_UTILS_LABEL_EXT)) } else { None },
            #[cfg(feature = "ext_debug_utils")]
            queue_insert_debug_utils_label_ext: if instance_enabled.ext_debug_utils { std::mem::transmute(symbol(crate::extensions::ext_debug_utils::FN_QUEUE_INSERT_DEBUG_UTILS_LABEL_EXT)) } else { None },
            #[cfg(feature = "ext_debug_utils")]
            cmd_begin_debug_utils_label_ext: if instance_enabled.ext_debug_utils { std::mem::transmute(symbol(crate::extensions::ext_debug_utils::FN_CMD_BEGIN_DEBUG_UTILS_LABEL_EXT)) } else { None },
            #[cfg(feature = "ext_debug_utils")]
            cmd_end_debug_utils_label_ext: if instance_enabled.ext_debug_utils { std::mem::transmute(symbol(crate::extensions::ext_debug_utils::FN_CMD_END_DEBUG_UTILS_LABEL_EXT)) } else { None },
            #[cfg(feature = "ext_debug_utils")]
            cmd_insert_debug_utils_label_ext: if instance_enabled.ext_debug_utils { std::mem::transmute(symbol(crate::extensions::ext_debug_utils::FN_CMD_INSERT_DEBUG_UTILS_LABEL_EXT)) } else { None },
            #[cfg(feature = "ext_external_memory_host")]
            get_memory_host_pointer_properties_ext: if device_enabled.ext_external_memory_host { std::mem::transmute(symbol(crate::extensions::ext_external_memory_host::FN_GET_MEMORY_HOST_POINTER_PROPERTIES_EXT)) } else { None },
            #[cfg(feature = "amd_buffer_marker")]
            cmd_write_buffer_marker_amd: if device_enabled.amd_buffer_marker { std::mem::transmute(symbol(crate::extensions::amd_buffer_marker::FN_CMD_WRITE_BUFFER_MARKER_AMD)) } else { None },
            create_render_pass2: if instance_enabled.vk1_2 { std::mem::transmute(symbol(crate::vk1_2::FN_CREATE_RENDER_PASS2)) } else { None },
            cmd_begin_render_pass2: if instance_enabled.vk1_2 { std::mem::transmute(symbol(crate::vk1_2::FN_CMD_BEGIN_RENDER_PASS2)) } else { None },
            cmd_next_subpass2: if instance_enabled.vk1_2 { std::mem::transmute(symbol(crate::vk1_2::FN_CMD_NEXT_SUBPASS2)) } else { None },
            cmd_end_render_pass2: if instance_enabled.vk1_2 { std::mem::transmute(symbol(crate::vk1_2::FN_CMD_END_RENDER_PASS2)) } else { None },
            get_semaphore_counter_value: if instance_enabled.vk1_2 { std::mem::transmute(symbol(crate::vk1_2::FN_GET_SEMAPHORE_COUNTER_VALUE)) } else { None },
            wait_semaphores: if instance_enabled.vk1_2 { std::mem::transmute(symbol(crate::vk1_2::FN_WAIT_SEMAPHORES)) } else { None },
            signal_semaphore: if instance_enabled.vk1_2 { std::mem::transmute(symbol(crate::vk1_2::FN_SIGNAL_SEMAPHORE)) } else { None },
            #[cfg(feature = "android_external_memory_android_hardware_buffer")]
            get_android_hardware_buffer_properties_android: if device_enabled.android_external_memory_android_hardware_buffer { std::mem::transmute(symbol(crate::extensions::android_external_memory_android_hardware_buffer::FN_GET_ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID)) } else { None },
            #[cfg(feature = "android_external_memory_android_hardware_buffer")]
            get_memory_android_hardware_buffer_android: if device_enabled.android_external_memory_android_hardware_buffer { std::mem::transmute(symbol(crate::extensions::android_external_memory_android_hardware_buffer::FN_GET_MEMORY_ANDROID_HARDWARE_BUFFER_ANDROID)) } else { None },
            cmd_draw_indirect_count: if instance_enabled.vk1_2 { std::mem::transmute(symbol(crate::vk1_2::FN_CMD_DRAW_INDIRECT_COUNT)) } else { None },
            cmd_draw_indexed_indirect_count: if instance_enabled.vk1_2 { std::mem::transmute(symbol(crate::vk1_2::FN_CMD_DRAW_INDEXED_INDIRECT_COUNT)) } else { None },
            #[cfg(feature = "nv_device_diagnostic_checkpoints")]
            cmd_set_checkpoint_nv: if device_enabled.nv_device_diagnostic_checkpoints { std::mem::transmute(symbol(crate::extensions::nv_device_diagnostic_checkpoints::FN_CMD_SET_CHECKPOINT_NV)) } else { None },
            #[cfg(feature = "nv_device_diagnostic_checkpoints")]
            get_queue_checkpoint_data_nv: if device_enabled.nv_device_diagnostic_checkpoints { std::mem::transmute(symbol(crate::extensions::nv_device_diagnostic_checkpoints::FN_GET_QUEUE_CHECKPOINT_DATA_NV)) } else { None },
            #[cfg(feature = "ext_transform_feedback")]
            cmd_bind_transform_feedback_buffers_ext: if device_enabled.ext_transform_feedback { std::mem::transmute(symbol(crate::extensions::ext_transform_feedback::FN_CMD_BIND_TRANSFORM_FEEDBACK_BUFFERS_EXT)) } else { None },
            #[cfg(feature = "ext_transform_feedback")]
            cmd_begin_transform_feedback_ext: if device_enabled.ext_transform_feedback { std::mem::transmute(symbol(crate::extensions::ext_transform_feedback::FN_CMD_BEGIN_TRANSFORM_FEEDBACK_EXT)) } else { None },
            #[cfg(feature = "ext_transform_feedback")]
            cmd_end_transform_feedback_ext: if device_enabled.ext_transform_feedback { std::mem::transmute(symbol(crate::extensions::ext_transform_feedback::FN_CMD_END_TRANSFORM_FEEDBACK_EXT)) } else { None },
            #[cfg(feature = "ext_transform_feedback")]
            cmd_begin_query_indexed_ext: if device_enabled.ext_transform_feedback { std::mem::transmute(symbol(crate::extensions::ext_transform_feedback::FN_CMD_BEGIN_QUERY_INDEXED_EXT)) } else { None },
            #[cfg(feature = "ext_transform_feedback")]
            cmd_end_query_indexed_ext: if device_enabled.ext_transform_feedback { std::mem::transmute(symbol(crate::extensions::ext_transform_feedback::FN_CMD_END_QUERY_INDEXED_EXT)) } else { None },
            #[cfg(feature = "ext_transform_feedback")]
            cmd_draw_indirect_byte_count_ext: if device_enabled.ext_transform_feedback { std::mem::transmute(symbol(crate::extensions::ext_transform_feedback::FN_CMD_DRAW_INDIRECT_BYTE_COUNT_EXT)) } else { None },
            #[cfg(feature = "nv_scissor_exclusive")]
            cmd_set_exclusive_scissor_nv: if device_enabled.nv_scissor_exclusive { std::mem::transmute(symbol(crate::extensions::nv_scissor_exclusive::FN_CMD_SET_EXCLUSIVE_SCISSOR_NV)) } else { None },
            #[cfg(feature = "nv_shading_rate_image")]
            cmd_bind_shading_rate_image_nv: if device_enabled.nv_shading_rate_image { std::mem::transmute(symbol(crate::extensions::nv_shading_rate_image::FN_CMD_BIND_SHADING_RATE_IMAGE_NV)) } else { None },
            #[cfg(feature = "nv_shading_rate_image")]
            cmd_set_viewport_shading_rate_palette_nv: if device_enabled.nv_shading_rate_image { std::mem::transmute(symbol(crate::extensions::nv_shading_rate_image::FN_CMD_SET_VIEWPORT_SHADING_RATE_PALETTE_NV)) } else { None },
            #[cfg(feature = "nv_shading_rate_image")]
            cmd_set_coarse_sample_order_nv: if device_enabled.nv_shading_rate_image { std::mem::transmute(symbol(crate::extensions::nv_shading_rate_image::FN_CMD_SET_COARSE_SAMPLE_ORDER_NV)) } else { None },
            #[cfg(feature = "nv_mesh_shader")]
            cmd_draw_mesh_tasks_nv: if device_enabled.nv_mesh_shader { std::mem::transmute(symbol(crate::extensions::nv_mesh_shader::FN_CMD_DRAW_MESH_TASKS_NV)) } else { None },
            #[cfg(feature = "nv_mesh_shader")]
            cmd_draw_mesh_tasks_indirect_nv: if device_enabled.nv_mesh_shader { std::mem::transmute(symbol(crate::extensions::nv_mesh_shader::FN_CMD_DRAW_MESH_TASKS_INDIRECT_NV)) } else { None },
            #[cfg(feature = "nv_mesh_shader")]
            cmd_draw_mesh_tasks_indirect_count_nv: if device_enabled.nv_mesh_shader { std::mem::transmute(symbol(crate::extensions::nv_mesh_shader::FN_CMD_DRAW_MESH_TASKS_INDIRECT_COUNT_NV)) } else { None },
            #[cfg(feature = "nv_ray_tracing")]
            compile_deferred_nv: if device_enabled.nv_ray_tracing { std::mem::transmute(symbol(crate::extensions::nv_ray_tracing::FN_COMPILE_DEFERRED_NV)) } else { None },
            #[cfg(feature = "nv_ray_tracing")]
            create_acceleration_structure_nv: if device_enabled.nv_ray_tracing { std::mem::transmute(symbol(crate::extensions::nv_ray_tracing::FN_CREATE_ACCELERATION_STRUCTURE_NV)) } else { None },
            #[cfg(feature = "khr_acceleration_structure")]
            destroy_acceleration_structure_khr: if device_enabled.khr_acceleration_structure { std::mem::transmute(symbol(crate::extensions::khr_acceleration_structure::FN_DESTROY_ACCELERATION_STRUCTURE_KHR)) } else { None },
            #[cfg(feature = "nv_ray_tracing")]
            destroy_acceleration_structure_nv: if device_enabled.nv_ray_tracing { std::mem::transmute(symbol(crate::extensions::nv_ray_tracing::FN_DESTROY_ACCELERATION_STRUCTURE_NV)) } else { None },
            #[cfg(feature = "nv_ray_tracing")]
            get_acceleration_structure_memory_requirements_nv: if device_enabled.nv_ray_tracing { std::mem::transmute(symbol(crate::extensions::nv_ray_tracing::FN_GET_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_NV)) } else { None },
            #[cfg(feature = "nv_ray_tracing")]
            bind_acceleration_structure_memory_nv: if device_enabled.nv_ray_tracing { std::mem::transmute(symbol(crate::extensions::nv_ray_tracing::FN_BIND_ACCELERATION_STRUCTURE_MEMORY_NV)) } else { None },
            #[cfg(feature = "nv_ray_tracing")]
            cmd_copy_acceleration_structure_nv: if device_enabled.nv_ray_tracing { std::mem::transmute(symbol(crate::extensions::nv_ray_tracing::FN_CMD_COPY_ACCELERATION_STRUCTURE_NV)) } else { None },
            #[cfg(feature = "khr_acceleration_structure")]
            cmd_copy_acceleration_structure_khr: if device_enabled.khr_acceleration_structure { std::mem::transmute(symbol(crate::extensions::khr_acceleration_structure::FN_CMD_COPY_ACCELERATION_STRUCTURE_KHR)) } else { None },
            #[cfg(feature = "khr_acceleration_structure")]
            copy_acceleration_structure_khr: if device_enabled.khr_acceleration_structure { std::mem::transmute(symbol(crate::extensions::khr_acceleration_structure::FN_COPY_ACCELERATION_STRUCTURE_KHR)) } else { None },
            #[cfg(feature = "khr_acceleration_structure")]
            cmd_copy_acceleration_structure_to_memory_khr: if device_enabled.khr_acceleration_structure { std::mem::transmute(symbol(crate::extensions::khr_acceleration_structure::FN_CMD_COPY_ACCELERATION_STRUCTURE_TO_MEMORY_KHR)) } else { None },
            #[cfg(feature = "khr_acceleration_structure")]
            copy_acceleration_structure_to_memory_khr: if device_enabled.khr_acceleration_structure { std::mem::transmute(symbol(crate::extensions::khr_acceleration_structure::FN_COPY_ACCELERATION_STRUCTURE_TO_MEMORY_KHR)) } else { None },
            #[cfg(feature = "khr_acceleration_structure")]
            cmd_copy_memory_to_acceleration_structure_khr: if device_enabled.khr_acceleration_structure { std::mem::transmute(symbol(crate::extensions::khr_acceleration_structure::FN_CMD_COPY_MEMORY_TO_ACCELERATION_STRUCTURE_KHR)) } else { None },
            #[cfg(feature = "khr_acceleration_structure")]
            copy_memory_to_acceleration_structure_khr: if device_enabled.khr_acceleration_structure { std::mem::transmute(symbol(crate::extensions::khr_acceleration_structure::FN_COPY_MEMORY_TO_ACCELERATION_STRUCTURE_KHR)) } else { None },
            #[cfg(feature = "khr_acceleration_structure")]
            cmd_write_acceleration_structures_properties_khr: if device_enabled.khr_acceleration_structure { std::mem::transmute(symbol(crate::extensions::khr_acceleration_structure::FN_CMD_WRITE_ACCELERATION_STRUCTURES_PROPERTIES_KHR)) } else { None },
            #[cfg(feature = "nv_ray_tracing")]
            cmd_write_acceleration_structures_properties_nv: if device_enabled.nv_ray_tracing { std::mem::transmute(symbol(crate::extensions::nv_ray_tracing::FN_CMD_WRITE_ACCELERATION_STRUCTURES_PROPERTIES_NV)) } else { None },
            #[cfg(feature = "nv_ray_tracing")]
            cmd_build_acceleration_structure_nv: if device_enabled.nv_ray_tracing { std::mem::transmute(symbol(crate::extensions::nv_ray_tracing::FN_CMD_BUILD_ACCELERATION_STRUCTURE_NV)) } else { None },
            #[cfg(feature = "khr_acceleration_structure")]
            write_acceleration_structures_properties_khr: if device_enabled.khr_acceleration_structure { std::mem::transmute(symbol(crate::extensions::khr_acceleration_structure::FN_WRITE_ACCELERATION_STRUCTURES_PROPERTIES_KHR)) } else { None },
            #[cfg(feature = "khr_ray_tracing_pipeline")]
            cmd_trace_rays_khr: if device_enabled.khr_ray_tracing_pipeline { std::mem::transmute(symbol(crate::extensions::khr_ray_tracing_pipeline::FN_CMD_TRACE_RAYS_KHR)) } else { None },
            #[cfg(feature = "nv_ray_tracing")]
            cmd_trace_rays_nv: if device_enabled.nv_ray_tracing { std::mem::transmute(symbol(crate::extensions::nv_ray_tracing::FN_CMD_TRACE_RAYS_NV)) } else { None },
            #[cfg(feature = "khr_ray_tracing_pipeline")]
            get_ray_tracing_shader_group_handles_khr: if device_enabled.khr_ray_tracing_pipeline { std::mem::transmute(symbol(crate::extensions::khr_ray_tracing_pipeline::FN_GET_RAY_TRACING_SHADER_GROUP_HANDLES_KHR)) } else { None },
            #[cfg(feature = "khr_ray_tracing_pipeline")]
            get_ray_tracing_capture_replay_shader_group_handles_khr: if device_enabled.khr_ray_tracing_pipeline { std::mem::transmute(symbol(crate::extensions::khr_ray_tracing_pipeline::FN_GET_RAY_TRACING_CAPTURE_REPLAY_SHADER_GROUP_HANDLES_KHR)) } else { None },
            #[cfg(feature = "nv_ray_tracing")]
            get_acceleration_structure_handle_nv: if device_enabled.nv_ray_tracing { std::mem::transmute(symbol(crate::extensions::nv_ray_tracing::FN_GET_ACCELERATION_STRUCTURE_HANDLE_NV)) } else { None },
            #[cfg(feature = "nv_ray_tracing")]
            create_ray_tracing_pipelines_nv: if device_enabled.nv_ray_tracing { std::mem::transmute(symbol(crate::extensions::nv_ray_tracing::FN_CREATE_RAY_TRACING_PIPELINES_NV)) } else { None },
            #[cfg(feature = "khr_ray_tracing_pipeline")]
            create_ray_tracing_pipelines_khr: if device_enabled.khr_ray_tracing_pipeline { std::mem::transmute(symbol(crate::extensions::khr_ray_tracing_pipeline::FN_CREATE_RAY_TRACING_PIPELINES_KHR)) } else { None },
            #[cfg(feature = "khr_ray_tracing_pipeline")]
            cmd_trace_rays_indirect_khr: if device_enabled.khr_ray_tracing_pipeline { std::mem::transmute(symbol(crate::extensions::khr_ray_tracing_pipeline::FN_CMD_TRACE_RAYS_INDIRECT_KHR)) } else { None },
            #[cfg(feature = "khr_acceleration_structure")]
            get_device_acceleration_structure_compatibility_khr: if device_enabled.khr_acceleration_structure { std::mem::transmute(symbol(crate::extensions::khr_acceleration_structure::FN_GET_DEVICE_ACCELERATION_STRUCTURE_COMPATIBILITY_KHR)) } else { None },
            #[cfg(feature = "khr_ray_tracing_pipeline")]
            get_ray_tracing_shader_group_stack_size_khr: if device_enabled.khr_ray_tracing_pipeline { std::mem::transmute(symbol(crate::extensions::khr_ray_tracing_pipeline::FN_GET_RAY_TRACING_SHADER_GROUP_STACK_SIZE_KHR)) } else { None },
            #[cfg(feature = "khr_ray_tracing_pipeline")]
            cmd_set_ray_tracing_pipeline_stack_size_khr: if device_enabled.khr_ray_tracing_pipeline { std::mem::transmute(symbol(crate::extensions::khr_ray_tracing_pipeline::FN_CMD_SET_RAY_TRACING_PIPELINE_STACK_SIZE_KHR)) } else { None },
            #[cfg(feature = "nvx_image_view_handle")]
            get_image_view_handle_nvx: if device_enabled.nvx_image_view_handle { std::mem::transmute(symbol(crate::extensions::nvx_image_view_handle::FN_GET_IMAGE_VIEW_HANDLE_NVX)) } else { None },
            #[cfg(feature = "nvx_image_view_handle")]
            get_image_view_address_nvx: if device_enabled.nvx_image_view_handle { std::mem::transmute(symbol(crate::extensions::nvx_image_view_handle::FN_GET_IMAGE_VIEW_ADDRESS_NVX)) } else { None },
            #[cfg(feature = "ext_full_screen_exclusive")]
            get_device_group_surface_present_modes2_ext: if (device_enabled.ext_full_screen_exclusive && device_enabled.khr_device_group) || (device_enabled.ext_full_screen_exclusive && instance_enabled.vk1_1) { std::mem::transmute(symbol(crate::extensions::ext_full_screen_exclusive::FN_GET_DEVICE_GROUP_SURFACE_PRESENT_MODES2_EXT)) } else { None },
            #[cfg(feature = "ext_full_screen_exclusive")]
            acquire_full_screen_exclusive_mode_ext: if device_enabled.ext_full_screen_exclusive { std::mem::transmute(symbol(crate::extensions::ext_full_screen_exclusive::FN_ACQUIRE_FULL_SCREEN_EXCLUSIVE_MODE_EXT)) } else { None },
            #[cfg(feature = "ext_full_screen_exclusive")]
            release_full_screen_exclusive_mode_ext: if device_enabled.ext_full_screen_exclusive { std::mem::transmute(symbol(crate::extensions::ext_full_screen_exclusive::FN_RELEASE_FULL_SCREEN_EXCLUSIVE_MODE_EXT)) } else { None },
            #[cfg(feature = "khr_performance_query")]
            acquire_profiling_lock_khr: if device_enabled.khr_performance_query { std::mem::transmute(symbol(crate::extensions::khr_performance_query::FN_ACQUIRE_PROFILING_LOCK_KHR)) } else { None },
            #[cfg(feature = "khr_performance_query")]
            release_profiling_lock_khr: if device_enabled.khr_performance_query { std::mem::transmute(symbol(crate::extensions::khr_performance_query::FN_RELEASE_PROFILING_LOCK_KHR)) } else { None },
            #[cfg(feature = "ext_image_drm_format_modifier")]
            get_image_drm_format_modifier_properties_ext: if device_enabled.ext_image_drm_format_modifier { std::mem::transmute(symbol(crate::extensions::ext_image_drm_format_modifier::FN_GET_IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT)) } else { None },
            get_buffer_opaque_capture_address: if instance_enabled.vk1_2 { std::mem::transmute(symbol(crate::vk1_2::FN_GET_BUFFER_OPAQUE_CAPTURE_ADDRESS)) } else { None },
            get_buffer_device_address: if instance_enabled.vk1_2 { std::mem::transmute(symbol(crate::vk1_2::FN_GET_BUFFER_DEVICE_ADDRESS)) } else { None },
            #[cfg(feature = "intel_performance_query")]
            initialize_performance_api_intel: if device_enabled.intel_performance_query { std::mem::transmute(symbol(crate::extensions::intel_performance_query::FN_INITIALIZE_PERFORMANCE_API_INTEL)) } else { None },
            #[cfg(feature = "intel_performance_query")]
            uninitialize_performance_api_intel: if device_enabled.intel_performance_query { std::mem::transmute(symbol(crate::extensions::intel_performance_query::FN_UNINITIALIZE_PERFORMANCE_API_INTEL)) } else { None },
            #[cfg(feature = "intel_performance_query")]
            cmd_set_performance_marker_intel: if device_enabled.intel_performance_query { std::mem::transmute(symbol(crate::extensions::intel_performance_query::FN_CMD_SET_PERFORMANCE_MARKER_INTEL)) } else { None },
            #[cfg(feature = "intel_performance_query")]
            cmd_set_performance_stream_marker_intel: if device_enabled.intel_performance_query { std::mem::transmute(symbol(crate::extensions::intel_performance_query::FN_CMD_SET_PERFORMANCE_STREAM_MARKER_INTEL)) } else { None },
            #[cfg(feature = "intel_performance_query")]
            cmd_set_performance_override_intel: if device_enabled.intel_performance_query { std::mem::transmute(symbol(crate::extensions::intel_performance_query::FN_CMD_SET_PERFORMANCE_OVERRIDE_INTEL)) } else { None },
            #[cfg(feature = "intel_performance_query")]
            acquire_performance_configuration_intel: if device_enabled.intel_performance_query { std::mem::transmute(symbol(crate::extensions::intel_performance_query::FN_ACQUIRE_PERFORMANCE_CONFIGURATION_INTEL)) } else { None },
            #[cfg(feature = "intel_performance_query")]
            release_performance_configuration_intel: if device_enabled.intel_performance_query { std::mem::transmute(symbol(crate::extensions::intel_performance_query::FN_RELEASE_PERFORMANCE_CONFIGURATION_INTEL)) } else { None },
            #[cfg(feature = "intel_performance_query")]
            queue_set_performance_configuration_intel: if device_enabled.intel_performance_query { std::mem::transmute(symbol(crate::extensions::intel_performance_query::FN_QUEUE_SET_PERFORMANCE_CONFIGURATION_INTEL)) } else { None },
            #[cfg(feature = "intel_performance_query")]
            get_performance_parameter_intel: if device_enabled.intel_performance_query { std::mem::transmute(symbol(crate::extensions::intel_performance_query::FN_GET_PERFORMANCE_PARAMETER_INTEL)) } else { None },
            get_device_memory_opaque_capture_address: if instance_enabled.vk1_2 { std::mem::transmute(symbol(crate::vk1_2::FN_GET_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS)) } else { None },
            #[cfg(feature = "khr_pipeline_executable_properties")]
            get_pipeline_executable_properties_khr: if device_enabled.khr_pipeline_executable_properties { std::mem::transmute(symbol(crate::extensions::khr_pipeline_executable_properties::FN_GET_PIPELINE_EXECUTABLE_PROPERTIES_KHR)) } else { None },
            #[cfg(feature = "khr_pipeline_executable_properties")]
            get_pipeline_executable_statistics_khr: if device_enabled.khr_pipeline_executable_properties { std::mem::transmute(symbol(crate::extensions::khr_pipeline_executable_properties::FN_GET_PIPELINE_EXECUTABLE_STATISTICS_KHR)) } else { None },
            #[cfg(feature = "khr_pipeline_executable_properties")]
            get_pipeline_executable_internal_representations_khr: if device_enabled.khr_pipeline_executable_properties { std::mem::transmute(symbol(crate::extensions::khr_pipeline_executable_properties::FN_GET_PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATIONS_KHR)) } else { None },
            #[cfg(feature = "ext_line_rasterization")]
            cmd_set_line_stipple_ext: if device_enabled.ext_line_rasterization { std::mem::transmute(symbol(crate::extensions::ext_line_rasterization::FN_CMD_SET_LINE_STIPPLE_EXT)) } else { None },
            #[cfg(feature = "khr_acceleration_structure")]
            create_acceleration_structure_khr: if device_enabled.khr_acceleration_structure { std::mem::transmute(symbol(crate::extensions::khr_acceleration_structure::FN_CREATE_ACCELERATION_STRUCTURE_KHR)) } else { None },
            #[cfg(feature = "khr_acceleration_structure")]
            cmd_build_acceleration_structures_khr: if device_enabled.khr_acceleration_structure { std::mem::transmute(symbol(crate::extensions::khr_acceleration_structure::FN_CMD_BUILD_ACCELERATION_STRUCTURES_KHR)) } else { None },
            #[cfg(feature = "khr_acceleration_structure")]
            cmd_build_acceleration_structures_indirect_khr: if device_enabled.khr_acceleration_structure { std::mem::transmute(symbol(crate::extensions::khr_acceleration_structure::FN_CMD_BUILD_ACCELERATION_STRUCTURES_INDIRECT_KHR)) } else { None },
            #[cfg(feature = "khr_acceleration_structure")]
            build_acceleration_structures_khr: if device_enabled.khr_acceleration_structure { std::mem::transmute(symbol(crate::extensions::khr_acceleration_structure::FN_BUILD_ACCELERATION_STRUCTURES_KHR)) } else { None },
            #[cfg(feature = "khr_acceleration_structure")]
            get_acceleration_structure_device_address_khr: if device_enabled.khr_acceleration_structure { std::mem::transmute(symbol(crate::extensions::khr_acceleration_structure::FN_GET_ACCELERATION_STRUCTURE_DEVICE_ADDRESS_KHR)) } else { None },
            #[cfg(feature = "khr_deferred_host_operations")]
            create_deferred_operation_khr: if device_enabled.khr_deferred_host_operations { std::mem::transmute(symbol(crate::extensions::khr_deferred_host_operations::FN_CREATE_DEFERRED_OPERATION_KHR)) } else { None },
            #[cfg(feature = "khr_deferred_host_operations")]
            destroy_deferred_operation_khr: if device_enabled.khr_deferred_host_operations { std::mem::transmute(symbol(crate::extensions::khr_deferred_host_operations::FN_DESTROY_DEFERRED_OPERATION_KHR)) } else { None },
            #[cfg(feature = "khr_deferred_host_operations")]
            get_deferred_operation_max_concurrency_khr: if device_enabled.khr_deferred_host_operations { std::mem::transmute(symbol(crate::extensions::khr_deferred_host_operations::FN_GET_DEFERRED_OPERATION_MAX_CONCURRENCY_KHR)) } else { None },
            #[cfg(feature = "khr_deferred_host_operations")]
            get_deferred_operation_result_khr: if device_enabled.khr_deferred_host_operations { std::mem::transmute(symbol(crate::extensions::khr_deferred_host_operations::FN_GET_DEFERRED_OPERATION_RESULT_KHR)) } else { None },
            #[cfg(feature = "khr_deferred_host_operations")]
            deferred_operation_join_khr: if device_enabled.khr_deferred_host_operations { std::mem::transmute(symbol(crate::extensions::khr_deferred_host_operations::FN_DEFERRED_OPERATION_JOIN_KHR)) } else { None },
            #[cfg(feature = "ext_extended_dynamic_state")]
            cmd_set_cull_mode_ext: if device_enabled.ext_extended_dynamic_state { std::mem::transmute(symbol(crate::extensions::ext_extended_dynamic_state::FN_CMD_SET_CULL_MODE_EXT)) } else { None },
            #[cfg(feature = "ext_extended_dynamic_state")]
            cmd_set_front_face_ext: if device_enabled.ext_extended_dynamic_state { std::mem::transmute(symbol(crate::extensions::ext_extended_dynamic_state::FN_CMD_SET_FRONT_FACE_EXT)) } else { None },
            #[cfg(feature = "ext_extended_dynamic_state")]
            cmd_set_primitive_topology_ext: if device_enabled.ext_extended_dynamic_state { std::mem::transmute(symbol(crate::extensions::ext_extended_dynamic_state::FN_CMD_SET_PRIMITIVE_TOPOLOGY_EXT)) } else { None },
            #[cfg(feature = "ext_extended_dynamic_state")]
            cmd_set_viewport_with_count_ext: if device_enabled.ext_extended_dynamic_state { std::mem::transmute(symbol(crate::extensions::ext_extended_dynamic_state::FN_CMD_SET_VIEWPORT_WITH_COUNT_EXT)) } else { None },
            #[cfg(feature = "ext_extended_dynamic_state")]
            cmd_set_scissor_with_count_ext: if device_enabled.ext_extended_dynamic_state { std::mem::transmute(symbol(crate::extensions::ext_extended_dynamic_state::FN_CMD_SET_SCISSOR_WITH_COUNT_EXT)) } else { None },
            #[cfg(feature = "ext_extended_dynamic_state")]
            cmd_bind_vertex_buffers2_ext: if device_enabled.ext_extended_dynamic_state { std::mem::transmute(symbol(crate::extensions::ext_extended_dynamic_state::FN_CMD_BIND_VERTEX_BUFFERS2_EXT)) } else { None },
            #[cfg(feature = "ext_extended_dynamic_state")]
            cmd_set_depth_test_enable_ext: if device_enabled.ext_extended_dynamic_state { std::mem::transmute(symbol(crate::extensions::ext_extended_dynamic_state::FN_CMD_SET_DEPTH_TEST_ENABLE_EXT)) } else { None },
            #[cfg(feature = "ext_extended_dynamic_state")]
            cmd_set_depth_write_enable_ext: if device_enabled.ext_extended_dynamic_state { std::mem::transmute(symbol(crate::extensions::ext_extended_dynamic_state::FN_CMD_SET_DEPTH_WRITE_ENABLE_EXT)) } else { None },
            #[cfg(feature = "ext_extended_dynamic_state")]
            cmd_set_depth_compare_op_ext: if device_enabled.ext_extended_dynamic_state { std::mem::transmute(symbol(crate::extensions::ext_extended_dynamic_state::FN_CMD_SET_DEPTH_COMPARE_OP_EXT)) } else { None },
            #[cfg(feature = "ext_extended_dynamic_state")]
            cmd_set_depth_bounds_test_enable_ext: if device_enabled.ext_extended_dynamic_state { std::mem::transmute(symbol(crate::extensions::ext_extended_dynamic_state::FN_CMD_SET_DEPTH_BOUNDS_TEST_ENABLE_EXT)) } else { None },
            #[cfg(feature = "ext_extended_dynamic_state")]
            cmd_set_stencil_test_enable_ext: if device_enabled.ext_extended_dynamic_state { std::mem::transmute(symbol(crate::extensions::ext_extended_dynamic_state::FN_CMD_SET_STENCIL_TEST_ENABLE_EXT)) } else { None },
            #[cfg(feature = "ext_extended_dynamic_state")]
            cmd_set_stencil_op_ext: if device_enabled.ext_extended_dynamic_state { std::mem::transmute(symbol(crate::extensions::ext_extended_dynamic_state::FN_CMD_SET_STENCIL_OP_EXT)) } else { None },
            #[cfg(feature = "ext_extended_dynamic_state2")]
            cmd_set_patch_control_points_ext: if device_enabled.ext_extended_dynamic_state2 { std::mem::transmute(symbol(crate::extensions::ext_extended_dynamic_state2::FN_CMD_SET_PATCH_CONTROL_POINTS_EXT)) } else { None },
            #[cfg(feature = "ext_extended_dynamic_state2")]
            cmd_set_rasterizer_discard_enable_ext: if device_enabled.ext_extended_dynamic_state2 { std::mem::transmute(symbol(crate::extensions::ext_extended_dynamic_state2::FN_CMD_SET_RASTERIZER_DISCARD_ENABLE_EXT)) } else { None },
            #[cfg(feature = "ext_extended_dynamic_state2")]
            cmd_set_depth_bias_enable_ext: if device_enabled.ext_extended_dynamic_state2 { std::mem::transmute(symbol(crate::extensions::ext_extended_dynamic_state2::FN_CMD_SET_DEPTH_BIAS_ENABLE_EXT)) } else { None },
            #[cfg(feature = "ext_extended_dynamic_state2")]
            cmd_set_logic_op_ext: if device_enabled.ext_extended_dynamic_state2 { std::mem::transmute(symbol(crate::extensions::ext_extended_dynamic_state2::FN_CMD_SET_LOGIC_OP_EXT)) } else { None },
            #[cfg(feature = "ext_extended_dynamic_state2")]
            cmd_set_primitive_restart_enable_ext: if device_enabled.ext_extended_dynamic_state2 { std::mem::transmute(symbol(crate::extensions::ext_extended_dynamic_state2::FN_CMD_SET_PRIMITIVE_RESTART_ENABLE_EXT)) } else { None },
            #[cfg(feature = "ext_private_data")]
            create_private_data_slot_ext: if device_enabled.ext_private_data { std::mem::transmute(symbol(crate::extensions::ext_private_data::FN_CREATE_PRIVATE_DATA_SLOT_EXT)) } else { None },
            #[cfg(feature = "ext_private_data")]
            destroy_private_data_slot_ext: if device_enabled.ext_private_data { std::mem::transmute(symbol(crate::extensions::ext_private_data::FN_DESTROY_PRIVATE_DATA_SLOT_EXT)) } else { None },
            #[cfg(feature = "ext_private_data")]
            set_private_data_ext: if device_enabled.ext_private_data { std::mem::transmute(symbol(crate::extensions::ext_private_data::FN_SET_PRIVATE_DATA_EXT)) } else { None },
            #[cfg(feature = "ext_private_data")]
            get_private_data_ext: if device_enabled.ext_private_data { std::mem::transmute(symbol(crate::extensions::ext_private_data::FN_GET_PRIVATE_DATA_EXT)) } else { None },
            #[cfg(feature = "khr_copy_commands2")]
            cmd_copy_buffer2_khr: if device_enabled.khr_copy_commands2 { std::mem::transmute(symbol(crate::extensions::khr_copy_commands2::FN_CMD_COPY_BUFFER2_KHR)) } else { None },
            #[cfg(feature = "khr_copy_commands2")]
            cmd_copy_image2_khr: if device_enabled.khr_copy_commands2 { std::mem::transmute(symbol(crate::extensions::khr_copy_commands2::FN_CMD_COPY_IMAGE2_KHR)) } else { None },
            #[cfg(feature = "khr_copy_commands2")]
            cmd_blit_image2_khr: if device_enabled.khr_copy_commands2 { std::mem::transmute(symbol(crate::extensions::khr_copy_commands2::FN_CMD_BLIT_IMAGE2_KHR)) } else { None },
            #[cfg(feature = "khr_copy_commands2")]
            cmd_copy_buffer_to_image2_khr: if device_enabled.khr_copy_commands2 { std::mem::transmute(symbol(crate::extensions::khr_copy_commands2::FN_CMD_COPY_BUFFER_TO_IMAGE2_KHR)) } else { None },
            #[cfg(feature = "khr_copy_commands2")]
            cmd_copy_image_to_buffer2_khr: if device_enabled.khr_copy_commands2 { std::mem::transmute(symbol(crate::extensions::khr_copy_commands2::FN_CMD_COPY_IMAGE_TO_BUFFER2_KHR)) } else { None },
            #[cfg(feature = "khr_copy_commands2")]
            cmd_resolve_image2_khr: if device_enabled.khr_copy_commands2 { std::mem::transmute(symbol(crate::extensions::khr_copy_commands2::FN_CMD_RESOLVE_IMAGE2_KHR)) } else { None },
            #[cfg(feature = "khr_fragment_shading_rate")]
            cmd_set_fragment_shading_rate_khr: if device_enabled.khr_fragment_shading_rate { std::mem::transmute(symbol(crate::extensions::khr_fragment_shading_rate::FN_CMD_SET_FRAGMENT_SHADING_RATE_KHR)) } else { None },
            #[cfg(feature = "nv_fragment_shading_rate_enums")]
            cmd_set_fragment_shading_rate_enum_nv: if device_enabled.nv_fragment_shading_rate_enums { std::mem::transmute(symbol(crate::extensions::nv_fragment_shading_rate_enums::FN_CMD_SET_FRAGMENT_SHADING_RATE_ENUM_NV)) } else { None },
            #[cfg(feature = "khr_acceleration_structure")]
            get_acceleration_structure_build_sizes_khr: if device_enabled.khr_acceleration_structure { std::mem::transmute(symbol(crate::extensions::khr_acceleration_structure::FN_GET_ACCELERATION_STRUCTURE_BUILD_SIZES_KHR)) } else { None },
            #[cfg(feature = "ext_vertex_input_dynamic_state")]
            cmd_set_vertex_input_ext: if device_enabled.ext_vertex_input_dynamic_state { std::mem::transmute(symbol(crate::extensions::ext_vertex_input_dynamic_state::FN_CMD_SET_VERTEX_INPUT_EXT)) } else { None },
            #[cfg(feature = "ext_color_write_enable")]
            cmd_set_color_write_enable_ext: if device_enabled.ext_color_write_enable { std::mem::transmute(symbol(crate::extensions::ext_color_write_enable::FN_CMD_SET_COLOR_WRITE_ENABLE_EXT)) } else { None },
            #[cfg(feature = "khr_synchronization2")]
            cmd_set_event2_khr: if device_enabled.khr_synchronization2 { std::mem::transmute(symbol(crate::extensions::khr_synchronization2::FN_CMD_SET_EVENT2_KHR)) } else { None },
            #[cfg(feature = "khr_synchronization2")]
            cmd_reset_event2_khr: if device_enabled.khr_synchronization2 { std::mem::transmute(symbol(crate::extensions::khr_synchronization2::FN_CMD_RESET_EVENT2_KHR)) } else { None },
            #[cfg(feature = "khr_synchronization2")]
            cmd_wait_events2_khr: if device_enabled.khr_synchronization2 { std::mem::transmute(symbol(crate::extensions::khr_synchronization2::FN_CMD_WAIT_EVENTS2_KHR)) } else { None },
            #[cfg(feature = "khr_synchronization2")]
            cmd_pipeline_barrier2_khr: if device_enabled.khr_synchronization2 { std::mem::transmute(symbol(crate::extensions::khr_synchronization2::FN_CMD_PIPELINE_BARRIER2_KHR)) } else { None },
            #[cfg(feature = "khr_synchronization2")]
            queue_submit2_khr: if device_enabled.khr_synchronization2 { std::mem::transmute(symbol(crate::extensions::khr_synchronization2::FN_QUEUE_SUBMIT2_KHR)) } else { None },
            #[cfg(feature = "khr_synchronization2")]
            cmd_write_timestamp2_khr: if device_enabled.khr_synchronization2 { std::mem::transmute(symbol(crate::extensions::khr_synchronization2::FN_CMD_WRITE_TIMESTAMP2_KHR)) } else { None },
            #[cfg(feature = "khr_synchronization2")]
            cmd_write_buffer_marker2_amd: if (device_enabled.khr_synchronization2 && device_enabled.amd_buffer_marker) { std::mem::transmute(symbol(crate::extensions::khr_synchronization2::FN_CMD_WRITE_BUFFER_MARKER2_AMD)) } else { None },
            #[cfg(feature = "khr_synchronization2")]
            get_queue_checkpoint_data2_nv: if (device_enabled.khr_synchronization2 && device_enabled.nv_device_diagnostic_checkpoints) { std::mem::transmute(symbol(crate::extensions::khr_synchronization2::FN_GET_QUEUE_CHECKPOINT_DATA2_NV)) } else { None },
            #[cfg(feature = "khr_video_queue")]
            create_video_session_khr: if device_enabled.khr_video_queue { std::mem::transmute(symbol(crate::extensions::khr_video_queue::FN_CREATE_VIDEO_SESSION_KHR)) } else { None },
            #[cfg(feature = "khr_video_queue")]
            destroy_video_session_khr: if device_enabled.khr_video_queue { std::mem::transmute(symbol(crate::extensions::khr_video_queue::FN_DESTROY_VIDEO_SESSION_KHR)) } else { None },
            #[cfg(feature = "khr_video_queue")]
            create_video_session_parameters_khr: if device_enabled.khr_video_queue { std::mem::transmute(symbol(crate::extensions::khr_video_queue::FN_CREATE_VIDEO_SESSION_PARAMETERS_KHR)) } else { None },
            #[cfg(feature = "khr_video_queue")]
            update_video_session_parameters_khr: if device_enabled.khr_video_queue { std::mem::transmute(symbol(crate::extensions::khr_video_queue::FN_UPDATE_VIDEO_SESSION_PARAMETERS_KHR)) } else { None },
            #[cfg(feature = "khr_video_queue")]
            destroy_video_session_parameters_khr: if device_enabled.khr_video_queue { std::mem::transmute(symbol(crate::extensions::khr_video_queue::FN_DESTROY_VIDEO_SESSION_PARAMETERS_KHR)) } else { None },
            #[cfg(feature = "khr_video_queue")]
            get_video_session_memory_requirements_khr: if device_enabled.khr_video_queue { std::mem::transmute(symbol(crate::extensions::khr_video_queue::FN_GET_VIDEO_SESSION_MEMORY_REQUIREMENTS_KHR)) } else { None },
            #[cfg(feature = "khr_video_queue")]
            bind_video_session_memory_khr: if device_enabled.khr_video_queue { std::mem::transmute(symbol(crate::extensions::khr_video_queue::FN_BIND_VIDEO_SESSION_MEMORY_KHR)) } else { None },
            #[cfg(feature = "khr_video_decode_queue")]
            cmd_decode_video_khr: if device_enabled.khr_video_decode_queue { std::mem::transmute(symbol(crate::extensions::khr_video_decode_queue::FN_CMD_DECODE_VIDEO_KHR)) } else { None },
            #[cfg(feature = "khr_video_queue")]
            cmd_begin_video_coding_khr: if device_enabled.khr_video_queue { std::mem::transmute(symbol(crate::extensions::khr_video_queue::FN_CMD_BEGIN_VIDEO_CODING_KHR)) } else { None },
            #[cfg(feature = "khr_video_queue")]
            cmd_control_video_coding_khr: if device_enabled.khr_video_queue { std::mem::transmute(symbol(crate::extensions::khr_video_queue::FN_CMD_CONTROL_VIDEO_CODING_KHR)) } else { None },
            #[cfg(feature = "khr_video_queue")]
            cmd_end_video_coding_khr: if device_enabled.khr_video_queue { std::mem::transmute(symbol(crate::extensions::khr_video_queue::FN_CMD_END_VIDEO_CODING_KHR)) } else { None },
            #[cfg(feature = "khr_video_encode_queue")]
            cmd_encode_video_khr: if device_enabled.khr_video_encode_queue { std::mem::transmute(symbol(crate::extensions::khr_video_encode_queue::FN_CMD_ENCODE_VIDEO_KHR)) } else { None },
            #[cfg(feature = "nvx_binary_import")]
            create_cu_module_nvx: if device_enabled.nvx_binary_import { std::mem::transmute(symbol(crate::extensions::nvx_binary_import::FN_CREATE_CU_MODULE_NVX)) } else { None },
            #[cfg(feature = "nvx_binary_import")]
            create_cu_function_nvx: if device_enabled.nvx_binary_import { std::mem::transmute(symbol(crate::extensions::nvx_binary_import::FN_CREATE_CU_FUNCTION_NVX)) } else { None },
            #[cfg(feature = "nvx_binary_import")]
            destroy_cu_module_nvx: if device_enabled.nvx_binary_import { std::mem::transmute(symbol(crate::extensions::nvx_binary_import::FN_DESTROY_CU_MODULE_NVX)) } else { None },
            #[cfg(feature = "nvx_binary_import")]
            destroy_cu_function_nvx: if device_enabled.nvx_binary_import { std::mem::transmute(symbol(crate::extensions::nvx_binary_import::FN_DESTROY_CU_FUNCTION_NVX)) } else { None },
            #[cfg(feature = "nvx_binary_import")]
            cmd_cu_launch_kernel_nvx: if device_enabled.nvx_binary_import { std::mem::transmute(symbol(crate::extensions::nvx_binary_import::FN_CMD_CU_LAUNCH_KERNEL_NVX)) } else { None },
            #[cfg(feature = "ext_host_query_reset")]
            reset_query_pool_ext: if device_enabled.ext_host_query_reset { std::mem::transmute(symbol(crate::extensions::ext_host_query_reset::FN_RESET_QUERY_POOL_EXT)) } else { None },
            #[cfg(feature = "khr_maintenance1")]
            trim_command_pool_khr: if device_enabled.khr_maintenance1 { std::mem::transmute(symbol(crate::extensions::khr_maintenance1::FN_TRIM_COMMAND_POOL_KHR)) } else { None },
            #[cfg(feature = "khr_device_group")]
            get_device_group_peer_memory_features_khr: if device_enabled.khr_device_group { std::mem::transmute(symbol(crate::extensions::khr_device_group::FN_GET_DEVICE_GROUP_PEER_MEMORY_FEATURES_KHR)) } else { None },
            #[cfg(feature = "khr_bind_memory2")]
            bind_buffer_memory2_khr: if device_enabled.khr_bind_memory2 { std::mem::transmute(symbol(crate::extensions::khr_bind_memory2::FN_BIND_BUFFER_MEMORY2_KHR)) } else { None },
            #[cfg(feature = "khr_bind_memory2")]
            bind_image_memory2_khr: if device_enabled.khr_bind_memory2 { std::mem::transmute(symbol(crate::extensions::khr_bind_memory2::FN_BIND_IMAGE_MEMORY2_KHR)) } else { None },
            #[cfg(feature = "khr_device_group")]
            cmd_set_device_mask_khr: if device_enabled.khr_device_group { std::mem::transmute(symbol(crate::extensions::khr_device_group::FN_CMD_SET_DEVICE_MASK_KHR)) } else { None },
            #[cfg(feature = "khr_device_group")]
            cmd_dispatch_base_khr: if device_enabled.khr_device_group { std::mem::transmute(symbol(crate::extensions::khr_device_group::FN_CMD_DISPATCH_BASE_KHR)) } else { None },
            #[cfg(feature = "khr_descriptor_update_template")]
            create_descriptor_update_template_khr: if device_enabled.khr_descriptor_update_template { std::mem::transmute(symbol(crate::extensions::khr_descriptor_update_template::FN_CREATE_DESCRIPTOR_UPDATE_TEMPLATE_KHR)) } else { None },
            #[cfg(feature = "khr_descriptor_update_template")]
            destroy_descriptor_update_template_khr: if device_enabled.khr_descriptor_update_template { std::mem::transmute(symbol(crate::extensions::khr_descriptor_update_template::FN_DESTROY_DESCRIPTOR_UPDATE_TEMPLATE_KHR)) } else { None },
            #[cfg(feature = "khr_descriptor_update_template")]
            update_descriptor_set_with_template_khr: if device_enabled.khr_descriptor_update_template { std::mem::transmute(symbol(crate::extensions::khr_descriptor_update_template::FN_UPDATE_DESCRIPTOR_SET_WITH_TEMPLATE_KHR)) } else { None },
            #[cfg(feature = "khr_get_memory_requirements2")]
            get_buffer_memory_requirements2_khr: if device_enabled.khr_get_memory_requirements2 { std::mem::transmute(symbol(crate::extensions::khr_get_memory_requirements2::FN_GET_BUFFER_MEMORY_REQUIREMENTS2_KHR)) } else { None },
            #[cfg(feature = "khr_get_memory_requirements2")]
            get_image_memory_requirements2_khr: if device_enabled.khr_get_memory_requirements2 { std::mem::transmute(symbol(crate::extensions::khr_get_memory_requirements2::FN_GET_IMAGE_MEMORY_REQUIREMENTS2_KHR)) } else { None },
            #[cfg(feature = "khr_get_memory_requirements2")]
            get_image_sparse_memory_requirements2_khr: if device_enabled.khr_get_memory_requirements2 { std::mem::transmute(symbol(crate::extensions::khr_get_memory_requirements2::FN_GET_IMAGE_SPARSE_MEMORY_REQUIREMENTS2_KHR)) } else { None },
            #[cfg(feature = "khr_sampler_ycbcr_conversion")]
            create_sampler_ycbcr_conversion_khr: if device_enabled.khr_sampler_ycbcr_conversion { std::mem::transmute(symbol(crate::extensions::khr_sampler_ycbcr_conversion::FN_CREATE_SAMPLER_YCBCR_CONVERSION_KHR)) } else { None },
            #[cfg(feature = "khr_sampler_ycbcr_conversion")]
            destroy_sampler_ycbcr_conversion_khr: if device_enabled.khr_sampler_ycbcr_conversion { std::mem::transmute(symbol(crate::extensions::khr_sampler_ycbcr_conversion::FN_DESTROY_SAMPLER_YCBCR_CONVERSION_KHR)) } else { None },
            #[cfg(feature = "khr_maintenance3")]
            get_descriptor_set_layout_support_khr: if device_enabled.khr_maintenance3 { std::mem::transmute(symbol(crate::extensions::khr_maintenance3::FN_GET_DESCRIPTOR_SET_LAYOUT_SUPPORT_KHR)) } else { None },
            #[cfg(feature = "khr_create_renderpass2")]
            create_render_pass2_khr: if device_enabled.khr_create_renderpass2 { std::mem::transmute(symbol(crate::extensions::khr_create_renderpass2::FN_CREATE_RENDER_PASS2_KHR)) } else { None },
            #[cfg(feature = "khr_create_renderpass2")]
            cmd_begin_render_pass2_khr: if device_enabled.khr_create_renderpass2 { std::mem::transmute(symbol(crate::extensions::khr_create_renderpass2::FN_CMD_BEGIN_RENDER_PASS2_KHR)) } else { None },
            #[cfg(feature = "khr_create_renderpass2")]
            cmd_next_subpass2_khr: if device_enabled.khr_create_renderpass2 { std::mem::transmute(symbol(crate::extensions::khr_create_renderpass2::FN_CMD_NEXT_SUBPASS2_KHR)) } else { None },
            #[cfg(feature = "khr_create_renderpass2")]
            cmd_end_render_pass2_khr: if device_enabled.khr_create_renderpass2 { std::mem::transmute(symbol(crate::extensions::khr_create_renderpass2::FN_CMD_END_RENDER_PASS2_KHR)) } else { None },
            #[cfg(feature = "khr_timeline_semaphore")]
            get_semaphore_counter_value_khr: if device_enabled.khr_timeline_semaphore { std::mem::transmute(symbol(crate::extensions::khr_timeline_semaphore::FN_GET_SEMAPHORE_COUNTER_VALUE_KHR)) } else { None },
            #[cfg(feature = "khr_timeline_semaphore")]
            wait_semaphores_khr: if device_enabled.khr_timeline_semaphore { std::mem::transmute(symbol(crate::extensions::khr_timeline_semaphore::FN_WAIT_SEMAPHORES_KHR)) } else { None },
            #[cfg(feature = "khr_timeline_semaphore")]
            signal_semaphore_khr: if device_enabled.khr_timeline_semaphore { std::mem::transmute(symbol(crate::extensions::khr_timeline_semaphore::FN_SIGNAL_SEMAPHORE_KHR)) } else { None },
            #[cfg(feature = "khr_draw_indirect_count")]
            cmd_draw_indirect_count_khr: if device_enabled.khr_draw_indirect_count { std::mem::transmute(symbol(crate::extensions::khr_draw_indirect_count::FN_CMD_DRAW_INDIRECT_COUNT_KHR)) } else { None },
            #[cfg(feature = "amd_draw_indirect_count")]
            cmd_draw_indirect_count_amd: if device_enabled.amd_draw_indirect_count { std::mem::transmute(symbol(crate::extensions::amd_draw_indirect_count::FN_CMD_DRAW_INDIRECT_COUNT_AMD)) } else { None },
            #[cfg(feature = "khr_draw_indirect_count")]
            cmd_draw_indexed_indirect_count_khr: if device_enabled.khr_draw_indirect_count { std::mem::transmute(symbol(crate::extensions::khr_draw_indirect_count::FN_CMD_DRAW_INDEXED_INDIRECT_COUNT_KHR)) } else { None },
            #[cfg(feature = "amd_draw_indirect_count")]
            cmd_draw_indexed_indirect_count_amd: if device_enabled.amd_draw_indirect_count { std::mem::transmute(symbol(crate::extensions::amd_draw_indirect_count::FN_CMD_DRAW_INDEXED_INDIRECT_COUNT_AMD)) } else { None },
            #[cfg(feature = "nv_ray_tracing")]
            get_ray_tracing_shader_group_handles_nv: if device_enabled.nv_ray_tracing { std::mem::transmute(symbol(crate::extensions::nv_ray_tracing::FN_GET_RAY_TRACING_SHADER_GROUP_HANDLES_NV)) } else { None },
            #[cfg(feature = "khr_buffer_device_address")]
            get_buffer_opaque_capture_address_khr: if device_enabled.khr_buffer_device_address { std::mem::transmute(symbol(crate::extensions::khr_buffer_device_address::FN_GET_BUFFER_OPAQUE_CAPTURE_ADDRESS_KHR)) } else { None },
            #[cfg(feature = "khr_buffer_device_address")]
            get_buffer_device_address_khr: if device_enabled.khr_buffer_device_address { std::mem::transmute(symbol(crate::extensions::khr_buffer_device_address::FN_GET_BUFFER_DEVICE_ADDRESS_KHR)) } else { None },
            #[cfg(feature = "ext_buffer_device_address")]
            get_buffer_device_address_ext: if device_enabled.ext_buffer_device_address { std::mem::transmute(symbol(crate::extensions::ext_buffer_device_address::FN_GET_BUFFER_DEVICE_ADDRESS_EXT)) } else { None },
            #[cfg(feature = "khr_buffer_device_address")]
            get_device_memory_opaque_capture_address_khr: if device_enabled.khr_buffer_device_address { std::mem::transmute(symbol(crate::extensions::khr_buffer_device_address::FN_GET_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_KHR)) } else { None },
            enabled: device_enabled,
        })
    }
    pub fn enabled(&self) -> &DeviceEnabled {
        &self.enabled
    }
}
#[doc = r" Provides Vulkan feature items"]
pub mod vk1_0;
#[doc = r" Provides Vulkan feature items"]
pub mod vk1_1;
#[doc = r" Provides Vulkan feature items"]
pub mod vk1_2;
