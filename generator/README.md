# The erupt generator

To run, make sure you are in the root directory of this git repository.

By default, the generator requires a working `clang` installation in order to
preprocess the C Vulkan Header files.

```
cargo run --release -p generator
```

To allow for more flexibility, you can provide a
custom preprocessor path with the `-p` option.

Examples include:

```
cargo run --release -p generator -- -p gcc
```

```
cargo run --release -p generator -- -p /path/to/clang
```
