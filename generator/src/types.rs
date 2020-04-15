use crate::{
    commands::CommandItem,
    constants::{self, ConstantItem},
    utils, Origin,
};
use heck::{ShoutySnakeCase, SnakeCase};
use itertools::Itertools;
use lazy_static::lazy_static;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use syn::Ident;
use utils::ParsedDeclaration;
use vk_parse::{
    Enums, EnumsChild, ExtensionChild, InterfaceItem, Type, TypeCode, TypeCodeMarkup, TypeMember,
    TypeMemberDefinition, TypeMemberMarkup, TypeSpec,
};

lazy_static! {
    static ref FLAGS_REGEX: Regex = Regex::new("Flags[A-Z]*$").unwrap();
    static ref ENUM_MAP: HashMap<String, String> = {
        let regex = Regex::new(r"(\w+) = ([-\w]+),").unwrap();
        regex
            .captures_iter(&crate::VULKAN_HEADER)
            .map(|capture| (capture[1].to_string(), capture[2].to_string()))
            .collect()
    };
    static ref ADF_REGEX: Regex =
        Regex::new(r"typedef (\w+)(\*)? \(VKAPI_PTR \*(\w+)\)\(").unwrap();
    static ref STRING_REGEX: Regex = Regex::new(r"\[std::os::raw::c_char; [^\[\]]+\]").unwrap();
    static ref EXT_TYPE_NAME: Regex = Regex::new("(.*[a-z])([A-Z]+)?").unwrap();
}

// Types that are either not relevant or have been manually implemented using const functions
const BLACKLIST: &[&str] = &[
    "vk_platform",
    "VK_API_VERSION",
    "VK_API_VERSION_1_0",
    "VK_VERSION_MAJOR",
    "VK_VERSION_MINOR",
    "VK_VERSION_PATCH",
    "VK_HEADER_VERSION",
    "VK_HEADER_VERSION_COMPLETE",
    "VK_NULL_HANDLE",
    "VK_API_VERSION_1_1",
    "VK_API_VERSION_1_2",
];
const STRUCTURE_TYPE: &str = "crate::vk1_0::StructureType";
const SHADER_MODULE_CREATE_INFO_BUILDER: &str = "ShaderModuleCreateInfoBuilder";

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TypeItem {
    BaseType {
        name: String,
        c_type: String,
    },
    Structure {
        name: String,
        fields: Vec<(ParsedDeclaration, Option<Ident>)>,
        union: bool,
        structure_type: Option<String>,
        extends: Vec<String>,
    },
    Handle {
        name: String,
        dispatchable: bool,
    },
    Enum {
        name: String,
        variants: HashMap<Option<Origin>, Vec<String>>,
    },
    Bitmask {
        flags: String,
        flag_bits: String,
        variants: HashMap<Option<Origin>, Vec<String>>,
    },
    Alias {
        name: String,
        to: String,
    },
    ApplicationDefinedFunction {
        name: String,
        code: String,
    },
}

impl TypeItem {
    pub fn names(&self) -> Vec<&String> {
        match self {
            TypeItem::BaseType { name, .. } => vec![name],
            TypeItem::Structure { name, .. } => vec![name],
            TypeItem::Handle { name, .. } => vec![name],
            TypeItem::Enum { name, .. } => vec![name],
            TypeItem::Bitmask {
                flags, flag_bits, ..
            } => vec![flags, flag_bits],
            TypeItem::Alias { name, .. } => vec![name],
            TypeItem::ApplicationDefinedFunction { name, .. } => vec![name],
        }
    }
}

fn recurse_fields(types: &[&Type], nested_types: &mut Vec<String>, type_name: &String) {
    if !nested_types.contains(type_name) {
        nested_types.push(type_name.into());

        let target_type = types
            .iter()
            .find(|vk_type| vk_type.name.as_ref() == Some(type_name));

        if let Some(target_type) = target_type {
            if let TypeSpec::Members(members) = &target_type.spec {
                if let Some(category) = &target_type.category {
                    match category.as_str() {
                        "struct" | "union" => {
                            for member in members {
                                match member {
                                    TypeMember::Definition(TypeMemberDefinition {
                                        markup: markups,
                                        ..
                                    }) => {
                                        for markup in markups {
                                            match markup {
                                                TypeMemberMarkup::Type(name) => {
                                                    recurse_fields(types, nested_types, name);
                                                }
                                                _ => (),
                                            }
                                        }
                                    }
                                    _ => (),
                                }
                            }
                        }
                        _ => (),
                    }
                }
            }
        }
    }
}

