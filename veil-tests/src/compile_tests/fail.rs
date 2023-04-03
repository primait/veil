macro_rules! fail_tests {
    {$($name:ident),*} => {$(
        #[test]
        fn $name() {
            trybuild::TestCases::new().compile_fail(concat!("src/compile_tests/fail/", stringify!($name), ".rs"));
        }
    )*};
}
fail_tests! {
    redact_invalid_flags,
    redact_all_on_field,
    redact_all_variant_on_variant,
    redact_enum_without_variant,
    redact_too_many,
    redact_unused,
    redact_variant_on_field,
    redact_variant_on_struct,
    redact_union,
    redact_units,
    redact_skip,
    redact_missing_all,
    redact_display_enum_variant,
    redact_incompatible_flags,
    redactable_empty_struct,
    redactable_inner_flags,
    redactable_multiple_fields,
    redactable_non_struct,
    redact_all_with_value,
    redactable_unknown_flag
}
