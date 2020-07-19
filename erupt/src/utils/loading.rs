//! Load functions using [`libloading`](https://crates.io/crates/libloading)
//!
//! Enabled using the `loading` cargo feature
use crate::{EntryLoader, LoaderError};
use libloading::Library;
use std::{
    error::Error,
    ffi::CStr,
    fmt::{self, Display},
};

/// The default `EntryLoader`, providing `EntryLoader::new`
pub type DefaultEntryLoader = EntryLoader<Library>;

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

/// An error that can occur while initializing a `EntryLoader`
#[derive(Debug)]
pub enum EntryLoaderError {
    /// The library failed to load
    Library(LibraryError),
    /// The entry loader failed to initialize
    EntryLoad(LoaderError),
}

impl Display for EntryLoaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EntryLoaderError::Library(err) => write!(f, "The library failed to load: {}", err),
            EntryLoaderError::EntryLoad(err) => {
                write!(f, "The entry loader failed to initialize: {}", err)
            }
        }
    }
}

impl Error for EntryLoaderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            EntryLoaderError::Library(err) => Error::source(err),
            EntryLoaderError::EntryLoad(err) => Error::source(err),
        }
    }
}

impl DefaultEntryLoader {
    /// Load functions using [`libloading`](https://crates.io/crates/libloading)
    ///
    /// Enabled using the `loading` cargo feature
    pub fn new() -> Result<DefaultEntryLoader, EntryLoaderError> {
        let library =
            Library::new(LIB_PATH).map_err(|err| EntryLoaderError::Library(LibraryError(err)))?;

        EntryLoader::custom(library, |library, name| unsafe {
            let cstr = CStr::from_ptr(name);
            let bytes = cstr.to_bytes_with_nul();
            library.get(bytes).ok().map(|symbol| *symbol)
        })
        .map_err(EntryLoaderError::EntryLoad)
    }
}
