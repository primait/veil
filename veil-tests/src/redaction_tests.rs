//! Tests that ensure that sensitive data is actually redacted.

#![cfg_attr(not(test), allow(unused))]

use veil::Redact;

const SENSITIVE_DATA: &[&str] = &[
    "William",
    "Assicurazioni",
    "039845734895",
    "10 Downing Street",
    "SensitiveVariant",
];

fn assert_no_sensitive_data<T: std::fmt::Debug>(data: T) {
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
