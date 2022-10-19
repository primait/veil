#[macro_use]
extern crate quote;

mod enums;
mod flags;
mod fmt;
mod sanitize;
mod structs;

use proc_macro::TokenStream;
use sanitize::DeriveAttributeFilter;
use syn::spanned::Spanned;

/// Keep track of whether we actually redact anything.
///
/// By default fields are not redacted. One must add #[redact(...)] to them.
///
/// We should throw an error if no fields are redacted, because the user should derive Debug instead.
///
/// This should also be aware of #[redact(skip)] - we shouldn't let users bypass this check via that.
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

#[proc_macro_derive(Redact, attributes(redact))]
/// Implements [`std::fmt::Debug`] for a struct or enum variant, with certain fields redacted.
///
/// See the [crate level documentation](index.html) for more information.
pub fn derive_redact(item: TokenStream) -> TokenStream {
    let mut item = syn::parse_macro_input!(item as syn::DeriveInput);

    // Remove all non-veil attributes to avoid conflicting with other
    // derive proc macro attributes.
    item.retain_veil_attrs();

    // Unfortunately this is somewhat complex to implement at this stage of the macro "pipeline",
    // so we'll pass around a mutable reference to this variable, and set it to false if we redact anything.
    // TBH this kind of smells, but I can't think of a better way to do it.
    let mut unused = UnusedDiagnostic::default();

    let item_span = item.span();

    let result = match item.data {
        syn::Data::Struct(s) => structs::derive_redact(s, item.attrs, item.ident, &mut unused),
        syn::Data::Enum(e) => enums::derive_redact(e, item.attrs, item.ident, &mut unused),
        syn::Data::Union(_) => Err(syn::Error::new(item_span, "this trait cannot be derived for unions")),
    };

    let result = result.and_then(|tokens| {
        if unused.should_throw_err() {
            Err(syn::Error::new(
                item_span,
                "`#[derive(Redact)]` does nothing by default, you must specify at least one field to redact. You should `#[derive(Debug)]` instead if this is intentional",
            ))
        } else {
            Ok(tokens)
        }
    });

    match result {
        Ok(tokens) => tokens,
        Err(err) => err.into_compile_error().into(),
    }
}

#[doc(hidden)]
#[proc_macro]
pub fn __private_version(_: TokenStream) -> TokenStream {
    format!("{:?}", env!("CARGO_PKG_VERSION")).parse().unwrap()
}
