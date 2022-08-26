use std::num::NonZeroU8;

use syn::spanned::Spanned;

#[derive(Clone, Copy)]
pub struct FieldFlags {
    /// Whether to blanket mask everything (fields, variants)
    pub all: bool,

    /// Masks the name of this enum variant, if applicable.
    pub variant: bool,

    /// Whether the field is partially or fully masked.
    ///
    /// Incompatible with `fixed`.
    pub partial: bool,

    /// The character to use for masking. Defaults to `*`.
    pub mask_char: char,

    /// Whether to mask with a fixed width, ignoring the length of the data.
    ///
    /// Incompatible with `partial`.
    pub fixed: Option<NonZeroU8>,
}
impl FieldFlags {
    pub fn extract<const AMOUNT: usize>(attrs: &[syn::Attribute]) -> Result<[Option<Self>; AMOUNT], syn::Error> {
        let mut extracted = [None; AMOUNT];
        let mut head = 0;

        for attr in attrs {
            if head > AMOUNT {
                return Err(syn::Error::new(attr.span(), "too many `#[mask(...)]` attributes specified"));
            }

            if let Some(flags) = Self::parse(attr)? {
                extracted[head] = Some(flags);
                head += 1;
            }
        }

        Ok(extracted)
    }

    /// Returns `FieldFlags` parsed from an attribute.
    fn parse(attr: &syn::Attribute) -> Result<Option<Self>, syn::Error> {
        let mut flags = FieldFlags::default();

        // The modifiers could be a single value or a list, so we need to handle both cases.
        let modifiers = match attr.parse_meta()? {
            // List
            syn::Meta::List(syn::MetaList { path, nested, .. }) if path.is_ident("mask") => {
                nested.into_iter().filter_map(|meta| match meta {
                    syn::NestedMeta::Meta(meta) => Some(meta),
                    _ => None
                })
            }

            // Single value
            meta => match meta {
                syn::Meta::Path(path) if path.is_ident("mask") => return Ok(Some(flags)),
                _ => return Ok(None)
            }
        };

        // Now we can finally process each modifier.
        for meta in modifiers {
            match meta {
                // #[mask(all)]
                syn::Meta::Path(path) if path.is_ident("all") => {
                    flags.all = true;
                }

                // #[mask(partial)]
                syn::Meta::Path(path) if path.is_ident("partial") => {
                    flags.partial = true;
                }

                // #[mask(variant)]
                syn::Meta::Path(path) if path.is_ident("variant") => {
                    flags.variant = true;
                }

                // #[mask(with = 'X')]
                syn::Meta::NameValue(kv) if kv.path.is_ident("with") => match kv.lit {
                    syn::Lit::Char(with) => flags.mask_char = with.value(),
                    _ => {
                        return Err(syn::Error::new_spanned(
                            kv.lit,
                            "expected a character literal",
                        ))
                    }
                },

                // #[mask(fixed = u8)]
                syn::Meta::NameValue(kv) if kv.path.is_ident("fixed") => match kv.lit {
                    syn::Lit::Int(int) => {
                        flags.fixed =
                            Some(NonZeroU8::new(int.base10_parse::<u8>()?).ok_or_else(|| {
                                syn::Error::new_spanned(
                                    int,
                                    "fixed masking width must be greater than zero",
                                )
                            })?)
                    }
                    _ => {
                        return Err(syn::Error::new_spanned(
                            kv.lit,
                            "expected a character literal",
                        ))
                    }
                },

                // Anything we don't expect
                syn::Meta::List(_) => return Err(syn::Error::new_spanned(attr, "unexpected list for `Mask` attribute")),
                _ => return Err(syn::Error::new_spanned(attr, "unknown modifier for `Mask` attribute")),
            }
        }

        if flags.partial && flags.fixed.is_some() {
            return Err(syn::Error::new_spanned(
                attr,
                "`#[mask(partial)]` and `#[mask(fixed = ...)]` are incompatible",
            ));
        }

        Ok(Some(flags))
    }
}
impl Default for FieldFlags {
    fn default() -> Self {
        Self {
            partial: false,
            fixed: None,
            mask_char: '*',
            variant: false,
            all: false,
        }
    }
}
impl quote::ToTokens for FieldFlags {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self {
            partial,
            mask_char,
            fixed,
            ..
        } = *self;

        let fixed = fixed.map(|fixed| fixed.get()).unwrap_or(0);

        tokens.extend(quote! {
            partial: #partial,
            mask_char: #mask_char,
            fixed: #fixed,
        });
    }
}
