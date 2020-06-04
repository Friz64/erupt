use crate::{types::TypeItem, utils, Origin};
use heck::{CamelCase, SnakeCase};
use itertools::Itertools;
use lazy_static::lazy_static;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use regex::Regex;
use std::{collections::HashMap, iter};
use syn::Ident;
use vk_parse::{Command, ExtensionChild, InterfaceItem};

lazy_static! {
    static ref COMMAND_REGEX: Regex = Regex::new(r"(\w+)\s+(\w+) \(").unwrap();
}

#[derive(Debug, PartialEq, Eq)]
pub struct CommandItem {
    pub name: String,
    pub aliased_name: String,
    pub code: String,
    pub param_properties: Vec<(Option<String>, Option<String>)>,
    // this is used in `types::collect`
    pub associated_types: Vec<String>,
}

pub fn collect(children: &[ExtensionChild], raw_commands: &[&Command]) -> Vec<CommandItem> {
    let mut commands: Vec<_> = children
        .iter()
        .filter_map(|child| {
            if let ExtensionChild::Require { items, .. } = child {
                let commands = items.iter().filter_map(|item| {
                    if let InterfaceItem::Command { name: cmd_name, .. } = item {
                        let mut search = cmd_name.clone();
                        let cmd_def = 'outer: loop {
                            for command in raw_commands {
                                match command {
                                    Command::Alias { name, alias } if name == &search => {
                                        search = alias.clone();
                                        continue 'outer;
                                    }
                                    Command::Definition(def) if &def.proto.name == &search => {
                                        break 'outer Some(def);
                                    }
                                    _ => (),
                                }
                            }

                            break None;
                        }
                        .unwrap();

                        let associated_types = cmd_def
                            .params
                            .iter()
                            .map(|param| param.definition.type_name.clone().unwrap())
                            .chain(iter::once(cmd_def.proto.type_name.clone().unwrap()))
                            .collect();

                        let param_properties = cmd_def
                            .params
                            .iter()
                            .map(|param| {
                                let len = utils::len(&param.len, &param.altlen).into_iter().next();
                                let optional = param.optional.clone();

                                (len, optional)
                            })
                            .collect();

                        Some(CommandItem {
                            name: cmd_name.into(),
                            aliased_name: search.into(),
                            code: cmd_def.code.clone(),
                            param_properties,
                            associated_types,
                        })
                    } else {
                        None
                    }
                });

                Some(commands)
            } else {
                None
            }
        })
        .flatten()
        .collect();
    commands.dedup();
    commands
}

#[derive(PartialEq, Eq)]
pub enum CommandLevel {
    Core,
    Instance,
    Device,
}

impl CommandLevel {
    fn from_first_arg_type(arg_type: Option<&str>) -> CommandLevel {
        if let Some(arg_type) = arg_type {
            match arg_type {
                "crate::vk1_0::Instance" | "crate::vk1_0::PhysicalDevice" => CommandLevel::Instance,
                "crate::vk1_0::Device" | "crate::vk1_0::CommandBuffer" | "crate::vk1_0::Queue" => {
                    CommandLevel::Device
                }
                _ => CommandLevel::Core,
            }
        } else {
            CommandLevel::Core
        }
    }

    fn name(&self) -> &str {
        match self {
            CommandLevel::Core => "Core",
            CommandLevel::Instance => "Instance",
            CommandLevel::Device => "Device",
        }
    }
}

pub struct Argument {
    raw_name: String,
    name: Ident,
    ty: String,
    c_type: String,
    dont_touch: bool,
    length: Option<String>,
    optional: Option<String>,
}

pub struct LoaderCommand {
    raw_name: String,
    pfn_name: Ident,
    arguments: Vec<Argument>,
    return_type: TokenStream,
    return_type_raw: String,
    command_level: CommandLevel,
    doc: String,
}

