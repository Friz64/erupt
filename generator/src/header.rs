pub mod eval;
mod root_gen;

use crate::{
    declaration::{Declaration, DeclarationMetadata, Mutability, Optional, Type},
    items::{
        basetypes::Basetype,
        constants::Constant,
        enums::{EnumKind, EnumVariant},
        functions::Function,
        structures::Structure,
    },
    name::{FunctionName, Name, TypeName},
    source::NotApplicable,
    Opt, XmlNode,
};
use eval::{Expression, Literal};
use lang_c::{
    ast::{
        ArraySize, DeclarationSpecifier, Declarator, DeclaratorKind, DerivedDeclarator,
        ExternalDeclaration, PointerQualifier, SpecifierQualifier, StorageClassSpecifier,
        TranslationUnit, TypeQualifier, TypeSpecifier,
    },
    driver::{self, Config},
    span::Node,
};
use std::{
    convert::TryFrom,
    fmt::Debug,
    fs, mem,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub enum BitWidth {
    Full,
    Partial(usize),
}

#[derive(Debug)]
pub struct DeclarationTypeInfo(Vec<SpecifierQualifier>);

impl From<&[Node<SpecifierQualifier>]> for DeclarationTypeInfo {
    fn from(info: &[Node<SpecifierQualifier>]) -> Self {
        DeclarationTypeInfo(info.iter().map(|node| node.node.clone()).collect())
    }
}

impl TryFrom<&[Node<DeclarationSpecifier>]> for DeclarationTypeInfo {
    type Error = NotApplicable;

    fn try_from(info: &[Node<DeclarationSpecifier>]) -> Result<Self, Self::Error> {
        let vec: Result<Vec<_>, _> = info
            .iter()
            .filter_map(|node| match &node.node {
                DeclarationSpecifier::TypeSpecifier(specifier) => {
                    Some(Ok(SpecifierQualifier::TypeSpecifier(specifier.clone())))
                }
                DeclarationSpecifier::TypeQualifier(qualifier) => {
                    Some(Ok(SpecifierQualifier::TypeQualifier(qualifier.clone())))
                }
                DeclarationSpecifier::StorageClass(storage_class) => match &storage_class.node {
                    StorageClassSpecifier::Typedef => None,
                    _ => Some(Err(NotApplicable)),
                },
                _ => Some(Err(NotApplicable)),
            })
            .collect();

        Ok(DeclarationTypeInfo(vec?))
    }
}

#[derive(Debug)]
pub struct DeclarationInfo<'a> {
    pub type_info: DeclarationTypeInfo,
    pub declarator: Option<&'a Declarator>,
    pub bitwidth: BitWidth,
}

impl From<XmlNode<'_, '_>> for DeclarationMetadata {
    fn from(node: XmlNode) -> Self {
        let values = node
            .attribute("values")
            .map(|values| values.split(',').map(|s| s.to_string()).collect())
            .unwrap_or_else(Vec::new);

        let length = node
            .attribute("altlen")
            .or_else(|| node.attribute("len"))
            .and_then(|values| {
                values
                    .split(',')
                    .filter(|s| !(s.starts_with("latexmath:") || *s == "null-terminated"))
                    .map(|s| s.to_string())
                    .next()
            });

        let optional = node
            .attribute("optional")
            .map(|v| match (v.contains("true"), v.contains("false")) {
                (true, false) => Optional::Always,
                (true, true) => Optional::Sometimes,
                (false, _) => Optional::Never,
            })
            .unwrap_or(Optional::Never);

        DeclarationMetadata {
            values,
            length,
            optional,
        }
    }
}

