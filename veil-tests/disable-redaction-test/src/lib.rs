#![cfg_attr(not(test), allow(unused))]
//! Simple test that ensures veil can actually be disabled

use std::sync::Once;

use veil::Redact;
use veil_tests::{assert_has_sensitive_data, SENSITIVE_DATA};

static INIT: Once = Once::new();

fn initialize() {
    INIT.call_once(|| {
        veil::disable().ok();
    })
}

#[test]
fn test_veil_can_be_disabled() {
    initialize();

    #[derive(Redact)]
    struct SensitiveWrapper(#[redact] String);

    assert_has_sensitive_data(SensitiveWrapper(SENSITIVE_DATA[0].to_string()));
}

#[test]
fn test_disabling_veil_keeps_secrets_hidden() {
    initialize();

    #[derive(Redact)]
    struct SensitiveWrapper {
        #[redact]
        just_sensitive: &'static str,
        #[redact(secret)]
        secret: &'static str,
    }

    let secret = r#"~7"9?X4s'3z},$cKNW2ae+7RHDtnk_nD@G{u;D-}1FLl)7P|+&tom!`jR;J/)/N"#;
    let just_sensitive = SENSITIVE_DATA[2];
    let wrapper = SensitiveWrapper { secret, just_sensitive };

    println!("{wrapper:?}");
    assert_must_contain_sensitive_value(&wrapper, just_sensitive);
    assert_must_not_contain_secret_value(&wrapper, secret);
    assert_has_sensitive_data(wrapper);
}

// Helper to assert that a particular value is not being redacted out.
fn assert_must_contain_sensitive_value<T: std::fmt::Debug>(data: &T, sensitive: &'static str) {
    for redacted in [format!("{data:?}"), format!("{data:#?}")] {
        assert!(
            redacted.contains(sensitive),
            "{redacted:?} does not contain sensitive value: {sensitive:?}"
        )
    }
}
// Helper to assert that certain values in particular are redacted.
fn assert_must_not_contain_secret_value<T: std::fmt::Debug>(data: &T, secret: &'static str) {
    for redacted in [format!("{data:?}"), format!("{data:#?}")] {
        assert!(
            !redacted.contains(secret),
            "{redacted:?} contains secret value: {secret:?}"
        );
    }
}
