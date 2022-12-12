use crate::{flags::FieldFlags, fmt::FormatData, UnusedDiagnostic};
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::spanned::Spanned;

pub(super) fn derive_redact(
    s: syn::DataStruct,
    generics: syn::Generics,
    attrs: Vec<syn::Attribute>,
    name_ident: syn::Ident,
    unused: &mut UnusedDiagnostic,
) -> Result<TokenStream, syn::Error> {
    // Parse #[redact(all, variant, ...)] from the enum attributes, if present.
    let top_level_flags = match attrs.len() {
        0 => None,
        1 => match FieldFlags::extract::<1>(&attrs, false)? {
            [Some(flags)] => {
                if flags.variant {
                    return Err(syn::Error::new(
                        attrs[0].span(),
                        "`#[redact(variant, ...)]` is invalid for structs",
                    ));
                } else if !flags.all {
                    return Err(syn::Error::new(
                        attrs[0].span(),
                        "at least `#[redact(all)]` is required here to redact all struct fields",
                    ));
                } else {
                    Some(flags)
                }
            }
            [None] => None,
        },
        _ => {
            return Err(syn::Error::new(
                attrs[1].span(),
                "expected only one or zero `#[redact(all, ...)]` attributes",
            ))
        }
    };

    // Convert the name of this struct into a string for use as the first argument to `.debug_struct` or `.debug_tuple`.
    let name_ident_str = name_ident.to_string().into_token_stream();

    // Generate the body of the std::fmt::Debug implementation
    let impl_debug = match &s.fields {
        syn::Fields::Named(named) => {
            FormatData::FieldsNamed(named).impl_debug(name_ident_str, top_level_flags, true, unused)?
        }
        syn::Fields::Unnamed(unnamed) => {
            FormatData::FieldsUnnamed(unnamed).impl_debug(name_ident_str, top_level_flags, true, unused)?
        }
        syn::Fields::Unit => {
            return Err(syn::Error::new(
                name_ident.span(),
                "unit structs do not need redacting as they contain no data, use `#[derive(Debug)]` instead",
            ))
        }
    };

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    Ok(quote! {
        impl #impl_generics ::std::fmt::Debug for #name_ident #ty_generics #where_clause {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                let debug_alternate = f.alternate();
                #impl_debug;
                Ok(())
            }
        }
    }
    .into())
}
