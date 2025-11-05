// Redaction works with nested structures too!

use veil::Redact;

#[derive(Redact)]
pub struct Address {
    #[redact(partial)]
    line1: String,

    #[redact(partial)]
    line2: String,

    #[redact]
    house_or_flat_number: Option<u32>,

    #[redact]
    postcode: String,

    #[redact(partial)]
    city: String,
}

#[derive(Redact)]
pub struct Person {
    #[redact(partial)]
    first_name: String,

    #[redact(partial)]
    last_name: String,

    address: Address,
}

#[derive(Redact)]
pub struct Vehicle {
    #[redact(partial)]
    license_plate: String,

    owner: Person,
}

fn main() {
    println!(
        "{:#?}",
        Vehicle {
            license_plate: "ABC123".to_string(),
            owner: Person {
                first_name: "John".to_string(),
                last_name: "Doe".to_string(),
                address: Address {
                    line1: "123 Main St".to_string(),
                    line2: "Apt 1".to_string(),
                    house_or_flat_number: Some(1),
                    postcode: "12345".to_string(),
                    city: "New York".to_string(),
                }
            }
        }
    );
}
