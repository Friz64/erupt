use crate::{
    cargotoml_gen,
    items::enums::{Enum, EnumKind},
    name::{FunctionName, Name, TypeName},
    source::Source,
    XmlNode,
};
use once_cell::sync::Lazy;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use regex::Regex;
use std::{
    fmt::{self, Debug},
    hash::Hash,
    path::PathBuf,
};

static FEATURE_NAME_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new("VK_VERSION_([1-9]+)_([0-9]+)").unwrap());

const TYPES_SKIPPED: &[&str] = &[
    "vk_platform",
    "VK_DEFINE_HANDLE",
    "VK_DEFINE_NON_DISPATCHABLE_HANDLE",
    "VK_API_VERSION",
    "VK_API_VERSION_1_0",
    "VK_HEADER_VERSION",
    "VK_HEADER_VERSION_COMPLETE",
    "VK_MAKE_VERSION",
    "VK_VERSION_MAJOR",
    "VK_VERSION_MINOR",
    "VK_VERSION_PATCH",
    "VK_MAKE_API_VERSION",
    "VK_API_VERSION_VARIANT",
    "VK_API_VERSION_MAJOR",
    "VK_API_VERSION_MINOR",
    "VK_API_VERSION_PATCH",
    "VK_NULL_HANDLE",
    "VK_API_VERSION_1_1",
    "VK_API_VERSION_1_2",
    "VK_PIPELINE_CREATE_DISPATCH_BASE",
    "VK_USE_64_BIT_PTR_DEFINES",
];

#[derive(Clone, PartialEq, Hash, Eq)]
pub enum Origin {
    Root,
    Feature { major: u32, minor: u32 },
    Extension { full: String },
    External { lib_name: String },
}

impl Origin {
    pub fn feature_from_name(name: &str) -> Origin {
        match FEATURE_NAME_REGEX.captures(name) {
            Some(captures) => Origin::Feature {
                major: captures[1].parse().expect("Invalid major version"),
                minor: captures[2].parse().expect("Invalid minor version"),
            },
            None => panic!("Pattern did not match: {:?}", name),
        }
    }

    pub fn from_registry_item(node: XmlNode) -> Origin {
        match (node.tag_name().name(), node.attribute("name")) {
            ("feature", Some(name)) => Origin::feature_from_name(name),
            ("extension", Some(name)) => Origin::Extension { full: name.into() },
            invalid => panic!("Failed to create origin from registry item: {:?} ", invalid),
        }
    }

    pub fn is_extension(&self) -> bool {
        matches!(self, Origin::Extension { .. })
    }

    pub fn is_vk1_0(&self) -> bool {
        matches!(self, Origin::Feature { major: 1, minor: 0 })
    }

    pub fn path(&self) -> Vec<String> {
        match self.clone() {
            Origin::Extension { full } => vec![
                "extensions".into(),
                full.trim_start_matches("VK_").to_lowercase(),
            ],
            Origin::Feature { major, minor } => vec![format!("vk{}_{}", major, minor)],
            Origin::Root => vec![],
            Origin::External { lib_name } => vec!["external".into(), lib_name],
        }
    }

    pub fn module_path(&self) -> TokenStream {
        let path = self.path();
        if path.is_empty() {
            TokenStream::new()
        } else {
            let items = path.iter().map(|item| format_ident!("{}", item));
            quote! { #(#items::)* }
        }
    }

    pub fn module_path_pretty(&self) -> String {
        self.path().join("::")
    }

    pub fn file_path(&self) -> PathBuf {
        let mut file_path: PathBuf = self.path().into_iter().collect();
        file_path.set_extension("rs");
        file_path
    }

    pub fn ident(&self) -> Ident {
        format_ident!("{}", self.path().last().unwrap())
    }

    pub fn feature(&self) -> Option<String> {
        match self {
            Origin::Extension { full } => Some(cargotoml_gen::ext_feature_name(full)),
            Origin::External { .. } => {
                // todo add (?)
                None
            }
            _ => None,
        }
    }

    pub fn feature_gate(&self) -> Option<TokenStream> {
        self.feature()
            .map(|feature| quote! { #[cfg(feature = #feature)] })
    }

    pub fn inverse_feature_gate(&self) -> Option<TokenStream> {
        self.feature()
            .map(|feature| quote! { #[cfg(not(feature = #feature))] })
    }
}

impl Debug for Origin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Origin::Root => write!(f, "(root)"),
            Origin::Feature { major, minor } => write!(f, "{}.{}", major, minor),
            Origin::Extension { full } => write!(f, "{}", full),
            Origin::External { lib_name } => write!(f, "(external) {}", lib_name),
        }
    }
}

impl Source {
    pub fn origin<'a>(&'a self, name: &Name) -> &'a Origin {
        let origin = match name {
            Name::Type(type_name) => {
                if let Some(alias) = self.find_type_alias(type_name) {
                    &alias.origin
                } else if let Some(structure) = self.find_structure(type_name) {
                    &structure.origin
                } else if let Some(en) = self.find_enum(type_name) {
                    &en.origin
                } else if let Some(handle) = self.find_handle(type_name) {
                    &handle.origin
                } else if let Some(basetype) = self.find_basetype(type_name) {
                    &basetype.origin
                } else if let Some(alias) = self.find_type_alias(type_name) {
                    &alias.origin
                } else {
                    panic!("Unknown origin for type name {:?}", name)
                }
            }
            Name::Function(function_name) => {
                if let Some(alias) = self.find_function_alias(function_name) {
                    &alias.origin
                } else if let Some(function) = self.find_function(function_name) {
                    &function.origin
                } else if let Some(pointer) = self.find_func_pointer(function_name) {
                    &pointer.origin
                } else {
                    panic!("Unknown origin for function name {:?}", name)
                }
            }
        };

        origin.as_ref().expect("Found Item has no origin")
    }

