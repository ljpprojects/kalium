use std::{marker::PhantomData, mem};

use crate::ecs::EntityRef;

/// Key-value map which owns all entities inside of it
pub struct OwnedMap<E, R>
where
    R: EntityRef
{
    entities: Vec<E>,
    _ref: PhantomData<R>,
}

impl<E, R> OwnedMap<E, R>
where
    R: EntityRef
{
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            _ref: PhantomData
        }
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self {
            entities: Vec::with_capacity(cap),
            _ref: PhantomData,
        }
    }

    /// Checks if an entity reference could refer to an entity in the map
    pub fn ref_is_valid(&self, r: R) -> bool {
        r.index() < self.entities.len()
    }

    /// Add an entity into the map, returning an entity reference for it
    pub fn insert(&mut self, entity: E) -> R {
        let eref = R::new(self.entities.len());
        self.entities.push(entity);
        eref
    }

    pub fn get(&self, eref: R) -> Option<&E> {
        self.entities.get(eref.index())
    }

    pub fn get_mut(&mut self, eref: R) -> Option<&mut E> {
        self.entities.get_mut(eref.index())
    }

    pub fn len(&self) -> usize {
        self.entities.len()
    }
}

/// A map that associates owned data with entity references
pub struct RefMap<D, R>
where
    D: Clone,
    R: EntityRef
{
    data: Vec<D>,
    default: D,
    _ref: PhantomData<R>
}

impl<D, R> RefMap<D, R>
where
    D: Clone,
    R: EntityRef
{
    pub fn new() -> Self
    where
        D: Default
    {
        Self {
            data: Vec::new(),
            default: D::default(),
            _ref: PhantomData,
        }
    }

    pub fn with_default(default: D) -> Self {
        Self {
            data: Vec::new(),
            default,
            _ref: PhantomData,
        }
    }

    pub fn with_capacity(cap: usize) -> Self
    where
        D: Default
    {
        Self {
            data: Vec::with_capacity(cap),
            default: D::default(),
            _ref: PhantomData,
        }
    }

    pub fn with_default_and_capacity(default: D, cap: usize) -> Self {
        Self {
            data: Vec::with_capacity(cap),
            default,
            _ref: PhantomData,
        }
    }

    /// Checks if an entity reference could refer to an entity in the map
    pub fn ref_is_valid(&self, r: R) -> bool {
        r.index() < self.data.len()
    }

    pub fn get(&self, eref: R) -> Option<&D> {
        self.data.get(eref.index())
    }

    pub fn get_mut(&mut self, eref: R) -> Option<&mut D> {
        self.data.get_mut(eref.index())
    }

    /// Remove the element the given entity reference references, replacing it
    /// with the default value and returning the old value.
    pub fn remove(&mut self, eref: R) -> Option<D> {
        if !self.ref_is_valid(eref) {
            return None
        }

        let idx = eref.index();
        let default = self.default.clone();

        Some(mem::replace(&mut self.data[idx], default))
    }

    fn ensure_space(&mut self, eref: R) {
        if eref.index() < self.data.len() {
            return
        }

        self.data.resize(eref.index(), self.default.clone());
    }

    /// Insert data into the RefMap, associating it with an entity reference.
    pub fn insert(&mut self, data: D, eref: R) {
        self.ensure_space(eref);
        self.data[eref.index()] = data;
    }
}