[![crates.io](https://img.shields.io/crates/v/veil.svg)](https://crates.io/crates/veil)
[![Documentation](https://docs.rs/veil/badge.svg)](https://docs.rs/veil/)
[![License](https://img.shields.io/crates/l/veil)](https://github.com/primait/veil/blob/master/LICENSE)
[![CI status](https://drone-1.prima.it/api/badges/primait/veil/status.svg?branch=master)](https://drone-1.prima.it/primait/veil)

A derive macro that implements [`std::fmt::Debug`](https://doc.rust-lang.org/std/fmt/trait.Debug.html) for a struct or enum variant, with certain fields redacted.

The purpose of this macro is to allow for easy, configurable and efficient redaction of sensitive data in structs and enum variants.
This can be used to hide sensitive data in logs or anywhere where personal data should not be exposed or stored.

# Usage

Add to your Cargo.toml:

```toml
[dependencies]
veil = "0.1"
```

Usage documentation can be found [here](https://docs.rs/veil).

# Example

The example is explained in detail [here](https://docs.rs/veil).

```rust
#[derive(Redact)]
struct CreditCard {
    #[redact(partial)]
    number: String,

    #[redact]
    expiry: String,

    #[redact(fixed = 3)]
    cvv: String,

    #[redact(partial)]
    cardholder_name: String,
}

#[derive(Redact)]
#[redact(all, variant)]
enum CreditCardIssuer {
    MasterCard,
    Visa,
    AmericanExpress,
}

#[derive(Redact)]
#[redact(all, partial)]
struct Vehicle {
    license_plate: String,
    make: String,
    model: String,
    color: String,
}

#[derive(Debug)]
struct Policy {
    id: Uuid,
    name: String,
    description: String,
}

#[derive(Redact)]
enum InsuranceStatus {
    #[redact(all, partial)]
    Insured {
        #[redact(fixed = 12)]
        policy: Policy,

        policy_started: String,
        policy_expires: String,

        #[redact(skip)]
        payment_card: CreditCard,

        #[redact(skip)]
        vehicles: Vec<Vehicle>,
    },

    Uninsured {
        policies_available: Vec<Policy>,
    },
}
```

# Skip redacting data
In testing environments it may be useful not to censor your logs. You can configure Veil to redact or skip redacting data based during runtime using the `set_debug_format` function. See this [example](examples/disable_redaction.rs)
