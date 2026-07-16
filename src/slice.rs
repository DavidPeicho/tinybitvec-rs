use crate::{bit_index, storage_range, Iter};
use std::ops::Range;

#[derive(Debug, Clone)]
pub struct Slice<'a> {
    pub(crate) storage: &'a [u32],
    pub(crate) range: Range<usize>,
}

impl<'a> Slice<'a> {
    pub fn slice(&self, range: Range<usize>) -> Self {
        let start = self.range.start + range.start;
        let end = self.range.start + range.end;
        Self {
            storage: &self.storage[storage_range(start..end)],
            range: bit_index(start)..(bit_index(start) + range.len()),
        }
    }
    #[inline]
    pub fn get(&self, index: usize) -> Option<bool> {
        if index < self.len() {
            Some(bit_get!(self.storage, self.range.start + index))
        } else {
            None
        }
    }
    #[inline]
    pub fn len(&self) -> usize {
        self.range.len()
    }
    #[inline]
    pub fn words(&self) -> &'a [u32] {
        self.storage
    }
    #[inline]
    pub fn iter(&self) -> Iter<'a> {
        Iter {
            slice: self.slice(0..self.len()),
            index: 0,
        }
    }
}

#[derive(Debug)]
pub struct SliceMut<'a> {
    pub(crate) storage: &'a mut [u32],
    pub(crate) range: std::ops::Range<usize>,
}

impl<'a> SliceMut<'a> {
    pub fn unset_all(&mut self) {
        self.storage.fill(0);
    }

    pub fn set_all(&mut self) {
        self.storage.fill(u32::MAX);
    }

    pub fn set_range(&mut self, range: std::ops::Range<usize>) {
        // TODO: Could be optimized based on alignment
        for i in range {
            self.set(i);
        }
    }

    #[inline]
    pub fn set_value(&mut self, index: usize, value: bool) {
        let index = self.relative_index(index);
        if value {
            bit_set!(self.storage, index);
        } else {
            bit_unset!(self.storage, index);
        }
    }

    #[inline]
    pub fn set(&mut self, index: usize) {
        let index = self.relative_index(index);
        bit_set!(self.storage, index);
    }

    #[inline]
    pub fn unset(&mut self, index: usize) {
        let index = self.relative_index(index);
        bit_unset!(self.storage, index);
    }

    pub fn slice(self, range: Range<usize>) -> Self {
        let start = self.range.start + range.start;
        let end = self.range.start + range.end;
        Self {
            storage: &mut self.storage[storage_range(start..end)],
            range: bit_index(start)..(bit_index(start) + range.len()),
        }
    }

    #[inline]
    pub fn get(&self, index: usize) -> Option<bool> {
        if index >= self.len() {
            return None;
        }
        let index = self.relative_index(index);
        Some(bit_get!(self.storage, index))
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.range.len()
    }

    #[inline]
    fn relative_index(&self, index: usize) -> usize {
        self.range.start + index
    }
}

impl_index!(Slice<'_>);
impl_index!(SliceMut<'_>);
