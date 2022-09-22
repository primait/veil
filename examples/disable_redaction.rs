use std::env;
use veil::Redact;

#[derive(Redact)]
pub struct Customer {
    #[redact(partial)]
    first_name: String,
}

fn main() {
    // If the environment variable DISABLE_REDACTION is set veil will not redact anything
    if let Ok(env) = env::var("APP_ENV") {
        if env == "dev" {
            // Note that veil::disable needs the `toggle` feature flag enabled
            veil::disable().unwrap();
        }
    }

    println!(
        "{:#?}",
        Customer {
            first_name: "John".to_string(),
        }
    );
}
