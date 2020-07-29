use crate::{
    items::enums::EnumKind,
    source::{NotApplicable, Source},
};
use heck::{ShoutySnakeCase, SnakeCase};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::{
    fmt::{self, Debug},
    hash::{Hash, Hasher},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Name {
    Function(FunctionName),
    Type(TypeName),
}

impl Name {
    pub fn original(&self) -> &str {
        match self {
            Name::Function(function_name) => &function_name.no_pfn,
            Name::Type(type_name) => &type_name.original,
        }
    }

    pub fn ident(&self) -> Ident {
        match self {
            Name::Function(function_name) => function_name.pointer_ident(),
            Name::Type(type_name) => type_name.ident(),
        }
    }

    pub fn path(&self, source: &Source) -> TokenStream {
        let path = source.origin(self).module_path();
        let ident = self.ident();
        quote! { crate::#path#ident }
    }
}

#[derive(Clone)]
pub struct FunctionName {
    /// Original type name
    pub original: Box<str>,
    /// `original`, with trimmed PFN_ prefix
    pub no_pfn: Box<str>,
    /// `no_pfn`, with trimmed vk prefix
    pub trimmed: Box<str>,
    /// Based on `trimmed`, but without the tag
    pub no_tag: Box<str>,
    /// The tag of the name, eg: KHR
    pub tag: Option<Box<str>>,
}

impl FunctionName {
    pub fn new(src: &str) -> FunctionName {
        let no_pfn = src.trim_start_matches("PFN_");
        let trimmed = no_pfn.trim_start_matches("vk");
        let boundary = trimmed.rfind(char::is_lowercase).unwrap_or(0);
        let (no_tag, tag) = if boundary + 1 == trimmed.len() {
            (trimmed, None)
        } else {
            let (no_tag, tag) = trimmed.split_at(boundary + 1);
            (no_tag, Some(tag))
        };

        let mut original = src.to_string();
        if !original.starts_with("PFN_") {
            original.insert_str(0, "PFN_");
        }

        FunctionName {
            original: original.into(),
            no_pfn: no_pfn.into(),
            trimmed: trimmed.into(),
            no_tag: no_tag.into(),
            tag: tag.map(|s| s.into()),
        }
    }

    pub fn pretty_ident(&self) -> Ident {
        format_ident!("{}", self.trimmed.to_snake_case())
    }

    pub fn pointer_ident(&self) -> Ident {
        format_ident!("{}", *self.original)
    }

    pub fn constant_name(&self) -> String {
        format!("FN_{}", self.trimmed.to_shouty_snake_case())
    }
}

impl PartialEq for FunctionName {
    fn eq(&self, other: &Self) -> bool {
        self.trimmed.eq(&other.trimmed)
    }
}

impl Debug for FunctionName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(&self.trimmed, f)
    }
}

#[derive(Clone, Eq)]
pub struct TypeName {
    /// Original type name
    pub original: Box<str>,
    /// `original`, with trimmed Vk prefix
    pub trimmed: Box<str>,
    /// Based on `trimmed`, but without the tag
    pub no_tag: Box<str>,
    /// The tag of the name, eg: KHR
    pub tag: Option<Box<str>>,
    /// True if this type is a builder
    pub builder: bool,
}

impl TypeName {
    pub fn new(src: &str) -> TypeName {
        let trimmed = src.trim_start_matches("Vk");
        let boundary = trimmed.rfind(char::is_lowercase).unwrap_or(0);
        let (no_tag, tag) = if boundary + 1 == trimmed.len() {
            (trimmed, None)
        } else {
            let (no_tag, tag) = trimmed.split_at(boundary + 1);
            (no_tag, Some(tag))
        };

        TypeName {
            original: src.into(),
            trimmed: trimmed.into(),
            no_tag: no_tag.into(),
            tag: tag.map(|s| s.into()),
            builder: false,
        }
    }

    pub fn set_builder(mut self, builder: bool) -> TypeName {
        self.builder = builder;
        self
    }

    pub fn ident(&self) -> Ident {
        format_ident!(
            "{}{}",
            *self.trimmed,
            if self.builder { "Builder" } else { "" }
        )
    }

    pub fn structure_type() -> TypeName {
        TypeName::new("VkStructureType")
    }

    pub fn result() -> TypeName {
        TypeName::new("VkResult")
    }

    pub fn instance() -> TypeName {
        TypeName::new("VkInstance")
    }

    pub fn device() -> TypeName {
        TypeName::new("VkDevice")
    }

    pub fn bool32() -> TypeName {
        TypeName::new("VkBool32")
    }

    pub fn command_buffer() -> TypeName {
        TypeName::new("VkCommandBuffer")
    }

    pub fn queue() -> TypeName {
        TypeName::new("VkQueue")
    }

    pub fn physical_device() -> TypeName {
        TypeName::new("VkPhysicalDevice")
    }
}

impl PartialEq for TypeName {
    fn eq(&self, other: &Self) -> bool {
        self.trimmed.eq(&other.trimmed)
    }
}

impl Hash for TypeName {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(self.trimmed.as_bytes());
    }
}

impl Debug for TypeName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(&self.trimmed, f)
    }
}

pub struct EnumVariantName {
    /// Original type name of this variant's enum
    pub enum_type: TypeName,
    /// Original variant name
    pub original: Box<str>,
    /// `original`, with trimmed VK_ prefix and removed BIT part
    pub trimmed: Box<str>,
    /// Just the enum variant, without the variant's enum type name
    pub variant: Box<str>,
    /// True if the enum variant properly cointains the enum type
    pub prefix_compliant: bool,
}

impl EnumVariantName {
    pub fn new(src: &str, enum_type: &TypeName) -> Result<EnumVariantName, NotApplicable> {
        let trimmed = src.trim_start_matches("VK_").replace("_BIT", "");

        let enum_type_prefix = enum_type
            .no_tag
            .trim_end_matches("FlagBits")
            .to_shouty_snake_case();
        let prefix_compliant = trimmed.starts_with(&enum_type_prefix);

        let mut variant = trimmed
            .trim_start_matches(&enum_type_prefix)
            .trim_start_matches('_')
            .to_uppercase();

        if variant.starts_with("MAX_ENUM") || variant.starts_with("FLAGS_MAX_ENUM") {
            return Err(NotApplicable);
        }

        assert_ne!(variant.len(), 0);
        if variant.chars().next().unwrap().is_ascii_digit() {
            variant.insert(0, '_');
        }

        Ok(EnumVariantName {
            enum_type: enum_type.clone(),
            original: src.into(),
            trimmed: trimmed.into(),
            variant: variant.into(),
            prefix_compliant,
        })
    }

    pub fn is_from(&self, enum_kind: &EnumKind) -> bool {
        let name = match enum_kind {
            EnumKind::Enum { name } => name,
            EnumKind::Bitflag { flagbits_name, .. } => flagbits_name,
        };

        name == &self.enum_type
    }

    pub fn ident(&self) -> Ident {
        format_ident!("{}", *self.variant)
    }
}

impl Debug for EnumVariantName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(&self.variant, f)
    }
}

impl PartialEq for EnumVariantName {
    fn eq(&self, other: &Self) -> bool {
        self.variant.eq(&other.variant) && self.enum_type.eq(&other.enum_type)
    }
}
