#[macro_use] extern crate quote;

use proc_macro::TokenStream;
use syn::spanned::Spanned;

mod flags;
mod structs;
mod enums;
mod fmt;

#[proc_macro_derive(Mask, attributes(mask))]
pub fn derive_mask(item: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(item as syn::DeriveInput);

    let result = match item.data {
        syn::Data::Struct(s) => structs::derive_mask(s, item.attrs, item.ident),
        syn::Data::Enum(e) => enums::derive_mask(e, item.attrs, item.ident),
        syn::Data::Union(_) => Err(syn::Error::new(
            item.span(),
            "this trait cannot be derived for unions",
        )),
    };

    match result {
        Ok(tokens) => tokens,
        Err(err) => err.into_compile_error().into(),
    }
}