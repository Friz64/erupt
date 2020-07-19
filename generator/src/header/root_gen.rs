use crate::{constants, declaration::Type};
use log::error;
use regex::Regex;
use std::{fs, path::Path};
use treexml::Element;

pub fn generate(root_header_path: &Path, include_vulkan: &Path, registry: &Element) {
    let mut output = format!("// THIS FILE IS AUTOGENERATED - DO NOT EDIT IT\n");

    let types = registry.find("types").expect("No `types` in registry");
    let mut typedef_names = Vec::new();
    for types_child in &types.children {
        match (
            types_child.attributes.get("name"),
            types_child.attributes.get("requires"),
        ) {
            (Some(name), Some(requires)) if requires.ends_with(".h") => typedef_names.push(name),
            _ => (),
        }
    }

    for typedef_name in typedef_names {
        if Type::try_from_external(typedef_name).is_none() {
            error!("Missing `try_from_external` mapping for {:?}", typedef_name)
        }

        output.push_str(&format!("typedef void {};\n", typedef_name));
    }

    let header_regex = Regex::new(r#"#include "(.+\.h)""#).unwrap();
    let vulkan_h_path = include_vulkan.join("vulkan.h");
    let vulkan_h = fs::read_to_string(vulkan_h_path).expect("Failed to read vulkan.h");
    for include in header_regex.captures_iter(&vulkan_h) {
        output.push_str(&format!("#include \"{}\"\n", &include[1]));
    }

    constants::header_ext(&mut output, registry);

    fs::write(root_header_path, output).expect("Failed to write root header");
}
