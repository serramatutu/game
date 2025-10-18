use std::path::PathBuf;

use manager::ResourceError;
use sdl3::video::WindowContext;
use sprite_map::SpriteMap;

use crate::types::Id;

pub mod manager;
pub mod sprite_map;

/// Holds all resource managers
pub struct Resources<'res> {
    pub root: PathBuf,
    pub sprites: sprite_map::SpriteMapManager<'res, WindowContext>,
}

impl<'res> Resources<'res> {
    pub fn load_sprite_map(
        &'res mut self,
        name: &str,
    ) -> Result<Id<SpriteMap<'res>>, ResourceError> {
        let full_name = self.root.join(name);
        self.sprites.load(&full_name)
    }
}
