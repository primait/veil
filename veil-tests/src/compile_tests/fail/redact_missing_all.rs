fn main() {}

#[derive(veil::Redact)]
#[redact]
struct Foos {
    bar: String
}

#[derive(veil::Redact)]
#[redact]
enum Fooe {
    Bar
}

#[derive(veil::Redact)]
#[redact(variant)]
enum Bar {
    Baz
}

#[derive(veil::Redact)]
#[redact(all)]
enum Baz {
    Qux
}
