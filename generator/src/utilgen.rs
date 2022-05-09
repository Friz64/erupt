use crate::{name::TypeName, origin::Origin, source::Source};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;

pub fn tokens(source: &Source) -> HashMap<Origin, TokenStream> {
    let features2 = TypeName::physical_device_features2();
    let features = source
        .find_structure(&TypeName::physical_device_features())
        .expect("PhysicalDeviceFeatures missing");
    let features2_structs = source.structures.iter().filter_map(|structure| {
        let is_features2 = structure.name == features2;
        let extends_features2 = structure.metadata.extends.contains(&features2);
        (is_features2 || extends_features2).then(|| {
            let structure_type = structure
                .structure_type_value(source)
                .expect("PhysicalDeviceFeatures2 struct with no structure type");
            let bool_count = if is_features2 {
                features.fields.len()
            } else {
                structure.fields.len() - 2
            };

            quote! { #structure_type => Some(#bool_count), }
        })
    });

    let tokens = quote! {
        pub(crate) fn features2_bool_count(
            structure_type: crate::vk1_0::StructureType
        ) -> Option<usize> {
            match structure_type {
                #(#features2_structs)*
                _ => None,
            }
        }
    };

    let mut code = HashMap::new();
    code.insert(Origin::Root, tokens);
    code
}
