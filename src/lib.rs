mod iter;
#[macro_use]
mod macros;
mod slice;

pub use iter::Iter;
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
const fn block_index(bit_index: usize) -> usize {
    bit_index / SIZE_IN_BITS
}
#[inline]
const fn bit_index(index: usize) -> usize {
    index % SIZE_IN_BITS
}
#[inline]
const fn align_count(bit_index: usize) -> usize {
    bit_index.div_ceil(SIZE_IN_BITS)
}
#[inline]
const fn storage_range(range: std::ops::Range<usize>) -> std::ops::Range<usize> {
    block_index(range.start)..align_count(range.end)
}

impl BitVec {
    pub fn new(size: usize, value: bool) -> Self {
        let value = if value { u32::MAX } else { 0 };
        let len = align_count(size);

        let mut bits = Self::default();
        bits.storage.resize(len, value);
        bits.len += size;
        bits
    }

    pub fn grow(&mut self, extra_capacity: usize) {
        if self.len > 0 {
            let last_block = block_index(self.len - 1);
            // Clear bits that migh be set when using `set_all`.
            let mask = match bit_index(self.len) {
                0 => u32::MAX,
                bits => (1 << bits) - 1,
            };
            self.storage[last_block] &= mask;
        }
        self.len += extra_capacity;
        self.storage.resize(align_count(self.len), 0);
    }

    pub fn push(&mut self, value: bool) {
        if self.capacity() == 0 {
            self.storage.resize(self.storage.len() + 1, 0);
        }
        self.set_value(self.len, value);
        self.len += 1;
    }

    pub fn drain(&mut self, range: std::ops::Range<usize>) {
        for src in range.end..self.len {
            let dst = range.start + (src - range.end);
            let value = bit_get!(self.storage, src);
            self.set_value(dst, value);
        }
        self.len -= range.len();
    }

    pub fn as_slice(&self) -> Slice {
        Slice {
            storage: &self.storage,
            range: 0..self.len,
        }
    }

    pub fn as_mut_slice(&mut self) -> SliceMut {
        SliceMut {
            storage: &mut self.storage,
            range: 0..self.len,
        }
    }

