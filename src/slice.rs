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

    pub fn drain(&mut self, range: std::ops::Range<usize>) {
        for src in range.end..*self.len {
            let dst = range.start + (src - range.end);
            let value = self.get_unsafe(src);
            self.set_value(dst, value);
        }
        *self.len -= range.len();
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
}
