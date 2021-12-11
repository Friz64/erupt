use crate::{comment_gen::DocCommentGen, items::defines, loaders, origin::Origin, source::Source};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
};

struct CodeMap {
    map: HashMap<Origin, TokenStream>,
}

impl CodeMap {
    fn new() -> CodeMap {
        CodeMap {
            map: HashMap::new(),
        }
    }

    fn extend(&mut self, tokens: HashMap<Origin, TokenStream>) {
        for (origin, tokens_inner) in tokens {
            self.map
                .entry(origin)
                .or_insert_with(TokenStream::new)
                .extend(tokens_inner);
        }
    }

    fn write<P>(
        &self,
        output_dir: P,
        comment_gen: &DocCommentGen,
        provisional_extensions: &HashSet<String>,
    ) where
        P: AsRef<Path>,
    {
        log::info!("Writing generated code...");

        let output_dir = output_dir.as_ref();
        fs::remove_dir_all(output_dir).expect("Failed to remove output dir");
        fs::create_dir(output_dir).expect("Failed to create output dir");

        let extensions_dir = output_dir.join("extensions");
        fs::create_dir(&extensions_dir).expect("Failed to create extensions output dir");

        let external_dir = output_dir.join("external");
        fs::create_dir(&external_dir).expect("Failed to create external output dir");

        let root_code = self.map.get(&Origin::Root);
        let mut root_mod = quote! {
            /// Provides Vulkan extension items.
            pub mod extensions;
            /// Provides external library items.
            pub mod external;
            /// Re-exports **every** Vulkan item.
            pub mod vk;

            #root_code
        };

        let mut extensions_mod = quote! {};
        let mut external_mod = quote! {};
        let mut vk_mod = quote! {};

        for origin in self.map.keys() {
            if *origin == Origin::Root {
                continue;
            }

            if !matches!(origin, Origin::External { .. }) {
                let origin_module_path = origin.module_path();
                vk_mod.extend(quote! {
                    #[doc(no_inline)]
                    pub use crate::#origin_module_path*;
                });
            }

            let name = format_ident!("{}", origin.path().last().unwrap());
            match origin {
                Origin::Root => (),
                Origin::Feature { .. } => root_mod.extend(quote! {
                    /// Provides Vulkan feature items.
                    pub mod #name;
                }),
                Origin::Extension { full } => {
                    let doc = comment_gen.def(Some(full), "Vulkan extension", None);
                    extensions_mod.extend(quote! {
                        #[doc = #doc]
                        pub mod #name;
                    });
                }
                Origin::External { .. } => {
                    let doc = comment_gen.def(None, "External library", None);
                    external_mod.extend(quote! {
                        #[doc = #doc]
                        pub mod #name;
                    });
                }
            }
        }

        fs::write(output_dir.join("mod.rs"), root_mod.to_string())
            .expect("Failed to write generated root module");
        fs::write(extensions_dir.join("mod.rs"), extensions_mod.to_string())
            .expect("Failed to write generated extensions module");
        fs::write(external_dir.join("mod.rs"), external_mod.to_string())
            .expect("Failed to write generated external module");
        fs::write(output_dir.join("vk.rs"), vk_mod.to_string())
            .expect("Failed to write generated vk re-export module");

        for (origin, tokens) in &self.map {
            if *origin == Origin::Root {
                continue;
            }

            let head = match origin {
                Origin::Extension { full } if provisional_extensions.contains(full) => {
                    "\
                    //! ## Versioning Warning ⚠️
                    //!
                    //! This is a Vulkan **provisional/beta** extension and **must** be used with
                    //! caution. Its API/behaviour has not been finalized yet and _may_ therefore
                    //! change in ways that break backwards compatibility between revisions, and
                    //! before final release of a non-provisional version of this extension.
                    "
                }
                _ => "",
            };

            let contents = head.to_string() + &tokens.to_string();
            fs::write(output_dir.join(origin.file_path()), contents)
                .expect("Failed to write generated module");
        }
    }
}

pub fn generate(source: &Source) {
    let mut codemap = CodeMap::new();
    let comment_gen = DocCommentGen::new((
        source.latest_vulkan_version.0,
        source.latest_vulkan_version.1,
        source.header_version,
    ));

    for constant in &source.constants {
        let mut constant = constant.clone();

        // Special case: If the origin didn't get set by vk.xml, just set it to 1.0
        if constant.origin.is_none() {
            constant.origin = Some(Origin::Feature { major: 1, minor: 0 });
        }

        codemap.extend(constant.tokens(&comment_gen, &source.deprecated_variants));
    }

    for alias in source.aliases.iter().filter(|v| v.origin.is_some()) {
        codemap.extend(alias.tokens(&comment_gen, source));
    }

    for basetype in source.basetypes.iter().filter(|v| v.origin.is_some()) {
        codemap.extend(basetype.tokens(&comment_gen, source));
    }

    for handle in source.handles.iter().filter(|v| v.origin.is_some()) {
        codemap.extend(handle.tokens(&comment_gen));
    }

    for en in source.enums.iter().filter(|v| v.origin.is_some()) {
        codemap.extend(en.tokens(&comment_gen, &source.deprecated_variants));
    }

    let functions = source
        .functions
        .iter()
        .chain(&source.func_pointers)
        .filter(|v| v.origin.is_some());
    for func in functions {
        codemap.extend(func.tokens(&comment_gen, source));
    }

    for structure in source.structures.iter().filter(|v| v.origin.is_some()) {
        codemap.extend(structure.tokens(&comment_gen, source));
    }

    codemap.extend(defines::tokens(&comment_gen));
    codemap.extend(loaders::tokens(&comment_gen, source));

    codemap.write(
        "erupt/src/generated/",
        &comment_gen,
        &source.provisional_extensions,
    );
}
