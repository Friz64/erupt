//! Create a [`SurfaceKHR`] using a [`RawWindowHandle`] (adapted from [`ash-window`])
//!
//! Enabled using the `loading` cargo feature
//!
//! [`SurfaceKHR`]: ../extensions/khr_surface/struct.SurfaceKHR.html
//! [`RawWindowHandle`]: https://docs.rs/raw-window-handle/*/raw_window_handle/enum.RawWindowHandle.html
//! [`ash-window`]: https://crates.io/crates/ash-window

use crate::{
    extensions::khr_surface::*,
    utils::VulkanResult,
    vk1_0::{Result as RawResult, *},
    InstanceLoader,
};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use std::os::raw::c_char;

/// Create a surface from a raw surface handle.
///
/// `instance` must have created with platform specific surface extensions enabled.
pub unsafe fn create_surface(
    instance: &mut InstanceLoader,
    window_handle: &impl HasRawWindowHandle,
    allocation_callbacks: Option<&AllocationCallbacks>,
) -> VulkanResult<SurfaceKHR> {
    if instance.load_khr_surface().is_none() {
        return VulkanResult::new_err(RawResult::ERROR_EXTENSION_NOT_PRESENT);
    }

    match window_handle.raw_window_handle() {
        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Wayland(handle) => {
            use crate::extensions::khr_wayland_surface::*;
            if instance.load_khr_wayland_surface().is_none() {
                return VulkanResult::new_err(RawResult::ERROR_EXTENSION_NOT_PRESENT);
            }

            let create_info = WaylandSurfaceCreateInfoKHR {
                display: handle.display,
                surface: handle.surface,
                ..Default::default()
            };

            instance.create_wayland_surface_khr(&create_info, allocation_callbacks, None)
        }

        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Xlib(handle) => {
            use crate::extensions::khr_xlib_surface::*;
            if instance.load_khr_xlib_surface().is_none() {
                return VulkanResult::new_err(RawResult::ERROR_EXTENSION_NOT_PRESENT);
            }

            let create_info = XlibSurfaceCreateInfoKHR {
                dpy: handle.display as *mut _,
                window: handle.window,
                ..Default::default()
            };

            instance.create_xlib_surface_khr(&create_info, allocation_callbacks, None)
        }

        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Xcb(handle) => {
            use crate::extensions::khr_xcb_surface::*;
            if instance.load_khr_xcb_surface().is_none() {
                return VulkanResult::new_err(RawResult::ERROR_EXTENSION_NOT_PRESENT);
            }

            let create_info = XcbSurfaceCreateInfoKHR {
                connection: handle.connection as *mut _,
                window: handle.window,
                ..Default::default()
            };

            instance.create_xcb_surface_khr(&create_info, allocation_callbacks, None)
        }

        #[cfg(any(target_os = "android"))]
        RawWindowHandle::Android(handle) => {
            use crate::extensions::khr_android_surface::*;
            if instance.load_khr_android_surface().is_none() {
                return VulkanResult::new_err(RawResult::ERROR_EXTENSION_NOT_PRESENT);
            }

            let create_info = AndroidSurfaceCreateInfoKHR {
                window: handle.a_native_window as _,
                ..Default::default()
            };

            instance.create_android_surface_khr(&create_info, allocation_callbacks, None)
        }

        #[cfg(any(target_os = "macos"))]
        RawWindowHandle::MacOS(handle) => {
            use crate::extensions::mvk_macos_surface::*;
            if instance.load_mvk_macos_surface().is_none() {
                return VulkanResult::new_err(RawResult::ERROR_EXTENSION_NOT_PRESENT);
            }

            let create_info = MacOSSurfaceCreateInfoMVK {
                p_view: &*handle.ns_view,
                ..Default::default()
            };

            instance.create_mac_os_surface_mvk(&create_info, allocation_callbacks, None)
        }

        #[cfg(any(target_os = "ios"))]
        RawWindowHandle::IOS(handle) => {
            use crate::extensions::mvk_ios_surface::*;
            if instance.load_mvk_ios_surface().is_none() {
                return VulkanResult::new_err(RawResult::ERROR_EXTENSION_NOT_PRESENT);
            }

            let create_info = IOSSurfaceCreateInfoMVK {
                p_view: &*handle.ui_view,
                ..Default::default()
            };

            instance.create_ios_surface_mvk(&create_info, allocation_callbacks, None)
        }

        #[cfg(target_os = "windows")]
        RawWindowHandle::Windows(handle) => {
            use crate::extensions::khr_win32_surface::*;
            if instance.load_khr_win32_surface().is_none() {
                return VulkanResult::new_err(RawResult::ERROR_EXTENSION_NOT_PRESENT);
            }

            let create_info = Win32SurfaceCreateInfoKHR {
                hinstance: handle.hinstance,
                hwnd: handle.hwnd,
                ..Default::default()
            };

            instance.create_win32_surface_khr(&create_info, allocation_callbacks, None)
        }

        _ => VulkanResult::new_err(RawResult::ERROR_EXTENSION_NOT_PRESENT), // not supported
    }
}

/// Query the required instance extensions for creating a surface from a window handle.
///
/// The returned extensions will include all extension dependencies.
pub fn enumerate_required_extensions(
    window_handle: &impl HasRawWindowHandle,
) -> VulkanResult<Vec<*const c_char>> {
    let extensions = match window_handle.raw_window_handle() {
        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Wayland(_) => vec![
            KHR_SURFACE_EXTENSION_NAME,
            crate::extensions::khr_wayland_surface::KHR_WAYLAND_SURFACE_EXTENSION_NAME,
        ],

        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Xlib(_) => vec![
            KHR_SURFACE_EXTENSION_NAME,
            crate::extensions::khr_xlib_surface::KHR_XLIB_SURFACE_EXTENSION_NAME,
        ],

        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Xcb(_) => vec![
            KHR_SURFACE_EXTENSION_NAME,
            crate::extensions::khr_xcb_surface::KHR_XCB_SURFACE_EXTENSION_NAME,
        ],

        #[cfg(any(target_os = "android"))]
        RawWindowHandle::Android(_) => vec![
            KHR_SURFACE_EXTENSION_NAME,
            crate::extensions::khr_android_surface::KHR_ANDROID_SURFACE_EXTENSION_NAME,
        ],

        #[cfg(any(target_os = "macos"))]
        RawWindowHandle::MacOS(_) => vec![
            KHR_SURFACE_EXTENSION_NAME,
            crate::extensions::mvk_macos_surface::MVK_MACOS_SURFACE_EXTENSION_NAME,
        ],

        #[cfg(any(target_os = "ios"))]
        RawWindowHandle::IOS(_) => vec![
            KHR_SURFACE_EXTENSION_NAME,
            crate::extensions::mvk_ios_surface::MVK_IOS_SURFACE_EXTENSION_NAME,
        ],

        #[cfg(target_os = "windows")]
        RawWindowHandle::Windows(_) => vec![
            KHR_SURFACE_EXTENSION_NAME,
            crate::extensions::khr_win32_surface::KHR_WIN32_SURFACE_EXTENSION_NAME,
        ],

        _ => return VulkanResult::new_err(RawResult::ERROR_EXTENSION_NOT_PRESENT), // not supported
    };

    VulkanResult::new_ok(extensions)
}
