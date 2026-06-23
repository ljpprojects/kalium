use core::ops;

pub trait BitsetBackingStorage: Copy + PartialEq + Default + From<u8> + ops::Shl<Output = Self> + ops::BitOrAssign + ops::BitAnd<Output = Self> + ops::BitAndAssign + ops::Not<Output = Self> {
    const MAX_INDEX: u8;
    fn count_ones(self) -> u32;
    fn trailing_zeroes(self) -> u32;
    fn leading_zeroes(self) -> u32;
}

macro_rules! backing_impl {
    ($int:ty) => {
        impl BitsetBackingStorage for $int {
            const MAX_INDEX: u8 = (size_of::<$int>() * 8 - 1) as u8;

            fn count_ones(self) -> u32 {
                self.count_ones()
            }

            fn trailing_zeroes(self) -> u32 {
                self.trailing_zeros()
            }

            fn leading_zeroes(self) -> u32 {
                self.leading_zeros()
            }
        }
    };
}

backing_impl!(u8);
backing_impl!(u16);
backing_impl!(u32);
backing_impl!(u64);
backing_impl!(usize);

pub struct Bitset<T>(T)
where
    T: BitsetBackingStorage;

impl<T> Bitset<T>
where
    T: BitsetBackingStorage
{
    pub fn new() -> Self {
        Self(T::from(0))
    }

    pub fn len(&self) -> usize {
        self.0.count_ones() as usize
    }

    pub fn contains(&self, i: u8) -> bool {
        if i > T::MAX_INDEX {
            return false
        }

        self.0 & (T::from(1) << T::from(i)) != T::from(0)
    }

    pub fn insert(&mut self, i: u8) -> Option<bool> {
        if i > T::MAX_INDEX {
            return None
        }

        let is_new = self.contains(i);
        self.0 |= T::from(1) << T::from(i);
        Some(is_new)
    }

    pub fn remove(&mut self, i: u8) -> Option<bool> {
        if i > T::MAX_INDEX {
            return None
        }

        let was_present = self.contains(i);
        self.0 &= !(T::from(1) << T::from(i));
        Some(was_present)
    }

    pub fn clear(&mut self) {
        self.0 = T::from(0);
    }

    pub fn min(&self) -> Option<u8> {
        if self.0 == T::from(0) {
            return None
        }

        Some(self.0.trailing_zeroes() as u8)
    }

    pub fn max(&self) -> Option<u8> {
        if self.0 == T::from(0) {
            return None
        }

        Some((T::MAX_INDEX as u32 - self.0.leading_zeroes()) as u8)
    }

    pub fn pop_min(&mut self) -> Option<u8> {
        let min = self.min()?;
        self.remove(min);
        Some(min)
    }

    pub fn pop_max(&mut self) -> Option<u8> {
        let min = self.max()?;
        self.remove(min);
        Some(min)
    }
}

pub struct ExpandingBitset<T>
where
    T: BitsetBackingStorage
{
    sets: Vec<Bitset<T>>,
}

impl<T> ExpandingBitset<T>
where
    T: BitsetBackingStorage
{
    pub fn new() -> Self {
        Self {
            sets: Vec::new(),
        }
    }

    fn get_sets_index_for(&self, i: usize) -> usize {
        i.div_floor((T::MAX_INDEX + 1) as usize)
    }

    fn get_set_offset_for(&self, i: usize) -> u8 {
        (i % (T::MAX_INDEX as usize + 1)) as u8
    }

    pub fn len(&self) -> usize {
        let mut acc = 0;
        for set in self.sets.iter() {
            acc += set.len();
        }

        acc
    }

    pub fn contains(&self, i: usize) -> bool {
        let seti = self.get_sets_index_for(i);
        if seti >= self.sets.len() {
            return false
        }

        let seto = self.get_set_offset_for(i);
        self.sets[seti].contains(seto)
    }

    pub fn insert(&mut self, i: usize) -> bool {
        let seti = self.get_sets_index_for(i);
        if seti >= self.sets.len() {
            self.sets.resize_with(seti + 1, Bitset::new);
        }

        let seto = self.get_set_offset_for(i);
        eprintln!("seti = {seti}, seto = {seto}");
        self.sets[seti].insert(seto).unwrap()
    }

    pub fn remove(&mut self, i: usize) -> Option<bool> {
        let seti = self.get_sets_index_for(i);
        if seti >= self.sets.len() {
            return None
        }

        let seto = self.get_set_offset_for(i);
        self.sets[seti].remove(seto)
    }

    pub fn clear(&mut self) {
        for set in self.sets.iter_mut() {
            set.clear();
        }
    }

    pub fn min(&self) -> Option<usize> {
        let first_set = self.sets.first()?;
        Some(first_set.min()? as usize)
    }

    pub fn max(&self) -> Option<usize> {
        let last_set = self.sets.last()?;
        Some(last_set.max()? as usize + (self.sets.len() - 1) as usize * (T::MAX_INDEX + 1) as usize)
    }

    pub fn pop_min(&mut self) -> Option<usize> {
        let min = self.min()?;
        self.remove(min);
        Some(min)
    }

    pub fn pop_max(&mut self) -> Option<usize> {
        let min = self.max()?;
        self.remove(min);
        Some(min)
    }
}

mod tests {
    use crate::ecs::bitset::{Bitset, ExpandingBitset};

    #[test]
    fn bitset() {
        let mut set = Bitset::<u64>::new();

        set.insert(17);
        set.insert(3);
        set.insert(59);

        assert!(set.contains(3));
        assert!(set.contains(17));
        assert!(set.contains(59));

        assert!(!set.contains(2));
        assert!(!set.contains(19));
        assert!(!set.contains(63));

        assert_eq!(set.min(), Some(3));
        assert_eq!(set.max(), Some(59));

        assert_eq!(set.len(), 3);

        set.clear();

        assert_eq!(set.len(), 0);
    }

    #[test]
    fn expanding_bitset() {
        let mut set = ExpandingBitset::<u64>::new();

        set.insert(65);
        set.insert(3);
        set.insert(198);
        set.insert(164);

        assert!(set.contains(3));
        assert!(set.contains(65));
        assert!(set.contains(164));
        assert!(set.contains(198));

        assert!(!set.contains(1));
        assert!(!set.contains(69));
        assert!(!set.contains(203));

        assert_eq!(set.min(), Some(3));
        assert_eq!(set.max(), Some(198));

        assert_eq!(set.len(), 4);

        set.clear();

        assert_eq!(set.len(), 0);
    }
}