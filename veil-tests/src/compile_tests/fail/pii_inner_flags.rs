fn main() {}

#[derive(veil::Pii)]
struct Foo {
    #[redact(partial)]
    bar: String
}
