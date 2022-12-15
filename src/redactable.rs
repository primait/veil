/// Types that are sensitive data or PII (Personally Identifiable Information) and can be redact-formatted.
///
/// The type must have an implementation of [`Display`](std::fmt::Display) so that it can also be formatted in plain text without redaction.
///
/// This trait can be derived using the [`Redactable`](derive.Redactable.html) macro.
pub trait Redactable: std::fmt::Display {
    /// Returns this value formatted as a string with all PII/sensitive data redacted.
    fn redact(&self) -> String {
        let mut buffer = String::new();

        self.redact_into(&mut buffer)
            .expect("writing to a String should never fail");

        buffer
    }

    /// Writes this value formatted as a string with all PII/sensitive data redacted into the given buffer.
    fn redact_into<W: std::fmt::Write>(&self, buffer: &mut W) -> std::fmt::Result;
}
