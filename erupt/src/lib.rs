#![doc(html_logo_url = "https://gitlab.com/Friz64/erupt/-/raw/master/logo.png")]
//! Vulkan API bindings
//!
//! ## Features
//! - Full Vulkan API coverage
//! - First-class support for all extensions
//! - High quality auto-generated function wrappers
//! - A [utility module] aiding your use of this crate
//!   - [`VulkanResult`]: Idiomatic wrapper around a Vulkan Result
//!   - [`surface`]: Create a [`SurfaceKHR`] using a [`RawWindowHandle`] (adapted from [`ash-window`])
//!   - [`allocator`]: Provides a basic Vulkan memory allocator, aiming to be *correct*
//! - Generated code distributed into multiple modules
//! - Function loading ([`EntryLoader`], [`InstanceLoader`], [`DeviceLoader`])
//! - Seperate `Flags` and `FlagBits` types
//! - A high level `Builder` for every struct
//! - Type-safe pointer chain support
//! - `Default` and `Debug` implementation for every type
//! - Confirmed support for Linux, Windows, macOS and Android
//! - Complete auto-generation of everything except [`utils`]
//!
//! ## Example: Instance Creation
//! ```rust ignore
//! use erupt::{vk1_0, EntryLoader, InstanceLoader};
//!
//! let entry = EntryLoader::new()?;
//!
//! let app_info = vk1_0::ApplicationInfoBuilder::new().api_version(vk1_0::make_version(1, 0, 0));
//! let instance_info = vk1_0::InstanceCreateInfoBuilder::new().application_info(&app_info);
//!
//! let instance = InstanceLoader::new(&entry, &instance_info, None)?;
//!
//! // ...
//!
//! instance.destroy_instance(None);
//! ```
//!
//! ## Additional examples
//! - [triangle](https://gitlab.com/Friz64/erupt/-/blob/master/erupt_examples/src/bin/triangle.rs)
//! - [pointer-chain](https://gitlab.com/Friz64/erupt/-/blob/master/erupt_examples/src/bin/pointer_chain.rs)
//! - [version](https://gitlab.com/Friz64/erupt/-/blob/master/erupt_examples/src/bin/version.rs)
//! - [compute](https://gitlab.com/Friz64/erupt/-/blob/master/erupt_examples/src/bin/compute.rs)
//!
//! ## Cargo Features
//! - `surface` (enabled by default): Enables the [`surface`] module, adds [`raw-window-handle`] dependency
//! - `loading` (enabled by default): Enables the [`EntryLoader::new`] function, adds [`libloading`] dependency
//!
//! ## FAQ
//! ### Q: What's the difference between this, ash and vulkano?
//! A: Vulkano is special because it provides hand-written Vulkan wrappers, which means that for example
//! it has a special hand-written wrapper around a Vulkan `PhysicalDevice`. On the other hand ash and
//! erupt both provide Vulkan API bindings too, but not exposing such *fancy* wrappers and instead
//! focusing on having good bindings to the *raw* Vulkan API.
//!
//! The big selling points of erupt is that it has better documentation, high level function support for
//! all extensions (which is only really relevant if you use those extensions), being fully generated
//! and some more smaller improvements. On the other hand ash has a bigger existing community.
//!
//! ### Q: What does the number at the end of the version mean?
//! A: It represents the Vulkan Header version this version of erupt was generated against and is purely
//! informational.
//!
//! ## Thank you
//! - [`ash`](https://crates.io/crates/ash) for helping inspiring and making this crate
//! - [`libloading`](https://crates.io/crates/libloading) for providing symbol loading
//! - [`ash-window`](https://crates.io/crates/ash-window) for providing a base for the [`surface`] module
//! - [`bitflags`](https://crates.io/crates/bitflags) for providing a perfect bitflag macro
//! - The Vulkan Community ❤️
//! - The Rust Community ❤️
//!
//! ## Licensing
//!
//! The logo is the Volcano Emoji of [Twemoji](https://twemoji.twitter.com/) ([License](https://creativecommons.org/licenses/by/4.0/)). The name "erupt" was added on top of it.
//!
//! This project is licensed under the [zlib License](https://gitlab.com/Friz64/erupt/-/blob/master/LICENSE).
//!
//! [utility module]: https://docs.rs/erupt/*/erupt/utils/index.html
//! [`VulkanResult`]: https://docs.rs/erupt/*/erupt/utils/struct.VulkanResult.html
//! [`surface`]: https://docs.rs/erupt/*/erupt/utils/surface/index.html
//! [`SurfaceKHR`]: https://docs.rs/erupt/*/erupt/extensions/khr_surface/struct.SurfaceKHR.html
//! [`allocator`]: https://docs.rs/erupt/*/erupt/utils/allocator/index.html
//! [`RawWindowHandle`]: https://docs.rs/raw-window-handle/*/raw_window_handle/enum.RawWindowHandle.html
//! [`libloading`]: https://crates.io/crates/libloading
//! [`raw-window-handle`]: https://crates.io/crates/raw-window-handle
//! [`ash-window`]: https://crates.io/crates/ash-window
//! [`EntryLoader`]: https://docs.rs/erupt/*/erupt/struct.EntryLoader.html
//! [`EntryLoader::new`]: https://docs.rs/erupt/*/erupt/struct.EntryLoader.html#method.new
//! [`InstanceLoader`]: https://docs.rs/erupt/*/erupt/struct.InstanceLoader.html
//! [`DeviceLoader`]: https://docs.rs/erupt/*/erupt/struct.DeviceLoader.html
//! [`utils`]: https://docs.rs/erupt/*/erupt/utils/index.html

