use crate::{
    comment_gen::DocCommentGen,
    header::{
        eval::{Expression, Literal},
        ValueDependencies,
    },
    items::aliases::Alias,
    name::{EnumVariantName, Name, TypeName},
    origin::Origin,
    source::{NotApplicable, Source},
    XmlNode,
};
use indexmap::IndexMap;
use lang_c::{
    ast::{
        Declaration as CDeclaration, DeclarationSpecifier, DeclaratorKind, Enumerator,
        Expression as CExpression, InitDeclarator, Initializer, StorageClassSpecifier,
        TypeQualifier, TypeSpecifier,
    },
    span::Node,
};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub enum FlagBitWidth {
    Bitwidth32,
    Bitwidth64,
    Unknown,
}

impl FlagBitWidth {
    fn bitbase(&self) -> TokenStream {
        match self {
            FlagBitWidth::Bitwidth32 => quote! { u32 },
            FlagBitWidth::Bitwidth64 => quote! { u64 },
            FlagBitWidth::Unknown => panic!("Tried to get bitbase of unknown bitwidth"),
        }
    }
}

impl PartialEq for FlagBitWidth {
    fn eq(&self, other: &Self) -> bool {
        matches!(self, FlagBitWidth::Unknown)
            || matches!(other, FlagBitWidth::Unknown)
            || matches!(
                (self, other),
                (FlagBitWidth::Bitwidth32, FlagBitWidth::Bitwidth32)
                    | (FlagBitWidth::Bitwidth64, FlagBitWidth::Bitwidth64)
            )
    }
}

#[derive(Debug, PartialEq)]
pub enum EnumKind {
    Enum {
        name: TypeName,
    },
    Bitflag {
        flags_name: TypeName,
        flagbits_name: TypeName,
        bitwidth: FlagBitWidth,
    },
}

impl EnumKind {
    fn enum_ident(&self) -> Ident {
        match self {
            EnumKind::Enum { name } => name.ident(),
            EnumKind::Bitflag { flagbits_name, .. } => flagbits_name.ident(),
        }
    }

    pub fn guess_from_name(name: &str) -> Self {
        if name.contains("FlagBits") {
            EnumKind::from_flagbits_name(name, FlagBitWidth::Unknown)
        } else {
            EnumKind::from_enum_name(name)
        }
    }

    pub fn from_enum_name(enum_name: &str) -> Self {
        EnumKind::Enum {
            name: TypeName::new(enum_name),
        }
    }

    pub fn from_flags_name(flags_name: &str, bitwidth: FlagBitWidth) -> Self {
        EnumKind::Bitflag {
            flags_name: TypeName::new(flags_name),
            flagbits_name: TypeName::new(&flags_name.replace("Flags", "FlagBits")),
            bitwidth,
        }
    }

    pub fn from_flagbits_name(flagbits_name: &str, bitwidth: FlagBitWidth) -> Self {
        EnumKind::Bitflag {
            flags_name: TypeName::new(&flagbits_name.replace("FlagBits", "Flags")),
            flagbits_name: TypeName::new(flagbits_name),
            bitwidth,
        }
    }
}

#[derive(Debug)]
pub enum EnumVariantKind {
    Alias(EnumVariantName),
    Value(Literal),
}

impl EnumVariantKind {
    pub fn from_c(
        expression: &CExpression,
        enum_type_name: &TypeName,
        value_dependencies: &ValueDependencies,
        deprecated_variants: &HashSet<String>,
    ) -> EnumVariantKind {
        match expression {
            CExpression::Identifier(identifier) => {
                let name = match EnumVariantName::new(
                    &identifier.node.name,
                    enum_type_name,
                    deprecated_variants,
                ) {
                    Ok(name) => name,
                    Err(_) => panic!(
                        "Enum variant name is not applicable: {:?}",
                        &identifier.node.name
                    ),
                };

                EnumVariantKind::Alias(name)
            }
            value => EnumVariantKind::Value(
                Expression::from_c(value, value_dependencies).eval_to_literal(),
            ),
        }
    }

