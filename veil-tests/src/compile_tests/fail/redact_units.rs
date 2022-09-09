fn main() {}

#[derive(veil::Redact)]
struct Foos;

#[derive(veil::Redact)]
enum Fooe {
    #[redact(all)]
    Bar,
}
