use super::{BitWidth, Structure, StructureField};
use crate::{
    comment_gen::DocCommentGen,
    declaration::{self, Mutability, Type},
    name::{Name, TypeName},
    origin::Origin,
    source::Source,
};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::collections::HashMap;
use syn::Lifetime;

thread_local!(static OVERRIDE_LIST: HashMap<TypeName, Vec<Override>> = override_list());
#[allow(clippy::redundant_clone)]
fn override_list() -> HashMap<TypeName, Vec<Override>> {
    let lifetime_a = Lifetime::new("'a", Span::call_site());

    let mut map = HashMap::new();
    map.insert(
        TypeName::new("VkShaderModuleCreateInfo"),
        vec![
            Override {
                index: 3,
                kind: FieldKind::Ignore,
            },
            Override {
                index: 4,
                kind: FieldKind::Overridden {
                    ty: Type::Slice {
                        of: Box::new(Type::UnsignedInt32),
                        kind: Mutability::Const,
                        lifetime: Some(lifetime_a.clone()),
                    },
                    body: quote! {
                        self.0.p_code = code.as_ptr() as _;
                        self.0.code_size = code.len() * 4;
                    },
                },
            },
        ],
    );

    map.insert(
        TypeName::new("VkPipelineMultisampleStateCreateInfo"),
        vec![Override {
            index: 6,
            kind: FieldKind::Overridden {
                ty: Type::Slice {
                    of: Box::new(Type::Named(Name::Type(TypeName::new("VkSampleMask")))),
                    kind: Mutability::Const,
                    lifetime: Some(lifetime_a.clone()),
                },
                body: quote! {
                    self.0.p_sample_mask = if sample_mask.is_empty() {
                        std::ptr::null()
                    } else {
                        sample_mask.as_ptr() as _
                    };
                },
            },
        }],
    );

    map.insert(
        TypeName::new("VkPipelineViewportStateCreateInfo"),
        vec![
            Override {
                index: 3,
                kind: FieldKind::Regular,
            },
            Override {
                index: 5,
                kind: FieldKind::Regular,
            },
        ],
    );

    map.insert(
        TypeName::new("VkDescriptorSetLayoutBinding"),
        vec![Override {
            index: 2,
            kind: FieldKind::Regular,
        }],
    );

    map.insert(
        TypeName::new("VkAccelerationStructureBuildGeometryInfoKHR"),
        vec![Override {
            index: 9,
            kind: FieldKind::Ignore,
        }],
    );

    map.insert(
        TypeName::new("VkAccelerationStructureVersionInfoKHR"),
        vec![Override {
            index: 2,
            kind: FieldKind::Overridden {
                ty: Type::Reference {
                    to: Box::new(Type::Array {
                        of: Box::new(Type::UnsignedInt8),
                        length: 32, // 2 * VK_UUID_SIZE
                    }),
                    kind: Mutability::Const,
                    lifetime: Some(lifetime_a.clone()),
                },
                body: quote! {
                    self.0.p_version_data = version_data.as_ptr() as _;
                },
            },
        }],
    );

    map
}

struct Override {
    index: usize,
    kind: FieldKind,
}

#[derive(Debug, Clone)]
enum FieldKind {
    Regular,
    Bitfield,
    Array { length: String },
    CStr,
    Passthrough,
    Ignore,
    Overridden { ty: Type, body: TokenStream },
}

