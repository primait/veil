use crate::util::give_me_a_formatter;
use std::{
    fmt::{Debug, Display, Write},
    num::NonZeroU8,
};

pub enum RedactSpecialization {
    /// Whether the type we're redacting is an [`Option<T>`] or not. Poor man's specialization! This is detected
    /// by the proc macro reading the path to the type, so it's not perfect.
    ///
    /// This could be improved & rid of in a number of different ways in the future:
    ///
    /// * Once specialization is stabilized, we can use a trait to override redacting behavior for some types,
    ///   one of which would be [`Option<T>`].
    ///
    /// * Once std::ptr::metadata and friends are stabilized, we could use it to unsafely cast the dyn Debug pointer
    ///   to a concrete [`Option<T>`] and redact it directly. Probably not the best idea.
    ///
    /// * Once trait upcasting is stabilized, we could use it to upcast the dyn Debug pointer to a dyn Any and then
    ///   downcast it to a concrete [`Option<T>`] and redact it directly.
    Option,
}

#[derive(Clone, Copy)]
pub enum RedactionLength {
    /// Redact the entire data.
    Full,

    /// Redact a portion of the data.
    Partial,

    /// Whether to redact with a fixed width, ignoring the length of the data.
    Fixed(NonZeroU8),
}

#[derive(Clone, Copy)]
pub enum RedactionContent<'a> {
    /// Redact with asterisks.
    Asterisks,

    /// Redact with a single character.
    Char(char),

    /// Redact with a string.
    Str(&'a str),
}

#[derive(Clone, Copy)]
pub struct RedactFlags {
    /// How much of the data to redact.
    pub redact_length: RedactionLength,

    /// The type of redaction to be used. Defaults to using `*`.
    pub redact_content: RedactionContent<'static>,
}
impl RedactFlags {
    /// How many characters must a word be for it to be partially redacted?
    ///
    /// Words smaller than this many characters (NOT bytes) will be fully redacted.
    const MIN_PARTIAL_CHARS: usize = 5;

    /// Maximum number of characters to expose at the beginning and end of a partial redact.
    const MAX_PARTIAL_EXPOSE: usize = 3;

    pub(crate) fn redact_partial(&self, fmt: &mut std::fmt::Formatter, to_redact: &str) -> std::fmt::Result {
        let count = to_redact.chars().filter(|char| char.is_alphanumeric()).count();
        match &self.redact_content {
            RedactionContent::Asterisks => self.redact_partial_with_char(fmt, to_redact, '*', count),
            RedactionContent::Char(c) => self.redact_partial_with_char(fmt, to_redact, *c, count),
            RedactionContent::Str(s) => fmt.write_str(s),
        }
    }

    fn redact_partial_with_char(
        &self,
        fmt: &mut std::fmt::Formatter,
        to_redact: &str,
        redact_char: char,
        count: usize,
    ) -> std::fmt::Result {
        if count < Self::MIN_PARTIAL_CHARS {
            for char in to_redact.chars() {
                if char.is_alphanumeric() {
                    fmt.write_char(redact_char)?;
                } else {
                    fmt.write_char(char)?;
                }
            }
        } else {
            // The number of characters (prefix and suffix) we'll EXPOSE (NOT redact over)
            let redact_count = (count / 3).min(Self::MAX_PARTIAL_EXPOSE);

            let mut prefix_gas = redact_count;
            let mut middle_gas = count - redact_count - redact_count;
            for char in to_redact.chars() {
                if char.is_alphanumeric() {
                    if prefix_gas > 0 {
                        prefix_gas -= 1;
                        fmt.write_char(char)?;
                    } else if middle_gas > 0 {
                        middle_gas -= 1;
                        fmt.write_char(redact_char)?;
                    } else {
                        fmt.write_char(char)?;
                    }
                } else {
                    fmt.write_char(char)?;
                }
            }
        }
        Ok(())
    }

    pub(crate) fn redact_full(&self, fmt: &mut std::fmt::Formatter, to_redact: &str) -> std::fmt::Result {
        match &self.redact_content {
            RedactionContent::Asterisks => self.redact_full_with_char(fmt, to_redact, '*'),
            RedactionContent::Char(c) => self.redact_full_with_char(fmt, to_redact, *c),
            RedactionContent::Str(s) => fmt.write_str(s),
        }
    }

