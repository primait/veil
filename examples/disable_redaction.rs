// If you are developing and want to disable redaction, you can!
//
// Beware of using this option in staging or production

use std::env;
use veil::Redact;

#[derive(Redact)]
pub struct Customer {
    #[redact(partial)]
    first_name: String,
}

fn main() {
    // If the environment variable APP_ENV is set to "dev" veil will not redact anything
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
