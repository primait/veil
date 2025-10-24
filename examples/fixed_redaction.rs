// By default, redaction will be applied to alphabetic characters. If you want to fully redact
// secrets, passwords, emails and so on, you can use the `fixed` option, which will also hide the
// length of the original string

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
