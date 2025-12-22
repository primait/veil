use veil::Redact;

#[derive(Redact)]
pub struct Customer {
    id: u64,

    #[redact(partial)]
    first_name: String,

    #[redact(partial)]
    last_name: String,

    #[redact]
    email: Option<String>,

    #[redact(fixed = 2)]
    age: u32,

    #[redact(with = "[REDACTED]")]
    address: String,
}

fn main() {
    println!(
        "{:#?}",
        Customer {
            id: 1,
            // This will be partially redacted
            first_name: "Johnathan".to_string(),
            // This will be fully redacted since the number of characters is not sufficient for
            // partial redaction
            last_name: "Doe".to_string(),
            // By default, only alphabetic characters are redacted
            email: Some("johndoe@example.com".to_string()),
            age: 30,
            address: "1234 Elm Street, Springfield, XZ".to_string(),
        }
    );
}
