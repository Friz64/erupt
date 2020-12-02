mod builders;

use crate::{
    comment_gen::DocCommentGen,
    declaration::Declaration,
    header::DeclarationInfo,
    items::aliases::Alias,
    name::{Name, TypeName},
    origin::Origin,
    source::{NotApplicable, Source},
};
use lang_c::ast::{
    Declaration as CDeclaration, DeclarationSpecifier, StructDeclaration, StructField, StructKind,
    TypeSpecifier,
};
use proc_macro2::TokenStream;
use quote::quote;
use std::convert::TryFrom;
use treexml::Element;

impl<'a> From<&'a StructField> for DeclarationInfo<'a> {
    fn from(field: &'a StructField) -> Self {
        let specifiers = field.specifiers.as_slice();

        assert_eq!(field.declarators.len(), 1);
        let struct_declarator = &field.declarators[0].node;

        let declarator = struct_declarator
            .declarator
            .as_ref()
            .map(|declarator| &declarator.node);

        DeclarationInfo {
            type_info: specifiers.into(),
            declarator,
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

impl From<&Element> for StructureMetadata {
    fn from(element: &Element) -> Self {
        let extends = element
            .attributes
            .get("structextends")
            .map(|values| values.split(',').map(|s| TypeName::new(s)).collect())
            .unwrap_or_default();

        StructureMetadata { extends }
    }
}

#[derive(Debug)]
pub struct Structure {
    pub origin: Option<Origin>,
    pub name: TypeName,
    pub kind: StructureKind,
    pub fields: Vec<Declaration>,
    pub metadata: StructureMetadata,
}

impl Structure {
    pub fn qualifies_as_builder(&self) -> bool {
        self.kind == StructureKind::Struct
    }

    pub fn tokens(&self, comment_gen: &DocCommentGen, source: &Source) -> TokenStream {
        let ident = self.name.ident();
        let doc_alias = &self.name.original;
        let keyword = self.kind.keyword();
        let doc = comment_gen.def(Some(&self.name.original), "Structure", None);

        let field_idents: Vec<_> = self.fields.iter().map(|field| field.ident()).collect();
        let field_types = self.fields.iter().map(|field| field.ty.rust_type(source));

        let default_impl = match self.kind {
            StructureKind::Struct => {
                let field_defaults = self.fields.iter().map(|field| field.default_impl(source));
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

        let builder = self.builder(source, comment_gen);
        quote! {
            #[doc = #doc]
            #[doc(alias = #doc_alias)]
            #[derive(Copy, Clone)]
            #[repr(C)]
            pub #keyword #ident {
                #(pub #field_idents: #field_types),*
            }

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

            #builder
        }
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
                        let fields = declarations
                            .iter()
                            .filter_map(|decl| match &decl.node {
                                StructDeclaration::Field(field) => {
                                    Some(Declaration::from(&field.node))
                                }
                                _ => None,
                            })
                            .collect();

                        assert!(result.is_err());
                        result = Ok(Structure {
                            origin: Default::default(),
                            name: TypeName::new(&identifier.node.name),
                            kind: struct_type.node.kind.node.clone().into(),
                            fields,
                            metadata: StructureMetadata::empty(),
                        });
                    }
                }
            }
        }

        result
    }
}

impl Source {
    pub fn collect_structure(&mut self, element: &Element) {
        let name = match element.attributes.get("name") {
            Some(name) => name,
            None => panic!("Structure has no name: {:?}", element),
        };

        match element.attributes.get("alias") {
            Some(alias) => self.aliases.push(Alias::new(
                Name::Type(TypeName::new(name)),
                Name::Type(TypeName::new(alias)),
            )),
            None => {
                if let Some(mut structure) = self.header.take_structure(name) {
                    structure.metadata = element.into();

                    let mut i = 0;
                    for structure_child in &element.children {
                        if structure_child.name == "member" {
                            structure.fields[i].metadata = structure_child.into();
                            i += 1;
                        }
                    }

                    self.structures.push(structure);
                }
            }
        }
    }
}
