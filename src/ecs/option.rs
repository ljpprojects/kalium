use std::{cmp, mem::MaybeUninit, ops::Deref, ptr::{self, Pointee}};

pub trait InvalidRepr: Clone + Copy {
    fn is_valid(self) -> bool;

    /// Construct an invalid representation of T. This is unsafe for obvious reasons.
    unsafe fn default_invalid() -> Self;
}

impl<T> InvalidRepr for &T
where
    T: ?Sized
{
    fn is_valid(self) -> bool {
        let (dat, _) = (&raw const self).to_raw_parts();
        dat as usize == 0
    }

    unsafe fn default_invalid() -> Self {
        let dat = 0 as *const ();
        let met = unsafe {
            MaybeUninit::<<T as Pointee>::Metadata>::uninit().assume_init()
        };

        let p: *const T = ptr::from_raw_parts(dat, met);
        unsafe {
            p.as_ref_unchecked()
        }
    }
}

impl<T> InvalidRepr for *const T
where
    T: ?Sized
{
    fn is_valid(self) -> bool {
        let (dat, _) = self.to_raw_parts();
        dat as usize == 0
    }

    unsafe fn default_invalid() -> Self {
        let dat = 0 as *const ();
        let met = unsafe {
            MaybeUninit::<<T as Pointee>::Metadata>::uninit().assume_init()
        };

        ptr::from_raw_parts::<T>(dat, met)
    }
}

impl<T> InvalidRepr for *mut T
where
    T: ?Sized
{
    fn is_valid(self) -> bool {
        let (dat, _) = self.to_raw_parts();
        dat as usize == 0
    }

    unsafe fn default_invalid() -> Self {
        let dat = 0 as *mut ();
        let met = unsafe {
            MaybeUninit::<<T as Pointee>::Metadata>::uninit().assume_init()
        };

        ptr::from_raw_parts_mut::<T>(dat, met)
    }
}

impl<A, B> InvalidRepr for (A, B)
where
    A: InvalidRepr,
    B: InvalidRepr
{
    fn is_valid(self) -> bool {
        self.0.is_valid() && self.1.is_valid()
    }

    unsafe fn default_invalid() -> Self {
        unsafe { (A::default_invalid(), B::default_invalid()) }
    }
}

impl<A, B, C> InvalidRepr for (A, B, C)
where
    A: InvalidRepr,
    B: InvalidRepr,
    C: InvalidRepr
{
    fn is_valid(self) -> bool {
        self.0.is_valid() && self.1.is_valid() && self.2.is_valid()
    }

    unsafe fn default_invalid() -> Self {
        unsafe { (A::default_invalid(), B::default_invalid(), C::default_invalid()) }
    }
}

pub struct CompactOption<T>(T)
where
    T: InvalidRepr;

impl<T> CompactOption<T>
where
    T: InvalidRepr
{
    pub fn some(val: T) -> Self {
        Self(val)
    }

    pub fn none() -> Self {
        Self(unsafe { T::default_invalid() })
    }

    pub fn is_some(&self) -> bool {
        self.0.is_valid()
    }

    pub fn is_none(&self) -> bool {
        !self.is_some()
    }

    pub fn unwrap(self) -> T {
        if self.is_none() {
            panic!("Attempted to unwrap none CompactOption.")
        }

        self.0
    }

    pub fn unwrap_or_else<D>(self, default_val: D) -> T
    where
        D: FnOnce() -> T
    {
        if self.is_none() {
            return default_val();
        }

        self.0
    }

    pub fn expect(self, message: &str) -> T {
        if !self.0.is_valid() {
            panic!("Attempted to unwrap none CompactOption: {message}")
        }

        self.0
    }

    pub fn as_ref(&self) -> CompactOption<&T> {
        if self.is_none() {
            CompactOption::none()
        } else {
            CompactOption::some(&self.0)
        }
    }

    pub fn take(&mut self) -> CompactOption<T> {
        let prev = *self;
        self.0 = unsafe { T::default_invalid() };
        prev
    }

    pub fn take_if<P>(&mut self, p: P) -> CompactOption<T>
    where
        P: FnOnce(&mut T) -> bool
    {
        if self.is_some() && p(&mut self.0) {
            let prev = *self;
            self.0 = unsafe { T::default_invalid() };
            return prev
        }

        *self
    }

    pub fn map<M, U>(self, mapping: M) -> CompactOption<U>
    where
        U: InvalidRepr,
        M: FnOnce(T) -> U
    {
        if self.is_none() {
            return CompactOption::none()
        }

        CompactOption::some(mapping(self.0))
    }

    pub fn map_or_else<D, M, U>(self, mapping: M, default_val: D) -> U
    where
        U: InvalidRepr,
        M: FnOnce(T) -> U,
        D: FnOnce() -> U
    {
        if self.is_none() {
            return default_val()
        }

        mapping(self.0)
    }

    pub fn map_or_default<D, M, U>(self, mapping: M) -> U
    where
        U: Default + InvalidRepr,
        M: FnOnce(T) -> U
    {
        if self.is_none() {
            return U::default();
        }

        mapping(self.0)
    }
}

