use std::num::NonZeroU8;
use syn::spanned::Spanned;

pub struct FieldFlagsParse {
    pub skip_allowed: bool,
}

pub enum TryParseMeta {
    Consumed,
    Unrecognised(syn::Meta),
    Err(syn::Error),
}

pub trait ExtractFlags: Sized + Copy + Default {
    type Options;

    fn try_parse_meta(&mut self, meta: syn::Meta) -> TryParseMeta;

    fn parse_meta(
        &mut self,
        derive_name: &'static str,
        attr: &syn::Attribute,
        meta: syn::Meta,
    ) -> Result<(), syn::Error> {
        match self.try_parse_meta(meta) {
            TryParseMeta::Consumed => Ok(()),
            TryParseMeta::Err(err) => Err(err),
            TryParseMeta::Unrecognised(meta) => match meta {
                // Anything we don't expect
                syn::Meta::List(_) => Err(syn::Error::new_spanned(
                    attr,
                    format!("unexpected list for `{}` attribute", derive_name),
                )),
                _ => Err(syn::Error::new_spanned(
                    attr,
                    format!("unknown modifier for `{}` attribute", derive_name),
                )),
            },
        }
    }

    fn extract<const AMOUNT: usize>(
        derive_name: &'static str,
        attrs: &[syn::Attribute],
        options: Self::Options,
    ) -> Result<[Option<Self>; AMOUNT], syn::Error> {
        let mut extracted = [None; AMOUNT];
        let mut head = 0;

        for attr in attrs {
            if head == AMOUNT {
                return Err(syn::Error::new(
                    attr.span(),
                    "too many `#[redact(...)]` attributes specified",
                ));
            }

            if let Some(flags) = Self::parse(derive_name, attr)? {
                flags.validate(attr, &options)?;
                extracted[head] = Some(flags);
                head += 1;
            }
        }

        Ok(extracted)
    }

    fn parse(derive_name: &'static str, attr: &syn::Attribute) -> Result<Option<Self>, syn::Error> {
        let mut flags = Self::default();

        // The modifiers could be a single value or a list, so we need to handle both cases.
        let modifiers = match attr.parse_meta()? {
            // List
            syn::Meta::List(syn::MetaList { nested, .. }) => nested.into_iter().filter_map(|meta| match meta {
                syn::NestedMeta::Meta(meta) => Some(meta),
                _ => None,
            }),

            // Single value
            meta => match meta {
                syn::Meta::Path(_) => return Ok(Some(flags)),
                _ => return Ok(None),
            },
        };

        // Now we can finally process each modifier.
        for meta in modifiers {
            flags.parse_meta(derive_name, attr, meta)?;
        }

        Ok(Some(flags))
    }

