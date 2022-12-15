/// HACK! To reduce code duplication in the generated binary, this macro is used to generate a
/// struct that gives us quick access to a `&mut std::fmt::Formatter` which is used extensively
/// throughout the `private` module.
///
/// This allows us to design & expose an API that can return `String` directly, but reuses the same
/// code we've already written that writes to a `&mut std::fmt::Formatter` internally.
///
/// The user can move some data into context, additionally with an attached lifetime to allow for
/// passing references. The user can then use the `fmt` argument to format the data with existing
/// redaction functions & code.
macro_rules! give_me_a_formatter {
    (
        $(move$(<$lifetime:lifetime>)? {
            $($field:ident: $ty:ty = $move:expr),+
        })?

        fn fmt(&self, $fmt:ident: &mut std::fmt::Formatter<'_>) -> std::fmt::Result $code:block
    ) => {{
        struct GiveMeAFormatter $($(<$lifetime>)?)? {
            _phantom: std::marker::PhantomData< $($(& $lifetime)?)? () >,
            $($($field: $ty),+)?
        }
        impl $($(<$lifetime>)?)? std::fmt::Display for GiveMeAFormatter $($(<$lifetime>)?)? {
            #[inline]
            fn fmt(&self, $fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                $(let Self { $($field,)+ .. } = self;)?
                $code
            }
        }
        #[allow(clippy::redundant_field_names)] {
            GiveMeAFormatter {
                _phantom: std::marker::PhantomData,
                $($($field: $move),+)?
            }.to_string()
        }
    }};
}
pub(crate) use give_me_a_formatter;
