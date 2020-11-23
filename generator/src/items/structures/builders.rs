use super::Structure;
use crate::{
    comment_gen::DocCommentGen,
    declaration::{self, Declaration, Type},
    name::{Name, TypeName},
    source::Source,
};
use declaration::Mutability;
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
                    self.0.p_sample_mask = sample_mask.as_ptr() as _;
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
    Array { length: String },
    CStr,
    Passthrough,
    Ignore,
    Overridden { ty: Type, body: TokenStream },
}

impl FieldKind {
    fn generate_list(fields: &[Declaration]) -> Vec<FieldKind> {
        let mut kinds = vec![None; fields.len()];
        let mut passthrough = Vec::new();
        for group in 0.. {
            // Finish algorithm if every kind is filled
            if kinds.iter().all(Option::is_some) {
                break;
            }

            let fields_iter = fields.iter().zip(kinds.iter_mut()).enumerate();
            for (i, (field, field_kind)) in fields_iter {
                // Don't test for this field if it already has a kind
                if field_kind.is_some() {
                    continue;
                }

                let is_passthrough = field.ty.has_types(&[Type::Void]);
                match group {
                    // Apply `Ignore` kinds for sType and pNext
                    0 => match (i, field.name_lossy(), &field.ty) {
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
                    1 => {
                        if let Some(length) = &field.metadata.length {
                            if is_passthrough {
                                passthrough.push(length.as_str());
                            }
                        }
                    }
                    // Apply `Passthrough` kind if applicable
                    2 => {
                        if is_passthrough || passthrough.contains(&field.name().as_str()) {
                            *field_kind = Some(FieldKind::Passthrough);
                        }
                    }
                    // Apply `Ignore` kind if the field is a length
                    3 => {
                        if field.array_indices(fields).is_some() {
                            *field_kind = Some(FieldKind::Ignore);
                        }
                    }
                    // Apply `CStr` kind if the field is a char pointer
                    4 => {
                        if field.ty == Type::char_pointer() {
                            *field_kind = Some(FieldKind::CStr);
                        }
                    }
                    // Apply other kinds
                    5 => match &field.metadata.length {
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
    pub(super) fn builder(&self, source: &Source, comment_gen: &DocCommentGen) -> TokenStream {
        if !self.qualifies_as_builder() {
            return TokenStream::new();
        }

        let inner_ident = self.name.ident();
        //log::debug!("Processing builder for `{}`", inner_ident);

        let builder_name = self.name.clone().set_builder(true);
        let ident = builder_name.ident();

        let doc = comment_gen.def(
            Some(&self.name.original),
            format!(
                "Builder of [`{name}`](struct.{name}.html)",
                name = inner_ident
            ),
            None,
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
        let field_builders =
            self.fields
                .iter()
                .zip(field_kinds.into_iter())
                .map(|(field, kind)| {
                    let raw_ident = field.ident();
                    let ident = field.cleaned_ident();

                    let ty;
                    let body;
                    match kind {
                        FieldKind::Regular => {
                            ty = field
                                .ty
                                .clone()
                                .pointer_to_ref(Some(lifetime_a.clone()))
                                .map_bool();

                            body = quote! {
                                self.0 .#raw_ident = #ident as _;
                            };
                        }
                        FieldKind::Array { length } => {
                            let len_ident = match declaration::declaration_ident(&length) {
                                Some(ident) => ident,
                                None => {
                                    panic!(
                                        "Custom builder override required:\
                                        builder: {}, field: {}, len: {:?}",
                                        inner_ident, ident, length
                                    )
                                }
                            };

                            ty = field
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
                            ty = field.ty.clone();
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

        let extends = source
            .structures
            .iter()
            .filter(|structure| structure.metadata.extends.contains(&self.name))
            .map(|other| {
                let other_path = other
                    .origin
                    .as_ref()
                    .expect("Structure has no origin")
                    .module_path();

                let mut other_name = other.name.clone();
                let other_ident = other_name.ident();
                other_name.builder = true;
                let other_builder_ident = other_name.ident();

                quote! {
                    impl<'a> crate::ExtendableFrom<'a, crate::#other_path#other_ident>
                        for #ident<'a> {}

                    impl<'a> crate::ExtendableFrom<'a, crate::#other_path#other_builder_ident<'_>>
                        for #ident<'a> {}
                }
            });

        quote! {
            impl #inner_ident {
                #[inline]
                pub fn into_builder<'a>(self) -> #ident<'a> {
                    #ident(self, std::marker::PhantomData)
                }
            }

            #(#extends)*

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
                #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
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
        }
    }
}
