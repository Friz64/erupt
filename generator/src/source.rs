use crate::{
    aliases::Alias,
    basetypes::Basetype,
    constants::Constant,
    enums::{Enum, EnumKind, EnumVariant},
    functions::Function,
    handles::Handle,
    header::HeaderSource,
    name::{FunctionName, Name, TypeName},
    origin::Origin,
    structures::Structure,
};
use std::{
    error::Error,
    fmt::{self, Debug, Display},
    fs::File,
};
use treexml::Document;

#[derive(Debug)]
pub struct NotApplicable;

impl Display for NotApplicable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for NotApplicable {}

pub struct Source {
    pub header: HeaderSource,
    pub aliases: Vec<Alias>,
    pub structures: Vec<Structure>,
    pub functions: Vec<Function>,
    pub func_pointers: Vec<Function>,
    pub handles: Vec<Handle>,
    pub constants: Vec<Constant>,
    pub basetypes: Vec<Basetype>,
    pub enums: Vec<Enum>,
    pub latest_vulkan_version: (u32, u32),
}

impl Source {
    pub fn collect() -> Source {
        let vk_xml =
            File::open("generator/Vulkan-Headers/registry/vk.xml").expect("Failed to open vk.xml");
        let document = Document::parse(vk_xml).expect("Failed to parse vk.xml");
        let registry = document.root.expect("Document has no root");

        let latest_vulkan_version = registry
            .children
            .iter()
            .filter_map(|registry_child| match registry_child.name.as_str() {
                "feature" => match Origin::from_registry_item(registry_child) {
                    Origin::Feature { major, minor } => Some((major, minor)),
                    _ => unreachable!(),
                },
                _ => None,
            })
            .last()
            .expect("No Vulkan version");

        let header_version = registry
            .find("types")
            .expect("No `types` in registry")
            .children
            .iter()
            .find_map(|ty| {
                ty.find("name")
                    .ok()
                    .and_then(|name| name.text.as_ref().map(|s| s.as_str()))
                    .filter(|text| *text == "VK_HEADER_VERSION")
                    .and_then(|_| ty.text.as_ref().and_then(|s| s.split_whitespace().last()))
            })
            .expect("Can't find header version element");
        log::info!("Generating against header version {}", header_version);

        let mut header = HeaderSource::new(&registry);
        let constants = header.take_constants();
        let mut source = Source {
            header,
            aliases: Vec::new(),
            structures: Vec::new(),
            functions: Vec::new(),
            func_pointers: Vec::new(),
            handles: Vec::new(),
            constants,
            basetypes: Vec::new(),
            enums: Vec::new(),
            latest_vulkan_version,
        };

        for registry_child in &registry.children {
            match registry_child.name.as_str() {
                "types" => {
                    for element in &registry_child.children {
                        if element.name == "type" {
                            match element.attributes.get("category").map(|s| s.as_str()) {
                                Some("struct") | Some("union") => source.collect_structure(element),
                                Some("handle") => source.collect_handle(element),
                                Some("funcpointer") => source.collect_funcpointer(element),
                                Some("basetype") => source.collect_basetype(element),
                                Some("bitmask") | Some("enum") => source.collect_enum_type(element),
                                _ => (),
                            }
                        }
                    }
                }
                "commands" => {
                    for element in &registry_child.children {
                        if element.name == "command" {
                            source.collect_function(element);
                        }
                    }
                }
                "enums" => {
                    source.collect_enum(registry_child);
                }
                "feature" => {
                    source.assign_origins(registry_child);
                    source.assign_requirements(registry_child);
                }
                "extensions" => {
                    for extension in &registry_child.children {
                        // Skip disabled extensions
                        let supported = extension.attributes.get("supported").map(|s| s.as_str());
                        if supported == Some("disabled") {
                            continue;
                        }

                        source.assign_origins(extension);
                        source.assign_requirements(extension);
                    }
                }
                _ => (),
            }
        }

        source.function_constants();
        source
    }

    pub fn all_functions_emulated(&self) -> Vec<Function> {
        let mut output = self.functions.clone();
        let aliases_emulated = self
            .aliases
            .iter()
            .filter_map(|alias| alias.emulate_function(self));
        output.extend(aliases_emulated);
        output
    }

    pub fn find_alias(&self, name: &Name) -> Option<&Alias> {
        self.aliases.iter().find(|alias| &alias.name == name)
    }

