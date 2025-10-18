use allocator_api2::alloc::{Allocator, Global as GlobalAllocator};
use hashbrown::{DefaultHashBuilder, HashMap};
use std::path::Path;
use thiserror::Error;

use crate::types::Id;

#[derive(Error, Debug)]
pub enum ResourceError {
    #[error("Resource could not be loaded")]
    LoadFailed,
}

/// Loads a resource of type `Res`
pub trait ResourceLoader<'l, Res> {
    fn load(&'l self, key: &Path) -> Result<Res, ResourceError>
    where
        Res: 'l;
}

/// Cache any resources loaded by a `ResourceLoader`
pub struct ResourceManager<Res, Load, Alloc: Allocator = GlobalAllocator> {
    next_id: Id<Res>,
    loader: Load,
    cache: HashMap<Id<Res>, Res, DefaultHashBuilder, Alloc>,
}

impl<'l, Res, Load, Alloc: Allocator> ResourceManager<Res, Load, Alloc>
where
    Res: 'l,
    Load: ResourceLoader<'l, Res>,
{
    pub fn new(allocator: Alloc, loader: Load) -> Self {
        ResourceManager {
            next_id: Id::new(0),
            cache: HashMap::new_in(allocator),
            loader,
        }
    }

    /// Load a resource into the cache so that subsquent calls to `get()`
    pub fn load(&'l mut self, key: &Path) -> Result<Id<Res>, ResourceError>
    where
        Load: ResourceLoader<'l, Res>,
    {
        let loaded = self.loader.load(key)?;

        let id = self.next_id;
        self.next_id = id.next();
        self.cache.insert(id, loaded);

        Ok(id)
    }

    /// Get a resource that was already preloaded otherwise panic
    pub fn get(&self, id: Id<Res>) -> &Res {
        self.cache
            .get(&id)
            .unwrap_or_else(|| panic!("Resource ID '{id:?}' was not loaded"))
    }
}
