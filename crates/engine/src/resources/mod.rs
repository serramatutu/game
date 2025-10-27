use std::path::PathBuf;

use allocator_api2::alloc::Allocator;
use sdl3::video::WindowContext;

pub mod manager;
pub mod sprite_map;

/// Holds all resource managers
pub struct Resources<'res, A: Allocator + Clone> {
    pub sprites: sprite_map::SpriteMapManager<'res, WindowContext, A>,
}

impl<'res, A: Allocator + Clone> Resources<'res, A> {
    pub fn set_root(&mut self, root: impl Into<PathBuf>) {
        self.sprites.loader.root_path = root.into();
    }
}
