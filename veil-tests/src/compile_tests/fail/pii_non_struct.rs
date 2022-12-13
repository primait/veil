fn main() {}

#[derive(veil::Pii)]
enum Foo {}

#[derive(veil::Pii)]
union Bar {
    a: i32
}
