use crate::Slice;

#[derive(Debug, Clone)]
pub struct Iter<'a> {
    slice: Slice<'a>,
    index: usize,
}

impl<'a> Iter<'a> {
    pub(crate) fn new(slice: Slice<'a>) -> Self {
        Self { slice, index: 0 }
    }
}

impl Iterator for Iter<'_> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.slice.get(self.index)?;
        self.index += 1;
        Some(value)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.slice.len() - self.index;
        (len, Some(len))
    }
}

impl ExactSizeIterator for Iter<'_> {}
