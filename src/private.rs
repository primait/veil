use std::fmt::{Debug, Write};

mod redact_buffer {
    //! We use a thread-local buffer to avoid frequent reallocations.
    //!
    //! AFAIK [`std::fmt::Formatter`] and friends have an internal buffer too, but it's not exposed, so we can't use it.

    use std::{
        cell::{Ref, RefCell},
        ops::Deref,
        rc::Rc,
    };

    /// The maximum size of the buffer's extra capacity.
    ///
    /// The buffer can grow larger than this value, but once used, it will be shrunk back to this size.
    const MAX_BUFFER_SIZE: usize = 1024;

    thread_local! {
        static REDACT_BUFFER: Rc<RefCell<RedactBuffers>> = Rc::new(RefCell::new(RedactBuffers {
            debug_buffer: String::with_capacity(MAX_BUFFER_SIZE),
            redacted_buffer: String::with_capacity(MAX_BUFFER_SIZE),
        }));
    }

    #[derive(Clone)]
    pub struct RedactBuffers {
        pub debug_buffer: String,
        pub redacted_buffer: String,
    }
    impl RedactBuffers {
        fn clear(&mut self) {
            self.debug_buffer.clear();
            self.debug_buffer.shrink_to(MAX_BUFFER_SIZE);
            self.redacted_buffer.clear();
            self.redacted_buffer.shrink_to(MAX_BUFFER_SIZE);
        }
    }

    pub struct RedactBufferRef(Rc<RefCell<RedactBuffers>>);
    impl RedactBufferRef {
        // HACK! This allows us to put a RedactBufferRef OR &str in place of a &str.
        pub fn as_ref(&self) -> Ref<'_, str> {
            Ref::map(self.borrow(), |inner| inner.redacted_buffer.as_str())
        }
    }
    impl Deref for RedactBufferRef {
        type Target = RefCell<RedactBuffers>;

        #[inline(always)]
        fn deref(&self) -> &Self::Target {
            self.0.deref()
        }
    }
    impl Drop for RedactBufferRef {
        fn drop(&mut self) {
            if let Some(cell) = Rc::get_mut(&mut self.0) {
                cell.get_mut().clear();
            } else {
                // It'll get cleared by the other holder
            }
        }
    }
    impl std::fmt::Debug for RedactBufferRef {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.borrow().redacted_buffer.as_str())
        }
    }

    #[inline]
    pub fn redact_buffer<F>(op: F) -> Result<RedactBufferRef, std::fmt::Error>
    where
        F: FnOnce(&mut RedactBuffers) -> Result<(), std::fmt::Error>,
    {
        let mut bufs = RedactBufferRef(REDACT_BUFFER.with(|bufs| bufs.clone()));
        {
            let bufs = Rc::make_mut(&mut bufs.0).get_mut();
            bufs.clear();
            op(bufs)?;
        }
        Ok(bufs)
    }
}
use redact_buffer::{redact_buffer, RedactBufferRef, RedactBuffers};

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

    fn redact_partial(&self, str: &str, redacted_buffer: &mut String) {
        let count = str.chars().filter(|char| char.is_alphanumeric()).count();
        if count < Self::MIN_PARTIAL_CHARS {
            for char in str.chars() {
                if char.is_alphanumeric() {
                    redacted_buffer.push(self.redact_char);
                } else {
                    redacted_buffer.push(char);
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
                        redacted_buffer.push(char);
                    } else if middle_gas > 0 {
                        middle_gas -= 1;
                        redacted_buffer.push(self.redact_char);
                    } else {
                        redacted_buffer.push(char);
                    }
                } else {
                    redacted_buffer.push(char);
                }
            }
        }
    }

    fn redact_full(&self, str: &str, redacted_buffer: &mut String) {
        for char in str.chars() {
            if char.is_whitespace() || !char.is_alphanumeric() {
                redacted_buffer.push(char);
            } else {
                redacted_buffer.push(self.redact_char);
            }
        }
    }

    fn redact_fixed(&self, width: usize, redacted_buffer: &mut String) {
        redacted_buffer.reserve_exact(width);
        for _ in 0..width {
            redacted_buffer.push(self.redact_char);
        }
    }
}

pub fn redact(this: &dyn Debug, redact: RedactFlags) -> Result<RedactBufferRef, std::fmt::Error> {
    redact_buffer(|buffers| {
        let RedactBuffers {
            debug_buffer,
            redacted_buffer,
        } = &mut *buffers;

        if redact.fixed > 0 {
            redact.redact_fixed(redact.fixed as usize, redacted_buffer);
            return Ok(());
        }

        if redact.debug_alternate {
            write!(debug_buffer, "{:#?}", this)?;
        } else {
            write!(debug_buffer, "{:?}", this)?;
        }

        // Specialize for Option<T>
        if redact.is_option {
            if debug_buffer == "None" {
                // We don't need to do any redacting
                // https://prima.slack.com/archives/C03URH9N43U/p1661423554871499
            } else if let Some(inner) = debug_buffer
                .strip_prefix("Some(")
                .and_then(|inner| inner.strip_suffix(')'))
            {
                redacted_buffer.push_str("Some(");
                redact.redact_partial(inner, redacted_buffer);
                redacted_buffer.push(')');
            } else {
                // This should never happen, but just in case...
                redact.redact_full(debug_buffer, redacted_buffer);
            }
            return Ok(());
        }

        if redact.partial {
            redact.redact_partial(debug_buffer, redacted_buffer);
        } else {
            redact.redact_full(debug_buffer, redacted_buffer);
        }

        Ok(())
    })
}
