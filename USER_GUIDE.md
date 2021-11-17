# `erupt` user guide

For any quick questions, don't hesitate to ask on the
[Game Development in Rust](https://discord.gg/yNtPTb2) discord (`Friz64#4876`).

## Builders

Every struct has a corresponding auto-generated builder. Create one using the
`*Builder::new()` function. Builders not only ease the use of the API by filling
in necessary fields for you (e.g. filling the length when passing in arrays),
they also hold lifetime information of referenced items.

```rust
let app_info = vk::ApplicationInfoBuilder::new()
    .api_version(vk::API_VERSION_1_1);

let instance_info = vk::InstanceCreateInfoBuilder::new()
    .application_info(&app_info); // <--- `Deref` instead of `.build_dangling()`
```

You can use the `.build_dangling()` method on builders to convert back to the
raw inner struct. Wherever possible, its use should be avoided because it gets
rid of the associated lifetime, allowing for memory safety hazards. Every
builder implements `Deref` and `DerefMut` to its inner struct, which is the
preferred method of accessing it. Only call `.build_dangling()` as late as
possible when you need ownership access.

## Loaders

The various loader structs are _dispatch tables_, storing function pointers and
providing the high level wrappers as associated methods. Note that those should
be stored in a heap pointer such as `Box` or `Arc` because of their big size.

There are three stages of Vulkan functions

- Entry level functions, stored in `EntryLoader`, internally loaded from the
  system's dynamic library for Vulkan.
- Instance level functions, stored in `InstanceLoader`, internally loaded using
  the previously obtained `vkGetInstanceProcAddr` function.
- Device level functions, stored in `DeviceLoader`, internally loaded using the
  `vkGetDeviceProcAddr` function obtained in the previous step.

### Usual setup

```rust
let entry = EntryLoader::new()?;

let instance_info: vk::InstanceCreateInfoBuilder = /* ... */;
let instance = InstanceLoader::new(
    &entry,
    &instance_info,
    None, // allocation callbacks
)?;

let physical_device: vk::PhysicalDevice = /* ... */;
let device_info: vk::DeviceCreateInfoBuilder = /* ... */;
let device = DeviceLoader::new(
    &instance,
    physical_device,
    &device_info,
    None, // allocation callbacks
)?;

// ...

device.destroy_device(None);
instance.destroy_instance(None);
```

### Dynamic lifetime checking

Loaders perform dynamic lifetime checking, so a `InstanceLoader` cannot live
longer than the `EntryLoader` it was created from and a `DeviceLoader` cannot
live longer than the `InstanceLoader` it was created from. When that invariant
is broken a panic occurs. This might hit you in the following setup, because
Rust structs drop from top to bottom.

```rust
// Broken - `entry` drops before `instance`
struct ExampleVulkanState {
    entry: EntryLoader, // 1st drop
    instance: InstanceLoader, // 2nd drop
}

// Works - `entry` drops after `instance`
struct ExampleVulkanState {
    instance: InstanceLoader, // 1st drop
    entry: EntryLoader, // 2nd drop
}
```

### Advanced loader creation

The `*Enabled` structs associated with each loader are an
implementation detail used while creating the loader.

<sup>TODO: document `::with_creation_fn` and `::custom` (with examples)</sup>

## Pointer chains

The Vulkan API uses the concept of pointer chains to allow future extensibility
of structs. `erupt` provides the `ExtendableFromConst` and `ExtendableFromMut`
traits, which provide type and memory safe pointer chain support through the
`extend_from` methods.

### `ExtendableFromConst`

```rust
let debug_utils_messenger_info = vk::DebugUtilsMessengerCreateInfoEXTBuilder::new()
    .message_severity(/* ... */)
    .message_type(/* ... */)
    .pfn_user_callback(Some(debug_utils_messenger_callback));
let instance_info = vk::InstanceCreateInfoBuilder::new()
    .application_info(/* ... */)
    .enabled_layer_names(/* ... */)
    .enabled_extension_names(/* ... */)
    // instance creation now has the ability to send debug messages
    .extend_from(&debug_utils_messenger_info);

let instance = InstanceLoader::new(&entry, &instance_info, None)?;
```

### `ExtenableFromMut`

```rust
let mut line_rasterization_features = vk::PhysicalDeviceLineRasterizationFeaturesEXT::default();
let mut shader_clock_features = vk::PhysicalDeviceShaderClockFeaturesKHR::default();
let features_builder = vk::PhysicalDeviceFeatures2Builder::new()
    .extend_from(&mut line_rasterization_features)
    .extend_from(&mut shader_clock_features);

let features = instance.get_physical_device_features2(
    physical_device,
    Some(features_builder.build_dangling()),
);

// `features`, `line_rasterization_features` and `shader_clock_features`
// are now filled with the correct values
```

## `Flags` and `FlagBits`

In Vulkan, there a `FlagBits` and corresponding `Flags`, which are a bitmask of
the `FlagBits`. `FlagBits` are enums which only store a single flag and `Flags`
combine those into storing multiple flags. You can convert from the
`FlagBits` to the `Flags` variant using the associated `.bitmask()` method.
