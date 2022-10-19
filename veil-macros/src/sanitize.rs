trait AttributeFilter {
    fn retain_veil_attrs(&mut self);
}
impl AttributeFilter for Vec<syn::Attribute> {
    fn retain_veil_attrs(&mut self) {
        self.retain(|attr| attr.path.is_ident("redact"));
    }
}
impl AttributeFilter for syn::Fields {
    fn retain_veil_attrs(&mut self) {
        self.iter_mut().for_each(|field| field.attrs.retain_veil_attrs());
    }
}
impl AttributeFilter for syn::FieldsNamed {
    fn retain_veil_attrs(&mut self) {
        self.named.iter_mut().for_each(|field| field.attrs.retain_veil_attrs());
    }
}
impl AttributeFilter for syn::FieldsUnnamed {
    fn retain_veil_attrs(&mut self) {
        self.unnamed
            .iter_mut()
            .for_each(|field| field.attrs.retain_veil_attrs());
    }
}
impl AttributeFilter for syn::Data {
    fn retain_veil_attrs(&mut self) {
        match self {
            syn::Data::Struct(s) => s.fields.retain_veil_attrs(),
            syn::Data::Enum(e) => e.variants.iter_mut().for_each(|variant| {
                variant.attrs.retain_veil_attrs();
                variant.fields.retain_veil_attrs()
            }),
            syn::Data::Union(u) => u.fields.retain_veil_attrs(),
        }
    }
}

pub(crate) trait DeriveAttributeFilter {
    /// Removes any non-veil attributes from the derive macro input.
    fn retain_veil_attrs(&mut self);
}
impl DeriveAttributeFilter for syn::DeriveInput {
    fn retain_veil_attrs(&mut self) {
        self.attrs.retain_veil_attrs();

        match &mut self.data {
            syn::Data::Struct(s) => s.fields.iter_mut().for_each(|field| field.attrs.retain_veil_attrs()),
            syn::Data::Enum(e) => e.variants.iter_mut().for_each(|variant| {
                variant.attrs.retain_veil_attrs();
                variant.fields.iter_mut();
            }),
            syn::Data::Union(u) => {
                u.fields.retain_veil_attrs();
            }
        }
    }
}