    fn validate(&self, _attr: &syn::Attribute, _options: &Self::Options) -> Result<(), syn::Error> {
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RedactionLength {
    /// Redact the entire data.
    Full,

    /// Redact a portion of the data.
    Partial,

    /// Whether to redact with a fixed width, ignoring the length of the data.
    Fixed(NonZeroU8),
}
impl quote::ToTokens for RedactionLength {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            RedactionLength::Full => quote! { ::veil::private::RedactionLength::Full }.to_tokens(tokens),
            RedactionLength::Partial => quote! { ::veil::private::RedactionLength::Partial }.to_tokens(tokens),
            RedactionLength::Fixed(n) => {
                let n = n.get();
                quote! { ::veil::private::RedactionLength::Fixed(::core::num::NonZeroU8::new(#n).unwrap()) }
                    .to_tokens(tokens)
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct RedactFlags {
    pub redact_length: RedactionLength,

    /// The character to use for redacting. Defaults to `*`.
    pub redact_char: char,
}
impl Default for RedactFlags {
    fn default() -> Self {
        Self {
            redact_length: RedactionLength::Full,
            redact_char: '*',
        }
    }
}
impl ExtractFlags for RedactFlags {
    type Options = ();

    fn try_parse_meta(&mut self, meta: syn::Meta) -> TryParseMeta {
        match meta {
            // #[redact(partial)]
            syn::Meta::Path(path) if path.is_ident("partial") => {
                if self.redact_length != RedactionLength::Full {
                    return TryParseMeta::Err(syn::Error::new_spanned(
                        path,
                        "`partial` clashes with an existing redaction length flag",
                    ));
                }
                self.redact_length = RedactionLength::Partial;
            }

            // #[redact(with = 'X')]
            syn::Meta::NameValue(kv) if kv.path.is_ident("with") => match kv.lit {
                syn::Lit::Char(with) => self.redact_char = with.value(),
                _ => return TryParseMeta::Err(syn::Error::new_spanned(kv.lit, "expected a character literal")),
            },

            // #[redact(fixed = u8)]
            syn::Meta::NameValue(kv) if kv.path.is_ident("fixed") => {
                if self.redact_length != RedactionLength::Full {
                    return TryParseMeta::Err(syn::Error::new_spanned(
                        kv.path,
                        "`fixed` clashes with an existing redaction length flag",
                    ));
                }
                if let syn::Lit::Int(int) = kv.lit {
                    self.redact_length = RedactionLength::Fixed(
                        match int.base10_parse::<u8>().and_then(|int| {
                            NonZeroU8::new(int).ok_or_else(|| {
                                syn::Error::new_spanned(int, "fixed redacting width must be greater than zero")
                            })
                        }) {
                            Ok(fixed) => fixed,
                            Err(err) => return TryParseMeta::Err(err),
                        },
                    )
                } else {
                    return TryParseMeta::Err(syn::Error::new_spanned(kv.lit, "expected a character literal"));
                }
            }

            _ => return TryParseMeta::Unrecognised(meta),
        }
        TryParseMeta::Consumed
    }
}
impl quote::ToTokens for RedactFlags {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self {
            redact_length,
            redact_char,
            ..
        } = self;

        tokens.extend(quote! {
            redact_length: #redact_length,
            redact_char: #redact_char
        });
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct FieldFlags {
    /// Whether to blanket redact everything (fields, variants)
    pub all: bool,

    /// Redacts the name of this enum variant, if applicable.
    pub variant: bool,

    /// Whether to skip redaction.
    ///
    /// Only allowed if this field is affected by a `#[redact(all)]` attribute.
    ///
    /// Fields are not redacted by default unless their parent is marked as `#[redact(all)]`, and this flag turns off that redaction for this specific field.
    pub skip: bool,

    /// Whether to use the type's [`Display`](std::fmt::Display) implementation instead of [`Debug`].
    pub display: bool,

    /// Flags that modify the redaction behavior.
    pub redact: RedactFlags,
}
impl ExtractFlags for FieldFlags {
    type Options = FieldFlagsParse;

    fn try_parse_meta(&mut self, meta: syn::Meta) -> TryParseMeta {
        // First try to parse the redaction flags.
        let meta = match self.redact.try_parse_meta(meta) {
            // This was a redaction flag, so we don't need to do anything else.
            // OR
            // This was an error, so we need to propagate it.
            result @ (TryParseMeta::Consumed | TryParseMeta::Err(_)) => return result,

            // This was not a redaction flag, so we need to continue processing.
            TryParseMeta::Unrecognised(meta) => meta,
        };

        match meta {
            // #[redact(all)]
            syn::Meta::Path(path) if path.is_ident("all") => {
                self.all = true;
            }

            // #[redact(skip)]
            syn::Meta::Path(path) if path.is_ident("skip") => {
                self.skip = true;
            }

            // #[redact(variant)]
            syn::Meta::Path(path) if path.is_ident("variant") => {
                self.variant = true;
            }

            // #[redact(display)]
            syn::Meta::Path(path) if path.is_ident("display") => {
                self.display = true;
            }

            _ => return TryParseMeta::Unrecognised(meta),
        }

        TryParseMeta::Consumed
    }

    fn validate(&self, attr: &syn::Attribute, options: &Self::Options) -> Result<(), syn::Error> {
        if self.skip {
            if !options.skip_allowed {
                return Err(syn::Error::new(attr.span(), "`#[redact(skip)]` is not allowed here"));
            }

            // It doesn't make sense for `skip` to be present with any other flags.
            // We'll throw an error if it is.
            let valid_skip_flags = FieldFlags {
                skip: true,
                variant: self.variant,
                ..Default::default()
            };
            if self != &valid_skip_flags {
                return Err(syn::Error::new(
                    attr.span(),
                    "`#[redact(skip)]` should not have any other modifiers present",
                ));
            }
        }

        Ok(())
    }
}
impl quote::ToTokens for FieldFlags {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        assert!(!self.skip, "internal error: skip flag should not be set here");
        self.redact.to_tokens(tokens)
    }
}
