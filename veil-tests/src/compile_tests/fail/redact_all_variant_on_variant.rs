fn main() {}

#[derive(veil::Redact)]
pub enum Foo {
    #[redact(all, variant)]
    Bar,
}
