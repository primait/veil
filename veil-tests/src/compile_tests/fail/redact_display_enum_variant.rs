fn main() {}

#[derive(veil::Redact)]
pub enum Foo {
    #[redact(variant, display)]
    Bar,
}

#[derive(veil::Redact)]
pub enum Bar {
    #[redact(display)]
    Baz,
}

#[derive(veil::Redact)]
#[redact(all, variant, display)]
pub enum Baz {
    Qux,
}
