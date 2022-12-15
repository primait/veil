use crate::{
    flags::{ExtractFlags, RedactFlags},
    sanitize::{AttributeFilter, DeriveAttributeFilter},
};
use proc_macro::TokenStream;
use syn::spanned::Spanned;

fn try_derive(mut item: syn::DeriveInput) -> Result<TokenStream, syn::Error> {
    // Remove all non-veil attributes to avoid conflicting with other
    // derive proc macro attributes.
    item.retain_veil_attrs();

    let item_span = item.span();

    let s = match item.data {
        syn::Data::Struct(s) => s,
        syn::Data::Enum(_) => return Err(syn::Error::new(item_span, "this trait cannot be derived for enums")),
        syn::Data::Union(_) => return Err(syn::Error::new(item_span, "this trait cannot be derived for unions")),
    };

    let mut field = match s.fields.len() {
        0 => {
            return Err(syn::Error::new(
                item_span,
                "this trait cannot be derived for structs with no fields",
            ))
        }
        1 => s.fields.into_iter().next().unwrap(),
        _ => {
            return Err(syn::Error::new(
                item_span,
                "this trait cannot be derived for structs with multiple fields",
            ))
        }
    };

    field.attrs.retain_veil_attrs();

    if !field.attrs.is_empty() {
        return Err(syn::Error::new(
            field.attrs[0].span(),
            "redaction modifiers are not allowed here, put them on the struct itself",
        ));
    }

    let flags = RedactFlags::extract::<1>("Redactable", &item.attrs, ())?[0].unwrap_or_default();

    let name_ident = &item.ident;
    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();
    Ok(quote! {
        impl #impl_generics ::veil::Redactable for #name_ident #ty_generics #where_clause {
            fn redact(&self) -> String {
                ::veil::private::derived_redactable(
                    self,
                    ::veil::private::RedactFlags { #flags }
                )
            }

            fn redact_into<W: ::std::fmt::Write>(&self, buffer: &mut W) -> ::std::fmt::Result {
                buffer.write_str(
                    ::veil::private::derived_redactable(
                        self,
                        ::veil::private::RedactFlags { #flags }
                    )
                    .as_str()
                )
            }
        }
    }
    .into())
}

pub fn derive(item: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(item as syn::DeriveInput);

    match try_derive(item) {
        Ok(tokens) => tokens,
        Err(err) => err.into_compile_error().into(),
    }
}
