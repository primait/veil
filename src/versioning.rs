//! Tests that ensure that we don't break the packages we publish to crates.io with invalid versions, and keep version in the README.md up to date.

#[test]
fn test_readme_version() {
    // Ensure the version in the README is up to date

    let readme_version = {
        let mut readme = include_str!("../README.md");

        readme = &readme[readme
            .find("[dependencies]")
            .expect("Couldn't find `[dependencies]` table")
            + "[dependencies]".len()..];

        readme = &readme[readme
            .find("veil = \"")
            .expect("Couldn't find `veil = \"` in the `[dependencies]` table")
            + "veil = \"".len()..];

        readme = &readme[..readme
            .find('"')
            .expect("Couldn't find the first `\"` in the `veil = \"` line")];

        readme
    };

    assert_eq!(
        readme_version,
        env!("CARGO_PKG_VERSION"),
        "The version in the README is out of date, please update it to match the version in Cargo.toml"
    );
}

#[test]
fn test_macros_version() {
    // Ensure that the version in `veil-macros` matches the version of `veil`

    assert_eq!(
        veil_macros::__private_version!(),
        env!("CARGO_PKG_VERSION"),
        "The version in `veil-macros` is out of date, please update it to match the version in Cargo.toml"
    );
}

#[test]
fn test_macros_crate_version() {
    // Ensure that the version in `veil-macros` in the `[dependencies]` table in `Cargo.toml` matches the version of `veil`

    #[derive(serde::Deserialize)]
    struct VeilMacrosCrate {
        version: String,
    }

    #[derive(serde::Deserialize)]
    struct Dependencies {
        #[serde(rename = "veil-macros")]
        veil_macros: VeilMacrosCrate,
    }

    #[derive(serde::Deserialize)]
    struct CargoManifest {
        dependencies: Dependencies,
    }

    let manifest: CargoManifest = toml::from_str(include_str!("../Cargo.toml")).expect("Couldn't parse Cargo.toml");

    assert!(
        manifest.dependencies.veil_macros.version.starts_with('='),
        "The version of `veil-macros` in the `[dependencies]` table in `veil` `Cargo.toml` should start with `=` to pin it to the specific matching version"
    );

    assert_eq!(
        manifest.dependencies.veil_macros.version,
        concat!("=", env!("CARGO_PKG_VERSION")),
        "The version of `veil-macros` that `veil` depends on in `[dependencies]` is out of date, please update it to match the version in Cargo.toml"
    );
}
