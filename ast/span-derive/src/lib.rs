//! Derive `ast::Spanned` by deferring to an internal `Spanned`.
//!
//! For internal use by `nafi_ast` only. Generates edition-2018 code.

#![feature(tool_lints)]

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;
extern crate single;

use proc_macro2::Span;
use syn::{Data, DeriveInput, Fields, Ident, Member};

#[proc_macro_derive(Spanned)]
pub fn derive_spanned(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_spanned_impl(syn::parse(input).unwrap())
        .unwrap_or_else(compile_error)
        .into()
}

#[allow(clippy::needless_pass_by_value)]
fn compile_error(error: String) -> proc_macro2::TokenStream {
    quote! {
        compile_error!(#error);
    }
}

fn derive_spanned_impl(input: DeriveInput) -> Result<proc_macro2::TokenStream, String> {
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let dummy_const = Ident::new(
        &format!("_impl_Spanned_for_{}", name.to_string()),
        Span::call_site(),
    );
    let implementation = match input.data {
        Data::Struct(data) => {
            let field = match data.fields {
                Fields::Named(_) => Member::Named(Ident::new("span", Span::call_site())),
                Fields::Unnamed(_) => {
                    // Member::Unnamed(Index { index: 0, span: Span::call_site() })
                    Err("Cannot derive for tuple struct")?
                }
                Fields::Unit => Err("Cannot derive for unit struct")?,
            };
            quote!(self.#field)
        }
        Data::Enum(data) => {
            let arms = data
                .variants
                .iter()
                .map(|variant| variant.ident.clone())
                .map(|ident| quote!(#name::#ident(spanned) => crate::Spanned::span(spanned)));
            quote! {
                match self {
                    #(#arms),*
                }
            }
        }
        Data::Union(_) => Err("Cannot derive for union")?,
    };
    Ok(quote! {
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const #dummy_const: () = {
            impl #impl_generics crate::Spanned for #name #ty_generics #where_clause {
                fn span(&self) -> crate::Span {
                    #implementation
                }
            }
        };
    })
}
