use bytecount::{count, num_chars};
use memchr::memrchr;
use serde::ser::{Serialize, Serializer};
use std::{
    fmt,
    ops::{Bound, RangeBounds},
    slice, str, u32,
};

pub use span_derive::Spanned;

/// A span of source code.
///
/// A span represents a region between two positions in the code.
/// This implementation lazily calculates the row/column position,
/// so if you need to access it frequently, consider caching it.
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

        if end_byte as usize > source.len() {
            panic!("Span slice extends past end of source");
        }

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
    /// The backing string must start at least `offset` bytes before the slice,
    /// and said byte boundary must be a character boundary.
    ///
    /// In debug mode this check is approximated via a UTF-8 well-formedness check.
    pub unsafe fn from_slice(slice: &'a str, offset: u32) -> Self {
        let source = {
            debug_assert_eq!(offset as isize as u32, offset);
            let ptr = slice.as_ptr().offset(-(offset as isize));
            let bytes = slice::from_raw_parts(ptr, slice.len() + offset as usize);
            if cfg!(debug_assertions) {
                str::from_utf8(bytes).expect("Invalid slice")
            } else {
                str::from_utf8_unchecked(bytes)
            }
        };
        Self::from_source(source, offset..)
    }
}

impl<'a> From<::pest::Span<'a>> for Span<'a> {
    fn from(span: ::pest::Span<'a>) -> Self {
        if span.end() <= u32::MAX as usize {
            unsafe { Span::from_slice(span.as_str(), span.start() as u32) }
        } else {
            panic!("too-large `Span` created")
        }
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

/// An object that represents some span of source.
pub trait Spanned {
    /// Get the span that this covers.
    fn span(&self) -> Span<'_>;
}
