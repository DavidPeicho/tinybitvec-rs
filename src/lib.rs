macro_rules! bit_get {
    ($storage:expr, $index:expr) => {{
        let block = $crate::align_index($index);
        let bit = $crate::bit_index($index);
        $storage[block] & (1 << bit) != 0
    }};
}

macro_rules! bit_set {
    ($storage:expr, $index:expr) => {{
        let block = $crate::align_index($index);
        let bit = $crate::bit_index($index);
        $storage[block] = $storage[block] | (1 << bit);
    }};
}

macro_rules! bit_unset {
    ($storage:expr, $index:expr) => {{
        let block = $crate::align_index($index);
        let bit = $crate::bit_index($index);
        $storage[block] = $storage[block] & !(1 << bit);
    }};
}

mod slice;

pub use slice::{Slice, SliceMut};

#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[derive(Debug, Default)]
pub struct BitVec {
    storage: Vec<u32>,
    len: usize,
}

const SIZE_IN_BYTES: usize = std::mem::size_of::<u32>();
const SIZE_IN_BITS: usize = 8 * SIZE_IN_BYTES;

#[inline]
fn align_index(bit_index: usize) -> usize {
    bit_index / SIZE_IN_BITS
}
#[inline]
fn bit_index(index: usize) -> usize {
    index % SIZE_IN_BITS
}
#[inline]
fn align_count(bit_index: usize) -> usize {
    bit_index.div_ceil(SIZE_IN_BITS)
}

impl BitVec {
    pub const ELEMENTS_PER_WORD: usize = SIZE_IN_BITS;

    pub fn new(size: usize) -> Self {
        let mut bits = Self::default();
        bits.grow(size);
        bits
    }

    pub fn new_with_value(size: usize, value: bool) -> Self {
        let value = if value { u32::MAX } else { 0 };
        let len = align_count(size);

        let mut bits = Self::default();
        bits.storage.resize(len, value);
        bits.len += size;
        bits
    }

