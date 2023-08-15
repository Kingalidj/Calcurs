// use proc_macro2::TokenStream;
// use quote::quote;
// use syn::parse_macro_input;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parser, parse_macro_input, Field, FieldsNamed};

/// this macro gets as input a type and will insert a field called base with that type
/// it will also implement the [calcurus_internals::Inhertied] trait
#[proc_macro_attribute]
pub fn inherit(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(item as syn::ItemStruct);
    let base_type = parse_macro_input!(attr as syn::Type);

    let struct_name = &item.ident;
    let struct_generics = &item.generics;

    let base = Field::parse_named
        .parse2(quote! {base: #base_type})
        .unwrap();

    match item.fields {
        syn::Fields::Named(FieldsNamed { ref mut named, .. }) => named.push(base),
        _ => {
            return syn::Error::new_spanned(
                item.fields,
                "Only named fields are supported for adding the base field.",
            )
            .into_compile_error()
            .into()
        }
    }

    quote! {
        #item

        impl #struct_generics calcurus_internals::Inherited<#base_type> for #struct_name #struct_generics {
            fn base(&self)  -> &#base_type {
                &self.base
            }
        }

    }
    .into()
}