pub fn collect(
    children: &[ExtensionChild],
    raw_types: &[&Type],
    raw_enums: &[&Enums],
    commands: &[CommandItem],
    used_items: &mut HashSet<String>,
    constants: &mut Vec<ConstantItem>,
) -> Vec<TypeItem> {
    commands
        .iter()
        .flat_map(|command| command.associated_types.iter())
        .chain(
            children
                .iter()
                .filter_map(|child| {
                    if let ExtensionChild::Require { items, .. } = child {
                        let types = items.iter().filter_map(|item| match item {
                            InterfaceItem::Type { name, .. } => Some(name),
                            _ => None,
                        });

                        Some(types)
                    } else {
                        None
                    }
                })
                .flatten(),
        )
        .flat_map(|type_name| {
            let mut nested_types = Vec::new();
            recurse_fields(raw_types, &mut nested_types, type_name);
            nested_types
        })
        .flat_map(|type_name| {
            let mut output = vec![type_name.clone()];
            for vk_type in raw_types {
                if let Some("funcpointer") = vk_type.category.as_ref().map(|s| s.as_str()) {
                    if let TypeSpec::Code(TypeCode { markup, .. }) = &vk_type.spec {
                        let mut name = None;
                        for markup in markup {
                            if let TypeCodeMarkup::Name(name_val) = markup {
                                name = Some(name_val);
                                break;
                            }
                        }

                        if name == Some(&type_name) {
                            for markup in markup {
                                if let TypeCodeMarkup::Type(name) = markup {
                                    output.push(name.clone());
                                }
                            }
                        }
                    }
                }
            }

            output
        })
        .filter(|type_name| utils::c_type_to_rust(type_name).is_none())
        .filter(|type_name| !BLACKLIST.contains(&type_name.as_str()))
        .map(|type_name| type_name.replace("FlagBits", "Flags"))
        // We now have all the type names, now convert them into `TypeItem`s
        .filter_map(|type_name| {
            if !used_items.insert(type_name.to_string()) {
                return None;
            }

            let flag_bits = if FLAGS_REGEX.is_match(&type_name) {
                Some(type_name.replace("Flags", "FlagBits"))
            } else {
                None
            };

            for vk_type in raw_types {
                if vk_type.name.as_ref() == Some(&type_name) {
                    if let Some(alias) = &vk_type.alias {
                        return Some(TypeItem::Alias {
                            name: type_name,
                            to: alias.into(),
                        });
                    }
                }

                if let Some(category) = &vk_type.category {
                    match category.as_str() {
                        "define" => match &vk_type.spec {
                            TypeSpec::Code(TypeCode {
                                markup: markups,
                                code,
                            }) => {
                                if code.starts_with("struct ") {
                                    let mut name = None;

                                    for markup in markups {
                                        match markup {
                                            TypeCodeMarkup::Name(name_val) => {
                                                name = Some(name_val.to_string())
                                            }
                                            _ => (),
                                        }
                                    }

                                    match name {
                                        Some(name) if name == type_name => {
                                            return Some(TypeItem::Structure {
                                                name: name.into(),
                                                fields: vec![],
                                                union: false,
                                                structure_type: None,
                                                extends: vec![],
                                            })
                                        }
                                        _ => (),
                                    }
                                }
                            }
                            _ => (),
                        },
                        "basetype" => match &vk_type.spec {
                            TypeSpec::Code(TypeCode {
                                markup: markups, ..
                            }) => {
                                let mut name = None;
                                let mut c_type = None;

                                for markup in markups {
                                    match markup {
                                        TypeCodeMarkup::Type(c_type_val) => {
                                            c_type = Some(c_type_val.into())
                                        }
                                        TypeCodeMarkup::Name(name_val) => {
                                            name = Some(name_val.to_string())
                                        }
                                        _ => (),
                                    }
                                }

                                match (name, c_type) {
                                    (Some(name), Some(c_type)) if name == type_name => {
                                        return Some(TypeItem::BaseType {
                                            name: type_name,
                                            c_type,
                                        });
                                    }
                                    _ => (),
                                }
                            }
                            _ => (),
                        },
                        "struct" | "union" => {
                            if vk_type.name.as_ref() == Some(&type_name) {
                                if let TypeSpec::Members(members) = &vk_type.spec {
                                    let mut structure_type = None;
                                    let fields: Vec<_> = members
                                        .iter()
                                        .filter_map(|member| match member {
                                            TypeMember::Definition(def) => {
                                                // fields can contain constants (for example array lengths)
                                                constants::extend(
                                                    constants,
                                                    &def.markup,
                                                    used_items,
                                                    &raw_enums,
                                                );

                                                if let Some(values) = &def.values {
                                                    for val in values.split(',') {
                                                        if val.starts_with("VK_STRUCTURE_TYPE_") {
                                                            structure_type = Some(val.to_string());
                                                        }
                                                    }
                                                }

                                                let length = utils::len(&def.len, &def.altlen)
                                                    .into_iter()
                                                    .filter_map(|len| {
                                                        if syn::parse_str::<Ident>(&len).is_ok() {
                                                            let parsed = syn::parse_str::<Ident>(
                                                                &len.to_snake_case(),
                                                            );

                                                            Some(parsed.unwrap())
                                                        } else {
                                                            None
                                                        }
                                                    })
                                                    .next();
                                                let parsed_declaration =
                                                    utils::parse_c_decl(&def.code).unwrap();

                                                Some((parsed_declaration, length))
                                            }
                                            _ => None,
                                        })
                                        .collect();

                                    let extends = vk_type
                                        .structextends
                                        .clone()
                                        .unwrap_or_default()
                                        .split(',')
                                        .filter(|s| !s.is_empty())
                                        .map(|s| s.to_string())
                                        .collect();

                                    return Some(TypeItem::Structure {
                                        name: type_name,
                                        fields,
                                        union: category == "union",
                                        structure_type,
                                        extends,
                                    });
                                }
                            }
                        }
                        "handle" => match &vk_type.spec {
                            TypeSpec::Code(TypeCode {
                                markup: markups, ..
                            }) => {
                                let mut dispatchable = None;
                                let mut name = None;

                                for markup in markups {
                                    match markup {
                                        TypeCodeMarkup::Type(type_val)
                                            if type_val == "VK_DEFINE_HANDLE" =>
                                        {
                                            dispatchable = Some(true)
                                        }
                                        TypeCodeMarkup::Type(type_val)
                                            if type_val == "VK_DEFINE_NON_DISPATCHABLE_HANDLE" =>
                                        {
                                            dispatchable = Some(false)
                                        }
                                        TypeCodeMarkup::Name(name_val) => {
                                            name = Some(name_val.to_string())
                                        }
                                        _ => (),
                                    }
                                }

                                match (name, dispatchable) {
                                    (Some(name), Some(dispatchable)) if name == type_name => {
                                        return Some(TypeItem::Handle {
                                            name: type_name,
                                            dispatchable,
                                        });
                                    }
                                    _ => (),
                                }
                            }
                            _ => (),
                        },
                        "enum" => {
                            let search_name = if let Some(flag_bits) = &flag_bits {
                                flag_bits
                            } else {
                                &type_name
                            };

                            if vk_type.name.as_ref() == Some(&search_name) {
                                for vk_enum in raw_enums {
                                    if vk_enum.name.as_ref() == Some(&search_name) {
                                        let mut map = HashMap::new();
                                        map.insert(
                                            None,
                                            vk_enum
                                                .children
                                                .iter()
                                                .filter_map(move |variant| {
                                                    if let EnumsChild::Enum(variant) = variant {
                                                        Some(variant.name.clone())
                                                    } else {
                                                        None
                                                    }
                                                })
                                                .collect(),
                                        );

                                        return Some(if let Some(flag_bits) = &flag_bits {
                                            TypeItem::Bitmask {
                                                flags: type_name,
                                                flag_bits: flag_bits.clone(),
                                                variants: map,
                                            }
                                        } else {
                                            TypeItem::Enum {
                                                name: type_name,
                                                variants: map,
                                            }
                                        });
                                    }
                                }
                            }
                        }
                        "funcpointer" => match &vk_type.spec {
                            TypeSpec::Code(TypeCode {
                                markup: markups,
                                code,
                            }) => {
                                let mut name = None;

                                for markup in markups {
                                    match markup {
                                        TypeCodeMarkup::Name(name_val) => {
                                            name = Some(name_val.to_string())
                                        }
                                        _ => (),
                                    }
                                }

                                match name {
                                    Some(name) if name == type_name => {
                                        return Some(TypeItem::ApplicationDefinedFunction {
                                            name: type_name,
                                            code: code.into(),
                                        });
                                    }
                                    _ => (),
                                }
                            }
                            _ => (),
                        },
                        _ => (),
                    }
                }
            }

            if let Some(flag_bits) = flag_bits {
                return Some(TypeItem::Bitmask {
                    flag_bits,
                    flags: type_name,
                    variants: HashMap::new(),
                });
            }

            println!("BUG => Not implemented: {}", type_name);
            None
        })
        .collect()
}

