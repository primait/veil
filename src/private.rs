use std::fmt::{Debug, Write};

mod mask_buffer {
    //! We use a thread-local buffer to avoid frequent reallocations.
    //!
    //! AFAIK [`std::fmt::Formatter`] and friends have an internal buffer too, but it's not exposed, so we can't use it.

    use std::{cell::{RefCell, Ref}, ops::Deref, rc::Rc};

    /// The maximum size of the buffer's extra capacity.
    ///
    /// The buffer can grow larger than this value, but once used, it will be shrunk back to this size.
    const MAX_BUFFER_SIZE: usize = 1024;

    thread_local! {
        static MASK_BUFFER: Rc<RefCell<MaskBuffers>> = Rc::new(RefCell::new(MaskBuffers {
            debug_buffer: String::with_capacity(MAX_BUFFER_SIZE),
            masked_buffer: String::with_capacity(MAX_BUFFER_SIZE),
        }));
    }

    #[derive(Clone)]
    pub struct MaskBuffers {
        pub debug_buffer: String,
        pub masked_buffer: String,
    }
    impl MaskBuffers {
        fn clear(&mut self) {
            self.debug_buffer.clear();
            self.debug_buffer.shrink_to(MAX_BUFFER_SIZE);
            self.masked_buffer.clear();
            self.masked_buffer.shrink_to(MAX_BUFFER_SIZE);
        }
    }

    pub struct MaskBufferRef(Rc<RefCell<MaskBuffers>>);
    impl MaskBufferRef {
        // HACK! This allows us to put a MaskBufferRef OR &str in place of a &str.
        pub fn as_ref(&self) -> Ref<'_, str> {
            Ref::map(self.borrow(), |inner| inner.masked_buffer.as_str())
        }
    }
    impl Deref for MaskBufferRef {
        type Target = RefCell<MaskBuffers>;

        #[inline(always)]
        fn deref(&self) -> &Self::Target {
            self.0.deref()
        }
    }
    impl Drop for MaskBufferRef {
        fn drop(&mut self) {
            if let Some(cell) = Rc::get_mut(&mut self.0) {
                cell.get_mut().clear();
            } else {
                // It'll get cleared by the other holder
            }
        }
    }
    impl std::fmt::Debug for MaskBufferRef {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.borrow().masked_buffer.as_str())
        }
    }

    #[inline]
    pub fn mask_buffer<F>(op: F) -> Result<MaskBufferRef, std::fmt::Error>
    where
        F: FnOnce(&mut MaskBuffers) -> Result<(), std::fmt::Error>,
    {
        let mut bufs = MaskBufferRef(MASK_BUFFER.with(|bufs| bufs.clone()));
        {
            let bufs = Rc::make_mut(&mut bufs.0).get_mut();
            bufs.clear();
            op(bufs)?;
        }
        Ok(bufs)
    }
}
use mask_buffer::{mask_buffer, MaskBufferRef, MaskBuffers};

pub struct MaskFlags {
    /// Sourced from [`std::fmt::Formatter::alternate`]
    pub debug_alternate: bool,

    /// Whether the type we're masking is an Option<T> or not. Poor man's specialization! This is detected
    /// by the proc macro reading the path to the type, so it's not perfect.
    ///
    /// This could be improved & rid of in a number of different ways in the future:
    ///
    /// * Once specialization is stabilized, we can use a trait to override masking behaviour for some types,
    /// one of which would be Option<T>.
    ///
    /// * Once std::ptr::metadata and friends are stabilized, we could use it to unsafely cast the dyn Debug pointer
    /// to a concrete Option<T> and mask it directly. Probably not the best idea.
    ///
    /// * Once trait upcasting is stabilized, we could use it to upcast the dyn Debug pointer to a dyn Any and then
    /// downcast it to a concrete Option<T> and mask it directly.
    pub is_option: bool,

    /// Whether to only partially mask the data.
    ///
    /// Incompatible with `fixed`.
    pub partial: bool,

    /// What character to use for masking.
    pub mask_char: char,

    /// Whether to mask with a fixed width, ignoring the length of the data.
    ///
    /// Incompatible with `partial`.
    pub fixed: u8,
}
impl MaskFlags {
    /// How many characters must a word be for it to be partially masked?
    ///
    /// Words smaller than this many characters (NOT bytes) will be fully masked.
    const MIN_PARTIAL_CHARS: usize = 5;

    /// Maximum number of characters to expose at the beginning and end of a partial mask.
    const MAX_PARTIAL_EXPOSE: usize = 3;

    fn mask_partial(&self, str: &str, masked_buffer: &mut String) {
        let count = str.chars().filter(|char| char.is_alphanumeric()).count();
        if count < Self::MIN_PARTIAL_CHARS {
            for char in str.chars() {
                if char.is_alphanumeric() {
                    masked_buffer.push(self.mask_char);
                } else {
                    masked_buffer.push(char);
                }
            }
        } else {
            // The number of characters (prefix and suffix) we'll EXPOSE (NOT mask over)
            let mask_count = (count / 3).min(Self::MAX_PARTIAL_EXPOSE);

            let mut prefix_gas = mask_count;
            let mut middle_gas = count - mask_count - mask_count;
            for char in str.chars() {
                if char.is_alphanumeric() {
                    if prefix_gas > 0 {
                        prefix_gas -= 1;
                        masked_buffer.push(char);
                    } else if middle_gas > 0 {
                        middle_gas -= 1;
                        masked_buffer.push(self.mask_char);
                    } else {
                        masked_buffer.push(char);
                    }
                } else {
                    masked_buffer.push(char);
                }
            }
        }
    }

    fn mask_full(&self, str: &str, masked_buffer: &mut String) {
        for char in str.chars() {
            if char.is_whitespace() || !char.is_alphanumeric() {
                masked_buffer.push(char);
            } else {
                masked_buffer.push(self.mask_char);
            }
        }
    }

    fn mask_fixed(&self, width: usize, masked_buffer: &mut String) {
        masked_buffer.reserve_exact(width);
        for _ in 0..width {
            masked_buffer.push(self.mask_char);
        }
    }
}

pub fn mask(this: &dyn Debug, mask: MaskFlags) -> Result<MaskBufferRef, std::fmt::Error> {
    mask_buffer(|buffers| {
        let MaskBuffers {
            debug_buffer,
            masked_buffer,
        } = &mut *buffers;

        if mask.fixed > 0 {
            mask.mask_fixed(mask.fixed as usize, masked_buffer);
            return Ok(());
        }

        if mask.debug_alternate {
            write!(debug_buffer, "{:#?}", this)?;
        } else {
            write!(debug_buffer, "{:?}", this)?;
        }

        // Specialize for Option<T>
        if mask.is_option {
            if debug_buffer == "None" {
                // We don't need to do any masking
                // https://prima.slack.com/archives/C03URH9N43U/p1661423554871499
            } else if let Some(inner) = debug_buffer
                .strip_prefix("Some(")
                .and_then(|inner| inner.strip_suffix(')'))
            {
                masked_buffer.push_str("Some(");
                mask.mask_partial(inner, masked_buffer);
                masked_buffer.push(')');
            } else {
                // This should never happen, but just in case...
                mask.mask_full(debug_buffer, masked_buffer);
            }
            return Ok(());
        }

        if mask.partial {
            mask.mask_partial(debug_buffer, masked_buffer);
        } else {
            mask.mask_full(debug_buffer, masked_buffer);
        }

        Ok(())
    })
}
