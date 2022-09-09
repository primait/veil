fn main() {}

#[derive(veil::Redact)]
#[redact(all, variant)]
struct Foo {
    #[redact]
    bar: String,
}
