#![allow(clippy::all, unreachable_patterns)]
#![doc(html_logo_url = "https://gitlab.com/Friz64/erupt/-/raw/master/logo.png")]
/*!
Vulkan API bindings

## Features
- Full Vulkan API coverage
- First-class support for all extensions
- High quality auto-generated function wrappers
- A [utility module] aiding your use of this crate
  - [`VulkanResult`]: Idiomatic wrapper around a Vulkan Result
  - [`surface`]: Create a [`SurfaceKHR`] using a [`RawWindowHandle`] (adapted from [`ash-window`])
- Generated code distributed into multiple modules
- Function loading ([`EntryLoader`], [`InstanceLoader`], [`DeviceLoader`])
- Seperate `Flags` and `FlagBits` types
- A high level `Builder` for every struct
- Type-safe pointer chain support
- `Default` and `Debug` implementation for every type
- Confirmed support for Linux, Windows, macOS and Android
- Complete auto-generation of everything except [`utils`]

## Example: Instance Creation
```rust ignore
use erupt::{vk, EntryLoader, InstanceLoader};

let entry = EntryLoader::new()?;

let app_info = vk::ApplicationInfoBuilder::new().api_version(vk::make_version(1, 0, 0));
let instance_info = vk::InstanceCreateInfoBuilder::new().application_info(&app_info);

let instance = InstanceLoader::new(&entry, &instance_info, None)?;

// ...

instance.destroy_instance(None);
```

## Additional examples
- [triangle](https://gitlab.com/Friz64/erupt/-/blob/master/erupt_examples/src/bin/triangle.rs)
- [pointer-chain](https://gitlab.com/Friz64/erupt/-/blob/master/erupt_examples/src/bin/pointer_chain.rs)
- [version](https://gitlab.com/Friz64/erupt/-/blob/master/erupt_examples/src/bin/version.rs)

## Cargo Features
- `surface` (enabled by default): Enables the [`surface`] module, adds [`raw-window-handle`] dependency
- `loading` (enabled by default): Enables the [`EntryLoader::new`] function, adds [`libloading`] dependency
- `bytemuck`: Implements [`Pod`] for some hand-picked structs (`*IndirectCommand`, etc.), adds [`bytemuck`] dependency

## FAQ
### Q: What's the difference between this, ash and vulkano?
A: Vulkano is special because it provides hand-written Vulkan wrappers, which means that for example
it has a special hand-written wrapper around a Vulkan `PhysicalDevice`. On the other hand ash and
erupt both provide Vulkan API bindings too, but not exposing such *fancy* wrappers and instead
focusing on having good bindings to the *raw* Vulkan API.

The big selling points of erupt is that it has better documentation, high level function support for
all extensions (which is only really relevant if you use those extensions), being fully generated
and some more smaller improvements. On the other hand ash has a bigger existing community.

### Q: What does the number at the end of the version mean?
A: It represents the Vulkan Header version this version of erupt was generated against and is purely
informational.

### Q: I need to easily allocate memory, what should i use?
A: Take a look at [`gpu-alloc`](https://github.com/zakarumych/gpu-alloc)
or [`vk-alloc`](https://github.com/hasenbanck/vk-alloc).

## Minimum Supported Rust Version (MSRV)
Rust 1.48 or higher.

## Thank you
- [`ash`](https://crates.io/crates/ash) for helping inspiring and making this crate
- [`libloading`](https://crates.io/crates/libloading) for providing symbol loading
- [`ash-window`](https://crates.io/crates/ash-window) for providing a base for the [`surface`] module
- [`bitflags`](https://crates.io/crates/bitflags) for providing a perfect bitflag macro
- The Vulkan Community ❤️
- The Rust Community ❤️

## Licensing

The logo is the Volcano Emoji of [Twemoji](https://twemoji.twitter.com/) ([License](https://creativecommons.org/licenses/by/4.0/)). The name "erupt" was added on top of it.

This project is licensed under the [zlib License](https://gitlab.com/Friz64/erupt/-/blob/master/LICENSE).

[utility module]: https://docs.rs/erupt/%2A/erupt/utils/index.html
[`VulkanResult`]: https://docs.rs/erupt/%2A/erupt/utils/struct.VulkanResult.html
[`surface`]: https://docs.rs/erupt/%2A/erupt/utils/surface/index.html
[`SurfaceKHR`]: https://docs.rs/erupt/%2A/erupt/extensions/khr_surface/struct.SurfaceKHR.html
[`allocator`]: https://docs.rs/erupt/%2A/erupt/utils/allocator/index.html
[`RawWindowHandle`]: https://docs.rs/raw-window-handle/%2A/raw_window_handle/enum.RawWindowHandle.html
[`libloading`]: https://crates.io/crates/libloading
[`raw-window-handle`]: https://crates.io/crates/raw-window-handle
[`ash-window`]: https://crates.io/crates/ash-window
[`EntryLoader`]: https://docs.rs/erupt/%2A/erupt/struct.EntryLoader.html
[`EntryLoader::new`]: https://docs.rs/erupt/%2A/erupt/struct.EntryLoader.html#method.new
[`Pod`]: https://docs.rs/bytemuck/%2A/bytemuck/trait.Pod.html
[`bytemuck`]: https://crates.io/crates/bytemuck
[`InstanceLoader`]: https://docs.rs/erupt/%2A/erupt/struct.InstanceLoader.html
[`DeviceLoader`]: https://docs.rs/erupt/%2A/erupt/struct.DeviceLoader.html
[`utils`]: https://docs.rs/erupt/%2A/erupt/utils/index.html
*/

