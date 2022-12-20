/// HACK! To reduce code duplication in the generated binary, this function is used to generate a
/// struct that gives us quick access to a `&mut std::fmt::Formatter` which is used extensively
/// throughout the `private` module.
///
/// This allows us to design & expose an API that can return `String` directly, but reuses the same
/// code we've already written that writes to a `&mut std::fmt::Formatter` internally.
pub fn give_me_a_formatter<F>(op: F) -> impl std::fmt::Display
where
    F: Fn(&mut std::fmt::Formatter<'_>) -> std::fmt::Result,
{
    struct GiveMeAFormatter<F>(F)
    where
        F: Fn(&mut std::fmt::Formatter<'_>) -> std::fmt::Result;

    impl<F> std::fmt::Display for GiveMeAFormatter<F>
    where
        F: Fn(&mut std::fmt::Formatter<'_>) -> std::fmt::Result,
    {
        #[inline(always)]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            (self.0)(f)
        }
    }

    GiveMeAFormatter(op)
}
