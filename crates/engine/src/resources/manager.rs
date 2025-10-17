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
pub trait ResourceLoader<'this, Res>
where
    Res: 'this,
{
    fn load(&'this self, key: &Path) -> Result<Res, ResourceError>;
}

/// Cache any resources loaded by a `ResourceLoader`
pub struct ResourceManager<'this, Res, Load, Alloc: Allocator = GlobalAllocator>
where
    Res: 'this,
    Load: 'this + ResourceLoader<'this, Res>,
{
    next_id: Id<'this, Res>,
    loader: Load,
    cache: HashMap<Id<'this, Res>, Res, DefaultHashBuilder, Alloc>,
}

impl<'this, Res, Load, Alloc: Allocator> ResourceManager<'this, Res, Load, Alloc>
where
    Res: 'this,
    Load: ResourceLoader<'this, Res>,
{
    pub fn new(allocator: Alloc, loader: Load) -> Self {
        ResourceManager {
            next_id: Id::new(0),
            cache: HashMap::new_in(allocator),
            loader,
        }
    }

    /// Load a resource into the cache so that subsquent calls to `get()`
    pub fn load(&'this mut self, key: &Path) -> Result<Id<'this, Res>, ResourceError>
    where
        Load: ResourceLoader<'this, Res>,
    {
        let loaded = self.loader.load(key)?;

        let id = self.next_id;
        self.next_id = id.next();
        self.cache.insert(id, loaded);

        Ok(id)
    }

    /// Get a resource that was already preloaded otherwise panic
    pub fn get(&'this self, id: Id<'this, Res>) -> &'this Res {
        self.cache
            .get(&id)
            .unwrap_or_else(|| panic!("Resource ID '{id:?}' was not loaded"))
    }
}
