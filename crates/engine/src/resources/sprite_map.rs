use std::path::Path;

use sdl3::image::LoadTexture;
use sdl3::render::{Texture, TextureCreator};
use serde::Deserialize;

use super::manager::{ResourceError, ResourceLoader, ResourceManager};

/// A sprite map JSON file that contains the metadata about individual frames.
///
/// This is in the same format as exported by Aseprite.
#[derive(Deserialize)]
pub struct SpriteMapMetadata {}

// Holds many sprites in one single image. Each frame can be indexed from this map.
pub struct SpriteMap<'sdlcanvas> {
    pub tex: Texture<'sdlcanvas>,
    // TODO
    // meta: SpriteMapMetadata,
}

/// Loads a `SpriteMap` from a PNG and a JSON file
pub struct SpriteMapLoader<T> {
    sdl_loader: TextureCreator<T>,
}

impl<T> SpriteMapLoader<T> {
    pub fn new(sdl_loader: TextureCreator<T>) -> SpriteMapLoader<T> {
        Self { sdl_loader }
    }
}

impl<'this, T> ResourceLoader<'this, SpriteMap<'this>> for SpriteMapLoader<T> {
    fn load(
        &'this self,
        full_path: &Path,
    ) -> Result<SpriteMap<'this>, super::manager::ResourceError> {
        let tex_path = full_path.with_extension("png");
        // let meta_path = full_path.with_extension("json");

        assert!(
            tex_path.is_file(),
            "No PNG found for sprite map ({}).",
            tex_path.to_str().unwrap_or("not a path")
        );
        // assert!(meta_path.is_file(), "No metadata found for sprite map ({}).", meta_path.to_str().unwrap_or("not a path"));

        // let meta_str = std::fs::read_to_string(meta_path).or(Err(ResourceError::LoadFailed))?;
        // let meta: SpriteMapMetadata = serde_json::from_str(&meta_str).or(Err(ResourceError::LoadFailed))?;

        let tex = self
            .sdl_loader
            .load_texture(tex_path)
            .or(Err(ResourceError::LoadFailed))?;

        Ok(SpriteMap {
            tex,
            // meta,
        })
    }
}

/// A resource manager for `SpriteMap`
pub type SpriteMapManager<'sdlcanvas, T> =
    ResourceManager<'sdlcanvas, SpriteMap<'sdlcanvas>, SpriteMapLoader<T>>;