impl FieldKind {
    fn generate_list(fields: &[StructureField]) -> Vec<FieldKind> {
        let main_decls: Vec<_> = fields.iter().map(|field| field.main_decl()).collect();

        let mut kinds = vec![None; main_decls.len()];
        let mut passthrough = Vec::new();
        for group in 0.. {
            // Finish algorithm if every kind is filled
            if kinds.iter().all(Option::is_some) {
                break;
            }

            let fields_iter = main_decls.iter().zip(kinds.iter_mut()).enumerate();
            for (i, (field, field_kind)) in fields_iter {
                // Don't test for this field if it already has a kind
                if field_kind.is_some() {
                    continue;
                }

                let is_passthrough = field.ty.has_types(&[Type::Void]);
                match group {
                    // Apply `BitField` kind
                    0 => {
                        if let StructureField::Bitfield(..) = fields[i] {
                            *field_kind = Some(FieldKind::Bitfield);
                        }
                    }
                    // Apply `Ignore` kinds
                    1 => match (i, field.name_lossy(), &field.ty) {
                        (0, "sType", Type::Named(Name::Type(name)))
                            if *name == TypeName::structure_type() =>
                        {
                            *field_kind = Some(FieldKind::Ignore)
                        }
                        (1, "pNext", Type::Pointer { to, .. }) if **to == Type::Void => {
                            *field_kind = Some(FieldKind::Ignore)
                        }
                        _ => (),
                    },
                    // Mark the length passthrough if this parameter is passthrough
                    2 => {
                        if let Some(length) = &field.metadata.length {
                            if is_passthrough {
                                passthrough.push(length.as_str());
                            }
                        }
                    }
                    // Apply `Passthrough` kind if applicable
                    3 => {
                        if is_passthrough || passthrough.contains(&field.name().as_str()) {
                            *field_kind = Some(FieldKind::Passthrough);
                        }
                    }
                    // Apply `Ignore` kind if the field is a length
                    4 => {
                        if field.array_indices(&main_decls).is_some() {
                            *field_kind = Some(FieldKind::Ignore);
                        }
                    }
                    // Apply `CStr` kind if the field is a char pointer
                    5 => {
                        if field.ty == Type::char_pointer() {
                            *field_kind = Some(FieldKind::CStr);
                        }
                    }
                    // Apply other kinds
                    6 => match &field.metadata.length {
                        Some(length) => {
                            *field_kind = Some(FieldKind::Array {
                                length: length.clone(),
                            })
                        }
                        None => *field_kind = Some(FieldKind::Regular),
                    },
                    _ => unreachable!(),
                }
            }
        }

        kinds.into_iter().map(Option::unwrap).collect()
    }
}