pub fn generate(
    vulkan_version: &str,
    command: &CommandItem,
    itemname_origin_map: &HashMap<String, Origin>,
    loader_commands: &mut Vec<LoaderCommand>,
) -> TokenStream {
    let captures = COMMAND_REGEX.captures(&command.code).expect(&command.code);
    let return_type_raw = &captures[1];
    let regex_name = &captures[2];
    assert_eq!(regex_name, command.aliased_name);

    let return_type: TokenStream =
        syn::parse_str(&utils::item_path(itemname_origin_map, return_type_raw)).unwrap();
    let name = utils::safe_ident(&format!("PFN_{}", &command.name));
    let arguments_raw: Vec<_> = command.code[captures.get(0).unwrap().end()..]
        .split(',')
        .map(|argument| {
            let parsed = utils::parse_c_decl(argument).unwrap();
            let converted = utils::convert_c_decl(itemname_origin_map, &parsed);
            (converted, parsed.1)
        })
        .collect();

    let command_level = CommandLevel::from_first_arg_type(
        arguments_raw
            .get(0)
            .map(|((_, _, arg_type, _, _), _)| arg_type.as_str()),
    );

    let doc = utils::doc(
        &format!("{} Command", command_level.name()),
        Some(&utils::man_doc(vulkan_version, &command.name)),
    );

    let code_arguments = arguments_raw.iter().map(|((_, _, _, _, code), _)| code);

    let code = quote! {
        #[doc = #doc]
        #[allow(non_camel_case_types)]
        pub type #name = unsafe extern "system" fn(#( #code_arguments )*) -> #return_type;
    };

    let arguments: Vec<_> = arguments_raw
        .into_iter()
        .map(|(arg, c_type)| {
            let dont_touch = arg.2.contains("std::ffi::c_void");
            (arg, dont_touch, c_type)
        })
        .zip(command.param_properties.clone())
        .collect();
    let arguments_cleaned = arguments
        .iter()
        .cloned()
        .map(
            |(((raw_name, name, ty, _, _), mut dont_touch, c_type), (length, optional))| {
                dont_touch = dont_touch
                    || arguments
                        .iter()
                        .find_map(|(((_, _, _, _, _), inner_dont_touch, _), (length, _))| {
                            match length {
                                Some(len) if len == &raw_name => Some(inner_dont_touch),
                                _ => None,
                            }
                        })
                        .cloned()
                        .unwrap_or(false);

                Argument {
                    raw_name,
                    name,
                    ty,
                    c_type,
                    dont_touch,
                    length,
                    optional,
                }
            },
        )
        .collect();

    loader_commands.push(LoaderCommand {
        raw_name: command.name.clone(),
        pfn_name: name,
        arguments: arguments_cleaned,
        return_type,
        return_type_raw: return_type_raw.into(),
        command_level,
        doc,
    });

    code
}

pub fn generate_loaders(
    origin: &Origin,
    commands: &[LoaderCommand],
    all_types: &[&TypeItem],
) -> (Vec<String>, TokenStream) {
    let mut loaders = Vec::new();
    let core_loader = generate_loader_generic(
        origin,
        CommandLevel::Core,
        commands,
        &mut loaders,
        all_types,
    );
    let instance_loader = generate_loader_generic(
        origin,
        CommandLevel::Instance,
        commands,
        &mut loaders,
        all_types,
    );
    let device_loader = generate_loader_generic(
        origin,
        CommandLevel::Device,
        commands,
        &mut loaders,
        all_types,
    );

    let code = quote! {
        #core_loader
        #instance_loader
        #device_loader
    };

    (loaders, code)
}

pub fn origin_field_name(origin: &Origin) -> Ident {
    utils::safe_ident(&match origin {
        Origin::Feature(major, minor) => format!("vk{}_{}", major, minor),
        Origin::Extension(_name, primary_secondary, _number) => primary_secondary.to_string(),
    })
}

