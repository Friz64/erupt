use crate::Origin;
use heck::SnakeCase;
use lazy_static::lazy_static;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use regex::Regex;
use std::collections::HashMap;
use syn::Ident;

pub const CONST_PTR: &str = "*const ";
pub const MUT_PTR: &str = "*mut ";
pub const CONST_REF_A: &str = "&'a ";
pub const MUT_REF_A: &str = "&'a mut ";
pub const CONST_REF: &str = "& ";
pub const MUT_REF: &str = "&mut ";
pub const BOOL32: &str = "crate::vk1_0::Bool32";

lazy_static! {
    static ref DECLARATION_REGEX: Regex =
        Regex::new(r"(const\s*)?(struct\s*)?(\w+)(\*)?(\*)?\s+(const\*\s+)?(\w+)").unwrap();
    static ref ARRAY_REGEX: Regex = Regex::new(r"\[\s*(\w+)\s*\]").unwrap();
    pub static ref RELEVANT_NAME_REGEX: Regex = Regex::new("(p+_)?(.*)").unwrap();
}

pub type ParsedDeclaration = (bool, String, bool, bool, bool, String, Vec<String>);

pub fn c_type_to_rust(c_type: &str) -> Option<&str> {
    match c_type {
        "char" => Some("std::os::raw::c_char"),
        "size_t" => Some("usize"),
        "uint8_t" => Some("u8"),
        "uint16_t" => Some("u16"),
        "uint32_t" => Some("u32"),
        "uint64_t" => Some("u64"),
        "int32_t" => Some("i32"),
        "int64_t" => Some("i64"),
        "int" => Some("std::os::raw::c_int"),
        "float" => Some("f32"),
        "double" => Some("f64"),
        "void" => Some("std::ffi::c_void"),
        "Display" => Some("*const std::ffi::c_void"),
        "Window" => Some("std::os::raw::c_ulong"),
        "VisualID" => Some("std::os::raw::c_uint"),
        "RROutput" => Some("std::os::raw::c_ulong"),
        "xcb_connection_t" => Some("*const std::ffi::c_void"),
        "xcb_window_t" => Some("u32"),
        "xcb_visualid_t" => Some("*const std::ffi::c_void"),
        "wl_display" => Some("std::ffi::c_void"),
        "wl_surface" => Some("std::ffi::c_void"),
        "ANativeWindow" => Some("std::ffi::c_void"),
        "AHardwareBuffer" => Some("std::ffi::c_void"),
        "HINSTANCE" => Some("*mut std::ffi::c_void"),
        "HWND" => Some("*mut std::ffi::c_void"),
        "HANDLE" => Some("*mut std::ffi::c_void"),
        "SECURITY_ATTRIBUTES" => Some("crate::SECURITY_ATTRIBUTES"),
        "DWORD" => Some("u32"),
        "LPCWSTR" => Some("*const u16"),
        "HMONITOR" => Some("*mut std::ffi::c_void"),
        "zx_handle_t" => Some("u32"),
        "GgpStreamDescriptor" => Some("u32"),
        "GgpFrameToken" => Some("u32"),
        "CAMetalLayer" => Some("std::ffi::c_void"),
        _ => None,
    }
}

pub fn safe_ident(ident: &str) -> Ident {
    match syn::parse_str(ident) {
        Ok(ident) => ident,
        Err(_) => format_ident!("_{}", ident),
    }
}

pub fn trim_vk_prefix(ty: &str) -> &str {
    ty.trim_start_matches("VK_")
        .trim_start_matches("Vk")
        .trim_start_matches("vk")
}

pub fn man_doc(vulkan_version: &str, raw_name: &str) -> String {
    format!(
        "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/{}-extensions/man/html/{}.html)",
        vulkan_version,
        raw_name
    )
}

pub fn origin_path(origin: &Origin) -> String {
    match origin {
        Origin::Feature(major, minor) => format!("vk{}_{}", major, minor),
        Origin::Extension(_, primary_secondary, _) => format!("extensions::{}", primary_secondary),
    }
}

pub fn doc_path(to_origin_path: &str, host_origin_path: &str) -> String {
    "../".repeat(host_origin_path.split("::").count()) + &to_origin_path.replace("::", "/")
}

pub fn item_path(itemname_origin_map: &HashMap<String, Origin>, itemname: &str) -> String {
    itemname_origin_map
        .get(itemname)
        .map(|origin| {
            let trimmed = trim_vk_prefix(itemname);
            format!("crate::{}::{}", origin_path(origin), trimmed)
        })
        .or_else(|| c_type_to_rust(itemname).map(Into::into))
        .unwrap_or_else(|| "ERUPT_BUG_NOT_IMPLEMENTED".into())
}

pub fn parse_c_decl(declaration: &str) -> Option<ParsedDeclaration> {
    let captures = DECLARATION_REGEX.captures(&declaration)?;

    let is_const = captures.get(1).is_some();
    let c_type = &captures[3];
    let is_pointer = captures.get(4).is_some();
    let second_mut_pointer = captures.get(5).is_some();
    let second_const_pointer = captures.get(6).is_some();
    let name = &captures[7];
    let mut array: Vec<String> = ARRAY_REGEX
        .captures_iter(&declaration)
        .map(|captures| captures[1].to_string())
        .collect();
    array.reverse();

    Some((
        is_const,
        c_type.to_string(),
        is_pointer,
        second_mut_pointer,
        second_const_pointer,
        name.to_string(),
        array,
    ))
}

pub fn convert_c_decl(
    itemname_origin_map: &HashMap<String, Origin>,
    (is_const, c_type, is_pointer, second_mut_pointer, second_const_pointer, name, array): &ParsedDeclaration,
) -> (String, Ident, String, TokenStream, TokenStream) {
    let mut rust_type = item_path(itemname_origin_map, &c_type);
    for length_raw in array {
        let length = if length_raw.parse::<i64>().is_ok() {
            length_raw.into()
        } else {
            item_path(itemname_origin_map, &length_raw) + " as usize"
        };

        rust_type = format!("[{}; {}]", rust_type, length);
    }

    if *is_pointer {
        rust_type = format!(
            "*{} {} {} {}",
            if *is_const { "const" } else { "mut" },
            if *second_mut_pointer { "*mut" } else { "" },
            if *second_const_pointer { "*const" } else { "" },
            rust_type
        );
    }

    if c_type.starts_with("PFN_") {
        rust_type = format!("Option<{}>", rust_type);
    }

    let rust_name = safe_ident(&name.to_snake_case());
    let rust_type_parsed: TokenStream = syn::parse_str(&rust_type).unwrap();
    let code = quote! { #rust_name: #rust_type_parsed, };

    (
        name.to_string(),
        rust_name,
        rust_type,
        rust_type_parsed,
        code,
    )
}

pub fn len(len: &Option<String>, altlen: &Option<String>) -> Vec<String> {
    len.clone()
        .unwrap_or_default()
        .split(',')
        .chain(altlen.clone().unwrap_or_default().split(','))
        .filter(|s| !s.is_empty() && s != &"null-terminated")
        .map(|s| s.to_string())
        .collect()
}

pub fn doc(item_type: &str, man_doc: Option<&str>) -> String {
    format!(
        "{} · {}",
        man_doc.unwrap_or("<s>Vulkan Manual Page</s>"),
        item_type
    )
}