    fn redact_full_with_char(
        &self,
        fmt: &mut std::fmt::Formatter,
        to_redact: &str,
        redact_char: char,
    ) -> std::fmt::Result {
        for char in to_redact.chars() {
            if char.is_whitespace() || !char.is_alphanumeric() {
                fmt.write_char(char)?;
            } else {
                fmt.write_char(redact_char)?;
            }
        }
        Ok(())
    }

    fn redact_fixed_str(fmt: &mut std::fmt::Formatter, width: usize, s: &str) -> std::fmt::Result {
        let mut buf = String::with_capacity(width);
        for char in s.chars().take(width) {
            buf.push(char);
        }
        fmt.write_str(&buf)
    }

    fn redact_fixed_char(fmt: &mut std::fmt::Formatter, width: usize, char: char) -> std::fmt::Result {
        let mut buf = String::with_capacity(width);
        for _ in 0..width {
            buf.push(char);
        }
        fmt.write_str(&buf)
    }
}

pub enum RedactionTarget<'a> {
    /// Redact the output of the type's [`Debug`] implementation.
    Debug {
        this: &'a dyn Debug,

        /// Sourced from [`std::fmt::Formatter::alternate`]
        alternate: bool,
    },

    /// Redact the output of the type's [`Display`] implementation.
    Display(&'a dyn Display),
}
impl RedactionTarget<'_> {
    /// Pass through directly to the formatter.
    #[cfg(feature = "toggle")]
    pub(crate) fn passthrough(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RedactionTarget::Debug { this, .. } => std::fmt::Debug::fmt(this, fmt),
            RedactionTarget::Display(this) => std::fmt::Display::fmt(this, fmt),
        }
    }
}

impl Display for RedactionTarget<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            RedactionTarget::Debug { this, alternate: false } => write!(f, "{this:?}"),
            RedactionTarget::Debug { this, alternate: true } => write!(f, "{this:#?}"),
            RedactionTarget::Display(this) => write!(f, "{this}"),
        }
    }
}

pub struct RedactionFormatter<'a> {
    pub this: RedactionTarget<'a>,
    pub flags: RedactFlags,
    pub specialization: Option<RedactSpecialization>,
}
impl std::fmt::Debug for RedactionFormatter<'_> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[cfg(feature = "toggle")]
        if crate::toggle::get_redaction_behavior().is_plaintext() {
            return self.this.passthrough(fmt);
        }

        if let RedactionLength::Fixed(n) = &self.flags.redact_length {
            match &self.flags.redact_content {
                RedactionContent::Asterisks => {
                    return RedactFlags::redact_fixed_char(fmt, n.get() as usize, '*');
                }
                RedactionContent::Char(c) => {
                    return RedactFlags::redact_fixed_char(fmt, n.get() as usize, *c);
                }
                RedactionContent::Str(s) => {
                    return RedactFlags::redact_fixed_str(fmt, n.get() as usize, s);
                }
            }
        }

        let redactable_string = self.this.to_string();

        #[allow(clippy::single_match)]
        match self.specialization {
            Some(RedactSpecialization::Option) => {
                if redactable_string == "None" {
                    // We don't need to do any redacting
                    // https://prima.slack.com/archives/C03URH9N43U/p1661423554871499
                    return fmt.write_str("None");
                } else if let Some(inner) = redactable_string
                    .strip_prefix("Some(")
                    .and_then(|inner| inner.strip_suffix(')'))
                {
                    fmt.write_str("Some(")?;
                    if let RedactionLength::Partial = &self.flags.redact_length {
                        self.flags.redact_partial(fmt, inner)?;
                    } else {
                        self.flags.redact_full(fmt, inner)?;
                    }
                    return fmt.write_char(')');
                } else {
                    // This should never happen, but just in case...
                    return self.flags.redact_full(fmt, &redactable_string);
                }
            }

            None => {}
        }

        if let RedactionLength::Partial = &self.flags.redact_length {
            self.flags.redact_partial(fmt, &redactable_string)
        } else {
            self.flags.redact_full(fmt, &redactable_string)
        }
    }
}

pub fn derived_redactable(this: &dyn Display, flags: RedactFlags) -> String {
    give_me_a_formatter(|fmt| {
        std::fmt::Debug::fmt(
            &RedactionFormatter {
                this: RedactionTarget::Display(this),
                flags,
                specialization: None,
            },
            fmt,
        )
    })
    .to_string()
}