fn generate_loader_generic(
    origin: &Origin,
    level: CommandLevel,
    commands: &[LoaderCommand],
    loaders: &mut Vec<String>,
    all_types: &[&TypeItem],
) -> Option<TokenStream> {
    let loader_commands: Vec<_> = commands
        .iter()
        .filter_map(|command| {
            if command.command_level == level {
                Some(command)
            } else {
                None
            }
        })
        .collect();

    if loader_commands.is_empty() {
        return None;
    }

    let level_name = level.name();
    let generics = if level == CommandLevel::Core {
        quote! { <T> }
    } else {
        quote! {}
    };

    let field_name = origin_field_name(origin);
    let trait_name = utils::safe_ident(&format!(
        "{}{}LoaderExt",
        field_name.to_string().to_camel_case(),
        level_name
    ));
    let struct_doc = format!(
        "Provides {} Commands for [`{trait_name}`](trait.{trait_name}.html)",
        level_name,
        trait_name = trait_name,
    );
    let struct_name_raw = format!(
        "{}{}Commands",
        field_name.to_string().to_camel_case(),
        level_name
    );
    loaders.push(struct_name_raw.clone());
    let struct_name = utils::safe_ident(&struct_name_raw);
    let struct_fields: Vec<_> = loader_commands
        .iter()
        .map(|command| {
            let field_name =
                utils::safe_ident(&utils::trim_vk_prefix(&command.raw_name).to_snake_case());
            let field_type = &command.pfn_name;

            (&command.raw_name, field_name, field_type)
        })
        .collect();
    let struct_fields_code = struct_fields.iter().map(
        |(_raw_name, field_name, field_type)| quote! { pub #field_name: Option<#field_type>, },
    );

    let loader: TokenStream = syn::parse_str(&format!(
        "crate::{}Loader{}",
        level_name,
        generics.to_string()
    ))
    .unwrap();

    let load_fields = struct_fields
        .iter()
        .map(|(raw_name, field_name, _field_type)| {
            quote! {
                #field_name: std::mem::transmute({
                    let symbol = loader.symbol(#raw_name);
                    success |= symbol.is_some();
                    symbol
                }),
            }
        });

    let loader_name = utils::safe_ident(&format!("{}Loader", level_name));
    let functions: Vec<_> = loader_commands
        .iter()
        .zip(struct_fields.iter())
        .map(|(command, (_, adjusted_name, _))| {
            let mut len_arg_map = HashMap::new();
            for (i, arg) in command.arguments.iter().enumerate() {
                if let Some(length) = &arg.length {
                    len_arg_map
                        .entry(length.clone())
                        .or_insert_with(|| Vec::new())
                        .push((arg.name.clone(), i, arg.ty.clone()));
                }
            }

            let mut value_call = None;
            let mut lengths = quote! {};
            let mut return_init = quote! {};
            let mut returns: Vec<(TokenStream, TokenStream)> = Vec::new();

            let arguments: Vec<_> = command
                .arguments
                .iter()
                .enumerate()
                .map(|(i, arg)| {
                    let arg_name = arg.name.to_string();
                    let arg_name_str = &utils::RELEVANT_NAME_REGEX.captures(&arg_name).unwrap()[2];
                    let arg_name = utils::safe_ident(arg_name_str);

                    let mut arg_type = arg.ty.clone();
                    let mut exposed = true;
                    let mut setter = quote! { #arg_name };
                    if !arg.dont_touch {
                        if arg_type.starts_with(utils::CONST_PTR) {
                            arg_type = arg_type.replacen(utils::CONST_PTR, utils::CONST_REF, 1);
                        } else if arg_type.starts_with(utils::MUT_PTR) {
                            arg_type = arg_type.replacen(utils::MUT_PTR, utils::MUT_REF, 1);
                        }

                        if let Some(arg_list) = len_arg_map.get(&arg.raw_name) {
                            if arg.optional.as_ref().map(|s| s.as_str()) == Some("false,true") {
                                arg_type = format!(
                                    "Option<{}>",
                                    arg_type.trim_start_matches(utils::MUT_REF)
                                );
                                setter = quote! { &mut #arg_name };

                                let null_indices: Vec<_> =
                                    arg_list.iter().map(|(_, i, _)| *i).collect();
                                assert!(value_call.is_none());
                                value_call = Some((i, arg_name.clone(), null_indices));
                            } else {
                                let min_len: TokenStream = arg_list
                                    .iter()
                                    .filter(|(_, _, ty)| ty.starts_with(utils::CONST_PTR))
                                    .enumerate()
                                    .map(|(i, (arg_name, _, _))| {
                                        let name_string = arg_name.to_string();
                                        let relevant = utils::safe_ident(
                                            &utils::RELEVANT_NAME_REGEX
                                                .captures(&name_string)
                                                .unwrap()[2],
                                        );

                                        match i {
                                            0 => quote! { #relevant.len() },
                                            _ => quote! { .min(#relevant.len()) },
                                        }
                                    })
                                    .collect();
                                lengths.extend(quote! {
                                    let #arg_name = #min_len as _;
                                });

                                setter = quote! { #arg_name };
                                exposed = false;
                            }
                        } else {
                            match (i, arg_name_str, arg_type.as_str()) {
                                (0, "device", "crate::vk1_0::Device") => {
                                    exposed = false;
                                    setter = quote! { self.handle };
                                }
                                (0, "instance", "crate::vk1_0::Instance") => {
                                    exposed = false;
                                    setter = quote! { self.handle };
                                }
                                (_, "allocator", "&   crate::vk1_0::AllocationCallbacks") => {
                                    arg_type = "Option<&crate::vk1_0::AllocationCallbacks>".into();
                                    setter = quote! {
                                        if let Some(allocator) = allocator {
                                            allocator
                                        } else {
                                            std::ptr::null()
                                        }
                                    };
                                }
                                (_, _, utils::BOOL32) => {
                                    arg_type = "bool".into();
                                    setter = quote! { #arg_name as _ };
                                }
                                (_, _, ty) if ty == "&   std::os::raw::c_char" => {
                                    arg_type = "Option<&std::ffi::CStr>".into();
                                    setter = quote! {
                                        #arg_name.map(|s| s.as_ptr())
                                            .unwrap_or(std::ptr::null())
                                    };
                                }
                                (_, _, ty)
                                    if ty.starts_with(utils::CONST_REF) && arg.length.is_some() =>
                                {
                                    let is_struct = all_types.iter().any(|inner_type| {
                                        if let TypeItem::Structure {
                                            union: false,
                                            name: inner_name,
                                            ..
                                        } = inner_type
                                        {
                                            inner_name == &arg.c_type
                                        } else {
                                            false
                                        }
                                    });

                                    arg_type = format!(
                                        "{}[{}{}]",
                                        utils::CONST_REF,
                                        arg_type.trim_start_matches(utils::CONST_REF),
                                        if is_struct { "Builder" } else { "" }
                                    );

                                    setter = quote! { #arg_name.as_ptr() as _ };
                                }
                                (_, _, ty) if ty.starts_with(utils::MUT_REF) => {
                                    let trimmed = arg_type.trim_start_matches(utils::MUT_REF);
                                    let trimmed_code: TokenStream =
                                        syn::parse_str(&trimmed).unwrap();
                                    if let Some(length) = &arg.length {
                                        let rust_len = length
                                            .split("->")
                                            .enumerate()
                                            .map(|(i, field)| {
                                                let field = field.to_snake_case();
                                                match i {
                                                    0 => utils::RELEVANT_NAME_REGEX
                                                        .captures(&field)
                                                        .unwrap()[2]
                                                        .to_string(),
                                                    _ => field,
                                                }
                                            })
                                            .join(".");
                                        let rust_len: TokenStream =
                                            syn::parse_str(&rust_len).unwrap();

                                        return_init.extend(quote! {
                                            let mut #arg_name = vec![
                                                Default::default();
                                                #rust_len as _
                                            ];
                                        });

                                        returns.push((
                                            quote! { #arg_name },
                                            quote! { Vec<#trimmed_code> },
                                        ));

                                        exposed = false;
                                        setter = quote! { #arg_name.as_mut_ptr() };
                                    } else {
                                        return_init.extend(quote! {
                                            let mut #arg_name = #arg_name
                                                .unwrap_or_else(|| Default::default());
                                        });

                                        let trimmed_type =
                                            arg_type.trim_start_matches(utils::MUT_REF).trim();
                                        returns.push(match trimmed_type {
                                            utils::BOOL32 => {
                                                (quote! { #arg_name != 0 }, quote! { bool })
                                            }
                                            _ => (quote! { #arg_name }, quote! { #trimmed_code }),
                                        });

                                        arg_type = format!("Option<{}>", trimmed_type);

                                        setter = quote! { &mut #arg_name };
                                    }
                                }
                                _ => (),
                            }
                        }
                    }

                    let arg_type: TokenStream = syn::parse_str(&arg_type).unwrap();

                    (exposed, arg_name, arg_type, setter)
                })
                .collect();

            let (returner, return_type) = match returns.as_slice() {
                [] => (quote! { () }, quote! { () }),
                [(name, ty)] => (quote! { #name }, quote! { #ty }),
                multiple => {
                    let (inner_returner, inner_return_type): (Vec<_>, Vec<_>) = multiple
                        .iter()
                        .map(|(name, ty)| (quote! { #name }, quote! { #ty }))
                        .unzip();

                    (
                        quote! { ( #( #inner_returner ),* ) },
                        quote! { ( #( #inner_return_type ),* ) },
                    )
                }
            };

            let (returner, return_type) = match command.return_type_raw.as_str() {
                "void" => (returner, return_type),
                "VkResult" => (
                    quote! { crate::utils::VulkanResult::new(_val, #returner) },
                    quote! { crate::utils::VulkanResult<#return_type> },
                ),
                _ => {
                    let return_type = &command.return_type;
                    (quote! { _val }, quote! { #return_type })
                }
            };

            (
                adjusted_name,
                arguments,
                value_call,
                lengths,
                return_init,
                returner,
                return_type,
                command.doc.clone(),
            )
        })
        .collect();

    let trait_functions = functions
        .iter()
        .map(|(name, args, _, _, _, _, return_type, doc)| {
            let arguments = args
                .iter()
                .filter(|cmd| cmd.0)
                .map(|(_, arg_name, arg_type, _)| quote! { #arg_name: #arg_type, });

            quote! {
                #[doc = #doc]
                unsafe fn #name(&self, #( #arguments )*) -> #return_type;
            }
        });

    let loading_error = format!("`{}` not loaded", field_name);
    let commands_function_name = match level {
        CommandLevel::Core => quote! { core_commands },
        CommandLevel::Instance => quote! { instance_commands },
        CommandLevel::Device => quote! { device_commands },
    };

    let commands_function = quote! {
        #[inline]
        fn #commands_function_name#generics(loader: &#loader) -> &#struct_name {
            loader.#field_name.as_ref().expect(#loading_error)
        }
    };

    let trait_impl_functions = functions.iter().map(
        |(name, args, value_call, lengths, return_init, returner, return_type, doc)| {
            let arguments = args
                .iter()
                .filter(|cmd| cmd.0)
                .map(|(_, arg_name, arg_type, _)| quote! { #arg_name: #arg_type, });
            let arguments_inner = args.iter().map(|(_, _, _, setter)| {
                quote! { #setter, }
            });
            let value_call = if let Some((index, name, null_indices)) = value_call {
                let arguments = args.iter().enumerate().map(|(i, (_, _, _, setter))| {
                    if &i == index {
                        quote! { &mut val, }
                    } else if null_indices.contains(&i) {
                        quote! { std::ptr::null_mut(), }
                    } else {
                        quote! { #setter, }
                    }
                });

                quote! {
                    let mut #name = #name.unwrap_or_else(|| {
                        let mut val = Default::default();
                        function(#( #arguments )*);
                        val
                    });
                }
            } else {
                quote! {}
            };

            let function_loading_error = format!("`{}` not available", name);

            quote! {
                #[inline]
                #[doc = #doc]
                unsafe fn #name(&self, #( #arguments )*) -> #return_type {
                    let function = #commands_function_name(self)
                        .#name
                        .as_ref()
                        .expect(#function_loading_error);

                    #value_call
                    #lengths
                    #return_init
                    let _val = function(#( #arguments_inner )*);
                    #returner
                }
            }
        },
    );

    let trait_doc = format!(
        "Provides high level command wrappers for [`{name}`](struct.{name}.html)",
        name = struct_name_raw
    );

    Some(quote! {
        #[doc = #struct_doc]
        pub struct #struct_name {
            #( #struct_fields_code )*
        }

        impl #struct_name {
            #[inline]
            pub fn load#generics(loader: &#loader) -> Option<#struct_name> {
                unsafe {
                    let mut success = false;
                    let table = #struct_name {
                        #( #load_fields )*
                    };

                    if success {
                        Some(table)
                    } else {
                        None
                    }
                }
            }
        }

        #commands_function

        #[doc = #trait_doc]
        pub trait #trait_name {
            #( #trait_functions )*
        }

        impl#generics #trait_name for crate::#loader_name#generics {
            #( #trait_impl_functions )*
        }
    })
}

pub fn generate_loaders_root(loader_map: &[(&&Origin, Vec<String>)]) -> TokenStream {
    let core_loader = generate_loader_root_generic(CommandLevel::Core, loader_map);
    let instance_loader = generate_loader_root_generic(CommandLevel::Instance, loader_map);
    let device_loader = generate_loader_root_generic(CommandLevel::Device, loader_map);

    quote! {
        #core_loader
        #instance_loader
        #device_loader
    }
}

fn generate_loader_root_generic(
    level: CommandLevel,
    loader_map: &[(&&Origin, Vec<String>)],
) -> TokenStream {
    let level_name = level.name();
    let struct_name = utils::safe_ident(&format!("{}Loader", level_name));
    let struct_fields: Vec<_> = loader_map
        .iter()
        .flat_map(|(origin, loaders)| {
            loaders
                .iter()
                .filter(|loader| loader.ends_with(&format!("{}Commands", level.name())))
                .map(move |loader| {
                    let field_type: TokenStream = syn::parse_str(&format!(
                        "crate::{}::{}",
                        utils::origin_path(&origin),
                        loader
                    ))
                    .unwrap();
                    let field_name = origin_field_name(origin);

                    (origin, field_name, field_type)
                })
        })
        .collect();

    let struct_fields_code = struct_fields
        .iter()
        .map(|(origin, field_name, field_type)| {
            let trait_name = utils::safe_ident(&format!(
                "{}{}LoaderExt",
                field_name.to_string().to_camel_case(),
                level_name
            ));
            let origin_path = utils::origin_path(origin);
            let path = format!(
                "{}/trait.{}.html",
                origin_path.replace("::", "/"),
                trait_name
            );
            let doc = format!(
                "Implemented in [`{}::{}`]({})",
                origin_path, trait_name, path
            );
            quote! {
                #[doc = #doc]
                pub #field_name: Option<#field_type>,
            }
        });

    let struct_fields_create = struct_fields.iter().map(|(_, field_name, _field_type)| {
        quote! { #field_name: None, }
    });
    let struct_fields_get = struct_fields.iter().map(|(_, field_name, field_type)| {
        let load_function_name = format_ident!("load_{}", field_name);
        quote! {
            #[inline]
            #[must_use = "Loading may have not been successful"]
            pub fn #load_function_name(&mut self) -> Option<()> {
                self.#field_name = #field_type::load(&self);
                self.#field_name.as_ref().map(|_| ())
            }
        }
    });

    match level {
        CommandLevel::Core => {
            quote! {
                #[doc = "Loader for Core Commands"]
                pub struct #struct_name<T> {
                    pub loader: T,
                    pub symbol_loader: Box<dyn Fn(&T, &str) -> Option<std::num::NonZeroUsize> + Send + Sync>,
                    #( #struct_fields_code )*
                }

                impl<T> #struct_name<T> {
                    pub fn custom(
                        loader: T,
                        symbol_loader: Box<dyn Fn(&T, &str) -> Option<std::num::NonZeroUsize> + Send + Sync>,
                    ) -> #struct_name<T> {
                        #struct_name {
                            loader,
                            symbol_loader,
                            #( #struct_fields_create )*
                        }
                    }

                    #[inline]
                    #[inline]
                    pub unsafe fn symbol(&self, name: &str) -> Option<std::num::NonZeroUsize> {
                        (self.symbol_loader)(&self.loader, name)
                    }

                    #( #struct_fields_get )*
                }
            }
        }
        CommandLevel::Instance => {
            quote! {
                #[doc = "Loader for Instance Commands"]
                #[doc = "# Safety"]
                #[doc = "[See here](#safety-1)"]
                pub struct #struct_name {
                    pub loader: crate::vk1_0::PFN_vkGetInstanceProcAddr,
                    pub handle: crate::vk1_0::Instance,
                    #( #struct_fields_code )*
                }

                impl #struct_name {
                    #[doc = "# Safety"]
                    #[doc = "Using this loader after dropping `core_loader` results in undefined behaviour"]
                    pub fn new<T>(core_loader: &CoreLoader<T>, instance: crate::vk1_0::Instance) -> Option<#struct_name> {
                        Some(#struct_name {
                            loader: unsafe {
                                std::mem::transmute(core_loader.symbol("vkGetInstanceProcAddr")?)
                            },
                            handle: instance,
                            #( #struct_fields_create )*
                        })
                    }

                    #[inline]
                    pub unsafe fn symbol(&self, name: &str) -> Option<std::num::NonZeroUsize> {
                        let cstring = std::ffi::CString::new(name).unwrap();
                        let ptr: usize = std::mem::transmute((self.loader)(self.handle, cstring.as_ptr()));
                        std::num::NonZeroUsize::new(ptr)
                    }

                    #( #struct_fields_get )*
                }
            }
        }
        CommandLevel::Device => {
            quote! {
                #[doc = "Loader for Device Commands"]
                #[doc = "# Safety"]
                #[doc = "[See here](#safety-1)"]
                pub struct #struct_name {
                    pub loader: crate::vk1_0::PFN_vkGetDeviceProcAddr,
                    pub handle: crate::vk1_0::Device,
                    #( #struct_fields_code )*
                }

                impl #struct_name {
                    #[doc = "# Safety"]
                    #[doc = "Using this loader after dropping `instance_loader` results in undefined behaviour"]
                    pub fn new(instance_loader: &InstanceLoader, device: crate::vk1_0::Device) -> Option<#struct_name> {
                        Some(#struct_name {
                            loader: unsafe {
                                std::mem::transmute(instance_loader.symbol("vkGetDeviceProcAddr")?)
                            },
                            handle: device,
                            #( #struct_fields_create )*
                        })
                    }

                    #[inline]
                    pub unsafe fn symbol(&self, name: &str) -> Option<std::num::NonZeroUsize> {
                        let cstring = std::ffi::CString::new(name).unwrap();
                        let ptr: usize = std::mem::transmute((self.loader)(self.handle, cstring.as_ptr()));
                        std::num::NonZeroUsize::new(ptr)
                    }

                    #( #struct_fields_get )*
                }
            }
        }
    }
}
