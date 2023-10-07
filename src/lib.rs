use proc_macro::TokenStream;


use proc_macro2::{Ident, TokenStream as TokenStream2};
use proc_macro2::TokenTree as TokenTree2;
use quote::{quote, ToTokens};
use syn::{Data, DeriveInput, Lit, parse_macro_input, Type};
use crate::attributes::EnvAttr;


mod attributes;

#[proc_macro_derive(FromEnv, attributes(env_attr))]
pub fn derive_from_env(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let DeriveInput { ident, .. } = input;
    type TokenStreamVec = Vec<TokenStream2>;
    let (field_arm, (comment_arm, exec_arm)): (TokenStreamVec, (TokenStreamVec, TokenStreamVec)) = match input.data {
        Data::Struct(syn::DataStruct { fields, .. }) if matches!(&fields, syn::Fields::Named(_)) => fields.iter()
            .map(|field| (&field.ident, &field.ty, EnvAttr::from_attributes(&field.attrs), ))
            .filter(|(_, _, attrs)| attrs.is_ok())
            .filter_map(|(ident, ty, attrs)| {
                let attrs = attrs.unwrap();
                let field = Some(quote!(#ident));
                // panic!("internal_ty: {:#?}", internal_ty);
                let default_statement = obtain_default(ident, &attrs, ty);
                let exec_statement = obtain_exec(ident, &attrs);
                if default_statement.is_none() && exec_statement.is_none() {
                    None
                } else {
                    let statement = default_statement.or(Some(TokenStream2::default()))
                        .zip(exec_statement.or(Some(TokenStream2::default())));
                    field.zip(statement)
                }
            })
            .unzip(),
        _ => panic!("only support annotate on struct")
    };
    quote!(
        impl #ident {
            pub fn from_env() -> Self {
                #(#comment_arm)*
                #(#exec_arm)*
                Self {
                    #(#field_arm,)*
                    .. Default::default()
                }
            }
        }
    ).into()
}

fn is_option(ty: &Type) -> bool {
    if let Type::Path(path) = &ty {
        path.path.segments.len() == 1 && path.path.segments[0].ident.to_token_stream().to_string().eq("Option")
    } else {
        false
    }
}

fn obtain_default(ident: &Option<Ident> , attrs: &EnvAttr, ty: &Type) -> Option<proc_macro2::TokenStream> {
    let var_name = &attrs.name;
    let var_default = &attrs.default;
    let internal_ty = ty.clone().to_token_stream().into_iter().reduce(|last,next| {
        if let TokenTree2::Ident(_) = next {
            next
        } else {
            last
        }
    }).unwrap().into_token_stream();
    var_default.clone().map(|var| {
        if let Lit::Str(_) = var {
            quote!(std::env::var(#var_name).unwrap_or(#var_default.to_owned()))
        } else {
            quote!(std::env::var(#var_name).unwrap_or_default().parse().unwrap_or(#var_default.to_owned()) as #internal_ty)
        }
    }).map(|default| {
        if is_option(&ty) {
            quote!(let #ident = Some(#default);)
        } else {
            quote!(let #ident = #default;)
        }
    })
}

fn obtain_exec(ident: &Option<Ident> , attrs: &EnvAttr) -> Option<proc_macro2::TokenStream> {
    let exec = &attrs.exec;
    match exec {
        Some(expr) => {
            Some(quote!(let #ident = #expr;))
        }
        _ => None
    }
}