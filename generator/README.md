# The erupt generator

To run, make sure you are in the root directory of this git repository.

By default, the generator requires a working `clang` installation in order to
preprocess the C Vulkan Header files.

```
cargo run --release -p generator
```

## Options

- `-p`, `--preprocessor`: Provide a custom preprocessor path.
- `--vulkan-headers-path`: Specify a custom `Vulkan-Headers` directory.
  - **Please be aware that versions outside of `generator/Vulkan-Headers/` are
    unsupported and may fail to generate.**

## CLI Examples

```
cargo run --release -p generator -- -p gcc
```

```
cargo run --release -p generator -- -p /path/to/clang
```
