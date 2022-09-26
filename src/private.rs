use std::fmt::Debug;

#[repr(transparent)]
pub struct DisplayDebug(String);
impl std::fmt::Debug for DisplayDebug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}
impl AsRef<str> for DisplayDebug {
    #[inline(always)]
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

pub struct RedactFlags {
    /// Sourced from [`std::fmt::Formatter::alternate`]
    pub debug_alternate: bool,

    /// Whether the type we're redacting is an Option<T> or not. Poor man's specialization! This is detected
    /// by the proc macro reading the path to the type, so it's not perfect.
    ///
    /// This could be improved & rid of in a number of different ways in the future:
    ///
    /// * Once specialization is stabilized, we can use a trait to override redacting behaviour for some types,
    /// one of which would be Option<T>.
    ///
    /// * Once std::ptr::metadata and friends are stabilized, we could use it to unsafely cast the dyn Debug pointer
    /// to a concrete Option<T> and redact it directly. Probably not the best idea.
    ///
    /// * Once trait upcasting is stabilized, we could use it to upcast the dyn Debug pointer to a dyn Any and then
    /// downcast it to a concrete Option<T> and redact it directly.
    pub is_option: bool,

    /// Whether to only partially redact the data.
    ///
    /// Incompatible with `fixed`.
    pub partial: bool,

    /// What character to use for redacting.
    pub redact_char: char,

    /// Whether to redact with a fixed width, ignoring the length of the data.
    ///
    /// Incompatible with `partial`.
    pub fixed: u8,
}
impl RedactFlags {
    /// How many characters must a word be for it to be partially redacted?
    ///
    /// Words smaller than this many characters (NOT bytes) will be fully redacted.
    const MIN_PARTIAL_CHARS: usize = 5;

    /// Maximum number of characters to expose at the beginning and end of a partial redact.
    const MAX_PARTIAL_EXPOSE: usize = 3;

    fn redact_partial(&self, str: &str, redacted: &mut String) {
        let count = str.chars().filter(|char| char.is_alphanumeric()).count();
        if count < Self::MIN_PARTIAL_CHARS {
            for char in str.chars() {
                if char.is_alphanumeric() {
                    redacted.push(self.redact_char);
                } else {
                    redacted.push(char);
                }
            }
        } else {
            // The number of characters (prefix and suffix) we'll EXPOSE (NOT redact over)
            let redact_count = (count / 3).min(Self::MAX_PARTIAL_EXPOSE);

            let mut prefix_gas = redact_count;
            let mut middle_gas = count - redact_count - redact_count;
            for char in str.chars() {
                if char.is_alphanumeric() {
                    if prefix_gas > 0 {
                        prefix_gas -= 1;
                        redacted.push(char);
                    } else if middle_gas > 0 {
                        middle_gas -= 1;
                        redacted.push(self.redact_char);
                    } else {
                        redacted.push(char);
                    }
                } else {
                    redacted.push(char);
                }
            }
        }
    }

    fn redact_full(&self, str: &str, redacted: &mut String) {
        for char in str.chars() {
            if char.is_whitespace() || !char.is_alphanumeric() {
                redacted.push(char);
            } else {
                redacted.push(self.redact_char);
            }
        }
    }

    fn redact_fixed(&self, width: usize, redacted: &mut String) {
        redacted.reserve_exact(width);
        for _ in 0..width {
            redacted.push(self.redact_char);
        }
    }
}

pub fn redact(this: &dyn Debug, flags: RedactFlags) -> DisplayDebug {
    let mut redacted = String::new();

    #[cfg(feature = "toggle")]
    if crate::toggle::get_redaction_behavior().is_plaintext() {
        return DisplayDebug(if flags.debug_alternate {
            format!("{:#?}", this)
        } else {
            format!("{:?}", this)
        });
    }

    (|| {
        if flags.fixed > 0 {
            flags.redact_fixed(flags.fixed as usize, &mut redacted);
            return;
        }

        let debug_formatted = if flags.debug_alternate {
            format!("{:#?}", this)
        } else {
            format!("{:?}", this)
        };

        redacted.reserve(debug_formatted.len());

        // Specialize for Option<T>
        if flags.is_option {
            if debug_formatted == "None" {
                // We don't need to do any redacting
                // https://prima.slack.com/archives/C03URH9N43U/p1661423554871499
            } else if let Some(inner) = debug_formatted
                .strip_prefix("Some(")
                .and_then(|inner| inner.strip_suffix(')'))
            {
                redacted.push_str("Some(");
                flags.redact_partial(inner, &mut redacted);
                redacted.push(')');
            } else {
                // This should never happen, but just in case...
                flags.redact_full(&debug_formatted, &mut redacted);
            }
            return;
        }

        if flags.partial {
            flags.redact_partial(&debug_formatted, &mut redacted);
        } else {
            flags.redact_full(&debug_formatted, &mut redacted);
        }
    })();

    DisplayDebug(redacted)
}
