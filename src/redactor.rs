//! The [`Redactor`] allows for redacting arbitrary strings using a pre-defined set of flags.
//!
//! To build a [`Redactor`], use the [`RedactorBuilder`].

use crate::{
    private::{RedactFlags, RedactionFormatter, RedactionLength, RedactionTarget},
    util::give_me_a_formatter,
};
use std::fmt::{Debug, Display};

/// A wrapped reference to some data that, when formatted as [`Debug`] or [`Display`] (if implemented for `T`), will be redacted.
///
/// See [`Redactor::wrap`] for more information.
#[derive(Clone, Copy)]
pub struct RedactWrapped<'a, T> {
    data: &'a T,
    flags: &'a RedactFlags,
}
impl<T> Display for RedactWrapped<'_, T>
where
    T: Display,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(
            &RedactionFormatter {
                this: RedactionTarget::Display(self.data),
                flags: *self.flags,
                specialization: None,
            },
            fmt,
        )
    }
}
impl<T> Debug for RedactWrapped<'_, T>
where
    T: Debug,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(
            &RedactionFormatter {
                this: RedactionTarget::Debug {
                    this: self.data,
                    alternate: fmt.alternate(),
                },
                flags: *self.flags,
                specialization: None,
            },
            fmt,
        )
    }
}

/// The `Redactor` allows for redacting arbitrary strings using a pre-defined set of flags.
///
/// To build a `Redactor`, use the [`RedactorBuilder`].
pub struct Redactor(RedactFlags);
impl Redactor {
    /// Returns a builder ([`RedactorBuilder`]) for this type.
    #[inline(always)]
    pub const fn builder() -> RedactorBuilder {
        RedactorBuilder::new()
    }

    /// Redact the given string.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use veil::redactor::RedactorBuilder;
    /// let email = "john.doe@prima.it".to_string();
    /// let name = "John Doe".to_string();
    ///
    /// let redactor = Redactor::builder().char('X').partial().build().unwrap();
    ///
    /// let email = redactor.redact(email);
    /// let name = redactor.redact(name);
    ///
    /// assert_eq!(
    ///     format!("{} <{}>", name, email),
    ///     "JoXX Xoe <johX.XXX@XXXXa.it>"
    /// );
    /// ```
    pub fn redact(&self, data: String) -> String {
        give_me_a_formatter(|fmt| {
            std::fmt::Debug::fmt(
                &RedactionFormatter {
                    this: RedactionTarget::Display(&data.as_str()),
                    flags: self.0,
                    specialization: None,
                },
                fmt,
            )
        })
        .to_string()
    }

    /// Redact the given string in-place.
    //
    /// Can be chained for convenience.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use veil::redactor::RedactorBuilder;
    /// let mut email = "john.doe@prima.it".to_string();
    /// let mut name = "John Doe".to_string();
    ///
    /// Redactor::builder()
    ///     .char('X')
    ///     .partial()
    ///     .build()
    ///     .unwrap()
    ///     .redact_in_place(&mut email)
    ///     .redact_in_place(&mut name);
    ///
    /// assert_eq!(
    ///     format!("{} <{}>", name, email),
    ///     "JoXX Xoe <johX.XXX@XXXXa.it>"
    /// );
    /// ```
    pub fn redact_in_place(&self, data: &mut String) -> &Self {
        *data = self.redact(core::mem::take(data));
        self
    }

    /// Wrap the given data in a [`RedactWrapped`], allowing it to be redacted when displayed or debugged.
    ///
    /// Currently, the only supported [`Debug`] formats are `{:?}` and `{:#?}`. Other flags will be ignored.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use veil::redactor::RedactorBuilder;
    /// let email = "john.doe@prima.it".to_string();
    /// let name = "John Doe".to_string();
    ///
    /// let redactor = Redactor::builder()
    ///     .char('X')
    ///     .partial()
    ///     .build()
    ///     .unwrap();
    ///
    /// let email = redactor.wrap(&email);
    /// let name = redactor.wrap(&name);
    ///
    /// assert_eq!(
    ///     format!("{} <{}>", name, email),
    ///     "JoXX Xoe <johX.XXX@XXXXa.it>"
    /// );
    ///
    /// assert_eq!(
    ///     format!("{:?} <{:#?}>", name, email),
    ///     "\"JoXX Xoe\" <\"johX.XXX@XXXXa.it\">"
    /// );
    /// ```
    pub const fn wrap<'a, T>(&'a self, data: &'a T) -> RedactWrapped<'a, T> {
        RedactWrapped { flags: &self.0, data }
    }
}

/// A checked builder for [`Redactor`]s.
pub struct RedactorBuilder {
    redact_char: Option<char>,
    partial: bool,
}
impl RedactorBuilder {
    /// Initialize a new redaction flag builder.
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            redact_char: None,
            partial: false,
        }
    }

    /// Set the character to use for redacting.
    ///
    /// Equivalent to `#[redact(with = '...')]` when deriving.
    #[inline(always)]
    pub const fn char(mut self, char: char) -> Self {
        self.redact_char = Some(char);
        self
    }

    /// Whether to only partially redact the data.
    ///
    /// Equivalent to `#[redact(partial)]` when deriving.
    #[inline(always)]
    pub const fn partial(mut self) -> Self {
        self.partial = true;
        self
    }

    /// Build the redaction flags.
    ///
    /// Returns an error if the state of the builder is invalid.
    /// The error will be optimised away by the compiler if the builder is valid at compile time, so it's safe and zero-cost to use `unwrap` on the result if you are constructing this at compile time.
    #[inline(always)]
    pub const fn build(self) -> Result<Redactor, &'static str> {
        let mut flags = RedactFlags {
            redact_length: if self.partial {
                RedactionLength::Partial
            } else {
                RedactionLength::Full
            },

            redact_char: '*',
        };

        if let Some(char) = self.redact_char {
            flags.redact_char = char;
        }

        Ok(Redactor(flags))
    }
}
