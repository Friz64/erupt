use std::borrow::Borrow;

use crate::{
    header::BitWidth,
    name::{EnumVariantName, FunctionName, Name, TypeName},
    source::Source,
};
use heck::SnakeCase;
use once_cell::sync::Lazy;
use proc_macro2::{Ident, Literal, TokenStream};
use quote::{format_ident, quote};
use regex::Regex;
use syn::Lifetime;

static CLEANED_IDENT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("(p+_)?(.*)").unwrap());

#[derive(Debug, Clone, PartialEq)]
pub enum Mutability {
    Const,
    Mut,
}

impl Mutability {
    pub fn new(is_const: bool) -> Mutability {
        if is_const {
            Mutability::Const
        } else {
            Mutability::Mut
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Void,
    Char,
    SignedChar,
    UnsignedChar,
    Short,
    UnsignedShort,
    Int,
    UnsignedInt,
    Long,
    UnsignedLong,
    LongLong,
    UnsignedLongLong,
    Float,
    Double,
    Int8,
    UnsignedInt8,
    Int16,
    UnsignedInt16,
    Int32,
    UnsignedInt32,
    Int64,
    UnsignedInt64,
    Size,
    UnsignedSize,
    Bool,
    CStr,
    Unit,
    Named(Name),
    Pointer {
        to: Box<Type>,
        kind: Mutability,
    },
    Reference {
        to: Box<Type>,
        kind: Mutability,
        lifetime: Option<Lifetime>,
    },
    Array {
        of: Box<Type>,
        length: usize,
    },
    Tuple(Vec<Type>),
    Option(Box<Type>),
    SmallVec(Box<Type>),
    Slice {
        of: Box<Type>,
        kind: Mutability,
        lifetime: Option<Lifetime>,
    },
}

impl Type {
    pub fn guess(src: &str) -> Type {
        if src.contains("PFN_") {
            let name = Name::Function(FunctionName::new(src));
            Type::Option(Box::new(Type::Named(name)))
        } else {
            Type::Named(Name::Type(TypeName::new(src)))
        }
    }

    pub fn try_from_external(external: &str) -> Option<Type> {
        match external {
            "Display" => Some(Type::Void),
            "VisualID" => Some(Type::UnsignedInt64),
            "Window" => Some(Type::UnsignedInt64),
            "RROutput" => Some(Type::UnsignedInt64),
            "wl_display" => Some(Type::Void),
            "wl_surface" => Some(Type::Void),
            "HINSTANCE" => Some(Type::Pointer {
                to: Box::new(Type::Void),
                kind: Mutability::Mut,
            }),
            "HWND" => Some(Type::Pointer {
                to: Box::new(Type::Void),
                kind: Mutability::Mut,
            }),
            "HMONITOR" => Some(Type::Pointer {
                to: Box::new(Type::Void),
                kind: Mutability::Mut,
            }),
            "HANDLE" => Some(Type::Pointer {
                to: Box::new(Type::Void),
                kind: Mutability::Mut,
            }),
            "SECURITY_ATTRIBUTES" => Some(Type::Void),
            "DWORD" => Some(Type::UnsignedInt32),
            "LPCWSTR" => Some(Type::Pointer {
                to: Box::new(Type::UnsignedInt16),
                kind: Mutability::Const,
            }),
            "xcb_connection_t" => Some(Type::Void),
            "xcb_visualid_t" => Some(Type::UnsignedInt32),
            "xcb_window_t" => Some(Type::UnsignedInt32),
            "zx_handle_t" => Some(Type::Pointer {
                to: Box::new(Type::Void),
                kind: Mutability::Mut,
            }),
            "GgpStreamDescriptor" => Some(Type::UnsignedInt32),
            "GgpFrameToken" => Some(Type::UnsignedInt64),
            "IDirectFB" => Some(Type::Void),
            "IDirectFBSurface" => Some(Type::Void),
            "_screen_context" => Some(Type::Void),
            "_screen_window" => Some(Type::Void),
            _ => None,
        }
    }

    pub fn char_pointer() -> Type {
        Type::Pointer {
            to: Box::new(Type::Char),
            kind: Mutability::Const,
        }
    }

    pub fn rust_type(&self, source: &Source) -> TokenStream {
        match self {
            Type::Void => quote! { std::ffi::c_void },
            Type::Char => quote! { std::os::raw::c_char },
            Type::SignedChar => quote! { std::os::raw::c_schar },
            Type::UnsignedChar => quote! { std::os::raw::c_uchar },
            Type::Short => quote! { std::os::raw::c_short },
            Type::UnsignedShort => quote! { std::os::raw::c_ushort },
            Type::Int => quote! { std::os::raw::c_int },
            Type::UnsignedInt => quote! { std::os::raw::c_uint },
            Type::Long => quote! { std::os::raw::c_long },
            Type::UnsignedLong => quote! { std::os::raw::c_ulong },
            Type::LongLong => quote! { std::os::raw::c_longlong },
            Type::UnsignedLongLong => quote! { std::os::raw::c_ulonglong },
            Type::Float => quote! { std::os::raw::c_float },
            Type::Double => quote! { std::os::raw::c_double },
            Type::Int8 => quote! { i8 },
            Type::UnsignedInt8 => quote! { u8 },
            Type::Int16 => quote! { i16 },
            Type::UnsignedInt16 => quote! { u16 },
            Type::Int32 => quote! { i32 },
            Type::UnsignedInt32 => quote! { u32 },
            Type::Int64 => quote! { i64 },
            Type::UnsignedInt64 => quote! { u64 },
            Type::Size => quote! { isize },
            Type::UnsignedSize => quote! { usize },
            Type::Bool => quote! { bool },
            Type::CStr => quote! { std::ffi::CStr },
            Type::Unit => quote! { () },
            Type::Named(name) => name.path(source),
            Type::Pointer { to, kind } => {
                let to = to.rust_type(source);
                match kind {
                    Mutability::Const => quote! { *const #to },
                    Mutability::Mut => quote! { *mut #to },
                }
            }
            Type::Reference { to, kind, lifetime } => {
                let to = to.rust_type(source);
                match kind {
                    Mutability::Const => quote! { &#lifetime #to },
                    Mutability::Mut => quote! { &#lifetime mut #to },
                }
            }
            Type::Array { of, length } => {
                let of = of.rust_type(source);
                let length = Literal::usize_unsuffixed(*length);
                quote! { [#of; #length] }
            }
            Type::Tuple(types) => match types.as_slice() {
                [] => quote! { () },
                [single] => single.rust_type(source),
                multiple => {
                    let types = multiple.iter().map(|ty| ty.rust_type(source));
                    quote! { (#(#types),*) }
                }
            },
            Type::Option(of) => {
                let of = of.rust_type(source);
                quote! { Option<#of> }
            }
            Type::SmallVec(of) => {
                let of = of.rust_type(source);
                quote! { crate::SmallVec<#of> }
            }
            Type::Slice { of, kind, lifetime } => {
                let of = of.rust_type(source);
                match kind {
                    Mutability::Const => quote! { &#lifetime [#of] },
                    Mutability::Mut => quote! { &#lifetime mut [#of] },
                }
            }
        }
    }

    pub fn default_value(&self, source: &Source) -> TokenStream {
        let mut ty = self;
        if let Type::Named(Name::Type(type_name)) = self {
            if let Some(basetype) = source.find_basetype(type_name) {
                ty = &basetype.ty;
            }
        }

        match ty {
            Type::Array { .. } => quote! { unsafe { std::mem::zeroed() } },
            Type::Pointer { kind, .. } => match kind {
                Mutability::Const => quote! { std::ptr::null() },
                Mutability::Mut => quote! { std::ptr::null_mut() },
            },
            _ => quote! { Default::default() },
        }
    }

    pub fn pointer_to_ref(self, lifetime: Option<Lifetime>) -> Type {
        match self {
            Type::Pointer { to, kind } => Type::Reference { to, kind, lifetime },
            other => other,
        }
    }

    pub fn pointer_to_slice(self, lifetime: Option<Lifetime>, source: &Source) -> Type {
        match self {
            Type::Pointer { to, kind } => Type::Slice {
                of: Box::new(to.builderify(source)),
                kind,
                lifetime,
            },
            other => other,
        }
    }

    pub fn builderify(self, source: &Source) -> Type {
        if let Type::Named(Name::Type(name)) = &self {
            let structure = source.find_structure(&name).or_else(|| {
                source
                    .find_type_alias(&name)
                    .and_then(|alias| source.find_structure(alias.resolve_to_type_name(source)))
            });

            if let Some(structure) = structure {
                if structure.qualifies_as_builder() {
                    return Type::Named(Name::Type(name.clone().set_builder(true)));
                }
            }
        }

        self
    }

    pub fn map_bool(self) -> Type {
        match self {
            Type::Named(Name::Type(name)) if name == TypeName::bool32() => Type::Bool,
            ty => ty,
        }
    }

    pub fn has_types(&self, types: &[Type]) -> bool {
        match self {
            Type::Pointer { to, .. } => to.has_types(types),
            Type::Reference { to, .. } => to.has_types(types),
            Type::Array { of, .. } => of.has_types(types),
            Type::Tuple(types) => types.iter().any(|ty| ty.has_types(types)),
            Type::Option(ty) => ty.has_types(types),
            Type::SmallVec(ty) => ty.has_types(types),
            Type::Slice { of, .. } => of.has_types(types),
            ty if types.contains(ty) => true,
            _ => false,
        }
    }

    pub fn bitwidth(&self) -> usize {
        match self {
            Type::UnsignedInt32 => 32,
            _ => unimplemented!(),
        }
    }

    pub fn is_flags(&self) -> bool {
        match self {
            Type::Named(Name::Type(name)) => name.is_flags(),
            _ => false,
        }
    }

    pub fn supports_hash_eq(&self, source: &Source) -> bool {
        match self {
            Type::Void | Type::Float | Type::Double | Type::Pointer { .. } => false,
            Type::Char
            | Type::SignedChar
            | Type::UnsignedChar
            | Type::Short
            | Type::UnsignedShort
            | Type::Int
            | Type::UnsignedInt
            | Type::Long
            | Type::UnsignedLong
            | Type::LongLong
            | Type::UnsignedLongLong
            | Type::Int8
            | Type::UnsignedInt8
            | Type::Int16
            | Type::UnsignedInt16
            | Type::Int32
            | Type::UnsignedInt32
            | Type::Int64
            | Type::UnsignedInt64
            | Type::Size
            | Type::UnsignedSize
            | Type::Bool
            | Type::CStr
            | Type::Unit => true,
            Type::Named(name) => name.supports_hash_eq(source),
            Type::Reference { to, .. } => to.supports_hash_eq(source),
            Type::Array { of, .. } => of.supports_hash_eq(source),
            Type::Tuple(types) => types.iter().all(|ty| ty.supports_hash_eq(source)),
            Type::Option(of) => of.supports_hash_eq(source),
            Type::SmallVec(of) => of.supports_hash_eq(source),
            Type::Slice { of, .. } => of.supports_hash_eq(source),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Optional {
    Always,
    // Under certain circumstances
    Sometimes,
    Never,
}

#[derive(Debug, Clone)]
pub struct DeclarationMetadata {
    /// Legal values the declaration can have
    pub values: Vec<String>,
    /// A optional length the declaration can have
    pub length: Option<String>,
    /// Whether the declaration be null under certain circumstances
    pub optional: Optional,
}

impl DeclarationMetadata {
    pub fn empty() -> DeclarationMetadata {
        DeclarationMetadata {
            values: Vec::new(),
            length: None,
            optional: Optional::Never,
        }
    }

    pub fn structure_type(&self) -> Option<EnumVariantName> {
        for value in &self.values {
            if let Ok(name) = EnumVariantName::new(value, &TypeName::structure_type()) {
                if name.prefix_compliant {
                    return Some(name);
                }
            }
        }

        None
    }
}

pub fn declaration_ident(name: &str) -> Option<Ident> {
    // Try to make the identifier valid if it's not
    match syn::parse_str(&name.to_snake_case()) {
        Ok(ident) => ident,
        Err(_) => syn::parse_str(&format!("_{}", name)).ok(),
    }
}

pub fn cleaned_declaration_ident(name: &str) -> Option<Ident> {
    let base = declaration_ident(name)?.to_string();
    let cleaned = &CLEANED_IDENT_REGEX.captures(&base).unwrap()[2];
    Some(format_ident!("{}", cleaned))
}

pub fn len_path(length: &str) -> TokenStream {
    let idents = length.split("->").map(|s| cleaned_declaration_ident(s));
    quote! { #(#idents).* }
}

#[derive(Debug, Clone)]
pub struct Declaration {
    pub name: Option<String>,
    pub ty: Type,
    pub metadata: DeclarationMetadata,
    pub bitwidth: BitWidth,
}

impl Declaration {
    pub fn name(&self) -> &String {
        match &self.name {
            Some(name) => name,
            None => panic!("Declaration has no name: {:?}", self),
        }
    }

    pub fn name_lossy(&self) -> &str {
        match &self.name {
            Some(name) => name.as_str(),
            None => "unnamed",
        }
    }

    pub fn ident(&self) -> Ident {
        let name = self.name_lossy();
        match declaration_ident(name) {
            Some(ident) => ident,
            None => panic!("Invalid declaration name: {:?}", self),
        }
    }

    pub fn cleaned_ident(&self) -> Ident {
        let name = self.name_lossy();
        match cleaned_declaration_ident(name) {
            Some(ident) => ident,
            None => panic!("Invalid declaration name: {:?}", self),
        }
    }

    pub fn structure_type_value(&self, source: &Source) -> Option<TokenStream> {
        self.metadata.structure_type().map(|variant| {
            let structure_type = Type::Named(Name::Type(TypeName::structure_type()));
            assert_eq!(self.ty, structure_type);

            let ty = structure_type.rust_type(source);
            let variant_ident = variant.ident();

            quote! { #ty::#variant_ident }
        })
    }

    pub fn default_impl(&self, source: &Source) -> TokenStream {
        if self.metadata.structure_type().is_some() {
            quote! { Self::STRUCTURE_TYPE }
        } else {
            self.ty.default_value(source)
        }
    }

    pub fn array_indices(&self, other: &[impl Borrow<Declaration>]) -> Option<Vec<usize>> {
        let array_indices: Vec<_> = other
            .iter()
            .enumerate()
            .filter(|(_, array)| {
                let array: &Declaration = (*array).borrow();
                array.metadata.length.as_ref() == Some(self.name())
            })
            .map(|(i, _)| i)
            .collect();

        (!array_indices.is_empty()).then(|| array_indices)
    }

    pub fn is_p_next(&self, kind: Mutability) -> bool {
        self.name() == "pNext"
            && match &self.ty {
                Type::Pointer { to, kind: ty_kind } => **to == Type::Void && kind == *ty_kind,
                _ => false,
            }
    }
}
