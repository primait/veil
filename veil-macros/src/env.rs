//! If the `environment-aware` feature is enabled, the user can configure Veil's behaviour in different environments.
//!
//! If the environment variable `VEIL_DISBALE_REDACTION` is set, Veil will not redact any data.
//!
//! If the user's project contains a `.veil.toml` file, Veil will use the configuration in that file to figure out whether to redact data.
//! This is done at compile time. Runtime changes to this file will have no effect.
//!
//! The configuration file allows the user to specify what environment variables and their values should enable or disable redaction.
//!
//! For example, I can configure the file to redact when APP_ENV="production", and not redact when APP_ENV="development".

use lazy_static::lazy_static;
use proc_macro::TokenStream;
use quote::ToTokens;
use serde::Deserialize;
use std::{collections::BTreeMap, path::Path};
use syn::spanned::Spanned;

#[derive(Deserialize)]
#[serde(untagged)]
enum FallbackBehavior {
    Redact(bool),
    Panic(String),
}
impl ToTokens for FallbackBehavior {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Self::Redact(redact) => redact.to_tokens(tokens),
            Self::Panic(_) => quote! { panic!("Veil expected environment variables to be set that determine whether sensitive data should be redacted or not. Please refer to .veil.toml in the project files to see what Veil was expecting.") }.to_tokens(tokens)
        }
    }
}

#[derive(Deserialize)]
/// Should we redact data based on the values of environment variables?
struct EnvRedactConfig {
    #[serde(default)]
    /// Redaction should be ON if the environment variable is set to one of these values.
    redact: Vec<String>,

    #[serde(default)]
    #[serde(rename = "skip-redact")]
    /// Redaction should be OFF if the environment variable is set to one of these values.
    skip_redact: Vec<String>,
}

#[derive(Deserialize)]
/// If none of those environment variables are present...
struct FallbackRedactConfig {
    /// ...then we should [redact|not redact] the data.
    redact: FallbackBehavior,
}
impl Default for FallbackRedactConfig {
    fn default() -> Self {
        Self {
            redact: FallbackBehavior::Redact(true),
        }
    }
}

#[derive(Deserialize)]
struct TomlVeilConfig {
    #[serde(default)]
    fallback: FallbackRedactConfig,
    env: Option<BTreeMap<String, EnvRedactConfig>>,
}

#[derive(Default)]
pub struct VeilConfig {
    fallback: FallbackRedactConfig,
    env: BTreeMap<String, EnvRedactConfig>,
}
impl VeilConfig {
    pub fn read(path: &Path) -> Result<Self, VeilConfigError> {
        let config = std::fs::read_to_string(path)?;
        let config: TomlVeilConfig = toml::from_str(&config)?;

        // Ensure there are no duplicate key-value environment variable pairs.
        if let Some(env) = &config.env {
            let mut pairs = Vec::new();
            for (key, config) in env {
                // Ensure there are no empty environment variable configs.
                if config.redact.is_empty() && config.skip_redact.is_empty() {
                    return Err(VeilConfigError::Custom(format!(
                        "Environment variable {key:?} has an empty configuration"
                    )));
                }

                for value in [&config.redact, &config.skip_redact].into_iter().flatten() {
                    let pair = (key.as_str(), value.as_str());
                    if pairs.contains(&pair) {
                        return Err(VeilConfigError::Custom(format!(
                            "duplicate key-value environment variable pair: {pair:?}"
                        )));
                    } else {
                        pairs.push(pair);
                    }
                }
            }
        }

        // Ensure the fallback is configured correctly.
        if let FallbackBehavior::Panic(value) = &config.fallback.redact {
            if value != "panic" {
                return Err(VeilConfigError::Custom(
                    "fallback redaction behavior must be \"panic\"".to_string(),
                ));
            }
        }

        Ok(Self {
            env: config.env.unwrap_or_default(),
            fallback: config.fallback,
        })
    }
}

#[derive(Debug)]
pub enum VeilConfigError {
    IoError(std::io::Error),
    TomlError(toml::de::Error),
    Custom(String),
}
impl std::error::Error for VeilConfigError {}
impl std::fmt::Display for VeilConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(err) => write!(f, "I/O error reading .veil.toml: {err}"),
            Self::TomlError(err) => write!(f, "TOML error reading .veil.toml: {err:#?}"),
            Self::Custom(err) => write!(f, ".veil.toml error: {err}"),
        }
    }
}
impl From<std::io::Error> for VeilConfigError {
    #[inline]
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}
impl From<toml::de::Error> for VeilConfigError {
    #[inline]
    fn from(err: toml::de::Error) -> Self {
        Self::TomlError(err)
    }
}

lazy_static! {
    static ref CACHED_VEIL_CONFIG: Result<VeilConfig, VeilConfigError> = find_config();
}
fn find_config() -> Result<VeilConfig, VeilConfigError> {
    let manifest_dir = match std::env::var_os("CARGO_MANIFEST_DIR") {
        Some(manifest_dir) => manifest_dir,
        None => return Ok(Default::default()),
    };

    let mut manifest_dir = Path::new(&manifest_dir);

    // Walk up the directory tree until we find a .veil.toml file
    // We should stop at the workspace root
    let config_path = loop {
        let config_path = manifest_dir.join(".veil.toml");
        if config_path.is_file() {
            break Some(config_path);
        }

        // HACK! We can detect the workspace root by looking for Cargo.lock
        if manifest_dir.join("Cargo.lock").is_file() {
            break None;
        }

        manifest_dir = match manifest_dir.parent() {
            Some(parent) => parent,
            None => break None,
        };
    };

    let config_path = match config_path {
        Some(config_path) => config_path,
        None => return Ok(Default::default()),
    };

    VeilConfig::read(&config_path)
}

/// This macro expands to a function that returns true or false depending on whether the
/// data should be redacted based on the values of environment variables, configured by
/// `.veil.toml` in the project's files, if present.
pub fn env_is_redaction_enabled(input: TokenStream) -> TokenStream {
    let config = match &*CACHED_VEIL_CONFIG {
        Ok(config) => config,
        Err(err) => {
            return syn::Error::new(proc_macro2::TokenStream::from(input).span(), err.to_string())
                .into_compile_error()
                .into()
        }
    };

    let env = config.env.iter().map(|(key, config)| {
        let redacts = &config.redact;
        let skips = &config.skip_redact;
        quote! {
            if let Ok(value) = ::std::env::var(#key) {
                static REDACTS: &[&str] = &[#(#redacts),*];
                if REDACTS.contains(&value.as_str()) {
                    return true;
                }

                static SKIPS: &[&str] = &[#(#skips),*];
                if SKIPS.contains(&value.as_str()) {
                    return false;
                }
            }
        }
    });

    let fallback = config.fallback.redact.to_token_stream();
    if config.env.is_empty() {
        quote! {
            #[inline(always)]
            fn __veil_env_is_redaction_enabled() -> bool {
                #fallback
            }
        }
        .into()
    } else {
        quote! {
            #[inline(never)]
            fn __veil_env_is_redaction_enabled() -> bool {
                #(#env)*
                #fallback
            }
        }
        .into()
    }
}
