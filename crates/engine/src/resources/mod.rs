use std::path::PathBuf;

use manager::ResourceError;
use sdl3::video::WindowContext;
use sprite_map::SpriteMap;

use crate::types::Id;

pub mod manager;
pub mod sprite_map;

/// Holds all resource managers
pub struct Resources<'tc> {
    pub root: PathBuf,
    pub sprites: sprite_map::SpriteMapManager<'tc, WindowContext>,
}

impl<'tc> Resources<'tc> {
    pub fn load_sprite_map(
        &'tc mut self,
        name: &str,
    ) -> Result<Id<'tc, SpriteMap<'tc>>, ResourceError> {
        let full_name = self.root.join(name);
        self.sprites.load(&full_name)
    }
}
