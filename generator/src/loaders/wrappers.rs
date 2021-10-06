use super::{value::Value, CommandLevel};
use crate::{
    comment_gen::DocCommentGen,
    declaration::{self, Declaration, Mutability, Optional, Type},
    items::functions::Function,
    name::{Name, TypeName},
    source::Source,
};
use proc_macro2::{Ident, TokenStream};
use quote::quote;

#[derive(Debug, Clone)]
enum ParameterKind {
    Regular,
    Array,
    Optional,
    Passthrough,
    Handle,
    CStr,
    ValueWrittenTo {
        inner: Type,
    },
    ValueWrittenToChained {
        inner: Type,
    },
    ArrayWrittenTo {
        inner: Type,
        length: String,
    },
    LengthByArray {
        array_indices: Vec<usize>,
    },
    LengthByExtraCall {
        array_indices: Vec<usize>,
        inner: Type,
    },
}

impl ParameterKind {
    fn generate_list(
        source: &Source,
        parameters: &[Declaration],
        handle_type: Option<&Type>,
    ) -> Vec<ParameterKind> {
        let mut kinds = vec![None; parameters.len()];
        let mut passthrough = Vec::new();
        for group in 0.. {
            // Finish algorithm if every kind is filled
            if kinds.iter().all(Option::is_some) {
                break;
            }

            let params_iter = parameters.iter().zip(kinds.iter_mut()).enumerate();
            for (i, (param, param_kind)) in params_iter {
                // Don't test for this parameter if it already has a kind
                if param_kind.is_some() {
                    continue;
                }

                let is_passthrough = param.ty.has_types(&[Type::Void]);
                match group {
                    // Apply `Handle` kind if it qualifies as a handle
                    0 => {
                        if i == 0 && Some(&param.ty) == handle_type {
                            *param_kind = Some(ParameterKind::Handle);
                        }
                    }
                    // Apply `CStr` kind if the parameter is a char pointer
                    1 => {
                        if param.ty == Type::char_pointer() {
                            *param_kind = Some(ParameterKind::CStr);
                        }
                    }
                    // Mark the length passthrough if this parameter is passthrough
                    2 => {
                        if let Some(length) = &param.metadata.length {
                            if is_passthrough {
                                passthrough.push(length.as_str());
                            }
                        }
                    }
                    // Apply `Passthrough` kind if applicable
                    3 => {
                        if is_passthrough || passthrough.contains(&param.name().as_str()) {
                            *param_kind = Some(ParameterKind::Passthrough);
                        }
                    }
                    // Apply length kinds
                    4 => {
                        let array_indices = param.array_indices(parameters);
                        if let Some(array_indices) = array_indices {
                            let opt = (&param.metadata.optional, &param.ty, &param.metadata.length);
                            *param_kind = match opt {
                                (Optional::Sometimes, Type::Pointer { kind, to }, None)
                                    if *kind == Mutability::Mut =>
                                {
                                    Some(ParameterKind::LengthByExtraCall {
                                        array_indices,
                                        inner: (**to).clone(),
                                    })
                                }
                                _ => Some(ParameterKind::LengthByArray { array_indices }),
                            };
                        }
                    }
                    // Apply other kinds
                    5 => match (&param.metadata.optional, &param.ty, &param.metadata.length) {
                        (Optional::Never, Type::Pointer { kind, to }, None)
                            if *kind == Mutability::Mut =>
                        {
                            *param_kind = Some(ParameterKind::ValueWrittenTo {
                                inner: (**to).clone(),
                            });

                            if let Type::Named(Name::Type(type_name)) = &**to {
                                if let Some(structure) = source.find_structure(&type_name) {
                                    if structure.has_p_next(Mutability::Mut) {
                                        *param_kind = Some(ParameterKind::ValueWrittenToChained {
                                            inner: (**to).clone(),
                                        });
                                    }
                                }
                            }
                        }
                        (optional, Type::Pointer { kind, to }, Some(length))
                            if matches!(optional, Optional::Never | Optional::Always)
                                && *kind == Mutability::Mut =>
                        {
                            *param_kind = Some(ParameterKind::ArrayWrittenTo {
                                inner: (**to).clone(),
                                length: length.into(),
                            })
                        }
                        (Optional::Always, Type::Named(Name::Type(name)), None)
                            if source.find_handle(name).is_some() =>
                        {
                            *param_kind = Some(ParameterKind::Regular)
                        }
                        (Optional::Always, _, None) => *param_kind = Some(ParameterKind::Optional),
                        (_, _, Some(_)) => *param_kind = Some(ParameterKind::Array),
                        (_, _, None) => *param_kind = Some(ParameterKind::Regular),
                    },
                    _ => unreachable!(),
                }
            }
        }

        kinds.into_iter().map(Option::unwrap).collect()
    }
}

