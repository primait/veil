use std::fmt::{Debug, Display};

#[repr(transparent)]
pub struct DisplayDebug(pub String);
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

pub enum RedactSpecialization {
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
    Option,
}

#[derive(Clone, Copy)]
pub struct RedactFlags {
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

    fn redact_fixed(width: usize, char: char) -> String {
        String::from_iter(std::iter::repeat_with(|| char).take(width))
    }
}

pub enum RedactionTarget<'a> {
    /// Redact the output of the type's [`std::fmt::Debug`] implementation.
    Debug {
        this: &'a dyn Debug,

        /// Sourced from [`std::fmt::Formatter::alternate`]
        alternate: bool,
    },

    /// Redact the output of the type's [`std::fmt::Display`] implementation.
    Display(&'a dyn Display),
}

fn redact_impl(redactable_string: String, flags: RedactFlags, specialization: Option<RedactSpecialization>) -> String {
    let mut redacted = String::with_capacity(redactable_string.len());

    #[allow(clippy::single_match)]
    match specialization {
        Some(RedactSpecialization::Option) => {
            if redactable_string == "None" {
                // We don't need to do any redacting
                // https://prima.slack.com/archives/C03URH9N43U/p1661423554871499
                return redactable_string;
            } else if let Some(inner) = redactable_string
                .strip_prefix("Some(")
                .and_then(|inner| inner.strip_suffix(')'))
            {
                redacted.push_str("Some(");
                flags.redact_partial(inner, &mut redacted);
                redacted.push(')');
            } else {
                // This should never happen, but just in case...
                flags.redact_full(&redactable_string, &mut redacted);
            }
            return redacted;
        }

        _ => {}
    }

    if flags.partial {
        flags.redact_partial(&redactable_string, &mut redacted);
    } else {
        flags.redact_full(&redactable_string, &mut redacted);
    }

    redacted
}

pub(crate) fn redact_from_builder(
    redactable_string: String,
    flags: RedactFlags,
    specialization: Option<RedactSpecialization>,
) -> String {
    #[cfg(feature = "toggle")]
    if crate::toggle::get_redaction_behavior().is_plaintext() {
        return redactable_string;
    }

    if flags.fixed > 0 {
        return RedactFlags::redact_fixed(flags.fixed as usize, flags.redact_char);
    }

    redact_impl(redactable_string, flags, specialization)
}

pub fn redact(this: RedactionTarget, flags: RedactFlags, specialization: Option<RedactSpecialization>) -> DisplayDebug {
    let to_redactable_string = || match this {
        RedactionTarget::Debug { this, alternate: false } => format!("{:?}", this),
        RedactionTarget::Debug { this, alternate: true } => format!("{:#?}", this),
        RedactionTarget::Display(this) => this.to_string(),
    };

    #[cfg(feature = "toggle")]
    if crate::toggle::get_redaction_behavior().is_plaintext() {
        return DisplayDebug(to_redactable_string());
    }

    if flags.fixed > 0 {
        return DisplayDebug(RedactFlags::redact_fixed(flags.fixed as usize, flags.redact_char));
    }

    DisplayDebug(redact_impl(to_redactable_string(), flags, specialization))
}

pub fn redact_pii(this: &dyn Display, flags: RedactFlags) -> String {
    redact(RedactionTarget::Display(this), flags, None).0
}
