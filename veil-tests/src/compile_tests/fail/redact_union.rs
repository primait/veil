fn main() {}

#[derive(veil::Redact)]
union Foo {
    bar: u32,
    baz: u64
}
