//! Makes it possible to disable veil's redaction behaviour
//!
use once_cell::sync::OnceCell;
/// Enum describing the behaviour of veil
#[derive(Debug, Copy, Clone)]
pub enum DebugFormat {
    /// Redact the fields as normal
    Redacted,
    /// Print the fields as plaintext
    Plaintext,
}

impl DebugFormat {
    pub fn is_redacted(&self) -> bool {
        matches!(self, &DebugFormat::Redacted)
    }

    pub fn is_plaintext(&self) -> bool {
        matches!(self, &DebugFormat::Plaintext)
    }
}

static DEBUG_FORMAT: OnceCell<DebugFormat> = OnceCell::new();

/// Sets the formatting of the debug logs
///
/// Should only be called once, preferrably at the top of main,
/// before any calls to [`std::fmt::debug`] or [`get_debug_format`].
///
/// If sucessfuly set the value returns Ok(()),
/// otherwise returns Err with the current value.
pub fn set_debug_format(v: DebugFormat) -> Result<(), DebugFormat> {
    DEBUG_FORMAT.set(v)
}

/// Get the current debug format value
pub fn get_debug_format() -> DebugFormat {
    *DEBUG_FORMAT.get_or_init(|| DebugFormat::Redacted)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn debug_format_can_only_be_set_once() {
        set_debug_format(DebugFormat::Redacted).unwrap();
        assert!(get_debug_format().is_redacted());
        set_debug_format(DebugFormat::Plaintext).unwrap_err();
        assert!(get_debug_format().is_redacted());
    }
}
