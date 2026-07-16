use crate::Slice;

#[derive(Debug, Clone)]
pub struct Iter<'a> {
    pub(crate) slice: Slice<'a>,
    pub(crate) index: usize,
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
