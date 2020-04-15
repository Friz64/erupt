use crate::{types::TypeItem, Origin};
use itertools::Itertools;
use std::collections::HashMap;
use vk_parse::{Enum, EnumSpec, ExtensionChild, InterfaceItem};

pub fn collect(children: &[ExtensionChild]) -> HashMap<String, Vec<String>> {
    let mut ext_map = HashMap::new();
    for child in children {
        match child {
            ExtensionChild::Require { items, .. } => {
                for item in items {
                    if let InterfaceItem::Enum(vk_enum) = item {
                        match vk_enum {
                            Enum {
                                name,
                                spec: EnumSpec::Offset { extends, .. },
                                ..
                            } => {
                                ext_map_insert(&mut ext_map, extends, name);
                            }
                            Enum {
                                name,
                                spec:
                                    EnumSpec::Bitpos {
                                        extends: Some(extends),
                                        ..
                                    },
                                ..
                            } => {
                                ext_map_insert(&mut ext_map, extends, name);
                            }
                            Enum {
                                name,
                                spec:
                                    EnumSpec::Alias {
                                        extends: Some(extends),
                                        ..
                                    },
                                ..
                            } => {
                                ext_map_insert(&mut ext_map, extends, name);
                            }
                            Enum {
                                name,
                                spec:
                                    EnumSpec::Value {
                                        extends: Some(extends),
                                        ..
                                    },
                                ..
                            } => {
                                ext_map_insert(&mut ext_map, extends, name);
                            }
                            _ => (),
                        }
                    }
                }
            }
            _ => (),
        }
    }

    ext_map
}

fn ext_map_insert(ext_map: &mut HashMap<String, Vec<String>>, extends: &str, name: &str) {
    ext_map
        .entry(extends.into())
        .or_insert_with(|| Vec::new())
        .push(name.into());
}

pub fn extend(
    enum_ext_map: &HashMap<String, HashMap<Origin, Vec<String>>>,
    vk_type: &mut TypeItem,
) {
    match vk_type {
        TypeItem::Bitmask {
            flag_bits,
            variants,
            ..
        } => {
            ext_append(enum_ext_map, flag_bits, variants);
        }
        TypeItem::Enum { name, variants } => {
            ext_append(enum_ext_map, name, variants);
        }
        _ => (),
    }
}

fn ext_append(
    enum_ext_map: &HashMap<String, HashMap<Origin, Vec<String>>>,
    name: &str,
    variants: &mut HashMap<Option<Origin>, Vec<String>>,
) {
    if let Some(enum_exts) = enum_ext_map.get(name) {
        for (origin, ext_variants) in enum_exts.iter().sorted() {
            let variants = variants
                .entry(Some(origin.clone()))
                .or_insert_with(|| Vec::new());

            variants.extend(ext_variants.iter().cloned())
        }
    }
}
