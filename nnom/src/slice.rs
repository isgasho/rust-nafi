use std::ops;

/// A slice with additional information of where it starts in the source buffer.
#[derive(Clone, Copy, Debug)]
pub struct PositionedIndex<'a, T: 'a>
where
    T: ops::Index<ops::RangeFrom<usize>>,
    T: ops::Index<ops::Range<usize>>,
    T: ops::Index<ops::RangeTo<usize>>,
    T: ops::Index<ops::RangeFull>,
    T: ?Sized,
{
    slice: &'a T,
    start: usize,
}

/// A string slice with associated start position in the original source slice.
pub type PositionedStr<'a> = PositionedIndex<'a, str>;

/// An array slice with associated start position in the original source slice.
pub type PositionedSlice<'a, T> = PositionedIndex<'a, [T]>;

impl<'a, T> PositionedIndex<'a, T>
where
    T: ops::Index<ops::RangeFrom<usize>, Output = T>,
    T: ops::Index<ops::Range<usize>, Output = T>,
    T: ops::Index<ops::RangeTo<usize>, Output = T>,
    T: ops::Index<ops::RangeFull, Output = T>,
{
    /// The wrapped slice.
    pub fn raw_slice(&self) -> &T { self.slice }

    /// The starting index of this slice.
    pub fn start(&self) -> usize { self.start }

    /// Split the slice into two parts around a given index.
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
