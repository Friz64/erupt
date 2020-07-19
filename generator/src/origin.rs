use crate::{
    name::{FunctionName, Name, TypeName},
    source::Source,
};
use itertools::Itertools;
use lazy_static::lazy_static;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use regex::Regex;
use std::{
    fmt::{self, Debug},
    iter,
    path::PathBuf,
};
use treexml::Element;

lazy_static! {
    static ref FEATURE_NAME_REGEX: Regex = Regex::new("VK_VERSION_([1-9]+)_([0-9]+)").unwrap();
}

const BLACKLIST: &[&str] = &[
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
    "VK_NULL_HANDLE",
    "VK_API_VERSION_1_1",
    "VK_API_VERSION_1_2",
    "VK_PIPELINE_CREATE_DISPATCH_BASE",
];

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Origin {
    Root,
    Feature { major: u32, minor: u32 },
    Extension { full: String },
}

impl Origin {
    pub fn from_feature_name(name: &str) -> Origin {
        match FEATURE_NAME_REGEX.captures(name) {
            Some(captures) => Origin::Feature {
                major: captures[1].parse().expect("Invalid major version"),
                minor: captures[2].parse().expect("Invalid minor version"),
            },
            None => panic!("Pattern did not match: {:?}", name),
        }
    }

    pub fn from_extension_name(name: &str) -> Origin {
        Origin::Extension { full: name.into() }
    }

    pub fn from_registry_item(element: &Element) -> Origin {
        match (element.name.as_str(), element.attributes.get("name")) {
            ("feature", Some(name)) => Origin::from_feature_name(name),
            ("extension", Some(name)) => Origin::from_extension_name(name),
            invalid => panic!("Failed to create origin from registry item: {:?} ", invalid),
        }
    }

    pub fn path(&self) -> Vec<String> {
        match self.clone() {
            Origin::Extension { full } => vec![
                "extensions".into(),
                full.trim_start_matches("VK_").to_lowercase(),
            ],
            Origin::Feature { major, minor } => vec![format!("vk{}_{}", major, minor)],
            Origin::Root => vec![],
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

    pub fn doc_path(&self, target: &Origin) -> String {
        if self == target {
            ".".into()
        } else {
            let current_path = self.path();
            let target_path = target.path();

            iter::repeat("..")
                .take(current_path.len())
                .chain(target_path.iter().map(|s| s.as_str()))
                .join("/")
        }
    }

    pub fn ident(&self) -> Ident {
        format_ident!("{}", self.path().last().unwrap())
    }
}

impl Debug for Origin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Origin::Root => f.write_str("(root)"),
            Origin::Feature { major, minor } => f.write_str(&format!("{}.{}", major, minor)),
            Origin::Extension { full } => f.write_str(&full),
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

    pub fn assign_origins(&mut self, element: &Element) {
        let origin = Origin::from_registry_item(element);
        for element_child in &element.children {
            if element_child.name == "require" {
                for item in &element_child.children {
                    let name = item.attributes.get("name");
                    if let Some(name) = name {
                        if BLACKLIST.contains(&name.as_str()) {
                            continue;
                        }
                    }

                    match (item.name.as_str(), name) {
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
                        unsupported => panic!(
                            "Unsupported item name: {:?} from {:?}",
                            unsupported, element
                        ),
                    }
                }
            }
        }
    }
}