    fn value(&self) -> TokenStream {
        match self {
            EnumVariantKind::Alias(to) => {
                let ident = format_ident!("{}", *to.variant);
                quote! {
                    Self::#ident
                }
            }
            EnumVariantKind::Value(literal) => {
                let value = literal.value();
                quote! {
                    Self(#value)
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct EnumVariant {
    pub origin: Option<Origin>,
    pub name: EnumVariantName,
    pub kind: EnumVariantKind,
}

impl EnumVariant {
    pub fn from_enumerator(
        enumerator: &Enumerator,
        enum_type_name: &TypeName,
        value_dependencies: &ValueDependencies,
        deprecated_variants: &HashSet<String>,
    ) -> Result<EnumVariant, NotApplicable> {
        let name = &enumerator.identifier.node.name;
        match &enumerator.expression {
            Some(expression) => Ok(EnumVariant {
                origin: Default::default(),
                name: EnumVariantName::new(name, enum_type_name, deprecated_variants)?,
                kind: EnumVariantKind::from_c(
                    &expression.node,
                    enum_type_name,
                    value_dependencies,
                    deprecated_variants,
                ),
            }),
            None => panic!("Enumerator has no expression: {:?}", enumerator),
        }
    }

    pub fn from_init_declarator(
        init_declarator: &InitDeclarator,
        enum_type_name: &TypeName,
        value_dependencies: &ValueDependencies,
        deprecated_variants: &HashSet<String>,
    ) -> Result<EnumVariant, NotApplicable> {
        let name = match &init_declarator.declarator.node.kind.node {
            DeclaratorKind::Identifier(ident) => &ident.node.name,
            _ => panic!("Declarator is not an identfier"),
        };

        match &init_declarator.initializer {
            Some(initializer) => match &initializer.node {
                Initializer::Expression(expression) => Ok(EnumVariant {
                    origin: Default::default(),
                    name: EnumVariantName::new(name, enum_type_name, deprecated_variants)?,
                    kind: EnumVariantKind::from_c(
                        &expression.node,
                        enum_type_name,
                        value_dependencies,
                        deprecated_variants,
                    ),
                }),
                _ => panic!("Initializer is not an expression"),
            },
            _ => panic!("Missing initializer"),
        }
    }

    pub fn all_from_c(
        declaration: &CDeclaration,
        value_dependencies: &ValueDependencies,
        deprecated_variants: &HashSet<String>,
    ) -> Result<Vec<EnumVariant>, NotApplicable> {
        let mut variants = Vec::new();

        if let [static_, const_, ident_specifier] = declaration.specifiers.as_slice() {
            if matches!(
                static_.node,
                DeclarationSpecifier::StorageClass(Node {
                    node: StorageClassSpecifier::Static,
                    ..
                })
            ) && matches!(
                const_.node,
                DeclarationSpecifier::TypeQualifier(Node {
                    node: TypeQualifier::Const,
                    ..
                })
            ) {
                let init_declarator = match declaration.declarators.as_slice() {
                    [init_declarator] => &init_declarator.node,
                    _ => panic!("Wrong amount of init declarators"),
                };

                let enum_type_name = match &ident_specifier.node {
                    DeclarationSpecifier::TypeSpecifier(ty) => match &ty.node {
                        TypeSpecifier::TypedefName(identifier) => {
                            // the typedef uses Flags instead of the expected FlagBits
                            let flagbits_name = identifier.node.name.replace("Flags", "FlagBits");
                            TypeName::new(&flagbits_name)
                        }
                        _ => panic!("Type specifier is not a typedef name"),
                    },
                    _ => panic!("Expected type specifier"),
                };

                let variant = EnumVariant::from_init_declarator(
                    init_declarator,
                    &enum_type_name,
                    value_dependencies,
                    deprecated_variants,
                );

                if let Ok(v) = variant {
                    if !variants.contains(&v) {
                        variants.push(v);
                    }
                }
            }
        }

        for specifier in &declaration.specifiers {
            if let DeclarationSpecifier::TypeSpecifier(ty) = &specifier.node {
                if let TypeSpecifier::Enum(enum_type) = &ty.node {
                    let enum_type_name = match &enum_type.node.identifier {
                        Some(identifier) => TypeName::new(&identifier.node.name),
                        None => panic!("Enum type has no identifier: {:?}", enum_type),
                    };

                    for enumerator in &enum_type.node.enumerators {
                        let variant = EnumVariant::from_enumerator(
                            &enumerator.node,
                            &enum_type_name,
                            value_dependencies,
                            deprecated_variants,
                        );
                        if let Ok(v) = variant {
                            if !variants.contains(&v) {
                                variants.push(v);
                            }
                        }
                    }
                }
            }
        }

        if variants.is_empty() {
            Err(NotApplicable)
        } else {
            Ok(variants)
        }
    }
}

impl PartialEq for EnumVariant {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

#[derive(Debug)]
pub struct Enum {
    pub origin: Option<Origin>,
    pub kind: EnumKind,
    pub variants: Vec<EnumVariant>,
}

impl Enum {
    pub fn tokens(
        &self,
        comment_gen: &DocCommentGen,
        deprecated_variants: &HashSet<String>,
    ) -> HashMap<Origin, TokenStream> {
        let enum_origin = self.origin.as_ref().expect("Enum missing origin");

        let mut stream = match &self.kind {
            EnumKind::Enum { name } => {
                let ident = name.ident();
                let doc_alias = name.doc_alias();
                let doc = comment_gen.def(Some(&name.original), "Enum", self.origin.as_ref());

                quote! {
                    #[doc = #doc]
                    #doc_alias
                    #[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
                    #[repr(transparent)]
                    pub struct #ident(pub i32);
                }
            }
            EnumKind::Bitflag {
                flags_name,
                flagbits_name,
                bitwidth,
            } => {
                let bitbase = bitwidth.bitbase();

                let flags_ident = flags_name.ident();
                let flags_doc_alias = flags_name.doc_alias();
                let flagbits_ident = flagbits_name.ident();
                let flagbits_doc_alias = flagbits_name.doc_alias();

                let flags_doc = comment_gen.def(
                    Some(&flags_name.original),
                    format!("Bitmask of [`{}`]", flagbits_ident),
                    self.origin.as_ref(),
                );

                let flagbits_doc = comment_gen.def(
                    (!self.variants.is_empty()).then(|| &*flagbits_name.original),
                    format!("Bits enum of [`{}`]", flags_ident),
                    self.origin.as_ref(),
                );

                let flagbits_deprecated = self.variants.iter().map(|variant| {
                    deprecated_variants
                        .contains(&*variant.name.original)
                        .then(|| {
                            quote! {
                                    #[allow(deprecated)]
                                    #[deprecated]
                            }
                        })
                });
                let flagbits_variants = self.variants.iter().map(|variant| variant.name.ident());
                let empty_bitflag_workaround = self.variants.is_empty().then(|| {
                    quote! {
                        #[cfg(empty_bitflag_workaround)]
                        const EMPTY_BITFLAG_WORKAROUND = 0;
                    }
                });

                quote! {
                    bitflags::bitflags! {
                        #[doc = #flags_doc]
                        #flags_doc_alias
                        #[derive(Default)]
                        #[repr(transparent)]
                        pub struct #flags_ident: #bitbase {
                            #empty_bitflag_workaround
                            #(
                                #flagbits_deprecated
                                const #flagbits_variants = #flagbits_ident::#flagbits_variants.0;
                            )*
                        }
                    }

                    #[doc = #flagbits_doc]
                    #flagbits_doc_alias
                    #[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
                    #[repr(transparent)]
                    pub struct #flagbits_ident(pub #bitbase);

                    impl #flagbits_ident {
                        #[inline]
                        #[doc = "Converts this enum variant to the corresponding bitmask"]
                        pub const fn bitmask(&self) -> #flags_ident {
                            #flags_ident::from_bits_truncate(self.0)
                        }
                    }
                }
            }
        };

        stream.extend(self.debug_impl());

        let mut tokens = HashMap::new();
        tokens.insert(enum_origin.clone(), stream);

        let mut variant_map = IndexMap::new();
        for variant in &self.variants {
            variant_map
                .entry(variant.origin.as_ref().or(Some(enum_origin)).unwrap())
                .or_insert_with(Vec::new)
                .push(variant);
        }

        let enum_ident = self.kind.enum_ident();
        for (origin, variants) in variant_map {
            let enum_path = enum_origin.module_path();
            let variant_deprecated = variants.iter().map(|variant| {
                deprecated_variants
                    .contains(&*variant.name.original)
                    .then(|| quote! { #[deprecated] })
            });
            let variant_idents = variants.iter().map(|variant| variant.name.ident());
            let variant_values = variants.iter().map(|variant| variant.kind.value());
            let doc = comment_gen.provided_by(origin);

            tokens
                .entry(origin.clone())
                .or_insert_with(TokenStream::new)
                .extend(quote! {
                    #[doc = #doc]
                    impl crate:: #enum_path #enum_ident {
                        #(
                            #variant_deprecated
                            pub const #variant_idents: Self = #variant_values;
                        )*
                    }
                });
        }

        tokens
    }

    fn debug_impl(&self) -> TokenStream {
        let enum_ident = self.kind.enum_ident();
        let variants: Vec<_> = self
            .variants
            .iter()
            .filter(|variant| matches!(variant.kind, EnumVariantKind::Value(_)))
            .collect();
        let variant_idents = variants.iter().map(|variant| variant.name.ident());
        let variants_unsanitized = variants.iter().map(|variant| &variant.name.unsanitized);

        quote! {
            impl std::fmt::Debug for #enum_ident {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    f.write_str(match self {
                        #(&Self::#variant_idents => #variants_unsanitized,)*
                        _ => "(unknown variant)",
                    })
                }
            }
        }
    }
}

