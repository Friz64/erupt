# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Changed
- Change cargo feature representation in the README

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
