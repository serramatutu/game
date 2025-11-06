use std::{
    cell::{Ref, RefCell},
    marker::PhantomData,
};

use allocator_api2::alloc::{Allocator, Global as GlobalAllocator};
use hashbrown::{DefaultHashBuilder, HashMap};
use thiserror::Error;

use crate::types::Id;

#[derive(Error, Debug)]
pub enum ResourceError {
    #[error("Resource could not be loaded")]
    LoadFailed,
}

/// A resource that can be identified by an ID
pub trait Resource<'res> {
    type Id;
}

/// Loads a resource of type `Res`
pub trait ResourceLoader<'l, 'res, Res: Resource<'res>> {
    fn load(&'l self, key: &'_ str) -> Result<Res, ResourceError>;
}

/// Used for interior mutability of `ResourceManager`
struct ResourceManagerInner<'res, Res, Alloc>
where
    Res: Resource<'res>,
    Alloc: Allocator + Clone,
{
    next_id: Id<Res::Id>,
    cache: HashMap<Id<Res::Id>, Res, DefaultHashBuilder, Alloc>,
}

/// Cache any resources loaded by a `ResourceLoader`
pub struct ResourceManager<'l, 'res, Res, Load, Alloc = GlobalAllocator>
where
    Load: ResourceLoader<'l, 'res, Res>,
    Res: Resource<'res>,
    Alloc: Allocator + Clone,
{
    _pd: PhantomData<&'l u8>,

    pub(super) loader: Load,
    inner: RefCell<ResourceManagerInner<'res, Res, Alloc>>,
}

pub struct LoadedResource<'rm, 'res, Res, Load, Alloc>
where
    Res: Resource<'res>,
    Alloc: Allocator + Clone,
    Load: ResourceLoader<'rm, 'res, Res>,
{
    _pd: PhantomData<&'res u8>,

    id: Id<Res::Id>,
    manager: &'rm ResourceManager<'rm, 'res, Res, Load, Alloc>,
}

impl<'rm, 'res, Res, Load, Alloc> LoadedResource<'rm, 'res, Res, Load, Alloc>
where
    Res: Resource<'res>,
    Alloc: Allocator + Clone,
    Load: ResourceLoader<'rm, 'res, Res>,
{
    pub fn and_then<F, R>(self, callback: F) -> R
    where
        F: FnOnce(Id<Res::Id>, Ref<Res>) -> R,
    {
        let res = self.manager.get(self.id);
        callback(self.id, res)
    }
}

impl<'l, 'res, Res, Load, Alloc> ResourceManager<'l, 'res, Res, Load, Alloc>
where
    Res: Resource<'res>,
    Alloc: Allocator + Clone,
    Load: ResourceLoader<'l, 'res, Res>,
{
    pub fn new(allocator: Alloc, loader: Load) -> Self {
        ResourceManager {
            _pd: PhantomData,
            loader,
            inner: RefCell::new(ResourceManagerInner {
                next_id: Id::new(0),
                cache: HashMap::new_in(allocator),
            }),
        }
    }

    /// Load a resource into the cache
    pub fn load(
        &'l self,
        key: &'_ str,
    ) -> Result<LoadedResource<'l, 'res, Res, Load, Alloc>, ResourceError> {
        let id = {
            let mut self_mut = self.inner.borrow_mut();
            let loaded = self.loader.load(key)?;
            let id = self_mut.next_id;
            let existing = self_mut.cache.insert(id, loaded);
            debug_assert!(existing.is_none(), "Double resource load");
            self_mut.next_id = self_mut.next_id.next();
            id
        };

        Ok(LoadedResource {
            id,
            manager: self,
            _pd: PhantomData,
        })
    }

    /// Get a resource that was already preloaded otherwise panic
    pub fn get(&self, id: Id<Res::Id>) -> Ref<'_, Res> {
        Ref::<'_, ResourceManagerInner<'res, Res, Alloc>>::map(self.inner.borrow(), |b| {
            b.cache
                .get(&id)
                .unwrap_or_else(|| panic!("Resource ID '{id:?}' was not loaded"))
        })
    }
}
