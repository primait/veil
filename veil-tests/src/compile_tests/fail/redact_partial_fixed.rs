fn main() {}

#[derive(veil::Redact)]
struct Foo {
    #[redact(fixed = 3, partial)]
    bar: String,
}

#[derive(veil::Redact)]
enum Fooe {
    #[redact(variant, fixed = 3, partial)]
    Bar {
        #[redact(fixed = 3, partial)]
        baz: String,
    }
}
