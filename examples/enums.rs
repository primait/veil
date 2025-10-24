#![allow(unused)]

use veil::Redact;

#[derive(Redact)]
#[redact(all, variant)] // Redact all the variant names! We can still skip individual variants later on by marking them as `#[redact(variant, skip)]`
enum CreditCardIssuer {
    Visa,
    Mastercard,
    Amex,
    Discover,
    DinersClub,
    Jcb,
    UnionPay,

    // Don't redact the name of this variant
    #[redact(variant, skip)]
    Other(
        // But do redact the contents of this field!
        #[redact] String,
    ),
}

fn main() {
    println!("{:#?}", CreditCardIssuer::Visa);
    println!("{:#?}", CreditCardIssuer::Other("Example Bank".to_string()));
}
