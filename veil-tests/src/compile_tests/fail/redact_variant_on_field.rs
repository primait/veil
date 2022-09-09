fn main() {}

#[derive(veil::Redact)]
struct Foos {
    #[redact(variant)]
    bar: String,
}

#[derive(veil::Redact)]
enum Fooe {
    Bar {
        #[redact(variant)]
        baz: String,
    }
}
