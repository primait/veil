#![allow(unused)]

use veil::Redact;

#[derive(Redact)]
#[redact(all, variant)]
enum CreditCardIssuer {
    Visa,
    Mastercard,
    Amex,
    Discover,
    DinersClub,
    Jcb,
    UnionPay,

    #[redact(skip, variant)]
    Other(#[redact] String),
}

fn main() {
    println!("{:#?}", CreditCardIssuer::Visa);
    println!("{:#?}", CreditCardIssuer::Other("Example Bank".to_string()));
}
