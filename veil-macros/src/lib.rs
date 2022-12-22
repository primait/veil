//! Macros for `veil`

#![cfg_attr(not(docsci), warn(missing_docs))]
#![cfg_attr(docsci, deny(missing_docs))]
#![cfg_attr(docsci, deny(rustdoc::broken_intra_doc_links))]

#[macro_use]
extern crate quote;

mod enums;
mod flags;
mod fmt;
mod redact;
mod redactable;
mod sanitize;
mod structs;

use proc_macro::TokenStream;

#[proc_macro_derive(Redact, attributes(redact))]
/// Implements [`Debug`] for a struct or enum, with certain fields redacted.
///
/// See the [crate level documentation](index.html) for flags and modifiers.
pub fn derive_redact(item: TokenStream) -> TokenStream {
    redact::derive(item)
}

#[proc_macro_derive(Redactable, attributes(redact))]
/// Implements [`Redactable`](trait.Redactable.html) for a type.
///
/// The type must have a [`Display`](std::fmt::Display) implementation. This is what will be used to redact the type.
///
/// See the [crate level documentation](index.html) for flags and modifiers.
pub fn derive_redactable(item: TokenStream) -> TokenStream {
    redactable::derive(item)
}

#[doc(hidden)]
#[proc_macro]
/// Used by the `versioning::test_macros_version` test.
pub fn __private_version(_: TokenStream) -> TokenStream {
    format!("{:?}", env!("CARGO_PKG_VERSION")).parse().unwrap()
}
