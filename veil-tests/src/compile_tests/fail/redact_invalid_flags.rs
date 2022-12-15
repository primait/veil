fn main() {}

#[derive(veil::Redact)]
struct InvalidChar(#[redact(with = "this isn't a char")] ());

#[derive(veil::Redact)]
struct InvalidFixedWidth(#[redact(fixed = 0)] ());
