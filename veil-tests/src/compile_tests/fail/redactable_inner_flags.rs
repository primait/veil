fn main() {}

#[derive(veil::Redactable)]
struct Foo {
    #[redact(partial)]
    bar: String
}
