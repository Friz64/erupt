use crate::{
    comment_gen::DocCommentGen,
    items::aliases::Alias,
    name::{Name, TypeName},
    origin::Origin,
    source::Source,
    XmlNode,
};
use heck::ShoutySnakeCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum HandleKind {
    Dispatchable,
    NonDispatchable,
}

impl HandleKind {
    fn macro_path(&self) -> TokenStream {
        match self {
            HandleKind::Dispatchable => quote! { crate::dispatchable_handle! },
            HandleKind::NonDispatchable => quote! { crate::non_dispatchable_handle! },
        }
    }
}

impl From<&str> for HandleKind {
    fn from(type_str: &str) -> Self {
        match type_str {
            "VK_DEFINE_HANDLE" => HandleKind::Dispatchable,
            "VK_DEFINE_NON_DISPATCHABLE_HANDLE" => HandleKind::NonDispatchable,
            _ => panic!("Invalid handle kind type string: {:?}", type_str),
        }
    }
}

impl Display for HandleKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            HandleKind::Dispatchable => "Dispatchable Handle",
            HandleKind::NonDispatchable => "Non-dispatchable Handle",
        })
    }
}

#[derive(Debug)]
pub struct Handle {
    pub origin: Option<Origin>,
    pub name: TypeName,
    pub kind: HandleKind,
}

impl Handle {
    pub fn tokens(&self, comment_gen: &DocCommentGen) -> TokenStream {
        let ident = self.name.ident();
        let doc_alias = &self.name.original;
        let macro_path = self.kind.macro_path();
        let object_type = format_ident!("{}", self.name.trimmed.to_shouty_snake_case());
        let doc = comment_gen.def(Some(&self.name.original), &self.kind, None);

        quote! {
            #macro_path(#ident, #object_type, #doc, #doc_alias);
        }
    }
}

impl Source {
    pub fn collect_handle(&mut self, node: XmlNode) {
        match (node.attribute("name"), node.attribute("alias")) {
            (Some(name), Some(alias)) => self.aliases.push(Alias::new(
                Name::Type(TypeName::new(name)),
                Name::Type(TypeName::new(alias)),
            )),
            _ => {
                let name = match node
                    .children()
                    .find(|n| n.has_tag_name("name"))
                    .and_then(|n| n.text())
                {
                    Some(name) => TypeName::new(name),
                    _ => panic!("Handle has no name: {:?}", node),
                };

                let kind = match node
                    .children()
                    .find(|n| n.has_tag_name("type"))
                    .and_then(|n| n.text())
                {
                    Some(ty) => ty.into(),
                    _ => panic!("Handle has no type: {:?}", node),
                };

                self.handles.push(Handle {
                    origin: Default::default(),
                    name,
                    kind,
                })
            }
        }
    }
}
