use crate::{
    header::HeaderSource,
    items::{
        aliases::Alias,
        basetypes::Basetype,
        constants::Constant,
        enums::{Enum, EnumKind, EnumVariant},
        functions::Function,
        handles::Handle,
        structures::Structure,
    },
    name::{FunctionName, Name, TypeName},
    origin::Origin,
    Opt,
};
use roxmltree::Document;
use std::{
    collections::HashSet,
    error::Error,
    fmt::{self, Debug, Display},
    fs,
    path::Path,
};

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
    pub header_version: u32,
    pub provisional_extensions: HashSet<String>,
}

impl Source {
    pub fn collect(opt: &Opt) -> Source {
        let vk_xml = fs::read_to_string("generator/Vulkan-Headers/registry/vk.xml")
            .expect("Failed to read vk.xml");
        let document = Document::parse(&vk_xml).expect("Failed to parse vk.xml");
        let registry = document.root_element();

        let latest_vulkan_version = registry
            .children()
            .filter_map(|registry_child| match registry_child.tag_name().name() {
                "feature" => match Origin::from_registry_item(registry_child) {
                    Origin::Feature { major, minor } => Some((major, minor)),
                    _ => unreachable!(),
                },
                _ => None,
            })
            .last()
            .expect("No Vulkan version");

        let header_version = registry
            .children()
            .find(|n| n.has_tag_name("types"))
            .expect("No `types` in registry")
            .children()
            .find_map(|ty| {
                ty.children()
                    .find(|n| n.has_tag_name("name"))
                    .and_then(|n| n.text())
                    .filter(|&text| text == "VK_HEADER_VERSION")
                    .and_then(|_| ty.children().filter_map(|n| n.text()).last())
            })
            .expect("Can't find header version element")
            .trim()
            .parse()
            .expect("Failed to parse Header version");
        log::info!("Generating against header version {}", header_version);

        let headers_include = Path::new("generator/Vulkan-Headers/include");
        let include_vulkan = headers_include.join("vulkan");
        let other_includes_headers: Vec<_> = fs::read_dir(headers_include)
            .expect("Failed to read include dir")
            .map(|entry| entry.expect("Failed to read include dir entry"))
            .filter(|entry| entry.file_type().unwrap().is_dir())
            .map(|entry| entry.path())
            .filter(|path| path != &include_vulkan)
            .flat_map(|include| {
                fs::read_dir(include)
                    .expect("Failed to read inner include dir")
                    .map(|entry| entry.unwrap().path())
            })
            .collect();

        let mut header = HeaderSource::new(
            registry,
            &headers_include,
            &include_vulkan,
            &other_includes_headers,
            opt,
        );

        let mut source = Source {
            constants: header.take_constants(),
            header,
            aliases: Vec::new(),
            structures: Vec::new(),
            functions: Vec::new(),
            func_pointers: Vec::new(),
            handles: Vec::new(),
            basetypes: Vec::new(),
            enums: Vec::new(),
            latest_vulkan_version,
            header_version,
            provisional_extensions: HashSet::new(),
        };

        for registry_child in registry.children().filter(|n| n.is_element()) {
            match registry_child.tag_name().name() {
                "types" => {
                    for element in registry_child.children() {
                        if element.tag_name().name() == "type" {
                            match element.attribute("category") {
                                Some("struct") | Some("union") => source.collect_structure(element),
                                Some("handle") => source.collect_handle(element),
                                Some("funcpointer") => source.collect_funcpointer(element),
                                Some("basetype") => source.collect_basetype(element),
                                Some("bitmask") | Some("enum") => source.collect_enum_type(element),
                                _ => {
                                    source.assign_external_origin(element, &other_includes_headers);
                                }
                            }
                        }
                    }
                }
                "commands" => {
                    for element in registry_child.children() {
                        if element.tag_name().name() == "command" {
                            source.collect_function(element);
                        }
                    }
                }
                "enums" => {
                    source.collect_enum(registry_child);
                }
                "feature" => {
                    source.assign_origins(registry_child);
                    source.assign_function_metadata(registry_child);
                }
                "extensions" => {
                    for extension in registry_child.children().filter(|n| n.is_element()) {
                        // Skip disabled extensions
                        if extension.attribute("supported") == Some("disabled") {
                            continue;
                        }

                        if extension.attribute("provisional") == Some("true") {
                            let name = extension
                                .attribute("name")
                                .expect("Extension is missing name");

                            source.provisional_extensions.insert(name.into());
                        }

                        source.assign_origins(extension);
                        source.assign_function_metadata(extension);
                    }
                }
                _ => (),
            }
        }

        source.add_function_constants();
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
                bitwidth: _,
            } => flagbits_name == type_name || flags_name == type_name,
        })
    }

    pub fn find_enum_mut(&mut self, type_name: &TypeName) -> Option<&mut Enum> {
        self.enums.iter_mut().find(|en| match &en.kind {
            EnumKind::Enum { name } => name == type_name,
            EnumKind::Bitflag {
                flagbits_name,
                flags_name,
                bitwidth: _,
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