    pub fn assign_origins(&mut self, node: XmlNode) {
        let origin = Origin::from_registry_item(node);
        for node_child in node.children() {
            if node_child.tag_name().name() == "require" {
                for item in node_child.children().filter(|n| n.is_element()) {
                    let name = item.attribute("name");
                    if let Some(name) = name {
                        if TYPES_SKIPPED.contains(&name) {
                            continue;
                        }
                    }

                    match (item.tag_name().name(), name) {
                        ("command", Some(name)) => {
                            let function_name = FunctionName::new(name);
                            if let Some(alias) = self.find_function_alias_mut(&function_name) {
                                alias.origin.get_or_insert(origin.clone());
                            } else if let Some(function) = self.find_function_mut(&function_name) {
                                function.origin.get_or_insert(origin.clone());
                            } else {
                                log::warn!("No function with name {:?}", name);
                            }
                        }
                        ("type", Some(name)) => {
                            let type_name = TypeName::new(name);
                            let function_name = FunctionName::new(name);
                            if let Some(alias) = self.find_type_alias_mut(&type_name) {
                                alias.origin.get_or_insert(origin.clone());
                            } else if let Some(structure) = self.find_structure_mut(&type_name) {
                                structure.origin.get_or_insert(origin.clone());
                            } else if let Some(en) = self.find_enum_mut(&type_name) {
                                en.origin.get_or_insert(origin.clone());
                            } else if let Some(func) = self.find_func_pointer_mut(&function_name) {
                                func.origin.get_or_insert(origin.clone());
                            } else if let Some(basetype) = self.find_basetype_mut(&type_name) {
                                basetype.origin.get_or_insert(origin.clone());
                            } else if let Some(handle) = self.find_handle_mut(&type_name) {
                                handle.origin.get_or_insert(origin.clone());
                            } else {
                                log::warn!("No type with name {:?}", name);
                            }
                        }
                        ("enum", Some(name)) => {
                            if let Some(constant) = self.find_constant_mut(name) {
                                constant.origin.get_or_insert(origin.clone());
                            } else if let Some(variant) = self.find_enum_variant_mut(name) {
                                variant.origin.get_or_insert(origin.clone());
                            } else {
                                log::warn!("No enum item with name {:?}", name);
                            }
                        }
                        ("comment", None) => (),
                        unsupported => {
                            panic!("Unsupported item name: {:?} from {:?}", unsupported, node)
                        }
                    }
                }
            }
        }
    }

    pub fn assign_external_origin(&mut self, element: XmlNode, other_includes_headers: &[PathBuf]) {
        if let (Some(name), Some(requires)) =
            (element.attribute("name"), element.attribute("requires"))
        {
            let is_external = other_includes_headers
                .iter()
                .any(|include| include.ends_with(requires));

            if is_external {
                let origin = Origin::External {
                    lib_name: requires[..requires.find('/').unwrap()].into(),
                };

                if let Some(mut structure) = self.header.take_structure(name) {
                    structure.origin = Some(origin);
                    self.structures.push(structure);
                } else {
                    let enum_kind = EnumKind::guess_from_name(name);
                    let enum_variants = self.header.take_enum_variants(&enum_kind);

                    self.enums.push(Enum {
                        origin: Some(origin),
                        kind: enum_kind,
                        variants: enum_variants,
                    });
                }
            }
        }
    }
}
