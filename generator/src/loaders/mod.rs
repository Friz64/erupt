mod value;
mod wrappers;

use super::comment_gen::DocCommentGen;
use crate::{
    declaration::Type,
    functions::Function,
    name::{Name, TypeName},
    origin::Origin,
    source::Source,
};
use indexmap::{IndexMap, IndexSet};
use proc_macro2::{Ident, Literal, TokenStream};
use quote::{format_ident, quote};
use std::{
    collections::HashMap,
    fmt::{self, Display},
};

#[derive(Clone, PartialEq, Eq, Hash)]
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

#[derive(Default)]
struct LoaderData {
    idents: Vec<Ident>,
    types: Vec<TokenStream>,
    loading: Vec<TokenStream>,
    requirement_origins: IndexSet<Origin>,
}

impl LoaderData {
    fn push(&mut self, origin: &Origin, function: &Function) {
        self.idents.push(function.name.pretty_ident());

        let pointer_ident = function.name.pointer_ident();
        let path = origin.module_path();
        self.types.push(quote! { Option<#path#pointer_ident> });

        let loading = {
            let module_path = origin.module_path();
            let constant_ident = format_ident!("{}", function.name.constant_name());
            let load = quote! {
                unsafe {
                    match symbol(crate::#module_path#constant_ident) {
                        Some(ptr) => Some(std::mem::transmute(ptr)),
                        None => return Err(crate::LoaderError::SymbolNotAvailable),
                    }
                }
            };

            let possibilities: Vec<_> = function
                .requirements
                .iter()
                .filter_map(|requirement| {
                    match (&requirement.base_origin, &requirement.require_origin) {
                        (Origin::Feature { major: 1, minor: 0 }, _) => None,
                        (base_origin, Some(require_origin)) => {
                            let base_ident = base_origin.ident();
                            self.requirement_origins.insert(base_origin.clone());

                            let require_ident = require_origin.ident();
                            self.requirement_origins.insert(require_origin.clone());

                            Some(quote! { (#base_ident && #require_ident) })
                        }
                        (base_origin, None) => {
                            let base_ident = base_origin.ident();
                            self.requirement_origins.insert(base_origin.clone());

                            Some(quote! { #base_ident })
                        }
                    }
                })
                .collect();

            if possibilities.is_empty() {
                load
            } else {
                quote! {
                    if #(#possibilities) ||* {
                        #load
                    } else {
                        None
                    }
                }
            }
        };

        self.loading.push(loading);
    }

    fn booleans(&self, source: &Source) -> TokenStream {
        let booleans = self.requirement_origins.iter().map(|origin| {
            let ident = origin.ident();
            let expr = match origin {
                Origin::Feature { major, minor } => {
                    let major = Literal::u32_unsuffixed(*major);
                    let minor = Literal::u32_unsuffixed(*minor);

                    quote! { version >= crate::vk1_0::make_version(#major, #minor, 0) }
                }
                Origin::Extension { .. } => {
                    let module_path = origin.module_path();
                    let name_constant = source.constants.iter().find(|constant| {
                        constant.trimmed_name.ends_with("_EXTENSION_NAME")
                            && constant.origin.as_ref() == Some(origin)
                    });

                    let ident = match name_constant {
                        Some(constant) => constant.ident(),
                        None => panic!("Can't find name constant for {:?}", origin),
                    };

                    quote! {
                        unsafe {
                            crate::c_str_array_contains(
                                extensions,
                                extensions_len,
                                crate::#module_path#ident
                            )
                        }
                    }
                }
                invalid => panic!("Invalid origin: {:?}", invalid),
            };

            quote! { let #ident = #expr; }
        });

        quote! { #(#booleans)* }
    }
}

pub(super) fn tokens(comment_gen: &DocCommentGen, source: &Source) -> HashMap<Origin, TokenStream> {
    let mut wrappers = IndexMap::new();

    let mut entry_data = LoaderData::default();
    let mut instance_data = LoaderData::default();
    let mut device_data = LoaderData::default();
    for function in source.all_functions_emulated() {
        let origin = function.origin.as_ref().expect("Function has no origin");

        let command_level = function.command_level();
        wrappers
            .entry((origin.clone(), command_level.clone()))
            .or_insert_with(Vec::new)
            .push(function.wrapper(&command_level, comment_gen, source));

        match function.command_level() {
            CommandLevel::Entry => entry_data.push(origin, &function),
            CommandLevel::Instance => instance_data.push(origin, &function),
            CommandLevel::Device => device_data.push(origin, &function),
        }
    }

    let entry_idents = &entry_data.idents;
    let entry_types = &entry_data.types;
    let entry_loading = &entry_data.loading;
    let entry_booleans = entry_data.booleans(source);

    let instance_idents = &instance_data.idents;
    let instance_types = &instance_data.types;
    let instance_loading = &instance_data.loading;
    let instance_booleans = instance_data.booleans(source);

    let device_idents = &device_data.idents;
    let device_types = &device_data.types;
    let device_loading = &device_data.loading;
    let device_booleans = device_data.booleans(source);

    let mut map = HashMap::new();
    for ((origin, command_level), wrappers) in wrappers {
        let generics = command_level.generics();
        let loader = command_level.loader();
        let doc = comment_gen.provided_by(&Origin::Root, &origin);

        map.entry(origin)
            .or_insert_with(TokenStream::new)
            .extend(quote! {
                #[doc = #doc]
                impl#generics #loader {
                    #(#wrappers)*
                }
            })
    }

    map.insert(Origin::Root, quote! {
        /// Loader for entry commands
        ///
        /// To create a new loader, call [`new`](#method.new).
        pub struct EntryLoader<T> {
            arc: std::sync::Arc<()>,
            pub loader: T,
            pub get_instance_proc_addr: crate::vk1_0::PFN_vkGetInstanceProcAddr,
            pub instance_version: u32,
            #(pub #entry_idents: #entry_types,)*
        }

        impl<T> EntryLoader<T> {
            #[inline]
            pub fn custom(
                mut loader: T,
                mut symbol: impl FnMut(&mut T, *const std::os::raw::c_char)
                    -> Option<crate::vk1_0::PFN_vkVoidFunction>,
            ) -> Result<EntryLoader<T>, crate::LoaderError> {
                let mut symbol = |name| symbol(&mut loader, name);

                let mut version = crate::vk1_0::make_version(1, 0, 0);
                if let Some(function) = symbol(crate::vk1_1::FN_ENUMERATE_INSTANCE_VERSION) {
                    let function: crate::vk1_1::PFN_vkEnumerateInstanceVersion =
                        unsafe { std::mem::transmute(function) };

                    let result = unsafe { function(&mut version) };
                    if result.0 < 0 {
                        return Err(crate::LoaderError::VulkanError(result));
                    }
                }

                #entry_booleans

                let get_instance_proc_addr = symbol(crate::vk1_0::FN_GET_INSTANCE_PROC_ADDR)
                    .ok_or(crate::LoaderError::SymbolNotAvailable)?;
                Ok(EntryLoader {
                    arc: std::sync::Arc::new(()),
                    get_instance_proc_addr: unsafe { std::mem::transmute(get_instance_proc_addr) },
                    instance_version: version,
                    #(#entry_idents: #entry_loading,)*
                    loader,
                })
            }
        }

        impl<T> Drop for EntryLoader<T> {
            fn drop(&mut self) {
                if std::sync::Arc::weak_count(&self.arc) != 0 {
                    panic!("Attempting to drop a entry loader with active references to it");
                }
            }
        }

        impl<T> std::fmt::Debug for EntryLoader<T> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "Entry")
            }
        }

        /// Loader for instance commands
        ///
        /// To create a new loader, call [`new`](#method.new).
        pub struct InstanceLoader {
            #[allow(dead_code)]
            parent: std::sync::Weak<()>,
            arc: std::sync::Arc<()>,
            selected_instance_version: u32,
            pub get_device_proc_addr: crate::vk1_0::PFN_vkGetDeviceProcAddr,
            pub handle: crate::vk1_0::Instance,
            #(pub #instance_idents: #instance_types,)*
        }

        impl InstanceLoader {
            #[inline]
            pub fn custom<T>(
                entry_loader: &EntryLoader<T>,
                instance: crate::vk1_0::Instance,
                version: u32,
                extensions_len: usize,
                extensions: *const *const std::os::raw::c_char,
                mut symbol: impl FnMut(*const std::os::raw::c_char)
                    -> Option<crate::vk1_0::PFN_vkVoidFunction>,
            ) -> Result<InstanceLoader, crate::LoaderError> {
                #instance_booleans

                let get_device_proc_addr = symbol(crate::vk1_0::FN_GET_DEVICE_PROC_ADDR)
                    .ok_or(crate::LoaderError::SymbolNotAvailable)?;
                Ok(InstanceLoader {
                    parent: std::sync::Arc::downgrade(&entry_loader.arc),
                    arc: std::sync::Arc::new(()),
                    selected_instance_version: version,
                    get_device_proc_addr: unsafe { std::mem::transmute(get_device_proc_addr) },
                    handle: instance,
                    #(#instance_idents: #instance_loading,)*
                })
            }
        }

        impl Drop for InstanceLoader {
            fn drop(&mut self) {
                if std::sync::Arc::weak_count(&self.arc) != 0 {
                    panic!("Attempting to drop a instance loader with active references to it");
                }
            }
        }

        impl std::fmt::Debug for InstanceLoader {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                std::fmt::Debug::fmt(&self.handle, f)
            }
        }

        /// Loader for device commands
        ///
        /// To create a new loader, call [`new`](#method.new).
        pub struct DeviceLoader {
            #[allow(dead_code)]
            parent: std::sync::Weak<()>,
            pub handle: crate::vk1_0::Device,
            #(pub #device_idents: #device_types,)*
        }

        impl DeviceLoader {
            #[inline]
            pub fn custom(
                instance_loader: &InstanceLoader,
                device: crate::vk1_0::Device,
                extensions_len: usize,
                extensions: *const *const std::os::raw::c_char,
                mut symbol: impl FnMut(*const std::os::raw::c_char)
                    -> Option<crate::vk1_0::PFN_vkVoidFunction>,
            ) -> Result<DeviceLoader, crate::LoaderError> {
                let version = instance_loader.selected_instance_version;
                #device_booleans

                Ok(DeviceLoader {
                    parent: std::sync::Arc::downgrade(&instance_loader.arc),
                    handle: device,
                    #(#device_idents: #device_loading,)*
                })
            }
        }

        impl std::fmt::Debug for DeviceLoader {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                std::fmt::Debug::fmt(&self.handle, f)
            }
        }
    });

    map
}