impl<'a, T> From<T> for Declaration
where
    T: Into<DeclarationInfo<'a>>,
{
    fn from(info: T) -> Self {
        let info: DeclarationInfo = info.into();

        let mut signed = false;
        let mut unsigned = false;
        let mut long = false;
        let mut primary_type = None;
        let mut next_ptr_const = false;
        for type_info in &info.type_info.0 {
            match type_info {
                SpecifierQualifier::TypeSpecifier(specifier) => match &specifier.node {
                    TypeSpecifier::Unsigned => unsigned = true,
                    TypeSpecifier::Signed => signed = true,
                    TypeSpecifier::Int if primary_type.is_some() => (),
                    TypeSpecifier::Long if primary_type.is_some() => long = true,
                    other => {
                        assert_eq!(primary_type, None);
                        primary_type = Some(other);
                    }
                },
                SpecifierQualifier::TypeQualifier(qualifier) => {
                    if let TypeQualifier::Const = &qualifier.node {
                        next_ptr_const = true;
                    }
                }
                SpecifierQualifier::Extension(_) => (),
            }
        }

        let mut ty = match (primary_type, unsigned, signed, long) {
            (Some(TypeSpecifier::TypedefName(name)), false, false, false) => {
                match name.node.name.as_str() {
                    "int8_t" => Type::Int8,
                    "uint8_t" => Type::UnsignedInt8,
                    "int16_t" => Type::Int16,
                    "uint16_t" => Type::UnsignedInt16,
                    "int32_t" => Type::Int32,
                    "uint32_t" => Type::UnsignedInt32,
                    "int64_t" => Type::Int64,
                    "uint64_t" => Type::UnsignedInt64,
                    "ssize_t" => Type::Size,
                    "size_t" => Type::UnsignedSize,
                    other => Type::try_from_external(other).unwrap_or_else(|| Type::guess(other)),
                }
            }
            (Some(TypeSpecifier::Struct(struct_ty)), false, false, false) => {
                match &struct_ty.node.identifier {
                    Some(identifier) => {
                        let name = &identifier.node.name;

                        Type::try_from_external(name)
                            .unwrap_or_else(|| Type::Named(Name::Type(TypeName::new(name))))
                    }
                    _ => Type::Void,
                }
            }
            (Some(TypeSpecifier::Enum(_)), false, false, false) => Type::Void,
            (Some(TypeSpecifier::Void), false, false, false) => Type::Void,
            (Some(TypeSpecifier::Char), false, false, false) => Type::Char,
            (Some(TypeSpecifier::Char), true, false, false) => Type::UnsignedChar,
            (Some(TypeSpecifier::Char), false, true, false) => Type::SignedChar,
            (Some(TypeSpecifier::Short), false, _, false) => Type::Short,
            (Some(TypeSpecifier::Short), true, false, false) => Type::UnsignedShort,
            (Some(TypeSpecifier::Int), false, _, false) => Type::Int,
            (Some(TypeSpecifier::Int), true, false, false) => Type::UnsignedInt,
            (Some(TypeSpecifier::Long), false, _, false) => Type::Long,
            (Some(TypeSpecifier::Long), true, false, false) => Type::UnsignedLong,
            (Some(TypeSpecifier::Long), false, _, true) => Type::LongLong,
            (Some(TypeSpecifier::Long), true, false, true) => Type::UnsignedLongLong,
            (Some(TypeSpecifier::Float), false, false, false) => Type::Float,
            (Some(TypeSpecifier::Double), false, false, false) => Type::Double,
            unexpected => panic!("Unexpected case: {:?} from {:?}", unexpected, info),
        };

        let mut derive_stack = Vec::new();
        let name = info.declarator.map(|declarator| {
            derive_stack.extend_from_slice(&declarator.derived);
            let mut current = &declarator.kind.node;
            while let DeclaratorKind::Declarator(declarator) = current {
                current = &declarator.node.kind.node;
                derive_stack.extend_from_slice(&declarator.node.derived);
            }

            match current {
                DeclaratorKind::Identifier(identifier) => &identifier.node.name,
                _ => panic!("Declarator has no identifier"),
            }
        });

        // Reverse multidimensional arrays
        let mut inversion_ranges = Vec::new();
        let mut start = None;
        let mut iterator = derive_stack
            .iter()
            .map(|derived| matches!(derived.node, DerivedDeclarator::Array(_)))
            .enumerate()
            .peekable();

        while let Some((i, is_array)) = iterator.next() {
            if is_array {
                let next_is_array = iterator
                    .peek()
                    .map(|(_, is_array)| *is_array)
                    .unwrap_or(false);

                if let Some(start) = start {
                    if !next_is_array {
                        inversion_ranges.push(start..=i);
                    }
                } else if next_is_array {
                    start = Some(i);
                }
            }
        }

        for range in inversion_ranges {
            derive_stack[range].reverse();
        }

        for derived in &derive_stack {
            match &derived.node {
                DerivedDeclarator::Pointer(qualifiers) => {
                    let this_ptr_const = next_ptr_const;

                    next_ptr_const = false;
                    for qualifier in qualifiers {
                        if let PointerQualifier::TypeQualifier(ty) = &qualifier.node {
                            if let TypeQualifier::Const = &ty.node {
                                next_ptr_const = true;
                            }
                        }
                    }

                    ty = Type::Pointer {
                        to: Box::new(ty),
                        kind: Mutability::new(this_ptr_const),
                    };
                }
                DerivedDeclarator::Array(declarator) => match &declarator.node.size {
                    ArraySize::VariableExpression(expression) => {
                        let val = match Expression::from(&expression.node).eval_to_literal() {
                            Literal::Int32(val) => val as usize,
                            Literal::Int64(val) => val as usize,
                            Literal::UnsignedInt32(val) => val as usize,
                            Literal::UnsignedInt64(val) => val as usize,
                            unexpected => panic!(
                                "Unexpected array size literal: {:?} from {:?}",
                                unexpected, info
                            ),
                        };

                        ty = Type::Array {
                            of: Box::new(ty),
                            length: val,
                        }
                    }
                    unexpected => panic!("Unexpected array size: {:?} from {:?}", unexpected, info),
                },
                DerivedDeclarator::Function(_) => (),
                unexpected => panic!(
                    "Unexpected derived declarator: {:?} from {:?}",
                    unexpected, info
                ),
            }
        }

        Declaration {
            name: name.cloned(),
            ty,
            metadata: DeclarationMetadata::empty(),
            bitwidth: info.bitwidth,
        }
    }
}

