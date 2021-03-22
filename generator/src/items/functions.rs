use crate::{
    comment_gen::DocCommentGen,
    declaration::{Declaration, Mutability, Type},
    header::{BitWidth, DeclarationInfo},
    items::aliases::Alias,
    name::{FunctionName, Name},
    origin::Origin,
    source::{NotApplicable, Source},
    XmlNode,
};
use lang_c::ast::{Declaration as CDeclaration, DerivedDeclarator, ParameterDeclaration};
use proc_macro2::TokenStream;
use quote::quote;
use std::{
    convert::{TryFrom, TryInto},
    fmt::Debug,
};

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
            bitwidth: BitWidth::Full,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Requirement {
    pub base_origin: Origin,
    pub require_origin: Option<Origin>,
}

impl Requirement {
    pub fn new(base_origin: Origin, require_node: XmlNode) -> Requirement {
        let require_origin = if let Some(feature) = require_node.attribute("feature") {
            Some(Origin::feature_from_name(feature))
        } else if let Some(extension) = require_node.attribute("extension") {
            Some(Origin::Extension {
                full: extension.into(),
            })
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
    pub fn assign_function_metadata(&mut self, node: XmlNode) {
        let base_origin = Origin::from_registry_item(node);
        let extension_type = base_origin.is_extension().then(|| {
            let attribute = node
                .attribute("type")
                .expect("No type attribute on extension");

            ExtensionType::from_attribute_name(attribute)
        });

        for node_child in node.children() {
            if node_child.has_tag_name("require") {
                let requirement = Requirement::new(base_origin.clone(), node_child);

                for command in node_child.children() {
                    if command.has_tag_name("command") {
                        let function_name = FunctionName::new(
                            command.attribute("name").expect("Command has no name"),
                        );

                        let extension_type = extension_type.clone();
                        if let Some(alias) = self.find_function_alias_mut(&function_name) {
                            alias.requirements.push(requirement.clone());

                            if let Some(extension_type) = extension_type {
                                alias.extension_type.get_or_insert(extension_type);
                            }
                        } else if let Some(function) = self.find_function_mut(&function_name) {
                            function.requirements.push(requirement.clone());

                            if let Some(extension_type) = extension_type {
                                function.extension_type.get_or_insert(extension_type);
                            }
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
pub enum ExtensionType {
    Instance,
    Device,
}

impl ExtensionType {
    fn from_attribute_name(name: &str) -> ExtensionType {
        match name {
            "instance" => ExtensionType::Instance,
            "device" => ExtensionType::Device,
            invalid => panic!("Invalid extension type attribute: {:?}", invalid),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub origin: Option<Origin>,
    pub extension_type: Option<ExtensionType>,
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
                    if let DerivedDeclarator::Function(function_declarator) = &derived.node {
                        let ty = Declaration::from(DeclarationInfo {
                            type_info: declaration.specifiers.as_slice().try_into()?,
                            declarator: Some(&declarator),
                            bitwidth: BitWidth::Full,
                        });

                        let parameters = function_declarator
                            .node
                            .parameters
                            .iter()
                            .map(|parameter| Declaration::from(&parameter.node))
                            .collect();

                        let name = match ty.name {
                            Some(name) => FunctionName::new(&name),
                            None => panic!("Function declaration has no name: {:?}", declaration),
                        };

                        let return_type = match ty.ty {
                            Type::Pointer {
                                to,
                                kind: Mutability::Mut,
                            } => match *to {
                                Type::Void => Type::Unit,
                                other => other,
                            },
                            _ => panic!("Can't unwrap function return type"),
                        };

                        return Ok(Function {
                            origin: Default::default(),
                            extension_type: Default::default(),
                            requirements: Vec::new(),
                            pfn: false,
                            name,
                            return_type,
                            parameters,
                        });
                    }
                }

                Err(NotApplicable)
            }
            _ => Err(NotApplicable),
        }
    }
}

impl Source {
    pub fn collect_function(&mut self, node: XmlNode) {
        match (node.attribute("name"), node.attribute("alias")) {
            (Some(name), Some(alias)) => self.aliases.push(Alias::new(
                Name::Function(FunctionName::new(name)),
                Name::Function(FunctionName::new(alias)),
            )),
            _ => {
                let name = match node
                    .children()
                    .find(|n| n.has_tag_name("proto"))
                    .and_then(|n| n.children().find(|n| n.has_tag_name("name")))
                    .and_then(|n| n.text())
                {
                    Some(name) => FunctionName::new(&name),
                    _ => panic!("Function has no proto/name text: {:?}", node),
                };

                if let Some(mut function) = self.header.take_function(&name) {
                    for (i, command_child) in node
                        .children()
                        .filter(|n| n.has_tag_name("param"))
                        .enumerate()
                    {
                        function.parameters[i].metadata = command_child.into();
                    }

                    self.functions.push(function);
                }
            }
        }
    }

    pub fn collect_funcpointer(&mut self, node: XmlNode) {
        if let Some(name) = node
            .children()
            .find(|n| n.has_tag_name("name"))
            .and_then(|n| n.text())
        {
            if let Some(mut function) = self.header.take_function(&FunctionName::new(&name)) {
                function.pfn = true;
                self.func_pointers.push(function);
            }
        }
    }
}
