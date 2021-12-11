# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.21.0+202] - 2021-12-11

### Added

- Add callback to wrappers returning a vector of extendable structs
- Add Builders for Instance and Device loaders to cover more advanced use cases
- Add `custom_loaders` example, showcasing usage of the new advanced loader APIs

### Changed

- Update Vulkan Headers to version 202
- Update `raw-window-handle` to 0.4
- Rename `.build()` to `.build_dangling()` to make it look less innocent
- Optimize construction of larger pointer chains
- Simplify function definitions containing handles
- Deprecate several invalid enum variants
- Improved clarity of `cstr!` macro implementation
  ([!9](https://gitlab.com/Friz64/erupt/-/merge_requests/9))

### Fixed

- Wrappers now return instead of taking a `&mut` in more cases
- Fix over-agressive trimming of "BIT" in enum variants

## [0.20.0+190] - 2021-08-30

### Added

- Added associated constants for structure types on structs

### Changed

- Update Vulkan Headers to version 190
- Derive `Hash`, `Eq` on applicable structs
- Use a `SmallVec` instead of a `Vec` when returning collections

### Fixed

- Fix erroneous instance version calculation
- Fix function definition of `vkVoidFunction`
- Correct the error handling implementation

## [0.19.0+182] - 2021-06-21

### Added

- Added versioning warning to provisional/beta extensions
- Allow for more ergonomic custom instance and device creation functions (useful for e.g. OpenXR)

### Changed

- Update Vulkan Headers to version 182
- Mark `{Instance,Device}Loader::new` functions unsafe
- Simplified function wrapper arguments
- Split `ExtendableFrom` into `Const` and `Mut` variants
- Remove function-specific "function not loaded" error messages
- Expose `FramebufferCreateInfoBuilder::attachment_count`
- Restructure `EntryLoader` naming
  - `DefaultEntryLoader` -> `EntryLoader`
  - `EntryLoader` -> `CustomEntryLoader`

### Fixed

- Fix possible violation of spec (`pSampleMask`)

## [0.18.0+174] - 2021-03-29

### Added

- Added `VulkanResult::map_err`
- Added `object_handle` method on handles
- Optional bytemuck support for some hand-picked structs (`*IndirectCommand`, etc.)

### Changed

- Update Vulkan Headers to version 174

### Fixed

- Bit-field support (Wrong layout of `VkAccelerationStructureInstanceKHR`)
  ([#10](https://gitlab.com/Friz64/erupt/-/issues/10))
- Fixed compilation errors in the `surface` module on macOS
  ([!6](https://gitlab.com/Friz64/erupt/-/merge_requests/6))

## [0.17.1+169] - 2021-02-09

### Fixed

- Fix nightly rustdoc errors

## [0.17.0+169] - 2021-02-09

### Added

- Added rustdoc alias support
- Added `vk` module, which re-exports every Vulkan item

### Changed

- Update `libloading` to 0.7
- Update Vulkan Headers to version 169
- Improved the idiomaticity of the `{Instance,Device}Enabled::new` API
- Transition to intra-doc links in the rustdoc

## [0.16.0+162] - 2020-11-24

### Changed

- Update Vulkan Headers to version 162
- Deprecate the included allocator in favor of `gpu-alloc`
- The `{Entry,Instance,Device}Enabled` information can now only be accessed through read-only getters

## [0.15.0+157] - 2020-10-12

### Changed

- Update Vulkan Headers to version 157

### Fixed

- Don't use `std::ffi::c_void` for the return type of functions
- Fix data types of `GgpStreamDescriptor` and `GgpFrameToken`

## [0.14.0+154] - 2020-09-21

### Added

- Add `#[track_caller]` to `VulkanResult`
- Add `Allocation::memory`, allowing access to the inner `DeviceMemory` handle

### Changed

- Update Vulkan Headers to version 154

### Fixed

- Add `#[repr(transparent)]` to enums/flagbits

## [0.13.0+150] - 2020-08-10

### Added

- Add `#[must_use]` to `ExtendableFrom::extend_from`

### Changed

- Accept null return values in the loaders
- Update Vulkan Headers to version 150

### Fixed

- Fix memory mapper calculations
  ([!5](https://gitlab.com/Friz64/erupt/-/merge_requests/5))

## [0.12.0+149] - 2020-08-04

### Added

- Add `DefaultEntryLoader::with_lib_path`

### Changed

- Update Vulkan Headers to version 149

### Fixed

- Fix loader not loading certain functions

## [0.11.0+148] - 2020-07-21

### Changed

- Rewrite generator from scratch
- Rework loader functionality
- Update Vulkan Headers to version 148

## [0.10.0+145] - 2020-06-21

### Added

- Derive `Ord` and `PartialOrd` for enums
- Implement `Debug` for loaders

### Fixed

- Fix some enum variants missing

## [0.9.0+143] - 2020-06-08

### Fixed

- Fix reversed array order
- Fix loaders failing on partial command availability

### Changed

- Update Vulkan Headers to version 143

## [0.8.0+142] - 2020-06-01

### Added

- Add confirmed platform support in the README

### Changed

- Update Vulkan Headers to version 142
- Support an extra layer of nested pointers
- Update surface util based on latest `ash-window` changes

### Fixed

- Fix surface format selection fallback in triangle example

## [0.7.0+141] - 2020-05-15

### Added

- Add documentation example for the allocator
- Add `DefaultCoreLoader` alias
- Better builder support for void pointers

### Changed

- Update Vulkan Headers to version 141
- Rename `erupt-examples` to `erupt_examples`
- Make `compile-shaders.sh` directory independent

### Fixed

- Fix extension ordering

## [0.6.0+140] - 2020-05-04

### Added

- Add basic Vulkan memory allocator to `utils`
- Add compute example, making use of the new allocator

### Changed

- Update Vulkan Headers to version 140
- Check for supported Vulkan version in `pointer-chain` example
- Convert `Bool32` to `bool` in function wrapper return type

## [0.5.1+139] - 2020-04-28

### Changed

- Update Vulkan Headers to version 139

## [0.5.0+137] - 2020-04-26

### Added

- Add `LibraryError` wrapper type to avoid exposing `libloading` types
- Add `try_vk` macro, like `try`, but for `VulkanResult`
- Add Manual Page to Builder Documentation
- Add `#[inline]` to `VulkanResult` functions

### Changed

- Change cargo feature representation in the README
- Make use of `try_vk` in Instance Creation Example
- Move loading to it's own module
- Move examples to bin dir to avoid manual entries
- Switch to Rust 1.43 associated constants in triangle example

### Fixed

- Add special cases for some builders

## [0.4.0+137] - 2020-04-19

### Added

- Add compile time comparison to the FAQ

### Changed

- Switch to generic `ExtendableBy` trait
- Correct README title indents

### Removed

- Remove redundant `info` module

### Fixed

- Fix small `VulkanResult` fmt nitpicks
- Fix more small generator nitpicks

## [0.3.0+137] - 2020-04-18

### Added

- Add `utils::decode_spv`
- Add FAQ to README
- Implement `Error` and `Display` for `vk1_0::Result`

### Changed

- Switch to `&[u32]` in `ShaderModuleCreateInfoBuilder`

### Fixed

- Fix small generator nitpicks

## [0.2.0+137] - 2020-04-17

### Added

- Add safety warning to `extend` functions

### Changed

- Mark `extend` functions as unsafe
- Update `Vulkan-Headers` submodule
- Refactor `pointer-chain` example

## [0.1.1+137] - 2020-04-16

### Added

- Add Vulkan Header version build metadata in semver

### Changed

- Switch to zlib license

### Fixed

- Fix wrong relative path in documention

## [0.1.0] - 2020-04-15

_Initial Development_

[Unreleased]: https://gitlab.com/Friz64/erupt/-/compare/v0.20.0+190...main
[0.20.0+190]: https://gitlab.com/Friz64/erupt/-/compare/v0.19.0+182...v0.20.0+190
[0.19.0+182]: https://gitlab.com/Friz64/erupt/-/compare/v0.18.0+174...v0.19.0+182
[0.18.0+174]: https://gitlab.com/Friz64/erupt/-/compare/v0.17.1+169...v0.18.0+174
[0.17.1+169]: https://gitlab.com/Friz64/erupt/-/compare/v0.17.0+169...v0.17.1+169
[0.17.0+169]: https://gitlab.com/Friz64/erupt/-/compare/v0.16.0+162...v0.17.0+169
[0.16.0+162]: https://gitlab.com/Friz64/erupt/-/compare/v0.15.0+157...v0.16.0+162
[0.15.0+157]: https://gitlab.com/Friz64/erupt/-/compare/v0.14.0+154...v0.15.0+157
[0.14.0+154]: https://gitlab.com/Friz64/erupt/-/compare/v0.13.0+150...v0.14.0+154
[0.13.0+150]: https://gitlab.com/Friz64/erupt/-/compare/v0.12.0+149...v0.13.0+150
[0.12.0+149]: https://gitlab.com/Friz64/erupt/-/compare/v0.11.0+148...v0.12.0+149
[0.11.0+148]: https://gitlab.com/Friz64/erupt/-/compare/v0.10.0+145...v0.11.0+148
[0.10.0+145]: https://gitlab.com/Friz64/erupt/-/compare/v0.9.0+143...v0.10.0+145
[0.9.0+143]: https://gitlab.com/Friz64/erupt/-/compare/v0.8.0+142...v0.9.0+143
[0.8.0+142]: https://gitlab.com/Friz64/erupt/-/compare/v0.7.0+141...v0.8.0+142
[0.7.0+141]: https://gitlab.com/Friz64/erupt/-/compare/v0.6.0+140...v0.7.0+141
[0.6.0+140]: https://gitlab.com/Friz64/erupt/-/compare/v0.5.1+139...v0.6.0+140
[0.5.1+139]: https://gitlab.com/Friz64/erupt/-/compare/v0.5.0+137...v0.5.1+139
[0.5.0+137]: https://gitlab.com/Friz64/erupt/-/compare/v0.4.0+137...v0.5.0+137
[0.4.0+137]: https://gitlab.com/Friz64/erupt/-/compare/v0.3.0+137...v0.4.0+137
[0.3.0+137]: https://gitlab.com/Friz64/erupt/-/compare/v0.2.0+137...v0.3.0+137
[0.2.0+137]: https://gitlab.com/Friz64/erupt/-/compare/v0.1.1+137...v0.2.0+137
[0.1.1+137]: https://gitlab.com/Friz64/erupt/-/compare/v0.1.0...v0.1.1+137
[0.1.0]: https://gitlab.com/Friz64/erupt/-/commit/c3083459
