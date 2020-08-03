use crate::{comment_gen::DocCommentGen, origin::Origin};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;

pub fn tokens(comment_gen: &DocCommentGen) -> HashMap<Origin, TokenStream> {
    let define_doc = |full| comment_gen.def(Some(full), "Define", None);
    let mut code = HashMap::new();

    let make_version_doc = define_doc("VK_MAKE_VERSION");
    let version_major_doc = define_doc("VK_VERSION_MAJOR");
    let version_minor_doc = define_doc("VK_VERSION_MINOR");
    let version_patch_doc = define_doc("VK_VERSION_PATCH");
    code.insert(
        Origin::Feature { major: 1, minor: 0 },
        quote! {
            #[doc = #make_version_doc]
            pub const fn make_version(major: u32, minor: u32, patch: u32) -> u32 {
                (major << 22) | (minor << 12) | patch
            }

            #[doc = #version_major_doc]
            pub const fn version_major(version: u32) -> u32 {
                version >> 22
            }

            #[doc = #version_minor_doc]
            pub const fn version_minor(version: u32) -> u32 {
                (version >> 12) & 0x3ff
            }

            #[doc = #version_patch_doc]
            pub const fn version_patch(version: u32) -> u32 {
                version & 0xfff
            }
        },
    );

    code
}
