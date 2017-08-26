//! Types for working with slices while retaining position information

use std::ops;

/// A slice with additional information of where it starts in the source buffer.
///
/// This acts like a `&T` in that it derefs to `T`; this means that methods on `T` are available.
/// More importantly, it means that indexing is possible, but it goes from 0 as do all other slices,
/// rather than the offset start recorded by this type.
#[derive(Clone, Copy, Debug)]
pub struct PositionedIndex<'a, T: 'a + ?Sized>
where
    T: ops::Index<ops::RangeFrom<usize>>,
    T: ops::Index<ops::RangeTo<usize>>,
{
    slice: &'a T,
    start: usize,
}

impl<'a, T> ops::Deref for PositionedIndex<'a, T>
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

impl<'a, T> PositionedIndex<'a, T>
where
    T: ops::Index<ops::RangeFrom<usize>, Output = T>,
    T: ops::Index<ops::RangeTo<usize>, Output = T>,
{
    /// The starting index of this slice.
    pub fn start(&self) -> usize { self.start }

    /// Split the slice into two parts around a given index.
    ///
    /// Note that the index used in this fn starts from 0 like in an unwrapped slice.
    /// This means that `positioned.slice(positioned.start())` _does not mean anything_.
    ///
    /// The first slice is before the index (exclusive) (`&slice[..idx]`).
    /// The second slice is after the index (inclusive) (`&slice[idx..]`).
    pub fn split(&'a self, idx: usize) -> (PositionedIndex<'a, T>, PositionedIndex<'a, T>) {
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
