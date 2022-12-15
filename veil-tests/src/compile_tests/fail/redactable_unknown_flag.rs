fn main() {}

#[derive(veil::Redactable)]
#[redact(skip)]
struct Foo {
    inner: String
}

#[derive(veil::Redactable)]
#[redact(all)]
struct Bar {
    inner: String
}

#[derive(veil::Redactable)]
#[redact(blah)]
struct Baz {
    inner: String
}
