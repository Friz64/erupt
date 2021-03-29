mod value;
mod wrappers;

use crate::{
    comment_gen::DocCommentGen,
    declaration::Type,
    items::functions::{ExtensionType, Function, Requirement},
    name::{Name, TypeName},
    origin::Origin,
    source::Source,
};
use indexmap::IndexMap;
use proc_macro2::{Ident, Literal, TokenStream};
use quote::{format_ident, quote};
use std::{
    collections::HashMap,
    fmt::{self, Display},
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum CommandLevel {
    Entry,
    Instance,
    Device,
}

impl CommandLevel {
    fn generics(&self) -> TokenStream {
        match self {
            CommandLevel::Entry => quote! { <T> },
            _ => TokenStream::new(),
        }
    }

    fn loader(&self) -> TokenStream {
        let ident = format_ident!("{}Loader", self.to_string());
        let generics = self.generics();

        quote! { crate::#ident#generics }
    }

    fn handle_type(&self) -> Option<Type> {
        match self {
            CommandLevel::Entry => None,
            CommandLevel::Instance => Some(Type::Named(Name::Type(TypeName::instance()))),
            CommandLevel::Device => Some(Type::Named(Name::Type(TypeName::device()))),
        }
    }
}

impl Display for CommandLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            CommandLevel::Entry => "Entry",
            CommandLevel::Instance => "Instance",
            CommandLevel::Device => "Device",
        })
    }
}

impl PartialEq<ExtensionType> for CommandLevel {
    fn eq(&self, other: &ExtensionType) -> bool {
        matches!(
            (self, other),
            (CommandLevel::Instance, ExtensionType::Instance)
                | (CommandLevel::Device, ExtensionType::Device)
        )
    }
}

impl Function {
    fn command_level(&self) -> CommandLevel {
        let device_types = [
            TypeName::device(),
            TypeName::command_buffer(),
            TypeName::queue(),
        ];

        let instance_types = [TypeName::instance(), TypeName::physical_device()];

        match self.parameters.get(0).map(|param| &param.ty) {
            Some(Type::Named(Name::Type(type_name))) => {
                if device_types.contains(type_name) {
                    CommandLevel::Device
                } else if instance_types.contains(type_name) {
                    CommandLevel::Instance
                } else {
                    CommandLevel::Entry
                }
            }
            _ => CommandLevel::Entry,
        }
    }
}

#[derive(Debug)]
enum EnabledKind {
    Normal,
    // https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetInstanceProcAddr.html#_description
    InstanceAvailableDeviceExtension,
}

#[derive(Default)]
struct EnabledData {
    target_command_level: Option<CommandLevel>,
    origins: IndexMap<Origin, EnabledKind>,
}

impl EnabledData {
    fn new(target_command_level: CommandLevel) -> EnabledData {
        EnabledData {
            target_command_level: Some(target_command_level),
            ..Default::default()
        }
    }

    fn push(&mut self, origin: &Origin, function: &Function) {
        if origin.is_vk1_0() {
            return;
        }

        let command_level = self.target_command_level.unwrap();
        if command_level == function.command_level() && command_level == CommandLevel::Instance {
            let iterator = function
                .requirements
                .iter()
                .flat_map(|requirement| match &requirement.require_origin {
                    Some(require_origin) => vec![&requirement.base_origin, require_origin],
                    None => vec![&requirement.base_origin],
                })
                .map(|origin| {
                    let origin = origin.clone();
                    (origin, EnabledKind::InstanceAvailableDeviceExtension)
                });

            for (key, value) in iterator {
                if !self.origins.contains_key(&key) {
                    self.origins.insert(key, value);
                }
            }
        }

        let insert = if origin.is_extension() {
            &command_level == function.extension_type.as_ref().unwrap()
        } else {
            command_level != CommandLevel::Device
        };

        if insert {
            // Replaces the value if the key already exists
            self.origins.insert(origin.clone(), EnabledKind::Normal);
        }
    }

