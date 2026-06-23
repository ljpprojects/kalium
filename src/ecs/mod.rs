use std::{iter::FusedIterator, marker::PhantomData, ops, range::Range};

mod bitset;
pub mod map;
pub mod option;
pub mod set;

pub trait EntityRef: Copy + Eq {
    /// Create a reference to an entity from the index
    fn new(idx: usize) -> Self;

    /// Get the index used to create the entity reference
    fn index(self) -> usize;
}

#[macro_export]
macro_rules! entity_impl {
    ($eref:ident : $inner:ty) => {
        impl crate::ecs::EntityRef for $eref {
            fn new(idx: usize) -> Self {
                $eref(idx as $inner)
            }

            fn index(self) -> usize {
                self.0 as usize
            }
        }

        impl Into<usize> for $eref {
            fn into(self) -> usize {
                crate::ecs::EntityRef::index(self)
            }
        }

        impl From<$inner> for $eref {
            fn from(value: $inner) -> Self {
                $eref(value)
            }
        }

        impl crate::ecs::option::InvalidRepr for $eref {
            fn is_valid(self) -> bool {
                self.0 < <$inner>::MAX
            }

            unsafe fn default_invalid() -> Self {
                $eref(<$inner>::MAX)
            }
        }
    };

    ($eref:ident : $inner:ty, !$invalid:literal) => {
        impl crate::ecs::EntityRef for $eref {
            fn new(idx: usize) -> Self {
                $eref(idx as $inner)
            }

            fn index(self) -> usize {
                self.0 as usize
            }
        }

        impl From<usize> for $eref {
            fn from(value: usize) -> Self {
                $eref::new(value)
            }
        }

        impl From<$inner> for $eref {
            fn from(value: $inner) -> Self {
                $eref(value)
            }
        }

        impl crate::ecs::option::InvalidRepr for $eref {
            fn is_valid(self) -> bool {
                self.0 != $invalid
            }

            unsafe fn default_invalid() -> Self {
                $eref($invalid)
            }
        }
    };
}

pub struct IterEntityRef<R>
where
    R: EntityRef
{
    range: Range<usize>,
    _ref: PhantomData<R>,
}

impl<R> Iterator for IterEntityRef<R>
where
    R: EntityRef
{
    type Item = R;

    fn next(&mut self) -> Option<Self::Item> {
        self.range.into_iter().next().map(R::new)
    }
}

impl<R> DoubleEndedIterator for IterEntityRef<R>
where
    R: EntityRef
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.range.into_iter().next_back().map(R::new)
    }
}

impl<R> FusedIterator for IterEntityRef<R>
where
    R: EntityRef
{}

impl<R> ExactSizeIterator for IterEntityRef<R>
where
    R: EntityRef
{}