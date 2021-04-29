<img src="https://gitlab.com/Friz64/erupt/-/raw/master/logo.svg" height=200>

# erupt

[![docs.rs](https://docs.rs/erupt/badge.svg)](https://docs.rs/erupt)
[![crates.io](https://img.shields.io/crates/v/erupt.svg)](https://crates.io/crates/erupt)

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
erupt both provide Vulkan API bindings too, but not exposing such _fancy_ wrappers and instead
focusing on having good bindings to the _raw_ Vulkan API.

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
