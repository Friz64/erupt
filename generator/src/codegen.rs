use crate::{comment_gen::DocCommentGen, items::defines, loaders, origin::Origin, source::Source};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::{collections::HashMap, fs, path::Path};

struct CodeMap {
    map: HashMap<Origin, TokenStream>,
}

impl CodeMap {
    fn new() -> CodeMap {
        CodeMap {
            map: HashMap::new(),
        }
    }

    fn extend(&mut self, origin: Option<Origin>, stream: impl FnOnce() -> TokenStream) {
        if let Some(origin) = origin {
            self.map
                .entry(origin)
                .or_insert_with(|| TokenStream::new())
                .extend(stream());
        }
    }

    fn write<P>(&self, output_dir: P, comment_gen: &DocCommentGen)
    where
        P: AsRef<Path>,
    {
        log::info!("Writing generated code...");

        let output_dir = output_dir.as_ref();
        fs::remove_dir_all(output_dir).expect("Failed to remove output dir");
        fs::create_dir(output_dir).expect("Failed to create output dir");

        let extensions_dir = output_dir.join("extensions");
        fs::create_dir(&extensions_dir).expect("Failed to create extensions output dir");

        let root_code = self.map.get(&Origin::Root);
        let mut root_mod = quote! {
            /// Provides Vulkan extension items
            pub mod extensions;

            #root_code
        };

        let mut extensions_mod = quote! {};

        for origin in self.map.keys() {
            if *origin == Origin::Root {
                continue;
            }

            let name = format_ident!("{}", origin.path().last().unwrap());
            match origin {
                Origin::Feature { .. } => root_mod.extend(quote! {
                    /// Provides Vulkan feature items
                    pub mod #name;
                }),
                Origin::Extension { full } => {
                    let doc = comment_gen.def(Some(full), "Vulkan extension", None);
                    extensions_mod.extend(quote! {
                        #[doc = #doc]
                        pub mod #name;
                    })
                }
                _ => (),
            }
        }

        fs::write(output_dir.join("mod.rs"), root_mod.to_string())
            .expect("Failed to write generated root module");
        fs::write(extensions_dir.join("mod.rs"), extensions_mod.to_string())
            .expect("Failed to write generated extensions module");

        for (origin, tokens) in &self.map {
            if *origin == Origin::Root {
                continue;
            }

            fs::write(output_dir.join(origin.file_path()), tokens.to_string())
                .expect("Failed to write generated module");
        }
    }
}

pub fn generate(source: &Source) {
    let mut codemap = CodeMap::new();
    let comment_gen = DocCommentGen::new(source.latest_vulkan_version);

    for constant in &source.constants {
        let mut constant = constant.clone();

        // Special case: If the origin didn't get set by vk.xml, just set it to 1.0
        if constant.origin.is_none() {
            constant.origin = Some(Origin::Feature { major: 1, minor: 0 });
        }

        codemap.extend(constant.origin.clone(), || constant.tokens(&comment_gen));
    }

    for alias in &source.aliases {
        codemap.extend(alias.origin.clone(), || alias.tokens(&comment_gen, source));
    }

    for basetype in &source.basetypes {
        codemap.extend(basetype.origin.clone(), || {
            basetype.tokens(&comment_gen, source)
        });
    }

    for handle in &source.handles {
        codemap.extend(handle.origin.clone(), || handle.tokens(&comment_gen));
    }

    for en in &source.enums {
        codemap.extend(en.origin.clone(), || en.tokens(&comment_gen));
    }

    for func in source.functions.iter().chain(&source.func_pointers) {
        codemap.extend(func.origin.clone(), || func.tokens(&comment_gen, source));
    }

    for structure in &source.structures {
        codemap.extend(structure.origin.clone(), || {
            structure.tokens(&comment_gen, &source)
        });
    }

    for (origin, stream) in defines::tokens(&comment_gen) {
        codemap.extend(Some(origin), || stream);
    }

    for (origin, stream) in loaders::tokens(&comment_gen, source) {
        codemap.extend(Some(origin), || stream);
    }

    codemap.write("erupt/src/generated/", &comment_gen);
}
