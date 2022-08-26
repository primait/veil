#![allow(unused)]

use veil::*;

#[derive(Mask)]
struct CreditCard {
    #[mask]
    cvv: String,

    #[mask(partial)]
    number: String,

    expiration: String,

    #[mask(with = 'X')]
    name: String,

    billing_address: Address,

    issuer: CreditCardIssuer,

    country: Country,
}

#[derive(Mask)]
struct Address {
    #[mask(partial)]
    line1: String,

    #[mask(partial)]
    line2: String,

    #[mask]
    house_or_flat_number: Option<u32>,

    #[mask]
    postcode: String,

    #[mask(partial)]
    city: String,
}

#[derive(Mask)]
#[mask(all)]
struct MaskAll {
    field: String,
    field2: String,
    field3: String,
}

#[derive(Mask)]
#[mask(all, partial, with = 'X')]
struct MaskAllWithFlags {
    field: String,
    field2: String,
    field3: String,
}

#[derive(Mask)]
enum CreditCardIssuer {
    #[mask(variant)]
    Visa {
        #[mask(partial)]
        visa_data_1: String,

        #[mask(partial)]
        visa_data_2: String,
    },

    #[mask(variant, partial)]
    MasterCard,

    #[mask(variant)]
    #[mask(all, fixed = 6, with = '$')]
    SecretAgentCard {
        secret_data_1: String,
        secret_data_2: String,
    },
}

#[derive(Mask, Default)]
#[mask(all, variant, partial)]
enum Country {
    #[default] // to test mixing attributes works ok
    #[mask(variant)]
    UnitedKingdom,
    Italy,
}

#[derive(Mask)]
struct TupleStruct(#[mask] u32, #[mask(partial)] u32);

#[test]
fn test_credit_card_masking() {
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
fn test_mask_all() {
    println!(
        "{:#?}",
        MaskAll {
            field: "Hello".to_string(),
            field2: "World".to_string(),
            field3: "!".to_string(),
        }
    );
}

#[test]
fn test_mask_all_with_flags() {
    println!(
        "{:#?}",
        MaskAllWithFlags {
            field: "Hello".to_string(),
            field2: "World".to_string(),
            field3: "!".to_string(),
        }
    );
}

#[test]
fn test_mask_tuple_struct() {
    println!("{:#?}", TupleStruct(100, 2000000));
}
