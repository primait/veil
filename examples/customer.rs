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
}

fn main() {
    println!(
        "{:#?}",
        Customer {
            id: 1,
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            email: Some("johndoe@example.com".to_string()),
            age: 30,
        }
    );
}