    pub fn find_function_alias(&self, function_name: &FunctionName) -> Option<&Alias> {
        self.aliases.iter().find(|alias| match &alias.name {
            Name::Function(name) => name == function_name,
            _ => false,
        })
    }

    pub fn find_function_alias_mut(&mut self, function_name: &FunctionName) -> Option<&mut Alias> {
        self.aliases.iter_mut().find(|alias| match &alias.name {
            Name::Function(name) => name == function_name,
            _ => false,
        })
    }

    pub fn find_type_alias(&self, type_name: &TypeName) -> Option<&Alias> {
        self.aliases.iter().find(|alias| match &alias.name {
            Name::Type(name) => name == type_name,
            _ => false,
        })
    }

    pub fn find_type_alias_mut(&mut self, type_name: &TypeName) -> Option<&mut Alias> {
        self.aliases.iter_mut().find(|alias| match &alias.name {
            Name::Type(name) => name == type_name,
            _ => false,
        })
    }

    pub fn find_structure(&self, type_name: &TypeName) -> Option<&Structure> {
        self.structures
            .iter()
            .find(|structure| &structure.name == type_name)
    }

    pub fn find_structure_mut(&mut self, type_name: &TypeName) -> Option<&mut Structure> {
        self.structures
            .iter_mut()
            .find(|structure| &structure.name == type_name)
    }

    pub fn find_function(&self, function_name: &FunctionName) -> Option<&Function> {
        self.functions
            .iter()
            .find(|function| &function.name == function_name)
    }

    pub fn find_function_mut(&mut self, function_name: &FunctionName) -> Option<&mut Function> {
        self.functions
            .iter_mut()
            .find(|function| &function.name == function_name)
    }

    pub fn find_func_pointer(&self, function_name: &FunctionName) -> Option<&Function> {
        self.func_pointers
            .iter()
            .find(|func_pointer| &func_pointer.name == function_name)
    }

    pub fn find_func_pointer_mut(&mut self, function_name: &FunctionName) -> Option<&mut Function> {
        self.func_pointers
            .iter_mut()
            .find(|func_pointer| &func_pointer.name == function_name)
    }

    pub fn find_handle(&self, type_name: &TypeName) -> Option<&Handle> {
        self.handles.iter().find(|handle| &handle.name == type_name)
    }

    pub fn find_handle_mut(&mut self, type_name: &TypeName) -> Option<&mut Handle> {
        self.handles
            .iter_mut()
            .find(|handle| &handle.name == type_name)
    }

    pub fn _find_constant(&self, original_name: &str) -> Option<&Constant> {
        self.constants
            .iter()
            .find(|constant| &*constant.original_name == original_name)
    }

    pub fn find_constant_mut(&mut self, original_name: &str) -> Option<&mut Constant> {
        self.constants
            .iter_mut()
            .find(|constant| &*constant.original_name == original_name)
    }

    pub fn find_basetype(&self, type_name: &TypeName) -> Option<&Basetype> {
        self.basetypes
            .iter()
            .find(|basetype| &basetype.name == type_name)
    }

    pub fn find_basetype_mut(&mut self, type_name: &TypeName) -> Option<&mut Basetype> {
        self.basetypes
            .iter_mut()
            .find(|basetype| &basetype.name == type_name)
    }

    pub fn find_enum(&self, type_name: &TypeName) -> Option<&Enum> {
        self.enums.iter().find(|en| match &en.kind {
            EnumKind::Enum { name } => name == type_name,
            EnumKind::Bitflag {
                flagbits_name,
                flags_name,
            } => flagbits_name == type_name || flags_name == type_name,
        })
    }

    pub fn find_enum_mut(&mut self, type_name: &TypeName) -> Option<&mut Enum> {
        self.enums.iter_mut().find(|en| match &en.kind {
            EnumKind::Enum { name } => name == type_name,
            EnumKind::Bitflag {
                flagbits_name,
                flags_name,
            } => flagbits_name == type_name || flags_name == type_name,
        })
    }

    pub fn _find_enum_variant(&self, original_name: &str) -> Option<&EnumVariant> {
        self.enums
            .iter()
            .flat_map(|en| &en.variants)
            .find(|variant| &*variant.name.original == original_name)
    }

    pub fn find_enum_variant_mut(&mut self, original_name: &str) -> Option<&mut EnumVariant> {
        self.enums
            .iter_mut()
            .flat_map(|en| &mut en.variants)
            .find(|variant| &*variant.name.original == original_name)
    }
}