    fn idents(&self) -> impl Iterator<Item = Ident> + '_ {
        self.origins.keys().map(|origin| origin.ident())
    }

    fn values<'a>(&'a self, source: &'a Source) -> impl Iterator<Item = TokenStream> + 'a {
        self.origins.iter().map(move |origin| match origin {
            (Origin::Feature { major, minor }, EnabledKind::Normal) => {
                let major = Literal::u32_unsuffixed(*major);
                let minor = Literal::u32_unsuffixed(*minor);

                quote! { version >= crate::vk1_0::make_version(#major, #minor, 0) }
            }
            (origin @ Origin::Extension { .. }, enabled_kind) => {
                let module_path = origin.module_path();
                let name_constant = source.constants.iter().find(|constant| {
                    constant.trimmed_name.ends_with("_EXTENSION_NAME")
                        && constant.origin.as_ref() == Some(origin)
                });

                let ident = match name_constant {
                    Some(constant) => constant.ident(),
                    None => panic!("Can't find name constant for {:?}", origin),
                };

                match enabled_kind {
                    EnabledKind::Normal => quote! {
                        enabled_extension(crate::#module_path#ident)
                    },
                    EnabledKind::InstanceAvailableDeviceExtension => quote! {
                        available_device_extension(crate::#module_path#ident)
                    },
                }
            }
            invalid => panic!("Invalid origin: {:?}", invalid),
        })
    }
}

#[derive(Default)]
struct LoaderData {
    target_command_level: Option<CommandLevel>,
    idents: Vec<Ident>,
    types: Vec<TokenStream>,
    loads: Vec<TokenStream>,
    requirements: Vec<Vec<Requirement>>,
}

impl LoaderData {
    fn new(target_command_level: CommandLevel) -> LoaderData {
        LoaderData {
            target_command_level: Some(target_command_level),
            ..Default::default()
        }
    }

    fn push(&mut self, origin: &Origin, function: &Function) {
        let command_level = self.target_command_level.unwrap();
        if command_level != function.command_level() {
            return;
        }

        self.idents.push(function.name.pretty_ident());

        let pointer_ident = function.name.pointer_ident();
        let path = origin.module_path();
        self.types.push(quote! { Option<#path#pointer_ident> });

        let module_path = origin.module_path();
        let constant_ident = format_ident!("{}", function.name.constant_name());
        self.loads.push(quote! {
            std::mem::transmute(symbol(crate::#module_path#constant_ident))
        });

        self.requirements.push(function.requirements.clone());
    }

    fn loading<'a>(
        &'a self,
        device_enabled: &'a EnabledData,
    ) -> impl Iterator<Item = TokenStream> + 'a {
        let command_level = self.target_command_level.unwrap();
        let enabled_ident = move |origin: &Origin| match command_level {
            CommandLevel::Entry => quote! { entry_enabled },
            CommandLevel::Instance => quote! { instance_enabled },
            CommandLevel::Device => {
                if device_enabled.origins.contains_key(origin) {
                    quote! { device_enabled }
                } else {
                    quote! { instance_enabled }
                }
            }
        };

        self.requirements
            .iter()
            .zip(self.loads.iter())
            .map(move |(requirements, load)| {
                let possibilities: Vec<_> = requirements
                    .iter()
                    .filter_map(|requirement| {
                        match (&requirement.base_origin, &requirement.require_origin) {
                            (base_origin, _) if base_origin.is_vk1_0() => None,
                            (base_origin, Some(require_origin)) => {
                                let base_enabled = enabled_ident(base_origin);
                                let base_ident = base_origin.ident();

                                let require_enabled = enabled_ident(require_origin);
                                let require_ident = require_origin.ident();

                                Some(quote! {
                                    (#base_enabled.#base_ident
                                        && #require_enabled.#require_ident)
                                })
                            }
                            (base_origin, None) => {
                                let base_enabled = enabled_ident(base_origin);
                                let base_ident = base_origin.ident();

                                Some(quote! { #base_enabled.#base_ident })
                            }
                        }
                    })
                    .collect();

                if possibilities.is_empty() {
                    load.clone()
                } else {
                    quote! {
                        if #(#possibilities) ||* {
                            #load
                        } else {
                            None
                        }
                    }
                }
            })
    }
}

pub(super) fn tokens(comment_gen: &DocCommentGen, source: &Source) -> HashMap<Origin, TokenStream> {
    let mut wrappers = IndexMap::new();

    let mut entry_enabled_data = EnabledData::new(CommandLevel::Entry);
    let mut instance_enabled_data = EnabledData::new(CommandLevel::Instance);
    let mut device_enabled_data = EnabledData::new(CommandLevel::Device);

    let mut entry_loader_data = LoaderData::new(CommandLevel::Entry);
    let mut instance_loader_data = LoaderData::new(CommandLevel::Instance);
    let mut device_loader_data = LoaderData::new(CommandLevel::Device);

    for function in source.all_functions_emulated() {
        let origin = function.origin.as_ref().expect("Function has no origin");
        let command_level = function.command_level();
        wrappers
            .entry((origin.clone(), command_level))
            .or_insert_with(Vec::new)
            .push(function.wrapper(&command_level, comment_gen, source));

        entry_enabled_data.push(origin, &function);
        instance_enabled_data.push(origin, &function);
        device_enabled_data.push(origin, &function);

        entry_loader_data.push(origin, &function);
        instance_loader_data.push(origin, &function);
        device_loader_data.push(origin, &function);
    }

    let entry_enabled_idents: Vec<_> = entry_enabled_data.idents().collect();
    let entry_enabled_values = entry_enabled_data.values(source);

    let instance_enabled_idents: Vec<_> = instance_enabled_data.idents().collect();
    let instance_enabled_values = instance_enabled_data.values(source);

    let device_enabled_idents: Vec<_> = device_enabled_data.idents().collect();
    let device_enabled_values = device_enabled_data.values(source);

    let entry_loader_idents = &entry_loader_data.idents;
    let entry_loader_types = &entry_loader_data.types;
    let entry_loader_loading = entry_loader_data.loading(&device_enabled_data);

    let instance_loader_idents = &instance_loader_data.idents;
    let instance_loader_types = &instance_loader_data.types;
    let instance_loader_loading = instance_loader_data.loading(&device_enabled_data);

    let device_loader_idents = &device_loader_data.idents;
    let device_loader_types = &device_loader_data.types;
    let device_loader_loading = device_loader_data.loading(&device_enabled_data);

    let mut map = HashMap::new();
    for ((origin, command_level), wrappers) in wrappers {
        let generics = command_level.generics();
        let loader = command_level.loader();
        let doc = comment_gen.provided_by(&origin);

        map.entry(origin)
            .or_insert_with(TokenStream::new)
            .extend(quote! {
                #[doc = #doc]
                impl#generics #loader {
                    #(#wrappers)*
                }
            })
    }

    let code = quote! {
        /// A list of requirements enabled in the entry loader
        #[derive(Debug)]
        pub struct EntryEnabled {
            pub instance_version: u32,
            #(pub #entry_enabled_idents: bool,)*
        }

        impl EntryEnabled {
            pub unsafe fn new<T>(
                loader: &mut T,
                mut symbol: impl FnMut(&mut T, *const std::os::raw::c_char)
                    -> Option<crate::vk1_0::PFN_vkVoidFunction>,
            ) -> Result<EntryEnabled, crate::LoaderError> {
                let mut version = crate::vk1_0::make_version(1, 0, 0);
                if let Some(function) = symbol(loader, crate::vk1_1::FN_ENUMERATE_INSTANCE_VERSION) {
                    let function: crate::vk1_1::PFN_vkEnumerateInstanceVersion =
                        std::mem::transmute(function);

                    let result = function(&mut version);
                    if result.0 < 0 {
                        return Err(crate::LoaderError::VulkanError(result));
                    }
                }

                Ok(EntryEnabled {
                    instance_version: version,
                    #(#entry_enabled_idents: #entry_enabled_values,)*
                })
            }
        }

        /// Loader for entry commands
        ///
        /// To create a new loader, call [`EntryLoader::new`].
        pub struct EntryLoader<T> {
            arc: std::sync::Arc<()>,
            pub loader: T,
            enabled: EntryEnabled,
            pub get_instance_proc_addr: crate::vk1_0::PFN_vkGetInstanceProcAddr,
            #(pub #entry_loader_idents: #entry_loader_types,)*
        }

        impl<T> EntryLoader<T> {
            #[allow(unused_parens)]
            pub unsafe fn custom(
                mut loader: T,
                mut symbol: impl FnMut(&mut T, *const std::os::raw::c_char)
                    -> Option<crate::vk1_0::PFN_vkVoidFunction>,
                entry_enabled: EntryEnabled,
            ) -> Result<EntryLoader<T>, crate::LoaderError> {
                let mut symbol = |name| symbol(&mut loader, name);

                let get_instance_proc_addr = symbol(crate::vk1_0::FN_GET_INSTANCE_PROC_ADDR)
                    .ok_or(crate::LoaderError::SymbolNotAvailable)?;
                Ok(EntryLoader {
                    arc: std::sync::Arc::new(()),
                    get_instance_proc_addr: std::mem::transmute(get_instance_proc_addr),
                    #(#entry_loader_idents: #entry_loader_loading,)*
                    loader,
                    enabled: entry_enabled,
                })
            }

            pub fn enabled(&self) -> &EntryEnabled {
                &self.enabled
            }

            pub fn instance_version(&self) -> u32 {
                self.enabled.instance_version
            }
        }

        impl<T> Drop for EntryLoader<T> {
            fn drop(&mut self) {
                if std::sync::Arc::weak_count(&self.arc) != 0 {
                    panic!("Attempting to drop a entry loader with active references to it");
                }
            }
        }

        /// A list of requirements enabled in the instance loader
        #[derive(Debug)]
        pub struct InstanceEnabled {
            #(pub #instance_enabled_idents: bool,)*
        }

        impl InstanceEnabled {
            pub unsafe fn new(
                instance_version: u32,
                enabled_extensions: &[&std::ffi::CStr],
                available_device_extensions: &[&std::ffi::CStr],
            ) -> Result<InstanceEnabled, crate::LoaderError> {
                let version = instance_version;
                let enabled_extension = |extension|
                    enabled_extensions.contains(&std::ffi::CStr::from_ptr(extension));
                let available_device_extension = |extension|
                    available_device_extensions.contains(&std::ffi::CStr::from_ptr(extension));

                Ok(InstanceEnabled {
                    #(#instance_enabled_idents: #instance_enabled_values,)*
                })
            }
        }

        /// Loader for instance commands
        ///
        /// To create a new loader, call [`InstanceLoader::new`].
        pub struct InstanceLoader {
            #[allow(dead_code)]
            parent: std::sync::Weak<()>,
            arc: std::sync::Arc<()>,
            pub handle: crate::vk1_0::Instance,
            enabled: InstanceEnabled,
            pub get_device_proc_addr: crate::vk1_0::PFN_vkGetDeviceProcAddr,
            #(pub #instance_loader_idents: #instance_loader_types,)*
        }

        impl InstanceLoader {
            #[allow(unused_parens)]
            pub unsafe fn custom<T>(
                entry_loader: &EntryLoader<T>,
                instance: crate::vk1_0::Instance,
                instance_enabled: InstanceEnabled,
                mut symbol: impl FnMut(*const std::os::raw::c_char)
                    -> Option<crate::vk1_0::PFN_vkVoidFunction>,
            ) -> Result<InstanceLoader, crate::LoaderError> {
                let get_device_proc_addr = symbol(crate::vk1_0::FN_GET_DEVICE_PROC_ADDR)
                    .ok_or(crate::LoaderError::SymbolNotAvailable)?;

                Ok(InstanceLoader {
                    parent: std::sync::Arc::downgrade(&entry_loader.arc),
                    arc: std::sync::Arc::new(()),
                    handle: instance,
                    get_device_proc_addr: std::mem::transmute(get_device_proc_addr),
                    #(#instance_loader_idents: #instance_loader_loading,)*
                    enabled: instance_enabled,
                })
            }

            pub fn enabled(&self) -> &InstanceEnabled {
                &self.enabled
            }
        }

        impl Drop for InstanceLoader {
            fn drop(&mut self) {
                if std::sync::Arc::weak_count(&self.arc) != 0 {
                    panic!("Attempting to drop a instance loader with active references to it");
                }
            }
        }

        /// A list of requirements enabled in the device loader
        #[derive(Debug)]
        pub struct DeviceEnabled {
            #(pub #device_enabled_idents: bool,)*
        }

        impl DeviceEnabled {
            pub unsafe fn new(enabled_extensions: &[&std::ffi::CStr]) -> DeviceEnabled {
                let enabled_extension = |extension|
                    enabled_extensions.contains(&std::ffi::CStr::from_ptr(extension));

                DeviceEnabled {
                    #(#device_enabled_idents: #device_enabled_values,)*
                }
            }
        }

        /// Loader for device commands
        ///
        /// To create a new loader, call [`DeviceLoader::new`].
        pub struct DeviceLoader {
            #[allow(dead_code)]
            parent: std::sync::Weak<()>,
            pub handle: crate::vk1_0::Device,
            enabled: DeviceEnabled,
            #(pub #device_loader_idents: #device_loader_types,)*
        }

        impl DeviceLoader {
            #[allow(unused_parens)]
            pub unsafe fn custom(
                instance_loader: &InstanceLoader,
                device: crate::vk1_0::Device,
                device_enabled: DeviceEnabled,
                mut symbol: impl FnMut(*const std::os::raw::c_char)
                    -> Option<crate::vk1_0::PFN_vkVoidFunction>,
            ) -> Result<DeviceLoader, crate::LoaderError> {
                let instance_enabled = &instance_loader.enabled;

                Ok(DeviceLoader {
                    parent: std::sync::Arc::downgrade(&instance_loader.arc),
                    handle: device,
                    #(#device_loader_idents: #device_loader_loading,)*
                    enabled: device_enabled,
                })
            }

            pub fn enabled(&self) -> &DeviceEnabled {
                &self.enabled
            }
        }
    };

    map.insert(Origin::Root, code);
    map
}
