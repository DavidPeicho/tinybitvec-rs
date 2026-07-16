use crate::Iter;

#[derive(Debug, Clone, Copy)]
pub struct Slice<'a> {
    storage: &'a [u32],
    len: usize,
}

#[derive(Debug)]
pub struct SliceMut<'a> {
    storage: &'a mut [u32],
    len: &'a mut usize,
}

impl<'a> Slice<'a> {
    pub(crate) fn new(storage: &'a [u32], len: usize) -> Self {
        Self { storage, len }
    }

    #[inline]
    pub fn get_unsafe(&self, index: usize) -> bool {
        bit_get!(self.storage, index)
    }
    #[inline]
    pub fn get(&self, index: usize) -> Option<bool> {
        if index < self.len {
            Some(self.get_unsafe(index))
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn iter(&self) -> Iter<'a> {
        Iter::new(*self)
    }
}

impl<'a> SliceMut<'a> {
    pub(crate) fn new(storage: &'a mut [u32], len: &'a mut usize) -> Self {
        Self { storage, len }
    }

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
        if value {
            bit_set!(self.storage, index);
        } else {
            bit_unset!(self.storage, index);
        }
    }

    #[inline]
    pub fn set(&mut self, index: usize) {
        bit_set!(self.storage, index);
    }

    #[inline]
    pub fn unset(&mut self, index: usize) {
        bit_unset!(self.storage, index);
    }

    #[inline]
    pub fn get_unsafe(&self, index: usize) -> bool {
        bit_get!(self.storage, index)
    }

    #[inline]
    pub fn get(&self, index: usize) -> Option<bool> {
        if index < *self.len {
            Some(self.get_unsafe(index))
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        *self.len
    }

    pub fn iter(&self) -> Iter<'_> {
        Iter::new(Slice::new(self.storage, *self.len))
    }
}

impl_index!(Slice<'_>, |slice: &Slice<'_>| slice.len);
impl_index!(SliceMut<'_>, |slice: &SliceMut<'_>| *slice.len);
