use crate::private::{redact_from_builder, RedactFlags};

/// The `Redactor` allows for redacting arbitrary strings using a pre-defined set of flags.
///
/// To build a `Redactor`, use the [`RedactorBuilder`].
pub struct Redactor(RedactFlags);
impl Redactor {
    /// Redact the given string.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use veil::RedactorBuilder;
    /// let email = "john.doe@prima.it".to_string();
    /// let name = "John Doe".to_string();
    ///
    /// let redactor = RedactorBuilder::new().char('X').partial().build().unwrap();
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
        redact_from_builder(data, self.0, None)
    }

    /// Redact the given string in-place.
    ///
    /// Convenience method for chaining.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use veil::RedactorBuilder;
    /// let mut email = "john.doe@prima.it".to_string();
    /// let mut name = "John Doe".to_string();
    ///
    /// RedactorBuilder::new()
    ///     .char('X')
    ///     .partial()
    ///     .build()
    ///     .unwrap()
    ///     .and_redact(&mut email)
    ///     .and_redact(&mut name);
    ///
    /// assert_eq!(
    ///     format!("{} <{}>", name, email),
    ///     "JoXX Xoe <johX.XXX@XXXXa.it>"
    /// );
    /// ```
    pub fn and_redact(&self, data: &mut String) -> &Self {
        *data = self.redact(core::mem::take(data));
        self
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
            partial: self.partial,
            redact_char: '*',
            fixed: 0,
        };

        if let Some(char) = self.redact_char {
            flags.redact_char = char;
        }

        Ok(Redactor(flags))
    }
}