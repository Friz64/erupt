#![allow(clippy::all, unreachable_patterns)]
#![doc(html_logo_url = "https://gitlab.com/Friz64/erupt/-/raw/main/logo.svg")]
/*!
Vulkan API bindings

Take a look at the [`erupt` user guide](https://gitlab.com/Friz64/erupt/-/blob/main/USER_GUIDE.md).

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

let app_info = vk::ApplicationInfoBuilder::new()
    .api_version(vk::API_VERSION_1_1);
let instance_info = vk::InstanceCreateInfoBuilder::new()
    .application_info(&app_info);
let instance = InstanceLoader::new(&entry, &instance_info)?;

// ...

instance.destroy_instance(None);
```

## Additional examples

- [triangle](https://gitlab.com/Friz64/erupt/-/blob/main/erupt_examples/src/bin/triangle.rs)
- [pointer_chain](https://gitlab.com/Friz64/erupt/-/blob/main/erupt_examples/src/bin/pointer_chain.rs)
- [version](https://gitlab.com/Friz64/erupt/-/blob/main/erupt_examples/src/bin/version.rs)
- [custom_loaders](https://gitlab.com/Friz64/erupt/-/blob/main/erupt_examples/src/bin/custom_loaders.rs)

## Cargo Features

- `surface` (enabled by default): Enables the [`surface`] module, adds [`raw-window-handle`] dependency
- `loading` (enabled by default): Enables the [`EntryLoader::new`] function, adds [`libloading`] dependency
- `bytemuck`: Implements [`Pod`] for some hand-picked structs (`*IndirectCommand`, etc.), adds [`bytemuck`] dependency

## FAQ

### Q: What's the difference between this, ash and vulkano?

A: Vulkano is special because it provides hand-written Vulkan wrappers, which means that for example
it has a special hand-written wrapper around a Vulkan `PhysicalDevice`. On the other hand ash and
erupt both provide Vulkan API bindings too, but not exposing such _fancy_ wrappers and instead
focusing on having good bindings to the _raw_ Vulkan API.

The big selling points of erupt is that it has better documentation, high level function support for
all extensions (which is only really relevant if you use those extensions), being fully generated
and some more smaller improvements. On the other hand ash has a bigger existing community.

### Q: What does the number at the end of the version mean?

A: It represents the Vulkan Header version this version of erupt was generated against and is purely
informational.

### Q: I need to easily allocate memory, what should i use?

A: Take a look at [`gpu-alloc`](https://github.com/zakarumych/gpu-alloc),
[`vk-alloc`](https://github.com/hasenbanck/vk-alloc) for Rust-native
solutions and [`vk-mem-erupt`](https://github.com/HindrikStegenga/vk-mem-rs) for
bindings to the C++ Vulkan Memory Allocator (VMA) library.

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

This project is licensed under the [zlib License](https://gitlab.com/Friz64/erupt/-/blob/main/LICENSE).

[utility module]: https://docs.rs/erupt/%2A/erupt/utils/index.html
[`vulkanresult`]: https://docs.rs/erupt/%2A/erupt/utils/struct.VulkanResult.html
[`surface`]: https://docs.rs/erupt/%2A/erupt/utils/surface/index.html
[`surfacekhr`]: https://docs.rs/erupt/%2A/erupt/extensions/khr_surface/struct.SurfaceKHR.html
[`allocator`]: https://docs.rs/erupt/%2A/erupt/utils/allocator/index.html
[`rawwindowhandle`]: https://docs.rs/raw-window-handle/%2A/raw_window_handle/enum.RawWindowHandle.html
[`libloading`]: https://crates.io/crates/libloading
[`raw-window-handle`]: https://crates.io/crates/raw-window-handle
[`ash-window`]: https://crates.io/crates/ash-window
[`entryloader`]: https://docs.rs/erupt/%2A/erupt/struct.EntryLoader.html
[`entryloader::new`]: https://docs.rs/erupt/%2A/erupt/struct.EntryLoader.html#method.new
[`pod`]: https://docs.rs/bytemuck/%2A/bytemuck/trait.Pod.html
[`bytemuck`]: https://crates.io/crates/bytemuck
[`instanceloader`]: https://docs.rs/erupt/%2A/erupt/struct.InstanceLoader.html
[`deviceloader`]: https://docs.rs/erupt/%2A/erupt/struct.DeviceLoader.html
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
    sync::Arc,
};
#[cfg(feature = "loading")]
pub use utils::loading::EntryLoader;

/// Construct a `*const std::os::raw::c_char` from a string.
///
/// # Example
/// ```ignore
/// const LAYER_KHRONOS_VALIDATION: *const c_char = cstr!("VK_LAYER_KHRONOS_validation");
/// ```
#[macro_export]
macro_rules! cstr {
    ($s:expr) => {
        concat!($s, "\0").as_ptr().cast::<::std::os::raw::c_char>()
    };
}

/// Like `try!`, but for [`utils::VulkanResult`](utils/struct.VulkanResult.html).
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

const NOT_LOADED_MESSAGE: &str = "tried to call a function that isn't loaded: \
    is the respective `enabled_extension_names` array correct? \
    does the configured Vulkan version support this function?";

/// Allows returning small amounts of data (specifically with a length <= 8)
/// without needlessly allocating heap memory.
pub type SmallVec<T> = smallvec::SmallVec<[T; 8]>;

/// An error which can occur while initializing a loader.
#[derive(Debug)]
pub enum LoaderError {
    /// A Vulkan function returned a negative `Result` value.
    VulkanError(vk1_0::Result),
    /// A symbol was not available while it should have been.
    SymbolNotAvailable,
}

impl Display for LoaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LoaderError::VulkanError(_) => {
                write!(f, "a Vulkan function returned a negative `Result` value")
            }
            LoaderError::SymbolNotAvailable => {
                write!(f, "a symbol was not available while it should have been")
            }
        }
    }
}

impl Error for LoaderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            LoaderError::VulkanError(vk_result) => Some(vk_result),
            LoaderError::SymbolNotAvailable => None,
        }
    }
}

impl<T> CustomEntryLoader<T> {
    /// Creates a entry loader with a custom library used for loading.
    pub unsafe fn with_library(
        mut library: T,
        mut symbol: impl FnMut(&mut T, *const std::os::raw::c_char) -> Option<vk1_0::PFN_vkVoidFunction>,
    ) -> Result<Self, LoaderError> {
        Ok(
            EntryEnabled::new(&mut library, &mut symbol).and_then(|entry_enabled| {
                CustomEntryLoader::custom(library, &mut symbol, entry_enabled)
            })?,
        )
    }

    /// Access enabled requirements of this entry loader.
    pub fn enabled(&self) -> &EntryEnabled {
        &self.enabled
    }

    /// This will be the result of `vkEnumerateInstanceVersion` if available, otherwise `0.1.0.0`.
    pub fn instance_version(&self) -> u32 {
        self.enabled.instance_version
    }
}

impl<T> Drop for CustomEntryLoader<T> {
    fn drop(&mut self) {
        if Arc::weak_count(&self.arc) != 0 {
            panic!("attempting to drop a entry loader with active references to it");
        }
    }
}

impl<T> Debug for CustomEntryLoader<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Entry")
    }
}

/// Builder for an instance loader.
pub struct InstanceLoaderBuilder<'a> {
    create_instance_fn: Option<
        Box<
            dyn FnOnce(
                &vk1_0::InstanceCreateInfo,
                Option<&vk1_0::AllocationCallbacks>,
            ) -> utils::VulkanResult<vk1_0::Instance>,
        >,
    >,
    symbol_fn: Option<
        &'a mut dyn FnMut(
            vk1_0::Instance,
            *const std::os::raw::c_char,
        ) -> Option<vk1_0::PFN_vkVoidFunction>,
    >,
    allocation_callbacks: Option<&'a vk1_0::AllocationCallbacks>,
}

impl<'a> InstanceLoaderBuilder<'a> {
    /// Create a new instance loader builder.
    pub fn new() -> Self {
        InstanceLoaderBuilder {
            create_instance_fn: None,
            symbol_fn: None,
            allocation_callbacks: None,
        }
    }

    /// Specify a custom instance creation function, to use in place of the
    /// default.
    ///
    /// This may be useful when creating the instance using e.g. OpenXR.
    pub fn create_instance_fn(
        mut self,
        create_instance: Box<
            dyn FnOnce(
                &vk1_0::InstanceCreateInfo,
                Option<&vk1_0::AllocationCallbacks>,
            ) -> utils::VulkanResult<vk1_0::Instance>,
        >,
    ) -> Self {
        self.create_instance_fn = Some(create_instance);
        self
    }

    /// Specify a custom symbol function, called to get instance function
    /// pointers, to use in place of the default.
    pub fn symbol_fn(
        mut self,
        symbol: &'a mut impl FnMut(
            vk1_0::Instance,
            *const std::os::raw::c_char,
        ) -> Option<vk1_0::PFN_vkVoidFunction>,
    ) -> Self {
        self.symbol_fn = Some(symbol);
        self
    }

    /// Specify custom allocation callback functions.
    pub fn allocation_callbacks(mut self, allocator: &'a vk1_0::AllocationCallbacks) -> Self {
        self.allocation_callbacks = Some(allocator);
        self
    }

    /// Create an instance loader.
    pub unsafe fn build<T>(
        self,
        entry_loader: &'a CustomEntryLoader<T>,
        create_info: &vk1_0::InstanceCreateInfo,
    ) -> Result<InstanceLoader, LoaderError> {
        let instance = match self.create_instance_fn {
            Some(create_instance) => create_instance(create_info, self.allocation_callbacks),
            None => entry_loader.create_instance(create_info, self.allocation_callbacks),
        };

        let instance = instance.result().map_err(LoaderError::VulkanError)?;

        let mut version = vk1_0::make_api_version(0, 1, 0, 0);
        if !create_info.p_application_info.is_null() {
            let user_version = (*create_info.p_application_info).api_version;
            if user_version != 0 {
                version = user_version;
            }
        }

        let enabled_extensions = std::slice::from_raw_parts(
            create_info.pp_enabled_extension_names,
            create_info.enabled_extension_count as _,
        );
        let enabled_extensions: Vec<_> = enabled_extensions
            .iter()
            .map(|&ptr| CStr::from_ptr(ptr))
            .collect();

        let mut default_symbol = move |name| (entry_loader.get_instance_proc_addr)(instance, name);
        let mut symbol: &mut dyn FnMut(
            *const std::os::raw::c_char,
        ) -> Option<vk1_0::PFN_vkVoidFunction> = &mut default_symbol;
        let mut user_symbol;
        if let Some(internal_symbol) = self.symbol_fn {
            user_symbol = move |name| internal_symbol(instance, name);
            symbol = &mut user_symbol;
        }

        let all_physical_device_extension_properties =
            all_physical_device_extension_properties(&mut symbol, instance)?;
        let available_device_extensions: Vec<_> = all_physical_device_extension_properties
            .iter()
            .map(|properties| CStr::from_ptr(properties.extension_name.as_ptr()))
            .collect();

        let instance_enabled =
            InstanceEnabled::new(version, &enabled_extensions, &available_device_extensions)?;
        InstanceLoader::custom(entry_loader, instance, instance_enabled, symbol)
    }
}

impl InstanceLoader {
    /// Creates a new instance loader.
    ///
    /// For more advanced use cases, take a look at [`InstanceLoaderBuilder`].
    #[inline]
    pub unsafe fn new<T>(
        entry_loader: &CustomEntryLoader<T>,
        create_info: &vk1_0::InstanceCreateInfo,
    ) -> Result<InstanceLoader, LoaderError> {
        InstanceLoaderBuilder::new().build(entry_loader, create_info)
    }

    /// Access enabled requirements of this instance loader.
    pub fn enabled(&self) -> &InstanceEnabled {
        &self.enabled
    }
}

impl Drop for InstanceLoader {
    fn drop(&mut self) {
        if Arc::weak_count(&self.arc) != 0 {
            panic!("attempting to drop a instance loader with active references to it");
        }
    }
}

// This is needed for available device extensions.
//
// Vulkan spec: An "available device extension" is a device extension supported
// by any physical device enumerated by instance.
// https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetInstanceProcAddr.html#_description
unsafe fn all_physical_device_extension_properties(
    symbol: &mut impl FnMut(*const std::os::raw::c_char) -> Option<vk1_0::PFN_vkVoidFunction>,
    instance: vk1_0::Instance,
) -> Result<Vec<vk1_0::ExtensionProperties>, LoaderError> {
    let enumerate_physical_devices: vk1_0::PFN_vkEnumeratePhysicalDevices = mem::transmute(
        symbol(vk1_0::FN_ENUMERATE_PHYSICAL_DEVICES).ok_or(LoaderError::SymbolNotAvailable)?,
    );
    let enumerate_device_extension_properties: vk1_0::PFN_vkEnumerateDeviceExtensionProperties =
        mem::transmute(
            symbol(vk1_0::FN_ENUMERATE_DEVICE_EXTENSION_PROPERTIES)
                .ok_or(LoaderError::SymbolNotAvailable)?,
        );

    let mut physical_device_count = 0;
    let result = enumerate_physical_devices(instance, &mut physical_device_count, ptr::null_mut());
    if result.0 < 0 {
        return Err(LoaderError::VulkanError(result));
    }

    let mut physical_devices = vec![Default::default(); physical_device_count as usize];
    let result = enumerate_physical_devices(
        instance,
        &mut physical_device_count,
        physical_devices.as_mut_ptr(),
    );
    if result.0 < 0 {
        return Err(LoaderError::VulkanError(result));
    }

    let mut all_physical_device_extension_properties = Vec::new();
    for physical_device in physical_devices {
        let mut property_count = 0;
        let result = enumerate_device_extension_properties(
            physical_device,
            ptr::null(),
            &mut property_count,
            ptr::null_mut(),
        );
        if result.0 < 0 {
            return Err(LoaderError::VulkanError(result));
        }

        let mut properties = vec![Default::default(); property_count as usize];
        let result = enumerate_device_extension_properties(
            physical_device,
            ptr::null(),
            &mut property_count,
            properties.as_mut_ptr(),
        );
        if result.0 < 0 {
            return Err(LoaderError::VulkanError(result));
        }

        all_physical_device_extension_properties.extend(properties.into_iter());
    }

    Ok(all_physical_device_extension_properties)
}

impl Debug for InstanceLoader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(&self.handle, f)
    }
}

/// Builder for an device loader.
pub struct DeviceLoaderBuilder<'a> {
    create_device_fn: Option<
        Box<
            dyn FnOnce(
                vk1_0::PhysicalDevice,
                &vk1_0::DeviceCreateInfo,
                Option<&vk1_0::AllocationCallbacks>,
            ) -> utils::VulkanResult<vk1_0::Device>,
        >,
    >,
    symbol_fn: Option<
        &'a mut dyn FnMut(
            vk1_0::Device,
            *const std::os::raw::c_char,
        ) -> Option<vk1_0::PFN_vkVoidFunction>,
    >,
    allocation_callbacks: Option<&'a vk1_0::AllocationCallbacks>,
}

impl<'a> DeviceLoaderBuilder<'a> {
    /// Create a new instance loader builder.
    pub fn new() -> Self {
        DeviceLoaderBuilder {
            create_device_fn: None,
            symbol_fn: None,
            allocation_callbacks: None,
        }
    }

    /// Specify a custom device creation function, to use in place of the
    /// default.
    ///
    /// This may be useful when creating the device using e.g. OpenXR.
    pub fn create_device_fn(
        mut self,
        create_device: Box<
            dyn FnOnce(
                vk1_0::PhysicalDevice,
                &vk1_0::DeviceCreateInfo,
                Option<&vk1_0::AllocationCallbacks>,
            ) -> utils::VulkanResult<vk1_0::Device>,
        >,
    ) -> Self {
        self.create_device_fn = Some(create_device);
        self
    }

    /// Specify a custom symbol function, called to get device function
    /// pointers, to use in place of the default.
    pub fn symbol_fn(
        mut self,
        symbol: &'a mut impl FnMut(
            vk1_0::Device,
            *const std::os::raw::c_char,
        ) -> Option<vk1_0::PFN_vkVoidFunction>,
    ) -> Self {
        self.symbol_fn = Some(symbol);
        self
    }

    /// Specify custom allocation callback functions.
    pub fn allocation_callbacks(mut self, allocator: &'a vk1_0::AllocationCallbacks) -> Self {
        self.allocation_callbacks = Some(allocator);
        self
    }

    /// Build the device loader. Ensure `create_info` is the same as used in the
    /// creation of `device`.
    pub unsafe fn build_with_existing_device(
        self,
        instance_loader: &'a InstanceLoader,
        device: vk1_0::Device,
        create_info: &vk1_0::DeviceCreateInfo,
    ) -> Result<DeviceLoader, LoaderError> {
        let device_enabled = {
            let enabled_extensions = std::slice::from_raw_parts(
                create_info.pp_enabled_extension_names,
                create_info.enabled_extension_count as _,
            );
            let enabled_extensions: Vec<_> = enabled_extensions
                .iter()
                .map(|&ptr| CStr::from_ptr(ptr))
                .collect();
            DeviceEnabled::new(&enabled_extensions)
        };

        let mut default_symbol = move |name| (instance_loader.get_device_proc_addr)(device, name);
        let mut symbol: &mut dyn FnMut(
            *const std::os::raw::c_char,
        ) -> Option<vk1_0::PFN_vkVoidFunction> = &mut default_symbol;
        let mut user_symbol;
        if let Some(internal_symbol) = self.symbol_fn {
            user_symbol = move |name| internal_symbol(device, name);
            symbol = &mut user_symbol;
        }

        DeviceLoader::custom(instance_loader, device, device_enabled, symbol)
    }

    /// Build the device loader. If you want to entirely create the device
    /// yourself, use [`DeviceLoaderBuilder::build_with_existing_device`].
    pub unsafe fn build(
        mut self,
        instance_loader: &'a InstanceLoader,
        physical_device: vk1_0::PhysicalDevice,
        create_info: &vk1_0::DeviceCreateInfo,
    ) -> Result<DeviceLoader, LoaderError> {
        let device = match self.create_device_fn.take() {
            Some(create_device) => {
                create_device(physical_device, create_info, self.allocation_callbacks)
            }
            None => instance_loader.create_device(
                physical_device,
                create_info,
                self.allocation_callbacks,
            ),
        };

        let device = device.result().map_err(LoaderError::VulkanError)?;
        self.build_with_existing_device(instance_loader, device, create_info)
    }
}

impl DeviceLoader {
    /// Creates a new device loader.
    ///
    /// For more advanced use cases, take a look at [`DeviceLoaderBuilder`].
    #[inline]
    pub unsafe fn new(
        instance_loader: &InstanceLoader,
        physical_device: vk1_0::PhysicalDevice,
        create_info: &vk1_0::DeviceCreateInfo,
    ) -> Result<DeviceLoader, LoaderError> {
        DeviceLoaderBuilder::new().build(instance_loader, physical_device, create_info)
    }

    /// Access enabled requirements of this device loader.
    pub fn enabled(&self) -> &DeviceEnabled {
        &self.enabled
    }
}

impl Debug for DeviceLoader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(&self.handle, f)
    }
}

/// Provides type-safe pointer chain support.
pub trait ExtendableFrom<'a, T> {
    /// Inserts `addition` (+ its pointer chain) between the head and tail of
    /// this pointer chain.
    ///
    /// - Head of `self`
    ///   - Head of `addition`
    ///   - _Tail of `addition`_
    ///   - _Tail of `addition`_
    ///   - _Tail of `addition`_
    ///   - _..._
    /// - _Tail of `self`_
    /// - _Tail of `self`_
    /// - _Tail of `self`_
    /// - _..._
    ///
    /// The implementation is like this specifically, because this trait is only
    /// ever implemented on the "main carrier" of the pointer chain, which means
    /// that `addition` doesn't usually have a "tail" (unless, you've set the
    /// pointers by yourself, which this consequently also supports). This saves
    /// us from iterating the entire pointer chain each time an item is added.
    #[must_use]
    fn extend_from(mut self, addition: &'a mut T) -> Self
    where
        Self: Sized,
    {
        unsafe {
            insert_ptr_chain(&mut self as *mut Self as _, addition as *mut T as _);
        }

        self
    }
}

// safety: assumes all pointers in the pointer chain are valid
#[inline]
unsafe fn insert_ptr_chain(
    mut host: *mut vk1_0::BaseOutStructure,
    mut addition: *mut vk1_0::BaseOutStructure,
) {
    let addition_head = addition;
    let addition_end = loop {
        let p_next = (*addition).p_next;
        if p_next.is_null() {
            break addition;
        } else {
            addition = p_next;
        }
    };

    let prev_host_next = (*host).p_next;
    (*host).p_next = addition_head;
    (*addition_end).p_next = prev_host_next;
}

#[cfg(test)]
mod tests {
    use super::{vk, ExtendableFrom};
    use std::{iter, ptr};

    // safety: assumes all pointers in the pointer chain are valid
    unsafe fn iterate_ptr_chain(
        mut item: *mut vk::BaseOutStructure,
    ) -> impl Iterator<Item = *mut vk::BaseOutStructure> {
        iter::from_fn(move || unsafe {
            if item.is_null() {
                None
            } else {
                let current_item = item;
                item = (*item).p_next;
                Some(current_item)
            }
        })
    }

    #[test]
    fn ptr_chain_simple() {
        // s1 -> s2 -> s3 -> (null)
        let mut s1 = vk::BaseOutStructure {
            s_type: vk::StructureType(1),
            p_next: ptr::null_mut(),
        };
        let s1 = ptr::addr_of_mut!(s1);
        let mut s2 = vk::BaseOutStructure {
            s_type: vk::StructureType(2),
            p_next: ptr::null_mut(),
        };
        let s2 = ptr::addr_of_mut!(s2);
        let mut s3 = vk::BaseOutStructure {
            s_type: vk::StructureType(3),
            p_next: ptr::null_mut(),
        };
        let s3 = ptr::addr_of_mut!(s3);
        unsafe {
            (*s1).p_next = s2;
            (*s2).p_next = s3;
        }

        // s4 -> (null)
        let mut s4 = vk::BaseOutStructure {
            s_type: vk::StructureType(4),
            p_next: ptr::null_mut(),
        };
        let s4 = ptr::addr_of_mut!(s4);
        // s5 -> (null)
        let mut s5 = vk::BaseOutStructure {
            s_type: vk::StructureType(5),
            p_next: ptr::null_mut(),
        };
        let s5 = ptr::addr_of_mut!(s5);
        // s6 -> (null)
        let mut s6 = vk::BaseOutStructure {
            s_type: vk::StructureType(6),
            p_next: ptr::null_mut(),
        };
        let s6 = ptr::addr_of_mut!(s6);

        // s1 -> s6 -> s5 -> s4 -> s2 -> s3 -> (null)
        unsafe {
            super::insert_ptr_chain(s1, s4);
            super::insert_ptr_chain(s1, s5);
            super::insert_ptr_chain(s1, s6);
        }

        let mut iter = unsafe { iterate_ptr_chain(s1) }.map(|item| unsafe { (*item).s_type.0 });
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(6));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn ptr_chain_addition_chain() {
        // s1 -> s2 -> s3 -> (null)
        let mut s1 = vk::BaseOutStructure {
            s_type: vk::StructureType(1),
            p_next: ptr::null_mut(),
        };
        let s1 = ptr::addr_of_mut!(s1);
        let mut s2 = vk::BaseOutStructure {
            s_type: vk::StructureType(2),
            p_next: ptr::null_mut(),
        };
        let s2 = ptr::addr_of_mut!(s2);
        let mut s3 = vk::BaseOutStructure {
            s_type: vk::StructureType(3),
            p_next: ptr::null_mut(),
        };
        let s3 = ptr::addr_of_mut!(s3);
        unsafe {
            (*s1).p_next = s2;
            (*s2).p_next = s3;
        }

        // s4 -> s5 -> s6 -> (null)
        let mut s4 = vk::BaseOutStructure {
            s_type: vk::StructureType(4),
            p_next: ptr::null_mut(),
        };
        let s4 = ptr::addr_of_mut!(s4);
        let mut s5 = vk::BaseOutStructure {
            s_type: vk::StructureType(5),
            p_next: ptr::null_mut(),
        };
        let s5 = ptr::addr_of_mut!(s5);
        let mut s6 = vk::BaseOutStructure {
            s_type: vk::StructureType(6),
            p_next: ptr::null_mut(),
        };
        let s6 = ptr::addr_of_mut!(s6);
        unsafe {
            (*s4).p_next = s5;
            (*s5).p_next = s6;
        }

        // s1 -> s4 -> s5 -> s6 -> s2 -> s3 -> (null)
        unsafe {
            super::insert_ptr_chain(s1, s4);
        }

        let mut iter = unsafe { iterate_ptr_chain(s1) }.map(|item| unsafe { (*item).s_type.0 });
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), Some(6));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn ptr_chain_real_world() {
        let mut vk1_1features = vk::PhysicalDeviceVulkan11FeaturesBuilder::new();
        let mut vk1_2features = vk::PhysicalDeviceVulkan12FeaturesBuilder::new();
        let mut features = vk::PhysicalDeviceFeatures2Builder::new()
            .extend_from(&mut vk1_1features)
            .extend_from(&mut vk1_2features);

        let base_ptr = ptr::addr_of_mut!(features) as *mut vk::BaseOutStructure;
        let mut iter = unsafe { iterate_ptr_chain(base_ptr) }.map(|item| unsafe { (*item).s_type });
        assert_eq!(
            iter.next(),
            Some(vk::StructureType::PHYSICAL_DEVICE_FEATURES_2)
        );
        assert_eq!(
            iter.next(),
            Some(vk::StructureType::PHYSICAL_DEVICE_VULKAN_1_2_FEATURES)
        );
        assert_eq!(
            iter.next(),
            Some(vk::StructureType::PHYSICAL_DEVICE_VULKAN_1_1_FEATURES)
        );
        assert_eq!(iter.next(), None);
    }
}
