use proc_macro::TokenStream;
use quote::ToTokens;
use syn::spanned::Spanned;
use crate::{flags::FieldFlags, fmt::FormatData};

pub fn derive_mask(s: syn::DataStruct, attrs: Vec<syn::Attribute>, name_ident: syn::Ident) -> Result<TokenStream, syn::Error> {
    // Parse #[mask(all, variant, ...)] from the enum attributes, if present.
    let top_level_flags = match attrs.len() {
        0 => None,
        1 => match FieldFlags::extract::<1>(&attrs)? {
            [Some(flags)] => {
                if flags.variant {
                    return Err(syn::Error::new(
                        attrs[0].span(),
                        "`#[mask(variant, ...)]` is invalid for structs",
                    ));
                } else if !flags.all {
                    return Err(syn::Error::new(
                        attrs[0].span(),
                        "at least `#[mask(all)]` is required here to mask all struct fields",
                    ));
                } else {
                    Some(flags)
                }
            },
            [None] => None,
        },
        _ => return Err(syn::Error::new(
            attrs[1].span(),
            "expected only one or zero `#[mask(all, ...)]` attributes",
        )),
    };

    // Convert the name of this struct into a string for use as the first argument to `.debug_struct` or `.debug_tuple`.
    let name_ident_str = name_ident.to_string().into_token_stream();

    // Generate the body of the std::fmt::Debug implementation
    let impl_debug = match &s.fields {
        syn::Fields::Named(named) => FormatData::FieldsNamed(named).impl_debug(name_ident_str, top_level_flags, true)?,
        syn::Fields::Unnamed(unnamed) => FormatData::FieldsUnnamed(unnamed).impl_debug(name_ident_str, top_level_flags, true)?,
        syn::Fields::Unit => return Err(syn::Error::new(
            name_ident.span(),
            "unit structs do not need masking as they contain no data, use `#[derive(Debug)]` instead"
        ))
    };

    Ok(quote! {
        impl ::std::fmt::Debug for #name_ident {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                let debug_alternate = f.alternate();
                #impl_debug;
                Ok(())
            }
        }
    }.into())
}