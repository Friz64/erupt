//! Load functions using [`libloading`](https://crates.io/crates/libloading)
//!
//! Enabled using the `loading` cargo feature
use crate::EntryLoader;
use libloading::Library;
use std::{
    error::Error,
    ffi::CStr,
    fmt::{self, Display},
};

/// The default `EntryLoader`, providing `EntryLoader::new`
pub type DefaultCoreLoader = EntryLoader<Library>;

#[cfg(all(
    unix,
    not(any(target_os = "macos", target_os = "ios", target_os = "android"))
))]
const LIB_PATH: &str = "libvulkan.so.1";

#[cfg(target_os = "android")]
const LIB_PATH: &str = "libvulkan.so";

#[cfg(any(target_os = "macos", target_os = "ios"))]
const LIB_PATH: &str = "libvulkan.dylib";

#[cfg(windows)]
const LIB_PATH: &str = "vulkan-1.dll";

#[derive(Debug)]
enum LibraryErrorInner {
    LibLoading(libloading::Error),
    EntryLoader,
}

impl Display for LibraryErrorInner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LibraryErrorInner::LibLoading(err) => Display::fmt(err, f),
            LibraryErrorInner::EntryLoader => write!(f, "Entry failed to load"),
        }
    }
}

impl Error for LibraryErrorInner {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            LibraryErrorInner::LibLoading(err) => Error::source(err),
            LibraryErrorInner::EntryLoader => None,
        }
    }
}

/// An error that can occur while loading a `Library`
#[derive(Debug)]
pub struct LibraryError(LibraryErrorInner);

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
        let library = Library::new(LIB_PATH)
            .map_err(|err| LibraryError(LibraryErrorInner::LibLoading(err)))?;

        EntryLoader::custom(library, |library, name| unsafe {
            let cstr = CStr::from_ptr(name);
            let bytes = cstr.to_bytes_with_nul();
            library.get(bytes).ok().map(|symbol| *symbol)
        })
        .ok_or(LibraryError(LibraryErrorInner::EntryLoader))
    }
}
