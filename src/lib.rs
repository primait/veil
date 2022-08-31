//! Implements [`std::fmt::Debug`] for a struct or enum variant, with certain fields redacted.
//!
//! The purpose of this macro is to allow for easy, configurable and efficient redaction of sensitive data in structs and enum variants.
//! This can be used to hide sensitive data in logs or anywhere where personal data should not be exposed or stored.
//! 
//! Redaction is unicode-aware. Only alphanumeric characters are redacted. Whitespace, symbols and other characters are left as-is.
//!
//! # Controlling Redaction
//!
//! Using the `#[redact]` attribute, you can control which fields are redacted and how.
//!
//! **Fields without this attribute will NOT be redacted and will be shown using their default [`std::fmt::Debug`] implementation.**
//!
//! Modifiers can be applied to control how the field is redacted:
//!
//! | **Modifier**                   |   | **Effects**                                                                                                                                                                       |   | **Default**                                   |
//! |--------------------------------|---|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|---|-----------------------------------------------|
//! | `#[redact(partial)]`           |   | If the string is long enough, a small part of the<br>beginning and end will be exposed.                                                                                           |   | Disabled. The entire string will be redacted. |
//! | `#[redact(with = 'X')]`        |   | Specifies the `char` the string will be redacted with.                                                                                                                            |   | `'*'`                                         |
//! | `#[redact(fixed = <integer>)]` |   | If this modifier is present, the length and contents of<br>the string are completely ignored and the string will always<br>be redacted as a fixed number of redaction characters. |   | Disabled.                                     |
//!
//! # Enum Variants
//!
//! If the variant names of an enum themselves are sensitive data, you can use the `#[redact(variant)]` modifier to redact the name of the variant.
//!
//! All the normal modifiers can be used on a redacted variant name as well.
//!
//! # Tips & Tricks
//!
//! ## Redacting all fields
//!
//! You can also quickly redact all fields in a struct using the `#[redact(all)]` modifier.
//!
//! This modifier can be applied to **structs** and **enum variants**.
//!
//! The above modifiers are also accepted as configuration options when using this modifier, for example: `#[redact(all, partial, with = 'X')]`
//!
//! This modifier acts as a default for all fields in the struct or enum variant. You can still individually control each field's redaction using the `#[redact]` modifier.
//!
//! Finally, you can also manually turn off redaction for a field by using the `#[redact(skip)]` modifier. This is of course only allowed when the field is affected by `#[redact(all)]`, as fields are not redacted by default.
//!
//! ## Specializations
//!
//! Currently, we specialize the implementation for the types below.
//!
//! **Please note that specializations are somewhat heuristic. For example, if you use a type alias in place of a specialized type, the specialization will not be applied as we can't detect the actual type used.**
//!
//! | **Type**    |   | **Specialization**                                      |
//! |-------------|---|---------------------------------------------------------|
//! | `Option<T>` |   | The data inside a `Some(...)` variant will be redacted. |
//!
//! # Examples
//!
//! ```rust
//! # type Uuid = ();
//! # use veil_macros::Redact;
//! #[derive(Redact)]
//! struct CreditCard {
//!     #[redact(partial)]
//!     number: String,
//!
//!     #[redact]
//!     expiry: String,
//!
//!     #[redact(fixed = 3)]
//!     cvv: String,
//!
//!     #[redact(partial)]
//!     cardholder_name: String,
//! }
//!
//! #[derive(Redact)]
//! #[redact(all, variant)]
//! enum CreditCardIssuer {
//!     MasterCard,
//!     Visa,
//!     AmericanExpress,
//! }
//!
//! #[derive(Redact)]
//! #[redact(all, partial)]
//! struct Vehicle {
//!     license_plate: String,
//!     make: String,
//!     model: String,
//!     color: String,
//! }
//!
//! // This struct doesn't contain any sensitive data, so we can derive `Debug` as normal.
//! #[derive(Debug)]
//! struct Policy {
//!     id: Uuid,
//!     name: String,
//!     description: String,
//! }
//!
//! #[derive(Redact)]
//! enum InsuranceStatus {
//!     #[redact(all, partial)]
//!     Insured {
//!         #[redact(fixed = 12)]
//!         policy: Policy,
//!
//!         policy_started: String,
//!         policy_expires: String,
//!
//!         #[redact(skip)]
//!         // We already derive `Redact` for `CreditCard`, so we shouldn't re-redact it.
//!         payment_card: CreditCard,
//!
//!         #[redact(skip)]
//!         // Redacting a `Vec<Vehicle>` would redact the entire list, so we disable redaction for this field.
//!         // This doesn't necessarily mean that the field is not redacted - because we derived `Redact` for `Vehicle`,
//!         // the `Vehicle`'s struct fields will still be redacted.
//!         vehicles: Vec<Vehicle>,
//!     },
//!
//!     // No redaction is necessary here as `Policy` is not sensitive data in this context.
//!     Uninsured {
//!         policies_available: Vec<Policy>,
//!     },
//! }
//! ```
//! 
//! # Limitations
//! 
//! Currently, this macro only supports [`std::fmt::Debug`] formatting with no modifiers (`{:?}`) or the "alternate" modifier (`{:#?}`).
//! Modifiers like padding, alignment, etc. are not supported as the Rust standard library does not expose any of this behaviour for us.
//!
//! ## A note on [`std::fmt::Display`]
//!
//! This derive macro does **NOT** implement [`std::fmt::Display`]. If you want to implement it, you can do so manually.
//!
//! [`std::fmt::Display`] should NOT be masked. It is meant to be human-readable, and also has a snowball effect on [`ToString`]
//! as [`std::fmt::Display`] automatically implements it, leading to confusing and unexpected behaviour.

pub use veil_macros::Redact;

#[doc(hidden)]
pub mod private;
