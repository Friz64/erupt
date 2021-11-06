use crate::{
    comment_gen::DocCommentGen,
    declaration::{Declaration, Type},
    header::{BitWidth, DeclarationInfo, ValueDependencies},
    name::TypeName,
    origin::Origin,
    source::{NotApplicable, Source},
    XmlNode,
};
use lang_c::ast::{Declaration as CDeclaration, DeclarationSpecifier, TypeSpecifier};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Basetype {
    pub origin: Option<Origin>,
    pub name: TypeName,
    pub ty: Type,
}

impl Basetype {
    pub fn tokens(
        &self,
        comment_gen: &DocCommentGen,
        source: &Source,
    ) -> HashMap<Origin, TokenStream> {
        let origin = self.origin.as_ref().expect("Basetype missing origin");

        let ident = self.name.ident();
        let doc_alias = self.name.doc_alias();

        let ty = self.ty.rust_type(source);
        let doc = comment_gen.def(Some(&self.name.original), "Basetype", self.origin.as_ref());

        let mut tokens = HashMap::new();
        tokens.insert(
            origin.clone(),
            quote! {
                #[doc = #doc]
                #doc_alias
                pub type #ident = #ty;
            },
        );

        tokens
    }

    pub fn from_c(
        declaration: &CDeclaration,
        value_dependencies: &ValueDependencies,
    ) -> Result<Self, NotApplicable> {
        match declaration.declarators.as_slice() {
            [init_declarator] => {
                let declaration = Declaration::from_decl_info(
                    DeclarationInfo {
                        type_info: declaration.specifiers.as_slice().try_into()?,
                        declarator: Some(&init_declarator.node.declarator.node),
                        bitwidth: BitWidth::Full,
                    },
                    value_dependencies,
                );

                let ty = declaration.ty;
                declaration.name.ok_or(NotApplicable).map(|name| Basetype {
                    origin: Default::default(),
                    name: TypeName::new(&name),
                    ty,
                })
            }
            [] => match declaration.specifiers.as_slice() {
                [specifier] => match &specifier.node {
                    DeclarationSpecifier::TypeSpecifier(ty) => match &ty.node {
                        TypeSpecifier::Struct(struct_type) => match &struct_type.node.identifier {
                            Some(identifier) => Ok(Basetype {
                                origin: Default::default(),
                                name: TypeName::new(&identifier.node.name),
                                ty: Type::Void,
                            }),
                            _ => Err(NotApplicable),
                        },
                        _ => Err(NotApplicable),
                    },
                    _ => Err(NotApplicable),
                },
                _ => Err(NotApplicable),
            },
            _ => Err(NotApplicable),
        }
    }
}

impl Source {
    pub fn collect_basetype(&mut self, node: XmlNode) {
        if let Some(name) = node
            .children()
            .find(|n| n.has_tag_name("name"))
            .and_then(|n| n.text())
        {
            if let Some(basetype) = self.header.take_basetype(&name) {
                self.basetypes.push(basetype);
            }
        }
    }
}
