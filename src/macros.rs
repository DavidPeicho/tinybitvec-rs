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
    ($ty:ty, $len:expr) => {
        impl ::std::ops::Index<usize> for $ty {
            type Output = bool;

            #[inline]
            fn index(&self, index: usize) -> &Self::Output {
                assert!(index < ($len)(self));
                let value = self.get_unsafe(index);
                if value { &true } else { &false }
            }
        }
    };
}
