fn main() {}

#[derive(veil::Redact)]
struct Foo {
    #[redact(partial, fixed = 3)]
    bar: String
}

#[derive(veil::Redact)]
struct Bar {
    #[redact(fixed = 3, partial)]
    baz: String
}

#[derive(veil::Redact)]
enum Baz {
    #[redact(variant, fixed = 3, partial)]
    Foo {
        #[redact(fixed = 3, partial)]
        bar: String,
    }
}

#[derive(veil::Redact)]
enum Qux {
    #[redact(variant, partial, fixed = 3)]
    Foo {
        #[redact(partial, fixed = 3)]
        bar: String,
    }
}
