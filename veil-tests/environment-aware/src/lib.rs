#![cfg_attr(not(test), allow(unused))]

use parking_lot::Mutex;
use veil::Redact;
use veil_tests::{assert_has_sensitive_data, assert_no_sensitive_data};

const SENSITIVE_DATA: &str = veil_tests::SENSITIVE_DATA[1];

static ENVIRONMENT_LOCK: Mutex<()> = parking_lot::const_mutex(());

#[derive(Redact)]
#[redact(all, partial)]
struct Redactable(&'static str);

#[test]
fn test_production() {
    // We've set up redaction to happen when APP_ENV="production"
    let _lock = ENVIRONMENT_LOCK.lock();

    std::env::set_var("APP_ENV", "production");

    assert_no_sensitive_data(Redactable(SENSITIVE_DATA));
}

#[test]
fn test_staging() {
    // We've set up redaction to happen when APP_ENV="staging"
    let _lock = ENVIRONMENT_LOCK.lock();

    std::env::set_var("APP_ENV", "staging");

    assert_no_sensitive_data(Redactable(SENSITIVE_DATA));
}

#[test]
fn test_dev() {
    // We've set up redaction to NOT happen when APP_ENV="dev"
    let _lock = ENVIRONMENT_LOCK.lock();

    std::env::set_var("APP_ENV", "dev");

    assert_has_sensitive_data(Redactable(SENSITIVE_DATA));
}

#[test]
fn test_qa() {
    // We've set up redaction to NOT happen when APP_ENV="qa"
    let _lock = ENVIRONMENT_LOCK.lock();

    std::env::set_var("APP_ENV", "qa");

    assert_has_sensitive_data(Redactable(SENSITIVE_DATA));
}