impl<T> InvalidRepr for CompactOption<T>
where
    T: InvalidRepr
{
    #[inline]
    fn is_valid(self) -> bool {
        self.is_none()
    }

    /// Creates a none CompactOption, which is a valid representation of CompactOption but makes the flattening of a CompactOption<CompactOption<T>> clearer.
    #[inline]
    unsafe fn default_invalid() -> Self {
        Self::none()
    }
}

impl<T> CompactOption<CompactOption<T>>
where
    T: InvalidRepr
{
    #[inline]
    pub fn flatten(self) -> CompactOption<T> {
        self.0 // If self is none, then self.0 must also be none (as guaranteed by CompacOption::default_invalid). If self is not none, then we can just return the value inside. Either way, self.0 has the flattened representation.
    }
}

impl<T> CompactOption<T>
where
    T: Default + InvalidRepr
{
    pub fn unwrap_or_default<D>(self) -> T {
        if self.is_none() {
            return T::default();
        }

        self.0
    }
}

impl<T> CompactOption<T>
where
    T: Deref + InvalidRepr,
{
    pub fn as_deref(&self) -> CompactOption<&<T as Deref>::Target> {
        if self.is_none() {
            CompactOption::none()
        } else {
            CompactOption::some(self.0.deref())
        }
    }
}

impl<T> Clone for CompactOption<T>
where
    T: Clone + InvalidRepr
{
    fn clone(&self) -> Self {
        if self.is_some() {
            Self::some(self.0.clone())
        } else {
            Self::none()
        }
    }
}

impl<T> PartialEq for CompactOption<T>
where
    T: PartialEq + InvalidRepr
{
    fn eq(&self, other: &Self) -> bool {
        self.is_none() && other.is_none() || self.0 == other.0
    }
}

impl<T> Eq for CompactOption<T>
where
    T: Eq + PartialEq + InvalidRepr
{}

impl<T> PartialOrd for CompactOption<T>
where
    T: PartialOrd + InvalidRepr
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.is_none() && other.is_none() {
            Some(cmp::Ordering::Equal)
        } else if self.is_none() && other.is_some() {
            Some(cmp::Ordering::Less)
        } else if self.is_some() && other.is_none() {
            Some(cmp::Ordering::Greater)
        } else {
            self.0.partial_cmp(&other.0)
        }
    }
}

impl<T> Ord for CompactOption<T>
where
    T: Ord + PartialOrd + InvalidRepr
{
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        if self.is_none() && other.is_none() {
            cmp::Ordering::Equal
        } else if self.is_none() && other.is_some() {
            cmp::Ordering::Less
        } else if self.is_some() && other.is_none() {
            cmp::Ordering::Greater
        } else {
            self.0.cmp(&other.0)
        }
    }
}

impl<T> Copy for CompactOption<T>
where
    T: Clone + Copy + InvalidRepr
{}

unsafe impl<T> Send for CompactOption<T>
where
    T: Send + InvalidRepr
{}

unsafe impl<T> Sync for CompactOption<T>
where
    T: Sync + InvalidRepr
{}