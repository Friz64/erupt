use crate::{comment_gen::DocCommentGen, origin::Origin};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;

pub fn tokens(comment_gen: &DocCommentGen) -> HashMap<Origin, TokenStream> {
    let major_version = comment_gen.major_version();
    let minor_version = comment_gen.minor_version();
    let patch_version = comment_gen.patch_version();

    let mut tokens = TokenStream::new();
    let mut define = |full_name, core_code| {
        let doc = comment_gen.def(Some(full_name), "Define", None);
        tokens.extend(quote! {
            #[doc = #doc]
            #[doc(alias = #full_name)]
            #core_code
        })
    };

    define(
        "VK_MAKE_VERSION",
        quote! {
            #[deprecated = "VK_MAKE_API_VERSION should be used instead."]
            pub const fn make_version(major: u32, minor: u32, patch: u32) -> u32 {
                (major << 22) | (minor << 12) | patch
            }
        },
    );

    define(
        "VK_VERSION_MAJOR",
        quote! {
            #[deprecated = "VK_API_VERSION_MAJOR should be used instead."]
            pub const fn version_major(version: u32) -> u32 {
                version >> 22
            }
        },
    );

    define(
        "VK_VERSION_MINOR",
        quote! {
            #[deprecated = "VK_API_VERSION_MINOR should be used instead."]
            pub const fn version_minor(version: u32) -> u32 {
                (version >> 12) & 0x3ff
            }
        },
    );

    define(
        "VK_VERSION_PATCH",
        quote! {
            #[deprecated = "VK_API_VERSION_PATCH should be used instead."]
            pub const fn version_patch(version: u32) -> u32 {
                version & 0xfff
            }
        },
    );

    define(
        "VK_MAKE_API_VERSION",
        quote! {
            pub const fn make_api_version(variant: u32, major: u32, minor: u32, patch: u32) -> u32 {
                (variant << 29) | (major << 22) | (minor << 12) | patch
            }
        },
    );

    define(
        "VK_API_VERSION_VARIANT",
        quote! {
            pub const fn api_version_variant(version: u32) -> u32 {
                version >> 29
            }
        },
    );

    define(
        "VK_API_VERSION_MAJOR",
        quote! {
            pub const fn api_version_major(version: u32) -> u32 {
                (version >> 22) & 0x7f
            }
        },
    );

    define(
        "VK_API_VERSION_MINOR",
        quote! {
            pub const fn api_version_minor(version: u32) -> u32 {
                (version >> 12) & 0x3ff
            }
        },
    );

    define(
        "VK_API_VERSION_PATCH",
        quote! {
            pub const fn api_version_patch(version: u32) -> u32 {
                version & 0xfff
            }
        },
    );

    define(
        "VK_API_VERSION_1_0",
        quote! {
            pub const API_VERSION_1_0: u32 = make_api_version(0, 1, 0, 0);
        },
    );

    define(
        "VK_API_VERSION_1_1",
        quote! {
            pub const API_VERSION_1_1: u32 = make_api_version(0, 1, 1, 0);
        },
    );

    define(
        "VK_API_VERSION_1_2",
        quote! {
            pub const API_VERSION_1_2: u32 = make_api_version(0, 1, 2, 0);
        },
    );

    define(
        "VK_HEADER_VERSION",
        quote! {
            pub const HEADER_VERSION: u32 = #patch_version;
        },
    );

    define(
        "VK_HEADER_VERSION_COMPLETE",
        quote! {
            pub const HEADER_VERSION_COMPLETE: u32 =
                make_api_version(0, #major_version, #minor_version, #patch_version);
        },
    );

    let mut code = HashMap::new();
    code.insert(Origin::Feature { major: 1, minor: 0 }, tokens);
    code
}
