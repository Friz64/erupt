use crate::utils;
use heck::ShoutySnakeCase;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashSet;
use vk_parse::{
    Enum, EnumSpec, Enums, EnumsChild, ExtensionChild, InterfaceItem, TypeMemberMarkup,
};

fn guess_type(value: &str) -> (String, &str) {
    if let Ok(_) = value.parse::<u32>() {
        (value.to_string(), "u32")
    } else if let Ok(_) = value.parse::<i32>() {
        (value.to_string(), "i32")
    } else if let Ok(_) = value.parse::<f32>() {
        (value.to_string(), "f32")
    } else {
        assert!(value.starts_with('"') && value.ends_with('"'));

        let value = format!("crate::cstr!({})", value);
        (value, "*const std::os::raw::c_char")
    }
}

pub struct ConstantItem {
    pub name: String,
    pub rust_type: String,
    pub value: String,
}

pub fn collect(
    children: &[ExtensionChild],
    raw_enums: &[&Enums],
    used_items: &mut HashSet<String>,
) -> Vec<ConstantItem> {
    children
        .iter()
        .filter_map(|child| {
            if let ExtensionChild::Require { items, .. } = child {
                let types: Vec<_> = items
                    .iter()
                    .filter_map(|item| match item {
                        InterfaceItem::Enum(Enum { name, spec, .. })
                            if used_items.insert(name.to_string()) =>
                        {
                            match spec {
                                EnumSpec::None => {
                                    let mut search = name.clone();
                                    let value = 'outer: loop {
                                        for raw_enum in raw_enums {
                                            for child in &raw_enum.children {
                                                match child {
                                                    EnumsChild::Enum(Enum {
                                                        name, spec, ..
                                                    }) if name == &search => match spec {
                                                        EnumSpec::Alias { alias, .. } => {
                                                            search = alias.clone();
                                                            continue 'outer;
                                                        }
                                                        EnumSpec::Value { value, .. } => {
                                                            break 'outer Some(value);
                                                        }
                                                        _ => (),
                                                    },
                                                    _ => (),
                                                }
                                            }
                                        }

                                        break None;
                                    }
                                    .unwrap();

                                    let rust_value = value
                                        .trim_start_matches('(')
                                        .trim_end_matches(')')
                                        .replace("~", "!")
                                        .replace("ULL", "u64")
                                        .replace("U", "u32")
                                        .replace("f", "f32");

                                    let rust_type = if rust_value.contains("f32") {
                                        "f32"
                                    } else if rust_value.contains("u64") {
                                        "u64"
                                    } else {
                                        "u32"
                                    };

                                    Some(ConstantItem {
                                        name: name.into(),
                                        rust_type: rust_type.into(),
                                        value: rust_value,
                                    })
                                }
                                EnumSpec::Value { value, extends } if extends.is_none() => {
                                    let (value, rust_type) = guess_type(&value);

                                    Some(ConstantItem {
                                        name: name.into(),
                                        rust_type: rust_type.into(),
                                        value,
                                    })
                                }
                                _ => None,
                            }
                        }
                        _ => None,
                    })
                    .collect();

                Some(types)
            } else {
                None
            }
        })
        .flatten()
        .collect()
}

pub fn extend(
    constants: &mut Vec<ConstantItem>,
    markups: &[TypeMemberMarkup],
    used_items: &mut HashSet<String>,
    raw_enums: &[&Enums],
) {
    for markup in markups {
        if let TypeMemberMarkup::Enum(target_name) = markup {
            if used_items.insert(target_name.into()) {
                for raw_enum in raw_enums {
                    for child in &raw_enum.children {
                        match child {
                            EnumsChild::Enum(Enum {
                                name,
                                spec: EnumSpec::Value { value, .. },
                                ..
                            }) if name == target_name => {
                                let (value, rust_type) = guess_type(&value);

                                constants.push(ConstantItem {
                                    name: name.into(),
                                    rust_type: rust_type.into(),
                                    value,
                                });
                            }
                            _ => (),
                        }
                    }
                }
            }
        }
    }
}

pub fn generate(constant: &ConstantItem) -> TokenStream {
    let doc = utils::doc("Constant", None);
    let name = utils::safe_ident(utils::trim_vk_prefix(&constant.name.to_shouty_snake_case()));
    let rust_type: TokenStream = syn::parse_str(&constant.rust_type).unwrap();
    let value: TokenStream = syn::parse_str(&constant.value).unwrap();

    quote! {
        #[doc = #doc]
        pub const #name: #rust_type = #value;
    }
}
