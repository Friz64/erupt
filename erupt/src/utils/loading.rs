//! Load functions using [`libloading`](https://crates.io/crates/libloading)
//!
//! Enabled using the `loading` cargo feature
use crate::CoreLoader;
use std::{
    error::Error,
    fmt::{self, Display},
};

/// The default `CoreLoader`, providing `CoreLoader::new`
pub type DefaultCoreLoader = CoreLoader<libloading::Library>;

// from ash
#[cfg(all(
    unix,
    not(any(target_os = "macos", target_os = "ios", target_os = "android"))
))]
const LIB_PATH: &str = "libvulkan.so.1";

// from ash
#[cfg(target_os = "android")]
const LIB_PATH: &str = "libvulkan.so";

// from ash
#[cfg(any(target_os = "macos", target_os = "ios"))]
const LIB_PATH: &str = "libvulkan.dylib";

// from ash
#[cfg(windows)]
const LIB_PATH: &str = "vulkan-1.dll";

/// An error that can occur while loading a `Library`
#[derive(Debug)]
pub struct LibraryError(libloading::Error);

impl Display for LibraryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl Error for LibraryError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Error::source(&self.0)
    }
}

impl DefaultCoreLoader {
    /// Load functions using [`libloading`](https://crates.io/crates/libloading)
    ///
    /// Enabled using the `loading` cargo feature
    pub fn new() -> Result<DefaultCoreLoader, LibraryError> {
        Ok(CoreLoader::custom(
            libloading::Library::new(LIB_PATH).map_err(LibraryError)?,
            Box::new(|loader, name| unsafe {
                let cstring = std::ffi::CString::new(name).unwrap();
                loader
                    .get(cstring.as_bytes_with_nul())
                    .ok()
                    .map(|symbol| *symbol)
            }),
        ))
    }
}
