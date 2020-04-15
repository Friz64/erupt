#[doc = r" Loader for Core Commands"]
pub struct CoreLoader<T> {
    pub loader: T,
    pub symbol_loader: Box<dyn Fn(&T, &str) -> Option<std::num::NonZeroUsize> + Send + Sync>,
    #[doc = "Implemented in [`vk1_0::Vk10CoreLoaderExt`](vk1_0/trait.Vk10CoreLoaderExt.html)"]
    pub vk1_0: Option<crate::vk1_0::Vk10CoreCommands>,
    #[doc = "Implemented in [`vk1_1::Vk11CoreLoaderExt`](vk1_1/trait.Vk11CoreLoaderExt.html)"]
    pub vk1_1: Option<crate::vk1_1::Vk11CoreCommands>,
}
impl<T> CoreLoader<T> {
    pub fn custom(
        loader: T,
        symbol_loader: Box<dyn Fn(&T, &str) -> Option<std::num::NonZeroUsize> + Send + Sync>,
    ) -> CoreLoader<T> {
        CoreLoader {
            loader,
            symbol_loader,
            vk1_0: None,
            vk1_1: None,
        }
    }
    #[inline]
    #[inline]
    pub unsafe fn symbol(&self, name: &str) -> Option<std::num::NonZeroUsize> {
        (self.symbol_loader)(&self.loader, name)
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_vk1_0(&mut self) -> Option<()> {
        self.vk1_0 = crate::vk1_0::Vk10CoreCommands::load(&self);
        self.vk1_0.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_vk1_1(&mut self) -> Option<()> {
        self.vk1_1 = crate::vk1_1::Vk11CoreCommands::load(&self);
        self.vk1_1.as_ref().map(|_| ())
    }
}
#[doc = r" Loader for Instance Commands. **Must** outlive [`CoreLoader`](struct.CoreLoader.html)"]
pub struct InstanceLoader { pub loader : crate :: vk1_0 :: PFN_vkGetInstanceProcAddr , pub handle : crate :: vk1_0 :: Instance , # [ doc = "Implemented in [`vk1_0::Vk10InstanceLoaderExt`](vk1_0/trait.Vk10InstanceLoaderExt.html)" ] pub vk1_0 : Option < crate :: vk1_0 :: Vk10InstanceCommands > , # [ doc = "Implemented in [`vk1_1::Vk11InstanceLoaderExt`](vk1_1/trait.Vk11InstanceLoaderExt.html)" ] pub vk1_1 : Option < crate :: vk1_1 :: Vk11InstanceCommands > , # [ doc = "Implemented in [`extensions::khr_get_physical_device_properties2::KhrGetPhysicalDeviceProperties2InstanceLoaderExt`](extensions/khr_get_physical_device_properties2/trait.KhrGetPhysicalDeviceProperties2InstanceLoaderExt.html)" ] pub khr_get_physical_device_properties2 : Option < crate :: extensions :: khr_get_physical_device_properties2 :: KhrGetPhysicalDeviceProperties2InstanceCommands > , # [ doc = "Implemented in [`extensions::khr_surface::KhrSurfaceInstanceLoaderExt`](extensions/khr_surface/trait.KhrSurfaceInstanceLoaderExt.html)" ] pub khr_surface : Option < crate :: extensions :: khr_surface :: KhrSurfaceInstanceCommands > , # [ doc = "Implemented in [`extensions::khr_swapchain::KhrSwapchainInstanceLoaderExt`](extensions/khr_swapchain/trait.KhrSwapchainInstanceLoaderExt.html)" ] pub khr_swapchain : Option < crate :: extensions :: khr_swapchain :: KhrSwapchainInstanceCommands > , # [ doc = "Implemented in [`extensions::khr_get_surface_capabilities2::KhrGetSurfaceCapabilities2InstanceLoaderExt`](extensions/khr_get_surface_capabilities2/trait.KhrGetSurfaceCapabilities2InstanceLoaderExt.html)" ] pub khr_get_surface_capabilities2 : Option < crate :: extensions :: khr_get_surface_capabilities2 :: KhrGetSurfaceCapabilities2InstanceCommands > , # [ doc = "Implemented in [`extensions::nv_cooperative_matrix::NvCooperativeMatrixInstanceLoaderExt`](extensions/nv_cooperative_matrix/trait.NvCooperativeMatrixInstanceLoaderExt.html)" ] pub nv_cooperative_matrix : Option < crate :: extensions :: nv_cooperative_matrix :: NvCooperativeMatrixInstanceCommands > , # [ doc = "Implemented in [`extensions::ext_debug_report::ExtDebugReportInstanceLoaderExt`](extensions/ext_debug_report/trait.ExtDebugReportInstanceLoaderExt.html)" ] pub ext_debug_report : Option < crate :: extensions :: ext_debug_report :: ExtDebugReportInstanceCommands > , # [ doc = "Implemented in [`extensions::ext_metal_surface::ExtMetalSurfaceInstanceLoaderExt`](extensions/ext_metal_surface/trait.ExtMetalSurfaceInstanceLoaderExt.html)" ] pub ext_metal_surface : Option < crate :: extensions :: ext_metal_surface :: ExtMetalSurfaceInstanceCommands > , # [ doc = "Implemented in [`extensions::fuchsia_imagepipe_surface::FuchsiaImagepipeSurfaceInstanceLoaderExt`](extensions/fuchsia_imagepipe_surface/trait.FuchsiaImagepipeSurfaceInstanceLoaderExt.html)" ] pub fuchsia_imagepipe_surface : Option < crate :: extensions :: fuchsia_imagepipe_surface :: FuchsiaImagepipeSurfaceInstanceCommands > , # [ doc = "Implemented in [`extensions::ggp_stream_descriptor_surface::GgpStreamDescriptorSurfaceInstanceLoaderExt`](extensions/ggp_stream_descriptor_surface/trait.GgpStreamDescriptorSurfaceInstanceLoaderExt.html)" ] pub ggp_stream_descriptor_surface : Option < crate :: extensions :: ggp_stream_descriptor_surface :: GgpStreamDescriptorSurfaceInstanceCommands > , # [ doc = "Implemented in [`extensions::ext_calibrated_timestamps::ExtCalibratedTimestampsInstanceLoaderExt`](extensions/ext_calibrated_timestamps/trait.ExtCalibratedTimestampsInstanceLoaderExt.html)" ] pub ext_calibrated_timestamps : Option < crate :: extensions :: ext_calibrated_timestamps :: ExtCalibratedTimestampsInstanceCommands > , # [ doc = "Implemented in [`extensions::khr_external_memory_capabilities::KhrExternalMemoryCapabilitiesInstanceLoaderExt`](extensions/khr_external_memory_capabilities/trait.KhrExternalMemoryCapabilitiesInstanceLoaderExt.html)" ] pub khr_external_memory_capabilities : Option < crate :: extensions :: khr_external_memory_capabilities :: KhrExternalMemoryCapabilitiesInstanceCommands > , # [ doc = "Implemented in [`extensions::nv_coverage_reduction_mode::NvCoverageReductionModeInstanceLoaderExt`](extensions/nv_coverage_reduction_mode/trait.NvCoverageReductionModeInstanceLoaderExt.html)" ] pub nv_coverage_reduction_mode : Option < crate :: extensions :: nv_coverage_reduction_mode :: NvCoverageReductionModeInstanceCommands > , # [ doc = "Implemented in [`extensions::ext_sample_locations::ExtSampleLocationsInstanceLoaderExt`](extensions/ext_sample_locations/trait.ExtSampleLocationsInstanceLoaderExt.html)" ] pub ext_sample_locations : Option < crate :: extensions :: ext_sample_locations :: ExtSampleLocationsInstanceCommands > , # [ doc = "Implemented in [`extensions::ext_debug_utils::ExtDebugUtilsInstanceLoaderExt`](extensions/ext_debug_utils/trait.ExtDebugUtilsInstanceLoaderExt.html)" ] pub ext_debug_utils : Option < crate :: extensions :: ext_debug_utils :: ExtDebugUtilsInstanceCommands > , # [ doc = "Implemented in [`extensions::mvk_macos_surface::MvkMacosSurfaceInstanceLoaderExt`](extensions/mvk_macos_surface/trait.MvkMacosSurfaceInstanceLoaderExt.html)" ] pub mvk_macos_surface : Option < crate :: extensions :: mvk_macos_surface :: MvkMacosSurfaceInstanceCommands > , # [ doc = "Implemented in [`extensions::mvk_ios_surface::MvkIosSurfaceInstanceLoaderExt`](extensions/mvk_ios_surface/trait.MvkIosSurfaceInstanceLoaderExt.html)" ] pub mvk_ios_surface : Option < crate :: extensions :: mvk_ios_surface :: MvkIosSurfaceInstanceCommands > , # [ doc = "Implemented in [`extensions::khr_display::KhrDisplayInstanceLoaderExt`](extensions/khr_display/trait.KhrDisplayInstanceLoaderExt.html)" ] pub khr_display : Option < crate :: extensions :: khr_display :: KhrDisplayInstanceCommands > , # [ doc = "Implemented in [`extensions::khr_win32_surface::KhrWin32SurfaceInstanceLoaderExt`](extensions/khr_win32_surface/trait.KhrWin32SurfaceInstanceLoaderExt.html)" ] pub khr_win32_surface : Option < crate :: extensions :: khr_win32_surface :: KhrWin32SurfaceInstanceCommands > , # [ doc = "Implemented in [`extensions::khr_performance_query::KhrPerformanceQueryInstanceLoaderExt`](extensions/khr_performance_query/trait.KhrPerformanceQueryInstanceLoaderExt.html)" ] pub khr_performance_query : Option < crate :: extensions :: khr_performance_query :: KhrPerformanceQueryInstanceCommands > , # [ doc = "Implemented in [`extensions::khr_external_fence_capabilities::KhrExternalFenceCapabilitiesInstanceLoaderExt`](extensions/khr_external_fence_capabilities/trait.KhrExternalFenceCapabilitiesInstanceLoaderExt.html)" ] pub khr_external_fence_capabilities : Option < crate :: extensions :: khr_external_fence_capabilities :: KhrExternalFenceCapabilitiesInstanceCommands > , # [ doc = "Implemented in [`extensions::ext_display_surface_counter::ExtDisplaySurfaceCounterInstanceLoaderExt`](extensions/ext_display_surface_counter/trait.ExtDisplaySurfaceCounterInstanceLoaderExt.html)" ] pub ext_display_surface_counter : Option < crate :: extensions :: ext_display_surface_counter :: ExtDisplaySurfaceCounterInstanceCommands > , # [ doc = "Implemented in [`extensions::ext_direct_mode_display::ExtDirectModeDisplayInstanceLoaderExt`](extensions/ext_direct_mode_display/trait.ExtDirectModeDisplayInstanceLoaderExt.html)" ] pub ext_direct_mode_display : Option < crate :: extensions :: ext_direct_mode_display :: ExtDirectModeDisplayInstanceCommands > , # [ doc = "Implemented in [`extensions::ext_acquire_xlib_display::ExtAcquireXlibDisplayInstanceLoaderExt`](extensions/ext_acquire_xlib_display/trait.ExtAcquireXlibDisplayInstanceLoaderExt.html)" ] pub ext_acquire_xlib_display : Option < crate :: extensions :: ext_acquire_xlib_display :: ExtAcquireXlibDisplayInstanceCommands > , # [ doc = "Implemented in [`extensions::khr_external_semaphore_capabilities::KhrExternalSemaphoreCapabilitiesInstanceLoaderExt`](extensions/khr_external_semaphore_capabilities/trait.KhrExternalSemaphoreCapabilitiesInstanceLoaderExt.html)" ] pub khr_external_semaphore_capabilities : Option < crate :: extensions :: khr_external_semaphore_capabilities :: KhrExternalSemaphoreCapabilitiesInstanceCommands > , # [ doc = "Implemented in [`extensions::khr_device_group_creation::KhrDeviceGroupCreationInstanceLoaderExt`](extensions/khr_device_group_creation/trait.KhrDeviceGroupCreationInstanceLoaderExt.html)" ] pub khr_device_group_creation : Option < crate :: extensions :: khr_device_group_creation :: KhrDeviceGroupCreationInstanceCommands > , # [ doc = "Implemented in [`extensions::nn_vi_surface::NnViSurfaceInstanceLoaderExt`](extensions/nn_vi_surface/trait.NnViSurfaceInstanceLoaderExt.html)" ] pub nn_vi_surface : Option < crate :: extensions :: nn_vi_surface :: NnViSurfaceInstanceCommands > , # [ doc = "Implemented in [`extensions::khr_device_group::KhrDeviceGroupInstanceLoaderExt`](extensions/khr_device_group/trait.KhrDeviceGroupInstanceLoaderExt.html)" ] pub khr_device_group : Option < crate :: extensions :: khr_device_group :: KhrDeviceGroupInstanceCommands > , # [ doc = "Implemented in [`extensions::nv_external_memory_capabilities::NvExternalMemoryCapabilitiesInstanceLoaderExt`](extensions/nv_external_memory_capabilities/trait.NvExternalMemoryCapabilitiesInstanceLoaderExt.html)" ] pub nv_external_memory_capabilities : Option < crate :: extensions :: nv_external_memory_capabilities :: NvExternalMemoryCapabilitiesInstanceCommands > , # [ doc = "Implemented in [`extensions::ext_tooling_info::ExtToolingInfoInstanceLoaderExt`](extensions/ext_tooling_info/trait.ExtToolingInfoInstanceLoaderExt.html)" ] pub ext_tooling_info : Option < crate :: extensions :: ext_tooling_info :: ExtToolingInfoInstanceCommands > , # [ doc = "Implemented in [`extensions::ext_full_screen_exclusive::ExtFullScreenExclusiveInstanceLoaderExt`](extensions/ext_full_screen_exclusive/trait.ExtFullScreenExclusiveInstanceLoaderExt.html)" ] pub ext_full_screen_exclusive : Option < crate :: extensions :: ext_full_screen_exclusive :: ExtFullScreenExclusiveInstanceCommands > , # [ doc = "Implemented in [`extensions::khr_android_surface::KhrAndroidSurfaceInstanceLoaderExt`](extensions/khr_android_surface/trait.KhrAndroidSurfaceInstanceLoaderExt.html)" ] pub khr_android_surface : Option < crate :: extensions :: khr_android_surface :: KhrAndroidSurfaceInstanceCommands > , # [ doc = "Implemented in [`extensions::khr_wayland_surface::KhrWaylandSurfaceInstanceLoaderExt`](extensions/khr_wayland_surface/trait.KhrWaylandSurfaceInstanceLoaderExt.html)" ] pub khr_wayland_surface : Option < crate :: extensions :: khr_wayland_surface :: KhrWaylandSurfaceInstanceCommands > , # [ doc = "Implemented in [`extensions::khr_xcb_surface::KhrXcbSurfaceInstanceLoaderExt`](extensions/khr_xcb_surface/trait.KhrXcbSurfaceInstanceLoaderExt.html)" ] pub khr_xcb_surface : Option < crate :: extensions :: khr_xcb_surface :: KhrXcbSurfaceInstanceCommands > , # [ doc = "Implemented in [`extensions::khr_xlib_surface::KhrXlibSurfaceInstanceLoaderExt`](extensions/khr_xlib_surface/trait.KhrXlibSurfaceInstanceLoaderExt.html)" ] pub khr_xlib_surface : Option < crate :: extensions :: khr_xlib_surface :: KhrXlibSurfaceInstanceCommands > , # [ doc = "Implemented in [`extensions::khr_get_display_properties2::KhrGetDisplayProperties2InstanceLoaderExt`](extensions/khr_get_display_properties2/trait.KhrGetDisplayProperties2InstanceLoaderExt.html)" ] pub khr_get_display_properties2 : Option < crate :: extensions :: khr_get_display_properties2 :: KhrGetDisplayProperties2InstanceCommands > , # [ doc = "Implemented in [`extensions::ext_headless_surface::ExtHeadlessSurfaceInstanceLoaderExt`](extensions/ext_headless_surface/trait.ExtHeadlessSurfaceInstanceLoaderExt.html)" ] pub ext_headless_surface : Option < crate :: extensions :: ext_headless_surface :: ExtHeadlessSurfaceInstanceCommands > , }
impl InstanceLoader {
    pub fn new<T>(
        core_loader: &CoreLoader<T>,
        instance: crate::vk1_0::Instance,
    ) -> Option<InstanceLoader> {
        Some(InstanceLoader {
            loader: unsafe { std::mem::transmute(core_loader.symbol("vkGetInstanceProcAddr")?) },
            handle: instance,
            vk1_0: None,
            vk1_1: None,
            khr_get_physical_device_properties2: None,
            khr_surface: None,
            khr_swapchain: None,
            khr_get_surface_capabilities2: None,
            nv_cooperative_matrix: None,
            ext_debug_report: None,
            ext_metal_surface: None,
            fuchsia_imagepipe_surface: None,
            ggp_stream_descriptor_surface: None,
            ext_calibrated_timestamps: None,
            khr_external_memory_capabilities: None,
            nv_coverage_reduction_mode: None,
            ext_sample_locations: None,
            ext_debug_utils: None,
            mvk_macos_surface: None,
            mvk_ios_surface: None,
            khr_display: None,
            khr_win32_surface: None,
            khr_performance_query: None,
            khr_external_fence_capabilities: None,
            ext_display_surface_counter: None,
            ext_direct_mode_display: None,
            ext_acquire_xlib_display: None,
            khr_external_semaphore_capabilities: None,
            khr_device_group_creation: None,
            nn_vi_surface: None,
            khr_device_group: None,
            nv_external_memory_capabilities: None,
            ext_tooling_info: None,
            ext_full_screen_exclusive: None,
            khr_android_surface: None,
            khr_wayland_surface: None,
            khr_xcb_surface: None,
            khr_xlib_surface: None,
            khr_get_display_properties2: None,
            ext_headless_surface: None,
        })
    }
    #[inline]
    pub unsafe fn symbol(&self, name: &str) -> Option<std::num::NonZeroUsize> {
        let cstring = std::ffi::CString::new(name).unwrap();
        let ptr: usize = std::mem::transmute((self.loader)(self.handle, cstring.as_ptr()));
        std::num::NonZeroUsize::new(ptr)
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_vk1_0(&mut self) -> Option<()> {
        self.vk1_0 = crate::vk1_0::Vk10InstanceCommands::load(&self);
        self.vk1_0.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_vk1_1(&mut self) -> Option<()> {
        self.vk1_1 = crate::vk1_1::Vk11InstanceCommands::load(&self);
        self.vk1_1.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_khr_get_physical_device_properties2(&mut self) -> Option<()> {
        self . khr_get_physical_device_properties2 = crate :: extensions :: khr_get_physical_device_properties2 :: KhrGetPhysicalDeviceProperties2InstanceCommands :: load ( & self ) ;
        self.khr_get_physical_device_properties2
            .as_ref()
            .map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_khr_surface(&mut self) -> Option<()> {
        self.khr_surface = crate::extensions::khr_surface::KhrSurfaceInstanceCommands::load(&self);
        self.khr_surface.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_khr_swapchain(&mut self) -> Option<()> {
        self.khr_swapchain =
            crate::extensions::khr_swapchain::KhrSwapchainInstanceCommands::load(&self);
        self.khr_swapchain.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_khr_get_surface_capabilities2(&mut self) -> Option<()> {
        self . khr_get_surface_capabilities2 = crate :: extensions :: khr_get_surface_capabilities2 :: KhrGetSurfaceCapabilities2InstanceCommands :: load ( & self ) ;
        self.khr_get_surface_capabilities2.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_nv_cooperative_matrix(&mut self) -> Option<()> {
        self.nv_cooperative_matrix =
            crate::extensions::nv_cooperative_matrix::NvCooperativeMatrixInstanceCommands::load(
                &self,
            );
        self.nv_cooperative_matrix.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_ext_debug_report(&mut self) -> Option<()> {
        self.ext_debug_report =
            crate::extensions::ext_debug_report::ExtDebugReportInstanceCommands::load(&self);
        self.ext_debug_report.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_ext_metal_surface(&mut self) -> Option<()> {
        self.ext_metal_surface =
            crate::extensions::ext_metal_surface::ExtMetalSurfaceInstanceCommands::load(&self);
        self.ext_metal_surface.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_fuchsia_imagepipe_surface(&mut self) -> Option<()> {
        self . fuchsia_imagepipe_surface = crate :: extensions :: fuchsia_imagepipe_surface :: FuchsiaImagepipeSurfaceInstanceCommands :: load ( & self ) ;
        self.fuchsia_imagepipe_surface.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_ggp_stream_descriptor_surface(&mut self) -> Option<()> {
        self . ggp_stream_descriptor_surface = crate :: extensions :: ggp_stream_descriptor_surface :: GgpStreamDescriptorSurfaceInstanceCommands :: load ( & self ) ;
        self.ggp_stream_descriptor_surface.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_ext_calibrated_timestamps(&mut self) -> Option<()> {
        self . ext_calibrated_timestamps = crate :: extensions :: ext_calibrated_timestamps :: ExtCalibratedTimestampsInstanceCommands :: load ( & self ) ;
        self.ext_calibrated_timestamps.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_khr_external_memory_capabilities(&mut self) -> Option<()> {
        self . khr_external_memory_capabilities = crate :: extensions :: khr_external_memory_capabilities :: KhrExternalMemoryCapabilitiesInstanceCommands :: load ( & self ) ;
        self.khr_external_memory_capabilities.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_nv_coverage_reduction_mode(&mut self) -> Option<()> {
        self . nv_coverage_reduction_mode = crate :: extensions :: nv_coverage_reduction_mode :: NvCoverageReductionModeInstanceCommands :: load ( & self ) ;
        self.nv_coverage_reduction_mode.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_ext_sample_locations(&mut self) -> Option<()> {
        self.ext_sample_locations =
            crate::extensions::ext_sample_locations::ExtSampleLocationsInstanceCommands::load(
                &self,
            );
        self.ext_sample_locations.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_ext_debug_utils(&mut self) -> Option<()> {
        self.ext_debug_utils =
            crate::extensions::ext_debug_utils::ExtDebugUtilsInstanceCommands::load(&self);
        self.ext_debug_utils.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_mvk_macos_surface(&mut self) -> Option<()> {
        self.mvk_macos_surface =
            crate::extensions::mvk_macos_surface::MvkMacosSurfaceInstanceCommands::load(&self);
        self.mvk_macos_surface.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_mvk_ios_surface(&mut self) -> Option<()> {
        self.mvk_ios_surface =
            crate::extensions::mvk_ios_surface::MvkIosSurfaceInstanceCommands::load(&self);
        self.mvk_ios_surface.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_khr_display(&mut self) -> Option<()> {
        self.khr_display = crate::extensions::khr_display::KhrDisplayInstanceCommands::load(&self);
        self.khr_display.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_khr_win32_surface(&mut self) -> Option<()> {
        self.khr_win32_surface =
            crate::extensions::khr_win32_surface::KhrWin32SurfaceInstanceCommands::load(&self);
        self.khr_win32_surface.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_khr_performance_query(&mut self) -> Option<()> {
        self.khr_performance_query =
            crate::extensions::khr_performance_query::KhrPerformanceQueryInstanceCommands::load(
                &self,
            );
        self.khr_performance_query.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_khr_external_fence_capabilities(&mut self) -> Option<()> {
        self . khr_external_fence_capabilities = crate :: extensions :: khr_external_fence_capabilities :: KhrExternalFenceCapabilitiesInstanceCommands :: load ( & self ) ;
        self.khr_external_fence_capabilities.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_ext_display_surface_counter(&mut self) -> Option<()> {
        self . ext_display_surface_counter = crate :: extensions :: ext_display_surface_counter :: ExtDisplaySurfaceCounterInstanceCommands :: load ( & self ) ;
        self.ext_display_surface_counter.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_ext_direct_mode_display(&mut self) -> Option<()> {
        self.ext_direct_mode_display =
            crate::extensions::ext_direct_mode_display::ExtDirectModeDisplayInstanceCommands::load(
                &self,
            );
        self.ext_direct_mode_display.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_ext_acquire_xlib_display(&mut self) -> Option<()> {
        self . ext_acquire_xlib_display = crate :: extensions :: ext_acquire_xlib_display :: ExtAcquireXlibDisplayInstanceCommands :: load ( & self ) ;
        self.ext_acquire_xlib_display.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_khr_external_semaphore_capabilities(&mut self) -> Option<()> {
        self . khr_external_semaphore_capabilities = crate :: extensions :: khr_external_semaphore_capabilities :: KhrExternalSemaphoreCapabilitiesInstanceCommands :: load ( & self ) ;
        self.khr_external_semaphore_capabilities
            .as_ref()
            .map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_khr_device_group_creation(&mut self) -> Option<()> {
        self . khr_device_group_creation = crate :: extensions :: khr_device_group_creation :: KhrDeviceGroupCreationInstanceCommands :: load ( & self ) ;
        self.khr_device_group_creation.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_nn_vi_surface(&mut self) -> Option<()> {
        self.nn_vi_surface =
            crate::extensions::nn_vi_surface::NnViSurfaceInstanceCommands::load(&self);
        self.nn_vi_surface.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_khr_device_group(&mut self) -> Option<()> {
        self.khr_device_group =
            crate::extensions::khr_device_group::KhrDeviceGroupInstanceCommands::load(&self);
        self.khr_device_group.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_nv_external_memory_capabilities(&mut self) -> Option<()> {
        self . nv_external_memory_capabilities = crate :: extensions :: nv_external_memory_capabilities :: NvExternalMemoryCapabilitiesInstanceCommands :: load ( & self ) ;
        self.nv_external_memory_capabilities.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_ext_tooling_info(&mut self) -> Option<()> {
        self.ext_tooling_info =
            crate::extensions::ext_tooling_info::ExtToolingInfoInstanceCommands::load(&self);
        self.ext_tooling_info.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_ext_full_screen_exclusive(&mut self) -> Option<()> {
        self . ext_full_screen_exclusive = crate :: extensions :: ext_full_screen_exclusive :: ExtFullScreenExclusiveInstanceCommands :: load ( & self ) ;
        self.ext_full_screen_exclusive.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_khr_android_surface(&mut self) -> Option<()> {
        self.khr_android_surface =
            crate::extensions::khr_android_surface::KhrAndroidSurfaceInstanceCommands::load(&self);
        self.khr_android_surface.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_khr_wayland_surface(&mut self) -> Option<()> {
        self.khr_wayland_surface =
            crate::extensions::khr_wayland_surface::KhrWaylandSurfaceInstanceCommands::load(&self);
        self.khr_wayland_surface.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_khr_xcb_surface(&mut self) -> Option<()> {
        self.khr_xcb_surface =
            crate::extensions::khr_xcb_surface::KhrXcbSurfaceInstanceCommands::load(&self);
        self.khr_xcb_surface.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_khr_xlib_surface(&mut self) -> Option<()> {
        self.khr_xlib_surface =
            crate::extensions::khr_xlib_surface::KhrXlibSurfaceInstanceCommands::load(&self);
        self.khr_xlib_surface.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_khr_get_display_properties2(&mut self) -> Option<()> {
        self . khr_get_display_properties2 = crate :: extensions :: khr_get_display_properties2 :: KhrGetDisplayProperties2InstanceCommands :: load ( & self ) ;
        self.khr_get_display_properties2.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_ext_headless_surface(&mut self) -> Option<()> {
        self.ext_headless_surface =
            crate::extensions::ext_headless_surface::ExtHeadlessSurfaceInstanceCommands::load(
                &self,
            );
        self.ext_headless_surface.as_ref().map(|_| ())
    }
}
#[doc = r" Loader for Device Commands. **Must** outlive [`InstanceLoader`](struct.InstanceLoader.html)"]
pub struct DeviceLoader { pub loader : crate :: vk1_0 :: PFN_vkGetDeviceProcAddr , pub handle : crate :: vk1_0 :: Device , # [ doc = "Implemented in [`vk1_0::Vk10DeviceLoaderExt`](vk1_0/trait.Vk10DeviceLoaderExt.html)" ] pub vk1_0 : Option < crate :: vk1_0 :: Vk10DeviceCommands > , # [ doc = "Implemented in [`vk1_1::Vk11DeviceLoaderExt`](vk1_1/trait.Vk11DeviceLoaderExt.html)" ] pub vk1_1 : Option < crate :: vk1_1 :: Vk11DeviceCommands > , # [ doc = "Implemented in [`vk1_2::Vk12DeviceLoaderExt`](vk1_2/trait.Vk12DeviceLoaderExt.html)" ] pub vk1_2 : Option < crate :: vk1_2 :: Vk12DeviceCommands > , # [ doc = "Implemented in [`extensions::nv_device_generated_commands::NvDeviceGeneratedCommandsDeviceLoaderExt`](extensions/nv_device_generated_commands/trait.NvDeviceGeneratedCommandsDeviceLoaderExt.html)" ] pub nv_device_generated_commands : Option < crate :: extensions :: nv_device_generated_commands :: NvDeviceGeneratedCommandsDeviceCommands > , # [ doc = "Implemented in [`extensions::khr_pipeline_executable_properties::KhrPipelineExecutablePropertiesDeviceLoaderExt`](extensions/khr_pipeline_executable_properties/trait.KhrPipelineExecutablePropertiesDeviceLoaderExt.html)" ] pub khr_pipeline_executable_properties : Option < crate :: extensions :: khr_pipeline_executable_properties :: KhrPipelineExecutablePropertiesDeviceCommands > , # [ doc = "Implemented in [`extensions::khr_deferred_host_operations::KhrDeferredHostOperationsDeviceLoaderExt`](extensions/khr_deferred_host_operations/trait.KhrDeferredHostOperationsDeviceLoaderExt.html)" ] pub khr_deferred_host_operations : Option < crate :: extensions :: khr_deferred_host_operations :: KhrDeferredHostOperationsDeviceCommands > , # [ doc = "Implemented in [`extensions::ext_host_query_reset::ExtHostQueryResetDeviceLoaderExt`](extensions/ext_host_query_reset/trait.ExtHostQueryResetDeviceLoaderExt.html)" ] pub ext_host_query_reset : Option < crate :: extensions :: ext_host_query_reset :: ExtHostQueryResetDeviceCommands > , # [ doc = "Implemented in [`extensions::ext_line_rasterization::ExtLineRasterizationDeviceLoaderExt`](extensions/ext_line_rasterization/trait.ExtLineRasterizationDeviceLoaderExt.html)" ] pub ext_line_rasterization : Option < crate :: extensions :: ext_line_rasterization :: ExtLineRasterizationDeviceCommands > , # [ doc = "Implemented in [`extensions::khr_buffer_device_address::KhrBufferDeviceAddressDeviceLoaderExt`](extensions/khr_buffer_device_address/trait.KhrBufferDeviceAddressDeviceLoaderExt.html)" ] pub khr_buffer_device_address : Option < crate :: extensions :: khr_buffer_device_address :: KhrBufferDeviceAddressDeviceCommands > , # [ doc = "Implemented in [`extensions::khr_swapchain::KhrSwapchainDeviceLoaderExt`](extensions/khr_swapchain/trait.KhrSwapchainDeviceLoaderExt.html)" ] pub khr_swapchain : Option < crate :: extensions :: khr_swapchain :: KhrSwapchainDeviceCommands > , # [ doc = "Implemented in [`extensions::khr_maintenance1::KhrMaintenance1DeviceLoaderExt`](extensions/khr_maintenance1/trait.KhrMaintenance1DeviceLoaderExt.html)" ] pub khr_maintenance1 : Option < crate :: extensions :: khr_maintenance1 :: KhrMaintenance1DeviceCommands > , # [ doc = "Implemented in [`extensions::ext_buffer_device_address::ExtBufferDeviceAddressDeviceLoaderExt`](extensions/ext_buffer_device_address/trait.ExtBufferDeviceAddressDeviceLoaderExt.html)" ] pub ext_buffer_device_address : Option < crate :: extensions :: ext_buffer_device_address :: ExtBufferDeviceAddressDeviceCommands > , # [ doc = "Implemented in [`extensions::khr_get_memory_requirements2::KhrGetMemoryRequirements2DeviceLoaderExt`](extensions/khr_get_memory_requirements2/trait.KhrGetMemoryRequirements2DeviceLoaderExt.html)" ] pub khr_get_memory_requirements2 : Option < crate :: extensions :: khr_get_memory_requirements2 :: KhrGetMemoryRequirements2DeviceCommands > , # [ doc = "Implemented in [`extensions::amd_display_native_hdr::AmdDisplayNativeHdrDeviceLoaderExt`](extensions/amd_display_native_hdr/trait.AmdDisplayNativeHdrDeviceLoaderExt.html)" ] pub amd_display_native_hdr : Option < crate :: extensions :: amd_display_native_hdr :: AmdDisplayNativeHdrDeviceCommands > , # [ doc = "Implemented in [`extensions::intel_performance_query::IntelPerformanceQueryDeviceLoaderExt`](extensions/intel_performance_query/trait.IntelPerformanceQueryDeviceLoaderExt.html)" ] pub intel_performance_query : Option < crate :: extensions :: intel_performance_query :: IntelPerformanceQueryDeviceCommands > , # [ doc = "Implemented in [`extensions::khr_timeline_semaphore::KhrTimelineSemaphoreDeviceLoaderExt`](extensions/khr_timeline_semaphore/trait.KhrTimelineSemaphoreDeviceLoaderExt.html)" ] pub khr_timeline_semaphore : Option < crate :: extensions :: khr_timeline_semaphore :: KhrTimelineSemaphoreDeviceCommands > , # [ doc = "Implemented in [`extensions::nv_device_diagnostic_checkpoints::NvDeviceDiagnosticCheckpointsDeviceLoaderExt`](extensions/nv_device_diagnostic_checkpoints/trait.NvDeviceDiagnosticCheckpointsDeviceLoaderExt.html)" ] pub nv_device_diagnostic_checkpoints : Option < crate :: extensions :: nv_device_diagnostic_checkpoints :: NvDeviceDiagnosticCheckpointsDeviceCommands > , # [ doc = "Implemented in [`extensions::nv_scissor_exclusive::NvScissorExclusiveDeviceLoaderExt`](extensions/nv_scissor_exclusive/trait.NvScissorExclusiveDeviceLoaderExt.html)" ] pub nv_scissor_exclusive : Option < crate :: extensions :: nv_scissor_exclusive :: NvScissorExclusiveDeviceCommands > , # [ doc = "Implemented in [`extensions::nv_mesh_shader::NvMeshShaderDeviceLoaderExt`](extensions/nv_mesh_shader/trait.NvMeshShaderDeviceLoaderExt.html)" ] pub nv_mesh_shader : Option < crate :: extensions :: nv_mesh_shader :: NvMeshShaderDeviceCommands > , # [ doc = "Implemented in [`extensions::khr_create_renderpass2::KhrCreateRenderpass2DeviceLoaderExt`](extensions/khr_create_renderpass2/trait.KhrCreateRenderpass2DeviceLoaderExt.html)" ] pub khr_create_renderpass2 : Option < crate :: extensions :: khr_create_renderpass2 :: KhrCreateRenderpass2DeviceCommands > , # [ doc = "Implemented in [`extensions::ext_calibrated_timestamps::ExtCalibratedTimestampsDeviceLoaderExt`](extensions/ext_calibrated_timestamps/trait.ExtCalibratedTimestampsDeviceLoaderExt.html)" ] pub ext_calibrated_timestamps : Option < crate :: extensions :: ext_calibrated_timestamps :: ExtCalibratedTimestampsDeviceCommands > , # [ doc = "Implemented in [`extensions::amd_buffer_marker::AmdBufferMarkerDeviceLoaderExt`](extensions/amd_buffer_marker/trait.AmdBufferMarkerDeviceLoaderExt.html)" ] pub amd_buffer_marker : Option < crate :: extensions :: amd_buffer_marker :: AmdBufferMarkerDeviceCommands > , # [ doc = "Implemented in [`extensions::khr_draw_indirect_count::KhrDrawIndirectCountDeviceLoaderExt`](extensions/khr_draw_indirect_count/trait.KhrDrawIndirectCountDeviceLoaderExt.html)" ] pub khr_draw_indirect_count : Option < crate :: extensions :: khr_draw_indirect_count :: KhrDrawIndirectCountDeviceCommands > , # [ doc = "Implemented in [`extensions::khr_maintenance3::KhrMaintenance3DeviceLoaderExt`](extensions/khr_maintenance3/trait.KhrMaintenance3DeviceLoaderExt.html)" ] pub khr_maintenance3 : Option < crate :: extensions :: khr_maintenance3 :: KhrMaintenance3DeviceCommands > , # [ doc = "Implemented in [`extensions::nv_shading_rate_image::NvShadingRateImageDeviceLoaderExt`](extensions/nv_shading_rate_image/trait.NvShadingRateImageDeviceLoaderExt.html)" ] pub nv_shading_rate_image : Option < crate :: extensions :: nv_shading_rate_image :: NvShadingRateImageDeviceCommands > , # [ doc = "Implemented in [`extensions::ext_validation_cache::ExtValidationCacheDeviceLoaderExt`](extensions/ext_validation_cache/trait.ExtValidationCacheDeviceLoaderExt.html)" ] pub ext_validation_cache : Option < crate :: extensions :: ext_validation_cache :: ExtValidationCacheDeviceCommands > , # [ doc = "Implemented in [`extensions::khr_bind_memory2::KhrBindMemory2DeviceLoaderExt`](extensions/khr_bind_memory2/trait.KhrBindMemory2DeviceLoaderExt.html)" ] pub khr_bind_memory2 : Option < crate :: extensions :: khr_bind_memory2 :: KhrBindMemory2DeviceCommands > , # [ doc = "Implemented in [`extensions::khr_sampler_ycbcr_conversion::KhrSamplerYcbcrConversionDeviceLoaderExt`](extensions/khr_sampler_ycbcr_conversion/trait.KhrSamplerYcbcrConversionDeviceLoaderExt.html)" ] pub khr_sampler_ycbcr_conversion : Option < crate :: extensions :: khr_sampler_ycbcr_conversion :: KhrSamplerYcbcrConversionDeviceCommands > , # [ doc = "Implemented in [`extensions::khr_ray_tracing::KhrRayTracingDeviceLoaderExt`](extensions/khr_ray_tracing/trait.KhrRayTracingDeviceLoaderExt.html)" ] pub khr_ray_tracing : Option < crate :: extensions :: khr_ray_tracing :: KhrRayTracingDeviceCommands > , # [ doc = "Implemented in [`extensions::nv_ray_tracing::NvRayTracingDeviceLoaderExt`](extensions/nv_ray_tracing/trait.NvRayTracingDeviceLoaderExt.html)" ] pub nv_ray_tracing : Option < crate :: extensions :: nv_ray_tracing :: NvRayTracingDeviceCommands > , # [ doc = "Implemented in [`extensions::ext_sample_locations::ExtSampleLocationsDeviceLoaderExt`](extensions/ext_sample_locations/trait.ExtSampleLocationsDeviceLoaderExt.html)" ] pub ext_sample_locations : Option < crate :: extensions :: ext_sample_locations :: ExtSampleLocationsDeviceCommands > , # [ doc = "Implemented in [`extensions::ext_debug_utils::ExtDebugUtilsDeviceLoaderExt`](extensions/ext_debug_utils/trait.ExtDebugUtilsDeviceLoaderExt.html)" ] pub ext_debug_utils : Option < crate :: extensions :: ext_debug_utils :: ExtDebugUtilsDeviceCommands > , # [ doc = "Implemented in [`extensions::android_external_memory_android_hardware_buffer::AndroidExternalMemoryAndroidHardwareBufferDeviceLoaderExt`](extensions/android_external_memory_android_hardware_buffer/trait.AndroidExternalMemoryAndroidHardwareBufferDeviceLoaderExt.html)" ] pub android_external_memory_android_hardware_buffer : Option < crate :: extensions :: android_external_memory_android_hardware_buffer :: AndroidExternalMemoryAndroidHardwareBufferDeviceCommands > , # [ doc = "Implemented in [`extensions::khr_performance_query::KhrPerformanceQueryDeviceLoaderExt`](extensions/khr_performance_query/trait.KhrPerformanceQueryDeviceLoaderExt.html)" ] pub khr_performance_query : Option < crate :: extensions :: khr_performance_query :: KhrPerformanceQueryDeviceCommands > , # [ doc = "Implemented in [`extensions::khr_external_fence_win32::KhrExternalFenceWin32DeviceLoaderExt`](extensions/khr_external_fence_win32/trait.KhrExternalFenceWin32DeviceLoaderExt.html)" ] pub khr_external_fence_win32 : Option < crate :: extensions :: khr_external_fence_win32 :: KhrExternalFenceWin32DeviceCommands > , # [ doc = "Implemented in [`extensions::khr_external_fence_fd::KhrExternalFenceFdDeviceLoaderExt`](extensions/khr_external_fence_fd/trait.KhrExternalFenceFdDeviceLoaderExt.html)" ] pub khr_external_fence_fd : Option < crate :: extensions :: khr_external_fence_fd :: KhrExternalFenceFdDeviceCommands > , # [ doc = "Implemented in [`extensions::khr_shared_presentable_image::KhrSharedPresentableImageDeviceLoaderExt`](extensions/khr_shared_presentable_image/trait.KhrSharedPresentableImageDeviceLoaderExt.html)" ] pub khr_shared_presentable_image : Option < crate :: extensions :: khr_shared_presentable_image :: KhrSharedPresentableImageDeviceCommands > , # [ doc = "Implemented in [`extensions::ext_hdr_metadata::ExtHdrMetadataDeviceLoaderExt`](extensions/ext_hdr_metadata/trait.ExtHdrMetadataDeviceLoaderExt.html)" ] pub ext_hdr_metadata : Option < crate :: extensions :: ext_hdr_metadata :: ExtHdrMetadataDeviceCommands > , # [ doc = "Implemented in [`extensions::ext_discard_rectangles::ExtDiscardRectanglesDeviceLoaderExt`](extensions/ext_discard_rectangles/trait.ExtDiscardRectanglesDeviceLoaderExt.html)" ] pub ext_discard_rectangles : Option < crate :: extensions :: ext_discard_rectangles :: ExtDiscardRectanglesDeviceCommands > , # [ doc = "Implemented in [`extensions::google_display_timing::GoogleDisplayTimingDeviceLoaderExt`](extensions/google_display_timing/trait.GoogleDisplayTimingDeviceLoaderExt.html)" ] pub google_display_timing : Option < crate :: extensions :: google_display_timing :: GoogleDisplayTimingDeviceCommands > , # [ doc = "Implemented in [`extensions::ext_display_control::ExtDisplayControlDeviceLoaderExt`](extensions/ext_display_control/trait.ExtDisplayControlDeviceLoaderExt.html)" ] pub ext_display_control : Option < crate :: extensions :: ext_display_control :: ExtDisplayControlDeviceCommands > , # [ doc = "Implemented in [`extensions::nv_clip_space_w_scaling::NvClipSpaceWScalingDeviceLoaderExt`](extensions/nv_clip_space_w_scaling/trait.NvClipSpaceWScalingDeviceLoaderExt.html)" ] pub nv_clip_space_w_scaling : Option < crate :: extensions :: nv_clip_space_w_scaling :: NvClipSpaceWScalingDeviceCommands > , # [ doc = "Implemented in [`extensions::khr_push_descriptor::KhrPushDescriptorDeviceLoaderExt`](extensions/khr_push_descriptor/trait.KhrPushDescriptorDeviceLoaderExt.html)" ] pub khr_push_descriptor : Option < crate :: extensions :: khr_push_descriptor :: KhrPushDescriptorDeviceCommands > , # [ doc = "Implemented in [`extensions::ext_conditional_rendering::ExtConditionalRenderingDeviceLoaderExt`](extensions/ext_conditional_rendering/trait.ExtConditionalRenderingDeviceLoaderExt.html)" ] pub ext_conditional_rendering : Option < crate :: extensions :: ext_conditional_rendering :: ExtConditionalRenderingDeviceCommands > , # [ doc = "Implemented in [`extensions::khr_descriptor_update_template::KhrDescriptorUpdateTemplateDeviceLoaderExt`](extensions/khr_descriptor_update_template/trait.KhrDescriptorUpdateTemplateDeviceLoaderExt.html)" ] pub khr_descriptor_update_template : Option < crate :: extensions :: khr_descriptor_update_template :: KhrDescriptorUpdateTemplateDeviceCommands > , # [ doc = "Implemented in [`extensions::khr_external_semaphore_win32::KhrExternalSemaphoreWin32DeviceLoaderExt`](extensions/khr_external_semaphore_win32/trait.KhrExternalSemaphoreWin32DeviceLoaderExt.html)" ] pub khr_external_semaphore_win32 : Option < crate :: extensions :: khr_external_semaphore_win32 :: KhrExternalSemaphoreWin32DeviceCommands > , # [ doc = "Implemented in [`extensions::khr_external_semaphore_fd::KhrExternalSemaphoreFdDeviceLoaderExt`](extensions/khr_external_semaphore_fd/trait.KhrExternalSemaphoreFdDeviceLoaderExt.html)" ] pub khr_external_semaphore_fd : Option < crate :: extensions :: khr_external_semaphore_fd :: KhrExternalSemaphoreFdDeviceCommands > , # [ doc = "Implemented in [`extensions::khr_external_memory_fd::KhrExternalMemoryFdDeviceLoaderExt`](extensions/khr_external_memory_fd/trait.KhrExternalMemoryFdDeviceLoaderExt.html)" ] pub khr_external_memory_fd : Option < crate :: extensions :: khr_external_memory_fd :: KhrExternalMemoryFdDeviceCommands > , # [ doc = "Implemented in [`extensions::khr_external_memory_win32::KhrExternalMemoryWin32DeviceLoaderExt`](extensions/khr_external_memory_win32/trait.KhrExternalMemoryWin32DeviceLoaderExt.html)" ] pub khr_external_memory_win32 : Option < crate :: extensions :: khr_external_memory_win32 :: KhrExternalMemoryWin32DeviceCommands > , # [ doc = "Implemented in [`extensions::ext_external_memory_host::ExtExternalMemoryHostDeviceLoaderExt`](extensions/ext_external_memory_host/trait.ExtExternalMemoryHostDeviceLoaderExt.html)" ] pub ext_external_memory_host : Option < crate :: extensions :: ext_external_memory_host :: ExtExternalMemoryHostDeviceCommands > , # [ doc = "Implemented in [`extensions::ext_image_drm_format_modifier::ExtImageDrmFormatModifierDeviceLoaderExt`](extensions/ext_image_drm_format_modifier/trait.ExtImageDrmFormatModifierDeviceLoaderExt.html)" ] pub ext_image_drm_format_modifier : Option < crate :: extensions :: ext_image_drm_format_modifier :: ExtImageDrmFormatModifierDeviceCommands > , # [ doc = "Implemented in [`extensions::khr_device_group::KhrDeviceGroupDeviceLoaderExt`](extensions/khr_device_group/trait.KhrDeviceGroupDeviceLoaderExt.html)" ] pub khr_device_group : Option < crate :: extensions :: khr_device_group :: KhrDeviceGroupDeviceCommands > , # [ doc = "Implemented in [`extensions::nv_external_memory_win32::NvExternalMemoryWin32DeviceLoaderExt`](extensions/nv_external_memory_win32/trait.NvExternalMemoryWin32DeviceLoaderExt.html)" ] pub nv_external_memory_win32 : Option < crate :: extensions :: nv_external_memory_win32 :: NvExternalMemoryWin32DeviceCommands > , # [ doc = "Implemented in [`extensions::amd_shader_info::AmdShaderInfoDeviceLoaderExt`](extensions/amd_shader_info/trait.AmdShaderInfoDeviceLoaderExt.html)" ] pub amd_shader_info : Option < crate :: extensions :: amd_shader_info :: AmdShaderInfoDeviceCommands > , # [ doc = "Implemented in [`extensions::amd_draw_indirect_count::AmdDrawIndirectCountDeviceLoaderExt`](extensions/amd_draw_indirect_count/trait.AmdDrawIndirectCountDeviceLoaderExt.html)" ] pub amd_draw_indirect_count : Option < crate :: extensions :: amd_draw_indirect_count :: AmdDrawIndirectCountDeviceCommands > , # [ doc = "Implemented in [`extensions::nvx_image_view_handle::NvxImageViewHandleDeviceLoaderExt`](extensions/nvx_image_view_handle/trait.NvxImageViewHandleDeviceLoaderExt.html)" ] pub nvx_image_view_handle : Option < crate :: extensions :: nvx_image_view_handle :: NvxImageViewHandleDeviceCommands > , # [ doc = "Implemented in [`extensions::ext_transform_feedback::ExtTransformFeedbackDeviceLoaderExt`](extensions/ext_transform_feedback/trait.ExtTransformFeedbackDeviceLoaderExt.html)" ] pub ext_transform_feedback : Option < crate :: extensions :: ext_transform_feedback :: ExtTransformFeedbackDeviceCommands > , # [ doc = "Implemented in [`extensions::ext_debug_marker::ExtDebugMarkerDeviceLoaderExt`](extensions/ext_debug_marker/trait.ExtDebugMarkerDeviceLoaderExt.html)" ] pub ext_debug_marker : Option < crate :: extensions :: ext_debug_marker :: ExtDebugMarkerDeviceCommands > , # [ doc = "Implemented in [`extensions::ext_full_screen_exclusive::ExtFullScreenExclusiveDeviceLoaderExt`](extensions/ext_full_screen_exclusive/trait.ExtFullScreenExclusiveDeviceLoaderExt.html)" ] pub ext_full_screen_exclusive : Option < crate :: extensions :: ext_full_screen_exclusive :: ExtFullScreenExclusiveDeviceCommands > , # [ doc = "Implemented in [`extensions::khr_display_swapchain::KhrDisplaySwapchainDeviceLoaderExt`](extensions/khr_display_swapchain/trait.KhrDisplaySwapchainDeviceLoaderExt.html)" ] pub khr_display_swapchain : Option < crate :: extensions :: khr_display_swapchain :: KhrDisplaySwapchainDeviceCommands > , }
impl DeviceLoader {
    pub fn new(
        instance_loader: &InstanceLoader,
        device: crate::vk1_0::Device,
    ) -> Option<DeviceLoader> {
        Some(DeviceLoader {
            loader: unsafe { std::mem::transmute(instance_loader.symbol("vkGetDeviceProcAddr")?) },
            handle: device,
            vk1_0: None,
            vk1_1: None,
            vk1_2: None,
            nv_device_generated_commands: None,
            khr_pipeline_executable_properties: None,
            khr_deferred_host_operations: None,
            ext_host_query_reset: None,
            ext_line_rasterization: None,
            khr_buffer_device_address: None,
            khr_swapchain: None,
            khr_maintenance1: None,
            ext_buffer_device_address: None,
            khr_get_memory_requirements2: None,
            amd_display_native_hdr: None,
            intel_performance_query: None,
            khr_timeline_semaphore: None,
            nv_device_diagnostic_checkpoints: None,
            nv_scissor_exclusive: None,
            nv_mesh_shader: None,
            khr_create_renderpass2: None,
            ext_calibrated_timestamps: None,
            amd_buffer_marker: None,
            khr_draw_indirect_count: None,
            khr_maintenance3: None,
            nv_shading_rate_image: None,
            ext_validation_cache: None,
            khr_bind_memory2: None,
            khr_sampler_ycbcr_conversion: None,
            khr_ray_tracing: None,
            nv_ray_tracing: None,
            ext_sample_locations: None,
            ext_debug_utils: None,
            android_external_memory_android_hardware_buffer: None,
            khr_performance_query: None,
            khr_external_fence_win32: None,
            khr_external_fence_fd: None,
            khr_shared_presentable_image: None,
            ext_hdr_metadata: None,
            ext_discard_rectangles: None,
            google_display_timing: None,
            ext_display_control: None,
            nv_clip_space_w_scaling: None,
            khr_push_descriptor: None,
            ext_conditional_rendering: None,
            khr_descriptor_update_template: None,
            khr_external_semaphore_win32: None,
            khr_external_semaphore_fd: None,
            khr_external_memory_fd: None,
            khr_external_memory_win32: None,
            ext_external_memory_host: None,
            ext_image_drm_format_modifier: None,
            khr_device_group: None,
            nv_external_memory_win32: None,
            amd_shader_info: None,
            amd_draw_indirect_count: None,
            nvx_image_view_handle: None,
            ext_transform_feedback: None,
            ext_debug_marker: None,
            ext_full_screen_exclusive: None,
            khr_display_swapchain: None,
        })
    }
    #[inline]
    pub unsafe fn symbol(&self, name: &str) -> Option<std::num::NonZeroUsize> {
        let cstring = std::ffi::CString::new(name).unwrap();
        let ptr: usize = std::mem::transmute((self.loader)(self.handle, cstring.as_ptr()));
        std::num::NonZeroUsize::new(ptr)
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_vk1_0(&mut self) -> Option<()> {
        self.vk1_0 = crate::vk1_0::Vk10DeviceCommands::load(&self);
        self.vk1_0.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_vk1_1(&mut self) -> Option<()> {
        self.vk1_1 = crate::vk1_1::Vk11DeviceCommands::load(&self);
        self.vk1_1.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_vk1_2(&mut self) -> Option<()> {
        self.vk1_2 = crate::vk1_2::Vk12DeviceCommands::load(&self);
        self.vk1_2.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_nv_device_generated_commands(&mut self) -> Option<()> {
        self . nv_device_generated_commands = crate :: extensions :: nv_device_generated_commands :: NvDeviceGeneratedCommandsDeviceCommands :: load ( & self ) ;
        self.nv_device_generated_commands.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_khr_pipeline_executable_properties(&mut self) -> Option<()> {
        self . khr_pipeline_executable_properties = crate :: extensions :: khr_pipeline_executable_properties :: KhrPipelineExecutablePropertiesDeviceCommands :: load ( & self ) ;
        self.khr_pipeline_executable_properties.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_khr_deferred_host_operations(&mut self) -> Option<()> {
        self . khr_deferred_host_operations = crate :: extensions :: khr_deferred_host_operations :: KhrDeferredHostOperationsDeviceCommands :: load ( & self ) ;
        self.khr_deferred_host_operations.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_ext_host_query_reset(&mut self) -> Option<()> {
        self.ext_host_query_reset =
            crate::extensions::ext_host_query_reset::ExtHostQueryResetDeviceCommands::load(&self);
        self.ext_host_query_reset.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_ext_line_rasterization(&mut self) -> Option<()> {
        self.ext_line_rasterization =
            crate::extensions::ext_line_rasterization::ExtLineRasterizationDeviceCommands::load(
                &self,
            );
        self.ext_line_rasterization.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_khr_buffer_device_address(&mut self) -> Option<()> {
        self . khr_buffer_device_address = crate :: extensions :: khr_buffer_device_address :: KhrBufferDeviceAddressDeviceCommands :: load ( & self ) ;
        self.khr_buffer_device_address.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_khr_swapchain(&mut self) -> Option<()> {
        self.khr_swapchain =
            crate::extensions::khr_swapchain::KhrSwapchainDeviceCommands::load(&self);
        self.khr_swapchain.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_khr_maintenance1(&mut self) -> Option<()> {
        self.khr_maintenance1 =
            crate::extensions::khr_maintenance1::KhrMaintenance1DeviceCommands::load(&self);
        self.khr_maintenance1.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_ext_buffer_device_address(&mut self) -> Option<()> {
        self . ext_buffer_device_address = crate :: extensions :: ext_buffer_device_address :: ExtBufferDeviceAddressDeviceCommands :: load ( & self ) ;
        self.ext_buffer_device_address.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_khr_get_memory_requirements2(&mut self) -> Option<()> {
        self . khr_get_memory_requirements2 = crate :: extensions :: khr_get_memory_requirements2 :: KhrGetMemoryRequirements2DeviceCommands :: load ( & self ) ;
        self.khr_get_memory_requirements2.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_amd_display_native_hdr(&mut self) -> Option<()> {
        self.amd_display_native_hdr =
            crate::extensions::amd_display_native_hdr::AmdDisplayNativeHdrDeviceCommands::load(
                &self,
            );
        self.amd_display_native_hdr.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_intel_performance_query(&mut self) -> Option<()> {
        self.intel_performance_query =
            crate::extensions::intel_performance_query::IntelPerformanceQueryDeviceCommands::load(
                &self,
            );
        self.intel_performance_query.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_khr_timeline_semaphore(&mut self) -> Option<()> {
        self.khr_timeline_semaphore =
            crate::extensions::khr_timeline_semaphore::KhrTimelineSemaphoreDeviceCommands::load(
                &self,
            );
        self.khr_timeline_semaphore.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_nv_device_diagnostic_checkpoints(&mut self) -> Option<()> {
        self . nv_device_diagnostic_checkpoints = crate :: extensions :: nv_device_diagnostic_checkpoints :: NvDeviceDiagnosticCheckpointsDeviceCommands :: load ( & self ) ;
        self.nv_device_diagnostic_checkpoints.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_nv_scissor_exclusive(&mut self) -> Option<()> {
        self.nv_scissor_exclusive =
            crate::extensions::nv_scissor_exclusive::NvScissorExclusiveDeviceCommands::load(&self);
        self.nv_scissor_exclusive.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_nv_mesh_shader(&mut self) -> Option<()> {
        self.nv_mesh_shader =
            crate::extensions::nv_mesh_shader::NvMeshShaderDeviceCommands::load(&self);
        self.nv_mesh_shader.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_khr_create_renderpass2(&mut self) -> Option<()> {
        self.khr_create_renderpass2 =
            crate::extensions::khr_create_renderpass2::KhrCreateRenderpass2DeviceCommands::load(
                &self,
            );
        self.khr_create_renderpass2.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_ext_calibrated_timestamps(&mut self) -> Option<()> {
        self . ext_calibrated_timestamps = crate :: extensions :: ext_calibrated_timestamps :: ExtCalibratedTimestampsDeviceCommands :: load ( & self ) ;
        self.ext_calibrated_timestamps.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_amd_buffer_marker(&mut self) -> Option<()> {
        self.amd_buffer_marker =
            crate::extensions::amd_buffer_marker::AmdBufferMarkerDeviceCommands::load(&self);
        self.amd_buffer_marker.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_khr_draw_indirect_count(&mut self) -> Option<()> {
        self.khr_draw_indirect_count =
            crate::extensions::khr_draw_indirect_count::KhrDrawIndirectCountDeviceCommands::load(
                &self,
            );
        self.khr_draw_indirect_count.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_khr_maintenance3(&mut self) -> Option<()> {
        self.khr_maintenance3 =
            crate::extensions::khr_maintenance3::KhrMaintenance3DeviceCommands::load(&self);
        self.khr_maintenance3.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_nv_shading_rate_image(&mut self) -> Option<()> {
        self.nv_shading_rate_image =
            crate::extensions::nv_shading_rate_image::NvShadingRateImageDeviceCommands::load(&self);
        self.nv_shading_rate_image.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_ext_validation_cache(&mut self) -> Option<()> {
        self.ext_validation_cache =
            crate::extensions::ext_validation_cache::ExtValidationCacheDeviceCommands::load(&self);
        self.ext_validation_cache.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_khr_bind_memory2(&mut self) -> Option<()> {
        self.khr_bind_memory2 =
            crate::extensions::khr_bind_memory2::KhrBindMemory2DeviceCommands::load(&self);
        self.khr_bind_memory2.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_khr_sampler_ycbcr_conversion(&mut self) -> Option<()> {
        self . khr_sampler_ycbcr_conversion = crate :: extensions :: khr_sampler_ycbcr_conversion :: KhrSamplerYcbcrConversionDeviceCommands :: load ( & self ) ;
        self.khr_sampler_ycbcr_conversion.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_khr_ray_tracing(&mut self) -> Option<()> {
        self.khr_ray_tracing =
            crate::extensions::khr_ray_tracing::KhrRayTracingDeviceCommands::load(&self);
        self.khr_ray_tracing.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_nv_ray_tracing(&mut self) -> Option<()> {
        self.nv_ray_tracing =
            crate::extensions::nv_ray_tracing::NvRayTracingDeviceCommands::load(&self);
        self.nv_ray_tracing.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_ext_sample_locations(&mut self) -> Option<()> {
        self.ext_sample_locations =
            crate::extensions::ext_sample_locations::ExtSampleLocationsDeviceCommands::load(&self);
        self.ext_sample_locations.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_ext_debug_utils(&mut self) -> Option<()> {
        self.ext_debug_utils =
            crate::extensions::ext_debug_utils::ExtDebugUtilsDeviceCommands::load(&self);
        self.ext_debug_utils.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_android_external_memory_android_hardware_buffer(&mut self) -> Option<()> {
        self . android_external_memory_android_hardware_buffer = crate :: extensions :: android_external_memory_android_hardware_buffer :: AndroidExternalMemoryAndroidHardwareBufferDeviceCommands :: load ( & self ) ;
        self.android_external_memory_android_hardware_buffer
            .as_ref()
            .map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_khr_performance_query(&mut self) -> Option<()> {
        self.khr_performance_query =
            crate::extensions::khr_performance_query::KhrPerformanceQueryDeviceCommands::load(
                &self,
            );
        self.khr_performance_query.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_khr_external_fence_win32(&mut self) -> Option<()> {
        self.khr_external_fence_win32 =
            crate::extensions::khr_external_fence_win32::KhrExternalFenceWin32DeviceCommands::load(
                &self,
            );
        self.khr_external_fence_win32.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_khr_external_fence_fd(&mut self) -> Option<()> {
        self.khr_external_fence_fd =
            crate::extensions::khr_external_fence_fd::KhrExternalFenceFdDeviceCommands::load(&self);
        self.khr_external_fence_fd.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_khr_shared_presentable_image(&mut self) -> Option<()> {
        self . khr_shared_presentable_image = crate :: extensions :: khr_shared_presentable_image :: KhrSharedPresentableImageDeviceCommands :: load ( & self ) ;
        self.khr_shared_presentable_image.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_ext_hdr_metadata(&mut self) -> Option<()> {
        self.ext_hdr_metadata =
            crate::extensions::ext_hdr_metadata::ExtHdrMetadataDeviceCommands::load(&self);
        self.ext_hdr_metadata.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_ext_discard_rectangles(&mut self) -> Option<()> {
        self.ext_discard_rectangles =
            crate::extensions::ext_discard_rectangles::ExtDiscardRectanglesDeviceCommands::load(
                &self,
            );
        self.ext_discard_rectangles.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_google_display_timing(&mut self) -> Option<()> {
        self.google_display_timing =
            crate::extensions::google_display_timing::GoogleDisplayTimingDeviceCommands::load(
                &self,
            );
        self.google_display_timing.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_ext_display_control(&mut self) -> Option<()> {
        self.ext_display_control =
            crate::extensions::ext_display_control::ExtDisplayControlDeviceCommands::load(&self);
        self.ext_display_control.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_nv_clip_space_w_scaling(&mut self) -> Option<()> {
        self.nv_clip_space_w_scaling =
            crate::extensions::nv_clip_space_w_scaling::NvClipSpaceWScalingDeviceCommands::load(
                &self,
            );
        self.nv_clip_space_w_scaling.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_khr_push_descriptor(&mut self) -> Option<()> {
        self.khr_push_descriptor =
            crate::extensions::khr_push_descriptor::KhrPushDescriptorDeviceCommands::load(&self);
        self.khr_push_descriptor.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_ext_conditional_rendering(&mut self) -> Option<()> {
        self . ext_conditional_rendering = crate :: extensions :: ext_conditional_rendering :: ExtConditionalRenderingDeviceCommands :: load ( & self ) ;
        self.ext_conditional_rendering.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_khr_descriptor_update_template(&mut self) -> Option<()> {
        self . khr_descriptor_update_template = crate :: extensions :: khr_descriptor_update_template :: KhrDescriptorUpdateTemplateDeviceCommands :: load ( & self ) ;
        self.khr_descriptor_update_template.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_khr_external_semaphore_win32(&mut self) -> Option<()> {
        self . khr_external_semaphore_win32 = crate :: extensions :: khr_external_semaphore_win32 :: KhrExternalSemaphoreWin32DeviceCommands :: load ( & self ) ;
        self.khr_external_semaphore_win32.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_khr_external_semaphore_fd(&mut self) -> Option<()> {
        self . khr_external_semaphore_fd = crate :: extensions :: khr_external_semaphore_fd :: KhrExternalSemaphoreFdDeviceCommands :: load ( & self ) ;
        self.khr_external_semaphore_fd.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_khr_external_memory_fd(&mut self) -> Option<()> {
        self.khr_external_memory_fd =
            crate::extensions::khr_external_memory_fd::KhrExternalMemoryFdDeviceCommands::load(
                &self,
            );
        self.khr_external_memory_fd.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_khr_external_memory_win32(&mut self) -> Option<()> {
        self . khr_external_memory_win32 = crate :: extensions :: khr_external_memory_win32 :: KhrExternalMemoryWin32DeviceCommands :: load ( & self ) ;
        self.khr_external_memory_win32.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_ext_external_memory_host(&mut self) -> Option<()> {
        self.ext_external_memory_host =
            crate::extensions::ext_external_memory_host::ExtExternalMemoryHostDeviceCommands::load(
                &self,
            );
        self.ext_external_memory_host.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_ext_image_drm_format_modifier(&mut self) -> Option<()> {
        self . ext_image_drm_format_modifier = crate :: extensions :: ext_image_drm_format_modifier :: ExtImageDrmFormatModifierDeviceCommands :: load ( & self ) ;
        self.ext_image_drm_format_modifier.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_khr_device_group(&mut self) -> Option<()> {
        self.khr_device_group =
            crate::extensions::khr_device_group::KhrDeviceGroupDeviceCommands::load(&self);
        self.khr_device_group.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_nv_external_memory_win32(&mut self) -> Option<()> {
        self.nv_external_memory_win32 =
            crate::extensions::nv_external_memory_win32::NvExternalMemoryWin32DeviceCommands::load(
                &self,
            );
        self.nv_external_memory_win32.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_amd_shader_info(&mut self) -> Option<()> {
        self.amd_shader_info =
            crate::extensions::amd_shader_info::AmdShaderInfoDeviceCommands::load(&self);
        self.amd_shader_info.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_amd_draw_indirect_count(&mut self) -> Option<()> {
        self.amd_draw_indirect_count =
            crate::extensions::amd_draw_indirect_count::AmdDrawIndirectCountDeviceCommands::load(
                &self,
            );
        self.amd_draw_indirect_count.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_nvx_image_view_handle(&mut self) -> Option<()> {
        self.nvx_image_view_handle =
            crate::extensions::nvx_image_view_handle::NvxImageViewHandleDeviceCommands::load(&self);
        self.nvx_image_view_handle.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_ext_transform_feedback(&mut self) -> Option<()> {
        self.ext_transform_feedback =
            crate::extensions::ext_transform_feedback::ExtTransformFeedbackDeviceCommands::load(
                &self,
            );
        self.ext_transform_feedback.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_ext_debug_marker(&mut self) -> Option<()> {
        self.ext_debug_marker =
            crate::extensions::ext_debug_marker::ExtDebugMarkerDeviceCommands::load(&self);
        self.ext_debug_marker.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_ext_full_screen_exclusive(&mut self) -> Option<()> {
        self . ext_full_screen_exclusive = crate :: extensions :: ext_full_screen_exclusive :: ExtFullScreenExclusiveDeviceCommands :: load ( & self ) ;
        self.ext_full_screen_exclusive.as_ref().map(|_| ())
    }
    #[inline]
    #[must_use = "Loading may have not been successful"]
    pub fn load_khr_display_swapchain(&mut self) -> Option<()> {
        self.khr_display_swapchain =
            crate::extensions::khr_display_swapchain::KhrDisplaySwapchainDeviceCommands::load(
                &self,
            );
        self.khr_display_swapchain.as_ref().map(|_| ())
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_MAKE_VERSION.html) · Const Function"]
pub const fn make_version(major: u32, minor: u32, patch: u32) -> u32 {
    (major << 22) | (minor << 12) | patch
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_VERSION_MAJOR.html) · Const Function"]
pub const fn version_major(version: u32) -> u32 {
    version >> 22
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_VERSION_MINOR.html) · Const Function"]
pub const fn version_minor(version: u32) -> u32 {
    (version >> 12) & 0x3ff
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_VERSION_PATCH.html) · Const Function"]
pub const fn version_patch(version: u32) -> u32 {
    version & 0xfff
}
#[doc = r" Provides Vulkan extension items"]
pub mod extensions;
#[doc = r" Provides Vulkan feature items"]
pub mod vk1_0;
#[doc = r" Provides Vulkan feature items"]
pub mod vk1_1;
#[doc = r" Provides Vulkan feature items"]
pub mod vk1_2;
#[doc = "Generated from `Vulkan 1.2.137`"]
pub mod info {}
