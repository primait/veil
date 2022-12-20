/// Types that are sensitive data or PII (Personally Identifiable Information) and can be redact-formatted.
//
/// This trait can be manually implemented or derived using the [`Redactable`](derive.Redactable.html) macro.
pub trait Redactable {
    /// Returns this value formatted as a string with all PII/sensitive data redacted.
    fn redact(&self) -> String {
        let mut buffer = String::new();

        self.redact_into(&mut buffer)
            .expect("writing to a String should never fail");

        buffer
    }

    /// Writes this value formatted as a string with all PII/sensitive data redacted into the given buffer.
    fn redact_into(&self, buffer: &mut dyn std::fmt::Write) -> std::fmt::Result;
}
