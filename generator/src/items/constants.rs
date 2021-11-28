use crate::{
    comment_gen::DocCommentGen,
    header::{
        eval::{Expression, Literal},
        ValueDependencies,
    },
    origin::Origin,
    source::{NotApplicable, Source},
    XmlNode,
};
use lang_c::ast::{
    Declaration as CDeclaration, DeclaratorKind, Expression as CExpression, Initializer,
};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::collections::{HashMap, HashSet};

const PREFIX: &str = "__ERUPT_CONSTANT_";

#[derive(Clone, Debug)]
pub enum ConstantValue {
    Number(Literal),
    String(String),
}

impl ConstantValue {
    fn value(&self) -> TokenStream {
        match self {
            ConstantValue::Number(literal) => {
                let val = literal.value();
                quote! { #val }
            }
            ConstantValue::String(string) => quote! { crate::cstr!(#string) },
        }
    }

    fn ty(&self) -> TokenStream {
        match self {
            ConstantValue::Number(literal) => {
                let ty = literal.ty();
                quote! { #ty }
            }
            ConstantValue::String(_) => quote! { *const std::os::raw::c_char },
        }
    }
}

#[derive(Clone, Debug)]
pub struct Constant {
    pub origin: Option<Origin>,
    pub original_name: Box<str>,
    pub trimmed_name: Box<str>,
    pub value: ConstantValue,
}

impl Constant {
    pub fn ident(&self) -> Ident {
        format_ident!("{}", *self.trimmed_name)
    }

    pub fn tokens(
        &self,
        comment_gen: &DocCommentGen,
        deprecated_variants: &HashSet<String>,
    ) -> HashMap<Origin, TokenStream> {
        let origin = self.origin.as_ref().expect("Constant missing origin");

        let ident = self.ident();
        let doc_alias = &self.original_name;
        let doc_alias_code = (&**doc_alias != ident.to_string().as_str())
            .then(|| quote! { #[doc(alias = #doc_alias)] });

        let value = self.value.value();
        let ty = self.value.ty();
        let doc = comment_gen.def(None, "Constant", self.origin.as_ref());

        let deprecated = deprecated_variants
            .contains(&*self.original_name)
            .then(|| quote! { #[deprecated] });

        let mut tokens = HashMap::new();
        tokens.insert(
            origin.clone(),
            quote! {
                #[doc = #doc]
                #deprecated
                #doc_alias_code
                pub const #ident: #ty = #value;
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
                let name = match &init_declarator.node.declarator.node.kind.node {
                    DeclaratorKind::Identifier(identifier) => match &identifier.node.name {
                        name if name.starts_with(PREFIX) => &name[PREFIX.len()..],
                        _ => return Err(NotApplicable),
                    },
                    _ => return Err(NotApplicable),
                };

                let value = match &init_declarator.node.initializer {
                    Some(initializer) => match &initializer.node {
                        Initializer::Expression(expression) => match &expression.node {
                            CExpression::StringLiteral(strings) => ConstantValue::String(
                                strings.node.iter().map(|s| s.trim_matches('"')).collect(),
                            ),
                            other => ConstantValue::Number(
                                Expression::from_c(other, value_dependencies).eval_to_literal(),
                            ),
                        },
                        invalid => panic!("Invalid constant initializer: {:?}", invalid),
                    },
                    None => panic!("Constant has no initializer: {:?}", declaration),
                };

                Ok(Constant {
                    origin: Default::default(),
                    original_name: name.to_string().into(),
                    trimmed_name: name.to_uppercase().trim_start_matches("VK_").into(),
                    value,
                })
            }
            _ => Err(NotApplicable),
        }
    }
}

pub fn header_ext(buf: &mut String, registry: XmlNode) {
    let mut constants = Vec::new();

    let constants_enum = registry
        .children()
        .find(|n| n.has_tag_name("enums") && n.attribute("name") == Some("API Constants"));

    match constants_enum {
        Some(constants_enum) => {
            for constant in constants_enum.children().filter(|n| n.is_element()) {
                let name = match constant.attribute("name") {
                    Some(name) => name,
                    None => panic!("Constant has no name: {:?}", constant),
                };

                constants.push(name);
            }
        }
        None => panic!("No `API Constants` in registry"),
    }

    match registry.children().find(|n| n.has_tag_name("extensions")) {
        Some(extensions) => {
            for extension in extensions.children() {
                // Skip disabled extensions
                if extension.attribute("supported") == Some("disabled") {
                    continue;
                }

                for require in extension.children() {
                    for element in require.children() {
                        if element.has_tag_name("enum")
                            && (element.has_attribute("value") || element.has_attribute("alias"))
                            && !element.has_attribute("extends")
                        {
                            if let Some(name) = element.attribute("name") {
                                constants.push(name);
                            }
                        }
                    }
                }
            }
        }
        None => panic!("No `extensions` in registry"),
    }

    for constant in constants {
        buf.push_str(&format!(
            "void {}{name} = {name};\n",
            PREFIX,
            name = constant
        ));
    }
}

impl Source {
    pub fn add_function_constants(&mut self) {
        for function in self.all_functions_emulated() {
            let name: Box<str> = function.name.constant_name().into();
            let value = function.name.no_pfn.clone();

            self.constants.push(Constant {
                origin: function.origin.clone(),
                original_name: name.clone(),
                trimmed_name: name.clone(),
                value: ConstantValue::String(value.into()),
            });
        }
    }
}
