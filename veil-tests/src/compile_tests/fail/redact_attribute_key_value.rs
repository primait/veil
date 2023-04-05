fn main() {}

#[derive(veil::Redact)]
#[redact = "ASDF"]
struct Foos {
    bar: String,
}
