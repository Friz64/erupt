use crate::{
    aliases::Alias,
    comment_gen::DocCommentGen,
    declaration::{Declaration, Mutability, Type},
    header::DeclarationInfo,
    name::{FunctionName, Name},
    origin::Origin,
    source::{NotApplicable, Source},
};
use lang_c::ast::{Declaration as CDeclaration, DerivedDeclarator, ParameterDeclaration};
use proc_macro2::TokenStream;
use quote::quote;
use std::{
    convert::{TryFrom, TryInto},
    fmt::Debug,
};
use treexml::Element;

impl<'a> From<&'a ParameterDeclaration> for DeclarationInfo<'a> {
    fn from(parameter_declaration: &'a ParameterDeclaration) -> Self {
        let type_info = match parameter_declaration.specifiers.as_slice().try_into() {
            Ok(type_info) => type_info,
            Err(_) => panic!(
                "Parameter declaration has invalid specifiers: {:?}",
                parameter_declaration
            ),
        };

        let declarator = parameter_declaration
            .declarator
            .as_ref()
            .map(|declarator| &declarator.node);

        DeclarationInfo {
            type_info,
            declarator,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Requirement {
    pub base_origin: Origin,
    pub require_origin: Option<Origin>,
}

impl Requirement {
    pub fn new(base_origin: Origin, require_element: &Element) -> Requirement {
        let require_origin = if let Some(feature) = require_element.attributes.get("feature") {
            Some(Origin::from_feature_name(feature))
        } else if let Some(extension) = require_element.attributes.get("extension") {
            Some(Origin::from_extension_name(extension))
        } else {
            None
        };

        Requirement {
            base_origin,
            require_origin,
        }
    }
}

impl Source {
    pub fn assign_requirements(&mut self, element: &Element) {
        let base_origin = Origin::from_registry_item(element);
        for element_child in &element.children {
            if element_child.name == "require" {
                let requirement = Requirement::new(base_origin.clone(), element_child);

                for command in &element_child.children {
                    if command.name == "command" {
                        let function_name = FunctionName::new(
                            command.attributes.get("name").expect("Command has no name"),
                        );

                        if let Some(alias) = self.find_function_alias_mut(&function_name) {
                            alias.requirements.push(requirement.clone());
                        } else if let Some(function) = self.find_function_mut(&function_name) {
                            function.requirements.push(requirement.clone());
                        } else {
                            panic!("Did not find function with name {:?}", function_name);
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub origin: Option<Origin>,
    pub requirements: Vec<Requirement>,
    pub pfn: bool,
    pub name: FunctionName,
    pub return_type: Type,
    pub parameters: Vec<Declaration>,
}

impl Function {
    pub fn doc(&self, comment_gen: &DocCommentGen) -> String {
        comment_gen.def(
            Some(if self.pfn {
                &self.name.original
            } else {
                &self.name.no_pfn
            }),
            "Function",
            None,
        )
    }

    pub fn tokens(&self, comment_gen: &DocCommentGen, source: &Source) -> TokenStream {
        let name = self.name.pointer_ident();
        let doc = self.doc(comment_gen);
        let return_type = self.return_type.rust_type(source);

        let parameter_idents = self.parameters.iter().map(|parameter| parameter.ident());
        let parameter_types = self
            .parameters
            .iter()
            .map(|parameter| parameter.ty.rust_type(source));

        quote! {
            #[doc = #doc]
            #[allow(non_camel_case_types)]
            pub type #name = unsafe extern "system" fn(
                #(#parameter_idents: #parameter_types),*
            ) -> #return_type;
        }
    }
}

impl TryFrom<&CDeclaration> for Function {
    type Error = NotApplicable;

    fn try_from(declaration: &CDeclaration) -> Result<Self, Self::Error> {
        match declaration.declarators.as_slice() {
            [init_declarator] => {
                let declarator = &init_declarator.node.declarator.node;
                for derived in declarator.derived.as_slice() {
                    match &derived.node {
                        DerivedDeclarator::Function(function_declarator) => {
                            let ty = Declaration::from(DeclarationInfo {
                                type_info: declaration.specifiers.as_slice().try_into()?,
                                declarator: Some(&declarator),
                            });

                            let parameters = function_declarator
                                .node
                                .parameters
                                .iter()
                                .map(|parameter| Declaration::from(&parameter.node))
                                .collect();

                            let name = match ty.name {
                                Some(name) => FunctionName::new(&name),
                                None => {
                                    panic!("Function declaration has no name: {:?}", declaration)
                                }
                            };

                            let return_type = match ty.ty {
                                Type::Pointer {
                                    to,
                                    kind: Mutability::Mut,
                                } => (*to).clone(),
                                _ => panic!("Can't unwrap function return type"),
                            };

                            return Ok(Function {
                                origin: Default::default(),
                                requirements: Vec::new(),
                                pfn: false,
                                name,
                                return_type,
                                parameters,
                            });
                        }
                        _ => (),
                    }
                }

                Err(NotApplicable)
            }
            _ => Err(NotApplicable),
        }
    }
}

impl Source {
    pub fn collect_function(&mut self, element: &Element) {
        match (
            element.attributes.get("name"),
            element.attributes.get("alias"),
        ) {
            (Some(name), Some(alias)) => self.aliases.push(Alias::new(
                Name::Function(FunctionName::new(name)),
                Name::Function(FunctionName::new(alias)),
            )),
            _ => {
                let name = match element.find_value::<String>("proto/name") {
                    Ok(Some(name)) => FunctionName::new(&name),
                    _ => panic!("Function has no proto/name text: {:?}", element),
                };

                if let Some(mut function) = self.header.take_function(&name) {
                    let mut i = 0;
                    for command_child in &element.children {
                        if command_child.name == "param" {
                            function.parameters[i].metadata = command_child.into();
                            i += 1;
                        }
                    }

                    self.functions.push(function);
                }
            }
        }
    }

    pub fn collect_funcpointer(&mut self, element: &Element) {
        if let Ok(Some(name)) = element.find_value::<String>("name") {
            if let Some(mut function) = self.header.take_function(&FunctionName::new(&name)) {
                function.pfn = true;
                self.func_pointers.push(function);
            }
        }
    }
}
