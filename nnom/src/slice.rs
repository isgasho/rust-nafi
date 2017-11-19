//! Types for working with slices while retaining position information

use std::ops;

mod protected {
    use std::ops;

    /// A trait to allow use of `&str` or `&[T]` generically
    pub trait Slice
        : ops::Index<ops::RangeFrom<usize>, Output = Self>
        + ops::Index<ops::RangeTo<usize>, Output = Self> {
        /// Is the slice empty (of length 0)?
        fn is_empty(&self) -> bool;
    }

    impl Slice for str {
        fn is_empty(&self) -> bool { self.is_empty() }
    }

    impl<T> Slice for [T] {
        fn is_empty(&self) -> bool { self.is_empty() }
    }
}
use self::protected::Slice;

/// A slice with additional information of where it starts in the source buffer.
///
/// This acts like a `&T` in that it derefs to `T`; this means that methods on `T` are available.
#[derive(Debug, Eq, PartialEq)]
pub struct PositionedIndex<'a, T: Slice + 'a + ?Sized> {
    slice: &'a T,
    start: usize,
}

// These impls should apply even when T is not Copy
impl<'a, T: Slice + 'a + ?Sized> Copy for PositionedIndex<'a, T> {}
impl<'a, T: Slice + 'a + ?Sized> Clone for PositionedIndex<'a, T> {
    fn clone(&self) -> Self { *self }
}

impl<'a, T: Slice + 'a + ?Sized> ops::Deref for PositionedIndex<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target { self.raw_slice() }
}

/// A string slice with associated start position in the original source slice.
pub type PositionedStr<'a> = PositionedIndex<'a, str>;

impl<'a> From<&'a str> for PositionedStr<'a> {
    fn from(slice: &'a str) -> Self { PositionedStr { slice, start: 0 } }
}

/// An array slice with associated start position in the original source slice.
pub type PositionedSlice<'a, T> = PositionedIndex<'a, [T]>;

impl<'a, T> From<&'a [T]> for PositionedSlice<'a, T> {
    fn from(slice: &'a [T]) -> Self { PositionedSlice { slice, start: 0 } }
}

impl<'a, T: Slice + 'a + ?Sized> PositionedIndex<'a, T> {
    /// Create a new positioned slice from a raw slice and start position
    pub fn new(slice: &'a T, start: usize) -> Self { PositionedIndex { slice, start } }

    /// The starting index of this slice.
    pub fn start(&self) -> usize { self.start }

    /// The raw slice this positioned slice describes.
    pub fn raw_slice(&self) -> &T { self.slice }

    /// Split the slice into two parts around a given index.
    ///
    /// Note that the index used in this fn starts from 0 like in an unwrapped slice.
    /// This means that `positioned.slice(positioned.start())` _does not mean anything_.
    ///
    /// The first slice is before the index (exclusive) (`&slice[..mid]`).
    /// The second slice is after the index (inclusive) (`&slice[mid..]`).
    pub fn split_at(&self, mid: usize) -> (Self, Self) {
        (
            PositionedIndex {
                slice: &self.slice[..mid],
                start: self.start,
            },
            PositionedIndex {
                slice: &self.slice[mid..],
                start: self.start + mid,
            },
        )
    }
}
