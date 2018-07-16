use bytecount::{count, num_chars};
use memchr::memrchr;
use serde::ser::{Serialize, Serializer};
use std::ops::{Bound, RangeBounds};
use std::{fmt, slice, str, u32};

/// A span of source code.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Span<'a> {
    /// The starting byte of the complete source.
    source: &'a u8,
    /// The inclusive start byte index for this slice.
    start_byte: u32,
    /// The exclusive end byte index for this slice.
    end_byte: u32,
}

impl<'a> fmt::Debug for Span<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Span")
            .field("source", &self.source())
            .field("start_byte", &self.start_byte)
            .field("end_byte", &self.end_byte)
            .finish()
    }
}

/// Constructors
impl<'a> Span<'a> {
    /// Create a span that slices into the source string.
    pub fn from_source(source: &'a str, slice: impl RangeBounds<u32>) -> Self {
        if source.len() > u32::MAX as usize {
            panic!("Span source string is too big");
        }

        let start_byte = match slice.start_bound() {
            Bound::Unbounded => 0u32,
            Bound::Included(&idx) => idx,
            Bound::Excluded(&idx) => idx + 1,
        };
        let end_byte = match slice.end_bound() {
            Bound::Unbounded => source.len() as u32,
            Bound::Included(&idx) => idx + 1,
            Bound::Excluded(&idx) => idx,
        };
        Span {
            source: unsafe { &*source.as_ptr() },
            start_byte,
            end_byte,
        }
    }

    /// Create a span of a slice given the offset from the beginning of the source.
    ///
    /// # Safety
    ///
    /// The allocated string must be at least `offset` bytes before the slice.
    ///
    /// In debug mode this check is approximated via a UTF-8 well-formedness check.
    pub(crate) unsafe fn from_slice(slice: &'a str, offset: u32) -> Self {
        let source = {
            let ptr = slice.as_ptr().offset(offset as isize);
            let bytes = slice::from_raw_parts(ptr, slice.len() + offset as usize);
            if cfg!(debug_assertions) {
                str::from_utf8(bytes).expect("Invalid string")
            } else {
                str::from_utf8_unchecked(bytes)
            }
        };
        Self::from_source(source, offset..)
    }

    /// Create a span from a `pest::Span`.
    #[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
    pub(crate) fn from_pest(span: ::pest::Span<'a>) -> Self {
        unsafe { Self::from_slice(span.as_str(), span.start() as u32) }
    }
}

/// Accessors
impl<'a> Span<'a> {
    fn source(&self) -> &'a str {
        unsafe {
            str::from_utf8_unchecked(slice::from_raw_parts(self.source, self.end_byte as usize))
        }
    }

    /// Get the slice of source that this span covers.
    #[inline]
    pub fn as_str(&self) -> &'a str {
        &self.source()[self.start_byte as usize..]
    }

    /// The byte offset of the beginning of this span.
    #[inline]
    pub fn start_byte(&self) -> u32 {
        self.start_byte
    }

    /// The one-indexed row that this span starts on.
    ///
    /// This should line up with the behavior of most editors.
    #[inline]
    pub fn start_row(&self) -> u32 {
        count(self.source()[..self.start_byte as usize].as_bytes(), b'\n') as u32 + 1
    }

    /// The one-indexed column that this span starts on.
    ///
    /// This should line up with the behavior of most editors.
    #[inline]
    pub fn start_col(&self) -> u32 {
        let row_idx =
            memrchr(b'\n', self.source()[..self.start_byte as usize].as_bytes()).unwrap_or(0);
        num_chars(self.source()[row_idx..self.start_byte as usize].as_bytes()) as u32
    }

    /// The byte offset of the end of this span.
    #[inline]
    pub fn end_byte(&self) -> u32 {
        self.end_byte
    }

    /// The one-indexed row that this span starts on.
    ///
    /// This should line up with the behavior of most editors.
    #[inline]
    pub fn end_row(&self) -> u32 {
        count(self.source()[..self.end_byte as usize].as_bytes(), b'\n') as u32 + 1
    }

    /// The one-indexed column that this span starts on.
    ///
    /// This should line up with the behavior of most editors.
    #[inline]
    pub fn end_col(&self) -> u32 {
        let row_idx =
            memrchr(b'\n', self.source()[..self.end_byte as usize].as_bytes()).unwrap_or(0);
        num_chars(self.source()[row_idx..self.end_byte as usize].as_bytes()) as u32 + 1
    }
}

impl<'a> Serialize for Span<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (self.start_byte, self.end_byte).serialize(serializer)
    }
}