    pub fn as_slice(&self) -> Slice<'_> {
        Slice::new(&self.storage, self.len)
    }

    pub fn as_mut_slice(&mut self) -> SliceMut<'_> {
        SliceMut::new(&mut self.storage, &mut self.len)
    }

    pub fn unset_all(&mut self) {
        self.as_mut_slice().unset_all();
    }

    pub fn set_all(&mut self) {
        self.as_mut_slice().set_all();
    }

    pub fn from_bools(booleans: &[bool]) -> Self {
        if booleans.len() == 0 {
            return Self::default();
        }
        let mut bits = BitVec::new(booleans.len());
        for i in 0..booleans.len() {
            bits.set_value(i, booleans[i]);
        }
        bits
    }

    pub fn push(&mut self, value: bool) {
        if self.capacity() == 0 {
            self.storage.resize(self.storage.len() + 1, 0);
        }
        self.set_value(self.len, value);
        self.len += 1;
    }

    pub fn set_range(&mut self, range: std::ops::Range<usize>) {
        if range.end > self.len {
            self.grow(range.end - self.len);
        }
        self.as_mut_slice().set_range(range);
    }

    pub fn grow(&mut self, extra_capacity: usize) {
        let len = align_count(extra_capacity);
        self.storage.resize(self.storage.len() + len, 0);
        self.len += extra_capacity;
    }

    pub fn drain(&mut self, range: std::ops::Range<usize>) {
        self.as_mut_slice().drain(range);
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
        if index < self.len {
            Some(self.get_unsafe(index))
        } else {
            None
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn words(&self) -> &[u32] {
        &self.storage
    }

    #[inline]
    pub(crate) fn capacity(&self) -> usize {
        let bits = self.storage.len() * SIZE_IN_BITS;
        assert!(bits >= self.len);
        bits - self.len
    }
}

#[cfg(test)]
mod tests {
    use crate::BitVec;

    #[test]
    fn new() {
        let bits = BitVec::new(33);
        assert_eq!(bits.len(), 33);
        assert_eq!(bits.words().len(), 2);
    }

    #[test]
    fn new_with_value() {
        let ones = BitVec::new_with_value(33, true);
        assert_eq!(ones.len(), 33);
        assert_eq!(ones.words().len(), 2);
        for i in 0..ones.len() {
            assert_eq!(ones.get(i), Some(true), "bit {}", i);
        }

        let zeros = BitVec::new_with_value(33, false);
        assert_eq!(zeros.len(), 33);
        assert_eq!(zeros.words().len(), 2);
        for i in 0..zeros.len() {
            assert_eq!(zeros.get(i), Some(false), "bit {}", i);
        }
    }

    #[test]
    fn unset_all() {
        let mut bits = BitVec::new_with_value(40, true);
        bits.unset_all();
        for i in 0..bits.len() {
            assert_eq!(bits.get(i), Some(false), "bit {}", i);
        }
    }

    #[test]
    fn set_all() {
        let mut bits = BitVec::new_with_value(40, false);

        bits.set_all();
        for i in 0..bits.len() {
            assert_eq!(bits.get(i), Some(true), "bit {}", i);
        }
    }

    #[test]
    fn push() {
        let mut bits = BitVec::default();
        bits.push(true);
        bits.push(false);
        bits.push(true);

        let mut expected = vec![true, false, true];

        assert_eq!(bits.len(), 3);
        assert_eq!(bits.get(0), Some(expected[0]));
        assert_eq!(bits.get(1), Some(expected[1]));
        assert_eq!(bits.get(2), Some(expected[2]));
        assert_eq!(bits.get(3), None);

        // Push beyond first word
        for i in 0..64 {
            let value = i % 2 == 0;
            expected.push(value);
            bits.push(value);
        }

        // TODO: collect
    }

    #[test]
    fn set_range() {
        let mut bits = BitVec::new(8);
        bits.set_range(2..6);

        for i in 0..2 {
            assert_eq!(bits.get(i), Some(false), "bit {}", i);
        }
        for i in 2..6 {
            assert_eq!(bits.get(i), Some(true), "bit {}", i);
        }
        for i in 6..8 {
            assert_eq!(bits.get(i), Some(false), "bit {}", i);
        }
    }

    #[test]
    fn grow() {
        let mut bits = BitVec::from_bools(&[true, false]);

        bits.grow(31);

        assert_eq!(bits.len(), 33);
        assert_eq!(bits.words().len(), 2);
        assert_eq!(bits.get(0), Some(true));
        assert_eq!(bits.get(1), Some(false));
        for i in 2..bits.len() {
            assert_eq!(bits.get(i), Some(false), "bit {}", i);
        }
    }

    #[test]
    fn unset() {
        let mut bits = BitVec::new_with_value(4, true);

        bits.unset(2);

        assert_eq!(bits.get(0), Some(true));
        assert_eq!(bits.get(1), Some(true));
        assert_eq!(bits.get(2), Some(false));
        assert_eq!(bits.get(3), Some(true));
    }

    #[test]
    fn capacity() {
        let bits = BitVec::new(33);
        assert_eq!(bits.capacity(), 31);
    }

    #[test]
    fn get_set() {
        // Less than a word
        let booleans = [true, false, false, true];
        let bits = BitVec::from_bools(&booleans);
        assert_eq!(bits.len(), 4);
        for i in 0..booleans.len() {
            assert_eq!(bits.get(i), Some(booleans[i]));
        }

        // Exactly a word
        let mut booleans = Vec::new();
        booleans.resize(32, false);
        booleans[1] = true;
        booleans[29] = true;
        booleans[31] = true;
        let bits = BitVec::from_bools(&booleans);
        assert_eq!(bits.len(), 32);
        for i in 0..booleans.len() {
            assert_eq!(bits.get(i), Some(booleans[i]), "bit {}", i);
        }

        // Multi-words
        let mut booleans = Vec::new();
        booleans.resize(33, false);
        booleans[0] = true;
        booleans[29] = true;
        booleans[32] = true;
        let bits = BitVec::from_bools(&booleans);
        assert_eq!(bits.len(), 33);
        for i in 0..booleans.len() {
            assert_eq!(bits.get(i), Some(booleans[i]));
        }
    }

    #[test]
    fn drain() {
        let mut expected = Vec::new();
        expected.resize(33, false);
        expected[0] = true;
        expected[4] = true;
        expected[29] = true;
        expected[32] = true;

        let mut bits = BitVec::from_bools(&expected);

        // Drain by the start
        let range = 0..5;
        bits.drain(range.clone());
        expected.drain(range);
        assert_eq!(bits.len(), expected.len());
        for i in 0..expected.len() {
            assert_eq!(bits.get(i), Some(expected[i]));
        }

        // Drain middle
        let range = 11..17;
        expected[13] = true;
        bits.set(13);
        bits.drain(range.clone());
        expected.drain(range);
        assert_eq!(bits.len(), expected.len());
        for i in 0..expected.len() {
            assert_eq!(bits.get(i), Some(expected[i]));
        }

        // Drain by the end
        let range = (bits.len() - 5)..bits.len();
        bits.drain(range.clone());
        expected.drain(range);
        assert_eq!(bits.len(), expected.len());
        for i in 0..expected.len() {
            assert_eq!(bits.get(i), Some(expected[i]));
        }

        // Drain all
        let range = 0..bits.len();
        bits.drain(range.clone());
        expected.drain(range);
    }
}
