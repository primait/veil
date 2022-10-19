//! Tests that ensure that the compiler can compile the code.

#![allow(unused)]

pub mod fail;
pub mod succeed;

use veil::*;

#[derive(Redact)]
struct CreditCard {
    #[redact]
    cvv: String,

    #[redact(partial)]
    number: String,

    expiration: String,

    #[redact(with = 'X')]
    name: String,

    billing_address: Address,

    issuer: CreditCardIssuer,

    country: Country,
}

#[derive(Redact)]
struct Address {
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
#[redact(all)]
struct RedactAll {
    field: String,
    field2: String,
    field3: String,
}

#[derive(Redact)]
#[redact(all, partial, with = 'X')]
struct RedactAllWithFlags {
    field: String,

    #[redact(skip)]
    field2: String,

    field3: String,
}

#[derive(Redact)]
enum CreditCardIssuer {
    #[redact(variant)]
    Visa {
        #[redact(partial)]
        visa_data_1: String,

        #[redact(partial)]
        visa_data_2: String,
    },

    #[redact(variant, partial)]
    MasterCard,

    #[redact(variant)]
    #[redact(all, fixed = 6, with = '$')]
    SecretAgentCard {
        secret_data_1: String,
        secret_data_2: String,
    },
}

#[derive(Redact)]
#[redact(all, variant, partial)]
enum Country {
    #[doc = "hello world!"] // to test mixing attributes works ok
    #[redact(variant)]
    UnitedKingdom,
    Italy,
}

#[derive(Redact)]
struct TupleStruct(#[redact] u32, #[redact(partial)] u32);

#[test]
fn test_credit_card_redacting() {
    println!(
        "{:#?}",
        CreditCard {
            cvv: "098".to_string(),
            number: "1234 5678 9012 3456".to_string(),
            expiration: "12/34".to_string(),
            name: "John Doe".to_string(),
            billing_address: Address {
                line1: "123 Fake Street".to_string(),
                line2: "Apt. 1".to_string(),
                house_or_flat_number: Some(64),
                postcode: "12345".to_string(),
                city: "London".to_string(),
            },
            issuer: CreditCardIssuer::Visa {
                visa_data_1: "Hello".to_string(),
                visa_data_2: "World".to_string()
            },
            country: Country::UnitedKingdom,
        }
    );

    println!(
        "{:#?}",
        CreditCard {
            cvv: "098".to_string(),
            number: "1234 5678 9012 3456".to_string(),
            expiration: "12/34".to_string(),
            name: "John Doe".to_string(),
            billing_address: Address {
                line1: "123 Fake Street".to_string(),
                line2: "Apt. 1".to_string(),
                house_or_flat_number: Some(64),
                postcode: "12345".to_string(),
                city: "London".to_string(),
            },
            issuer: CreditCardIssuer::SecretAgentCard {
                secret_data_1: "Hello".to_string(),
                secret_data_2: "World".to_string()
            },
            country: Country::Italy,
        }
    );
}

#[test]
fn test_redact_all() {
    println!(
        "{:#?}",
        RedactAll {
            field: "Hello".to_string(),
            field2: "World".to_string(),
            field3: "!".to_string(),
        }
    );
}

#[test]
fn test_redact_all_with_flags() {
    println!(
        "{:#?}",
        RedactAllWithFlags {
            field: "Hello".to_string(),
            field2: "World".to_string(),
            field3: "!".to_string(),
        }
    );
}

#[test]
fn test_redact_tuple_struct() {
    println!("{:#?}", TupleStruct(100, 2000000));
}

#[test]
fn test_redact_multiple_attributes() {
    #[derive(serde::Serialize, serde::Deserialize, Redact)]
    struct MultipleAttributes {
        #[redact]
        #[serde(default)]
        hidden: bool,
        exposed: bool,
    }

    #[derive(serde::Serialize, serde::Deserialize, Redact)]
    #[serde(rename_all = "camelCase")]
    #[redact(all)]
    struct MultipleAttributesAll {
        #[serde(default)]
        hidden: bool,
        exposed: bool,
    }

    let json = serde_json::json!({
        "exposed": true
    });

    let attributes: MultipleAttributesAll = serde_json::from_value(json.clone()).unwrap();
    assert!(attributes.exposed);
    assert!(!attributes.hidden);

    let attributes: MultipleAttributes = serde_json::from_value(json).unwrap();
    assert!(attributes.exposed);
    assert!(!attributes.hidden);
}
