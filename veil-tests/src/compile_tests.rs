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

#[derive(Redact)]
struct GenericStruct<Foo: std::fmt::Debug, Bar: std::fmt::Debug>(Foo, #[redact] Bar);

#[derive(Redact)]
struct GenericWhereStruct<Foo, Bar>(Foo, #[redact] Bar)
where
    Foo: std::fmt::Debug,
    Bar: std::fmt::Debug;

#[derive(Redact)]
enum GenericWhereEnum<Foo, Bar>
where
    Foo: std::fmt::Debug,
    Bar: std::fmt::Debug,
{
    FooVariant(Foo),
    BarVariant(#[redact] Bar),
}

#[derive(Redact)]
enum GenericEnum<Foo: std::fmt::Debug, Bar: std::fmt::Debug> {
    FooVariant(Foo),
    BarVariant(#[redact] Bar),
}

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
    use rand_derive2::RandGen;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Redact, RandGen)]
    struct MultipleAttributes {
        #[redact]
        #[serde(default)]
        foo: bool,
        bar: bool,
    }

    #[derive(Serialize, Deserialize, Redact, RandGen)]
    #[serde(rename_all = "camelCase")]
    #[redact(all)]
    struct MultipleAttributesAll {
        #[serde(default)]
        #[redact(partial)]
        foo: bool,
        bar: bool,
    }

    #[derive(Serialize, Deserialize, Redact, RandGen)]
    struct MultipleAttributesTuple(
        #[redact]
        #[serde(default)]
        bool,
        bool,
    );

    #[derive(Serialize, Deserialize, Redact, RandGen)]
    #[serde(rename_all = "camelCase")]
    #[redact(all)]
    struct MultipleAttributesAllTuple(
        #[serde(default)]
        #[redact(partial)]
        bool,
        bool,
    );

    #[derive(Serialize, Deserialize, Redact, RandGen)]
    #[serde(rename_all = "camelCase")]
    enum MultipleAttributesEnum {
        #[serde(rename = "foo")]
        #[redact(variant)]
        Foo,

        #[serde(rename = "bar")]
        #[redact(all)]
        Bar {
            #[serde(default)]
            #[redact(partial)]
            foo: bool,
            bar: bool,
        },

        #[redact(all)]
        #[serde(rename = "baz")]
        Baz(
            #[serde(default)]
            #[redact(partial)]
            bool,
            bool,
        ),
    }

    #[derive(Serialize, Deserialize, Redact, RandGen)]
    #[serde(rename_all = "camelCase")]
    #[redact(all, variant)]
    enum MultipleAttributesEnumAll {
        #[serde(rename = "foo")]
        Foo,

        #[serde(rename = "bar")]
        #[redact(all)]
        Bar { foo: bool, bar: bool },

        #[redact(all)]
        #[serde(rename = "baz")]
        Baz(bool, bool),

        #[serde(rename = "qux")]
        #[redact(all)]
        Qux {
            #[serde(default)]
            #[redact(partial)]
            foo: bool,
            bar: bool,
        },

        #[serde(rename = "quux")]
        #[redact(all)]
        Quux(
            #[serde(default)]
            #[redact(partial)]
            bool,
            bool,
        ),
    }

    let json = serde_json::json!({
        "bar": true
    });
    let attributes: MultipleAttributesAll = serde_json::from_value(json).unwrap();
    assert!(!attributes.foo);
    assert!(attributes.bar);

    macro_rules! test_serde_attributes {
        {$($ty:ty),*} => {
            $({
                let random = <$ty>::generate_random();
                let json = serde_json::to_string(&random).unwrap();
                serde_json::from_str::<$ty>(&json).unwrap();
            })*
        };
    }
    test_serde_attributes! {
        MultipleAttributes,
        MultipleAttributesAll,
        MultipleAttributesEnum,
        MultipleAttributesEnumAll,
        MultipleAttributesTuple,
        MultipleAttributesAllTuple
    }
}
