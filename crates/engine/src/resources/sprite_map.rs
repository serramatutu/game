use std::path::Path;

use indexmap::IndexMap;
use sdl3::image::LoadTexture;
use sdl3::render::{FRect, ScaleMode, Texture, TextureCreator};
use serde::{Deserialize, Serialize};

use crate::coords::WorldSize;

use super::manager::{ResourceError, ResourceLoader, ResourceManager};

#[derive(Serialize, Deserialize)]
struct AsepriteRect {
    x: u16,
    y: u16,
    w: u16,
    h: u16,
}

impl AsepriteRect {
    /// Turn this into an SDL `FRect` for rendering
    fn to_sdl(&self) -> FRect {
        FRect {
            x: self.x as f32,
            y: self.y as f32,
            w: self.w as f32,
            h: self.h as f32,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct AsepriteSize {
    w: u16,
    h: u16,
}

/// Metadata about a single frame in a `SpriteMap`
#[derive(Serialize, Deserialize)]
pub struct FrameMetadata {
    #[serde(rename(deserialize = "frame"))]
    rect: AsepriteRect,
    rotated: bool,
    trimmed: bool,
    #[serde(rename(deserialize = "spriteSourceSize"))]
    sprite_source_size: AsepriteRect,
    #[serde(rename(deserialize = "sourceSize"))]
    source_size: AsepriteSize,
    duration: u16,
}

/// A sprite map JSON file that contains the metadata about individual frames.
///
/// This is in the same format as exported by Aseprite.
#[derive(Deserialize)]
pub struct SpriteMapMetadata {
    frames: IndexMap<String, FrameMetadata>,
}

// Holds many sprites in one single image. Each frame can be indexed from this map.
pub struct SpriteMap<'sdlcanvas> {
    pub tex: Texture<'sdlcanvas>,
    metadata: SpriteMapMetadata,
}

impl<'sdlcanvas> SpriteMap<'sdlcanvas> {
    /// Get the rect in the sprite for a given frame
    pub fn get_frame_rect(&self, index: usize) -> Option<FRect> {
        let frame = self.metadata.frames.get_index(index);
        let (_name, frame_meta) = frame?;
        Some(frame_meta.rect.to_sdl())
    }

    /// Get the size of this texture in world coordinates
    pub fn get_world_size(&self, pixel_to_world: f32, index: usize) -> Option<WorldSize> {
        let frame = self.metadata.frames.get_index(index);
        let (_name, frame_meta) = frame?;
        let pixel_rect = frame_meta.rect.to_sdl();
        Some(WorldSize::new(
            pixel_rect.w * pixel_to_world,
            pixel_rect.h * pixel_to_world,
        ))
    }
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
        let meta_path = full_path.with_extension("json");

        assert!(
            tex_path.is_file(),
            "No PNG found for sprite map ({}).",
            tex_path.to_str().unwrap_or("not a path")
        );
        assert!(
            meta_path.is_file(),
            "No metadata found for sprite map ({}).",
            meta_path.to_str().unwrap_or("not a path")
        );

        let meta_str = std::fs::read_to_string(meta_path).or(Err(ResourceError::LoadFailed))?;
        let metadata: SpriteMapMetadata =
            serde_json::from_str(&meta_str).or(Err(ResourceError::LoadFailed))?;

        let mut tex = self
            .sdl_loader
            .load_texture(tex_path)
            .or(Err(ResourceError::LoadFailed))?;
        tex.set_scale_mode(ScaleMode::Nearest);

        Ok(SpriteMap { tex, metadata })
    }
}

/// A resource manager for `SpriteMap`
pub type SpriteMapManager<'sdlcanvas, T> =
    ResourceManager<'sdlcanvas, SpriteMap<'sdlcanvas>, SpriteMapLoader<T>>;