mod generated;
pub mod utils;

pub use generated::*;
use std::{
    error::Error,
    ffi::CStr,
    fmt::{self, Display},
};
pub use utils::loading::DefaultEntryLoader;

/// Construct a `*const std::os::raw::c_char` from a string
///
/// # Example
/// ```ignore
/// const LAYER_KHRONOS_VALIDATION: *const c_char = cstr!("VK_LAYER_KHRONOS_validation");
/// ```
#[macro_export]
macro_rules! cstr {
    ($s:expr) => {
        concat!($s, "\0") as *const str as *const ::std::os::raw::c_char
    };
}

/// Like `try!`, but for [`utils::VulkanResult`](utils/struct.VulkanResult.html)
///
/// ```ignore
/// unsafe fn example(device: &DeviceLoader) -> VulkanResult<(Semaphore, Semaphore)> {
///     let create_info = SemaphoreCreateInfoBuilder::new();
///
///     let semaphore1 = try_vk!(device.create_semaphore(&create_info, None, None));
///     let semaphore2 = try_vk!(device.create_semaphore(&create_info, None, None));
///     VulkanResult::new_ok((semaphore1, semaphore2))
/// }
/// ```
#[macro_export]
macro_rules! try_vk {
    ($expr:expr) => {
        match $crate::utils::VulkanResult::result($expr) {
            Ok(value) => value,
            Err(raw) => return $crate::utils::VulkanResult::new_err(raw),
        }
    };
}

// adapted from ash
#[doc(hidden)]
#[macro_export]
macro_rules! non_dispatchable_handle {
    ($name:ident, $ty:ident, $doc:meta) => {
        #[$doc]
        #[repr(transparent)]
        #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Default)]
        pub struct $name(pub u64);

        impl $name {
            pub const TYPE: crate::vk1_0::ObjectType = crate::vk1_0::ObjectType::$ty;

            pub const fn null() -> $name {
                $name(0)
            }

            pub const fn is_null(&self) -> bool {
                self.0 == 0
            }
        }

        impl std::fmt::Pointer for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "0x{:x}", self.0)
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "0x{:x}", self.0)
            }
        }
    };
}

