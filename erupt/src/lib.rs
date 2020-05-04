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
//! - Function loading ([`CoreLoader`], [`InstanceLoader`], [`DeviceLoader`])
//! - Seperate `Flags` and `FlagBits` types
//! - A high level `Builder` for every struct
//! - Type-safe pointer chain support
//! - `Default` and `Debug` implementation for every type
//! - Complete auto-generation of everything except [`utils`]
//!
//! ## Example: Instance Creation
//! ```rust ignore
//! use erupt::{vk1_0::*, CoreLoader, InstanceLoader};
//!
//! let mut core = CoreLoader::new()?;
//! core.load_vk1_0()?;
//!
//! let app_info = ApplicationInfoBuilder::new().api_version(erupt::make_version(1, 0, 0));
//! let instance_info = InstanceCreateInfoBuilder::new().application_info(&app_info);
//! let instance_handle = try_vk!(core.create_instance(&instance_info, None, None));
//!
//! let mut instance = InstanceLoader::new(&core, instance_handle)?;
//! instance.load_vk1_0()?;
//!
//! // ...
//!
//! instance.destroy_instance(None);
//! ```
//!
//! ## Additional examples
//! - [triangle](https://gitlab.com/Friz64/erupt/-/blob/master/erupt-examples/src/bin/triangle.rs)
//! - [pointer-chain](https://gitlab.com/Friz64/erupt/-/blob/master/erupt-examples/src/bin/pointer_chain.rs)
//! - [version](https://gitlab.com/Friz64/erupt/-/blob/master/erupt-examples/src/bin/version.rs)
//!
//! ## Cargo Features
//! - `surface` (enabled by default): Enables the [`surface`] module, adds [`raw-window-handle`] dependency
//! - `loading` (enabled by default): Enables the [`CoreLoader::new`] function, adds [`libloading`] dependency
//!
//! ## FAQ
//! ### Q: What's the difference between this, ash and vulkano?
//! A: Vulkano is special because it provides hand-written Vulkan wrappers, which means that for example it
//! has a special hand-written written wrapper around a Vulkan `PhysicalDevice`. On the other hand ash and erupt
//! both provide Vulkan API bindings too, but not exposing such *fancy* wrappers and instead focusing on having
//! good bindings to the *raw* Vulkan API.
//!
//! The big selling points of erupt is that it has better documentation, high level function support for all
//! extensions (which is only really relevant if you use those extensions), being fully generated (which is not
//! visible to the end user), having faster compile times and some more smaller improvements. On the other hand
//! ash has a bigger existing community.
//!
//! ### Q: How do the compile times compare to ash?
//! A: erupt `0.5.1+139` compiles in around 72% of the time it takes for ash `0.30.0` to compile, tested on
//! Linux with rustc nightly `1.44.0` using `-Ztimings=info`.
//!
//! ### Q: What does the number at the end of the version mean?
//! A: It represents the Vulkan Header version this version of erupt was generated against and is purely informational.
//!
//! ## Thank you
//! - [`vk-parse`](https://crates.io/crates/vk-parse) for helping parse `vk.xml` in the [`generator`](https://gitlab.com/Friz64/erupt/-/tree/master/generator)
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
//! [`CoreLoader`]: https://docs.rs/erupt/*/erupt/struct.CoreLoader.html
//! [`CoreLoader::new`]: https://docs.rs/erupt/*/erupt/struct.CoreLoader.html#method.new
//! [`InstanceLoader`]: https://docs.rs/erupt/*/erupt/struct.CoreLoader.html
//! [`DeviceLoader`]: https://docs.rs/erupt/*/erupt/struct.CoreLoader.html
//! [`utils`]: https://docs.rs/erupt/*/erupt/utils/index.html

mod generated;
pub mod utils;

pub use generated::*;

/// Construct a `*const std::os::raw::c_char` from a string
///
/// # Example
/// ```ignore
/// const LAYER_KHRONOS_VALIDATION: *const c_char = cstr!("VK_LAYER_KHRONOS_validation");
/// ```
#[macro_export]
macro_rules! cstr {
    ($s:expr) => {
        concat!($s, "\0") as *const str as *const std::os::raw::c_char
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
    ($name:ident, $ty:ident, $doc_link: meta) => {
        #[repr(transparent)]
        #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Default)]
        #[$doc_link]
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
macro_rules! handle {
    ($name:ident, $ty:ident, $doc_link:meta) => {
        #[repr(transparent)]
        #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash)]
        #[$doc_link]
        pub struct $name(pub *mut u8);

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

/// Used in `extend` functions for type safe pointer chains
pub trait ExtendableBy<T> {}

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

// from winapi
#[doc(hidden)]
#[allow(non_camel_case_types, non_snake_case)]
pub struct SECURITY_ATTRIBUTES {
    pub nLength: u32,
    pub lpSecurityDescriptor: *mut std::ffi::c_void,
    pub bInheritHandle: i32,
}
