#![cfg_attr(not(test), allow(unused))]

use veil::Redact;
use veil_tests::{assert_no_sensitive_data, SENSITIVE_DATA};

#[derive(Redact)]
#[redact(all, partial)]
struct Redactable(&'static str);

#[test]
fn test_fallback_redact_on() {
    assert_no_sensitive_data(Redactable(SENSITIVE_DATA[1]));
}
