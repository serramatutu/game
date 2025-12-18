use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
    marker::PhantomData,
};

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
struct ResourceManagerInner<'res, Res>
where
    Res: Resource<'res>,
{
    next_id: Id<Res::Id>,
    cache: HashMap<Id<Res::Id>, Res>,
}

/// Cache any resources loaded by a `ResourceLoader`
pub struct ResourceManager<'l, 'res, Res, Load>
where
    Load: ResourceLoader<'l, 'res, Res>,
    Res: Resource<'res>,
{
    _pd: PhantomData<&'l u8>,

    pub(super) loader: Load,
    inner: RefCell<ResourceManagerInner<'res, Res>>,
}

pub struct LoadedResource<'rm, 'res, Res, Load>
where
    Res: Resource<'res>,
    Load: ResourceLoader<'rm, 'res, Res>,
{
    _pd: PhantomData<&'res u8>,

    id: Id<Res::Id>,
    manager: &'rm ResourceManager<'rm, 'res, Res, Load>,
}

impl<'rm, 'res, Res, Load> LoadedResource<'rm, 'res, Res, Load>
where
    Res: Resource<'res>,
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

impl<'l, 'res, Res, Load> ResourceManager<'l, 'res, Res, Load>
where
    Res: Resource<'res>,
    Load: ResourceLoader<'l, 'res, Res>,
{
    pub fn new(loader: Load) -> Self {
        ResourceManager {
            _pd: PhantomData,
            loader,
            inner: RefCell::new(ResourceManagerInner {
                next_id: Id::new(0),
                cache: HashMap::new(),
            }),
        }
    }

    /// Load a resource into the cache
    pub fn load(
        &'l self,
        key: &'_ str,
    ) -> Result<LoadedResource<'l, 'res, Res, Load>, ResourceError> {
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
        Ref::<'_, ResourceManagerInner<'res, Res>>::map(self.inner.borrow(), |b| {
            b.cache
                .get(&id)
                .unwrap_or_else(|| panic!("Resource ID '{id:?}' was not loaded"))
        })
    }
}
