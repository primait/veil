#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::spanned::Spanned;

mod enums;
mod flags;
mod fmt;
mod structs;

#[proc_macro_derive(Redact, attributes(redact))]
/// Implements [`std::fmt::Debug`] for a struct or enum variant, with certain fields redacted.
///
/// See the [crate level documentation](index.html) for more information.
pub fn derive_redact(item: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(item as syn::DeriveInput);

    let result = match item.data {
        syn::Data::Struct(s) => structs::derive_redact(s, item.attrs, item.ident),
        syn::Data::Enum(e) => enums::derive_redact(e, item.attrs, item.ident),
        syn::Data::Union(_) => Err(syn::Error::new(item.span(), "this trait cannot be derived for unions")),
    };

    match result {
        Ok(tokens) => tokens,
        Err(err) => err.into_compile_error().into(),
    }
}