    pub fn iter(&self) -> Iter<'_> {
        self.as_slice().iter()
    }

    pub fn unset_all(&mut self) {
        self.as_mut_slice().unset_all();
    }

    pub fn set_all(&mut self) {
        self.as_mut_slice().set_all();
    }

    pub fn set_range(&mut self, range: std::ops::Range<usize>) {
        self.as_mut_slice().set_range(range);
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
    pub fn get(&self, index: usize) -> Option<bool> {
        if index < self.len {
            Some(bit_get!(self.storage, index))
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
    fn capacity(&self) -> usize {
        let bits = self.storage.len() * SIZE_IN_BITS;
        assert!(bits >= self.len);
        bits - self.len
    }
}

impl From<&[bool]> for BitVec {
    fn from(booleans: &[bool]) -> Self {
        if booleans.len() == 0 {
            return Self::default();
        }
        let mut bits = BitVec::new(booleans.len(), false);
        for (i, value) in booleans.iter().copied().enumerate() {
            bits.set_value(i, value);
        }
        bits
    }
}

impl_index!(BitVec);

#[cfg(test)]
mod tests {
    use crate::BitVec;

    #[test]
    fn new() {
        let bits = BitVec::new(33, false);
        assert_eq!(bits.len(), 33);
        assert_eq!(bits.words().len(), 2);
    }

    #[test]
    fn unset_all() {
        let mut bits = BitVec::new(40, true);
        bits.unset_all();
        assert_eq!(bits.as_slice().iter().collect::<Vec<_>>(), vec![false; 40]);
    }

    #[test]
    fn set_all() {
        let mut bits = BitVec::new(40, false);

        bits.set_all();
        assert_eq!(bits.as_slice().iter().collect::<Vec<_>>(), vec![true; 40]);
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

        assert_eq!(bits.as_slice().iter().collect::<Vec<_>>(), expected);
    }

    #[test]
    fn set_range() {
        let mut bits = BitVec::new(8, false);
        bits.set_range(2..6);

        assert_eq!(
            bits.as_slice().iter().collect::<Vec<_>>(),
            vec![false, false, true, true, true, true, false, false]
        );
    }

    #[test]
    fn slice_range() {
        let mut booleans = vec![false; 70];
        booleans[29] = true;
        booleans[32] = true;
        booleans[34] = true;

        let bits = BitVec::from(booleans.as_slice());
        let slice = bits.as_slice().slice(30..35);

        assert_eq!(slice.words().len(), 2);
        assert_eq!(
            slice.iter().collect::<Vec<_>>(),
            vec![false, false, true, false, true]
        );
    }

    #[test]
    fn slice_range_can_be_sliced_again() {
        let booleans = [
            false, true, false, true, true, false, true, false, true, false,
        ];
        let bits = BitVec::from(booleans.as_slice());

        let slice = bits.as_slice().slice(1..9).slice(2..7);
        assert_eq!(slice.iter().collect::<Vec<_>>(), booleans[3..8]);
    }

    #[test]
    fn slice_mut_range() {
        let mut bits = BitVec::new(70, false);
        {
            let mut slice = bits.as_mut_slice().slice(30..35);
            slice.set(2);
            slice.set(4);
        }
        assert!(bits[32]);
        assert!(bits[34]);
        assert!(!bits[31]);
        assert!(!bits[33]);
    }

    #[test]
    fn grow() {
        let mut bits = BitVec::from(&[true, false][..]);

        bits.grow(31);

        assert_eq!(bits.len(), 33);
        assert_eq!(bits.words().len(), 2);
        let mut expected = vec![false; 33];
        expected[0] = true;
        assert_eq!(bits.as_slice().iter().collect::<Vec<_>>(), expected);
    }

    #[test]
    fn grow_clears_exposed_padding_bits() {
        let mut bits = BitVec::new(1, true);

        bits.grow(31);

        let mut expected = vec![false; 32];
        expected[0] = true;
        assert_eq!(bits.as_slice().iter().collect::<Vec<_>>(), expected);
    }

    #[test]
    fn unset() {
        let mut bits = BitVec::new(4, true);

        bits.unset(2);

        assert_eq!(bits.get(0), Some(true));
        assert_eq!(bits.get(1), Some(true));
        assert_eq!(bits.get(2), Some(false));
        assert_eq!(bits.get(3), Some(true));
    }

    #[test]
    fn capacity() {
        let mut bits = BitVec::new(33, false);
        assert_eq!(bits.len(), 33);
        assert_eq!(bits.capacity(), 31);

        bits.drain(10..21);
        assert_eq!(bits.len(), 22);
        assert_eq!(bits.capacity(), 42);
    }

    #[test]
    fn index() {
        let mut bits = BitVec::from(&[true, false, true][..]);

        assert!(bits[0]);
        assert!(!bits[1]);
        assert!(bits[2]);

        let slice = bits.as_slice();
        assert!(slice[0]);
        assert!(!slice[1]);
        assert!(slice[2]);

        let mut slice = bits.as_mut_slice();
        slice.set(1);
        assert!(slice[0]);
        assert!(slice[1]);
        assert!(slice[2]);
    }

    #[test]
    fn get_set() {
        // Less than a word
        let booleans = [true, false, false, true];
        let bits = BitVec::from(booleans.as_slice());
        assert_eq!(bits.len(), 4);
        assert_eq!(bits.as_slice().iter().collect::<Vec<_>>(), booleans);

        // Exactly a word
        let mut booleans = Vec::new();
        booleans.resize(32, false);
        booleans[1] = true;
        booleans[29] = true;
        booleans[31] = true;
        let bits = BitVec::from(booleans.as_slice());
        assert_eq!(bits.len(), 32);
        assert_eq!(bits.as_slice().iter().collect::<Vec<_>>(), booleans);

        // Multi-words
        let mut booleans = Vec::new();
        booleans.resize(33, false);
        booleans[0] = true;
        booleans[29] = true;
        booleans[32] = true;
        let bits = BitVec::from(booleans.as_slice());
        assert_eq!(bits.len(), 33);
        assert_eq!(bits.as_slice().iter().collect::<Vec<_>>(), booleans);
    }

    #[test]
    fn drain() {
        let mut expected = Vec::new();
        expected.resize(33, false);
        expected[0] = true;
        expected[4] = true;
        expected[29] = true;
        expected[32] = true;

        let mut bits = BitVec::from(expected.as_slice());

        // Drain by the start
        let range = 0..5;
        bits.drain(range.clone());
        expected.drain(range);
        assert_eq!(bits.len(), expected.len());
        assert_eq!(bits.as_slice().iter().collect::<Vec<_>>(), expected);

        // Drain middle
        let range = 11..17;
        expected[13] = true;
        bits.set(13);
        bits.drain(range.clone());
        expected.drain(range);
        assert_eq!(bits.len(), expected.len());
        assert_eq!(bits.as_slice().iter().collect::<Vec<_>>(), expected);

        // Drain by the end
        let range = (bits.len() - 5)..bits.len();
        bits.drain(range.clone());
        expected.drain(range);
        assert_eq!(bits.len(), expected.len());
        assert_eq!(bits.as_slice().iter().collect::<Vec<_>>(), expected);

        // Drain all
        let range = 0..bits.len();
        bits.drain(range.clone());
        expected.drain(range);
    }
}
