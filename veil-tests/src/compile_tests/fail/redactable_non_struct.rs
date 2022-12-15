fn main() {}

#[derive(veil::Redactable)]
enum Foo {}

#[derive(veil::Redactable)]
union Bar {
    a: i32
}