// adapted from ash
#[doc(hidden)]
#[macro_export]
macro_rules! dispatchable_handle {
    ($name:ident, $ty:ident, $doc:meta) => {
        #[$doc]
        #[repr(transparent)]
        #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash)]
        pub struct $name(pub *mut ());

        impl $name {
            pub const TYPE: crate::vk1_0::ObjectType = crate::vk1_0::ObjectType::$ty;

            pub const fn null() -> Self {
                $name(std::ptr::null_mut())
            }

            pub fn is_null(&self) -> bool {
                self.0.is_null()
            }
        }

        unsafe impl Send for $name {}
        unsafe impl Sync for $name {}

        impl Default for $name {
            fn default() -> $name {
                $name::null()
            }
        }

        impl std::fmt::Pointer for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                std::fmt::Pointer::fmt(&self.0, f)
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                std::fmt::Debug::fmt(&self.0, f)
            }
        }
    };
}

/// An error which can occur while initializing a loader
#[derive(Debug)]
pub enum LoaderError {
    /// A Vulkan function returned a negative `Result` value
    VulkanError(vk1_0::Result),
    /// A symbol was not available while it should have been
    SymbolNotAvailable,
}

impl Display for LoaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LoaderError::VulkanError(err) => write!(
                f,
                "A Vulkan function returned a negative `Result` value: {}",
                err
            ),
            LoaderError::SymbolNotAvailable => {
                write!(f, "A symbol was not available while it should have been")
            }
        }
    }
}

impl Error for LoaderError {}

impl InstanceLoader {
    #[inline]
    pub fn new<T>(
        entry_loader: &EntryLoader<T>,
        create_info: &crate::vk1_0::InstanceCreateInfo,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
    ) -> Result<InstanceLoader, LoaderError> {
        let instance = unsafe { entry_loader.create_instance(create_info, allocator, None) }
            .result()
            .map_err(LoaderError::VulkanError)?;

        let mut version = crate::vk1_0::make_version(1, 0, 0);
        if !create_info.p_application_info.is_null() {
            let user_version = unsafe { *create_info.p_application_info }.api_version;
            if user_version != 0 {
                version = user_version;
            }
        }

        InstanceLoader::custom(
            entry_loader,
            instance,
            version,
            create_info.enabled_extension_count as usize,
            create_info.pp_enabled_extension_names,
            |name| unsafe { (entry_loader.get_instance_proc_addr)(instance, name) },
        )
    }
}

impl DeviceLoader {
    #[inline]
    pub fn new(
        instance_loader: &InstanceLoader,
        physical_device: crate::vk1_0::PhysicalDevice,
        create_info: &crate::vk1_0::DeviceCreateInfo,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
    ) -> Result<DeviceLoader, LoaderError> {
        let device =
            unsafe { instance_loader.create_device(physical_device, create_info, allocator, None) }
                .result()
                .map_err(LoaderError::VulkanError)?;

        DeviceLoader::custom(
            instance_loader,
            device,
            create_info.enabled_extension_count as usize,
            create_info.pp_enabled_extension_names,
            |name| unsafe { (instance_loader.get_device_proc_addr)(device, name) },
        )
    }
}

// Used by loaders to check for extensions
#[inline]
unsafe fn c_str_array_contains(
    array: *const *const std::os::raw::c_char,
    array_length: usize,
    contains: *const std::os::raw::c_char,
) -> bool {
    let contains = CStr::from_ptr(contains);
    for i in 0..array_length {
        if CStr::from_ptr(*array.add(i)) == contains {
            return true;
        }
    }

    false
}

/// Provides type-safe pointer chain support
pub trait ExtendableFrom<'a, T> {
    /// Appends `other`'s pointer chain to the end of this pointer chain
    fn extend_from(mut self, other: &'a mut T) -> Self
    where
        Self: Sized,
    {
        unsafe {
            crate::append_ptr_chain(&mut self as *mut Self as _, other as *mut T as _);
        }

        self
    }
}

#[inline]
unsafe fn append_ptr_chain(
    mut host: *mut vk1_0::BaseOutStructure,
    tail: *mut vk1_0::BaseOutStructure,
) {
    loop {
        let p_next = &mut (*host).p_next;

        if p_next.is_null() {
            *p_next = tail;
            break;
        } else {
            host = *p_next;
        }
    }
}
