fn main() {}

#[derive(veil::Redact)]
struct Foos {
    #[redact]
    #[redact]
    bar: String,
}

#[derive(veil::Redact)]
enum Fooe {
    #[redact(variant)]
    #[redact(variant)]
    Bar,
}

#[derive(veil::Redact)]
#[redact(all)]
#[redact(all)]
struct Bars {
    baz: String,
}

#[derive(veil::Redact)]
#[redact(all, variant)]
#[redact(all, variant)]
enum Bare {
    #[redact(all)]
    #[redact(all)]
    Baz {
        qux: String,
    }
}

#[derive(veil::Redact)]
enum Qux {
    #[redact(all)]
    #[redact(all)]
    #[redact(variant)]
    #[redact(variant)]
    Quux {
        quuz: String,
    }
}

#[derive(veil::Redact)]
enum Corge {
    #[redact(all)]
    #[redact(all)]
    Grault {
        garply: String,
    }
}

#[derive(veil::Redact)]
enum Waldo {
    #[redact(variant)]
    #[redact(variant)]
    Fred
}
