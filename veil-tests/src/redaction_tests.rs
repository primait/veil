//! Tests that ensure that sensitive data is actually redacted.

#![cfg_attr(not(test), allow(unused))]

use veil::{Redact, Redactable};

pub const SENSITIVE_DATA: &[&str] = &[
    "William",
    "Assicurazioni",
    "039845734895",
    "10 Downing Street",
    "SensitiveVariant",
];

const DEBUGGY_PHRASE: &str = "Hello \"William\"!\nAnd here's the newline...";

pub fn assert_has_sensitive_data<T: std::fmt::Debug>(data: T) {
    for redacted in [format!("{data:?}"), format!("{data:#?}")] {
        assert!(
            SENSITIVE_DATA.iter().any(|sensitive| redacted.contains(sensitive)),
            "{redacted:?} doesn't contain any sensitive data"
        );
    }
}

pub fn assert_no_sensitive_data<T: std::fmt::Debug>(data: T) {
    for redacted in [format!("{data:?}"), format!("{data:#?}")] {
        for sensitive in SENSITIVE_DATA {
            assert!(
                !redacted.contains(sensitive),
                "{redacted:?} contains sensitive data: {sensitive:?}"
            );
        }
    }
}

#[test]
fn test_sensitive_enum_variants() {
    #[derive(Redact)]
    enum SensitiveVariants {
        #[redact(variant)]
        SensitiveVariant1(#[redact] &'static str, #[redact] &'static str),

        #[redact(variant, partial)]
        SensitiveVariant2 {
            #[redact(partial)]
            data1: &'static str,

            #[redact(partial)]
            data2: &'static str,
        },
    }

    #[derive(Redact)]
    #[redact(all, variant, partial)]
    enum SensitiveVariantsAll {
        #[redact(all)]
        SensitiveVariant1(&'static str, &'static str),

        #[redact(all, partial)]
        SensitiveVariant2 { data1: &'static str, data2: &'static str },
    }

    assert_no_sensitive_data(SensitiveVariants::SensitiveVariant1(
        SENSITIVE_DATA[0],
        SENSITIVE_DATA[1],
    ));
    assert_no_sensitive_data(SensitiveVariants::SensitiveVariant2 {
        data1: SENSITIVE_DATA[2],
        data2: SENSITIVE_DATA[3],
    });

    assert_no_sensitive_data(SensitiveVariantsAll::SensitiveVariant1(
        SENSITIVE_DATA[0],
        SENSITIVE_DATA[1],
    ));
    assert_no_sensitive_data(SensitiveVariantsAll::SensitiveVariant2 {
        data1: SENSITIVE_DATA[2],
        data2: SENSITIVE_DATA[3],
    });
}

#[test]
fn test_sensitive_structs() {
    #[derive(Redact)]
    struct SensitiveStruct {
        #[redact]
        data1: &'static str,

        #[redact(partial)]
        data2: &'static str,

        #[redact(fixed = 6)]
        data3: &'static str,

        #[redact(with = '$')]
        data4: &'static str,
    }

    #[derive(Redact)]
    #[redact(all, partial)]
    struct SensitiveStructAll {
        data1: &'static str,
        data2: &'static str,
        data3: &'static str,
        data4: &'static str,
    }

    assert_no_sensitive_data(SensitiveStruct {
        data1: SENSITIVE_DATA[0],
        data2: SENSITIVE_DATA[1],
        data3: SENSITIVE_DATA[2],
        data4: SENSITIVE_DATA[3],
    });

    assert_no_sensitive_data(SensitiveStructAll {
        data1: SENSITIVE_DATA[0],
        data2: SENSITIVE_DATA[1],
        data3: SENSITIVE_DATA[2],
        data4: SENSITIVE_DATA[3],
    });
}

#[test]
fn test_sensitive_tuple_structs() {
    #[derive(Redact)]
    struct SensitiveStruct(
        #[redact] &'static str,
        #[redact(partial)] &'static str,
        #[redact(fixed = 6)] &'static str,
        #[redact(with = '$')] &'static str,
    );

    #[derive(Redact)]
    #[redact(all, partial)]
    struct SensitiveStructAll(&'static str, &'static str, &'static str, &'static str);

    assert_no_sensitive_data(SensitiveStruct(
        SENSITIVE_DATA[0],
        SENSITIVE_DATA[1],
        SENSITIVE_DATA[2],
        SENSITIVE_DATA[3],
    ));

    assert_no_sensitive_data(SensitiveStructAll(
        SENSITIVE_DATA[0],
        SENSITIVE_DATA[1],
        SENSITIVE_DATA[2],
        SENSITIVE_DATA[3],
    ));
}

#[test]
fn test_display_redaction() {
    #[derive(Redact)]
    struct RedactDisplay(#[redact(display)] String);

    #[derive(Redact)]
    struct RedactDebug(#[redact] String);

    assert_eq!(format!("{:?}", RedactDebug("\"".to_string())), r#"RedactDebug("\"")"#);
    assert_eq!(format!("{:?}", RedactDisplay("\"".to_string())), r#"RedactDisplay(")"#);
}

#[test]
fn test_named_display_redaction() {
    #[derive(Redact)]
    struct RedactMultipleNamedDisplay {
        #[redact(display)]
        foo: String,
        #[redact]
        bar: String,
    }

    assert_eq!(
        format!("{:?}", RedactMultipleNamedDisplay { foo: DEBUGGY_PHRASE.to_string(), bar: DEBUGGY_PHRASE.to_string() }),
        "RedactMultipleNamedDisplay { foo: ***** \"*******\"!\n*** ****'* *** *******..., bar: \"***** \\\"*******\\\"!\\**** ****'* *** *******...\" }"
    );
}

#[test]
fn test_enum_display_redaction() {
    #[derive(Redact)]
    enum RedactEnum {
        Foo {
            #[redact(display)]
            foo: String,
            #[redact]
            bar: String,
        },
    }

    assert_eq!(
        format!("{:?}", RedactEnum::Foo { foo: DEBUGGY_PHRASE.to_string(), bar: DEBUGGY_PHRASE.to_string() }),
        "Foo { foo: ***** \"*******\"!\n*** ****'* *** *******..., bar: \"***** \\\"*******\\\"!\\**** ****'* *** *******...\" }"
    );
}

#[test]
fn test_derive_sensitive() {
    #[derive(Redactable)]
    struct SensitiveString(String);
    impl std::fmt::Display for SensitiveString {
        fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.0.fmt(fmt)
        }
    }

    let sensitive = SensitiveString(SENSITIVE_DATA[0].to_string());

    assert_no_sensitive_data(sensitive.redact());

    let mut buffer = String::new();
    sensitive.redact_into(&mut buffer).unwrap();
    assert_no_sensitive_data(buffer);
}

#[test]
fn test_derive_sensitive_modifiers() {
    #[derive(Redactable)]
    #[redact(fixed = 3, with = '-')]
    struct SensitiveString(String);
    impl std::fmt::Display for SensitiveString {
        fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.0.fmt(fmt)
        }
    }

    let sensitive = SensitiveString(SENSITIVE_DATA[0].to_string());

    assert_eq!(sensitive.redact(), "---");

    let mut buffer = String::new();
    sensitive.redact_into(&mut buffer).unwrap();
    assert_eq!(buffer, "---");
}

#[test]
fn test_enum_variant_names() {
    #[derive(Debug)]
    enum Control {
        Foo(String),
        Bar,
    }

    #[derive(Redact)]
    enum Redacted {
        #[redact(variant)]
        Foo(#[redact] String),
        Bar,
    }

    #[derive(Redact)]
    #[redact(all, variant)]
    enum RedactedAll {
        #[redact(skip, variant)]
        Foo(#[redact] String),
        #[redact(skip, variant)]
        Bar,
    }

    assert_eq!(format!("{:?}", RedactedAll::Bar), format!("{:?}", Redacted::Bar));
    assert_ne!(
        format!("{:?}", Redacted::Foo("Hello".to_string())),
        format!("{:?}", RedactedAll::Foo("Hello".to_string()))
    );
    assert_eq!(
        format!("{:?} {:?}", Control::Foo("Hello".to_string()), Control::Bar),
        "Foo(\"Hello\") Bar"
    );
    assert_eq!(
        format!("{:?} {:?}", Redacted::Foo("Hello".to_string()), Redacted::Bar),
        "***(\"*****\") Bar"
    );
    assert_eq!(format!("{:?}", Control::Bar), format!("{:?}", Redacted::Bar));
    assert_ne!(
        format!("{:?}", Control::Foo("Hello".to_string())),
        format!("{:?}", Redacted::Foo("Hello".to_string()))
    );
}

#[test]
fn test_struct_name() {
    #[derive(Debug)]
    struct Control {
        #[allow(dead_code)]
        foo: String,
    }

    #[derive(Redact)]
    struct Redacted {
        #[redact]
        foo: String,
    }

    assert_eq!(
        format!(
            "{:?}",
            Control {
                foo: "Hello".to_string()
            }
        ),
        "Control { foo: \"Hello\" }"
    );
    assert_eq!(
        format!(
            "{:?}",
            Redacted {
                foo: "Hello".to_string()
            }
        ),
        "Redacted { foo: \"*****\" }"
    );
}
