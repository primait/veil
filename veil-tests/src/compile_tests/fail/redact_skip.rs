fn main() {}

#[derive(veil::Redact)]
struct Foos {
    #[redact(skip)]
    bar: String,
}

#[derive(veil::Redact)]
enum Fooe {
    #[redact(skip)]
    Bar {
        #[redact(skip)]
        baz: String
    }
}

#[derive(veil::Redact)]
enum Bar {
    Baz {
        #[redact(skip)]
        qux: String
    }
}

#[derive(veil::Redact)]
#[redact(all)]
struct Baz {
    #[redact(skip, partial)]
    qux: String,
}
