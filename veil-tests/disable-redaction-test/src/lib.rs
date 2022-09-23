#![cfg_attr(not(test), allow(unused))]
//! Simple test that ensures veil can actually be disabled

use veil::Redact;
use veil_tests::{SENSITIVE_DATA, assert_has_sensitive_data};
#[test]
fn test_veil_can_be_disabled() {
    #[derive(Redact)]
    struct SensitiveWrapper(#[redact] String);
    veil::disable().ok();
    assert_has_sensitive_data(SensitiveWrapper(SENSITIVE_DATA[0].to_string()));
}
