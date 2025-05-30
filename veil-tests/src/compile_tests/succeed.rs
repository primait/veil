//! Tests that ensure that the compiler can compile the code.
#![allow(unused, dead_code)]

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
#[redact(all, partial, with = 'X', display)]
struct RedactAllWithFlagsDisplay {
    field: String,

    #[redact(skip)]
    field2: String,

    field3: String,
}

#[derive(Redact)]
struct RedactNamedDisplay {
    #[redact(display)]
    field: String,
    #[redact]
    field2: String,
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
    #[redact(all, fixed = 6, with = '$', display)]
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
    use arbitrary::{Arbitrary, Unstructured};
    use rand::prelude::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Redact, Arbitrary)]
    struct MultipleAttributes {
        #[redact]
        #[serde(default)]
        foo: bool,
        bar: bool,
    }

    #[derive(Serialize, Deserialize, Redact, Arbitrary)]
    #[serde(rename_all = "camelCase")]
    #[redact(all)]
    struct MultipleAttributesAll {
        #[serde(default)]
        #[redact(partial)]
        foo: bool,
        bar: bool,
    }

    #[derive(Serialize, Deserialize, Redact, Arbitrary)]
    struct MultipleAttributesTuple(
        #[redact]
        #[serde(default)]
        bool,
        #[serde(default)] bool,
    );

    #[derive(Serialize, Deserialize, Redact, Arbitrary)]
    #[serde(rename_all = "camelCase")]
    #[redact(all)]
    struct MultipleAttributesAllTuple(
        #[serde(default)]
        #[redact(partial)]
        bool,
        #[serde(default)] bool,
    );

    #[derive(Serialize, Deserialize, Redact, Arbitrary)]
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

    #[derive(Serialize, Deserialize, Redact, Arbitrary)]
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

    let mut rng = rand_pcg::Pcg64Mcg::new(
        // chosen by a fair dice roll
        // guarnteed to be random
        // (deterministic RNG keeps our test runs consistent)
        221,
    );
    macro_rules! test_serde_attributes {
        {$($ty:ty),*} => {
            $({
                // Arbitrary can fail if there isn't enough data, 1024 should be enough for everything
                let data = rng.random::<[u8;1024]>();
                let random = <$ty>::arbitrary(&mut Unstructured::new(&data)).unwrap();
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
