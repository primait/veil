fn main() {}

#[derive(veil::Redact)]
struct InvalidChar(#[redact(with = 9)] ());

#[derive(veil::Redact)]
struct InvalidFixedWidth(#[redact(fixed = 0)] ());
