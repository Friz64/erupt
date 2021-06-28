use crate::{
    declaration::Type,
    name::{Name, TypeName},
    source::Source,
};
use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug, Clone)]
pub struct Value {
    pub expr: TokenStream,
    pub ty: Type,
}

impl Value {
    pub fn new(expr: TokenStream, ty: Type) -> Value {
        Value { expr, ty }
    }

    pub fn expr_type(self, source: &Source) -> (TokenStream, TokenStream) {
        (self.expr, self.ty.rust_type(source))
    }

    pub fn tuple(values: &[Value]) -> Value {
        Value {
            expr: match values {
                [] => quote! { () },
                [single] => single.expr.clone(),
                multiple => {
                    let code = multiple.iter().map(|expr| &expr.expr);
                    quote! { (#(#code),*) }
                }
            },
            ty: Type::Tuple(values.iter().map(|expr| expr.ty.clone()).collect()),
        }
    }

    pub fn vec_cloning(ty: Type, value_expr: TokenStream, len_expr: TokenStream) -> Value {
        Value {
            expr: quote! { crate::SmallVec::from_elem(#value_expr, #len_expr) },
            ty: Type::SmallVec(Box::new(ty)),
        }
    }

    pub fn map_bool(mut self) -> Value {
        if self.ty == Type::Named(Name::Type(TypeName::bool32())) {
            self.ty = Type::Bool;

            let expr = &self.expr;
            self.expr = quote! { #expr != 0 };
        }

        self
    }
}
