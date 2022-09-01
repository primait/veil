//! This needs its own crate because VEIL_DISABLE_REDACTION is only checked once
//! and another test in the other crates might run first, causing THIS test to fail.

#![cfg_attr(not(test), allow(unused))]

use veil::Redact;
use veil_tests::{assert_has_sensitive_data, SENSITIVE_DATA};

#[derive(Redact)]
#[redact(all, partial)]
struct Redactable(&'static str);

#[test]
fn test_disable_redact_env_var() {
    std::env::set_var("VEIL_DISABLE_REDACTION", "1");
    assert_has_sensitive_data(Redactable(SENSITIVE_DATA[1]));

    // Ensure it's only checked once
    std::env::remove_var("VEIL_DISABLE_REDACTION");
    assert_has_sensitive_data(Redactable(SENSITIVE_DATA[1]));
}
