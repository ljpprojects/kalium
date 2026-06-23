use std::{iter::FusedIterator, marker::PhantomData};
use crate::ecs::{EntityRef, bitset::{Bitset, BitsetBackingStorage, ExpandingBitset}};

/// A set of unique entity references, backed by a bitset with the storage S
pub struct FixedRefSet<R, S>
where
    R: EntityRef,
    S: BitsetBackingStorage
{
    storage: Bitset<S>,
    _ref: PhantomData<R>,
}

impl<R, S> FixedRefSet<R, S>
where
    R: EntityRef,
    S: BitsetBackingStorage
{
    pub fn new() -> Self {
        Self {
            storage: Bitset::new(),
            _ref: PhantomData,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.storage.len()
    }

    pub fn insert(&mut self, eref: R) -> Option<bool> {
        if eref.index() > 255 {
            return None
        }

        self.storage.insert(eref.index() as u8)
    }

    pub fn contains(&self, eref: R) -> bool {
        if eref.index() > 255 {
            return false
        }

        self.storage.contains(eref.index() as u8)
    }

    pub fn clear(&mut self) {
        self.storage.clear();
    }
}

impl<R, S> Iterator for FixedRefSet<R, S>
where
    R: EntityRef,
    S: BitsetBackingStorage
{
    type Item = R;

    fn next(&mut self) -> Option<Self::Item> {
        self.storage.pop_min().map(|i| R::new(i as usize))
    }
}

/// A set of unique entity references, backed by an expanding bitset with the storage S
pub struct ExpandingRefSet<R, S>
where
    R: EntityRef,
    S: BitsetBackingStorage
{
    storage: ExpandingBitset<S>,
    _ref: PhantomData<R>,
}

impl<R, S> ExpandingRefSet<R, S>
where
    R: EntityRef,
    S: BitsetBackingStorage
{
    pub fn new() -> Self {
        Self {
            storage: ExpandingBitset::new(),
            _ref: PhantomData,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.storage.len()
    }

    #[inline]
    pub fn insert(&mut self, eref: R) -> bool {
        self.storage.insert(eref.index())
    }

    #[inline]
    pub fn contains(&self, eref: R) -> bool {
        self.storage.contains(eref.index())
    }

    #[inline]
    pub fn clear(&mut self) {
        self.storage.clear();
    }
}

impl<R, S> Iterator for ExpandingRefSet<R, S>
where
    R: EntityRef,
    S: BitsetBackingStorage
{
    type Item = R;

    fn next(&mut self) -> Option<Self::Item> {
        self.storage.pop_min().map(R::new)
    }
}

impl<R, S> DoubleEndedIterator for ExpandingRefSet<R, S>
where
    R: EntityRef,
    S: BitsetBackingStorage
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.storage.pop_max().map(R::new)
    }
}

impl<R, S> FusedIterator for ExpandingRefSet<R, S>
where
    R: EntityRef,
    S: BitsetBackingStorage
{}

impl<R, S> ExactSizeIterator for ExpandingRefSet<R, S>
where
    R: EntityRef,
    S: BitsetBackingStorage
{}