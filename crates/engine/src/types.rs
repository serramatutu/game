use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

/// An ID tagged with a type for what it indexes.
///
/// This prevents us from accidentally mixing IDs for different things.
pub struct Id<'container, T>(pub u32, PhantomData<&'container T>);

impl<'container, T> Id<'container, T> {
    pub fn new(val: u32) -> Self {
        Self(val, PhantomData)
    }

    pub fn next(&self) -> Self {
        Self(self.0 + 1, PhantomData)
    }
}

impl<'container, T> Clone for Id<'container, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'container, T> Copy for Id<'container, T> {}

impl<'container, T> PartialEq for Id<'container, T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<'container, T> Eq for Id<'container, T> {}

impl<'container, T> Hash for Id<'container, T> {
    fn hash<H: Hasher>(&self, h: &mut H) {
        self.0.hash(h);
    }
}

impl<'container, T> Debug for Id<'container, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
