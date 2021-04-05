//! Create a [`SurfaceKHR`] using a [`RawWindowHandle`] (adapted from [`ash-window`])
//!
//! Enabled using the `loading` cargo feature
//!
//! [`SurfaceKHR`]: ../extensions/khr_surface/struct.SurfaceKHR.html
//! [`RawWindowHandle`]: https://docs.rs/raw-window-handle/*/raw_window_handle/enum.RawWindowHandle.html
//! [`ash-window`]: https://crates.io/crates/ash-window

use crate::{extensions::khr_surface, utils::VulkanResult, vk1_0, InstanceLoader};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use std::os::raw::c_char;

/// Create a surface from a raw surface handle.
///
/// `instance` must have created with platform specific surface extensions enabled.
pub unsafe fn create_surface(instance: &InstanceLoader, window_handle: &impl HasRawWindowHandle, allocation_callbacks: Option<&vk1_0::AllocationCallbacks>) -> VulkanResult<khr_surface::SurfaceKHR> {
    match window_handle.raw_window_handle() {
        #[cfg(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd"))]
        RawWindowHandle::Wayland(handle) => {
            use crate::extensions::khr_wayland_surface;

            let create_info = khr_wayland_surface::WaylandSurfaceCreateInfoKHR {
                display: handle.display,
                surface: handle.surface,
                ..Default::default()
            };

            instance.create_wayland_surface_khr(&create_info, allocation_callbacks)
        }

        #[cfg(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd"))]
        RawWindowHandle::Xlib(handle) => {
            use crate::extensions::khr_xlib_surface;

            let create_info = khr_xlib_surface::XlibSurfaceCreateInfoKHR {
                dpy: handle.display as *mut _,
                window: handle.window,
                ..Default::default()
            };

            instance.create_xlib_surface_khr(&create_info, allocation_callbacks)
        }

        #[cfg(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd"))]
        RawWindowHandle::Xcb(handle) => {
            use crate::extensions::khr_xcb_surface;

            let create_info = khr_xcb_surface::XcbSurfaceCreateInfoKHR {
                connection: handle.connection as *mut _,
                window: handle.window,
                ..Default::default()
            };

            instance.create_xcb_surface_khr(&create_info, allocation_callbacks)
        }

        #[cfg(any(target_os = "android"))]
        RawWindowHandle::Android(handle) => {
            use crate::extensions::khr_android_surface;

            let create_info = khr_android_surface::AndroidSurfaceCreateInfoKHR {
                window: handle.a_native_window as _,
                ..Default::default()
            };

            instance.create_android_surface_khr(&create_info, allocation_callbacks)
        }

        #[cfg(any(target_os = "macos"))]
        RawWindowHandle::MacOS(handle) => {
            use crate::{extensions::ext_metal_surface, vk1_0};
            use raw_window_metal::{macos, Layer};

            let layer = match macos::metal_layer_from_handle(handle) {
                Layer::Existing(layer) | Layer::Allocated(layer) => layer as *mut _,
                Layer::None => return VulkanResult::new_err(vk1_0::Result::ERROR_INITIALIZATION_FAILED),
            };

            let create_info = ext_metal_surface::MetalSurfaceCreateInfoEXT { p_layer: layer, ..Default::default() };

            instance.create_metal_surface_ext(&create_info, allocation_callbacks)
        }

        #[cfg(any(target_os = "ios"))]
        RawWindowHandle::IOS(handle) => {
            use crate::{extensions::ext_metal_surface, vk1_0};
            use raw_window_metal::{ios, Layer};

            let layer = match ios::metal_layer_from_handle(handle) {
                Layer::Existing(layer) | Layer::Allocated(layer) => layer as *mut _,
                Layer::None => return VulkanResult::new_err(vk1_0::Result::ERROR_INITIALIZATION_FAILED),
            };

            let create_info = ext_metal_surface::MetalSurfaceCreateInfoEXT { p_layer: layer, ..Default::default() };

            instance.create_metal_surface_ext(&create_info, allocation_callbacks)
        }

        #[cfg(target_os = "windows")]
        RawWindowHandle::Windows(handle) => {
            use crate::extensions::khr_win32_surface;

            let create_info = khr_win32_surface::Win32SurfaceCreateInfoKHR {
                hinstance: handle.hinstance,
                hwnd: handle.hwnd,
                ..Default::default()
            };

            instance.create_win32_surface_khr(&create_info, allocation_callbacks)
        }

        _ => VulkanResult::new_err(vk1_0::Result::ERROR_EXTENSION_NOT_PRESENT), // not supported
    }
}

/// Query the required instance extensions for creating a surface from a window handle.
///
/// The returned extensions will include all extension dependencies.
pub fn enumerate_required_extensions(window_handle: &impl HasRawWindowHandle) -> VulkanResult<Vec<*const c_char>> {
    let extensions = match window_handle.raw_window_handle() {
        #[cfg(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd"))]
        RawWindowHandle::Wayland(_) => vec![khr_surface::KHR_SURFACE_EXTENSION_NAME, crate::extensions::khr_wayland_surface::KHR_WAYLAND_SURFACE_EXTENSION_NAME],

        #[cfg(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd"))]
        RawWindowHandle::Xlib(_) => vec![khr_surface::KHR_SURFACE_EXTENSION_NAME, crate::extensions::khr_xlib_surface::KHR_XLIB_SURFACE_EXTENSION_NAME],

        #[cfg(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd"))]
        RawWindowHandle::Xcb(_) => vec![khr_surface::KHR_SURFACE_EXTENSION_NAME, crate::extensions::khr_xcb_surface::KHR_XCB_SURFACE_EXTENSION_NAME],

        #[cfg(any(target_os = "android"))]
        RawWindowHandle::Android(_) => vec![khr_surface::KHR_SURFACE_EXTENSION_NAME, crate::extensions::khr_android_surface::KHR_ANDROID_SURFACE_EXTENSION_NAME],

        #[cfg(any(target_os = "macos"))]
        RawWindowHandle::MacOS(_) => vec![khr_surface::KHR_SURFACE_EXTENSION_NAME, crate::extensions::ext_metal_surface::EXT_METAL_SURFACE_EXTENSION_NAME],

        #[cfg(any(target_os = "ios"))]
        RawWindowHandle::IOS(_) => vec![khr_surface::KHR_SURFACE_EXTENSION_NAME, crate::extensions::ext_metal_surface::EXT_METAL_SURFACE_EXTENSION_NAME],

        #[cfg(target_os = "windows")]
        RawWindowHandle::Windows(_) => vec![khr_surface::KHR_SURFACE_EXTENSION_NAME, crate::extensions::khr_win32_surface::KHR_WIN32_SURFACE_EXTENSION_NAME],

        _ => return VulkanResult::new_err(vk1_0::Result::ERROR_EXTENSION_NOT_PRESENT), // not supported
    };

    VulkanResult::new_ok(extensions)
}