struct ExtraCallInfo {
    array_indices: Vec<usize>,
    length_index: usize,
    param: Declaration,
    cleaned_ident: Ident,
    inner: Type,
}

impl Function {
    pub(super) fn wrapper(
        &self,
        command_level: &CommandLevel,
        comment_gen: &DocCommentGen,
        source: &Source,
    ) -> TokenStream {
        let ident = self.name.pretty_ident();
        //log::trace!("Processing wrapper for `{}`", ident);

        let handle_type = command_level.handle_type();
        let kinds = ParameterKind::generate_list(source, &self.parameters, handle_type.as_ref());

        // Parameters to the user-facing function
        let mut user_params: Vec<Declaration> = Vec::new();
        // Return values of the user-facing function
        let mut return_values: Vec<Value> = Vec::new();
        // Arguments to the main inner function call
        let mut call_args: Vec<TokenStream> = Vec::new();
        // Code that initializes values which are to be returned
        let mut return_init = TokenStream::new();
        // Code that initializes length values
        let mut length_init = TokenStream::new();
        // Information for the generation of an extra call for length
        let mut extra_call_info = None;

        let param_iterator = self
            .parameters
            .iter()
            .cloned()
            .zip(kinds.into_iter())
            .enumerate();

        for (i, (param, kind)) in param_iterator {
            let cleaned_ident = param.cleaned_ident();

            call_args.push(match kind {
                ParameterKind::Regular => {
                    let mut user_param = param;
                    user_param.ty = user_param.ty.pointer_to_ref(None).map_bool();
                    user_params.push(user_param);

                    quote! { #cleaned_ident as _ }
                }
                ParameterKind::Array => {
                    let mut user_param = param;
                    user_param.ty = user_param.ty.pointer_to_slice(None, source);
                    user_params.push(user_param);

                    quote! { #cleaned_ident.as_ptr() as _ }
                }
                ParameterKind::Optional => {
                    let default = param.default_impl(source);

                    let mut user_param = param;
                    user_param.ty = Type::Option(Box::new(user_param.ty.pointer_to_ref(None)));
                    user_params.push(user_param);

                    quote! {
                        match #cleaned_ident {
                            Some(v) => v,
                            None => #default,
                        }
                    }
                }
                ParameterKind::Passthrough => {
                    user_params.push(param);

                    quote! { #cleaned_ident }
                }
                ParameterKind::Handle => {
                    quote! { self.handle }
                }
                ParameterKind::CStr => {
                    let default = param.default_impl(source);

                    let mut user_param = param;
                    user_param.ty = Type::Option(Box::new(Type::Reference {
                        to: Box::new(Type::CStr),
                        kind: Mutability::Const,
                        lifetime: None,
                    }));
                    user_params.push(user_param);

                    quote! {
                        match #cleaned_ident {
                            Some(v) => v.as_ptr(),
                            None => #default,
                        }
                    }
                }
                ParameterKind::ValueWrittenTo { inner } => {
                    let mut inner_declaration = param;
                    inner_declaration.ty = inner.clone();
                    let default = inner_declaration.default_impl(source);

                    return_init.extend(quote! {
                        let mut #cleaned_ident = #default;
                    });

                    return_values.push(Value::new(quote! { #cleaned_ident }, inner).map_bool());

                    quote! { &mut #cleaned_ident }
                }
                ParameterKind::ValueWrittenToChained { inner } => {
                    let mut user_param = param.clone();
                    user_param.ty = Type::Option(Box::new(inner.clone()));
                    user_params.push(user_param);

                    let mut inner_declaration = param;
                    inner_declaration.ty = inner.clone();
                    let default = inner_declaration.default_impl(source);

                    return_init.extend(quote! {
                        let mut #cleaned_ident = match #cleaned_ident {
                            Some(v) => v,
                            None => #default,
                        };
                    });

                    return_values.push(Value::new(quote! { #cleaned_ident }, inner).map_bool());

                    quote! { &mut #cleaned_ident }
                }
                ParameterKind::ArrayWrittenTo { inner, length } => {
                    let mut inner_declaration = param;
                    inner_declaration.ty = inner.clone();
                    let default = inner_declaration.default_impl(source);

                    let len_path = declaration::len_path(&length);

                    let vec = Value::vec_cloning(inner, default, quote! { #len_path as _ });
                    let vec_expr = vec.expr;
                    return_init.extend(quote! {
                        let mut #cleaned_ident = #vec_expr;
                    });

                    return_values.push(Value::new(quote! { #cleaned_ident }, vec.ty).map_bool());

                    quote! { #cleaned_ident.as_mut_ptr() }
                }
                ParameterKind::LengthByArray { array_indices } => {
                    #[allow(clippy::match_like_matches_macro)]
                    let length_exprs = array_indices
                        .into_iter()
                        .map(|array_index| &self.parameters[array_index])
                        .filter(|array| match &array.ty {
                            Type::Pointer { kind, .. } if *kind == Mutability::Const => true,
                            _ => false,
                        })
                        .enumerate()
                        .map(|(i, array)| {
                            let ident = array.cleaned_ident();
                            match i {
                                0 => quote! { #ident.len() },
                                _ => quote! { .min(#ident.len()) },
                            }
                        });
                    let length_expr = quote! { #(#length_exprs)* };
                    if length_expr.is_empty() {
                        panic!("Empty LengthByArray expr in {:?}", self.name.no_pfn);
                    }

                    length_init.extend(quote! {
                        let #cleaned_ident = #length_expr;
                    });

                    quote! { #cleaned_ident as _ }
                }
                ParameterKind::LengthByExtraCall {
                    array_indices,
                    inner,
                } => {
                    assert!(extra_call_info.is_none());
                    extra_call_info = Some(ExtraCallInfo {
                        array_indices,
                        length_index: i,
                        param: param.clone(),
                        cleaned_ident: cleaned_ident.clone(),
                        inner: inner.clone(),
                    });

                    let mut user_param = param;
                    user_param.ty = Type::Option(Box::new(inner));
                    user_params.push(user_param);

                    quote! { &mut #cleaned_ident }
                }
            });
        }

        let extra_call = extra_call_info.map(|info| {
            let mut inner_declaration = info.param;
            inner_declaration.ty = info.inner.clone();
            let default = inner_declaration.default_impl(source);

            let length_index = info.length_index;
            let array_indices = info.array_indices;
            let call_args = call_args.iter().enumerate().map(|(i, arg)| {
                if i == length_index {
                    quote! { &mut v }
                } else if array_indices.contains(&i) {
                    quote! { std::ptr::null_mut() }
                } else {
                    arg.clone()
                }
            });

            let cleaned_ident = info.cleaned_ident;

            quote! {
                let mut #cleaned_ident = match #cleaned_ident {
                    Some(v) => v,
                    None => {
                        let mut v = #default;
                        _function(#(#call_args),*);
                        v
                    }
                };
            }
        });

        let doc = self.doc(comment_gen);
        let param_idents = user_params.iter().map(|param| param.cleaned_ident());
        let param_types = user_params.iter().map(|param| param.ty.rust_type(source));
        let (return_expr, return_type) = match &self.return_type {
            Type::Unit => Value::tuple(&return_values).expr_type(source),
            result if result == &Type::Named(Name::Type(TypeName::result())) => {
                let (expr, ty) = Value::tuple(&return_values).expr_type(source);
                let expr = quote! { crate::utils::VulkanResult::new(_return, #expr) };
                let ty = quote! { crate::utils::VulkanResult<#ty> };
                (expr, ty)
            }
            other => {
                return_values.push(Value::new(quote! { _return }, other.clone()).map_bool());
                Value::tuple(&return_values).expr_type(source)
            }
        };

        let doc_alias = &self.name.no_pfn;
        quote! {
            #[inline]
            #[track_caller]
            #[doc = #doc]
            #[doc(alias = #doc_alias)]
            pub unsafe fn #ident(&self, #(#param_idents: #param_types),*) -> #return_type {
                let _function = self.#ident.expect(crate::NOT_LOADED_MESSAGE);
                #extra_call
                #length_init
                #return_init
                let _return = _function(#(#call_args),*);
                #return_expr
            }
        }
    }
}
