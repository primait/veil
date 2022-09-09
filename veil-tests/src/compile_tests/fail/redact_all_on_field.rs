fn main() {}

#[derive(veil::Redact)]
pub enum Fooe {
    Bar {
        #[redact(all)]
        baz: String
    }
}

#[derive(veil::Redact)]
struct Foos {
    #[redact(all)]
    bar: String,
}
