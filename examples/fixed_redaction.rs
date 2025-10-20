use veil::Redact;

#[derive(Redact)]
struct AuthInfo {
    login: String,
    #[redact(fixed = 3)]
    password: String,
}

fn main() {
    let info = AuthInfo {
        login: "name".to_string(),
        password: "/Jb@`?f.?%!hj2".to_string(),
    };

    println!("{info:#?}");
}
