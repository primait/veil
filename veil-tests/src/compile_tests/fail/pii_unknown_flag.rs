fn main() {}

#[derive(veil::Pii)]
#[redact(skip)]
struct Foo {
    inner: String
}

#[derive(veil::Pii)]
#[redact(all)]
struct Bar {
    inner: String
}

#[derive(veil::Pii)]
#[redact(blah)]
struct Baz {
    inner: String
}