mod generated;
pub mod utils;

use fmt::Debug;
pub use generated::*;
use std::{
    error::Error,
    ffi::CStr,
    fmt::{self, Display},
    mem, ptr,
};
#[cfg(feature = "loading")]
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
    ($name:ident, $ty:ident, $doc:literal, $doc_alias:literal) => {
        #[doc = $doc]
        #[doc(alias = $doc_alias)]
        #[repr(transparent)]
        #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Default)]
        pub struct $name(pub u64);

        impl $name {
            /// The [`vk::ObjectType`](`crate::vk1_0::ObjectType`) of this handle.
            pub const TYPE: $crate::vk1_0::ObjectType = $crate::vk1_0::ObjectType::$ty;

            /// Returns a null handle.
            pub const fn null() -> $name {
                $name(0)
            }

            /// Returns `true` if this handle is null.
            pub const fn is_null(&self) -> bool {
                self.0 == 0
            }

            /// Returns the raw handle value.
            ///
            /// This may for example be useful [here](`crate::extensions::ext_debug_utils::DebugUtilsObjectNameInfoEXTBuilder::object_handle`).
            #[inline]
            pub fn object_handle(&self) -> u64 {
                self.0
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
    ($name:ident, $ty:ident, $doc:literal, $doc_alias:literal) => {
        #[doc = $doc]
        #[doc(alias = $doc_alias)]
        #[repr(transparent)]
        #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash)]
        pub struct $name(pub *mut ());

        impl $name {
            /// The [`vk::ObjectType`](`crate::vk1_0::ObjectType`) of this handle.
            pub const TYPE: $crate::vk1_0::ObjectType = $crate::vk1_0::ObjectType::$ty;

            /// Returns a null handle.
            pub const fn null() -> Self {
                $name(std::ptr::null_mut())
            }

            /// Returns `true` if this handle is null.
            pub fn is_null(&self) -> bool {
                self.0.is_null()
            }

            /// Returns the raw handle value.
            ///
            /// This may for example be useful [here](`crate::extensions::ext_debug_utils::DebugUtilsObjectNameInfoEXTBuilder::object_handle`).
            #[inline]
            pub fn object_handle(&self) -> u64 {
                self.0 as u64
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

// used in builders for bitwidth fields
#[doc(hidden)]
#[macro_export]
macro_rules! bits_copy {
    ($dst:expr, $src:expr, $start:expr, $end:expr) => {{
        let mut dst = $dst;
        let src = $src;
        let start: usize = $start;
        let end: usize = $end;

        // bits in the range are 1 (end inclusive)
        let mask = !0 << start & !(!0 << end);

        // clear bits
        dst &= !mask;
        // set bits from src
        dst |= (src << start) & mask;

        dst
    }};
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
            LoaderError::VulkanError(err) => write!(f, "A Vulkan function returned a negative `Result` value: {}", err),
            LoaderError::SymbolNotAvailable => {
                write!(f, "A symbol was not available while it should have been")
            }
        }
    }
}

impl Error for LoaderError {}

impl<T> Debug for EntryLoader<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Entry")
    }
}

impl InstanceLoader {
    #[inline]
    pub fn new<T>(entry_loader: &EntryLoader<T>, create_info: &crate::vk1_0::InstanceCreateInfo, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> Result<InstanceLoader, LoaderError> {
        let instance = unsafe { entry_loader.create_instance(create_info, allocator, None) }.result().map_err(LoaderError::VulkanError)?;

        let mut version = crate::vk1_0::make_version(1, 0, 0);
        if !create_info.p_application_info.is_null() {
            let user_version = unsafe { *create_info.p_application_info }.api_version;
            if user_version != 0 {
                version = user_version;
            }
        }

        unsafe {
            let enabled_extensions = std::slice::from_raw_parts(create_info.pp_enabled_extension_names, create_info.enabled_extension_count as _);

            let enabled_extensions: Vec<_> = enabled_extensions.iter().map(|&ptr| CStr::from_ptr(ptr)).collect();

            let symbol = |name| (entry_loader.get_instance_proc_addr)(instance, name);

            let enumerate_physical_devices: vk1_0::PFN_vkEnumeratePhysicalDevices = mem::transmute(symbol(crate::vk1_0::FN_ENUMERATE_PHYSICAL_DEVICES).ok_or(crate::LoaderError::SymbolNotAvailable)?);

            let enumerate_device_extension_properties: vk1_0::PFN_vkEnumerateDeviceExtensionProperties =
                mem::transmute(symbol(crate::vk1_0::FN_ENUMERATE_DEVICE_EXTENSION_PROPERTIES).ok_or(crate::LoaderError::SymbolNotAvailable)?);

            let mut physical_device_count = 0;
            let result = enumerate_physical_devices(instance, &mut physical_device_count, ptr::null_mut());

            if result.0 < 0 {
                return Err(LoaderError::VulkanError(result));
            }

            let mut physical_devices = vec![Default::default(); physical_device_count as usize];
            let result = enumerate_physical_devices(instance, &mut physical_device_count, physical_devices.as_mut_ptr());

            if result.0 < 0 {
                return Err(LoaderError::VulkanError(result));
            }

            let mut all_device_extension_properties = Vec::new();
            for physical_device in physical_devices {
                let mut property_count = 0;
                let result = enumerate_device_extension_properties(physical_device, ptr::null(), &mut property_count, ptr::null_mut());

                if result.0 < 0 {
                    return Err(LoaderError::VulkanError(result));
                }

                let mut properties = vec![Default::default(); property_count as usize];
                let result = enumerate_device_extension_properties(physical_device, ptr::null(), &mut property_count, properties.as_mut_ptr());

                if result.0 < 0 {
                    return Err(LoaderError::VulkanError(result));
                }

                all_device_extension_properties.extend(properties.into_iter());
            }

            let available_device_extensions: Vec<_> = all_device_extension_properties.iter().map(|properties| CStr::from_ptr(properties.extension_name.as_ptr())).collect();

            let instance_enabled = InstanceEnabled::new(version, &enabled_extensions, &available_device_extensions)?;

            InstanceLoader::custom(entry_loader, instance, instance_enabled, symbol)
        }
    }
}

impl Debug for InstanceLoader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(&self.handle, f)
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
        let device = unsafe { instance_loader.create_device(physical_device, create_info, allocator, None) }
            .result()
            .map_err(LoaderError::VulkanError)?;

        let device_enabled = unsafe {
            let enabled_extensions = std::slice::from_raw_parts(create_info.pp_enabled_extension_names, create_info.enabled_extension_count as _);

            let enabled_extensions: Vec<_> = enabled_extensions.iter().map(|&ptr| CStr::from_ptr(ptr)).collect();

            DeviceEnabled::new(&enabled_extensions)
        };

        unsafe { DeviceLoader::custom(instance_loader, device, device_enabled, |name| (instance_loader.get_device_proc_addr)(device, name)) }
    }
}

impl Debug for DeviceLoader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(&self.handle, f)
    }
}

/// Any type implementing this trait may be safely transmuted/pointer-casted into `T`.
///
/// This is used to allow either builder or non-builder variants of Vulkan structs
/// to be passed into APIs.
///
/// Please note that is *strongly* encouraged to pass the builder variants
/// because they hold lifetime information. This helps prevent dangling pointer bugs.
pub unsafe trait Repr<T> {}

unsafe impl<T> Repr<T> for T {}

/// Provides type-safe pointer chain support.
pub trait ExtendableFrom<'a, T> {
    /// Appends `other`'s pointer chain to the end of this pointer chain.
    #[must_use]
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
unsafe fn append_ptr_chain(mut host: *mut vk1_0::BaseOutStructure, tail: *mut vk1_0::BaseOutStructure) {
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
