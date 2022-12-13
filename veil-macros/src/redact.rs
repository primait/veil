use crate::{enums, sanitize::DeriveAttributeFilter, structs, UnusedDiagnostic};
use proc_macro::TokenStream;
use syn::spanned::Spanned;

fn try_derive(mut item: syn::DeriveInput) -> Result<TokenStream, syn::Error> {
    // Remove all non-veil attributes to avoid conflicting with other
    // derive proc macro attributes.
    item.retain_veil_attrs();

    let item_span = item.span();

    // Unfortunately this is somewhat complex to implement at this stage of the macro "pipeline",
    // so we'll pass around a mutable reference to this variable, and set it to false if we redact anything.
    // TBH this kind of smells, but I can't think of a better way to do it.
    let mut unused = UnusedDiagnostic::default();

    let tokens = match item.data {
        syn::Data::Struct(s) => structs::derive_redact(s, item.generics, item.attrs, item.ident, &mut unused)?,
        syn::Data::Enum(e) => enums::derive_redact(e, item.generics, item.attrs, item.ident, &mut unused)?,
        syn::Data::Union(_) => return Err(syn::Error::new(item_span, "this trait cannot be derived for unions")),
    };

    if unused.should_throw_err() {
        return Err(syn::Error::new(
            item_span,
            "`#[derive(Redact)]` does nothing by default, you must specify at least one field to redact. You should `#[derive(Debug)]` instead if this is intentional",
        ));
    }

    Ok(tokens)
}

pub fn derive(item: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(item as syn::DeriveInput);

    match try_derive(item) {
        Ok(tokens) => tokens,
        Err(err) => err.into_compile_error().into(),
    }
}
