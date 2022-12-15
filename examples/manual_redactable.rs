use veil::Redactable;

// As an alternative to redacting in a type's `std::fmt::Debug` implementation (which is what the `Redact` derive macro implements),
// you can also use the `Redactable` type to more explicitly redact structured data.
//
// The `Redactable` trait requires that a type implements `std::fmt::Display`, as this is what will be used to redact the type.

#[derive(Redactable)]
#[derive(Debug)] // `Redactable` doesn't touch `Debug` at all, so you can still derive it.
#[redact(with = 'X', partial)] // All the modifier flags you know and love from the `Redact` derive macro are also available here.
struct EmailAddress(String);

// Our `Display` implementation for `EmailAddress` will simply print out the email address as-is.
// This is what will be used to redact the type.
impl std::fmt::Display for EmailAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

fn main() {
    let email = EmailAddress("john.doe@prima.it".to_string());

    // The `Debug` implementation is untouched and will work as expected.
    assert_eq!(format!("{:?}", email), "EmailAddress(\"john.doe@prima.it\")");

    // So will the `Display` implementation.
    assert_eq!(format!("{}", email), "john.doe@prima.it");

    // And this is how we redact the data!
    assert_eq!(email.redact(), "johX.XXX@XXXXa.it");

    // We can also redact the data into an existing buffer, which is slightly more efficient if you've already got one lying around.
    let mut buffer = String::new();
    email.redact_into(&mut buffer).unwrap();
    assert_eq!(buffer, "johX.XXX@XXXXa.it");
}
