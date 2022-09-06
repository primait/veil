use crate::flags::FieldFlags;
use quote::ToTokens;
use syn::spanned::Spanned;

#[rustfmt::skip]
/// Returns whether a syn::Type is an Option<T>
///
/// We try and match as many possible paths as possible because
/// some macros can output very verbose paths to items.
fn is_ty_option(ty: &syn::Type) -> bool {
    if let syn::Type::Path(syn::TypePath { path, .. }) = &ty {
        match path.segments.len() {
            1 if path.segments[0].ident == "Option" => true,

            // [std|core]::option::Option
            3 if (path.segments[0].ident == "std" || path.segments[0].ident == "core") && path.segments[1].ident == "option" && path.segments[2].ident == "Option" => true,

            // [std|core]::prelude::*::Option
            4 if (path.segments[0].ident == "std" || path.segments[0].ident == "core") && path.segments[1].ident == "prelude" && path.segments[3].ident == "Option" => true,

            _ => false,
        }
    } else {
        false
    }
}

pub enum FormatData<'a> {
    /// Structs, struct enum variants
    FieldsNamed(&'a syn::FieldsNamed),

    /// Tuple structs, tuple enum variants
    FieldsUnnamed(&'a syn::FieldsUnnamed),
}
impl FormatData<'_> {
    /// `name`: The name of the struct or enum variant.
    ///
    /// `all_field_flags`: `FieldFlags` that apply to all fields, if set
    ///
    /// `with_self`: prepends `self.` to the field name for accessing struct fields
    pub fn impl_debug(
        self,
        name: proc_macro2::TokenStream,
        all_fields_flags: Option<FieldFlags>,
        with_self: bool,
    ) -> Result<proc_macro2::TokenStream, syn::Error> {
        let fields = match self {
            Self::FieldsNamed(syn::FieldsNamed { named: fields, .. })
            | Self::FieldsUnnamed(syn::FieldsUnnamed { unnamed: fields, .. }) => fields,
        };

        let mut field_bodies = Vec::with_capacity(fields.len());
        for (i, field) in fields.iter().enumerate() {
            // The field accessor is how we actually get a reference to the value of a field.
            // This could be `self.field`, `self.0`, or just `field` or `arg0`, depending on whether
            // we destructured the enum variant or we're printing a struct.
            #[allow(clippy::collapsible_else_if)]
            let field_accessor = if with_self {
                if let Some(ident) = &field.ident {
                    quote! { &self.#ident }
                } else {
                    let i = syn::Index::from(i);
                    quote! { &self.#i }
                }
            } else {
                if let Some(ident) = &field.ident {
                    ident.into_token_stream()
                } else {
                    syn::Ident::new(&format!("arg{i}"), field.span()).into_token_stream()
                }
            };

            // Parse field flags from attributes on this field
            let field_flags = match field.attrs.len() {
                0 => all_fields_flags,
                1 => match FieldFlags::extract::<1>(&field.attrs, all_fields_flags.is_some())? {
                    [Some(flags)] => {
                        if flags.variant {
                            return Err(syn::Error::new(
                                field.attrs[0].span(),
                                "`#[redact(variant)]` is invalid for structs",
                            ));
                        } else {
                            Some(flags)
                        }
                    }

                    [None] => None,
                },
                _ => {
                    return Err(syn::Error::new(
                        field.span(),
                        "only one `#[redact(...)]` attribute is allowed per field",
                    ))
                }
            };

            // If we have field flags...
            if let Some(field_flags) = field_flags {
                // And we actually want to redact this field...
                if !field_flags.skip {
                    // Redact it!

                    // Specialization for Option<T>
                    let is_option = is_ty_option(&field.ty);

                    field_bodies.push(quote! {
                        ::veil::private::redact(#field_accessor, ::veil::private::RedactFlags {
                            debug_alternate,
                            is_option: #is_option,
                            #field_flags
                        })
                    });

                    continue;
                }
            }

            // Otherwise, just use the normal `Debug` implementation.
            field_bodies.push(quote! { #field_accessor });
        }

        Ok(match self {
            Self::FieldsNamed(syn::FieldsNamed { named, .. }) => {
                let field_names = named.iter().map(|field| field.ident.as_ref().unwrap().to_string());

                quote! {
                    f.debug_struct(&#name.as_ref())
                    #(
                        .field(#field_names, &#field_bodies)
                    )*
                    .finish()?
                }
            }

            Self::FieldsUnnamed(syn::FieldsUnnamed { .. }) => {
                quote! {
                    f.debug_tuple(&#name.as_ref())
                    #(
                        .field(&#field_bodies)
                    )*
                    .finish()?
                }
            }
        })
    }
}
