fn main() {}

#[derive(veil::Redact)]
#[redact(all = "ASDF")]
struct Foos {
    bar: String,
}
