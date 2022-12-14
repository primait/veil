macro_rules! fail_tests {
    {$($name:ident),*} => {$(
        #[test]
        fn $name() {
            trybuild::TestCases::new().compile_fail(concat!("src/compile_tests/fail/", stringify!($name), ".rs"));
        }
    )*};
}
fail_tests! {
    redact_all_on_field,
    redact_all_variant_on_variant,
    redact_enum_without_variant,
    redact_partial_fixed,
    redact_too_many,
    redact_unused,
    redact_variant_on_field,
    redact_variant_on_struct,
    redact_union,
    redact_units,
    redact_skip,
    redact_missing_all,
    redact_display_enum_variant
}
