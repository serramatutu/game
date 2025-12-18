use std::path::PathBuf;

use sdl3::video::WindowContext;

pub mod manager;
pub mod sprite_map;

/// Holds all resource managers
pub struct Resources<'res> {
    pub sprites: sprite_map::SpriteMapManager<'res, WindowContext>,
}

impl<'res> Resources<'res> {
    pub fn set_root(&mut self, root: impl Into<PathBuf>) {
        self.sprites.loader.root_path = root.into();
    }
}