impl Source {
    pub fn collect_enum_type(&mut self, node: XmlNode) {
        if !node.has_attribute("requires") {
            match (node.attribute("name"), node.attribute("alias")) {
                (Some(name), Some(alias)) => self.aliases.push(Alias::new(
                    Name::Type(TypeName::new(name)),
                    Name::Type(TypeName::new(alias)),
                )),
                (name_attribute, _) => {
                    let name = node
                        .children()
                        .find(|n| n.has_tag_name("name"))
                        .and_then(|n| n.text())
                        .or(name_attribute);

                    let kind = match name {
                        Some(name) => match node.attribute("category") {
                            Some("bitmask") => {
                                let type_child = node
                                    .children()
                                    .find(|c| c.has_tag_name("type"))
                                    .expect("Expected type child")
                                    .text()
                                    .expect("type child missing text");
                                let bitwidth = match type_child {
                                    "VkFlags" => FlagBitWidth::Bitwidth32,
                                    "VkFlags64" => FlagBitWidth::Bitwidth64,
                                    other => panic!("Unexpected bitmask type: {:?}", other),
                                };

                                EnumKind::from_flags_name(name, bitwidth)
                            }
                            Some("enum") => EnumKind::guess_from_name(name),
                            invalid => {
                                panic!("Invalid enum type category: {:?} from {:?}", invalid, node)
                            }
                        },
                        _ => panic!("Enum type has no name: {:?}", node),
                    };

                    if !self.enums.iter().any(|en| en.kind == kind) {
                        self.enums.push(Enum {
                            origin: Default::default(),
                            kind,
                            variants: Vec::new(),
                        })
                    }
                }
            }
        }
    }

    pub fn collect_enum(&mut self, node: XmlNode) {
        let name = match node.attribute("name") {
            Some(name) => name,
            None => panic!("Enum has no name: {:?}", node),
        };

        let kind = match node.attribute("type") {
            Some("bitmask") => {
                let bitwidth = match node.attribute("bitwidth") {
                    Some("64") => FlagBitWidth::Bitwidth64,
                    None => FlagBitWidth::Bitwidth32,
                    other => panic!("Unknown bitwidth: {:?}", other),
                };

                EnumKind::from_flagbits_name(name, bitwidth)
            }
            Some("enum") => EnumKind::from_enum_name(name),
            None => return,
            unknown => panic!("Unknown enum type: {:?} from {:?}", unknown, node),
        };

        let existing_enum = self.enums.iter_mut().find(|en| en.kind == kind);

        let variants = self.header.take_enum_variants(&kind);
        let new_enum = Enum {
            origin: Default::default(),
            kind,
            variants,
        };

        if let Some(existing_enum) = existing_enum {
            *existing_enum = new_enum;
        } else {
            self.enums.push(new_enum);
        }
    }
}
