use crate::{SIZE_IN_BITS, bit_index, slice::storage_range};
use std::ops::Range;

#[derive(Debug)]
/// A mutable view over a range of bits.
pub struct SliceMut<'a> {
    pub(crate) storage: &'a mut [u32],
    pub(crate) range: std::ops::Range<usize>,
}

impl<'a> SliceMut<'a> {
    /// Sets every bit in `range`.
    ///
    /// Panics if `range` reaches outside allocated storage.
    pub fn set_range(&mut self, range: std::ops::Range<usize>) {
        self.set_range_to(range, true);
    }

    /// Clears all bits in this slice.
    #[inline]
    pub fn unset_all(&mut self) {
        self.set_range_to(0..self.len(), false);
    }

    /// Sets all bits in this slice.
    #[inline]
    pub fn set_all(&mut self) {
        self.set_range_to(0..self.len(), true);
    }

    /// Sets `index` to `value`.
    ///
    /// Panics if `index` reaches outside allocated storage.
    pub fn set_value(&mut self, index: usize, value: bool) {
        let index = self.relative_index(index);
        if value {
            bit_set!(self.storage, index);
        } else {
            bit_unset!(self.storage, index);
        }
    }

    /// Sets `index` to `true`.
    ///
    /// Panics if `index` reaches outside allocated storage.
    #[inline]
    pub fn set(&mut self, index: usize) {
        let index = self.relative_index(index);
        bit_set!(self.storage, index);
    }

    /// Sets `index` to `false`.
    ///
    /// Panics if `index` reaches outside allocated storage.
    #[inline]
    pub fn unset(&mut self, index: usize) {
        let index = self.relative_index(index);
        bit_unset!(self.storage, index);
    }

    /// Returns a mutable sub-slice of this slice.
    ///
    /// Panics if `range` is out of bounds.
    pub fn slice(self, range: Range<usize>) -> Self {
        let start = self.range.start + range.start;
        let end = self.range.start + range.end;
        Self {
            storage: &mut self.storage[storage_range(start..end)],
            range: bit_index(start)..(bit_index(start) + range.len()),
        }
    }

    #[inline]
    fn relative_index(&self, index: usize) -> usize {
        self.range.start + index
    }

    fn set_range_to(&mut self, range: Range<usize>, value: bool) {
        if range.is_empty() {
            return;
        }

        let end = range.end;
        let mut index = range.start;
        while index < end && bit_index(self.relative_index(index)) != 0 {
            self.set_value(index, value);
            index += 1;
        }

        while index + SIZE_IN_BITS <= end {
            let block = self.relative_index(index) / SIZE_IN_BITS;
            self.storage[block] = if value { u32::MAX } else { 0 };
            index += SIZE_IN_BITS;
        }

        for i in index..end {
            self.set_value(i, value);
        }
    }
}

impl_slice!(SliceMut<'a>);
impl_index!(SliceMut<'_>);
