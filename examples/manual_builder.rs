use veil::{Redactor, RedactorBuilder};

fn main() {
    // Build a new Redactor.
    // We'll set up the Redactor to use flags that are equivalent to:
    // `#[redact(with = 'X', partial))]`
    // on a field, when using the `Redact` derive macro.
    let redactor: Redactor = RedactorBuilder::new().char('X').partial().build().unwrap();

    // We can now redact any string we want in a number of different ways...

    // Firstly, we can simply redact directly to a `String`:
    assert_eq!(redactor.redact("Hello, world!".to_string()), "HelXX, XXrld!");

    // Or, we can redact a `String` in-place, which is slightly more efficient,
    // and we can also chain multiple redactions together:
    let mut hello = "Hello, world!".to_string();
    let mut goodbye = "Goodbye, world!".to_string();
    redactor.and_redact(&mut hello).and_redact(&mut goodbye);
    assert_eq!(hello, "HelXX, XXrld!");
    assert_eq!(goodbye, "GooXXXX, XXrld!");

    // Finally, we can use the `wrap` method to wrap a string in a `RedactWrapped` struct,
    // which implements `Debug` and `Display` to redact the string when displayed or debugged.
    let hello = "Hello, world!".to_string();
    let hello_wrapped = redactor.wrap(&hello);

    assert_ne!(hello_wrapped.to_string(), hello);
    assert_ne!(format!("{:?}", hello_wrapped), format!("{:?}", hello));

    assert_eq!(hello_wrapped.to_string(), "HelXX, XXrld!");
    assert_eq!(format!("{:?}", hello_wrapped), "\"HelXX, XXrld!\"");
}
