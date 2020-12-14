# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- Added rustdoc alias support
- Added `vk` module, which re-exports every Vulkan item

### Changed
- Update Vulkan Headers to version 165
- Improved the idiomaticity of the `{Instance,Device}Enabled::new` API
- Transition to intra-doc links in the rustdoc

## [0.16.0+162] - 2020-11-24
### Changed
- Update Vulkan Headers to version 162
- Deprecate the included allocator in favor of `gpu-alloc`
- The `{Entry,Instance,Device}Enabled` information can now only be accessed through read-only getters

## [0.15.0+157] - 2020-10-25
### Changed
- Update Vulkan Headers to version 158

### Fixed
- Don't use `std::ffi::c_void` for the return type of functions
- Fix data types of `GgpStreamDescriptor` and `GgpFrameToken`

## [0.14.0+154] - 2020-09-21
### Changed
- Update Vulkan Headers to version 157

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
- Fix memory mapper calculations (https://gitlab.com/Friz64/erupt/-/merge_requests/5)

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
*Initial Development*
