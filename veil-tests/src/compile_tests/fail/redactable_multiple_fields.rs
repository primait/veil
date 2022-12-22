fn main() {}

#[derive(veil::Redactable)]
struct Foo {
    bar: String,
    baz: String,
}

#[derive(veil::Redactable)]
struct Foo2(String, String);
