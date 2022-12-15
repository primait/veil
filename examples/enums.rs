#![allow(unused)]

use veil::Redact;

#[derive(Redact)]
#[redact(all, variant)] // Redact all the variant names!
enum CreditCardIssuer {
    Visa,
    Mastercard,
    Amex,
    Discover,
    DinersClub,
    Jcb,
    UnionPay,

    #[redact(variant, skip)] // Don't redact the name of this variant
    Other(
        #[redact] // But do redact the contents of this field!
        String
    ),
}

fn main() {
    println!("{:#?}", CreditCardIssuer::Visa);
    println!("{:#?}", CreditCardIssuer::Other("Example Bank".to_string()));
}
