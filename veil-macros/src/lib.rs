//! Macros for [`veil`].

#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

#[macro_use]
extern crate quote;

mod enums;
mod flags;
mod fmt;
mod pii;
mod redact;
mod sanitize;
mod structs;

use proc_macro::TokenStream;

/// Keep track of whether we actually redact anything.
///
/// By default fields are not redacted. One must add `#[redact(...)]` to them.
///
/// We should throw an error if no fields are redacted, because the user should derive Debug instead.
///
/// This should also be aware of `#[redact(skip)]` - we shouldn't let users bypass this check via that.
struct UnusedDiagnostic(bool);
impl UnusedDiagnostic {
    #[inline(always)]
    /// We redacted something! Don't throw an error saying the derive was unused.
    pub(crate) fn redacted_something(&mut self) {
        self.0 = false;
    }

    #[inline(always)]
    #[must_use]
    fn should_throw_err(self) -> bool {
        self.0
    }
}
impl Default for UnusedDiagnostic {
    #[inline(always)]
    fn default() -> Self {
        Self(true)
    }
}

#[proc_macro_derive(Pii, attributes(redact))]
/// Implements [`RedactPii`](trait.RedactPii.html) for a type.
///
/// The type must have a [`std::fmt::Display`] implementation. This is what will be used to redact the type.
///
/// See the [crate level documentation](index.html) for flags and modifiers.
pub fn derive_pii(item: TokenStream) -> TokenStream {
    pii::derive(item)
}

#[proc_macro_derive(Redact, attributes(redact))]
/// Implements [`std::fmt::Debug`] for a struct or enum, with certain fields redacted.
///
/// See the [crate level documentation](index.html) for flags and modifiers.
pub fn derive_redact(item: TokenStream) -> TokenStream {
    redact::derive(item)
}

#[doc(hidden)]
#[proc_macro]
pub fn __private_version(_: TokenStream) -> TokenStream {
    format!("{:?}", env!("CARGO_PKG_VERSION")).parse().unwrap()
}