fn enum_impl_doc(to_origin: &Origin, host_origin: &Origin) -> String {
    let origin_path_raw = utils::origin_path(to_origin);
    let doc_path =
        utils::doc_path(&origin_path_raw, &utils::origin_path(host_origin)) + "/index.html";
    format!("[Part of `{}`]({})", origin_path_raw, doc_path)
}

fn enum_trim_variant(variant: &str, enum_name: &str) -> String {
    let captures = EXT_TYPE_NAME
        .captures(utils::trim_vk_prefix(enum_name))
        .unwrap();
    let adjusted_name = captures[1]
        .trim_end_matches("FlagBits")
        .to_shouty_snake_case()
        + "_";

    utils::trim_vk_prefix(variant)
        .to_shouty_snake_case()
        .trim_start_matches(&adjusted_name)
        .replace("_BIT", "")
}

fn value_code(enum_name: &str, variant: &str) -> Option<TokenStream> {
    let value_raw = &ENUM_MAP.get(variant)?;
    let code = if value_raw.parse::<i64>().is_ok() || value_raw.starts_with("0x") {
        let parsed: TokenStream = syn::parse_str(value_raw).unwrap();
        quote! {
            Self(#parsed)
        }
    } else {
        let alias = utils::safe_ident(&enum_trim_variant(&value_raw, enum_name));
        quote! {
            Self::#alias
        }
    };
    Some(code)
}

pub fn generate(
    vulkan_version: &str,
    vk_type: &TypeItem,
    all_types: &[&TypeItem],
    itemname_origin_map: &HashMap<String, Origin>,
) -> TokenStream {
    match vk_type {
        TypeItem::BaseType { name, c_type } => {
            let man_doc = utils::man_doc(vulkan_version, &name);
            let name_ident = utils::safe_ident(utils::trim_vk_prefix(&name));
            let doc = utils::doc("Base Type", Some(&man_doc));
            let c_type = utils::safe_ident(utils::c_type_to_rust(c_type).unwrap());

            quote! {
                #[doc = #doc]
                pub type #name_ident = #c_type;
            }
        }
        TypeItem::Structure {
            name,
            fields,
            union,
            structure_type,
            extends,
        } => {
            let trimmed_name = utils::trim_vk_prefix(&name);
            let man_doc = utils::man_doc(vulkan_version, &name);
            let name_ident = utils::safe_ident(trimmed_name);
            let doc = utils::doc("Structure", Some(&man_doc));
            let fields: Vec<_> = fields
                .iter()
                .map(|(field, length)| {
                    let converted = utils::convert_c_decl(itemname_origin_map, field);
                    (converted, length, field.1.clone())
                })
                .collect();

            let field_code = fields.iter().map(|((_, _, _, _, declaration), _, _)| {
                quote! {
                    pub #declaration
                }
            });

            let (structure, derives, impls) = if *union {
                (
                    quote! { union },
                    quote! { #[derive(Copy, Clone)] },
                    quote! {
                        impl std::fmt::Debug for #name_ident {
                            fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                                fmt.debug_struct(#trimmed_name).finish()
                            }
                        }

                        impl Default for #name_ident {
                            fn default() -> #name_ident {
                                unsafe { std::mem::zeroed() }
                            }
                        }
                    },
                )
            } else {
                let extendable_by_name = format_ident!("ExtendableBy{}", trimmed_name);
                let extendable_by_doc = format!(
                    "Used by [`{name}::extend`](struct.{name}.html#method.extend)",
                    name = trimmed_name
                );
                let (extendable_by_code, extend_function) = if !extends.is_empty() {
                    (
                        quote! {
                            #[doc = #extendable_by_doc]
                            pub trait #extendable_by_name {}
                        },
                        quote! {
                            #[inline]
                            #[doc = "Appends `self` to `other` pointer chain"]
                            pub fn extend<T>(&mut self, other: &mut T) where T: #extendable_by_name {
                                unsafe {
                                    crate::append_ptr_chain(
                                        other as *mut T as _,
                                        self as *mut Self as _,
                                    );
                                }
                            }
                        },
                    )
                } else {
                    (quote! {}, quote! {})
                };
                let extendable_by_impls = extends.iter().map(|name| {
                    let path: TokenStream =
                        syn::parse_str(&utils::item_path(itemname_origin_map, name)).unwrap();
                    quote! { impl #extendable_by_name for #path {} }
                });

                let debug_fields = fields.iter().map(|((_, name, field_type, _, _), _, _)| {
                    let value = if STRING_REGEX.is_match(&field_type) {
                        quote! {
                            unsafe { std::ffi::CStr::from_ptr(self.#name.as_ptr() as _) }
                        }
                    } else if field_type == utils::BOOL32 {
                        quote! { (self.#name != 0) }
                    } else if field_type.contains("PFN_") {
                        quote! {
                            unsafe {
                                std::mem::transmute::<_, std::ptr::NonNull<()>>(self.#name)
                            }
                        }
                    } else {
                        quote! { self.#name }
                    };
                    let name_string = name.to_string();

                    quote! { .field(#name_string, &#value) }
                });

                let field_len_map: HashMap<_, _> = fields
                    .iter()
                    .filter_map(|((_, name, _, _, _), length, _)| {
                        length.as_ref().map(|length| (name, length))
                    })
                    .collect();
                let builder_name = format_ident!("{}Builder", name_ident);
                let builder_doc = format!(
                    "Builder of [`{name}`](struct.{name}.html)",
                    name = trimmed_name
                );
                let builder_functions: Vec<_> = fields
                    .iter()
                    .filter(|((_, name, field_type, _, _), _, _)| {
                        !field_len_map.values().any(|v| v == &name)
                            && field_type != STRUCTURE_TYPE
                            && name != "p_next"
                            && !(builder_name == SHADER_MODULE_CREATE_INFO_BUILDER
                                && name == "code_size")
                    })
                    .map(|((_, name_ident, field_type, _, _), _, c_type)| {
                        let mut field_type = field_type.clone();
                        let name_string = name_ident.to_string();
                        let adjusted_name = utils::safe_ident(
                            &utils::RELEVANT_NAME_REGEX.captures(&name_string).unwrap()[2],
                        );

                        let (starts_ref, mut_ref) = if field_type.starts_with(utils::CONST_PTR) {
                            field_type =
                                field_type.replacen(utils::CONST_PTR, utils::CONST_REF_A, 1);
                            (true, false)
                        } else if field_type.starts_with(utils::MUT_PTR) {
                            field_type = field_type.replacen(utils::MUT_PTR, utils::MUT_REF_A, 1);
                            (true, true)
                        } else {
                            (false, false)
                        };
                        let setter = if let Some(len) = field_len_map.get(name_ident) {
                            assert!(starts_ref);

                            let is_struct = all_types.iter().any(|inner_type| {
                                if let TypeItem::Structure {
                                    union: false,
                                    name: inner_name,
                                    ..
                                } = inner_type
                                {
                                    inner_name == c_type
                                } else {
                                    false
                                }
                            });

                            let ref_str = if mut_ref {
                                utils::MUT_REF_A
                            } else {
                                utils::CONST_REF_A
                            };

                            field_type = format!(
                                "{}[{}{}]",
                                ref_str,
                                field_type.trim_start_matches(ref_str),
                                if is_struct { "Builder" } else { "" }
                            );

                            if mut_ref {
                                quote! {
                                    self.0 .#len = #adjusted_name.len() as _;
                                    self.0 .#name_ident = #adjusted_name.as_mut_ptr() as _;
                                }
                            } else {
                                quote! {
                                    self.0 .#len = #adjusted_name.len() as _;
                                    self.0 .#name_ident = #adjusted_name.as_ptr() as _;
                                }
                            }
                        } else {
                            match (
                                &*field_type,
                                &*adjusted_name.to_string(),
                                &*builder_name.to_string(),
                            ) {
                                (utils::BOOL32, _, _) => {
                                    field_type = "bool".into();
                                    quote! {
                                        self.0 .#name_ident = #adjusted_name as u32;
                                    }
                                }
                                ("&'a   std::os::raw::c_char", _, _) => {
                                    field_type = "&'a std::ffi::CStr".into();
                                    quote! {
                                        self.0 .#name_ident = #adjusted_name.as_ptr();
                                    }
                                }
                                (_, "code", crate::types::SHADER_MODULE_CREATE_INFO_BUILDER) => {
                                    field_type = "&'a [u8]".into();
                                    quote! {
                                        assert_eq!(code.len() % 4, 0);
                                        self.0 .code_size = code.len();
                                        self.0 .p_code = code.as_ptr() as _;
                                    }
                                }
                                _ => quote! {
                                    self.0 .#name_ident = #adjusted_name;
                                },
                            }
                        };

                        let field_type_code: TokenStream = syn::parse_str(&field_type).unwrap();
                        quote! {
                            #[allow(unused_mut)]
                            #[inline]
                            pub fn #adjusted_name(
                                mut self,
                                #adjusted_name: #field_type_code
                            ) -> Self {
                                #setter
                                self
                            }
                        }
                    })
                    .collect();

                let defaults =
                    fields
                        .iter()
                        .map(|((_, name, field_type, _field_type_code, _), _, _)| {
                            let value = if field_type == STRUCTURE_TYPE
                                && !["BaseInStructure", "BaseOutStructure"].contains(&trimmed_name)
                            {
                                let value: TokenStream = syn::parse_str(&format!(
                                    "{}::{}",
                                    STRUCTURE_TYPE,
                                    structure_type
                                        .as_ref()
                                        .expect(trimmed_name)
                                        .trim_start_matches("VK_STRUCTURE_TYPE_"),
                                ))
                                .unwrap();

                                quote! { #value }
                            } else if field_type.starts_with(utils::CONST_PTR) {
                                quote! { std::ptr::null() }
                            } else if field_type.starts_with(utils::MUT_PTR) {
                                quote! { std::ptr::null_mut() }
                            } else if STRING_REGEX.is_match(field_type) {
                                quote! { unsafe { std::mem::zeroed() } }
                            } else {
                                quote! { Default::default() }
                            };

                            quote! { #name: #value, }
                        });

                (
                    quote! { struct },
                    quote! { #[derive(Copy, Clone)] },
                    quote! {
                        impl #name_ident {
                            #extend_function

                            #[inline]
                            pub fn builder<'a>(self) -> #builder_name<'a> {
                                #builder_name(self, std::marker::PhantomData)
                            }
                        }

                        impl std::fmt::Debug for #name_ident {
                            fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                                fmt.debug_struct(#trimmed_name)#( #debug_fields )*.finish()
                            }
                        }

                        impl Default for #name_ident {
                            fn default() -> #name_ident {
                                #name_ident {
                                    #( #defaults )*
                                }
                            }
                        }

                        #extendable_by_code
                        #( #extendable_by_impls )*

                        #[derive(Copy, Clone)]
                        #[doc = #builder_doc]
                        #[repr(transparent)]
                        pub struct #builder_name<'a>(#name_ident, std::marker::PhantomData<&'a ()>);

                        impl<'a> #builder_name<'a> {
                            #[inline]
                            pub fn new() -> #builder_name<'a> {
                                #builder_name(Default::default(), std::marker::PhantomData)
                            }

                            #( #builder_functions )*

                            #[inline]
                            #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
                            pub unsafe fn discard(self) -> #name_ident {
                                self.0
                            }
                        }

                        impl<'a> std::fmt::Debug for #builder_name<'a> {
                            fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                                std::fmt::Debug::fmt(&self.0, fmt)
                            }
                        }

                        impl<'a> std::ops::Deref for #builder_name<'a> {
                            type Target = #name_ident;

                            fn deref(&self) -> &Self::Target {
                                &self.0
                            }
                        }

                        impl<'a> std::ops::DerefMut for #builder_name<'a> {
                            fn deref_mut(&mut self) -> &mut Self::Target {
                                &mut self.0
                            }
                        }
                    },
                )
            };

            quote! {
                #[doc = #doc]
                #derives
                #[repr(C)]
                pub #structure #name_ident {
                    #( #field_code )*
                }

                #impls
            }
        }
        TypeItem::Handle { name, dispatchable } => {
            let man_doc = utils::man_doc(vulkan_version, &name);
            let name_ident = utils::safe_ident(utils::trim_vk_prefix(&name));
            let (name, handle_macro) = if *dispatchable {
                ("Handle", quote! { crate::handle! })
            } else {
                (
                    "Non-dispatchable Handle",
                    quote! { crate::non_dispatchable_handle! },
                )
            };
            let doc = utils::doc(name, Some(&man_doc));

            let object_type: Ident =
                syn::parse_str(&name_ident.to_string().to_shouty_snake_case()).unwrap();

            quote! {
                #handle_macro(#name_ident, #object_type, doc = #doc);
            }
        }
        TypeItem::Enum { name, variants } => {
            let man_doc = utils::man_doc(vulkan_version, &name);
            let name_ident = utils::safe_ident(utils::trim_vk_prefix(&name));
            let doc = utils::doc("Enum", Some(&man_doc));

            let enum_origin = itemname_origin_map.get(name).unwrap();
            let mut generated_variants = HashSet::new();
            let mut used_values = HashSet::new();
            let mut debug_arms = Vec::new();
            let impls: Vec<_> = variants
                .iter()
                .sorted()
                .map(|(variant_origin, variants)| {
                    let variants = variants.iter().filter_map(|variant| {
                        let name_raw = enum_trim_variant(&variant, &name);

                        if !generated_variants.insert(name_raw.clone()) {
                            return None;
                        }

                        let variant_name = utils::safe_ident(&name_raw);
                        let value = value_code(name, variant)?;

                        let value_string = value.to_string();
                        if !value_string.contains("::") && used_values.insert(value_string) {
                            debug_arms.push(quote! { &Self::#variant_name => #name_raw, });
                        }

                        Some(quote! {
                            pub const #variant_name: Self = #value;
                        })
                    });

                    let variant_origin = variant_origin.as_ref().unwrap_or(enum_origin);
                    let impl_doc = enum_impl_doc(&variant_origin, enum_origin);

                    quote! {
                        #[doc = #impl_doc]
                        impl #name_ident {
                            #( #variants )*
                        }
                    }
                })
                .collect();

            quote! {
                #[doc = #doc]
                #[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
                #[repr(transparent)]
                pub struct #name_ident(pub i32);

                #( #impls )*

                impl std::fmt::Debug for #name_ident {
                    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                        fmt.write_str(match self {
                            #( #debug_arms )*
                            _ => "Unknown enum variant",
                        })
                    }
                }
            }
        }
        TypeItem::Bitmask {
            flags,
            flag_bits,
            variants: variants_raw,
        } => {
            let flags_name_ident = utils::safe_ident(utils::trim_vk_prefix(flags));
            let flag_bits_name_ident = utils::safe_ident(utils::trim_vk_prefix(flag_bits));

            let flag_bits_type_origin = itemname_origin_map.get(flag_bits).unwrap();
            let mut generated_variants = HashSet::new();
            let variants_processed: Vec<_> = variants_raw
                .iter()
                .sorted()
                .map(|(origin, variants)| {
                    let origin = origin.as_ref().unwrap_or(flag_bits_type_origin);
                    let variants: Vec<_> = variants
                        .iter()
                        .filter_map(|variant| {
                            let name_raw = enum_trim_variant(&variant, &flag_bits);

                            if !generated_variants.insert(name_raw.clone()) {
                                return None;
                            }

                            let variant_name = utils::safe_ident(&name_raw);
                            let value = value_code(flag_bits, variant)?;

                            Some((variant_name, value))
                        })
                        .collect();

                    (origin, variants)
                })
                .collect();

            let flags_variants: Vec<_> = variants_processed
                .iter()
                .flat_map(|(_origin, variants)| {
                    variants.iter().map(|(name, _value)| {
                        quote! {
                            const #name = #flag_bits_name_ident::#name.0;
                        }
                    })
                })
                .collect();

            let flags_doc = utils::doc(
                &format!(
                    "Flags of [`{flag_bits}`](struct.{flag_bits}.html)",
                    flag_bits = flag_bits_name_ident
                ),
                Some(&utils::man_doc(vulkan_version, &flags)),
            );

            let mut debug_arms = Vec::new();
            let mut used_values = HashSet::new();
            let flag_bits_variants: Vec<_> = variants_processed
                .iter()
                .map(|(origin, variants)| {
                    let flag_bits_impl_doc = enum_impl_doc(&origin, flag_bits_type_origin);
                    let variants = variants.iter().map(|(name, value)| {
                        let name_string = name.to_string();

                        let value_string = value.to_string();
                        if !value_string.contains("::") && used_values.insert(value_string) {
                            debug_arms.push(quote! { &Self::#name => #name_string, });
                        }

                        quote! {
                            pub const #name: Self = #value;
                        }
                    });

                    quote! {
                        #[doc = #flag_bits_impl_doc]
                        impl #flag_bits_name_ident {
                            #( #variants )*
                        }
                    }
                })
                .collect();

            let flag_bits_man_doc = utils::man_doc(vulkan_version, &flag_bits);
            let flag_bits_doc = utils::doc(
                &format!(
                    "Flag Bits of [`{flags}`](struct.{flags}.html)",
                    flags = flags_name_ident
                ),
                if flags_variants.is_empty() {
                    None
                } else {
                    Some(&flag_bits_man_doc)
                },
            );

            quote! {
                #[doc = #flag_bits_doc]
                #[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
                #[repr(transparent)]
                pub struct #flag_bits_name_ident(pub u32);

                impl #flag_bits_name_ident {
                    #[inline]
                    #[doc = "Converts this enum variant to the corresponding bitmask"]
                    pub const fn bitmask(&self) -> #flags_name_ident {
                        #flags_name_ident::from_bits_truncate(self.0)
                    }
                }

                #( #flag_bits_variants )*

                impl std::fmt::Debug for #flag_bits_name_ident {
                    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                        fmt.write_str(match self {
                            #( #debug_arms )*
                            _ => "Unknown enum variant",
                        })
                    }
                }

                bitflags::bitflags! {
                    #[doc = #flags_doc]
                    #[derive(Default)]
                    #[repr(transparent)]
                    pub struct #flags_name_ident: u32 {
                        #[cfg(empty_bitflag_workaround)]
                        const EMPTY_BITFLAG_WORKAROUND = 0;

                        #( #flags_variants )*
                    }
                }
            }
        }
        TypeItem::Alias { name, to } => {
            let man_doc = utils::man_doc(vulkan_version, &name);
            let name_ident = utils::safe_ident(utils::trim_vk_prefix(&name));
            let doc = utils::doc("Alias", Some(&man_doc));
            let to_type: TokenStream =
                syn::parse_str(&utils::item_path(itemname_origin_map, to)).unwrap();

            quote! {
                #[doc = #doc]
                pub type #name_ident = #to_type;
            }
        }
        TypeItem::ApplicationDefinedFunction { name, code } => {
            let captures = ADF_REGEX.captures(&code).expect(code);
            let return_type = &captures[1];
            let returns_pointer = captures.get(2).is_some();
            let regex_name = &captures[3];
            assert_eq!(regex_name, name);

            let man_doc = utils::man_doc(vulkan_version, &name);
            let doc = utils::doc("Application defined function", Some(&man_doc));

            let name_ident = utils::safe_ident(&name);

            let return_code: TokenStream = syn::parse_str(&format!(
                "{}{}",
                if returns_pointer { "*const " } else { "" },
                utils::item_path(itemname_origin_map, return_type)
            ))
            .unwrap();

            let arguments = code[captures.get(0).unwrap().end()..]
                .split(',')
                .filter_map(|argument| {
                    let parsed = utils::parse_c_decl(argument.trim_start_matches("\n"))?;
                    Some(utils::convert_c_decl(itemname_origin_map, &parsed))
                })
                .map(|(_, _, _, _, code)| code);

            quote! {
                #[doc = #doc]
                #[allow(non_camel_case_types)]
                pub type #name_ident = unsafe extern "system" fn(#( #arguments )*) -> #return_code;
            }
        }
    }
}
