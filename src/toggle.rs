#![cfg_attr(docsrs, doc(cfg(feature = "toggle")))]

//! Makes it possible to disable veil's redaction behaviour

use once_cell::sync::OnceCell;

/// Enum describing how Veil should behave when `std::fmt::Debug` is called on a `#[derive(Redact)]` item
#[derive(Debug, Copy, Clone)]
#[cfg_attr(docsrs, doc(cfg(feature = "toggle")))]
pub enum RedactionBehavior {
    /// Redact the fields as normal
    Redact,
    /// Print the fields as plaintext
    Plaintext,
}
impl RedactionBehavior {
    /// Returns whether the current redaction behavior is to print redacted data
    pub fn is_redact(&self) -> bool {
        matches!(self, &RedactionBehavior::Redact)
    }

    /// Returns whether the current redaction behavior is to print data as plaintext
    pub fn is_plaintext(&self) -> bool {
        matches!(self, &RedactionBehavior::Plaintext)
    }
}

static DEBUG_FORMAT: OnceCell<RedactionBehavior> = OnceCell::new();

#[cfg_attr(docsrs, doc(cfg(feature = "toggle")))]
/// Disables Veil redaction globally.
///
/// See the "Environmental Awareness" section in the [crate level documentation](../index.html) for more information.
///
/// Should only be called once, preferrably at the top of main,
/// before any calls to [`std::fmt::Debug`], otherwise `Err` will be returned.
///
/// Overrides the `VEIL_DISABLE_REDACTION` environment variable, if set.
/// ```
/// // If the environment variable DISABLE_REDACTION is set veil will not redact anything
/// if let Ok(env) = std::env::var("APP_ENV") {
///     if env == "dev" {
///         veil::disable().unwrap();
///     }
/// }
/// ```
pub fn disable() -> Result<(), RedactionBehavior> {
    DEBUG_FORMAT.set(RedactionBehavior::Plaintext)
}

/// Get the current debug format value
pub(crate) fn get_redaction_behavior() -> RedactionBehavior {
    if let "1" | "on" = std::env::var("VEIL_DISABLE_REDACTION").unwrap_or_default().as_str() {
        *DEBUG_FORMAT.get_or_init(|| RedactionBehavior::Plaintext)
    } else {
        *DEBUG_FORMAT.get_or_init(|| RedactionBehavior::Redact)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn redaction_cant_be_set_after_reading() {
        assert!(get_redaction_behavior().is_redact());
        disable().unwrap_err();
        assert!(get_redaction_behavior().is_redact());
    }
}
