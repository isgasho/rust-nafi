//! Types for working with slices while retaining position information

use std::ops;

/// A slice with additional information of where it starts in the source buffer.
///
/// This acts like a `&T` in that it derefs to `T`; this means that methods on `T` are available.
/// More importantly, it means that indexing is possible, but it goes from 0 as do all other slices,
/// rather than the offset start recorded by this type.
#[derive(Debug, Eq, PartialEq)]
pub struct PositionedIndex<'a, T: 'a + ?Sized>
where
    T: ops::Index<ops::RangeFrom<usize>>,
    T: ops::Index<ops::RangeTo<usize>>,
{
    slice: &'a T,
    start: usize,
}

impl<'a, T: 'a + ?Sized> Copy for PositionedIndex<'a, T>
where
    T: ops::Index<ops::RangeFrom<usize>>,
    T: ops::Index<ops::RangeTo<usize>>,
{
}

impl<'a, T: 'a + ?Sized> Clone for PositionedIndex<'a, T>
where
    T: ops::Index<ops::RangeFrom<usize>>,
    T: ops::Index<ops::RangeTo<usize>>,
{
    fn clone(&self) -> Self { *self }
}

impl<'a, T: ?Sized> ops::Deref for PositionedIndex<'a, T>
where
    T: ops::Index<ops::RangeFrom<usize>>,
    T: ops::Index<ops::RangeTo<usize>>,
{
    type Target = T;

    fn deref(&self) -> &Self::Target { self.slice }
}

/// A string slice with associated start position in the original source slice.
pub type PositionedStr<'a> = PositionedIndex<'a, str>;

/// An array slice with associated start position in the original source slice.
pub type PositionedSlice<'a, T> = PositionedIndex<'a, [T]>;

impl<'a, T: 'a + ?Sized> PositionedIndex<'a, T>
where
    T: ops::Index<ops::RangeFrom<usize>, Output = T>,
    T: ops::Index<ops::RangeTo<usize>, Output = T>,
{
    /// Create a new positioned slice from a raw slice and start position
    pub fn new(slice: &'a T, start: usize) -> Self { PositionedIndex { slice, start } }

    /// The starting index of this slice.
    pub fn start(&self) -> usize { self.start }

    /// Split the slice into two parts around a given index.
    ///
    /// Note that the index used in this fn starts from 0 like in an unwrapped slice.
    /// This means that `positioned.slice(positioned.start())` _does not mean anything_.
    ///
    /// The first slice is before the index (exclusive) (`&slice[..idx]`).
    /// The second slice is after the index (inclusive) (`&slice[idx..]`).
    pub fn split_at(&self, idx: usize) -> (Self, Self) {
        (
            PositionedIndex {
                slice: &self.slice[..idx],
                start: self.start,
            },
            PositionedIndex {
                slice: &self.slice[idx..],
                start: self.start + idx,
            },
        )
    }
}

impl<'a> From<&'a str> for PositionedStr<'a> {
    fn from(slice: &'a str) -> Self { PositionedStr { slice, start: 0 } }
}

impl<'a, T> From<&'a [T]> for PositionedSlice<'a, T> {
    fn from(slice: &'a [T]) -> Self { PositionedSlice { slice, start: 0 } }
}
