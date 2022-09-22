use std::env;
use veil::Redact;

#[derive(Redact)]
pub struct Customer {
    #[redact(partial)]
    first_name: String,
}

fn main() {
    // If the environment variable DISABLE_REDACTION is set veil will not redact anything
    if let Ok(_) = env::var("DISABLE_REDACTION") {
        veil::set_debug_format(veil::DebugFormat::Plaintext).ok();
    }

    println!(
        "{:#?}",
        Customer {
            first_name: "John".to_string(),
        }
    );
}
