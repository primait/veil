fn main() {}

#[derive(veil::Redact)]
#[redact(all)]
pub enum Foo {
    Bar,
}

#[derive(veil::Redact)]
enum Bar {
    #[redact]
    Baz,
}
