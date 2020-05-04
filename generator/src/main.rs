mod commands;
mod constants;
mod enum_exts;
mod types;
mod utils;

use incremental_topo::IncrementalTopo;
use itertools::Itertools;
use lazy_static::lazy_static;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs,
    io::Cursor,
    path::Path,
    process::Command as ProcessCommand,
};
use vk_parse::{CommentedChildren, ExtensionChild, Registry, RegistryChild, TypesChild};

lazy_static! {
    static ref VULKAN_HEADER: String = {
        // this helps us avoid a bunch of ugly code
        let preprocessed = ProcessCommand::new("gcc")
        .args(&[
            "-DVK_USE_PLATFORM_ANDROID_KHR",
            "-DVK_USE_PLATFORM_FUCHSIA",
            "-DVK_USE_PLATFORM_IOS_MVK",
            "-DVK_USE_PLATFORM_MACOS_MVK",
            "-DVK_USE_PLATFORM_METAL_EXT",
            "-DVK_USE_PLATFORM_VI_NN",
            "-DVK_USE_PLATFORM_WAYLAND_KHR",
            "-DVK_USE_PLATFORM_WIN32_KHR",
            "-DVK_USE_PLATFORM_XCB_KHR",
            "-DVK_USE_PLATFORM_XLIB_KHR",
            "-DVK_USE_PLATFORM_XLIB_XRANDR_EXT",
            "-DVK_USE_PLATFORM_GGP",
            "-DVK_ENABLE_BETA_EXTENSIONS",
            "-E",
            "generator/Vulkan-Headers/include/vulkan/vulkan.h",
        ])
        .output()
        .unwrap();

        String::from_utf8(preprocessed.stdout).unwrap()
    };
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Origin {
    // (major, minor)
    Feature(u32, u32),
    // (name, primary_secondary, number)
    Extension(String, Ident, u32),
}

fn main() {
    let header_version: u32 = {
        let regex = Regex::new(r"#define VK_HEADER_VERSION (\d+)").unwrap();
        let source = include_str!("../Vulkan-Headers/include/vulkan/vulkan_core.h");
        regex.captures(source).unwrap()[1].parse().unwrap()
    };

    eprintln!("=> Setting up (Vulkan Header {})", header_version);
    let Registry(registry_children) = vk_parse::parse_stream(Cursor::new(include_str!(
        "../Vulkan-Headers/registry/vk.xml"
    )))
    .expect("Failed to parse Vulkan Headers, `vk-parse` needs a fix")
    .0;

    let features: Vec<_> = registry_children
        .iter()
        .filter_map(|registry_child| match registry_child {
            RegistryChild::Feature(feature) => Some(feature),
            _ => None,
        })
        .collect();
    let extensions: Vec<_> = registry_children
        .iter()
        .filter_map(|registry_child| match registry_child {
            RegistryChild::Extensions(CommentedChildren {
                children: extensions_children,
                ..
            }) => Some(extensions_children),
            _ => None,
        })
        .flatten()
        .filter(|extension| {
            extension.name.starts_with("VK_")
                && extension.supported.as_ref().map(|s| s.as_str()) != Some("disabled")
        })
        .rev()
        .collect();

    let vulkan_version = &features.last().unwrap().number;

    let raw_types: Vec<_> = registry_children
        .iter()
        .filter_map(|registry_child| -> Option<Vec<_>> {
            match registry_child {
                RegistryChild::Types(CommentedChildren {
                    children: types_children,
                    ..
                }) => Some(
                    types_children
                        .iter()
                        .filter_map(|types_child| match types_child {
                            TypesChild::Type(vulkan_type) => Some(vulkan_type),
                            _ => None,
                        })
                        .collect(),
                ),
                _ => None,
            }
        })
        .flatten()
        .collect();
    let raw_enums: Vec<_> = registry_children
        .iter()
        .filter_map(|registry_child| match registry_child {
            RegistryChild::Enums(vulkan_enum) => Some(vulkan_enum),
            _ => None,
        })
        .collect();
    let raw_commands: Vec<_> = registry_children
        .iter()
        .filter_map(|registry_child| match registry_child {
            RegistryChild::Commands(CommentedChildren {
                children: commands_children,
                ..
            }) => Some(commands_children),
            _ => None,
        })
        .flatten()
        .collect();

    let origin_children_map: Vec<_> = features
        .iter()
        .map(|feature| {
            let mut split = feature.number.split('.');
            let major = split.next().unwrap().parse().unwrap();
            let minor = split.next().unwrap().parse().unwrap();
            (Origin::Feature(major, minor), feature.children.clone())
        })
        .chain({
            // Solving extension dependencies
            let mut topo = IncrementalTopo::new();
            let core = "core";
            topo.add_node(core);

            for extension in &extensions {
                topo.add_node(extension.name.as_str());
                topo.add_dependency(core, extension.name.as_str()).unwrap();
            }

            for extension in &extensions {
                if let Some(requires) = &extension.requires {
                    for requirement in requires.split(',') {
                        topo.add_dependency(requirement, extension.name.as_str())
                            .unwrap();
                    }
                }

                for child in &extension.children {
                    if let ExtensionChild::Require {
                        extension: Some(requirement),
                        ..
                    } = child
                    {
                        // Avoid dependency cycles
                        if topo.contains_transitive_dependency(
                            extension.name.as_str(),
                            requirement.as_str(),
                        ) {
                            topo.add_dependency(requirement.as_str(), extension.name.as_str())
                                .unwrap();
                        }
                    }
                }
            }

            let descendants: Vec<_> = topo.descendants(core).unwrap().cloned().collect();
            descendants.into_iter().map(|name| {
                let primary_secondary =
                    utils::safe_ident(&name["VK_".len()..].to_lowercase().as_str());
                let extension = extensions
                    .iter()
                    .find(|extension| extension.name == name)
                    .unwrap();

                (
                    Origin::Extension(
                        name.into(),
                        primary_secondary,
                        extension.number.unwrap() as _,
                    ),
                    extension.children.clone(),
                )
            })
        })
        .collect();

    eprintln!("=> Collecting item information");
    let mut itemname_origin_map = HashMap::new();
    let mut used_items = HashSet::new();
    let mut enum_ext_map = HashMap::new();
    let origin_items_map: Vec<_> = origin_children_map
        .iter()
        .map(|(origin, children)| {
            let mut constants = constants::collect(&children, &raw_enums, &mut used_items);
            let commands = commands::collect(&children, &raw_commands);

            let enum_exts = enum_exts::collect(&children);
            for (extends, variants) in enum_exts.clone() {
                enum_ext_map
                    .entry(extends)
                    .or_insert_with(|| HashMap::new())
                    .insert(origin.clone(), variants);
            }

            let types = types::collect(
                &children,
                &raw_types,
                &raw_enums,
                &commands,
                &mut used_items,
                &mut constants,
            );

            for constant in &constants {
                itemname_origin_map.insert(constant.name.clone(), origin.clone());
            }

            for vk_type in &types {
                for name in vk_type.names() {
                    itemname_origin_map.insert(name.clone(), origin.clone());
                }
            }

            let extends: Vec<_> = enum_exts
                .into_iter()
                .map(|(extends, _)| extends)
                .sorted()
                .collect();
            (origin, constants, commands, types, extends)
        })
        .collect();

    let generated_path = Path::new("./erupt/src/generated/");
    fs::remove_dir_all(&generated_path).unwrap();
    fs::create_dir(&generated_path).unwrap();

    let extension_path = generated_path.join("extensions");
    fs::create_dir(&extension_path).unwrap();

    eprintln!("=> Generating code");
    let mut generated_features = Vec::new();
    let mut generated_extensions = Vec::new();
    let mut loader_map = Vec::new();
    let all_types: Vec<_> = origin_items_map
        .iter()
        .flat_map(|(_, _, _, types, _)| types)
        .collect();
    for (origin, constants, commands, types, extends) in &origin_items_map {
        let mut output_stream = TokenStream::new();
        if let Origin::Extension(name, _, _) = origin {
            let mut doc = utils::man_doc(vulkan_version, name);

            let mut extends_string = format!("\n\n## Extends");
            let mut extends_pushed = false;
            for extends in extends {
                let to_origin = itemname_origin_map.get(extends).unwrap();
                if to_origin != *origin {
                    let name = utils::trim_vk_prefix(extends);
                    let path = utils::doc_path(
                        &utils::origin_path(to_origin),
                        &utils::origin_path(origin),
                    );

                    extends_pushed = true;
                    extends_string.push_str(&format!(
                        "\n- [`{name}`]({}/struct.{name}.html)",
                        path,
                        name = name
                    ));
                }
            }

            if extends_pushed {
                doc.push_str(&extends_string);
            }

            output_stream.extend(quote! {
                #![doc = #doc]
            });
        }

        output_stream.extend(
            constants
                .iter()
                .map(|constant| constants::generate(constant)),
        );

        let mut loader_commands = Vec::new();
        output_stream.extend(commands.iter().map(|command| {
            commands::generate(
                vulkan_version,
                command,
                &itemname_origin_map,
                &mut loader_commands,
            )
        }));

        let (loaders, loader_code) =
            commands::generate_loaders(origin, &loader_commands, &all_types);
        loader_map.push((origin, loaders));
        output_stream.extend(loader_code);

        output_stream.extend(types.iter().cloned().map(|mut vk_type| {
            enum_exts::extend(&enum_ext_map, &mut vk_type);
            types::generate(&vulkan_version, &vk_type, &all_types, &itemname_origin_map)
        }));

        let save_path = match origin {
            Origin::Feature(major, minor) => {
                let mod_name = format!("vk{}_{}", major, minor);
                generated_features.push(utils::safe_ident(&mod_name));
                generated_path.join(mod_name + ".rs")
            }
            Origin::Extension(_, primary_secondary, _) => {
                generated_extensions.push(primary_secondary.clone());
                extension_path.join(primary_secondary.to_string() + ".rs")
            }
        };

        fs::write(save_path, output_stream.to_string()).unwrap();
    }

    let const_function_doc = |name| {
        utils::doc(
            "Const Function",
            Some(&utils::man_doc(vulkan_version, name)),
        )
    };
    let make_version_doc = const_function_doc("VK_MAKE_VERSION");
    let version_major_doc = const_function_doc("VK_VERSION_MAJOR");
    let version_minor_doc = const_function_doc("VK_VERSION_MINOR");
    let version_patch_doc = const_function_doc("VK_VERSION_PATCH");
    let loaders_root = commands::generate_loaders_root(&loader_map);
    let generated_mod_path = generated_path.join("mod.rs");
    fs::write(
        &generated_mod_path,
        quote! {
            #loaders_root

            // from ash
            #[doc = #make_version_doc]
            pub const fn make_version(major: u32, minor: u32, patch: u32) -> u32 {
                (major << 22) | (minor << 12) | patch
            }

            // from ash
            #[doc = #version_major_doc]
            pub const fn version_major(version: u32) -> u32 {
                version >> 22
            }

            // from ash
            #[doc = #version_minor_doc]
            pub const fn version_minor(version: u32) -> u32 {
                (version >> 12) & 0x3ff
            }

            // from ash
            #[doc = #version_patch_doc]
            pub const fn version_patch(version: u32) -> u32 {
                version & 0xfff
            }

            /// Provides Vulkan extension items
            pub mod extensions;
            #(
                /// Provides Vulkan feature items
                pub mod #generated_features;
            )*
        }
        .to_string(),
    )
    .unwrap();

    let extension_mod_path = extension_path.join("mod.rs");
    fs::write(
        &extension_mod_path,
        quote! {
            #(
                pub mod #generated_extensions;
            )*
        }
        .to_string(),
    )
    .unwrap();

    eprintln!("=> Running rustfmt");
    ProcessCommand::new("cargo")
        .args(&["fmt", "-p", "erupt"])
        .status()
        .unwrap();

    eprintln!("=> Running rustdoc");
    ProcessCommand::new("cargo")
        .args(&["doc", "-p", "erupt"])
        .status()
        .unwrap();
}
