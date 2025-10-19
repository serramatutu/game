use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

/// An ID tagged with a type for what it indexes.
///
/// This prevents us from accidentally mixing IDs for different things.
pub struct Id<T>(pub u32, PhantomData<T>);

impl<T> Id<T> {
    pub fn new(val: u32) -> Self {
        Self(val, PhantomData)
    }

    pub fn new_split(hi: u16, lo: u16) -> Self {
        Self::new(((hi as u32) << 16) | (lo as u32))
    }

    pub fn full(&self) -> u32 {
        self.0
    }

    pub fn hi(&self) -> u16 {
        (self.0 >> 16) as u16
    }

    pub fn lo(&self) -> u16 {
        (self.0 & 0xFFFF) as u16
    }

    pub fn next(&self) -> Self {
        Self(self.0 + 1, PhantomData)
    }
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Id<T> {}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<T> Eq for Id<T> {}

impl<T> Hash for Id<T> {
    fn hash<H: Hasher>(&self, h: &mut H) {
        self.0.hash(h);
    }
}

impl<T> Debug for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl<T> Default for Id<T> {
    fn default() -> Self {
        Self(Default::default(), PhantomData)
    }
}

/// A struct that can reset its own state without reallocating
/// or needing a stack assign
pub trait Reset {
    fn reset(&mut self);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_hi_lo() {
        let hi = 1234;
        let lo = 4321;

        let id = Id::<u32>::new_split(hi, lo);

        assert_eq!(id.hi(), hi);
        assert_eq!(id.lo(), lo);
    }
}