#[derive(Default, Debug)]
pub struct HeaderSource {
    pub structures: Vec<Structure>,
    pub functions: Vec<Function>,
    pub basetypes: Vec<Basetype>,
    pub constants: Vec<Constant>,
    pub enum_variants: Vec<EnumVariant>,
}

impl HeaderSource {
    pub fn new(
        registry: XmlNode,
        headers_include: &Path,
        include_vulkan: &Path,
        other_includes_headers: &[PathBuf],
        opt: &Opt,
    ) -> HeaderSource {
        let header_config = Config {
            cpp_command: opt.preprocessor.display().to_string(),
            cpp_options: vec![
                "-DVK_NO_PROTOTYPES".into(),
                "-DVK_ENABLE_BETA_EXTENSIONS".into(),
                format!("-I{}", headers_include.display()),
                format!("-I{}", include_vulkan.display()),
                "-E".into(),
            ],
            flavor: driver::Flavor::ClangC11,
        };

        let root_header_path = Path::new("generator/root.h");
        root_gen::generate(
            &root_header_path,
            include_vulkan,
            &other_includes_headers,
            registry,
        );

        let unit = &driver::parse(&header_config, root_header_path)
            .expect("Failed to parse header")
            .unit;

        // toggle manually
        if false {
            log::info!("Writing header_debug");

            let debug_print = format!("{:#?}", unit);
            fs::write("header_debug", debug_print).expect("Failed to write header_debug");
        }

        unit.into()
    }

    pub fn take_structure(&mut self, name: &str) -> Option<Structure> {
        self.structures
            .iter()
            .position(|structure| &*structure.name.original == name)
            .map(|idx| self.structures.swap_remove(idx))
    }

    pub fn take_function(&mut self, name: &FunctionName) -> Option<Function> {
        self.functions
            .iter()
            .position(|function| &function.name == name)
            .map(|idx| self.functions.swap_remove(idx))
    }

    pub fn take_basetype(&mut self, name: &str) -> Option<Basetype> {
        self.basetypes
            .iter()
            .position(|basetype| &*basetype.name.original == name)
            .map(|idx| self.basetypes.swap_remove(idx))
    }

    pub fn take_constants(&mut self) -> Vec<Constant> {
        mem::take(&mut self.constants)
    }

    pub fn take_enum_variants(&mut self, kind: &EnumKind) -> Vec<EnumVariant> {
        let mut variants = Vec::new();
        let mut i = 0;
        while i != self.enum_variants.len() {
            if self.enum_variants[i].name.is_from(kind) {
                variants.push(self.enum_variants.remove(i));
            } else {
                i += 1;
            }
        }

        variants
    }
}

impl From<&TranslationUnit> for HeaderSource {
    fn from(unit: &TranslationUnit) -> Self {
        let mut structures = Vec::new();
        let mut functions = Vec::new();
        let mut basetypes = Vec::new();
        let mut constants = Vec::new();
        let mut enum_variants = Vec::new();

        for external in &unit.0 {
            if let ExternalDeclaration::Declaration(declaration) = &external.node {
                if let Ok(constant) = Constant::try_from(&declaration.node) {
                    constants.push(constant);
                } else if let Ok(structure) = Structure::try_from(&declaration.node) {
                    structures.push(structure);
                } else if let Ok(function) = Function::try_from(&declaration.node) {
                    functions.push(function);
                } else if let Ok(enum_variant) = EnumVariant::all_from(&declaration.node) {
                    enum_variants.extend(enum_variant.into_iter());
                } else if let Ok(basetype) = Basetype::try_from(&declaration.node) {
                    basetypes.push(basetype);
                }
            }
        }

        HeaderSource {
            structures,
            functions,
            basetypes,
            constants,
            enum_variants,
        }
    }
}
