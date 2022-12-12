use crate::{
    flags::FieldFlags,
    fmt::{self, FormatData},
    UnusedDiagnostic,
};
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::spanned::Spanned;

#[derive(Default)]
struct EnumVariantFieldFlags {
    variant_flags: Option<FieldFlags>,
    all_fields_flags: Option<FieldFlags>,
}

pub(super) fn derive_redact(
    e: syn::DataEnum,
    generics: syn::Generics,
    attrs: Vec<syn::Attribute>,
    name_ident: syn::Ident,
    unused: &mut UnusedDiagnostic,
) -> Result<TokenStream, syn::Error> {
    // Parse #[redact(all, variant, ...)] from the enum attributes, if present.
    let top_level_flags = match FieldFlags::extract::<1>(&attrs, false)? {
        [Some(flags)] => {
            if !flags.all || !flags.variant {
                return Err(syn::Error::new(
                    attrs[0].span(),
                    "at least `#[redact(all, variant)]` is required here to redact all variant names",
                ));
            } else {
                Some(flags)
            }
        }

        _ => None,
    };

    // Collect each variant's flags
    let mut variant_flags = Vec::with_capacity(e.variants.len());
    for variant in &e.variants {
        let mut flags = match FieldFlags::extract::<2>(&variant.attrs, top_level_flags.is_some())? {
            [None, None] => EnumVariantFieldFlags::default(),

            [Some(flags), None] => {
                if flags.all && flags.variant {
                    // #[redact(all, variant, ...)]
                    return Err(syn::Error::new(
                        variant.attrs[0].span(),
                        "`#[redact(all, variant, ...)]` is invalid here, split into two separate attributes instead to apply redacting options to the variant name or all fields respectively",
                    ));
                } else if flags.all {
                    // #[redact(all, ...)]
                    EnumVariantFieldFlags {
                        variant_flags: None,
                        all_fields_flags: Some(flags),
                    }
                } else if flags.variant {
                    // #[redact(variant, ...)]
                    EnumVariantFieldFlags {
                        variant_flags: Some(flags),
                        all_fields_flags: None,
                    }
                } else {
                    return Err(syn::Error::new(
                        variant.span(),
                        "expected `#[redact(all, ...)]` or `#[redact(variant, ...)]`, or both as separate attributes",
                    ));
                }
            }

            [Some(flags0), Some(flags1)] => {
                let mut variant_flags = EnumVariantFieldFlags::default();

                for flags in [flags0, flags1] {
                    if flags.all && flags.variant {
                        // #[redact(all, variant, ...)]
                        return Err(syn::Error::new(
                            variant.span(),
                            "`#[redact(all, variant, ...)]` is invalid here, split into two separate attributes instead to apply redacting options to the variant name or all fields respectively",
                        ));
                    } else if flags.all {
                        // #[redact(all, ...)]
                        if variant_flags.all_fields_flags.is_some() {
                            return Err(syn::Error::new(
                                variant.span(),
                                "a `#[redact(all, ...)]` is already present",
                            ));
                        }
                        variant_flags.all_fields_flags = Some(flags);
                    } else if flags.variant {
                        // #[redact(variant, ...)]
                        if variant_flags.variant_flags.is_some() {
                            return Err(syn::Error::new(
                                variant.span(),
                                "a `#[redact(variant, ...)]` is already present",
                            ));
                        }
                        variant_flags.variant_flags = Some(flags);
                    } else {
                        return Err(syn::Error::new(
                            variant.span(),
                            "expected `#[redact(all, ...)]` or `#[redact(variant, ...)]`, or both as separate attributes",
                        ));
                    }
                }

                variant_flags
            }

            [None, ..] => unreachable!(),
        };

        // If there's top level flags, apply them to the variant's flags if they're not already set.
        if flags.variant_flags.is_none() {
            if let Some(top_level_flags) = top_level_flags {
                flags.variant_flags = Some(top_level_flags);
            }
        }

        variant_flags.push(flags);
    }

    // Create an iterator that will yield variant names as an identifier.
    // We'll use this to match on the variants in the Debug impl.
    let variant_idents = e.variants.iter().map(|variant| &variant.ident);

    // Create an iterator that will yield tokens that destructure an enum variant into its respective fields.
    // Struct variant fields are destructed as normal.
    // Tuple variant fields are destructed as arg0, arg1, ... argN.
    // Unit variants yield no tokens.
    let variant_destructures = e.variants.iter().map(|variant| match &variant.fields {
        syn::Fields::Named(syn::FieldsNamed { named, .. }) => {
            let idents = named.iter().map(|field| field.ident.as_ref().unwrap());
            quote! {
                { #(#idents),* }
            }
        }
        syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed, .. }) => {
            let args = (0..unnamed.len())
                .into_iter()
                .map(|i| syn::Ident::new(&format!("arg{i}"), unnamed.span()));
            quote! {
                ( #(#args),* )
            }
        }
        syn::Fields::Unit => Default::default(),
    });

    // Create an iterator that will yield the tokens of the body of the match arm for each variant.
    // These match arm bodies will actually print data into the Formatter.
    let mut variant_bodies = Vec::with_capacity(e.variants.len());
    for (variant, flags) in e.variants.iter().zip(variant_flags.into_iter()) {
        // Variant name redacting
        let variant_name = variant.ident.to_string();
        let variant_name = if let Some(flags) = &flags.variant_flags {
            fmt::generate_redact_call(quote! { &#variant_name }, false, flags, unused)
        } else {
            variant_name.into_token_stream()
        };

        variant_bodies.push(match &variant.fields {
            syn::Fields::Named(named) => {
                FormatData::FieldsNamed(named).impl_debug(variant_name, flags.all_fields_flags, false, unused)?
            }
            syn::Fields::Unnamed(unnamed) => {
                FormatData::FieldsUnnamed(unnamed).impl_debug(variant_name, flags.all_fields_flags, false, unused)?
            }
            syn::Fields::Unit => {
                if flags.all_fields_flags.is_some() {
                    return Err(syn::Error::new(
                        variant.attrs[0].span(),
                        "unit structs do not need redacting as they contain no data",
                    ));
                } else {
                    quote! { write!(f, "{:?}", #variant_name)? }
                }
            }
        });
    }

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    Ok(quote! {
        impl #impl_generics ::std::fmt::Debug for #name_ident #ty_generics #where_clause {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                let debug_alternate = f.alternate();
                match self {
                    #(Self::#variant_idents #variant_destructures => { #variant_bodies; },)*
                }
                Ok(())
            }
        }
    }
    .into())
}
