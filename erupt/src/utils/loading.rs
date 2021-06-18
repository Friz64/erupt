//! Load functions using [`libloading`](https://crates.io/crates/libloading)
//!
//! Enabled using the `loading` cargo feature
use crate::{CustomEntryLoader, EntryEnabled, LoaderError};
use libloading::Library;
use std::{
    error::Error,
    ffi::{CStr, OsStr},
    fmt::{self, Display},
};

#[cfg(all(unix, not(any(target_os = "macos", target_os = "ios", target_os = "android"))))]
const LIB_PATH: &str = "libvulkan.so.1";

#[cfg(target_os = "android")]
const LIB_PATH: &str = "libvulkan.so";

#[cfg(any(target_os = "macos", target_os = "ios"))]
const LIB_PATH: &str = "libvulkan.dylib";

#[cfg(windows)]
const LIB_PATH: &str = "vulkan-1.dll";

/// The default entry loader library, used in [`EntryLoader`]
pub type DefaultEntryLoaderLibrary = Library;

/// The default [`EntryLoader`], providing `EntryLoader::new`
pub type EntryLoader = CustomEntryLoader<DefaultEntryLoaderLibrary>;

/// An error that can occur while loading a library
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

/// An error that can occur while initializing a [`EntryLoader`]
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

impl EntryLoader {
    /// Load functions using [`libloading`](https://crates.io/crates/libloading)
    ///
    /// Enabled using the `loading` cargo feature
    pub fn new() -> Result<EntryLoader, EntryLoaderError> {
        EntryLoader::with_lib_path(LIB_PATH)
    }

    /// Load functions using [`libloading`](https://crates.io/crates/libloading)
    /// providing a custom library path
    ///
    /// Enabled using the `loading` cargo feature
    pub fn with_lib_path<P: AsRef<OsStr>>(lib_path: P) -> Result<EntryLoader, EntryLoaderError> {
        let mut library = unsafe { Library::new(lib_path).map_err(|err| EntryLoaderError::Library(LibraryError(err)))? };

        let symbol = |library: &mut Library, name| unsafe {
            let cstr = CStr::from_ptr(name);
            let bytes = cstr.to_bytes_with_nul();
            library.get(bytes).ok().map(|symbol| *symbol)
        };

        Ok(unsafe { EntryEnabled::new(&mut library, symbol).and_then(|entry_enabled| EntryLoader::custom(library, symbol, entry_enabled)).map_err(EntryLoaderError::EntryLoad)? })
    }
}
