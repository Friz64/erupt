use crate::{
    items::enums::EnumKind,
    source::{NotApplicable, Source},
};
use heck::{ToShoutySnakeCase, ToSnakeCase};
use log::warn;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::{
    collections::HashSet,
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
        quote! { crate:: #path #ident }
    }

    pub fn supports_hash_eq(&self, source: &Source) -> bool {
        match self {
            Name::Type(type_name) => {
                if let Some(alias) = source.find_type_alias(type_name) {
                    alias.resolve(source).supports_hash_eq(source)
                } else if let Some(structure) = source.find_structure(type_name) {
                    structure.supports_hash_eq(source)
                } else if source.find_enum(type_name).is_some()
                    || source.find_handle(type_name).is_some()
                {
                    true
                } else if let Some(basetype) = source.find_basetype(type_name) {
                    basetype.ty.supports_hash_eq(source)
                } else {
                    panic!("Unknown origin for type name {:?}", type_name)
                }
            }
            Name::Function(_) => false,
        }
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
        let boundary = trimmed
            .rfind(|c: char| c.is_lowercase() || c.is_ascii_digit())
            .unwrap_or(0);
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

    pub fn doc_alias(&self) -> Option<TokenStream> {
        let ident = self.ident();
        let doc_alias = &self.original;

        (ident != **doc_alias).then(|| quote! { #[doc(alias = #doc_alias)] })
    }

    pub fn is_flags(&self) -> bool {
        self.no_tag.ends_with("Flags")
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

    pub fn physical_device_features() -> TypeName {
        TypeName::new("VkPhysicalDeviceFeatures")
    }

    pub fn physical_device_features2() -> TypeName {
        TypeName::new("VkPhysicalDeviceFeatures2")
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
    pub unsanitized: Box<str>,
    /// Just the enum variant, sanitized to be a valid Rust identifier
    pub variant: Box<str>,
    /// True if the enum variant properly contains the enum type
    pub prefix_compliant: bool,
}

impl EnumVariantName {
    pub fn new(
        src: &str,
        enum_type: &TypeName,
        deprecated_variants: &HashSet<String>,
    ) -> Result<EnumVariantName, NotApplicable> {
        let mut trimmed = src.trim_start_matches("VK_").to_uppercase();
        trim_trailing_bit_part(&mut trimmed);

        let mut enum_type_prefix = enum_type
            .no_tag
            .replace("FlagBits", "")
            .to_shouty_snake_case();

        if enum_type_prefix.ends_with(|c: char| c.is_ascii_digit()) {
            enum_type_prefix.insert(enum_type_prefix.len() - 1, '_');
        }

        let prefix_compliant = trimmed.starts_with(&enum_type_prefix);

        let prefix_compliance_warning_exempt = [TypeName::result()];
        if !prefix_compliant
            && !deprecated_variants.contains(src)
            && !prefix_compliance_warning_exempt.contains(enum_type)
        {
            warn!("{:?} (from {:?}) is not prefix compliant", src, enum_type);
        }

        let unsanitized = trimmed
            .trim_start_matches(&enum_type_prefix)
            .trim_start_matches('_')
            .to_uppercase();
        assert_ne!(unsanitized.len(), 0);

        if unsanitized.contains("MAX_ENUM") || unsanitized.contains("FLAGS_MAX_ENUM") {
            return Err(NotApplicable);
        }

        let mut variant = unsanitized.clone();
        if variant.chars().next().unwrap().is_ascii_digit() {
            variant.insert(0, '_');
        }

        Ok(EnumVariantName {
            enum_type: enum_type.clone(),
            original: src.into(),
            trimmed: trimmed.into(),
            unsanitized: unsanitized.into(),
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

// invalid:
// VK_VIDEO_COMPONENT_BIT_DEPTH_INVALID_KHR
//                   ^^^^ - last bit str
//                                     ^ - last underscore

// valid:
// VK_VIDEO_COMPONENT_BIT_DEPTH_8_BIT_KHR
//                               ^^^^ - last bit str
//                                   ^ - last underscore
// VK_ACCESS_INDIRECT_COMMAND_READ_BIT
//                                ^^^^ - last bit str
//                                ^ - last underscore
fn trim_trailing_bit_part(trimmed: &mut String) {
    if let Some((last_bit_idx, bit_str)) = trimmed.match_indices("_BIT").last() {
        let (last_underscore_idx, _) = trimmed.match_indices('_').last().unwrap();
        let bit_str_len = bit_str.len();
        if last_bit_idx + bit_str_len >= last_underscore_idx {
            trimmed.replace_range(last_bit_idx..last_bit_idx + bit_str_len, "");
        }
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
