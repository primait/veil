#![cfg_attr(not(test), allow(unused))]

use veil::Redact;

#[derive(Redact)]
#[redact(all, partial)]
struct Redactable(&'static str);

#[test]
#[should_panic]
fn test_fallback_redact_on() {
    format!("{:?}", Redactable(""));
}
