# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- Add documentation example for the allocator

### Changed
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
