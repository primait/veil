#![cfg_attr(not(test), allow(unused))]

use veil::Redact;
use veil_tests::{SENSITIVE_DATA, assert_has_sensitive_data};

#[derive(Redact)]
#[redact(all, partial)]
struct Redactable(&'static str);

#[test]
fn test_fallback_redact_off() {
    assert_has_sensitive_data(Redactable(SENSITIVE_DATA[1]));
}
