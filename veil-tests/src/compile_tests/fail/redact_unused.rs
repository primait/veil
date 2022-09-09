fn main() {}

#[derive(veil::Redact)]
#[redact(all)]
struct Foos {
    #[redact(skip)]
    baz: String,
}

#[derive(veil::Redact)]
#[redact(all, variant)]
enum Fooe {
    #[redact(skip, variant)]
    #[redact(all)]
    Bar {
        #[redact(skip)]
        baz: String,
    }
}
