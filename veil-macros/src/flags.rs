use std::num::NonZeroU8;
use syn::{spanned::Spanned, LitChar, LitInt};

pub struct FieldFlagsParse {
    pub skip_allowed: bool,
}

pub enum ParseMeta {
    Consumed,
    Unrecognised,
}

type TryParseMeta = Result<ParseMeta, syn::Error>;

pub trait ExtractFlags: Sized + Copy + Default {
    type Options;

    fn try_parse_meta(&mut self, meta: &mut syn::meta::ParseNestedMeta) -> TryParseMeta;

    fn parse_meta(
        &mut self,
        derive_name: &'static str,
        meta: &mut syn::meta::ParseNestedMeta,
    ) -> Result<(), syn::Error> {
        match self.try_parse_meta(meta)? {
            ParseMeta::Consumed => Ok(()),
            ParseMeta::Unrecognised => Err(meta.error(format!("unknown attribute for `{derive_name}` attribute"))),
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

        match attr.meta {
            // List
            syn::Meta::List(_) => {
                attr.parse_nested_meta(|mut meta| flags.parse_meta(derive_name, &mut meta))?;

                Ok(Some(flags))
            }
            syn::Meta::Path(_) => Ok(Some(flags)),
            _ => todo!(),
        }
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

    fn try_parse_meta(&mut self, meta: &mut syn::meta::ParseNestedMeta) -> TryParseMeta {
        // #[redact(partial)]
        if meta.path.is_ident("partial") {
            if self.redact_length != RedactionLength::Full {
                return TryParseMeta::Err(meta.error("`partial` clashes with an existing redaction length flag"));
            }
            self.redact_length = RedactionLength::Partial;
        // #[redact(with = 'X')]
        } else if meta.path.is_ident("with") {
            let ch: LitChar = meta.value()?.parse()?;
            self.redact_char = ch.value();
            // #[redact(fixed = u8)]
        } else if meta.path.is_ident("fixed") {
            if self.redact_length != RedactionLength::Full {
                return TryParseMeta::Err(meta.error("`fixed` clashes with an existing redaction length flag"));
            }
            let int: LitInt = meta.value()?.parse()?;
            self.redact_length = RedactionLength::Fixed(
                match int.base10_parse::<u8>().and_then(|int| {
                    NonZeroU8::new(int)
                        .ok_or_else(|| syn::Error::new_spanned(int, "fixed redacting width must be greater than zero"))
                }) {
                    Ok(fixed) => fixed,
                    Err(err) => return TryParseMeta::Err(err),
                },
            )
        } else {
            return Ok(ParseMeta::Unrecognised);
        }
        Ok(ParseMeta::Consumed)
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

    fn try_parse_meta(&mut self, meta: &mut syn::meta::ParseNestedMeta) -> TryParseMeta {
        // First try to parse the redaction flags.
        if let result @ (Ok(ParseMeta::Consumed) | Err(_)) = self.redact.try_parse_meta(meta) {
            // This was a redaction flag, so we don't need to do anything else.
            // OR
            // This was an error, so we need to propagate it.
            return result;
        }
        // This was not a redaction flag, so we need to continue processing.

        // #[redact(all)]
        if meta.path.is_ident("all") {
            self.all = true;
        }
        // #[redact(skip)]
        else if meta.path.is_ident("skip") {
            self.skip = true;
        } else if meta.path.is_ident("variant") {
            self.variant = true;
        } else if meta.path.is_ident("display") {
            self.display = true;
        } else {
            return Ok(ParseMeta::Unrecognised);
        }

        Ok(ParseMeta::Consumed)
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