impl Structure {
    pub(super) fn builder(
        &self,
        source: &Source,
        comment_gen: &DocCommentGen,
    ) -> HashMap<Origin, TokenStream> {
        if !self.qualifies_as_builder() {
            return HashMap::new();
        }

        let origin = self.origin.as_ref().expect("Structure missing origin");

        let inner_ident = self.name.ident();
        //log::debug!("Processing builder for `{}`", inner_ident);

        let builder_name = self.name.clone().set_builder(true);
        let ident = builder_name.ident();

        let doc = comment_gen.def(
            Some(&self.name.original),
            format!("Builder of [`{}`]", inner_ident),
            self.origin.as_ref(),
        );

        let mut field_kinds = FieldKind::generate_list(&self.fields);
        OVERRIDE_LIST.with(|override_list| {
            if let Some(overrides) = override_list.get(&self.name) {
                for field_override in overrides {
                    field_kinds[field_override.index] = field_override.kind.clone();
                }
            }
        });

        let lifetime_a = Lifetime::new("'a", Span::call_site());
        let mut bitfield_start = 0;
        let field_builders = self
            .fields
            .iter()
            .zip(field_kinds.into_iter())
            .flat_map(|(field, kind)| match field {
                StructureField::Normal(decl) => vec![(true, field, decl, kind)],
                StructureField::Bitfield(decls) => decls
                    .iter()
                    .enumerate()
                    .map(|(i, decl)| (i == 0, field, decl, kind.clone()))
                    .collect(),
            })
            .map(|(new_field, field, decl, kind)| {
                let raw_ident = decl.ident();
                let ident = decl.cleaned_ident();

                let ty;
                let body;
                match kind {
                    FieldKind::Regular => {
                        ty = decl
                            .ty
                            .clone()
                            .pointer_to_ref(Some(lifetime_a.clone()))
                            .map_bool();

                        body = quote! {
                            self.0 .#raw_ident = #ident as _;
                        };
                    }
                    FieldKind::Bitfield => {
                        ty = decl
                            .ty
                            .clone()
                            .pointer_to_ref(Some(lifetime_a.clone()))
                            .map_bool();

                        let field_ident = field.ident();
                        let field_access = quote! { self.0 .#field_ident };

                        let src_int_ident = if decl.ty.is_flags() {
                            quote! { #ident.bits() }
                        } else {
                            quote! { #ident }
                        };

                        if new_field {
                            bitfield_start = 0;
                        }

                        let bitwidth = match decl.bitwidth {
                            BitWidth::Full => unreachable!(),
                            BitWidth::Partial(bitwidth) => bitwidth,
                        };

                        let start = bitfield_start;
                        let end = start + bitwidth - 1;
                        body = quote! {
                            #field_access =
                                crate::bits_copy!(#field_access, #src_int_ident, #start, #end);
                        };

                        bitfield_start += bitwidth;
                    }
                    FieldKind::Array { length } => {
                        let len_ident = match declaration::declaration_ident(&length) {
                            Some(ident) => ident,
                            None => {
                                panic!(
                                    "Custom builder override required - \
                                        builder: {}, field: {}, len: {:?}",
                                    inner_ident, ident, length
                                )
                            }
                        };

                        ty = decl
                            .ty
                            .clone()
                            .pointer_to_slice(Some(lifetime_a.clone()), source);

                        body = quote! {
                            self.0 .#raw_ident = #ident.as_ptr() as _;
                            self.0 .#len_ident = #ident.len() as _;
                        };
                    }
                    FieldKind::CStr => {
                        ty = Type::Reference {
                            to: Box::new(Type::CStr),
                            kind: Mutability::Const,
                            lifetime: Some(lifetime_a.clone()),
                        };

                        body = quote! {
                            self.0 .#raw_ident = #ident.as_ptr();
                        };
                    }
                    FieldKind::Passthrough => {
                        ty = decl.ty.clone();
                        body = quote! {
                            self.0 .#raw_ident = #ident;
                        };
                    }
                    FieldKind::Ignore => {
                        return TokenStream::new();
                    }
                    FieldKind::Overridden {
                        ty: ty_val,
                        body: body_val,
                    } => {
                        ty = ty_val;
                        body = body_val;
                    }
                };

                let ty = ty.rust_type(source);
                quote! {
                    #[inline]
                    pub fn #ident(mut self, #ident: #ty) -> Self {
                        #body
                        self
                    }
                }
            });

        let extendable_from_kind = match (
            self.has_p_next(Mutability::Const),
            self.has_p_next(Mutability::Mut),
        ) {
            (true, false) => Some(quote! { ExtendableFromConst }),
            (false, true) => Some(quote! { ExtendableFromMut }),
            (false, false) => None,
            (true, true) => Some(quote! { unreachable!() }),
        };

        let mut tokens = HashMap::new();
        tokens.insert(
            origin.clone(),
            quote! {
                impl #inner_ident {
                    #[inline]
                    pub fn into_builder<'a>(self) -> #ident<'a> {
                        #ident(self, std::marker::PhantomData)
                    }
                }
            },
        );

        for other in source
            .structures
            .iter()
            .filter(|structure| structure.metadata.extends.contains(&self.name))
        {
            let this_path = origin.module_path();
            let other_origin = other
                .origin
                .as_ref()
                .expect("(other) Structure missing origin");

            let mut other_name = other.name.clone();
            let other_ident = other_name.ident();
            other_name.builder = true;
            let other_builder_ident = other_name.ident();

            tokens
                .entry(other_origin.clone())
                .or_insert_with(TokenStream::new)
                .extend(quote! {
                    impl<'a> crate::#extendable_from_kind<'a, #other_ident>
                        for crate::#this_path#ident<'a> {}

                    impl<'a> crate::#extendable_from_kind<'a, #other_builder_ident<'_>>
                        for crate::#this_path#ident<'a> {}
                });
        }

        tokens.get_mut(origin).unwrap().extend(quote! {
            #[derive(Copy, Clone)]
            #[doc = #doc]
            #[repr(transparent)]
            pub struct #ident<'a>(#inner_ident, std::marker::PhantomData<&'a ()>);

            impl<'a> #ident<'a> {
                #[inline]
                pub fn new() -> #ident<'a> {
                    #ident(Default::default(), std::marker::PhantomData)
                }

                #(#field_builders)*

                #[inline]
                /// Discards all lifetime information.
                /// Use the `Deref` and `DerefMut` implementations if possible.
                pub fn build(self) -> #inner_ident {
                    self.0
                }
            }

            impl<'a> std::default::Default for #ident<'a> {
                fn default() -> #ident<'a> {
                    Self::new()
                }
            }

            impl<'a> std::fmt::Debug for #ident<'a> {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    std::fmt::Debug::fmt(&self.0, f)
                }
            }

            impl<'a> std::ops::Deref for #ident<'a> {
                type Target = #inner_ident;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl<'a> std::ops::DerefMut for #ident<'a> {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }
        });

        tokens
    }
}
