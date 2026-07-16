macro_rules! bit_get {
    ($storage:expr, $index:expr) => {{
        let block = $crate::block_index($index);
        let bit = $crate::bit_index($index);
        $storage[block] & (1 << bit) != 0
    }};
}

macro_rules! bit_set {
    ($storage:expr, $index:expr) => {{
        let block = $crate::block_index($index);
        let bit = $crate::bit_index($index);
        $storage[block] = $storage[block] | (1 << bit);
    }};
}

macro_rules! bit_unset {
    ($storage:expr, $index:expr) => {{
        let block = $crate::block_index($index);
        let bit = $crate::bit_index($index);
        $storage[block] = $storage[block] & !(1 << bit);
    }};
}

macro_rules! impl_index {
    ($ty:ty) => {
        impl ::std::ops::Index<usize> for $ty {
            type Output = bool;

            #[inline]
            fn index(&self, index: usize) -> &Self::Output {
                let value = self.get(index).unwrap();
                if value {
                    &true
                } else {
                    &false
                }
            }
        }
    };
}

macro_rules! impl_slice {
    ($ty:ty) => {
        impl<'a> $ty {
            /// Returns the bit at `index`, or `None` if out of bounds.
            #[inline]
            pub fn get(&self, index: usize) -> Option<bool> {
                if index < self.len() {
                    Some(bit_get!(self.storage, self.range.start + index))
                } else {
                    None
                }
            }

            /// Returns the number of bits in the slice.
            #[inline]
            pub fn len(&self) -> usize {
                self.range.len()
            }

            /// Returns the backing words spanned by this slice.
            ///
            /// Note: The first and last words may contain bits outside this slice.
            #[inline]
            pub fn words(&self) -> &[u32] {
                &self.storage
            }

            /// Iterates over the bits in this slice.
            #[inline]
            pub fn iter(&self) -> $crate::Iter<'_> {
                $crate::Iter {
                    slice: $crate::Slice {
                        storage: self.storage,
                        range: self.range.clone(),
                    },
                    index: 0,
                }
            }
        }
    };
}
