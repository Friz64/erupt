mod builders;

use crate::{
    comment_gen::DocCommentGen,
    declaration::{Declaration, Mutability, Type},
    header::{
        eval::{Expression, Literal},
        BitWidth, DeclarationInfo,
    },
    items::aliases::Alias,
    name::{Name, TypeName},
    origin::Origin,
    source::{NotApplicable, Source},
    XmlNode,
};
use lang_c::ast::{
    Declaration as CDeclaration, DeclarationSpecifier, StructDeclaration, StructField, StructKind,
    TypeSpecifier,
};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::{cell::RefCell, cmp::Ordering, collections::HashMap, convert::TryFrom};

impl<'a> From<&'a StructField> for DeclarationInfo<'a> {
    fn from(field: &'a StructField) -> Self {
        let specifiers = field.specifiers.as_slice();

        assert_eq!(field.declarators.len(), 1);
        let struct_declarator = &field.declarators[0].node;

        let declarator = struct_declarator
            .declarator
            .as_ref()
            .map(|declarator| &declarator.node);

        let bitwidth = match struct_declarator.bit_width.as_ref() {
            Some(bitwidth) => {
                let bitwidth = match Expression::from(&bitwidth.node).eval_to_literal() {
                    Literal::Int32(val) => val,
                    unexpected => panic!("Unexpected bit-width type: {:?}", unexpected),
                };

                BitWidth::Partial(bitwidth as _)
            }
            None => BitWidth::Full,
        };

        DeclarationInfo {
            type_info: specifiers.into(),
            declarator,
            bitwidth,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum StructureKind {
    Struct,
    Union,
}

impl StructureKind {
    fn keyword(&self) -> TokenStream {
        match self {
            StructureKind::Struct => quote! { struct },
            StructureKind::Union => quote! { union },
        }
    }
}

impl From<StructKind> for StructureKind {
    fn from(struct_kind: StructKind) -> Self {
        match struct_kind {
            StructKind::Struct => StructureKind::Struct,
            StructKind::Union => StructureKind::Union,
        }
    }
}

#[derive(Debug)]
pub struct StructureMetadata {
    /// List of other structs this struct can extend
    pub extends: Vec<TypeName>,
}

impl StructureMetadata {
    pub fn empty() -> StructureMetadata {
        StructureMetadata {
            extends: Vec::new(),
        }
    }
}

impl From<XmlNode<'_, '_>> for StructureMetadata {
    fn from(node: XmlNode) -> Self {
        let extends = node
            .attribute("structextends")
            .map(|values| values.split(',').map(|s| TypeName::new(s)).collect())
            .unwrap_or_default();

        StructureMetadata { extends }
    }
}

#[derive(Debug)]
pub enum StructureField {
    Normal(Declaration),
    Bitfield(Vec<Declaration>),
}

impl StructureField {
    fn distribute_field_decls(
        field_decls: impl Iterator<Item = Declaration>,
    ) -> Vec<StructureField> {
        let mut fields = Vec::new();
        let mut required_bitwidth = 0;
        let mut bitwidth_sum = 0;
        let mut finished = false;
        for field_decl in field_decls {
            match field_decl.bitwidth {
                BitWidth::Full => fields.push(StructureField::Normal(field_decl)),
                BitWidth::Partial(bitwidth) => match fields.last_mut() {
                    Some(StructureField::Bitfield(v)) if !finished => {
                        bitwidth_sum += bitwidth;
                        match bitwidth_sum.cmp(&required_bitwidth) {
                            Ordering::Less => v.push(field_decl),
                            Ordering::Equal => {
                                v.push(field_decl);
                                bitwidth_sum = 0;
                                finished = true;
                            }
                            Ordering::Greater => panic!(
                                "Unmatched bitwidth ({} > {})",
                                bitwidth_sum, required_bitwidth
                            ),
                        }
                    }
                    _ => {
                        finished = false;
                        bitwidth_sum = bitwidth;
                        required_bitwidth = field_decl.ty.bitwidth();
                        fields.push(StructureField::Bitfield(vec![field_decl]));
                    }
                },
            }
        }

        fields
    }

    pub fn ident(&self) -> Ident {
        let string = match self {
            StructureField::Normal(decl) => decl.ident().to_string(),
            StructureField::Bitfield(decls) => {
                assert!(!decls.is_empty());
                let mut idents = decls.iter().map(|decl| decl.ident().to_string());
                match (idents.next(), idents.next(), idents.next()) {
                    (Some(ident), None, None) => ident,
                    (Some(ident1), Some(ident2), None) => format!("{}_and_{}", ident1, ident2),
                    (Some(ident), _, _) => format!("{}_and_more_bitfield", ident),
                    _ => unreachable!(),
                }
            }
        };

        format_ident!("{}", string)
    }

    pub fn debug_impl(&self) -> TokenStream {
        let ident = self.ident();
        let field = quote! { self.#ident };

        match self {
            StructureField::Normal(_) => match &self.main_decl().ty {
                Type::Array { of, .. } if matches!(**of, Type::Char) => quote! {
                    unsafe { &std::ffi::CStr::from_ptr(#field.as_ptr()) }
                },
                Type::Option(of) if matches!(**of, Type::Named(Name::Function(_))) => quote! {
                    unsafe { &std::mem::transmute::<_, *const ()>(#field) }
                },
                Type::Named(Name::Type(name)) if *name == TypeName::bool32() => {
                    quote! { &(#field != 0) }
                }
                _ => quote! { &#field },
            },
            StructureField::Bitfield(_) => {
                quote! { &format!("{:#b}", &#field) }
            }
        }
    }

    pub fn main_decl(&self) -> &Declaration {
        match self {
            StructureField::Normal(decl) => decl,
            StructureField::Bitfield(decls) => &decls[0],
        }
    }
}

#[derive(Debug)]
pub struct Structure {
    pub origin: Option<Origin>,
    pub name: TypeName,
    pub kind: StructureKind,
    pub fields: Vec<StructureField>,
    pub metadata: StructureMetadata,
    // caches the result of `::supports_hash_eq()`
    supports_hash_eq: RefCell<Option<bool>>,
}

impl Structure {
    pub fn origin(&self) -> &Origin {
        self.origin.as_ref().expect("Structure missing origin")
    }

    pub fn qualifies_as_builder(&self) -> bool {
        self.kind == StructureKind::Struct
    }

    pub fn structure_type_value(&self, source: &Source) -> Option<TokenStream> {
        self.fields
            .get(0)
            .and_then(|field| field.main_decl().structure_type_value(source))
    }

    pub fn tokens(
        &self,
        comment_gen: &DocCommentGen,
        source: &Source,
    ) -> HashMap<Origin, TokenStream> {
        let origin = self.origin();
        let ident = self.name.ident();

        let doc_alias = self.name.doc_alias();

        let keyword = self.kind.keyword();
        let doc = comment_gen.def(Some(&self.name.original), "Structure", self.origin.as_ref());

        let field_idents: Vec<_> = self.fields.iter().map(|field| field.ident()).collect();
        let field_types = self
            .fields
            .iter()
            .map(|field| field.main_decl().ty.rust_type(source));

        let structure_type = self.structure_type_value(source).map(|value| {
            quote! {
                impl #ident {
                    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = #value;
                }
            }
        });

        let default_impl = match self.kind {
            StructureKind::Struct => {
                let field_defaults = self
                    .fields
                    .iter()
                    .map(|field| field.main_decl().default_impl(source));
                quote! {
                    Self {
                        #(#field_idents: #field_defaults),*
                    }
                }
            }
            StructureKind::Union => quote! {
                unsafe { std::mem::zeroed() }
            },
        };

        let debug_name = ident.to_string();
        let debug_impl = match self.kind {
            StructureKind::Struct => {
                let field_names = self.fields.iter().map(|field| field.ident().to_string());
                let field_debugs = self.fields.iter().map(|field| field.debug_impl());
                quote! {
                    f.debug_struct(#debug_name)
                        #(.field(#field_names, #field_debugs))*
                        .finish()
                }
            }
            StructureKind::Union => quote! {
                f.debug_struct(#debug_name).finish()
            },
        };

        let hash_eq_derive = self
            .supports_hash_eq(source)
            .then(|| quote! { Hash, PartialEq, Eq, });

        let mut tokens = HashMap::new();
        tokens.insert(
            origin.clone(),
            quote! {
                #[doc = #doc]
                #doc_alias
                #[derive(Copy, Clone, #hash_eq_derive)]
                #[repr(C)]
                pub #keyword #ident {
                    #(pub #field_idents: #field_types),*
                }

                #structure_type

                impl Default for #ident {
                    fn default() -> Self {
                        #default_impl
                    }
                }

                impl std::fmt::Debug for #ident {
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                        #debug_impl
                    }
                }
            },
        );

        for (origin, tokens_inner) in self.builder(source, comment_gen) {
            tokens
                .entry(origin)
                .or_insert_with(TokenStream::new)
                .extend(tokens_inner);
        }

        tokens
    }

    pub fn has_p_next(&self, kind: Mutability) -> bool {
        match self.fields.get(1) {
            Some(StructureField::Normal(decl)) => decl.is_p_next(kind),
            _ => false,
        }
    }

    pub fn supports_hash_eq(&self, source: &Source) -> bool {
        *(self.supports_hash_eq.borrow_mut().get_or_insert_with(|| {
            self.kind == StructureKind::Struct
                && self.fields.iter().all(|field| match field {
                    StructureField::Normal(field) => field.ty.supports_hash_eq(source),
                    StructureField::Bitfield(fields) => {
                        fields.iter().all(|field| field.ty.supports_hash_eq(source))
                    }
                })
        }))
    }
}

impl TryFrom<&CDeclaration> for Structure {
    type Error = NotApplicable;

    fn try_from(declaration: &CDeclaration) -> Result<Self, Self::Error> {
        let mut result = Err(NotApplicable);

        for specifier in &declaration.specifiers {
            if let DeclarationSpecifier::TypeSpecifier(ty) = &specifier.node {
                if let TypeSpecifier::Struct(struct_type) = &ty.node {
                    if let (Some(identifier), Some(declarations)) = (
                        struct_type.node.identifier.as_ref(),
                        struct_type.node.declarations.as_ref(),
                    ) {
                        let field_decls = declarations.iter().filter_map(|decl| match &decl.node {
                            StructDeclaration::Field(field) => Some(Declaration::from(&field.node)),
                            _ => None,
                        });

                        assert!(result.is_err());
                        result = Ok(Structure {
                            origin: Default::default(),
                            name: TypeName::new(&identifier.node.name),
                            kind: struct_type.node.kind.node.clone().into(),
                            fields: StructureField::distribute_field_decls(field_decls),
                            metadata: StructureMetadata::empty(),
                            supports_hash_eq: RefCell::new(None),
                        });
                    }
                }
            }
        }

        result
    }
}

impl Source {
    pub fn collect_structure(&mut self, node: XmlNode) {
        let name = match node.attribute("name") {
            Some(name) => name,
            None => panic!("Structure has no name: {:?}", node),
        };

        match node.attribute("alias") {
            Some(alias) => self.aliases.push(Alias::new(
                Name::Type(TypeName::new(name)),
                Name::Type(TypeName::new(alias)),
            )),
            None => {
                if let Some(mut structure) = self.header.take_structure(name) {
                    structure.metadata = node.into();

                    // assign metadata to every declaration in order (respecting bitfields)
                    let mut i = 0;
                    let mut inner_i = 0;
                    for structure_child in node.children() {
                        if structure_child.has_tag_name("member") {
                            let field_metadata = structure_child.into();
                            match &mut structure.fields[i] {
                                StructureField::Normal(decl) => {
                                    decl.metadata = field_metadata;
                                    i += 1;
                                }
                                StructureField::Bitfield(decls) => {
                                    decls[inner_i].metadata = field_metadata;
                                    inner_i += 1;
                                    if inner_i >= decls.len() {
                                        inner_i = 0;
                                        i += 1;
                                    }
                                }
                            }
                        }
                    }

                    self.structures.push(structure);
                }
            }
        }
    }
}
